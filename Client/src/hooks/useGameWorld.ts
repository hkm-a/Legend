import { useReducer, useEffect, useCallback, useRef } from 'react';
import { useConnection } from './useConnection';
import { ServerPacketIds } from '../types/packets';
import { parseServerPacket } from '../network/packets/server_packets';
import {
  WalkPacket,
  RunPacket,
  AttackPacket,
  ChatPacket,
  TurnPacket,
  PickUpClientPacket,
  UseItemClientPacket,
  DropItemClientPacket,
  MoveItemClientPacket,
} from '../network/packets/client_packets';
import {
  GameWorldState,
  PlayerInfo,
  MonsterInfo,
  GroundItemInfo,
  InventoryItem,
  ChatMessage,
  TileData,
} from '../types/game';

// ========================================
// Action 类型定义
// ========================================

type GameWorldAction =
  | { type: 'LOAD_MAP'; mapId: number; width: number; height: number; tiles: number[] }
  | { type: 'SET_PLAYER'; player: PlayerInfo }
  | { type: 'UPDATE_PLAYER_LOCATION'; location: { x: number; y: number }; direction: number }
  | { type: 'UPDATE_PLAYER_HP'; hp: number; mp: number }
  | { type: 'SET_PLAYER_HP_MP'; hp: number; maxHp: number; mp: number; maxMp: number }
  | { type: 'ADD_MONSTER'; monster: MonsterInfo }
  | { type: 'UPDATE_MONSTER_LOCATION'; objectId: number; location: { x: number; y: number }; direction: number }
  | { type: 'REMOVE_MONSTER'; objectId: number }
  | { type: 'UPDATE_MONSTER_HP'; objectId: number; hp: number }
  | { type: 'ADD_OTHER_PLAYER'; player: PlayerInfo }
  | { type: 'REMOVE_OTHER_PLAYER'; objectId: number }
  | { type: 'UPDATE_OTHER_PLAYER_LOCATION'; objectId: number; location: { x: number; y: number }; direction: number }
  | { type: 'ADD_GROUND_ITEM'; item: GroundItemInfo }
  | { type: 'REMOVE_GROUND_ITEM'; objectId: number }
  | { type: 'ADD_INVENTORY_ITEM'; item: InventoryItem }
  | { type: 'REMOVE_INVENTORY_ITEM'; uid: number }
  | { type: 'ADD_CHAT_MESSAGE'; message: ChatMessage }
  | { type: 'SET_EXPERIENCE'; experience: number }
  | { type: 'SET_LEVEL'; level: number }
  | { type: 'SET_MAX_EXPERIENCE'; maxExperience: number }
  | { type: 'SET_GOLD'; gold: number }
  | { type: 'TOGGLE_INVENTORY' }
  | { type: 'TOGGLE_CHARACTER' }
  | { type: 'TOGGLE_SKILL' }
  | { type: 'TOGGLE_SETTINGS' }
  | { type: 'CLOSE_ALL_PANELS' }
  | { type: 'PLAYER_DEATH' }
  | { type: 'PLAYER_RESPAWN' };

// ========================================
// 初始状态
// ========================================

const initialState: GameWorldState = {
  mapId: 0,
  mapWidth: 0,
  mapHeight: 0,
  tiles: [],
  player: null,
  monsters: new Map<number, MonsterInfo>(),
  groundItems: new Map<number, GroundItemInfo>(),
  otherPlayers: new Map<number, PlayerInfo>(),
  inventory: [],
  chatMessages: [],
  currentHp: 0,
  maxHp: 0,
  currentMp: 0,
  maxMp: 0,
  experience: 0,
  maxExperience: 0,
  level: 0,
  gold: 0,
  isInventoryOpen: false,
  isCharacterOpen: false,
  isSkillOpen: false,
  isSettingsOpen: false,
  isDead: false,
};

// ========================================
// Reducer
// ========================================

let nextChatId = 0;

