use crate::enums::{MirDirection, Spell};
use crate::net::error::NetError;
use crate::net::packet_id::ClientOpcode;
use crate::packets::{Packet, PacketCodec};

/// 心跳包（客户端 -> 服务端）
///
/// 空载荷
#[derive(Debug, Clone)]
pub struct KeepAlivePacket;

impl Packet for KeepAlivePacket {
    fn packet_id(&self) -> u16 {
        ClientOpcode::KeepAlive as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        PacketCodec::encode(self.packet_id(), &[])
    }
}

/// 移动包（客户端 -> 服务端）
///
/// 载荷：[direction: u8]
#[derive(Debug, Clone)]
pub struct WalkPacket {
    pub direction: MirDirection,
}

impl WalkPacket {
    pub fn new(direction: MirDirection) -> Self {
        Self { direction }
    }
}

impl Packet for WalkPacket {
    fn packet_id(&self) -> u16 {
        ClientOpcode::Walk as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = vec![self.direction as u8];
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 跑步包（客户端 -> 服务端）
///
/// 载荷：[direction: u8]
#[derive(Debug, Clone)]
pub struct RunPacket {
    pub direction: MirDirection,
}

impl RunPacket {
    pub fn new(direction: MirDirection) -> Self {
        Self { direction }
    }
}

impl Packet for RunPacket {
    fn packet_id(&self) -> u16 {
        ClientOpcode::Run as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = vec![self.direction as u8];
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 攻击包（客户端 -> 服务端）
///
/// 载荷：[direction: u8][spell: u8]
#[derive(Debug, Clone)]
pub struct AttackPacket {
    pub direction: MirDirection,
    pub spell: Spell,
}

impl AttackPacket {
    pub fn new(direction: MirDirection, spell: Spell) -> Self {
        Self { direction, spell }
    }
}

impl Packet for AttackPacket {
    fn packet_id(&self) -> u16 {
        ClientOpcode::Attack as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = vec![self.direction as u8, self.spell as u8];
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 聊天包（客户端 -> 服务端）
///
/// 载荷：[message_len: u16 LE][message: u8[message_len]] (UTF-8)
#[derive(Debug, Clone)]
pub struct ChatPacket {
    pub message: String,
}

impl ChatPacket {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Packet for ChatPacket {
    fn packet_id(&self) -> u16 {
        ClientOpcode::Chat as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let msg_bytes = self.message.as_bytes();
        let msg_len = msg_bytes.len();

        let mut payload = Vec::with_capacity(2 + msg_len);
        payload.extend_from_slice(&(msg_len as u16).to_le_bytes());
        payload.extend_from_slice(msg_bytes);

        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 转向包（客户端 -> 服务端）
///
/// 载荷：[direction: u8]
#[derive(Debug, Clone)]
pub struct TurnPacket {
    pub direction: MirDirection,
}

impl TurnPacket {
    pub fn new(direction: MirDirection) -> Self {
        Self { direction }
    }
}

impl Packet for TurnPacket {
    fn packet_id(&self) -> u16 {
        ClientOpcode::Turn as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = vec![self.direction as u8];
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 登出包（客户端 -> 服务端）
///
/// 空载荷
#[derive(Debug, Clone)]
pub struct LogOutPacket;

impl Packet for LogOutPacket {
    fn packet_id(&self) -> u16 {
        ClientOpcode::LogOut as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        PacketCodec::encode(self.packet_id(), &[])
    }
}

/// 移动物品包（客户端 -> 服务端）
///
/// 载荷：[grid_from: u8][grid_to: u8]
#[derive(Debug, Clone)]
pub struct MoveItemPacket {
    pub grid_from: u8,
    pub grid_to: u8,
}

impl MoveItemPacket {
    pub fn new(grid_from: u8, grid_to: u8) -> Self {
        Self { grid_from, grid_to }
    }
}

impl Packet for MoveItemPacket {
    fn packet_id(&self) -> u16 {
        ClientOpcode::MoveItem as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = vec![self.grid_from, self.grid_to];
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 拾取物品包（客户端 -> 服务端）
///
/// 空载荷
#[derive(Debug, Clone)]
pub struct PickUpPacket;

impl Packet for PickUpPacket {
    fn packet_id(&self) -> u16 {
        ClientOpcode::PickUp as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        PacketCodec::encode(self.packet_id(), &[])
    }
}

/// 仓库操作包（客户端 -> 服务端）
///
/// 空载荷
#[derive(Debug, Clone)]
pub struct UserStoragePacket;

impl Packet for UserStoragePacket {
    fn packet_id(&self) -> u16 {
        ClientOpcode::ChangeAMode as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        PacketCodec::encode(self.packet_id(), &[])
    }
}

// ============================================================
// 登录与账户相关包
// ============================================================

/// 登录包（客户端 -> 服务端）
///
/// 载荷：[username_len: u16 LE][username: u8[username_len]][password_len: u16 LE][password: u8[password_len]]
#[derive(Debug, Clone)]
pub struct LoginClientPacket {
    pub username: String,
    pub password: String,
}

impl LoginClientPacket {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

impl Packet for LoginClientPacket {
    fn packet_id(&self) -> u16 {
        ClientOpcode::Login as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let username_bytes = self.username.as_bytes();
        let password_bytes = self.password.as_bytes();

        let mut payload = Vec::with_capacity(2 + username_bytes.len() + 2 + password_bytes.len());
        payload.extend_from_slice(&(username_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(username_bytes);
        payload.extend_from_slice(&(password_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(password_bytes);

        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 新账号注册包（客户端 -> 服务端）
///
/// 载荷：[username_len: u16 LE][username][password_len: u16 LE][password]
#[derive(Debug, Clone)]
pub struct NewAccountClientPacket {
    pub username: String,
    pub password: String,
}

impl NewAccountClientPacket {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

impl Packet for NewAccountClientPacket {
    fn packet_id(&self) -> u16 {
        ClientOpcode::NewAccount as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let username_bytes = self.username.as_bytes();
        let password_bytes = self.password.as_bytes();

        let mut payload = Vec::with_capacity(2 + username_bytes.len() + 2 + password_bytes.len());
        payload.extend_from_slice(&(username_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(username_bytes);
        payload.extend_from_slice(&(password_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(password_bytes);

        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 修改密码包（客户端 -> 服务端）
///
/// 载荷：[old_len: u16 LE][old_pwd: u8[old_len]][new_len: u16 LE][new_pwd: u8[new_len]]
#[derive(Debug, Clone)]
pub struct ChangePasswordClientPacket {
    pub old_password: String,
    pub new_password: String,
}

impl ChangePasswordClientPacket {
    pub fn new(old_password: String, new_password: String) -> Self {
        Self { old_password, new_password }
    }
}

impl Packet for ChangePasswordClientPacket {
    fn packet_id(&self) -> u16 {
        ClientOpcode::ChangePassword as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let old_bytes = self.old_password.as_bytes();
        let new_bytes = self.new_password.as_bytes();

        let mut payload = Vec::with_capacity(2 + old_bytes.len() + 2 + new_bytes.len());
        payload.extend_from_slice(&(old_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(old_bytes);
        payload.extend_from_slice(&(new_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(new_bytes);

        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 创建角色包（客户端 -> 服务端）
///
/// 载荷：[name_len: u16 LE][name: u8[name_len]][class: u8][gender: u8]
#[derive(Debug, Clone)]
pub struct NewCharacterClientPacket {
    pub name: String,
    pub class: u8,
    pub gender: u8,
}

impl NewCharacterClientPacket {
    pub fn new(name: String, class: u8, gender: u8) -> Self {
        Self { name, class, gender }
    }
}

impl Packet for NewCharacterClientPacket {
    fn packet_id(&self) -> u16 {
        ClientOpcode::NewCharacter as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let name_bytes = self.name.as_bytes();

        let mut payload = Vec::with_capacity(2 + name_bytes.len() + 1 + 1);
        payload.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(name_bytes);
        payload.push(self.class);
        payload.push(self.gender);

        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 删除角色包（客户端 -> 服务端）
///
/// 载荷：[char_index: u32 LE]
#[derive(Debug, Clone)]
pub struct DeleteCharacterClientPacket {
    pub char_index: u32,
}

impl DeleteCharacterClientPacket {
    pub fn new(char_index: u32) -> Self {
        Self { char_index }
    }
}

impl Packet for DeleteCharacterClientPacket {
    fn packet_id(&self) -> u16 {
        ClientOpcode::DeleteCharacter as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = self.char_index.to_le_bytes().to_vec();
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 开始游戏包（客户端 -> 服务端）
///
/// 载荷：[char_index: u32 LE]
#[derive(Debug, Clone)]
pub struct StartGameClientPacket {
    pub char_index: u32,
}

impl StartGameClientPacket {
    pub fn new(char_index: u32) -> Self {
        Self { char_index }
    }
}

impl Packet for StartGameClientPacket {
    fn packet_id(&self) -> u16 {
        ClientOpcode::StartGame as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = self.char_index.to_le_bytes().to_vec();
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

// ============================================================
// NPC/商店相关包（T03）
// ============================================================

/// 呼叫 NPC 包（客户端 -> 服务端）
///
/// 载荷：[npc_id: u16 LE]
#[derive(Debug, Clone)]
pub struct CallNpcPacket {
    pub npc_id: u16,
}

impl CallNpcPacket {
    pub fn new(npc_id: u16) -> Self {
        Self { npc_id }
    }
}

impl Packet for CallNpcPacket {
    fn packet_id(&self) -> u16 {
        ClientOpcode::CallNPC as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = self.npc_id.to_le_bytes().to_vec();
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 购买物品包（客户端 -> 服务端）
///
/// 载荷：[npc_id: u16 LE][item_id: u16 LE][count: u16 LE]
#[derive(Debug, Clone)]
pub struct BuyItemPacket {
    pub npc_id: u16,
    pub item_id: u16,
    pub count: u16,
}

impl BuyItemPacket {
    pub fn new(npc_id: u16, item_id: u16, count: u16) -> Self {
        Self { npc_id, item_id, count }
    }
}

impl Packet for BuyItemPacket {
    fn packet_id(&self) -> u16 {
        ClientOpcode::BuyItem as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let mut payload = Vec::with_capacity(2 + 2 + 2);
        payload.extend_from_slice(&self.npc_id.to_le_bytes());
        payload.extend_from_slice(&self.item_id.to_le_bytes());
        payload.extend_from_slice(&self.count.to_le_bytes());
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 出售物品包（客户端 -> 服务端）
///
/// 载荷：[slot: u8][item_uid: u32 LE][count: u16 LE]
#[derive(Debug, Clone)]
pub struct SellItemPacket {
    pub slot: u8,
    pub item_uid: u32,
    pub count: u16,
}

impl SellItemPacket {
    pub fn new(slot: u8, item_uid: u32, count: u16) -> Self {
        Self { slot, item_uid, count }
    }
}

impl Packet for SellItemPacket {
    fn packet_id(&self) -> u16 {
        ClientOpcode::SellItem as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let mut payload = Vec::with_capacity(1 + 4 + 2);
        payload.push(self.slot);
        payload.extend_from_slice(&self.item_uid.to_le_bytes());
        payload.extend_from_slice(&self.count.to_le_bytes());
        PacketCodec::encode(self.packet_id(), &payload)
    }
}
