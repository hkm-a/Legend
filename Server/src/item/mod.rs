pub mod drops;
pub mod inventory;

use std::collections::HashMap;

use mir2_shared::enums::ItemType as ItemTypeEnum;
use serde::Deserialize;

/// 物品模板（从 JSON 加载）
#[derive(Debug, Clone, Deserialize)]
pub struct ItemTemplate {
    pub id: u16,
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: ItemTypeEnum,
    pub level: u16,
    #[serde(default)]
    pub hp_restore: u32,
    #[serde(default)]
    pub mp_restore: u32,
    pub weight: u32,
    pub stack_size: u32,
    pub price: u32,
    pub image: u16,
    #[serde(default)]
    pub required_class: i32,
    #[serde(default)]
    pub required_level: u16,
    #[serde(default)]
    pub description: String,
    /// 物理攻击力 (min, max)
    #[serde(default)]
    pub dc_min: i32,
    #[serde(default)]
    pub dc_max: i32,
    /// 防御
    #[serde(default)]
    pub ac: i32,
    /// 魔法防御
    #[serde(default)]
    pub mac: i32,
    /// 准确
    #[serde(default)]
    pub accuracy: u32,
    /// 敏捷
    #[serde(default)]
    pub agility: u32,
    /// 耐久度
    #[serde(default)]
    pub durability: i32,
    /// 魔法攻击力 (min, max)
    #[serde(default)]
    pub mc_min: i32,
    #[serde(default)]
    pub mc_max: i32,
    /// 道术攻击力 (min, max)
    #[serde(default)]
    pub sc_min: i32,
    #[serde(default)]
    pub sc_max: i32,
}

/// 地面掉落物品
#[derive(Debug, Clone)]
pub struct GroundItem {
    pub object_id: u32,
    pub item_id: u16,
    pub item_name: String,
    pub map_id: u16,
    pub x: i32,
    pub y: i32,
    pub count: u32,
    pub drop_time: std::time::Instant,
}

impl GroundItem {
    pub fn new(object_id: u32, item_id: u16, item_name: String, map_id: u16, x: i32, y: i32) -> Self {
        Self {
            object_id,
            item_id,
            item_name,
            map_id,
            x,
            y,
            count: 1,
            drop_time: std::time::Instant::now(),
        }
    }

    pub fn distance_to(&self, x: i32, y: i32) -> i32 {
        (self.x - x).abs() + (self.y - y).abs()
    }
}

/// 物品掉落规则
#[derive(Debug, Clone, Deserialize)]
pub struct DropRule {
    pub monster_id: u16,
    pub item_id: u16,
    /// 掉落概率 (0.0 ~ 1.0)
    pub probability: f64,
    /// 掉落数量
    #[serde(default = "default_count")]
    pub count: u32,
}

fn default_count() -> u32 {
    1
}

/// 物品管理器
pub struct ItemManager {
    pub templates: HashMap<u16, ItemTemplate>,
    pub drops: Vec<DropRule>,
    /// 地面物品
    pub ground_items: HashMap<u32, GroundItem>,
    next_ground_id: u32,
}

impl ItemManager {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
            drops: Vec::new(),
            ground_items: HashMap::new(),
            next_ground_id: 20000,
        }
    }

    /// 从 JSON 加载物品模板
    pub fn load_templates(&mut self, path: &str) -> Result<(), anyhow::Error> {
        let content = std::fs::read_to_string(path)?;
        let templates: Vec<ItemTemplate> = serde_json::from_str(&content)?;
        for t in templates {
            self.templates.insert(t.id, t);
        }
        tracing::info!("ItemManager loaded {} item templates", self.templates.len());
        Ok(())
    }

    /// 从 JSON 加载掉落规则
    pub fn load_drops(&mut self, path: &str) -> Result<(), anyhow::Error> {
        let content = std::fs::read_to_string(path)?;
        let drops: Vec<DropRule> = serde_json::from_str(&content)?;
        self.drops = drops;
        tracing::info!("ItemManager loaded {} drop rules", self.drops.len());
        Ok(())
    }

    /// 获取物品模板
    pub fn get_template(&self, id: u16) -> Option<&ItemTemplate> {
        self.templates.get(&id)
    }

    /// 在地面上放置物品
    pub fn drop_item_on_ground(
        &mut self,
        item_id: u16,
        map_id: u16,
        x: i32,
        y: i32,
    ) -> Option<u32> {
        let template = self.templates.get(&item_id)?;
        let object_id = self.next_ground_id;
        self.next_ground_id += 1;

        let ground = GroundItem::new(object_id, item_id, template.name.clone(), map_id, x, y);
        self.ground_items.insert(object_id, ground);
        Some(object_id)
    }

    /// 获取地面物品
    pub fn get_ground_item(&self, object_id: u32) -> Option<&GroundItem> {
        self.ground_items.get(&object_id)
    }

    /// 移除地面物品
    pub fn remove_ground_item(&mut self, object_id: u32) -> Option<GroundItem> {
        self.ground_items.remove(&object_id)
    }

    /// 获取某地图上某位置附近的物品
    pub fn get_ground_items_near(&self, map_id: u16, x: i32, y: i32, range: i32) -> Vec<u32> {
        self.ground_items
            .iter()
            .filter(|(_, item)| item.map_id == map_id && item.distance_to(x, y) <= range)
            .map(|(&id, _)| id)
            .collect()
    }

    /// 获取怪物对应的掉落物
    pub fn get_drops_for_monster(&self, monster_id: u16) -> Vec<(u16, u32)> {
        self.drops
            .iter()
            .filter(|d| d.monster_id == monster_id)
            .filter(|d| fastrand::f64() < d.probability)
            .map(|d| (d.item_id, d.count))
            .collect()
    }

    /// 清理过期的地面物品（超过 60 秒）
    pub fn clean_expired_ground_items(&mut self) {
        let now = std::time::Instant::now();
        let expired: Vec<u32> = self
            .ground_items
            .iter()
            .filter(|(_, item)| now.duration_since(item.drop_time).as_secs() > 60)
            .map(|(&id, _)| id)
            .collect();
        for id in expired {
            self.ground_items.remove(&id);
        }
    }
}

