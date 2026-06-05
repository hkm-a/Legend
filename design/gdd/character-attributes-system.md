# 角色属性系统

> **Status**: Approved for Implementation Planning — implementation blocked pending real Godot test runner and story/test slicing
> **Author**: hkm + Claude Code Game Studios
> **Last Updated**: 2026-06-05
> **Implements Pillar**: Primary — 每次登录都带走成长; Supports — 稳刷不断流 / 传奇骨架，现代皮肤 / 爆装有戏
> **Quick reference** — Layer: `Foundation` · Priority: `MVP` · Key deps: `OpenMir2 行为映射 Spike`
> **Review note**: 2026-06-04 full design review verdict was `MAJOR REVISION NEEDED`; re-review returned targeted `NEEDS REVISION`; formula/registry/testability fixes have been applied. ADR-0006 through ADR-0014 are Accepted. Implementation Done remains blocked until an approved real Godot GDScript test runner executes the required story/unit tests.

## Overview

角色属性系统是 Phase 1 离线刷怪爆装切片的角色数值合同层。它定义玩家、普通怪物和后续可扩展 actor 的基础输入、当前资源、派生属性、属性来源、只读快照、属性变化事件、装备修正边界、校验失败语义，以及 HUD、装备比较、战斗、生命/死亡、存档、怪物生成和成长反馈系统读取属性时必须遵守的规则。

该系统不直接决定攻击流程、最终伤害公式、死亡流程、装备词条生成、掉落价值、UI 布局或 OpenMir2 最终数值。它的职责是保证两件事同时成立：

1. **数值真实可信**：最终属性由明确输入和数据配置计算，不能由 UI、装备节点、战斗脚本或全局临时状态随意写入。
2. **成长可被玩家理解**：当玩家升级、获得装备并穿戴后，下游 UI / 反馈系统能用属性系统提供的主属性、战力代理、delta、原因和可见性标签，让玩家快速确认“我穿了，我涨了，我杀怪更快了”。

Phase 1 可以使用明确标记的 MVP provisional 数值和公式来验证 30 秒刷怪爆装循环。任何声称 OpenMir2-authentic 的属性名、初始值、成长曲线、装备挂接规则或资源行为，都必须引用 `OpenMir2 行为映射 Spike` 的 E3/E4 evidence ID 或源码位置。

## Player Fantasy

玩家不需要理解完整属性表。玩家需要在一次短循环中感到：怪物死了、装备爆了、我捡起来、我能立刻判断是否更好、我穿上后角色明确变强。最小体验目标是：

- 装备比较能快速告诉玩家“更强 / 更弱 / 侧向变化 / 未知”；
- 穿戴后至少一个主变化被清楚展示，例如 `攻击 8–14 → 10–18 ↑ +2~+4`、`生命上限 +10` 或 `战力 +12`；
- 对标准 Phase 1 怪物，真正的攻击成长应能被后续伤害/战斗系统验证为更高伤害、较少击杀次数或更短 TTK；
- inactive / reserved / debug-only stats 不得干扰玩家判断装备价值；
- invalid、stale、debug failure 不能伪装成普通成长反馈。

Phase 1 的数值可以不是 OpenMir2-authentic，但必须是数据驱动、可追溯、被标记为 provisional，并能支持核心幻想：**这趟刷怪没有白刷，我的角色正在一步一步变强。**

## Detailed Rules

### 1. Ownership Boundary

角色属性系统 owns：

- actor 属性输入的语义合同；
- Phase 1 stat registry；
- 属性计算与校验；
- 只读 attribute snapshot；
- snapshot version；
- snapshot / resource / invalidation event payload 语义；
- 属性 delta 和 player-facing growth handoff 所需的最小数据；
- invalid / stale / debug evidence 语义。

角色属性系统 does not own：

- damage formula、hit / crit / attack speed；
- death、revive、death attribution、loot-on-death；
- item definition、equip legality、drop value；
- HUD layout、stat panel layout、VFX、audio、animation；
- save file format；
- OpenMir2 evidence mapping itself。

禁止流程：装备、HUD、combat、AI、save/load、scene node 或 debug panel 直接写最终属性、base stats、derived stats 或 snapshot 内部字段。所有变更必须通过 approved source update、resource mutation 或 debug mutation boundary。

### 2. Phase 1 Stat Registry and Visibility

Phase 1 使用 MVP provisional stat registry。该 registry 同时记录属性是否进入 snapshot、是否参与战斗、是否可被装备修正、是否可被玩家 UI 显示，以及是否仅为 debug / reserved。

| Stat / Field ID | Actor Type | Category | Snapshot Required | Base/Input Required | Modifier Target | Combat Active P1 | Player Visible P1 | Debug Visible | Source Status |
|---|---|---|---:|---:|---:|---:|---:|---:|---|
| `actor_id` | all | identity | Yes | Yes | No | Yes | No | Yes | Project-local |
| `actor_type` | all | identity | Yes | Yes | No | Yes | No | Yes | Project-local |
| `class_id` | player | identity | Yes for player | Yes for player | No | Indirect | Maybe label | Yes | Provisional |
| `monster_template_id` | monster | identity | Yes for monster | Yes for monster | No | Indirect | No | Yes | Provisional |
| `level` | all | identity / progression | Yes | Yes | No | Indirect | Optional | Yes | Provisional |
| `health_max` | all combat actors | derived resource max | Yes | derived from config/input | Yes | Yes | HUD/stat panel | Yes | Provisional |
| `health_current` | all combat actors | current resource | Yes | Yes | No | Yes | HUD | Yes | Provisional |
| `mana_max` | player; monster if configured | reserved resource max | Yes for player; optional for monster | fixture/config | Yes only if enabled | No unless skill system enables | Hidden by default | Yes | Provisional |
| `mana_current` | player; monster if configured | reserved current resource | Yes for player; optional for monster | fixture/config | No | No unless skill system enables | Hidden by default | Yes | Provisional |
| `physical_attack_min` | all combat actors | derived combat stat | Yes | fixture/config | Yes | Yes | comparison/stat panel | Yes | Provisional |
| `physical_attack_max` | all combat actors | derived combat stat | Yes | fixture/config | Yes | Yes | comparison/stat panel | Yes | Provisional |
| `physical_defense_min` | all combat actors | derived combat stat | Yes | fixture/config | Yes | Yes | comparison/stat panel | Yes | Provisional |
| `physical_defense_max` | all combat actors | derived combat stat | Yes | fixture/config | Yes | Yes | comparison/stat panel | Yes | Provisional |
| `magic_defense_min` | player; monster if magic threat exists | reserved combat stat | Required for player snapshot; optional for ordinary P1 monster unless enabled | fixture/config | Yes only if enabled | No by default | Hidden by default | Yes | Provisional |
| `magic_defense_max` | player; monster if magic threat exists | reserved combat stat | Required for player snapshot; optional for ordinary P1 monster unless enabled | fixture/config | Yes only if enabled | No by default | Hidden by default | Yes | Provisional |
| `accuracy` | player; monster if hit checks enabled | secondary combat stat | Required but combat-inactive unless enabled | fixture/config | Yes only if enabled | No by default | Hidden by default | Yes | Provisional |
| `evasion` | player; monster if hit checks enabled | secondary combat stat | Required but combat-inactive unless enabled | fixture/config | Yes only if enabled | No by default | Hidden by default | Yes | Provisional |
| `magic_attack_min/max` | future | reserved schema | No | No | No | No | No | Debug/schema only | Provisional |
| `skill_attack_min/max` | future | reserved schema | No | No | No | No | No | Debug/schema only | Provisional |

Rules:

