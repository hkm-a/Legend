# 交互目标 / 选择系统

> **Status**: In Design
> **Author**: hkm + Claude Code Game Studios
> **Last Updated**: 2026-06-05
> **Implements Pillar**: Primary — 稳刷不断流; Supports — 爆装有戏 / 传奇骨架，现代皮肤
> **Quick reference** — Layer: `Core` · Priority: `MVP` · Key deps: `点击移动系统; 地图坐标 / 阻挡 / Y-sort 系统`

## Overview

交互目标 / 选择系统是 Phase 1 离线刷怪爆装切片中把一次已通过 UI/input gate 的世界交互意图解释为“点击了什么、想做什么”的核心判定层：它在 `MapProjection`、地图空间查询、对象可交互事实和屏幕/世界候选信息之间建立优先级规则，决定一次点击应被解释为地面移动、敌对目标选择/攻击意图、掉落物拾取意图，还是无效/不可用交互。该系统不执行移动、不计算攻击、不完成拾取、不拥有 UI consumption、不修改地图占位、不决定掉落价值；它负责把玩家的鼠标意图稳定地路由到点击移动、基础战斗、掉落与拾取、UI/反馈等下游系统，并在目标重叠、目标消失、距离不足、路径不可达或规则未决时产出清晰、typed、可测试的选择结果。它必须保留经典传奇“点怪打怪、点地移动、点物想捡”的低摩擦骨架，同时用现代客户端的优先级、反馈和可访问性约束避免误点打断 `稳刷不断流`。

## Player Fantasy

玩家幻想是成为一个熟练的玛法冒险者，用最直接的点击就能让世界理解自己的意图：点地面就顺势移动，点敌人就进入战斗节奏，点掉落就冲向收益，点到无效位置也不会打断刷怪流。玩家不需要在“移动模式、攻击模式、拾取模式”之间思考切换，而是感觉自己的注意力始终留在战场、爆装和变强上。

这个系统要保护的是传奇式刷怪的连续性：玩家看到机会、点击机会、游戏理解机会。它不替玩家移动、不替玩家计算攻击、不替玩家完成拾取，而是在输入通过 UI/input gate 后，把那次世界点击可靠地翻译成一个清晰的意图。理想状态下，玩家不会意识到这个系统存在；他们只会感觉“我点的就是我想做的”。

在 Phase 1 的 30 秒离线 loot loop 中，本系统支撑从移动、打怪、掉落、拾取到穿戴变强的顺滑衔接。爆装出现时，点击掉落物应该延续奖励兴奋，而不是制造歧义或摩擦；怪物进入视野时，点击敌人应该自然表达攻击意图，而不是和地面移动抢解释权。每一次正确的意图识别都在强化 `稳刷不断流` 的核心体验。

**设计测试：** 如果同一次点击可能被解释为移动、攻击或拾取，优先选择最符合玩家当下可见目标与刷怪收益节奏的意图，并且必须给出清晰反馈；如果无法安全解释，宁可返回无效交互，也不要让玩家产生“我点的不是这个”的感觉。

## Detailed Rules

### Core Rules

1. **Ownership boundary rule**
   - 本系统只负责把已通过 UI/input gate 的世界 hover/click 解释为一个 typed world interaction result。
   - 本系统 owns：候选目标查询结果消费、hover primary candidate、click primary intent、敌对/掉落/地面/无效之间的优先级、persistent hostile selection state、typed interaction feedback facts。
   - 本系统 does not own：UI 是否消费输入、坐标投影 authority、移动路径/预约/提交、攻击距离/冷却/伤害、拾取距离/背包容量/拾取提交、MapSpaceState mutation、UI 文案、声音、VFX 或 tooltip 内容。

2. **Same-event gate rule**
   - 本系统只能处理上游 input gate 标记为 world-eligible 的同一输入事件。
   - 不得通过轮询鼠标状态、缓存上一帧点击点、读取 UI hover state、或绕过 Control 事件消费来生成世界交互 intent。
   - UI/input gate 失败时，本系统不得输出 hostile、loot 或 ground intent；只允许传播 typed gate failure fact 给反馈层。

3. **Projection and candidate query rule**
   - 世界点击解释必须先获得 `MapProjection` 或批准世界查询边界的 typed result。
   - Projection success 只代表可以查询世界候选或 logical cell candidate，不代表地面可走、目标可攻击或物品可拾取。
   - 点击时必须重新查询候选；不得直接信任上一帧 hover candidate。
   - 候选查询结果必须是 immutable-by-contract snapshot，不得暴露 actor node、item node、Control node、Area2D、PhysicsBody 或 mutable runtime records 作为 gameplay authority。

4. **Single primary intent rule**
   - 每一次世界 click 最多生成一个 primary intent：`hostile_interaction`、`loot_interaction`、`ground_movement`、`unsupported_interaction` 或 `invalid_interaction`。
   - 系统不得从一次点击同时执行多个 primary intents，例如“攻击敌人并顺路捡物品”。
   - 连锁行为必须由 downstream systems 明确处理，例如 Combat 请求 approach-to-attack、Pickup 请求 approach-to-pickup；本系统只负责初始语义解释和 handoff。

5. **Candidate category rule**
   - Phase 1 识别以下候选类别：

| Candidate Category | Meaning | Possible Output |
|---|---|---|
| `hostile_actor` | 可被玩家选择/攻击的敌对 actor snapshot。 | Select hostile, attack intent, approach-for-attack request, hostile unavailable feedback. |
| `ground_loot` | 地面掉落物或 pickup candidate snapshot。 | Pickup intent, approach-for-pickup request, loot unavailable feedback. |
| `ground_cell` | 可解释为地面移动目标的 logical cell candidate。 | Movement intent or movement failure feedback. |
| `unsupported_world_object` | NPC、机关、装饰物等 Phase 1 未支持但被识别为 gameplay object 的目标。 | Unsupported interaction feedback. |
| `none` | 没有有效候选。 | No-op or invalid feedback depending on input context. |

6. **Primary priority rule**
   - Hover 与 click resolution 使用同一 primary priority：

```text
Hostile actor > Ground loot > Ground cell > Unsupported world object > Invalid/none
```

   - Hostile beats loot because combat targets are high-risk/high-timing objects.
   - Loot beats ground because clicking visible loot expresses “I want that item,” not “walk to the floor under it.”
   - Ground movement is fallback only when no higher-priority gameplay candidate is under the cursor.
   - Unsupported gameplay objects should fail with unsupported feedback rather than silently becoming ground movement. Visual-only props that are not gameplay candidates may be ignored and allow ground fallback.

7. **Direct-hit and tie-break rule**
   - Candidate resolution sorts by:

```text
1. Higher candidate category priority.
2. direct_hit before near_hit before cell_hit.
3. Smaller screen-space distance from click point to candidate interaction anchor.
4. Smaller approved hit shape if both contain click point and shape area is available.
5. Stable deterministic ID tie-breaker.
```

   - Hostile stable ID is the stable actor interaction ID.
   - Loot stable ID order is `stable_drop_sequence` ascending, then stable item/ground drop ID.
   - The system must not use scene-tree order, physics result order, dictionary iteration order, wall-clock time, frame delta, UI label draw order, rarity/value, current HP, player distance, or Y-sort order as gameplay tie-break authority.

8. **Hostile interaction rule**
   - Hovering a hostile emits hover feedback request only: outline/bracket/cursor/nameplate preview. It must not select, attack, move, or mutate state.
   - Clicking a valid hostile sets or refreshes persistent hostile selection and emits a hostile interaction intent.
   - If Combat integration can validate range and action readiness, the output may be `attack_intent` when in range or `approach_for_attack_intent` when out of range but reachable.
   - If Combat/movement integration is not stable, the output is `select_hostile_intent` plus typed out-of-range/unresolved feedback rather than inventing attack or movement behavior.
   - Combat owns attack legality, range, cooldown, damage, hit timing and final target validation.

