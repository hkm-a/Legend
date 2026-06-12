//! 技能系统
//!
//! 管理玩家技能的学习、使用、熟练度增长与升级。
//! 每个技能有 3 个等级，通过累积熟练度提升。

use std::collections::HashMap;

pub mod effects;

/// 技能状态
#[derive(Debug, Clone)]
pub struct SkillState {
    pub spell_id: u16,
    pub level: u8,
    pub proficiency: u32,
}

impl SkillState {
    pub fn new(spell_id: u16) -> Self {
        Self {
            spell_id,
            level: 0,
            proficiency: 0,
        }
    }

    /// 获取升到下一级所需的熟练度
    ///
    /// - Lv0 → Lv1: 100
    /// - Lv1 → Lv2: 300
    /// - Lv2 → Lv3: 600
    /// - Lv3+ : None (满级)
    pub fn proficiency_for_next_level(level: u8) -> Option<u32> {
        match level {
            0 => Some(100),
            1 => Some(300),
            2 => Some(600),
            _ => None,
        }
    }

    /// 当前技能等级的名称
    pub fn level_name(&self) -> &'static str {
        match self.level {
            0 => "未学习",
            1 => "初级",
            2 => "中级",
            3 => "高级",
            _ => "未知",
        }
    }
}

/// 技能管理器
#[derive(Debug, Clone)]
pub struct SkillManager {
    pub skills: HashMap<u16, SkillState>,
}

impl SkillManager {
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
        }
    }

    /// 学习新技能
    ///
    /// 如果技能已经学习则返回错误。
    pub fn learn_skill(&mut self, spell_id: u16) -> Result<(), String> {
        if self.skills.contains_key(&spell_id) {
            return Err(format!("Skill {} is already learned", spell_id));
        }
        let state = SkillState {
            spell_id,
            level: 0,
            proficiency: 0,
        };
        self.skills.insert(spell_id, state);
        Ok(())
    }

    /// 使用技能
    ///
    /// 每次使用增加 1~3 点熟练度（随机）。
    /// 如果熟练度达到升级阈值则自动升级。
    /// 返回 `Ok(Some(new_level))` 表示升级，`Ok(None)` 表示未升级，
    /// `Err` 表示技能未学习。
    pub fn use_skill(&mut self, spell_id: u16) -> Result<Option<u8>, String> {
        let state = self
            .skills
            .get_mut(&spell_id)
            .ok_or_else(|| format!("Skill {} not learned", spell_id))?;

        // 满级不再增加熟练度
        if state.level >= 3 {
            return Ok(None);
        }

        // 增加 1~3 点熟练度
        let gain = fastrand::u32(1..=3);
        state.proficiency = state.proficiency.saturating_add(gain);

        // 检测升级
        let required = SkillState::proficiency_for_next_level(state.level);
        if let Some(req) = required {
            if state.proficiency >= req {
                state.level += 1;
                state.proficiency = 0; // 重置熟练度
                return Ok(Some(state.level));
            }
        }

        Ok(None)
    }

    /// 获取所有已学习的技能
    pub fn get_learned_skills(&self) -> Vec<&SkillState> {
        self.skills.values().collect()
    }

    /// 批量加载技能（从数据库加载时使用）
    pub fn load_from_list(&mut self, list: Vec<SkillState>) {
        for skill in list {
            self.skills.insert(skill.spell_id, skill);
        }
    }

    /// 获取指定技能的等级
    pub fn get_skill_level(&self, spell_id: u16) -> Option<u8> {
        self.skills.get(&spell_id).map(|s| s.level)
    }

    /// 检查技能是否已学习
    pub fn has_skill(&self, spell_id: u16) -> bool {
        self.skills.contains_key(&spell_id)
    }

    /// 已学习的技能数量
    pub fn skill_count(&self) -> usize {
        self.skills.len()
    }
}

