//! 刷怪逻辑
//!
//! 提供刷怪批量辅助函数。
//! `spawn_monster` 和 `find_spawn_position` 等核心方法在 `MonsterManager` 中实现（`mod.rs`）。

use crate::map::MonsterSpawnConfig;
use crate::monster::MonsterManager;

/// 辅助函数：计算需要刷新的怪物数量
pub fn calculate_spawn_count(
    configured_count: u8,
    current_alive: usize,
    max_per_spawn: u8,
) -> u8 {
    let missing = (configured_count as usize).saturating_sub(current_alive) as u8;
    missing.min(max_per_spawn)
}

/// 生成一批怪物并返回所有生成的 object_id
pub fn spawn_batch(
    manager: &mut MonsterManager,
    map_id: u16,
    configs: &[MonsterSpawnConfig],
) -> Vec<u32> {
    let mut spawned_ids = Vec::new();
    for spawn in configs {
        let _template = match manager.templates.get(&spawn.monster_id) {
            Some(t) => t,
            None => continue,
        };
        let alive = manager.alive_count_on_map(map_id, spawn.monster_id) as u8;
        if alive >= spawn.count {
            continue;
        }
        let to_spawn = spawn.count.saturating_sub(alive);
        for _ in 0..to_spawn {
            if let Some(id) = manager.spawn_monster(map_id, spawn) {
                spawned_ids.push(id);
            }
        }
    }
    spawned_ids
}
