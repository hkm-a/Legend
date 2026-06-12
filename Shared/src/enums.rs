use binrw::{BinRead, BinWrite};
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

/// 职业
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum MirClass {
    Warrior = 0,
    Wizard = 1,
    Taoist = 2,
    Assassin = 3,
    Archer = 4,
}

/// 性别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum MirGender {
    Male = 0,
    Female = 1,
}

/// 方向（8 方向）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum MirDirection {
    Up = 0,
    UpRight = 1,
    Right = 2,
    DownRight = 3,
    Down = 4,
    DownLeft = 5,
    Left = 6,
    UpLeft = 7,
}

/// 对象类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum ObjectType {
    None = 0,
    Player = 1,
    Item = 2,
    Merchant = 3,
    Spell = 4,
    Monster = 5,
    Deco = 6,
    Creature = 7,
    Hero = 8,
}

/// 动作
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum MirAction {
    Standing = 0,
    Walking = 1,
    Running = 2,
    Pushed = 3,
    DashL = 4,
    DashR = 5,
    DashFail = 6,
    Stance = 7,
    Stance2 = 8,
    Attack1 = 9,
    Attack2 = 10,
    Attack3 = 11,
    Attack4 = 12,
    Attack5 = 13,
    AttackRange1 = 14,
    AttackRange2 = 15,
    AttackRange3 = 16,
    Special = 17,
    Struck = 18,
    Harvest = 19,
    Spell = 20,
    Die = 21,
    Dead = 22,
    Skeleton = 23,
    Show = 24,
    Hide = 25,
    Stoned = 26,
    Appear = 27,
    Revive = 28,
    SitDown = 29,
    Mine = 30,
    Sneek = 31,
    DashAttack = 32,
    Lunge = 33,
    WalkingBow = 34,
    RunningBow = 35,
    Jump = 36,
    MountStanding = 37,
    MountWalking = 38,
    MountRunning = 39,
    MountStruck = 40,
    MountAttack = 41,
    FishingCast = 42,
    FishingWait = 43,
    FishingReel = 44,
}

/// 格子属性
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum CellAttribute {
    Walk = 0,
    HighWall = 1,
    LowWall = 2,
}

/// 光照设置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum LightSetting {
    Normal = 0,
    Dawn = 1,
    Day = 2,
    Evening = 3,
    Night = 4,
}

/// 聊天类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum ChatType {
    Normal = 0,
    Shout = 1,
    System = 2,
    Hint = 3,
    Announcement = 4,
    Group = 5,
    WhisperIn = 6,
    WhisperOut = 7,
    Guild = 8,
    Trainer = 9,
    LevelUp = 10,
    System2 = 11,
    Relationship = 12,
    Mentor = 13,
    Shout2 = 14,
    Shout3 = 15,
    LineMessage = 16,
}

/// 物品类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum ItemType {
    Nothing = 0,
    Weapon = 1,
    Armour = 2,
    Helmet = 4,
    Necklace = 5,
    Bracelet = 6,
    Ring = 7,
    Amulet = 8,
    Belt = 9,
    Boots = 10,
    Stone = 11,
    Torch = 12,
    Potion = 13,
    Ore = 14,
    Meat = 15,
    CraftingMaterial = 16,
    Scroll = 17,
    Gem = 18,
    Mount = 19,
    Book = 20,
    Script = 21,
    Reins = 22,
    Bells = 23,
    Saddle = 24,
    Ribbon = 25,
    Mask = 26,
    Food = 27,
    Hook = 28,
    Float = 29,
    Bait = 30,
    Finder = 31,
    Reel = 32,
    Fish = 33,
    Quest = 34,
    Awakening = 35,
    Pets = 36,
    Transform = 37,
    Deco = 38,
    Socket = 39,
    MonsterSpawn = 40,
    SiegeAmmo = 41,
    SealedHero = 42,
}

/// 装备槽位
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum EquipmentSlot {
    Weapon = 0,
    Armour = 1,
    Helmet = 2,
    Torch = 3,
    Necklace = 4,
    BraceletL = 5,
    BraceletR = 6,
    RingL = 7,
    RingR = 8,
    Amulet = 9,
    Belt = 10,
    Boots = 11,
    Stone = 12,
    Mount = 13,
}