- Snapshot completeness is not the same as player-facing display.
- Inactive or reserved stats must not appear in normal HUD, equipment comparison, or growth celebration unless their gameplay effect is implemented and enabled.
- Ordinary Phase 1 monsters may use an actor-type-specific required stat set. They must not be forced to fabricate `class_id` or unused MP/magic fields unless the config explicitly chooses a universal actor schema.
- Equipment modifiers may target `health_max`, `mana_max` only if enabled, physical attack/defense pairs, and enabled secondary stats. Equipment may not target `health_current` or `mana_current` in Phase 1.

### 3. Player-Facing Growth Handoff Rule

属性系统 must provide enough structured data for downstream presentation to communicate growth without recomputing gameplay rules.

Every accepted growth-relevant rebuild must expose:

| Field | Meaning |
|---|---|
| `growth_reason` | `equipment_equipped`, `equipment_removed`, `equipment_replaced`, `level_up`, `template_changed`, `debug_change`, etc. |
| `growth_salience` | `none`, `minor`, `meaningful`, `major`, `debug_only`, `invalid` |
| `primary_comparison_stat` | Phase 1 default: `physical_attack_pair` for offensive equipment; fallback `health_max` / `physical_defense_pair` for survivability equipment. |
| `combat_power_before` / `combat_power_after` | Provisional display-only power score if enabled. |
| `combat_power_delta` | Signed difference for quick comparison; not OpenMir2-authentic. |
| `visible_delta_summary` | Ordered, filtered list of player-visible deltas, capped by presentation config. |
| `hidden_delta_summary` | Debug/details-only deltas. |
| `invalid_or_stale_status` | Whether this change is safe for normal player-facing celebration. |

Phase 1 provisional combat power is allowed for readability and MVP validation. It must be labeled `mvp_provisional` and must not be called OpenMir2-authentic.

Recommended provisional display formula:

`combat_power = weighted_attack + weighted_survival + weighted_secondary`

Where exact weights are fixture/config values. Until a dedicated equipment/combat-power ADR exists, the minimum Phase 1 rule is:

- attack range increase is primary for weapon-like upgrades;
- HP max and physical defense range are secondary survivability improvements;
- inactive/reserved stats contribute 0 to player-facing combat power;
- combat power is display/comparison aid, not damage formula authority.

### 4. Equipment Comparison and Preview Boundary

Phase 1 must choose one approved comparison path before equipment UI implementation:

1. **Pure preview query**: attribute system accepts current sources plus a hypothetical modifier set and returns a non-authoritative `AttributePreviewResult` with no snapshot version publication.
2. **Equipment-owned preview**: equipment system computes resolved modifier preview and asks attribute system for formula-only delta; UI receives preview data from equipment.
3. **No preview**: Phase 1 only shows post-equip delta, not pre-equip comparison.

Recommended default: **Pure preview query** for formula correctness, with equipment owning item legality and modifier resolution.

Preview rules:

- preview must not mutate authoritative source state;
- preview must not increment authoritative `snapshot_version`;
- preview must identify invalid modifiers and unsupported stats;
- preview may use current resource values from the current valid snapshot, but max-resource changes must be labeled preview-only;
- preview output uses the same visible/hidden stat policy as real deltas.

### 5. Attribute Layers

| Layer | Meaning | Persistence | Write Authority | Read Access |
|---|---|---:|---|---|
| `identity_fields` | actor ID, actor type, player class or monster template, level. | Persistent / template-owned | creation/load/spawn/progression | snapshots/debug |
| `base_stats` | source values before modifiers. | Persistent or template-owned | creation/progression/spawn/load/migration | attribute system/debug |
| `current_resources` | current HP/MP values. | Persistent for player; actor-state for monsters | approved resource mutation/load/spawn | snapshot/resource consumers |
| `derived_stats` | final effective stats from base/growth/modifiers/config. | Not authoritative save data | attribute system only | snapshot consumers |
| `stat_modifiers` | equipped item/status/debug contributions. | source persists; aggregate does not | source system owns; attribute system aggregates | attribute system/debug |
| `attribute_snapshot` | immutable/read-only current truth view. | Not authoritative save data | attribute system publishes | HUD, combat, AI, save, debug |
| `attribute_debug_trace` | source breakdown / detailed failure evidence. | Debug/test only | generated lazily or on invalid result | QA/debug |

`current_resources` are not ordinary modifier-targetable stats. A damage/heal/spend/restore operation must use resource mutation semantics, not `StatModifier`.

### 6. Structural Rebuild vs Resource Mutation

The system has two update paths:

#### Structural rebuild path

Used for base stat changes, level/growth changes, equipment changes, modifier changes, config/template changes, and load rebuild.

Required behavior:

1. stage source inputs;
2. validate input and config;
3. aggregate active modifiers in one pass O(M + S);
4. compute derived stats;
5. validate pairs and output invariants;
6. clamp current resources only where correction policy allows;
7. publish one new valid snapshot version if successful;
8. emit one compact `AttributeRebuildEvent`;
9. on failure, do not replace current valid snapshot; return `AttributeRebuildFailure` with previous valid snapshot marked stale/display-only if available.

#### Resource mutation path

Used for HP/MP changes such as damage, heal, spend, restore, load correction, or max-resource clamp.

Required behavior:

1. validate resource mutation request;
2. use cached current max from latest valid structural snapshot;
3. clamp current resource to legal range only when the resource reason permits correction;
4. publish updated resource state and snapshot version or resource version according to ADR;
5. emit compact `ResourceChangedEvent`;
6. do not re-aggregate equipment modifiers for resource-only changes.

Resource correction policy by reason:

| Reason | Clamp Allowed | Player-Facing Meaning | Growth Feedback Allowed | Event / Failure Rule |
|---|---:|---|---:|---|
| `damage` | Yes | Combat damage/resource loss | No | Emits `damage`; death interpretation belongs to life/death. |
| `heal` | Yes | Healing/recovery | No, unless a future feedback rule says healing is celebratory | Emits `heal`; overflow clamps to max. |
| `spend` | Yes if spend request is valid | Resource spend | No | Invalid spend request may fail before clamp if the resource system requires affordability. |
| `restore` | Yes | Resource restoration | No by default | Emits `restore`; overflow clamps to max. |
| `max_resource_reduction` | Yes | Resource correction, not damage | No | Emits `max_resource_clamp`; must not show damage numbers. |
| `max_resource_increase` | N/A for current unless configured | Capacity growth | Yes for max-stat delta only | Default `keep_current_with_feedback`; emits positive max-resource delta. |
| `load_correction` | Only if migration/recovery policy approves | Recovery/debug, not normal gameplay | No | Otherwise returns structured migration/rebuild failure. |
| `spawn_initialization` | Yes within fixture/template bounds | Initial actor state | No | Invalid live monster spawn fails combat readiness. |
| `debug_change` | Development-only | Debug/QA | No | Requires approved debug mutation boundary and trace. |

### 7. Atomic Source Update Rule

Equipment replacement, level-up, load rebuild, and spawn initialization must be transaction-like. Consumers must never observe an intermediate state where:

- old and new equipment both contribute;
- removed equipment still contributes after source commit;
- new equipment appears equipped but attributes still use old modifiers without stale/error label;
- failed rebuild is celebrated as successful growth.

If rebuild fails after a staged change:

- authoritative source commit/rollback policy must be defined by ADR or technical design;
- attribute query must return `failed_rebuild(previous_valid_snapshot, failure_reasons, failed_source_version)` or equivalent;
- combat must not consume failed rebuild as current truth;
- UI may show previous valid values only with stale / unavailable status.

### 8. Validation Pipeline

