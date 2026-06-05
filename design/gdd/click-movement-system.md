# 点击移动系统

> **Status**: In Design
> **Author**: hkm + Claude Code Game Studios
> **Last Updated**: 2026-06-05
> **Implements Pillar**: Primary — 稳刷不断流; Supports — 传奇骨架，现代皮肤
> **Quick reference** — Layer: `Core` · Priority: `MVP` · Key deps: `地图坐标 / 阻挡 / Y-sort 系统`

## Overview

点击移动系统是 Phase 1 离线刷怪爆装切片中把玩家鼠标意图转化为角色移动请求的核心操作层：玩家点击地面或可投影的世界位置后，系统通过已批准的输入投影、逻辑格、阻挡、距离事实与 MapSpaceState 预约/提交合同，尝试让角色朝目标移动，并在目标不可达、被阻挡、越界、规则未决或 UI 已消费输入时给出清晰反馈。该系统不拥有地图数据、阻挡真相、占位 mutation、目标选择优先级、战斗追击、拾取完成、AI 路径或 Godot Navigation authority；它负责表达“我要去那里”的玩家意图、选择 MVP 范围内可验证的移动策略、提交合法移动命令、替换或取消当前目的地，并让移动在低摩擦、可解释、可测试的边界内服务 `稳刷不断流`。实现细节必须遵守 ADR-0005 的 `MapProjection`、ADR-0019 的 `MovementLegalityService`、ADR-0003 的 `MapSpaceState` command queue，以及 ADR-0002 的 typed query result schema。

## Player Fantasy

玩家幻想是：**我点到哪，角色就稳稳走到哪；刷怪节奏不会被移动打断。**

点击移动不是高操作、高技巧的动作系统，而是现代化传奇 PC 客户端的基础交互层。玩家应该感觉自己在自然地指挥角色进入下一个有价值的位置：靠近怪物、走向掉落、调整站位、继续刷怪。每一次点击都应快速、清晰、可预期地转化为角色行动，让玩家的注意力停留在“打怪、爆装、拾取、判断价值、穿戴变强”的循环上，而不是停留在路径、阻挡、坐标或误点问题上。

本系统应保留经典传奇 PC 客户端的点击移动骨架：鼠标点击世界，角色遵守地图坐标、阻挡和占位规则移动。但现代化体验要求它更顺滑、更可理解、更少打断。当玩家看到怪物或掉落时，移动应该立即承接玩家的意图，让“我想过去”变成“角色正在过去”，并尽量避免等待、迷路、卡住或反馈不明造成的刷怪断流。

这个系统不追求闪避、连招、走位炫技或动作游戏式高压操作。它的成功标准是低压力、可预期、稳定、听话：玩家能用最少心智成本持续推进刷怪路线，并始终感觉自己在向下一个反馈点移动。

**设计测试：** 如果一个移动规则会让玩家更稳定、更少困惑地进入下一个刷怪、拾取或装备判断动作，它符合本系统幻想；如果它增加操作负担、制造动作技巧门槛，或让玩家频繁关注路径系统本身，它就偏离本系统幻想。

## Detailed Rules

### Core Rules

1. **Input eligibility rule**
   - 点击移动只响应项目配置中的 named input action（暂称 `move_click`，正式名称可在输入配置阶段批准），不得把裸鼠标按钮、键码或 Godot 事件函数本身作为设计规则来源。
   - `move_click` 可以绑定到鼠标左键，也可在未来绑定到其他 PC 输入方式；无论来源如何，进入世界移动前都必须通过 UI/input gate。
   - UI 已消费、覆盖、捕获、阻止或声明拥有该输入时，点击移动系统不得投影、不得生成移动请求、不得取消当前目的地。
   - Godot 4.6 的 mouse/touch focus 与 keyboard/gamepad focus 是独立事实；点击移动以当前输入事件的 UI ownership 为准，不能只因 keyboard focus 不在 UI 上就允许世界移动。
   - 当 UI/input gate 无法可靠判定事件归属时，MVP fail closed：不产生世界移动，只允许记录或发出结构化反馈。
   - Movement intent must be derived from the same input event whose action match and UI ownership were evaluated. Frame-level polling of action state alone must not create click movement intents. Godot implementation may use `_unhandled_input(event)` or an equivalent centralized input router that receives events only after UI consumption has been evaluated; if `_input(event)` is used, it must explicitly consult the approved UI/input gate before projection and must not bypass Control-owned events.

2. **Projection boundary rule**
   - 所有通过 input gate 的世界点击必须交给 `MapProjection` 转换为 logical `Vector2i` candidate。
   - 点击移动系统不得自行实现 screen/viewport/world 到 logical cell 的转换，不得读取 TileMapLayer 视觉格、physics raycast、Node2D position 或 Y-sort 结果作为 gameplay 坐标真相。
   - `MapProjection` 必须返回 ADR-0002/ADR-0005 定义的 typed `SpatialQueryResult`。只有结果为 allowed 且包含明确 logical cell candidate 时，系统才可继续移动评估。
   - `invalid_coordinate`、ambiguous projection、unknown/unloaded、out-of-bounds 或任何投影失败不得 fallback 到最近合法格、上一有效格、角色当前格或 `(0, 0)`。
   - 投影失败只触发 invalid/unavailable feedback，不改变当前有效移动路径。

3. **Movement request rule**
   - 每个点击移动请求至少包含：requesting actor id、active map id、source logical cell、target logical cell、request ordering fact，以及 request origin（MVP 为 player click）。
   - source logical cell 必须来自 `MapSpaceState` 或其一致性快照中的 authoritative committed occupancy，不得从 actor 的 Node2D/world/render position 反推。
   - target logical cell 必须来自 `MapProjection` 的 typed result。
   - movement request 只表达意图，不直接 mutation occupancy、reservation、actor position 或 map facts。

4. **Destination and path rule**
   - Phase 1 支持玩家点击远端 logical cell 作为 desired destination，以符合经典传奇式低摩擦点击移动。
   - 实际执行必须拆解为一系列正交单步；每一步的 target 必须是当前 committed cell 的 orthogonal neighbor。
   - Path planning / step selection 只能使用 read-only facts：`MapDefinition` static facts、`MapDistanceService` distance facts、`MovementLegalityService` evaluation，以及 `MapSpaceState` 的一致性 occupancy/reservation snapshot。
   - MVP 路径策略为 `mvp_provisional_orthogonal_only`：不允许 diagonal step，不允许 corner-cutting，不允许跨地图移动。
   - 若没有可用路径，系统发出 no-path feedback；若当前已有有效移动路径，则继续当前路径，不因失败点击而停止。
   - 自动 fallback 到“附近可走格”不属于 Phase 1；点击目标不可达时不得猜测替代目标。
   - Path search 的最大路径长度、最大搜索节点数或搜索预算属于 tuning knobs，必须数据驱动；达到上限视为 no-path/path-limit failure，而不是 silent failure。
   - A planned path is advisory after creation; every step must be revalidated against the latest approved snapshot/command result before reservation. Orthogonal neighbor expansion order must be deterministic and documented, or tests must assert path validity rather than exact route unless tie-break order is part of an approved contract.

5. **Single-step legality rule**
   - 每个实际 movement step 在提交 reservation 前必须调用 `MovementLegalityService.evaluate_single_step()` 或等价批准合同进行 read-only legality preflight。
   - `MovementLegalityService` 不拥有 path search、target fallback、occupancy mutation 或 reservation mutation。
   - same-cell target 视为 `NO_MOVEMENT_REQUESTED`：不创建 reservation，不提交 movement command，不进入 moving state。
   - MVP 中 same-cell click 默认不取消当前远端目的地；如需“停止移动”，应由未来明确的 stop action 或批准规则处理。
   - diagonal neighbor、corner-cutting、cross-map movement 或未批准 movement policy 必须返回 unresolved/blocked typed reason，不得临时允许。
   - Static blocked、actor occupied、reserved 等情况必须通过 ADR-0002 canonical reason 表达，不得使用本地字符串。

6. **Reservation and mutation rule**
   - `MapSpaceState` command queue 是 occupancy/reservation mutation 的唯一 authority。
   - 点击移动系统不得直接写 actor occupancy、reservation owner、source cell、target cell、created_sequence 或 conflict winner。
   - Legality preflight allowed 只表示“可以提交命令尝试”，不保证最终成功；最终结果以 `MapSpaceState` authoritative update 为准。
   - 每一步开始前提交 `RESERVE_MOVEMENT`；一步完成时提交 `COMMIT_MOVEMENT`；移动被取消、替换、actor 失效或地图切换时提交 `CANCEL_MOVEMENT`。
   - 如果 reservation/commit/cancel command 被拒绝、冲突、过期或 owner mismatch，系统必须停止当前不可信 step，保留最后 committed logical cell，并发出 typed failure feedback。On `COMMIT_MOVEMENT` failure, click movement must treat local step state as untrusted and consume/query the authoritative `MapSpaceState` result. If the failed commit leaves an active reservation, click movement must request `CANCEL_MOVEMENT` or wait for authoritative cleanup before starting another step; if MapSpaceState atomically cleared it, no extra cancel is submitted.
   - 两个 actor 同 tick 竞争同一 target cell 时，点击移动系统不得本地决定胜负；由 `MapSpaceState` deterministic ordering 处理。

