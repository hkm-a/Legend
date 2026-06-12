//! 伤害计算相关核心公式
//!
//! 本模块提供独立的伤害计算函数，供 CombatSystem 和外部调用。

use fastrand;

/// 计算物理命中率
///
/// `accuracy`: 攻击方准确
/// `agility`: 防御方敏捷
///
/// 返回命中概率百分比 (0-100)
///
/// 公式: min(95, max(5, 50 + (accuracy - agility) * 5))
pub fn hit_rate(accuracy: i32, agility: i32) -> i32 {
    let rate = 50 + (accuracy - agility) * 5;
    rate.clamp(5, 95)
}

/// 判定是否命中
pub fn is_hit(accuracy: i32, agility: i32) -> bool {
    let rate = hit_rate(accuracy, agility);
    fastrand::i32(0..100) < rate
}

/// 计算物理伤害
///
/// `raw_damage`: 攻击方原始伤害（在 DC 范围内随机取值）
/// `target_ac`: 防御方防御值
///
/// 公式: max(1, raw_damage - target_ac / 2)
pub fn physical_damage(raw_damage: i32, target_ac: i32) -> i32 {
    let dmg = raw_damage.saturating_sub(target_ac / 2);
    if dmg < 1 {
        1
    } else {
        dmg
    }
}

/// 在攻击范围内随机取值
///
/// `min`: 最小值
/// `max`: 最大值
pub fn random_damage_in_range(min: i32, max: i32) -> i32 {
    if min >= max {
        min
    } else {
        min + fastrand::i32(0..(max - min + 1))
    }
}

/// 检查攻击是否在冷却中
///
/// `last_attack_time`: 上次攻击时间
/// `cooldown_ms`: 冷却时间（毫秒）
pub fn is_on_cooldown(
    last_attack_time: &std::time::Instant,
    cooldown_ms: u64,
) -> bool {
    let elapsed = last_attack_time.elapsed().as_millis() as u64;
    elapsed < cooldown_ms
}

/// 检查近战攻击距离
///
/// 曼哈顿距离 ≤ 1
pub fn is_melee_range(ax: i32, ay: i32, bx: i32, by: i32) -> bool {
    let dist = (ax - bx).abs() + (ay - by).abs();
    dist <= 1
}
