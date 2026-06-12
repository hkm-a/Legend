use std::sync::Arc;

use mir2_server::config;
use mir2_server::database;
use mir2_server::game::WorldState;
use mir2_server::item::ItemManager;
use mir2_server::map::MapManager;
use mir2_server::monster::MonsterManager;
use mir2_server::network;
use mir2_server::npc::NpcManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 初始化 tracing 日志
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // 2. 加载配置
    let config = config::ServerConfig::from_file("Config.toml").unwrap_or_else(|e| {
        tracing::warn!("Failed to load config: {e}, using defaults");
        config::ServerConfig::default()
    });

    tracing::info!("Mir2 Server starting...");
    tracing::info!("Network: {}:{}", config.network.host, config.network.port);
    tracing::info!("Database: {}", config.database.path);
    tracing::info!("Max players: {}", config.game.max_players);
    tracing::info!("Tick rate: {}ms", config.game.tick_rate_ms);

    // 3. 初始化数据库
    let pool = database::init_db(&config.database.path).await?;
    tracing::info!("Database initialized successfully");

    // 4. 初始化游戏世界
    tracing::info!("Loading game world...");

    // 加载地图
    let map_manager = MapManager::load_all(&config.game.map_dir).await?;
    let map_manager = Arc::new(map_manager);

    // 加载怪物模板
    let monster_templates =
        MonsterManager::load_templates(&config.game.monster_data_path).await?;
    let mut monster_manager = MonsterManager::new(Arc::clone(&map_manager));
    monster_manager.init_templates(monster_templates);
    let monster_manager = monster_manager;

    // 加载物品模板
    let mut item_manager = ItemManager::new();
    if let Err(e) = item_manager.load_templates(&config.game.item_data_path) {
        tracing::warn!("Failed to load item data: {e}");
    }

    // 加载掉落表
    if let Err(e) = item_manager.load_drops(&config.game.drops_table_path) {
        tracing::warn!("Failed to load drops table: {e}");
    }
    let item_manager = item_manager;

    // 加载 NPC 模板
    let mut npc_manager = NpcManager::new();
    if let Err(e) = npc_manager.load_templates(&config.game.npc_data_path) {
        tracing::warn!("Failed to load NPC data: {e}");
    }

    // 5. 初始化 WebSocket 服务器（首先创建以获取 session_manager）
    let mut ws_server = network::server::WebSocketServer::new(config.clone(), pool);
    let session_manager = ws_server.session_manager();

    let world_state = Arc::new(WorldState::new(
        (*map_manager).clone(),
        monster_manager,
        item_manager,
        npc_manager,
        session_manager,
        config.game.tick_rate_ms,
    ));

    ws_server.set_world_state(Arc::clone(&world_state));

    tracing::info!("Game world initialized");

    // 5. 启动全局 tick 循环
    let ws_tick = Arc::clone(&world_state);
    let tick_handle = tokio::spawn(async move {
        ws_tick.start_tick_loop().await;
    });

    // 6. 启动 WebSocket 服务器
    let shutdown_sender = ws_server.shutdown_sender();

    let server_handle = tokio::spawn(async move {
        if let Err(e) = ws_server.start().await {
            tracing::error!("WebSocket server error: {}", e);
        }
    });

    // 7. 等待信号（Ctrl+C）
    tokio::signal::ctrl_c().await?;
    tracing::info!("Shutdown signal received, stopping server...");

    // 发送关闭信号让 accept 循环退出
    let _ = shutdown_sender.send(true);

    // 等待服务器优雅关闭
    let _ = tokio::time::timeout(
        tokio::time::Duration::from_secs(5),
        server_handle,
    )
    .await;

    // 停止 tick 循环
    tick_handle.abort();

    tracing::info!("Server shutdown complete.");

    Ok(())
}
