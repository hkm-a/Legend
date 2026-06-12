import { ServerPacketIds } from '../../types/packets';
import { MirDirection, MirClass, MirGender, ChatType } from '../../types/enums';

// ========================================
// 服务端 -> 客户端 包接口定义 + 反序列化函数
// ========================================

/** 从 ArrayBuffer 中读取 u16 LE */
function readU16(data: DataView, offset: number): number {
  return data.getUint16(offset, true);
}

/** 从 ArrayBuffer 中读取 i32 LE */
function readI32(data: DataView, offset: number): number {
  return data.getInt32(offset, true);
}

/** 从 ArrayBuffer 中读取 u32 LE */
function readU32(data: DataView, offset: number): number {
  return data.getUint32(offset, true);
}

/** 从 ArrayBuffer 中读取带长度前缀的字符串 (u16 LE length + UTF-8) */
function readString(data: DataView, offset: number): { value: string; nextOffset: number } {
  const length = readU16(data, offset);
  const bytes = new Uint8Array(data.buffer, data.byteOffset + offset + 2, length);
  const value = new TextDecoder().decode(bytes);
  return { value, nextOffset: offset + 2 + length };
}

// ---- 包接口定义 ----

/** 玩家位置信息 */
export interface IUserLocation {
  location: { x: number; y: number };
  direction: MirDirection;
}

/** 对象玩家信息 */
export interface IObjectPlayer {
  object_id: number;
  name: string;
  class: MirClass;
  gender: MirGender;
  location: { x: number; y: number };
  direction: MirDirection;
}

/** 服务端聊天消息 */
export interface IServerChat {
  message: string;
  chat_type: ChatType;
}

/** 游戏消息 */
export interface IGameMessage {
  message: string;
  chat_type: ChatType;
  type: number;
}

/** 地图信息 */
export interface IMapInformation {
  map_id: number;
  width: number;
  height: number;
  title: string;
  filename: string;
}

/** 时间天气 */
export interface ITimeOfDay {
  light: number;
}

// ---- 登录与账户相关接口 ----

/** 登录结果 */
export interface ILoginResult {
  result: number;
}

/** 登录被封禁 */
export interface ILoginBanned {}

/** 登录成功 */
export interface ILoginSuccess {
  account_id: number;
  char_count: number;
}

/** 创建角色结果 */
export interface INewCharacterResult {
  result: number;
}

/** 角色信息 */
export interface ICharacterInfo {
  index: number;
  name: string;
  class: number;
  gender: number;
  level: number;
  hp: number;
  mp: number;
  max_hp: number;
  max_mp: number;
}

/** 创建角色成功 */
export interface INewCharacterSuccess {
  char_info: ICharacterInfo;
}

/** 删除角色结果 */
export interface IDeleteCharacterResult {
  result: number;
}

/** 删除角色成功 */
export interface IDeleteCharacterSuccess {
  char_index: number;
}

/** 开始游戏结果 */
export interface IStartGameResult {
  result: number;
}

/** 开始游戏被封禁 */
export interface IStartGameBanned {
  reason: string;
}

/** 开始游戏延迟 */
export interface IStartGameDelay {
  seconds: number;
}

/** 新账号结果 */
export interface INewAccountResult {
  result: number;
}

/** 修改密码结果 */
export interface IChangePasswordResult {
  result: number;
}

/** 修改密码被封禁 */
export interface IChangePasswordBanned {}

// ========================================
// 游戏相关包接口
// ========================================

/** 用户完整信息 */
export interface IUserInformation {
  object_id: number;
  name: string;
  char_class: number;
  gender: number;
  level: number;
  location: { x: number; y: number };
  direction: number;
  hp: number;
  max_hp: number;
  mp: number;
  max_mp: number;
  experience: number;
  max_experience: number;
  gold: number;
}

/** 怪物信息 */
export interface IObjectMonster {
  object_id: number;
  template_id: number;
  name: string;
  level: number;
  location: { x: number; y: number };
  direction: number;
  hp: number;
  max_hp: number;
  is_alive: boolean;
}

/** 对象移动 */
export interface IObjectWalk {
  object_id: number;
  location: { x: number; y: number };
  direction: number;
}

/** 地图变更 */
export interface IMapChanged {
  map_id: number;
  width: number;
  height: number;
  tiles: number[];
}

/** 对象传送进入 */
export interface IObjectTeleportIn {
  object_id: number;
  location: { x: number; y: number };
  direction: number;
}

/** 对象攻击 */
export interface IObjectAttack {
  object_id: number;
  direction: number;
  spell: number;
  target_id: number;
}

