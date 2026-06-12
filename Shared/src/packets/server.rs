use crate::enums::{ChatType, MirClass, MirDirection, MirGender};
use crate::net::error::NetError;
use crate::net::packet_id::ServerOpcode;
use crate::packets::{Packet, PacketCodec};
use crate::types::Point;

/// 心跳回复包（服务端 -> 客户端）
///
/// 空载荷
#[derive(Debug, Clone)]
pub struct ServerKeepAlivePacket;

impl Packet for ServerKeepAlivePacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::KeepAlive as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        PacketCodec::encode(self.packet_id(), &[])
    }
}

/// 连接成功包（服务端 -> 客户端）
///
/// 空载荷（触发客户端进入握手流程）
#[derive(Debug, Clone)]
pub struct ServerConnectedPacket;

impl Packet for ServerConnectedPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::Connected as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        PacketCodec::encode(self.packet_id(), &[])
    }
}

/// 断开连接包（服务端 -> 客户端）
///
/// 载荷：[reason: u8]
#[derive(Debug, Clone)]
pub struct DisconnectPacket {
    pub reason: u8,
}

impl DisconnectPacket {
    pub fn new(reason: u8) -> Self {
        Self { reason }
    }
}

impl Packet for DisconnectPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::Disconnect as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = vec![self.reason];
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 玩家位置包（服务端 -> 客户端）
///
/// 载荷：[location_x: i32 LE][location_y: i32 LE][direction: u8]
#[derive(Debug, Clone)]
pub struct UserLocationPacket {
    pub location: Point,
    pub direction: MirDirection,
}

impl UserLocationPacket {
    pub fn new(location: Point, direction: MirDirection) -> Self {
        Self { location, direction }
    }
}

impl Packet for UserLocationPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::UserLocation as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let mut payload = Vec::with_capacity(9);
        payload.extend_from_slice(&self.location.x.to_le_bytes());
        payload.extend_from_slice(&self.location.y.to_le_bytes());
        payload.push(self.direction as u8);
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 对象玩家包（服务端 -> 客户端）
///
/// 载荷：[object_id: u32 LE][name_len: u16 LE][name: u8[name_len]][class: u8][gender: u8][location_x: i32 LE][location_y: i32 LE][direction: u8]
#[derive(Debug, Clone)]
pub struct ObjectPlayerPacket {
    pub object_id: u32,
    pub name: String,
    pub class: MirClass,
    pub gender: MirGender,
    pub location: Point,
    pub direction: MirDirection,
}

impl ObjectPlayerPacket {
    pub fn new(
        object_id: u32,
        name: String,
        class: MirClass,
        gender: MirGender,
        location: Point,
        direction: MirDirection,
    ) -> Self {
        Self {
            object_id,
            name,
            class,
            gender,
            location,
            direction,
        }
    }
}

impl Packet for ObjectPlayerPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::ObjectPlayer as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let name_bytes = self.name.as_bytes();
        let name_len = name_bytes.len();

        let mut payload = Vec::with_capacity(4 + 2 + name_len + 1 + 1 + 8 + 1);
        payload.extend_from_slice(&self.object_id.to_le_bytes());
        payload.extend_from_slice(&(name_len as u16).to_le_bytes());
        payload.extend_from_slice(name_bytes);
        payload.push(self.class as u8);
        payload.push(self.gender as u8);
        payload.extend_from_slice(&self.location.x.to_le_bytes());
        payload.extend_from_slice(&self.location.y.to_le_bytes());
        payload.push(self.direction as u8);
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 移除对象包（服务端 -> 客户端）
///
/// 载荷：[object_id: u32 LE]
#[derive(Debug, Clone)]
pub struct ObjectRemovePacket {
    pub object_id: u32,
}

impl ObjectRemovePacket {
    pub fn new(object_id: u32) -> Self {
        Self { object_id }
    }
}

impl Packet for ObjectRemovePacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::ObjectRemove as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = self.object_id.to_le_bytes().to_vec();
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 聊天消息包（服务端 -> 客户端）
///
/// 载荷：[message_len: u16 LE][message: u8[message_len]][chat_type: u8]
#[derive(Debug, Clone)]
pub struct ServerChatPacket {
    pub message: String,
    pub chat_type: ChatType,
}

impl ServerChatPacket {
    pub fn new(message: String, chat_type: ChatType) -> Self {
        Self { message, chat_type }
    }
}

