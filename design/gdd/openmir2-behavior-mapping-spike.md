# OpenMir2 行为映射 Spike

> **Status**: In Design
> **Author**: hkm + Claude Code Game Studios
> **Last Updated**: 2026-06-03
> **Implements Pillar**: Primary — 传奇骨架，现代皮肤; Supports — 稳刷不断流 / 爆装有戏 / 每次登录都带走成长
> **Creative Director Review (CD-GDD-ALIGN)**: APPROVE 2026-06-03
> **Quick reference** — Layer: `Foundation / Spike` · Priority: `MVP` · Key deps: `None`

## Overview

OpenMir2 行为映射 Spike 是 Phase 1 的行为权威确认系统：它不实现完整 OpenMir2 兼容，也不直接产出可玩功能，而是通过只读分析 OpenMir2 原源码，定位移动、地图坐标、阻挡、攻击、死亡、掉落、拾取、背包、装备和最小协议入口的权威文件、关键函数、数据结构与消息常量，并把这些证据整理成后续 Godot GDScript 系统 GDD 可引用的映射表。它存在的目的是防止 Phase 1 离线刷怪爆装切片凭记忆或参考客户端误写规则；每个后续 MVP 系统都应能说明自己继承、简化或暂时排除哪些 OpenMir2 行为。该 Spike 的交付物是源码证据清单、MVP 行为范围、明确的“采用 / 简化 / 排除”决策，以及给地图、战斗、掉落、拾取、背包、装备、存档和未来协议系统使用的 provisional contracts。

## Player Fantasy

玩家不会直接接触 OpenMir2 行为映射 Spike；他们感受到的是后续系统没有破坏熟悉的传奇判断：这一格能不能走、这个距离能不能打到、怪物死亡后为什么会掉落、掉落物为什么能被拾取、装备穿上后为什么会变强。这个 Spike 的幻想目标是让玩家在更清晰、更现代的 Godot 客户端里，仍然相信自己多年形成的传奇直觉是有效的。

它保护的不是源码考据本身，而是“传奇骨架，现代皮肤”的底层可信感：客户端可以重做 UI、反馈、渲染和工程结构，但移动、攻击、死亡、掉落、拾取、背包、装备这些核心行为必须能追溯到 OpenMir2 原源码或被明确标记为 Phase 1 简化。玩家最终应感到这个新客户端更顺滑、更可读，但不是一个披着传奇皮的新 ARPG。

## Detailed Design

### Core Rules

#### 1. Spike Scope Rule

本 Spike 只负责产出 OpenMir2 行为映射证据和下游 GDD 可引用的 provisional contracts，不实现玩法、不编写 Godot 架构、不确认最终数值平衡。

本 Spike 必须覆盖 Phase 1 30 秒离线刷怪爆装 thin slice 直接依赖的行为域：

| 行为域 | Phase 1 目的 |
|---|---|
| 地图坐标 / 格子关系 | 确认移动、攻击距离、掉落位置、拾取距离使用的空间基础。 |
| 地图阻挡 / 可通行判断 | 确认哪些地图格、对象或状态会阻止移动。 |
| 移动入口 | 确认玩家和怪物移动请求如何进入 OpenMir2 行为链。 |
| 目标选择 / 攻击入口 | 确认攻击请求、目标合法性、方向或距离检查的源码位置。 |
| 伤害 / 受击 / 死亡入口 | 确认 HP 变化、死亡触发、死亡事件顺序的权威来源。 |
| 怪物生成 / 基础 AI | 确认普通怪从哪里生成，如何进入可攻击或可死亡状态。 |
| 掉落生成 | 确认怪物死亡到物品生成之间的源码入口和数据来源。 |
| 地面物品 / 拾取 | 确认地面物生命周期、拾取请求、拾取成功/失败的状态变化。 |
| 背包 / 物品实例 | 确认物品定义、物品实例、背包添加/删除入口。 |
| 装备 / 属性挂接 | 确认装备槽、穿戴/卸下入口、属性变化来源。 |
| 最小协议入口 | 只映射与上述行为相关的消息常量和处理入口，作为行为触发证据；Phase 1 不实现联网。 |

本 Spike 明确不覆盖完整联网架构、登录/选角、完整技能系统、NPC/商店、交易、行会、PvP/攻沙、复杂 AI、完整 OpenMir2 数据兼容、Godot 节点/脚本架构、UI/VFX/音频实现。

#### 2. Evidence Authority Rule

每条行为结论必须标记证据来源等级，且不得把参考实现或观察结果当作 OpenMir2 权威。

| 证据来源 | 等级 | 允许用途 |
|---|---|---|
| OpenMir2 原源码 | Tier 1 | 行为入口、状态变化顺序、条件判断、消息常量、数据结构的最高权威。 |
| MirServer 配置 / 数据文件 | Tier 2 | 可说明数据值、掉落表、怪物配置、物品配置；不能单独证明行为规则。 |
| mir2x / mir2x-v0.0.12 参考实现 | Tier 3 | 可辅助理解结构或命名；不得作为 OpenMir2 行为权威。 |
| MinimalMirClient / 客户端观察 | Tier 4 | 只用于验证现象或辅助定位；不得授权服务端行为规则。 |

若 OpenMir2 source 与其他资料冲突，以 OpenMir2 source 为准；若冲突无法解释，该行为域标记为 `Conflicting`，不得进入 downstream contract。

