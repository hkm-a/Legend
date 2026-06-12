import { useState, useEffect, useCallback } from 'react';
import { connectionManager } from '../network/connection';
import { ConnectionState, ConnectionEventHandler } from '../network/types';
import { ClientPacket } from '../network/packets/client_packets';
import { ServerPacketIds } from '../types/packets';

/**
 * React Hook — 封装 ConnectionManager 单例，提供响应式连接状态管理。
 *
 * 返回值：
 * - state: 当前连接状态（响应式）
 * - connect(url): 连接到指定 WS 地址
 * - disconnect(): 断开连接
 * - send(packet): 发送客户端包
 * - onServerPacket(id, handler): 监听特定服务端包
 * - clearServerPacket(id, handler): 取消监听
 *
 * 注意：组件卸载时不会自动断开连接，后续组件可继续使用同一连接。
 */
export function useConnection() {
  const [state, setState] = useState<ConnectionState>(connectionManager.getState());

  useEffect(() => {
    const onStateChange = (newState: ConnectionState) => {
      setState(newState);
    };
    const onConnected = () => setState(ConnectionState.Connected);
    const onDisconnected = () => setState(ConnectionState.Disconnected);
    const onReconnecting = () => setState(ConnectionState.Reconnecting);

    connectionManager.on('state_change', onStateChange);
    connectionManager.on('connected', onConnected);
    connectionManager.on('disconnected', onDisconnected);
    connectionManager.on('reconnecting', onReconnecting);

    return () => {
      connectionManager.off('state_change', onStateChange);
      connectionManager.off('connected', onConnected);
      connectionManager.off('disconnected', onDisconnected);
      connectionManager.off('reconnecting', onReconnecting);
    };
  }, []);

  const connect = useCallback((url: string) => {
    connectionManager.connect(url);
  }, []);

  const disconnect = useCallback(() => {
    connectionManager.disconnect();
  }, []);

  const send = useCallback((packet: ClientPacket) => {
    connectionManager.send(packet.packet_id(), packet.serialize());
  }, []);

  const onServerPacket = useCallback(
    (id: number, handler: ConnectionEventHandler) => {
      // 与 connection.ts 中 emit(ServerPacketIds[packet_id], ...) 保持一致
      connectionManager.on(ServerPacketIds[id] || `packet_${id}`, handler);
    },
    [],
  );

  const clearServerPacket = useCallback(
    (id: number, handler: ConnectionEventHandler) => {
      connectionManager.off(ServerPacketIds[id] || `packet_${id}`, handler);
    },
    [],
  );

  return { state, connect, disconnect, send, onServerPacket, clearServerPacket };
}
