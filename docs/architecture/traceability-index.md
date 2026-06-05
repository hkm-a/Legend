# Architecture Traceability Index
Last Updated: 2026-06-05
Engine: Godot 4.6.3

> Refresh note: `docs/architecture/architecture-review-refresh-2026-06-05.md` superseded the previous 53/18/12 coverage summary after ADR-0001 through ADR-0020 were accepted. This index was refreshed again on 2026-06-05 after ADR-0021 acceptance to register Drop Table TR rows and remove the false-negative omission of the approved Drop Table GDD.
>
> Scoped review note: `docs/architecture/architecture-review-adr-0006-0015-independent-2026-06-05.md` records an independent scoped PASS for ADR-0006 through ADR-0015 against ADR-0001 through ADR-0005, with pre-gate CONCERNS. Test/CI and UX/accessibility baselines have since been created, but the test runner remains a failing guard until an approved GUT/GdUnit4 addon runs real tests.
>
> Scoped review note: `docs/architecture/architecture-review-adr-0015-0021-traceability-refresh-2026-06-05.md` records a targeted FAIL for stale traceability/status tracking, not for ADR ownership conflict. The blocking traceability omission is remediated in this file and `tr-registry.yaml`; implementation Done remains blocked by the real Godot test runner.
>
> Scoped update note: ADR-0022 was independently reviewed, targeted fixes were applied, and it is now Accepted. Click Movement TR rows are covered at ADR-boundary level; implementation Done remains blocked by real Godot GDScript test runner evidence and story-level implementation tests.

## Coverage Summary

- Total requirements: 99
- Covered: 91 (92%)
- Partial: 8
- Gaps: 0

## Full Matrix

