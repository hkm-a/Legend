# 掉落表系统

> **Status**: Approved for Implementation Planning — implementation blocked pending real Godot test runner and story/test slicing
> **Author**: hkm + Claude Code Game Studios
> **Last Updated**: 2026-06-05
> **Implements Pillar**: Primary — 爆装有戏; Supports — 稳刷不断流 / 每次登录都带走成长
> **Quick reference** — Layer: `Foundation / Economy` · Priority: `MVP` · Key deps: `物品定义系统`, `OpenMir2 行为映射 Spike`, ADR-0016, ADR-0017, ADR-0021

## Overview

掉落表系统是 Phase 1 离线 30 秒刷怪爆装循环的奖励概率与掉落候选层。它把 reward source（例如普通怪物模板或死亡奖励上下文）映射到经过验证的 drop table、roll group、weighted row、no-drop policy 和 quantity policy，并用可复现的 deterministic roll 输出 item grant candidate。该系统只拥有“什么来源可以 roll 哪些物品引用、权重是多少、数量意图是多少、结果如何复现”；它不拥有物品定义真相、地面掉落生命周期、地图放置、拾取距离、背包接收、装备合法性、UI/VFX/audio 反馈或最终物品实例身份。Phase 1 可以使用 `mvp_provisional` 掉率和少量项目本地物品引用来验证循环，但任何声称 OpenMir2-authentic 的掉落行为、物品表或概率都必须引用 ADR-0016 governance 下的 Accepted evidence 或 Accepted contract readiness；raw E3/E4 evidence alone is not enough until governance accepts it.

## Player Fantasy

玩家感受到的幻想是：“这个怪可能会爆出值得停下来看的东西。”掉落表系统不直接绘制光柱、不播放音效、也不把物品放进背包，但它决定击杀之后是否出现奖励候选、奖励候选属于材料还是装备、装备是否有机会形成明显成长。Phase 1 的目标不是还原完整 MMO 掉落经济，而是在短时间技术切片里制造可信的 variable-ratio reward 节奏：多数击杀保持稳刷速度，部分击杀产出材料或普通装备，少数击杀产出更醒目的升级/展示物，让玩家愿意点击拾取、打开背包、比较并继续刷。

玩家不应感觉掉落是调试随机数或静态演示脚本。即使 Phase 1 只有一个普通怪掉落表，表中的 no-drop、材料、普通装备、明确升级装备和可选 rare/showcase role 也应让玩家相信：继续打下一只怪有意义，爆装链路已经有可扩展骨架。

## Detailed Rules

### 1. Ownership Boundary

掉落表系统 owns：

- `drop_table_id`、`drop_table_version` 和 reward source → drop table 映射。
- Drop pool / roll group、row、weight、no-drop weight、roll count、selection mode、quantity policy、eligibility/source labels。
- 掉落表 validation 和结构化失败原因。
- 每个 item row 是否通过物品定义系统的 spawn eligibility 检查。
- Deterministic roll request/result DTO 和 QA replay provenance。
- MVP provisional 掉落表内容、权重和 expected acquisition 文档。

掉落表系统 does not own：

- Item Definition template truth：名称、图标、品质、类型、堆叠、装备字段、modifier payload、source/evidence label truth。
- Ground drop：`ground_drop_id`、生命周期、地图放置、despawn、claim、pickup state、MapSpaceState item occupancy。
- Inventory：容量、格子、stack merge/split、item instance/stack ID、最终存储、存档。
- Equipment：穿戴合法性、装备槽、替换事务、装备对属性系统的最终 modifier source handoff。
- Combat/death：伤害、HP、死亡触发、kill credit、怪物 AI 或刷怪。
- UI/presentation/audio：掉落标签、光柱、音效、拾取 toast、品质颜色、tooltip 布局。

Forbidden patterns：

- Drop row 复制 item display name、quality truth、icon、stack policy、equipment modifier 或 combat power。
- 掉落表在地面放置失败后自行 reroll、silent delete 或改写地图位置。
- 掉落表根据当前背包容量动态改变掉率。
- 使用 Godot scene tree order、signal order、wall-clock time、frame delta、global RNG 或 Dictionary iteration order 作为 roll authority。
- 未经 ADR-0016 evidence readiness，把 `mvp_provisional` 掉率标为 `openmir2_verified`。

### 2. Data Model

#### DropTableDefinition

Minimum fields：