/// 攻击模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum AttackMode {
    Peace = 0,
    Group = 1,
    Guild = 2,
    EnemyGuild = 3,
    RedBrown = 4,
    All = 5,
}

/// 宠物模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum PetMode {
    Both = 0,
    MoveOnly = 1,
    AttackOnly = 2,
    None = 3,
    FocusMasterTarget = 4,
}

bitflags! {
    #[doc = "毒类型（Flags）"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct PoisonType: u16 {
        const NONE = 0;
        const GREEN = 1;
        const RED = 2;
        const SLOW = 4;
        const FROZEN = 8;
        const STUN = 16;
        const PARALYSIS = 32;
        const DELAYED_EXPLOSION = 64;
        const BLEEDING = 128;
        const LR_PARALYSIS = 256;
        const BLINDNESS = 512;
        const DAZED = 1024;
    }
}

/// 怪物稀有度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum MonsterType {
    Normal = 0,
    Uncommon = 1,
    Rare = 2,
    Elite = 3,
}

/// 防御类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum DefenceType {
    ACAgility = 0,
    AC = 1,
    MACAgility = 2,
    MAC = 3,
    Agility = 4,
    Repulsion = 5,
    None = 6,
}

/// Buff 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum BuffType {
    None = 0,
    // Magics
    TemporalFlux = 1,
    Hiding = 2,
    Haste = 3,
    SwiftFeet = 4,
    Fury = 5,
    SoulShield = 6,
    BlessedArmour = 7,
    LightBody = 8,
    UltimateEnhancer = 9,
    ProtectionField = 10,
    Rage = 11,
    Curse = 12,
    MoonLight = 13,
    DarkBody = 14,
    Concentration = 15,
    VampireShot = 16,
    PoisonShot = 17,
    CounterAttack = 18,
    MentalState = 19,
    EnergyShield = 20,
    MagicBooster = 21,
    PetEnhancer = 22,
    ImmortalSkin = 23,
    MagicShield = 24,
    ElementalBarrier = 25,
    // Monster
    HornedArcherBuff = 50,
    ColdArcherBuff = 51,
    GeneralMeowMeowShield = 52,
    RhinoPriestDebuff = 53,
    PowerBeadBuff = 54,
    HornedWarriorShield = 55,
    HornedCommanderShield = 56,
    Blindness = 57,
    // Special
    GameMaster = 100,
    General = 101,
    Exp = 102,
    Drop = 103,
    Gold = 104,
    BagWeight = 105,
    Transform = 106,
    Lover = 107,
    Mentee = 108,
    Mentor = 109,
    Guild = 110,
    Prison = 111,
    Rested = 112,
    Skill = 113,
    ClearRing = 114,
    Newbie = 115,
    // Stats
    Impact = 200,
    Magic = 201,
    Taoist = 202,
    Storm = 203,
    HealthAid = 204,
    ManaAid = 205,
    Defence = 206,
    MagicDefence = 207,
    WonderDrug = 208,
    Knapsack = 209,
}

bitflags! {
    #[doc = "Buff 属性（Flags）"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct BuffProperty: u8 {
        const NONE = 0;
        const REMOVE_ON_DEATH = 1;
        const REMOVE_ON_EXIT = 2;
        const DEBUFF = 4;
        const PAUSE_IN_SAFE_ZONE = 8;
    }
}

/// Buff 堆叠类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum BuffStackType {
    None = 0,
    ResetDuration = 1,
    StackDuration = 2,
    StackStat = 3,
    StackStatAndDuration = 4,
    Infinite = 5,
    ResetStat = 6,
    ResetStatAndDuration = 7,
}

