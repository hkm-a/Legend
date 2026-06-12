//! 装备系统
//!
//! 管理玩家的装备穿戴/卸下、属性计算、耐久度损耗。
//! 与背包系统 (`InventoryManager`) 配合使用。

use std::collections::HashMap;

use mir2_shared::enums::ItemType;

use crate::item::ItemTemplate;
use crate::item::inventory::InventoryManager;
use crate::item::ItemManager;

/// 玩家属性集合
///
/// 包含所有战斗相关属性，由基础等级属性和装备属性叠加而成。
#[derive(Debug, Clone, Default)]
pub struct PlayerStats {
    pub dc_min: i32,
    pub dc_max: i32,
    pub mc_min: i32,
    pub mc_max: i32,
    pub sc_min: i32,
    pub sc_max: i32,
    pub ac: i32,
    pub mac: i32,
    pub accuracy: u32,
    pub agility: u32,
}

impl PlayerStats {
    /// 基础属性（等级加成）
    ///
    /// 每级提供微量的基础属性增长：
    /// - AC: +level
    /// - MAC: +level/2
    /// - DC/MC/SC: +level (各职业通过装备侧重)
    /// - accuracy: +1/3级
    /// - agility: +1/5级
    pub fn from_level(level: u16) -> Self {
        let l = level as i32;
        Self {
            dc_min: l,
            dc_max: l + 1,
            mc_min: l,
            mc_max: l + 1,
            sc_min: l,
            sc_max: l + 1,
            ac: l,
            mac: l / 2,
            accuracy: (level / 3) as u32,
            agility: (level / 5) as u32,
        }
    }

    /// 叠加一件装备的属性
    ///
    /// 将所有战斗属性累加到当前属性上。
    pub fn add_equipment(&mut self, template: &ItemTemplate) {
        self.dc_min += template.dc_min;
        self.dc_max += template.dc_max;
        self.mc_min += template.mc_min;
        self.mc_max += template.mc_max;
        self.sc_min += template.sc_min;
        self.sc_max += template.sc_max;
        self.ac += template.ac;
        self.mac += template.mac;
        self.accuracy = self.accuracy.saturating_add(template.accuracy);
        self.agility = self.agility.saturating_add(template.agility);
    }

    /// 计算最终属性
    ///
    /// 1. 从等级获得基础属性
    /// 2. 叠加所有装备的属性
    pub fn calculate(level: u16, templates: &[&ItemTemplate]) -> Self {
        let mut stats = Self::from_level(level);
        for t in templates {
            stats.add_equipment(t);
        }
        stats
    }
}

/// 已装备的物品信息
#[derive(Debug, Clone)]
struct EquippedItem {
    pub template_id: u16,
    pub durability: i32,
}

/// 装备管理器
///
/// 管理当前穿戴在装备位上的物品，`slot_index` 与 `EquipmentSlot` 枚举值对应。
#[derive(Debug, Clone)]
pub struct EquipmentManager {
    slots: HashMap<u8, EquippedItem>,
}

impl EquipmentManager {
    pub fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    /// 将 `ItemType` 映射到 `EquipmentSlot` 的索引值
    fn item_type_to_slot(item_type: ItemType) -> Option<u8> {
        Some(match item_type {
            ItemType::Weapon => 0,
            ItemType::Armour => 1,
            ItemType::Helmet => 2,
            ItemType::Torch => 3,
            ItemType::Necklace => 4,
            ItemType::Bracelet => 5, // 默认左腕
            ItemType::Ring => 7,     // 默认左戒
            ItemType::Amulet => 9,
            ItemType::Belt => 10,
            ItemType::Boots => 11,
            ItemType::Stone => 12,
            ItemType::Mount => 13,
            _ => return None,
        })
    }

