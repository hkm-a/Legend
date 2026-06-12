# 端到端集成测试报告 — Crystal Mir2

**测试日期**: 2026-06-12
**测试人员**: QA 工程师 (software-qa-engineer-2)
**项目**: Crystal Mir2 (传奇2 Rust + TypeScript 重写)
**测试版本**: T02 桩阶段

---

## 1. 测试环境

| 项目 | 版本 |
|------|------|
| 操作系统 | Windows 11 (10.0.26100.8457) |
| Rust | 1.96.0 (ac68faa20 2026-05-25) |
| Cargo | 1.96.0 (30a34c682 2026-05-25) |
| Node.js | v22.22.2 |
| npm | 10.9.7 |
| Visual Studio Build Tools | 2022 (17.14.34, MSVC 14.44.35207) |
| 工具链 | stable-x86_64-pc-windows-msvc |
| 端口 7000 | 未被占用 ✓ |

---

## 2. 测试用例执行结果

### TC-001: 服务端编译

| 字段 | 内容 |
|------|------|
| **步骤** | 在项目根目录执行 `cargo build --bin mir2-server` |
| **预期结果** | 编译成功，生成 mir2-server.exe |
| **实际结果** | **编译失败** - `mir2-shared` crate 编译错误 |
| **状态** | **FAIL** |

**错误详情**:
`Shared\src\enums.rs` 中存在 194 个编译错误，主要包括：

1. **Unit-like enums 缺少 repr 注解** — `binrw` v0.15 要求 `#[br(repr = u8/u16/...)]` 或 `#[br(magic = ...)]` 在衍生 `BinRead`/`BinWrite` 的单元枚举上：

```
error: BinRead on unit-like enums requires either `#[br(repr = ...)]` on the enum
       or `#[br(magic = ...)]` on at least one variant
  --> Shared\src\enums.rs:6:75
```

涉及枚举：`MirDirection`, `ChatType`, `GameObjectType`, `ItemType`, `EquipmentSlot` 等。

2. **bitflags! 宏不满足 BinRead/BinWrite trait bound** — `binrw` 无法为 `bitflags!` 生成的 `InternalBitFlags` 类型实现 `BinRead`/`BinWrite`：

```
error[E0277]: the trait bound `enums::_::InternalBitFlags: BinRead` is not satisfied
```

涉及：`CellAttribute`, `LevelEffects`, `WeatherSetting`, `GuildRankOptions`, `GMOptions` 等。

**修复建议**:
- 对 `MirDirection`, `ChatType` 等简单枚举添加 `#[br(repr = u8)]` 和 `#[bw(repr = u8)]`
- 对 `bitflags!` 类型，移除 `BinRead`/`BinWrite` derive 或使用 `#[derive_read_write]` 替代

---

### TC-002: WebSocket 连接测试（测试脚本）

| 字段 | 内容 |
|------|------|
| **步骤** | 编写测试脚本 `test_ws.js`，连接 ws://localhost:7000 |
| **预期结果** | 可以连接并发送/接收二进制包 |
| **实际结果** | **无法执行** — 服务端未编译成功，无法启动 |
| **状态** | **BLOCKED** |

**已准备的测试脚本**: `test_ws.js`

协议格式验证通过代码审查：
- 包格式：`[PacketID: u16 LE][Length: u16 LE][Payload: u8[Length]]` (Header=4字节)
- 客户端 Packet IDs:
  - `KeepAlive = 2` (注意：任务描述中写的是 1002/0x03EA，实际代码中为 2)
  - `Walk = 11` (任务描述 1004/0x03EC)
  - `Chat = 13` (任务描述 1006/0x03EE)
  - `Run = 12`, `Turn = 10`, `Attack = 47`, `LogOut = 9`
- Walk 载荷: 1 字节方向 (0=Up, 1=UpRight, ..., 7=UpLeft)
- Chat 载荷: u16 LE 字符串长度 + UTF-8 字符串
- KeepAlive 载荷: 空

**任务描述中的 packet_id 与实际代码不符**，建议更新任务文档。

---

### TC-003: 客户端构建

| 字段 | 内容 |
|------|------|
| **步骤** | 在 `client/` 目录执行 `npx vite build` |
| **预期结果** | 构建成功，生成 dist/ 目录 |
| **实际结果** | **构建成功** ✓ |
| **状态** | **PASS** |

```
vite v6.4.3 building for production...
✓ 924 modules transformed.
✓ built in 6.80s
dist/index.html                   0.40 kB
dist/assets/index-CRIpFIcA.css    5.50 kB
dist/assets/index-Dqz14v7A.js   363.08 kB
```

---

### TC-004: 客户端开发服务器

| 字段 | 内容 |
|------|------|
| **步骤** | 在 `client/` 目录执行 `npm run dev` |
| **预期结果** | Vite 开发服务器启动成功 |
| **实际结果** | — |
| **状态** | **NOT TESTED** |

(需要依赖 TC-001 服务端可用才能验证连接状态，但前端服务器本身应该可以启动)

---

### TC-005: 协议格式审查

| 字段 | 内容 |
|------|------|
| **步骤** | 审查客户端和服务端编解码一致性 |
| **预期结果** | 客户端和服务端使用相同的包格式 |
| **实际结果** | **通过审查** ✓ |
| **状态** | **PASS** |

- 客户端 `codec.ts`: `[PacketID: u16 LE][Length: u16 LE][Payload: u8[Length]]`
- 服务端 `PacketCodec`: 使用 `PacketCodec::decode_payload` 和 `PacketCodec::decode_header`
- 两端一致 ✓

---

## 3. 问题列表

| ID | 严重度 | 描述 | 涉及文件 | 建议 |
|----|--------|------|----------|------|
| BUG-001 | **BLOCKER** | `mir2-shared` 编译失败 (194 错误) | `Shared/src/enums.rs` | 为 unit-like 枚举添加 `#[br(repr = u8)]`；为 bitflags 移除或重构 `BinRead`/`BinWrite` derive |
| BUG-002 | **MINOR** | 任务文档中 packet_id 与实际代码不符 | 任务描述 | 更新任务文档，将 KeepAlive=1002→2, Walk=1004→11, Chat=1006→13 |
| BUG-003 | **INFO** | MSVC link.exe 被 Git Bash 的 link.exe 遮蔽 | 构建环境 | 需先执行 `vcvars64.bat` 初始化 VS 环境后再编译 (已解决) |

---

## 4. 整体评估

| 项目 | 状态 |
|------|------|
| 服务端编译 | **FAIL** — `mir2-shared` 有 194 个编译错误 |
| 客户端构建 | **PASS** ✓ |
| WebSocket 连接测试 | **BLOCKED** — 服务端不可用 |
| 协议格式一致性 | **PASS** ✓ (代码审查) |
| **整体结论** | **FAIL** |

**总结**: 项目当前因 `mir2-shared` crate 的 `binrw` 兼容性问题无法编译。主要原因是 `binrw` v0.15 对 unit-like 枚举和 `bitflags!` 宏有更严格的要求。客户端 TypeScript 部分构建正常。建议优先修复 `Shared/src/enums.rs` 中的编译错误后重新测试。

---

## 5. 测试脚本

测试脚本已保存至 `test_ws.js`，待服务端就绪后可直接运行：

```bash
cd /c/Users/hkm/Documents/Code/crystal-mir2
node test_ws.js
```