7. **Repeated click and replacement rule**
   - 每次新的有效世界点击都可以替换当前 desired destination。
   - Phase 1 中，一旦 `RESERVE_MOVEMENT` 被接受，actor 即视为进入 `MovingStep`，即使 presentation 尚未 visibly advance；此后的 replacement click 不取消当前 reservation，而是记录 pending destination 并等待 commit。
   - During `MovingStep`, a valid replacement click is accepted as a pending destination after input eligibility and projection acceptance, but path validation is deferred until the current step successfully commits. The replan source is the newly committed logical cell, not the previous source or visual position. If post-commit replan to the pending destination fails, movement stops at the newly committed cell, emits no-path/path-limit feedback, and clears the pending destination.
   - `CANCEL_MOVEMENT` before movement starts applies only to reservations that have not yet entered `MovingStep` under an explicitly implemented pre-move state or to external cancellation flows.
   - 新点击若 projection invalid、UI consumed、blocked 或 no-path，不替换当前有效 path，不取消当前有效移动，只发出对应反馈。
   - 重复点击不叠加速度、不排队多个 destination、不触发 dash、dodge、combo 或任何动作游戏式特殊移动。

8. **Movement progress rule**
   - 角色按 logical cell step 前进，不直接 teleport 到远端 destination。
   - 视觉移动、动画、朝向、插值、脚步音和 destination marker 都是 presentation；gameplay authority 始终是 committed logical cell、active reservation 和 `MapSpaceState` command result。
   - actor Node2D/world/render position 与 logical state 短暂不同步时，以 logical state 为准。
   - Commit 失败时，presentation 必须纠正到最后 valid committed cell 对应的 render anchor；不得用视觉位置强行完成移动。
   - 移动速度必须来自外部配置或属性/规则输入，不得硬编码在点击移动规则中。

9. **Arrival and handoff rule**
   - 最后一段 path 成功 commit 后，系统发出 movement-arrived fact/event，并清除当前 movement intent。
   - 到达只表示移动完成，不表示攻击、拾取、对话、任务触发或交互成功。
   - Pickup、Combat Targeting、Quest、Tutorial、UI/VFX/Audio 等系统可以监听 arrival 或请求移动，但必须在自身系统中重新验证业务条件。

10. **Non-authority rule**
    - Godot Navigation、NavigationAgent2D、NavigationServer2D、Physics/Area2D/Raycast、CharacterBody2D collision、TileMapLayer collision/navigation/custom data、Node2D global position 和 Y-sort order 均不得作为点击移动 gameplay authority。
    - 这些系统可用于 presentation、debug、editor visualization 或未来非权威 helper，但任何实际 movement step 都必须重新落到 logical cell、`MovementLegalityService` 和 `MapSpaceState` command queue 合同上。

11. **Feedback rule**
    - invalid projection、same-cell/no movement、blocked static map、blocked by actor、reserved、no path、reservation failure、commit failure、cancel failure、unresolved movement rule 必须能被区分。
    - 点击移动系统只发布 typed feedback facts/reasons，不直接决定 UI 文案、音效、VFX 或 cursor 样式。
    - Feedback 不得 mutation gameplay state；视觉 destination marker 也不是 authoritative destination。

### States and Transitions

点击移动系统使用行为状态机描述可测试结果；GDD 不规定具体 GDScript enum、Node、Autoload 或 signal bus 实现。

| State | Meaning | Valid Transitions |
|---|---|---|
| `Idle` | 没有 active destination、active path 或 active reservation；actor 占据一个 committed logical cell。 | valid gated click → `Planning`; invalid/UI-consumed click → `Idle`; external disabled/map unload/entity invalid → `Disabled` or `Idle`。 |
| `Planning` | 已收到 input-gated 且 projection-accepted 的 movement intent，正在验证 same-cell、生成 orthogonal-only path 或下一步候选；尚未 mutation space。 | path found → `ReservingStep`; same-cell/no-path/blocked → `Blocked` or `Idle` with feedback; new valid click → replace intent and remain `Planning`; external cancel → `Cancelling`。 |
| `ReservingStep` | 已选择下一步正交 candidate，正在向 `MapSpaceState` command queue 提交或等待 `RESERVE_MOVEMENT` 结果。 | reservation accepted → `MovingStep`; reservation rejected → `Blocked`; replacement before movement starts → `Cancelling`; external cancel → `Cancelling`。 |
| `MovingStep` | 当前 step 已有 active reservation，presentation 正在表现角色朝 reserved target cell 移动。 | step presentation complete → `CommittingStep`; valid replacement click → record pending destination and remain `MovingStep`; invalid/blocked/no-path click → feedback and remain `MovingStep`; external cancel/actor invalid → `Cancelling`。 |
| `CommittingStep` | 当前 step 表现完成，正在提交或等待 `COMMIT_MOVEMENT` authoritative result。 | commit success + pending destination → `Planning`; commit success + remaining path → `ReservingStep`; commit success + final destination reached → `Arrived`; commit failed → `Blocked`; external invalidation → `Cancelling`。 |
| `Arrived` | 最终 destination 已成功到达并提交。 | emit movement-arrived fact/event, clear intent/path → `Idle`; downstream systems may start their own validation outside this system。 |
| `Blocked` | 当前 active request/path 因 canonical blocked/no-path/reservation/commit reason 不能继续，或一次 replacement intent 被拒绝。 | If blocking the active movement, emit typed feedback and clear failed active intent/path → `Idle`; if only a replacement click failed while an existing movement remains valid, emit feedback and return to the prior moving/planning state without clearing that movement. Valid new click → `Planning`。Phase 1 不自动重路由。 |
| `Unresolved` | 当前 request 触发未批准规则，如 diagonal、corner-cutting、cross-map、missing movement policy 或 evidence-gated behavior。 | emit unresolved feedback → `Idle`; valid new click → `Planning`; 只有新 ADR/evidence 批准后才可改变规则。 |
| `Cancelling` | 已请求取消 active movement/reservation，等待 `MapSpaceState` cancellation result。 | cancel success + queued replacement → `Planning`; cancel success without replacement → `Idle`; cancel failed/owner mismatch → `Blocked` or correction flow。 |
| `Disabled` | Actor 无法移动，例如死亡、地图切换、控制权丢失或未来状态效果禁止移动。 | regain movement eligibility → `Idle`; incoming clicks produce unable/unavailable feedback and do not plan movement。 |

Additional transition rules:

- Input eligibility and projection are pre-FSM gates: UI-consumed, input-ineligible, projection invalid, ambiguous, unknown/unloaded or out-of-bounds clicks do not enter `Planning` and do not change the current movement state/path.
- `Idle → Planning` 只允许发生在 input gate allowed 且 projection allowed 之后。
- `Planning → ReservingStep` 只允许发生在 path/step candidate 存在且 single-step legality preflight allowed 之后。
- `ReservingStep → MovingStep` 只允许发生在 `MapSpaceState` 接受 reservation 之后。
- `MovingStep → CommittingStep` 不代表 occupancy 已改变；只有 commit accepted 后 authoritative occupancy 才改变。
- `Blocked` 与 `Unresolved` 必须区分：blocked 表示当前规则下明确不可走；unresolved 表示规则/evidence 未批准。
- UI-consumed click、projection failure 或 no-path click 不得从任何 moving state 隐式取消当前有效 movement。

### Interactions with Other Systems

| System | Input to Click Movement | Output from Click Movement | Ownership Boundary |
|---|---|---|---|
| Input / UI Gate | Named `move_click` action plus event ownership result. | Movement intent only if world interaction is eligible. | Input owns remapping and event routing; UI gate owns whether input is consumed. Click movement must fail closed when ownership is unknown. |
| `MapProjection` | Screen/viewport/world point after UI gate. | Typed projection result with logical cell candidate or canonical failure. | Projection is the only coordinate conversion boundary. Click movement never performs local coordinate conversion or fallback. |
| `MapDefinition` | Static map facts consumed indirectly through legality/planning services. | None. | `MapDefinition` is static map truth. TileMapLayer visuals/editor data are not movement authority. |
| `MapDistanceService` | Source and target logical cells. | Distance facts: same-cell, orthogonal neighbor, diagonal neighbor, manhattan/chebyshev facts. | Distance facts are read-only; movement policy decides which facts are eligible. |
| `MovementLegalityService` | Actor id, source cell, candidate target cell, static facts, cell snapshot, movement policy. | Typed `SpatialQueryResult` for single-step eligibility. | Legality is read-only and never mutates occupancy/reservation. Phase 1 policy is `mvp_provisional_orthogonal_only`. |
| `MapSpaceState` | `RESERVE_MOVEMENT`, `COMMIT_MOVEMENT`, `CANCEL_MOVEMENT` commands. | Authoritative command results, occupancy/reservation snapshots and typed query results. | Sole mutation authority for actor occupancy and movement reservation. Preflight success cannot bypass command queue. |
| Movement Planner / Path Query | Desired destination plus read-only map, distance, legality and occupancy facts. | Ordered orthogonal cell path or typed path failure. | Planner may choose candidate steps but cannot mutate state, cannot use Godot Navigation as authority, and cannot invent fallback destinations. |
| Actor / Character Presentation | Next reserved/committed logical target and render anchor. | Step presentation complete notification. | Presentation animates/interpolates/rotates actor but does not decide legality or mutate occupancy. |
| Combat Targeting | Future request to move into attack range. | Movement arrived/failure facts. | Enemy clicks and attack decisions belong to Combat Targeting; click movement only executes approved movement intents. |
| Pickup / Ground Drop | Future request to move into pickup range. | Movement arrived/failure facts. | Click movement does not pick up items. Pickup revalidates item existence, range, inventory and ownership after arrival. |
| UI / VFX / Audio Feedback | Typed feedback facts and movement state facts. | Visual markers, cursor state, messages, sounds, debug overlay. | Feedback is presentation only and never changes movement legality, destination or occupancy. |
| Y-sort / Visual Sorting | Actor visual position or anchor for rendering. | Sorted visual order. | Y-sort is visual-only; it never affects projection, pathing, blocking, distance, occupancy or arrival. |
| Save/Load / Map Transition | External movement cancellation or actor/map availability changes. | Cancelled/blocked feedback and cleared movement intent. | Map unload, actor despawn or save/load transition must release reservations via `MapSpaceState`; click movement must not commit stale movement. |

