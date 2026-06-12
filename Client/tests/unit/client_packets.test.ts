import { describe, it, expect } from 'vitest';
import {
  KeepAliveClientPacket,
  WalkPacket,
  RunPacket,
  AttackPacket,
  ChatPacket,
  TurnPacket,
  LogOutClientPacket,
  ClientVersionPacket,
  PickUpClientPacket,
} from '../../src/network/packets/client_packets';
import { ClientPacketIds } from '../../src/types/packets';
import { PacketCodec } from '../../src/network/codec';

/**
 * 客户端数据包序列化单元测试
 *
 * 验证每个客户端包的正确序列化，包括 packet_id 和载荷格式。
 */
describe('Client Packets', () => {
  describe('KeepAliveClientPacket', () => {
    it('should have correct packet_id (2)', () => {
      const pkt = new KeepAliveClientPacket();
      expect(pkt.packet_id()).toBe(ClientPacketIds.KeepAlive);
      expect(pkt.packet_id()).toBe(2);
    });

    it('should serialize to empty buffer', () => {
      const pkt = new KeepAliveClientPacket();
      const data = pkt.serialize();
      expect(data.byteLength).toBe(0);
    });

    it('should produce valid wire format via PacketCodec', () => {
      const pkt = new KeepAliveClientPacket();
      const encoded = PacketCodec.encode(pkt.packet_id(), pkt.serialize());
      const decoded = PacketCodec.decode(encoded);

      expect(decoded.packet_id).toBe(2);
      expect(decoded.payload.byteLength).toBe(0);
    });
  });

  describe('WalkPacket', () => {
    it('should have correct packet_id (11)', () => {
      const pkt = new WalkPacket(0);
      expect(pkt.packet_id()).toBe(ClientPacketIds.Walk);
      expect(pkt.packet_id()).toBe(11);
    });

    it('should serialize direction as single byte', () => {
      const pkt = new WalkPacket(4); // Down
      const data = pkt.serialize();
      expect(data.byteLength).toBe(1);
      expect(new DataView(data).getUint8(0)).toBe(4);
    });

    it('should handle all 8 directions', () => {
      for (let dir = 0; dir < 8; dir++) {
        const pkt = new WalkPacket(dir);
        const data = pkt.serialize();
        expect(new DataView(data).getUint8(0)).toBe(dir);
      }
    });

    it('should produce valid wire format via PacketCodec', () => {
      const pkt = new WalkPacket(2); // Right
      const encoded = PacketCodec.encode(pkt.packet_id(), pkt.serialize());
      const decoded = PacketCodec.decode(encoded);

      expect(decoded.packet_id).toBe(11);
      expect(decoded.payload.byteLength).toBe(1);
      expect(new DataView(decoded.payload).getUint8(0)).toBe(2);
    });
  });

  describe('RunPacket', () => {
    it('should have correct packet_id (12)', () => {
      const pkt = new RunPacket(0);
      expect(pkt.packet_id()).toBe(ClientPacketIds.Run);
      expect(pkt.packet_id()).toBe(12);
    });

    it('should serialize direction as single byte', () => {
      const pkt = new RunPacket(1);
      const data = pkt.serialize();
      expect(data.byteLength).toBe(1);
      expect(new DataView(data).getUint8(0)).toBe(1);
    });

    it('should produce valid wire format', () => {
      const pkt = new RunPacket(6); // Left
      const encoded = PacketCodec.encode(pkt.packet_id(), pkt.serialize());
      const decoded = PacketCodec.decode(encoded);

      expect(decoded.packet_id).toBe(12);
      expect(new DataView(decoded.payload).getUint8(0)).toBe(6);
    });
  });

  describe('AttackPacket', () => {
    it('should have correct packet_id (47)', () => {
      const pkt = new AttackPacket(0);
      expect(pkt.packet_id()).toBe(ClientPacketIds.Attack);
      expect(pkt.packet_id()).toBe(47);
    });

    it('should serialize direction and spell', () => {
      const pkt = new AttackPacket(2, 5);
      const data = pkt.serialize();
      expect(data.byteLength).toBe(2);

      const view = new DataView(data);
      expect(view.getUint8(0)).toBe(2); // direction
      expect(view.getUint8(1)).toBe(5); // spell
    });

    it('should default spell to 0', () => {
      const pkt = new AttackPacket(3);
      const data = pkt.serialize();
      const view = new DataView(data);
      expect(view.getUint8(0)).toBe(3);
      expect(view.getUint8(1)).toBe(0);
    });
  });

  describe('ChatPacket', () => {
    it('should have correct packet_id (13)', () => {
      const pkt = new ChatPacket('hello');
      expect(pkt.packet_id()).toBe(ClientPacketIds.Chat);
      expect(pkt.packet_id()).toBe(13);
    });

    it('should serialize message with u16 length prefix', () => {
      const pkt = new ChatPacket('Hi');
      const data = pkt.serialize();

      const view = new DataView(data);
      expect(view.getUint16(0, true)).toBe(2); // length prefix
      expect(new Uint8Array(data, 2, 2)).toEqual(new Uint8Array([0x48, 0x69])); // "Hi"
    });

    it('should serialize empty message correctly', () => {
      const pkt = new ChatPacket('');
      const data = pkt.serialize();

      expect(data.byteLength).toBe(2);
      expect(new DataView(data).getUint16(0, true)).toBe(0);
    });

    it('should handle Chinese characters', () => {
      const pkt = new ChatPacket('你好');
      const data = pkt.serialize();

      const view = new DataView(data);
      const len = view.getUint16(0, true);
      expect(len).toBe(6); // UTF-8: each Chinese char = 3 bytes

      const bytes = new Uint8Array(data, 2, len);
      const decoded = new TextDecoder().decode(bytes);
      expect(decoded).toBe('你好');
    });

    it('should produce valid wire format', () => {
      const pkt = new ChatPacket('Test message');
      const encoded = PacketCodec.encode(pkt.packet_id(), pkt.serialize());
      const decoded = PacketCodec.decode(encoded);

      expect(decoded.packet_id).toBe(13);

      const dv = new DataView(decoded.payload);
      const msgLen = dv.getUint16(0, true);
      const msgBytes = new Uint8Array(decoded.payload, 2, msgLen);
      expect(new TextDecoder().decode(msgBytes)).toBe('Test message');
    });
  });

  describe('TurnPacket', () => {
    it('should have correct packet_id (10)', () => {
      const pkt = new TurnPacket(0);
      expect(pkt.packet_id()).toBe(ClientPacketIds.Turn);
      expect(pkt.packet_id()).toBe(10);
    });

    it('should serialize direction as single byte', () => {
      const pkt = new TurnPacket(7); // UpLeft
      const data = pkt.serialize();
      expect(data.byteLength).toBe(1);
      expect(new DataView(data).getUint8(0)).toBe(7);
    });
  });

  describe('LogOutClientPacket', () => {
    it('should have correct packet_id (9)', () => {
      const pkt = new LogOutClientPacket();
      expect(pkt.packet_id()).toBe(ClientPacketIds.LogOut);
      expect(pkt.packet_id()).toBe(9);
    });

    it('should serialize to empty buffer', () => {
      const pkt = new LogOutClientPacket();
      expect(pkt.serialize().byteLength).toBe(0);
    });
  });

  describe('ClientVersionPacket', () => {
    it('should have correct packet_id (0)', () => {
      const pkt = new ClientVersionPacket();
      expect(pkt.packet_id()).toBe(ClientPacketIds.ClientVersion);
      expect(pkt.packet_id()).toBe(0);
    });

    it('should serialize with default version [1,0,0,0] and lang 0', () => {
      const pkt = new ClientVersionPacket();
      const data = pkt.serialize();

      // 4 (version) + 2 (lang) + 20 (pad) = 26 bytes
      expect(data.byteLength).toBe(26);

      const view = new DataView(data);
      expect(view.getUint8(0)).toBe(1); // major
      expect(view.getUint8(1)).toBe(0); // minor
      expect(view.getUint8(2)).toBe(0); // patch
      expect(view.getUint8(3)).toBe(0); // build
      expect(view.getUint16(4, true)).toBe(0); // lang
    });

    it('should serialize custom version', () => {
      const pkt = new ClientVersionPacket([2, 1, 3, 7], 1);
      const data = pkt.serialize();
      const view = new DataView(data);

      expect(view.getUint8(0)).toBe(2);
      expect(view.getUint8(1)).toBe(1);
      expect(view.getUint8(2)).toBe(3);
      expect(view.getUint8(3)).toBe(7);
      expect(view.getUint16(4, true)).toBe(1);
    });
  });

  describe('PickUpClientPacket', () => {
    it('should have correct packet_id (35)', () => {
      const pkt = new PickUpClientPacket(12345);
      expect(pkt.packet_id()).toBe(ClientPacketIds.PickUp);
      expect(pkt.packet_id()).toBe(35);
    });

    it('should serialize objectId as u32 LE', () => {
      const pkt = new PickUpClientPacket(0x12345678);
      const buf = pkt.serialize();
      expect(buf.byteLength).toBe(4);
      const view = new DataView(buf);
      expect(view.getUint32(0, true)).toBe(0x12345678);
    });

    it('should handle objectId = 0', () => {
      const pkt = new PickUpClientPacket(0);
      const buf = pkt.serialize();
      expect(buf.byteLength).toBe(4);
      const view = new DataView(buf);
      expect(view.getUint32(0, true)).toBe(0);
    });
  });
});
