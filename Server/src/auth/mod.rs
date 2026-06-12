use std::sync::Arc;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use sqlx::SqlitePool;

use mir2_shared::net::packet_id::ClientOpcode;
use mir2_shared::packets::server::{
    ChangePasswordBannedPacket, ChangePasswordResponsePacket, DeleteCharacterResponsePacket,
    DeleteCharacterSuccessPacket, LoginBannedPacket, LoginResponsePacket, LoginSuccessPacket,
    NewAccountResponsePacket, NewCharacterResponsePacket, NewCharacterSuccessPacket,
    StartGameResponsePacket,
};
use mir2_shared::packets::Packet;

use crate::database::models::{Account, NewCharacterInput};
use crate::database::repository::{AccountRepository, CharacterRepository};
use crate::network::handler::PacketRouter;
use crate::network::session_manager::{SessionManager, SessionState};

/// 认证处理器
///
/// 处理所有与登录、注册、角色管理相关的客户端包。
pub struct AuthHandler {
    pool: SqlitePool,
    session_manager: Arc<SessionManager>,
}

impl AuthHandler {
    pub fn new(pool: SqlitePool, session_manager: Arc<SessionManager>) -> Self {
        Self {
            pool,
            session_manager,
        }
    }

    /// 在 PacketRouter 上注册所有认证包 handler
    pub fn register_all(&self, router: &PacketRouter) {
        self.register_login(router);
        self.register_new_account(router);
        self.register_change_password(router);
        self.register_new_character(router);
        self.register_delete_character(router);
        self.register_start_game(router);

        tracing::info!(
            "Registered auth handlers, total: {}",
            router.handler_count()
        );
    }

    /// 通过 id 查询账号
    async fn find_account_by_id(pool: &SqlitePool, account_id: i64) -> Result<Option<Account>, sqlx::Error> {
        sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE id = ?")
            .bind(account_id)
            .fetch_optional(pool)
            .await
    }

    /// 注册登录包 handler (ClientOpcode::Login = 5)
    fn register_login(&self, router: &PacketRouter) {
        let pool = self.pool.clone();
        let sm = Arc::clone(&self.session_manager);

        router.register(ClientOpcode::Login as u16, move |session_id, payload| {
            let pool = pool.clone();
            let sm = sm.clone();
            let payload = payload.to_vec();

            tokio::spawn(async move {
                if payload.len() < 4 {
                    tracing::warn!("Session {} Login packet too short", session_id);
                    send_login_result(&sm, session_id, 1).await;
                    return;
                }

                let username_len = u16::from_le_bytes([payload[0], payload[1]]) as usize;
                if payload.len() < 2 + username_len + 2 {
                    tracing::warn!("Session {} Login payload too short for username", session_id);
                    send_login_result(&sm, session_id, 1).await;
                    return;
                }

                let username = String::from_utf8_lossy(&payload[2..2 + username_len]).to_string();
                let offset = 2 + username_len;
                let password_len = u16::from_le_bytes([payload[offset], payload[offset + 1]]) as usize;

                if payload.len() < offset + 2 + password_len {
                    tracing::warn!("Session {} Login payload too short for password", session_id);
                    send_login_result(&sm, session_id, 1).await;
                    return;
                }

                let password = String::from_utf8_lossy(&payload[offset + 2..offset + 2 + password_len])
                    .to_string();

                tracing::info!("Session {} login attempt: username={}", session_id, username);

                // 查找账号
                let repo = AccountRepository::new(pool.clone());
                let account = match repo.find_by_username(&username).await {
                    Ok(Some(acc)) => acc,
                    Ok(None) => {
                        tracing::warn!("Login failed: account not found: {}", username);
                        send_login_result(&sm, session_id, 2).await;
                        return;
                    }
                    Err(e) => {
                        tracing::error!("Database error during login: {}", e);
                        send_login_result(&sm, session_id, 1).await;
                        return;
                    }
                };

                // 检查是否被封禁
                if account.banned != 0 {
                    tracing::warn!("Login failed: account banned: {}", username);
                    send_login_banned(&sm, session_id).await;
                    return;
                }

                // 验证密码
                let password_match = match argon2::password_hash::PasswordHash::new(&account.password) {
                    Ok(parsed_hash) => {
                        Argon2::default()
                            .verify_password(password.as_bytes(), &parsed_hash)
                            .is_ok()
                    }
                    Err(_) => false,
                };

                if !password_match {
                    tracing::warn!("Login failed: wrong password for: {}", username);
                    send_login_result(&sm, session_id, 3).await;
                    return;
                }

                // 登录成功
                tracing::info!("Login success: username={}, account_id={}", username, account.id);

                // 查询角色数
                let char_repo = CharacterRepository::new(pool.clone());
                let char_count = match char_repo.count_by_account(account.id).await {
                    Ok(count) => count as u8,
                    Err(_) => 0,
                };

                // 设置 session state
                sm.set_state(session_id, SessionState::new(account.id)).await;

                // 发送登录成功包
                let packet = LoginSuccessPacket::new(account.id as u32, char_count);
                if let Ok(data) = packet.encode() {
                    sm.send_to_session(session_id, &data).await;
                }

                // 发送角色列表（每个角色用一个 NewCharacterSuccess 包）
                let characters = char_repo.find_by_account(account.id).await.unwrap_or_default();
                for character in &characters {
                    let ci = character.to_char_info(character.id as u32);
                    let char_packet = NewCharacterSuccessPacket::new(
                        ci.name, ci.class, ci.gender, ci.level,
                        ci.hp, ci.mp, ci.max_hp, ci.max_mp,
                    ).with_id(character.id as u32);
                    if let Ok(data) = char_packet.encode() {
                        sm.send_to_session(session_id, &data).await;
                    }
                }
            });
        });
    }