Validation is staged. Implementers must not guess whether to clamp or fail.

#### Stage A — Config validation

Before any actor rebuild:

- all required stat definitions exist;
- every clamp has `min_bound <= max_bound`;
- every resource has `resource_min_bound <= resource_max_after` after max stat calculation;
- pair-bound config satisfies `stat_min_bound(pair_min) <= stat_max_bound(pair_max)`;
- enabled/reserved/unsupported stat statuses are explicit;
- unsupported modifier operations have a selected policy;
- source-authentic labels have evidence IDs.

Config failure returns structured failure and must not publish a gameplay snapshot.

#### Stage B — Source/input validation

Validate actor identity, level, template/class ID, base/current payload, active source IDs, modifier target IDs, modifier operation, numeric finite-ness, and duplicate modifier source keys.

#### Stage C — Calculation and correction

Allowed corrections:

- derived stat values may clamp to stat bounds if their stat config allows clamp;
- current resources may clamp for damage/heal/spend/restore, max-resource reduction, load correction, or spawn initialization according to reason policy;
- corrections must emit traceable reason codes.

Not allowed:

- clamping inverted config bounds;
- silently replacing missing required fields;
- silently activating reserved stats;
- converting unsupported modifier operations into supported ones;
- using raw invalid OpenMir2-authentic claims without evidence.

#### Stage D — Output validation

Final publishable snapshots must satisfy:

- required actor-type fields present;
- effective stat values within bounds;
- pair stats valid where active / required;
- current resources within legal range;
- combat-ready player and ordinary monster actors have `health_max > 0`;
- ordinary live monsters have `health_current > 0` at spawn unless explicitly configured as noncombat/dead test actor;
- snapshot version and schema/config version recorded.

### 9. Invalid Modifier Policy

Phase 1 default policy:

| Invalid Case | Policy | Snapshot Impact |
|---|---|---|
| unsupported operation such as percent/multiply/override | reject structural change; keep previous valid snapshot if any | failed rebuild |
| unknown stat target | reject source or preview; structured failure | failed rebuild / preview failure |
| known but inactive reserved stat | exclude from player-facing output; reject if source claims active gameplay effect | no gameplay contribution |
| inventory-only item contributes modifier | exclude with reason | valid if no other blocker |
| duplicate modifier source key | blocking unless explicit stacking group allows | failed rebuild |
| expired/inactive status source | exclude with reason | valid if no other blocker |
| source-authentic claim without E3/E4 evidence | reject label; value may be provisional only if relabeled | schema/evidence failure |

Modifier identity must be a stable composite key: actor ID, source system, source instance ID, optional slot ID, modifier row ID, target stat, and stacking group.

### 10. Snapshot, Version, and Query Contract

`AttributeSnapshot` is an immutable/read-only semantic object. Implementation must enforce this through ADR-approved representation, such as typed `RefCounted` with getter-only API, defensive copies, or immutable value object. Mutable nested `Dictionary` / `Array` snapshots passed directly to consumers are not acceptable unless the ADR proves mutation cannot affect authoritative state.

Snapshot must include or be queryable for:

- actor ID and actor type;
- actor-local monotonic snapshot version;
- snapshot schema version;
- stat registry/config version;
- source status summary (`mvp_provisional`, `openmir2_evidence_pending`, `openmir2_verified`);
- effective combat stats;
- current resources;
- valid/invalid/stale state;
- lightweight reason codes or changed mask.

Full debug source breakdown must be generated lazily or only for invalid/debug requests. Normal runtime snapshots must not carry heavy formatted debug strings or unbounded history.

### 11. Event Contract

Events are data-first domain events. Godot signals may adapt them later, but the core attribute logic must be testable without scene tree or Autoload.

Required event categories:

| Event | Trigger | Minimum Payload | Notes |
|---|---|---|---|
| `AttributeRebuildEvent` | successful structural rebuild | actor ID, old version, new version, changed mask/stat IDs, visible deltas, hidden deltas, reason, growth salience, validity | no full old+new snapshot by default |
| `ResourceChangedEvent` | HP/MP current mutation | actor ID, resource ID, before, after, max, delta, resource reason, snapshot/resource version | no full derived rebuild |
| `AttributeInvalidatedEvent` | failed rebuild or config/source failure | actor ID, previous valid version if any, failed source version, failure reason codes, player-safe status | debug detail pull-based |
| `AttributePreviewResult` | hypothetical equip/preview query | actor ID, preview deltas, visible summary, invalid reasons, no authoritative version increment | non-authoritative |

Events must be coalesced per actor transaction/update point. UI, combat, AI, and debug tools must not rely on callback order as gameplay authority. Consumers must not trigger rebuilds by reading snapshots.

### 12. UI / Presentation Handoff

The attribute system does not own UI layout, VFX, sound, or animation, but it must provide enough metadata for downstream systems.

Player-facing display policy:

- HUD binds to the local player actor ID supplied by player/session/controller state; it must ignore unrelated monster attribute events unless a target frame explicitly requests them.
- HUD default Phase 1 display: HP current/max. MP is hidden unless skill/resource UI scope enables it.
- Equipment comparison default: show no more than 2–3 primary deltas; inactive/reserved stats hidden; full breakdown debug/details only.
- Positive/negative deltas must not rely on color alone. Use signs, arrows, labels, or icons plus color.
- Invalid/stale snapshots must show player-safe states such as `属性暂不可用` or `装备变更未生效`; raw failure reasons appear only in debug/QA surfaces.
- Growth feedback may celebrate only valid changes with allowed reasons (`equipment_equipped`, `equipment_replaced`, `level_up`, etc.). It must not celebrate clamp correction, invalid rebuild, load migration, damage, or debug-only changes.

Player-facing labels must come from localization/display metadata, not raw stat IDs. Stat IDs, reason IDs, class IDs, actor type labels, provisional/source-authentic labels, and invalid reason codes require localization/display keys.

### 13. Save/Load and Persistent Input Boundary

Save/load must persist authoritative inputs, not final derived stats as truth.

Minimum `AttributePersistentInput` contract:

- actor identity and actor type;
- player `class_id` or monster template reference as applicable;
- level/progression payload;
- base stat payload or reference to base stat table entry;
- current resource values;
- equipped item references / slot IDs;
- persistent modifier source references if any;
- stat registry version;
- config/fixture version;
- source status labels.

Load rebuild must return a rebuild result. Missing config/equipment, incompatible versions, impossible current resource state, or unsupported modifier schema produce structured failure or approved migration. Persisted final snapshots may be used only for debug comparison.

### 14. ADR / Technical Design Gates

Before implementation stories are marked ready, the project must create ADRs or technical designs for:

1. attribute data representation and stat ID typing;
2. snapshot/query API shape and read-only enforcement;
3. event/signal contract and scene-tree-independent core;
4. atomic source update / transaction model;
5. save/load persistence boundary for base/current/modifier sources;
6. fixture/config loading strategy for MVP provisional values;
7. formula-only GUT test strategy without scene tree, UI, or Autoload;
8. Godot Resource duplication/shared-reference policy if `.tres` Resources are used;
9. combat power/main stat display proxy ownership.

These are implementation-blocking, not optional recommendations. ADRs must target Godot 4.6.3 and verify current APIs against `docs/engine-reference/godot/VERSION.md`, `breaking-changes.md`, and `deprecated-apis.md`.

### 15. Performance Rules

