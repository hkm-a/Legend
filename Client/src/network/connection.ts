import { PacketCodec } from './codec';
import { ConnectionState, ConnectionEventHandler } from './types';
import { ServerPacketIds } from '../types/packets';

/**
 * 连接管理器（单例）
 *
 * 封装 WebSocket 生命周期：连接、重连、心跳、消息派发。
 */
class ConnectionManager {
  private url: string = '';
  private ws: WebSocket | null = null;
  private state: ConnectionState = ConnectionState.Disconnected;
  private listeners: Map<string, Set<ConnectionEventHandler>> = new Map();
  private reconnectTimer: ReturnType<typeof setTimeout> | null = null;
  private reconnectAttempts: number = 0;
  private maxReconnectAttempts: number = 10;
  private heartbeatInterval: ReturnType<typeof setInterval> | null = null;
  private lastPongTime: number = 0;

  /** 获取当前连接状态 */
  getState(): ConnectionState {
    return this.state;
  }

  /** 连接至服务器 */
  connect(url: string): void {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      console.warn('[ConnectionManager] Already connected');
      return;
    }

    this.url = url;
    this.setState(ConnectionState.Connecting);

    try {
      this.ws = new WebSocket(url);
      this.ws.binaryType = 'arraybuffer';

      this.ws.onopen = () => this.onOpen();
      this.ws.onmessage = (ev: MessageEvent) => this.onMessage(ev);
      this.ws.onclose = () => this.onClose();
      this.ws.onerror = () => this.onError();
    } catch (e) {
      console.error('[ConnectionManager] Failed to create WebSocket:', e);
      this.setState(ConnectionState.Disconnected);
      this.scheduleReconnect();
    }
  }

  /** 断开连接 */
  disconnect(): void {
    this.stopHeartbeat();
    this.clearReconnectTimer();

    if (this.ws) {
      this.ws.close(1000, 'Client disconnect');
      this.ws = null;
    }

    this.setState(ConnectionState.Disconnected);
    this.reconnectAttempts = 0;
  }

  /** 发送数据包 */
  send(packet_id: number, payload?: ArrayBuffer): void {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
      console.warn('[ConnectionManager] Cannot send: not connected');
      return;
    }

    const data = PacketCodec.encode(packet_id, payload || new ArrayBuffer(0));
    this.ws.send(data);
  }

  /** 注册事件监听 */
  on(event: string, cb: ConnectionEventHandler): void {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, new Set());
    }
    this.listeners.get(event)!.add(cb);
  }

  /** 移除事件监听 */
  off(event: string, cb: ConnectionEventHandler): void {
    this.listeners.get(event)?.delete(cb);
  }

  /** 设置最大重连次数 */
  setMaxReconnectAttempts(max: number): void {
    this.maxReconnectAttempts = max;
  }

  // ---- Private Methods ----

  private setState(state: ConnectionState): void {
    this.state = state;
    this.emit('state_change', state);
  }

  private emit(event: string, ...args: any[]): void {
    this.listeners.get(event)?.forEach((cb) => cb(...args));

    // 同时发出通用事件
    if (event !== 'packet') {
      this.listeners.get('*')?.forEach((cb) => cb(event, ...args));
    }
  }

  private onOpen(): void {
    console.log('[ConnectionManager] Connected to', this.url);
    this.setState(ConnectionState.Connected);
    this.reconnectAttempts = 0;
    this.startHeartbeat();
    this.emit('connected');
  }

  private onMessage(ev: MessageEvent): void {
    if (!(ev.data instanceof ArrayBuffer)) return;

    try {
      const { packet_id, payload } = PacketCodec.decode(ev.data);

      // 如果是心跳回复，更新 lastPongTime
      if (packet_id === ServerPacketIds.KeepAlive) {
        this.lastPongTime = Date.now();
      }

      this.emit('packet', packet_id, payload);
      this.emit(ServerPacketIds[packet_id] || `packet_${packet_id}`, payload);
    } catch (e) {
      console.warn('[ConnectionManager] Failed to decode packet:', e);
    }
  }

  private onClose(): void {
    console.log('[ConnectionManager] Connection closed');
    this.stopHeartbeat();
    this.ws = null;
    this.setState(ConnectionState.Disconnected);
    this.emit('disconnected');
    this.scheduleReconnect();
  }

  private onError(): void {
    console.error('[ConnectionManager] WebSocket error');
    this.emit('error');
  }

  private scheduleReconnect(): void {
    if (this.reconnectTimer !== null) return;
    if (this.reconnectAttempts >= this.maxReconnectAttempts) {
      console.warn('[ConnectionManager] Max reconnect attempts reached');
      this.emit('reconnect_failed');
      return;
    }

    this.reconnectAttempts++;
    const delay = Math.min(1000 * Math.pow(2, this.reconnectAttempts - 1), 30000);

    console.log(
      `[ConnectionManager] Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts}/${this.maxReconnectAttempts})`
    );

    this.setState(ConnectionState.Reconnecting);
    this.emit('reconnecting', this.reconnectAttempts);

    this.reconnectTimer = setTimeout(() => {
      this.reconnectTimer = null;
      this.connect(this.url);
    }, delay);
  }

  private clearReconnectTimer(): void {
    if (this.reconnectTimer !== null) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }
  }

  private startHeartbeat(): void {
    this.lastPongTime = Date.now();
    this.stopHeartbeat();

    // 每 5 秒发送心跳
    this.heartbeatInterval = setInterval(() => {
      // 如果超过 15 秒未收到回复，认为连接已断开
      if (Date.now() - this.lastPongTime > 15000) {
        console.warn('[ConnectionManager] Heartbeat timeout, reconnecting...');
        this.disconnect();
        this.scheduleReconnect();
        return;
      }

      this.send(2); // ClientPacketIds.KeepAlive = 2
    }, 5000);
  }

  private stopHeartbeat(): void {
    if (this.heartbeatInterval !== null) {
      clearInterval(this.heartbeatInterval);
      this.heartbeatInterval = null;
    }
  }
}

/** 全局单例 */
export const connectionManager = new ConnectionManager();
