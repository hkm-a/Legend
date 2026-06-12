import { ClientPacketIds } from '../../types/packets';

/**
 * 客户端数据包接口
 */
export interface ClientPacket {
  packet_id(): number;
  serialize(): ArrayBuffer;
}

/**
 * 将字符串编码为 u16 长度前缀 + UTF-8 格式
 */
function encodeString(str: string): ArrayBuffer {
  const encoder = new TextEncoder();
  const bytes = encoder.encode(str);
  const buffer = new ArrayBuffer(2 + bytes.length);
  const view = new DataView(buffer);
  view.setUint16(0, bytes.length, true);
  new Uint8Array(buffer, 2).set(bytes);
  return buffer;
}

/** 心跳包（空载荷） */
export class KeepAliveClientPacket implements ClientPacket {
  packet_id(): number {
    return ClientPacketIds.KeepAlive;
  }
  serialize(): ArrayBuffer {
    return new ArrayBuffer(0);
  }
}

/** 客户端版本包 */
export class ClientVersionPacket implements ClientPacket {
  constructor(
    public version: [number, number, number, number] = [1, 0, 0, 0],
    public lang: number = 0
  ) {}

  packet_id(): number {
    return ClientPacketIds.ClientVersion;
  }

  serialize(): ArrayBuffer {
    const buffer = new ArrayBuffer(4 + 2 + 20);
    const view = new DataView(buffer);
    view.setUint8(0, this.version[0]);
    view.setUint8(1, this.version[1]);
    view.setUint8(2, this.version[2]);
    view.setUint8(3, this.version[3]);
    view.setUint16(4, this.lang, true);
    // pad 20 bytes already zeroed
    return buffer;
  }
}

/** 行走包 */
export class WalkPacket implements ClientPacket {
  constructor(public direction: number) {}

  packet_id(): number {
    return ClientPacketIds.Walk;
  }

  serialize(): ArrayBuffer {
    const buffer = new ArrayBuffer(1);
    new DataView(buffer).setUint8(0, this.direction);
    return buffer;
  }
}

/** 跑步包 */
export class RunPacket implements ClientPacket {
  constructor(public direction: number) {}

  packet_id(): number {
    return ClientPacketIds.Run;
  }

  serialize(): ArrayBuffer {
    const buffer = new ArrayBuffer(1);
    new DataView(buffer).setUint8(0, this.direction);
    return buffer;
  }
}

/** 攻击包 */
export class AttackPacket implements ClientPacket {
  constructor(public direction: number, public spell: number = 0) {}

  packet_id(): number {
    return ClientPacketIds.Attack;
  }

  serialize(): ArrayBuffer {
    const buffer = new ArrayBuffer(2);
    const view = new DataView(buffer);
    view.setUint8(0, this.direction);
    view.setUint8(1, this.spell);
    return buffer;
  }
}

/** 聊天包 */
export class ChatPacket implements ClientPacket {
  constructor(public message: string) {}

  packet_id(): number {
    return ClientPacketIds.Chat;
  }

  serialize(): ArrayBuffer {
    return encodeString(this.message);
  }
}

/** 转向包 */
export class TurnPacket implements ClientPacket {
  constructor(public direction: number) {}

  packet_id(): number {
    return ClientPacketIds.Turn;
  }

  serialize(): ArrayBuffer {
    const buffer = new ArrayBuffer(1);
    new DataView(buffer).setUint8(0, this.direction);
    return buffer;
  }
}

/** 登出包 */
export class LogOutClientPacket implements ClientPacket {
  packet_id(): number {
    return ClientPacketIds.LogOut;
  }

  serialize(): ArrayBuffer {
    return new ArrayBuffer(0);
  }
}

/** 拾取物品包 */
export class PickUpClientPacket implements ClientPacket {
  constructor(public objectId: number) {}

  packet_id(): number {
    return ClientPacketIds.PickUp;
  }

  serialize(): ArrayBuffer {
    const buffer = new ArrayBuffer(4);
    new DataView(buffer).setUint32(0, this.objectId, true);
    return buffer;
  }
}

/** 使用物品包 */
export class UseItemClientPacket implements ClientPacket {
  constructor(public uid: number) {}

  packet_id(): number {
    return ClientPacketIds.UseItem;
  }

  serialize(): ArrayBuffer {
    const buffer = new ArrayBuffer(4);
    new DataView(buffer).setUint32(0, this.uid, true);
    return buffer;
  }
}