- Structural rebuilds must not run per frame.
- Resource-only HP/MP changes must not re-aggregate all modifiers.
- Modifier aggregation must be one pass O(M + S), where M is active modifiers and S is enabled stats.
- Runtime hot paths should use ADR-approved compact stat IDs, not repeated string dictionary lookups in combat/resource loops.
- Full old+new snapshot payloads and formatted debug traces are debug-only or pull-based.
- HUD should update from events or cached snapshot reference, not rebuild/poll full stats every frame.
- Combat captures minimal snapshot references/versions or resolved combat inputs once per action phase.
- Ordinary monster updates must not broadcast UI-grade deltas globally unless subscribed.

## Formulas

The formulas below are Phase 1 MVP provisional formulas and are registered as cross-system facts in `design/registry/entities.yaml`. If any formula changes, the registry must be updated in the same revision.

Formula-wide conventions:

- Formula outputs are either a typed success value or `structured_failure(reason_code, context...)`.
- If a formula depends on another formula that returns structured failure, the parent formula must propagate that failure reason or aggregate it into a deterministic failure-reason set. Parent formulas must not coerce failed child outputs to `0`, omit them silently, or clamp them as normal values.
- Phase 1 numeric inputs are finite signed integers unless a formula explicitly says otherwise. Normal MVP fixtures use `0–9999` for non-negative combat/resource stats and `-9999–9999` for flat modifier test values. Values outside the configured technical safe range produce structured failure unless the stat-specific correction policy explicitly allows clamp.
- Intermediate aggregation must be overflow-safe. Until the ADR chooses concrete integer representation, tests must include an overflow/technical-limit case that returns structured failure rather than wrapping.
- Display-only formulas may produce provisional integers after rounding; they are not OpenMir2-authentic and do not define combat damage.

### 1. `effective_stat`

Preconditions:

`stat_config_valid(stat_id) = stat_id_enabled AND stat_min_bound(stat_id) <= stat_max_bound(stat_id)`

If `stat_config_valid(stat_id) = false`, return `structured_failure(invalid_stat_config, stat_id)`.

Formula:

`effective_stat(stat_id) = clamp(base_stat_value(stat_id) + sum_active_flat_modifiers(stat_id), stat_min_bound(stat_id), stat_max_bound(stat_id))`

Where:

`sum_active_flat_modifiers(stat_id) = Σ modifier_value_i for each active add_flat modifier i where modifier_stat_id_i = stat_id`

Variables:

| Variable | Type | Phase 1 Range / Constraint | Description |
|---|---|---|---|
| `stat_id` | enum / ADR-defined compact ID | enabled stat registry ID | Attribute being calculated. |
| `base_stat_value(stat_id)` | int | within fixture/config technical safe range | Source value before modifiers. |
| `modifier_value_i` | int | fixture/config technical safe range | Active flat modifier value. |
| `sum_active_flat_modifiers` | int | computed overflow-safely | Sum of valid active flat modifiers. |
| `stat_min_bound` | int | `<= stat_max_bound` | Lower final bound. |
| `stat_max_bound` | int | `>= stat_min_bound` | Upper final bound. |

Phase 1 effective stats are integers only. Any operation that produces non-integer output is unsupported.

**Output Range:** success value is an integer in `[stat_min_bound(stat_id), stat_max_bound(stat_id)]`; otherwise structured failure such as `invalid_stat_config`, `unknown_stat_id`, `scalar_input_out_of_bounds`, `unsupported_modifier_operation`, or `numeric_overflow`.

**Example:** `base_stat_value(physical_attack_min) = 8`, active `add_flat` weapon modifier `+2`, bounds `0–9999`. `effective_stat(physical_attack_min) = clamp(8 + 2, 0, 9999) = 10`. If bounds are `10–5`, the result is `structured_failure(invalid_stat_config, physical_attack_min)` before clamp.

### 2. `effective_stat_pair`

Preconditions:

`pair_bound_config_valid(pair_id) = stat_min_bound(pair_min_stat_id) <= stat_max_bound(pair_max_stat_id)`

If false, return `structured_failure(invalid_pair_bound_config, pair_id)`.

Formula:

`pair_min_final = effective_stat(pair_min_stat_id)`

`pair_max_final = effective_stat(pair_max_stat_id)`

`effective_stat_pair(pair_id) = success(pair_min_final, pair_max_final) IF pair_min_final <= pair_max_final; otherwise structured_failure(invalid_stat_pair, pair_id, pair_min_final, pair_max_final)`

MVP pair IDs: `physical_attack`, `physical_defense`, active `magic_defense` only if enabled.

Variables:

| Variable | Type | Phase 1 Range / Constraint | Description |
|---|---|---|---|
| `pair_id` | enum / compact ID | active pair registry ID | Logical min/max pair. |
| `pair_min_stat_id` | stat ID | active stat ID | Min stat referenced by the pair. |
| `pair_max_stat_id` | stat ID | active stat ID | Max stat referenced by the pair. |
| `pair_min_final` | int or structured failure | result of `effective_stat(pair_min_stat_id)` | Final min value if child formula succeeds. |
| `pair_max_final` | int or structured failure | result of `effective_stat(pair_max_stat_id)` | Final max value if child formula succeeds. |

**Output Range:** `success(pair_min_final, pair_max_final)` with `pair_min_final <= pair_max_final`, or structured failure. Child `effective_stat` failures propagate before pair comparison.

**Example:** base physical attack `8–14`, weapon modifiers `+2/+4`, bounds `0–9999`. Child effective stats are `10` and `18`, so `effective_stat_pair(physical_attack) = success(10, 18)`. If min becomes `20` while max is `10`, result is `structured_failure(invalid_stat_pair, physical_attack, 20, 10)`.

### 3. `current_resource_after`

Preconditions:

`resource_bounds_valid(resource_id) = resource_min_bound(resource_id) <= resource_max_after(resource_id)`

`combat_ready_health_valid(actor_type) = health_max > 0 for player and ordinary live monster actors`

If bounds are invalid, return `structured_failure(invalid_resource_bounds, resource_id)`.

Formula:

`current_resource_after(resource_id) = clamp(resource_current_before + resource_delta, resource_min_bound(resource_id), resource_max_after(resource_id))`

Where:

`resource_max_after(resource_id) = effective_stat(resource_max_stat_id)`

Rules:

- `health_max <= 0` is invalid for combat-ready player and ordinary live monster snapshots.
- `health_current = 0` may be a valid resource value, but life/death system owns death interpretation.
- Negative HP/MP values are invalid in Phase 1 unless a later life/death GDD explicitly permits them.
- When max HP/MP increases, Phase 1 must use one configured policy: `keep_current_with_feedback`, `preserve_ratio`, or `fill_by_delta`. Default for revised GDD: `keep_current_with_feedback`, meaning current value stays but event payload must expose positive max-resource growth so UI does not present it as damage or downgrade.

Variables:

| Variable | Type | Phase 1 Range / Constraint | Description |
|---|---|---|---|
| `resource_id` | enum | `health`, `mana` | Resource being updated. |
| `resource_current_before` | int | within current resource technical range or approved correction case | Current value before mutation. |
| `resource_delta` | int | reason-specific signed amount | Damage/spend negative; heal/restore positive; max-only changes usually `0`. |
| `resource_min_bound` | int | default `0` in Phase 1 | Lowest allowed current value. |
| `resource_max_after` | int or structured failure | result of `effective_stat(resource_max_stat_id)` | Current max after structural state/config. |
| `resource_reason` | enum | correction policy table | Determines whether clamp is allowed or failure is required. |

**Output Range:** success value is an integer in `[resource_min_bound(resource_id), resource_max_after(resource_id)]`; otherwise structured failure such as `invalid_resource_bounds`, propagated max-stat failure, or migration/resource request failure.