9. **Loot interaction rule**
   - Hovering loot emits pickup hover feedback only: label emphasis, pickup icon, pulse/highlight request. It must not pick up, claim, move, or mutate inventory/map state.
   - Clicking valid loot emits a loot interaction intent.
   - If Pickup integration can validate range and movement-to-range, the output may be `pickup_intent` when in range or `approach_for_pickup_intent` when out of range but reachable.
   - If Pickup/movement integration is not stable, the output is `loot_target_intent` plus typed out-of-range/unresolved feedback.
   - Pickup owns item availability, pickup distance, claim, inventory capacity, commit, item removal and success publication.
   - Clicking loot does not automatically clear hostile selection in MVP unless selected hostile is stale or a future combat-state rule says otherwise.

10. **Ground movement interaction rule**
    - A ground movement intent may be emitted only when no higher-priority hostile/loot/supported gameplay object candidate is selected and projection provides a logical ground cell candidate.
    - Ground click uses Click Movement contracts for movement intent creation, path query, reservation, commit and failure feedback.
    - If clicked ground is blocked, out of bounds, unknown or unreachable, no movement intent is emitted by this system; feedback must distinguish the typed reason where available.
    - Invalid ground clicks do not clear hostile selection.

11. **No silent fallback rule**
    - If the player clicks a recognized hostile or loot candidate and that object is out of range, blocked, stale or unavailable, the system must not silently downgrade the click to ground movement.
    - It must emit an object-specific intent or object-specific failure so downstream systems can approach, revalidate or explain failure.
    - This protects the player from “I clicked the monster/item but my character walked somewhere else.”

12. **Hover vs persistent selection rule**
    - Hover focus is transient and previews the current click result.
    - Persistent hostile selection changes only on committed hostile click, explicit target-selection input, explicit clear/cancel, target replacement, target death/removal, map unload or player control reset.
    - Movement, invalid clicks and loot pickup do not automatically clear hostile selection in MVP.
    - For partial gamepad support, the last active input method owns transient focus; persistent hostile selection is shared across input methods.

13. **Typed result rule**
    - Candidate category, intent kind, hover state, selection state, failure reason and tie-break evidence must be represented by typed enums/DTO fields or registry IDs.
    - Gameplay logic must not branch on local strings, UI labels, node names, scene names, localization text or cursor art.
    - Presentation maps typed reasons to localization keys and visual/audio feedback.

14. **Fail-closed ambiguity rule**
    - Unknown input ownership, unresolved projection, map unavailable, unsupported candidate type, ambiguous candidate priority, stale target, or missing tie-break data must fail closed.
    - Fail closed means no attack, no pickup, and no movement unless ground is the only clear candidate and the object ambiguity is explicitly visual-only.
    - Ambiguous gameplay targets return typed unresolved/invalid feedback instead of guessing.

15. **Accessibility feedback rule**
    - Hostile, loot, ground and invalid states must be distinguishable without color alone.
    - Each primary hover category must have at least one non-color cue: outline shape, icon, cursor shape, marker style, label treatment, text or motion within flash safety limits.
    - Click failure feedback must explain why when possible: cannot reach target, cannot reach item, blocked ground, inventory full, item gone, target lost, unsupported object.

### States and Transitions

The system has two lightweight state groups plus a stateless click resolution pipeline.

#### Hover Preview State

| State | Meaning | Transitions / Output |
|---|---|---|
| `NoHoverCandidate` | No eligible world hover candidate or input/projection ownership blocks hover query. | Clear hover highlights/cursor affordance. |
| `HoverHostile` | Primary hover candidate is a hostile actor. | Request hostile outline/bracket/attack cursor/nameplate preview. |
| `HoverLoot` | Primary hover candidate is ground loot. | Request loot highlight/label emphasis/pickup cursor. |
| `HoverGround` | Primary hover candidate is ground cell. | Request movement cursor or optional ground preview. |
| `HoverUnsupported` | Primary hover candidate is recognized but unsupported in Phase 1. | Request unsupported/disabled affordance. |
| `HoverUnresolved` | Candidate query cannot safely choose a primary target. | Request invalid/unresolved cursor/debug feedback; no gameplay intent. |

Hover transitions must never create movement, attack, pickup, selection mutation or MapSpaceState commands.

#### Persistent Hostile Selection State

| State | Meaning | Transitions / Output |
|---|---|---|
| `NoSelection` | No persistent hostile target. | Valid hostile click → `HostileSelected`. |
| `HostileSelected` | A hostile actor is selected/combat-focused. | Different valid hostile click replaces selection; same hostile click refreshes intent; target death/despawn → `SelectionStalePendingClear`; explicit clear → `NoSelection`. |
| `SelectionStalePendingClear` | Previously selected hostile is confirmed dead, despawned, map-invalid or not targetable. | Emit selection invalidated; clear after presentation/consumer acknowledgement → `NoSelection`. |

MVP default: ground movement, invalid click and loot click do not clear `HostileSelected` unless selected target is stale. This preserves target awareness during movement and looting.

#### Click Resolution Pipeline

A click is evaluated as a deterministic query pipeline rather than a long-lived state:

```text
InputRejected
ProjectionUnresolved
CandidateQueryUnresolved
HostileClickResolved
LootClickResolved
GroundClickResolved
UnsupportedClickResolved
InvalidClickResolved
```

| Resolved Outcome | Output |
|---|---|
| `InputRejected` | No gameplay intent; typed input/gate reason. |
| `ProjectionUnresolved` | No gameplay intent; invalid/unavailable projection feedback. |
| `CandidateQueryUnresolved` | No gameplay intent; unresolved/ambiguous priority feedback. |
| `HostileClickResolved` | Select hostile and emit select/attack/approach intent or hostile failure. |
| `LootClickResolved` | Emit pickup/approach intent or loot failure. |
| `GroundClickResolved` | Emit movement intent or movement-target failure. |
| `UnsupportedClickResolved` | Emit unsupported interaction feedback. |
| `InvalidClickResolved` | Emit no-candidate/invalid feedback. |

### Interactions with Other Systems

| System | Input to Target Selection | Output from Target Selection | Ownership Boundary |
|---|---|---|---|
| Input / UI Gate | Same-event named action, world eligibility, event ownership, cursor/screen position facts. | Only processes world-eligible events; propagates typed gate failures. | UI/input gate owns event consumption and Godot 4.6 dual-focus decisions. |
| `MapProjection` | UI-gated event position or world query point. | Candidate logical cell / projection failure used for ground and spatial candidate queries. | Projection owns coordinate conversion; target selection does not locally convert. |
| MapSpaceState / Map Facts | Actor occupancy snapshot, item occupancy snapshot, map availability, stable IDs, candidate facts. | Read-only candidate query consumption. | MapSpaceState owns runtime occupancy/reservation mutation; target selection never mutates it. |
| Click Movement | Ground movement intent; approach-for-attack/pickup movement request origin. | Movement accepted/blocked/arrived/failure facts consumed by downstream presentation or interaction chain. | Movement owns path query, replacement behavior, reservation/commit/cancel. |
| Combat / Targeting | Hostile candidate facts, selected hostile ID, attack/approach intent. | Target validity, range/cooldown/action readiness, target death/despawn events. | Combat owns attack legality, damage, cooldown, hit timing and final attack execution. |
| Ground Drop / Pickup | Loot candidate facts, ground_drop_id/item candidate, pickup/approach intent. | Pickup availability, range, claim, inventory capacity, commit result, item-gone events. | Pickup owns pickup completion and inventory handoff. |
| UI / Cursor / HUD / VFX / Audio | Hover state, selection state, typed feedback facts, intended interaction facts. | Highlights, rings, cursors, labels, messages, sounds. | Presentation is non-authoritative and cannot change target priority or gameplay result. |
| Accessibility / Localization | Typed reason IDs and state categories. | Localized messages, non-color cues, scalable text. | Localization maps reason IDs to text; gameplay never branches on player-facing strings. |
| Save/Load | None for Phase 1 persistent truth. | On load, target selection starts empty. | Hover and selection are runtime-only in MVP; save/load must not persist stale target refs. |
| QA / Debug Tools | Candidate snapshots, tie-break evidence, typed outcomes. | Test assertions, debug overlay, replay evidence. | Debug display cannot affect priority or result. |

