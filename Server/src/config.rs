use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// 服务端配置，从 TOML 文件加载
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub network: NetworkConfig,
    pub database: DatabaseConfig,
    pub game: GameConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub max_players: u32,
    pub tick_rate_ms: u64,
    pub map_dir: String,
    pub monster_data_path: String,
    pub item_data_path: String,
    pub npc_data_path: String,
    pub skill_data_path: String,
    pub drops_table_path: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            network: NetworkConfig {
                host: "0.0.0.0".to_string(),
                port: 7000,
            },
            database: DatabaseConfig {
                path: "./data/mir2.db".to_string(),
            },
            game: GameConfig {
                max_players: 500,
                tick_rate_ms: 50,
                map_dir: "data/maps".to_string(),
                monster_data_path: "data/monsters.json".to_string(),
                item_data_path: "data/items.json".to_string(),
                npc_data_path: "data/npcs.json".to_string(),
                skill_data_path: "data/skills.json".to_string(),
                drops_table_path: "data/drops.json".to_string(),
            },
        }
    }
}

impl ServerConfig {
    /// 从文件加载配置，若文件不存在则创建默认配置
    pub fn from_file(path: &str) -> Result<Self, anyhow::Error> {
        if !Path::new(path).exists() {
            // 文件不存在，创建默认配置并写入
            let default_config = ServerConfig::default();
            let toml_str = toml::to_string_pretty(&default_config)?;
            if let Some(parent) = Path::new(path).parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(path, &toml_str)?;
            tracing::info!("Created default config file: {}", path);
            return Ok(default_config);
        }

        let content = fs::read_to_string(path)?;
        let config: ServerConfig = toml::from_str(&content)?;
        Ok(config)
    }
}