function gameWorldReducer(state: GameWorldState, action: GameWorldAction): GameWorldState {
  switch (action.type) {
    case 'LOAD_MAP': {
      const tiles: TileData[] = action.tiles.map((attr) => ({
        attr,
        isSafeZone: attr === 0,
      }));
      return {
        ...state,
        mapId: action.mapId,
        mapWidth: action.width,
        mapHeight: action.height,
        tiles,
      };
    }

    case 'SET_PLAYER':
      return {
        ...state,
        player: action.player,
        currentHp: action.player.currentHp,
        maxHp: action.player.maxHp,
        level: action.player.level,
      };

    case 'UPDATE_PLAYER_LOCATION': {
      if (!state.player) return state;
      return {
        ...state,
        player: {
          ...state.player,
          location: action.location,
          direction: action.direction,
        },
      };
    }

    case 'UPDATE_PLAYER_HP':
      return {
        ...state,
        currentHp: action.hp,
        currentMp: action.mp,
      };

    case 'SET_PLAYER_HP_MP':
      return {
        ...state,
        currentHp: action.hp,
        maxHp: action.maxHp,
        currentMp: action.mp,
        maxMp: action.maxMp,
      };

    case 'ADD_MONSTER': {
      const newMonsters = new Map(state.monsters);
      newMonsters.set(action.monster.objectId, action.monster);
      return { ...state, monsters: newMonsters };
    }

    case 'UPDATE_MONSTER_LOCATION': {
      const monster = state.monsters.get(action.objectId);
      if (!monster) return state;
      const newMonsters = new Map(state.monsters);
      newMonsters.set(action.objectId, {
        ...monster,
        location: action.location,
        direction: action.direction,
      });
      return { ...state, monsters: newMonsters };
    }

    case 'REMOVE_MONSTER': {
      const newMonsters = new Map(state.monsters);
      newMonsters.delete(action.objectId);
      return { ...state, monsters: newMonsters };
    }

    case 'UPDATE_MONSTER_HP': {
      const monster = state.monsters.get(action.objectId);
      if (!monster) return state;
      const newMonsters = new Map(state.monsters);
      newMonsters.set(action.objectId, { ...monster, currentHp: action.hp });
      return { ...state, monsters: newMonsters };
    }

    case 'ADD_OTHER_PLAYER': {
      const newPlayers = new Map(state.otherPlayers);
      newPlayers.set(action.player.objectId, action.player);
      return { ...state, otherPlayers: newPlayers };
    }

    case 'REMOVE_OTHER_PLAYER': {
      const newPlayers = new Map(state.otherPlayers);
      newPlayers.delete(action.objectId);
      return { ...state, otherPlayers: newPlayers };
    }

    case 'UPDATE_OTHER_PLAYER_LOCATION': {
      const player = state.otherPlayers.get(action.objectId);
      if (!player) return state;
      const newPlayers = new Map(state.otherPlayers);
      newPlayers.set(action.objectId, {
        ...player,
        location: action.location,
        direction: action.direction,
      });
      return { ...state, otherPlayers: newPlayers };
    }

    case 'ADD_GROUND_ITEM': {
      const newItems = new Map(state.groundItems);
      newItems.set(action.item.objectId, action.item);
      return { ...state, groundItems: newItems };
    }

    case 'REMOVE_GROUND_ITEM': {
      const newItems = new Map(state.groundItems);
      newItems.delete(action.objectId);
      return { ...state, groundItems: newItems };
    }

    case 'ADD_INVENTORY_ITEM':
      return { ...state, inventory: [...state.inventory, action.item] };

    case 'REMOVE_INVENTORY_ITEM':
      return { ...state, inventory: state.inventory.filter((i) => i.uid !== action.uid) };

    case 'ADD_CHAT_MESSAGE':
      return {
        ...state,
        chatMessages: [...state.chatMessages, action.message].slice(-50),
      };

    case 'SET_EXPERIENCE':
      return { ...state, experience: action.experience };

    case 'SET_LEVEL':
      return { ...state, level: action.level };

    case 'SET_MAX_EXPERIENCE':
      return { ...state, maxExperience: action.maxExperience };

    case 'SET_GOLD':
      return { ...state, gold: action.gold };

    case 'TOGGLE_INVENTORY':
      return {
        ...state,
        isInventoryOpen: !state.isInventoryOpen,
        isCharacterOpen: false,
        isSkillOpen: false,
        isSettingsOpen: false,
      };

    case 'TOGGLE_CHARACTER':
      return {
        ...state,
        isCharacterOpen: !state.isCharacterOpen,
        isInventoryOpen: false,
        isSkillOpen: false,
        isSettingsOpen: false,
      };

    case 'TOGGLE_SKILL':
      return {
        ...state,
        isSkillOpen: !state.isSkillOpen,
        isInventoryOpen: false,
        isCharacterOpen: false,
        isSettingsOpen: false,
      };

    case 'TOGGLE_SETTINGS':
      return {
        ...state,
        isSettingsOpen: !state.isSettingsOpen,
        isInventoryOpen: false,
        isCharacterOpen: false,
        isSkillOpen: false,
      };

    case 'CLOSE_ALL_PANELS':
      return {
        ...state,
        isInventoryOpen: false,
        isCharacterOpen: false,
        isSkillOpen: false,
        isSettingsOpen: false,
      };

    case 'PLAYER_DEATH':
      return { ...state, isDead: true };

    case 'PLAYER_RESPAWN':
      return { ...state, isDead: false };

    default:
      return state;
  }
}