#### 3. Evidence Level Rule

每条 mapping item 必须标记证据成熟度：

| Level | Name | Definition | Allowed Use |
|---|---|---|---|
| E0 | Unverified Note | 只有猜测、文件名或待查线索。 | 不可用于后续 GDD。 |
| E1 | Symbol Located | 找到类、函数、常量或结构体，但未确认职责。 | 可用于任务拆分，不可作为行为规则。 |
| E2 | Responsibility Observed | 已能说明源码位置负责什么。 | 可写入 mapping table，但不能作为 Phase 1 规则合同。 |
| E3 | Flow Traced | 已追踪入口、关键调用链和输出影响。 | 可作为后续 GDD 的 provisional contract。 |
| E4 | Cross-Checked | 同一行为被多个源码位置、数据结构或配置相互印证。 | 可作为后续 GDD 默认权威参考。 |

Phase 1 必需行为至少达到 **E3** 才能被后续 GDD 当作规则引用。E2 及以下只能作为待查项或延后项。

#### 4. Mapping Table Schema Rule

每个行为域必须产出结构化映射记录，而不是散文结论。

| Field | Required | Description |
|---|---:|---|
| `behavior_domain` | Yes | Movement、Combat、Death、Drop、Pickup、Inventory、Equipment、Protocol 等。 |
| `phase1_relevance` | Yes | Required、Useful、Deferred、Excluded。 |
| `openmir2_source` | Yes | OpenMir2 源码文件路径；未找到则写 `Unconfirmed`。 |
| `symbols` | Yes | 类、函数、结构、常量、消息名。 |
| `evidence_level` | Yes | E0–E4。 |
| `trigger` | Yes | 输入、AI、死亡事件、拾取请求、装备操作等触发源。 |
| `preconditions` | Yes | 行为开始前必须成立的条件。 |
| `state_changes` | Yes | 成功后哪些状态改变。 |
| `failure_conditions` | Yes | 失败条件及失败结果。 |
| `messages_or_constants` | No | 相关协议消息、枚举、常量；离线 Phase 1 标记为 reference only。 |
| `decision` | Yes | Adopt、Simplify、Exclude、Defer。 |
| `decision_reason` | Yes | 为什么采用、简化、排除或延后。 |
| `confidence` | Yes | Confirmed、Partial、Unconfirmed、Conflicting。 |
| `downstream_contract` | Yes if Adopt/Simplify | 后续 GDD 可引用的 contract 名称。 |
| `open_questions` | No | 仍需追踪的问题。 |

#### 5. Adopt / Simplify / Exclude / Defer Rule

每个行为域必须做出明确决策：

| Decision | Meaning | Constraint |
|---|---|---|
| Adopt | Phase 1 保留 OpenMir2 行为语义。 | 必须有 E3 或 E4 证据。 |
| Simplify | 保留行为意图，但降低 Phase 1 实现复杂度。 | 必须说明保留什么直觉、删除什么复杂度。 |
| Exclude | 明确不进入 Phase 1。 | 必须说明不会破坏 30 秒刷装体验。 |
| Defer | 保留证据，未来系统重新打开。 | 必须说明未来由哪个系统接手。 |

默认策略：

- 坐标、阻挡、移动合法性、死亡触发、掉落触发、拾取合法性默认 **Adopt** 或 **Adopt with narrow simplification**。
- 完整网络、复杂 AI、技能、PvP、装备耐久、掉落归属保护、仓库、交易、商店默认 **Simplify / Exclude / Defer**。
- 若 Phase 1 简化会破坏玩家熟悉的传奇判断，必须暂停该行为域的后续 GDD，回到源码确认。

#### 6. No Behavior Value Invention Rule

本 Spike 不得发明 OpenMir2 源码中尚未确认的具体数值。除非 mapping item 达到 E3/E4 并记录源码依据，否则不得写入攻击距离、移动速度、掉率、背包格数、装备数值、刷新时间等具体值。

允许写“需映射攻击合法性入口”“需记录死亡到掉落的调用顺序”；不允许写“攻击距离为 X”“掉率为 Y”。

#### 7. Provisional Contract Rule

Adopt 或 Simplify 的行为域必须输出 provisional contract，供后续 GDD 引用。每个 contract 至少包含：

| Field | Description |
|---|---|
| `contract_name` | 例如 `Map Blocking Contract`、`Pickup Legality Contract`。 |
| `source_basis` | OpenMir2 文件、函数、结构或常量。 |
| `preserved_semantics` | Godot 后续系统必须保留的行为语义。 |
| `allowed_simplifications` | Phase 1 允许省略或简化的部分。 |
| `forbidden_changes` | 会破坏传奇直觉或兼容风险的改动。 |
| `dependent_gdds` | 应引用该合同的下游 GDD。 |
| `verification_need` | 后续需要单测、playtest 或源码复核的点。 |

本 Spike 至少应产出以下 contracts：

- `Map Coordinate Contract`
- `Map Blocking Contract`
- `Movement Request Contract`
- `Attack Flow Contract`
- `Damage Intake Source Contract`
- `Death Event Contract`
- `Monster Spawn Source Contract`
- `Drop Creation Contract`
- `Ground Item Contract`
- `Pickup Legality Contract`
- `Inventory Operation Contract`
- `Equipment Operation Contract`
- `Minimal Protocol Message Contract`

