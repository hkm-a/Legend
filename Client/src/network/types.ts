/** 连接状态 */
export enum ConnectionState {
  Disconnected = 'disconnected',
  Connecting = 'connecting',
  Connected = 'connected',
  Reconnecting = 'reconnecting',
}

/** 数据包头部 */
export interface PacketHeader {
  packet_id: number;
  length: number;
}

/** 客户端数据包接口 */
export interface ClientPacket {
  packet_id(): number;
  serialize(): ArrayBuffer;
}

/** 连接事件回调类型 */
export type ConnectionEventHandler = (...args: any[]) => void;