    /// 注册新账号包 handler (ClientOpcode::NewAccount = 3)
    fn register_new_account(&self, router: &PacketRouter) {
        let pool = self.pool.clone();
        let sm = Arc::clone(&self.session_manager);

        router.register(ClientOpcode::NewAccount as u16, move |session_id, payload| {
            let pool = pool.clone();
            let sm = sm.clone();
            let payload = payload.to_vec();

            tokio::spawn(async move {
                if payload.len() < 4 {
                    send_new_account_result(&sm, session_id, 1).await;
                    return;
                }

                let username_len = u16::from_le_bytes([payload[0], payload[1]]) as usize;
                if payload.len() < 2 + username_len + 2 {
                    send_new_account_result(&sm, session_id, 1).await;
                    return;
                }

                let username = String::from_utf8_lossy(&payload[2..2 + username_len]).to_string();
                let offset = 2 + username_len;
                let password_len = u16::from_le_bytes([payload[offset], payload[offset + 1]]) as usize;

                if payload.len() < offset + 2 + password_len {
                    send_new_account_result(&sm, session_id, 1).await;
                    return;
                }

                let password = String::from_utf8_lossy(&payload[offset + 2..offset + 2 + password_len])
                    .to_string();

                // 校验用户名长度
                if username.len() < 3 || username.len() > 20 {
                    send_new_account_result(&sm, session_id, 1).await;
                    return;
                }

                // 校验密码长度
                if password.len() < 3 || password.len() > 64 {
                    send_new_account_result(&sm, session_id, 1).await;
                    return;
                }

                // 哈希密码
                let password_hash = match hash_password(&password) {
                    Some(h) => h,
                    None => {
                        send_new_account_result(&sm, session_id, 1).await;
                        return;
                    }
                };

                let repo = AccountRepository::new(pool.clone());
                match repo.create(&username, &password_hash).await {
                    Ok(_account_id) => {
                        tracing::info!("New account created: username={}", username);
                        send_new_account_result(&sm, session_id, 0).await;
                    }
                    Err(e) => {
                        tracing::warn!("Failed to create account {}: {}", username, e);
                        send_new_account_result(&sm, session_id, 1).await;
                    }
                }
            });
        });
    }