## Formulas

点击移动系统不拥有角色成长、掉落概率、战斗伤害或装备价值公式。它只定义把点击意图转化为 movement request、path/step 选择、移动时长和失败分类所需的规则公式。所有公式必须使用 logical grid facts、typed query result 和外部配置；不得使用屏幕像素、Node2D position、TileMap visual data 或本地字符串作为 gameplay truth。

### Formula 1: Movement Intent Eligibility

**Purpose:** 判断一次输入是否可以进入世界点击移动流程。

```text
movement_intent_eligible =
    input_action_matched
    AND ui_gate_allows_world_interaction
    AND actor_can_accept_player_movement
    AND active_map_accepts_movement
```

| Variable | Type / Range | Meaning | Source |
|---|---|---|---|
| `input_action_matched` | Boolean | 当前输入是否匹配批准的 named movement action（MVP 暂称 `move_click`）。 | Input configuration / input routing |
| `ui_gate_allows_world_interaction` | Boolean | 当前输入事件是否未被 UI 消费且可解释为世界交互。Unknown/unresolved counts as `false` in MVP. | UI/input gate |
| `actor_can_accept_player_movement` | Boolean | Actor 是否存在、未死亡、未处于地图切换/控制权丢失/禁止移动状态。 | Actor/status/controller facts |
| `active_map_accepts_movement` | Boolean | 当前地图是否 loaded、active，且允许 player movement requests。 | Map/session state |

**Expected output:** Boolean. `true` allows projection; `false` produces no movement request.

**Example:**

```text
input_action_matched = true
ui_gate_allows_world_interaction = false  # inventory panel consumed click
actor_can_accept_player_movement = true
active_map_accepts_movement = true
movement_intent_eligible = false
```

Result: no projection call, no path change, no reservation.

### Formula 2: Projection Acceptance

**Purpose:** 判断 `MapProjection` 返回结果是否可作为 target logical cell。

```text
projection_accepted =
    projection_result.status == ALLOWED
    AND projection_result.has_logical_cell == true
    AND projection_result.primary_reason in approved_projection_success_reasons
```

Because ADR-0002 reasons are canonical and may represent coordinate conversion via specific enum values, implementation must use the approved typed coordinate-conversion success reason set from ADR-0005/ADR-0002, not local strings. Projection success is pure coordinate/candidate success and does not imply walkability. Any failure reason such as `INVALID_COORDINATE`, `UNKNOWN_OR_UNLOADED`, `OUT_OF_BOUNDS`, or ambiguous projection yields `projection_accepted = false`. Missing logical cell must be represented by an approved typed field such as `has_logical_cell`; `(0, 0)`, previous valid cell, or any sentinel coordinate is forbidden.

| Variable | Type / Range | Meaning | Source |
|---|---|---|---|
| `projection_result.status` | `SpatialQueryStatus` | Typed result status. | `MapProjection` |
| `projection_result.primary_reason` | `SpatialQueryReason` | Canonical coordinate-conversion reason. | `MapProjection` |
| `projection_result.has_logical_cell` | Boolean | Whether a candidate logical cell is present. | `MapProjection` / ADR-0005 schema |
| `projection_result.logical_cell` | `Vector2i` when `has_logical_cell == true`; ignored otherwise | Candidate logical cell. | `MapProjection` |
| `approved_projection_success_reasons` | Set of `SpatialQueryReason`; non-empty approved enum set | Canonical projection-success reasons accepted by ADR-0005/ADR-0002; must not contain local strings. | ADR-0005 / ADR-0002 |

**Expected output:** Boolean.

**Example:**

```text
projection_result.status = BLOCKED
projection_result.primary_reason = INVALID_COORDINATE
projection_result.logical_cell = absent
projection_accepted = false
```

Result: invalid feedback, current path unchanged.

### Formula 3: Same-Cell No Movement

**Purpose:** 将点击 actor 当前 committed cell 的情况分类为 no movement，而不是 blocked/path failure。

```text
same_cell_no_movement = (source_cell == target_cell)
```

| Variable | Type / Range | Meaning | Source |
|---|---|---|---|
| `source_cell` | `Vector2i` within active map bounds | Actor 当前 authoritative committed logical cell。 | `MapSpaceState` snapshot |
| `target_cell` | `Vector2i` within active map bounds | Projection accepted 后的 desired destination。 | `MapProjection` |

**Expected output:** Boolean. If `true`, return/emit `NO_MOVEMENT_REQUESTED`; do not reserve movement. MVP default: same-cell does not cancel an existing far destination.

**Example:**

```text
source_cell = Vector2i(10, 8)
target_cell = Vector2i(10, 8)
same_cell_no_movement = true
```

Result: no `RESERVE_MOVEMENT` command.

### Formula 4: Orthogonal Step Eligibility

**Purpose:** 判断一个 candidate step 是否满足 MVP 正交单步边界。

```text
orthogonal_step_candidate =
    distance_facts.is_orthogonal_neighbor
    AND NOT distance_facts.is_same_cell
    AND NOT distance_facts.is_diagonal_neighbor
```

| Variable | Type / Range | Meaning | Source |
|---|---|---|---|
| `distance_facts.is_orthogonal_neighbor` | Boolean | source 与 candidate target 是否为正交相邻。 | `MapDistanceService` |
| `distance_facts.is_same_cell` | Boolean | source 与 candidate target 是否相同。 | `MapDistanceService` |
| `distance_facts.is_diagonal_neighbor` | Boolean | source 与 candidate target 是否为 diagonal neighbor。 | `MapDistanceService` |

**Expected output:** Boolean. `true` means the candidate can proceed to `MovementLegalityService`; `false` means it is not an executable MVP step.

**Example:**

```text
source_cell = Vector2i(10, 8)
candidate_step = Vector2i(11, 8)
is_orthogonal_neighbor = true
is_same_cell = false
is_diagonal_neighbor = false
orthogonal_step_candidate = true
```

A diagonal example:

```text
source_cell = Vector2i(10, 8)
candidate_step = Vector2i(11, 9)
is_orthogonal_neighbor = false
is_same_cell = false
is_diagonal_neighbor = true
orthogonal_step_candidate = false
```

Result: diagonal movement remains unresolved/blocked until evidence and ADR approval.

### Formula 5: Step Submission Eligibility

**Purpose:** 判断单步是否可以提交 `RESERVE_MOVEMENT` command。

```text
step_submission_eligible =
    orthogonal_step_candidate
    AND legality_result.status == ALLOWED
    AND legality_result.primary_reason in approved_actor_entry_success_reasons
```

| Variable | Type / Range | Meaning | Source |
|---|---|---|---|
| `orthogonal_step_candidate` | Boolean | Formula 4 output. | `MapDistanceService` + movement policy |
| `legality_result.status` | `SpatialQueryStatus` | 单步 legality preflight 结果。 | `MovementLegalityService` |
| `legality_result.primary_reason` | `SpatialQueryReason` | Canonical reason for allowed actor entry. | `MovementLegalityService` |
| `approved_actor_entry_success_reasons` | Set of `SpatialQueryReason`; non-empty approved enum set | Canonical success reasons for actor entry from `MovementLegalityService`; must align with ADR-0002/ADR-0019 and the registered `actor_enterable` contract. | MovementLegalityService / ADR-0002 / ADR-0019 |

**Expected output:** Boolean. `true` allows submitting `RESERVE_MOVEMENT`; `false` produces typed feedback or path rejection. `legality_result` is expected to be produced by `MovementLegalityService` using the registered `actor_enterable` contract for actor entry. Click movement consumes the result and must not redefine actor-enterable truth locally.

**Example:**

```text
orthogonal_step_candidate = true
legality_result.status = ALLOWED
legality_result.primary_reason = WALKABLE
step_submission_eligible = true
```

Result: submit `RESERVE_MOVEMENT` to `MapSpaceState`.

### Formula 6: Path Search Budget

**Purpose:** 限制 Phase 1 path search 规模，避免一次点击造成不可控搜索成本。

```text
path_search_within_budget =
    visited_node_count <= max_path_search_nodes
    AND candidate_path_length <= max_click_move_path_steps
```

