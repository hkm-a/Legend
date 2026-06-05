# 物品定义系统

> **Status**: Approved for Implementation Planning — implementation blocked pending real Godot test runner and story/test slicing
> **Author**: hkm + Claude Code Game Studios
> **Last Updated**: 2026-06-04
> **Implements Pillar**: Primary — 爆装有戏; Supports — 每次登录都带走成长 / 传奇骨架，现代皮肤 / 稳刷不断流
> **Quick reference** — Layer: `Foundation` · Priority: `MVP` · Key deps: `OpenMir2 行为映射 Spike`

## Overview

物品定义系统是 Phase 1 离线刷怪爆装切片的共享物品事实层。它定义每个物品的稳定身份、类型、品质、显示元数据、图标/视觉提示键、堆叠规则、实例化需求、装备数据入口、装备可提交给属性系统的 modifier payload 边界，以及掉落表、掉落与拾取、背包、装备、UI、存档和调试工具读取物品时必须遵守的合同。该系统不决定怪物掉率、地面掉落生命周期、拾取距离、背包容量、穿戴流程、最终属性公式、战力公式、商店价格、UI 布局或 OpenMir2-authentic 数值；它的职责是保证“怪物爆出一个东西”之后，所有下游系统都指向同一份可验证的物品定义，而不是各自复制、猜测或临时拼接 item truth。Phase 1 允许使用明确标记的 MVP provisional 物品与装备定义来验证 30 秒循环；任何声称 OpenMir2-authentic 的物品字段、分类、装备槽、基础数值或实例结构，都必须引用 `OpenMir2 行为映射 Spike` 的 E3/E4 evidence 或源码位置。

## Player Fantasy

玩家应当在刷怪爆装的瞬间感到：“这件东西可能让我变强。”物品定义系统本身不是玩家主动操作的菜单或战斗技能，但它决定掉落物是否具备清晰、可信、可延展的玛法战利品身份：玩家看到地上的名称、品质色、图标和类型信号时，能一眼判断它是否值得捡；拾取后能快速理解它是装备、消耗品、材料还是货币；如果它是装备，玩家会自然期待它能进入对比和穿戴流程，并在下一轮战斗中验证角色变强。

这个幻想服务的是 30 秒循环中的最小成长承诺：**爆出战利品 → 看懂它 → 捡起来 → 判断价值 → 穿上或保留 → 继续稳刷**。物品定义系统不负责制造掉落概率、战斗伤害或成长特效，但它必须让每个物品拥有稳定身份和足够的显示/装备语义，使下游掉落反馈、背包、装备、属性、HUD 和成长反馈系统能共同传达“这趟刷怪没有白刷”。

Phase 1 的物品可以是少量、简化、MVP provisional 的测试物品，但不能像调试数据。即使只有几件装备和少量非装备掉落，它们也应具备一致的命名、品质、图标键、类型、可堆叠性和用途边界，让玩家相信这里已经有一套可继续扩展的传奇式战利品体系。

## Detailed Rules

### Core Rules

#### 1. Ownership Boundary

物品定义系统 owns：

- 稳定 `item_id` 的命名、唯一性、版本语义和弃用语义；
- 物品模板的数据合同，包括类型、品质、显示元数据、图标键、堆叠策略和实例化需求；
- Phase 1 装备定义入口，包括装备分类、候选装备槽标签、主比较提示和可提交给装备 / 属性链路的 modifier payload；
- 物品定义、装备字段和 modifier payload 的 `source_status` / evidence 标签；
- 物品定义配置的验证规则和结构化失败原因；
- `active`、`debug_only`、`deprecated`、`invalid`、`blocked_unconfirmed` 等定义状态的语义。

物品定义系统 does not own：

- 怪物掉率、掉落权重、掉落池选择、保底、数量 roll 或来源怪物；
- 地面掉落存在时间、拾取距离、拾取优先级、地图放置和地面物生命周期；
- 背包容量、格子布局、排序、合并/拆分算法、溢出处理或当前槽位；
- 装备穿戴流程、最终穿戴合法性、槽位占用、替换事务或当前装备状态；
- 最终属性公式、effective stats、attribute snapshot、attribute delta、combat power 或 TTK；
- 商店买卖价、回收价、分解产物、强化材料消耗或经济 sink/faucet；
- tooltip 布局、品质颜色映射、VFX、音效、动画、最终本地化文本或 UI 交互流程；
- OpenMir2 行为映射 Spike 本身。

禁止流程：

- 掉落表、背包、装备、UI、存档或调试工具各自复制物品定义字段作为事实来源。
- 物品定义直接写角色最终属性、combat power、attribute snapshot 或 HUD 数值。
- 装备定义将 derived/effective stat 写成已计算结果。
- 在 OpenMir2 Spike 未达到 E3/E4 前，将具体字段、槽位、数值或实例结构标为 OpenMir2-authentic。

#### 2. Item Identity

每个可被掉落、拾取、放入背包、穿戴、显示、保存或调试引用的物品模板必须拥有稳定 `item_id`。

`item_id` rules：

- `item_id` 是跨系统引用键，不是本地化显示名。
- `item_id` 在同一配置版本内必须唯一。
- `item_id` 不得因显示名、描述、图标、品质色或数值调整而改变。
- 已被存档、掉落表、测试 fixture、registry 或其他 GDD 引用的 `item_id` 不得删除；必须改为 `deprecated` 并提供迁移、隐藏或 blocked-load 策略。
- Phase 1 推荐使用项目本地命名，例如 `mvp_bronze_sword`、`mvp_cloth_armor`、`mvp_small_material_shard`。这些名称必须标记为 `mvp_provisional` 或 `project_local`，不能暗示 OpenMir2-authentic。

Minimum identity fields：

| Field | Required | Meaning |
|---|---:|---|
| `item_id` | Yes | Stable cross-system item key. |
| `definition_version` | Yes | Definition/config version used for validation, save/load, migration, and debug. |
| `item_type` | Yes | Primary item type. |
| `quality_id` | Yes | Classification/display metadata, not price/drop/power authority. |
| `source_status` | Yes | `mvp_provisional`, `project_local`, `openmir2_evidence_pending`, `openmir2_verified`, or `debug_only`. |
| `evidence_ref` | Conditional | Required when `source_status = openmir2_verified`. |
| `lifecycle_status` | Yes | `active`, `debug_only`, `deprecated`, `invalid`, or `blocked_unconfirmed`. |

#### 3. Template vs Instance

物品定义系统区分 template、instance、inventory stack 和 equipped reference。

A **template** is the shared definition row for an item. It answers：

- 这是什么物品？
- 它如何被显示和分类？
- 它是否可堆叠？
- 它是否能生成具体实例？
- 如果是装备，它声明哪些装备候选信息和 modifier payload？
- 下游系统是否允许正常引用它？

An **instance** is a concrete copy or stack entry created during gameplay. It answers：

- 这一个背包格、地面掉落或装备槽中的具体物品是什么？
- 它引用哪个 `item_id` 和 `definition_version`？
- 它的数量是多少，或它是否需要唯一 `item_instance_id`？
- 它是否携带被批准的 instance-level fields？

Rules：

- 所有实例必须引用一个 valid item definition。
- Stackable items may use lightweight stack records rather than unique per-unit instances.
- Non-stackable equipment must have a stable `item_instance_id` once spawned into gameplay state.
- Instance records may not redefine template truth such as display name, item type, quality, stackability, base equipment category, source status, or template modifier rows.
- Phase 1 不启用随机词条、耐久、绑定、孔位、强化等级或洗练；若未来加入，它们必须作为显式 instance fields、linked modifier source 或独立成长系统输出，而不是隐藏覆盖 template。

#### 4. Item Type Contract

Every item definition must declare exactly one primary `item_type`.

Recommended Phase 1 item types：

| `item_type` | Meaning | Phase 1 Behavior |
|---|---|---|
| `equipment` | Can participate in equipment / equip-preview flow. | Requires equipment data block; non-stackable. |
| `material` | Loot/crafting/vendor-style object. | MVP display/stack only unless crafting/economy exists. |
| `currency` | Currency-like pickup. | Optional; if account currency exists later, currency system owns amount. |
| `consumable` | Can later trigger an effect. | Reserved / optional; Phase 1 may define display/stack only. |
| `quest` | Quest-bound object. | Reserved; not recommended for Phase 1. |
| `debug` | Test-only item. | Must be hidden from normal drop tables and player saves. |

Rules：

- Phase 1 required enabled types are `equipment` and `material`; `currency` is allowed only if the drop/pickup slice needs a currency-like pickup.
- `consumable` and `quest` may exist as reserved enum/category values but are not Phase 1 implementation requirements.
- `equipment` items must include an equipment data block.
- Non-equipment items must not include active equipment modifiers in Phase 1.
- `debug` items must not appear in normal drop tables, save fixtures, or player-facing loot UI unless debug mode is explicitly active.
- Type does not define drop chance, pickup radius, inventory size cost, sell price, use effect, or final equip legality by itself.

#### 5. Quality Contract

Every item definition must declare one `quality_id`.

Recommended Phase 1 quality IDs：

| `quality_id` | Meaning | Phase 1 Rule |
|---|---|---|
| `normal` | Baseline loot. | Allowed. |
| `fine` | Slightly better / more noticeable. | Allowed. |
| `rare` | Higher-salience loot for `爆装有戏`. | Allowed if drop/feedback systems need it. |
| `debug` | Test-only quality. | Debug-only, not normal loot. |

Reserved future quality IDs such as `epic` or `legendary` may be documented later, but Phase 1 should not require them unless the thin slice specifically needs a showcase drop.

Rules：

- Quality is classification and display/salience metadata, not an automatic stat, drop-rate, price, or combat-power formula.
- If a downstream system wants quality to affect drop table grouping, loot beam intensity, audio intensity, compare emphasis, recycle output, or shop price, that system must define the rule and cite `quality_id` as input.
- Phase 1 qualities are `mvp_provisional` unless OpenMir2 evidence later verifies authentic categories and mapping.
- UI may style by quality, but item definition must not store final UI color values or layout directives.

#### 6. Display Metadata

Every player-facing item definition must include display metadata sufficient for loot labels, pickup feedback, inventory display, equipment tooltip, save/debug inspection, and UI handoff.

Minimum display fields：

| Field | Required | Meaning |
|---|---:|---|
| `display_name_key` | Yes | Localization/display key; raw `item_id` must not be final player text. |
| `description_key` | Recommended | Localization/display key for tooltip/details. |
| `icon_key` | Yes | Logical icon reference, not necessarily a final asset path. |
| `world_visual_key` | Optional | Logical ground-drop visual reference. |
| `pickup_label_key` | Optional | Override for ground label if needed. |
| `debug_label` | Optional | Developer-only readable label. |

Rules：

- Display keys are stable logical references; UI owns layout, final localized text, style, colors, icons fallback, VFX and animation.
- Missing final art asset may use approved placeholder icon keys, but the definition must still name a logical icon key.
- UI must not infer item type or quality from icon path, color, display text, or string prefix.

#### 7. Phase 1 Player-Facing Minimum Metadata

Normal Phase 1 player-facing item definitions must provide enough semantic metadata for a downstream UI / loot-feedback system to let the player understand the item without debug fields or raw IDs. These fields are data-side requirements; UI, localization, art, VFX and audio systems still own final rendering and assets.

Minimum fields for every normal spawnable item:

| Field | Requirement | Meaning |
|---|---:|---|
| `display_name_key` | Required | Player-facing name lookup key. |
| `type_label_key` or resolvable `item_type` label mapping | Required | Text/semantic item type label such as Equipment or Material. |
| `quality_label_key` or resolvable `quality_id` label mapping | Required | Text/semantic quality label; quality cannot be color-only. |
| `icon_key` | Required | Logical icon token, placeholder allowed only as explicit debt. |
| `world_visual_key` | Required for droppable normal items | Logical ground-drop visual token. |
| `pickup_label_key` or `short_name_key` | Required for droppable normal items | Ground label / pickup toast text key. |
| `visual_family` | Required for droppable normal items | Art-direction material family token. |
| `audio_family` | Required for droppable normal items | Audio material family token; not an audio file path. |
| `pickup_visual_salience` or derivable default from `quality_id` | Required for droppable normal items | Semantic loot salience token. |

Additional fields for normal spawnable equipment:

| Field | Requirement | Meaning |
|---|---:|---|
| `main_comparison_hint` | Required | Player-facing comparison focus such as attack, defense, health, or none only for non-spawnable placeholder/cosmetic rows. |
| `visible_modifier_facts` derivable from valid `modifier_payload` | Required for normal stat equipment | Filtered player-visible modifier facts suitable for tooltip display; does not include effective stats or better/worse judgment. |
| `comparison_unavailable_reason_key` | Conditional | Required if an equipment-like row is intentionally not comparable; such rows are not normal MVP loot unless explicitly approved. |

Rules:

- A normal Phase 1 equipment drop must be player-explainable: it either exposes at least one valid player-visible modifier fact and comparison hint, or it is blocked from normal spawnable content.
- `quality_id` may raise presentation salience, but final stronger/weaker judgment must come from Equipment / Character Attributes preview or post-equip delta.
- If `pickup_visual_salience` is absent, downstream presentation may derive a default from `quality_id`; `rare` must not silently present at a lower-than-normal salience unless explicitly marked debug, muted, or placeholder and excluded from normal loot.
- Fallback text must be represented as localization keys or semantic statuses. Literal strings in this GDD are examples, not final player-facing text.

#### 8. Stacking Rules

Every item definition must declare stack behavior.

Minimum stack fields：

| Field | Required | Meaning |
|---|---:|---|
| `stack_policy` | Yes | `non_stackable` or `stackable`; `debug_unique_stack` is debug-only if needed. |
| `max_stack_size` | Yes | `1` for non-stackable; `> 1` for stackable. |
| `quantity_unit` | Conditional | Recommended for stackable items, e.g. `count`. |

Rules：

- Equipment is `non_stackable` in Phase 1.
- If `stack_policy = non_stackable`, `max_stack_size` must be `1`.
- If `stack_policy = stackable`, `max_stack_size` must be greater than `1`.
- Quantity `0` is not a valid live inventory or ground stack. Removing the final unit destroys the stack record.
- Drop/pickup/inventory systems own partial stack merge, overflow split, pickup failure, capacity, and UI split/merge flows.
- Item definition only declares the maximum legal stack capacity for that item.

#### 9. Equipment Data Contract

Every `equipment` item must include an equipment data block.

Minimum equipment fields：

| Field | Required | Meaning |
|---|---:|---|
| `equipment_category` | Yes | Broad classification such as weapon-like, armor-like, accessory-like. |
| `equip_slot_tags` | Yes | Candidate slot tags used by Equipment; not final slot authority. |
| `modifier_payload` | Yes for stat equipment | Modifier rows to be resolved by Equipment and validated by Character Attributes. |
| `main_comparison_hint` | Recommended | Suggested player-facing comparison focus such as attack, defense, or health. |
| `equip_requirement_payload` | Optional / Future | Requirements data if Equipment later owns legality gates. |
| `source_status` | Yes | Evidence/provisional label for equipment data. |

Rules：

- Equipment data does not mean the item is always equip-legal. Equipment system owns legality, slot occupancy, replacement and equip/unequip transactions.
- `equip_slot_tags` are candidate tags, not final OpenMir2-authentic slot IDs unless E3/E4 evidence exists.
- If slot tags, categories, requirement fields, or equipment classifications claim OpenMir2 authenticity, they require evidence references.
- Phase 1 may use project-local provisional slot tags only if clearly marked `mvp_provisional`.
- Equipment data must not include final effective stats, derived stats, combat power, damage formulas, DPS, TTK, sell value or build recommendations.

#### 10. Modifier Payload Boundary

Equipment definitions may declare modifier payloads only as inputs for the Equipment → Character Attributes integration path.

Phase 1 allowed modifier operation：

| Operation | Status | Meaning |
|---|---|---|
| `add_flat` | Allowed | Adds an integer flat amount to an enabled modifier-targetable stat. |

Unsupported in Phase 1 unless later approved：

- percent modifiers;
- multiplicative modifiers;
- override/set modifiers;
- conditional proc modifiers;
- random roll ranges as runtime rolls;
- temporary buffs;
- durability-based scaling;
- class-scaling modifiers;
- hidden combat power modifiers.

Minimum modifier row fields：

| Field | Required | Meaning |
|---|---:|---|
| `modifier_row_id` | Yes | Stable row ID within the item definition. |
| `operation` | Yes | Phase 1 allowed: `add_flat`. |
| `target_stat_id` | Yes | Must match a Character Attributes modifier-targetable stat ID. |
| `value` | Yes | Integer flat value within technical safe range. |
| `source_status` | Yes | Evidence/provisional label. |
| `evidence_ref` | Conditional | Required for `openmir2_verified`. |
| `player_visible_hint` | Recommended | Whether this modifier should normally appear in comparison. |

Rules：

