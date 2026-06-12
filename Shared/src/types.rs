use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

/// 2D 坐标点
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(little)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// 计算到另一个点的曼哈顿距离
    pub fn distance_to(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

/// 属性值对（最小/最大）
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(little)]
pub struct StatRange {
    pub min: i32,
    pub max: i32,
}

impl StatRange {
    pub fn new(min: i32, max: i32) -> Self {
        Self { min, max }
    }

    pub fn zero() -> Self {
        Self { min: 0, max: 0 }
    }

    /// 随机在 [min, max] 之间取一个值
    pub fn random(&self) -> i32 {
        if self.min >= self.max {
            self.min
        } else {
            self.min + fastrand::i32(0..(self.max - self.min + 1))
        }
    }
}

/// 角色基础属性
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(little)]
pub struct Stats {
    /// 最小/最大物理攻击力（DC）
    pub dc: StatRange,
    /// 最小/最大魔法攻击力（MC）
    pub mc: StatRange,
    /// 最小/最大道术攻击力（SC）
    pub sc: StatRange,
    /// 最小/最大防御力（AC）
    pub ac: StatRange,
    /// 最小/最大魔法防御力（MAC）
    pub mac: StatRange,
    /// 生命值
    pub hp: i32,
    /// 魔法值
    pub mp: i32,
    /// 准确
    pub accuracy: u16,
    /// 敏捷
    pub agility: u16,
    /// 攻击速度（正数增加，负数减少）
    pub attack_speed: i16,
    /// 魔法速度
    pub magic_speed: i16,
    /// 幸运
    pub luck: i16,
    /// 诅咒
    pub curse: i16,
    /// 最低攻击
    pub min_dc: i16,
    /// 最低魔法
    pub min_mc: i16,
    /// 最低道术
    pub min_sc: i16,
}

impl Stats {
    pub fn zero() -> Self {
        Self {
            dc: StatRange::zero(),
            mc: StatRange::zero(),
            sc: StatRange::zero(),
            ac: StatRange::zero(),
            mac: StatRange::zero(),
            hp: 0,
            mp: 0,
            accuracy: 0,
            agility: 0,
            attack_speed: 0,
            magic_speed: 0,
            luck: 0,
            curse: 0,
            min_dc: 0,
            min_mc: 0,
            min_sc: 0,
        }
    }
}
