//! 背包管理系统
//!
//! 管理玩家背包中的物品（内存中），提供增删改查操作。
//! 与数据库 InventoryRepository 配合使用。

use std::collections::HashMap;

use crate::database::repository::InventoryRepository;

const INVENTORY_SLOTS: usize = 40;

/// 背包中的单个物品
#[derive(Debug, Clone)]
pub struct InventorySlot {
    pub uid: i64, // 数据库 user_items.id
    pub item_id: u16,
    pub count: u32,
    pub durability: i32,
    pub max_durability: i32,
    pub is_equipped: bool,
}

/// 背包管理器（内存态）
pub struct InventoryManager {
    slots: Vec<Option<InventorySlot>>,
    character_id: i64,
}

impl InventoryManager {
    pub fn new(character_id: i64) -> Self {
        Self {
            slots: vec![None; INVENTORY_SLOTS],
            character_id,
        }
    }

    /// 从数据库加载背包
    pub async fn load_from_db(
        &mut self,
        repo: &InventoryRepository,
    ) -> Result<(), sqlx::Error> {
        let items = repo.get_inventory_items(self.character_id).await?;
        for item in items {
            let slot = item.slot as usize;
            if slot < INVENTORY_SLOTS {
                self.slots[slot] = Some(InventorySlot {
                    uid: item.id,
                    item_id: item.item_id as u16,
                    count: item.count as u32,
                    durability: item.durability,
                    max_durability: item.max_durability,
                    is_equipped: false,
                });
            }
        }
        Ok(())
    }

    /// 获取指定槽位的物品
    pub fn get_slot(&self, slot: usize) -> Option<&InventorySlot> {
        self.slots.get(slot).and_then(|s| s.as_ref())
    }

    /// 添加物品到第一个空格
    pub fn add_item(&mut self, item_id: u16, count: u32) -> Option<usize> {
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(InventorySlot {
                    uid: 0, // 未持久化
                    item_id,
                    count,
                    durability: 0,
                    max_durability: 0,
                    is_equipped: false,
                });
                return Some(i);
            }
        }
        None // 背包已满
    }

    /// 从指定槽位移除物品（减少数量或完全移除）
    pub fn remove_item(&mut self, slot: usize, count: u32) -> bool {
        if let Some(Some(item)) = self.slots.get_mut(slot) {
            if item.count <= count {
                self.slots[slot] = None;
            } else {
                item.count -= count;
            }
            true
        } else {
            false
        }
    }

    /// 获取空格数量
    pub fn empty_slots(&self) -> usize {
        self.slots.iter().filter(|s| s.is_none()).count()
    }

    /// 检查是否有足够空格
    pub fn has_space_for(&self, slots_needed: usize) -> bool {
        self.empty_slots() >= slots_needed
    }

    /// 获取所有物品（非空槽位）
    pub fn all_items(&self) -> Vec<(usize, &InventorySlot)> {
        self.slots
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.as_ref().map(|item| (i, item)))
            .collect()
    }

    /// 获取角色 ID
    pub fn character_id(&self) -> i64 {
        self.character_id
    }

    /// 根据 item_id 查找第一个槽位
    pub fn find_slot_by_item_id(&self, item_id: u16) -> Option<usize> {
        self.slots.iter().position(|s| {
            s.as_ref().map_or(false, |item| item.item_id == item_id)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================
    // InventorySlot 扩展字段测试
    // ============================================

    /// 测试 InventorySlot 的装备标记字段能正确设置和读取
    #[test]
    fn test_inventory_slot_equipment_marker() {
        // 创建一个已装备的物品
        let equipped = InventorySlot {
            uid: 1001,
            item_id: 10,
            count: 1,
            durability: 30,
            max_durability: 30,
            is_equipped: true,
        };
        assert!(equipped.is_equipped, "装备物品 is_equipped 应为 true");

        // 创建一个未装备的物品
        let not_equipped = InventorySlot {
            uid: 1002,
            item_id: 20,
            count: 5,
            durability: 0,
            max_durability: 0,
            is_equipped: false,
        };
        assert!(!not_equipped.is_equipped, "普通物品 is_equipped 应为 false");

        // 默认值应为 false
        let default_slot = InventorySlot {
            uid: 1003,
            item_id: 30,
            count: 1,
            durability: 10,
            max_durability: 10,
            is_equipped: false,
        };
        assert!(!default_slot.is_equipped, "默认 is_equipped 应为 false");

        // 切换装备状态
        let mut toggle_slot = equipped.clone();
        toggle_slot.is_equipped = false;
        assert!(!toggle_slot.is_equipped, "切换后 is_equipped 应为 false");
        toggle_slot.is_equipped = true;
        assert!(toggle_slot.is_equipped, "再次切换后 is_equipped 应为 true");
    }

    /// 验证装备标记在 InventoryManager 操作中保持正确
    #[test]
    fn test_equipment_marker_in_inventory_manager() {
        let mut manager = InventoryManager::new(1);

        // 添加一个装备物品
        manager.slots[0] = Some(InventorySlot {
            uid: 2001,
            item_id: 50,
            count: 1,
            durability: 20,
            max_durability: 20,
            is_equipped: true,
        });

        // 通过 get_slot 读取装备标记
        let slot = manager.get_slot(0).unwrap();
        assert!(slot.is_equipped, "装备槽位的 is_equipped 应为 true");

        // all_items 应包含装备标记
        let items = manager.all_items();
        assert_eq!(items.len(), 1);
        assert!(items[0].1.is_equipped);
    }

    /// 验证 InventorySlot Clone 后装备标记保持一致
    #[test]
    fn test_equipment_marker_clone_consistency() {
        let original = InventorySlot {
            uid: 3001,
            item_id: 100,
            count: 1,
            durability: 50,
            max_durability: 50,
            is_equipped: true,
        };

        let cloned = original.clone();
        assert_eq!(cloned.is_equipped, original.is_equipped, "Clone 后装备标记应一致");
        assert_eq!(cloned.uid, original.uid);
        assert_eq!(cloned.item_id, original.item_id);
    }
}

/// 检查物品能否被拾取
pub fn can_pick_up(
    ground_item_id: u32,
    player_x: i32,
    player_y: i32,
    ground_items: &HashMap<u32, super::GroundItem>,
) -> bool {
    if let Some(item) = ground_items.get(&ground_item_id) {
        item.distance_to(player_x, player_y) <= 1
    } else {
        false
    }
}
