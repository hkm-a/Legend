//! NPC 管理器
//!
//! 管理所有 NPC 模板数据，提供按地图、ID 查找的能力。
//! 数据从 `data/npcs.json` 加载。

pub mod shop;

use std::collections::HashMap;

use serde::Deserialize;

/// NPC 模板（从 JSON 加载）
#[derive(Debug, Clone, Deserialize)]
pub struct NpcTemplate {
    pub id: u16,
    pub name: String,
    pub map_id: u16,
    pub x: i32,
    pub y: i32,
    pub image: u16,
    /// 商店类型（空字符串表示非商店 NPC）
    #[serde(default)]
    pub shop_type: String,
    /// 出售物品 ID 列表
    #[serde(default)]
    pub selling_items: Vec<u16>,
}

/// NPC 管理器
pub struct NpcManager {
    pub templates: HashMap<u16, NpcTemplate>,
    /// 按地图 ID 索引的 NPC ID 列表
    pub npcs_by_map: HashMap<u16, Vec<u16>>,
}

impl NpcManager {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
            npcs_by_map: HashMap::new(),
        }
    }

    /// 从 JSON 文件加载 NPC 模板
    pub fn load_templates(&mut self, path: &str) -> Result<(), anyhow::Error> {
        let content = std::fs::read_to_string(path)?;
        let npcs: Vec<NpcTemplate> = serde_json::from_str(&content)?;
        for npc in npcs {
            let id = npc.id;
            self.templates.insert(id, npc.clone());
            self.npcs_by_map.entry(npc.map_id).or_default().push(id);
        }
        tracing::info!("NpcManager loaded {} NPCs", self.templates.len());
        Ok(())
    }

    /// 根据 ID 获取 NPC 模板
    pub fn get_npc(&self, id: u16) -> Option<&NpcTemplate> {
        self.templates.get(&id)
    }

    /// 获取某地图上的所有 NPC
    pub fn get_npcs_on_map(&self, map_id: u16) -> Vec<&NpcTemplate> {
        self.npcs_by_map
            .get(&map_id)
            .map(|ids| ids.iter().filter_map(|id| self.templates.get(id)).collect())
            .unwrap_or_default()
    }

    /// 查找附近 NPC（曼哈顿距离）
    pub fn find_nearby_npc(
        &self,
        map_id: u16,
        x: i32,
        y: i32,
        range: i32,
    ) -> Option<(u16, &NpcTemplate)> {
        for (&id, npc) in &self.templates {
            if npc.map_id == map_id {
                let dist = (npc.x - x).abs() + (npc.y - y).abs();
                if dist <= range {
                    return Some((id, npc));
                }
            }
        }
        None
    }
}

impl Default for NpcManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 验证 NPC JSON 反序列化
    #[test]
    fn test_npc_template_deserialize() {
        let json = r#"{
            "id": 1,
            "name": "药店老板",
            "map_id": 0,
            "x": 14,
            "y": 14,
            "image": 10,
            "shop_type": "BuyAndSell",
            "selling_items": [0, 1, 6, 7]
        }"#;

        let npc: NpcTemplate = serde_json::from_str(json).unwrap();
        assert_eq!(npc.id, 1);
        assert_eq!(npc.name, "药店老板");
        assert_eq!(npc.map_id, 0);
        assert_eq!(npc.x, 14);
        assert_eq!(npc.y, 14);
        assert_eq!(npc.shop_type, "BuyAndSell");
        assert_eq!(npc.selling_items, vec![0, 1, 6, 7]);
    }

    /// 验证无商店的 NPC 默认值
    #[test]
    fn test_npc_template_default_fields() {
        let json = r#"{
            "id": 5,
            "name": "路人甲",
            "map_id": 0,
            "x": 10,
            "y": 10,
            "image": 99
        }"#;

        let npc: NpcTemplate = serde_json::from_str(json).unwrap();
        assert_eq!(npc.id, 5);
        assert!(npc.shop_type.is_empty(), "无 shop_type 时应为空字符串");
        assert!(npc.selling_items.is_empty(), "无 selling_items 时应为空");
    }

    /// 验证 npcs.json 文件存在且完整解析
    #[test]
    fn test_load_npcs_json_file() {
        let mut manager = NpcManager::new();
        manager
            .load_templates("data/npcs.json")
            .expect("npcs.json 应能加载");
        assert_eq!(manager.templates.len(), 4, "应有 4 个 NPC");
        assert!(manager.npcs_by_map.contains_key(&0), "NPC 应在地图 0");
    }

    /// 验证 get_npc 方法
    #[test]
    fn test_get_npc() {
        let mut manager = NpcManager::new();
        manager.load_templates("data/npcs.json").unwrap();

        let npc = manager.get_npc(1).expect("NPC 1 应存在");
        assert_eq!(npc.name, "药店老板");

        let none = manager.get_npc(99);
        assert!(none.is_none(), "不存在的 NPC 应返回 None");
    }

    /// 验证 get_npcs_on_map 方法
    #[test]
    fn test_get_npcs_on_map() {
        let mut manager = NpcManager::new();
        manager.load_templates("data/npcs.json").unwrap();

        let npcs = manager.get_npcs_on_map(0);
        assert_eq!(npcs.len(), 4, "地图 0 应有 4 个 NPC");

        let empty = manager.get_npcs_on_map(999);
        assert!(empty.is_empty(), "不存在的地图应返回空列表");
    }

    /// 验证查找附近 NPC
    #[test]
    fn test_find_nearby_npc() {
        let mut manager = NpcManager::new();
        manager.load_templates("data/npcs.json").unwrap();

        // 在药店老板附近
        let result = manager.find_nearby_npc(0, 14, 14, 1);
        assert!(result.is_some(), "应在 (14,14) 找到 NPC");
        assert_eq!(result.unwrap().0, 1); // 药店老板 id=1

        // 超出范围
        let far = manager.find_nearby_npc(0, 100, 100, 1);
        assert!(far.is_none(), "远处不应找到 NPC");
    }
}
