use binrw;

/// 网络层错误类型
#[derive(Debug, thiserror::Error)]
pub enum NetError {
    /// IO 错误
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// BinRW 编解码错误
    #[error("BinRW error: {0}")]
    BinRw(#[from] binrw::Error),

    /// 非法的数据包 ID
    #[error("Invalid packet ID: {0}")]
    InvalidPacketId(u16),

    /// 连接已关闭
    #[error("Connection closed")]
    ConnectionClosed,

    /// 数据包过大
    #[error("Packet too large: {0} bytes")]
    PacketTooLarge(u16),
}