/** 伤害飘字 */
export interface IDamageIndicator {
  object_id: number;
  damage: number;
  damage_type: number;
}

/** 自身受击 */
export interface IStruck {
  damage: number;
}

/** 对象受击 */
export interface IObjectStruck {
  object_id: number;
  attacker_id: number;
  damage: number;
}

/** 血量变化 */
export interface IHealthChanged {
  hp: number;
  mp: number;
}

/** 自身死亡 */
export interface IDeath {}

/** 对象死亡 */
export interface IObjectDied {
  object_id: number;
  killer_id: number;
}

/** 获得经验 */
export interface IGainExperience {
  amount: number;
}

/** 等级变化 */
export interface ILevelChanged {
  level: number;
}

/** 地上物品 */
export interface IObjectItem {
  object_id: number;
  item_id: number;
  location: { x: number; y: number };
  name: string;
  image: number;
}

/** 获得物品 */
export interface IGainedItem {
  item_id: number;
  name: string;
  count: number;
  image: number;
  item_type: string;
}

/** 包解析函数注册表 */
const parsers: Map<number, (payload: ArrayBuffer) => any> = new Map();

// ---- 反序列化函数 ----

/** 解析 Connected 包（空载荷） */
function parseConnected(_payload: ArrayBuffer): {} {
  return {};
}

/** 解析 KeepAlive 包（空载荷） */
function parseKeepAlive(_payload: ArrayBuffer): {} {
  return {};
}

/** 解析 Disconnect 包 */
function parseDisconnect(payload: ArrayBuffer): { reason: number } {
  const view = new DataView(payload);
  return { reason: view.getUint8(0) };
}

/** 解析 UserLocation 包 */
function parseUserLocation(payload: ArrayBuffer): IUserLocation {
  const view = new DataView(payload);
  const x = readI32(view, 0);
  const y = readI32(view, 4);
  const direction = view.getUint8(8) as MirDirection;
  return { location: { x, y }, direction };
}

/** 解析 ObjectPlayer 包 */
function parseObjectPlayer(payload: ArrayBuffer): IObjectPlayer {
  const view = new DataView(payload);
  let offset = 0;

  const object_id = readU32(view, offset);
  offset += 4;

  const { value: name, nextOffset } = readString(view, offset);
  offset = nextOffset;

  const cls = view.getUint8(offset) as MirClass;
  offset += 1;

  const gender = view.getUint8(offset) as MirGender;
  offset += 1;

  const x = readI32(view, offset);
  offset += 4;

  const y = readI32(view, offset);
  offset += 4;

  const direction = view.getUint8(offset) as MirDirection;

  return { object_id, name, class: cls, gender, location: { x, y }, direction };
}

/** 解析 ObjectRemove 包 */
function parseObjectRemove(payload: ArrayBuffer): { object_id: number } {
  const view = new DataView(payload);
  return { object_id: readU32(view, 0) };
}

/** 解析 Chat 包 */
function parseChat(payload: ArrayBuffer): IServerChat {
  const view = new DataView(payload);
  const { value: message, nextOffset } = readString(view, 0);
  const chat_type = view.getUint8(nextOffset) as ChatType;
  return { message, chat_type };
}

/** 解析 MapInformation 包 */
function parseMapInformation(payload: ArrayBuffer): IMapInformation {
  const view = new DataView(payload);
  let offset = 0;

  const map_id = readU16(view, offset);
  offset += 2;

  const width = readU16(view, offset);
  offset += 2;

  const height = readU16(view, offset);
  offset += 2;

  const { value: title, nextOffset: off1 } = readString(view, offset);
  offset = off1;

  const { value: filename, nextOffset: off2 } = readString(view, offset);
  offset = off2;

  return { map_id, width, height, title, filename };
}

/** 解析 TimeOfDay 包 */
function parseTimeOfDay(payload: ArrayBuffer): { light: number } {
  const view = new DataView(payload);
  return { light: view.getUint8(0) };
}

// ---- 登录与账户相关解析函数 ----

/** 解析 LoginResult 包 */
function parseLoginResult(payload: ArrayBuffer): ILoginResult {
  const view = new DataView(payload);
  return { result: view.getUint8(0) };
}

/** 解析 LoginBanned 包 */
function parseLoginBanned(_payload: ArrayBuffer): ILoginBanned {
  return {};
}

/** 解析 LoginSuccess 包 */
function parseLoginSuccess(payload: ArrayBuffer): ILoginSuccess {
  const view = new DataView(payload);
  return { account_id: readU32(view, 0), char_count: view.getUint8(4) };
}

