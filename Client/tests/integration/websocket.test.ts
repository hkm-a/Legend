import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import WebSocket from 'ws';
import { PacketCodec } from '../../src/network/codec';

/**
 * WebSocket 集成测试
 *
 * 测试客户端与 Rust 服务端之间的 WebSocket 通信。
 * 需要服务端已在 ws://localhost:7000 上运行。
 *
 * 启动服务端：cargo run --bin mir2-server（从项目根目录）
 */

const SERVER_URL = 'ws://localhost:7000';

// 协议常量
const ClientOpcode = {
  KeepAlive: 2,
  Walk: 11,
  Run: 12,
  Chat: 13,
  Turn: 10,
  Attack: 47,
  LogOut: 9,
  ClientVersion: 0,
} as const;

const ServerOpcode = {
  Connected: 0,
  KeepAlive: 3,
  Disconnect: 2,
  UserLocation: 23,
  Chat: 30,
  SendOutputMessage: 223,
} as const;

/**
 * 发送二进制数据包到 WebSocket
 */
function sendPacket(ws: WebSocket, packetId: number, payload: ArrayBuffer = new ArrayBuffer(0)) {
  const data = PacketCodec.encode(packetId, payload);
  ws.send(data);
}

/**
 * 等待 WebSocket 收到指定 packet_id 的数据包（超时 5s）
 */
function waitForPacket(
  ws: WebSocket,
  expectedPacketId: number,
  timeoutMs: number = 5000
): Promise<{ packet_id: number; payload: ArrayBuffer }> {
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => {
      ws.removeListener('message', onMessage);
      reject(new Error(`Timeout waiting for packet_id=${expectedPacketId}`));
    }, timeoutMs);

    function onMessage(data: Buffer) {
      try {
        const buf = data.buffer.slice(data.byteOffset, data.byteOffset + data.byteLength);
        const decoded = PacketCodec.decode(buf);

        if (decoded.packet_id === expectedPacketId) {
          clearTimeout(timer);
          ws.removeListener('message', onMessage);
          resolve(decoded);
        }
        // Non-matching packets are ignored (e.g., KeepAlive responses)
      } catch (e) {
        // Ignore decode errors for unexpected data
      }
    }

    ws.on('message', onMessage);
  });
}

/**
 * 等待最多 N 个数据包，返回匹配的 packet
 */
function waitForAnyPacket(
  ws: WebSocket,
  timeoutMs: number = 5000
): Promise<{ packet_id: number; payload: ArrayBuffer }> {
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => {
      ws.removeListener('message', onMessage);
      reject(new Error('Timeout waiting for any packet'));
    }, timeoutMs);

    function onMessage(data: Buffer) {
      try {
        const buf = data.buffer.slice(data.byteOffset, data.byteOffset + data.byteLength);
        const decoded = PacketCodec.decode(buf);

        clearTimeout(timer);
        ws.removeListener('message', onMessage);
        resolve(decoded);
      } catch (e) {
        // Ignore decode errors
      }
    }

    ws.on('message', onMessage);
  });
}

/**
 * 检查服务端是否可达
 */
async function checkServerAvailable(): Promise<boolean> {
  return new Promise((resolve) => {
    const ws = new WebSocket(SERVER_URL);
    const timer = setTimeout(() => {
      ws.close();
      resolve(false);
    }, 2000);

    ws.on('open', () => {
      clearTimeout(timer);
      ws.close();
      resolve(true);
    });

    ws.on('error', () => {
      clearTimeout(timer);
      resolve(false);
    });
  });
}

