/** PacketCodec 编解码结果 */
export interface PacketParseResult {
  packet_id: number;
  payload: ArrayBuffer;
}

/**
 * 数据包编解码器
 *
 * 使用 DataView 实现小端序编解码，与 Rust 端 PacketCodec 兼容。
 * 包格式：[PacketID: u16 LE][Length: u16 LE][Payload: u8[Length]]
 */
export class PacketCodec {
  /** 数据包头大小：PacketID(2) + Length(2) = 4 字节 */
  static readonly HEADER_SIZE = 4;

  /** 最大数据包大小 */
  static readonly MAX_PACKET_SIZE = 65535;

  /**
   * 编码数据包
   * @param packet_id - 操作码
   * @param payload - 载荷数据
   * @returns 完整的数据包 ArrayBuffer
   */
  static encode(packet_id: number, payload: ArrayBuffer): ArrayBuffer {
    const payloadLen = payload.byteLength;
    if (payloadLen > PacketCodec.MAX_PACKET_SIZE - PacketCodec.HEADER_SIZE) {
      throw new Error(`Packet too large: ${payloadLen} bytes`);
    }

    const buffer = new ArrayBuffer(PacketCodec.HEADER_SIZE + payloadLen);
    const view = new DataView(buffer);

    // PacketID: u16 LE
    view.setUint16(0, packet_id, true);
    // Length: u16 LE (only payload length)
    view.setUint16(2, payloadLen, true);

    // Copy payload
    if (payloadLen > 0) {
      new Uint8Array(buffer, PacketCodec.HEADER_SIZE).set(
        new Uint8Array(payload)
      );
    }

    return buffer;
  }

  /**
   * 解码数据包头部
   * @param buffer - 完整的数据包
   * @returns 解析后的头部和载荷
   */
  static decode(buffer: ArrayBuffer): PacketParseResult {
    if (buffer.byteLength < PacketCodec.HEADER_SIZE) {
      throw new Error(`Incomplete packet header: ${buffer.byteLength} bytes`);
    }

    const view = new DataView(buffer);

    // PacketID: u16 LE
    const packet_id = view.getUint16(0, true);
    // Length: u16 LE
    const length = view.getUint16(2, true);

    if (length > PacketCodec.MAX_PACKET_SIZE) {
      throw new Error(`Packet too large: ${length} bytes`);
    }

    const payloadEnd = PacketCodec.HEADER_SIZE + length;
    if (buffer.byteLength < payloadEnd) {
      throw new Error(
        `Incomplete packet payload: expected ${payloadEnd}, got ${buffer.byteLength}`
      );
    }

    const payload = buffer.slice(PacketCodec.HEADER_SIZE, payloadEnd);

    return { packet_id, payload };
  }

  /**
   * 仅创建数据包头部（用于测试）
   */
  static createHeader(packet_id: number, length: number): ArrayBuffer {
    const buffer = new ArrayBuffer(PacketCodec.HEADER_SIZE);
    const view = new DataView(buffer);
    view.setUint16(0, packet_id, true);
    view.setUint16(2, length, true);
    return buffer;
  }
}
