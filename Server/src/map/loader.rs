use std::collections::HashMap;
use std::fs;

use mir2_shared::enums::CellAttribute;

use super::{Map, MapConnection, MonsterSpawnConfig, MonsterSpawnConfigs, Rect, Tile};

/// 从 Tiled 兼容 JSON 文件加载地图
///
/// JSON 格式：
/// { "width": 30, "height": 30, "tilewidth": 32, "tileheight": 32,
///   "layers": [{"name": "ground", "type": "tilelayer", "width": 30, "height": 30, "data": [...]}],
///   "properties": [{"name": "map_id", "type": "int", "value": 0}, ...] }
pub fn load_map_from_json(path: &str) -> Result<Map, anyhow::Error> {
    let content = fs::read_to_string(path)?;
    let raw: RawMapJson = serde_json::from_str(&content)?;

    let width = raw.width;
    let height = raw.height;

    // 解析 properties
    let props = parse_properties(&raw.properties);

    let map_id = props
        .get("map_id")
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(0);

    let title = props
        .get("title")
        .cloned()
        .unwrap_or_else(|| format!("Map {}", map_id));

    let filename = props
        .get("filename")
        .cloned()
        .unwrap_or_else(|| {
            std::path::Path::new(path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string()
        });

    // 解析 connections
    let connections: Vec<MapConnection> = if let Some(conn_str) = props.get("connections") {
        parse_connections(conn_str)
    } else {
        Vec::new()
    };

    // 解析 safe_zone
    let safe_zone: Option<Rect> = props.get("safe_zone").and_then(|s| parse_safe_zone(s));

    // 解析 spawns
    let spawn_configs = if let Some(spawn_str) = props.get("spawns") {
        parse_spawns(spawn_str)
    } else {
        MonsterSpawnConfigs::default()
    };

    // 解析瓦片数据（从每个 tilelayer 合并）
    // 支持多个 tilelayer：用 "ground" 层的数据 + "safezone" 层标记安全区
    let mut tiles: Vec<Tile> = Vec::with_capacity((width * height) as usize);
    // 初始化所有瓦片为 Walk
    for _ in 0..(width * height) {
        tiles.push(Tile {
            attr: CellAttribute::Walk,
            is_safe_zone: false,
        });
    }

    for layer in &raw.layers {
        if layer.layer_type != "tilelayer" {
            continue;
        }
        if let Some(ref data) = layer.data {
            // 检查是否为安全区层
            let is_safety_layer = layer.name.to_lowercase().contains("safety")
                || layer.name.to_lowercase().contains("safe");

            for (i, &tile_id) in data.iter().enumerate() {
                if i >= tiles.len() {
                    break;
                }
                if is_safety_layer {
                    // 安全区层：非 0 表示安全区
                    if tile_id != 0 {
                        tiles[i].is_safe_zone = true;
                        tiles[i].attr = CellAttribute::Walk;
                    }
                } else {
                    // 普通层：只覆盖非 0 的 tile
                    if tile_id != 0 {
                        let attr = crate::map::MapManager::tile_id_to_attr(tile_id);
                        let is_sz = crate::map::MapManager::tile_id_is_safe_zone(tile_id);
                        tiles[i] = Tile {
                            attr,
                            is_safe_zone: is_sz,
                        };
                    }
                }
            }
        }
    }

    Ok(Map {
        id: map_id,
        width,
        height,
        title,
        filename,
        tiles,
        connections,
        safe_zone,
        spawn_configs,
    })
}

// ============================================================
// JSON 数据结构
// ============================================================

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct RawMapJson {
    width: u16,
    height: u16,
    #[serde(default)]
    tilewidth: u32,
    #[serde(default)]
    tileheight: u32,
    #[serde(default)]
    layers: Vec<RawLayer>,
    #[serde(default)]
    properties: Vec<RawProperty>,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct RawLayer {
    name: String,
    #[serde(rename = "type")]
    layer_type: String,
    width: u16,
    height: u16,
    data: Option<Vec<u32>>,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct RawProperty {
    name: String,
    #[serde(rename = "type")]
    prop_type: String,
    value: serde_json::Value,
}

// ============================================================
// 解析辅助
// ============================================================

fn parse_properties(props: &[RawProperty]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for prop in props {
        let value_str = match &prop.value {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            _ => prop.value.to_string(),
        };
        map.insert(prop.name.clone(), value_str);
    }
    map
}

/// 解析 connections 属性
/// JSON 格式: [{"sx":28,"sy":15,"tm":1,"tx":3,"ty":25}]
fn parse_connections(json_str: &str) -> Vec<MapConnection> {
    #[derive(serde::Deserialize)]
    struct RawConnection {
        sx: i32,
        sy: i32,
        tm: u16,
        tx: i32,
        ty: i32,
    }

    match serde_json::from_str::<Vec<RawConnection>>(json_str) {
        Ok(raw_list) => raw_list
            .into_iter()
            .map(|r| MapConnection {
                source_x: r.sx,
                source_y: r.sy,
                target_map_id: r.tm,
                target_x: r.tx,
                target_y: r.ty,
            })
            .collect(),
        Err(e) => {
            tracing::warn!("Failed to parse connections: {}", e);
            Vec::new()
        }
    }
}

/// 解析 safe_zone 属性
/// JSON 格式: {"x":2,"y":2,"w":10,"h":10}
fn parse_safe_zone(json_str: &str) -> Option<Rect> {
    #[derive(serde::Deserialize)]
    struct RawRect {
        x: i32,
        y: i32,
        w: i32,
        h: i32,
    }

    match serde_json::from_str::<RawRect>(json_str) {
        Ok(r) => Some(Rect {
            x: r.x,
            y: r.y,
            w: r.w,
            h: r.h,
        }),
        Err(e) => {
            tracing::warn!("Failed to parse safe_zone: {}", e);
            None
        }
    }
}

/// 解析 spawns 属性
/// JSON 格式: {"monsters":[{"monster_id":1,"count":3,"interval":10000,"x":0,"y":0,"w":10,"h":10,"range":5}]}
fn parse_spawns(json_str: &str) -> MonsterSpawnConfigs {
    #[derive(serde::Deserialize)]
    struct RawSpawns {
        monsters: Vec<RawSpawnConfig>,
    }

    #[derive(serde::Deserialize)]
    struct RawSpawnConfig {
        monster_id: u16,
        #[serde(default)]
        count: u8,
        #[serde(default = "default_interval")]
        interval: u64,
        #[serde(default)]
        x: i32,
        #[serde(default)]
        y: i32,
        #[serde(default)]
        w: i32,
        #[serde(default)]
        h: i32,
        #[serde(default)]
        range: u8,
    }

    fn default_interval() -> u64 {
        10000
    }

    match serde_json::from_str::<RawSpawns>(json_str) {
        Ok(raw) => MonsterSpawnConfigs {
            monsters: raw
                .monsters
                .into_iter()
                .map(|r| MonsterSpawnConfig {
                    monster_id: r.monster_id,
                    count: r.count,
                    interval: r.interval,
                    x: r.x,
                    y: r.y,
                    w: r.w,
                    h: r.h,
                    range: r.range,
                })
                .collect(),
        },
        Err(e) => {
            tracing::warn!("Failed to parse spawns: {}", e);
            MonsterSpawnConfigs::default()
        }
    }
}
