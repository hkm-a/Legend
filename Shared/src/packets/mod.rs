pub mod client;
pub mod handshake;
pub mod server;

pub use client::*;
pub use handshake::*;
pub use server::*;

use crate::net::error::NetError;

/// 数据包头大小：PacketID(2) + Length(2) = 4 字节
pub const HEADER_SIZE: usize = 4;

/// 最大数据包大小（包含头部）
pub const MAX_PACKET_SIZE: u16 = 65535;

/// 所有数据包必须实现的 trait
pub trait Packet: Send + Sync {
    /// 返回该数据包的操作码 (u16)
    fn packet_id(&self) -> u16;

    /// 将数据包编码为二进制字节序列
    fn encode(&self) -> Result<Vec<u8>, NetError>;
}

/// 数据包编解码器
pub struct PacketCodec;

impl PacketCodec {
    /// 编码：将 packet_id + payload 组装为完整数据包
    ///
    /// 格式：[PacketID: u16 LE][Length: u16 LE][Payload: u8[Length]]
    /// Length 只包含 Payload 的字节数
    pub fn encode(packet_id: u16, payload: &[u8]) -> Result<Vec<u8>, NetError> {
        let payload_len = payload.len();
        if payload_len > MAX_PACKET_SIZE as usize - HEADER_SIZE {
            return Err(NetError::PacketTooLarge(payload_len as u16));
        }

        let mut buf = Vec::with_capacity(HEADER_SIZE + payload_len);
        buf.extend_from_slice(&packet_id.to_le_bytes());
        buf.extend_from_slice(&(payload_len as u16).to_le_bytes());
        buf.extend_from_slice(payload);
        Ok(buf)
    }

    /// 解码数据包头，返回 (packet_id, length)
    ///
    /// 从完整的帧数据中提取 PacketID 和 Length
    pub fn decode_header(data: &[u8]) -> Result<(u16, u16), NetError> {
        if data.len() < HEADER_SIZE {
            return Err(NetError::Io(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Incomplete packet header",
            )));
        }

        let packet_id = u16::from_le_bytes([data[0], data[1]]);
        let length = u16::from_le_bytes([data[2], data[3]]);

        if length > MAX_PACKET_SIZE {
            return Err(NetError::PacketTooLarge(length));
        }

        Ok((packet_id, length))
    }

    /// 从完整的帧数据中提取 payload（跳过头部）
    ///
    /// 注意：此方法假设 data 已经是包含完整头部和载荷的帧
    pub fn decode_payload(data: &[u8]) -> Result<Vec<u8>, NetError> {
        if data.len() < HEADER_SIZE {
            return Err(NetError::Io(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Incomplete packet header",
            )));
        }

        let (_, length) = Self::decode_header(data)?;
        let payload_end = HEADER_SIZE + length as usize;

        if data.len() < payload_end {
            return Err(NetError::Io(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Incomplete packet payload",
            )));
        }

        Ok(data[HEADER_SIZE..payload_end].to_vec())
    }
}
