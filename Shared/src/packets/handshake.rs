use crate::net::error::NetError;
use crate::net::packet_id::{ClientOpcode, ServerOpcode};
use crate::packets::{Packet, PacketCodec};

/// 客户端版本包（客户端 -> 服务端）
///
/// 载荷：[version: u8[4]][lang: u16 LE][pad: u8[20]]
#[derive(Debug, Clone)]
pub struct ClientVersionPacket {
    pub version: [u8; 4],
    pub lang: u16,
    pub pad: [u8; 20],
}

impl ClientVersionPacket {
    pub fn new(version: [u8; 4], lang: u16) -> Self {
        Self {
            version,
            lang,
            pad: [0u8; 20],
        }
    }
}

impl Packet for ClientVersionPacket {
    fn packet_id(&self) -> u16 {
        ClientOpcode::ClientVersion as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let mut payload = Vec::with_capacity(26);
        payload.extend_from_slice(&self.version);
        payload.extend_from_slice(&self.lang.to_le_bytes());
        payload.extend_from_slice(&self.pad);
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 连接成功包（服务端 -> 客户端）
///
/// 空载荷，仅包含头部
#[derive(Debug, Clone)]
pub struct ConnectedPacket;

impl Packet for ConnectedPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::Connected as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        PacketCodec::encode(self.packet_id(), &[])
    }
}