| Field | Required | Meaning |
|---|---:|---|
| `drop_table_id` | Yes | Stable table key used by reward source context. |
| `drop_table_version` | Yes | Version used for validation, replay and migration/debug. |
| `source_status` | Yes | `mvp_provisional`, `project_local`, `openmir2_evidence_pending`, `openmir2_verified`, or `debug_only`. `project_local` means intentionally non-OpenMir2-authentic and cannot be upgraded to `openmir2_verified` without a separate accepted evidence/contract reference. |
| `evidence_ref` | Conditional | Required for `openmir2_verified`. |
| `roll_groups` | Yes | Ordered list of roll groups. |
| `profile_tags` | Optional | Normal/debug/test profile eligibility. |

#### DropRollGroup

Minimum fields：

| Field | Required | Meaning |
|---|---:|---|
| `group_id` | Yes | Stable group key unique within table. |
| `group_order` | Yes | Explicit deterministic order. |
| `selection_mode` | Yes | Phase 1 default: `weighted_single`. |
| `roll_count` | Yes | Phase 1 default: `1`. |
| `no_drop_weight` | Yes | Explicit no-reward candidate weight; may be `0`. |
| `rows` | Yes | Ordered list of item rows. |

#### DropRow

Minimum fields：

| Field | Required | Meaning |
|---|---:|---|
| `row_id` | Yes | Stable row key unique within group. |
| `row_order` | Yes | Explicit deterministic row order. |
| `item_id` | Yes for item rows | Stable Item Definition ID. |
| `definition_version_policy` | Recommended | Phase 1 allowed values: `exact_version` or `current_compatible`; roll results must record the resolved policy/result. |
| `weight` | Yes | Integer non-negative selection weight. |
| `quantity_min` | Yes | Minimum quantity if selected. |
| `quantity_max` | Yes | Maximum quantity if selected. |
| `source_status` | Yes | Evidence/provisional label. |
| `eligibility_tags` | Optional | Profile/source restrictions. |

### 3. MVP Provisional Table

Phase 1 starts with one normal-monster table:

```text
drop_table_id = mvp_basic_monster_drop
drop_table_version = 1
source_status = mvp_provisional
selection_mode = weighted_single
roll_count = 1
```

Recommended `main_loot` group:

| Entry | Provisional Role / Example ID | Weight | Quantity | Purpose |
|---|---|---:|---:|---|
| no-drop | `NO_DROP` | 6000 | — | Preserve steady kill flow and avoid ground clutter. |
| material | `mvp_small_material_shard` | 2500 | 1–3 | Frequent reward reinforcement. |
| common equipment | `mvp_bronze_sword` | 1200 | 1 | Comparison/equip loop exercise. |
| upgrade equipment | `mvp_iron_sword` | 250 | 1 | Clear positive growth test. |
| rare/showcase | `mvp_showcase_blade` or approved rare fixture | 50 | 1 | Loot feedback salience test; optional until item set approved. |

Total group weight: `10000`.

All concrete IDs are `mvp_provisional` cross-system entries registered in `design/registry/entities.yaml`; they are still not final Item Definition catalog rows until Item Definition implementation data is authored and validated. The approved GDD authorizes these as MVP provisional fixture IDs only, and implementation must resolve each concrete `item_id` through Item Definition validation before normal gameplay.

Expected MVP acquisition outputs:

| Output | Rate | Expected Attempts | Notes |
|---|---:|---:|---|
| No drop | 60% | 1.67 kills per no-drop | Controls clutter and pacing. |
| Any drop | 40% | 2.5 kills | Core reinforcement cadence. |
| Material | 25% | 4 kills | Uniform quantity 1–3; average 2.0 per material drop. |
| Common equipment | 12% | 8.33 kills | Exercises comparison loop. |
| Upgrade equipment | 2.5% | 40 kills | Clear growth moment. |
| Rare/showcase | 0.5% | 200 kills | QA uses seed/debug fixture, not raw chance. |
| Any equipment | 15% | 6.67 kills | Common + upgrade + rare/showcase. |
| Upgrade-or-better | 3% | 33.33 kills | Upgrade + rare/showcase. |

Material faucet calculation:

```text
average_material_quantity = (1 + 3) / 2 = 2.0
expected_material_per_kill = 0.25 * 2.0 = 0.5 material units per kill
```

Economy health note: these rates are validation-biased for a technical slice, not long-term economy balance. Assuming 10 kills per 30-second smoke loop, the table produces about 4 total drops, 5 material units, 1.5 equipment items, 0.3 upgrade-or-better items, and 0.05 rare/showcase items per loop. Future economy passes must rebalance after inventory pressure, sell/salvage/crafting sinks, equipment variance, and OpenMir2 evidence contracts exist.