## Formulas

The interaction target / selection system does not own combat damage, pickup success, movement pathing, drop value, item rarity, or equipment comparison formulas. Its formulas define deterministic candidate eligibility, priority, tie-breaks, selection persistence, and feedback classification.

### Formula 1: World Interaction Eligibility

```text
world_interaction_eligible =
    input_gate_status == WORLD_ELIGIBLE
    AND projection_or_query_context_available
    AND active_map_accepts_world_interaction
```

| Variable | Type / Range | Meaning | Source |
|---|---|---|---|
| `input_gate_status` | Typed input gate enum | Same-event UI/input ownership status. | Input/UI gate / ADR-0022 |
| `projection_or_query_context_available` | Boolean | Whether the click/hover event has enough approved screen/viewport/world facts to query candidates. | Input router + MapProjection boundary |
| `active_map_accepts_world_interaction` | Boolean | Current map is loaded/active and can provide candidate snapshots. | Map/session state |

**Expected output:** Boolean. `false` means no target candidate query and no gameplay intent.

### Formula 2: Candidate Eligibility

```text
candidate_eligible =
    candidate_snapshot_valid
    AND candidate_category_supported
    AND candidate_targetable_status != UNAVAILABLE
    AND candidate_map_id == active_map_id
```

| Variable | Type / Range | Meaning | Source |
|---|---|---|---|
| `candidate_snapshot_valid` | Boolean | Candidate snapshot has required stable IDs, anchors, category and state facts. | Candidate query adapter |
| `candidate_category_supported` | Boolean | Candidate category is supported in Phase 1. | This GDD |
| `candidate_targetable_status` | Typed enum | Candidate is available, stale, unavailable or unresolved. | Candidate snapshot / downstream adapter |
| `candidate_map_id` | `StringName` | Candidate's active map. | Candidate snapshot |
| `active_map_id` | `StringName` | Player/current interaction map. | Session/map state |

**Expected output:** Boolean. Ineligible gameplay objects return typed unavailable/unsupported feedback rather than ground fallback if they were directly clicked.

### Formula 3: Candidate Category Priority

```text
category_priority(candidate) =
    400 if candidate.category == HOSTILE_ACTOR
    300 if candidate.category == GROUND_LOOT
    200 if candidate.category == GROUND_CELL
    100 if candidate.category == UNSUPPORTED_WORLD_OBJECT
    0   if candidate.category == NONE_OR_INVALID
```

| Variable | Type / Range | Meaning | Source |
|---|---|---|---|
| `candidate.category` | Typed candidate category enum | Hostile, loot, ground, unsupported, none. | Candidate query result |
| `category_priority` | Integer `0..400` | Higher wins. Values are ordinal ranks only, not balance values. | This GDD |

**Example:** Hostile and loot both directly under cursor: hostile `400` beats loot `300`.

### Formula 4: Hit Strength Priority

```text
hit_strength_priority(candidate) =
    30 if candidate.hit_kind == DIRECT_HIT
    20 if candidate.hit_kind == NEAR_HIT
    10 if candidate.hit_kind == CELL_HIT
    0  otherwise
```

| Variable | Type / Range | Meaning | Source |
|---|---|---|---|
| `candidate.hit_kind` | Typed enum | Direct shape hit, near/forgiveness hit, logical cell hit, none. | Candidate query adapter |
| `hit_strength_priority` | Integer `0..30` | Higher wins within category priority. | This GDD |

### Formula 5: Cursor Distance Tie-Break

```text
cursor_distance_px = distance(click_screen_position, candidate.screen_anchor)
normalized_cursor_distance_score = max(0, max_candidate_query_radius_px - cursor_distance_px)
```

| Variable | Type / Range | Meaning | Source |
|---|---|---|---|
| `click_screen_position` | `Vector2` | Same-event cursor/click position. | Input gate result |
| `candidate.screen_anchor` | `Vector2` | Candidate interaction anchor projected to screen. | Candidate snapshot |
| `cursor_distance_px` | Float `>= 0` | Screen-space distance from click to candidate anchor. | Derived |
| `max_candidate_query_radius_px` | Float safe MVP range `0..96`, default `32` for object near-hit unless UX testing revises | Maximum near-hit radius. | Data-driven tuning |
| `normalized_cursor_distance_score` | Float `0..max_candidate_query_radius_px` | Higher means closer; used only after category and hit strength. | Derived |

**Example:** With `max_candidate_query_radius_px = 32`, candidate A at 6 px gets score 26; candidate B at 18 px gets score 14, so A wins if prior ranks tie.

### Formula 6: Stable Tie-Break Key

```text
stable_tie_break_key =
    candidate.category,
    candidate.stable_interaction_id
```

For loot:

```text
stable_tie_break_key =
    GROUND_LOOT,
    stable_drop_sequence,
    stable_ground_drop_id_or_item_instance_id
```

| Variable | Type / Range | Meaning | Source |
|---|---|---|---|
| `stable_interaction_id` | Integer or approved comparable stable ID | Deterministic candidate ID within active map/category. | Candidate snapshot |
| `stable_drop_sequence` | Integer `>= 0` | Stable drop ordering for ground loot. | GroundDrop/Pickup facts |
| `stable_ground_drop_id_or_item_instance_id` | Integer or approved stable ID | Final deterministic loot tie-breaker. | GroundDrop/Pickup facts |

**Expected output:** Comparable tuple. If stable IDs are missing or incomparable, result is unresolved; do not use scene order.

### Formula 7: Primary Candidate Ordering

```text
primary_candidate = max_by(
    candidates,
    candidate_eligible,
    category_priority,
    hit_strength_priority,
    normalized_cursor_distance_score,
    -hit_shape_area_if_available,
    -stable_tie_break_key
)
```

Equivalent sorting order:

1. Eligible candidates only.
2. Higher category priority.
3. Higher hit strength priority.
4. Higher normalized cursor distance score.
5. Smaller hit shape area if both candidates contain the click and area is available.
6. Lower stable tie-break key.

**Expected output:** Exactly one primary candidate, or typed unresolved/no-candidate result.

### Formula 8: Ground Fallback Eligibility

```text
ground_fallback_eligible =
    no_eligible_hostile_candidate
    AND no_eligible_loot_candidate
    AND no_unresolved_higher_priority_gameplay_candidate
    AND ground_candidate_available
```

| Variable | Type / Range | Meaning | Source |
|---|---|---|---|
| `no_eligible_hostile_candidate` | Boolean | No hostile candidate is directly/near hit and eligible. | Candidate query result |
| `no_eligible_loot_candidate` | Boolean | No loot candidate is directly/near hit and eligible. | Candidate query result |
| `no_unresolved_higher_priority_gameplay_candidate` | Boolean | No higher-priority gameplay object exists but cannot be safely resolved. | Candidate query result |
| `ground_candidate_available` | Boolean | Projection produced a logical ground candidate suitable for movement handoff. | MapProjection / map facts |

**Expected output:** Boolean. `false` prevents silent fallback from object click to movement.