- Item definitions own the declared modifier payload for an item template.
- Equipment system owns resolving equipped instances into active modifier sources.
- Character Attributes owns validation, aggregation, effective stats, snapshots, delta, and combat power.
- Modifier target IDs must use the Character Attributes stat registry. Unknown stat IDs are invalid.
- Modifiers targeting `health_current` or `mana_current` are forbidden in Phase 1.
- Modifiers targeting reserved/inactive stats are invalid for player-facing effect unless Character Attributes explicitly enables those stats.
- A non-equipment item with active modifier payload is invalid unless a future consumable/buff system explicitly defines non-equipment modifier sources.

#### 11. Source and Evidence Labels

Every item definition and every OpenMir2-sensitive subfield must carry source status.

Recommended labels：

| Label | Meaning | Rule |
|---|---|---|
| `mvp_provisional` | Project-created value used to validate Phase 1 loop. | Allowed for Phase 1; must not be called authentic. |
| `project_local` | Intentional non-OpenMir2 project design. | Allowed if documented. |
| `openmir2_evidence_pending` | Intended to map to OpenMir2 but not verified yet. | May exist as blocked/unconfirmed design data; not implementation authority. |
| `openmir2_verified` | Backed by accepted E3/E4 evidence. | Requires evidence reference. |
| `debug_only` | Used only for tests/dev tools. | Must not appear in normal player flow. |

Rules：

- A definition may be overall `mvp_provisional` while individual fields are later upgraded to `openmir2_verified`.
- Any concrete OpenMir2-authentic claim without E3/E4 evidence is invalid.
- If evidence later conflicts with a provisional row, the row must be revised or deprecated; consumers must not silently reinterpret the old meaning.
- Evidence labels are part of validation, save/load diagnostics, and design review.

#### 12. Definition Validation

An item definition is valid only if all required fields and type-specific rules pass.

Validation gates：

1. **Identity validation**
   - `item_id` exists and is unique.
   - `definition_version` exists.
   - `source_status` and `lifecycle_status` are valid.

2. **Display validation**
   - Player-facing rows have `display_name_key` and `icon_key`.
   - `quality_id` and `item_type` exist.
   - Debug-only rows are not marked normal player-facing.

3. **Stack validation**
   - Stack policy exists.
   - Non-stackable rows have `max_stack_size = 1`.
   - Stackable rows have `max_stack_size > 1`.
   - Equipment rows are non-stackable in Phase 1.

4. **Type validation**
   - Equipment rows include equipment data.
   - Non-equipment rows do not include active equipment data.
   - Consumable/material/currency rows do not imply use effects unless a future system defines them.

5. **Equipment validation**
   - Equipment rows include category and slot tags.
   - Modifier payload rows use allowed operations.
   - Modifier targets are known and modifier-targetable according to Character Attributes.
   - Modifier values are finite integers within safe config range.
   - Source/evidence labels are present.

6. **Evidence validation**
   - `openmir2_verified` rows and fields include evidence references.
   - `openmir2_evidence_pending` fields are not treated as final implementation authority.
   - Provisional values are clearly labeled.

Failure behavior：

- Invalid definitions must not be spawnable.
- Drop tables must reject invalid item references at validation time.
- Inventory/equipment/UI/save/debug consumers must receive structured invalid reason codes, not guess fallback behavior.
- Debug tools may inspect invalid definitions, but normal gameplay cannot use them.

#### 13. Runtime Definition Truth

Runtime systems must consume normalized definition data, not mutable authoring graphs.

Rules：

- If `.tres` Resources are used later, they are authoring envelopes only.
- Runtime truth must be a normalized DTO/table representation.
- Runtime systems must not retain shared mutable Resource graphs as authoritative item truth.
- Runtime item instances store references to normalized definition IDs and versions.
- Definition data should be treated read-only after validation/load.
- Runtime hot reload, if later supported, must produce a new validated definition version rather than mutating live truth silently.

This aligns with ADR-0013: Resource graph identity and shared references are not gameplay authority.

#### 14. Runtime Query Result and UI Projection Contract

Downstream systems must consume status-bearing query/projection results rather than raw authoring rows or mutable Resource graphs. The exact Godot class shape belongs to ADR, but the design-level contract is mandatory.

Minimum item definition query result fields:

| Field | Meaning |
|---|---|
| `request_item_id` / `request_definition_version` | The reference being resolved. |
| `reference_resolution_status` | `resolved`, `missing`, `version_mismatch`, `unsupported_version`, `newer_than_client`, `deprecated_resolved`, or `blocked_unconfirmed`. |
| `definition_validation_status` | `semantic_valid`, `semantic_invalid`, or `not_loaded`. |
| `runtime_eligibility_status` | `normal_spawnable`, `display_only`, `debug_only_blocked`, `deprecated_legacy_readable`, `equip_blocked`, or `not_runtime_eligible`. |
| `ui_availability_status` | `normal`, `unavailable_missing`, `unavailable_invalid`, `unavailable_stale`, `legacy_deprecated`, `hidden_debug_only`, or `blocked_unconfirmed`. |
| `player_safe_display` | Display/localization keys, icon key, fallback icon key, type/quality label keys, and player-safe fallback status keys. |
| `classification` | `item_type`, `quality_id`, stack policy, max stack size, and inert classification tags. |
| `equipment_candidate_view` | Equipment category, candidate slot tags, comparison hint, visible modifier facts, and comparison-unavailable reason key. |
| `presentation_view` | World visual key, visual family, audio family, presentation tier, pickup visual salience, and optional cue tokens. |
| `debug_reasons` | Ordered debug-only reason codes and field paths. |

UI-facing projections must be narrower than raw definition rows:

- `LootLabelItemView` reads label key, quality/type labels, world visual key, salience, audio family, and fallback status only.
- `InventoryCellItemView` reads icon key, quantity display eligibility, type/quality labels, stack policy, and availability status only.
- `TooltipItemView` reads display keys, classification, safe visible modifier facts, and player-safe invalid/deprecated states.
- `EquipmentComparisonItemView` reads item identity, equipment candidate metadata, main comparison hint, and an Equipment / Character Attributes preview handoff target; it must not compute final deltas directly from modifier rows.

Fallback strings in examples such as “Unknown Item” are semantic examples only. Runtime/UI contracts must use localization keys or semantic fallback statuses; final text belongs to UI/localization.

#### 15. Deprecation and Migration

Definitions may be deprecated but should not be deleted once referenced by saves, tests, drop tables, registry entries or downstream GDDs.

Deprecation rules：

- `deprecated` definitions are not allowed in new normal drop tables.
- Existing saved instances referencing deprecated definitions may load only through an explicit migration, fallback, or blocked-load policy.
- Deprecated definitions may remain visible in debug tools.
- A deprecated item must not be silently remapped to another item if the player-facing meaning changes.
- If migration exists, it must record old `item_id`, old version, new `item_id`, new version, and reason.

#### 16. MVP Provisional Item Set and Loot-Loop Fixture

Phase 1 must include a small provisional item set sufficient to validate the player-facing loot loop, not only schema validity. The minimum fixture is intentionally narrow but must support **baseline → sidegrade/worse → upgrade → optional rare showcase** item judgment.

Minimum fixture shape:

| Fixture Role | Minimum Count | Normal Spawnable? | Purpose |
|---|---:|---:|---|
| Baseline equipped item or baseline equipment reference | 1 | May be starter/save fixture rather than drop | Provides a compare target; empty-slot comparison cannot be the only MVP validation path. |
| Clear upgrade equipment, same comparable slot/category | 1 | Yes | Produces a positive visible Attribute preview or post-equip delta. |
| Sidegrade or weaker equipment, same comparable slot/category | 1 | Yes | Proves the player can judge “not better” rather than always equipping the only item. |
| Stackable material | 1 | Yes unless material is deferred | Proves non-equipment loot classification and stack display. If no material use exists, its description/usage hint must state it is future/provisional. |
| Rare/showcase equipment or material | Optional but recommended for loot-feedback slice | Yes if included | Proves high-salience `quality_id`, `presentation_tier` / `pickup_visual_salience`, and audio/visual handoff. |

Candidate example IDs such as `mvp_bronze_sword` are examples only until registered as canonical item candidates in `design/registry/entities.yaml`; downstream drop tables must not treat example IDs as approved content facts until the MVP item set is named and registered.

The fixture must validate that:

- monster death can reference a drop table item;
- ground drop can show item label/icon/quality/type and audio/visual salience tokens;
- pickup can create an inventory record;
- inventory can display stackable and non-stackable items;
- at least two same-category equipment items can enter comparison against a baseline;
- equipping the upgrade item can produce valid `add_flat` modifier sources;
- Character Attributes can publish visible stat/combat power delta after equipment change;
- debug, invalid, blocked-unconfirmed, and placeholder-only items do not leak into normal MVP loot.

Rules:

- The MVP item set should remain small, but must not be so small that every equipment drop is automatically better by being the only item.
- Provisional equipment must still be valid, data-driven, source-labeled, player-explainable, and comparison-ready.
- Debug items must not be mixed with normal MVP loot.
- “Temporary test item” is not allowed as a reason to bypass identity, display, stack, evidence, modifier, presentation, accessibility, or comparison validation.
- OpenMir2 authenticity gates decide whether a field may be called authentic; they do not block clearly labeled provisional content needed to prove the Phase 1 loot loop.

### States and Transitions

#### 1. Definition Lifecycle States

| State | Meaning | Gameplay Use |
|---|---|---|
| `draft_definition` | Authored but not schema-validated. | Not usable by gameplay. |
| `config_row_loaded` | Raw row/resource loaded from config/authoring source. | Not yet spawnable. |
| `validated_definition` | Passed all required validation gates. | May be referenced by gameplay if active. |
| `spawn_eligible_reference` | Valid, active, and allowed for normal drop/spawn/inventory flows. | Drop tables may reference. |
| `debug_only_definition` | Valid but restricted to debug/test tools. | Not normal loot. |
| `invalid_definition` | Failed validation. | Blocked from gameplay. |
| `deprecated_definition` | Previously valid but no longer used for new content. | Existing saves may require migration. |
| `blocked_unconfirmed_definition` | Intended OpenMir2 mapping lacks required evidence. | Not authentic implementation authority. |

#### 2. Instance Lifecycle States

| State | Meaning | Owner |
|---|---|---|
| `instance_requested` | A system requests creation of item instance or stack from `item_id`. | Drop/pickup/inventory/equipment caller. |
| `spawnable_instance` | Definition was valid and instance fields pass requirements. | Item instance factory / data layer. |
| `ground_drop_instance` | Concrete dropped item exists in world/drop state. | Drop and pickup system. |
| `inventory_stack_or_instance` | Item is held in inventory. | Inventory system. |
| `equipped_instance` | Equipment instance is in an equipment slot. | Equipment system. |
| `consumed_or_removed` | Stack/unit/instance has been removed by use, equip replacement, pickup merge, debug, or migration. | Owning gameplay system. |
| `orphaned_invalid_instance` | Instance references missing/invalid/deprecated definition without migration. | Save/load/debug recovery. |

#### 3. Definition Transition Rules

| From | To | Trigger | Rule |
|---|---|---|---|
| `draft_definition` | `config_row_loaded` | Config/authoring source loaded. | Raw data exists but is not trusted. |
| `config_row_loaded` | `validated_definition` | Schema and semantic validation pass. | Required fields, type rules, stack rules, evidence labels, and modifier payload all pass. |
| `config_row_loaded` | `invalid_definition` | Validation fails. | Must include structured failure reasons. |
| `validated_definition` | `spawn_eligible_reference` | Active gameplay flag enabled. | Debug-only, deprecated, or blocked-evidence rows cannot become spawnable. |
| `validated_definition` | `debug_only_definition` | Debug restriction flag enabled. | Valid for tests/tools only. |
| `validated_definition` | `deprecated_definition` | Content revision retires item. | New normal drops blocked; existing references require migration/fallback policy. |
| `blocked_unconfirmed_definition` | `validated_definition` | Evidence supplied or row relabeled provisional. | OpenMir2-authentic claims require E3/E4; provisional relabel allowed if design accepts non-authentic value. |
| Any non-draft | `invalid_definition` | Later validation detects broken reference/config. | Gameplay must stop spawning it; existing instances become recovery/migration cases. |

#### 4. Instance Transition Rules

| From | To | Trigger | Rule |
|---|---|---|---|
| `instance_requested` | `spawnable_instance` | Valid active definition found. | Stack/equipment instance requirements validated. |
| `instance_requested` | blocked failure | Missing, invalid, deprecated, or debug-only definition requested by normal gameplay. | Caller receives structured failure. |
| `spawnable_instance` | `ground_drop_instance` | Drop system creates world drop. | Drop system owns position, lifetime, pickup eligibility, y-sort/placement integration. |
| `ground_drop_instance` | `inventory_stack_or_instance` | Pickup succeeds. | Inventory owns merge/split/capacity; item definition supplies stack limits. |
| `inventory_stack_or_instance` | `equipped_instance` | Equip transaction succeeds. | Equipment owns slot occupancy and legality; Attributes owns final stat rebuild. |
| `equipped_instance` | `inventory_stack_or_instance` | Unequip/replacement succeeds. | Equipment removes modifier source in same transaction boundary. |
| Any live instance | `consumed_or_removed` | Use, deletion, merge to zero, debug cleanup, migration. | Quantity cannot remain zero in live inventory/world state. |
| Any live instance | `orphaned_invalid_instance` | Load or config mismatch references unavailable definition. | Normal gameplay blocked until migration/recovery. |

#### 5. State Invariants

- Only `spawn_eligible_reference` may be used by normal drop tables.
- Only `validated_definition` or `spawn_eligible_reference` may be displayed in normal inventory/equipment UI.
- `debug_only_definition` requires debug/test context.
- `invalid_definition` must never create live player-facing instances.
- `deprecated_definition` may load only through explicit migration or fallback policy.
- A live non-stackable equipment instance must have a stable instance ID.
- A live stackable item must have positive quantity within stack rules.
- An equipped item’s modifier contribution must be derived from its item definition and instance reference through Equipment; item definition itself does not mutate Attributes.

### Interactions with Other Systems

#### 1. Drop Table System

Data flow：

1. Drop table references `item_id`.
2. Drop table validation queries item definition status.
3. Only `spawn_eligible_reference` rows are valid for normal drops.
4. Drop table owns chance, weight, source monster, quantity roll, guarantees and pool selection.
5. Item definition supplies type, quality, display metadata, stack policy, and instance requirements.

Rules：

- Drop table must not duplicate display name, icon, quality, stackability, or equipment modifiers.
- Drop table must not reference `invalid`, `deprecated`, `debug_only`, or `blocked_unconfirmed` definitions for normal player loot.
- Quantity generation must respect stack rules but is not owned by item definition.
- If a referenced item is invalid, the drop table should fail validation rather than silently skipping the item unless a debug-only test explicitly requests skip behavior.

#### 2. Drop and Pickup System

Data flow：

1. Monster death or debug command requests ground drop using `item_id` and quantity/instance request.
2. Item definition validates whether the item can create a ground item record.
3. Drop/pickup system owns world position, map placement, pickup range, pickup timing, despawn and pickup candidate selection.
4. Ground presentation reads display metadata from item definition.
5. Pickup attempts transfer item to inventory according to inventory rules.

Rules：

- Item definition does not decide whether a cell can hold an item; map coordinate/blocking system owns placement validity.
- Item definition does not decide pickup radius or click priority.
- Ground drop should store `item_id`, definition version, quantity or instance ID, and source context for debug/save.
- If definition is missing after load, the ground drop becomes invalid/recovery state rather than becoming a placeholder loot item silently.

#### 3. Inventory System

Data flow：

1. Inventory receives item instance or stack request from pickup/debug/load.
2. Inventory queries item definition for stack policy, max stack size, item type, quality, icon/display keys, and whether the item can exist as an inventory record.
3. Inventory owns capacity, slot placement, sorting, merge/split, drag/drop and item selection.
4. Inventory passes equipment candidates to Equipment, not directly to Attributes.

Rules：

- Inventory must not duplicate stack limits as local truth.
- Inventory may cache display data only as a view cache; item definition remains source of truth.
- Inventory cannot treat a non-equipment item as equipable merely because it has a stat-like description.
- Inventory cannot mutate item definition fields.
- Inventory save payload should store item references and instance fields, not copied template truth as authoritative data.

#### 4. Equipment System

Data flow：

1. Player selects an inventory item to equip.
2. Equipment queries item definition for equipment data.
3. Equipment owns equip legality, slot selection, replacement, transaction order and active equipped source set.
4. Equipment resolves item definition modifier rows plus instance identity into active modifier sources.
5. Character Attributes validates and aggregates those modifier sources.

Rules：

- Item definition declares equipment payload; Equipment decides whether and where it can be equipped.
- Equipment must not invent modifiers absent from item definition unless a later affix/enchant system explicitly supplies instance modifiers.
- Equipment must not calculate final effective stats.
- Equipment replacement must be atomic from the attribute consumer perspective: no state where old and new equipment both contribute unless explicitly intended by later multi-slot rules.
- Equipment preview may use item definition modifier payload, but preview authority belongs to the approved Character Attributes / Equipment preview path.

