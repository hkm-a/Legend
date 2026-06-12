//! 怪物 AI 状态机
//!
//! 每个 tick 驱动怪物的状态行为：
//! - Idle: 待机，一定时间后转为 Patrol
//! - Patrol: 向随机方向走 1-2 步
//! - Chase: 玩家进入警戒范围，向玩家方向移动（简化：巡逻时自动切换）
//! - Attack: 在攻击距离内攻击玩家
//! - Dead: 标记死亡

use std::time::Instant;

use mir2_shared::enums::MirDirection;

use super::MonsterState;

/// 攻击范围
const ATTACK_RANGE: i32 = 1;
/// 返回范围（超出此距离回出生点）
const RETURN_RANGE: i32 = 10;
/// 巡逻间隔（毫秒）
const PATROL_INTERVAL_MS: u64 = 2500;

/// 驱动单个怪物的 AI 逻辑（简化版：不依赖 MonsterManager 引用）
pub fn tick_monster_simple(monster: &mut super::Monster) {
    if !monster.is_alive {
        return;
    }

    match monster.state {
        MonsterState::Dead => {}
        MonsterState::Idle => {
            tick_idle(monster);
        }
        MonsterState::Patrol => {
            tick_patrol(monster);
        }
        MonsterState::Chase => {
            tick_chase(monster);
        }
        MonsterState::Attack => {
            tick_attack(monster);
        }
    }
}

/// Idle → Patrol：一定时间后开始巡逻
fn tick_idle(monster: &mut super::Monster) {
    let now = Instant::now();
    let elapsed = now.duration_since(monster.last_move_time).as_millis() as u64;
    if elapsed > PATROL_INTERVAL_MS {
        monster.state = MonsterState::Patrol;
        monster.last_move_time = now;
    }
}

/// Patrol：向随机方向走 1-2 步
fn tick_patrol(monster: &mut super::Monster) {
    let now = Instant::now();
    let elapsed = now.duration_since(monster.last_move_time).as_millis() as u64;
    if elapsed < monster.template.move_speed {
        return;
    }

    // 随机选择 1-2 步方向
    let dir = random_direction();
    let steps = if fastrand::bool() { 1 } else { 2 };

    for _ in 0..steps {
        let (dx, dy) = direction_delta(dir);
        let new_x = monster.location.0 + dx;
        let new_y = monster.location.1 + dy;

        // 检查边界（不超出地图边界，由外部 Manager 校验）
        // 简化：只更新方向，不实际行走（由外部 tick 处理碰撞）
        monster.direction = dir;

        // 简单检查：只在原地附近走动
        let dist_to_spawn = monster.distance_to(monster.spawn_point.0, monster.spawn_point.1);
        if dist_to_spawn <= RETURN_RANGE {
            monster.location = (new_x, new_y);
        } else {
            // 超出返回范围，走回出生点
            let back_dir = direction_toward(monster.location, monster.spawn_point);
            let (bdx, bdy) = direction_delta(back_dir);
            monster.location = (monster.location.0 + bdx, monster.location.1 + bdy);
            monster.direction = back_dir;
            break;
        }
    }

    monster.last_move_time = now;
}

/// Chase：追逐目标（简化）
fn tick_chase(monster: &mut super::Monster) {
    let target_pos = match monster.target_id {
        Some(_) => monster.spawn_point, // fallback: 无真实玩家位置时回出生点
        None => {
            monster.state = MonsterState::Patrol;
            return;
        }
    };

    // 检查返回范围
    let dist_to_spawn = monster.distance_to(monster.spawn_point.0, monster.spawn_point.1);
    if dist_to_spawn > RETURN_RANGE {
        monster.state = MonsterState::Patrol;
        monster.target_id = None;
        return;
    }

    let now = Instant::now();
    let elapsed = now.duration_since(monster.last_move_time).as_millis() as u64;
    if elapsed < monster.template.move_speed {
        return;
    }

    // 检查是否在攻击范围
    let dist_to_target = monster.distance_to(target_pos.0, target_pos.1);
    if dist_to_target <= ATTACK_RANGE {
        monster.state = MonsterState::Attack;
        return;
    }

    // 向目标方向走一步
    let dir = direction_toward(monster.location, target_pos);
    let (dx, dy) = direction_delta(dir);
    monster.direction = dir;
    monster.location = (monster.location.0 + dx, monster.location.1 + dy);

    monster.last_move_time = now;
}

/// Attack：攻击目标（简化）
fn tick_attack(monster: &mut super::Monster) {
    let target_pos = monster.spawn_point; // fallback
    let dist = monster.distance_to(target_pos.0, target_pos.1);

    if dist > ATTACK_RANGE {
        monster.state = MonsterState::Chase;
        return;
    }

    // 攻击冷却
    let now = Instant::now();
    let elapsed = now.duration_since(monster.last_attack_time).as_millis() as u64;
    if elapsed < monster.template.attack_speed {
        return;
    }

    monster.last_attack_time = now;
    // 实际伤害由 CombatSystem 在外部处理
}

/// 获取方向对应的位移偏移
fn direction_delta(dir: MirDirection) -> (i32, i32) {
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

/// 返回朝向目标点的方向
fn direction_toward(from: (i32, i32), to: (i32, i32)) -> MirDirection {
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;

    if dx.abs() > dy.abs() {
        if dx > 0 {
            MirDirection::Right
        } else {
            MirDirection::Left
        }
    } else if dy > 0 {
        MirDirection::Down
    } else {
        MirDirection::Up
    }
}

/// 随机方向
fn random_direction() -> MirDirection {
    match fastrand::u8(0..8) {
        0 => MirDirection::Up,
        1 => MirDirection::UpRight,
        2 => MirDirection::Right,
        3 => MirDirection::DownRight,
        4 => MirDirection::Down,
        5 => MirDirection::DownLeft,
        6 => MirDirection::Left,
        _ => MirDirection::UpLeft,
    }
}
