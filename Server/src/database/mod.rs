pub mod models;
pub mod repository;

use sqlx::sqlite::SqlitePool;

/// 初始化数据库：确保目录存在、连接数据库、运行迁移
pub async fn init_db(db_path: &str) -> Result<SqlitePool, anyhow::Error> {
    // 确保父目录存在
    if let Some(parent) = std::path::Path::new(db_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    let pool = SqlitePool::connect(&format!("sqlite:{db_path}?mode=rwc")).await?;
    run_migrations(&pool).await?;

    Ok(pool)
}

/// 运行数据库迁移（建表）
async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS accounts (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            username    TEXT NOT NULL UNIQUE,
            password    TEXT NOT NULL,
            email       TEXT DEFAULT '',
            banned      INTEGER NOT NULL DEFAULT 0,
            ban_reason  TEXT DEFAULT '',
            created_at  TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS characters (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            account_id   INTEGER NOT NULL,
            name         TEXT NOT NULL UNIQUE,
            class        INTEGER NOT NULL DEFAULT 0,
            gender       INTEGER NOT NULL DEFAULT 0,
            level        INTEGER NOT NULL DEFAULT 1,
            experience   INTEGER NOT NULL DEFAULT 0,
            map_id       INTEGER NOT NULL DEFAULT 0,
            location_x   INTEGER NOT NULL DEFAULT 50,
            location_y   INTEGER NOT NULL DEFAULT 50,
            direction    INTEGER NOT NULL DEFAULT 0,
            hp           INTEGER NOT NULL DEFAULT 100,
            mp           INTEGER NOT NULL DEFAULT 50,
            gold         INTEGER NOT NULL DEFAULT 0,
            deleted      INTEGER NOT NULL DEFAULT 0,
            created_at   TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at   TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (account_id) REFERENCES accounts(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_characters_account_id ON characters(account_id)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_characters_name ON characters(name)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS user_items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            character_id INTEGER NOT NULL,
            item_id INTEGER NOT NULL,
            slot INTEGER NOT NULL,
            count INTEGER NOT NULL DEFAULT 1,
            durability INTEGER NOT NULL DEFAULT 0,
            max_durability INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (character_id) REFERENCES characters(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS character_storage (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            character_id INTEGER NOT NULL UNIQUE,
            gold INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (character_id) REFERENCES characters(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS user_skills (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            character_id INTEGER NOT NULL,
            spell_id INTEGER NOT NULL,
            level INTEGER NOT NULL DEFAULT 1,
            proficiency INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (character_id) REFERENCES characters(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    tracing::info!("Database migrations completed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::models::NewCharacterInput;
    use crate::database::repository::{AccountRepository, CharacterRepository};

    /// 测试用数据库路径
    fn test_db_path(name: &str) -> String {
        format!("./data/test_{}.db", name)
    }

    /// 初始化测试数据库，返回 (pool, path)
    async fn setup_test_db(name: &str) -> (SqlitePool, String) {
        let path = test_db_path(name);
        // 清理旧文件
        let _ = std::fs::remove_file(&path);
        let pool = init_db(&path).await.unwrap();
        (pool, path)
    }

    /// 清理测试数据库文件
    async fn teardown(pool: SqlitePool, path: &str) {
        pool.close().await;
        let _ = std::fs::remove_file(path);
    }

    // ============================================
    // init_db 测试
    // ============================================

    #[tokio::test]
    async fn test_init_db_creates_file() {
        let (pool, path) = setup_test_db("init_file").await;

        // 验证表存在
        let row: (i32,) = sqlx::query_as("SELECT COUNT(*) FROM accounts")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(row.0, 0);

        let row: (i32,) = sqlx::query_as("SELECT COUNT(*) FROM characters")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(row.0, 0);

        // 验证文件确实被创建
        assert!(std::path::Path::new(&path).exists(), "DB file should exist");

        teardown(pool, &path).await;
    }

    #[tokio::test]
    async fn test_init_db_idempotent() {
        // 多次调用 init_db 不应报错（CREATE IF NOT EXISTS）
        let path = test_db_path("init_idempotent");
        let _ = std::fs::remove_file(&path);

        let pool1 = init_db(&path).await.unwrap();
        pool1.close().await;

        let pool2 = init_db(&path).await.unwrap();
        // 再次运行迁移不应报错
        let row: (i32,) = sqlx::query_as("SELECT COUNT(*) FROM accounts")
            .fetch_one(&pool2).await.unwrap();
        assert_eq!(row.0, 0);

        teardown(pool2, &path).await;
    }

    // ============================================
    // AccountRepository 测试
    // ============================================

    #[tokio::test]
    async fn test_account_create_and_find() {
        let (pool, path) = setup_test_db("account_cf").await;
        let repo = AccountRepository::new(pool.clone());

        let id = repo.create("testuser", "$argon2hash_v1").await.unwrap();
        assert!(id > 0, "Account ID should be positive");

        let found = repo.find_by_username("testuser").await.unwrap();
        assert!(found.is_some());
        let account = found.unwrap();
        assert_eq!(account.username, "testuser");
        assert_eq!(account.password, "$argon2hash_v1");
        assert_eq!(account.banned, 0);
        assert_eq!(account.id, id);

        let not_found = repo.find_by_username("nobody").await.unwrap();
        assert!(not_found.is_none());

        teardown(pool, &path).await;
    }

    #[tokio::test]
    async fn test_account_duplicate_username() {
        let (pool, path) = setup_test_db("account_dup").await;
        let repo = AccountRepository::new(pool.clone());

        repo.create("unique_user", "$hash1").await.unwrap();
        let result = repo.create("unique_user", "$hash2").await;
        assert!(result.is_err(), "Duplicate username should fail");

        teardown(pool, &path).await;
    }

    #[tokio::test]
    async fn test_account_update_password() {
        let (pool, path) = setup_test_db("account_up").await;
        let repo = AccountRepository::new(pool.clone());

        let id = repo.create("pwduser", "$old_hash").await.unwrap();
        let updated = repo.update_password(id, "$new_hash").await.unwrap();
        assert!(updated, "Password should be updated");

        let account = repo.find_by_username("pwduser").await.unwrap().unwrap();
        assert_eq!(account.password, "$new_hash");

        // 更新不存在的 id 应返回 false
        let no_update = repo.update_password(99999, "$hash").await.unwrap();
        assert!(!no_update, "Updating non-existent account should return false");

        teardown(pool, &path).await;
    }

    // ============================================
    // CharacterRepository 测试
    // ============================================

    #[tokio::test]
    async fn test_character_create_and_find_by_account() {
        let (pool, path) = setup_test_db("char_cf").await;
        let acc_repo = AccountRepository::new(pool.clone());
        let char_repo = CharacterRepository::new(pool.clone());

        // 先创建账号
        let account_id = acc_repo.create("charowner", "$hash").await.unwrap();

        // 创建角色
        let input = NewCharacterInput {
            account_id,
            name: "HeroOne".to_string(),
            class: 0,
            gender: 0,
        };
        let char_id = char_repo.create(&input).await.unwrap();
        assert!(char_id > 0);

        // 按账号查询
        let chars = char_repo.find_by_account(account_id).await.unwrap();
        assert_eq!(chars.len(), 1);
        assert_eq!(chars[0].name, "HeroOne");
        assert_eq!(chars[0].class, 0);
        assert_eq!(chars[0].level, 1); // default value
        assert_eq!(chars[0].hp, 100);  // default value

        // 其它账号不应查到
        let other_chars = char_repo.find_by_account(99999).await.unwrap();
        assert!(other_chars.is_empty());

        teardown(pool, &path).await;
    }

    #[tokio::test]
    async fn test_character_count_by_account() {
        let (pool, path) = setup_test_db("char_count").await;
        let acc_repo = AccountRepository::new(pool.clone());
        let char_repo = CharacterRepository::new(pool.clone());

        let account_id = acc_repo.create("countowner", "$hash").await.unwrap();

        // 初始计数应为0
        let count = char_repo.count_by_account(account_id).await.unwrap();
        assert_eq!(count, 0);

        // 创建3个角色
        for i in 0..3 {
            let input = NewCharacterInput {
                account_id,
                name: format!("Char{}", i),
                class: 0,
                gender: 0,
            };
            char_repo.create(&input).await.unwrap();
        }

        let count = char_repo.count_by_account(account_id).await.unwrap();
        assert_eq!(count, 3);

        teardown(pool, &path).await;
    }

    #[tokio::test]
    async fn test_character_soft_delete() {
        let (pool, path) = setup_test_db("char_del").await;
        let acc_repo = AccountRepository::new(pool.clone());
        let char_repo = CharacterRepository::new(pool.clone());

        let account_id = acc_repo.create("delowner", "$hash").await.unwrap();

        let input = NewCharacterInput {
            account_id,
            name: "DeleteMe".to_string(),
            class: 1,
            gender: 0,
        };
        let char_id = char_repo.create(&input).await.unwrap();

        // 软删除
        let deleted = char_repo.soft_delete(char_id, account_id).await.unwrap();
        assert!(deleted, "Soft delete should return true");

        // 按账号查询应不再包含该角色
        let chars = char_repo.find_by_account(account_id).await.unwrap();
        assert!(chars.is_empty(), "Deleted character should not be found");

        // count 也应排除已删除的角色
        let count = char_repo.count_by_account(account_id).await.unwrap();
        assert_eq!(count, 0);

        // find_by_id 也应返回 None
        let found = char_repo.find_by_id(char_id).await.unwrap();
        assert!(found.is_none(), "Deleted character should not be found by id");

        teardown(pool, &path).await;
    }

    #[tokio::test]
    async fn test_soft_delete_wrong_account() {
        let (pool, path) = setup_test_db("char_wdel").await;
        let acc_repo = AccountRepository::new(pool.clone());
        let char_repo = CharacterRepository::new(pool.clone());

        let account_id = acc_repo.create("owner1", "$hash").await.unwrap();
        let other_id = acc_repo.create("owner2", "$hash").await.unwrap();

        let input = NewCharacterInput {
            account_id,
            name: "MyChar".to_string(),
            class: 0,
            gender: 0,
        };
        let char_id = char_repo.create(&input).await.unwrap();

        // 使用错误的 account_id 不应删除
        let deleted = char_repo.soft_delete(char_id, other_id).await.unwrap();
        assert!(!deleted, "Wrong account should not be able to delete character");

        // 角色应该仍然存在
        let chars = char_repo.find_by_account(account_id).await.unwrap();
        assert_eq!(chars.len(), 1);

        teardown(pool, &path).await;
    }

    #[tokio::test]
    async fn test_character_limit_max_4() {
        let (pool, path) = setup_test_db("char_limit").await;
        let acc_repo = AccountRepository::new(pool.clone());
        let char_repo = CharacterRepository::new(pool.clone());

        let account_id = acc_repo.create("limitowner", "$hash").await.unwrap();

        // 创建4个角色（上限）
        for i in 0..4 {
            let input = NewCharacterInput {
                account_id,
                name: format!("Hero{}", i),
                class: i % 3,
                gender: i % 2,
            };
            let char_id = char_repo.create(&input).await.unwrap();
            assert!(char_id > 0);
        }

        let count = char_repo.count_by_account(account_id).await.unwrap();
        assert_eq!(count, 4, "Should have exactly 4 characters");

        // 第5个角色在 repo 层面可以创建（repo 不限制，由 auth handler 限制）
        let input5 = NewCharacterInput {
            account_id,
            name: "HeroExtra".to_string(),
            class: 0,
            gender: 0,
        };
        let char_id5 = char_repo.create(&input5).await.unwrap();
        assert!(char_id5 > 0, "Repo should allow creating more than 4");

        // 但 count 应返回 5
        let count = char_repo.count_by_account(account_id).await.unwrap();
        assert_eq!(count, 5, "Count should include all non-deleted chars");

        teardown(pool, &path).await;
    }

    #[tokio::test]
    async fn test_character_unique_name() {
        let (pool, path) = setup_test_db("char_uname").await;
        let acc_repo = AccountRepository::new(pool.clone());
        let char_repo = CharacterRepository::new(pool.clone());

        let account_id = acc_repo.create("unameowner", "$hash").await.unwrap();

        let input = NewCharacterInput {
            account_id,
            name: "UniqueName".to_string(),
            class: 0,
            gender: 0,
        };
        char_repo.create(&input).await.unwrap();

        // 同名角色应失败
        let dup = NewCharacterInput {
            account_id,
            name: "UniqueName".to_string(),
            class: 1,
            gender: 1,
        };
        let result = char_repo.create(&dup).await;
        assert!(result.is_err(), "Duplicate character name should fail");

        teardown(pool, &path).await;
    }

    // ============================================
    // to_char_info 测试
    // ============================================

    #[tokio::test]
    async fn test_to_char_info_conversion() {
        let (pool, path) = setup_test_db("char_info").await;
        let acc_repo = AccountRepository::new(pool.clone());
        let char_repo = CharacterRepository::new(pool.clone());

        let account_id = acc_repo.create("infotest", "$hash").await.unwrap();
        let input = NewCharacterInput {
            account_id,
            name: "InfoChar".to_string(),
            class: 2,
            gender: 1,
        };
        let char_id = char_repo.create(&input).await.unwrap();

        let character = char_repo.find_by_id(char_id).await.unwrap().unwrap();
        let info = character.to_char_info(char_id as u32);

        assert_eq!(info.index, char_id as u32);
        assert_eq!(info.name, "InfoChar");
        assert_eq!(info.class, 2);
        assert_eq!(info.gender, 1);
        assert_eq!(info.level, 1);
        assert_eq!(info.hp, 100);
        assert_eq!(info.mp, 50);
        assert_eq!(info.max_hp, 100);
        assert_eq!(info.max_mp, 50);

        teardown(pool, &path).await;
    }
}