    /// 注册修改密码包 handler (ClientOpcode::ChangePassword = 4)
    fn register_change_password(&self, router: &PacketRouter) {
        let pool = self.pool.clone();
        let sm = Arc::clone(&self.session_manager);

        router.register(ClientOpcode::ChangePassword as u16, move |session_id, payload| {
            let pool = pool.clone();
            let sm = sm.clone();
            let payload = payload.to_vec();

            tokio::spawn(async move {
                // 检查是否已认证
                let state = match sm.get_state(session_id).await {
                    Some(s) if s.authenticated => s,
                    _ => {
                        send_change_password_banned(&sm, session_id).await;
                        return;
                    }
                };

                if payload.len() < 4 {
                    send_change_password_result(&sm, session_id, 1).await;
                    return;
                }

                let old_len = u16::from_le_bytes([payload[0], payload[1]]) as usize;
                if payload.len() < 2 + old_len + 2 {
                    send_change_password_result(&sm, session_id, 1).await;
                    return;
                }

                let old_pwd = String::from_utf8_lossy(&payload[2..2 + old_len]).to_string();
                let offset = 2 + old_len;
                let new_len = u16::from_le_bytes([payload[offset], payload[offset + 1]]) as usize;

                if payload.len() < offset + 2 + new_len {
                    send_change_password_result(&sm, session_id, 1).await;
                    return;
                }

                let new_pwd = String::from_utf8_lossy(&payload[offset + 2..offset + 2 + new_len])
                    .to_string();

                // 用 id 查询账号
                let account = match AuthHandler::find_account_by_id(&pool, state.account_id).await {
                    Ok(Some(a)) => a,
                    _ => {
                        send_change_password_result(&sm, session_id, 1).await;
                        return;
                    }
                };

                // 验证旧密码
                let old_ok = match argon2::password_hash::PasswordHash::new(&account.password) {
                    Ok(parsed_hash) => {
                        Argon2::default()
                            .verify_password(old_pwd.as_bytes(), &parsed_hash)
                            .is_ok()
                    }
                    Err(_) => false,
                };

                if !old_ok {
                    send_change_password_result(&sm, session_id, 1).await;
                    return;
                }

                // 对新密码哈希
                let new_hash = match hash_password(&new_pwd) {
                    Some(h) => h,
                    None => {
                        send_change_password_result(&sm, session_id, 1).await;
                        return;
                    }
                };

                let repo = AccountRepository::new(pool.clone());
                match repo.update_password(account.id, &new_hash).await {
                    Ok(true) => {
                        tracing::info!("Password changed for account_id={}", account.id);
                        send_change_password_result(&sm, session_id, 0).await;
                    }
                    _ => {
                        send_change_password_result(&sm, session_id, 1).await;
                    }
                }
            });
        });
    }