impl Default for SkillManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试学习新技能
    #[test]
    fn test_learn_skill() {
        let mut sm = SkillManager::new();
        assert!(sm.learn_skill(31).is_ok()); // FireBall
        assert!(sm.has_skill(31));
        assert_eq!(sm.skill_count(), 1);
    }

    /// 测试重复学习技能返回错误
    #[test]
    fn test_learn_skill_duplicate() {
        let mut sm = SkillManager::new();
        sm.learn_skill(31).unwrap();
        let result = sm.learn_skill(31);
        assert!(result.is_err(), "重复学习应返回错误");
    }

    /// 测试使用未学习的技能返回错误
    #[test]
    fn test_use_unlearned_skill() {
        let mut sm = SkillManager::new();
        let result = sm.use_skill(31);
        assert!(result.is_err(), "使用未学习技能应返回错误");
    }

    /// 测试技能熟练度增长和升级
    #[test]
    fn test_skill_proficiency_and_level_up() {
        let mut sm = SkillManager::new();
        sm.learn_skill(31).unwrap(); // FireBall, level 0

        // Lv0 → Lv1 需要 100 熟练度
        // 每次使用 +1~3, 模拟多次使用直到升级
        let mut leveled_up = false;
        for _ in 0..200 {
            if let Ok(Some(_new_level)) = sm.use_skill(31) {
                leveled_up = true;
                break;
            }
        }
        assert!(leveled_up, "技能应在 200 次使用内从 Lv0 升到 Lv1");
        let state = sm.skills.get(&31).unwrap();
        assert!(state.level >= 1, "等级应至少为 1");
        assert_eq!(state.proficiency, 0, "升级后熟练度应重置为 0");
    }

    /// 测试技能满级后不再增加熟练度
    #[test]
    fn test_max_level_skill_no_proficiency_gain() {
        let mut sm = SkillManager::new();

        // 直接创建一个满级技能
        sm.skills.insert(
            31,
            SkillState {
                spell_id: 31,
                level: 3,
                proficiency: 0,
            },
        );

        // 满级后使用不应增加熟练度
        for _ in 0..10 {
            let result = sm.use_skill(31).unwrap();
            assert!(result.is_none(), "满级技能不应返回升级");
        }

        let state = sm.skills.get(&31).unwrap();
        assert_eq!(state.level, 3, "满级技能等级不变");
        assert_eq!(state.proficiency, 0, "满级技能熟练度不变");
    }

    /// 测试 get_learned_skills 返回所有技能
    #[test]
    fn test_get_learned_skills() {
        let mut sm = SkillManager::new();
        sm.learn_skill(31).unwrap();
        sm.learn_skill(61).unwrap(); // Healing
        sm.learn_skill(1).unwrap(); // Fencing

        let skills = sm.get_learned_skills();
        assert_eq!(skills.len(), 3);

        let ids: Vec<u16> = skills.iter().map(|s| s.spell_id).collect();
        assert!(ids.contains(&31));
        assert!(ids.contains(&61));
        assert!(ids.contains(&1));
    }

    /// 测试 load_from_list 批量加载
    #[test]
    fn test_load_from_list() {
        let list = vec![
            SkillState {
                spell_id: 31,
                level: 1,
                proficiency: 50,
            },
            SkillState {
                spell_id: 61,
                level: 2,
                proficiency: 0,
            },
        ];

        let mut sm = SkillManager::new();
        sm.load_from_list(list);

        assert_eq!(sm.skill_count(), 2);
        assert_eq!(sm.get_skill_level(31), Some(1));
        assert_eq!(sm.get_skill_level(61), Some(2));
    }

    /// 测试 SkillState::proficiency_for_next_level 阈值
    #[test]
    fn test_proficiency_thresholds() {
        assert_eq!(SkillState::proficiency_for_next_level(0), Some(100));
        assert_eq!(SkillState::proficiency_for_next_level(1), Some(300));
        assert_eq!(SkillState::proficiency_for_next_level(2), Some(600));
        assert_eq!(SkillState::proficiency_for_next_level(3), None);
        assert_eq!(SkillState::proficiency_for_next_level(4), None);
    }
}