**Example:** `health_current = 42`, `resource_delta = 0`, new `health_max = 35`, `resource_min_bound = 0`, reason `max_resource_reduction`. `current_resource_after(health) = clamp(42, 0, 35) = 35`, emitting `max_resource_clamp`, not damage. For max increase from `40` to `50` with default `keep_current_with_feedback`, `health_current = 40` remains `40`, while max-resource delta is exposed as `+10`.

### 4. `snapshot_valid`

`snapshot_valid = config_valid AND identity_valid AND all_required_inputs_present AND source_input_valid AND all_active_stats_calculated AND all_required_pairs_valid AND all_current_resources_valid AND combat_readiness_valid`

Failure reasons are canonical reason codes sorted by registry order for deterministic tests/debug output.

Minimum failure reason categories:

- `invalid_stat_config`
- `invalid_pair_bound_config`
- `invalid_resource_bounds`
- `missing_identity_field`
- `missing_required_stat`
- `unknown_stat_id`
- `inactive_reserved_stat_used_as_active`
- `unsupported_modifier_operation`
- `duplicate_modifier_source`
- `scalar_input_out_of_bounds`
- `invalid_stat_pair`
- `current_resource_out_of_bounds`
- `combat_readiness_invalid`
- `config_version_mismatch`
- `source_authentic_evidence_missing`

Variables:

| Variable | Type | Range | Description |
|---|---|---|---|
| `config_valid` | bool | true/false | Stat registry, bounds, resource, pair, modifier, and evidence config are valid. |
| `identity_valid` | bool | true/false | Actor-type identity fields such as `class_id` or `monster_template_id` are valid. |
| `all_required_inputs_present` | bool | true/false | Actor-type required base/current/template inputs exist. |
| `source_input_valid` | bool | true/false | Sources, modifiers, finite numeric values, and source labels are valid. |
| `all_active_stats_calculated` | bool | true/false | Required active effective stats succeeded. |
| `all_required_pairs_valid` | bool | true/false | Required active pairs passed pair validation. |
| `all_current_resources_valid` | bool | true/false | Current resources are in legal range or approved correction path. |
| `combat_readiness_valid` | bool | true/false | Combat-ready actors satisfy HP and actor-type readiness invariants. |

**Output Range:** `true` if every gate is true; `false` with deterministic `snapshot_failure_reasons` otherwise.

**Example:** A player fixture with valid config and identity but `physical_attack_min = 20`, `physical_attack_max = 10`, and no other failures produces `snapshot_valid = false` with `{invalid_stat_pair}`. If the same fixture also lacks `class_id`, failure reasons are sorted by registry order, e.g. `{missing_identity_field, invalid_stat_pair}`.

### 5. `attribute_delta`

Preconditions:

`delta_comparable(stat_id) = snapshot_before.valid AND snapshot_after.valid AND snapshot_before.actor_id == snapshot_after.actor_id AND snapshot_before.schema_version == snapshot_after.schema_version AND snapshot_before.config_version == snapshot_after.config_version AND stat_id present in both snapshots`

For authoritative progression deltas, `snapshot_after.version > snapshot_before.version` is required. Preview deltas may compare current snapshot to preview result and must be labeled `preview_non_authoritative`; preview comparability still requires matching schema/config or an explicit migration policy.

Formula:

`attribute_delta(stat_id) = snapshot_after_value(stat_id) - snapshot_before_value(stat_id)`

If preconditions fail, return structured failure such as `delta_invalid_snapshot`, `delta_actor_mismatch`, `delta_incomparable_schema`, `delta_incomparable_config`, `delta_missing_stat`, or `delta_stale_or_reversed_version`.

Variables:

| Variable | Type | Range / Constraint | Description |
|---|---|---|---|
| `stat_id` | stat ID | comparable active/visible/debug stat ID | Stat being compared. |
| `snapshot_before_value` | int | valid value for matching schema/config | Before value. |
| `snapshot_after_value` | int | valid value for matching schema/config | After value. |
| `snapshot_before.version` / `snapshot_after.version` | int | actor-local versions | Used for authoritative ordering. |
| `snapshot_before.schema_version` / `snapshot_after.schema_version` | version ID | equal unless migration policy exists | Schema comparability. |
| `snapshot_before.config_version` / `snapshot_after.config_version` | version ID | equal unless migration policy exists | Config comparability. |

**Output Range:** signed integer difference, or structured failure.

**Example:** before `physical_attack_min = 8`, after `physical_attack_min = 10`, same actor/schema/config and after version newer: `attribute_delta(physical_attack_min) = 10 - 8 = +2`. If config versions differ, result is `structured_failure(delta_incomparable_config, physical_attack_min)`.

### 6. `snapshot_delta`

`snapshot_delta = {stat_id: attribute_delta(stat_id) for each stat_id in comparable_stat_ids where attribute_delta(stat_id) != 0}`

Optional pair view:

`snapshot_pair_delta = {pair_id: (attribute_delta(pair_min_stat_id), attribute_delta(pair_max_stat_id)) for each active comparable pair_id where pair delta != (0, 0)}`

Rules:

- `comparable_stat_ids` is a requested set, not an implicit intersection, unless the caller explicitly requests `visible_player_summary` mode.
- In explicit requested-set mode, a missing active requested stat returns structured failure.
- In `visible_player_summary` mode, inactive/reserved/debug-only stats are filtered out before delta calculation.
- In `debug_full` mode, hidden/debug stats may appear if schema/config comparable.
- scalar and pair views must not double-count in player-facing summaries;
- UI-facing deltas use display metadata to group min/max pairs as one row;
- invalid or incomparable snapshots produce structured failure, not partial silent deltas.

Variables:

| Variable | Type | Range / Constraint | Description |
|---|---|---|---|
| `comparable_stat_ids` | ordered set | explicit requested, visible summary, or debug full mode | Stats considered for scalar deltas. |
| `comparable_pair_ids` | ordered set | active comparable pair IDs | Pairs considered for grouped deltas. |
| `attribute_delta(stat_id)` | signed int or failure | child formula result | Scalar delta. |
| `attribute_pair_delta(pair_id)` | tuple or failure | child deltas for pair min/max | Pair delta. |

**Output Range:** sparse ordered map of non-zero scalar deltas, sparse ordered map of non-zero pair deltas, empty maps when comparable snapshots are identical, or structured failure.

**Example:** before attack `8–14`, after `10–18`, visible summary mode groups the pair as `physical_attack: (+2, +4)` and does not separately show duplicate scalar rows. If `accuracy` is inactive/reserved, visible summary filters it out even if debug mode could show it.

### 7. `combat_power` — MVP Provisional Display Formula

Preconditions:

`combat_power_config_valid = attack_weight >= 0 AND defense_weight >= 0 AND health_weight >= 0 AND each enabled secondary_weight >= 0 AND at least one active player-facing weight > 0`

`combat_power_inputs_valid = effective_stat_pair(physical_attack) succeeds AND effective_stat_pair(physical_defense) succeeds AND effective_stat(health_max) succeeds AND every enabled secondary stat succeeds`

If either precondition fails, return structured failure such as `invalid_combat_power_config`, propagated pair/stat failure, or `combat_power_no_active_weight`.

Formula:

`average_pair(pair_id) = (pair_min_final + pair_max_final) / 2.0`

`secondary_contribution = Σ secondary_weight_j * effective_stat(secondary_stat_id_j) for each enabled, combat-active, player-facing secondary stat j`

`combat_power_raw = attack_weight * average_pair(physical_attack) + defense_weight * average_pair(physical_defense) + health_weight * health_max + secondary_contribution`

`combat_power = round_to_int(combat_power_raw)`

Variables:

| Variable | Type | Phase 1 Range / Constraint | Description |
|---|---|---|---|
| `attack_weight` | float/int config | `> 0` for Phase 1 attack-primary fixtures; technical range `0–100` | Attack contribution weight. |
| `defense_weight` | float/int config | `0–100` | Defense contribution weight. |
| `health_weight` | float/int config | `0–100` | HP max contribution weight. |
| `secondary_weight_j` | float/int config | `0–100`; inactive stats forced `0` | Optional enabled secondary contribution. |
| `physical_attack` | pair | active valid pair | Offensive main stat. |
| `physical_defense` | pair | active valid pair | Survivability stat. |
| `health_max` | int | valid effective stat, `> 0` for combat-ready actors | Survivability/resource max. |
| `combat_power_raw` | float | non-negative if inputs valid | Intermediate display score. |
| `combat_power` | int | `0–COMBAT_POWER_DISPLAY_MAX` if a display cap is configured; otherwise non-negative rounded int | Player-facing estimate. |

Inactive/reserved stats contribute exactly `0`. Combat power is display/comparison aid only and does not define damage, DPS, TTK, or OpenMir2-authentic power. UI should label it as estimated/provisional if shown.

**Output Range:** non-negative rounded integer display score, or structured failure. If all active player-facing weights are `0`, return `structured_failure(combat_power_no_active_weight)` rather than showing every item as equal.

**Example — attack upgrade:** weights `attack = 10`, `defense = 2`, `health = 0.1`, no secondary. Before: attack `8–14` average `11`, defense `2–4` average `3`, HP `40`; `combat_power_raw = 10*11 + 2*3 + 0.1*40 = 120`, so `combat_power = 120`. After weapon upgrade attack `10–18` average `14`; defense/HP unchanged; `combat_power_raw = 10*14 + 2*3 + 0.1*40 = 150`, so `combat_power_delta = +30`.

**Example — invalid config:** all weights are `0`; result is `structured_failure(combat_power_no_active_weight)`, and UI must rely on visible stat deltas or mark comparison power unavailable rather than displaying `0` as meaningful.

This formula must be revisited after `伤害计算系统` and equipment comparison ADRs are created.

## Edge Cases

### 1. Missing Required Data

Missing identity, actor-type required stats, config bounds, pair definitions, source status, or formula configuration returns structured failure. The system must not infer default `0`, `1`, empty modifiers, placeholder class, or fake monster fields unless an explicit fixture/config rule defines that value.

### 2. Invalid or Unsupported Stat IDs

Unknown stat IDs fail validation. Known but inactive reserved stats may be present only as schema/debug facts and must not affect combat, HUD, equipment comparison, save/load, or growth feedback until activated.

### 3. Invalid Clamp Bounds

If scalar, resource, or pair bounds are inverted, the system must fail before calling clamp. Clamp behavior with invalid bounds is not a gameplay rule.

### 4. Out-of-Range Values

Allowed correction depends on source and reason. Equipment-driven derived stat overflow may clamp with evidence if stat config permits. Missing config, inverted bounds, non-finite values, unsupported operations, or source-authentic claim without evidence fail. Save/load impossible values require migration/recovery policy before becoming valid.

### 5. Invalid Min/Max Pairs

If active `physical_attack`, `physical_defense`, or enabled `magic_defense` min exceeds max after valid calculation, the rebuild fails for combat-consumable snapshot. Reserved inactive pairs are debug/schema validated but do not block ordinary monster snapshots unless activated.

### 6. Max Resource Increase / Reduction

Reduction clamps current resource to new max and emits `max_resource_clamp`, not damage. Increase uses configured policy. Default `keep_current_with_feedback` must emit a positive max-resource delta so the UI can avoid making the HP/MP bar change feel like a penalty.

### 7. Equipment Switching Failure

If equip/unequip/replace rebuild fails, previous valid snapshot may be retained only as stale display/debug fallback. Combat must not treat failed rebuild as current truth. UI must not celebrate the change.

### 8. Multiple Modifiers and Duplicate Sources

Multiple active flat modifiers targeting the same enabled stat are summed in one aggregation pass. Duplicate modifier source key is blocking unless stacking group explicitly allows it. Modifier order must not affect output or debug ordering.

### 9. Save/Load Rebuild Mismatch

If loaded payload references missing equipment, missing config, incompatible version, unsupported modifier schema, or impossible current resource data, the rebuild result is invalid until migration/recovery. Persisted snapshots are debug comparison only.

### 10. Combat Snapshot Staleness

Combat actions must bind snapshot versions or immutable lightweight snapshot references. Attribute system guarantees versioned snapshots; combat GDD owns whether a later version change causes captured-use, revalidate, or cancel. Old snapshots handed to combat must not be mutated.

### 11. Layer Pollution

Invalid writes include equipment writing base stats, HUD mutating current resources, combat mutating derived stats, save/load using derived stats as authoritative state, or consumers mutating snapshots. Debug tools may request dev-only mutations only through an approved debug boundary that records source/reason and rebuilds normally.

### 12. Pending Initialization

During creation, load, map transfer, or monster spawn, incomplete input produces unavailable/pending state. UI may show loading/unknown; combat, AI, equipment preview, save confirmation, and growth celebration must wait for valid snapshot or explicit fallback.

### 13. Player vs Monster Identity

Player snapshots require `class_id`. Monster snapshots require `monster_template_id` or approved monster type ID. Validators must not reject monsters solely because they lack player class ID.

### 14. Debug Evidence Overhead

Debug traceability is required, but heavy source breakdown and formatted strings must be generated on demand, in invalid result payloads, or in debug/test mode. Runtime snapshots must not maintain unbounded history.

## Dependencies

### Upstream Dependencies

| Dependency | Needed From It | Gate |
|---|---|---|
| `OpenMir2 行为映射 Spike` | Source evidence for authentic stat names, actor fields, equipment hooks, resource behavior, and combat stat usage. | E3/E4 evidence required before any value/formula/name is labeled OpenMir2-authentic. |
| MVP Gameplay Config / Fixture Contract | Stat registry, bounds, class/template fixtures, monster fixtures, equipment modifier fixtures, source status labels. | Must be owned before implementation. If no standalone system exists, Character Attributes implementation owns Phase 1 fixtures locally. |
| `物品定义系统` / `装备系统` | Resolved equipped item modifiers and stat target IDs. | Required before full equip loop; preview can use test fixtures before these GDDs exist. |
| ADRs / technical designs | Representation, API, event, transaction, save/load, config, tests. | Implementation-blocking. |

### Downstream Dependencies

| System | Consumes | Boundary / Staging |
|---|---|---|
| `伤害计算系统` | attacker/defender snapshot versions, active combat stats, optional combat_power for display only. | Damage formula owns final damage and TTK verification. |
| `生命 / 死亡 / 复活规则` | HP current/max, resource mutation results, zero-HP state. | Death semantics and death events not owned here. |
| `装备系统` | modifier validation, preview/rebuild results, deltas. | Equipment owns equip legality and item stat definitions. |
| `极简 HUD 系统` | local player resource snapshot, compact resource events, stale/invalid player-safe status. | HUD owns layout and rendering. |
| `背包 / 装备 UI 系统` | preview/delta display metadata, main stat, combat power delta, localization keys. | UI owns presentation and interaction. |
| `成长反馈系统` | growth_salience, visible delta summary, reason tags. | Feedback owns VFX/audio/text timing. |
| `存档系统` | persistent input contract and rebuild result. | Save owns file schema and migration. |
| `怪物生成系统` | monster template required inputs and rebuild result. | Spawn owns where/when monsters appear. |
| `数据调试 / 开发工具` | debug trace and failure reason payloads. | Full tools may be Future; lightweight evidence is Phase 1 required. |