| Requirement ID | GDD | System | Requirement | ADR Coverage | Status |
|---|---|---|---|---|---|
| TR-concept-001 | design/gdd/game-concept.md | concept | Phase 1 必须验证 30 秒离线 loot loop：移动、攻击、掉落、拾取、背包、装备、属性/战力变化反馈。 | ADR-0001–ADR-0022 | ⚠️ Partial |
| TR-concept-002 | design/gdd/game-concept.md | concept | Godot 4.6.3 PC 原生客户端；2D/2.5D，需版本文档核对 API。 | ADR-0001–ADR-0015 | ✅ Covered |
| TR-concept-003 | design/gdd/game-concept.md | concept | OpenMir2 行为/协议对齐必须 source-first；MinimalMirClient 不可作为权威。 | ADR-0016 | ✅ Covered |
| TR-concept-004 | design/gdd/game-concept.md | concept | 传奇地图、资源、坐标、遮挡、Y-sort 需要早期 spike/架构验证。 | ADR-0001–ADR-0005, ADR-0020 | ✅ Covered |
| TR-concept-005 | design/gdd/game-concept.md | concept | Phase 1 不依赖完整联网；Godot PC Socket / 最小协议仅 spike 或未来扩展。 | ADR-0016 | ✅ Covered |
| TR-concept-006 | design/gdd/game-concept.md | concept | 掉落反馈与装备变强链路需要可见、可听、可快速判断价值。 | ADR-0004, ADR-0014, ADR-0015 | ⚠️ Partial |
| TR-concept-007 | design/gdd/game-concept.md | concept | 背包、装备对比、穿戴、属性变化提示必须控制范围，避免 UI/数据结构膨胀。 | ADR-0014, ADR-0015 | ⚠️ Partial |
| TR-systems-index-001 | design/gdd/systems-index.md | systems-index | 所有系统需遵守 Foundation → Core → Presentation 分层依赖，避免 God Object。 | ADR-0001–ADR-0022 | ✅ Covered |
| TR-systems-index-002 | design/gdd/systems-index.md | systems-index | UI 只读取或调用 gameplay 接口，不拥有 gameplay 数据。 | ADR-0007, ADR-0008, ADR-0014, ADR-0015, ADR-0018, ADR-0022 | ✅ Covered |
| TR-systems-index-003 | design/gdd/systems-index.md | systems-index | Phase 1 网络/协议保持 Spike/Future，不进入主线依赖。 | ADR-0001–ADR-0015 | ✅ Covered |
| TR-systems-index-004 | design/gdd/systems-index.md | systems-index | MVP 优先系统为 OpenMir2 映射、地图空间、角色属性、物品定义、掉落表、移动、战斗、拾取、背包、装备、HUD/反馈、存档。 | ADR-0001–ADR-0022 | ⚠️ Partial |
| TR-systems-index-005 | design/gdd/systems-index.md | systems-index | 掉落与拾取必须拆分事件链：死亡、掉落生成、地面物、拾取请求、背包接收。 | ADR-0017, ADR-0021 | ✅ Covered |
| TR-systems-index-006 | design/gdd/systems-index.md | systems-index | 装备系统通过 StatModifier 数据影响属性；属性系统拥有聚合结果，战力为只读显示。 | ADR-0006, ADR-0009, ADR-0014, ADR-0015 | ✅ Covered |
| TR-systems-index-007 | design/gdd/systems-index.md | systems-index | 地图坐标/阻挡/Y-sort 必须分离逻辑坐标、阻挡查询、视觉排序责任。 | ADR-0001–ADR-0005 | ✅ Covered |
| TR-systems-index-008 | design/gdd/systems-index.md | systems-index | 资源/地图转换管线需作为独立 Spike，Phase 1 可用临时素材。 | ADR-0020 | ✅ Covered |
| TR-openmir2-spike-001 | design/gdd/openmir2-behavior-mapping-spike.md | openmir2-spike | Spike 必须覆盖地图坐标、阻挡、移动、攻击、伤害/死亡、刷怪、掉落、地面物、拾取、背包、装备、最小协议入口。 | ADR-0016 | ⚠️ Partial |
| TR-openmir2-spike-002 | design/gdd/openmir2-behavior-mapping-spike.md | openmir2-spike | OpenMir2 原源码为 Tier 1 权威；MirServer 配置、mir2x、MinimalMirClient 不能单独决定行为。 | ADR-0016 | ✅ Covered |
| TR-openmir2-spike-003 | design/gdd/openmir2-behavior-mapping-spike.md | openmir2-spike | Phase 1 Required 行为至少 E3 才能作为 downstream contract；E2 以下不得作为规则依据。 | ADR-0016 | ✅ Covered |
| TR-openmir2-spike-004 | design/gdd/openmir2-behavior-mapping-spike.md | openmir2-spike | 每条 mapping item 必须结构化记录 source、symbols、证据等级、触发、前置条件、状态变化、失败条件、决策、信心、downstream contract。 | ADR-0016 | ✅ Covered |
| TR-openmir2-spike-005 | design/gdd/openmir2-behavior-mapping-spike.md | openmir2-spike | Adopt/Simplify/Exclude/Defer 必须明确，且 Adopt/Simplify 需输出 provisional contract。 | ADR-0016 | ✅ Covered |
| TR-openmir2-spike-006 | design/gdd/openmir2-behavior-mapping-spike.md | openmir2-spike | 未达 E3/E4 不得发明攻击距离、移动速度、掉率、背包格数、装备数值等具体玩法值。 | ADR-0001–ADR-0016, ADR-0021, ADR-0022 | ✅ Covered |
| TR-openmir2-spike-007 | design/gdd/openmir2-behavior-mapping-spike.md | openmir2-spike | 至少产出 Map Coordinate、Blocking、Movement、Attack、Damage、Death、Spawn、Drop、Ground Item、Pickup、Inventory、Equipment、Protocol contracts。 | ADR-0016 | ⚠️ Partial |
| TR-openmir2-spike-008 | design/gdd/openmir2-behavior-mapping-spike.md | openmir2-spike | 下游 GDD 若偏离 E3/E4 contract，必须标记 intentional divergence 并说明影响和验证方式。 | ADR-0016 | ✅ Covered |
| TR-map-space-001 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | 逻辑网格是 gameplay 权威；移动、阻挡、占位、刷怪、掉落、拾取、战斗距离输入必须基于 logical grid。 | ADR-0001 | ✅ Covered |
| TR-map-space-002 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | 必须分离 Logical Grid、World/Render、Screen/Input 三个坐标空间。 | ADR-0005 | ✅ Covered |
| TR-map-space-003 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | OpenMir2-derived 与 MVP provisional 规则必须标记；source-authentic 行为需 E3/E4。 | ADR-0001–ADR-0005, ADR-0016 | ✅ Covered |
| TR-map-space-004 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | 支持 MVP provisional playable contract：合成地图、单格 actor blocker、目标 reservation、掉落物不阻挡、同格物品容量 1、Y-sort tie-break。 | ADR-0001–ADR-0005, ADR-0017 | ✅ Covered |
| TR-map-space-005 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | Map Coordinate Contract 必须包含 map_id、bounds、cell→render anchor、input→cell、失败语义。 | ADR-0001, ADR-0005 | ✅ Covered |
| TR-map-space-006 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | 越界和未加载数据必须 fail-closed，区分 out_of_bounds 与 unknown_or_unloaded。 | ADR-0002, ADR-0005 | ✅ Covered |
| TR-map-space-007 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | Map Blocking Contract 必须分离 static blocking、dynamic blocking、item occupancy、visual-only obstruction。 | ADR-0001, ADR-0003 | ✅ Covered |
| TR-map-space-008 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | 空间查询必须返回 typed/structured result：status、primary_reason、secondary_reasons、query_context、cell_facts、retry_hint。 | ADR-0002 | ✅ Covered |
| TR-map-space-009 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | 每张 playable map 必须有 QA/authoring metadata，可验证 player start、spawn、item-placeable、drop-readable、visual obstruction、Y-sort anchor。 | ADR-0001, ADR-0020 | ✅ Covered |
| TR-map-space-010 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | Player 与普通怪在 MVP 中是单格 blocking actors。 | ADR-0003 | ✅ Covered |
| TR-map-space-011 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | movement reservation 由地图空间权威拥有，source 保持 occupied，target reserved，commit/cancel 原子更新。 | ADR-0003 | ✅ Covered |
| TR-map-space-012 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | occupancy/reservation/item mutations 必须经过 deterministic authoritative update ordering。 | ADR-0003 | ✅ Covered |
| TR-map-space-013 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | 掉落物占 logical cell、参与 pickup/Y-sort，但 MVP 中不阻挡 actor；pickup-complete 阶段同格容量为 1。 | ADR-0003, ADR-0017 | ✅ Covered |
| TR-map-space-014 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | 掉落物必须保持 readability floor；空间放置失败不能删除、重 roll 或静默移动奖励。 | ADR-0004, ADR-0017, ADR-0021 | ✅ Covered |
| TR-map-space-015 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | gameplay query 不得使用瞬时 sprite pixel position；必须使用 logical cell 和 anchor。 | ADR-0001, ADR-0005 | ✅ Covered |
| TR-map-space-016 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | diagonal、corner-cutting、distance metric、pickup/attack range 等保持 evidence-gated。 | ADR-0016, ADR-0019, ADR-0022 | ✅ Covered |
| TR-map-space-017 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | spawn/drop/pickup 需要通过 map-space 查询合法性，但具体 spawn/drop/pickup 规则归下游系统。 | ADR-0001–ADR-0003, ADR-0017, ADR-0019, ADR-0021 | ✅ Covered |
| TR-map-space-018 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | Y-sort 使用 stable anchor、type rank、stable instance ID；视觉排序不得影响 gameplay。 | ADR-0004 | ✅ Covered |
| TR-map-space-019 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | 逻辑地图数据是 gameplay 权威；visual tiles、sprites、physics helpers 不得覆盖逻辑状态。 | ADR-0001, ADR-0005 | ✅ Covered |
| TR-map-space-020 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | 查询/排序/日志/debug overlay 必须有性能 guardrails；正常 gameplay 不全图每帧扫描。 | ADR-0001–ADR-0004 | ✅ Covered |
| TR-map-space-021 | design/gdd/map-coordinate-blocking-y-sort-system.md | map-space | 实现前需要 ADR 覆盖 map representation、query schema、input projection、deterministic Y-sort、occupancy ordering。 | ADR-0001–ADR-0005 | ✅ Covered |
| TR-attributes-001 | design/gdd/character-attributes-system.md | attributes | 属性系统拥有属性输入语义、registry、计算、校验、只读 snapshot、版本、delta、invalid/stale/debug 语义。 | ADR-0006–ADR-0014 | ✅ Covered |
| TR-attributes-002 | design/gdd/character-attributes-system.md | attributes | 装备、HUD、combat、AI、save/load、scene node 不得直接写最终属性、base stats、derived stats 或 snapshot 内部字段。 | ADR-0006, ADR-0007, ADR-0009 | ✅ Covered |
| TR-attributes-003 | design/gdd/character-attributes-system.md | attributes | Phase 1 stat registry 必须区分 required、active、visible、debug-only、reserved、modifier-targetable、actor-type-specific 字段。 | ADR-0006, ADR-0011 | ✅ Covered |
| TR-attributes-004 | design/gdd/character-attributes-system.md | attributes | Growth handoff 必须提供 reason、salience、primary comparison stat、combat_power before/after/delta、visible/hidden deltas、invalid/stale status。 | ADR-0014 | ✅ Covered |
| TR-attributes-005 | design/gdd/character-attributes-system.md | attributes | 装备 preview 必须选择 pure preview、equipment-owned preview 或 no preview；preview 不得 mutate authoritative state 或增加 snapshot_version。 | ADR-0008, ADR-0009, ADR-0014, ADR-0018 | ✅ Covered |
| TR-attributes-006 | design/gdd/character-attributes-system.md | attributes | 属性层必须分离 identity、base_stats、current_resources、derived_stats、stat_modifiers、snapshot、debug_trace。 | ADR-0006, ADR-0007, ADR-0010 | ✅ Covered |
| TR-attributes-007 | design/gdd/character-attributes-system.md | attributes | Structural rebuild 与 resource mutation 是两条路径；resource-only HP/MP 变化不得重聚合 modifiers。 | ADR-0009 | ✅ Covered |
| TR-attributes-008 | design/gdd/character-attributes-system.md | attributes | Equipment replacement、level-up、load rebuild、spawn init 必须 transaction-like，不能暴露 intermediate state。 | ADR-0009 | ✅ Covered |
| TR-attributes-009 | design/gdd/character-attributes-system.md | attributes | Validation pipeline 分 Stage A-D，invalid config/source/calculation/output 必须 structured failure。 | ADR-0011, ADR-0012 | ✅ Covered |
| TR-attributes-010 | design/gdd/character-attributes-system.md | attributes | Invalid modifier policy 必须拒绝 unsupported op、unknown stat、duplicate source key、source-authentic without evidence 等。 | ADR-0006, ADR-0009, ADR-0012 | ✅ Covered |
| TR-attributes-011 | design/gdd/character-attributes-system.md | attributes | AttributeSnapshot 必须 immutable/read-only；不能向消费者暴露可变 Dictionary/Array 权威状态。 | ADR-0007 | ✅ Covered |
| TR-attributes-012 | design/gdd/character-attributes-system.md | attributes | Events 是 data-first domain events；核心逻辑需 scene-tree independent，不依赖 Autoload。 | ADR-0008 | ✅ Covered |
| TR-attributes-013 | design/gdd/character-attributes-system.md | attributes | UI/presentation 只能消费 metadata/deltas/status，不拥有布局/VFX/audio，也不得展示 invalid/stale 为正常成长。 | ADR-0014 | ✅ Covered |
| TR-attributes-014 | design/gdd/character-attributes-system.md | attributes | Save/Load 持久化 authoritative inputs，不以 derived snapshot 为 truth。 | ADR-0010 | ✅ Covered |
| TR-attributes-015 | design/gdd/character-attributes-system.md | attributes | 实现前必须有 9 个 ADR/技术设计：data representation、snapshot API、event contract、transaction、persistence、fixture config、GUT strategy、Resource policy、combat power ownership。 | ADR-0006–ADR-0014 | ✅ Covered |
| TR-attributes-016 | design/gdd/character-attributes-system.md | attributes | Structural rebuild 不得 per-frame；aggregation O(M+S)；hot path 使用 compact stat IDs。 | ADR-0006, ADR-0009, ADR-0011, ADR-0012 | ✅ Covered |
| TR-attributes-017 | design/gdd/character-attributes-system.md | attributes | 必须公式化并测试 effective_stat、effective_stat_pair、current_resource_after、snapshot_valid、attribute_delta、snapshot_delta、combat_power。 | ADR-0012, ADR-0014 | ✅ Covered |
| TR-attributes-018 | design/gdd/character-attributes-system.md | attributes | 阻塞测试证据在 tests/unit/character_attributes/，覆盖 AC-01 至 AC-17 的逻辑/fixture 部分。 | ADR-0012 | ✅ Covered |
| TR-item-definition-001 | design/gdd/item-definition-system.md | item-definition | 物品定义系统拥有稳定 item_id、template 合同、type/quality/display/stack/equipment/modifier/source/evidence/lifecycle。 | ADR-0015 | ✅ Covered |
| TR-item-definition-002 | design/gdd/item-definition-system.md | item-definition | 掉落、背包、装备、UI、存档不得各自复制物品定义字段作为事实来源。 | ADR-0015 | ✅ Covered |
| TR-item-definition-003 | design/gdd/item-definition-system.md | item-definition | 每个可引用物品模板必须有稳定唯一 item_id；不得因显示/数值调整改变。 | ADR-0015 | ✅ Covered |
| TR-item-definition-004 | design/gdd/item-definition-system.md | item-definition | 必须区分 template、instance、inventory stack、equipped reference；instance 不得重定义 template truth。 | ADR-0015 | ✅ Covered |
| TR-item-definition-005 | design/gdd/item-definition-system.md | item-definition | Phase 1 item types 至少支持 equipment、material；currency optional；debug/reserved 不得进入正常 loot。 | ADR-0015 | ✅ Covered |
| TR-item-definition-006 | design/gdd/item-definition-system.md | item-definition | quality 是分类/展示/显著性元数据，不是 power、drop rate、price 或 combat power authority。 | ADR-0015 | ✅ Covered |
| TR-item-definition-007 | design/gdd/item-definition-system.md | item-definition | player-facing item 必须有 display metadata 和 Phase 1 player-facing minimum metadata。 | ADR-0015 | ✅ Covered |
| TR-item-definition-008 | design/gdd/item-definition-system.md | item-definition | stack_policy/max_stack_size 必须明确；equipment Phase 1 non-stackable。 | ADR-0015 | ✅ Covered |
| TR-item-definition-009 | design/gdd/item-definition-system.md | item-definition | equipment item 必须有 equipment data block，但 Item Definition 不决定 equip legality。 | ADR-0015 | ✅ Covered |
| TR-item-definition-010 | design/gdd/item-definition-system.md | item-definition | Phase 1 modifier payload 仅允许 add_flat，目标必须是 Character Attributes modifier-targetable stat，禁止 current resources。 | ADR-0006, ADR-0014, ADR-0015 | ✅ Covered |
| TR-item-definition-011 | design/gdd/item-definition-system.md | item-definition | source/evidence labels 必填；openmir2_verified 必须有 E3/E4 evidence ref。 | ADR-0015 | ✅ Covered |
| TR-item-definition-012 | design/gdd/item-definition-system.md | item-definition | definition validation 必须覆盖 identity、display、stack、type、equipment、evidence，失败时不得 spawnable。 | ADR-0015 | ✅ Covered |
| TR-item-definition-013 | design/gdd/item-definition-system.md | item-definition | runtime truth 必须 normalized DTO/table，.tres 只可作为 authoring envelope。 | ADR-0013, ADR-0015 | ✅ Covered |
| TR-item-definition-014 | design/gdd/item-definition-system.md | item-definition | 下游必须通过 status-bearing query/projection results 消费定义，不得读 raw authoring rows。 | ADR-0015 | ✅ Covered |
| TR-item-definition-015 | design/gdd/item-definition-system.md | item-definition | deprecated definitions 不可删除或静默 remap；save/debug/migration 需显式策略。 | ADR-0015 | ✅ Covered |
| TR-item-definition-016 | design/gdd/item-definition-system.md | item-definition | MVP provisional item set 至少含 baseline、upgrade、sidegrade/weaker、stackable material，可选 rare showcase。 | ADR-0015 | ⚠️ Partial |
| TR-item-definition-017 | design/gdd/item-definition-system.md | item-definition | Drop Table、Drop/Pickup、Inventory、Equipment、Attributes、UI、Save/Load 的边界必须遵守 Item Definition source-of-truth。 | ADR-0015, ADR-0017, ADR-0018, ADR-0021 | ✅ Covered |
| TR-item-definition-018 | design/gdd/item-definition-system.md | item-definition | 公式/验证必须覆盖 stack_quantity_valid、modifier_payload_row_valid、modifier_payload_valid、lookup、semantic validity、profile eligibility、spawn eligibility。 | ADR-0015 | ✅ Covered |
| TR-item-definition-019 | design/gdd/item-definition-system.md | item-definition | 正常 gameplay/UI lookup 不得全扫 authoring rows 或每次重跑全验证；需 validated indexed runtime data。 | ADR-0015 | ✅ Covered |
| TR-item-definition-020 | design/gdd/item-definition-system.md | item-definition | 实现前需要 Item Definition ADR 覆盖 data representation、validation result schema、version/migration boundary、authoring pipeline。 | ADR-0015 | ✅ Covered |
| TR-item-definition-021 | design/gdd/item-definition-system.md | item-definition | Item UI projection 必须提供 fallback/status keys，且 accessibility metadata 不得只依赖颜色/icon。 | ADR-0015 | ⚠️ Partial |
| TR-drop-table-001 | design/gdd/drop-table-system.md | drop-table | Drop Table 必须拥有 reward source → drop table mapping、roll groups、weights、no-drop policy、quantity policy、validation、deterministic roll results 和 QA replay provenance。 | ADR-0021 | ✅ Covered |
| TR-drop-table-002 | design/gdd/drop-table-system.md | drop-table | Drop Table 不得拥有 item template truth、ground drop lifecycle、map placement、pickup、inventory receive、equipment legality、UI/VFX/audio 或 final item instance identity。 | ADR-0015, ADR-0017, ADR-0018, ADR-0021 | ✅ Covered |
| TR-drop-table-003 | design/gdd/drop-table-system.md | drop-table | Every item row 必须通过 Item Definition spawn_eligible_reference 与 stack/equipment quantity constraints 验证。 | ADR-0015, ADR-0021 | ✅ Covered |
| TR-drop-table-004 | design/gdd/drop-table-system.md | drop-table | Drop Table 必须使用 deterministic weighted selection、inclusive RNG convention、deterministic quantity roll、exact boundary behavior 和 expected acquisition formulas。 | ADR-0021 | ✅ Covered |
| TR-drop-table-005 | design/gdd/drop-table-system.md | drop-table | Drop Table 必须把非证据支持的 rates/items 标记为 mvp_provisional，并阻止缺少 ADR-0016 accepted readiness 的 openmir2_verified claims。 | ADR-0016, ADR-0021 | ✅ Covered |
| TR-drop-table-006 | design/gdd/drop-table-system.md | drop-table | Phase 1 QA 必须使用 fake RNG、exact boundary tests、deterministic fixtures 和真实 Godot GDScript test runner；guard-only runner 不可作为 passing evidence。 | ADR-0012, ADR-0021 | ✅ Covered |