    /// 穿戴装备：从背包指定槽位移除物品 → 放入对应装备位
    ///
    /// 如果目标装备位已有物品，先自动卸下（放回背包空格）。
    pub fn equip(
        &mut self,
        item_template: &ItemTemplate,
        inventory: &mut InventoryManager,
        inv_slot: usize,
    ) -> Result<(), String> {
        // 确定装备位
        let slot_index = Self::item_type_to_slot(item_template.item_type).ok_or_else(|| {
            format!(
                "Item type {:?} cannot be equipped",
                item_template.item_type
            )
        })?;

        // 先读取背包槽位信息，释放借用
        let (item_id, item_durability) = {
            let inv_item = inventory
                .get_slot(inv_slot)
                .ok_or_else(|| format!("Inventory slot {} is empty", inv_slot))?;
            (inv_item.item_id, inv_item.durability)
        };

        if item_id != item_template.id {
            return Err("Item ID mismatch between template and inventory slot".to_string());
        }

        // 如果装备位已有物品，先卸下（会借用 inventory 可变引用，因此 item_id 等已复制）
        if self.slots.contains_key(&slot_index) {
            self.unequip(slot_index, inventory)?;
        }

        let durability = if item_template.durability > 0 {
            item_template.durability
        } else {
            item_durability
        };

        // 从背包移除物品
        if !inventory.remove_item(inv_slot, 1) {
            return Err("Failed to remove item from inventory".to_string());
        }

        // 插入装备位
        self.slots.insert(
            slot_index,
            EquippedItem {
                template_id: item_template.id,
                durability,
            },
        );

        Ok(())
    }

    /// 卸下装备：从装备位移除 → 放回背包空格
    ///
    /// 返回被卸下物品的 template_id。
    pub fn unequip(
        &mut self,
        slot_index: u8,
        inventory: &mut InventoryManager,
    ) -> Result<u16, String> {
        let template_id = self
            .slots
            .remove(&slot_index)
            .ok_or_else(|| format!("Slot {} is not equipped", slot_index))?
            .template_id;

        // 找背包空格，如果背包满则放回装备位
        let free_slot = inventory.add_item(template_id, 1);
        if free_slot.is_none() {
            // 背包满，恢复装备位
            self.slots.insert(
                slot_index,
                EquippedItem {
                    template_id,
                    durability: 0,
                },
            );
            return Err("Inventory is full, cannot unequip".to_string());
        }

        Ok(template_id)
    }

    /// 计算总属性
    ///
    /// 从等级 + 所有已装备物品计算最终属性。
    pub fn calculate_stats(&self, level: u16, item_manager: &ItemManager) -> PlayerStats {
        let templates: Vec<&ItemTemplate> = self
            .slots
            .values()
            .filter_map(|eq| item_manager.get_template(eq.template_id))
            .collect();

        PlayerStats::calculate(level, &templates)
    }

    /// 扣除已装备物品的耐久度（每次攻击/受击时调用）
    ///
    /// 返回 `Vec<(slot_index, template_id)>`，供调用方向客户端发送 `DuraChanged` 包。
    pub fn apply_durability_damage(&mut self) -> Vec<(u8, u16)> {
        let mut result = Vec::new();
        let damaged_slots: Vec<u8> = self
            .slots
            .iter()
            .filter(|(_, eq)| eq.durability > 0)
            .map(|(&slot, _)| slot)
            .collect();

        for slot in damaged_slots {
            if let Some(eq) = self.slots.get_mut(&slot) {
                eq.durability -= 1;
                result.push((slot, eq.template_id));
            }
        }

        result
    }

    /// 获取指定装备位的 template_id
    pub fn get_equipped_template_id(&self, slot_index: u8) -> Option<u16> {
        self.slots.get(&slot_index).map(|eq| eq.template_id)
    }

    /// 当前装备数量
    pub fn equipped_count(&self) -> usize {
        self.slots.len()
    }

    /// 检查指定槽位是否有装备
    pub fn is_slot_equipped(&self, slot_index: u8) -> bool {
        self.slots.contains_key(&slot_index)
    }

    /// 获取所有已装备的 (slot_index, template_id)
    pub fn all_equipped(&self) -> Vec<(u8, u16)> {
        self.slots
            .iter()
            .map(|(&slot, eq)| (slot, eq.template_id))
            .collect()
    }
}