    /// 注册创建角色包 handler (ClientOpcode::NewCharacter = 6)
    fn register_new_character(&self, router: &PacketRouter) {
        let pool = self.pool.clone();
        let sm = Arc::clone(&self.session_manager);

        router.register(ClientOpcode::NewCharacter as u16, move |session_id, payload| {
            let pool = pool.clone();
            let sm = sm.clone();
            let payload = payload.to_vec();

            tokio::spawn(async move {
                // 检查是否已认证
                let state = match sm.get_state(session_id).await {
                    Some(s) if s.authenticated => s,
                    _ => {
                        send_new_character_result(&sm, session_id, 1).await;
                        return;
                    }
                };

                if payload.len() < 4 {
                    send_new_character_result(&sm, session_id, 1).await;
                    return;
                }

                let name_len = u16::from_le_bytes([payload[0], payload[1]]) as usize;
                if payload.len() < 2 + name_len + 2 {
                    send_new_character_result(&sm, session_id, 1).await;
                    return;
                }

                let name = String::from_utf8_lossy(&payload[2..2 + name_len]).to_string();
                let offset = 2 + name_len;
                let class = payload[offset];
                let gender = payload[offset + 1];

                // 校验角色名长度 3~14
                if name.len() < 3 || name.len() > 14 {
                    send_new_character_result(&sm, session_id, 1).await;
                    return;
                }

                // 校验角色名只含中英文数字
                let valid_name = name.chars().all(|c| {
                    c.is_ascii_alphanumeric() || ('\u{4e00}'..='\u{9fa5}').contains(&c)
                });
                if !valid_name {
                    send_new_character_result(&sm, session_id, 1).await;
                    return;
                }

                // 检查角色数上限（最多 4 个）
                let char_repo = CharacterRepository::new(pool.clone());
                match char_repo.count_by_account(state.account_id).await {
                    Ok(count) if count >= 4 => {
                        send_new_character_result(&sm, session_id, 4).await;
                        return;
                    }
                    Err(e) => {
                        tracing::error!("Database error counting characters: {}", e);
                        send_new_character_result(&sm, session_id, 1).await;
                        return;
                    }
                    _ => {}
                }

                // 创建角色
                let input = NewCharacterInput {
                    account_id: state.account_id,
                    name: name.clone(),
                    class: class as i32,
                    gender: gender as i32,
                };

                match char_repo.create(&input).await {
                    Ok(char_id) => {
                        tracing::info!(
                            "New character created: name={}, class={}, account_id={}",
                            name, class, state.account_id
                        );

                        // 查询刚创建的角色以获取默认值
                        match char_repo.find_by_id(char_id).await {
                            Ok(Some(character)) => {
                                let ci = character.to_char_info(char_id as u32);
                                let packet = NewCharacterSuccessPacket::new(
                                    ci.name, ci.class, ci.gender, ci.level,
                                    ci.hp, ci.mp, ci.max_hp, ci.max_mp,
                                ).with_id(character.id as u32);
                                if let Ok(data) = packet.encode() {
                                    sm.send_to_session(session_id, &data).await;
                                }
                            }
                            _ => {
                                let packet = NewCharacterSuccessPacket::new(
                                    name, class, gender, 1, 100, 50, 100, 50,
                                ).with_id(char_id as u32);
                                if let Ok(data) = packet.encode() {
                                    sm.send_to_session(session_id, &data).await;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to create character {}: {}", name, e);
                        send_new_character_result(&sm, session_id, 5).await;
                    }
                }
            });
        });
    }

    /// 注册删除角色包 handler (ClientOpcode::DeleteCharacter = 7)
    fn register_delete_character(&self, router: &PacketRouter) {
        let pool = self.pool.clone();
        let sm = Arc::clone(&self.session_manager);

        router.register(ClientOpcode::DeleteCharacter as u16, move |session_id, payload| {
            let pool = pool.clone();
            let sm = sm.clone();
            let payload = payload.to_vec();

            tokio::spawn(async move {
                let state = match sm.get_state(session_id).await {
                    Some(s) if s.authenticated => s,
                    _ => {
                        send_delete_character_result(&sm, session_id, 1).await;
                        return;
                    }
                };

                if payload.len() < 4 {
                    send_delete_character_result(&sm, session_id, 1).await;
                    return;
                }

                let char_index = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]);

                let char_repo = CharacterRepository::new(pool.clone());
                match char_repo.soft_delete(char_index as i64, state.account_id).await {
                    Ok(true) => {
                        tracing::info!(
                            "Character {} deleted for account_id={}",
                            char_index, state.account_id
                        );
                        let packet = DeleteCharacterSuccessPacket::new(char_index);
                        if let Ok(data) = packet.encode() {
                            sm.send_to_session(session_id, &data).await;
                        }
                    }
                    Ok(false) => {
                        send_delete_character_result(&sm, session_id, 1).await;
                    }
                    Err(e) => {
                        tracing::error!("Database error deleting character: {}", e);
                        send_delete_character_result(&sm, session_id, 1).await;
                    }
                }
            });
        });
    }

    /// 注册开始游戏包 handler (ClientOpcode::StartGame = 8)
    fn register_start_game(&self, router: &PacketRouter) {
        let pool = self.pool.clone();
        let sm = Arc::clone(&self.session_manager);

        router.register(ClientOpcode::StartGame as u16, move |session_id, payload| {
            let pool = pool.clone();
            let sm = sm.clone();
            let payload = payload.to_vec();

            tokio::spawn(async move {
                let state = match sm.get_state(session_id).await {
                    Some(s) if s.authenticated => s,
                    _ => {
                        send_start_game_result(&sm, session_id, 1).await;
                        return;
                    }
                };

                if payload.len() < 4 {
                    send_start_game_result(&sm, session_id, 1).await;
                    return;
                }

                let char_index = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]);

                let char_repo = CharacterRepository::new(pool.clone());
                match char_repo.find_by_id(char_index as i64).await {
                    Ok(Some(character)) => {
                        if character.account_id != state.account_id {
                            send_start_game_result(&sm, session_id, 1).await;
                            return;
                        }

                        let mut new_state = state.clone();
                        new_state.character_id = Some(char_index as i64);
                        new_state.map_id = character.map_id as u16;
                        new_state.location = (character.location_x, character.location_y);
                        new_state.direction = match character.direction {
                            0 => mir2_shared::enums::MirDirection::Up,
                            1 => mir2_shared::enums::MirDirection::UpRight,
                            2 => mir2_shared::enums::MirDirection::Right,
                            3 => mir2_shared::enums::MirDirection::DownRight,
                            4 => mir2_shared::enums::MirDirection::Down,
                            5 => mir2_shared::enums::MirDirection::DownLeft,
                            6 => mir2_shared::enums::MirDirection::Left,
                            7 => mir2_shared::enums::MirDirection::UpLeft,
                            _ => mir2_shared::enums::MirDirection::Down,
                        };
                        new_state.character_name = character.name.clone();
                        new_state.level = character.level as u16;
                        new_state.current_hp = character.hp as u32;
                        new_state.current_mp = character.mp as u32;
                        new_state.max_hp = character.hp as u32;
                        new_state.max_mp = character.mp as u32;
                        new_state.experience = character.experience as u64;
                        new_state.gold = character.gold as u64;
                        sm.set_state(session_id, new_state).await;

                        tracing::info!(
                            "StartGame: account_id={}, character={} ({})",
                            state.account_id, character.id, character.name
                        );

                        send_start_game_result(&sm, session_id, 0).await;
                    }
                    Ok(None) => {
                        send_start_game_result(&sm, session_id, 1).await;
                    }
                    Err(e) => {
                        tracing::error!("Database error finding character: {}", e);
                        send_start_game_result(&sm, session_id, 1).await;
                    }
                }
            });
        });
    }
}