| Variable | Type / Range | Meaning | Source |
|---|---|---|---|
| `visited_node_count` | Integer, `0..max_path_search_nodes` during successful search; may exceed by one at failure boundary | 本次 path query 访问的 logical cell 数。 | Movement Planner / Path Query |
| `max_path_search_nodes` | Integer, safe MVP range `32..4096`, default recommendation `512` until map fixtures prove otherwise | 单次点击最大搜索节点数。 | Data-driven tuning config |
| `candidate_path_length` | Integer, `0..max_click_move_path_steps` for accepted path | 生成 path 的 step 数。 | Movement Planner / Path Query |
| `max_click_move_path_steps` | Integer, safe MVP range `1..128`, default recommendation `48` until map size and encounter spacing are validated | 单次点击可接受最大路径步数。 | Data-driven tuning config |

**Expected output:** Boolean. `false` returns no-path/path-limit feedback and must not silently truncate to a partial path unless a future fallback rule is approved.

**Example:**

```text
visited_node_count = 620
max_path_search_nodes = 512
candidate_path_length = 34
max_click_move_path_steps = 48
path_search_within_budget = false
```

Result: no path accepted; existing movement continues if any.

### Formula 7: Step Duration

**Purpose:** 将 logical step 转化为 presentation movement duration while keeping gameplay authority logical。

```text
step_duration_seconds = cell_travel_distance_units / movement_speed_cells_per_second
    IF movement_speed_config_valid
    ELSE structured_failure(INVALID_MOVEMENT_SPEED_CONFIG)
```

For MVP orthogonal movement:

```text
cell_travel_distance_units = 1.0
step_duration_seconds = 1.0 / movement_speed_cells_per_second
```

| Variable | Type / Range | Meaning | Source |
|---|---|---|---|
| `cell_travel_distance_units` | Float, MVP orthogonal value `1.0` | 一次正交 logical step 的距离单位。 | Movement policy / distance facts |
| `movement_speed_cells_per_second` | Float, safe MVP range `2.0..8.0`, provisional default `4.0` until feel testing | 每秒跨越 logical cells 数。不得硬编码。 | Data-driven tuning config or approved attribute feed |
| `movement_speed_config_valid` | Boolean | `true` iff speed is finite and `movement_speed_cells_per_second > 0`; MVP tuning should remain within `2.0..8.0`. | Data-driven tuning validation |
| `step_duration_seconds` | Float, derived; expected MVP range `0.125..0.5` with above speed range | Presentation step duration。 | Derived |

**Expected output:** Float seconds in `(0, +Infinity)` when valid; expected MVP tuned range `0.125..0.5`. Invalid speed config returns structured failure and must block config validation rather than divide by zero, silently clamp, or wrap. This duration controls visual interpolation/timing only; occupancy changes remain governed by `MapSpaceState` command results.

**Example:**

```text
movement_speed_cells_per_second = 4.0
step_duration_seconds = 1.0 / 4.0 = 0.25 seconds
```

### Formula 8: Destination Replacement Acceptance

**Purpose:** 判断新点击是否能替换当前 desired destination。

```text
replacement_accepted =
    movement_intent_eligible
    AND projection_accepted
    AND NOT same_cell_no_movement
    AND path_result.status == PATH_FOUND
```

Because `PATH_FOUND` is a design-level path result, implementation must map it to an approved typed result schema or future path-query DTO; it must not use local strings.

| Variable | Type / Range | Meaning | Source |
|---|---|---|---|
| `movement_intent_eligible` | Boolean | Formula 1 output. | Input/UI/actor/map facts |
| `projection_accepted` | Boolean | Formula 2 output. | `MapProjection` |
| `same_cell_no_movement` | Boolean | Formula 3 output. | `MapDistanceService` |
| `path_result.status` | Typed path result | Whether an orthogonal-only path exists within budget. | Movement Planner / Path Query |

**Expected output:** Boolean. `true` replaces destination/path according to state rules. `false` emits feedback and preserves current valid path unless an explicit cancel/stop action is approved.

**Example:**

```text
movement_intent_eligible = true
projection_accepted = true
same_cell_no_movement = false
path_result.status = NO_PATH
replacement_accepted = false
```

Result: show no-path feedback; existing movement continues.

### Formula 9: Arrival Condition

**Purpose:** 判断 click movement 是否完成当前 destination。

```text
movement_arrived =
    commit_result.status == ALLOWED
    AND committed_cell == desired_destination_cell
    AND remaining_path_steps == 0
```

| Variable | Type / Range | Meaning | Source |
|---|---|---|---|
| `commit_result.status` | `SpatialQueryStatus` or approved command result status | `COMMIT_MOVEMENT` authoritative result。 | `MapSpaceState` |
| `committed_cell` | `Vector2i` | Commit accepted 后 actor authoritative cell。 | `MapSpaceState` snapshot/result |
| `desired_destination_cell` | `Vector2i` | 当前 accepted movement intent 的 final target。 | Click movement state |
| `remaining_path_steps` | Integer, `0..max_click_move_path_steps` | Path 中尚未执行的 step 数。 | Movement Planner / click movement state |

**Expected output:** Boolean. `true` emits movement-arrived fact/event; downstream interactions still revalidate their own conditions.

**Example:**

```text
commit_result.status = ALLOWED
committed_cell = Vector2i(14, 8)
desired_destination_cell = Vector2i(14, 8)
remaining_path_steps = 0
movement_arrived = true
```

### Formula 10: Feedback Classification Priority

**Purpose:** 当一次点击/移动失败有多个事实时，选择稳定、可测试的 primary feedback reason。

```text
primary_feedback_reason = first_reason_by_priority(candidate_reasons, feedback_priority_order)
secondary_feedback_reasons = candidate_reasons - { primary_feedback_reason }

feedback_priority_order:
1. UI_CONSUMED_OR_INPUT_NOT_ELIGIBLE
2. ACTOR_UNAVAILABLE_OR_DISABLED
3. INVALID_COORDINATE / UNKNOWN_OR_UNLOADED / OUT_OF_BOUNDS
4. NO_MOVEMENT_REQUESTED
5. MOVEMENT_RULE_UNRESOLVED / DIAGONAL_MOVEMENT_UNRESOLVED / CORNER_CUTTING_UNRESOLVED
6. BLOCKED_BY_STATIC_MAP
7. BLOCKED_BY_ACTOR
8. RESERVED / ACTOR_ALREADY_RESERVED
9. NO_PATH_OR_PATH_LIMIT
10. RESERVATION_OR_COMMIT_FAILURE
```

ADR-0002 canonical enum names must be used where they exist. Design-level labels above that do not yet have enum coverage are candidates for ADR/schema extension before implementation. This feedback priority extends, but does not replace, the registered `query_result_priority` formula for spatial query reasons. When a failure reason comes from `SpatialQueryResult`, its canonical reason and priority must remain compatible with `query_result_priority`; click-movement-only reasons such as UI input gating, actor unavailable, path-limit, reservation failure, and commit failure require approved schema coverage before implementation.

| Variable | Type / Range | Meaning | Source |
|---|---|---|---|
| `candidate_reasons` | Ordered or unordered set of typed reason enums; size `0..N` | All typed reasons produced by input/projection/path/legality/command processing for one request. Empty set is invalid for failure classification. | Respective services |
| `feedback_priority_order` | Total ordered list of reason categories; fixed by this GDD unless ADR/schema revises it | Stable priority used to choose the primary reason. | This GDD / ADR-0002 / `query_result_priority` |
| `primary_feedback_reason` | One typed reason enum or structured failure if `candidate_reasons` is empty | Highest-priority reason present in `candidate_reasons`. | Derived |
| `secondary_feedback_reasons` | Set of typed reason enums; size `0..N-1` | Remaining reasons for debug/QA/presentation context. | Derived |

**Expected output:** One primary typed reason when `candidate_reasons` is non-empty; structured failure `NO_FEEDBACK_REASON_AVAILABLE` if a failure path attempts classification with no reason.

**Example:**

A click on a diagonal blocked wall cell may produce both diagonal unresolved and static blocked facts. In MVP, unresolved movement policy takes priority:

```text
candidate reasons = { DIAGONAL_MOVEMENT_UNRESOLVED, BLOCKED_BY_STATIC_MAP }
primary_feedback_reason = DIAGONAL_MOVEMENT_UNRESOLVED
secondary_reasons = { BLOCKED_BY_STATIC_MAP }
```

Result: QA can verify that the rule gap is not hidden behind a generic blocked message.

## Edge Cases