### 4. Validation Rules

A drop table is valid for normal gameplay only if:

1. `drop_table_id` exists and is unique.
2. `drop_table_version` exists and is supported.
3. Every group has unique `group_id`, explicit `group_order`, valid `selection_mode`, and `roll_count >= 1`.
4. Every group has `drop_group_total_weight > 0`.
5. Every row has unique `row_id`, explicit `row_order`, integer `weight >= 0`, valid source/evidence label, and required quantity fields.
6. Every authored item row, including zero-weight placeholder rows in normal gameplay tables, references a syntactically valid Item Definition row; every selectable/non-zero item row passes `spawn_eligible_reference` for the active profile. Missing/unapproved placeholder references are allowed only in debug/test/profile-gated tables.
7. Quantity policies cannot output `0`, negative, fractional, or item-illegal quantities.
8. Equipment rows use `quantity_min = quantity_max = 1` in Phase 1.
9. Stackable material rows must never exceed the referenced Item Definition `max_stack_size`.
10. Debug-only, deprecated, invalid, blocked-unconfirmed, or missing item definitions are rejected for normal tables.
11. `openmir2_verified` table or row labels fail unless accepted evidence references are present through ADR-0016 governance.
12. Authored order is deterministic; runtime roll order never depends on Dictionary iteration.

Invalid normal gameplay tables must fail validation as a whole. The system must not silently skip bad rows in normal mode because that changes probabilities without authoring awareness.

### 5. Runtime Roll Rules

Canonical Phase 1 roll flow：

1. Death/reward source provides `source_context_id`, `drop_table_id`, `rng_stream_id`, `roll_sequence`, and source tags.
2. Drop Table Runtime Catalog resolves the table by ID/version.
3. Validation uses prevalidated table data or returns structured failure.
4. Roll groups execute by `group_order`.
5. For each group attempt, build candidates in stable order: explicit no-drop candidate, then rows by `row_order`.
6. Compute `drop_group_total_weight`.
7. Draw deterministic integer `roll_value` from injected RNG in the required range.
8. Select the first candidate whose cumulative range includes `roll_value`. `rng_int(a, b)` returns an integer in the inclusive range `[a, b]`; if an implementation helper uses an exclusive upper bound, the adapter must preserve these GDD ranges.
9. If no-drop selected, return `NO_DROP` with provenance.
10. If item row selected, roll quantity deterministically, validate quantity against item stack/equipment rules, and return item grant candidate.
11. Output goes to GroundDropService / ADR-0017 handoff. Drop Table does not create ground records or inventory items.

### 6. Result Contract

Phase 1 returns one result DTO per group attempt. Future aggregate table results may contain an ordered array of group-attempt results, but invalid normal gameplay tables fail as a whole before any partial group result is emitted.

A roll result must include enough provenance to replay and debug:

| Field | Required | Meaning |
|---|---:|---|
| `status` | Yes | `ROLLED_DROP`, `NO_DROP`, or structured failure. |
| `drop_table_id` | Yes | Table rolled. |
| `drop_table_version` | Yes | Version rolled. |
| `source_context_id` | Yes | Death/reward source context. |
| `rng_stream_id` | Yes | Deterministic RNG stream/seed label. |
| `roll_sequence` | Yes | Stable roll sequence number. |
| `group_id` | Yes | Group rolled. |
| `row_id` | Conditional | Selected row for item result. |
| `item_id` | Conditional | Selected item reference. |
| `definition_version` | Conditional | Definition version policy/result. |
| `quantity` | Conditional | Quantity intent. |
| `primary_reason` | Conditional | Required for failure/no-drop status. |
| `secondary_reasons` | Optional | Additional validation/debug reasons. |

Recommended statuses:

| Status | Meaning |
|---|---|
| `ROLLED_DROP` | A row produced an item grant candidate. |
| `NO_DROP` | Roll completed and selected explicit no-drop. |
| `MISSING_TABLE` | Requested table ID is not loaded. |
| `INVALID_TABLE` | Table failed validation or has unsupported version. |
| `INVALID_WEIGHT_CONFIG` | Weight values are malformed, negative, overflow, or total to zero. |
| `INVALID_ITEM_REFERENCE` | Row points to missing/malformed item reference. |
| `INELIGIBLE_ITEM_REFERENCE` | Item resolves but is not spawn-eligible. |
| `INVALID_QUANTITY_POLICY` | Quantity policy can produce illegal quantity. |
| `OPENMIR2_EVIDENCE_NOT_READY` | Authenticity label lacks accepted evidence. |
| `RNG_STREAM_INVALID` | Deterministic RNG context is missing or unusable. |