### States and Transitions

#### 1. Mapping Item Lifecycle

每条 mapping item 从待查线索到下游合同，必须经过以下状态：

| State | Meaning | Exit Condition |
|---|---|---|
| Candidate | 行为域或源码线索被列入调查范围。 | 找到至少一个源码入口或确认未找到。 |
| Located | 已定位文件和符号。 | 记录源码路径、符号名、证据类型。 |
| Interpreted | 已描述源码职责。 | 能说明该源码负责什么，且未扩展成设计猜测。 |
| Cross-Checked | 已检查调用点、数据结构、消息常量或配置关联。 | 至少一个相关证据互相印证。 |
| Decided | 已标记 Adopt / Simplify / Exclude / Defer。 | 决策原因写入 mapping table。 |
| Contracted | 已输出 provisional contract。 | 下游 GDD 可以引用该 contract 继续设计。 |

#### 2. Evidence Transition Rules

- `Candidate → Located`: 必须有明确文件路径和符号。
- `Located → Interpreted`: 必须能用一句话说明源码职责。
- `Interpreted → Cross-Checked`: 必须连接到至少一个调用点、数据结构、消息常量或配置来源。
- `Cross-Checked → Decided`: 必须结合 Phase 1 thin slice 范围做 Adopt / Simplify / Exclude / Defer。
- `Decided → Contracted`: 只有 Adopt 或 Simplify 可以进入 provisional contract；Exclude / Defer 进入 backlog notes。

#### 3. Failure States

| Failure State | Trigger | Required Handling |
|---|---|---|
| Source Missing | 核心行为域找不到 OpenMir2 源码入口。 | 标记 E0/E1；不得作为后续 GDD 规则。 |
| Ambiguous Responsibility | 函数职责不清或命名误导。 | 保留 open question，要求二次源码追踪。 |
| Conflicting Evidence | 源码、配置或参考实现看似表达不同规则。 | 标记 `Conflicting`；不做设计裁决。 |
| Out of Phase Scope | 行为与 Phase 1 30 秒离线刷装无直接关系。 | 标记 Defer 或 Exclude。 |
| Implementation Temptation | 讨论转向 Godot 节点、脚本、资源或架构。 | 拉回 evidence、contract、dependency 输出；实现方案留给 ADR。 |
| Confidence Too Low | 必需行为未达到 E3。 | 暂停该下游 GDD 的规则引用，回到源码确认。 |

#### 4. Behavior Domain Completion Criteria

每个行为域完成时必须至少拥有：

- 一个 primary OpenMir2 source entry；
- 一个 mapping table row；
- 一个 evidence level；
- 一个 Adopt / Simplify / Exclude / Defer decision；
- 一个 downstream GDD target；
- 若 Adopt / Simplify：一个 provisional contract；
- 若 Exclude / Defer：明确理由和未来接手系统。

### Interactions with Other Systems

#### 1. Owns / Reads / Writes / Emits / Listens

| Boundary | This Spike Means |
|---|---|
| Owns | 行为证据、源码引用、evidence level、Adopt/Simplify/Exclude/Defer 决策、provisional contracts。它不 owns 任何运行时游戏状态。 |
| Reads | OpenMir2 原源码、MirServer 配置/数据、mir2x 参考实现、MinimalMirClient 验证观察、systems index、Phase 1 thin slice 范围。 |
| Writes | 行为映射表、证据清单、contract notes、risk gates、open questions。它不写 Godot 实现、运行时资源或最终数据 schema。 |
| Emits | `behavior_confirmed`、`behavior_simplified`、`behavior_excluded`、`source_gap_found`、`conflict_found`、`risk_gate_triggered` 等设计流程信号；这些不是 Godot runtime signals。 |
| Listens | 项目支柱、Phase 1 MVP 范围、OpenMir2 源码可用性、下游 GDD 对行为合同的需求。 |

#### 2. Downstream Dependency Map

| Downstream System | This Spike Provides | This Spike Does Not Decide |
|---|---|---|
| 地图坐标 / 阻挡 / Y-sort 系统 | 坐标、阻挡、地图格、对象占位相关源码入口和 contract。 | Godot TileMapLayer、Y-sort、渲染排序实现。 |
| 角色属性系统 | OpenMir2 actor / character 属性结构来源。 | Phase 1 最终属性表、成长曲线、战力公式。 |
| 物品定义系统 | StdItem、UserItem、item id / instance 相关源码入口。 | Godot Resource 格式、最终物品库结构。 |
| 掉落表系统 | 怪物死亡、掉落表、地面生成相关源码入口。 | 具体 Phase 1 掉率和装备池。 |
| 点击移动系统 | 移动请求、方向、阻挡、坐标合同。 | 输入手感、路径规划算法、鼠标反馈。 |
| 交互目标 / 选择系统 | 攻击、拾取、移动等行为入口的优先级线索。 | 现代 UI hover、cursor、selection visual。 |
| 伤害计算系统 | 攻击入口、受击入口、伤害来源线索。 | 最终伤害公式和平衡数值。 |
| 生命 / 死亡 / 复活规则 | HP 变化、死亡触发、死亡事件顺序来源。 | Phase 1 失败惩罚和复活 UX。 |
| 基础战斗系统 | 攻击 flow、合法性检查、死亡链接。 | 战斗动画、命中特效、完整技能。 |
| 怪物生成 / AI 系统 | 生成入口、怪物对象、基础行为源码来源。 | Phase 1 场景布局、完整 AI 行为树。 |
| 掉落与拾取系统 | 掉落生成、地面物、拾取合法性、背包接收合同。 | 掉落 UI、音效、VFX 强度。 |
| 背包系统 | 背包操作入口、物品实例结构、添加/删除线索。 | 背包 UI 布局、排序、完整整理功能。 |
| 装备系统 | 装备槽、穿戴/卸下入口、属性挂接来源。 | Phase 1 装备槽数量、装备成长深度。 |
| 存档系统 | 哪些行为事实可能需要持久化。 | 存档格式、版本策略、文件 IO。 |
| 网络 / 最小协议系统 | 消息常量、协议入口、handler 与 gameplay command 映射。 | Socket 架构、packet serialization、服务端兼容承诺。 |

