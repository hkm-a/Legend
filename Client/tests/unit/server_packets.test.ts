import { describe, it, expect } from 'vitest';
import { parseServerPacket } from '../../src/network/packets/server_packets';
import { PacketCodec } from '../../src/network/codec';
import { ServerPacketIds } from '../../src/types/packets';

/**
 * 服务端数据包反序列化单元测试
 *
 * 构建与服务端 encode() 一致的二进制数据，验证客户端 parseServerPacket 能正确反序列化。
 */
describe('Server Packets Parsing', () => {
  // ========================================
  // Connected (0) — 空载荷
  // ========================================
  describe('Connected (id=0)', () => {
    it('should parse Connected packet with empty payload', () => {
      const encoded = PacketCodec.encode(ServerPacketIds.Connected, new ArrayBuffer(0));
      const decoded = PacketCodec.decode(encoded);

      const result = parseServerPacket(decoded.packet_id, decoded.payload);
      expect(result).toEqual({});
    });
  });

  // ========================================
  // KeepAlive (3) — 空载荷
  // ========================================
  describe('KeepAlive (id=3)', () => {
    it('should parse KeepAlive packet with empty payload', () => {
      const encoded = PacketCodec.encode(ServerPacketIds.KeepAlive, new ArrayBuffer(0));
      const decoded = PacketCodec.decode(encoded);

      const result = parseServerPacket(decoded.packet_id, decoded.payload);
      expect(result).toEqual({});
    });
  });

  // ========================================
  // Disconnect (2) — 载荷 [reason: u8]
  // ========================================
  describe('Disconnect (id=2)', () => {
    it('should parse Disconnect packet with reason code', () => {
      const payload = new Uint8Array([1]).buffer; // reason=1
      const encoded = PacketCodec.encode(ServerPacketIds.Disconnect, payload);
      const decoded = PacketCodec.decode(encoded);

      const result = parseServerPacket(decoded.packet_id, decoded.payload);
      expect(result).toEqual({ reason: 1 });
    });
  });

  // ========================================
  // UserLocation (23) — 载荷 [x: i32 LE][y: i32 LE][dir: u8]
  // ========================================
  describe('UserLocation (id=23)', () => {
    it('should parse UserLocation packet', () => {
      const payload = new ArrayBuffer(9);
      const view = new DataView(payload);
      view.setInt32(0, 100, true); // x
      view.setInt32(4, 200, true); // y
      view.setUint8(8, 4); // direction=4 (Down)

      const encoded = PacketCodec.encode(ServerPacketIds.UserLocation, payload);
      const decoded = PacketCodec.decode(encoded);

      const result = parseServerPacket(decoded.packet_id, decoded.payload);
      expect(result).toEqual({
        location: { x: 100, y: 200 },
        direction: 4,
      });
    });

    it('should handle negative coordinates', () => {
      const payload = new ArrayBuffer(9);
      const view = new DataView(payload);
      view.setInt32(0, -50, true);
      view.setInt32(4, -100, true);
      view.setUint8(8, 2);

      const encoded = PacketCodec.encode(ServerPacketIds.UserLocation, payload);
      const decoded = PacketCodec.decode(encoded);

      const result = parseServerPacket(decoded.packet_id, decoded.payload);
      expect(result.location.x).toBe(-50);
      expect(result.location.y).toBe(-100);
    });
  });

  // ========================================
  // ObjectPlayer (24) — 复杂结构
  // ========================================
  describe('ObjectPlayer (id=24)', () => {
    it('should parse ObjectPlayer packet', () => {
      // Build payload: [object_id: u32 LE][name_len: u16 LE][name: u8[]][class: u8][gender: u8][x: i32 LE][y: i32 LE][dir: u8]
      const name = 'TestPlayer';
      const nameBytes = new TextEncoder().encode(name);
      const payload = new ArrayBuffer(4 + 2 + nameBytes.length + 1 + 1 + 4 + 4 + 1);
      const view = new DataView(payload);
      let offset = 0;

      // object_id
      view.setUint32(offset, 42, true);
      offset += 4;

      // name length
      view.setUint16(offset, nameBytes.length, true);
      offset += 2;

      // name
      new Uint8Array(payload, offset, nameBytes.length).set(nameBytes);
      offset += nameBytes.length;

      // class = Warrior(0)
      view.setUint8(offset, 0);
      offset += 1;

      // gender = Male(0)
      view.setUint8(offset, 0);
      offset += 1;

      // x
      view.setInt32(offset, 300, true);
      offset += 4;

      // y
      view.setInt32(offset, 400, true);
      offset += 4;

      // direction = DownRight(3)
      view.setUint8(offset, 3);

      const encoded = PacketCodec.encode(ServerPacketIds.ObjectPlayer, payload);
      const decoded = PacketCodec.decode(encoded);

      const result = parseServerPacket(decoded.packet_id, decoded.payload);
      expect(result.object_id).toBe(42);
      expect(result.name).toBe('TestPlayer');
      expect(result.class).toBe(0);
      expect(result.gender).toBe(0);
      expect(result.location.x).toBe(300);
      expect(result.location.y).toBe(400);
      expect(result.direction).toBe(3);
    });

    it('should handle Chinese player names', () => {
      const name = '玩家一';
      const nameBytes = new TextEncoder().encode(name);
      const payload = new ArrayBuffer(4 + 2 + nameBytes.length + 1 + 1 + 4 + 4 + 1);
      const view = new DataView(payload);
      let offset = 0;

      view.setUint32(offset, 1, true);
      offset += 4;
      view.setUint16(offset, nameBytes.length, true);
      offset += 2;
      new Uint8Array(payload, offset, nameBytes.length).set(nameBytes);
      offset += nameBytes.length + 1 + 1; // class + gender
      view.setInt32(offset, 0, true);
      offset += 4;
      view.setInt32(offset, 0, true);

      const encoded = PacketCodec.encode(ServerPacketIds.ObjectPlayer, payload);
      const decoded = PacketCodec.decode(encoded);
      const result = parseServerPacket(decoded.packet_id, decoded.payload);

      expect(result.name).toBe('玩家一');
    });
  });

  // ========================================
  // ObjectRemove (26) — 载荷 [object_id: u32 LE]
  // ========================================
  describe('ObjectRemove (id=26)', () => {
    it('should parse ObjectRemove packet', () => {
      const payload = new Uint32Array([999]).buffer;
      const encoded = PacketCodec.encode(ServerPacketIds.ObjectRemove, payload);
      const decoded = PacketCodec.decode(encoded);

      const result = parseServerPacket(decoded.packet_id, decoded.payload);
      expect(result.object_id).toBe(999);
    });
  });

  // ========================================
  // Chat (30) — 载荷 [msg_len: u16 LE][msg: u8[]][chat_type: u8]
  // ========================================
  describe('Chat (id=30)', () => {
    it('should parse Chat packet', () => {
      const msg = 'Hello, world!';
      const msgBytes = new TextEncoder().encode(msg);
      const payload = new ArrayBuffer(2 + msgBytes.length + 1);
      const view = new DataView(payload);
      view.setUint16(0, msgBytes.length, true);
      new Uint8Array(payload, 2, msgBytes.length).set(msgBytes);
      view.setUint8(2 + msgBytes.length, 0); // chat_type = Normal(0)

      const encoded = PacketCodec.encode(ServerPacketIds.Chat, payload);
      const decoded = PacketCodec.decode(encoded);

      const result = parseServerPacket(decoded.packet_id, decoded.payload);
      expect(result.message).toBe('Hello, world!');
      expect(result.chat_type).toBe(0);
    });

    it('should parse Chat packet with different chat types', () => {
      const testCases = [
        { msg: 'Normal chat', chatType: 0 },
        { msg: 'Shout!', chatType: 1 },
        { msg: 'System msg', chatType: 2 },
        { msg: 'Announcement', chatType: 4 },
      ];

      for (const tc of testCases) {
        const msgBytes = new TextEncoder().encode(tc.msg);
        const payload = new ArrayBuffer(2 + msgBytes.length + 1);
        const view = new DataView(payload);
        view.setUint16(0, msgBytes.length, true);
        new Uint8Array(payload, 2, msgBytes.length).set(msgBytes);
        view.setUint8(2 + msgBytes.length, tc.chatType);

        const encoded = PacketCodec.encode(ServerPacketIds.Chat, payload);
        const decoded = PacketCodec.decode(encoded);
        const result = parseServerPacket(decoded.packet_id, decoded.payload);

        expect(result.message).toBe(tc.msg);
        expect(result.chat_type).toBe(tc.chatType);
      }
    });
  });

  // ========================================
  // MapInformation (17) — 复杂结构
  // ========================================
  describe('MapInformation (id=17)', () => {
    it('should parse MapInformation packet', () => {
      const title = '比奇省';
      const filename = '0.map';
      const titleBytes = new TextEncoder().encode(title);
      const filenameBytes = new TextEncoder().encode(filename);

      const payload = new ArrayBuffer(2 + 2 + 2 + 2 + titleBytes.length + 2 + filenameBytes.length);
      const view = new DataView(payload);
      let offset = 0;

      view.setUint16(offset, 1, true); // map_id
      offset += 2;
      view.setUint16(offset, 100, true); // width
      offset += 2;
      view.setUint16(offset, 100, true); // height
      offset += 2;

      view.setUint16(offset, titleBytes.length, true); // title_len
      offset += 2;
      new Uint8Array(payload, offset, titleBytes.length).set(titleBytes);
      offset += titleBytes.length;

      view.setUint16(offset, filenameBytes.length, true); // filename_len
      offset += 2;
      new Uint8Array(payload, offset, filenameBytes.length).set(filenameBytes);

      const encoded = PacketCodec.encode(ServerPacketIds.MapInformation, payload);
      const decoded = PacketCodec.decode(encoded);

      const result = parseServerPacket(decoded.packet_id, decoded.payload);
      expect(result.map_id).toBe(1);
      expect(result.width).toBe(100);
      expect(result.height).toBe(100);
      expect(result.title).toBe('比奇省');
      expect(result.filename).toBe('0.map');
    });
  });

  // ========================================
  // TimeOfDay (61) — 载荷 [light: u8]
  // ========================================
  describe('TimeOfDay (id=61)', () => {
    it('should parse TimeOfDay packet', () => {
      const payload = new Uint8Array([2]).buffer; // light=2 (Day)
      const encoded = PacketCodec.encode(ServerPacketIds.TimeOfDay, payload);
      const decoded = PacketCodec.decode(encoded);

      const result = parseServerPacket(decoded.packet_id, decoded.payload);
      expect(result.light).toBe(2);
    });
  });

  // ========================================
  // Unknown packet_id
  // ========================================
  describe('Unknown packet', () => {
    it('should return null for unregistered packet_id', () => {
      const result = parseServerPacket(9999, new ArrayBuffer(0));
      expect(result).toBeNull();
    });
  });
});