#### 5. Character Attributes System

Data flow：

1. Equipment submits resolved active modifier sources derived from equipped item instances.
2. Character Attributes validates target stat IDs, operation, source identity, source status, numeric bounds and duplicate source keys.
3. Character Attributes computes effective stats, snapshot, delta and combat power if enabled.
4. Item definition never receives or stores final attribute outputs.

Rules：

- Item definition may target only modifier-targetable stat IDs from Character Attributes.
- `add_flat` is the only allowed Phase 1 modifier operation.
- `health_current` and `mana_current` are not valid item modifier targets.
- Combat power is display-only and owned by Character Attributes; item definition must not use combat power for item valuation, equip legality or damage authority.
- If Character Attributes rejects a modifier, the equip/preview flow must report structured failure; item definition must not fallback to raw UI-only numbers.

#### 6. UI / Tooltip / Equipment Comparison

Data flow：

1. UI reads display metadata from item definition.
2. UI reads inventory/equipment state from Inventory/Equipment.
3. UI reads stat deltas, preview results or combat power delta from Character Attributes.
4. UI combines these views for presentation but does not become authority.

Rules：

- UI must display localized text through display keys, not raw item IDs.
- UI may use quality as display salience, but quality does not imply stats unless another system provides stat delta.
- Equipment tooltip may show declared modifier rows as item facts if they are valid and player-visible.
- Equipment comparison must use Character Attributes preview/delta output for final before/after values.
- UI must not compute effective stats, combat power, equip legality, stack merge or drop value.
- Invalid/stale item definitions must show player-safe unavailable states; raw validation failures are debug-only.

#### 7. Save/Load System

Data flow：

1. Save stores item instances/stacks as references to `item_id`, definition version, quantity, instance ID, owner/container context and allowed instance fields.
2. Save does not store copied item template data as truth.
3. Load resolves each item reference against current validated definitions.
4. Missing/deprecated/incompatible definitions trigger migration, fallback or blocked-load state.

Rules：

- Persisted item display names, icons, type, quality, modifiers, or stack rules are debug snapshots only if stored at all.
- Loading a non-stackable item without a required instance ID is invalid unless migration explicitly repairs it.
- Loading stack quantity outside legal bounds requires Inventory/Save migration policy; item definition only supplies legal bounds.
- Loading an equipped item must rebuild Equipment modifier sources and then Character Attributes snapshot; saved final stats are not authoritative.

#### 8. OpenMir2 Behavior Mapping Spike

Data flow：

1. OpenMir2 Spike provides evidence IDs/source locations for item categories, fields, slots, instance structure, stack behavior and stat mappings.
2. Item definition system may upgrade fields from `mvp_provisional` or `openmir2_evidence_pending` to `openmir2_verified`.
3. Systems consuming item definitions can rely on `openmir2_verified` only when evidence references are present.

Rules：

- Until specific `StdItem` / `UserItem` evidence reaches E3/E4, OpenMir2 field names, slot IDs, type IDs and numeric values remain unconfirmed.
- Provisional rows must not use “authentic” wording in player-facing or implementation docs.
- If evidence contradicts MVP provisional assumptions, revise or deprecate definitions rather than silently reusing old IDs with new meanings.
- OpenMir2 Spike does not directly mutate item definitions; design review accepts the mapping before implementation treats it as authority.

#### 9. Debug Tools and Validation Reports

Data flow：

1. Debug tools query normalized item definition table.
2. They can inspect raw config row, normalized validated row, validation failures, source/evidence labels and downstream references.
3. Debug spawn tools may request debug-only items only in debug/test context.

Rules：

- Debug tools may display raw `item_id`, validation reason codes, evidence refs, modifier row IDs and config versions.
- Debug tools must clearly distinguish `spawnable`, `debug_only`, `deprecated`, `invalid` and `blocked_unconfirmed`.
- Debug spawn of invalid definitions should be blocked by default; if a developer override exists later, it must be visibly marked and must not contaminate normal saves.
- Validation output must be deterministic so tests and QA can compare expected failure reasons.

## Formulas

物品定义系统的公式只用于定义验证边界、引用可解析性、堆叠合法性、modifier payload 可交付性和 spawn eligibility。它们不计算掉率、拾取距离、背包容量、装备事务合法性、最终有效属性、combat power、商店价格、伤害、DPS、TTK 或 UI 布局。

Until implementation ADRs bind concrete integer representation, validator profiles must provide explicit configured values for `technical_safe_int_max`, modifier safe range, supported definition versions, and runtime profile gates. If those configured bounds are absent, validation fails deterministically instead of assuming engine defaults.

### 1. `stack_quantity_valid`

The `stack_quantity_valid` formula is defined as:

`stack_quantity_valid = quantity_is_integer AND quantity >= 1 AND ((stack_policy = non_stackable AND max_stack_size = 1 AND quantity = 1) OR (stack_policy = stackable AND max_stack_size > 1 AND quantity <= max_stack_size))`

**Variables:**

| Variable | Symbol | Type | Range | Description |
|---|---|---|---|---|
| Stack quantity valid | `stack_quantity_valid` | bool | `false–true` | Result: true only when the candidate quantity is legal for the item definition stack policy and max stack size. |
| Stack policy | `stack_policy` | enum | `non_stackable`, `stackable` | Declares whether instances of this item definition may share one stack. |
| Max stack size | `max_stack_size` | int | `1–technical_safe_int_max` | Maximum quantity allowed in one stack for this item definition. |
| Quantity | `quantity` | int | `0–technical_safe_int_max` before validation | Candidate stack quantity being validated. |
| Quantity is integer | `quantity_is_integer` | bool | `false–true` | True only if the candidate quantity is an integer value, not fractional, missing, NaN, or malformed. |

**Output Range:** `false` or `true`. Invalid stack quantities are rejected rather than clamped because clamping would silently change item counts and could hide data or transaction bugs.

**Example:** A material item has `stack_policy = stackable`, `max_stack_size = 99`, `quantity = 20`, and `quantity_is_integer = true`.

`stack_quantity_valid = true AND 20 >= 1 AND ((false) OR (true AND 99 > 1 AND 20 <= 99)) = true`

A sword has `stack_policy = non_stackable`, `max_stack_size = 1`, and `quantity = 2`.

`stack_quantity_valid = true AND 2 >= 1 AND (true AND 1 = 1 AND 2 = 1) = false`

### 2. `modifier_payload_row_valid`

The `modifier_payload_row_valid` formula is defined as:

`modifier_payload_row_valid = modifier_row_present AND modifier_row_enabled AND modifier_op = add_flat AND target_stat_known AND target_stat_modifier_targetable AND NOT target_stat_current_resource AND modifier_value_is_integer AND modifier_value_within_safe_range`

**Variables:**

| Variable | Symbol | Type | Range | Description |
|---|---|---|---|---|
| Modifier payload row valid | `modifier_payload_row_valid` | bool | `false–true` | Result: true only when one modifier row is active, allowed, targetable, non-current-resource, integer, and in safe range. |
| Modifier row present | `modifier_row_present` | bool | `false–true` | True when the item definition contains a modifier payload row to validate. |
| Modifier row enabled | `modifier_row_enabled` | bool | `false–true` | True when the row is active for runtime use. Unknown, reserved, deprecated, or inactive rows are not valid runtime payload. |
| Modifier operation | `modifier_op` | enum | `add_flat`, `reserved`, `unknown` | Operation requested by the payload row. Phase 1 only permits `add_flat`. |
| Target stat known | `target_stat_known` | bool | `false–true` | True when `target_stat_id` resolves to a known Character Attributes stat definition. |
| Target stat modifier-targetable | `target_stat_modifier_targetable` | bool | `false–true` | True when Character Attributes declares the target stat may receive item/equipment modifiers. |
| Target stat is current resource | `target_stat_current_resource` | bool | `false–true` | True for current resource fields such as `health_current` or `mana_current`; these are forbidden modifier targets. |
| Modifier value is integer | `modifier_value_is_integer` | bool | `false–true` | True only if the modifier value is an integer payload value. |
| Modifier value within safe range | `modifier_value_within_safe_range` | bool | `false–true` | True when the value is inside the project technical safe integer range for data validation. |

**Output Range:** `false` or `true`. Invalid rows are not coerced, downgraded, or partially applied because Item Definition is responsible for clean modifier payload boundaries.

**Example:** A weapon modifier row is present and enabled, with `modifier_op = add_flat`, target stat `physical_attack_min`, and value `3`. Character Attributes declares the stat known and modifier-targetable, and it is not a current resource.

`modifier_payload_row_valid = true AND true AND true AND true AND true AND NOT false AND true AND true = true`

A row targeting `health_current` fails even if the value is otherwise valid:

`modifier_payload_row_valid = true AND true AND true AND true AND true AND NOT true AND true AND true = false`

### 3. `modifier_payload_valid`

The `modifier_payload_valid` formula is defined as:

`modifier_payload_valid = (modifier_row_count = 0 AND item_type != equipment) OR (item_type = equipment AND modifier_row_count >= equipment_min_modifier_rows AND modifier_row_count <= equipment_max_modifier_rows AND all_modifier_payload_rows_valid)`

Where:

`all_modifier_payload_rows_valid = every(modifier_payload_row_valid_i = true for i in modifier_rows)`

**Variables:**

| Variable | Symbol | Type | Range | Description |
|---|---|---|---|---|
| Modifier payload valid | `modifier_payload_valid` | bool | `false–true` | Result: true only when the item type and all modifier rows satisfy the Phase 1 modifier payload contract. |
| Item type | `item_type` | enum | `equipment`, `material`, `currency`, `consumable`, `quest`, `debug`, `unknown` | Classification of the item definition. |
| Modifier row count | `modifier_row_count` | int | `0–technical_safe_int_max` | Number of modifier payload rows on the item definition. |
| Equipment minimum modifier rows | `equipment_min_modifier_rows` | int | `0–technical_safe_int_max` | Configurable lower bound for equipment modifier rows. MVP may use `0` if blank equipment is allowed, or `1` if every equipment candidate must affect Attributes. |
| Equipment maximum modifier rows | `equipment_max_modifier_rows` | int | `0–technical_safe_int_max` | Configurable upper bound for modifier rows allowed on one equipment definition. |
| Modifier payload row valid | `modifier_payload_row_valid_i` | bool | `false–true` | Validation result for each modifier row using `modifier_payload_row_valid`. |
| All modifier payload rows valid | `all_modifier_payload_rows_valid` | bool | `false–true` | True only when every modifier row on this definition is valid. For zero rows, this is true by empty-set convention but still gated by item type and row-count rules. |

**Output Range:** `false` or `true`. Any invalid row invalidates the whole modifier payload because downstream systems should not need to inspect partially trusted item-definition data.

**Example:** An equipment definition has `modifier_row_count = 2`, `equipment_min_modifier_rows = 0`, `equipment_max_modifier_rows = 4`, and both rows pass `modifier_payload_row_valid`.

`modifier_payload_valid = false OR (true AND 2 >= 0 AND 2 <= 4 AND true) = true`

A material definition has no modifier rows:

`modifier_payload_valid = (0 = 0 AND material != equipment) OR false = true`

A material definition with one modifier row fails because modifier payloads belong to equipment candidates in Phase 1:

`modifier_payload_valid = (1 = 0 AND material != equipment) OR false = false`

### 4. `item_reference_lookup_resolvable`

The `item_reference_lookup_resolvable` formula is defined as:

`item_reference_lookup_resolvable = item_id_present AND item_id_format_valid AND item_id_registered_in_runtime_definitions AND referenced_definition_version_supported`

This formula answers only whether a reference can be looked up in the normalized runtime table. It does **not** decide whether the referenced definition may spawn, equip, display normally, or load as legacy content.

**Variables:**

| Variable | Symbol | Type | Range | Description |
|---|---|---|---|---|
| Item reference lookup resolvable | `item_reference_lookup_resolvable` | bool plus structured reason set | `false–true`; reasons sorted deterministically | Result: true only when the reference can locate a supported runtime definition row. |
| Item ID present | `item_id_present` | bool | `false–true` | True when a downstream reference provides an item ID. |
| Item ID format valid | `item_id_format_valid` | bool | `false–true` | True when the item ID matches the project stable item ID format. |
| Item ID registered in runtime definitions | `item_id_registered_in_runtime_definitions` | bool | `false–true` | True when the normalized runtime item-definition index contains this item ID. |
| Referenced definition version supported | `referenced_definition_version_supported` | bool | `false–true` | True when the referenced `definition_version` is loadable by the current data schema and migration policy. |

**Output Range:** status-bearing validation result with `passed = false/true` and ordered structured reasons. Deprecated or debug-only definitions may still be lookup-resolvable; later profile gates decide their use.

**Example:** A loot result references `item_id = mvp_bronze_sword` with a supported definition version. The ID is present, format-valid, registered, and supported.

`item_reference_lookup_resolvable = true AND true AND true AND true = true`

A missing item ID fails lookup:

`item_reference_lookup_resolvable = false AND false AND false AND false = false`

### 5. `item_definition_semantically_valid`

The `item_definition_semantically_valid` formula is defined as:

`item_definition_semantically_valid = identity_valid AND display_metadata_valid AND item_type_valid AND quality_valid AND stack_policy_valid AND equipment_stack_policy_valid AND equipment_data_valid AND modifier_payload_valid AND evidence_labels_valid AND player_facing_metadata_valid`

Where suggested child gates are:

`identity_valid = item_id_present AND item_id_format_valid AND item_id_unique AND definition_version_present AND definition_version_supported`

`stack_policy_valid = stack_policy_config_present AND max_stack_size_is_integer AND max_stack_size_within_safe_range AND ((stack_policy = non_stackable AND max_stack_size = 1) OR (stack_policy = stackable AND max_stack_size > 1))`

`equipment_stack_policy_valid = (item_type != equipment) OR (stack_policy = non_stackable AND max_stack_size = 1)`

`equipment_data_valid = (item_type != equipment AND equipment_data_absent) OR (item_type = equipment AND equipment_data_present AND equipment_slot_candidate_valid AND normal_equipment_comparison_ready)`

**Variables:**

| Variable | Symbol | Type | Range | Description |
|---|---|---|---|---|
| Item definition semantically valid | `item_definition_semantically_valid` | bool plus structured reason set | `false–true`; reasons sorted deterministically | Result: true only when the definition row is structurally and semantically valid independent of runtime profile. |
| Identity valid | `identity_valid` | bool | `false–true` | True when stable `item_id`, uniqueness, and supported `definition_version` checks pass. |
| Display metadata valid | `display_metadata_valid` | bool | `false–true` | True when required display metadata fields are present and schema-valid. |
| Item type valid | `item_type_valid` | bool | `false–true` | True when `item_type` is recognized by the schema. |
| Quality valid | `quality_valid` | bool | `false–true` | True when `quality_id` resolves to an allowed quality classification. |
| Stack policy valid | `stack_policy_valid` | bool | `false–true` | True when stack fields are parseable and internally consistent. |
| Equipment stack policy valid | `equipment_stack_policy_valid` | bool | `false–true` | True when equipment rows are non-stackable in Phase 1. |
| Equipment data valid | `equipment_data_valid` | bool | `false–true` | True when equipment definitions include valid candidate data and comparison readiness. |
| Modifier payload valid | `modifier_payload_valid` | bool | `false–true` | True when modifier rows are valid for this item type using `modifier_payload_valid`. |
| Evidence labels valid | `evidence_labels_valid` | bool | `false–true` | True when required source/evidence labels are present and recognized. |
| Player-facing metadata valid | `player_facing_metadata_valid` | bool | `false–true` | True when normal spawnable content has required type/quality labels, world visual/audio tokens, salience, and fallback keys. |

**Output Range:** status-bearing validation result with `passed = false/true` and ordered structured reasons. Semantic validity is not the same as normal spawnability; debug-only or deprecated definitions may be semantically valid but fail a later profile gate.

**Example:** A Phase 1 material definition has valid identity, display/type/quality labels, stack policy, no equipment data, no modifier rows, evidence labels, and player-facing metadata.

`item_definition_semantically_valid = true AND true AND true AND true AND true AND true AND true AND true AND true AND true = true`

An equipment definition with valid identity but stackable policy fails:

`item_definition_semantically_valid = true AND true AND true AND true AND true AND false AND true AND true AND true AND true = false`

### 6. `definition_profile_eligible`

The `definition_profile_eligible` formula is defined as:

`definition_profile_eligible = item_definition_semantically_valid AND lifecycle_state_allowed_for_profile AND item_type_allowed_for_profile AND source_status_allowed_for_profile AND NOT debug_hidden_in_profile AND NOT blocked_unconfirmed_in_profile`

**Variables:**

