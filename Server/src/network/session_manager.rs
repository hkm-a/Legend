use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Instant;

use mir2_shared::enums::MirDirection;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, RwLock};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

use super::handler::PacketRouter;
use super::session::ClientSession;
use crate::config::ServerConfig;
use crate::equipment::EquipmentManager;
use crate::skill::SkillManager;

/// 会话状态：记录当前会话的完整游戏状态
#[derive(Debug, Clone)]
pub struct SessionState {
    /// 登录成功的账号 ID
    pub account_id: i64,
    /// 是否已验证通过
    pub authenticated: bool,
    /// 已选角色 ID（进入游戏后设置）
    pub character_id: Option<i64>,
    /// 当前所在地图 ID
    pub map_id: u16,
    /// 当前坐标
    pub location: (i32, i32),
    /// 朝向
    pub direction: MirDirection,
    /// 角色名
    pub character_name: String,
    /// 等级
    pub level: u16,
    /// 当前生命值
    pub current_hp: u32,
    /// 当前魔法值
    pub current_mp: u32,
    /// 最大生命值
    pub max_hp: u32,
    /// 最大魔法值
    pub max_mp: u32,
    /// 经验值
    pub experience: u64,
    /// 金币
    pub gold: u64,
    /// 上次攻击时间（用于攻击冷却）
    pub last_attack_time: Instant,
    /// 装备管理器
    pub equipment: EquipmentManager,
    /// 技能管理器
    pub skills: SkillManager,
}

impl SessionState {
    pub fn new(account_id: i64) -> Self {
        Self {
            account_id,
            authenticated: true,
            character_id: None,
            map_id: 0,
            location: (50, 50),
            direction: MirDirection::Down,
            character_name: String::new(),
            level: 1,
            current_hp: 100,
            current_mp: 50,
            max_hp: 100,
            max_mp: 50,
            experience: 0,
            gold: 0,
            last_attack_time: Instant::now(),
            equipment: EquipmentManager::new(),
            skills: SkillManager::new(),
        }
    }
}

/// 会话管理器全局注册表
///
/// 管理所有活跃的客户端连接 Session 及其状态。
/// 使用 RwLock<HashMap> 保证并发安全，AtomicU32 分配递增 ID。
pub struct SessionManager {
    sessions: RwLock<HashMap<u32, mpsc::UnboundedSender<Message>>>,
    states: RwLock<HashMap<u32, SessionState>>,
    next_id: AtomicU32,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            states: RwLock::new(HashMap::new()),
            next_id: AtomicU32::new(1), // ID 从 1 开始，0 保留
        }
    }

    /// 添加一个新的会话
    ///
    /// 分配 session_id，spawn 一个 tokio task 运行 ClientSession，
    /// 将其发送端存入注册表。返回分配的 session_id。
    pub async fn add(
        self: &Arc<Self>,
        stream: WebSocketStream<TcpStream>,
        config: Arc<ServerConfig>,
        router: Arc<PacketRouter>,
    ) -> u32 {
        let session_id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let (tx, rx) = mpsc::unbounded_channel::<Message>();

        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(session_id, tx.clone());
        }

        let session = ClientSession::new(
            session_id,
            stream,
            config,
            rx,
            Arc::clone(self),
            router,
        );
        let manager = Arc::clone(self);

        tokio::spawn(async move {
            tracing::info!("Session {} started", session_id);
            if let Err(e) = session.run().await {
                tracing::warn!("Session {} error: {}", session_id, e);
            }
            tracing::info!("Session {} ended", session_id);
            manager.remove(session_id).await;
        });

        session_id
    }

    /// 移除一个会话及其状态
    pub async fn remove(&self, session_id: u32) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(&session_id);

        let mut states = self.states.write().await;
        states.remove(&session_id);

        tracing::debug!(
            "Session {} removed, active: {}, states: {}",
            session_id,
            sessions.len(),
            states.len()
        );
    }

    /// 向所有活跃会话广播数据
    pub async fn broadcast(&self, data: &[u8]) {
        let sessions = self.sessions.read().await;
        for (sid, sender) in sessions.iter() {
            if let Err(e) = sender.send(Message::Binary(data.to_vec().into())) {
                tracing::warn!("Failed to send to session {}: {}", sid, e);
            }
        }
    }

    /// 向指定会话发送数据
    pub async fn send_to_session(&self, session_id: u32, data: &[u8]) {
        let sessions = self.sessions.read().await;
        if let Some(sender) = sessions.get(&session_id) {
            if let Err(e) = sender.send(Message::Binary(data.to_vec().into())) {
                tracing::warn!("Failed to send to session {}: {}", session_id, e);
            }
        } else {
            tracing::debug!("Session {} not found for send_to_session", session_id);
        }
    }

    /// 向某地图上的所有玩家广播（排除指定 session）
    pub async fn broadcast_to_map(
        &self,
        map_id: u16,
        exclude_session: Option<u32>,
        data: &[u8],
    ) {
        let states = self.states.read().await;
        let sessions = self.sessions.read().await;

        for (&sid, sender) in sessions.iter() {
            if let Some(exclude) = exclude_session {
                if sid == exclude {
                    continue;
                }
            }

            // 检查该会话是否在目标地图上
            let on_map = states.get(&sid).map_or(false, |s| s.map_id == map_id);
            if !on_map {
                continue;
            }

            if let Err(e) = sender.send(Message::Binary(data.to_vec().into())) {
                tracing::warn!("Failed to broadcast to session {}: {}", sid, e);
            }
        }
    }

    /// 向某地图上的所有玩家广播（包含所有）
    pub async fn broadcast_to_map_all(&self, map_id: u16, data: &[u8]) {
        self.broadcast_to_map(map_id, None, data).await;
    }

    /// 获取指定地图上所有会话的状态（用于广播等）
    pub async fn get_states_on_map(&self, map_id: u16) -> Vec<(u32, SessionState)> {
        let states = self.states.read().await;
        states
            .iter()
            .filter(|(_, s)| s.map_id == map_id)
            .map(|(&sid, s)| (sid, s.clone()))
            .collect()
    }

    /// 获取当前会话数量
    pub async fn session_count(&self) -> usize {
        let sessions = self.sessions.read().await;
        sessions.len()
    }

    /// 根据 session_id 获取发送端
    pub async fn get_sender(&self, session_id: u32) -> Option<mpsc::UnboundedSender<Message>> {
        let sessions = self.sessions.read().await;
        sessions.get(&session_id).cloned()
    }

    // =============================================
    // SessionState 相关方法
    // =============================================

    /// 获取会话状态
    pub async fn get_state(&self, session_id: u32) -> Option<SessionState> {
        let states = self.states.read().await;
        states.get(&session_id).cloned()
    }

    /// 设置会话状态
    pub async fn set_state(&self, session_id: u32, state: SessionState) {
        let mut states = self.states.write().await;
        states.insert(session_id, state);
    }

    /// 更新会话状态的某个字段（通过闭包）
    pub async fn update_state<F>(&self, session_id: u32, updater: F)
    where
        F: FnOnce(&mut SessionState),
    {
        let mut states = self.states.write().await;
        if let Some(state) = states.get_mut(&session_id) {
            updater(state);
        }
    }

    /// 获取所有会话状态
    pub async fn get_all_states(&self) -> Vec<(u32, SessionState)> {
        let states = self.states.read().await;
        states
            .iter()
            .map(|(&sid, s)| (sid, s.clone()))
            .collect()
    }

    /// 移除会话状态
    pub async fn remove_state(&self, session_id: u32) {
        let mut states = self.states.write().await;
        states.remove(&session_id);
    }
}
