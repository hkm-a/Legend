// ========================================
// 游戏世界数据类型定义
// ========================================

// ============ 基础类型 ============

/** 二维坐标 */
export interface Point {
  x: number;
  y: number;
}

/** 地图格子数据 */
export interface TileData {
  attr: number; // 0=Walk, 1=HighWall, 2=LowWall
  isSafeZone: boolean;
}

// ============ 玩家信息 ============

/** 玩家信息 */
export interface PlayerInfo {
  objectId: number;
  name: string;
  level: number;
  characterClass: number;
  gender: number;
  location: Point;
  direction: number;
  currentHp: number;
  maxHp: number;
}

// ============ 怪物信息 ============

/** 怪物信息 */
export interface MonsterInfo {
  objectId: number;
  templateId: number;
  name: string;
  level: number;
  location: Point;
  direction: number;
  currentHp: number;
  maxHp: number;
  isAlive: boolean;
}

// ============ 地上物品 ============

/** 地上物品 */
export interface GroundItemInfo {
  objectId: number;
  itemId: number;
  location: Point;
  name: string;
  image: number;
}

// ============ 背包物品 ============

/** 背包物品 */
export interface InventoryItem {
  uid: number;
  itemId: number;
  name: string;
  slot: number;
  count: number;
  itemType: string;
  image: number;
}

// ============ 聊天 ============

/** 聊天消息 */
export interface ChatMessage {
  id: number;
  text: string;
  sender: string;
  type: 'system' | 'chat' | 'whisper';
  timestamp: number;
}

// ============ 游戏世界状态 ============

/** 完整游戏世界状态 */
export interface GameWorldState {
  mapId: number;
  mapWidth: number;
  mapHeight: number;
  tiles: TileData[];
  player: PlayerInfo | null;
  monsters: Map<number, MonsterInfo>;
  groundItems: Map<number, GroundItemInfo>;
  otherPlayers: Map<number, PlayerInfo>;
  inventory: InventoryItem[];
  chatMessages: ChatMessage[];
  currentHp: number;
  maxHp: number;
  currentMp: number;
  maxMp: number;
  experience: number;
  maxExperience: number;
  level: number;
  gold: number;
  isInventoryOpen: boolean;
  isCharacterOpen: boolean;
  isSkillOpen: boolean;
  isSettingsOpen: boolean;
  isDead: boolean;
}
