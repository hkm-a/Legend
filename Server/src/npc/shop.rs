//! 商店交易系统
//!
//! 处理玩家与 NPC 之间的购买和出售逻辑。

use super::NpcTemplate;
use crate::item::inventory::InventoryManager;
use crate::item::ItemTemplate;

/// 商店交易结果
pub enum ShopResult {
    Success,
    InsufficientGold,
    ItemNotFound,
    InventoryFull,
    NpcNotSellItem,
}

/// 商店交易系统
pub struct ShopSystem;

impl ShopSystem {
    /// 尝试购买（不含背包修改的简化版，用于 handler 层预处理）
    ///
    /// 检查 NPC 是否出售该物品、玩家金币是否足够。
    /// 不修改背包或金币——调用方需自行处理状态更新。
    pub fn try_buy(
        npc: &NpcTemplate,
        item_template: &ItemTemplate,
        gold: u64,
    ) -> ShopResult {
        if !npc.selling_items.contains(&item_template.id) {
            return ShopResult::NpcNotSellItem;
        }
        let price = item_template.price as u64;
        if gold < price {
            return ShopResult::InsufficientGold;
        }
        ShopResult::Success
    }

    /// 从 NPC 处购买物品
    ///
    /// 检查 NPC 是否出售该物品、玩家金币是否足够、背包是否有空位。
    /// 成功后扣除金币并添加物品到背包。
    pub fn buy_item(
        npc: &NpcTemplate,
        item_template: &ItemTemplate,
        gold: &mut u64,
        inventory: &mut InventoryManager,
    ) -> ShopResult {
        // 检查 NPC 是否出售该物品
        if !npc.selling_items.contains(&item_template.id) {
            return ShopResult::NpcNotSellItem;
        }

        let price = item_template.price as u64;
        if *gold < price {
            return ShopResult::InsufficientGold;
        }

        // 尝试添加到背包
        match inventory.add_item(item_template.id, 1) {
            Some(_) => {
                *gold -= price;
                ShopResult::Success
            }
            None => ShopResult::InventoryFull,
        }
    }

    /// 向 NPC 出售物品
    ///
    /// 检查物品是否已装备，成功后添加金币并移除物品。
    /// 出售价格为买入价的一半（向下取整）。
    pub fn sell_item(
        item_template: &ItemTemplate,
        gold: &mut u64,
        inventory: &mut InventoryManager,
        slot: usize,
    ) -> ShopResult {
        let item = match inventory.get_slot(slot) {
            Some(slot_item) => slot_item.clone(),
            None => return ShopResult::ItemNotFound,
        };

        // 已装备的物品不能出售
        if item.is_equipped {
            return ShopResult::ItemNotFound;
        }

        let sell_price = (item_template.price / 2) as u64;
        *gold = gold.saturating_add(sell_price);
        inventory.remove_item(slot, 1);
        ShopResult::Success
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item::inventory::InventoryManager;
    use crate::item::ItemTemplate;
    use mir2_shared::enums::ItemType;

    fn make_potion_template() -> ItemTemplate {
        ItemTemplate {
            id: 0,
            name: "金创药(小)".to_string(),
            item_type: ItemType::Potion,
            level: 1,
            hp_restore: 40,
            mp_restore: 0,
            weight: 1,
            stack_size: 99,
            price: 50,
            image: 0,
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
        }
    }

    fn make_npc() -> NpcTemplate {
        NpcTemplate {
            id: 1,
            name: "药店老板".to_string(),
            map_id: 0,
            x: 14,
            y: 14,
            image: 10,
            shop_type: "BuyAndSell".to_string(),
            selling_items: vec![0, 1],
        }
    }

    /// 测试购买成功
    #[test]
    fn test_buy_item_success() {
        let npc = make_npc();
        let item = make_potion_template();
        let mut gold = 100u64;
        let mut inventory = InventoryManager::new(1);

        let result = ShopSystem::buy_item(&npc, &item, &mut gold, &mut inventory);
        assert!(matches!(result, ShopResult::Success));
        assert_eq!(gold, 50, "金币应扣除 50");
        let items = inventory.all_items();
        assert_eq!(items.len(), 1, "背包应有 1 个物品");
    }

    /// 测试金币不足
    #[test]
    fn test_buy_item_insufficient_gold() {
        let npc = make_npc();
        let item = make_potion_template();
        let mut gold = 10u64;
        let mut inventory = InventoryManager::new(1);

        let result = ShopSystem::buy_item(&npc, &item, &mut gold, &mut inventory);
        assert!(matches!(result, ShopResult::InsufficientGold));
        assert_eq!(gold, 10, "金币不应变化");
    }

    /// 测试 NPC 不出售该物品
    #[test]
    fn test_buy_item_npc_not_sell() {
        let npc = make_npc();
        let mut item = make_potion_template();
        item.id = 99; // NPC 不卖这个
        let mut gold = 100u64;
        let mut inventory = InventoryManager::new(1);

        let result = ShopSystem::buy_item(&npc, &item, &mut gold, &mut inventory);
        assert!(matches!(result, ShopResult::NpcNotSellItem));
    }

    /// 测试出售成功
    #[test]
    fn test_sell_item_success() {
        let item_template = make_potion_template();
        let mut inventory = InventoryManager::new(1);
        inventory.add_item(0, 1);

        let mut gold = 0u64;
        let slot = 0;

        let result = ShopSystem::sell_item(&item_template, &mut gold, &mut inventory, slot);
        assert!(matches!(result, ShopResult::Success));
        assert_eq!(gold, 25, "出售价为半价 (50/2=25)");
        assert!(inventory.get_slot(slot).is_none(), "物品应从背包移除");
    }

    /// 测试出售空背包
    #[test]
    fn test_sell_item_empty_inventory() {
        let item_template = make_potion_template();
        let mut inventory = InventoryManager::new(1);
        let mut gold = 100u64;

        let result = ShopSystem::sell_item(&item_template, &mut gold, &mut inventory, 0);
        assert!(matches!(result, ShopResult::ItemNotFound));
    }
}