/** 解析 NewCharacterResult 包 */
function parseNewCharacterResult(payload: ArrayBuffer): INewCharacterResult {
  const view = new DataView(payload);
  return { result: view.getUint8(0) };
}

/** 解析 NewCharacterSuccess 包 */
function parseNewCharacterSuccess(payload: ArrayBuffer): INewCharacterSuccess {
  const view = new DataView(payload);
  let offset = 0;
  const { value: name, nextOffset } = readString(view, offset);
  offset = nextOffset;
  const cls = view.getUint8(offset); offset += 1;
  const gender = view.getUint8(offset); offset += 1;
  const level = readU16(view, offset); offset += 2;
  const hp = readU32(view, offset); offset += 4;
  const mp = readU32(view, offset); offset += 4;
  const max_hp = readU32(view, offset); offset += 4;
  const max_mp = readU32(view, offset); offset += 4;
  const char_id = readU32(view, offset);
  return {
    char_info: { index: char_id, name, class: cls, gender, level, hp, mp, max_hp, max_mp },
  };
}

/** 解析 DeleteCharacterResult 包 */
function parseDeleteCharacterResult(payload: ArrayBuffer): IDeleteCharacterResult {
  const view = new DataView(payload);
  return { result: view.getUint8(0) };
}

/** 解析 DeleteCharacterSuccess 包 */
function parseDeleteCharacterSuccess(payload: ArrayBuffer): IDeleteCharacterSuccess {
  const view = new DataView(payload);
  return { char_index: readU32(view, 0) };
}

/** 解析 StartGameResult 包 */
function parseStartGameResult(payload: ArrayBuffer): IStartGameResult {
  const view = new DataView(payload);
  return { result: view.getUint8(0) };
}

/** 解析 StartGameBanned 包 */
function parseStartGameBanned(payload: ArrayBuffer): IStartGameBanned {
  const view = new DataView(payload);
  const { value: reason } = readString(view, 0);
  return { reason };
}

/** 解析 StartGameDelay 包 */
function parseStartGameDelay(payload: ArrayBuffer): IStartGameDelay {
  const view = new DataView(payload);
  return { seconds: readU32(view, 0) };
}

/** 解析 NewAccountResult 包 */
function parseNewAccountResult(payload: ArrayBuffer): INewAccountResult {
  const view = new DataView(payload);
  return { result: view.getUint8(0) };
}

/** 解析 ChangePasswordResult 包 */
function parseChangePasswordResult(payload: ArrayBuffer): IChangePasswordResult {
  const view = new DataView(payload);
  return { result: view.getUint8(0) };
}

/** 解析 ChangePasswordBanned 包 */
function parseChangePasswordBanned(_payload: ArrayBuffer): IChangePasswordBanned {
  return {};
}

// ---- 游戏相关解析函数 ----

/** 解析 UserInformation 包 */
function parseUserInformation(payload: ArrayBuffer): IUserInformation {
  const view = new DataView(payload);
  let offset = 0;

  const object_id = readU32(view, offset);
  offset += 4;

  const { value: name, nextOffset } = readString(view, offset);
  offset = nextOffset;

  const char_class = view.getUint8(offset);
  offset += 1;

  const gender = view.getUint8(offset);
  offset += 1;

  const level = readU16(view, offset);
  offset += 2;

  const x = readI32(view, offset);
  offset += 4;

  const y = readI32(view, offset);
  offset += 4;

  const direction = view.getUint8(offset);
  offset += 1;

  const hp = readU32(view, offset);
  offset += 4;

  const max_hp = readU32(view, offset);
  offset += 4;

  const mp = readU32(view, offset);
  offset += 4;

  const max_mp = readU32(view, offset);
  offset += 4;

  const experience = readU32(view, offset);
  offset += 4;

  const max_experience = readU32(view, offset);
  offset += 4;

  const gold = readU32(view, offset);

  return {
    object_id, name, char_class, gender, level,
    location: { x, y }, direction,
    hp, max_hp, mp, max_mp,
    experience, max_experience, gold,
  };
}

/** 解析 ObjectMonster 包 */
function parseObjectMonsterData(payload: ArrayBuffer): IObjectMonster {
  const view = new DataView(payload);
  let offset = 0;

  const object_id = readU32(view, offset);
  offset += 4;

  const template_id = readU16(view, offset);
  offset += 2;

  const { value: name, nextOffset } = readString(view, offset);
  offset = nextOffset;

  const level = readU16(view, offset);
  offset += 2;

  const x = readI32(view, offset);
  offset += 4;

  const y = readI32(view, offset);
  offset += 4;

  const direction = view.getUint8(offset);
  offset += 1;

  const hp = readU32(view, offset);
  offset += 4;

  const max_hp = readU32(view, offset);
  offset += 4;

  const is_alive = view.getUint8(offset) !== 0;

  return { object_id, template_id, name, level, location: { x, y }, direction, hp, max_hp, is_alive };
}