### Dependency Graph Notes

- No standalone data/config pipeline GDD currently exists. Until one is created, Phase 1 attribute fixtures and stat registry are owned by Character Attributes implementation stories, with a later migration path to shared data/config tooling.
- Downstream integration ACs are binding for those systems when their GDDs/stories start; they do not require those systems to exist before this GDD can be approved.

## Tuning Knobs

All values are data/config owned. Ranges below are **technical safe ranges for MVP fixtures**, not OpenMir2-authentic balance.

| Tuning Knob | Safe Phase 1 Range / Values | Gameplay Impact | Phase 1 Rule |
|---|---|---|---|
| `mvp_attribute_stat_ids` | explicit registry rows only | Defines active vs reserved stats | Must match this GDD or approved revision. |
| `stat_min_bound(stat_id)` | int; must be `<= max`; typical tests `0–9999` except special debug cases | Prevents impossible effective stats | Config-owned; inverted bounds fail. |
| `stat_max_bound(stat_id)` | int; must be `>= min`; typical tests `1–9999` for combat-ready HP max | Caps effective stats | Config-owned; not authentic. |
| `resource_min_bound(health)` | default `0` | Defines HP floor and death handoff | Negative invalid in Phase 1. |
| `resource_min_bound(mana)` | default `0` | Defines MP floor | Hidden unless skills/resource UI enabled. |
| `health_max` fixture values | `1–9999` for combat-ready actors | Survivability, HUD denominator | `0` invalid for live combat actors. |
| `mana_max` fixture values | `0–9999` | Reserved resource capacity | May be hidden if inactive. |
| `physical_attack_min/max` fixtures | `0–9999`, pair min <= max | Damage inputs / main comparison | Provisional; pair invariant required. |
| `physical_defense_min/max` fixtures | `0–9999`, pair min <= max | Survivability / damage mitigation input | Provisional. |
| `class_base_stat_table` | explicit fixture rows | Player progression baseline | Must include source_status. |
| `monster_base_stat_table` | explicit fixture rows | Monster combat readiness | Actor-type required fields allowed. |
| `equipment_flat_modifier_table` | int modifiers within technical safe range | Equipment growth feel | `add_flat` only in Phase 1. |
| `unsupported_modifier_policy` | fixed default table in this GDD | Invalid item/source handling | Not left undecided. |
| `snapshot_failure_reason_registry` | canonical enum/list | QA/debug determinism | Sorted by registry order. |
| `snapshot_delta_comparable_stats` | visible subset + debug subset | Equipment compare/growth feedback | Inactive stats hidden from player summary. |
| `combat_power_weights` | provisional non-negative weights; inactive stat weight `0` | Quick value judgment | Display-only; not damage authority. |
| `resource_max_increase_policy` | `keep_current_with_feedback` default; alternatives `preserve_ratio`, `fill_by_delta` | Avoids upgrade-feels-worse HP bar | Must be explicit. |
| `event_coalescing_policy` | one event per actor transaction/update point | Prevents event storms | Required for implementation. |
| `debug_trace_mode` | off/lazy/default; on for debug/tests | Traceability vs performance | No unbounded runtime history. |
| `provisional_value_labeling` | `mvp_provisional`, `openmir2_evidence_pending`, `openmir2_verified` | Prevents false authenticity | Verified requires evidence ID. |

## Acceptance Criteria

### AC-01 — Immutable Authoritative Snapshot

- **Scope:** Character Attributes System
- **Story Type:** Logic / API
- **Gate:** BLOCKING — automated unit test required
- Given valid MVP inputs, the system publishes an `AttributeSnapshot` or equivalent read-only view with actor ID, actor type, version, schema/config version, effective stats, current resources, and source status.
- Attempts by a consumer to mutate exposed snapshot data are impossible, rejected, or proven not to change authoritative state.
- Rebuilding from identical inputs produces identical effective values.

### AC-02 — Phase 1 Stat Registry Coverage

- **Scope:** Data / Schema
- **Story Type:** Logic / Schema
- **Gate:** BLOCKING — schema/unit validation required
- The stat registry distinguishes required, active, visible, debug-only, reserved, unsupported, modifier-targetable, and actor-type-specific fields.
- Player snapshots support required player fields including `class_id`; monster snapshots support required monster fields including `monster_template_id` or equivalent.
- Inactive/reserved stats cannot silently affect combat, HUD, save/load, equipment preview, or growth feedback.

### AC-03 — Data-Driven Provisional Fixtures

- **Scope:** Data / Config
- **Story Type:** Logic / Data Schema
- **Gate:** BLOCKING — automated schema/unit validation required
- Base stat fixtures, stat bounds, resource bounds, monster template fixtures, combat power weights, and equipment modifier fixtures load from explicit config/fixture sources or injected test data.
- Gameplay implementation does not hardcode provisional or OpenMir2-authentic stat values. Boundary tests may inline values only when the value is the test subject.
- Every value/formula label has `source_status`; `openmir2_verified` requires evidence ID/source reference.

### AC-04 — Effective Stat Formula

- **Scope:** Formula Validation
- **Story Type:** Logic
- **Gate:** BLOCKING — automated unit test required
- Tests cover no modifier, one active flat modifier, multiple active modifiers, inactive modifier exclusion, min clamp, max clamp, invalid stat ID, invalid clamp bounds, unsupported operation rejection, and modifier order independence.
- Functional tests verify aggregation is independent of modifier order, inactive modifiers are excluded, and duplicate modifier sources are not double-counted.
- Implementation evidence or instrumentation must confirm each active modifier row is processed once per structural rebuild, with aggregation behavior equivalent to O(M + S).

### AC-05 — Min/Max Pair Validation

- **Scope:** Formula Validation
- **Story Type:** Logic
- **Gate:** BLOCKING — automated unit test required
- Tests cover valid, equal, invalid, missing, and config-incoherent pairs for active physical attack and physical defense; magic defense only if enabled.
- Invalid active pairs return structured failure and do not publish combat-consumable snapshots.

### AC-06 — Current Resource Mutation and Clamp

- **Scope:** Resource Boundary
- **Story Type:** Logic
- **Gate:** BLOCKING — automated unit test required
- Resource-only damage/heal/spend/restore mutates current HP/MP through the resource mutation path without full structural rebuild.
- Current resource clamps into valid range; max reduction emits resource clamp reason distinct from damage/heal.
- Max increase uses configured policy and emits enough data for UI to avoid presenting growth as damage/downgrade.
- Combat-ready player and ordinary live monster snapshots reject `health_max <= 0`.

### AC-07A — Snapshot Validity Formula

- **Scope:** Snapshot Validation
- **Story Type:** Logic
- **Gate:** BLOCKING — automated unit test required
- `snapshot_valid` rejects invalid config, missing identity, missing required actor-type stats, unknown active stat IDs, invalid pairs, invalid resource bounds, invalid source labels, and combat readiness failures.
- Multiple simultaneous failure reasons are returned in deterministic registry order.

### AC-07B — Invalid Snapshot Consumer Blocking

- **Scope:** Downstream Integration Contract
- **Story Type:** Integration
- **Gate:** BLOCKING before each consuming system story completes
- Combat, equipment preview, HUD, save confirmation, monster spawn/behavior, and growth feedback must each verify they reject, block, or display invalid/stale snapshots as player-safe unknown/debug state rather than normal gameplay data.
- Required downstream evidence: combat invalid-attacker/defender test; equipment preview invalid-modifier fixture; HUD stale/invalid display walkthrough or screenshot; save confirmation load-failure walkthrough or integration test; monster spawn invalid-template fixture; growth feedback fixture proving invalid/stale changes are not celebrated.