| Variable | Symbol | Type | Range | Description |
|---|---|---|---|---|
| Definition profile eligible | `definition_profile_eligible` | bool plus structured reason set | `false–true`; reasons sorted deterministically | Result: true only when a semantically valid definition is allowed for the active validation/runtime profile. |
| Item definition semantically valid | `item_definition_semantically_valid` | bool | `false–true` | True when semantic definition validation passes. |
| Lifecycle state allowed for profile | `lifecycle_state_allowed_for_profile` | bool | `false–true` | True when active/debug/deprecated/blocked status is allowed for the current profile/use case. |
| Item type allowed for profile | `item_type_allowed_for_profile` | bool | `false–true` | True when equipment/material/currency/reserved/debug type is allowed by the current profile. |
| Source status allowed for profile | `source_status_allowed_for_profile` | bool | `false–true` | True when provisional/project-local/verified/pending/debug source status is allowed by the current profile. |
| Debug hidden in profile | `debug_hidden_in_profile` | bool | `false–true` | True when debug-only content is hidden from the active profile. |
| Blocked unconfirmed in profile | `blocked_unconfirmed_in_profile` | bool | `false–true` | True when evidence-pending unconfirmed content is blocked for this profile. |

**Output Range:** status-bearing validation result with `passed = false/true` and ordered structured reasons.

**Example:** A semantically valid active equipment item in the normal spawn profile passes if its lifecycle, type, and source status are allowed and it is not debug-hidden.

`definition_profile_eligible = true AND true AND true AND true AND NOT false AND NOT false = true`

A semantically valid debug-only item in normal runtime fails eligibility:

`definition_profile_eligible = true AND false AND true AND true AND NOT true AND NOT false = false`

### 7. `spawn_eligible_reference`

The `spawn_eligible_reference` formula is defined as:

`spawn_eligible_reference = item_reference_lookup_resolvable AND referenced_definition_profile_eligible AND spawn_profile_allows_item_type AND NOT referenced_definition_deprecated_for_new_spawn`

**Variables:**

| Variable | Symbol | Type | Range | Description |
|---|---|---|---|---|
| Spawn eligible reference | `spawn_eligible_reference` | bool plus structured reason set | `false–true`; reasons sorted deterministically | Result: true only when an external drop/spawn reference resolves to a definition eligible for normal spawning. |
| Item reference lookup resolvable | `item_reference_lookup_resolvable` | bool | `false–true` | True when the reference can locate a supported runtime definition row. |
| Referenced definition profile eligible | `referenced_definition_profile_eligible` | bool | `false–true` | True when the referenced definition is eligible for the active spawn profile. |
| Spawn profile allows item type | `spawn_profile_allows_item_type` | bool | `false–true` | True when the spawn/drop profile allows this item type. |
| Referenced definition deprecated for new spawn | `referenced_definition_deprecated_for_new_spawn` | bool | `false–true` | True when the definition may be legacy-readable but may not enter new normal drops. |

**Output Range:** status-bearing validation result with `passed = false/true` and ordered structured reasons. It does not define drop chance, timing, placement, or quantity.

**Example:** A normal active material reference resolves, its definition is eligible, its type is allowed, and it is not deprecated for new spawn.

`spawn_eligible_reference = true AND true AND true AND NOT false = true`

A deprecated item may be save-load-resolvable but not spawn-eligible:

`spawn_eligible_reference = true AND false AND true AND NOT true = false`

### Formula Boundary Notes

Cross-system formula facts introduced here include `stack_quantity_valid`, `modifier_payload_row_valid`, `modifier_payload_valid`, `item_reference_lookup_resolvable`, `item_definition_semantically_valid`, `definition_profile_eligible`, and `spawn_eligible_reference`. These must stay synchronized with `design/registry/entities.yaml` because downstream GDDs such as drop tables, pickup, inventory, equipment, save/load, UI and debug tools may reference them by name.

Downstream systems may consume a lookup-resolvable `item_id`, semantic validity status, profile eligibility status, stack policy, equipment candidate data and filtered modifier facts, but they own their own decisions:

- Drop systems own drop tables and drop probabilities.
- Ground item systems own placement, lifetime and pickup targeting.
- Inventory systems own capacity, slot placement, stack merge transactions and item movement.
- Equipment systems own equip legality and transaction ordering.
- Character Attributes owns final effective stats, current resource behavior, attribute deltas and `combat_power`.
- Economy systems own prices and sell/buy/recycle rules.
- UI systems own presentation layout and comparison display.

## Edge Cases

- **If an item definition is missing `item_id`**: the definition is invalid and must not be spawnable, referenced by drop tables, loaded into inventory fixtures, equipped, saved, or shown as a normal item. `item_id` is the stable cross-system identity.
- **If two active item definitions share the same `item_id`**: both definitions are invalid until the duplicate is resolved; the system must not choose one arbitrarily. Duplicate IDs create non-deterministic references for drops, saves, inventory entries, and equipment comparisons.
- **If two definitions share a display name but have different `item_id` values**: both definitions may remain valid if their `item_id` values are unique. Display names are player-facing metadata and must not be used as authoritative identity.
- **If a cross-system reference uses display name, array index, file path, or Resource instance identity instead of `item_id`**: the reference is invalid for durable gameplay logic. These identifiers are unstable across localization, content edits, exports, or Resource duplication.
- **If an item reference points to a missing definition**: the reference is unresolved and the referencing system must treat it as invalid data, not as an empty item. For Phase 1, this blocks spawnability and surfaces validation evidence.
- **If an item reference points to a deprecated definition**: the definition may still resolve for save/load compatibility, but it must not be considered spawnable unless explicitly marked as allowed for legacy/debug use. Deprecated items are readable history, not normal content.
- **If an item reference points to an inactive or `blocked_unconfirmed` definition**: the reference may resolve for diagnostics, but the item must not be generated by drops, fixtures, or normal gameplay. Unknown readiness fails closed.
- **If a save file references an `item_id` that no longer exists**: load must not crash; the item must become an unresolved/blocked item record or fail that load path with explicit evidence, depending on the owning save/load policy. The item definition system reports the reference as unresolved.
- **If a save file references an older `definition_version` than the current definition**: the item definition system must report a version mismatch and expose the saved and current versions for migration handling. It must not silently reinterpret changed modifier payloads as equivalent.
- **If multiple `definition_version` rows for the same `item_id` coexist in normalized runtime data**: normal content references must resolve through an explicit version or documented latest-active policy. Save/load references must preserve the saved version until migration resolves it; drop-table and fixture references should bind deterministic version policy so tests do not depend on row order.
- **If a save file references a newer `definition_version` than the client supports**: the item must not be treated as valid normal content. Forward migration cannot be assumed.
- **If a definition migration fails**: the item must not be silently upgraded. The result must be either a blocked load for that item or an unresolved placeholder record with evidence identifying `item_id`, source version, target version, and reason.
- **If a definition changes modifier payload values without incrementing `definition_version`**: the definition is a versioning error. Save/load and migration behavior depend on version changes being explicit.
- **If a definition changes `item_type`, stack policy, or equipment semantics under the same `item_id` without migration notes**: the definition must be flagged for review because these fields affect stack rules, equipment routing, inventory behavior, and save interpretation.
- **If an existing equipment definition is converted into a material under the same `item_id`**: the change is a breaking semantic migration and must not be silently accepted. Existing saves or references may interpret the old item as equipment.
- **If an existing stackable definition is converted into non-stackable under the same `item_id`**: the change requires migration handling for any saved stacks with quantity greater than `1`.
- **If an existing non-stackable definition is converted into stackable under the same `item_id`**: the change requires migration review because saved separate instances may now be mergeable, but merging is not owned by this system.
- **If an item stack quantity is missing**: the quantity is invalid unless the owning inventory/save system explicitly supplies a default before validation. Stack validation requires a concrete integer quantity.
- **If an item stack quantity is less than `1`**: the stack instance is invalid. Quantity `0`, negative values, and empty stacks are not valid live item instances in Phase 1.
- **If an item stack quantity is non-integer**: the stack instance is invalid. Partial items are not supported by the MVP inventory/equipment loop.
- **If a non-stackable item has quantity greater than `1`**: the stack instance is invalid. Multiple copies must be represented as separate item instances.
- **If a stackable item has quantity greater than `max_stack_size`**: the single stack is invalid. Splitting, merging, or overflow placement belongs to Inventory, not Item Definition.
- **If a stackable item has `max_stack_size <= 1`**: the definition is invalid. A stackable definition must have `max_stack_size > 1`.
- **If a non-stackable item has `max_stack_size != 1`**: the definition is invalid. Non-stackable definitions must have exactly one item per stack.
- **If an equipment definition is marked stackable or has `max_stack_size > 1`**: the definition is invalid. Equipment is non-stackable in Phase 1 so each item can be compared, equipped, saved, and extended independently.
- **If a material definition is non-stackable**: the definition may be valid only if intentionally configured; it should produce validation evidence for review. Materials are expected to be stackable in MVP unless a specific design reason exists.
- **If a currency definition is enabled in Phase 1**: it must follow the same stack validation rules as other stackable items. Currency is optional and must not introduce special stacking semantics inside Item Definition.
- **If a consumable, quest, or other reserved item type appears in Phase 1 content**: the definition is not normal-spawnable unless explicitly marked as reserved/test-only. Reserved item types are not part of the Foundation / MVP gameplay loop.
- **If an item definition has an unknown `item_type`**: the definition is invalid. Unknown types cannot be safely routed to equipment comparison, inventory display, stack rules, or future systems.
- **If an equipment item has no equipment candidate data**: the definition is invalid for equipment usage. Equipment candidate data is required so downstream equip and comparison systems can interpret the item as equipment.
- **If a non-equipment item has equipment candidate data or modifier payload rows**: the definition is invalid in Phase 1. Non-equipment equipment behavior is out of scope and would blur ownership between Item Definition, Inventory, Equipment, and Attributes.
- **If an equipment item has no modifier payload rows**: the definition may remain valid if intentionally configured as cosmetic or placeholder equipment, but it should produce validation evidence for review. This avoids accidentally creating equipment with no gameplay effect.
- **If a modifier payload row uses an operation other than `add_flat`**: the row is invalid. Phase 1 supports only flat additive modifiers to keep attribute math deterministic and testable.
- **If a modifier payload row targets an unknown stat**: the row is invalid. Modifier targets must resolve to registered Character Attributes targets.
- **If a modifier payload row targets an inactive or non-modifier-targetable stat**: the row is invalid. A stat being known is not enough; it must also be explicitly allowed as a modifier target.
- **If a modifier payload row targets `health_current` or `mana_current`**: the row is invalid. Current resources are runtime state and must not be modified by item definition payloads.
- **If a modifier payload contains both valid and invalid rows**: the whole modifier payload is invalid. Partial modifier application is not allowed because it would make item power unclear and hide data errors.
- **If a modifier payload row is missing its target stat or numeric value**: the row is invalid. Every row must identify the exact Character Attributes target and deterministic value.
- **If a modifier payload row has a non-numeric value**: the row is invalid. Modifier values must be numeric so Character Attributes can apply them without type coercion.
- **If a modifier payload row has value `0`**: the row may be valid only if zero-value modifiers are explicitly allowed as placeholder/test data; otherwise it should produce validation evidence for review. Zero-value rows are usually accidental content noise.
- **If a modifier payload row attempts to define `combat_power` directly**: the row is invalid. `combat_power` is display-only and owned by Character Attributes per ADR-0014.
- **If `quality_id` is unknown**: the definition is invalid because quality metadata must resolve to a known quality entry.
- **If `quality_id` is `debug` on a normal gameplay item**: the definition must not be spawnable in normal Phase 1 content. Debug quality is hidden/test-only metadata.
- **If an item definition uses quality to modify stat values**: the definition is invalid unless the resulting stat values are explicitly represented in modifier payload rows. `quality_id` is metadata only and must not implicitly alter power.
- **If an item definition uses quality to determine drop rate, price, equip legality, or combat power**: that behavior is out of scope for this system and must be moved to the owning drop table, economy, equipment, or Character Attributes/UI system. Quality may be cited as input only if that downstream GDD defines the rule.
- **If a debug-only item is referenced by a normal drop table**: the drop table reference is invalid for normal gameplay. Debug-only content must not leak into the player-facing loot loop.
- **If a debug-only item is present in a developer fixture**: the item may be valid only if the fixture/source is explicitly marked debug or test. Debug items require explicit source labeling.
- **If a debug-only item appears in a player save without debug context**: the item must be reported as debug leakage or unresolved restricted content. It must not silently become normal player inventory.
- **If display metadata is missing for an otherwise valid item**: the definition is not player-display-ready and must not be used in normal UI-facing content. UI may show raw `item_id` only as error/debug fallback.
- **If icon or world visual metadata is missing**: the item may remain mechanically valid, but it is not UI/loot-feedback complete. UI should use a placeholder icon only as an explicit fallback, not as evidence that the definition is complete.
- **If a raw `item_id` fallback is shown to the player during normal Phase 1 play**: this is a content completeness issue and should fail player-facing polish acceptance. Raw IDs are allowed only to preserve stability during errors or development.
- **If a drop table references an invalid item definition**: the drop table entry is invalid and must not produce a normal drop. The drop system owns whether invalid entries are skipped, block the table, or surface validation failure.
- **If a drop table references a valid but non-spawnable item definition**: the reference is invalid for drop generation. Resolvable definitions are not automatically spawnable.
- **If inventory has no space for a valid spawned item**: Item Definition does not decide the outcome. Inventory overflow behavior belongs to Inventory and may choose reject, leave on ground, split stack, or another approved rule.
- **If a stackable pickup would exceed an existing stack’s `max_stack_size`**: Item Definition only declares the maximum valid stack size. Splitting or overflow placement belongs to Inventory.
- **If equipment comparison receives an invalid or unresolved item definition**: comparison must not calculate normal output from missing definition data. Item Definition exposes invalid/unresolved status.
- **If equip legality receives a valid item definition**: Item Definition still does not guarantee the item can be equipped by the current character. Equip legality, slot rules, class restrictions and transactions belong to Equipment.
- **If `spawn_eligible_reference = false` for a valid definition**: it may be referenced for legacy, debug, or test purposes, but must not appear in normal drop generation or normal player-facing fixtures.
- **If a definition is marked spawnable but fails `item_definition_semantically_valid`**: it is not spawnable. Validity is a prerequisite for spawnability.
- **If a placeholder item fallback is used for an unresolved item**: the placeholder must preserve the unresolved `item_id` and must not gain normal gameplay modifiers, price, quality power, equip behavior, or drop behavior. Placeholder fallback is for diagnostics/save tolerance, not content substitution.
- **If a placeholder item fallback would be equipped**: equip behavior must be blocked because the placeholder has no valid equipment candidate data. Item Definition must not fabricate equipment data.
- **If a placeholder item fallback would be dropped as loot**: the drop is invalid content. Placeholder items must not enter normal loot tables as replacements for missing definitions.
- **If validation evidence is missing for a definition**: the definition may be mechanically parseable but is not content-complete. Source/evidence labels are required for validation lifecycle tracking.
- **If source/evidence labels contradict the item’s actual fields**: the definition must be flagged for review. Evidence labels distinguish authentic, provisional, debug, and test content.
- **If OpenMir2 `StdItem` evidence conflicts with the current item definition**: the definition must be flagged as an evidence conflict and current behavior must be labeled provisional or intentionally divergent. The system must not present conflicted behavior as authentic.
- **If OpenMir2 `UserItem` evidence conflicts with current instance/save assumptions**: the affected save/load or instance behavior must be flagged as provisional. Item Definition may document the conflict but must not invent authenticity rules without evidence.
- **If OpenMir2 evidence is missing for an item field**: the field may use an MVP provisional value only if the source/evidence label says so. Missing evidence must not be hidden behind authoritative wording.
- **If later OpenMir2 evidence disproves an MVP provisional value**: the definition must be reviewed for migration impact before changing live IDs or modifier semantics. Authenticity updates can affect saves and downstream balance.
- **If a `.tres` Resource graph contains item data that differs from normalized runtime item definition data**: normalized item definition data wins. Per ADR-0013, mutable Resource graphs are not runtime truth.
- **If a `.tres` Resource instance is mutated at runtime**: the mutation must not be treated as a persistent item definition change. Runtime state belongs to item instances or owning systems, not shared definition Resources.
- **If two item instances share a mutable Resource reference**: that shared reference must not cause one item’s runtime changes to alter another item’s definition or modifiers. Per-instance variation must be stored outside the shared definition.
- **If a designer edits a `.tres` Resource but does not update the authoritative data source / normalization pipeline**: the change is not valid content until it is reflected in the authoritative item definition pipeline. This prevents editor-only drift.
- **If a definition includes future-facing fields for consumables, quests, prices, crafting, durability, binding, sockets, affixes, or enhancement**: those fields must not activate behavior in Phase 1. Future fields may be stored only as inert metadata if explicitly allowed by schema.
- **If an item definition attempts to own pickup radius, inventory capacity, equipment transaction rules, effective stats, combat power, drop probability, shop price, or UI layout**: that behavior is out of scope and must be rejected or moved to the owning system’s GDD.
- **If a material item has modifier payload rows intended for crafting or future systems**: the definition is invalid in Phase 1. Non-equipment modifiers are not supported until a later system explicitly owns and validates them.
- **If a definition is valid in isolation but fails because a dependency registry entry is missing**: the definition is invalid until the dependency is registered or the reference is removed. Cross-system references must resolve deterministically.
- **If Character Attributes removes, renames, or deactivates a stat targeted by existing item modifiers**: existing item definitions targeting that stat become invalid until migrated. Item Definition must not remap stat targets implicitly.
- **If Character Attributes marks a previously valid stat as no longer modifier-targetable**: any item modifier targeting that stat becomes invalid for new validation. Existing saves require migration handling rather than silent continued application.
- **If UI requests item data for an unresolved `item_id`**: Item Definition should return unresolved status plus the raw ID, not null data that causes UI failure. UI presentation of that fallback remains owned by UI.
- **If UI sorts or colors items by quality**: it may use `quality_id` as metadata only. UI must not imply quality-derived stat power unless actual modifier/effective stat data supports that comparison.
- **If equipment comparison treats quality as a power proxy**: that behavior is invalid for this system’s rules. Equipment comparison must use actual modifier/effective stat data owned by downstream systems.
- **If balance tuning attempts to change drop frequency by editing item definition quality**: the change is invalid as a drop tuning method. Drop frequency belongs to Drop Table.
- **If balance tuning attempts to change item power by editing only item definition quality**: the change has no mechanical effect under this system. Item power must be changed through modifier payload rows or downstream attribute rules.
- **If an invalid item definition is loaded during development**: the validation lifecycle must surface exact failing rules and source labels. Development builds may continue only if the invalid definition is excluded from spawnable/player-facing content.
- **If an invalid item definition is encountered in a release/content-locked build**: the safest expected outcome is to block the affected content path rather than allow undefined item behavior. Exact build failure policy may be owned by production/technical direction.
- **If a definition passes schema parsing but fails semantic validation**: it is invalid. Schema correctness alone is not enough; stack policy, item type, modifier payload, source labels and spawnability rules must also pass.
- **If a definition cannot be parsed at all**: it must not enter the item registry. Parse failures are data loading failures, not invalid gameplay items.
- **If validation order produces multiple errors for one definition**: the system should report all detectable independent errors where practical. This improves content authoring feedback.
- **If fixing one validation error would change whether another error applies**: the system should still report the clearest current failure state and revalidate after correction. Validation evidence is iterative until all rules pass.
- **If a test fixture needs intentionally invalid item data**: the fixture must be clearly marked as validation-test content and must not be part of normal gameplay data.
- **If a fixture uses debug items to test pickup, inventory, or equipment flows**: the debug source must be explicit so those items cannot be mistaken for player-facing loot content.
- **If a normal Phase 1 loot-loop test depends on debug-only item definitions**: the test setup is invalid unless the test is explicitly about debug behavior. Normal loop tests should use normal spawnable definitions.
- **If an item definition is removed while drop tables, fixtures, saves, tests, registry entries or GDDs still reference it**: those references become unresolved and must be updated or migrated. Removing a definition is a cross-system change.
- **If a definition’s display metadata changes but `item_id` and mechanics remain unchanged**: existing references remain valid and no gameplay migration is required. UI/cache refresh may be needed but is outside this system’s ownership.
- **If an item definition is valid but not yet approved for the lifecycle stage required by Phase 1**: it must not be included in normal Phase 1 spawnable content. Lifecycle state gates content readiness separately from raw validity.