## Formulas

### 1. `drop_group_total_weight`

```text
drop_group_total_weight = no_drop_weight + Σ(row_weight_i for all enabled rows in a prevalidated-valid group)
```

| Variable | Type | Expected Range | Description |
|---|---|---:|---|
| `no_drop_weight` | int | `0–1,000,000` | Explicit weight for no reward. |
| `row_weight_i` | int | `0–1,000,000` | Weight for item row `i`; `0` disables normal selection. |
| `drop_group_total_weight` | int | `1–1,000,000` recommended | Total selectable weight for one roll group. |

Output range: `drop_group_total_weight` is bounded to `1–1,000,000` in Phase 1 recommended content. It is not clamped; invalid zero, negative, or overflow configurations fail validation before runtime selection. If any enabled row is invalid in normal gameplay, `drop_table_valid = false` and this formula must not be used for runtime selection.

Validation：

- `drop_group_total_weight` must be greater than `0`.
- Negative weights are invalid.
- Overflow above configured safe max is invalid.

Example：

```text
no_drop_weight = 6000
row weights = 2500 + 1200 + 250 + 50

drop_group_total_weight = 6000 + 2500 + 1200 + 250 + 50 = 10000
```

### 2. `selected_drop_entry`

```text
roll_value = rng_int(0, drop_group_total_weight - 1)
selected_drop_entry = first candidate where cumulative_weight_candidate > roll_value
```

| Variable | Type | Expected Range | Description |
|---|---|---:|---|
| `roll_value` | int | `0–drop_group_total_weight - 1` | Deterministic RNG output from inclusive `rng_int(0, drop_group_total_weight - 1)`. |
| `drop_group_total_weight` | int | `1–1,000,000` recommended | Total selectable group weight. |
| `cumulative_weight_candidate` | int | `1–drop_group_total_weight` | Running total after each stable-order candidate. |
| `selected_drop_entry` | enum/ref | `NO_DROP` or `row_id` | Selected candidate. |

Output range: `selected_drop_entry` is bounded to exactly one candidate (`NO_DROP` or a `row_id`) for a valid prevalidated group. Invalid RNG context, invalid total weight, or invalid table state returns structured failure instead of selection.

Example candidate ranges：

| Candidate | Weight | Roll Values |
|---|---:|---|
| no-drop | 6000 | `0–5999` |
| material | 2500 | `6000–8499` |
| common equipment | 1200 | `8500–9699` |
| upgrade equipment | 250 | `9700–9949` |
| rare/showcase | 50 | `9950–9999` |

If `roll_value = 9725`, `selected_drop_entry = upgrade equipment`.

### 3. `drop_row_probability`

```text
drop_row_probability_i = row_weight_i / drop_group_total_weight
no_drop_probability = no_drop_weight / drop_group_total_weight
```

| Variable | Type | Expected Range | Description |
|---|---|---:|---|
| `row_weight_i` | int | `0–1,000,000` | Candidate row weight. |
| `no_drop_weight` | int | `0–1,000,000` | No-drop candidate weight. |
| `drop_group_total_weight` | int | `1–1,000,000` recommended | Group total weight. |
| `drop_row_probability_i` | float | `0.0–1.0` | Per-attempt probability for row `i`. |
| `no_drop_probability` | float | `0.0–1.0` | Per-attempt probability for no-drop candidate. |

Output range: `drop_row_probability_i` and `no_drop_probability` are bounded to `0.0–1.0` because weights are non-negative and denominator is the positive group total. Results are not clamped; invalid weights or zero total fail before probability calculation.

Example：

```text
upgrade_probability = 250 / 10000 = 0.025 = 2.5%
no_drop_probability = 6000 / 10000 = 0.60 = 60%
```

### 4. `expected_attempts_for_row`

```text
expected_attempts_for_row_i = 1 / drop_row_probability_i
```

| Variable | Type | Expected Range | Description |
|---|---|---:|---|
| `drop_row_probability_i` | float | `(0.0–1.0]` | Per-attempt probability. |
| `expected_attempts_for_row_i` | float | `1–∞` | Average attempts to receive this row once. |

Output range: `expected_attempts_for_row_i` is bounded from `1` to unbounded positive values when probability is greater than `0`. If probability is `0`, expected attempts are undefined and the row is not normally acquirable.

Example：