#### 3. Contract Priority Rule

如果下游 GDD 与本 Spike 输出冲突，处理优先级为：

1. E4 OpenMir2 contract；
2. E3 OpenMir2 contract；
3. Phase 1 thin slice scope；
4. 现代化 UX / feel 改进；
5. 未验证设计假设。

若确实需要改变 E3/E4 contract，必须显式记录为 `intentional divergence from OpenMir2`，说明原因、影响系统和验证方式。

#### 4. Cross-System Risk Flags

| Risk | Why It Matters | Handling |
|---|---|---|
| 坐标或阻挡规则错误 | 会级联污染移动、攻击距离、掉落位置、拾取距离。 | 坐标/阻挡合同优先达到 E3/E4。 |
| 战斗 flow 与伤害公式混写 | Spike 只定位来源，不设计公式。 | Combat / Damage GDD 单独定义公式。 |
| 掉落、背包、装备共用物品事实 | 多系统可能重复定义 item id、instance、属性。 | 后续物品定义 GDD 和 registry 统一事实。 |
| 协议入口诱导过早联网 | Phase 1 是离线 slice。 | 只做 message → gameplay command 映射，不设计网络架构。 |
| OpenMir2 命名与实际职责不完全一致 | 单看函数名可能误判行为。 | 以调用链、数据结构、消息常量交叉验证。 |

## Formulas

本 Spike 的公式用于衡量证据成熟度、行为域完成度、下游合同可引用性和风险门触发状态。它们不计算攻击距离、掉率、背包容量、刷新时间或任何未经 OpenMir2 源码确认的玩法值。

### 1. Evidence Readiness Score

The `evidence_readiness_score` formula is defined as:

`evidence_readiness_score = min(1.0, (evidence_level / 4) * source_authority_weight * conflict_multiplier)`

**Variables:**

| Variable | Symbol | Type | Range | Description |
|---|---|---|---|---|
| Evidence level | `evidence_level` | int | 0–4 | Mapping item 的 E0–E4 成熟度。 |
| Source authority weight | `source_authority_weight` | float | 0.25–1.0 | 最高证据来源权重：OpenMir2 原源码=1.0，MirServer 配置=0.75，mir2x 参考实现=0.5，MinimalMirClient/观察=0.25。 |
| Conflict multiplier | `conflict_multiplier` | float | 0.0 or 1.0 | 若存在未解决冲突则为 0.0；否则为 1.0。 |

**Output Range:** `0.0–1.0`。如果证据冲突未解决，分数强制为 `0.0`。
**Example:** E3 OpenMir2 原源码且无冲突：`min(1.0, (3 / 4) * 1.0 * 1.0) = 0.75`。

### 2. Domain Completion Status

The `domain_completion_status` formula is defined as:

`domain_completion_status = required_fields_complete / required_fields_total`

**Variables:**

| Variable | Symbol | Type | Range | Description |
|---|---|---|---|---|
| Required fields complete | `required_fields_complete` | int | 0–6 | 已完成的必需字段数量：primary source、mapping row、evidence level、decision、downstream GDD target、required contract/backlog handling。 |
| Required fields total | `required_fields_total` | int | 6 | 行为域完成判定所需字段总数。 |

**Output Range:** `0.0–1.0`。若工具输入异常，应 clamp 到 `0.0–1.0`。
**Example:** 已完成 5 个字段：`5 / 6 = 0.8333`。

### 3. Downstream Contract Readiness

The `downstream_contract_readiness` formula is defined as:

`downstream_contract_readiness = min(evidence_gate_pass, conflict_gate_pass, contract_fields_status)`

**Variables:**

| Variable | Symbol | Type | Range | Description |
|---|---|---|---|---|
| Evidence gate pass | `evidence_gate_pass` | int | 0 or 1 | Phase 1 Required 行为达到 E3/E4 则为 1；否则为 0。 |
| Conflict gate pass | `conflict_gate_pass` | int | 0 or 1 | 无 `Conflicting` 状态则为 1；存在未解决冲突则为 0。 |
| Contract fields status | `contract_fields_status` | float | 0.0–1.0 | Provisional contract 字段完成度：已填写字段数 / 必需字段数。 |