| Edge Case | Required Behavior | Feedback / Result | State Impact |
|---|---|---|---|
| Click is consumed by UI | Do not project, do not plan, do not cancel current movement. | No world movement feedback required; optional debug/input feedback. | Current state/path unchanged. |
| UI/input ownership is unresolved | Fail closed: treat as not eligible for world movement. | Optional unavailable/input-gated feedback. | Current state/path unchanged. |
| Named input action not matched | Ignore as non-movement input. | None. | Current state/path unchanged. |
| Actor has no authoritative source cell | Do not infer from visual position. | `UNKNOWN_OR_UNLOADED` or approved actor-unavailable reason. | Enter/remain `Disabled` or `Idle`; no command submitted. |
| Actor is dead, despawned, immobilized, control-locked or map-transitioning | Reject new movement; if already moving, cancel through `MapSpaceState` if reservation exists. | Actor unavailable / unable-to-move typed feedback; exact enum may require ADR extension. | Clear path after cancel; suppress arrival. |
| Active map is unloaded or not accepting movement | Reject input before projection or during request validation. | `UNKNOWN_OR_UNLOADED` / unavailable feedback. | No new movement; active movement cancels on map transition. |
| Projection returns invalid coordinate | Do not fallback or snap to nearest cell. | `INVALID_COORDINATE`. | Current valid path unchanged. |
| Projection is ambiguous | Treat as invalid coordinate under ADR-0005. | `INVALID_COORDINATE` with optional secondary ambiguous/debug fact if schema supports it. | Current valid path unchanged. |
| Projected cell is out of map bounds | Reject as out-of-bounds; do not clamp to edge. | `OUT_OF_BOUNDS`. | Current valid path unchanged. |
| Click target equals current committed cell | No movement requested; do not reserve. | `NO_MOVEMENT_REQUESTED`; optional light acknowledgement. | MVP default: current far destination unchanged unless a separate stop action is approved. |
| Same-cell-like click during `MovingStep` | Same-cell is evaluated against the current authoritative committed source cell, not interpolated visual position. Clicking the reserved target cell during the step is not same-cell until commit succeeds; after commit, it resolves as arrival or no movement according to the current intent. | `NO_MOVEMENT_REQUESTED` only after source and target are equal in authoritative logical state. | Current reserved step continues. |
| Click target is diagonal neighbor | Treat the clicked cell as a desired destination, not as an executable diagonal step. If an orthogonal-only path reaches it without corner-cutting, movement may proceed; if the only route would require diagonal movement or corner-cutting, reject under `mvp_provisional_orthogonal_only`. | `DIAGONAL_MOVEMENT_UNRESOLVED`, `CORNER_CUTTING_UNRESOLVED`, or no-path according to planner evidence. | No direct diagonal reservation is ever submitted; current valid path unchanged on failure. |
| Candidate path would require corner-cutting | Reject under MVP policy. | `CORNER_CUTTING_UNRESOLVED`. | No new path; current valid path unchanged. |
| Candidate target is static-blocked | Do not plan through or into the cell. | `BLOCKED_BY_STATIC_MAP`. | Current valid path unchanged. |
| Candidate target occupied by blocking actor | Do not reserve or path through the occupied cell. | `BLOCKED_BY_ACTOR`. | Current valid path unchanged. |
| Candidate target reserved by another actor | Do not treat as available. | `RESERVED`. | Current valid path unchanged. |
| Target contains a ground item | Item presence does not block actor movement in MVP unless `MapSpaceState` or static facts say otherwise. | Movement proceeds if actor entry is legal. | Pickup is not automatic; pickup system revalidates after arrival. |
| No orthogonal-only path exists | Do not move toward guessed adjacent/fallback cell. | No-path/path-failure typed result; ADR/schema extension may be required. | Current valid path unchanged; otherwise return to `Idle`. |
| Path search exceeds configured node/step budget | Treat as path failure, not partial success. | No-path/path-limit typed feedback; schema extension may be required. | Current valid path unchanged. |
| Valid replacement click while idle | Replace destination and start planning. | Destination accepted feedback optional. | `Idle → Planning`. |
| Valid replacement click before current reservation starts moving | Submit `CANCEL_MOVEMENT` for old reservation, then plan/reserve new destination after cancel success. | Optional destination-changed feedback. | `ReservingStep → Cancelling → Planning`. |
| Valid replacement click during an active moving step | Do not reverse mid-step. Store pending destination, finish/commit current step, then replan from new committed cell. | Optional destination-changed feedback. | Stay `MovingStep`; after commit → `Planning`. |
| Invalid/blocked/no-path click during movement | Feedback only; do not cancel current valid path. | Reason-specific feedback. | Current movement continues. |
| Reservation preflight allowed but command rejected | Treat command result as authoritative. Do not move into target. | Reservation failure with canonical reason such as `RESERVED`, `ACTOR_ALREADY_RESERVED`, owner mismatch, etc. | Enter `Blocked`; keep last committed cell. |
| Two actors request same target in same update | Click movement does not choose winner. | Loser receives command failure reason from `MapSpaceState`. | Winner proceeds; loser blocked/idle according to command result. |
| Commit fails after presentation reaches target | Correct presentation to last committed logical cell; do not claim arrival. Treat local step state as untrusted and query/consume authoritative reservation state. If a reservation remains active, request `CANCEL_MOVEMENT` or wait for authoritative cleanup before accepting another step. | Commit failure feedback; reason from `MapSpaceState`. | Enter `Blocked` or correction flow; suppress arrival. |
| Cancel command fails | Do not assume reservation released. | Cancel failure / owner mismatch typed feedback. | Remain blocked/correction until authoritative state is known. |
| Actor is removed during active reservation | Submit/allow cleanup cancellation through `MapSpaceState`; do not commit. | Usually no player-facing arrival; optional debug/state feedback. | Clear local intent; suppress arrival. |
| Map changes during movement | Cancel active reservation; clear destination/path; do not carry path across maps. | Movement cancelled/map unavailable feedback if player-visible. | `Any → Cancelling/Disabled → Idle` after new map is ready. |
| Destination becomes blocked after path was planned | Do not revalidate entire path every frame; detect at next step reservation. | Reservation/legality failure at the first invalid next step. | Stop and feedback; Phase 1 no auto-reroute. |
| Dynamic obstacle appears in next step | Reservation or legality fails at step boundary. | `BLOCKED_BY_ACTOR` / `RESERVED` / relevant reason. | Stop current path; no auto-reroute in Phase 1. |
| Visual position desyncs from logical cell | Use logical state as authority; presentation corrects to committed render anchor. | Optional debug correction feedback. | Gameplay state unchanged unless `MapSpaceState` changes. |
| Y-sort order changes during movement | Ignore for gameplay legality. | None. | No gameplay state impact. |
| Godot Navigation suggests a shorter route | Ignore unless the route is converted to logical steps and each step passes approved services. | None or debug-only. | No authority impact. |
| Physics collision disagrees with MapSpaceState | MapSpaceState/MovementLegality wins for gameplay. | Optional debug inconsistency report. | No gameplay change unless authoritative facts change. |
| Downstream pickup target disappears before arrival | Click movement still only reports movement result; pickup system revalidates and fails pickup if needed. | Pickup system owns item-gone feedback. | Movement arrival may still occur; no pickup guarantee. |
| Enemy target moves while player is moving to attack range | Combat Targeting owns target/range revalidation and may issue new movement intent. | Combat system owns targeting feedback. | Click movement only follows accepted movement requests. |
| Save/load requested during movement | Movement must be cancelled or resolved to a consistent committed logical cell before save snapshot is accepted. | Save/load system owns save-state feedback. | No half-step logical state may be saved. |
| Test fixture contains diagonal-only corridor | MVP must report no orthogonal path/unresolved diagonal, not silently allow diagonal. | `DIAGONAL_MOVEMENT_UNRESOLVED` or no-path according to exact query. | No movement through diagonal-only route. |

## Dependencies

### Upstream Dependencies

| Dependency | Status | What Click Movement Requires | Contract / Boundary |
|---|---|---|---|
| Game Concept / Pillars | Existing GDD | `稳刷不断流` and `传奇骨架，现代皮肤` define the movement fantasy: low-friction classic PC click movement with modern clarity. | Movement rules must reduce interruption to the 30-second loot-loop and must not become high-skill dodge/combo movement. |
| 地图坐标 / 阻挡 / Y-sort 系统 | Designed dependency | Logical grid, static map facts, actor occupancy, reservation rules, Y-sort visual-only boundary. | Click movement consumes logical map contracts and must not own map truth, occupancy truth or visual sorting authority. |
| ADR-0001 Map Data Representation | Accepted | `MapDefinition` Resource-first static map truth. | TileMapLayer visuals/editor data are not movement authority. Static blocking comes from approved map facts. |
| ADR-0002 Typed Query Result Schema | Accepted | `SpatialQueryResult`, canonical statuses/reasons, cell snapshots and retry hints. | Click movement must not branch on local strings. New path/feedback result types discovered here require ADR/schema extension before implementation. |
| ADR-0003 Authoritative Occupancy / Reservation Update Ordering | Accepted | `MapSpaceState` command queue, `RESERVE_MOVEMENT`, `COMMIT_MOVEMENT`, `CANCEL_MOVEMENT`, atomic authoritative update. | Click movement submits commands only; it never mutates occupancy/reservation directly. |
| ADR-0004 Deterministic Y-sort Implementation | Accepted | Visual sorting tuple and visual-only guarantees. | Y-sort may use movement presentation positions but never affects movement legality, pathing, blocking or arrival. |
| ADR-0005 Input Projection / Coordinate Conversion | Accepted | `MapProjection` converts screen/viewport/world points to logical cells after input gate. | Click movement consumes typed projection results and never performs coordinate conversion itself. |
| ADR-0019 Map Distance Facts and Movement Legality Boundary | Accepted | `MapDistanceService`, `MovementLegalityService`, `mvp_provisional_orthogonal_only`, same-cell and diagonal/corner-cutting policy. | Every executable step must be orthogonal-only and read-only preflighted before MapSpaceState command submission. |
| Input Configuration / Rebinding | Baseline UX requirement; detailed implementation pending | Named movement action such as `move_click`. | Hardcoded mouse button checks are not design-authoritative. Input remapping must remain possible. |
| UI/Input Gate | UX baseline exists; detailed implementation pending | A reliable event ownership result before projection. | UI-consumed or unknown ownership input must fail closed and not move/cancel. Godot 4.6 dual-focus must be respected. |
| Movement Planner / Path Query | Not yet separately designed | Orthogonal-only path or typed no-path/path-limit result from read-only facts. | This GDD defines required behavior but not final DTO/API. Architecture must formalize result schema before implementation. |
| Actor / Status Facts | Partially external / future systems | Actor existence, current committed cell, movement-disabled states, map transition availability. | Click movement cannot infer actor availability from visuals; it needs authoritative actor/status facts. |
| Data-driven Tuning Config | Pending | `max_path_search_nodes`, `max_click_move_path_steps`, `movement_speed_cells_per_second`. | Values must be external config/fixtures, not hardcoded. |