### Formula 9: Persistent Selection Validity

```text
selection_valid =
    selected_actor_id_exists
    AND selected_actor_map_id == active_map_id
    AND selected_actor_alive
    AND selected_actor_targetable
```

| Variable | Type / Range | Meaning | Source |
|---|---|---|---|
| `selected_actor_id_exists` | Boolean | Actor still exists in authoritative actor registry/snapshot. | Combat/actor facts |
| `selected_actor_map_id` | `StringName` | Selected actor's map. | Actor snapshot |
| `active_map_id` | `StringName` | Player/current map. | Session/map state |
| `selected_actor_alive` | Boolean | Actor has not died/despawned. | Life/death system |
| `selected_actor_targetable` | Boolean | Actor remains valid hostile target. | Combat targeting facts |

**Expected output:** Boolean. `false` transitions selection to stale pending clear.

### Formula 10: Feedback Classification

```text
primary_interaction_feedback = first_reason_by_priority(
    input_gate_reason,
    projection_reason,
    candidate_reason,
    downstream_readiness_reason,
    target_state_reason
)
```

Priority order:

1. Input/UI gate failure.
2. Projection/map unavailable failure.
3. Candidate priority unresolved.
4. Target stale/unavailable.
5. Unsupported object.
6. Higher-priority object blocks ground fallback.
7. Downstream range/readiness unavailable.
8. No candidate / invalid click.

All reasons must be typed. Player-facing messages are localization mappings, not gameplay logic.

## Edge Cases

- **If the same input event is consumed by UI or fails the UI/input gate**: return no hostile, no loot, no ground, and no unsupported world intent; propagate a typed input/gate failure fact only. This prevents inventory/equipment UI clicks from leaking into world interaction.
- **If mouse/touch event ownership is unresolved even though keyboard/gamepad focus is not on UI**: fail closed as input/gate failure feedback; do not query candidates, do not project, and do not reuse prior hover. Godot 4.6 dual-focus means keyboard focus alone cannot prove world eligibility.
- **If keyboard/gamepad focus is inside UI but the current mouse click is explicitly world-eligible under the input gate**: process the click normally through projection and candidate resolution. Same-event ownership, not global focus alone, decides eligibility.
- **If projection or approved world-query context cannot resolve the event position**: return `invalid_interaction` with typed projection reason such as `invalid_coordinate`, `unknown_or_unloaded`, or an approved equivalent; do not snap to nearest cell, previous hover cell, current player cell, or `(0, 0)`.
- **If the active map is unloaded, transitioning, or cannot provide immutable candidate snapshots**: return no gameplay intent with typed map/query unavailable feedback. Persistent hostile selection transitions toward stale only if authoritative facts confirm the selected actor is invalid.
- **If a valid hostile, valid loot, and valid ground cell all overlap under the cursor**: resolve to hostile interaction because priority is `Hostile > Loot > Ground > Unsupported > Invalid`; do not emit pickup or movement intent from the same click.
- **If loot and ground overlap but no hostile candidate is present**: resolve to loot interaction, not ground movement, even if the loot is out of pickup range. Downstream Pickup or movement-to-pickup owns approach and revalidation.
- **If an unsupported gameplay object overlaps a valid ground cell and no hostile/loot candidate is present**: return `unsupported_interaction` with a typed unsupported reason; do not silently downgrade to ground movement.
- **If a purely visual prop overlaps a valid ground cell but is not registered as a gameplay candidate**: ignore the visual-only prop and allow ground resolution if ground is otherwise the only clear candidate. Visual-only obstruction is not gameplay authority.
- **If a directly clicked hostile is stale, dead, despawned, not targetable, or on a different map**: return hostile-specific unavailable/stale feedback and do not fallback to ground movement or loot selection. Persistent selection is cleared or moved to `SelectionStalePendingClear` only through the approved stale-selection transition.
- **If a directly clicked loot candidate has been picked up, expired, removed, or no longer exists in the authoritative item occupancy snapshot**: return loot-specific unavailable/item-gone feedback and do not fallback to ground movement. Pickup/inventory mutation remains downstream-owned.
- **If hostile and loot candidates have equal screen distance or both contain the click shape**: category priority still wins before hit strength, distance, hit shape area, or stable ID; hostile wins over loot regardless of Y-sort or visual draw order.
- **If two candidates in the same category have the same hit kind and same screen-space distance**: choose the candidate with smaller approved hit shape area if both shape areas are available; if shape area is missing or incomparable, continue to stable deterministic ID tie-break. Do not use scene-tree order, physics query order, dictionary order, or Y-sort order.
- **If required stable tie-break data is missing or incomparable for otherwise tied gameplay candidates**: return `CandidateQueryUnresolved` with typed unresolved feedback and no gameplay intent, unless the ambiguity is explicitly visual-only and ground is the only clear gameplay candidate.
- **If the hover candidate from the previous frame differs from the click-time candidate**: resolve the click using a fresh same-event candidate query; do not trust or commit the previous hover candidate. Hover may only preview, never select, attack, pick up, move, or mutate state.
- **If a hostile click is out of attack range, blocked, or combat readiness is unknown**: emit `select_hostile_intent`, `attack_intent`, `approach_for_attack_intent`, or hostile-specific failure only according to approved Combat/Movement integration facts; never convert the click to ground movement.
- **If a loot click is out of pickup range, blocked, or pickup readiness is unknown**: emit `loot_target_intent`, `pickup_intent`, `approach_for_pickup_intent`, or loot-specific failure only according to approved Pickup/Movement integration facts; never convert the click to ground movement.
- **If a ground candidate is blocked, out of bounds, unknown/unloaded, same-cell, unreachable, or path-limited under Click Movement rules**: return a ground/movement-target typed failure or hand off a ground intent only when Click Movement can validate it; do not clear persistent hostile selection and do not invent fallback-to-nearest-walkable behavior.

## Dependencies

### Upstream Dependencies

| Dependency | Type | Interface / Contract Consumed | Requirement for This System |
|---|---|---|---|
| Input / UI Gate | Hard | Same-event world eligibility, input ownership status, screen/viewport position facts, Godot 4.6 dual-focus handling. | Target selection may only evaluate events that are world-eligible. Unknown ownership fails closed before projection/candidate query. |
| `MapProjection` | Hard | Typed coordinate/world query result from ADR-0005; logical cell candidate or canonical projection failure. | Target selection never converts screen/world coordinates locally and never uses sentinel coordinates. Projection success enables candidate lookup but does not imply walkability or object validity. |
| Map Coordinate / Blocking / Y-sort System | Hard | Logical grid, `MapDefinition`, `MapSpaceState` snapshots, candidate cell facts, static/dynamic/item occupancy semantics, stable Y-sort/render anchors for presentation only. | Candidate queries must use logical/candidate facts and stable IDs, not scene-tree order, visual tile authority, physics order, or Y-sort draw order. |
| Click Movement System | Hard for ground intent; soft for approach-to-target until Combat/Pickup are designed | `ground_movement` handoff, path/reachability/failure feedback, approach request origin for future attack/pickup chains. | Target selection emits ground or approach intents only; Click Movement owns path query, replacement behavior, reservation, commit and movement failure. |
| Candidate Query Adapter / World Interaction Snapshot | Hard | Immutable-by-contract candidate snapshots for hostile actors, loot, ground cells, unsupported objects and tie-break evidence. | This adapter must expose stable IDs, category, hit kind, screen anchor, map id, targetable status, and availability facts without exposing nodes or mutable runtime records. |
| OpenMir2 Evidence Governance | Soft / policy | Evidence labels and accepted contract readiness for source-authentic targeting behavior. | Phase 1 priority and tie-break rules are MVP provisional unless later OpenMir2 evidence approves or revises them. |