// =============================================
// 辅助函数
// =============================================

/// 使用 Argon2 对密码进行哈希
fn hash_password(password: &str) -> Option<String> {
    let salt = SaltString::generate(&mut OsRng);
    match Argon2::default().hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Some(hash.to_string()),
        Err(e) => {
            tracing::error!("Argon2 hash error: {}", e);
            None
        }
    }
}

// ---- 发送响应包辅助函数 ----

async fn send_login_result(sm: &SessionManager, session_id: u32, result: u8) {
    let packet = LoginResponsePacket::new(result);
    if let Ok(data) = packet.encode() {
        sm.send_to_session(session_id, &data).await;
    }
}

async fn send_login_banned(sm: &SessionManager, session_id: u32) {
    let packet = LoginBannedPacket;
    if let Ok(data) = packet.encode() {
        sm.send_to_session(session_id, &data).await;
    }
}

async fn send_new_account_result(sm: &SessionManager, session_id: u32, result: u8) {
    let packet = NewAccountResponsePacket::new(result);
    if let Ok(data) = packet.encode() {
        sm.send_to_session(session_id, &data).await;
    }
}

async fn send_change_password_result(sm: &SessionManager, session_id: u32, result: u8) {
    let packet = ChangePasswordResponsePacket::new(result);
    if let Ok(data) = packet.encode() {
        sm.send_to_session(session_id, &data).await;
    }
}