```text
rare_probability = 50 / 10000 = 0.005
expected_attempts_for_rare = 1 / 0.005 = 200 kills
```

### 5. `expected_attempts_for_tier`

For mutually exclusive rows within the same `weighted_single` group:

```text
tier_probability = Σ(drop_row_probability_i for rows in tier)
expected_attempts_for_tier = 1 / tier_probability
```

| Variable | Type | Expected Range | Description |
|---|---|---:|---|
| `drop_row_probability_i` | float | `0.0–1.0` | Per-row probability included in the tier. |
| `tier_probability` | float | `0.0–1.0` | Combined chance for tier rows. |
| `expected_attempts_for_tier` | float | `1–∞` | Average attempts to receive any row in tier. |

Example：

```text
equipment_tier_probability = 0.12 + 0.025 + 0.005 = 0.15
expected_attempts_for_equipment = 1 / 0.15 ≈ 6.67 kills
```

Output range: `expected_attempts_for_tier` is bounded from `1` to unbounded positive values when `tier_probability > 0`. If `tier_probability = 0`, expected attempts are undefined and the tier is not normally acquirable.

If future groups roll independently, tier probability must use complement logic:

```text
tier_probability = 1 - Π(1 - p_i for each independent group)
```

### 6. `drop_quantity_result`

For fixed quantity:

```text
drop_quantity_result = fixed_quantity
```

For uniform integer range:

```text
drop_quantity_result = quantity_min + rng_int(0, quantity_max - quantity_min)
```

Then validate:

```text
drop_quantity_valid_for_item = stack_quantity_valid(item_stack_policy, item_max_stack_size, drop_quantity_result)
```

| Variable | Type | Expected Range | Description |
|---|---|---:|---|
| `quantity_min` | int | `1–item_max_stack_size` | Minimum row quantity. |
| `quantity_max` | int | `quantity_min–item_max_stack_size` | Maximum row quantity. |
| `quantity_range` | int | `0–item_max_stack_size - 1` | `quantity_max - quantity_min`, used as inclusive RNG upper bound. |
| `fixed_quantity` | int | `1–item_max_stack_size` | Exact quantity if fixed policy. |
| `rng_offset` | int | `0–quantity_range` | Deterministic inclusive RNG offset. |
| `item_stack_policy` | enum | Item Definition stack policies | Referenced item stack behavior. |
| `item_max_stack_size` | int | `1–configured max` | Referenced item max stack size. |
| `drop_quantity_result` | int | `1–item_max_stack_size` | Quantity intent. |
| `stack_quantity_valid` | bool | `false/true` | Item Definition quantity validity formula. |

Output range: `drop_quantity_result` is bounded by the row quantity policy and the referenced Item Definition stack constraints. Invalid quantities fail validation rather than being clamped or split.

Examples：

```text
material quantity_min = 1
material quantity_max = 3
rng_int(0, 2) = 2

drop_quantity_result = 1 + 2 = 3
```

For equipment:

```text
fixed_quantity = 1
drop_quantity_result = 1
```

### 7. `drop_table_valid`

```text
drop_table_valid = table_identity_valid
  AND all_groups_valid
  AND all_rows_valid
  AND all_item_references_spawn_eligible
  AND all_quantity_policies_valid
  AND evidence_labels_valid
```

| Variable | Type | Expected Range | Description |
|---|---|---:|---|
| `table_identity_valid` | bool | `false/true` | Table ID/version present and unique. |
| `all_groups_valid` | bool | `false/true` | Groups have IDs, order, policy, and total weight > 0. |
| `all_rows_valid` | bool | `false/true` | Rows have IDs, order, weights, labels, and required fields. |
| `all_item_references_spawn_eligible` | bool | `false/true` | Every item row passes Item Definition eligibility. |
| `all_quantity_policies_valid` | bool | `false/true` | Quantity policies cannot produce illegal quantities. |
| `evidence_labels_valid` | bool | `false/true` | Source/evidence labels are present and truthful. |
| `drop_table_valid` | bool | `false/true` | Whether table may be used for normal gameplay. |
| `drop_table_failure_reasons` | ordered reason set | empty or structured reason IDs | Deterministic validation failures ordered by stage, table/group/row order, then reason enum order. |

Output range: `drop_table_valid` returns `true` with an empty reason set, or `false` with deterministic structured reasons. Failure reasons are ordered by validation stage, then table/group/row authoring order, then reason enum registry order; Dictionary iteration order must not affect reason order.

Example：

```text
true AND true AND true AND true AND true AND true = true
```