### Downstream Dependents

| Dependent System | What It Uses From Click Movement | Required Handoff | Boundary |
|---|---|---|---|
| Player Controller / Actor Presentation | Accepted movement step, committed target cell, movement state facts, step timing. | Presentation receives logical anchors and state facts to animate/interpolate. | Presentation does not decide legality or mutate occupancy. |
| Combat Targeting / Basic Attack | Future movement-to-range requests and arrival/failure facts. | Combat may request movement toward an attack range cell and must revalidate range before attacking. | Click movement does not select enemy targets or perform attacks. |
| Ground Drop / Pickup | Movement-to-pickup-range requests and arrival/failure facts. | Pickup may request movement and then revalidate item existence/range/inventory after arrival. | Click movement does not pick up items or guarantee item availability. |
| Loot-loop UX / HUD Feedback | Typed movement feedback facts and destination/path state. | UI can show destination marker, blocked marker, no-path message, cursor feedback or debug overlay. | UI feedback is presentation and never changes movement authority. |
| Audio / VFX | Movement accepted, blocked, arrived, cancelled, step started/completed facts. | Audio/VFX can play clicks, footsteps, blocked thuds or arrival cues. | Audio/VFX cannot affect movement legality or command results. |
| Tutorial / Onboarding | Movement failure and success facts. | Tutorial can teach valid click areas, blocked cells and no-path meaning. | Tutorial hints do not override movement rules. |
| Save/Load | Stable committed logical cell and no half-step authoritative save state. | Save/load must snapshot only committed movement state or force/cancel to a consistent state. | Click movement cannot save presentation-only progress as occupancy truth. |
| QA / Automated Tests | Deterministic state transitions, typed reasons, data-driven tuning values. | Unit/integration tests verify formulas, edge cases and command boundaries. | Visual feel is playtest/screenshot evidence, not formula-only automation. |
| Architecture Review / ADR Pipeline | New schema needs discovered in Formulas and Detailed Rules. | Path result DTO, path-limit reason, actor-unavailable feedback and input-gate result may require ADR updates. | Implementation stories are blocked until these contracts are accepted if they are needed in code. |

### Dependency Risks and Provisional Contracts

1. **Path result schema is not yet fully standardized.** This GDD uses design-level terms such as `PATH_FOUND`, `NO_PATH_OR_PATH_LIMIT`, and path-limit feedback. Before implementation, architecture must either extend ADR-0002 or approve a path-query DTO so these are typed, not local strings.
2. **Input gate result schema is pending.** The GDD requires fail-closed UI/input ownership, especially for Godot 4.6 dual-focus, but the final API belongs to input/UI architecture.
3. **Movement disabled actor states are not fully designed.** Death, stun, map transition and control lock are named as required facts; final status-effect ownership will be defined by future actor/status/combat systems.
4. **Movement speed source is provisional.** The GDD defines safe ranges and externalization requirements, but whether speed comes from a config fixture, attribute stat or movement profile requires later architecture/design approval.
5. **Downstream combat/pickup behavior is handoff-only.** This GDD deliberately stops at movement arrival/failure facts; future Combat and Pickup GDDs must define target interpretation, range validation and business outcomes.
6. **Implementation must be split into testable stories.** Recommended sequence: headless movement core, orthogonal path planner + typed path DTO, MapSpaceState command integration, then input/projection/presentation integration. Headless movement logic and command boundaries are blocking for core Done; visual/audio/cursor presentation criteria are blocking for presentation integration Done.

## Tuning Knobs

All tuning knobs must be data-driven through approved config/resources/fixtures. No click movement tuning value may be hardcoded into gameplay code. Safe ranges below are MVP guardrails, not final balance claims; they must be validated through playtest and Godot runtime profiling.

| Knob | Provisional Safe Range | Provisional Default | Affects | Source / Rationale |
|---|---:|---:|---|---|
| `movement_speed_cells_per_second` | `2.0..8.0` | `4.0` | Visual travel duration, perceived responsiveness, loot-loop flow. Too low causes sluggish travel; too high reduces readable classic movement. | Formula 7. Default gives `0.25s` per orthogonal cell, suitable for MVP feel testing. Final source may be movement profile or attribute feed. |
| `max_click_move_path_steps` | `1..128` | `48` | Maximum accepted path length for one click. Too low forces excessive clicking; too high may allow confusing long travel and costly searches. | Formula 6. Default supports remote click movement on small MVP maps while bounding search. |
| `max_path_search_nodes` | `32..4096` | `512` | CPU/search budget per click. Too low rejects valid routes; too high risks frame spikes. | Formula 6. Must be profiled against real Phase 1 map fixtures. |
| `movement_replacement_policy` | Enum: `commit_current_step_then_replan` only for MVP | `commit_current_step_then_replan` | Feel of repeated clicks while moving. | Section C rule: avoids mid-step reversal/reservation complexity while still making clicks responsive after commit. |
| `invalid_click_cancels_current_path` | Boolean | `false` | Misclick tolerance. | Default protects `稳刷不断流`: invalid/UI/blocked/no-path clicks do not stop current valid movement. |
| `same_cell_click_policy` | Enum: `no_op`, `cancel_current_destination`, `requires_stop_action` | `no_op` for MVP | Whether clicking actor's current cell stops movement. | Default avoids accidental cancellation. If player stop control is needed, prefer separate stop action. |
| `auto_fallback_to_nearest_walkable` | Boolean | `false` | Whether blocked clicks redirect to nearby cells. | Default false to preserve predictable rules and avoid hidden target guessing. Future UX may revisit with explicit fallback design. |
| `auto_reroute_on_dynamic_block` | Boolean | `false` | Whether path replans automatically when next reservation fails. | Default false to keep Phase 1 deterministic and testable. Player can click again to replan. |
| `destination_marker_duration_seconds` | `0.1..2.0` | `0.6` | How long accepted click marker remains visible. | Presentation-only; supports clarity without clutter. Not gameplay authority. |
| `blocked_feedback_cooldown_seconds` | `0.0..0.5` | `0.15` | Prevents rapid repeated blocked feedback spam. | Presentation/UI/audio protection; must not suppress typed gameplay failure facts used by tests/debug. |
| `arrival_event_delay_seconds` | `0.0..0.1` | `0.0` | Delay before downstream systems receive arrival fact. | Default immediate after commit; non-zero only if presentation requires staging and architecture approves. |
| `path_debug_overlay_enabled` | Boolean | `false` in player builds | Debug visibility for path cells, reservations and reasons. | Development/QA aid; must not affect gameplay logic. |

### Non-Tunable MVP Rules

These are intentionally not tuning knobs in Phase 1:

- **Diagonal movement:** Disabled/unresolved until evidence and ADR approval.
- **Corner-cutting:** Disabled/unresolved until evidence and ADR approval.
- **Coordinate conversion source:** Always `MapProjection`; never local conversion.
- **Occupancy/reservation mutation source:** Always `MapSpaceState` command queue.
- **Static map truth:** Always approved map data contracts; never TileMap visuals.
- **Y-sort influence:** Always visual-only; cannot be tuned into movement authority.
- **Fallback-to-nearest behavior:** Disabled by default and requires explicit future design approval before becoming tunable in player builds.

### Tuning Validation Notes

- Movement speed must be validated against the 30-second loot-loop target: travel should not dominate the loop or make reaching drops feel tedious.
- Path search budgets must be profiled on representative MVP maps before implementation stories claim Done.
- Feedback cooldowns must preserve accessibility: reducing spam must not hide important failure information from players who need clear repeated confirmation.
- Any knob that changes authoritative movement behavior must be covered by automated tests; presentation-only knobs may be validated through walkthrough/playtest evidence.

## Acceptance Criteria

Acceptance criteria are written so QA and implementation can verify pass/fail. Logic and integration criteria are blocking for implementation Done; visual/audio feel criteria require walkthrough or playtest evidence.

### Input / UI Gate Criteria

- **AC-01 — Named action only:** Given an input event that does not match the approved movement input action, no movement intent is created.
- **AC-02 — UI consumed click ignored:** Given a `move_click` event consumed by UI, `MapProjection` is not called, no path changes, and no `MapSpaceState` command is submitted.
- **AC-03 — Dual-focus coverage:** Godot 4.6 dual-focus cases are tested: mouse click over UI while keyboard focus is elsewhere does not move; keyboard/gamepad focus in UI does not by itself block a clearly eligible world mouse click unless the input gate policy says so.
- **AC-04 — Unknown ownership fails closed:** Given unresolved input ownership, the system produces no world movement request and preserves current path.