| TR-click-movement-001 | design/gdd/click-movement-system.md | click-movement | 点击移动必须使用 named input action，并在同一输入事件上通过 UI/input gate 后才允许世界投影；UI-consumed 或 unknown ownership 必须 fail closed。 | ADR-0005, ADR-0022 | ✅ Covered |
| TR-click-movement-002 | design/gdd/click-movement-system.md | click-movement | 所有点击坐标必须通过 MapProjection 转换为 logical Vector2i candidate；点击移动不得本地转换或 fallback 到最近格、上一格或 (0,0)。 | ADR-0005, ADR-0022 | ✅ Covered |
| TR-click-movement-003 | design/gdd/click-movement-system.md | click-movement | Phase 1 允许远端点击 destination，但实际执行必须拆为 orthogonal-only 单步；diagonal step、corner-cutting、cross-map movement 保持 unresolved/evidence-gated。 | ADR-0019, ADR-0022 | ✅ Covered |
| TR-click-movement-004 | design/gdd/click-movement-system.md | click-movement | 每个 movement step 必须通过 MovementLegalityService read-only preflight，然后通过 MapSpaceState RESERVE_MOVEMENT / COMMIT_MOVEMENT / CANCEL_MOVEMENT mutation authority。 | ADR-0003, ADR-0019, ADR-0022 | ✅ Covered |
| TR-click-movement-005 | design/gdd/click-movement-system.md | click-movement | 无效、阻挡、no-path replacement click 不得取消当前有效 movement；MovingStep 中 valid replacement 作为 pending destination，当前 step commit 后从新 committed cell 重规划。 | ADR-0022 | ✅ Covered |
| TR-click-movement-006 | design/gdd/click-movement-system.md | click-movement | Path query 必须返回 typed path-found/no-path/path-limit/unresolved result DTO，包含 ordered path cells、budget facts、canonical reasons 和 deterministic tie-break evidence。 | ADR-0022 | ✅ Covered |
| TR-click-movement-007 | design/gdd/click-movement-system.md | click-movement | 点击移动不得使用 Godot Navigation、Physics、TileMapLayer、Node2D position 或 Y-sort 作为 movement gameplay authority。 | ADR-0001, ADR-0004, ADR-0019, ADR-0022 | ✅ Covered |
| TR-click-movement-008 | design/gdd/click-movement-system.md | click-movement | Movement speed、path search budgets 和 MVP policy knobs 必须数据驱动；invalid speed/path budget config 必须 fail validation，不得硬编码或 silent clamp。 | ADR-0022 | ✅ Covered |
| TR-click-movement-009 | design/gdd/click-movement-system.md | click-movement | 移动反馈必须 typed、可区分 invalid/same-cell/blocked/reserved/no-path/command failure/unresolved；玩家反馈不得只依赖颜色或音频。 | ADR-0002, ADR-0022 | ✅ Covered |
| TR-click-movement-010 | design/gdd/click-movement-system.md | click-movement | 点击移动 public logic 必须通过 testable dependencies/adapters 消费 projection、legality、map state、planner、tuning、actor/status facts；direct singleton-only core logic 不可作为 Done。 | ADR-0022 | ✅ Covered |