**Output Range:** `0.0–1.0`。任何关键 gate 失败都会让结果为 `0`。
**Example:** 证据和冲突 gate 通过，但 contract 字段完成 6/7：`min(1, 1, 0.8571) = 0.8571`。

### 4. Risk Gate Trigger

The `risk_gate_trigger` formula is defined as:

`risk_gate_trigger = max(required_evidence_gap, conflict_present, missing_primary_source, forbidden_invention_detected)`

**Variables:**

| Variable | Symbol | Type | Range | Description |
|---|---|---|---|---|
| Required evidence gap | `required_evidence_gap` | int | 0 or 1 | Phase 1 Required 行为低于 E3 时为 1；否则为 0。 |
| Conflict present | `conflict_present` | int | 0 or 1 | 证据状态为 `Conflicting` 时为 1；否则为 0。 |
| Missing primary source | `missing_primary_source` | int | 0 or 1 | 必需行为没有 OpenMir2 primary source entry 时为 1；否则为 0。 |
| Forbidden invention detected | `forbidden_invention_detected` | int | 0 or 1 | 出现未经 E3/E4 支撑的具体玩法值时为 1；否则为 0。 |

**Output Range:** `0 or 1`。`1` 表示必须暂停引用并回到源码复核。
**Example:** 有未解决冲突：`max(0, 1, 0, 0) = 1`。

## Edge Cases

- **If OpenMir2 原源码未找到某个 Phase 1 required 行为入口**: 该 mapping item 标记为 `Source Missing`，证据等级设为 E0/E1，`confidence` 设为 `Unconfirmed`，不得生成 downstream contract。
- **If OpenMir2 源码与 MirServer config / mir2x / MinimalMirClient 对同一行为给出冲突结论**: 以 OpenMir2 源码为准；若源码含义仍无法解释冲突，则该行为标记为 `Conflicting`，不得被下游 GDD 引用。
- **If MirServer config 中的数值或表结构与 OpenMir2 源码行为判断不一致**: config 只作为 Tier 2 数据来源记录，不能覆盖 Tier 1 源码行为；该差异必须写入 `open_questions` 或 `decision_reason`。
- **If MinimalMirClient 的现象或实现与 OpenMir2 源码冲突**: MinimalMirClient 只作为 Tier 4 verification-only 记录，不能改变 mapping decision，也不能作为服务端行为规则依据。
- **If 下游 GDD 尝试引用 E2 或更低证据等级的 Phase 1 required 行为**: 该引用无效；下游 GDD 必须暂停该规则定义，直到对应 mapping item 达到 E3+。
- **If 设计讨论需要攻击距离、移动速度、掉率、背包格数、装备属性等具体值但源码尚未 E3/E4 确认**: 不得填写具体数值；该字段必须写为 `Unconfirmed`、`TBD from source` 或进入 open question。
- **If 某行为不直接服务 Phase 1 30 秒离线刷怪爆装 thin slice**: 该行为必须标记为 `Defer` 或 `Exclude`，并说明未来由哪个系统重新接手。
- **If Spike 讨论开始转向 Godot 节点结构、GDScript 类设计、资源格式、网络架构或 UI 实现**: 该内容不得写入本 Spike 的行为规则；只能记录为后续 ADR、技术设计或对应系统 GDD 的待处理事项。
- **If OpenMir2 存在多个源码版本、分支或文件路径表达不同实现**: mapping item 必须记录所用版本 / 路径；若无法确认 Phase 1 应采用哪个版本，则标记为 `Conflicting` 或 `Unconfirmed`，不得输出 contract。
- **If 下游 GDD 的规则与本 Spike 的 E3/E4 contract 冲突**: 本 Spike contract 优先；若下游必须改变该规则，必须显式标记为 `intentional divergence from OpenMir2`，并记录原因、影响系统和验证方式。
- **If registry 中已有 cross-system entity / formula / item 与本 Spike mapping 或 contract 命名、含义、数值范围冲突**: 不得直接覆盖 registry；必须提出 registry update，并在批准前将该 mapping item 标记为 blocked / open question。

## Dependencies

### Upstream Dependencies

| Dependency | Type | Status | Interface |
|---|---|---|---|
| Game Concept | Hard | Exists | Provides core fantasy, pillars, Phase 1 thin-slice scope, and OpenMir2 authority requirement. |
| Systems Index | Hard | Exists | Provides priority, layer, downstream systems, and boundary requirements. |
| OpenMir2 Source Tree | Hard | Exists locally | Provides Tier 1 behavior evidence for mapping items and provisional contracts. |
| MirServer Config / Data | Soft | Exists locally | Provides Tier 2 data context for monsters, drops, item tables, and server parameters; cannot prove behavior alone. |
| mir2x Reference Source | Soft | Exists locally | Provides Tier 3 interpretation help only. |
| MinimalMirClient / Client Observation | Soft | Not authoritative | Verification-only; cannot authorize behavior rules. |

### Downstream Dependencies