### Downstream Dependencies

| Downstream System | Output Provided by Target Selection | Ownership Boundary |
|---|---|---|
| Basic Combat / Combat Targeting | Persistent hostile selection, hostile hover facts, `select_hostile_intent`, `attack_intent`, `approach_for_attack_intent`, hostile unavailable facts. | Combat owns attack range, cooldown, hit timing, damage, final target validation and combat state. |
| Damage / Life / Death | Selected hostile identity may be consumed indirectly through Combat. Death/despawn facts invalidate selection. | Damage and Life/Death own health, death state and death events; target selection only clears or marks stale selection after authoritative facts. |
| Ground Drop / Pickup | Loot hover facts, `loot_target_intent`, `pickup_intent`, `approach_for_pickup_intent`, loot unavailable facts. | Pickup owns item availability, claim, range, inventory capacity, pickup commit, item removal and success publication. |
| UI / Cursor / HUD | Hover category, selected hostile state, typed feedback facts, candidate priority evidence, unsupported/invalid reasons. | UI owns cursor art, label layout, tooltip rendering, text and non-authoritative highlights. UI cannot change priority or gameplay result. |
| VFX / Audio Feedback | Typed hover/click/failure facts and state changes. | Presentation chooses effects and sounds; it must not create, cancel or alter interaction intents. |
| Accessibility / Localization | Typed category and reason IDs. | Accessibility/localization map IDs to non-color cues and text. Gameplay never branches on localized strings. |
| QA / Debug Tools | Candidate snapshots, primary ordering inputs, selected result, failure reason, tie-break evidence. | Debug overlay and test logs are read-only; they cannot affect candidate order. |
| Save/Load | None for persistent truth in Phase 1. | Hover and persistent hostile selection are runtime-only; save/load must not persist stale target references. |

### Bidirectional Consistency Notes

- The Click Movement GDD lists Combat Targeting and Pickup as downstream consumers of movement arrival/failure facts. This GDD supplies the initial semantic intent that may cause those downstream systems to request movement.
- The Map Coordinate / Blocking / Y-sort GDD owns logical grid, spatial query results, and visual-only Y-sort boundaries. This GDD consumes those facts only for target interpretation and must not redefine map passability, pickup distance, attack distance or visual sorting authority.
- Future Combat and Pickup GDDs must mention this system as their upstream source for player click target intent, but they must revalidate target/range/action legality inside their own systems.

## Tuning Knobs

| Knob | Type / Safe Range | Default MVP Value | Affects | Too Low / Too High Behavior |
|---|---|---:|---|---|
| `max_candidate_query_radius_px` | Float `0..96` px | `32` | Maximum near-hit radius for object candidate lookup around the cursor. | Too low makes small loot/enemies hard to click; too high causes surprising target capture away from the cursor. |
| `hostile_near_hit_radius_px` | Float `0..96` px | `24` | Forgiveness radius for hostile candidate near-hits. | Too low makes combat targeting feel picky; too high steals clicks from loot/ground too often. |
| `loot_near_hit_radius_px` | Float `0..96` px | `32` | Forgiveness radius for ground loot candidate near-hits. | Too low weakens `爆装有戏`; too high makes movement near loot difficult. |
| `ground_click_requires_no_object_candidate` | Boolean | `true` | Whether ground movement is allowed when any higher-priority gameplay object is unresolved or unavailable. | `false` risks silent fallback; `true` may produce more explicit failures but preserves intent clarity. |
| `unsupported_object_blocks_ground_fallback` | Boolean | `true` | Whether recognized unsupported gameplay objects prevent ground movement fallback. | `false` can make future NPC/object clicks unexpectedly move; `true` may require clearer unsupported feedback. |
| `persistent_selection_cleared_by_loot_click` | Boolean | `false` | Whether clicking loot clears hostile selection. | `true` can reduce stale target confusion but breaks target awareness while looting; MVP keeps it false. |
| `persistent_selection_cleared_by_ground_click` | Boolean | `false` | Whether clicking ground clears hostile selection. | `true` makes movement act like target cancel; `false` preserves combat focus during repositioning. |
| `hover_update_min_interval_ms` | Integer `0..250` ms | `0` for MVP tests; presentation may throttle later | Minimum interval for hover candidate refresh in presentation-facing adapters. | Too high makes hover feel laggy; too low may increase allocation/debug load. Gameplay click resolution still re-queries on click. |
| `candidate_shape_area_tiebreak_enabled` | Boolean | `true` | Whether smaller approved hit shape wins after category, hit kind and distance tie. | `false` makes ties rely more on stable ID; `true` favors precise direct hits. |
| `debug_tie_break_evidence_enabled` | Boolean | `true` in tests/debug, optional in release | Emits category, hit strength, distance score, shape area and stable ID evidence. | Disabled in tests reduces QA traceability; always enabled in release may create unnecessary log/debug overhead. |
| `invalid_feedback_cooldown_ms` | Integer `0..1000` ms | `250` | Presentation throttling for repeated invalid/unsupported click feedback. | Too low can spam sounds/messages; too high can hide why repeated clicks fail. Gameplay result is not throttled. |
| `hover_visual_requires_non_color_cue` | Boolean | `true` | Accessibility requirement for hover and target category presentation. | `false` violates accessibility baseline; `true` forces cursor/icon/shape/text or motion cue in addition to color. |

Tuning knobs are data-driven configuration inputs. They must not be hardcoded in target-selection gameplay logic, and they must not override typed priority rules unless the GDD is revised. Category priority values in the formulas are ordinal ranks, not tuning knobs; changing `Hostile > Loot > Ground` requires a design revision and review.

## Acceptance Criteria

### Evidence Requirements

- **Logic / Unit evidence (blocking):** deterministic resolution, formulas, state transitions, typed reason/result, and tie-break behavior. Recommended location: `tests/unit/interaction_target_selection/`.
- **Integration evidence (blocking):** Input/UI Gate, `MapProjection`, candidate query, Click Movement, Combat, and Pickup handoff boundaries. Recommended location: `tests/integration/interaction_target_selection/` or documented playtest where automation is not yet possible.
- **Manual evidence (advisory):** player-perceived hover/click behavior, debug overlay, visual feedback and repeated-click scenarios. Recommended location: `production/qa/evidence/`.
- **Accessibility evidence (advisory):** non-color category cues and typed failure explanation. Recommended location: `production/qa/evidence/`.

### Criteria

