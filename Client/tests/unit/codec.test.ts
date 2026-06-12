import { describe, it, expect } from 'vitest';
import { PacketCodec } from '../../src/network/codec';

/**
 * PacketCodec 单元测试
 *
 * 验证编解码器与 Rust 端 PacketCodec 的二进制兼容性。
 * 包格式：[PacketID: u16 LE][Length: u16 LE][Payload: u8[Length]]
 */
describe('PacketCodec', () => {
  // ========================================
  // encode
  // ========================================

  describe('encode', () => {
    it('should encode a packet with empty payload', () => {
      const result = PacketCodec.encode(0, new ArrayBuffer(0));
      const view = new DataView(result);

      // 4 bytes header: packet_id(0) + length(0)
      expect(result.byteLength).toBe(4);
      expect(view.getUint16(0, true)).toBe(0); // packet_id
      expect(view.getUint16(2, true)).toBe(0); // length
    });

    it('should encode a packet with non-empty payload', () => {
      const payload = new Uint8Array([0x01, 0x02, 0x03]).buffer;
      const result = PacketCodec.encode(42, payload);
      const view = new DataView(result);

      expect(result.byteLength).toBe(7); // 4 header + 3 payload
      expect(view.getUint16(0, true)).toBe(42); // packet_id
      expect(view.getUint16(2, true)).toBe(3); // length

      const payloadBytes = new Uint8Array(result, 4, 3);
      expect([...payloadBytes]).toEqual([0x01, 0x02, 0x03]);
    });

    it('should encode a packet with KeepAlive packet_id (3)', () => {
      const result = PacketCodec.encode(3, new ArrayBuffer(0));
      const view = new DataView(result);

      expect(result.byteLength).toBe(4);
      expect(view.getUint16(0, true)).toBe(3); // ServerOpcode::KeepAlive
      expect(view.getUint16(2, true)).toBe(0);
    });

    it('should throw error for too large payload', () => {
      const largeSize = PacketCodec.MAX_PACKET_SIZE - PacketCodec.HEADER_SIZE + 1;
      const largePayload = new ArrayBuffer(largeSize);
      expect(() => PacketCodec.encode(1, largePayload)).toThrow('Packet too large');
    });

    it('should accept maximum allowed payload size', () => {
      const maxSize = PacketCodec.MAX_PACKET_SIZE - PacketCodec.HEADER_SIZE;
      const maxPayload = new ArrayBuffer(maxSize);
      const result = PacketCodec.encode(65535, maxPayload);
      expect(result.byteLength).toBe(PacketCodec.MAX_PACKET_SIZE);
    });
  });

  // ========================================
  // decode
  // ========================================

  describe('decode', () => {
    it('should decode a packet with empty payload', () => {
      const buffer = PacketCodec.encode(5, new ArrayBuffer(0));
      const result = PacketCodec.decode(buffer);

      expect(result.packet_id).toBe(5);
      expect(result.payload.byteLength).toBe(0);
    });

    it('should decode a packet with payload', () => {
      const payload = new Uint8Array([0x64, 0x00, 0x00, 0x00]).buffer; // u32 LE = 100
      const buffer = PacketCodec.encode(23, payload); // UserLocation = 23
      const result = PacketCodec.decode(buffer);

      expect(result.packet_id).toBe(23);
      expect(result.payload.byteLength).toBe(4);

      const view = new DataView(result.payload);
      expect(view.getUint32(0, true)).toBe(100);
    });

    it('should decode a Walk packet (ClientOpcode=11, payload=[direction])', () => {
      const payload = new Uint8Array([4]).buffer; // direction=4 (Down)
      const buffer = PacketCodec.encode(11, payload); // Walk = 11
      const result = PacketCodec.decode(buffer);

      expect(result.packet_id).toBe(11);
      expect(result.payload.byteLength).toBe(1);

      const view = new DataView(result.payload);
      expect(view.getUint8(0)).toBe(4);
    });

    it('should throw error for incomplete header', () => {
      const shortBuffer = new ArrayBuffer(2);
      expect(() => PacketCodec.decode(shortBuffer)).toThrow('Incomplete packet header');
    });

    it('should throw error for incomplete payload', () => {
      // Build a buffer that claims 10 bytes payload but only has 2
      const buffer = new ArrayBuffer(6); // header(4) + 2 actual payload bytes
      const view = new DataView(buffer);
      view.setUint16(0, 1, true); // packet_id = 1
      view.setUint16(2, 10, true); // length = 10 (claims more than available)

      expect(() => PacketCodec.decode(buffer)).toThrow('Incomplete packet payload');
    });

    // Note: The "packet too large" guard (length > MAX_PACKET_SIZE) in decode()
    // is unreachable via u16 — a u16 stored value can never exceed 65535
    // which == MAX_PACKET_SIZE. This is a dead code path kept for parity with
    // the Rust PacketCodec. Verified as unreachable; no test provided.
  });

  // ========================================
  // createHeader
  // ========================================

  describe('createHeader', () => {
    it('should create a valid header buffer', () => {
      const header = PacketCodec.createHeader(10, 100);
      expect(header.byteLength).toBe(PacketCodec.HEADER_SIZE);

      const view = new DataView(header);
      expect(view.getUint16(0, true)).toBe(10);
      expect(view.getUint16(2, true)).toBe(100);
    });
  });

  // ========================================
  // Round-trip (encode → decode)
  // ========================================

  describe('round-trip', () => {
    it('should round-trip a simple packet correctly', () => {
      const payload = new Uint8Array([1, 2, 3, 4, 5]).buffer;
      const encoded = PacketCodec.encode(7, payload);
      const decoded = PacketCodec.decode(encoded);

      expect(decoded.packet_id).toBe(7);
      expect(new Uint8Array(decoded.payload)).toEqual(new Uint8Array([1, 2, 3, 4, 5]));
    });

    it('should round-trip an empty packet correctly', () => {
      const encoded = PacketCodec.encode(0, new ArrayBuffer(0));
      const decoded = PacketCodec.decode(encoded);

      expect(decoded.packet_id).toBe(0);
      expect(decoded.payload.byteLength).toBe(0);
    });

    it('should round-trip a KeepAlive packet (packet_id=3)', () => {
      // ServerOpcode::KeepAlive = 3, empty payload
      const encoded = PacketCodec.encode(3, new ArrayBuffer(0));
      const decoded = PacketCodec.decode(encoded);

      expect(decoded.packet_id).toBe(3);
      expect(decoded.payload.byteLength).toBe(0);
    });
  });
});