| Downstream System | Dependency Type | What It Reads From This Spike | Blocking Condition |
|---|---|---|---|
| 地图坐标 / 阻挡 / Y-sort 系统 | Hard | `Map Coordinate Contract`, `Map Blocking Contract`, source paths for map/cell/blocking behavior. | Cannot define grid/blocking rules until required map contracts reach E3+. |
| 角色属性系统 | Hard | Actor / character stat source mapping, damage intake source contract, ownership notes. | Cannot define stat ownership if OpenMir2 actor/character data source remains unconfirmed. |
| 物品定义系统 | Hard | `StdItem` / item template / item instance source mapping. | Cannot define item template vs instance if source structures remain E2 or lower. |
| 掉落表系统 | Hard | Drop data source mapping, drop trigger source, config/source distinction. | Cannot define drop table authority if monster drop source is unconfirmed or conflicting. |
| 点击移动系统 | Hard | `Movement Request Contract`, coordinate and blocking contracts. | Cannot design movement acceptance/rejection rules without E3+ movement legality evidence. |
| 交互目标 / 选择系统 | Soft / Hard for pickup-combat overlap | Source mapping for move/attack/pickup entry points. | Must pause if click target priority would contradict mapped OpenMir2 behavior. |
| 伤害计算系统 | Hard | Attack/struck source mapping, damage intake source contract, no-value-invention rule. | Cannot write formulas as OpenMir2-derived until damage flow reaches E3+. |
| 生命 / 死亡 / 复活规则 | Hard | `Death Event Contract`, actor lifecycle evidence. | Cannot define death-to-drop order if death event order is unconfirmed. |
| 基础战斗系统 | Hard | `Attack Flow Contract`, attack validity evidence, death linkage. | Cannot author combat flow if attack entry and target legality remain below E3. |
| 怪物生成系统 | Hard | `Monster Spawn Source Contract`, MonGen source mapping. | Cannot define spawn source as OpenMir2-derived without primary source and config relation. |
| 怪物 AI / 行为系统 | Soft / Phase 1 narrow | MonsterObject / AnimalObject behavior source mapping. | Full AI remains simplified unless relevant behavior reaches E3+. |
| 掉落与拾取系统 | Hard | `Drop Creation Contract`, `Ground Item Contract`, `Pickup Legality Contract`. | Cannot define drop-to-pickup flow if ground item lifecycle or pickup legality is unconfirmed. |
| 背包系统 | Hard | `Inventory Operation Contract`, item instance source mapping. | Cannot define inventory add/remove rules if item instance structure remains unconfirmed. |
| 装备系统 | Hard | `Equipment Operation Contract`, slot/equip source mapping, stat hook notes. | Cannot define equip legality or stat delta as OpenMir2-derived until source reaches E3+. |
| 存档系统 | Soft / Boundary | Candidate persistent facts from mapped inventory/equipment/stat behaviors. | Save schema cannot claim OpenMir2 compatibility from this Spike alone. |
| 网络 / 最小协议系统 | Soft / Future | `Minimal Protocol Message Contract`, message constants and handlers. | Networking must not start architecture from this Spike alone; protocol mapping is reference-only for Phase 1. |

### Bidirectional Consistency Notes

- Downstream GDDs must cite this GDD when they adopt an E3/E4 provisional contract.
- If a downstream GDD intentionally diverges from an E3/E4 contract, it must record `intentional divergence from OpenMir2` with reason, affected systems, and verification plan.
- This GDD must not claim ownership of downstream runtime data; it only owns evidence, decisions, and provisional contracts.
- Systems index should be updated to link this GDD after it is completed.

## Tuning Knobs

| Tuning Knob | Default | Safe Range | Affects | Notes |
|---|---:|---:|---|---|
| `required_behavior_min_evidence_level` | E3 | E3–E4 | Phase 1 必需行为能否被下游 GDD 引用。 | 低于 E3 会让规则依据不足；E4 更稳但会拖慢设计。 |
| `useful_behavior_min_evidence_level` | E2 | E2–E4 | 非阻塞但有用行为是否可记录为待查或 Defer。 | Useful 行为可先 E2 记录，但不得成为 Phase 1 contract。 |
| `max_unconfirmed_required_behaviors` | 0 | 0–1 | 是否允许继续后续 GDD。 | 推荐保持 0；若设为 1，必须有明确 risk gate 和 owner。 |
| `contract_required_fields_total` | 7 | 6–9 | Provisional contract 完整度计算。 | 字段越多越稳，但写作成本更高。 |
| `mapping_domain_priority_order` | Core loop first | Fixed list | 调研顺序。 | 推荐顺序：坐标/阻挡 → 移动 → 攻击/死亡 → 掉落/拾取 → 背包/装备 → 刷怪/AI → 协议入口。 |
| `reference_source_allowed_for_decision` | false | false only | 是否允许 mir2x 或 MinimalMirClient 直接决定行为。 | 必须保持 false；改为 true 会违反 OpenMir2 source-first 规则。 |
| `intentional_divergence_requires_approval` | true | true only | 下游系统能否偏离 E3/E4 contract。 | 必须由用户或后续 review 明确接受。 |
| `forbidden_value_invention_policy` | strict | strict only | 是否允许未确认具体玩法值。 | 必须 strict；未达到 E3/E4 的数值只能写 `Unconfirmed` 或 open question。 |

### Non-Tunable Values

以下不是本 Spike 的 tuning knobs，必须由后续系统 GDD 或源码映射结果决定：

- 攻击距离、攻击速度、伤害范围；
- 移动速度、跑步规则、路径算法；
- 掉率、掉落数量、金币数量；
- 背包容量、堆叠上限；
- 装备属性、装备槽位数量；
- 怪物 HP、攻击、防御、刷新时间。