impl Default for ItemManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================
    // ItemTemplate 基础序列化/反序列化测试
    // ============================================

    /// 验证已有的 JSON 反序列化正常工作（基础字段）
    #[test]
    fn test_item_template_basic_deserialize() {
        let json = r#"{
            "id": 1,
            "name": "木剑",
            "type": "Weapon",
            "level": 1,
            "weight": 10,
            "stack_size": 1,
            "price": 100,
            "image": 1001,
            "dc_min": 2,
            "dc_max": 5,
            "ac": 0,
            "mac": 0,
            "accuracy": 5,
            "agility": 0
        }"#;

        let template: ItemTemplate = serde_json::from_str(json).unwrap();
        assert_eq!(template.id, 1);
        assert_eq!(template.name, "木剑");
        assert_eq!(template.item_type, ItemTypeEnum::Weapon);
        assert_eq!(template.level, 1);
        assert_eq!(template.weight, 10);
        assert_eq!(template.dc_min, 2);
        assert_eq!(template.dc_max, 5);
        assert_eq!(template.ac, 0);
        assert_eq!(template.accuracy, 5);
    }

    // ============================================
    // ItemTemplate 扩展字段测试（durability, mc, sc）
    // ============================================

    /// 加载 JSON 中不包含扩展字段时，应默认为 0
    #[test]
    fn test_item_template_extended_fields_default_to_zero() {
        let json = r#"{
            "id": 10,
            "name": "铁剑",
            "type": "Weapon",
            "level": 5,
            "weight": 30,
            "stack_size": 1,
            "price": 500,
            "image": 1010,
            "dc_min": 5,
            "dc_max": 12,
            "ac": 1,
            "mac": 0,
            "accuracy": 3,
            "agility": 0
        }"#;

        let template: ItemTemplate = serde_json::from_str(json).unwrap();
        // 验证扩展字段默认为 0
        assert_eq!(template.durability, 0, "durability 应默认为 0");
        assert_eq!(template.mc_min, 0, "mc_min 应默认为 0");
        assert_eq!(template.mc_max, 0, "mc_max 应默认为 0");
        assert_eq!(template.sc_min, 0, "sc_min 应默认为 0");
        assert_eq!(template.sc_max, 0, "sc_max 应默认为 0");
        // 已有字段不受影响
        assert_eq!(template.dc_min, 5);
        assert_eq!(template.dc_max, 12);
    }

    /// 使用带扩展字段值的 JSON 字符串反序列化，验证所有字段正确读取
    #[test]
    fn test_item_template_with_extended_values() {
        let json = r#"{
            "id": 20,
            "name": "魔杖",
            "type": "Weapon",
            "level": 10,
            "weight": 15,
            "stack_size": 1,
            "price": 2000,
            "image": 1020,
            "dc_min": 2,
            "dc_max": 4,
            "mc_min": 5,
            "mc_max": 10,
            "sc_min": 0,
            "sc_max": 0,
            "ac": 0,
            "mac": 1,
            "accuracy": 3,
            "agility": 0,
            "durability": 30,
            "required_class": 1,
            "required_level": 10,
            "description": "一把魔法杖"
        }"#;

        let template: ItemTemplate = serde_json::from_str(json).unwrap();
        // 验证扩展字段正确读取
        assert_eq!(template.durability, 30, "durability 应读取为 30");
        assert_eq!(template.mc_min, 5, "mc_min 应读取为 5");
        assert_eq!(template.mc_max, 10, "mc_max 应读取为 10");
        assert_eq!(template.sc_min, 0, "sc_min 应读取为 0");
        assert_eq!(template.sc_max, 0, "sc_max 应读取为 0");
        // 已有字段仍正确
        assert_eq!(template.dc_min, 2);
        assert_eq!(template.dc_max, 4);
        assert_eq!(template.required_class, 1);
        assert_eq!(template.required_level, 10);
        assert_eq!(template.description, "一把魔法杖");
    }

    /// 验证耐久度字段的合理范围（扩展字段测试的补充）
    #[test]
    fn test_item_template_durability_zero_on_consumable() {
        let json = r#"{
            "id": 30,
            "name": "金创药(小量)",
            "type": "Potion",
            "level": 1,
            "weight": 1,
            "stack_size": 10,
            "price": 50,
            "image": 1030,
            "hp_restore": 50,
            "mp_restore": 0,
            "dc_min": 0,
            "dc_max": 0,
            "ac": 0,
            "mac": 0,
            "accuracy": 0,
            "agility": 0
        }"#;

        let template: ItemTemplate = serde_json::from_str(json).unwrap();
        // 药品类物品耐久度为 0
        assert_eq!(template.durability, 0, "消耗品 durability 应为 0");
        assert_eq!(template.mc_min, 0);
        assert_eq!(template.mc_max, 0);
        assert_eq!(template.hp_restore, 50);
    }

    // ============================================
    // 数据文件完整性测试 — 验证 items.json 实际文件
    // ============================================

    /// 验证 items.json 文件存在且完整解析（所有物品）
    #[test]
    fn test_load_items_json_file() {
        let path = "data/items.json";
        let content = std::fs::read_to_string(path)
            .expect("items.json 文件应存在");
        let items: Vec<ItemTemplate> = serde_json::from_str(&content)
            .expect("items.json 应能被反序列化为 Vec<ItemTemplate>");
        // 应有 8 个物品 (0-7)
        assert_eq!(items.len(), 8, "items.json 应有 8 个物品");

        // 验证 item 2 (木剑) 使用 dc_min/dc_max 扁平字段格式
        let item2 = items.iter().find(|i| i.id == 2).expect("物品 2 (木剑) 应存在");
        // BUG: 实际 items.json 中 item 2 使用 "dc":{"min":2,"max":5} 嵌套格式
        // 但 ItemTemplate 期望扁平 dc_min/dc_max — serde 会静默忽略 "dc" 字段
        // 所以木剑的 dc_min 将为 0 而不是 2
        assert_eq!(
            item2.dc_min, 2,
            "木剑 dc_min 应为 2 — 若为 0 说明 'dc' 嵌套格式未被扁平字段识别"
        );
        assert_eq!(item2.dc_max, 5, "木剑 dc_max 应为 5");

        // 验证 item 6 (金创药(中)) 存在且拥有 level 字段
        let item6 = items.iter().find(|i| i.id == 6);
        assert!(
            item6.is_some(),
            "物品 6 (金创药(中)) 应存在 — 若解析失败说明缺少 level 字段"
        );
        if let Some(i6) = item6 {
            assert_eq!(i6.name, "金创药(中)");
            // BUG: items.json 中 item 6 使用 health_recovery 而非 hp_restore
            // 所以 hp_restore 将为 0 而非 80
            assert_eq!(
                i6.hp_restore, 80,
                "金创药(中) hp_restore 应为 80 — 若为 0 说明 health_recovery 字段名不匹配"
            );
        }

        // 验证 item 7 (魔法药(中))
        let item7 = items.iter().find(|i| i.id == 7);
        assert!(
            item7.is_some(),
            "物品 7 (魔法药(中)) 应存在 — 若解析失败说明缺少 level 字段"
        );
        if let Some(i7) = item7 {
            assert_eq!(i7.name, "魔法药(中)");
            assert_eq!(
                i7.mp_restore, 80,
                "魔法药(中) mp_restore 应为 80 — 若为 0 说明 mana_recovery 字段名不匹配"
            );
        }
    }

    /// 验证 items.json 所有物品均可正确反序列化
    #[test]
    fn test_load_items_json_has_correct_types() {
        let path = "data/items.json";
        let content = std::fs::read_to_string(path).unwrap();
        let items: Vec<ItemTemplate> = serde_json::from_str(&content).unwrap();
        // 验证药品类消耗品耐久度为 0
        for id in [0usize, 1, 6, 7] {
            let item = items.iter().find(|i| i.id as usize == id)
                .unwrap_or_else(|| panic!("item {} 应存在", id));
            assert_eq!(item.durability, 0, "药水物品 durability 应为 0");
        }
        // 验证装备类物品有耐久度
        for id in [2usize, 3, 5] {
            let item = items.iter().find(|i| i.id as usize == id)
                .unwrap_or_else(|| panic!("item {} 应存在", id));
            assert!(item.durability > 0, "装备物品 durability 应 > 0");
        }
    }
}
