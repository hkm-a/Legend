use std::sync::Arc;

use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tokio::sync::watch;
use tokio_tungstenite::accept_async;

use super::handler::{GameLogicHandler, PacketRouter};
use super::session_manager::SessionManager;
use crate::auth::AuthHandler;
use crate::config::ServerConfig;
use crate::game::WorldState;

/// WebSocket 服务器
///
/// 监听端口，接受连接，创建 Session。
/// 管理路由初始化、GameLogicHandler/AuthHandler 注册和优雅关闭。
pub struct WebSocketServer {
    config: ServerConfig,
    pool: SqlitePool,
    session_manager: Arc<SessionManager>,
    world_state: Option<Arc<WorldState>>,
    shutdown_tx: watch::Sender<bool>,
    shutdown_rx: watch::Receiver<bool>,
}

impl WebSocketServer {
    pub fn new(config: ServerConfig, pool: SqlitePool) -> Self {
        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        Self {
            config,
            pool,
            session_manager: Arc::new(SessionManager::new()),
            world_state: None,
            shutdown_tx,
            shutdown_rx,
        }
    }

    /// 设置世界状态（在 WorldState 初始化后调用）
    pub fn set_world_state(&mut self, world_state: Arc<WorldState>) {
        self.world_state = Some(world_state);
    }

    /// 获取关闭信号发送端，用于外部触发优雅关闭
    pub fn shutdown_sender(&self) -> watch::Sender<bool> {
        self.shutdown_tx.clone()
    }

    /// 获取会话管理器引用
    pub fn session_manager(&self) -> Arc<SessionManager> {
        self.session_manager.clone()
    }

    /// 启动服务器监听循环
    ///
    /// 1. 初始化 PacketRouter 并注册所有 GameLogicHandler + AuthHandler
    /// 2. 绑定 TCP 端口
    /// 3. 循环 accept 连接，每个连接 spawn 独立 task
    /// 4. 收到关闭信号时优雅退出
    pub async fn start(&mut self) -> Result<(), anyhow::Error> {
        // 1. 初始化路由系统
        let router = Arc::new(PacketRouter::new());

        // 优先注册 AuthHandler（先注册先处理，但 dispatch 是按 packet_id 独立查找的）
        let auth_handler = AuthHandler::new(self.pool.clone(), Arc::clone(&self.session_manager));
        auth_handler.register_all(&router);

        // 初始化 GameLogicHandler 并注册
        let mut handler = GameLogicHandler::new(Arc::clone(&self.session_manager));
        if let Some(ref ws) = self.world_state {
            handler.set_world_state(Arc::clone(ws));
        }
        handler.register_all(&router);

        // 2. 绑定地址
        let addr = format!("{}:{}", self.config.network.host, self.config.network.port);
        let listener = TcpListener::bind(&addr).await?;

        tracing::info!("WebSocket server listening on {}", addr);
        tracing::info!(
            "Session manager ready, registered handlers: {}",
            router.handler_count()
        );

        // 3. Accept 循环
        loop {
            tokio::select! {
                result = listener.accept() => {
                    match result {
                        Ok((stream, peer_addr)) => {
                            tracing::info!("New connection from {}", peer_addr);

                            let config = Arc::new(self.config.clone());
                            let session_manager = Arc::clone(&self.session_manager);
                            let router = Arc::clone(&router);

                            tokio::spawn(async move {
                                match accept_async(stream).await {
                                    Ok(ws_stream) => {
                                        tracing::info!("WebSocket handshake complete for {}", peer_addr);
                                        let sid = session_manager.add(ws_stream, config, router).await;
                                        tracing::debug!("Assigned session_id={} for {}", sid, peer_addr);
                                    }
                                    Err(e) => {
                                        tracing::warn!(
                                            "WebSocket handshake failed for {}: {}",
                                            peer_addr, e
                                        );
                                    }
                                }
                            });
                        }
                        Err(e) => {
                            tracing::error!("Failed to accept connection: {}", e);
                        }
                    }
                }

                // 分支 2：接收关闭信号
                _ = self.shutdown_rx.changed() => {
                    if *self.shutdown_rx.borrow() {
                        tracing::info!("Shutdown signal received, stopping accept loop");
                        break;
                    }
                }
            }
        }

        tracing::info!(
            "Accept loop ended, active sessions: {}",
            self.session_manager.session_count().await
        );
        Ok(())
    }
}