/// 技能
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum Spell {
    None = 0,
    // Warrior
    Fencing = 1,
    Slaying = 2,
    Thrusting = 3,
    HalfMoon = 4,
    ShoulderDash = 5,
    TwinDrakeBlade = 6,
    Entrapment = 7,
    FlamingSword = 8,
    LionRoar = 9,
    CrossHalfMoon = 10,
    BladeAvalanche = 11,
    ProtectionField = 12,
    Rage = 13,
    CounterAttack = 14,
    SlashingBurst = 15,
    Fury = 16,
    ImmortalSkin = 17,
    // Wizard
    FireBall = 31,
    Repulsion = 32,
    ElectricShock = 33,
    GreatFireBall = 34,
    HellFire = 35,
    ThunderBolt = 36,
    Teleport = 37,
    FireBang = 38,
    FireWall = 39,
    Lightning = 40,
    FrostCrunch = 41,
    ThunderStorm = 42,
    MagicShield = 43,
    TurnUndead = 44,
    Vampirism = 45,
    IceStorm = 46,
    FlameDisruptor = 47,
    Mirroring = 48,
    FlameField = 49,
    Blizzard = 50,
    MagicBooster = 51,
    MeteorStrike = 52,
    IceThrust = 53,
    FastMove = 54,
    StormEscape = 55,
    // Taoist
    Healing = 61,
    SpiritSword = 62,
    Poisoning = 63,
    SoulFireBall = 64,
    SummonSkeleton = 65,
    Hiding = 67,
    MassHiding = 68,
    SoulShield = 69,
    Revelation = 70,
    BlessedArmour = 71,
    EnergyRepulsor = 72,
    TrapHexagon = 73,
    Purification = 74,
    MassHealing = 75,
    Hallucination = 76,
    UltimateEnhancer = 77,
    SummonShinsu = 78,
    Reincarnation = 79,
    SummonHolyDeva = 80,
    Curse = 81,
    Plague = 82,
    PoisonCloud = 83,
    EnergyShield = 84,
    PetEnhancer = 85,
    HealingCircle = 86,
    // Assassin
    FatalSword = 91,
    DoubleSlash = 92,
    Haste = 93,
    FlashDash = 94,
    LightBody = 95,
    HeavenlySword = 96,
    FireBurst = 97,
    Trap = 98,
    PoisonSword = 99,
    MoonLight = 100,
    MPEater = 101,
    SwiftFeet = 102,
    DarkBody = 103,
    Hemorrhage = 104,
    CrescentSlash = 105,
    MoonMist = 106,
    CatTongue = 107,
    // Archer
    Focus = 121,
    StraightShot = 122,
    DoubleShot = 123,
    ExplosiveTrap = 124,
    DelayedExplosion = 125,
    Meditation = 126,
    BackStep = 127,
    ElementalShot = 128,
    Concentration = 129,
    Stonetrap = 130,
    ElementalBarrier = 131,
    SummonVampire = 132,
    VampireShot = 133,
    SummonToad = 134,
    PoisonShot = 135,
    CrippleShot = 136,
    SummonSnakes = 137,
    NapalmShot = 138,
    OneWithNature = 139,
    BindingShot = 140,
    MentalState = 141,
    // Custom
    Blink = 151,
    Portal = 152,
    BattleCry = 153,
    FireBounce = 154,
    MeteorShower = 155,
    // Map Events
    DigOutZombie = 200,
    Rubble = 201,
    MapLightning = 202,
    MapLava = 203,
    MapQuake1 = 204,
    MapQuake2 = 205,
    DigOutArmadillo = 206,
    GeneralMeowMeowThunder = 207,
    StoneGolemQuake = 208,
    EarthGolemPile = 209,
    TreeQueenRoot = 210,
    TreeQueenMassRoots = 211,
    TreeQueenGroundRoots = 212,
    TucsonGeneralRock = 213,
    FlyingStatueIceTornado = 214,
    DarkOmaKingNuke = 215,
    HornedSorcererDustTornado = 216,
    HornedCommanderRockFall = 217,
    HornedCommanderRockSpike = 218,
}