- **AC-01 — GIVEN** the same input event has `input_gate_status != WORLD_ELIGIBLE`, **WHEN** the system processes the event, **THEN** it outputs only a typed input/gate failure fact, performs no hostile/loot/ground/unsupported candidate query, emits no primary gameplay intent, and does not modify persistent hostile selection.
- **AC-02 — GIVEN** `world_interaction_eligible` inputs include input gate status, query-context availability and active-map availability, **WHEN** any input is false or not world-eligible, **THEN** the formula returns false, no candidate query runs, no gameplay intent is emitted, and the output includes the corresponding typed failure reason.
- **AC-03 — GIVEN** an input passes UI/input gate but `MapProjection` returns `invalid_coordinate`, `unknown_or_unloaded` or an approved equivalent failure, **WHEN** the system resolves the click, **THEN** it emits invalid/projection feedback, emits no hostile/loot/ground intent, does not snap to nearest cell/current cell/previous hover/`(0, 0)`, and does not change selection unless authoritative stale-selection facts exist.
- **AC-04 — GIVEN** the previous frame hover primary candidate differs from the current click-time candidate query, **WHEN** the click is resolved, **THEN** the system uses the fresh same-event query result and debug evidence records the hover/click candidate difference.
- **AC-05 — GIVEN** hover query returns hostile, loot, ground, unsupported or unresolved, **WHEN** hover state updates, **THEN** the system outputs only hover feedback/clear requests and never emits movement, attack, pickup, approach or selection mutation.
- **AC-06 — GIVEN** a candidate has invalid snapshot data, unsupported category, unavailable targetable status or mismatched map id, **WHEN** `candidate_eligible` is evaluated, **THEN** it returns false and a directly clicked gameplay object produces object-specific feedback rather than ground fallback.
- **AC-07 — GIVEN** eligible hostile, loot and ground candidates overlap under one click, **WHEN** primary candidate is calculated, **THEN** hostile wins; **GIVEN** only loot and ground overlap, **THEN** loot wins; **GIVEN** only clear ground exists, **THEN** ground may emit movement intent or ground-specific failure.
- **AC-08 — GIVEN** an unsupported gameplay object overlaps a valid ground cell and no hostile/loot exists, **WHEN** `unsupported_object_blocks_ground_fallback == true`, **THEN** the result is `unsupported_interaction` typed feedback, not ground movement.
- **AC-09 — GIVEN** a visual-only prop overlaps a valid ground cell but is not a gameplay candidate, **WHEN** the click is resolved, **THEN** the prop is ignored, ground may resolve normally, and debug evidence does not record the prop as primary gameplay candidate.
- **AC-10 — GIVEN** two candidates in the same category have different hit kinds, **WHEN** primary ordering is calculated, **THEN** `DIRECT_HIT` beats `NEAR_HIT`, and `NEAR_HIT` beats `CELL_HIT`.
- **AC-11 — GIVEN** two same-category candidates have the same hit kind but different same-event screen distances, **WHEN** `normalized_cursor_distance_score` is calculated, **THEN** the closer candidate wins and the system does not use previous hover position, actor world distance or player distance as substitutes.
- **AC-12 — GIVEN** two same-category candidates tie on category, hit kind and cursor distance while both contain the click and expose approved hit shape area, **WHEN** `candidate_shape_area_tiebreak_enabled == true`, **THEN** the smaller approved hit shape wins.
- **AC-13 — GIVEN** candidates remain tied after category, hit kind, cursor distance and shape area, **WHEN** comparable stable IDs exist, **THEN** the lower approved stable key wins consistently across repeated runs and result does not depend on scene-tree order, physics query order, dictionary iteration, Y-sort order, frame delta or wall-clock time.
- **AC-14 — GIVEN** otherwise tied gameplay candidates lack comparable stable tie-break data, **WHEN** the click is resolved, **THEN** the system outputs `CandidateQueryUnresolved` typed feedback, emits no gameplay intent, and debug evidence identifies missing/incomparable tie-break data.
- **AC-15 — GIVEN** a directly clicked hostile is stale, dead, despawned, not targetable, blocked, out of range or combat readiness is unknown while ground is valid underneath, **WHEN** the click is resolved, **THEN** the output is hostile-specific intent/failure and never ground movement or loot interaction.
- **AC-16 — GIVEN** a directly clicked loot candidate is picked up, expired, removed, blocked, out of range or pickup readiness is unknown while ground is valid underneath, **WHEN** the click is resolved, **THEN** the output is loot-specific intent/failure and never ground movement or hostile interaction.
- **AC-17 — GIVEN** no hostile selection exists, **WHEN** the player clicks a valid hostile actor, **THEN** persistent selection becomes `HostileSelected(actor)` and output is select/attack/approach or approved hostile result without dealing damage, consuming cooldown, submitting movement or mutating target state.
- **AC-18 — GIVEN** `HostileSelected(A)` is valid, **WHEN** the player clicks valid ground or valid loot and MVP selection-clearing knobs are false, **THEN** the ground/loot result may be emitted but selection remains `HostileSelected(A)`.
- **AC-19 — GIVEN** `HostileSelected(A)` exists and authoritative facts show A is missing, dead, despawned, map-mismatched or not targetable, **WHEN** `selection_valid` is evaluated, **THEN** selection becomes `SelectionStalePendingClear`, emits selection invalidated typed fact, and clears to `NoSelection` only through approved cleanup/acknowledgement.
- **AC-20 — GIVEN** a clear ground candidate is the primary result, **WHEN** the system emits ground output, **THEN** it only emits ground movement intent or ground-specific failure and does not run path query, reserve movement, commit movement, mutate position or mutate `MapSpaceState`.
- **AC-21 — GIVEN** a ground candidate is blocked, out of bounds, unknown/unloaded, same-cell, unreachable or path-limited under Click Movement rules, **WHEN** it is resolved, **THEN** the output is ground/movement-target typed failure or Click Movement handoff failure, with no nearest-walkable fallback and no hostile selection clear.
- **AC-22 — GIVEN** a valid hostile click and Combat reports in-range/action-ready or out-of-range/reachable, **WHEN** the click resolves, **THEN** the system may output `attack_intent` or `approach_for_attack_intent`, but Combat retains final target legality, cooldown, timing, hit and damage authority.
- **AC-23 — GIVEN** a valid loot click and Pickup reports in-range/ready or out-of-range/reachable, **WHEN** the click resolves, **THEN** the system may output `pickup_intent` or `approach_for_pickup_intent`, but Pickup retains claim, item removal, inventory capacity, commit and success authority.
- **AC-24 — GIVEN** one click contains multiple candidates, **WHEN** resolution completes, **THEN** output contains at most one primary intent of kind `hostile_interaction`, `loot_interaction`, `ground_movement`, `unsupported_interaction` or `invalid_interaction`; it never outputs attack+pickup, pickup+move or attack+move as multiple primary intents.
- **AC-25 — GIVEN** category, intent kind, hover state, selection state, reason or tie-break evidence participates in gameplay branching, **WHEN** QA inspects tests or code review evidence, **THEN** those values are typed enum/DTO/registry IDs and gameplay logic does not branch on local strings, node names, scene names, localized text, cursor art or tooltip text.
- **AC-26 — GIVEN** multiple reason domains are present, **WHEN** `primary_interaction_feedback` is calculated, **THEN** reason priority is Input/UI gate, Projection/map unavailable, Candidate priority unresolved, Target stale/unavailable, Unsupported object, Higher-priority object blocks ground fallback, Downstream readiness, then No candidate/invalid click.
- **AC-27 — GIVEN** category priority ordinal values are defined, **WHEN** tuning config loads, **THEN** config cannot reorder `Hostile > Loot > Ground > Unsupported > Invalid`; any attempt to do so fails validation or smoke check.
- **AC-28 — GIVEN** near-hit radius tuning values change candidate inclusion, **WHEN** hostile/loot/ground all become eligible candidates, **THEN** primary priority still resolves Hostile > Loot > Ground.
- **AC-29 — GIVEN** a ground candidate exists but an unresolved higher-priority gameplay candidate also exists, **WHEN** `ground_fallback_eligible` is evaluated, **THEN** it returns false and outputs unresolved/higher-priority-blocks-ground fallback feedback, not ground movement.
- **AC-30 — GIVEN** map or candidate snapshot data is unavailable, **WHEN** hover or click resolves, **THEN** no gameplay intent is emitted and the system never exposes actor node, item node, Control node, Area2D, PhysicsBody or mutable runtime record as authority.
- **AC-31 — GIVEN** `debug_tie_break_evidence_enabled == true`, **WHEN** hover or click resolves, **THEN** debug/test output contains category, hit kind, cursor distance or score, shape-area participation, stable tie-break key, primary candidate, failure reason and final typed outcome, without affecting gameplay result.
- **AC-32 — GIVEN** candidate query adapter returns snapshots, **WHEN** Target Selection consumes them, **THEN** it treats them as immutable-by-contract facts, mutates no source records, and tests prove original candidate facts remain unchanged after click resolution.
- **AC-33 — GIVEN** hover category is hostile, loot, ground, unsupported or invalid/unresolved, **WHEN** UI/cursor/HUD presents feedback, **THEN** each category has at least one non-color cue such as outline shape, icon, cursor shape, marker style, label treatment, text or safe motion.
- **AC-34 — GIVEN** a click failure has a typed reason, **WHEN** feedback is presented, **THEN** at least one non-color feedback form can explain the reason, and gameplay does not read player-visible text to decide behavior.
- **AC-35 — GIVEN** persistent hostile selection was created by mouse, **WHEN** the player switches to keyboard/gamepad focus, **THEN** transient focus may change but persistent selection remains until explicit clear, replacement, death/removal, map unload or control reset.
- **AC-36 — GIVEN** hover or hostile selection exists at runtime, **WHEN** save/load or a new runtime session begins, **THEN** Target Selection starts with `NoHoverCandidate` and `NoSelection` and does not restore stale actor, loot or hover references.
- **AC-37 — GIVEN** QA test map contains controlled overlapping hostile/loot/ground/unsupported candidates, **WHEN** QA repeats the same screen-position click at least 10 times, **THEN** the same primary result occurs each time and debug evidence verifies the priority/tie-break path.
- **AC-38 — GIVEN** downstream systems are disabled or in debug-only mode, **WHEN** QA clicks hostile, loot and ground, **THEN** this system outputs only typed intents/failures and actor position, monster health, item ownership, inventory content and `MapSpaceState` remain unchanged.
- **AC-39 — GIVEN** `HostileSelected(A)` remains valid, **WHEN** the player clicks invalid area, projection failure area, blocked ground or unsupported gameplay object, **THEN** selection remains `HostileSelected(A)` and the system emits typed feedback without executing attack, pickup or movement.
- **AC-40 — GIVEN** this system is handed to QA, **WHEN** smoke/evidence review runs, **THEN** evidence exists for UI gate failure, projection failure, Hostile > Loot > Ground priority, same-category tie-break, no silent fallback, stale target transition, hover-vs-click fresh query, persistent hostile selection, ground handoff without execution, typed feedback reason, accessibility non-color cue and debug tie-break evidence; if blocking logic/integration evidence is missing, the system cannot be marked Done.