impl Packet for ServerChatPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::Chat as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let msg_bytes = self.message.as_bytes();
        let msg_len = msg_bytes.len();

        let mut payload = Vec::with_capacity(2 + msg_len + 1);
        payload.extend_from_slice(&(msg_len as u16).to_le_bytes());
        payload.extend_from_slice(msg_bytes);
        payload.push(self.chat_type as u8);
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 游戏消息包（服务端 -> 客户端）
///
/// 载荷：[message_len: u16 LE][message: u8[message_len]][chat_type: u8][type: u8]
#[derive(Debug, Clone)]
pub struct GameMessagePacket {
    pub message: String,
    pub chat_type: ChatType,
    pub r#type: u8,
}

impl GameMessagePacket {
    pub fn new(message: String, chat_type: ChatType, r#type: u8) -> Self {
        Self {
            message,
            chat_type,
            r#type,
        }
    }
}

impl Packet for GameMessagePacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::SendOutputMessage as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let msg_bytes = self.message.as_bytes();
        let msg_len = msg_bytes.len();

        let mut payload = Vec::with_capacity(2 + msg_len + 2);
        payload.extend_from_slice(&(msg_len as u16).to_le_bytes());
        payload.extend_from_slice(msg_bytes);
        payload.push(self.chat_type as u8);
        payload.push(self.r#type);
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 地图信息包（服务端 -> 客户端）
///
/// 载荷：[map_id: u16 LE][width: u16 LE][height: u16 LE][title_len: u16 LE][title: u8[title_len]][filename_len: u16 LE][filename: u8[filename_len]]
#[derive(Debug, Clone)]
pub struct MapInformationPacket {
    pub map_id: u16,
    pub width: u16,
    pub height: u16,
    pub title: String,
    pub filename: String,
}

impl MapInformationPacket {
    pub fn new(map_id: u16, width: u16, height: u16, title: String, filename: String) -> Self {
        Self {
            map_id,
            width,
            height,
            title,
            filename,
        }
    }
}

impl Packet for MapInformationPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::MapInformation as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let title_bytes = self.title.as_bytes();
        let filename_bytes = self.filename.as_bytes();

        let mut payload = Vec::with_capacity(2 + 2 + 2 + 2 + title_bytes.len() + 2 + filename_bytes.len());
        payload.extend_from_slice(&self.map_id.to_le_bytes());
        payload.extend_from_slice(&self.width.to_le_bytes());
        payload.extend_from_slice(&self.height.to_le_bytes());
        payload.extend_from_slice(&(title_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(title_bytes);
        payload.extend_from_slice(&(filename_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(filename_bytes);
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 时间天气包（服务端 -> 客户端）
///
/// 载荷：[light: u8]
#[derive(Debug, Clone)]
pub struct TimeOfDayPacket {
    pub light: u8,
}

impl TimeOfDayPacket {
    pub fn new(light: u8) -> Self {
        Self { light }
    }
}

impl Packet for TimeOfDayPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::TimeOfDay as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = vec![self.light];
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

// ============================================================
// 登录与账户相关包
// ============================================================

/// 登录响应包（服务端 -> 客户端）
///
/// 载荷：[result: u8] — 0=成功, 1=失败, 2=账号不存在, 3=密码错误
#[derive(Debug, Clone)]
pub struct LoginResponsePacket {
    pub result: u8,
}

impl LoginResponsePacket {
    pub fn new(result: u8) -> Self {
        Self { result }
    }
}

impl Packet for LoginResponsePacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::Login as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = vec![self.result];
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 登录被封禁包（服务端 -> 客户端）
///
/// 空载荷
#[derive(Debug, Clone)]
pub struct LoginBannedPacket;

impl Packet for LoginBannedPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::LoginBanned as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        PacketCodec::encode(self.packet_id(), &[])
    }
}

/// 登录成功包（服务端 -> 客户端）
///
/// 载荷：[account_id: u32 LE][char_count: u8]
#[derive(Debug, Clone)]
pub struct LoginSuccessPacket {
    pub account_id: u32,
    pub char_count: u8,
}

impl LoginSuccessPacket {
    pub fn new(account_id: u32, char_count: u8) -> Self {
        Self { account_id, char_count }
    }
}

impl Packet for LoginSuccessPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::LoginSuccess as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let mut payload = Vec::with_capacity(5);
        payload.extend_from_slice(&self.account_id.to_le_bytes());
        payload.push(self.char_count);
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 创建角色响应包（服务端 -> 客户端）
///
/// 载荷：[result: u8]
#[derive(Debug, Clone)]
pub struct NewCharacterResponsePacket {
    pub result: u8,
}

impl NewCharacterResponsePacket {
    pub fn new(result: u8) -> Self {
        Self { result }
    }
}

impl Packet for NewCharacterResponsePacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::NewCharacter as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = vec![self.result];
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 创建角色成功包（服务端 -> 客户端）
///
/// 载荷：[name_len: u16 LE][name: u8[name_len]][class: u8][gender: u8][level: u16 LE][hp: u32 LE][mp: u32 LE][max_hp: u32 LE][max_mp: u32 LE][char_id: u32 LE]
#[derive(Debug, Clone)]
pub struct NewCharacterSuccessPacket {
    pub name: String,
    pub class: u8,
    pub gender: u8,
    pub level: u16,
    pub hp: u32,
    pub mp: u32,
    pub max_hp: u32,
    pub max_mp: u32,
    pub char_id: u32,
}

impl NewCharacterSuccessPacket {
    pub fn new(name: String, class: u8, gender: u8, level: u16, hp: u32, mp: u32, max_hp: u32, max_mp: u32) -> Self {
        Self {
            name, class, gender, level, hp, mp, max_hp, max_mp,
            char_id: 0, // 默认值，在登录列表时会被覆盖
        }
    }

    pub fn with_id(mut self, char_id: u32) -> Self {
        self.char_id = char_id;
        self
    }
}

impl Packet for NewCharacterSuccessPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::NewCharacterSuccess as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let name_bytes = self.name.as_bytes();

        let mut payload = Vec::with_capacity(2 + name_bytes.len() + 1 + 1 + 2 + 4 + 4 + 4 + 4 + 4);
        payload.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(name_bytes);
        payload.push(self.class);
        payload.push(self.gender);
        payload.extend_from_slice(&self.level.to_le_bytes());
        payload.extend_from_slice(&self.hp.to_le_bytes());
        payload.extend_from_slice(&self.mp.to_le_bytes());
        payload.extend_from_slice(&self.max_hp.to_le_bytes());
        payload.extend_from_slice(&self.max_mp.to_le_bytes());
        payload.extend_from_slice(&self.char_id.to_le_bytes());

        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 删除角色响应包（服务端 -> 客户端）
///
/// 载荷：[result: u8]
#[derive(Debug, Clone)]
pub struct DeleteCharacterResponsePacket {
    pub result: u8,
}

impl DeleteCharacterResponsePacket {
    pub fn new(result: u8) -> Self {
        Self { result }
    }
}

impl Packet for DeleteCharacterResponsePacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::DeleteCharacter as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = vec![self.result];
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 删除角色成功包（服务端 -> 客户端）
///
/// 载荷：[char_index: u32 LE]
#[derive(Debug, Clone)]
pub struct DeleteCharacterSuccessPacket {
    pub char_index: u32,
}

impl DeleteCharacterSuccessPacket {
    pub fn new(char_index: u32) -> Self {
        Self { char_index }
    }
}

impl Packet for DeleteCharacterSuccessPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::DeleteCharacterSuccess as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = self.char_index.to_le_bytes().to_vec();
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 开始游戏响应包（服务端 -> 客户端）
///
/// 载荷：[result: u8]
#[derive(Debug, Clone)]
pub struct StartGameResponsePacket {
    pub result: u8,
}

impl StartGameResponsePacket {
    pub fn new(result: u8) -> Self {
        Self { result }
    }
}

impl Packet for StartGameResponsePacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::StartGame as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = vec![self.result];
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 开始游戏被封禁包（服务端 -> 客户端）
///
/// 载荷：[reason_len: u16 LE][reason: u8[reason_len]]
#[derive(Debug, Clone)]
pub struct StartGameBannedPacket {
    pub reason: String,
}

impl StartGameBannedPacket {
    pub fn new(reason: String) -> Self {
        Self { reason }
    }
}

impl Packet for StartGameBannedPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::StartGameBanned as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let reason_bytes = self.reason.as_bytes();
        let mut payload = Vec::with_capacity(2 + reason_bytes.len());
        payload.extend_from_slice(&(reason_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(reason_bytes);
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 开始游戏延迟包（服务端 -> 客户端）
///
/// 载荷：[seconds: u32 LE]
#[derive(Debug, Clone)]
pub struct StartGameDelayPacket {
    pub seconds: u32,
}

impl StartGameDelayPacket {
    pub fn new(seconds: u32) -> Self {
        Self { seconds }
    }
}

impl Packet for StartGameDelayPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::StartGameDelay as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = self.seconds.to_le_bytes().to_vec();
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 新账号响应包（服务端 -> 客户端）
///
/// 载荷：[result: u8]
#[derive(Debug, Clone)]
pub struct NewAccountResponsePacket {
    pub result: u8,
}

impl NewAccountResponsePacket {
    pub fn new(result: u8) -> Self {
        Self { result }
    }
}

impl Packet for NewAccountResponsePacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::NewAccount as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = vec![self.result];
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 修改密码响应包（服务端 -> 客户端）
///
/// 载荷：[result: u8]
#[derive(Debug, Clone)]
pub struct ChangePasswordResponsePacket {
    pub result: u8,
}

impl ChangePasswordResponsePacket {
    pub fn new(result: u8) -> Self {
        Self { result }
    }
}

impl Packet for ChangePasswordResponsePacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::ChangePassword as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = vec![self.result];
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 修改密码被封禁包（服务端 -> 客户端）
///
/// 空载荷
#[derive(Debug, Clone)]
pub struct ChangePasswordBannedPacket;

impl Packet for ChangePasswordBannedPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::ChangePasswordBanned as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        PacketCodec::encode(self.packet_id(), &[])
    }
}

// ============================================================
// 游戏内包（T02+）
// ============================================================

/// 对象行走包（服务端 -> 客户端）
///
/// 载荷：[object_id: u32 LE][location_x: i32 LE][location_y: i32 LE][direction: u8]
#[derive(Debug, Clone)]
pub struct ObjectWalkPacket {
    pub object_id: u32,
    pub location: Point,
    pub direction: MirDirection,
}

impl ObjectWalkPacket {
    pub fn new(object_id: u32, location: Point, direction: MirDirection) -> Self {
        Self { object_id, location, direction }
    }
}

impl Packet for ObjectWalkPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::ObjectWalk as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let mut payload = Vec::with_capacity(4 + 4 + 4 + 1);
        payload.extend_from_slice(&self.object_id.to_le_bytes());
        payload.extend_from_slice(&self.location.x.to_le_bytes());
        payload.extend_from_slice(&self.location.y.to_le_bytes());
        payload.push(self.direction as u8);
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 对象转向包（服务端 -> 客户端）
///
/// 载荷：[object_id: u32 LE][direction: u8]
#[derive(Debug, Clone)]
pub struct ObjectTurnPacket {
    pub object_id: u32,
    pub direction: MirDirection,
}

impl ObjectTurnPacket {
    pub fn new(object_id: u32, direction: MirDirection) -> Self {
        Self { object_id, direction }
    }
}

impl Packet for ObjectTurnPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::ObjectTurn as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let mut payload = Vec::with_capacity(4 + 1);
        payload.extend_from_slice(&self.object_id.to_le_bytes());
        payload.push(self.direction as u8);
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 对象攻击包（服务端 -> 客户端）
///
/// 载荷：[object_id: u32 LE][direction: u8][spell: u8]
#[derive(Debug, Clone)]
pub struct ObjectAttackPacket {
    pub object_id: u32,
    pub direction: MirDirection,
    pub spell: u8,
}

impl ObjectAttackPacket {
    pub fn new(object_id: u32, direction: MirDirection, spell: u8) -> Self {
        Self { object_id, direction, spell }
    }
}

impl Packet for ObjectAttackPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::ObjectAttack as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let mut payload = Vec::with_capacity(4 + 1 + 1);
        payload.extend_from_slice(&self.object_id.to_le_bytes());
        payload.push(self.direction as u8);
        payload.push(self.spell);
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 对象受击包（服务端 -> 客户端）
///
/// 载荷：[object_id: u32 LE][attacker_id: u32 LE]
#[derive(Debug, Clone)]
pub struct ObjectStruckPacket {
    pub object_id: u32,
    pub attacker_id: u32,
}

impl ObjectStruckPacket {
    pub fn new(object_id: u32, attacker_id: u32) -> Self {
        Self { object_id, attacker_id }
    }
}

impl Packet for ObjectStruckPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::ObjectStruck as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let mut payload = Vec::with_capacity(4 + 4);
        payload.extend_from_slice(&self.object_id.to_le_bytes());
        payload.extend_from_slice(&self.attacker_id.to_le_bytes());
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 伤害指示包（服务端 -> 客户端）
///
/// 载荷：[object_id: u32 LE][damage: u32 LE][damage_type: u8]
#[derive(Debug, Clone)]
pub struct DamageIndicatorPacket {
    pub object_id: u32,
    pub damage: u32,
    pub damage_type: u8,
}

impl DamageIndicatorPacket {
    pub fn new(object_id: u32, damage: u32, damage_type: u8) -> Self {
        Self { object_id, damage, damage_type }
    }
}

impl Packet for DamageIndicatorPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::DamageIndicator as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let mut payload = Vec::with_capacity(4 + 4 + 1);
        payload.extend_from_slice(&self.object_id.to_le_bytes());
        payload.extend_from_slice(&self.damage.to_le_bytes());
        payload.push(self.damage_type);
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 拾取物品包（服务端 -> 客户端）
///
/// 载荷：[item_id: u32 LE][item_name_len: u16 LE][item_name: u8[item_name_len]]
#[derive(Debug, Clone)]
pub struct GainedItemPacket {
    pub item_id: u32,
    pub item_name: String,
}

impl GainedItemPacket {
    pub fn new(item_id: u32, item_name: String) -> Self {
        Self { item_id, item_name }
    }
}

impl Packet for GainedItemPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::GainedItem as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let name_bytes = self.item_name.as_bytes();
        let mut payload = Vec::with_capacity(4 + 2 + name_bytes.len());
        payload.extend_from_slice(&self.item_id.to_le_bytes());
        payload.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(name_bytes);
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 地面物品包（服务端 -> 客户端）
///
/// 载荷：[object_id: u32 LE][item_id: u16 LE][location_x: i32 LE][location_y: i32 LE]
#[derive(Debug, Clone)]
pub struct ObjectItemPacket {
    pub object_id: u32,
    pub item_id: u16,
    pub location: Point,
}

impl ObjectItemPacket {
    pub fn new(object_id: u32, item_id: u16, location: Point) -> Self {
        Self { object_id, item_id, location }
    }
}

impl Packet for ObjectItemPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::ObjectItem as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let mut payload = Vec::with_capacity(4 + 2 + 4 + 4);
        payload.extend_from_slice(&self.object_id.to_le_bytes());
        payload.extend_from_slice(&self.item_id.to_le_bytes());
        payload.extend_from_slice(&self.location.x.to_le_bytes());
        payload.extend_from_slice(&self.location.y.to_le_bytes());
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 对象怪物包（服务端 -> 客户端）
///
/// 载荷：[object_id: u32 LE][monster_id: u16 LE][name_len: u16 LE][name: u8[name_len]][location_x: i32 LE][location_y: i32 LE][direction: u8][max_hp: u32 LE]
#[derive(Debug, Clone)]
pub struct ObjectMonsterPacket {
    pub object_id: u32,
    pub monster_id: u16,
    pub name: String,
    pub location: Point,
    pub direction: MirDirection,
    pub max_hp: u32,
}

impl ObjectMonsterPacket {
    pub fn new(object_id: u32, monster_id: u16, name: String, location: Point, direction: MirDirection, max_hp: u32) -> Self {
        Self { object_id, monster_id, name, location, direction, max_hp }
    }
}

impl Packet for ObjectMonsterPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::ObjectMonster as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let name_bytes = self.name.as_bytes();
        let mut payload = Vec::with_capacity(4 + 2 + 2 + name_bytes.len() + 4 + 4 + 1 + 4);
        payload.extend_from_slice(&self.object_id.to_le_bytes());
        payload.extend_from_slice(&self.monster_id.to_le_bytes());
        payload.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(name_bytes);
        payload.extend_from_slice(&self.location.x.to_le_bytes());
        payload.extend_from_slice(&self.location.y.to_le_bytes());
        payload.push(self.direction as u8);
        payload.extend_from_slice(&self.max_hp.to_le_bytes());
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 获取经验包（服务端 -> 客户端）
///
/// 载荷：[amount: u32 LE]
#[derive(Debug, Clone)]
pub struct GainExperiencePacket {
    pub amount: u32,
}

impl GainExperiencePacket {
    pub fn new(amount: u32) -> Self {
        Self { amount }
    }
}

impl Packet for GainExperiencePacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::GainExperience as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = self.amount.to_le_bytes().to_vec();
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 角色死亡包（服务端 -> 客户端）
///
/// 载荷：[object_id: u32 LE]
#[derive(Debug, Clone)]
pub struct DeathPacket {
    pub object_id: u32,
}

impl DeathPacket {
    pub fn new(object_id: u32) -> Self {
        Self { object_id }
    }
}

impl Packet for DeathPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::Death as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = self.object_id.to_le_bytes().to_vec();
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 对象死亡包（服务端 -> 客户端）
///
/// 载荷：[object_id: u32 LE]
#[derive(Debug, Clone)]
pub struct ObjectDiedPacket {
    pub object_id: u32,
}

impl ObjectDiedPacket {
    pub fn new(object_id: u32) -> Self {
        Self { object_id }
    }
}

impl Packet for ObjectDiedPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::ObjectDied as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = self.object_id.to_le_bytes().to_vec();
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 地图切换包（服务端 -> 客户端）
///
/// 载荷：[map_id: u16 LE]
#[derive(Debug, Clone)]
pub struct MapChangedPacket {
    pub map_id: u16,
}

impl MapChangedPacket {
    pub fn new(map_id: u16) -> Self {
        Self { map_id }
    }
}

impl Packet for MapChangedPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::MapChanged as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = self.map_id.to_le_bytes().to_vec();
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 玩家等级变化包（服务端 -> 客户端）
///
/// 载荷：[level: u16 LE][experience: u64 LE]
#[derive(Debug, Clone)]
pub struct LevelChangedPacket {
    pub level: u16,
    pub experience: u64,
}

impl LevelChangedPacket {
    pub fn new(level: u16, experience: u64) -> Self {
        Self { level, experience }
    }
}

impl Packet for LevelChangedPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::LevelChanged as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let mut payload = Vec::with_capacity(2 + 8);
        payload.extend_from_slice(&self.level.to_le_bytes());
        payload.extend_from_slice(&self.experience.to_le_bytes());
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// 生命值变化包（服务端 -> 客户端）
///
/// 载荷：[hp: u32 LE][max_hp: u32 LE]
#[derive(Debug, Clone)]
pub struct HealthChangedPacket {
    pub hp: u32,
    pub max_hp: u32,
}

impl HealthChangedPacket {
    pub fn new(hp: u32, max_hp: u32) -> Self {
        Self { hp, max_hp }
    }
}

impl Packet for HealthChangedPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::HealthChanged as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let mut payload = Vec::with_capacity(4 + 4);
        payload.extend_from_slice(&self.hp.to_le_bytes());
        payload.extend_from_slice(&self.max_hp.to_le_bytes());
        PacketCodec::encode(self.packet_id(), &payload)
    }
}

// ============================================================
// NPC/商店相关包（T03）
// ============================================================

/// NPC 出售的商品项
#[derive(Debug, Clone)]
pub struct NPCGoodsItem {
    pub item_id: u16,
    pub price: u32,
    pub name: String,
}

/// NPC 商品列表包（服务端 -> 客户端）
///
/// 载荷：[npc_id: u16 LE][count: u16 LE][{item_id: u16 LE][price: u32 LE][name_len: u16 LE][name: u8[name_len]}] * count
#[derive(Debug, Clone)]
pub struct NPCGoodsPacket {
    pub npc_id: u16,
    pub goods: Vec<NPCGoodsItem>,
}

impl NPCGoodsPacket {
    pub fn new(npc_id: u16, goods: Vec<NPCGoodsItem>) -> Self {
        Self { npc_id, goods }
    }
}

impl Packet for NPCGoodsPacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::NPCGoods as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        // 计算总大小：npc_id(2) + count(2) + items...
        let mut payload = Vec::with_capacity(2 + 2);
        payload.extend_from_slice(&self.npc_id.to_le_bytes());
        payload.extend_from_slice(&(self.goods.len() as u16).to_le_bytes());

        for item in &self.goods {
            let name_bytes = item.name.as_bytes();
            payload.extend_from_slice(&item.item_id.to_le_bytes());
            payload.extend_from_slice(&item.price.to_le_bytes());
            payload.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
            payload.extend_from_slice(name_bytes);
        }

        PacketCodec::encode(self.packet_id(), &payload)
    }
}

/// NPC 响应包（服务端 -> 客户端）
///
/// 载荷：[response: u8]
#[derive(Debug, Clone)]
pub struct NPCResponsePacket {
    pub response: u8,
}

impl NPCResponsePacket {
    pub fn new(response: u8) -> Self {
        Self { response }
    }
}

impl Packet for NPCResponsePacket {
    fn packet_id(&self) -> u16 {
        ServerOpcode::NPCResponse as u16
    }

    fn encode(&self) -> Result<Vec<u8>, NetError> {
        let payload = vec![self.response];
        PacketCodec::encode(self.packet_id(), &payload)
    }
}