If one row references a deprecated item:

```text
all_item_references_spawn_eligible = false
drop_table_valid = false
primary_reason = INELIGIBLE_ITEM_REFERENCE
```

## Edge Cases

| Edge Case | Required Handling |
|---|---|
| Missing `drop_table_id` in roll request | Return `MISSING_TABLE`; produce no item candidate. |
| Duplicate drop table IDs during catalog validation | Catalog invalid; normal gameplay cannot use ambiguous tables. |
| Unsupported `drop_table_version` | Return `INVALID_TABLE` with version reason. |
| Group total weight equals zero | Table invalid; do not divide by zero or auto-grant no-drop. |
| Negative row weight | Table invalid; never clamp negative to zero. |
| Weight overflow | Table invalid with `INVALID_WEIGHT_CONFIG`; do not wrap integer values. |
| Row weight is zero | Row is disabled for selection; in normal tables it must still have syntactically valid item reference, source label and quantity policy. Missing placeholder references are debug/test-profile only. |
| No-drop selected | Valid `NO_DROP` result with provenance; downstream receives no item candidate. |
| No no-drop weight and valid item weights | Valid; every roll in that group produces an item row. |
| Multiple roll groups produce multiple candidates | Allowed only if groups are explicitly authored; Phase 1 emits one result DTO per group attempt or an ordered aggregate. If any group/table validation fails, the normal gameplay table fails as a whole before partial results emit. |
| Missing item reference | Validation fails with `INVALID_ITEM_REFERENCE`; no placeholder loot substitution. |
| Deprecated/debug/blocked item reference in normal table | Validation fails with `INELIGIBLE_ITEM_REFERENCE`. |
| Row references display name instead of `item_id` | Validation fails with `INVALID_ITEM_REFERENCE`. |
| Equipment quantity greater than 1 | Invalid in Phase 1. |
| Material quantity exceeds `max_stack_size` | Invalid; do not silently clamp or split. |
| Quantity roll outputs 0 or negative | Invalid quantity policy or RNG bug; no normal item candidate. |
| Fractional quantity | Invalid; Phase 1 quantities are integers. |
| Item Definition changes after table validation | Runtime must bind to a validated definition version or force revalidation before normal play. |
| `openmir2_verified` label lacks accepted evidence | Validation fails with `OPENMIR2_EVIDENCE_NOT_READY`. |
| Roll uses global RNG/time/frame/signal/scene order | Implementation fails determinism acceptance. |
| Two monsters die in same frame/tick | Roll order uses authoritative source event sequence plus stable source IDs, not scene-tree order. |
| Source monster has no configured table | Return `MISSING_TABLE` or configured source-level no-drop; do not infer a default table silently. |
| Debug table references debug items | Allowed only in debug/test profile; normal profile rejects it. |
| Valid roll but ground placement later fails | Drop Table result remains valid grant intent; ADR-0017 GroundDrop handles `FAILED_PLACEMENT`. |
| Inventory is full during pickup | Outside Drop Table; probabilities do not change. |
| Pity/dry-streak guard requested | Requires separate state owner/policy; base Drop Table remains stateless until approved. |
| QA needs rare result reliably | Use deterministic seed fixture or debug table; do not tune live rare chance solely for QA convenience. |

## Dependencies

### Upstream Dependencies

| Dependency | Relationship |
|---|---|
| `OpenMir2 行为映射 Spike` | Defines whether drop behavior/rates may claim source authenticity. Until ADR-0016 accepts evidence or contract readiness, values remain `mvp_provisional` or `openmir2_evidence_pending`. |
| `物品定义系统` / ADR-0015 | Owns item template truth and formulas such as `spawn_eligible_reference` and stack validity. Drop Table reads item IDs and eligibility only. |
| ADR-0016 OpenMir2 Evidence Governance | Controls `openmir2_verified` labels and evidence readiness. |
| ADR-0017 Drop/Ground/Pickup Lifecycle | Defines handoff boundary: Drop Table emits grant candidates; GroundDrop/Pickup own placement and pickup lifecycle. |
| ADR-0021 Drop Table Runtime Boundary | Defines Drop Table runtime catalog, validation stages, deterministic roll service, quantity policy, result DTOs, and grant candidate handoff. |
| Monster / Reward Source definitions | Provide source IDs/archetypes that map to drop table IDs. Drop Table does not define monster stats, AI, spawn, or death. |
| Deterministic RNG contract | Provides replayable roll values; Drop Table owns deterministic use of RNG for weights/quantity. |

