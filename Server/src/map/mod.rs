pub mod loader;

use std::collections::HashMap;

use mir2_shared::enums::CellAttribute;

/// 地图数据
#[derive(Debug, Clone)]
pub struct Map {
    pub id: u16,
    pub width: u16,
    pub height: u16,
    pub title: String,
    pub filename: String,
    pub tiles: Vec<Tile>,
    pub connections: Vec<MapConnection>,
    pub safe_zone: Option<Rect>,
    pub spawn_configs: MonsterSpawnConfigs,
}

impl Map {
    /// 获取瓦片索引 (x, y) → index
    pub fn tile_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }
        Some((y as usize) * (self.width as usize) + (x as usize))
    }

    /// 获取瓦片
    pub fn tile_at(&self, x: i32, y: i32) -> Option<&Tile> {
        self.tile_index(x, y).and_then(|i| self.tiles.get(i))
    }

    /// 是否为可行走格子
    pub fn is_walkable_at(&self, x: i32, y: i32) -> bool {
        self.tile_at(x, y)
            .map(|t| t.attr == CellAttribute::Walk)
            .unwrap_or(false)
    }

    /// 是否为安全区
    pub fn is_safe_zone_at(&self, x: i32, y: i32) -> bool {
        // 先检查 tile 级 safe_zone 标记
        if let Some(tile) = self.tile_at(x, y) {
            if tile.is_safe_zone {
                return true;
            }
        }
        // 再检查区域型 safe_zone
        if let Some(ref sz) = self.safe_zone {
            return sz.contains(x, y);
        }
        false
    }
}

/// 地图瓦片
#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub attr: CellAttribute,
    pub is_safe_zone: bool,
}

/// 地图连接（传送点）
#[derive(Debug, Clone)]
pub struct MapConnection {
    pub source_x: i32,
    pub source_y: i32,
    pub target_map_id: u16,
    pub target_x: i32,
    pub target_y: i32,
}

/// 矩形区域
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x && x < self.x + self.w && y >= self.y && y < self.y + self.h
    }
}

/// 怪物刷新配置（从地图 JSON 解析）
#[derive(Debug, Clone, Default)]
pub struct MonsterSpawnConfigs {
    pub monsters: Vec<MonsterSpawnConfig>,
}

/// 单个怪物的刷新配置
#[derive(Debug, Clone)]
pub struct MonsterSpawnConfig {
    pub monster_id: u16,
    pub count: u8,
    pub interval: u64, // ms
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub range: u8,
}

/// 地图管理器
#[derive(Clone)]
pub struct MapManager {
    maps: HashMap<u16, Map>,
}

impl MapManager {
    /// 创建空地图管理器
    pub fn new() -> Self {
        Self {
            maps: HashMap::new(),
        }
    }

    /// 从指定目录加载所有 JSON 地图
    pub async fn load_all(map_dir: &str) -> Result<Self, anyhow::Error> {
        let mut manager = MapManager::new();
        let dir = std::path::Path::new(map_dir);

        if !dir.exists() {
            tracing::warn!("Map directory does not exist: {}", map_dir);
            return Ok(manager);
        }

        let mut read_dir = tokio::fs::read_dir(dir).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                match loader::load_map_from_json(path.to_str().unwrap()) {
                    Ok(map) => {
                        let map_id = map.id;
                        tracing::info!("Loaded map {}: {} ({}x{})", map_id, map.title, map.width, map.height);
                        manager.maps.insert(map_id, map);
                    }
                    Err(e) => {
                        tracing::warn!("Failed to load map {:?}: {}", path, e);
                    }
                }
            }
        }

        tracing::info!("MapManager loaded {} maps", manager.maps.len());
        Ok(manager)
    }

    /// 获取地图
    pub fn get_map(&self, id: u16) -> Option<&Map> {
        self.maps.get(&id)
    }

    /// 检查地图上某格是否可行走
    pub fn is_walkable(&self, map_id: u16, x: i32, y: i32) -> bool {
        self.get_map(map_id)
            .map(|m| m.is_walkable_at(x, y))
            .unwrap_or(false)
    }

    /// 检查地图上某格是否为安全区
    pub fn is_safe_zone(&self, map_id: u16, x: i32, y: i32) -> bool {
        self.get_map(map_id)
            .map(|m| m.is_safe_zone_at(x, y))
            .unwrap_or(false)
    }

    /// 从 JSON tile id 获取 CellAttribute
    pub fn tile_id_to_attr(tile_id: u32) -> CellAttribute {
        match tile_id {
            0 | 1 => CellAttribute::Walk,          // 草地/道路
            2 => CellAttribute::HighWall,           // 墙壁
            3 => CellAttribute::LowWall,            // 水
            10 => CellAttribute::Walk,              // 安全区草地（safe_zone 由另一属性标记）
            _ if tile_id > 10 => CellAttribute::HighWall, // 其他墙壁
            _ => CellAttribute::HighWall,
        }
    }

    /// 判断 tile_id 是否为安全区
    pub fn tile_id_is_safe_zone(tile_id: u32) -> bool {
        tile_id == 10
    }

    /// 查找连接点（用于地图切换）
    pub fn find_connection(&self, map_id: u16, x: i32, y: i32) -> Option<MapConnection> {
        self.get_map(map_id).and_then(|map| {
            map.connections
                .iter()
                .find(|c| c.source_x == x && c.source_y == y)
                .cloned()
        })
    }

    /// 地图数量
    pub fn map_count(&self) -> usize {
        self.maps.len()
    }
}

impl Default for MapManager {
    fn default() -> Self {
        Self::new()
    }
}