## Known Gaps

None after the 2026-06-05 coherent ADR-set refresh: the previous hard architecture gaps are covered at ADR-boundary level by Accepted ADR-0016 through ADR-0022, and the matrix now contains no `❌ Gap` rows.

## Scoped Review — ADR-0006 through ADR-0015

- Report: `docs/architecture/architecture-review-adr-0006-0015-independent-2026-06-05.md`
- Verdict: PASS for scoped ADR architecture coverage, with pre-gate CONCERNS.
- Scope: Character Attributes and Item Definition ADRs (`ADR-0006` through `ADR-0015`) checked against the map-space baseline (`ADR-0001` through `ADR-0005`) and the current GDD/TR registry context.
- Result: no new scoped ADR coverage gaps, no blocking cross-ADR conflict, and no Godot 4.6.3 compatibility blocker.
- Registry impact: no TR-ID additions, renumbering, deprecations, or wording revisions required.
- Pre-gate concerns after remediation: test/CI, UX/accessibility baseline files, and architecture overview now exist; real passing test evidence is still blocked until an approved GUT/GdUnit4 runner is installed and wired.

## Remaining Partial Coverage

- ⚠️ `TR-openmir2-spike-001` remains Partial: ADR-0016 defines governance, but actual accepted OpenMir2 behavior contracts are not yet produced for every required domain.
- ⚠️ `TR-openmir2-spike-007` remains Partial: the contract registry model exists, but actual Map Coordinate, Blocking, Movement, Attack, Damage, Death, Spawn, Drop, Ground Item, Pickup, Inventory, Equipment, and Protocol contracts are not all present as accepted evidence-backed artifacts.
- ⚠️ Additional partial requirements remain around the full 30-second loop, MVP priority-system completion, concrete MVP item content, UX/accessibility projection details, and implementation/test evidence. See the latest coherent ADR-set review report for the refreshed summary.

## Superseded Requirements

None — this is the first populated TR registry pass.