/** 解析 ObjectWalk 包 */
function parseObjectWalkData(payload: ArrayBuffer): IObjectWalk {
  const view = new DataView(payload);
  let offset = 0;

  const object_id = readU32(view, offset);
  offset += 4;

  const x = readI32(view, offset);
  offset += 4;

  const y = readI32(view, offset);
  offset += 4;

  const direction = view.getUint8(offset);

  return { object_id, location: { x, y }, direction };
}

/** 解析 MapChanged 包 */
function parseMapChangedData(payload: ArrayBuffer): IMapChanged {
  const view = new DataView(payload);
  let offset = 0;

  const map_id = readU16(view, offset);
  offset += 2;

  const width = readU16(view, offset);
  offset += 2;

  const height = readU16(view, offset);
  offset += 2;

  const tileCount = width * height;
  const tiles: number[] = [];

  for (let i = 0; i < tileCount; i++) {
    tiles.push(view.getUint8(offset));
    offset += 1;
  }

  return { map_id, width, height, tiles };
}

/** 解析 ObjectTeleportIn 包 */
function parseObjectTeleportInData(payload: ArrayBuffer): IObjectTeleportIn {
  const view = new DataView(payload);
  let offset = 0;

  const object_id = readU32(view, offset);
  offset += 4;

  const x = readI32(view, offset);
  offset += 4;

  const y = readI32(view, offset);
  offset += 4;

  const direction = view.getUint8(offset);

  return { object_id, location: { x, y }, direction };
}

/** 解析 ObjectAttack 包 */
function parseObjectAttackData(payload: ArrayBuffer): IObjectAttack {
  const view = new DataView(payload);
  let offset = 0;

  const object_id = readU32(view, offset);
  offset += 4;

  const direction = view.getUint8(offset);
  offset += 1;

  const spell = view.getUint8(offset);
  offset += 1;

  const target_id = readU32(view, offset);

  return { object_id, direction, spell, target_id };
}

/** 解析 DamageIndicator 包 */
function parseDamageIndicatorData(payload: ArrayBuffer): IDamageIndicator {
  const view = new DataView(payload);
  let offset = 0;

  const object_id = readU32(view, offset);
  offset += 4;

  const damage = readU32(view, offset);
  offset += 4;

  const damage_type = view.getUint8(offset);

  return { object_id, damage, damage_type };
}

/** 解析 Struck 包 */
function parseStruckData(payload: ArrayBuffer): IStruck {
  const view = new DataView(payload);
  return { damage: readU32(view, 0) };
}

/** 解析 ObjectStruck 包 */
function parseObjectStruckData(payload: ArrayBuffer): IObjectStruck {
  const view = new DataView(payload);
  let offset = 0;

  const object_id = readU32(view, offset);
  offset += 4;

  const attacker_id = readU32(view, offset);
  offset += 4;

  const damage = readU32(view, offset);

  return { object_id, attacker_id, damage };
}

/** 解析 HealthChanged 包 */
function parseHealthChangedData(payload: ArrayBuffer): IHealthChanged {
  const view = new DataView(payload);
  let offset = 0;

  const hp = readU32(view, offset);
  offset += 4;

  const mp = readU32(view, offset);

  return { hp, mp };
}

/** 解析 Death 包 */
function parseDeathData(_payload: ArrayBuffer): IDeath {
  return {};
}

/** 解析 ObjectDied 包 */
function parseObjectDiedData(payload: ArrayBuffer): IObjectDied {
  const view = new DataView(payload);
  let offset = 0;

  const object_id = readU32(view, offset);
  offset += 4;

  const killer_id = readU32(view, offset);

  return { object_id, killer_id };
}

/** 解析 GainExperience 包 */
function parseGainExperienceData(payload: ArrayBuffer): IGainExperience {
  const view = new DataView(payload);
  return { amount: readU32(view, 0) };
}

/** 解析 LevelChanged 包 */
function parseLevelChangedData(payload: ArrayBuffer): ILevelChanged {
  const view = new DataView(payload);
  return { level: readU16(view, 0) };
}