## Visual/Audio Requirements

本 Spike 不定义玩家可见的游戏内视觉、音频、VFX、动画或 UI 风格。它的输出是设计/调研文档，不产生资产规格，也不触发 `/asset-spec`。

唯一要求是报告可读性：所有 mapping table、evidence level、contract、risk gate 和 open question 必须用清晰表格呈现，使后续 GDD 作者能快速判断某个行为是否可引用、是否需要源码复核、是否已被 Adopt / Simplify / Exclude / Defer。

## UI Requirements

本 Spike 不定义游戏内 UI。后续 HUD、背包/装备 UI、掉落视觉反馈等系统不得从本 Spike 推导界面布局或交互样式；它们只能引用本 Spike 中的行为 contract，例如拾取合法性、装备操作来源、地面物生命周期。

如需要展示 mapping 结果，应使用文档表格或开发工具报告，不进入 Phase 1 玩家-facing UI 范围。

## Acceptance Criteria

1. **GIVEN** Spike 进入完成审查，**WHEN** QA 或设计审查者检查 Phase 1 required 行为域，**THEN** 每个 required 行为域都必须至少有一条 mapping table row，并包含明确的 `openmir2_source` 文件路径或 `Unconfirmed` 标记；任何写成 `Unconfirmed` 的 required 行为必须同时触发 risk gate。
2. **GIVEN** 任一 mapping table row 已被填写，**WHEN** 审查者核对该 row，**THEN** 必须能看到以下必填字段均非空：`behavior_domain`、`phase1_relevance`、`openmir2_source`、`symbols`、`evidence_level`、`trigger`、`preconditions`、`state_changes`、`failure_conditions`、`decision`、`decision_reason`、`confidence`；若 `decision` 为 `Adopt` 或 `Simplify`，还必须填写 `downstream_contract`。
3. **GIVEN** 某行为域被标记为 `phase1_relevance = Required`，**WHEN** 该行为域被后续 GDD 引用为规则依据，**THEN** 对应 mapping item 的 `evidence_level` 必须为 E3 或 E4；E0、E1、E2 条目只能作为待查证据或任务拆分线索，不得作为 Phase 1 规则合同。
4. **GIVEN** 某 required 行为域低于 E3、缺少 OpenMir2 primary source、存在 `Conflicting` 状态，或包含未经 E3/E4 支撑的具体玩法值，**WHEN** QA 计算或审查 `risk_gate_trigger`，**THEN** `risk_gate_trigger` 必须为 `1`，并且该行为域不得进入 downstream contract，也不得被后续 GDD 当作已确认规则引用。
5. **GIVEN** OpenMir2 原源码、MirServer 配置、mir2x 参考实现或 MinimalMirClient 之间存在行为差异，**WHEN** mapping item 记录该行为结论，**THEN** 必须明确遵守 source authority hierarchy：OpenMir2 原源码为 Tier 1 最高权威；MirServer 配置只能说明数据来源；mir2x 只能辅助理解；MinimalMirClient 只能 verification-only，不能授权服务端行为规则。
6. **GIVEN** 审查者发现 mapping item 引用了 MirServer、mir2x 或 MinimalMirClient，**WHEN** 该引用被用于行为决策或 downstream contract，**THEN** 文档必须同时记录对应 OpenMir2 Tier 1 source basis；否则该行为必须标记为 `Partial`、`Unconfirmed` 或 `Conflicting`，不得作为 Adopt contract。
7. **GIVEN** Spike 文档中出现攻击距离、移动速度、掉率、背包格数、装备数值、刷新时间等具体玩法值，**WHEN** QA 检查该数值来源，**THEN** 该数值必须能追溯到 E3/E4 OpenMir2 source evidence；若不能追溯，必须改为 `Unconfirmed`、`TBD from source` 或写入 `open_questions`，并触发 forbidden value invention risk。
8. **GIVEN** 某行为域的 `decision` 为 `Adopt`，**WHEN** 审查者检查其证据和说明，**THEN** 该行为必须达到 E3/E4，并说明 Phase 1 将保留的 OpenMir2 行为语义；若证据不足 E3，则该 decision 不合格。
9. **GIVEN** 某行为域的 `decision` 为 `Simplify`，**WHEN** 审查者检查 `decision_reason` 和 contract，**THEN** 文档必须同时说明“保留什么传奇直觉”和“删除或降低什么复杂度”；如果简化可能破坏移动、攻击、死亡、掉落、拾取、背包或装备的核心判断，必须标记为 risk / open question，而不能直接通过。
10. **GIVEN** 某行为域的 `decision` 为 `Exclude` 或 `Defer`，**WHEN** QA 检查该 row，**THEN** 必须说明为什么它不进入 Phase 1 30 秒离线刷怪爆装 thin slice，并记录未来由哪个系统或 GDD 接手；Exclude / Defer 条目不得输出 provisional contract。
11. **GIVEN** 任一 Adopt 或 Simplify 行为域进入 `Contracted` 状态，**WHEN** 审查者检查对应 provisional contract，**THEN** contract 必须至少包含 `contract_name`、`source_basis`、`preserved_semantics`、`allowed_simplifications`、`forbidden_changes`、`dependent_gdds`、`verification_need` 七个字段，且 `source_basis` 必须指向 OpenMir2 文件、函数、结构或常量。
12. **GIVEN** 后续 GDD 引用本 Spike 的 mapping 或 contract，**WHEN** 审查者检查引用关系，**THEN** 后续 GDD 必须引用 E3/E4 provisional contract 名称或明确 source mapping；如果后续规则与 E3/E4 contract 冲突，必须标记 `intentional divergence from OpenMir2`，并记录原因、影响系统和验证方式。
13. **GIVEN** 本 Spike 是 Foundation / Spike GDD，**WHEN** QA 审查交付物范围，**THEN** 文档不得要求实现 Godot 节点、GDScript 类、资源格式、UI screen、VFX、音频、玩家-facing UI 或 gameplay code；验收对象仅限源码证据、mapping table、risk gate、contracts、downstream dependency notes 和 open questions。
14. **GIVEN** Spike 完成前仍存在未确认源码路径、冲突证据、E2 误用风险、source/config mismatch、MinimalMirClient conflict 或 registry 命名/事实冲突，**WHEN** 审查者检查 `Open Questions` 章节，**THEN** 每个未解决问题必须有明确的问题描述、受影响行为域、当前阻塞原因、需要补充的证据类型，以及它是否阻止 downstream contract 引用。