/// 技能效果
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum SpellEffect {
    None = 0,
    FatalSword = 1,
    Teleport = 2,
    Healing = 3,
    RedMoonEvil = 4,
    TwinDrakeBlade = 5,
    MagicShieldUp = 6,
    MagicShieldDown = 7,
    GreatFoxSpirit = 8,
    Entrapment = 9,
    Reflect = 10,
    Critical = 11,
    Mine = 12,
    ElementalBarrierUp = 13,
    ElementalBarrierDown = 14,
    DelayedExplosion = 15,
    MPEater = 16,
    Hemorrhage = 17,
    Bleeding = 18,
    AwakeningSuccess = 19,
    AwakeningFail = 20,
    AwakeningMiss = 21,
    AwakeningHit = 22,
    StormEscape = 23,
    TurtleKing = 24,
    Behemoth = 25,
    Stunned = 26,
    IcePillar = 27,
    KingGuard = 28,
    KingGuard2 = 29,
    DeathCrawlerBreath = 30,
    FlamingMutantWeb = 31,
    FurbolgWarriorCritical = 32,
    Tester = 33,
    MoonMist = 34,
}

/// 伤害类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum DamageType {
    Hit = 0,
    Miss = 1,
    Critical = 2,
}

/// 网格类型（背包、装备等）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum MirGridType {
    None = 0,
    Inventory = 1,
    Equipment = 2,
    Trade = 3,
    Storage = 4,
    BuyBack = 5,
    DropPanel = 6,
    Inspect = 7,
    TrustMerchant = 8,
    GuildStorage = 9,
    GuestTrade = 10,
    Mount = 11,
    Fishing = 12,
    QuestInventory = 13,
    AwakenItem = 14,
    Mail = 15,
    Refine = 16,
    Renting = 17,
    GuestRenting = 18,
    Craft = 19,
    Socket = 20,
    HeroEquipment = 21,
    HeroInventory = 22,
    HeroHPItem = 23,
    HeroMPItem = 24,
}

/// 面板类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum PanelType {
    Buy = 0,
    BuySub = 1,
    Craft = 2,
    Sell = 3,
    Repair = 4,
    SpecialRepair = 5,
    Consign = 6,
    Refine = 7,
    CheckRefine = 8,
    Disassemble = 9,
    Downgrade = 10,
    Reset = 11,
    CollectRefine = 12,
    ReplaceWedRing = 13,
}

bitflags! {
    #[doc = "所需职业（Flags）"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct RequiredClass: u8 {
        const WARRIOR = 1;
        const WIZARD = 2;
        const TAOIST = 4;
        const ASSASSIN = 8;
        const ARCHER = 16;
        const WAR_WIZ_TAO = 1 | 2 | 4;
        const NONE = 1 | 2 | 4 | 8 | 16;
    }
}

bitflags! {
    #[doc = "所需性别（Flags）"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct RequiredGender: u8 {
        const MALE = 1;
        const FEMALE = 2;
        const NONE = Self::MALE.bits() | Self::FEMALE.bits();
    }
}

/// 所需类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum RequiredType {
    Level = 0,
    MaxAC = 1,
    MaxMAC = 2,
    MaxDC = 3,
    MaxMC = 4,
    MaxSC = 5,
    MaxLevel = 6,
    MinAC = 7,
    MinMAC = 8,
    MinDC = 9,
    MinMC = 10,
    MinSC = 11,
}

bitflags! {
    #[doc = "绑定模式（Flags）"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct BindMode: i16 {
        const NONE = 0;
        const DONT_DEATHDROP = 1;
        const DONT_DROP = 2;
        const DONT_SELL = 4;
        const DONT_STORE = 8;
        const DONT_TRADE = 16;
        const DONT_REPAIR = 32;
        const DONT_UPGRADE = 64;
        const DESTROY_ON_DROP = 128;
        const BREAK_ON_DEATH = 256;
        const BIND_ON_EQUIP = 512;
        const NO_S_REPAIR = 1024;
        const NO_WEDDING_RING = 2048;
        const UNABLE_TO_RENT = 4096;
        const UNABLE_TO_DISASSEMBLE = 8192;
        const NO_MAIL = 16384;
        const NO_HERO = i16::MIN;
    }
}