### Projection Criteria

- **AC-05 — Projection boundary:** Every accepted world click calls `MapProjection`; no test path may pass through local coordinate conversion in click movement code.
- **AC-06 — Invalid coordinate:** Given `MapProjection` returns `INVALID_COORDINATE`, no destination/path changes and no movement command is submitted.
- **AC-07 — Out of bounds:** Given a projected out-of-bounds candidate, the target is rejected without clamping or nearest-cell fallback.
- **AC-08 — Ambiguous projection:** Given ambiguous projection, the result is treated as invalid/unavailable and current valid movement continues.

### Movement Rule Criteria

- **AC-09 — Same-cell no movement:** Given source cell equals target cell, the system returns/records `NO_MOVEMENT_REQUESTED`, does not submit `RESERVE_MOVEMENT`, and does not cancel a current far destination in MVP.
- **AC-10 — Orthogonal neighbor accepted:** Given an orthogonal neighboring target that is walkable, unoccupied and unreserved, the system can submit `RESERVE_MOVEMENT`.
- **AC-11 — Diagonal step rejected:** Given a candidate executable step from source to a diagonal neighbor under `mvp_provisional_orthogonal_only`, no movement reservation is submitted and the result is unresolved/blocked with canonical reason. A diagonal-adjacent desired destination may proceed only if the planner finds a legal orthogonal-only path.
- **AC-12 — Corner-cutting rejected:** Given a path that would require corner-cutting, the path is rejected unless a future ADR changes policy.
- **AC-13 — Static blocked rejected:** Given a target or next step blocked by static map facts, no reservation is submitted and feedback uses `BLOCKED_BY_STATIC_MAP`.
- **AC-14 — Actor occupied rejected:** Given a target or next step occupied by a blocking actor, no reservation is submitted and feedback uses `BLOCKED_BY_ACTOR`.
- **AC-15 — Reserved target rejected:** Given a target or next step reserved by another actor, no reservation is submitted and feedback uses `RESERVED` or approved equivalent.
- **AC-16 — Ground item does not block movement:** Given a cell containing only a ground item and no actor/static/reservation blocker, actor movement remains eligible in MVP.

### Path and Replacement Criteria

- **AC-17 — Remote click path:** Given a reachable remote destination, the system generates an orthogonal-only path within configured budget and executes it step-by-step.
- **AC-18 — No-path preserves current movement:** Given a no-path click while actor is already following a valid path, the existing movement continues and no-path feedback is emitted.
- **AC-19 — No fallback:** Given a blocked or unreachable clicked destination, the system does not redirect to a nearby walkable cell.
- **AC-20 — Path budget enforced:** Given path search exceeds `max_path_search_nodes` or `max_click_move_path_steps`, no path is accepted and a typed path-limit/no-path result is produced.
- **AC-21 — Valid replacement while moving:** Given a valid new destination during `MovingStep`, the current step is not reversed mid-step; after commit, movement replans from the new committed cell.
- **AC-22 — Invalid replacement ignored:** Given invalid/blocked/no-path replacement click during movement, feedback is emitted and current valid movement continues.

### MapSpaceState Command Criteria

- **AC-23 — Reservation command boundary:** A movement step starts only after a `RESERVE_MOVEMENT` command is accepted by `MapSpaceState`.
- **AC-24 — Commit command boundary:** Authoritative actor occupancy changes only after `COMMIT_MOVEMENT` is accepted by `MapSpaceState`.
- **AC-25 — Cancel command boundary:** Movement cancellation or destination replacement with active reservation releases/changes reservation only through `CANCEL_MOVEMENT` or approved command flow.
- **AC-26 — Command rejection authority:** Given legality preflight allowed but `MapSpaceState` rejects reservation/commit, the command result wins; actor remains at last committed logical cell and no arrival is emitted.
- **AC-27 — Same-tick conflict:** Given two actors request the same target in one authoritative update, click movement does not decide winner; `MapSpaceState` deterministic ordering does.

### State Machine Criteria

- **AC-28 — Valid state sequence:** Reachable movement follows valid transitions: `Idle → Planning → ReservingStep → MovingStep → CommittingStep → Arrived → Idle` for a successful one-step move.
- **AC-29 — Blocked state:** Blocked/static/occupied/reserved/no-path failures enter a blocked/failure outcome with typed reason, then clear failed intent without mutating occupancy.
- **AC-30 — Unresolved state:** Diagonal/corner-cutting/cross-map or missing policy cases are distinguishable from ordinary blocked results.
- **AC-31 — Disabled actor:** Dead/despawned/map-transitioning/control-locked actors cannot start new movement and cancel active reservations if applicable.
- **AC-32 — Map transition cleanup:** Active movement does not survive map unload/change; path and reservations are cleared through authoritative flow.

### Authority Boundary Criteria

- **AC-33 — No visual authority:** Changing actor Node2D/world position alone does not change source cell, movement eligibility or arrival.
- **AC-34 — No TileMap authority:** TileMapLayer visual/collision/custom data cannot override `MapDefinition`/legality facts for movement.
- **AC-35 — No physics authority:** Physics overlap/collision does not block or allow movement unless reflected in approved map/occupancy facts.
- **AC-36 — No Godot Navigation authority:** Any Navigation/NavigationAgent-suggested route must be rejected if its logical steps fail `MovementLegalityService`.
- **AC-37 — No Y-sort authority:** Y-sort order changes do not affect projection, pathing, blocking, distance, occupancy or arrival.

### Feedback / UI / Accessibility Criteria

- **AC-38 — Typed feedback:** Invalid, same-cell, static blocked, actor blocked, reserved, no-path/path-limit, reservation failure, commit failure and unresolved movement all produce distinguishable typed facts.
- **AC-39 — Player message coverage:** Player-facing UI can map each major failure category to a short localized message or non-text equivalent.
- **AC-40 — Not color-only:** At least blocked/invalid/no-path feedback has a non-color cue such as icon shape, animation, text, cursor change or sound.
- **AC-41 — Marker non-authority:** Destination marker visibility never counts as movement acceptance, movement success or arrival in logic tests.
- **AC-42 — Feedback cooldown non-suppression:** Visual/audio cooldown may reduce spam, but debug/test typed failure events remain observable.
- **AC-42A — Not audio-only:** Movement accepted, blocked, invalid, no-path and unresolved feedback remain perceivable when audio is muted; audio cues are supplemental and never the only player-facing feedback.
- **AC-42B — Flash safety:** Destination markers, blocked markers and movement feedback animations do not exceed the WCAG three-flashes threshold.
- **AC-42C — Scalable movement messages:** Player-facing movement messages use the approved localized/scalable text UI pipeline when available and remain readable at supported text scale settings.

### Visual / Feel Criteria

- **AC-43 — Immediate acknowledgement:** Accepted movement clicks show a destination marker or equivalent acknowledgement on the same frame or next visible update after acceptance.
- **AC-44 — Readable step movement:** Ordinary movement appears as travel between logical anchors, not teleportation, unless debug/test mode intentionally snaps.
- **AC-45 — Arrival handoff:** Movement arrival fact/event fires only after authoritative final commit; downstream pickup/combat systems must revalidate their own success conditions.
- **AC-46 — 30-second loop fit:** In a representative MVP map, configured movement between nearby monster/drop loop points has timing-capture evidence and explicit design-lead approval that travel time does not dominate the 30-second combat-loot-pickup cycle; provisional target is ≤ 25% total traversal time unless playtest approves otherwise.

### Architecture / Schema Gate Criteria

- **AC-47 — Path result schema approved:** Before implementation Done, path-found/no-path/path-limit results must be represented by an approved typed DTO/schema, not local strings.
- **AC-48 — Input gate contract approved:** Before implementation Done, input ownership/gate results must be represented by an approved contract suitable for Godot 4.6 dual-focus testing.
- **AC-49 — Actor unavailable reasons approved:** Before implementation Done, actor disabled/dead/map-transition/control-lock movement failures must map to approved typed reasons or DTO fields.
- **AC-50 — Test runner available:** Logic acceptance criteria must run under the project's approved Godot GDScript test runner; placeholder/guard test infrastructure alone is not sufficient for story Done.
- **AC-51 — Testable dependency boundary:** Click movement public logic must receive projection, legality, map state, planner, tuning and actor/status facts through testable dependencies or approved adapters; direct singleton-only logic is not sufficient for unit-testable Done.
- **AC-52 — Event routing review evidence:** Implementation evidence must show movement intents derive from the same event whose action match and UI ownership were evaluated, and that `_input`/polling paths cannot bypass UI consumption.

## Visual/Audio Requirements

Visual and audio feedback must make movement intent readable without becoming gameplay authority. Every presentation element in this section consumes typed movement facts; none may decide legality, pathing, occupancy, reservation or arrival.

### Visual Requirements

1. **Accepted destination marker**
   - When a click is accepted as a movement destination, show an immediate marker at the destination's approved world/render anchor.
   - The marker should appear the same frame or next visible update after the accepted result is available.
   - Marker lifetime is controlled by `destination_marker_duration_seconds`.
   - The marker is presentation-only; if movement later fails due to reservation/commit changes, the authoritative result wins.