describe('WebSocket Server Integration', () => {
  let serverAvailable = false;

  beforeAll(async () => {
    serverAvailable = await checkServerAvailable();
    if (!serverAvailable) {
      console.warn('⚠ Server not available at', SERVER_URL, '— skipping integration tests');
    }
  });

  // ========================================
  // TC01: 基本连接测试
  // ========================================
  describe('TC01: Basic Connection', () => {
    it('should successfully connect to the WebSocket server', async () => {
      if (!serverAvailable) return;

      const ws = new WebSocket(SERVER_URL);
      await new Promise<void>((resolve, reject) => {
        const timer = setTimeout(() => reject(new Error('Connection timeout')), 3000);
        ws.on('open', () => {
          clearTimeout(timer);
          resolve();
        });
        ws.on('error', reject);
      });

      expect(ws.readyState).toBe(WebSocket.OPEN);
      ws.close();
    });
  });

  // ========================================
  // TC02: KeepAlive 心跳测试
  // ========================================
  describe('TC02: KeepAlive', () => {
    it('should send KeepAlive and receive KeepAlive response', async () => {
      if (!serverAvailable) return;

      const ws = new WebSocket(SERVER_URL);
      await new Promise<void>((resolve) => ws.on('open', resolve));

      // 发送心跳包 (ClientOpcode.KeepAlive = 2)
      sendPacket(ws, ClientOpcode.KeepAlive);

      // 服务端 KeepAlive 约定为空回复，但 Handler 不会回复
      // 这里验证连接保持活跃（不超时断开）
      await new Promise((resolve) => setTimeout(resolve, 1000));

      // 连接应仍然正常
      expect(ws.readyState).toBe(WebSocket.OPEN);
      ws.close();
    });
  });

  // ========================================
  // TC03: Walk 包测试
  // ========================================
  describe('TC03: Walk Packet', () => {
    it('should send Walk and receive UserLocation response', async () => {
      if (!serverAvailable) return;

      const ws = new WebSocket(SERVER_URL);
      await new Promise<void>((resolve) => ws.on('open', resolve));

      // 发送 Walk 包 (direction=4 = Down)
      const payload = new Uint8Array([4]).buffer;
      sendPacket(ws, ClientOpcode.Walk, payload);

      // 期望收到 UserLocation 回复 (ServerOpcode.UserLocation = 23)
      const response = await waitForPacket(ws, ServerOpcode.UserLocation);
      expect(response.packet_id).toBe(23);

      // 解析 UserLocation: [x: i32 LE][y: i32 LE][dir: u8]
      const view = new DataView(response.payload);
      const x = view.getInt32(0, true);
      const y = view.getInt32(4, true);
      const dir = view.getUint8(8);

      // 桩阶段返回 (0, 0) + 请求方向
      expect(x).toBe(0);
      expect(y).toBe(0);
      expect(dir).toBe(4);

      ws.close();
    });

    it('should handle empty Walk payload gracefully', async () => {
      if (!serverAvailable) return;

      const ws = new WebSocket(SERVER_URL);
      await new Promise<void>((resolve) => ws.on('open', resolve));

      // 发送 Walk 包（空载荷）
      sendPacket(ws, ClientOpcode.Walk);

      // 空载荷应被服务器忽略（不回复），连接不应断开
      await new Promise((resolve) => setTimeout(resolve, 500));
      expect(ws.readyState).toBe(WebSocket.OPEN);

      ws.close();
    });
  });

  // ========================================
  // TC04: Run 包测试
  // ========================================
  describe('TC04: Run Packet', () => {
    it('should send Run and receive UserLocation response', async () => {
      if (!serverAvailable) return;

      const ws = new WebSocket(SERVER_URL);
      await new Promise<void>((resolve) => ws.on('open', resolve));

      // 发送 Run 包 (direction=2 = Right)
      const payload = new Uint8Array([2]).buffer;
      sendPacket(ws, ClientOpcode.Run, payload);

      const response = await waitForPacket(ws, ServerOpcode.UserLocation);
      expect(response.packet_id).toBe(23);

      const view = new DataView(response.payload);
      const dir = view.getUint8(8);
      expect(dir).toBe(2);

      ws.close();
    });
  });

  // ========================================
  // TC05: Attack 包测试
  // ========================================
  describe('TC05: Attack Packet', () => {
    it('should send Attack and receive GameMessage response', async () => {
      if (!serverAvailable) return;

      const ws = new WebSocket(SERVER_URL);
      await new Promise<void>((resolve) => ws.on('open', resolve));

      // 发送 Attack 包 (direction=0, spell=0)
      const payload = new Uint8Array([0, 0]).buffer;
      sendPacket(ws, ClientOpcode.Attack, payload);

      // 服务端 Attack handler 回复 GameMessagePacket (ServerOpcode.SendOutputMessage = 223)
      const response = await waitForPacket(ws, ServerOpcode.SendOutputMessage);
      expect(response.packet_id).toBe(223);

      // 解析 GameMessage: [msg_len: u16 LE][msg: u8[]][chat_type: u8][type: u8]
      const view = new DataView(response.payload);
      const msgLen = view.getUint16(0, true);
      const msgBytes = new Uint8Array(response.payload, 2, msgLen);
      const msg = new TextDecoder().decode(msgBytes);
      expect(msg).toContain('Attack');

      ws.close();
    });
  });

  // ========================================
  // TC06: Chat 包测试
  // ========================================
  describe('TC06: Chat Packet', () => {
    it('should send Chat and receive broadcast Chat response', async () => {
      if (!serverAvailable) return;

      const ws = new WebSocket(SERVER_URL);
      await new Promise<void>((resolve) => ws.on('open', resolve));

      // 发送 Chat 包 (message = "Hello Server!")
      const msg = 'Hello Server!';
      const msgBytes = new TextEncoder().encode(msg);
      const payload = new ArrayBuffer(2 + msgBytes.length);
      const view = new DataView(payload);
      view.setUint16(0, msgBytes.length, true);
      new Uint8Array(payload, 2, msgBytes.length).set(msgBytes);

      sendPacket(ws, ClientOpcode.Chat, payload);

      // 服务端 Chat handler 广播 ServerChatPacket (ServerOpcode.Chat = 30)
      const response = await waitForPacket(ws, ServerOpcode.Chat);
      expect(response.packet_id).toBe(30);

      // 解析 Chat: [msg_len: u16 LE][msg: u8[]][chat_type: u8]
      const dv = new DataView(response.payload);
      const respMsgLen = dv.getUint16(0, true);
      const respMsgBytes = new Uint8Array(response.payload, 2, respMsgLen);
      const respMsg = new TextDecoder().decode(respMsgBytes);
      expect(respMsg).toContain('Hello Server!');

      ws.close();
    });
  });

  // ========================================
  // TC07: Turn 包测试
  // ========================================
  describe('TC07: Turn Packet', () => {
    it('should send Turn and not crash server', async () => {
      if (!serverAvailable) return;

      const ws = new WebSocket(SERVER_URL);
      await new Promise<void>((resolve) => ws.on('open', resolve));

      // 发送 Turn 包 (direction=7 = UpLeft)
      const payload = new Uint8Array([7]).buffer;
      sendPacket(ws, ClientOpcode.Turn, payload);

      // Turn handler 只记录日志，不回复 — 连接应保持
      await new Promise((resolve) => setTimeout(resolve, 500));
      expect(ws.readyState).toBe(WebSocket.OPEN);

      ws.close();
    });
  });

  // ========================================
  // TC08: LogOut 包测试
  // ========================================
  describe('TC08: LogOut Packet', () => {
    it('should send LogOut and disconnect session', async () => {
      if (!serverAvailable) return;

      const ws = new WebSocket(SERVER_URL);
      await new Promise<void>((resolve) => ws.on('open', resolve));

      // 发送 LogOut 包 (ClientOpcode.LogOut = 9)
      sendPacket(ws, ClientOpcode.LogOut);

      // LogOut handler 从 session manager 中移除会话，
      // 由于 session manager 中保存的是 channel 的唯一 sender，
      // 移除后 channel 关闭，session 的 run() 循环退出，WebSocket 断开
      await new Promise((resolve) => setTimeout(resolve, 500));
      expect(ws.readyState).toBe(WebSocket.CLOSED);
    });
  });

  // ========================================
  // TC09: 多个 Handler 连续调用的稳定性测试
  // ========================================
  describe('TC09: Multiple Packets Stress', () => {
    it('should handle rapid packet sequence without disconnect', async () => {
      if (!serverAvailable) return;

      const ws = new WebSocket(SERVER_URL);
      await new Promise<void>((resolve) => ws.on('open', resolve));

      // 快速连续发送多个包
      const packets = [
        { id: ClientOpcode.KeepAlive, payload: new ArrayBuffer(0) },
        { id: ClientOpcode.Turn, payload: new Uint8Array([0]).buffer },
        { id: ClientOpcode.Walk, payload: new Uint8Array([2]).buffer },
        { id: ClientOpcode.Turn, payload: new Uint8Array([4]).buffer },
        { id: ClientOpcode.Run, payload: new Uint8Array([6]).buffer },
      ];

      for (const pkt of packets) {
        sendPacket(ws, pkt.id, pkt.payload);
      }

      // 等待所有包处理完成
      await new Promise((resolve) => setTimeout(resolve, 1000));

      // 连接应仍然正常
      expect(ws.readyState).toBe(WebSocket.OPEN);

      ws.close();
    });
  });

  // ========================================
  // TC10: 断开连接测试
  // ========================================
  describe('TC10: Disconnection', () => {
    it('should cleanly close the WebSocket connection', async () => {
      if (!serverAvailable) return;

      const ws = new WebSocket(SERVER_URL);
      await new Promise<void>((resolve) => ws.on('open', resolve));

      await new Promise<void>((resolve) => {
        ws.on('close', () => resolve());
        ws.close(1000, 'Test disconnect');
      });

      expect(ws.readyState).toBe(WebSocket.CLOSED);
    });
  });
});