bitflags! {
    #[doc = "特殊物品模式（Flags）"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct SpecialItemMode: i16 {
        const NONE = 0;
        const PARALIZE = 0x0001;
        const TELEPORT = 0x0002;
        const CLEAR_RING = 0x0004;
        const PROTECTION = 0x0008;
        const REVIVAL = 0x0010;
        const MUSCLE = 0x0020;
        const FLAME = 0x0040;
        const HEALING = 0x0080;
        const PROBE = 0x0100;
        const SKILL = 0x0200;
        const NO_DURA_LOSS = 0x0400;
        const BLINK = 0x0800;
    }
}

bitflags! {
    #[doc = "等级特效（Flags）"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct LevelEffects: u16 {
        const NONE = 0;
        const MIST = 1;
        const RED_DRAGON = 2;
        const BLUE_DRAGON = 4;
        const REBIRTH1 = 8;
        const REBIRTH2 = 16;
        const REBIRTH3 = 32;
        const NEW_BLUE = 64;
        const YELLOW_DRAGON = 128;
        const PHOENIX = 256;
    }
}

bitflags! {
    #[doc = "天气设置（Flags）"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct WeatherSetting: u16 {
        const NONE = 0;
        const FOG = 1;
        const RED_EMBER = 2;
        const WHITE_EMBER = 4;
        const YELLOW_EMBER = 8;
        const FIRE_PARTICLE = 16;
        const SNOW = 32;
        const RAIN = 64;
        const LEAVES = 128;
        const FIREY_LEAVES = 256;
        const PURPLE_LEAVES = 512;
    }
}

/// 游戏阶段（从 MirConnection 逻辑推断）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum GameStage {
    None = 0,
    Login = 1,
    SelectChar = 2,
    Loading = 3,
    Game = 4,
}

/// 英雄行为
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum HeroBehaviour {
    Attack = 0,
    CounterAttack = 1,
    Follow = 2,
    Custom = 3,
}

/// 英雄生成状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum HeroSpawnState {
    None = 0,
    Unsummoned = 1,
    Summoned = 2,
    Dead = 3,
}

/// 征服类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum ConquestType {
    Request = 0,
    Auto = 1,
    Forced = 2,
}

/// 门状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum DoorState {
    Closed = 0,
    Opening = 1,
    Open = 2,
    Closing = 3,
}

bitflags! {
    #[doc = "行会等级权限（Flags）"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct GuildRankOptions: u8 {
        const CAN_CHANGE_RANK = 1;
        const CAN_RECRUIT = 2;
        const CAN_KICK = 4;
        const CAN_STORE_ITEM = 8;
        const CAN_RETRIEVE_ITEM = 16;
        const CAN_ALTER_ALLIANCE = 32;
        const CAN_CHANGE_NOTICE = 64;
        const CAN_ACTIVATE_BUFF = 128;
    }
}

/// 智能生物类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum IntelligentCreatureType {
    BabyPig = 0,
    Chick = 1,
    Kitten = 2,
    BabySkeleton = 3,
    Baekdon = 4,
    Wimaen = 5,
    BlackKitten = 6,
    BabyDragon = 7,
    OlympicFlame = 8,
    BabySnowMan = 9,
    Frog = 10,
    BabyMonkey = 11,
    AngryBird = 12,
    Foxey = 13,
    MedicalRat = 14,
    None = 99,
}

bitflags! {
    #[doc = "GM 选项（Flags）"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct GMOptions: u8 {
        const NONE = 0;
        const GAME_MASTER = 0x0001;
        const OBSERVER = 0x0002;
        const SUPERMAN = 0x0004;
    }
}

/// 坐骑槽位
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum MountSlot {
    Reins = 0,
    Bells = 1,
    Saddle = 2,
    Ribbon = 3,
    Mask = 4,
}

/// 钓鱼槽位
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum FishingSlot {
    Hook = 0,
    Float = 1,
    Bait = 2,
    Finder = 3,
    Reel = 4,
}