/** 丢弃物品包 */
export class DropItemClientPacket implements ClientPacket {
  constructor(public uid: number, public count: number) {}

  packet_id(): number {
    return ClientPacketIds.DropItem;
  }

  serialize(): ArrayBuffer {
    const buffer = new ArrayBuffer(8);
    const view = new DataView(buffer);
    view.setUint32(0, this.uid, true);
    view.setUint32(4, this.count, true);
    return buffer;
  }
}

/** 移动物品包 */
export class MoveItemClientPacket implements ClientPacket {
  constructor(public fromSlot: number, public toSlot: number) {}

  packet_id(): number {
    return ClientPacketIds.MoveItem;
  }

  serialize(): ArrayBuffer {
    const buffer = new ArrayBuffer(8);
    const view = new DataView(buffer);
    view.setUint32(0, this.fromSlot, true);
    view.setUint32(4, this.toSlot, true);
    return buffer;
  }
}

// ========================================
// 登录与账户相关包
// ========================================

/** 登录包 */
export class LoginPacket implements ClientPacket {
  constructor(public username: string, public password: string) {}

  packet_id(): number {
    return ClientPacketIds.Login;
  }

  serialize(): ArrayBuffer {
    const userBytes = encodeString(this.username);
    const pwdBytes = encodeString(this.password);
    const buffer = new ArrayBuffer(userBytes.byteLength + pwdBytes.byteLength);
    new Uint8Array(buffer, 0, userBytes.byteLength).set(new Uint8Array(userBytes));
    new Uint8Array(buffer, userBytes.byteLength).set(new Uint8Array(pwdBytes));
    return buffer;
  }
}

/** 新账号注册包 */
export class NewAccountPacket implements ClientPacket {
  constructor(public username: string, public password: string) {}

  packet_id(): number {
    return ClientPacketIds.NewAccount;
  }

  serialize(): ArrayBuffer {
    const userBytes = encodeString(this.username);
    const pwdBytes = encodeString(this.password);
    const buffer = new ArrayBuffer(userBytes.byteLength + pwdBytes.byteLength);
    new Uint8Array(buffer, 0, userBytes.byteLength).set(new Uint8Array(userBytes));
    new Uint8Array(buffer, userBytes.byteLength).set(new Uint8Array(pwdBytes));
    return buffer;
  }
}

/** 修改密码包 */
export class ChangePasswordPacket implements ClientPacket {
  constructor(public oldPassword: string, public newPassword: string) {}

  packet_id(): number {
    return ClientPacketIds.ChangePassword;
  }

  serialize(): ArrayBuffer {
    const oldBytes = encodeString(this.oldPassword);
    const newBytes = encodeString(this.newPassword);
    const buffer = new ArrayBuffer(oldBytes.byteLength + newBytes.byteLength);
    new Uint8Array(buffer, 0, oldBytes.byteLength).set(new Uint8Array(oldBytes));
    new Uint8Array(buffer, oldBytes.byteLength).set(new Uint8Array(newBytes));
    return buffer;
  }
}

/** 创建角色包 */
export class NewCharacterPacket implements ClientPacket {
  constructor(public name: string, public charClass: number, public gender: number) {}

  packet_id(): number {
    return ClientPacketIds.NewCharacter;
  }

  serialize(): ArrayBuffer {
    const nameBytes = encodeString(this.name);
    const buffer = new ArrayBuffer(nameBytes.byteLength + 2);
    new Uint8Array(buffer, 0, nameBytes.byteLength).set(new Uint8Array(nameBytes));
    const view = new DataView(buffer, nameBytes.byteLength);
    view.setUint8(0, this.charClass);
    view.setUint8(1, this.gender);
    return buffer;
  }
}

/** 删除角色包 */
export class DeleteCharacterPacket implements ClientPacket {
  constructor(public charIndex: number) {}

  packet_id(): number {
    return ClientPacketIds.DeleteCharacter;
  }

  serialize(): ArrayBuffer {
    const buffer = new ArrayBuffer(4);
    new DataView(buffer).setUint32(0, this.charIndex, true);
    return buffer;
  }
}

/** 开始游戏包 */
export class StartGamePacket implements ClientPacket {
  constructor(public charIndex: number) {}

  packet_id(): number {
    return ClientPacketIds.StartGame;
  }

  serialize(): ArrayBuffer {
    const buffer = new ArrayBuffer(4);
    new DataView(buffer).setUint32(0, this.charIndex, true);
    return buffer;
  }
}
