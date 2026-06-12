//! 掉落系统
//!
//! 处理怪物死亡后的物品掉落逻辑。

use crate::item::ItemManager;

/// 处理怪物死亡掉落
///
/// 根据怪物模板 ID 查找掉落规则，将掉落物品放置在地面上。
/// 返回放置在地面上的物品 object_id 列表。
pub fn handle_monster_drop(
    item_manager: &mut ItemManager,
    monster_id: u16,
    map_id: u16,
    x: i32,
    y: i32,
) -> Vec<u32> {
    let drops = item_manager.get_drops_for_monster(monster_id);
    let mut dropped_ids = Vec::new();

    for (item_id, count) in drops {
        for _ in 0..count {
            if let Some(obj_id) = item_manager.drop_item_on_ground(item_id, map_id, x, y) {
                dropped_ids.push(obj_id);
            }
        }
    }

    dropped_ids
}

/// 处理玩家拾取物品
///
/// 返回 (item_id, count) 如果拾取成功
pub fn handle_player_pickup(
    item_manager: &mut ItemManager,
    ground_object_id: u32,
) -> Option<(u16, u32)> {
    if let Some(item) = item_manager.remove_ground_item(ground_object_id) {
        Some((item.item_id, item.count))
    } else {
        None
    }
}
