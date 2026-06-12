// ========================================
// Mir2 核心枚举定义（TypeScript 版本）
// ========================================

/** 职业 */
export const enum MirClass {
  Warrior = 0,
  Wizard = 1,
  Taoist = 2,
  Assassin = 3,
  Archer = 4,
}

/** 性别 */
export const enum MirGender {
  Male = 0,
  Female = 1,
}

/** 方向（8 方向） */
export const enum MirDirection {
  Up = 0,
  UpRight = 1,
  Right = 2,
  DownRight = 3,
  Down = 4,
  DownLeft = 5,
  Left = 6,
  UpLeft = 7,
}

/** 对象类型 */
export const enum ObjectType {
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

/** 动作 */
export const enum MirAction {
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

/** 物品类型 */
export const enum ItemType {
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

/** 格子属性 */
export const enum CellAttribute {
  Walk = 0,
  HighWall = 1,
  LowWall = 2,
}

/** 光照设置 */
export const enum LightSetting {
  Normal = 0,
  Dawn = 1,
  Day = 2,
  Evening = 3,
  Night = 4,
}

/** 聊天类型 */
export const enum ChatType {
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

/** 攻击模式 */
export const enum AttackMode {
  Peace = 0,
  Group = 1,
  Guild = 2,
  EnemyGuild = 3,
  RedBrown = 4,
  All = 5,
}

/** 宠物模式 */
export const enum PetMode {
  Both = 0,
  MoveOnly = 1,
  AttackOnly = 2,
  None = 3,
  FocusMasterTarget = 4,
}

/** 毒类型（Flags） */
export const enum PoisonType {
  None = 0,
  Green = 1,
  Red = 2,
  Slow = 4,
  Frozen = 8,
  Stun = 16,
  Paralysis = 32,
  DelayedExplosion = 64,
  Bleeding = 128,
  LRParalysis = 256,
  Blindness = 512,
  Dazed = 1024,
}

/** 网格类型 */
export const enum MirGridType {
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

/** 装备槽位 */
export const enum EquipmentSlot {
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

/** 技能 */
export const enum Spell {
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

/** 伤害类型 */
export const enum DamageType {
  Hit = 0,
  Miss = 1,
  Critical = 2,
}

/** 怪物稀有度 */
export const enum MonsterType {
  Normal = 0,
  Uncommon = 1,
  Rare = 2,
  Elite = 3,
}

/** 防御类型 */
export const enum DefenceType {
  ACAgility = 0,
  AC = 1,
  MACAgility = 2,
  MAC = 3,
  Agility = 4,
  Repulsion = 5,
  None = 6,
}

/** 游戏阶段 */
export const enum GameStage {
  None = 0,
  Login = 1,
  SelectChar = 2,
  Loading = 3,
  Game = 4,
}

/** 英雄行为 */
export const enum HeroBehaviour {
  Attack = 0,
  CounterAttack = 1,
  Follow = 2,
  Custom = 3,
}

/** 英雄生成状态 */
export const enum HeroSpawnState {
  None = 0,
  Unsummoned = 1,
  Summoned = 2,
  Dead = 3,
}

/** 门状态 */
export const enum DoorState {
  Closed = 0,
  Opening = 1,
  Open = 2,
  Closing = 3,
}

/** 物品品质 */
export const enum ItemGrade {
  None = 0,
  Common = 1,
  Rare = 2,
  Legendary = 3,
  Mythical = 4,
  Heroic = 5,
}