## Open Questions

| Question | Affected Domain | Current Blocker | Evidence Needed | Blocks Downstream Contract? | Owner |
|---|---|---|---|---|---|
| OpenMir2 中地图坐标、方向、阻挡、对象占格的最小权威链路是什么？ | Map / Movement | 只定位到候选文件，尚未追踪调用链。 | `Envirnoment.cs`、`MapCellInfo.cs`、`BaseObject.cs`、`PlayObject.Base.cs` 的 E3+ 追踪。 | Yes | technical-director / gameplay-programmer |
| 玩家移动请求从消息入口到位置更新的完整顺序是什么？ | Movement / Protocol | 已定位 `CM_WALK` / `CM_RUN` 和 handler，但未确认所有 guard。 | `Messages.cs`、`PlayObject.Message.cs`、`PlayObject.Attack.cs`、`PlayObject.Base.cs` 调用链。 | Yes | gameplay-programmer |
| 普通攻击的合法性、伤害入口和受击入口分别在哪里？ | Combat | 已定位候选 attack files，但未确认 flow。 | `PlayObject.Attack.cs`、`BaseObject.Attack.cs`、`BaseObject.Base.cs` 的 E3+ 追踪。 | Yes | systems-designer / gameplay-programmer |
| HP 归零、死亡状态、对象移除和掉落触发的顺序是什么？ | Death / Loot | 死亡与掉落顺序影响多个下游系统。 | `DamageHealth`、`Die()`、`DieDropItems`、相关 actor state fields 的调用链。 | Yes | systems-designer |
| 怪物生成配置如何进入运行时 MonsterObject？ | Monster Spawn / AI | 已定位 MonGen，但未确认生成数据结构和 Phase 1 最小字段。 | `WorldServer.MonGen.cs`、`MonsterThread.cs`、`MonsterObject.cs`、MirServer MonGen config 关系。 | Yes | ai-programmer / gameplay-programmer |
| 怪物死亡到掉落物落地的调用链和地面位置规则是什么？ | Drop / Ground Item | 掉落位置和地面物生命周期未确认。 | `BaseObject.cs`、`BaseObject.Base.cs`、`Envirnoment.cs`、`MapItem.cs` 的 E3+ 追踪。 | Yes | economy-designer / gameplay-programmer |
| 拾取请求的合法性条件是什么？ | Pickup / Inventory | 距离、归属、背包容量、地面物删除顺序未确认。 | `ClientPickUpItem()`、`GetItem()`、`VisibleMapItem`、背包添加逻辑。 | Yes | economy-designer |
| 物品定义与物品实例的最小字段是什么？ | Item Definition / Inventory | `StdItem`、`UserItem`、client/server packets 关系未追踪。 | `GameItemSystem.cs`、`StdItem.cs`、`ClientUserItem.cs`、`ServerUserItem.cs`。 | Yes | economy-designer / systems-designer |
| 装备穿戴/卸下如何改变角色属性或外观？ | Equipment / Stats | 装备槽、穿戴合法性、属性挂接点未确认。 | `ClientTakeOnItems()`、`ClientTakeOffItems()`、`CheckTakeOnItems()`、`CharacterObject.cs`、`ItemLocation.cs`。 | Yes | systems-designer |
| Phase 1 离线 slice 需要保留哪些协议消息作为行为入口参考？ | Protocol | 协议只作为行为入口证据，范围需确认。 | `Messages.cs`、`PlayObject.Message.cs` 中 move/hit/pickup/takeon/takeoff 相关常量和 handler。 | Soft | technical-director |
| MirServer config 与 OpenMir2 source 的关系如何记录？ | Data Context | config 可提供数据，但不能证明行为。 | 每个 domain 对应 config path 与源码使用点的映射。 | Sometimes | producer / systems-designer |
| 资源 / 地图转换合法性是否影响本 Spike 的 OpenMir2 行为采用？ | Asset / Map Pipeline | 法律与转换路径仍未调查。 | 后续 `资源 / 地图转换管线 Spike` 输出。 | No for behavior contracts; Yes for production asset route | technical-director / art-director |