## Dependencies

### Upstream Dependencies

| Dependency | Type | Status | Needed From It | Boundary |
|---|---|---|---|---|
| `Game Concept` | Hard | Exists | Phase 1 30 秒离线刷怪爆装切片目标、核心支柱、MVP scope、OpenMir2 source-first 原则。 | Item Definition must support loot readability and growth handoff, but must not expand into full MMO economy. |
| `Systems Index` | Hard | Exists | System priority, Foundation layer placement, downstream dependency map, and TD-SYSTEM-BOUNDARY note requiring `Owns / Reads / Writes / Emits / Listens`. | This GDD must keep item truth separate from drop, inventory, equipment, UI, persistence and economy behavior. |
| `OpenMir2 行为映射 Spike` | Hard for authenticity; Soft for MVP provisional slice | Exists but item-specific evidence remains open | `StdItem`, `UserItem`, item template, item instance, inventory operation and equipment operation evidence; E3/E4 contracts before any field is called OpenMir2-authentic. | Until E3/E4 item evidence exists, item fields, slot tags, stack behavior, modifiers and instance shape remain `mvp_provisional`, `project_local`, or `openmir2_evidence_pending`. |
| `角色属性系统` | Hard for equipment modifier payload | Approved for ADR authoring; ADRs Proposed | Modifier-targetable stat IDs, `add_flat` operation boundary, forbidden current-resource modifier targets, effective stat ownership, snapshot/delta/combat_power ownership. | Item Definition may declare modifier payload rows only; Character Attributes owns validation, aggregation, final stats, delta and combat power. |
| ADR-0013 Resource Authoring Boundary | Hard if `.tres` / Resource authoring is used | Proposed | `.tres` Resource graphs are authoring envelopes only; runtime truth must be normalized DTO/table data. | Item Definition implementation must not retain mutable Resource graphs as gameplay authority. |
| ADR-0014 Attribute Display Proxy Ownership | Hard for combat_power boundary | Proposed | `combat_power`, main stat, visible deltas and growth display proxy are Character Attributes display-only outputs. | Item Definition must not define combat_power, price, equip legality or item value from attribute display proxy. |
| Engine / Technical Preferences | Hard for implementation stage | Exists | Godot 4.6.3, GDScript, data-driven values, GUT testing, PC target, no hardcoded gameplay values. | GDD remains behavior/data-contract level; implementation API and Resource/file format require ADR/technical design before stories. |

### Downstream Dependencies

| Downstream System | Dependency Type | Reads From Item Definition | Item Definition Does Not Provide | Blocking / Staging Note |
|---|---|---|---|---|
| `掉落表系统` | Hard | `item_id`, `item_type`, `quality_id`, `stack_policy`, `max_stack_size`, `spawn_eligible_reference`, display/world visual metadata for validation. | Drop chance, drop weight, source monster, quantity roll, guarantees, pool selection. | Drop tables cannot produce normal loot from invalid, deprecated, debug-only, or non-spawnable definitions. |
| `掉落与拾取系统` | Hard | `item_id`, display metadata, `stack_quantity_valid`, spawnability, instance requirements. | Ground position, lifetime, pickup radius, click priority, pickup ownership, map placement. | Ground drops should store item reference/version and let map/pickup/inventory systems own runtime behavior. |
| `背包系统` | Hard | `item_id`, type, quality, icon/display keys, stack policy, max stack size, instance/stack requirements. | Capacity, slot layout, merge/split, sorting, drag/drop, overflow behavior. | Inventory must store references/instances and must not duplicate template truth as authority. |
| `装备系统` | Hard | Equipment category, candidate slot tags, requirement payload if any, modifier payload rows, main comparison hint. | Equip legality, slot occupancy, replacement transaction, active equipped set, final stat rebuild. | Equipment resolves item definitions into active modifier sources and submits them to Character Attributes. |
| `背包 / 装备 UI 系统` | Hard for UI content | `display_name_key`, `description_key`, `icon_key`, `quality_id`, item type, equipment metadata, player-visible modifier hints. | Tooltip layout, final localized text, colors, comparison layout, input flow, drag/drop UX. | UI must use Character Attributes preview/delta for final before/after values; raw item IDs only as debug/error fallback. |
| `极简 HUD 系统` | Soft | Item metadata only if HUD surfaces pickup/equip notifications. | HP/MP/stat truth, combat power, growth celebration. | HUD should not read item definitions to calculate player power. |
| `成长反馈系统` | Hard for equipment growth chain | Item/equipment source identity and display metadata via Equipment; Character Attributes deltas/combat_power via Attributes. | Growth salience, VFX/audio timing, celebration, final text. | Growth feedback consumes Attribute display proxy, not item-defined combat power. |
| `存档系统` | Hard | Stable `item_id`, `definition_version`, stack constraints, lifecycle/deprecated status, instance field requirements. | Save file schema, migration algorithm, container state, equipped state. | Save stores references and instance state, not copied template metadata as truth. |
| `商店 / 回收系统` | Future / Soft | `item_id`, category, quality, tags as optional price-table inputs. | Buy price, sell price, recycle output, price scaling, vendor rules. | Future economy must own price tables; quality is not implicit price. |
| `强化 / 材料成长系统` | Future / Soft | Item category, material tags, equipment references if later enabled. | Enhancement levels, material costs, success rates, affixes, sockets. | Future systems must add explicit instance fields / modifier sources, not hidden template overrides. |
| `数据调试 / 开发工具` | Soft / Future but useful in Phase 1 | Raw/normalized item definitions, validation failures, source/evidence labels, lifecycle status, downstream references. | Production content policy, player-facing UI, gameplay fallback behavior. | Debug tools may inspect invalid definitions but must not make them normal gameplay content. |
| `本地化系统` | Future / Soft | `display_name_key`, `description_key`, `pickup_label_key`, reason/source labels. | Final localized text, font/layout fallback, locale QA. | Raw IDs are never final player text. |
| `网络 / 最小协议系统` | Future | Stable IDs and possible mapping to protocol item IDs once OpenMir2 evidence exists. | Replication authority, packet format, server compatibility. | Phase 1 offline slice must not design networking from this GDD. |

### Owns / Reads / Writes / Emits / Listens

| Boundary | Contract |
|---|---|
| Owns | Stable item definition identity, taxonomy, display metadata references, stack constraints, equipment candidate data, modifier payload declaration, source/evidence labels, validation lifecycle and definition-level structured failure reasons. |
| Reads | Game concept, systems index, OpenMir2 evidence contracts when available, Character Attributes stat registry/modifier-target rules, ADR-0013 Resource boundary, ADR-0014 display proxy boundary, technical preferences. |
| Writes | Item definition GDD contract, future normalized item definition rows/tables, validation results, lifecycle status, migration/deprecation metadata, registry entries for cross-system item/formula facts after completion. |
| Emits | Design/runtime-domain events or results such as `item_definition_semantically_validated`, `item_definition_invalid`, `item_reference_unresolved`, `item_definition_deprecated`, `item_definition_migration_required`, `spawn_eligible_reference_available`. These are contract concepts, not necessarily Godot signals. |
| Listens | OpenMir2 evidence upgrades/conflicts, Character Attributes stat registry changes, downstream drop/inventory/equipment/save requirements, content validation failures, migration needs. |

### Hard vs Soft Dependency Rules

- `OpenMir2 行为映射 Spike` is hard only for authenticity claims. MVP provisional item definitions may proceed without E3/E4 item evidence if every unverified field is labeled provisional or pending.
- `角色属性系统` is hard for any equipment modifier payload. If Character Attributes stat IDs or modifier-target flags are absent, equipment modifiers remain blocked/unvalidated.
- `掉落表系统`, `掉落与拾取系统`, `背包系统`, `装备系统`, `背包 / 装备 UI 系统`, and `存档系统` are downstream hard consumers for the 30-second loot loop, but they do not need to exist before this GDD can define the item contract.
- `商店 / 回收系统`, `强化 / 材料成长系统`, `本地化系统`, `网络 / 最小协议系统`, and broad debug tools are future/soft consumers. This GDD reserves safe metadata boundaries for them without implementing their rules.

### Bidirectional Consistency Notes

- If a downstream GDD says it depends on item identity, stackability, equipment metadata, item display keys, or item lifecycle status, it must cite this GDD as the source of those facts.
- If this GDD lists a downstream system as consuming a field, that downstream GDD must not redefine that field as its own authority.
- If a downstream GDD needs a field not defined here, it must either add the field through a future Item Definition revision or own it explicitly in its own domain without pretending it is item definition truth.
- If OpenMir2 evidence later changes item type, slot, stack, or instance semantics, this GDD and all affected downstream GDDs must be reviewed for migration impact before implementation stories proceed.
- If Character Attributes changes stat IDs or modifier targetability, item definitions with affected modifier payloads become invalid until migrated; Item Definition must not silently remap stat targets.
- If UI, economy, or drop systems use `quality_id`, they must state whether quality is only display/salience metadata or an input to their own rule. Item Definition does not supply implicit quality math.

### Provisional Assumptions

- Phase 1 uses project-local `mvp_provisional` item definitions until OpenMir2 `StdItem` / `UserItem` evidence reaches E3/E4.
- Phase 1 equipment modifiers are flat integer `add_flat` payload rows only.
- Phase 1 enabled item types are `equipment` and `material`; `currency` is optional; `consumable`, `quest`, and complex equipment growth fields are reserved.
- Phase 1 item definitions use normalized DTO/table runtime truth. If `.tres` Resources are introduced later, they remain authoring envelopes and must be normalized before runtime use.
- Phase 1 item definitions do not include vendor price, recycle value, random affixes, durability, binding, sockets, enhancement level, class scaling, or combat power.

## Tuning Knobs

All tuning knobs below are data/config values for item definition validation and metadata behavior. They are not OpenMir2-authentic unless later backed by E3/E4 evidence. They do not define drop rates, prices, final combat power, inventory capacity, equipment legality, or UI layout.

