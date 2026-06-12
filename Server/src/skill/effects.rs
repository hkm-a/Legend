//! 技能效果计算
//!
//! 提供各技能的具体数值计算函数，供战斗系统调用。

use fastrand;

/// 火球术伤害
///
/// `mc`: 玩家魔法攻击力（在 mc_min~mc_max 范围内的取值）
/// `skill_level`: 技能等级 (0~3)
///
/// 公式: `mc * (1.0 + 0.3 * level)`，向下取整
/// 最低伤害: 1
pub fn fireball_damage(mc: i32, skill_level: u8) -> i32 {
    let multiplier = 1.0 + 0.3 * (skill_level as f64);
    let damage = (mc as f64 * multiplier).floor() as i32;
    if damage < 1 {
        1
    } else {
        damage
    }
}

/// 治愈术治疗量
///
/// `sc`: 玩家道术攻击力
/// `skill_level`: 技能等级 (0~3)
///
/// 公式: `(sc / 2 + 10) * (1 + level * 0.5)`，向上取整
pub fn healing_amount(sc: i32, skill_level: u8) -> u32 {
    let base = (sc as f64 / 2.0 + 10.0).floor();
    let multiplier = 1.0 + (skill_level as f64) * 0.5;
    let amount = (base * multiplier).ceil() as u32;
    if amount < 1 {
        1
    } else {
        amount
    }
}

/// 基本剑术攻击加成
///
/// `skill_level`: 技能等级 (0~3)
///
/// 返回 `(dc_min_bonus, dc_max_bonus)` 的额外攻击力加成。
///
/// Lv0: +0/+0
/// Lv1: +1/+3
/// Lv2: +2/+6
/// Lv3: +3/+10
pub fn fencing_bonus(skill_level: u8) -> (i32, i32) {
    match skill_level {
        0 => (0, 0),
        1 => (1, 3),
        2 => (2, 6),
        3 => (3, 10),
        _ => (0, 0),
    }
}

/// 在攻击范围内随机取值（辅助函数）
pub fn random_in_range(min: i32, max: i32) -> i32 {
    if min >= max {
        min
    } else {
        min + fastrand::i32(0..(max - min + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================
    // fireball_damage 测试
    // ============================================

    /// 验证火球术伤害随技能等级和魔法攻击力增长
    #[test]
    fn test_fireball_damage_basic() {
        // MC=20, Lv0: 20 * 1.0 = 20
        assert_eq!(fireball_damage(20, 0), 20);
        // MC=20, Lv1: 20 * 1.3 = 26
        assert_eq!(fireball_damage(20, 1), 26);
        // MC=20, Lv2: 20 * 1.6 = 32
        assert_eq!(fireball_damage(20, 2), 32);
        // MC=20, Lv3: 20 * 1.9 = 38
        assert_eq!(fireball_damage(20, 3), 38);
    }

    /// 验证火球术最低伤害为 1
    #[test]
    fn test_fireball_damage_minimum() {
        // MC=0, Lv0: 0 * 1.0 = 0 → clamp to 1
        assert_eq!(fireball_damage(0, 0), 1);
        // MC=0, Lv3: 0 * 1.9 = 0 → clamp to 1
        assert_eq!(fireball_damage(0, 3), 1);
        // MC=1, Lv0: 1 * 1.0 = 1
        assert_eq!(fireball_damage(1, 0), 1);
    }

    /// 验证火球术伤害随 MC 线性增长
    #[test]
    fn test_fireball_damage_scaling() {
        // 每提升 1 级，系数 +0.3
        let lv3_damage = fireball_damage(50, 3); // 50 * 1.9 = 95
        assert_eq!(lv3_damage, 95);

        let lv0_damage = fireball_damage(50, 0); // 50 * 1.0 = 50
        assert_eq!(lv0_damage, 50);
    }

    // ============================================
    // healing_amount 测试
    // ============================================

    /// 验证治愈术治疗量基础值
    #[test]
    fn test_healing_amount_basic() {
        // SC=20, Lv0: (20/2 + 10) * 1.0 = 20
        assert_eq!(healing_amount(20, 0), 20);
        // SC=20, Lv1: (20/2 + 10) * 1.5 = 30
        assert_eq!(healing_amount(20, 1), 30);
        // SC=20, Lv2: (20/2 + 10) * 2.0 = 40
        assert_eq!(healing_amount(20, 2), 40);
        // SC=20, Lv3: (20/2 + 10) * 2.5 = 50
        assert_eq!(healing_amount(20, 3), 50);
    }

    /// 验证治愈术最低治疗量为 1
    #[test]
    fn test_healing_amount_minimum() {
        // SC=0, Lv0: (0/2 + 10) * 1.0 = 10
        assert_eq!(healing_amount(0, 0), 10);
        // 不应该低于 1
        assert!(healing_amount(0, 0) >= 1);
    }

    /// 验证治愈术随 SC 和技能等级提升
    #[test]
    fn test_healing_amount_scaling() {
        // 高 SC 收益
        let high_sc = healing_amount(100, 3); // (100/2 + 10) * 2.5 = 150
        assert_eq!(high_sc, 150);

        let mid = healing_amount(50, 2); // (50/2 + 10) * 2.0 = 70
        assert_eq!(mid, 70);
    }

    // ============================================
    // fencing_bonus 测试
    // ============================================

    /// 验证基本剑术加成
    #[test]
    fn test_fencing_bonus_basic() {
        assert_eq!(fencing_bonus(0), (0, 0));
        assert_eq!(fencing_bonus(1), (1, 3));
        assert_eq!(fencing_bonus(2), (2, 6));
        assert_eq!(fencing_bonus(3), (3, 10));
    }

    /// 验证无效等级返回 (0, 0)
    #[test]
    fn test_fencing_bonus_invalid_level() {
        assert_eq!(fencing_bonus(4), (0, 0));
        assert_eq!(fencing_bonus(255), (0, 0));
    }

    /// 验证基本剑术加成应用到 DC
    #[test]
    fn test_fencing_bonus_application() {
        // 假设玩家 DC=5-10，剑术 Lv2
        let (min_bonus, max_bonus) = fencing_bonus(2);
        let final_dc_min = 5 + min_bonus; // 7
        let final_dc_max = 10 + max_bonus; // 16

        assert_eq!(final_dc_min, 7);
        assert_eq!(final_dc_max, 16);
    }

    // ============================================
    // random_in_range 测试
    // ============================================

    /// 验证 random_in_range 返回的值在范围内
    #[test]
    fn test_random_in_range() {
        // 确定性的边界
        assert_eq!(random_in_range(5, 5), 5);
        assert_eq!(random_in_range(0, 0), 0);
        assert_eq!(random_in_range(-3, -3), -3);

        // 随机范围，验证在边界内
        for _ in 0..100 {
            let v = random_in_range(10, 20);
            assert!(v >= 10 && v <= 20, "值 {} 应在 [10, 20] 范围内", v);
        }
    }
}
