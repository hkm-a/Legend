import { useState, useCallback, useEffect, useRef } from 'react';
import { useConnection } from './useConnection';
import { parseServerPacket } from '../network/packets/server_packets';
import {
  LoginPacket,
  NewAccountPacket,
  ChangePasswordPacket,
  NewCharacterPacket,
  DeleteCharacterPacket,
  StartGamePacket,
} from '../network/packets/client_packets';
import { ServerPacketIds } from '../types/packets';

/** 游戏阶段 */
export type GameStage = 'none' | 'login' | 'select_char' | 'loading' | 'game';

/** 客户端角色信息 */
export interface CharacterInfo {
  index: number;
  name: string;
  class: number;
  gender: number;
  level: number;
  hp: number;
  mp: number;
  max_hp: number;
  max_mp: number;
}

/** 游戏状态 */
export interface GameState {
  stage: GameStage;
  accountId: number | null;
  username: string;
  characters: CharacterInfo[];
  error: string | null;
}

/** useGameState 返回值 */
export interface UseGameStateReturn {
  state: GameState;
  requestLogin: (username: string, password: string) => void;
  requestRegister: (username: string, password: string) => void;
  requestChangePassword: (oldPassword: string, newPassword: string) => void;
  requestNewCharacter: (name: string, charClass: number, gender: number) => void;
  requestDeleteCharacter: (charIndex: number) => void;
  requestStartGame: (charIndex: number) => void;
  clearError: () => void;
  resetToLogin: () => void;
}

/**
 * 游戏状态管理 Hook
 *
 * 管理游戏阶段切换（login → select_char → loading → game）
 * 封装与认证/角色管理相关的网络包收发。
 */
