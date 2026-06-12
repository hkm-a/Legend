use std::sync::Arc;
use std::time::{Duration, Instant};

use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

use mir2_shared::net::error::NetError;
use mir2_shared::packets::PacketCodec;

use super::handler::PacketRouter;
use super::session_manager::SessionManager;
use crate::config::ServerConfig;

/// 心跳超时时长（秒）
const HEARTBEAT_TIMEOUT_SECS: u64 = 15;

/// 客户端会话 Actor
///
/// 每个客户端连接对应一个 ClientSession，在独立的 tokio task 中运行。
/// 使用 tokio::select! 在 ws read / mpsc rx / heartbeat tick 之间轮询。
pub struct ClientSession {
    session_id: u32,
    ws_stream: WebSocketStream<TcpStream>,
    #[allow(dead_code)]
    config: Arc<ServerConfig>,
    rx: mpsc::UnboundedReceiver<Message>,
    session_manager: Arc<SessionManager>,
    router: Arc<PacketRouter>,
}

impl ClientSession {
    /// 创建新的客户端会话
    pub fn new(
        session_id: u32,
        ws_stream: WebSocketStream<TcpStream>,
        config: Arc<ServerConfig>,
        rx: mpsc::UnboundedReceiver<Message>,
        session_manager: Arc<SessionManager>,
        router: Arc<PacketRouter>,
    ) -> Self {
        Self {
            session_id,
            ws_stream,
            config,
            rx,
            session_manager,
            router,
        }
    }

    /// 运行 Session 主循环
    ///
    /// 使用 tokio::select! 在三个分支间轮询：
    /// 1. ws read：收到二进制帧 → decode payload → PacketRouter dispatch
    /// 2. mpsc rx：收到数据 → ws write
    /// 3. heartbeat tick：检查是否超时
    pub async fn run(mut self) -> Result<(), NetError> {
        let (ws_write, mut ws_read) = self.ws_stream.split();
        let session_id = self.session_id;
        let session_manager = Arc::clone(&self.session_manager);
        let router = Arc::clone(&self.router);

        // 写任务：从 channel 接收消息并写入 WebSocket
        let (write_tx, mut write_rx) = mpsc::unbounded_channel::<Message>();
        let _write_handle = tokio::spawn(async move {
            use futures_util::SinkExt;
            let mut write_sink = ws_write;
            while let Some(msg) = write_rx.recv().await {
                if let Err(e) = write_sink.send(msg).await {
                    tracing::warn!("Session {} write error: {}", session_id, e);
                    break;
                }
            }
        });

        // 心跳超时检查
        let mut heartbeat = tokio::time::interval(Duration::from_secs(HEARTBEAT_TIMEOUT_SECS));
        heartbeat.tick().await; // 跳过第一次立即触发
        let mut last_activity = Instant::now();

        loop {
            tokio::select! {
                // 分支 1：从 WebSocket 读取数据
                msg = ws_read.next() => {
                    match msg {
                        Some(Ok(Message::Binary(data))) => {
                            // 解码并解析数据包
                            match PacketCodec::decode_payload(&data) {
                                Ok(payload) => {
                                    // 解码头部获取 packet_id
                                    let (packet_id, _) = PacketCodec::decode_header(&data)
                                        .unwrap_or((0, 0));

                                    tracing::debug!(
                                        "Session {} received packet_id={}, payload_len={}",
                                        session_id, packet_id, payload.len()
                                    );

                                    // 更新活跃时间
                                    last_activity = Instant::now();

                                    // 路由分发
                                    router.dispatch(session_id, packet_id, &payload);
                                }
                                Err(e) => {
                                    tracing::warn!(
                                        "Session {} failed to decode packet: {}",
                                        session_id, e
                                    );
                                }
                            }
                        }
                        Some(Ok(Message::Close(_))) => {
                            tracing::info!("Session {} received close frame", session_id);
                            break;
                        }
                        Some(Ok(_)) => {
                            // 忽略其他类型的消息（Text, Ping, Pong）
                        }
                        Some(Err(e)) => {
                            tracing::warn!("Session {} websocket error: {}", session_id, e);
                            break;
                        }
                        None => {
                            tracing::info!("Session {} read stream ended", session_id);
                            break;
                        }
                    }
                }

                // 分支 2：从内部 channel 接收要发送的数据
                msg = self.rx.recv() => {
                    match msg {
                        Some(msg) => {
                            if let Err(e) = write_tx.send(msg) {
                                tracing::warn!(
                                    "Session {} send to write task failed: {}",
                                    session_id, e
                                );
                                break;
                            }
                        }
                        None => {
                            tracing::info!("Session {} rx channel closed", session_id);
                            break;
                        }
                    }
                }

                // 分支 3：心跳超时检测
                _ = heartbeat.tick() => {
                    let elapsed = last_activity.elapsed();
                    if elapsed > Duration::from_secs(HEARTBEAT_TIMEOUT_SECS) {
                        tracing::warn!(
                            "Session {} heartbeat timeout after {:.0}s, disconnecting",
                            session_id,
                            elapsed.as_secs()
                        );
                        break;
                    }
                }
            }
        }

        // 清理：从 SessionManager 中移除
        tracing::info!("Session {} cleaning up", session_id);
        session_manager.remove(session_id).await;

        Ok(())
    }
}