/** 解析 ObjectItem 包 */
function parseObjectItemData(payload: ArrayBuffer): IObjectItem {
  const view = new DataView(payload);
  let offset = 0;

  const object_id = readU32(view, offset);
  offset += 4;

  const item_id = readU16(view, offset);
  offset += 2;

  const x = readI32(view, offset);
  offset += 4;

  const y = readI32(view, offset);
  offset += 4;

  const { value: name, nextOffset } = readString(view, offset);
  offset = nextOffset;

  const image = readU16(view, offset);

  return { object_id, item_id, location: { x, y }, name, image };
}

/** 解析 GainedItem 包 */
function parseGainedItemData(payload: ArrayBuffer): IGainedItem {
  const view = new DataView(payload);
  let offset = 0;

  const item_id = readU16(view, offset);
  offset += 2;

  const { value: name, nextOffset } = readString(view, offset);
  offset = nextOffset;

  const count = readU16(view, offset);
  offset += 2;

  const image = readU16(view, offset);
  offset += 2;

  const item_type = view.getUint8(offset).toString();

  return { item_id, name, count, image, item_type };
}

// ---- 注册解析函数 ----

parsers.set(ServerPacketIds.Connected, parseConnected);
parsers.set(ServerPacketIds.KeepAlive, parseKeepAlive);
parsers.set(ServerPacketIds.Disconnect, parseDisconnect);
parsers.set(ServerPacketIds.UserLocation, parseUserLocation);
parsers.set(ServerPacketIds.ObjectPlayer, parseObjectPlayer);
parsers.set(ServerPacketIds.ObjectRemove, parseObjectRemove);
parsers.set(ServerPacketIds.Chat, parseChat);
parsers.set(ServerPacketIds.MapInformation, parseMapInformation);
parsers.set(ServerPacketIds.TimeOfDay, parseTimeOfDay);
parsers.set(ServerPacketIds.Login, parseLoginResult);
parsers.set(ServerPacketIds.LoginBanned, parseLoginBanned);
parsers.set(ServerPacketIds.LoginSuccess, parseLoginSuccess);
parsers.set(ServerPacketIds.NewCharacter, parseNewCharacterResult);
parsers.set(ServerPacketIds.NewCharacterSuccess, parseNewCharacterSuccess);
parsers.set(ServerPacketIds.DeleteCharacter, parseDeleteCharacterResult);
parsers.set(ServerPacketIds.DeleteCharacterSuccess, parseDeleteCharacterSuccess);
parsers.set(ServerPacketIds.StartGame, parseStartGameResult);
parsers.set(ServerPacketIds.StartGameBanned, parseStartGameBanned);
parsers.set(ServerPacketIds.StartGameDelay, parseStartGameDelay);
parsers.set(ServerPacketIds.NewAccount, parseNewAccountResult);
parsers.set(ServerPacketIds.ChangePassword, parseChangePasswordResult);
parsers.set(ServerPacketIds.ChangePasswordBanned, parseChangePasswordBanned);

// ---- 游戏相关解析函数注册 ----

parsers.set(ServerPacketIds.UserInformation, parseUserInformation);
parsers.set(ServerPacketIds.ObjectMonster, parseObjectMonsterData);
parsers.set(ServerPacketIds.ObjectWalk, parseObjectWalkData);
parsers.set(ServerPacketIds.MapChanged, parseMapChangedData);
parsers.set(ServerPacketIds.ObjectTeleportIn, parseObjectTeleportInData);
parsers.set(ServerPacketIds.ObjectAttack, parseObjectAttackData);
parsers.set(ServerPacketIds.DamageIndicator, parseDamageIndicatorData);
parsers.set(ServerPacketIds.Struck, parseStruckData);
parsers.set(ServerPacketIds.ObjectStruck, parseObjectStruckData);
parsers.set(ServerPacketIds.HealthChanged, parseHealthChangedData);
parsers.set(ServerPacketIds.Death, parseDeathData);
parsers.set(ServerPacketIds.ObjectDied, parseObjectDiedData);
parsers.set(ServerPacketIds.GainExperience, parseGainExperienceData);
parsers.set(ServerPacketIds.LevelChanged, parseLevelChangedData);
parsers.set(ServerPacketIds.ObjectItem, parseObjectItemData);
parsers.set(ServerPacketIds.GainedItem, parseGainedItemData);

/** 按 packet_id 查找并解析服务端数据包，返回解析后的对象 */
export function parseServerPacket(packet_id: number, payload: ArrayBuffer): any {
  const parser = parsers.get(packet_id);
  if (!parser) {
    console.warn(`[ServerPackets] No parser registered for packet_id=${packet_id}`);
    return null;
  }
  return parser(payload);
}