impl Default for EquipmentManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mir2_shared::enums::ItemType as ItemTypeEnum;

    // ============================================
    // PlayerStats 测试
    // ============================================

    /// 验证 from_level 随着等级增长而增长
    #[test]
    fn test_player_stats_from_level() {
        let stats_l1 = PlayerStats::from_level(1);
        assert_eq!(stats_l1.dc_min, 1);
        assert_eq!(stats_l1.dc_max, 2);
        assert_eq!(stats_l1.ac, 1);
        assert_eq!(stats_l1.mac, 0); // 1/2 = 0
        assert_eq!(stats_l1.accuracy, 0); // 1/3 = 0
        assert_eq!(stats_l1.agility, 0); // 1/5 = 0

        let stats_l10 = PlayerStats::from_level(10);
        assert_eq!(stats_l10.dc_min, 10);
        assert_eq!(stats_l10.dc_max, 11);
        assert_eq!(stats_l10.ac, 10);
        assert_eq!(stats_l10.mac, 5);
        assert_eq!(stats_l10.accuracy, 3);
        assert_eq!(stats_l10.agility, 2);
    }

    /// 验证 add_equipment 正确叠加属性
    #[test]
    fn test_player_stats_add_equipment() {
        let mut stats = PlayerStats::from_level(1);

        let sword = ItemTemplate {
            id: 1,
            name: "木剑".to_string(),
            item_type: ItemTypeEnum::Weapon,
            level: 1,
            hp_restore: 0,
            mp_restore: 0,
            weight: 10,
            stack_size: 1,
            price: 100,
            image: 1001,
            required_class: 0,
            required_level: 0,
            description: String::new(),
            dc_min: 2,
            dc_max: 5,
            ac: 0,
            mac: 0,
            accuracy: 5,
            agility: 0,
            durability: 10,
            mc_min: 0,
            mc_max: 0,
            sc_min: 0,
            sc_max: 0,
        };

        stats.add_equipment(&sword);

        assert_eq!(stats.dc_min, 3); // 1 + 2
        assert_eq!(stats.dc_max, 7); // 2 + 5
        assert_eq!(stats.accuracy, 5); // 0 + 5
        assert_eq!(stats.ac, 1); // 不变
    }

    /// 验证 calculate 合并等级 + 多件装备属性
    #[test]
    fn test_player_stats_calculate() {
        let sword = ItemTemplate {
            id: 1,
            name: "木剑".to_string(),
            item_type: ItemTypeEnum::Weapon,
            level: 1,
            hp_restore: 0,
            mp_restore: 0,
            weight: 10,
            stack_size: 1,
            price: 100,
            image: 1001,
            required_class: 0,
            required_level: 0,
            description: String::new(),
            dc_min: 2,
            dc_max: 5,
            ac: 0,
            mac: 0,
            accuracy: 5,
            agility: 0,
            durability: 10,
            mc_min: 0,
            mc_max: 0,
            sc_min: 0,
            sc_max: 0,
        };

        let armour = ItemTemplate {
            id: 2,
            name: "布衣".to_string(),
            item_type: ItemTypeEnum::Armour,
            level: 1,
            hp_restore: 0,
            mp_restore: 0,
            weight: 5,
            stack_size: 1,
            price: 50,
            image: 1002,
            required_class: 0,
            required_level: 0,
            description: String::new(),
            dc_min: 0,
            dc_max: 0,
            ac: 3,
            mac: 1,
            accuracy: 0,
            agility: 0,
            durability: 8,
            mc_min: 0,
            mc_max: 0,
            sc_min: 0,
            sc_max: 0,
        };

        let stats = PlayerStats::calculate(10, &[&sword, &armour]);

        // Level 10 base: dc_min=10, dc_max=11, ac=10
        // Sword: +2 dc_min, +5 dc_max
        // Armour: +3 ac, +1 mac
        assert_eq!(stats.dc_min, 12);
        assert_eq!(stats.dc_max, 16);
        assert_eq!(stats.ac, 13);
        assert_eq!(stats.mac, 6); // 5 (base) + 1
        assert_eq!(stats.accuracy, 8); // 3 (base) + 5
    }

    // ============================================
    // EquipmentManager 测试
    // ============================================

    /// 创建一个测试用的 ItemManager，包含基本武器和护甲
    fn setup_test_item_manager() -> ItemManager {
        let mut im = ItemManager::new();

        let sword = ItemTemplate {
            id: 1,
            name: "木剑".to_string(),
            item_type: ItemTypeEnum::Weapon,
            level: 1,
            hp_restore: 0,
            mp_restore: 0,
            weight: 10,
            stack_size: 1,
            price: 100,
            image: 1001,
            required_class: 0,
            required_level: 0,
            description: String::new(),
            dc_min: 2,
            dc_max: 5,
            ac: 0,
            mac: 0,
            accuracy: 5,
            agility: 0,
            durability: 10,
            mc_min: 0,
            mc_max: 0,
            sc_min: 0,
            sc_max: 0,
        };
        im.templates.insert(1, sword);

        let armour = ItemTemplate {
            id: 2,
            name: "布衣".to_string(),
            item_type: ItemTypeEnum::Armour,
            level: 1,
            hp_restore: 0,
            mp_restore: 0,
            weight: 5,
            stack_size: 1,
            price: 50,
            image: 1002,
            required_class: 0,
            required_level: 0,
            description: String::new(),
            dc_min: 0,
            dc_max: 0,
            ac: 3,
            mac: 1,
            accuracy: 0,
            agility: 0,
            durability: 8,
            mc_min: 0,
            mc_max: 0,
            sc_min: 0,
            sc_max: 0,
        };
        im.templates.insert(2, armour);

        let potion = ItemTemplate {
            id: 3,
            name: "金创药".to_string(),
            item_type: ItemTypeEnum::Potion,
            level: 1,
            hp_restore: 50,
            mp_restore: 0,
            weight: 1,
            stack_size: 10,
            price: 50,
            image: 1003,
            required_class: 0,
            required_level: 0,
            description: String::new(),
            dc_min: 0,
            dc_max: 0,
            ac: 0,
            mac: 0,
            accuracy: 0,
            agility: 0,
            durability: 0,
            mc_min: 0,
            mc_max: 0,
            sc_min: 0,
            sc_max: 0,
        };
        im.templates.insert(3, potion);

        im
    }

    /// 创建一个含两格物品的背包：木剑(槽0)、布衣(槽1)
    fn setup_inventory_with_items(inventory: &mut InventoryManager) {
        inventory.add_item(1, 1); // 木剑 → 槽0
        inventory.add_item(2, 1); // 布衣 → 槽1
    }

    /// 测试 equip：穿戴武器
    #[test]
    fn test_equip_weapon() {
        let im = setup_test_item_manager();
        let mut inventory = InventoryManager::new(1);
        setup_inventory_with_items(&mut inventory);

        let mut em = EquipmentManager::new();
        let sword = im.get_template(1).unwrap();

        assert!(em.equip(sword, &mut inventory, 0).is_ok());
        assert_eq!(em.equipped_count(), 1);
        assert_eq!(em.get_equipped_template_id(0), Some(1)); // Weapon slot = 0
    }

    /// 测试 equip 后背包物品被移除
    #[test]
    fn test_equip_removes_from_inventory() {
        let im = setup_test_item_manager();
        let mut inventory = InventoryManager::new(1);
        setup_inventory_with_items(&mut inventory);

        let mut em = EquipmentManager::new();
        let sword = im.get_template(1).unwrap();

        em.equip(sword, &mut inventory, 0).unwrap();
        assert!(inventory.get_slot(0).is_none(), "装备后背包槽0应为空");
        assert!(inventory.get_slot(1).is_some(), "背包槽1应保留布衣");
    }

    /// 测试 unequip：卸下武器放回背包空格
    #[test]
    fn test_unequip_weapon() {
        let im = setup_test_item_manager();
        let mut inventory = InventoryManager::new(1);
        setup_inventory_with_items(&mut inventory);

        let mut em = EquipmentManager::new();
        let sword = im.get_template(1).unwrap();
        em.equip(sword, &mut inventory, 0).unwrap();

        // 卸下武器（slot_index=0）
        let template_id = em.unequip(0, &mut inventory).unwrap();
        assert_eq!(template_id, 1);
        assert_eq!(em.equipped_count(), 0, "卸下后装备位应为空");
        assert!(!em.is_slot_equipped(0));

        // 武器应回到背包
        let slot = inventory.find_slot_by_item_id(1);
        assert!(slot.is_some(), "卸下的武器应回到背包");
    }

    /// 测试自动替换：在同一装备位穿戴新装备时自动卸下旧装备
    #[test]
    fn test_equip_replaces_existing() {
        let im = setup_test_item_manager();
        let mut inventory = InventoryManager::new(1);
        inventory.add_item(1, 1); // 木剑 → 槽0
        inventory.add_item(2, 1); // 布衣 → 槽1

        // 创建另一把武器 (item_id=4)
        let iron_sword = ItemTemplate {
            id: 4,
            name: "铁剑".to_string(),
            item_type: ItemTypeEnum::Weapon,
            level: 5,
            hp_restore: 0,
            mp_restore: 0,
            weight: 30,
            stack_size: 1,
            price: 500,
            image: 1004,
            required_class: 0,
            required_level: 0,
            description: String::new(),
            dc_min: 5,
            dc_max: 12,
            ac: 1,
            mac: 0,
            accuracy: 3,
            agility: 0,
            durability: 20,
            mc_min: 0,
            mc_max: 0,
            sc_min: 0,
            sc_max: 0,
        };

        let mut em = EquipmentManager::new();
        let sword = im.get_template(1).unwrap();

        // 先装备木剑
        em.equip(sword, &mut inventory, 0).unwrap();

        // 背包添加铁剑，然后装备铁剑到同一武器位
        inventory.add_item(4, 1);
        let iron_slot = inventory.find_slot_by_item_id(4).unwrap();

        // 装备铁剑应自动替换木剑
        assert!(em.equip(&iron_sword, &mut inventory, iron_slot).is_ok());
        assert_eq!(em.get_equipped_template_id(0), Some(4)); // 现在是铁剑

        // 木剑应自动回到背包
        let old_sword_slot = inventory.find_slot_by_item_id(1);
        assert!(old_sword_slot.is_some(), "旧武器应回到背包");
    }

    /// 测试 calculate_stats 通过 EquipmentManager 正确计算
    #[test]
    fn test_equipment_manager_calculate_stats() {
        let im = setup_test_item_manager();
        let mut inventory = InventoryManager::new(1);
        setup_inventory_with_items(&mut inventory);

        let mut em = EquipmentManager::new();
        let sword = im.get_template(1).unwrap();
        em.equip(sword, &mut inventory, 0).unwrap();

        let stats = em.calculate_stats(1, &im);
        // Level 1: dc_min=1, dc_max=2, ac=1
        // Wood Sword: +2 dc_min, +5 dc_max, +5 accuracy
        assert_eq!(stats.dc_min, 3);
        assert_eq!(stats.dc_max, 7);
        assert_eq!(stats.ac, 1);
        assert_eq!(stats.accuracy, 5);
    }

    /// 测试 apply_durability_damage 扣除耐久
    #[test]
    fn test_apply_durability_damage() {
        let im = setup_test_item_manager();
        let mut inventory = InventoryManager::new(1);
        setup_inventory_with_items(&mut inventory);

        let mut em = EquipmentManager::new();
        let sword = im.get_template(1).unwrap();
        let armour = im.get_template(2).unwrap();
        em.equip(sword, &mut inventory, 0).unwrap();
        em.equip(armour, &mut inventory, 1).unwrap();

        let result = em.apply_durability_damage();
        assert_eq!(result.len(), 2, "两件装备都应扣除耐久");

        // 再次扣除
        let result2 = em.apply_durability_damage();
        assert_eq!(result2.len(), 2);
    }

    /// 测试不可装备的物品类型（如药水）应当返回错误
    #[test]
    fn test_equip_non_equippable_item_fails() {
        let im = setup_test_item_manager();
        let potion = im.get_template(3).unwrap();
        let mut inventory = InventoryManager::new(1);
        inventory.add_item(3, 1);

        let mut em = EquipmentManager::new();
        let result = em.equip(potion, &mut inventory, 0);
        assert!(result.is_err(), "药水不应能被装备");
        assert_eq!(em.equipped_count(), 0);
    }

    /// 测试装备空背包槽位应当返回错误
    #[test]
    fn test_equip_empty_slot_fails() {
        let im = setup_test_item_manager();
        let mut inventory = InventoryManager::new(1);
        let mut em = EquipmentManager::new();
        let sword = im.get_template(1).unwrap();

        let result = em.equip(sword, &mut inventory, 0);
        assert!(result.is_err(), "空背包槽位不应能装备");
    }
}