### Downstream Dependencies

| Consumer | Relationship |
|---|---|
| Ground Drop Service / 掉落与拾取系统 | Consumes grant candidates and owns ground records, placement, lifecycle, pickup claim, despawn, and no-half-commit pickup. |
| Map Coordinate / Blocking / Y-sort System | Used downstream by GroundDrop for placement/readability; Drop Table does not query map placement. |
| Inventory System | Receives committed pickup rewards later; owns capacity, merge, slots, stack split, final storage, and save truth. |
| Equipment System | Interprets equipment after inventory receive; owns equip legality, slot transactions, and modifier-source handoff. |
| Loot Visual / Audio Feedback System | Reads post-handoff result and Item Definition projections for presentation; cannot affect roll outcome. |
| QA / Analytics / Debug Tools | Read provenance, expected acquisition, validation reports, and replay data for deterministic verification. |

### Bidirectional Dependency Requirements

| System | Required Reciprocal Statement | Current Status |
|---|---|---|
| Item Definition | Drop Table validates references only; Item Definition owns template truth. | Satisfied by Item Definition GDD / ADR-0015; this GDD references `spawn_eligible_reference`. |
| Ground Drop / Pickup | Consumes grant candidates and owns placement/pickup lifecycle. | Satisfied at ADR level by ADR-0017; future GDD must mirror before approval. |
| Inventory | Receives committed rewards downstream of Drop/Pickup, not direct Drop Table output. | Future GDD blocker. |
| Monster / Death | Death emits reward source context but does not roll loot directly. | Future GDD blocker. |
| Loot Feedback | Observes roll/pickup results and Item Definition projections without changing probabilities. | Future GDD blocker. |

## Tuning Knobs

| Knob | Safe Range | Recommended Phase 1 | Affects | Rationale / Warning |
|---|---:|---:|---|---|
| `no_drop_weight` | `0–1,000,000` | `6000` in 10000 table | Drop frequency, ground clutter | Higher reduces loot volume; lower increases pickup pressure. |
| `material_row_weight` | `0–1,000,000` | `2500` | Common reward cadence | Frequent reinforcement without fast equipment growth. |
| `common_equipment_weight` | `0–1,000,000` | `1200` | Compare/equip frequency | Too high makes equipment feel disposable. |
| `upgrade_equipment_weight` | `0–1,000,000` | `250` | Progression excitement | Tune carefully to avoid trivial growth. |
| `rare_showcase_weight` | `0–1,000,000` | `50` | High-salience surprise | Test with deterministic seeds; raw chance is too low for routine QA. |
| `group_total_weight_target` | `1–1,000,000` | `10000` | Authoring readability | 10000 gives basis-point precision. |
| `roll_group_attempt_count` | `1–10` | `1` | Number of chances per kill | Keep `1` for MVP clarity. |
| `material_quantity_min` | `1–max_stack_size` | `1` | Stack gain speed | Must satisfy Item Definition stack rules. |
| `material_quantity_max` | `quantity_min–max_stack_size` | `3` | Stack gain variance | Avoid large stacks before sinks exist. |
| `equipment_quantity` | `1` | `1` | Equipment instance clarity | Equipment does not stack in Phase 1. |
| `row_count_max_per_group` | `1–200` | `5–20` | Content scope/performance | Small tables are easier to validate and tune. |
| `normal_table_allows_debug_items` | `false` only | `false` | Content safety | Debug leakage is a validation failure. |
| `openmir2_claim_required_evidence_level` | Accepted E3/E4 policy | Accepted evidence only | Authenticity labeling | Otherwise mark provisional/evidence-pending. |
| `dry_streak_guard_enabled` | `false/true` | `false` | Bad-luck protection | Requires separate state owner; do not hide it in base table. |
| `qa_seed_fixture_enabled` | `false/true` | `true` for tests | QA repeatability | Does not change live probabilities. |

Economy health / faucet risk:

- Recommended rates are technical-slice validation-biased, not long-term economy rates.
- There are no mature sell, salvage, crafting, durability, material sink, or inventory-pressure systems yet.
- At an assumed 10 kills per 30-second smoke loop, expected outputs are about 4 drops, 5 material units, 1.5 equipment items, 0.3 upgrade-or-better items, and 0.05 rare/showcase items.
- Future economy review must rebalance rates once sinks, content length, monster density, inventory friction, and OpenMir2 evidence contracts are available.

Statistical test policy:

- Blocking unit tests should validate exact boundary roll values with fake deterministic RNG, not rely on probabilistic sampling.
- Optional statistical smoke checks may exist only with fixed seeds and wide non-blocking tolerances.
- QA fixtures must be able to force exact `roll_value` and quantity RNG offsets directly; seed-hunting is not acceptable as primary evidence.

Safe tuning guidance：

- Increase `no_drop_weight` if inventory or ground clutter overwhelms the loop.
- Increase material weight if kills feel unrewarding but equipment progression is too fast.
- Increase common equipment weight if comparison/equip UI lacks enough test coverage.
- Decrease upgrade/rare weights if players outgrow the slice too quickly.
- Do not tune item power by changing drop weights alone; item power belongs to Item Definition modifier payloads and Character Attributes.
- Do not tune drop frequency by editing Item Definition quality; quality is metadata unless a downstream system explicitly defines its use.

## Acceptance Criteria

### Design / Data Validation

- [ ] Every normal drop table has unique `drop_table_id` and supported `drop_table_version`.
- [ ] Every normal roll group has deterministic `group_id`, explicit `group_order`, valid policy, and `drop_group_total_weight > 0`.
- [ ] Every row has stable `row_id`, explicit `row_order`, integer non-negative weight, valid source/evidence label, and required quantity fields.
- [ ] Every authored item row in a normal gameplay table has a syntactically valid item reference; every selectable/non-zero item row validates against Item Definition `spawn_eligible_reference`.
- [ ] Deprecated, debug-only, invalid, blocked-unconfirmed, or missing item definitions are rejected for normal gameplay tables.
- [ ] Quantity policies cannot output `0`, negative, fractional, or item-illegal quantities.
- [ ] Equipment rows produce quantity `1` in Phase 1.
- [ ] All concrete non-evidence-backed drop rates are labeled `mvp_provisional`.

### Formula / Probability QA

- [ ] QA can calculate each row probability from `row_weight / drop_group_total_weight`.
- [ ] QA can calculate expected attempts for each output tier.
- [ ] The MVP provisional table sums to total weight `10000` if the recommended table is used.
- [ ] Any change to row weights includes updated expected acquisition numbers.
- [ ] A table with total weight `0`, negative weights, or overflow fails validation with structured reasons.

### Determinism Tests

- [ ] Same table version + same source context + same RNG stream + same roll sequence produces identical results across repeated runs.
- [ ] Different roll sequences produce deterministic but independently replayable results.
- [ ] Roll result provenance includes table ID/version, source context, RNG stream/seed label, roll sequence, group ID, selected candidate, quantity result, and status.
- [ ] Tests prove roll selection does not depend on Dictionary iteration order, scene-tree order, signal order, wall-clock time, frame delta, or global RNG.

### Boundary Tests

- [ ] Drop Table emits item grant candidates only; it does not create ground drops, mutate map occupancy, grant inventory, equip items, or play feedback.
- [ ] Drop Table rows do not copy item display name, icon, quality truth, stack truth, equipment modifiers, or combat power.
- [ ] Ground placement failure is not handled by rerolling in Drop Table.
- [ ] Inventory full behavior does not change Drop Table probabilities.
- [ ] Pickup success is not inferred from Drop Table roll success.

### Player Loop QA

- [ ] In a deterministic Phase 1 smoke fixture, a tester can kill monsters and observe at least one no-drop, one material-role drop, one equipment-role drop, and one upgrade-role drop through controlled seed/table setup.
- [ ] At recommended MVP weights, any-drop probability is documented as 40%, expected once per 2.5 kills.
- [ ] At recommended MVP weights, any-equipment probability is documented as 15%, expected once per 6.67 kills.
- [ ] At recommended MVP weights, upgrade-or-better probability is documented as 3%, expected once per 33.33 kills.
- [ ] Material expected faucet is documented as 0.5 material units per kill before downstream sinks.
- [ ] Rare/showcase acquisition is tested through deterministic seed or debug fixture rather than relying on raw 0.5% chance during manual QA.

### Registry / Cross-System Follow-Up

- [ ] Before implementation or final GDD approval, all concrete item IDs used by drop rows are added to `design/registry/entities.yaml` or explicitly approved as local fixture-only test data.
- [ ] If new cross-system constants such as `group_total_weight_target` become canonical, they are registered.
- [ ] If formulas such as `drop_group_total_weight`, `drop_row_probability`, `expected_attempts_for_row`, `expected_attempts_for_tier`, `drop_quantity_result`, or `drop_table_valid` are referenced by downstream GDDs, they are added to the registry formula section.