async fn send_change_password_banned(sm: &SessionManager, session_id: u32) {
    let packet = ChangePasswordBannedPacket;
    if let Ok(data) = packet.encode() {
        sm.send_to_session(session_id, &data).await;
    }
}

async fn send_new_character_result(sm: &SessionManager, session_id: u32, result: u8) {
    let packet = NewCharacterResponsePacket::new(result);
    if let Ok(data) = packet.encode() {
        sm.send_to_session(session_id, &data).await;
    }
}

async fn send_delete_character_result(sm: &SessionManager, session_id: u32, result: u8) {
    let packet = DeleteCharacterResponsePacket::new(result);
    if let Ok(data) = packet.encode() {
        sm.send_to_session(session_id, &data).await;
    }
}

async fn send_start_game_result(sm: &SessionManager, session_id: u32, result: u8) {
    let packet = StartGameResponsePacket::new(result);
    if let Ok(data) = packet.encode() {
        sm.send_to_session(session_id, &data).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use argon2::password_hash::SaltString;
    use argon2::password_hash::rand_core::OsRng;

    /// 辅助：生成测试用密码哈希
    fn create_test_hash(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string()
    }

    #[test]
    fn test_hash_password_produces_valid_hash() {
        // hash_password 应返回 Some(hash) 且 hash 非空
        let hash = hash_password("test_password123");
        assert!(hash.is_some(), "hash_password should return Some");
        let hash = hash.unwrap();
        assert!(!hash.is_empty(), "Hash should not be empty");
        // Argon2 hash string 应包含特定前缀
        assert!(hash.starts_with("$argon2"), "Hash should start with $argon2");
    }

    #[test]
    fn test_hash_password_empty_string() {
        // 空密码也应能哈希
        let hash = hash_password("");
        assert!(hash.is_some(), "Empty password should still hash");
    }

    #[test]
    fn test_hash_password_long_string() {
        // 长密码（64字符）也应能正常哈希
        let long_pwd = "a".repeat(64);
        let hash = hash_password(&long_pwd);
        assert!(hash.is_some(), "64-char password should hash");
    }

    #[test]
    fn test_argon2_verify_correct_password() {
        let hash = create_test_hash("my_password");
        let parsed = argon2::password_hash::PasswordHash::new(&hash).unwrap();
        let result = Argon2::default()
            .verify_password("my_password".as_bytes(), &parsed);
        assert!(result.is_ok(), "Correct password should verify");
    }

    #[test]
    fn test_argon2_verify_wrong_password() {
        let hash = create_test_hash("correct_password");
        let parsed = argon2::password_hash::PasswordHash::new(&hash).unwrap();
        let result = Argon2::default()
            .verify_password("wrong_password".as_bytes(), &parsed);
        assert!(result.is_err(), "Wrong password should not verify");
    }

    #[test]
    fn test_argon2_different_salts_produce_different_hashes() {
        // 相同密码两次哈希应不同（因为 salt 不同）
        let hash1 = create_test_hash("same_password");
        let hash2 = create_test_hash("same_password");
        assert_ne!(hash1, hash2, "Same password should produce different hashes due to salt");
    }

    #[test]
    fn test_hash_password_very_long() {
        // 超长密码（128字符）
        let very_long = "x".repeat(128);
        let hash = hash_password(&very_long);
        assert!(hash.is_some(), "128-char password should still hash");
    }

    #[test]
    fn test_argon2_verify_different_hash_formats() {
        // 验证 hash_password 产生的哈希能被正确解析和验证
        let pwd = "verify_me_123!";
        let hash = hash_password(pwd).unwrap();
        let parsed = argon2::password_hash::PasswordHash::new(&hash).unwrap();
        let result = Argon2::default()
            .verify_password(pwd.as_bytes(), &parsed);
        assert!(result.is_ok(), "hash_password output should be verifiable");
    }
}