export function useGameState(): UseGameStateReturn {
  const { state: connectionState, send, onServerPacket, clearServerPacket } = useConnection();

  const [state, setState] = useState<GameState>({
    stage: 'none',
    accountId: null,
    username: '',
    characters: [],
    error: null,
  });

  // 使用 ref 存储当前 characters 以便在回调中访问最新值
  const charactersRef = useRef<CharacterInfo[]>([]);
  charactersRef.current = state.characters;

  // 清空错误
  const clearError = useCallback(() => {
    setState((prev) => ({ ...prev, error: null }));
  }, []);

  // 重置到登录页
  const resetToLogin = useCallback(() => {
    setState({
      stage: 'login',
      accountId: null,
      username: '',
      characters: [],
      error: null,
    });
  }, []);

  // 切换到 login 阶段（当 WebSocket 连接成功后）
  useEffect(() => {
    if (connectionState === 'connected' && state.stage === 'none') {
      setState((prev) => ({ ...prev, stage: 'login' }));
    }
    if (connectionState === 'disconnected') {
      setState({
        stage: 'none',
        accountId: null,
        username: '',
        characters: [],
        error: null,
      });
    }
  }, [connectionState, state.stage]);

  // ---- 登录 ----
  const requestLogin = useCallback(
    (username: string, password: string) => {
      setState((prev) => ({ ...prev, error: null, username }));
      const packet = new LoginPacket(username, password);
      send(packet);
    },
    [send],
  );

  // ---- 注册 ----
  const requestRegister = useCallback(
    (username: string, password: string) => {
      setState((prev) => ({ ...prev, error: null }));
      const packet = new NewAccountPacket(username, password);
      send(packet);
    },
    [send],
  );

  // ---- 修改密码 ----
  const requestChangePassword = useCallback(
    (oldPassword: string, newPassword: string) => {
      setState((prev) => ({ ...prev, error: null }));
      const packet = new ChangePasswordPacket(oldPassword, newPassword);
      send(packet);
    },
    [send],
  );

  // ---- 创建角色 ----
  const requestNewCharacter = useCallback(
    (name: string, charClass: number, gender: number) => {
      setState((prev) => ({ ...prev, error: null }));
      const packet = new NewCharacterPacket(name, charClass, gender);
      send(packet);
    },
    [send],
  );

  // ---- 删除角色 ----
  const requestDeleteCharacter = useCallback(
    (charIndex: number) => {
      setState((prev) => ({ ...prev, error: null }));
      const packet = new DeleteCharacterPacket(charIndex);
      send(packet);
    },
    [send],
  );

  // ---- 开始游戏 ----
  const requestStartGame = useCallback(
    (charIndex: number) => {
      setState((prev) => ({ ...prev, error: null, stage: 'loading' }));
      const packet = new StartGamePacket(charIndex);
      send(packet);
    },
    [send],
  );

  // ---- 注册服务端包监听 ----
  useEffect(() => {
    // 登录成功
    const onLoginSuccess = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.LoginSuccess, payload);
      if (result) {
        setState((prev) => ({
          ...prev,
          stage: 'select_char',
          accountId: result.account_id,
          characters: [], // 清空角色列表，等待后续的 NewCharacterSuccess 包
          error: null,
        }));
      }
    };

    // 登录失败
    const onLoginResult = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.Login, payload);
      if (result && result.result !== 0) {
        const messages: Record<number, string> = {
          1: '登录失败，请重试',
          2: '账号不存在',
          3: '密码错误',
        };
        setState((prev) => ({
          ...prev,
          error: messages[result.result] || '登录失败',
        }));
      }
    };

    // 登录被封禁
    const onLoginBanned = (_payload: ArrayBuffer) => {
      setState((prev) => ({
        ...prev,
        error: '账号已被封禁',
      }));
    };

    // 注册结果
    const onNewAccountResult = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.NewAccount, payload);
      if (result) {
        if (result.result === 0) {
          setState((prev) => ({ ...prev, error: null }));
          alert('账号注册成功，请登录');
        } else {
          setState((prev) => ({ ...prev, error: '注册失败，用户名可能已存在' }));
        }
      }
    };

    // 修改密码结果
    const onChangePasswordResult = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.ChangePassword, payload);
      if (result) {
        if (result.result === 0) {
          alert('密码修改成功');
          setState((prev) => ({ ...prev, error: null }));
        } else {
          setState((prev) => ({ ...prev, error: '密码修改失败' }));
        }
      }
    };

    // 创建角色成功
    const onNewCharacterSuccess = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.NewCharacterSuccess, payload);
      if (result && result.char_info) {
        const newChar: CharacterInfo = {
          index: result.char_info.index,
          name: result.char_info.name,
          class: result.char_info.class,
          gender: result.char_info.gender,
          level: result.char_info.level,
          hp: result.char_info.hp,
          mp: result.char_info.mp,
          max_hp: result.char_info.max_hp,
          max_mp: result.char_info.max_mp,
        };
        setState((prev) => ({
          ...prev,
          characters: [...prev.characters, newChar],
          error: null,
        }));
      }
    };

    // 创建角色失败
    const onNewCharacterResult = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.NewCharacter, payload);
      if (result && result.result !== 0) {
        const messages: Record<number, string> = {
          1: '创建角色失败',
          4: '角色数量已达上限（最多4个）',
          5: '角色名已存在',
        };
        setState((prev) => ({
          ...prev,
          error: messages[result.result] || '创建角色失败',
        }));
      }
    };

    // 删除角色成功
    const onDeleteCharacterSuccess = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.DeleteCharacterSuccess, payload);
      if (result) {
        const updatedChars = charactersRef.current.filter(
          (c) => c.index !== result.char_index,
        );
        setState((prev) => ({
          ...prev,
          characters: updatedChars,
          error: null,
        }));
      }
    };

    // 删除角色失败
    const onDeleteCharacterResult = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.DeleteCharacter, payload);
      if (result && result.result !== 0) {
        setState((prev) => ({ ...prev, error: '删除角色失败' }));
      }
    };

    // 开始游戏结果
    const onStartGameResult = (payload: ArrayBuffer) => {
      const result = parseServerPacket(ServerPacketIds.StartGame, payload);
      if (result) {
        if (result.result === 0) {
          setState((prev) => ({ ...prev, stage: 'game', error: null }));
        } else {
          setState((prev) => ({ ...prev, stage: 'select_char', error: '进入游戏失败' }));
        }
      }
    };

    onServerPacket(ServerPacketIds.LoginSuccess, onLoginSuccess);
    onServerPacket(ServerPacketIds.Login, onLoginResult);
    onServerPacket(ServerPacketIds.LoginBanned, onLoginBanned);
    onServerPacket(ServerPacketIds.NewAccount, onNewAccountResult);
    onServerPacket(ServerPacketIds.ChangePassword, onChangePasswordResult);
    onServerPacket(ServerPacketIds.NewCharacterSuccess, onNewCharacterSuccess);
    onServerPacket(ServerPacketIds.NewCharacter, onNewCharacterResult);
    onServerPacket(ServerPacketIds.DeleteCharacterSuccess, onDeleteCharacterSuccess);
    onServerPacket(ServerPacketIds.DeleteCharacter, onDeleteCharacterResult);
    onServerPacket(ServerPacketIds.StartGame, onStartGameResult);

    return () => {
      clearServerPacket(ServerPacketIds.LoginSuccess, onLoginSuccess);
      clearServerPacket(ServerPacketIds.Login, onLoginResult);
      clearServerPacket(ServerPacketIds.LoginBanned, onLoginBanned);
      clearServerPacket(ServerPacketIds.NewAccount, onNewAccountResult);
      clearServerPacket(ServerPacketIds.ChangePassword, onChangePasswordResult);
      clearServerPacket(ServerPacketIds.NewCharacterSuccess, onNewCharacterSuccess);
      clearServerPacket(ServerPacketIds.NewCharacter, onNewCharacterResult);
      clearServerPacket(ServerPacketIds.DeleteCharacterSuccess, onDeleteCharacterSuccess);
      clearServerPacket(ServerPacketIds.DeleteCharacter, onDeleteCharacterResult);
      clearServerPacket(ServerPacketIds.StartGame, onStartGameResult);
    };
  }, [onServerPacket, clearServerPacket, send]);

  return {
    state,
    requestLogin,
    requestRegister,
    requestChangePassword,
    requestNewCharacter,
    requestDeleteCharacter,
    requestStartGame,
    clearError,
    resetToLogin,
  };
}