| Tuning Knob | Safe Phase 1 Range / Values | Gameplay / Pipeline Impact | Owner / Boundary |
|---|---|---|---|
| `enabled_item_types` | Required: `equipment`, `material`; optional: `currency`; reserved: `consumable`, `quest`; debug-only: `debug` | Controls which item categories can pass normal runtime validation. Keeping this small prevents MVP scope creep. | Item Definition owns type validation; use/effect behavior belongs to downstream systems. |
| `enabled_quality_ids` | Recommended active: `normal`, `fine`, `rare`; debug-only: `debug`; future reserved only if documented | Determines allowed quality metadata for display/salience. | Item Definition owns classification; Drop/UI/Economy own any use of quality as input. |
| `default_source_status` | `mvp_provisional` for Phase 1 content unless evidence exists | Prevents false OpenMir2-authentic claims while still allowing MVP content. | Item Definition owns labels; OpenMir2 Spike owns evidence. |
| `required_evidence_level_for_openmir2_verified` | E3 or E4; recommended E3 minimum, E4 preferred | Gates when fields may be called OpenMir2-authentic. Higher threshold improves fidelity but slows content authoring. | OpenMir2 evidence contract. |
| `item_id_format_policy` | Stable snake_case / semantic key; no localized text; no file-path identity | Affects durability of save/load, drop table references, tests and registry entries. | Item Definition owns identity format. |
| `definition_version_policy` | Monotonic explicit version per definition or definition table version; must change on semantic item rule changes | Controls migration detectability and save/load compatibility. | Item Definition + Save/Load boundary. |
| `normal_runtime_lifecycle_states` | `spawn_eligible_reference` only for normal drops; `validated_definition` allowed for non-spawn reference; `debug_only`, `deprecated`, `invalid`, `blocked_unconfirmed` not normal-spawnable | Prevents accidental debug/deprecated/unconfirmed content in player loot. | Item Definition lifecycle validation. |
| `debug_runtime_profile_enabled` | `false` in normal play; `true` only for debug/test fixtures | Allows debug items and invalid-case fixtures without polluting normal content. | Debug/tooling owns activation; Item Definition owns restrictions. |
| `stackable_max_stack_size_default` | No implicit default for player content; must be explicit; recommended MVP examples `10–99` for materials if used | Controls validation for stackable item definitions. | Item Definition owns per-item max; Inventory owns merge/split/overflow. |
| `max_stack_size_upper_bound` | Technical safe positive integer; recommended Phase 1 content cap `1–999` unless economy requires more | Prevents huge quantities from hiding save/inventory bugs. | Item Definition validation; Inventory/economy may impose stricter rules later. |
| `non_stackable_max_stack_size` | Fixed `1` | Ensures equipment and other non-stackable items use distinct instances. | Non-tunable rule; listed for validation clarity. |
| `equipment_stack_policy` | Fixed `non_stackable` in Phase 1 | Keeps equipment comparable, saveable and instance-extensible. | Non-tunable Phase 1 rule. |
| `equipment_min_modifier_rows` | Recommended `1` for normal stat equipment; `0` only for explicitly cosmetic/placeholder equipment | Determines whether equipment without gameplay effect is allowed. | Item Definition validates count; Equipment/Attributes own application. |
| `equipment_max_modifier_rows` | Recommended Phase 1 `1–4` | Limits complexity of early equipment while supporting weapon/armor-style bonuses. | Item Definition validates payload size. |
| `allowed_modifier_operations` | Phase 1 fixed: `add_flat` only | Keeps integration with Character Attributes formula tests deterministic. | Item Definition declares payload; Character Attributes owns operation semantics. |
| `modifier_value_safe_range` | Must fit Character Attributes technical safe range; recommended MVP content `-9999–9999`, normal positive gear values usually `0–9999` | Prevents overflow and untestable content. | Character Attributes owns final numeric safety; Item Definition validates payload before handoff. |
| `zero_modifier_policy` | Recommended: allowed only for explicit placeholder/test/cosmetic rows with validation evidence | Avoids silent no-op equipment payloads. | Item Definition validation. |
| `allowed_modifier_target_stats` | Must be subset of Character Attributes modifier-targetable stat IDs; Phase 1 likely `health_max`, `mana_max` if enabled, physical attack/defense pairs and enabled secondary stats | Controls which stat IDs item definitions may target. | Character Attributes owns registry; Item Definition reads and validates. |
| `forbidden_modifier_target_stats` | Fixed Phase 1: `health_current`, `mana_current`, unknown, inactive/reserved/non-targetable stats | Prevents item definitions from mutating runtime resources or hidden stats. | Non-tunable boundary. |
| `equipment_slot_tag_set` | MVP provisional tags only until OpenMir2 evidence exists; keep small, e.g. weapon-like / armor-like / accessory-like candidate tags | Enables Equipment to route items without pretending authentic slot semantics. | Item Definition owns candidate tags; Equipment owns final legality. |
| `main_comparison_hint_set` | Recommended: `attack`, `defense`, `health`, `none`, `debug`; no formula authority | Helps UI/Equipment request the right comparison focus without calculating power here. | Hint only; Character Attributes owns actual delta/combat_power. |
| `display_metadata_required_policy` | Player-facing items require `display_name_key` and `icon_key`; `description_key` recommended | Controls UI readiness and prevents raw ID leakage. | Item Definition owns keys; UI/localization owns final text and layout. |
| `placeholder_icon_policy` | Allowed only as explicit placeholder key for incomplete art; not accepted as final polish evidence | Lets MVP proceed with temporary assets while keeping content debt visible. | Item Definition + UI/art pipeline boundary. |
| `raw_item_id_player_fallback_policy` | Error/debug only; normal player-facing acceptance should fail if raw IDs appear | Protects player-facing polish and localization readiness. | UI owns presentation; Item Definition supplies display keys. |
| `spawn_profile_allowed_types` | MVP normal spawn: `equipment`, `material`, optional `currency`; exclude `debug`, reserved `quest/consumable` unless explicitly approved | Controls which valid definitions can be generated by normal drop/spawn paths. | Item Definition supplies eligibility; Drop owns selection. |
| `deprecated_spawn_policy` | Default: deprecated items are not new-spawnable; load only through migration/fallback | Prevents retired content from re-entering loot. | Item Definition + Save/Load boundary. |
| `missing_definition_fallback_policy` | Recommended: unresolved/blocked placeholder for save/debug only; never normal loot/equip | Defines safe failure for missing references. | Save/Load/UI may present fallback; Item Definition reports unresolved. |
| `validation_error_reporting_mode` | Recommended: collect all independent detectable errors; deterministic ordering by validation stage then item_id/field/row order | Improves authoring feedback and test reproducibility. | Item Definition validation. |
| `resource_authoring_enabled` | Phase 1 default: `false`; if `true`, `.tres` remains authoring envelope only per ADR-0013 | Controls whether Godot Resource import path exists. | ADR/implementation required before use. |
| `future_field_policy` | Phase 1 default: reject unknown behavior-bearing fields unless the schema explicitly marks them inert metadata | Prevents durability, binding, affixes, sockets, prices or crafting fields from silently becoming gameplay rules. | Item Definition validation + downstream GDD ownership. |
| `phase1_item_id_format` | ASCII snake_case stable semantic keys | Keeps save/drop/test references durable and avoids localization identity drift. | Item Definition validation; localization owns player text. |
| `phase1_equipment_min_modifier_rows` | `1` for normal spawnable stat equipment; `0` only for debug/cosmetic/placeholder rows excluded from normal loot | Ensures normal equipment is player-explainable and comparison-ready. | Item Definition validation reads Character Attributes target rules. |
| `phase1_equipment_max_modifier_rows` | `4` | Limits early item complexity and test matrix size. | Item Definition validation. |
| `phase1_zero_modifier_policy` | Normal spawnable stat equipment fails on zero-value modifier rows; debug/placeholder/cosmetic rows require explicit status and reason | Prevents no-op equipment from feeling like loot. | Item Definition validation. |
| `runtime_lookup_policy` | Validated runtime definitions must be indexed by `item_id` and version/profile as needed; normal gameplay/UI paths must not full-scan authoring rows | Protects 60 fps loop and avoids stutter in inventory/tooltip/drop rendering. | Implementation ADR chooses structure. |
| `validation_timing_policy` | Full schema/semantic validation runs at load/build/smoke/hot-reload swap; runtime gameplay queries read cached validated status/results | Prevents per-frame or per-tooltip revalidation cost. | Item Definition core + ADR. |
| `ui_projection_cache_policy` | UI may cache lightweight projection DTOs keyed by item_id/version/locale/profile; cache is view-only and invalidated on version/profile/locale change | Prevents UI copying full definition truth or holding mutable Resource authority. | UI owns cache; Item Definition owns query result truth. |
| `quality_default_salience_policy` | If `presentation_tier` or `pickup_visual_salience` is absent, downstream may derive default salience from `quality_id`; `rare` must not default below normal salience | Preserves loot hierarchy without encoding final VFX/audio values. | Item Definition supplies tokens; presentation/audio own mapping. |

### Non-Tunable Boundaries

These are not tuning knobs and must not be changed through item data alone:

- Drop probability, drop weight, source monster, guarantee/pity logic and quantity roll are owned by `掉落表系统`.
- Ground drop lifetime, pickup radius, pickup priority and map placement are owned by `掉落与拾取系统` plus map/interaction systems.
- Inventory capacity, slot dimensions, sorting, merge/split algorithm and overflow behavior are owned by `背包系统`.
- Equip legality, slot occupancy, replacement transaction and equipped state are owned by `装备系统`.
- Effective stats, current resources, deltas, snapshots and `combat_power` are owned by `角色属性系统`.
- Buy/sell/recycle value and material conversion are owned by future economy systems.
- UI colors, tooltip layout, comparison panel layout, animation, VFX and audio are owned by UI/presentation/audio systems.
- OpenMir2-authentic field names, type IDs, slot IDs and instance structures require OpenMir2 evidence rather than tuning.

## Acceptance Criteria

### QA Evidence Gate Summary

| Area | Story Type | Required Evidence | Gate |
|---|---|---|---|
| Item identity, semantic validation, profile eligibility | Logic | GUT unit tests in `tests/unit/item_definition/` | BLOCKING |
| Stack formula and stack policy validation | Logic | Boundary-value GUT unit tests | BLOCKING |
| Modifier payload validation and visible modifier facts | Logic / Integration | GUT tests with Character Attributes stat-registry fixture; integration tests when real registry is queried | BLOCKING |
| Reference lookup and spawn eligibility | Logic / Integration | Unit tests plus fixture validation against drop/save-style references | BLOCKING |
| Source/evidence label enforcement | Logic / Config/Data | Validator tests for evidence thresholds and spawn blocking | BLOCKING |
| Player-facing metadata, accessibility semantics, and fallback keys | Logic / Config/Data | Validator tests plus content smoke report | BLOCKING for data contract; UI walkthrough ADVISORY until UI exists |
| Ownership boundary rejection | Logic | Schema/validation tests proving forbidden fields are rejected or non-authoritative | BLOCKING |
| MVP loot-loop fixture validation | Config/Data | Smoke check proving baseline/upgrade/sidegrade/material fixture shape | BLOCKING before QA hand-off for loot-loop slice |
| UI projection and fallback behavior | UI / Integration | Projection DTO tests; manual walkthrough once UI exists | BLOCKING for projection contract; ADVISORY for final presentation |
| Runtime truth / lookup / cache boundary | Logic / Integration | DTO immutability/aliasing tests; indexed lookup policy tests or ADR trace before implementation | BLOCKING once implementation begins |

### AC-01 — Stable Item Identity

- **GIVEN** item definition config contains one or more item rows, **WHEN** the validator runs, **THEN** every normal row has a non-empty ASCII snake_case format-valid `item_id`, no two active runtime rows share the same `item_id`, and missing/duplicate IDs produce deterministic structured validation reasons.
- **GIVEN** two rows share a display name but have different valid `item_id` values, **WHEN** validation runs, **THEN** both may remain semantically valid if all other rules pass because display text is not identity.
- **Story Type:** Logic
- **Blocking Evidence:** GUT unit tests for missing ID, malformed ID, unique ID pass, duplicate ID fail, duplicate display metadata pass.
- **Gate:** BLOCKING

### AC-02 — Definition Version and Migration Visibility

- **GIVEN** an item definition row is loaded, **WHEN** validation runs, **THEN** it includes a supported `definition_version`, and missing, unsupported, newer-than-client or migration-required versions return structured validation/migration results rather than silent reinterpretation.
- **GIVEN** old and new fixture rows share `item_id` but differ in rule-bearing fields such as modifier payload, item type, stack policy, equipment semantics or instance requirements, **WHEN** content diff validation runs, **THEN** unchanged `definition_version`, missing migration note, or incomplete old/new version migration metadata fails with deterministic reasons.
- **Story Type:** Logic / Config/Data
- **Blocking Evidence:** Unit tests for supported, missing, unsupported, newer-than-client, retired and migration-required versions; fixture-pair diff tests for semantic changes.
- **Gate:** BLOCKING for validator/diff behavior; save migration execution remains Save/Load-owned.

### AC-03 — Required Semantic Definition Fields

- **GIVEN** an item definition is intended for normal runtime use, **WHEN** `item_definition_semantically_valid` runs, **THEN** it requires valid `item_id`, `definition_version`, `item_type`, `quality_id`, `source_status`, `lifecycle_status`, required display metadata, stack policy, type-specific fields, evidence labels, and Phase 1 player-facing metadata where applicable.
- **GIVEN** any required child gate fails, **WHEN** semantic validation runs, **THEN** it returns `passed = false` and ordered structured failure reasons.
- **Story Type:** Logic / Config/Data
- **Blocking Evidence:** Unit tests with all gates passing and one failing child gate at a time.
- **Gate:** BLOCKING

### AC-04 — Source and Evidence Labels

- **GIVEN** an item definition or OpenMir2-sensitive subfield declares `source_status = openmir2_verified`, **WHEN** validation runs, **THEN** a valid E3/E4 `evidence_ref` is required.
- **GIVEN** evidence is absent or below threshold, **WHEN** normal spawn eligibility is evaluated, **THEN** the row remains `mvp_provisional`, `project_local`, `openmir2_evidence_pending` or `blocked_unconfirmed`, and cannot become normal spawnable content while claiming authenticity.
- **Story Type:** Logic / Config/Data
- **Blocking Evidence:** Unit tests for verified-with-evidence, verified-without-evidence, below-threshold evidence, pending/provisional rows, blocked-unconfirmed lifecycle, and spawn attempt rejection for authenticity claims without evidence.
- **Gate:** BLOCKING

### AC-05 — Lifecycle, Profile Eligibility, and Spawn Eligibility

- **GIVEN** semantically valid item definitions exist in states `active`, `debug_only`, `deprecated`, `invalid` and `blocked_unconfirmed`, **WHEN** `definition_profile_eligible` runs under normal, debug, content-smoke, and save-load profiles, **THEN** each state returns the profile-specific eligibility status defined in the GDD.
- **GIVEN** a normal drop/spawn reference targets a debug-only, deprecated, invalid, blocked-unconfirmed, unresolved, or reserved-type definition, **WHEN** `spawn_eligible_reference` is evaluated, **THEN** it returns false with a structured reason.
- **Story Type:** Logic
- **Blocking Evidence:** Unit tests for every lifecycle/profile pair and a one-false-per-gate spawn eligibility matrix.
- **Gate:** BLOCKING

### AC-06 — Item Type Contract

- **GIVEN** an item definition declares `item_type`, **WHEN** validation runs, **THEN** exactly one recognized primary type is present.
- **GIVEN** Phase 1 normal runtime validates content, **WHEN** item types are checked, **THEN** `equipment` and `material` are accepted, `currency` only if explicitly enabled, and reserved `consumable`, `quest` and `debug` are not normal-spawnable unless an explicit debug/test profile allows them.
- **Story Type:** Logic
- **Blocking Evidence:** Unit tests for allowed, optional, reserved, debug, unknown, missing and multiple type cases.
- **Gate:** BLOCKING

### AC-07 — Quality Metadata Is Not Power Authority

- **GIVEN** an item declares `quality_id`, **WHEN** validation runs, **THEN** it resolves to an enabled quality classification such as `normal`, `fine`, `rare` or `debug`, and to a textual/semantic quality label mapping for player-facing content.
- **GIVEN** item data attempts to use quality as stat value, drop rate, price, equip legality, combat power, exact UI color authority, or audio/VFX implementation authority, **WHEN** schema/semantic validation runs, **THEN** the row fails with a deterministic boundary-violation reason.
- **Story Type:** Logic / Config/Data
- **Blocking Evidence:** Schema/unit tests rejecting forbidden quality-to-power/price/drop/color/audio fields; smoke check for allowed quality IDs and label mappings.
- **Gate:** BLOCKING for forbidden mechanics and data label mapping; visual styling walkthrough ADVISORY.

### AC-08 — Display and Player-Facing Metadata Readiness

- **GIVEN** a normal player-facing item definition, **WHEN** display validation runs, **THEN** `display_name_key`, `icon_key`, type label mapping, quality label mapping, and required Phase 1 loot salience metadata are present.
- **GIVEN** `description_key` or usage hint is missing, **WHEN** validation runs, **THEN** it emits a deterministic advisory warning unless the active profile marks tooltip-ready content as required.
- **GIVEN** UI renders accepted normal content, **WHEN** loot labels, inventory cells, pickup toasts or tooltips appear, **THEN** they use display/localization keys rather than raw IDs.
- **Story Type:** Logic / UI
- **Blocking Evidence:** Unit tests for required display metadata validation, missing-description warning, raw-ID-as-final-text rejection, and salience/audio token presence for droppable items.
- **Advisory Evidence:** Manual UI walkthrough or interaction test for raw-ID absence once UI exists.
- **Gate:** BLOCKING for data contract; ADVISORY for final UI presentation.

### AC-09 — Stack Quantity Formula

- **GIVEN** parsed stack fields are valid, `stack_policy = stackable`, and `max_stack_size > 1`, **WHEN** `stack_quantity_valid` receives integer quantities from `1` through `max_stack_size`, **THEN** it returns true.
- **GIVEN** quantity is `0`, negative, fractional, missing, malformed, NaN-equivalent or greater than `max_stack_size`, **WHEN** `stack_quantity_valid` runs, **THEN** it returns false and does not clamp; missing/malformed quantity fails before numeric comparison.
- **GIVEN** `stack_policy = non_stackable` and `max_stack_size = 1`, **WHEN** stack validation runs, **THEN** only quantity `1` passes.
- **Story Type:** Logic
- **Blocking Evidence:** Boundary-value GUT tests for stackable, non-stackable, parse failure, and invalid quantity cases.
- **Gate:** BLOCKING

### AC-10 — Stack Policy Consistency

- **GIVEN** an item definition declares stack fields, **WHEN** semantic validation evaluates `stack_policy_valid`, **THEN** `non_stackable` requires `max_stack_size = 1`, `stackable` requires integer `max_stack_size > 1`, and contradictory or malformed data fails validation.
- **GIVEN** `item_type = equipment`, **WHEN** stack validation runs, **THEN** `equipment_stack_policy_valid` requires `non_stackable` and `max_stack_size = 1`.
- **Story Type:** Logic
- **Blocking Evidence:** Unit tests for valid/invalid stack policy combinations, malformed max stack size, and stackable-equipment rejection.
- **Gate:** BLOCKING

### AC-11 — Equipment Definition Data and Comparison Readiness

- **GIVEN** `item_type = equipment`, **WHEN** validation runs, **THEN** the row includes valid equipment candidate data: `equipment_category`, `equip_slot_tags`, source status, valid modifier payload rules, and `main_comparison_hint` for normal spawnable stat equipment.
- **GIVEN** a normal spawnable stat equipment item has no visible valid modifier facts or comparison hint, **WHEN** semantic/profile validation runs, **THEN** it is not eligible for normal MVP loot.
- **GIVEN** a non-equipment item in Phase 1, **WHEN** validation runs, **THEN** active equipment data and active modifier payload rows are rejected.
- **Story Type:** Logic
- **Blocking Evidence:** Unit tests for valid equipment, missing equipment block, missing category/slot tags, missing comparison hint, non-equipment equipment block and non-equipment modifier payload.
- **Gate:** BLOCKING

### AC-12 — Modifier Payload Row Formula