/// 物品套装
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum ItemSet {
    None = 0,
    Spirit = 1,
    Recall = 2,
    RedOrchid = 3,
    RedFlower = 4,
    Smash = 5,
    HwanDevil = 6,
    Purity = 7,
    FiveString = 8,
    Mundane = 9,
    NokChi = 10,
    TaoProtect = 11,
    Mir = 12,
    Bone = 13,
    Bug = 14,
    WhiteGold = 15,
    WhiteGoldH = 16,
    RedJade = 17,
    RedJadeH = 18,
    Nephrite = 19,
    NephriteH = 20,
    Whisker1 = 21,
    Whisker2 = 22,
    Whisker3 = 23,
    Whisker4 = 24,
    Whisker5 = 25,
    Hyeolryong = 26,
    Monitor = 27,
    Oppressive = 28,
    Paeok = 29,
    Sulgwan = 30,
    BlueFrost = 31,
    DarkGhost = 38,
    BlueFrostH = 39,
}

/// 物品品质
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum ItemGrade {
    None = 0,
    Common = 1,
    Rare = 2,
    Legendary = 3,
    Mythical = 4,
    Heroic = 5,
}

/// 任务类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum QuestType {
    General = 0,
    Daily = 1,
    Repeatable = 2,
    Story = 3,
}

/// 任务图标
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum QuestIcon {
    None = 0,
    QuestionWhite = 1,
    ExclamationYellow = 2,
    QuestionYellow = 3,
    ExclamationBlue = 5,
    QuestionBlue = 6,
    ExclamationGreen = 52,
    QuestionGreen = 53,
}

/// 任务状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum QuestState {
    Add = 0,
    Update = 1,
    Remove = 2,
}

/// 任务动作
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum QuestAction {
    TimeExpired = 0,
}

/// 默认 NPC 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum DefaultNPCType {
    Login = 0,
    LevelUp = 1,
    UseItem = 2,
    MapCoord = 3,
    MapEnter = 4,
    Die = 5,
    Trigger = 6,
    CustomCommand = 7,
    OnAcceptQuest = 8,
    OnFinishQuest = 9,
    Daily = 10,
    Client = 11,
}

/// 征服游戏模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum ConquestGame {
    CapturePalace = 0,
    KingOfHill = 1,
    Random = 2,
    Classic = 3,
    ControlPoints = 4,
}

/// 智能生物拾取模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum IntelligentCreaturePickupMode {
    Automatic = 0,
    SemiAutomatic = 1,
}

/// 技能开关状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = i8)]
#[bw(repr = i8)]
#[brw(little)]
#[repr(i8)]
pub enum SpellToggleState {
    None = -1,
    False = 0,
    True = 1,
}

/// 市场收集模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum MarketCollectionMode {
    Any = 0,
    Sold = 1,
    Expired = 2,
}

/// 鼠标光标
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum MouseCursor {
    None = 0,
    Default = 1,
    Attack = 2,
    AttackRed = 3,
    NPCTalk = 4,
    TextPrompt = 5,
    Trash = 6,
    Upgrade = 7,
}

/// 市场物品类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum MarketItemType {
    Consign = 0,
    Auction = 1,
    GameShop = 2,
}

/// 市场面板类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum MarketPanelType {
    Market = 0,
    Consign = 1,
    Auction = 2,
    GameShop = 3,
}

/// 市场价格过滤
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum MarketPriceFilter {
    Normal = 0,
    High = 1,
    Low = 2,
}

/// 混合模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = i8)]
#[bw(repr = i8)]
#[brw(little)]
#[repr(i8)]
pub enum BlendMode {
    NONE = -1,
    NORMAL = 0,
    LIGHT = 1,
    LIGHTINV = 2,
    INVNORMAL = 3,
    INVLIGHT = 4,
    INVLIGHTINV = 5,
    INVCOLOR = 6,
    INVBACKGROUND = 7,
}

/// 觉醒类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum AwakeType {
    None = 0,
    DC = 1,
    MC = 2,
    SC = 3,
    AC = 4,
    MAC = 5,
    HPMP = 6,
}

/// 输出消息类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum OutputMessageType {
    Normal = 0,
    Quest = 1,
    Guild = 2,
}

/// 精炼值类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[brw(little)]
#[repr(u8)]
pub enum RefinedValue {
    None = 0,
    DC = 1,
    MC = 2,
    SC = 3,
}
