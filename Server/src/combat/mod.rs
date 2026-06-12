pub mod damage;

use std::sync::Arc;

use mir2_shared::enums::MirDirection;
use mir2_shared::packets::server::DeathPacket;
use mir2_shared::packets::Packet;

use crate::map::MapManager;
use crate::network::session_manager::SessionManager;

/// 攻击结果
#[derive(Debug, Clone)]
pub struct AttackResult {
    pub hit: bool,
    pub damage: u32,
    pub target_id: u32,
    pub target_hp_remaining: i32,
    pub target_alive: bool,
    /// 玩家是否在此次攻击中刚刚死亡（HP 从 >0 降为 0）
    pub player_just_died: bool,
}

/// 战斗系统 - 处理玩家与怪物之间的战斗逻辑
pub struct CombatSystem;

impl CombatSystem {
    /// 玩家攻击怪物
    ///
    /// 检查距离和冷却，计算命中率和伤害。
    pub fn player_attack_monster(
        _player_level: u16,
        player_dc_min: i32,
        player_dc_max: i32,
        player_accuracy: u32,
        player_last_attack: &std::time::Instant,
        player_x: i32,
        player_y: i32,
        monster_ac: u32,
        monster_agility: u32,
        monster_current_hp: &mut i32,
        monster_object_id: u32,
        monster_x: i32,
        monster_y: i32,
    ) -> AttackResult {
        let now = std::time::Instant::now();

        // 攻击冷却检查
        let elapsed = now.duration_since(*player_last_attack).as_millis() as u64;
        if elapsed < 500 {
            return AttackResult {
                hit: false,
                damage: 0,
                target_id: monster_object_id,
                target_hp_remaining: *monster_current_hp,
                target_alive: *monster_current_hp > 0,
                player_just_died: false,
            };
        }

        // 距离检查（近战：曼哈顿距离 ≤ 1）
        let dist = (player_x - monster_x).abs() + (player_y - monster_y).abs();
        if dist > 1 {
            return AttackResult {
                hit: false,
                damage: 0,
                target_id: monster_object_id,
                target_hp_remaining: *monster_current_hp,
                target_alive: *monster_current_hp > 0,
                player_just_died: false,
            };
        }

        // 命中率计算
        let hit_rate = Self::calculate_hit_rate(player_accuracy as i32, monster_agility as i32);

        // 掷骰判断是否命中
        let hit = fastrand::i32(0..100) < hit_rate;

        if !hit {
            return AttackResult {
                hit: false,
                damage: 0,
                target_id: monster_object_id,
                target_hp_remaining: *monster_current_hp,
                target_alive: *monster_current_hp > 0,
                player_just_died: false,
            };
        }

        // 伤害计算
        let raw_damage = if player_dc_min >= player_dc_max {
            player_dc_min
        } else {
            player_dc_min + fastrand::i32(0..(player_dc_max - player_dc_min + 1))
        };

        let damage = Self::calculate_damage(raw_damage, monster_ac as i32);

        // 应用伤害
        *monster_current_hp = monster_current_hp.saturating_sub(damage as i32);
        let alive = *monster_current_hp > 0;

        AttackResult {
            hit: true,
            damage: damage as u32,
            target_id: monster_object_id,
            target_hp_remaining: *monster_current_hp,
            target_alive: alive,
            player_just_died: false,
        }
    }