2. **Invalid / blocked / no-path feedback**
   - Invalid coordinate, blocked target, reserved target, no-path and unresolved movement rule must produce visually distinguishable feedback where practical.
   - Color may be used, but player-facing failure feedback must always include at least one non-color cue such as icon shape, animation/pulse pattern, cursor state, marker shape/location or text.
   - Feedback must map from typed reasons rather than local strings.
   - Repeated blocked feedback may be rate-limited by `blocked_feedback_cooldown_seconds`, but tests/debug facts must still record each failure.

3. **Movement readability**
   - The actor's visual movement must clearly travel from one logical cell anchor to the next; it must not appear to teleport during ordinary click movement.
   - Orthogonal movement should be readable enough that players understand why diagonal-only gaps cannot be crossed in MVP.
   - The actor may interpolate smoothly between anchors, but presentation must correct if authoritative commit fails.

4. **Path visibility**
   - Player-facing full path preview is not required for Phase 1.
   - A lightweight destination marker is required; optional short path hints may be added only if they do not imply uncommitted authority.
   - Developer/debug builds may display path cells, checked cells, reservations and typed failure reasons.

5. **Y-sort compatibility**
   - Actor visuals must continue to sort correctly via the approved Y-sort system while moving.
   - Y-sort updates may use current visual position/anchor for rendering, but must never feed back into movement legality.

6. **Accessibility and color safety**
   - Blocked/invalid/no-path states must not rely on color alone. Pair any color change with a non-color cue such as icon shape, animation pattern, cursor change, text or sound. Audio may supplement feedback but must not be the only player-facing cue.
   - Marker intensity and animation must avoid excessive flashing and must not exceed the WCAG three-flashes threshold.

### Audio Requirements

1. **Click accepted cue**
   - An accepted destination may play a subtle confirmation sound.
   - The cue must be low priority and should not compete with loot-drop or combat sounds.

2. **Blocked / invalid cue**
   - Blocked, invalid, no-path and unresolved movement feedback may play short negative cues.
   - The cue should be clearly different from loot failure, inventory error or combat miss if those exist later.
   - Rapid repeated failures should respect feedback cooldown to avoid audio spam.

3. **Footstep / movement loop**
   - Footstep or movement-loop audio is optional for Phase 1 but recommended if it improves readability.
   - Footstep timing should follow presentation movement steps, not create gameplay timing authority.
   - If audio desyncs from logical movement, gameplay state remains authoritative.

4. **Arrival cue**
   - Arrival cue is optional and should be subtle; major reward emphasis belongs to combat/drop/pickup systems.
   - Arrival audio must only play after authoritative arrival or approved presentation handoff, not when a marker is merely shown.

### Presentation Priority

When multiple events occur close together, presentation priority is:

1. Loot/drop reward cues from downstream systems.
2. Combat danger or hit feedback.
3. Movement blocked/invalid feedback.
4. Accepted movement click marker/cue.
5. Ambient movement footsteps.

This priority protects `爆装有戏` and combat readability while still keeping movement understandable.

## UI Requirements

### Input and Remapping

1. Click movement must be bound through a named Godot input action, provisionally `move_click`, rather than hardcoded mouse button checks.
2. The default PC binding may be left mouse button, but the design contract is the named action.
3. Future keyboard/gamepad bindings may generate the same movement intent only if they provide an eligible world point/cursor target and pass the same UI/input gate.
4. UI screens must not duplicate movement interpretation; they either consume/block the input or allow it to pass to world interaction.

### UI/Input Gate

1. Any Control/UI layer that receives or owns the current event must be able to prevent world movement.
2. Inventory, equipment, menus, dialogs, tooltips, drag/drop overlays and modal screens must block movement clicks in their owned regions.
3. The input gate must be evaluated before `MapProjection`.
4. Godot 4.6 dual-focus must be treated explicitly: mouse/touch event ownership and keyboard/gamepad focus are separate. UI tests must cover mouse click over UI while keyboard focus is elsewhere, and keyboard/gamepad focus in UI while mouse click targets the world.
5. If the gate cannot determine ownership, it must fail closed and prevent movement.

### Player-Facing Messages

Movement failures should use short, readable messages where text feedback is appropriate. Exact localization keys can be defined later, but semantic coverage is required:

| Reason Category | Suggested English Message | Suggested Chinese Message | Notes |
|---|---|---|---|
| Invalid coordinate / off-map | `Cannot move there.` | `无法移动到那里。` | Use for invalid/out-of-bounds projection. |
| Static blocked | `Path is blocked.` | `道路被阻挡。` | Use when target/step is blocked by map facts. |
| Actor occupied / reserved | `Something is in the way.` | `有东西挡住了。` | Avoid exposing reservation internals to normal players. |
| No orthogonal path | `Cannot reach target.` | `无法到达目标。` | Aligns with accessibility baseline. |
| Movement rule unresolved | `Movement rule not available yet.` | `该移动规则暂不可用。` | Debug/player build wording may differ; useful for provisional MVP gates. |
| Actor unavailable | `Cannot move now.` | `现在无法移动。` | Death/stun/map transition/control lock. |
| Same-cell no movement | Usually no text | Usually no text | Optional subtle acknowledgement only. |

Messages must be driven by typed reasons and localization keys, not hardcoded strings in movement logic.

### Cursor and Marker UI

1. Cursor state may indicate valid ground, invalid ground or blocked destination if UI design supports it.
2. Cursor state is advisory presentation and must not determine movement legality.
3. Destination markers must use approved projection/logical anchors, not raw screen position.
4. Debug overlays may show logical cell, path budget, reservation state and typed failure reason for QA/dev builds.

### Accessibility Requirements

1. Movement feedback should be perceivable through more than one channel where practical: visual marker plus sound, icon plus text, or debug text plus marker.
2. Failure feedback must not rely on color alone.
3. Movement action must be remappable through named input action support.
4. No required simultaneous button presses are allowed for basic click movement.
5. UI-blocked movement must be predictable: players should not accidentally move when interacting with inventory/equipment.
6. Text messages should support the project's text scaling/localization pipeline when implemented.

### UI Non-Responsibilities

The UI must not:

- Decide whether a cell is walkable.
- Decide whether an actor reservation succeeds.
- Convert screen/world points to authoritative logical cells.
- Mutate destination/path/occupancy directly.
- Infer movement success from destination marker visibility.
- Hide typed movement failures from automated/debug validation, even if player-facing messages are rate-limited.

## Open Questions

These questions do not block the GDD's MVP behavioral direction, but they do block or shape implementation stories until resolved by ADR, schema work, tuning fixtures or downstream GDDs.

1. **Path result schema**
   - What final DTO/enum represents path found, no-path and path-limit results?
   - Current GDD stance: must be typed and approved before implementation Done; no local strings.

2. **Input gate contract**
   - What exact contract reports UI/input ownership for Godot 4.6 dual-focus cases?
   - Current GDD stance: fail closed when unresolved; architecture must formalize the result shape.

3. **Movement disabled reasons**
   - Which system owns dead, stun, immobilized, casting-lock, control-lock and map-transition movement denial facts?
   - Current GDD stance: click movement consumes these facts and cancels/blocks movement, but does not own status-effect semantics.

4. **Movement speed source**
   - Does `movement_speed_cells_per_second` come from a static movement profile, character attributes, equipment modifiers, class data or another source?
   - Current GDD stance: use data-driven provisional config for MVP; later systems may feed the value through approved contracts.

5. **Stop movement command**
   - Should the player have an explicit stop/cancel movement action separate from same-cell click?
   - Current GDD stance: same-cell click is no-op/no movement for MVP and does not cancel current far destination. A separate stop action can be designed later if needed.

6. **Automatic reroute**
   - Should the system ever automatically replan when a dynamic obstacle blocks the next step?
   - Current GDD stance: no auto-reroute in Phase 1. Reservation failure stops and provides feedback; the player may click again.

7. **Fallback to nearest walkable cell**
   - Should blocked destination clicks ever redirect to an adjacent legal cell?
   - Current GDD stance: disabled for MVP to avoid hidden target guessing. Any future fallback requires explicit UX/design and typed reason handling.

8. **Combat and pickup click interpretation**
   - When clicking an enemy or item sprite, which system converts that into movement-to-range versus direct interaction intent?
   - Current GDD stance: Combat Targeting and Pickup own semantic interpretation; click movement executes movement requests and emits arrival/failure facts only.

9. **Path preview**
   - Should players see a short preview/path trace or only a destination marker?
   - Current GDD stance: destination marker required; full path preview not required for Phase 1. Debug overlay may show path facts.

10. **Future keyboard/gamepad target production**
    - What approved method produces an eligible world point or target for non-mouse movement input: virtual cursor, focus target, reticle, or another accessible targeting model?
    - Current GDD stance: click movement consumes such a target only after the same UI/input gate passes; target production belongs to a future input/UX design.

11. **OpenMir2 evidence for diagonal behavior**
    - Does original behavior allow diagonal movement or any special corner behavior relevant to this client?
    - Current GDD stance: diagonal and corner-cutting remain unresolved/evidence-gated under ADR-0019; do not implement until evidence and ADR update approve it.