- **GIVEN** an enabled modifier row has stable `modifier_row_id`, valid source/evidence labels, uses `operation = add_flat`, targets a known modifier-targetable non-current-resource stat, and has integer non-zero value within safe range under the Phase 1 normal profile, **WHEN** `modifier_payload_row_valid` runs, **THEN** it returns true.
- **GIVEN** a row uses unsupported operation, unknown stat, inactive/non-targetable stat, `health_current`, `mana_current`, missing row ID, missing source status, missing verified evidence, zero value in normal profile, missing value, non-numeric value, fractional value or out-of-range value, **WHEN** validation runs, **THEN** it returns false or configured warning/failure with the target gate reason.
- **Story Type:** Logic / Integration
- **Blocking Evidence:** Unit tests using a Character Attributes stat-registry fixture; once a real registry is queried by implementation, integration tests are BLOCKING before dependent story Done.
- **Gate:** BLOCKING

### AC-13 — Modifier Payload Whole-Payload Validation

- **GIVEN** an equipment item has active modifier row count between configured Phase 1 `equipment_min_modifier_rows = 1` and `equipment_max_modifier_rows = 4`, and every active row passes `modifier_payload_row_valid`, **WHEN** `modifier_payload_valid` runs, **THEN** it returns true.
- **GIVEN** any active modifier row is invalid, bounds config is missing/inverted, or active row count is outside configured bounds, **WHEN** `modifier_payload_valid` runs, **THEN** the entire payload returns false; downstream systems must not receive partially trusted modifier data.
- **GIVEN** a non-equipment item in Phase 1, **WHEN** `modifier_payload_valid` runs, **THEN** it returns true only when active modifier row count is `0`.
- **Story Type:** Logic
- **Blocking Evidence:** Unit tests for min/max row counts, valid multi-row payload, mixed valid/invalid rows, missing/inverted bounds config, non-equipment zero-row pass and non-equipment active-row fail.
- **Gate:** BLOCKING

### AC-14 — Zero Modifier Policy

- **GIVEN** a normal spawnable stat-equipment modifier payload row has value `0`, **WHEN** validation runs under the Phase 1 normal profile, **THEN** it fails with deterministic zero-modifier reason.
- **GIVEN** a debug/cosmetic/placeholder row explicitly permits zero-value modifiers and is excluded from normal loot, **WHEN** validation runs, **THEN** it passes or warns according to `zero_modifier_policy` and reports the reason in smoke/debug output.
- **Story Type:** Logic / Config/Data
- **Blocking Evidence:** Unit tests for zero-disallowed normal profile and zero-allowed debug/placeholder profile; smoke report listing zero-value rows.
- **Gate:** BLOCKING for policy behavior; ADVISORY for content review.

### AC-15 — Item Reference Lookup Resolution

- **GIVEN** a downstream system references `item_id` and `definition_version`, **WHEN** `item_reference_lookup_resolvable` runs, **THEN** it returns true only if the ID is present, format-valid, registered in normalized runtime definitions, and version-supported.
- **GIVEN** the reference is missing, malformed, unregistered, unsupported-version or newer-than-client, **WHEN** resolution runs, **THEN** it returns structured unresolved/blocked status and does not silently substitute a normal placeholder.
- **GIVEN** the reference resolves to deprecated/debug/blocked content, **WHEN** profile eligibility or spawn eligibility runs, **THEN** lookup may succeed while normal spawn/equip/use eligibility fails with profile-specific reason.
- **Story Type:** Logic / Integration
- **Blocking Evidence:** Unit tests for each lookup gate; integration fixture covering drop-table and save/load-style references.
- **Gate:** BLOCKING

### AC-16 — Invalid or Ineligible Definitions Are Never Spawnable

- **GIVEN** a definition fails `item_definition_semantically_valid`, **WHEN** `spawn_eligible_reference` is evaluated, **THEN** it returns false.
- **GIVEN** normal drop tables or spawn fixtures reference items, **WHEN** validation runs, **THEN** every normal loot reference must resolve to `spawn_eligible_reference = true`; invalid, deprecated-for-new-spawn, debug-only, blocked-unconfirmed, missing and reserved-type definitions fail validation.
- **Story Type:** Logic / Integration
- **Blocking Evidence:** Unit test for formula; integration validation with valid and invalid drop-table fixtures.
- **Gate:** BLOCKING

### AC-17 — Runtime Truth Is Normalized, Indexed, and Read-Only

- **GIVEN** item definitions are loaded from authoring data or future `.tres` Resources, **WHEN** runtime systems consume them, **THEN** they consume normalized DTO/table/query-result data rather than mutable authoring graphs.
- **GIVEN** runtime query results, rows, modifier arrays, display metadata maps, or inbound authoring payloads are mutated by a test consumer after normalization, **WHEN** runtime item truth is queried again, **THEN** authoritative data is unchanged.
- **GIVEN** normal gameplay or UI paths query item definitions, **WHEN** lookup executes, **THEN** it uses validated indexed runtime data rather than full-scanning authoring rows or re-running full validation.
- **Story Type:** Integration / Logic
- **Blocking Evidence:** Implementation stories must include DTO aliasing/defensive-copy tests and indexed lookup policy tests or an accepted ADR binding equivalent behavior. Resource importer tests are additionally BLOCKING if `.tres`/Resource authoring exists. GDD-stage ADR trace is acceptable only before implementation stories are Ready.
- **Gate:** BLOCKING once implementation begins.

### AC-18 — Instances Do Not Redefine Template Truth

- **GIVEN** an item instance, inventory stack, ground drop, equipped reference or save payload references an item definition, **WHEN** it is validated or loaded, **THEN** it may store allowed instance fields such as quantity, instance ID, owner/container context and definition version, but must not redefine display name, item type, quality, stackability, equipment category, source status or template modifier rows as authoritative data.
- **Story Type:** Integration
- **Blocking Evidence:** Schema/integration test for instance/save fixture rejecting copied authoritative template fields.
- **Gate:** BLOCKING

### AC-19 — Forbidden Ownership Fields Are Rejected

- **GIVEN** an item definition attempts to define drop chance, drop weight, pickup radius, inventory capacity, slot layout, equip transaction legality, effective stats, combat power, shop price, UI layout, concrete color, VFX parameters, audio file paths, audio mix parameters, or final localized strings as sole truth, **WHEN** schema/semantic validation runs, **THEN** the row fails validation with deterministic boundary violation output.
- **Story Type:** Logic
- **Blocking Evidence:** Schema/unit tests for representative forbidden fields from each out-of-scope boundary and expected reason codes.
- **Gate:** BLOCKING

### AC-20 — Equipment → Attributes Handoff

- **GIVEN** any implementation story reads item definition modifier payload or comparison hints to equip or preview equipment, **WHEN** modifier sources are derived, **THEN** they are derived from item definition payload plus equipped instance identity and submitted through Equipment to Character Attributes for validation/aggregation.
- **GIVEN** Attributes returns effective stats, delta or combat power, **WHEN** UI or feedback consumes them, **THEN** Item Definition does not store or recompute those outputs.
- **Story Type:** Integration
- **Activation:** Any story implementing equipment equip/preview/comparison from Item Definition.
- **Blocking Evidence:** Integration test or documented playtest checklist covering equip handoff and Attribute validation.
- **Gate:** BLOCKING once activated.

### AC-21 — UI Projection Metadata Handoff and Fallback

- **GIVEN** UI renders a loot label, inventory cell, tooltip or equipment comparison entry, **WHEN** it receives item information, **THEN** it receives a query result/projection contract, not mutable raw config rows or authoring Resources.
- **GIVEN** UI reads item data, **WHEN** it renders normal content, **THEN** it may read display keys, icon key, type/quality label keys, stack policy, equipment candidate labels, comparison hint, visible modifier facts and validation/status fields, but must not compute effective stats, combat power, equip legality, stack merge, drop value or price from item data.
- **GIVEN** UI requests unresolved, invalid, stale, deprecated or debug-only item data, **WHEN** UI renders it, **THEN** query result provides fallback localization/status keys, distinct fallback icon key or status, disabled interaction reason, and debug-only reason codes; UI must not show blank normal item UI or fabricate gameplay behavior.
- **Story Type:** UI / Integration
- **Blocking Evidence:** Projection/result contract tests and fallback oracle tests.
- **Advisory Evidence:** Manual UI walkthrough, interaction test or screenshots for final presentation once UI exists.
- **Gate:** BLOCKING for projection/fallback data contract; ADVISORY for final layout/presentation.

### AC-22 — Accessibility Semantic Metadata

- **GIVEN** a player-facing item definition is valid for Phase 1 UI content, **WHEN** item metadata is queried, **THEN** item name, type, quality, stack count eligibility, invalid/fallback state, and visible modifier facts have text/semantic representations and do not rely only on icon or color.
- **GIVEN** equipment comparison presents positive/negative deltas, **WHEN** UI displays them, **THEN** red/green color is not the only differentiator; symbols/text/icons or labels must supplement color.
- **Story Type:** UI / Config/Data
- **Blocking Evidence:** Metadata smoke check covering loot label, inventory cell, stack count, quality indicator, invalid/fallback state, positive delta and negative delta semantic availability.
- **Advisory Evidence:** Accessibility checklist, screenshots, or interaction capture once UI exists.
- **Gate:** BLOCKING for data-side semantic metadata; ADVISORY for final UI presentation.

### AC-23 — MVP Loot-Loop Fixture Smoke

- **GIVEN** the MVP provisional item definition set is loaded, **WHEN** the content smoke check runs, **THEN** all normal rows pass required validation gates and the set includes at minimum: one baseline equipped item/reference, one same-category clear upgrade equipment, one same-category sidegrade or weaker equipment, one stackable material if materials are in Phase 1, and optional rare/showcase item if loot-feedback salience is under test.
- **GIVEN** the MVP fixture is used by normal loot-loop tests, **WHEN** it spawns, displays, picks up, inventories, compares or equips items, **THEN** it uses normal spawnable definitions, includes no debug-only/reserved/invalid/unsupported-authentic/missing-display rows, and can produce a visible positive Attribute delta for the upgrade path.
- **Story Type:** Config/Data / Integration
- **Blocking Evidence:** Smoke check pass with validation report artifact such as `production/qa/smoke-[date].md` when the smoke workflow exists; fixture test proving baseline/upgrade/sidegrade/material shape.
- **Gate:** BLOCKING before QA hand-off for any build depending on this content.

### AC-24 — Debug and Invalid Fixtures Are Isolated

- **GIVEN** tests require intentionally invalid or debug-only item definitions, **WHEN** fixtures load, **THEN** those rows are marked validation-test/debug content and cannot appear in normal gameplay item data, normal drop tables, normal saves or accepted Phase 1 content smoke checks.
- **GIVEN** a normal Phase 1 loot-loop test runs, **WHEN** it spawns, displays, picks up, inventories, compares or equips items, **THEN** it uses normal spawnable definitions, not debug-only definitions unless testing debug behavior.
- **Story Type:** Config/Data / Integration
- **Blocking Evidence:** Fixture validation test or smoke-check assertion proving profile/path/tag isolation.
- **Gate:** BLOCKING

### AC-25 — Save/Load Reference Boundary

- **GIVEN** an inventory stack, equipment instance or ground item is saved, **WHEN** the save payload is inspected or loaded, **THEN** it uses `item_id` and `definition_version` as definition references and does not treat copied display names, icons, type, quality, modifiers, stack rules or final stats as authoritative save truth.
- **GIVEN** a missing, deprecated, or incompatible definition is encountered during load, **WHEN** lookup succeeds or fails according to save/load profile, **THEN** the result is unresolved, migration-required, fallback or blocked-load status, not normal placeholder loot.
- **Story Type:** Integration
- **Activation:** Any story writing or reading save payload item references.
- **Blocking Evidence:** Integration test or documented save/load fixture validation once Save/Load depends on Item Definition.
- **Gate:** BLOCKING once activated.

### Minimum Blocking Test Matrix

The first implementation story that introduces Item Definition validation should include automated coverage for:

1. `stack_quantity_valid` / AC-09: stackable lower/mid/upper bounds, invalid zero/negative/fractional/missing/malformed/over max, non-stackable `1` pass and `2` fail.
2. `modifier_payload_row_valid` / AC-12: valid `add_flat`, missing row ID/source/evidence, unsupported op, unknown stat, non-targetable stat, current-resource target, zero-value normal row, non-integer and out-of-range value.
3. `modifier_payload_valid` / AC-13/AC-14: valid equipment payload, mixed valid/invalid rows, non-equipment zero-row pass, non-equipment active-row fail, min/max row count boundaries, missing/inverted bounds config.
4. `item_reference_lookup_resolvable` / AC-15/AC-25: registered active reference pass; missing/malformed/unregistered/unsupported-version/newer-than-client references fail; deprecated save-load lookup distinct from spawn eligibility.
5. `item_definition_semantically_valid` / AC-01/AC-03/AC-06/AC-07/AC-08/AC-10/AC-11/AC-19: all gates pass; one failing child gate at a time; deterministic reason codes; required display metadata and accessibility labels; forbidden ownership fields.
6. `definition_profile_eligible` / AC-04/AC-05/AC-24: normal/debug/content-smoke/save-load profiles; evidence threshold; debug/invalid fixture isolation.
7. `spawn_eligible_reference` / AC-05/AC-16/AC-23: valid normal equipment/material pass; invalid, debug-only, deprecated-for-new-spawn, blocked-unconfirmed, reserved type and unresolved reference fail.
8. Runtime truth / AC-17/AC-18/AC-21: DTO/query result immutability, no raw Resource/runtime authority, no copied template truth in instances/save fixtures, indexed lookup / no full-scan gameplay query policy.
9. MVP content smoke / AC-23: baseline equipment/reference, upgrade equipment, sidegrade/weaker equipment, stackable material if enabled, optional rare showcase, all normal rows have required identity/display/stack/source/lifecycle/presentation fields.

## Appendix — Visual/Audio Requirements

物品定义系统不直接实现掉落视觉、UI 布局、VFX 或音频播放。它只提供下游表现系统可读取的稳定显示元数据，使 UI、掉落反馈、装备比较、音频和可访问性系统能用统一语义解析物品表现。

### Presentation Metadata Contract

Item definitions may declare these presentation metadata fields:

| Field | Requirement | Purpose | Boundary |
|---|---|---|---|
| `display_name_key` | Required for player-facing items | Localization/display lookup for player-facing name. | Final localized text belongs to Localization/UI. |
| `description_key` | Recommended | Tooltip/details lookup. | Tooltip layout and text rendering belong to UI. |
| `short_name_key` | Optional | Compact ground label or log label. | Label visibility and truncation belong to UI/loot feedback. |
| `icon_key` | Required for player-facing items | Stable logical icon reference for inventory/equipment/UI. | Actual atlas path, import settings and icon fallback belong to UI/art pipeline. |
| `world_visual_key` | Optional but recommended for droppable items | Stable logical ground-drop visual token. | Ground sprite rendering, y-sort and visibility belong to loot/map/presentation systems. |
| `visual_family` | Recommended | Material/type family such as `metal`, `cloth`, `leather`, `gem`, `paper`, `potion_glass`, `coin`, `organic`, `debug`. | Art direction maps family to visual language. |
| `quality_id` | Required | Quality classification such as `normal`, `fine`, `rare`, `debug`. | Quality does not define power, drop rate, price or exact color. |
| `presentation_tier` | Optional | Abstract emphasis token such as `plain`, `accented`, `rare`, `debug`. | Loot/UI systems map tier to concrete color/VFX/audio intensity. |
| `pickup_visual_salience` | Optional | Semantic visual hierarchy input: `low`, `standard`, `high`, `critical`, `debug`. | Does not define pickup range, label distance or drop chance. |
| `audio_family` | Recommended | Material sound family such as `metal_light`, `metal_heavy`, `cloth`, `leather`, `paper`, `gem`, `coin`, `potion_glass`, `debug`. | Audio system owns concrete files, mix and playback. |
| `pickup_audio_cue` | Optional override | Special semantic cue ID for notable pickup sounds. | Cue ID only; no file path, volume or pitch values. |
| `equip_audio_cue` | Optional override | Special semantic cue ID for notable equip sounds. | Equipment/audio systems decide whether and when to play it. |
| `accessibility_label_key` | Optional / Future | Screen-reader or accessible text fallback key. | Accessibility/UI systems own final behavior. |

Recommended Phase 1 minimum metadata for normal player-facing items:

```yaml
display_name_key: "item.mvp_bronze_sword.name"
description_key: "item.mvp_bronze_sword.description"
icon_key: "ui_icon_mvp_bronze_sword"
world_visual_key: "drop_visual_metal_weapon_small"
visual_family: "metal"
quality_id: "normal"
pickup_visual_salience: "standard"
audio_family: "metal_light"
```

### Visual Direction Alignment

The presentation metadata should support the selected visual identity anchor, `赤金玛法`:

- Important loot and growth-related equipment should be easy to distinguish from the ground and environment.
- `visual_family = metal` should allow downstream art systems to express “火光照亮铁器” through warm highlights, metal edge readability and grounded material language.
- `quality_id` / `presentation_tier` should support clear drop hierarchy without forcing exact colors into item data.
- `rare` items may request higher salience through `presentation_tier` or `pickup_visual_salience`, but the loot feedback system owns whether that becomes blue/purple/gold/赤金 coloring, light beams, outlines, animation or audio emphasis.
- `debug` items must use debug-only presentation and must not pollute normal player-facing loot visuals.

### Downstream Visual Ownership

UI / Inventory / Equipment UI own:

- actual icon file paths, atlas coordinates, import settings and fallback icons;
- inventory cell size, tooltip width, font, line breaks, stat ordering and compare layout;
- final quality color mapping, hover/selected/disabled states and focus navigation;
- positive/negative stat delta formatting and accessibility treatment;
- final localized text, text fallback and locale-specific layout.

Loot Feedback / Ground Drop systems own:

- ground label visibility, range, priority and occlusion behavior;
- drop outline, glow, beam, sparkle, pickup animation and fly-to-inventory effects;
- y-sort / map visibility integration for ground items;
- screenshot-scale readability of loot against the environment;
- concrete mapping from `quality_id`, `presentation_tier`, `pickup_visual_salience` and `visual_family` to VFX intensity.

Audio systems own:

- actual `.wav` / `.ogg` asset paths;
- volume, bus routing, pitch randomization, spatialization, reverb and ducking;
- rare pickup stingers, equip sounds, UI click sounds and pickup spam prevention;
- audio cue resolution from `audio_family`, `pickup_audio_cue`, `equip_audio_cue`, quality and presentation tier.

### Forbidden Presentation Data in Item Definitions

Item definitions must not hardcode:

- concrete hex/RGB color values such as `#D4AF37`;
- VFX parameters such as glow radius, beam height, sparkle count, particle lifetime or animation duration;
- UI layout values such as tooltip width, font size, label offset, icon scale or inventory cell size;
- audio implementation values such as sound file paths, volume dB, pitch variance, bus name or cooldown;
- Godot scene/script paths such as `.tscn`, `.gd`, node names or animation player states;
- final localized strings as the only source of truth for player-facing text;
- quality-to-power, quality-to-price or quality-to-drop-rate formulas.

### Asset Spec Flag

📌 **Asset Spec** — Visual/Audio metadata requirements are defined at the item-definition boundary. After the art bible is approved and the loot feedback/UI systems have their presentation rules, run `/asset-spec system:物品定义系统` or the relevant downstream presentation-system asset spec to produce per-asset visual descriptions, dimensions and generation prompts.


## Appendix — UI Requirements
物品定义系统提供 UI-facing display metadata only。UI 系统可以读取稳定的物品身份、显示键、图标键、类型、品质、堆叠提示、装备候选提示和定义有效性状态，用于掉落提示、拾取反馈、背包格、tooltip、装备对比入口和错误 fallback。物品定义系统不拥有 UI 布局、输入行为、背包状态、装备状态、tooltip 结构、比较计算、combat power delta 或 presentation styling。

### Readable Display Metadata

UI systems may read these fields from a resolved item definition:

| Field | UI Use | Boundary |
|---|---|---|
| `item_id` | Stable lookup, debug logs, fallback text when definition display data is unavailable. | Not final player-facing name in normal content. |
| `display_name_key` | Player-facing name lookup. | Localization/UI owns final text. |
| `description_key` | Tooltip/detail panel text lookup. | Tooltip layout and line wrapping belong to UI. |
| `short_name_key` | Compact loot label, pickup toast, or log label. | Optional; UI owns when to use it. |
| `icon_key` | Inventory, equipment, pickup toast and tooltip icon resolution. | Actual asset path and atlas handling belong to UI/art pipeline. |
| `world_visual_key` | Optional input for ground-drop UI/feedback. | Loot feedback owns rendering and visibility. |
| `item_type` | Filtering, category label, context menu eligibility hint. | Does not define interaction behavior by itself. |
| `quality_id` | Sorting, quality label, border/style token lookup, loot salience hint. | Does not define power, price, drop chance or exact color. |
| `stack_policy` / `max_stack_size` | UI can decide whether to display quantity and validate quantity formatting. | Current quantity and stack merge state belong to Inventory. |
| `equipment_category` / `equip_slot_tags` | UI can show equipment candidate labels or route to equipment comparison entry. | Equipment owns final equip legality and slot occupancy. |
| `main_comparison_hint` | UI can request an appropriate comparison focus. | Character Attributes / Equipment own actual comparison output. |
| `lifecycle_status` / validation status | UI can distinguish normal, missing, invalid, deprecated, debug-only, or unresolved items. | Item Definition reports status; UI owns player-safe presentation. |
| `accessibility_label_key` | Optional accessible text fallback. | Accessibility/UI systems own final behavior. |

### Runtime State Boundary

Item definitions must not store or imply UI/runtime state:

- current inventory quantity;
- current inventory slot or container;
- whether an item is selected, hovered, focused, locked, favorited, new, dragged, equipped or quick-slotted;
- current owner character;
- current equipment slot occupancy;
- current compare target;
- current combat power delta or effective stat delta;
- whether the current player can equip/use/drop/sell the item in this context.

Those are owned by Inventory, Equipment, Character Attributes, UI state, Save/Load or future economy systems. Item Definition only provides static metadata and validation status.

### Downstream UI Responsibilities

Backpack / Inventory UI owns:

- grid/list layout, slot size, paging and locked slot display;
- item sorting, filtering, drag/drop, click, right-click, double-click and gamepad focus behavior;
- stack split/merge interaction, quantity badge placement and overflow messaging;
- pickup toast / loot feed placement and animation if not owned by a separate loot feedback UI;
- empty slot, full inventory and invalid item presentation.

Equipment UI owns:

- equipment slot layout;
- equip/unequip interaction entry points;
- comparison panel placement, expansion, field order and error display;
- unmet requirement presentation;
- button disabled states, error toasts and equip success feedback;
- how item metadata combines with Attribute preview/delta results.

Tooltip / Detail UI owns:

- tooltip size, position, columns, font, line height and scrolling;
- stat row ordering and advanced/detail collapse behavior;
- quality style mapping;
- long text truncation or wrapping;
- mouse hover versus keyboard/gamepad focus behavior;
- player-safe invalid/missing/stale wording.

Gameplay/rule systems own:

- whether an item can be picked up;
- whether inventory has space;
- whether an item can be equipped;
- how attributes change;
- how combat power changes;
- whether a comparison is stronger, weaker, sidegrade or unavailable.

### Prohibited UI Responsibilities in Item Definitions

Item definitions must not define:

- inventory cell coordinates, tooltip width/height, font size, border pixels, icon scale or screen offsets;
- concrete quality color values, color gradients, outlines or style sheets;
- hover delay, click behavior, double-click behavior, drag rules, shortcut keys, controller buttons or context menu entries;
- tooltip layout, compare panel layout or stat row ordering;
- DPS, combat power, “better/worse” arrows, recommended equipment or build fit;
- UI animation timelines, particle counts, transition durations or sound playback settings;
- final localized strings as the only source of truth.

### Fallback and Error Display Contract

UI must handle unresolved item references safely. Missing, invalid, stale, deprecated or debug-only definitions must never produce empty UI, crashes, or misleading normal item presentation.

| Status | Player-Safe UI | Debug / QA Detail | Interaction Rule |
|---|---|---|---|
| Missing definition | `Unknown Item` / `物品不可用` with missing icon. | Show raw `item_id` and `Missing item definition`. | Disable equip/use/sell by default; safe discard/migration behavior belongs to owner. |
| Invalid definition | Show best available safe fields plus `Item unavailable`. | Show validation reason codes and failing fields. | Disable interactions depending on invalid fields. |
| Stale definition | Prefer current display metadata if safe; otherwise `Item data outdated`. | Show saved/current `definition_version`. | Disable unsafe equip/use until migration or validation passes. |
| Deprecated definition | Show normal metadata if still resolvable, with optional legacy/debug badge outside normal player flow. | Show deprecation and migration reason. | Not new-spawnable; load only through migration/fallback policy. |
| Debug-only definition | Hidden from normal player UI. | Show debug label/status in debug tools. | Normal gameplay interaction disabled unless debug profile is active. |

Fallback rules:

- If UI only has raw `item_id`, it may show `Unknown Item` plus `ID: {item_id}` as diagnostic fallback.
- Raw `item_id` must not be considered final player-facing text for accepted Phase 1 content.
- Missing icon fallback must be visually distinct from valid item icons.
- Placeholder fallback items must not gain normal modifiers, price, quality power, equip behavior or drop behavior.

### Quality Display Contract

Item Definition provides `quality_id` as semantic metadata. UI/presentation systems map it to final visuals.

Rules:

- Quality must have a textual or semantic label, not only a color.
- `quality_id` may support sorting, grouping, border style, loot emphasis and audio/VFX lookup through downstream systems.
- UI must not treat quality as implicit item power, drop chance, price, equip legality or combat power.
- Debug quality must be visually and semantically separated from normal player-facing quality.
- Phase 1 quality display should support `normal`, `fine`, `rare` and `debug` without requiring future `epic` / `legendary` tiers.

### Accessibility Requirements

Item definition metadata must enable accessible UI behavior:

- player-facing items require localizable name keys;
- item type and quality must be semantic enum values, not only icon/color information;
- missing/invalid/stale states must have text fallback;
- stack count must be represented as text, not only visual stacking;
- equipment eligibility and comparison deltas must not rely only on red/green color;
- icons must have text alternatives derived from item name, type and quality;
- tooltip/details must be reachable by keyboard/gamepad focus in downstream UI specs, not only mouse hover;
- raw debug details may be hidden from players but must be accessible to QA/debug surfaces.

### UX Flag

**📌 UX Flag — 物品定义系统**: This system has UI metadata requirements that affect loot labels, inventory cells, equipment tooltips, compare entry points and invalid-item fallback. In Phase 4 (Pre-Production), run `/ux-design` for the relevant pickup/inventory/equipment screens before writing UI implementation stories. Stories that reference UI should cite `design/ux/[screen].md`, not this GDD directly.

## Open Questions

| Question | Current Assumption | Owner | Target Resolution | Blocks Implementation? |
|---|---|---|---|---|
| OpenMir2 `StdItem` 最小字段、类型 ID、品质/分类语义和 `UserItem` 实例结构是什么？ | Phase 1 使用 `mvp_provisional` / `project_local` item definition schema，不声称 authentic。 | OpenMir2 行为映射 Spike owner + systems-designer | Before any OpenMir2-authentic item data story; after item source evidence reaches E3/E4 | Blocks authentic claims; does not block MVP provisional implementation. |
| OpenMir2 装备槽、穿戴入口和 item stat hook 如何映射到 Phase 1 `equip_slot_tags` 和 modifier payload？ | Phase 1 使用 candidate slot tags and `add_flat` modifier payload only. | systems-designer + gameplay-programmer + Equipment GDD owner | During `装备系统` GDD / ADR authoring | Blocks authentic equip slot claims; does not block provisional item definitions. |
| Phase 1 是否启用 `currency` item type，还是把金币作为未来经济/货币系统处理？ | 默认不要求 `currency`；只启用 `equipment` 和 `material`，除非掉落/拾取切片需要金币-like pickup。 | economy-designer | Before `掉落表系统` and `掉落与拾取系统` finalize MVP reward set | Soft blocker for currency drops only. |
| MVP 第一批物品需要多少件装备、多少材料、是否需要一件 rare showcase item？ | 最小要求：1 个 baseline equipped item/reference、1 个同类 clear upgrade equipment、1 个同类 sidegrade 或 weaker equipment、1 个 stackable material；rare/showcase item optional for loot-feedback showcase. | game-designer + economy-designer | Before MVP content smoke / first playable loot-loop test | Blocks content fixture completion, not schema implementation. |
| `equipment_min_modifier_rows` Phase 1 是否固定为 `1`，还是允许 cosmetic/placeholder equipment 为 `0`？ | 推荐 normal stat equipment ≥ 1；`0` 只允许 explicit cosmetic/placeholder/debug with validation evidence。 | systems-designer | Before implementation story for item validator | Blocks exact validation profile. |
| `equipment_max_modifier_rows` Phase 1 采用多少？ | 推荐 `1–4`。 | systems-designer | Before implementation story for item validator | Blocks exact validation profile. |
| Modifier value technical safe range 是否完全继承 Character Attributes `-9999–9999` / `0–9999` fixture convention？ | 默认继承 Character Attributes technical safe range until implementation ADR tightens numeric representation. | systems-designer + Character Attributes implementation owner | Before item validator implementation | Blocks numeric validation tests. |
| `item_id_format_policy` 是否要求英文 snake_case，还是允许中文/混合语义键？ | 推荐英文/ASCII snake_case stable semantic keys；中文只出现在 display/localization。 | technical-director + localization-lead | Before data pipeline implementation | Blocks durable ID validator. |
| Phase 1 item definitions 采用 JSON/YAML/GDScript factory/`.tres` 哪种 authoring envelope？ | GDD 不选择实现格式；runtime truth 必须 normalized DTO/table。`.tres` 如使用必须遵守 ADR-0013。 | technical-director + godot-specialist | During architecture-decision / implementation planning | Blocks implementation file format, not GDD contract. |
| 是否需要为 item definitions 创建专门 ADR：data representation, validation result schema, version/migration boundary, and authoring pipeline? | Required before implementation stories are Ready; a single combined ADR is acceptable for Phase 1. | technical-director | Before item definition implementation stories are Ready | Blocks implementation readiness. |
| Missing definition fallback 是 blocked-load only，还是允许 unresolved placeholder record for save/debug UI？ | 推荐 save/debug/UI 可显示 unresolved placeholder；normal loot/equip/use blocked。 | technical-director + qa-lead + UI owner | Before save/load or inventory UI implementation | Blocks fallback implementation details. |
| `quality_id` Phase 1 采用 `normal/fine/rare/debug`，未来是否迁移到 blue/purple/gold/赤金映射？ | Phase 1 uses `normal/fine/rare/debug`; presentation system maps to colors/tokens. | art-director + UI/loot feedback owner | During loot feedback / UI visual spec | Does not block item schema. |
| `presentation_tier` 是否独立于 `quality_id`，还是由下游完全从 quality 推导？ | 推荐 optional; if absent, downstream derives from quality. | art-director + ux-designer | Before loot feedback UI implementation | Soft UI/presentation blocker. |
| Item UI fallback 文案与 localization keys 何时确定？ | GDD defines semantic fallback; final text belongs to UI/localization. | ux-designer + localization-lead | Before inventory/equipment UI stories | Blocks polished UI, not item validation. |
| 是否需要 telemetry / debug event for invalid item definitions and unresolved references？ | 推荐 validation/debug reports first; telemetry later if analytics pipeline exists. | qa-lead + analytics-engineer | Before debug tools / analytics implementation | Soft. |
| Future fields such as durability, binding, sockets, affixes, enhancement, consumable effects and prices should be rejected or inert metadata? | Phase 1 default: reject unless schema explicitly marks inert metadata; no behavior activation. | technical-director + systems-designer | Before schema implementation | Blocks schema policy. |
| 是否需要注册具体 MVP item candidates？ | 暂不注册；item formulas 已同步到 `design/registry/entities.yaml`，实际 item candidates 等第一批 MVP fixture 命名后再注册。 | game-designer + economy-designer + producer | Before MVP content smoke / first playable loot-loop test | Blocks content fixture completion, not GDD contract. |
| 是否需要为 Item Definition 创建专门 review log？ | Yes；major review and targeted revision outcome should be recorded in `design/gdd/reviews/item-definition-system-review-log.md`. | producer / main session | Post-revision documentation sync | Does not block GDD content; supports handoff traceability. |

### Current Provisional Decisions

- Phase 1 item definitions are allowed to be `mvp_provisional` and do not need OpenMir2 E3/E4 evidence unless claiming authenticity.
- Phase 1 enabled normal types are `equipment` and `material`; `currency` is optional and unresolved.
- Phase 1 quality IDs are `normal`, `fine`, `rare`, and `debug`.
- Phase 1 equipment modifier operation is `add_flat` only.
- Phase 1 item definitions do not include prices, drop rates, random affixes, durability, binding, sockets, enhancement, combat power, final effective stats, UI layout, VFX parameters, or audio implementation values.
- Runtime truth must be normalized DTO/table data regardless of authoring envelope.
- Missing/invalid/stale item definitions must fail safe and must not become normal loot, normal equipment, or normal player-facing content without explicit fallback status.

### Review Notes

- Post-design validation 2026-06-04: three-agent review found no ownership-conflict rewrite required, but required top-level section ordering fix, formula result rows, registry sync, and implementation-blocking ADR follow-up.
- Run `/design-review design/gdd/item-definition-system.md` in a fresh Claude Code session after this GDD is complete.
- Run `/consistency-check` after item formulas are registered, because downstream systems will reference these formula names.
- Do not run `/architecture-review` in this same authoring session for the existing ADR set; architecture review should remain independent.