    /// 怪物攻击玩家（简化版）
    ///
    /// session_id / session_manager / map_manager / player_map_id / player_x / player_y 用于死亡处理。
    /// 当玩家 HP 降至 0 时，自动发送 Death 包、传送回城并恢复状态。
    pub fn monster_attack_player(
        monster_dc_min: i32,
        monster_dc_max: i32,
        monster_accuracy: u32,
        _monster_level: u16,
        player_ac: u32,
        player_agility: u32,
        player_current_hp: &mut i32,
        player_session_id: u32,
        session_manager: Option<Arc<SessionManager>>,
        _map_manager: Option<&MapManager>,
        player_map_id: u16,
        _player_x: i32,
        _player_y: i32,
    ) -> AttackResult {
        // 如果玩家已经死亡，直接返回
        if *player_current_hp <= 0 {
            return AttackResult {
                hit: false,
                damage: 0,
                target_id: 0,
                target_hp_remaining: *player_current_hp,
                target_alive: false,
                player_just_died: false,
            };
        }

        let hit_rate = Self::calculate_hit_rate(monster_accuracy as i32, player_agility as i32);
        let hit = fastrand::i32(0..100) < hit_rate;

        if !hit {
            return AttackResult {
                hit: false,
                damage: 0,
                target_id: 0,
                target_hp_remaining: *player_current_hp,
                target_alive: true,
                player_just_died: false,
            };
        }

        let raw_damage = if monster_dc_min >= monster_dc_max {
            monster_dc_min
        } else {
            monster_dc_min + fastrand::i32(0..(monster_dc_max - monster_dc_min + 1))
        };

        let damage = Self::calculate_damage(raw_damage, player_ac as i32);
        let hp_before = *player_current_hp;
        *player_current_hp = player_current_hp.saturating_sub(damage as i32);
        let alive = *player_current_hp > 0;
        let player_just_died = hp_before > 0 && !alive;

        // 玩家死亡处理：发送 Death 包、传送回城、恢复状态
        if player_just_died {
            if let Some(sm) = session_manager {
                let sm = sm.clone();
                tokio::spawn(async move {
                    // 发送 Death 包
                    let death_packet = DeathPacket::new(player_session_id);
                    if let Ok(data) = death_packet.encode() {
                        sm.send_to_session(player_session_id, &data).await;
                    }

                    // 传送到安全区（map 0, 出生点）并恢复 HP/MP
                    sm.update_state(player_session_id, |s| {
                        s.map_id = 0;
                        s.location = (50, 50);
                        s.current_hp = s.max_hp;
                        s.current_mp = s.max_mp;
                    })
                    .await;

                    // 发送地图变更通知
                    let map_changed =
                        mir2_shared::packets::server::MapChangedPacket::new(0);
                    if let Ok(data) = map_changed.encode() {
                        sm.send_to_session(player_session_id, &data).await;
                    }

                    // 发送位置更新
                    let new_loc = mir2_shared::packets::server::UserLocationPacket::new(
                        mir2_shared::types::Point { x: 50, y: 50 },
                        mir2_shared::enums::MirDirection::Down,
                    );
                    if let Ok(data) = new_loc.encode() {
                        sm.send_to_session(player_session_id, &data).await;
                    }

                    // 广播移除旧地图玩家
                    let remove_packet =
                        mir2_shared::packets::server::ObjectRemovePacket::new(
                            player_session_id,
                        );
                    if let Ok(data) = remove_packet.encode() {
                        sm.broadcast_to_map(player_map_id, None, &data).await;
                    }

                    // 发送 HP/MP 更新
                    if let Some(state) = sm.get_state(player_session_id).await {
                        let hp_packet =
                            mir2_shared::packets::server::HealthChangedPacket::new(
                                state.current_hp,
                                state.max_hp,
                            );
                        if let Ok(data) = hp_packet.encode() {
                            sm.send_to_session(player_session_id, &data).await;
                        }
                    }
                });
            }
        }

        AttackResult {
            hit: true,
            damage: damage as u32,
            target_id: 0,
            target_hp_remaining: *player_current_hp,
            target_alive: alive,
            player_just_died,
        }
    }

    /// 计算物理命中率
    /// 公式: min(95, max(5, 50 + (accuracy - agility) * 5)) (%)
    pub fn calculate_hit_rate(accuracy: i32, agility: i32) -> i32 {
        let rate = 50 + (accuracy - agility) * 5;
        rate.clamp(5, 95)
    }

    /// 计算物理伤害
    /// 公式: max(1, raw_damage - target_ac / 2)
    pub fn calculate_damage(raw_damage: i32, target_ac: i32) -> i32 {
        let damage = raw_damage.saturating_sub(target_ac / 2);
        if damage < 1 {
            1
        } else {
            damage
        }
    }
}

/// 便捷函数：使用 PlayerStats 执行玩家攻击
///
/// 自动从 PlayerStats 提取 DC 和准确/敏捷参数。
pub fn player_attack_with_stats(
    stats: &crate::equipment::PlayerStats,
    player_last_attack: &std::time::Instant,
    player_x: i32,
    player_y: i32,
    monster_ac: u32,
    monster_agility: u32,
    monster_current_hp: &mut i32,
    monster_object_id: u32,
    monster_x: i32,
    monster_y: i32,
) -> crate::combat::AttackResult {
    CombatSystem::player_attack_monster(
        0, // level 由调用方处理
        stats.dc_min,
        stats.dc_max,
        stats.accuracy,
        player_last_attack,
        player_x,
        player_y,
        monster_ac,
        monster_agility,
        monster_current_hp,
        monster_object_id,
        monster_x,
        monster_y,
    )
}

/// 便捷函数：使用 PlayerStats 执行怪物攻击玩家
pub fn monster_attack_with_stats(
    stats: &crate::equipment::PlayerStats,
    monster_dc_min: i32,
    monster_dc_max: i32,
    monster_accuracy: u32,
    monster_level: u16,
    player_current_hp: &mut i32,
    player_session_id: u32,
    session_manager: Option<Arc<SessionManager>>,
    map_manager: Option<&MapManager>,
    player_map_id: u16,
    player_x: i32,
    player_y: i32,
) -> crate::combat::AttackResult {
    CombatSystem::monster_attack_player(
        monster_dc_min,
        monster_dc_max,
        monster_accuracy,
        monster_level,
        stats.ac as u32,
        stats.agility,
        player_current_hp,
        player_session_id,
        session_manager,
        map_manager,
        player_map_id,
        player_x,
        player_y,
    )
}

/// 获取某方向的偏移坐标
pub fn direction_offset(dir: MirDirection) -> (i32, i32) {
    match dir {
        MirDirection::Up => (0, -1),
        MirDirection::UpRight => (1, -1),
        MirDirection::Right => (1, 0),
        MirDirection::DownRight => (1, 1),
        MirDirection::Down => (0, 1),
        MirDirection::DownLeft => (-1, 1),
        MirDirection::Left => (-1, 0),
        MirDirection::UpLeft => (-1, -1),
    }
}