// ========================================
// useGameWorld Hook
// ========================================

export interface UseGameWorldReturn {
  state: GameWorldState;
  walk: (direction: number) => void;
  run: (direction: number) => void;
  attack: (direction: number, spell?: number) => void;
  turn: (direction: number) => void;
  pickUp: (objectId: number) => void;
  useItem: (uid: number) => void;
  dropItem: (uid: number, count: number) => void;
  moveItem: (fromSlot: number, toSlot: number) => void;
  chat: (text: string) => void;
  toggleInventory: () => void;
  toggleCharacter: () => void;
  toggleSkill: () => void;
  toggleSettings: () => void;
  closeAllPanels: () => void;
  respawn: () => void;
  respawnTown: () => void;
}

/**
 * 核心游戏世界状态管理 Hook
 *
 * 管理 GameWorldState 的变更，处理所有服务端游戏包，
 * 并提供客户端操作包装方法。
 */
export function useGameWorld(
  connection: ReturnType<typeof useConnection>,
): UseGameWorldReturn {
  const [state, dispatch] = useReducer(gameWorldReducer, initialState);
  const { send, onServerPacket, clearServerPacket } = connection;

  // 使用 ref 避免 stale closure
  const stateRef = useRef(state);
  stateRef.current = state;

  // ---- 服务端包监听 ----
  useEffect(() => {
    // 地图加载
    const onMapChanged = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.MapChanged, payload);
      if (result) {
        dispatch({
          type: 'LOAD_MAP',
          mapId: result.map_id,
          width: result.width,
          height: result.height,
          tiles: result.tiles,
        });
      }
    };

    // 玩家完整信息
    const onUserInformation = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.UserInformation, payload);
      if (result) {
        const player: PlayerInfo = {
          objectId: result.object_id,
          name: result.name,
          level: result.level,
          characterClass: result.char_class,
          gender: result.gender,
          location: result.location,
          direction: result.direction,
          currentHp: result.hp,
          maxHp: result.max_hp,
        };
        dispatch({ type: 'SET_PLAYER', player });
        dispatch({ type: 'SET_EXPERIENCE', experience: result.experience });
        dispatch({ type: 'SET_MAX_EXPERIENCE', maxExperience: result.max_experience });
        dispatch({ type: 'SET_LEVEL', level: result.level });
        dispatch({ type: 'SET_GOLD', gold: result.gold });
        dispatch({
          type: 'SET_PLAYER_HP_MP',
          hp: result.hp,
          maxHp: result.max_hp,
          mp: result.mp,
          maxMp: result.max_mp,
        });
      }
    };

    // 玩家位置更新
    const onUserLocation = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.UserLocation, payload);
      if (result && stateRef.current.player) {
        dispatch({
          type: 'UPDATE_PLAYER_LOCATION',
          location: { x: result.location.x, y: result.location.y },
          direction: result.direction,
        });
      }
    };

    // 怪物出现
    const onObjectMonster = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.ObjectMonster, payload);
      if (result) {
        const monster: MonsterInfo = {
          objectId: result.object_id,
          templateId: result.template_id,
          name: result.name,
          level: result.level,
          location: result.location,
          direction: result.direction,
          currentHp: result.hp,
          maxHp: result.max_hp,
          isAlive: result.is_alive,
        };
        dispatch({ type: 'ADD_MONSTER', monster });
      }
    };

    // 怪物/对象移动
    const onObjectWalk = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.ObjectWalk, payload);
      if (result) {
        dispatch({
          type: 'UPDATE_MONSTER_LOCATION',
          objectId: result.object_id,
          location: result.location,
          direction: result.direction,
        });
        // Also try other players
        dispatch({
          type: 'UPDATE_OTHER_PLAYER_LOCATION',
          objectId: result.object_id,
          location: result.location,
          direction: result.direction,
        });
      }
    };

    // 其他玩家出现
    const onObjectPlayer = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.ObjectPlayer, payload);
      if (result) {
        const otherPlayer: PlayerInfo = {
          objectId: result.object_id,
          name: result.name,
          level: 0,
          characterClass: result.class,
          gender: result.gender,
          location: result.location,
          direction: result.direction,
          currentHp: 0,
          maxHp: 0,
        };
        dispatch({ type: 'ADD_OTHER_PLAYER', player: otherPlayer });
      }
    };

    // 对象移除
    const onObjectRemove = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.ObjectRemove, payload);
      if (result) {
        dispatch({ type: 'REMOVE_OTHER_PLAYER', objectId: result.object_id });
        dispatch({ type: 'REMOVE_MONSTER', objectId: result.object_id });
        dispatch({ type: 'REMOVE_GROUND_ITEM', objectId: result.object_id });
      }
    };

    // 对象攻击
    const onObjectAttack = (payload: ArrayBuffer) => {
      parseServerPacket(ServerPacketIds.ObjectAttack, payload);
      // Attack is an animation trigger — state updates are visual-only
      // We don't store attack state, but the canvas reads it
    };

    // 伤害飘字
    const onDamageIndicator = (payload: ArrayBuffer) => {
      parseServerPacket(ServerPacketIds.DamageIndicator, payload);
      // DamageIndicator is a visual effect — the canvas handles it
    };

    // 自身受击
    const onStruck = (payload: ArrayBuffer) => {
      parseServerPacket(ServerPacketIds.Struck, payload);
      // Visual effect handled by canvas
    };

    // 对象受击
    const onObjectStruck = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.ObjectStruck, payload);
      if (result) {
        dispatch({
          type: 'UPDATE_MONSTER_HP',
          objectId: result.object_id,
          hp: Math.max(0, (stateRef.current.monsters.get(result.object_id)?.currentHp ?? 0) - result.damage),
        });
      }
    };

    // 血量变化
    const onHealthChanged = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.HealthChanged, payload);
      if (result) {
        dispatch({ type: 'UPDATE_PLAYER_HP', hp: result.hp, mp: result.mp });
      }
    };

    // 自身死亡
    const onDeath = (_payload: ArrayBuffer) => {
      dispatch({ type: 'PLAYER_DEATH' });
    };

    // 对象死亡
    const onObjectDied = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.ObjectDied, payload);
      if (result) {
        dispatch({ type: 'REMOVE_MONSTER', objectId: result.object_id });
        dispatch({ type: 'REMOVE_OTHER_PLAYER', objectId: result.object_id });
      }
    };

    // 获得经验
    const onGainExperience = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.GainExperience, payload);
      if (result) {
        const currentState = stateRef.current;
        dispatch({
          type: 'SET_EXPERIENCE',
          experience: currentState.experience + result.amount,
        });
      }
    };

    // 等级变化
    const onLevelChanged = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.LevelChanged, payload);
      if (result) {
        dispatch({ type: 'SET_LEVEL', level: result.level });
      }
    };

    // 地上物品
    const onObjectItem = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.ObjectItem, payload);
      if (result) {
        const item: GroundItemInfo = {
          objectId: result.object_id,
          itemId: result.item_id,
          location: result.location,
          name: result.name,
          image: result.image,
        };
        dispatch({ type: 'ADD_GROUND_ITEM', item });
      }
    };

    // 获得物品
    const onGainedItem = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.GainedItem, payload);
      if (result) {
        const invItem: InventoryItem = {
          uid: Date.now(),
          itemId: result.item_id,
          name: result.name,
          slot: stateRef.current.inventory.length,
          count: result.count,
          itemType: result.item_type,
          image: result.image,
        };
        dispatch({ type: 'ADD_INVENTORY_ITEM', item: invItem });
      }
    };

    // 传送进入
    const onObjectTeleportIn = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.ObjectTeleportIn, payload);
      if (result) {
        dispatch({
          type: 'UPDATE_MONSTER_LOCATION',
          objectId: result.object_id,
          location: result.location,
          direction: result.direction,
        });
        dispatch({
          type: 'UPDATE_OTHER_PLAYER_LOCATION',
          objectId: result.object_id,
          location: result.location,
          direction: result.direction,
        });
      }
    };

    // 聊天消息
    const onChat = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.Chat, payload);
      if (result) {
        const msg: ChatMessage = {
          id: nextChatId++,
          text: result.message,
          sender: '',
          type: result.chat_type === 2 ? 'system' : result.chat_type === 6 ? 'whisper' : 'chat',
          timestamp: Date.now(),
        };
        dispatch({ type: 'ADD_CHAT_MESSAGE', message: msg });
      }
    };

    // 注册监听
    onServerPacket(ServerPacketIds.MapChanged, onMapChanged);
    onServerPacket(ServerPacketIds.UserInformation, onUserInformation);
    onServerPacket(ServerPacketIds.UserLocation, onUserLocation);
    onServerPacket(ServerPacketIds.ObjectMonster, onObjectMonster);
    onServerPacket(ServerPacketIds.ObjectWalk, onObjectWalk);
    onServerPacket(ServerPacketIds.ObjectPlayer, onObjectPlayer);
    onServerPacket(ServerPacketIds.ObjectRemove, onObjectRemove);
    onServerPacket(ServerPacketIds.ObjectAttack, onObjectAttack);
    onServerPacket(ServerPacketIds.DamageIndicator, onDamageIndicator);
    onServerPacket(ServerPacketIds.Struck, onStruck);
    onServerPacket(ServerPacketIds.ObjectStruck, onObjectStruck);
    onServerPacket(ServerPacketIds.HealthChanged, onHealthChanged);
    onServerPacket(ServerPacketIds.Death, onDeath);
    onServerPacket(ServerPacketIds.ObjectDied, onObjectDied);
    onServerPacket(ServerPacketIds.GainExperience, onGainExperience);
    onServerPacket(ServerPacketIds.LevelChanged, onLevelChanged);
    onServerPacket(ServerPacketIds.ObjectItem, onObjectItem);
    onServerPacket(ServerPacketIds.GainedItem, onGainedItem);
    onServerPacket(ServerPacketIds.ObjectTeleportIn, onObjectTeleportIn);
    onServerPacket(ServerPacketIds.Chat, onChat);

    return () => {
      clearServerPacket(ServerPacketIds.MapChanged, onMapChanged);
      clearServerPacket(ServerPacketIds.UserInformation, onUserInformation);
      clearServerPacket(ServerPacketIds.UserLocation, onUserLocation);
      clearServerPacket(ServerPacketIds.ObjectMonster, onObjectMonster);
      clearServerPacket(ServerPacketIds.ObjectWalk, onObjectWalk);
      clearServerPacket(ServerPacketIds.ObjectPlayer, onObjectPlayer);
      clearServerPacket(ServerPacketIds.ObjectRemove, onObjectRemove);
      clearServerPacket(ServerPacketIds.ObjectAttack, onObjectAttack);
      clearServerPacket(ServerPacketIds.DamageIndicator, onDamageIndicator);
      clearServerPacket(ServerPacketIds.Struck, onStruck);
      clearServerPacket(ServerPacketIds.ObjectStruck, onObjectStruck);
      clearServerPacket(ServerPacketIds.HealthChanged, onHealthChanged);
      clearServerPacket(ServerPacketIds.Death, onDeath);
      clearServerPacket(ServerPacketIds.ObjectDied, onObjectDied);
      clearServerPacket(ServerPacketIds.GainExperience, onGainExperience);
      clearServerPacket(ServerPacketIds.LevelChanged, onLevelChanged);
      clearServerPacket(ServerPacketIds.ObjectItem, onObjectItem);
      clearServerPacket(ServerPacketIds.GainedItem, onGainedItem);
      clearServerPacket(ServerPacketIds.ObjectTeleportIn, onObjectTeleportIn);
      clearServerPacket(ServerPacketIds.Chat, onChat);
    };
  }, [onServerPacket, clearServerPacket]);

  // ---- 客户端操作包装 ----

  const walk = useCallback(
    (direction: number) => {
      send(new WalkPacket(direction));
    },
    [send],
  );

  const run = useCallback(
    (direction: number) => {
      send(new RunPacket(direction));
    },
    [send],
  );

  const attack = useCallback(
    (direction: number, spell: number = 0) => {
      send(new AttackPacket(direction, spell));
    },
    [send],
  );

  const turn = useCallback(
    (direction: number) => {
      send(new TurnPacket(direction));
    },
    [send],
  );

  const pickUp = useCallback(
    (objectId: number) => {
      send(new PickUpClientPacket(objectId));
    },
    [send],
  );

  const useItem = useCallback(
    (uid: number) => {
      send(new UseItemClientPacket(uid));
    },
    [send],
  );

  const dropItem = useCallback(
    (uid: number, count: number) => {
      send(new DropItemClientPacket(uid, count));
    },
    [send],
  );

  const moveItem = useCallback(
    (fromSlot: number, toSlot: number) => {
      send(new MoveItemClientPacket(fromSlot, toSlot));
    },
    [send],
  );

  const chat = useCallback(
    (text: string) => {
      send(new ChatPacket(text));
    },
    [send],
  );

  const toggleInventory = useCallback(() => dispatch({ type: 'TOGGLE_INVENTORY' }), []);
  const toggleCharacter = useCallback(() => dispatch({ type: 'TOGGLE_CHARACTER' }), []);
  const toggleSkill = useCallback(() => dispatch({ type: 'TOGGLE_SKILL' }), []);
  const toggleSettings = useCallback(() => dispatch({ type: 'TOGGLE_SETTINGS' }), []);
  const closeAllPanels = useCallback(() => dispatch({ type: 'CLOSE_ALL_PANELS' }), []);
  const respawn = useCallback(() => dispatch({ type: 'PLAYER_RESPAWN' }), []);
  const respawnTown = useCallback(() => dispatch({ type: 'PLAYER_RESPAWN' }), []);

  return {
    state,
    walk,
    run,
    attack,
    turn,
    pickUp,
    useItem,
    dropItem,
    moveItem,
    chat,
    toggleInventory,
    toggleCharacter,
    toggleSkill,
    toggleSettings,
    closeAllPanels,
    respawn,
    respawnTown,
  };
}