## Visual/Audio Requirements

交互目标 / 选择系统不绘制视觉元素、不播放音频，但必须输出足够明确的 typed feedback facts，让 UI/VFX/Audio 层可把 hover/click 结果表现为清晰、稳定、可访问的反馈。视觉方向遵循“赤金玛法”：暗暖世界底色、赤金/热橙高光、传奇式直接点击骨架，以及现代 UI 的清晰状态层级。

### Visual Feedback

| State / Result | Visual Requirement | Non-Color Cue Requirement |
|---|---|---|
| `HoverHostile` | 敌对目标可显示攻击向轮廓、脚下目标括号、锁定角标或 nameplate 强调。赤金/热红高光可表达战斗热度。 | 尖角/爪形括号、攻击 cursor、敌对图标或 nameplate 形态；不得只靠红色。 |
| `HoverLoot` | 掉落物获得高优先级拾取高亮、标签强调、拾取图标或短促脉冲；掉落可读性高于环境装饰。 | 手形/包裹图标、标签样式、边框形状、轻微脉冲；不得只靠金色。 |
| `HoverGround` | 可移动地面可显示低强度地面 marker、脚印、移动 cursor 或 cell preview。 | 圆形 marker、脚印、移动 cursor；不得抢过 hostile/loot 层级。 |
| `HoverUnsupported` | 支持禁用 affordance，如斜杠图标、锁、问号、禁用 cursor 或灰化框。 | 斜杠/锁/问号/禁用 cursor；不得伪装成 ground movement。 |
| `HoverUnresolved` / invalid | 短暂、低侵入的无效/未知反馈；不得产生攻击、拾取或移动预览。 | 叉号、断裂环、invalid cursor 或短文本。 |

Click feedback must preserve object semantics:

- Hostile click may request selection ring, target frame, nameplate emphasis, attack cursor flash or hostile-specific failure. Failure must still read as “I clicked a hostile,” never as ground movement.
- Loot click may request item label flash, pickup icon pulse, ground ring or pickup-target confirmation. Loot click feedback should be stronger than ordinary ground click to support `爆装有戏`.
- Ground click may request low-intensity target marker or movement cursor flash, but must not appear when hostile/loot is the primary candidate.
- Unsupported click shows “object exists but this interaction is unsupported.” Invalid click shows “no valid target/position.” These must be visually distinguishable.

Visual hierarchy must match primary priority:

```text
Hostile actor > Ground loot > Ground cell > Unsupported world object > Invalid/none
```

Visual-only props must not use the same affordance strength as supported gameplay candidates. Y-sort/draw order, rarity color, label draw order and scene-tree order must never change gameplay target priority.

### Audio Feedback

Audio is mapped from typed result/reason facts by the audio/presentation layer. This system never plays sounds directly.

| Result / Fact | Audio Direction | Requirement |
|---|---|---|
| Hostile hover | Usually silent, or very light cue in accessibility/debug modes. | Avoid noise when sweeping over monster groups. |
| Hostile click / selection | Short metallic/lock-on confirmation with warm combat character. | Means “target understood,” not attack hit. |
| Hostile unavailable | Short low rejection cue. | Distinct from generic invalid; preserves hostile semantics. |
| Loot hover | Usually silent, or light important-loot cue where approved. | Avoid loot-pile audio fatigue. |
| Loot click | Clear pickup-intent confirmation with bright/赤金 character. | Means “loot targeted,” not pickup success. |
| Loot unavailable | Soft but clear failure cue. | Should pair with typed text/icon reason where possible. |
| Ground click | Low-intensity movement confirmation. | Quieter/less salient than hostile or loot. |
| Unsupported | Short disabled/locked cue. | Means recognized but unsupported. |
| Invalid / unresolved | Very short low-intrusion invalid cue. | Must be rate-limited by presentation/audio config. |

Hover audio should be conservative. Click audio reflects interpreted intent; attack hit, pickup success, movement start/arrival and reward stingers are owned by Combat, Pickup, Click Movement and presentation/audio systems.

### Accessibility Requirements

- Hostile, loot, ground, unsupported and invalid states must each expose at least one non-color cue.
- Key failure reasons must be representable as text/icon/cursor/marker feedback: cannot reach target, cannot reach item, blocked ground, inventory full, item gone, target lost, unsupported object, unresolved/ambiguous target.
- Motion feedback must avoid high-frequency flashing; pulses and heat effects should be short and reducible by accessibility settings.
- Cursor, marker and label presentation must remain readable under UI scaling, high DPI, window resizing and visually busy maps.
- Audio cannot be the only feedback path. Visual/text/icon alternatives must exist for all key states and failures.
- Debug overlay is not an accessibility substitute; normal presentation must independently communicate target state.

### Ownership Boundary

This system owns typed interaction results, hover state, selection state, failure reason, intent kind and tie-break evidence only.

Presentation/UI/VFX owns cursor art, outlines, target rings, ground markers, nameplates, labels, tooltips, localization keys, rarity visuals, high-contrast modes, UI scaling and debug overlay display. Audio owns sound assets, mix, bus routing, cooldown/rate limiting and sound mapping. Combat owns attack legality and combat feedback. Pickup/Inventory owns pickup success/failure, inventory capacity and item removal feedback. Click Movement owns movement execution and movement feedback.

This system must never branch on cursor art, colors, tooltip text, sound names, VFX names, draw order, rarity visuals, Y-sort order or scene-tree order.

## UI Requirements

### Cursor and Hover UI