### AC-08 — Attribute and Snapshot Delta

- **Scope:** Change Detection
- **Story Type:** Logic
- **Gate:** BLOCKING — automated unit test required
- Delta generation validates actor ID, schema/config version, snapshot validity, stat presence, and version direction unless preview-labeled.
- Deltas include before value, after value, signed delta, stat/pair ID, reason, and before/after version or preview marker.
- Invalid or incomparable inputs return structured delta failure.

### AC-09 — Equipment Modifier Boundary and Atomic Rebuild

- **Scope:** Equipment Integration Contract
- **Story Type:** Integration / Logic with fixtures
- **Gate:** BLOCKING before equipment story completion; fixture test required for attribute implementation
- For each accepted equipment transaction, removed and added modifiers are applied as one staged source update and publish exactly one valid rebuild event.
- No intermediate snapshot exposes both old and new item modifiers.
- Failed rebuild keeps previous valid snapshot only as stale/display fallback and exposes structured failure evidence.

### AC-10 — Combat Read-Only Consumption

- **Scope:** Combat Boundary
- **Story Type:** Integration
- **Gate:** BLOCKING before combat story completion
- Combat captures attacker/defender snapshot versions or immutable lightweight snapshot references.
- Combat does not aggregate equipment modifiers or calculate effective stats.
- Combat routes HP/MP changes through approved resource mutation boundary.
- Invalid attacker/defender snapshot before action start returns explicit blocked/revalidate/cancel result as defined by combat GDD. Until that GDD is approved, the attribute-side fixture must expose invalid/stale status and combat owns final blocked/revalidate/cancel vocabulary.

### AC-11 — HUD / UI Read-Only Consumption

- **Scope:** HUD / UI Boundary
- **Story Type:** UI / Integration
- **Gate:** ADVISORY for attribute completion; BLOCKING before HUD/equipment UI story completion
- HUD binds to the local player actor and ignores unrelated monster events.
- HUD displays HP from valid snapshot/resource event and does not compute max independently.
- UI receives localization/display metadata, visible delta summaries, stale/invalid player-safe status, and does not show inactive/reserved stats as normal upgrade values.
- Delta presentation is distinguishable without relying only on color; UI implementation evidence must include screenshot, interaction test, or manual walkthrough showing at least one positive and one negative delta distinguishable through symbols/text/layout.

### AC-12 — Save/Load Rebuild Contract

- **Scope:** Persistence Boundary
- **Story Type:** Integration
- **Gate:** BLOCKING before persistence story completion
- Save payload stores authoritative inputs and versions, not derived snapshot as truth.
- Loading unchanged inputs rebuilds equivalent effective values: same actor identity, effective stat values, current resource values, source_status labels, schema/config versions, and validity state; snapshot version may differ if load rebuild publishes a new version.
- Missing config/equipment or incompatible version produces structured failure or approved migration.
- Failed rebuild is not consumed by combat/HUD as normal data.

### AC-13 — Monster Template Support

- **Scope:** Monster Spawn Boundary
- **Story Type:** Integration / Fixture
- **Gate:** BLOCKING before monster combat story completion; fixture test required for attribute implementation
- Ordinary Phase 1 monster fixtures provide actor-type required identity, level/template, health, physical attack, and physical defense inputs sufficient for valid combat snapshots.
- Monster validators do not require player-only `class_id` unless a universal schema explicitly maps it.
- Invalid monster templates do not create combat-ready monster actors.

### AC-14 — Debug Traceability Payload

- **Scope:** QA / Debuggability
- **Story Type:** Tooling / Evidence
- **Gate:** BLOCKING before implementation handoff
- Invalid rebuild/debug trace payload includes actor/template ID, attempted source version, current valid snapshot version if any, failed formula/reason ID, failed stat/pair/resource ID, input values used by failed formula, configured bounds, active modifier source IDs, invalid modifier reasons, source_status, and config/fixture ID.
- A test verifies the payload can reproduce at least one failing formula case without HUD, scene tree, or Autoload.
- Full interactive debug tooling remains Future / Polish; lightweight structured evidence is Phase 1 required.

### AC-15 — Snapshot Versioning and Event Contract

- **Scope:** Event / Versioning
- **Story Type:** Logic / API
- **Gate:** BLOCKING — automated unit test required
- Each published authoritative snapshot has an actor-local monotonic version.
- Accepted structural changes emit one `AttributeRebuildEvent` after successful rebuild.
- Resource-only changes emit compact `ResourceChangedEvent` without full structural rebuild.
- Failed rebuilds do not replace the current valid snapshot version and emit invalidation/failure result.
- Event payloads do not require full old+new snapshots in normal runtime path.

### AC-16 — Player-Facing Growth Handoff

- **Scope:** Growth Feedback / Equipment Comparison Contract
- **Story Type:** UX / Integration Contract
- **Gate:** BLOCKING before equipment UI / growth feedback story completion; fixture evidence required for attribute implementation
- Given a positive equipment or level change, the attribute output includes growth reason, salience, primary comparison stat, visible delta summary, and provisional combat power delta if enabled.
- Inactive/reserved stats are excluded from normal player-facing growth summary.
- Invalid/stale rebuilds cannot produce celebratory growth feedback.
- A fixture demonstrates an attack-up equipment change producing a visible summary suitable for `攻击 x–y → a–b` or equivalent main-stat display.

### AC-17 — Blocking Test Evidence Rollup

- **Scope:** Test Evidence
- **Story Type:** Logic
- **Gate:** BLOCKING
- Before Character Attributes implementation stories are marked Complete, QA evidence shows automated tests pass for AC-01, AC-02, AC-03, AC-04, AC-05, AC-06, AC-07A, AC-08, AC-14, AC-15, and the fixture portions of AC-09, AC-13, and AC-16.
- Evidence records test file paths, command run, pass/fail result, fixture/config source, and source_status labels.
- Expected unit test area: `tests/unit/character_attributes/`.

## Appendix — Open Questions

These questions do not block GDD approval if the revised contracts above are accepted, but they block implementation or downstream GDD lock-in where noted.

### OpenMir2 Evidence

1. What is the verified OpenMir2 field list for player and ordinary monster attributes?
2. Which fields map directly to this GDD's MVP stat IDs, and which are project-local aliases?
3. What are source-authentic initial values, class differences, growth tables, and equipment hooks?
4. What is OpenMir2 behavior when max HP/MP increases/decreases?
5. Are zero or negative resource/stat values possible internally in OpenMir2?
6. What is the authentic order for base stats, level growth, equipment, buffs, and other modifiers?

### MVP Scope

1. Does Phase 1 use one fixed player class/template?
2. Do accuracy/evasion participate in Phase 1 combat or remain hidden required fields?
3. Is MP hidden entirely in player HUD until skills exist?
4. Does Phase 1 equipment preview use pure preview query, equipment-owned preview, or post-equip-only delta?
5. Which system owns final combat power weighting after equipment/combat GDDs exist?

### ADR / Technical Design

1. Runtime stat IDs: enum/int, `StringName`, Resource ID, or hybrid?
2. Snapshot representation: immutable `RefCounted`, Resource, typed class, or hybrid?
3. Event dispatch: returned domain events, typed signals on scene-tree-independent object, injected event sink, or adapter layer?
4. Source transaction model: commit/rollback/stale previous behavior?
5. Fixture/config format: `.tres`, JSON, YAML, GDScript factories, or hybrid?
6. Save/load serialization boundary and Godot Resource duplication policy?