- UI must render typed hover results from this system; target selection itself does not own cursor assets or cursor drawing.
- Cursor/hover feedback must distinguish `hostile`, `loot`, `ground`, `unsupported` and `invalid/unresolved` before click where possible.
- If multiple candidates overlap, UI must receive the resolved primary category and enough debug metadata to explain why that candidate won priority.
- Cursor feedback should be immediate and stable under small pointer jitter. Rapid flicker between targets near entity/loot/ground boundaries is a UX defect.
- Hovering hostile may expose target id, display key/name if available, hostility state and targetability status to UI.
- Hovering loot may expose item id, display key/name if available, stack count if relevant and pickup-readiness/status facts.
- Hovering ground may expose movement-target status, but UI must not treat hover as movement execution.
- Hovering unsupported objects must show explicit unsupported affordance rather than making them look like empty ground.

### Click Result UI

- Click processing outputs a typed click result; UI decides how to show it.
- UI must be able to show whether the click resolved to hostile, loot, ground, unsupported or invalid.
- Failed click attempts must be explainable through short message, icon, cursor, tooltip or status line. Repeated identical failures may be rate-limited by presentation, but the latest typed reason must remain available.
- UI interactions such as inventory slots, equipment slots, menus, HUD panels, text input, modal overlays, drag operations and dropdowns must not trigger world targeting.
- If a click is blocked by UI focus or modal state, the target selection result is no world interaction.

### Persistent Hostile Selection UI

- Persistent hostile selection is separate from transient hover.
- UI should be able to show selected hostile via target ring, target frame, nameplate emphasis, HUD target panel or outline.
- Players must be able to distinguish: current hover target, current selected hostile, and selected target validity.
- If selected hostile dies, despawns, becomes invalid, changes map or becomes not targetable, UI must receive a typed invalidation reason.
- Selection invalidation should not silently disappear without feedback unless caused by an intentional player clear or replacement action.
- Persistent hostile selection must not prevent hovering or inspecting loot, ground or unsupported targets.

### Typed Feedback Contract

UI-facing feedback must be typed data, not hardcoded UI strings in target-selection logic. Result payloads should include:

- result type / primary intent kind;
- target category;
- target id if any;
- candidate priority result and optional tie-break evidence;
- validity flag;
- failure reason if invalid/unsupported/unavailable;
- optional display metadata keys for UI rendering.

Presentation may map stable reason IDs to localized text such as `no_target`, `blocked_cell`, `out_of_range`, `unsupported_target`, `target_despawned`, `target_not_hostile`, `loot_unavailable`, `ui_focus_blocked`, or `ambiguous_target_unresolved`. Gameplay logic must not branch on these localized strings.

### Input Gate and Godot 4.6 Dual-Focus Requirements

- UI input consumption takes priority over world target selection.
- World targeting is gated when a modal UI is open, pointer is over an interactive UI panel, text input/dropdown/drag operation is active, inventory/equipment slot receives the event, or ownership is unknown.
- Godot 4.6 separates mouse/touch focus from keyboard/gamepad focus. Target selection must use same-event ownership facts from the input gate rather than assuming keyboard focus determines world eligibility.
- Keyboard/mouse is the primary path: mouse hover resolves transient target information and primary click resolves a click target.
- Gamepad support is partial, but the system should permit future abstract cursor, reticle, nearest-target query or focus-based world selection sources without coupling to UI focus navigation.
- Gamepad UI focus movement must not mutate world hover or hostile selection unless an explicit world-selection input is used.

### Accessibility Requirements

- All hover/click categories must be understandable without color alone.
- Text feedback must support scaling and remain readable at project minimum UI scale.
- Cursor and selection feedback must remain legible against visually busy maps, sprites, loot piles and combat scenes.
- Persistent hostile selection must be visually distinct from hover selection for color vision deficiencies.
- Audio or haptic cues require equivalent visual/text feedback.
- Feedback timing must avoid rapid flashing or repeated flicker.
- Keyboard-only and gamepad-only users must have a future-compatible path to understand target state where applicable, even if Phase 1 gamepad world targeting remains partial.

### Debug and Developer UI

Debug tools may display target resolution information without changing behavior:

- input/UI gate state;
- pointer screen/world/projection facts;
- candidate list;
- candidate priority and tie-break order;
- rejected candidates and reasons;
- winning primary candidate;
- final hover result;
- final click result;
- persistent hostile selection state;
- stale/invalidation reason.

Debug overlay is optional and must not be required for normal player feedback. A target state that is only clear in debug mode fails UI requirements.

### UI Ownership Boundary

This system owns target resolution only. It does not own cursor art, cursor rendering, selection ring rendering, nameplates, health bars, tooltips, combat HUD, inventory UI, pickup notifications, modal prompts, sound effects, haptics, localization strings or final UI copy. UI systems may consume typed results to render feedback, but cannot change target priority or gameplay outcome.

## Open Questions

| Question | Owner | Target Resolution | Current Handling |
|---|---|---|---|
| What are the final approved enum/DTO names for interaction results, candidate categories, failure reasons and selection states? | Technical Director / Gameplay Programmer | Before implementation ADR or dev story slicing | Use names in this GDD as semantic placeholders; final code names must preserve meanings. |
| Which system owns the concrete Candidate Query Adapter that gathers hostile, loot, ground and unsupported snapshots? | Technical Director | Before implementation ADR | This GDD requires immutable-by-contract snapshots but does not choose implementation location. |
| What is the exact Combat handoff once Combat GDD exists: `select_hostile_intent`, `attack_intent`, `approach_for_attack_intent`, or a staged result envelope? | Combat GDD owner | During Combat / Targeting ADR | MVP may emit select/approach semantic facts and leave final attack legality to Combat. |
| What is the exact Pickup handoff once Pickup GDD exists: `loot_target_intent`, `pickup_intent`, `approach_for_pickup_intent`, or staged claim request? | Pickup / Ground Drop GDD owner | During Drop/Pickup GDD and ADR | MVP keeps loot intent separate from pickup commit and inventory mutation. |
| Should explicit target clear use right-click, Escape, keyboard shortcut, gamepad button, timeout, or only target replacement/death in Phase 1? | UX Designer / Combat Designer | Before Combat/HUD implementation | MVP supports explicit clear conceptually but does not bind input yet. |
| Should ground click ever clear hostile selection in later combat tuning? | Combat Designer / UX Designer | After combat playtest | MVP default is no; persistent selection remains through ground movement. |
| Should loot click clear hostile selection after successful pickup or only if combat target becomes stale? | Combat Designer / UX Designer | After pickup/combat integration playtest | MVP default is no; looting preserves hostile awareness. |
| What exact near-hit radii are comfortable for final art scale and camera zoom? | UX Designer / QA | During first playable test map tuning | Use MVP defaults (`hostile_near_hit_radius_px = 24`, `loot_near_hit_radius_px = 32`) until tested. |
| How should partial gamepad world targeting work: virtual cursor, nearest hostile cycling, reticle, or focus-based world selection? | UX Designer / Accessibility Specialist | Post-MVP or when gamepad support is promoted | Phase 1 keeps gamepad partial and does not block future abstraction. |
| Which visual/audio assets represent hostile, loot, ground, unsupported and invalid states? | Art Director / Audio Director | After art bible and audio direction approval | GDD defines requirements only; run `/asset-spec` after visual/audio direction is ready. |
| Which failure reason strings are localized and how are they grouped for player-facing brevity? | Localization Lead / UX Designer | Before UI/HUD stories | Gameplay emits typed reason IDs; UI/localization owns player-facing copy. |
| How much debug evidence is shown in release builds versus editor/test builds? | Technical Director / QA Lead | Before implementation plan | Debug evidence is required for tests; release overlay is optional. |
