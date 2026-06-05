# ADR-0015: Item Definition Runtime Data, Validation, Query, and Versioning

## Status

Accepted

## Date

2026-06-04

## Last Verified

2026-06-04

## Decision Makers

hkm + Claude Code Game Studios

## Summary

This ADR defines the implementation-blocking architecture for the Item Definition system. Phase 1 item definitions are authored through project-owned fixture/config payloads or future authoring envelopes, validated and normalized into immutable-by-contract runtime tables keyed by stable `item_id` and `definition_version`, and exposed through status-bearing query/projection DTOs. Item Definition owns template truth, display metadata keys, stack/equipment/modifier payload declarations, lifecycle/evidence labels, lookup/indexing, and deterministic validation. It does not own drop rates, inventory capacity, equip legality, final attributes, combat power, economy value, UI layout, final localization text, or OpenMir2 authenticity without accepted evidence.

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Core / Data / Scripting / UI Handoff |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff; this ADR avoids post-cutoff-only APIs in the Phase 1 runtime core and requires local verification for Resource/FileAccess/copy behavior if external authoring is added. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md`; `design/gdd/item-definition-system.md`; `design/gdd/character-attributes-system.md`; `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`; `docs/architecture/adr-0011-attribute-fixture-config-loading-strategy-mvp-provisional-values.md`; `docs/architecture/adr-0013-attribute-godot-resource-duplication-shared-reference-policy-if-tres-resources-are-used.md`; `docs/architecture/adr-0014-attribute-combat-power-main-stat-display-proxy-ownership.md` |
| **Post-Cutoff APIs Used** | None required. Phase 1 does not require Godot 4.5+ `@abstract`, variadic arguments, `Resource.duplicate_deep()`, SceneTree focus APIs, or UI-specific post-cutoff APIs. |
| **Verification Required** | Verify `StringName` key normalization; `RefCounted` DTO aliasing/defensive-copy behavior; `Array`/`Dictionary` copy behavior; deterministic sort/failure ordering; integer range and overflow checks; typed signal adapters if added later; no deprecated string-based `connect()`; no ResourceLoader/FileAccess/SceneTree/Autoload dependency in formula/contract tests; `.tres`/Resource behavior only in supplemental importer tests if that path is introduced. |

> **Note**: If Knowledge Risk is MEDIUM or HIGH, this ADR must be re-validated if the project upgrades engine versions. Flag it as "Superseded" and write a new ADR.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | Approved Item Definition GDD: `design/gdd/item-definition-system.md`; ADR-0006 Attribute Data Representation and Stat ID Typing; ADR-0011 Attribute Fixture Config Loading Strategy; ADR-0013 Attribute Godot Resource Duplication Policy; ADR-0014 Attribute Combat Power and Main Stat Display Proxy Ownership. |
| **Enables** | Item Definition runtime table implementation; item definition fixture/config loader; item reference query API; UI projection DTOs; item definition validation tests; drop table validation against item references; inventory/equipment/save/debug stories that consume item template truth. |
| **Blocks** | Any implementation story that defines, loads, validates, indexes, queries, displays, spawns, saves, migrates, or consumes item definitions, item template metadata, stack policy, equipment candidate metadata, item modifier payload rows, lifecycle/evidence labels, or item definition query/projection DTOs. |
| **Ordering Note** | This ADR is for Item Definition architecture only. It does not replace future Drop Table, Drop/Pickup, Inventory, Equipment, Save/Load, UI/HUD, Localization, Economy, or OpenMir2 evidence ADRs. Those systems must consume Item Definition through the contracts defined here instead of duplicating template truth. |

## Context

### Problem Statement

The Phase 1 loot loop requires monsters to drop recognizable items, inventory/equipment systems to reference them safely, UI to present them without guessing, Equipment to resolve modifier payloads, Character Attributes to validate and aggregate resolved modifier sources, and Save/Load to preserve item references across sessions.

Without an explicit architecture decision, implementers could store item truth in raw dictionaries, mutable `.tres` Resource graphs, duplicated UI tables, drop table copies, save payload copies, or equipment-specific shadow schemas. That would create inconsistent item IDs, unsafe OpenMir2 authenticity claims, mutable shared-state bugs, invalid modifier payloads entering Attributes, and UI/economy/equipment systems treating display metadata or combat power as authority.

### Current State

No Item Definition implementation exists. The Item Definition GDD is approved for ADR authoring and explicitly blocks implementation pending an Item Definition ADR. Character Attributes ADRs already define stat ID typing, fixture/config normalization, Resource boundaries, and combat power display proxy ownership. This ADR mirrors those guardrails for item template truth and defines how downstream systems query item definitions.

### Constraints

- Phase 1 is an offline 2D/2.5D loot-loop slice targeting PC and GDScript.
- Item definitions must be fast to author for MVP provisional content but still deterministic, versioned, source-labeled, and testable.
- Runtime systems must consume normalized item data, not mutable authoring graphs.
- UI needs semantic display/projection data but does not own final item truth.
- Equipment needs modifier payload inputs but owns equip legality and modifier-source resolution.
- Character Attributes owns final stat formulas, snapshots, deltas, and combat power; Item Definition must not precompute them.
- OpenMir2-authentic item claims require E3/E4 evidence from the OpenMir2 mapping process.
- Formula/contract tests must remain scene-tree-independent and not depend on ResourceLoader, FileAccess, UI, Autoload, or signal order.

### Requirements

- Provide a canonical runtime representation for item template truth.
- Validate identity, lifecycle, display metadata, type, quality, stack policy, equipment candidate data, modifier payloads, source/evidence labels, and runtime eligibility before normal gameplay use.
- Expose status-bearing query/projection DTOs instead of raw rows or mutable collections.
- Preserve stable `item_id` and explicit `definition_version` semantics for save/load and migration.
- Support deterministic indexed lookup by item ID, version, lifecycle/eligibility, type, quality, and equipment candidate metadata.
- Keep `.tres`/Resource and future external files as authoring envelopes only, never runtime authority.
- Provide formula/contract test requirements for validation, query, DTO read-only behavior, and invalid reference handling.

## Decision

Phase 1 Item Definition uses project-owned typed fixture/config factories plus primitive semantic-key payloads as the initial authoring/loading strategy. Those payloads are normalized into immutable-by-contract runtime item definition tables before gameplay use. Later JSON, `.tres`, editor tooling, or imported OpenMir2 data may be added only as external authoring envelopes that pass through the same validation and normalization boundary.

Runtime Item Definition authority is:

- stable `item_id` semantic keys stored as `StringName` or validated equivalent string-key type;
- explicit `definition_version` and schema/source-status versions;
- immutable-by-contract DTO/table rows;
- project-owned indexes built from validated rows;
- status-bearing query/projection DTOs returned to consumers.

Runtime Item Definition authority is **not**:

- raw JSON;
- raw `Dictionary[StringName, Variant]` maps;
- mutable Godot `Resource` / `.tres` graphs;
- ResourceLoader cache entries;
- UI widget state;
- drop table copied fields;
- inventory/equipment/save copied template fields;
- enum ordinals as durable item truth;
- combat power, final effective stats, sell value, drop weight, or OpenMir2-authentic truth without evidence.

### Ownership Boundary

Item Definition owns:

- `item_id` naming, uniqueness, lifecycle, version, and deprecation semantics;
- template classification: `item_type`, `quality_id`, stack policy, instance requirements, and inert tags;
- player-safe display metadata keys and presentation handoff tokens;
- equipment candidate declarations: category, candidate slot tags, `main_comparison_hint`, and declared modifier payload rows;
- source status, evidence labels, and lifecycle status validation;
- normalized runtime item definition tables and indexes;
- item reference query/projection DTOs and structured invalid reasons.

Item Definition does not own:

- drop chance, drop pools, quantity rolls, loot guarantees, or source monster rules;
- ground drop placement, pickup radius, despawn, click priority, or map occupancy;
- inventory capacity, slot layout, sorting, merge/split transaction rules, or drag/drop UI;
- equip legality, slot occupancy, replacement order, active equipped source set, or equip transactions;
- final Attribute snapshots, effective stats, deltas, current resources, combat power, damage, DPS, TTK, AI targeting, or growth celebration;
- shop price, recycle output, crafting requirements, rarity economics, or item valuation;
- final localized text, UI layout, icon asset fallback algorithm, colors, VFX/audio playback, animation timing, or accessibility presentation;
- OpenMir2 evidence gathering or authentic mapping approval.

### Runtime Data Representation

Each normalized item definition row must include, at minimum:

- `item_id: StringName`;
- `definition_version: StringName` or equivalent explicit version label;
- `item_schema_version`;
- `item_type` typed enum/validated ID;
- `quality_id: StringName` or typed quality ID with semantic-key mapping;
- `source_status` and conditional `evidence_ref`;
- `lifecycle_status`;
- display metadata keys: `display_name_key`, `description_key` where applicable, `icon_key`, world/pickup/presentation tokens where applicable;
- stack policy fields: `stack_policy`, `max_stack_size`, optional `quantity_unit`;
- equipment candidate block for equipment rows;
- modifier payload row DTOs for stat equipment;
- debug-only ordered validation metadata and source row references where safe.

Runtime rows are immutable-by-contract. Constructors/factories validate required fields, copy inbound arrays/dictionaries, expose only getters, return defensive copies for collection output, and provide no public mutable vars, setters, or mutator methods.

`item_id` remains the cross-system semantic reference. If compact integer `ItemDefinitionId` handles are introduced for hot-path indexing, they are runtime cache handles only and must be derived from validated `item_id` tables. They are not durable save/config truth unless a later versioned migration ADR explicitly approves that representation.

### Validation Pipeline

Item definition loading follows this staged pipeline:

```text
Authoring payload / fixture row / future import envelope
        |
        | schema decode; no gameplay trust
        v
Primitive semantic item payload
        |
        | deterministic validation stages
        v
Normalized immutable item definition rows
        |
        | index construction and cross-reference validation
        v
ItemDefinitionRuntimeCatalog
        |
        | status-bearing query/projection DTOs
        v
Drop tables / drop-pickup / inventory / equipment / UI / save-load / debug
```

Validation fails deterministically before a row can become normal spawnable for:

- missing or duplicate `item_id`;
- invalid item ID format;
- missing or unsupported `definition_version` / schema version;
- invalid or missing `item_type`, `quality_id`, `source_status`, or `lifecycle_status`;
- `openmir2_verified` without accepted evidence reference;
- OpenMir2-authentic wording or status before E3/E4 evidence approval;
- player-facing row missing required display metadata keys;
- debug-only row marked normal player-facing or normal spawnable;
- invalid stack policy or max stack size;
- equipment row missing equipment data;
- non-equipment row containing active equipment modifier payload in Phase 1;
- unknown Character Attributes stat ID in modifier payload;
- modifier target not modifier-targetable;
- modifier target is `health_current` or `mana_current`;
- unsupported modifier operation, including percent/multiply/override in Phase 1;
- modifier value missing, non-integer, out of configured safe range, or overflow-prone;
- equipment candidate row lacking player-facing comparison metadata when normal spawnable;
- missing presentation tokens required by the GDD for normal droppable items;
- version mismatch without explicit migration/fallback policy;
- deprecated/debug/blocked-unconfirmed rows included in normal spawnable indexes.

Failure ordering comes from explicit validation stage order, failure reason registry order, field order, and sorted semantic keys. Implementation must not rely on raw `Dictionary` iteration order, Resource property order, filesystem order, or editor display order.

### Lifecycle and Runtime Eligibility

A definition row may be schema-valid but still not eligible for normal gameplay. Runtime query results separate:

- reference resolution status;
- semantic validation status;
- lifecycle status;
- runtime eligibility status;
- UI availability status;
- debug reason set.

Normal drop tables may reference only rows that resolve to active, validated, normal-spawnable definitions. Debug-only, invalid, deprecated, blocked-unconfirmed, and missing definitions must not silently become normal loot.

Deprecated definitions are retained for save/debug/migration lookup. They are not allowed in new normal drop tables unless a later migration/debug-only policy explicitly permits a limited recovery path. A deprecated item must not be silently remapped to another item if player-facing meaning changes.

### Query and Projection Contract

Consumers access definitions through `ItemDefinitionRuntimeCatalog` or equivalent query service. They do not retain raw authoring rows or mutable runtime tables.

Conceptual query DTOs:

```gdscript
class_name ItemDefinitionQueryResult
extends RefCounted

func is_resolved() -> bool:
    pass

func get_request_item_id() -> StringName:
    pass

func get_request_definition_version() -> StringName:
    pass

func get_reference_resolution_status() -> int:
    pass

func get_definition_validation_status() -> int:
    pass

func get_runtime_eligibility_status() -> int:
    pass

func get_ui_availability_status() -> int:
    pass

func get_definition_view() -> ItemDefinitionView:
    pass

func get_debug_reasons_copy() -> Array[int]:
    pass
```

```gdscript
class_name ItemDefinitionView
extends RefCounted

func get_item_id() -> StringName:
    pass

func get_definition_version() -> StringName:
    pass

func get_item_type() -> int:
    pass

func get_quality_id() -> StringName:
    pass

func get_stack_policy() -> int:
    pass

func get_max_stack_size() -> int:
    pass

func get_player_safe_display() -> ItemPlayerSafeDisplayView:
    pass

func get_equipment_candidate_view() -> ItemEquipmentCandidateView:
    pass

func get_presentation_view() -> ItemPresentationView:
    pass
```

```gdscript
class_name ItemEquipmentCandidateView
extends RefCounted

func is_equipment_candidate() -> bool:
    pass

func get_equipment_category() -> StringName:
    pass

func get_candidate_slot_tags_copy() -> Array[StringName]:
    pass

func get_main_comparison_hint() -> StringName:
    pass

func get_visible_modifier_facts_copy() -> Array[ItemVisibleModifierFact]:
    pass

func get_modifier_payload_rows_copy() -> Array[ItemModifierPayloadRow]:
    pass
```

Interface names are conceptual. Exact filenames, enum names, and constructor APIs may be refined during implementation while preserving status, version, read-only, and ownership boundaries.

UI projection DTOs must be narrower than full definition rows:

- `LootLabelItemView` exposes only label/type/quality keys, world visual key, salience, audio family, and fallback status.
- `InventoryCellItemView` exposes icon/display keys, quantity display eligibility, type/quality labels, stack policy, and availability status.
- `TooltipItemView` exposes display keys, classification, safe visible modifier facts, and player-safe invalid/deprecated states.
- `EquipmentComparisonItemView` exposes item identity, equipment candidate metadata, main comparison hint, and the handoff target for Equipment / Character Attributes preview. It must not compute final deltas directly from modifier rows.

### Indexing and Lookup

Runtime catalog construction creates validated indexes for:

- exact `(item_id, definition_version)` lookup;
- latest supported active version by `item_id`, only where policy allows defaulting;
- lifecycle/runtime eligibility filtering;
- normal-spawnable item IDs for drop table validation;
- item type filtering;
- quality filtering;
- equipment candidate category / slot tag filtering;
- debug and invalid reason inspection.

Indexes are generated from normalized rows after validation. They are not manually maintained shadow truth. Index construction fails deterministically on duplicate active rows, ambiguous latest-version policy, unsupported versions, or cross-index inconsistencies.

### Modifier Payload Boundary

Item Definition declares modifier payload rows as template input data. Equipment resolves equipped item instances into active modifier sources. Character Attributes validates and aggregates those sources.

Phase 1 modifier payload rows may use only `add_flat`. Rows must reference Character Attributes semantic stat IDs from ADR-0006-compatible registry data and must not target `health_current` or `mana_current`. Unsupported operations fail validation; they are not ignored, coerced, or downgraded.

Item Definition must not include final effective stats, derived stats, combat power, DPS, TTK, sell value, equip recommendations, or build optimization. It may provide `main_comparison_hint` as a semantic handoff for UI/Equipment/Attributes preview, but final before/after deltas and combat power remain owned by Character Attributes per ADR-0014.

### Authoring and Resource Policy

Phase 1 prefers GDScript fixture/config factories or primitive semantic payloads. These factories are centralized authoring/config sources, not scattered gameplay constants.

If JSON, `.tres`, editor importers, or OpenMir2 extraction tools are introduced later:

- they are external authoring envelopes only;
- importer/bootstrap code reads explicit whitelisted fields;
- unsupported `Object`, `Resource`, `RefCounted`, `Node`, `Callable`, `Signal`, and arbitrary script values are rejected by default;
- output is reconstructed as primitive semantic payloads and then normalized DTO/table data;
- loaded Resources, duplicated Resources, Resource paths, ResourceLoader cache identity, editor object identity, and serialized property order never become gameplay authority;
- Resource/FileAccess tests are supplemental importer evidence and do not replace scene-tree-independent contract tests.

This aligns Item Definition with the Resource boundary established for Character Attributes in ADR-0013.

### Save/Load and Migration Boundary

Save payloads store item references and instance fields, not copied template truth as authoritative data. At minimum, saved item records include `item_id`, `definition_version`, quantity or stable non-stackable instance ID, container/owner context as defined by Inventory/Equipment/Save, and any explicitly approved instance-level fields.

On load, Save/Load resolves item references through Item Definition query APIs. Missing, deprecated, unsupported, blocked-unconfirmed, or invalid definitions produce structured migration/recovery/block results. Persisted display names, icon keys, type, quality, modifiers, or stack rules are debug snapshots only if stored at all.

Definition migrations must record old `item_id`, old version, new `item_id`, new version, reason, and whether player-facing meaning changes. Silent remap is forbidden when meaning changes.

### System Boundaries

Drop Table validates item references against Item Definition but owns probabilities, weights, pools, guarantees, source monster rules, and quantity rolls.

Drop/Pickup stores item references on ground drop records and owns placement, lifetime, pickup eligibility, pickup range, click priority, and map integration.

Inventory queries stack policy and display metadata but owns capacity, slot placement, sorting, merge/split, drag/drop, and selected item state.

Equipment queries equipment candidate data and modifier payload declarations but owns equip legality, slot occupancy, replacement, active source set, transaction ordering, and preview request orchestration.

Character Attributes consumes Equipment-resolved modifier sources and owns validation, aggregation, snapshots, deltas, and combat power.

UI/Localization consumes projection DTOs and display keys but owns final localized text, layout, colors, icons, VFX/audio, accessibility presentation, and interaction flow.

Economy systems may later consume `item_type`, `quality_id`, and item IDs as inputs, but Item Definition does not define shop value, recycle outputs, drop value, or rarity economics.

## Alternatives Considered

### Alternative 1: Raw Dictionary item rows as runtime truth

- **Description**: Load item definitions as `Dictionary` maps and let consumers read fields directly.
- **Pros**: Fast to prototype; flexible for changing schemas.
- **Cons**: Typo-prone; no read-only boundary; unstable failure ordering; consumers can see or mutate unintended fields; hard to enforce projection and lifecycle rules.
- **Rejection Reason**: The GDD requires shared item truth with structured validation and consumer-safe query results.

### Alternative 2: Godot `.tres` Resources as runtime authority

- **Description**: Author item definitions as Resources and pass loaded Resource objects to gameplay systems.
- **Pros**: Editor-friendly; familiar Godot workflow; asset references can be convenient.
- **Cons**: Shared mutable graphs, ResourceLoader cache identity, nested container aliasing, and editor/runtime coupling can leak authority and break deterministic tests.
- **Rejection Reason**: Resources may be future authoring envelopes, but runtime authority must be normalized immutable-by-contract DTO/table data.

### Alternative 3: Each downstream system stores its own item subset

- **Description**: Drop tables, inventory, equipment, UI, save/load, and economy each copy the fields they need.
- **Pros**: Short-term subsystem autonomy; fewer central DTOs.
- **Cons**: Duplicated truth; inconsistent versions; stale display/equipment data; migration ambiguity; no single validation source.
- **Rejection Reason**: Item Definition exists specifically to keep all downstream systems pointing at one validated item template truth.

### Alternative 4: Equipment owns equipment item definitions

- **Description**: Item Definition owns only generic items; Equipment owns equipment templates, modifiers, slots, and comparison hints.
- **Pros**: Equipment logic and equipment data sit together.
- **Cons**: Splits item identity and display truth; makes inventory/drop/UI save references ambiguous; duplicates lifecycle/evidence/version rules.
- **Rejection Reason**: Item Definition owns template declarations; Equipment owns legality and active equipped state.

### Alternative 5: Persist full copied item template data in saves

- **Description**: Save display names, modifiers, quality, stack limits, and equipment metadata directly in item instances.
- **Pros**: Saves can load even if current definitions change; easier debugging.
- **Cons**: Save becomes stale authority; migration becomes unclear; copied modifiers can diverge from validated definitions.
- **Rejection Reason**: Saves persist references and approved instance fields; current template truth is resolved through Item Definition with explicit migration/fallback.

## Consequences

### Positive

- All systems reference one validated item template source.
- Invalid/debug/deprecated/blocked-unconfirmed rows are prevented from leaking into normal loot.
- UI receives player-safe projections without recomputing or guessing item semantics.
- Equipment and Character Attributes have a clean modifier payload handoff.
- Save/load migration has explicit versioned item references.
- Future JSON/Resource/OpenMir2 import pipelines can be added without changing runtime authority.

### Negative

- More DTO/query surface area than raw dictionaries.
- Validation and index construction add implementation and test burden.
- Authoring provisional items requires enough metadata to be player-explainable, not just quick test rows.
- Some convenient editor Resource workflows are deferred until importer isolation is implemented.

### Neutral

- This ADR does not define final item art assets, localization files, UI layouts, drop table probabilities, inventory size, equipment slots, sell values, combat formulas, or OpenMir2-authentic mappings.
- This ADR allows future compact runtime IDs, JSON, `.tres`, importers, and tooling if they preserve normalized item truth.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| Consumers bypass query DTOs and read raw fixture rows. | Medium | High | Register forbidden pattern; implementation tests/code review require catalog queries and projections. |
| Debug/deprecated/blocked items leak into normal drop tables. | Medium | High | Normal-spawnable index excludes them; drop table validation must fail on such references. |
| Resource authoring later becomes mutable runtime authority. | Medium | High | Follow ADR-0013-style importer boundary; Resource tests are supplemental only. |
| UI treats quality as power or price authority. | Medium | Medium | Projection docs label quality as display/classification metadata only; economy/equipment must define their own rules. |
| Equipment applies item modifier payload directly without transaction boundaries. | Medium | High | Equipment resolves sources; Attributes validates through ADR-0006/0009/0014 boundaries. |
| Save/load silently remaps changed item definitions. | Medium | High | Migration records old/new IDs/versions and reason; player-facing meaning changes cannot silently remap. |
| DTOs leak mutable arrays for slot tags or modifier rows. | Medium | High | Constructor copying, defensive getters, aliasing tests. |
| Missing provisional metadata produces debug-looking MVP loot. | Medium | Medium | Validation requires minimum player-facing metadata for normal spawnable items. |
| OpenMir2 claims are made without evidence. | Medium | High | `openmir2_verified` requires evidence refs; blocked-unconfirmed rows cannot be implementation authority. |
| Lookup indexes drift from rows. | Low | High | Build indexes only from normalized rows; index consistency tests; no manual shadow truth. |

## Performance Implications

| Metric | Before | Expected After | Budget |
|--------|--------|---------------|--------|
| CPU (frame time) | No implementation. | Most validation/indexing occurs at bootstrap/test setup; runtime lookup is direct indexed query by item ID/version. | Overall project frame budget 16.6 ms at 60 fps; no per-frame full catalog validation. |
| Memory | No implementation. | Immutable rows, indexes, and projection DTOs for a small MVP catalog; future catalogs can add compact handles if needed. | Phase 1 client under 1 GB RAM. |
| Load Time | No implementation. | Bootstrap validates item definitions and builds indexes; small MVP set should be negligible. | Acceptable for Phase 1; fail fast on invalid data. |
| Network | Not applicable for Phase 1 offline slice. | Not applicable. | None. |

## Migration Plan

No existing Item Definition implementation needs migration.

1. Define typed enums/constant registries for item type, stack policy, lifecycle, resolution status, validation status, eligibility status, UI availability, source status, and validation failure reasons.
2. Define immutable-by-contract DTOs for authoring payload, normalized row, modifier payload row, visible modifier fact, display view, equipment candidate view, presentation view, query result, and UI projection views.
3. Implement Phase 1 fixture/config factory payloads for the small MVP item set required by the GDD.
4. Implement deterministic validator stages for identity, display, stack, type, equipment, modifier payload, evidence/source labels, lifecycle, and runtime eligibility.
5. Normalize valid rows into `ItemDefinitionRuntimeCatalog` and build lookup/filter indexes.
6. Implement query APIs for exact version lookup, latest-policy lookup, spawnable validation, stack quantity validation, equipment candidate lookup, and UI projections.
7. Add formula/contract tests for validation, lookup statuses, projection narrowing, DTO aliasing, invalid/deprecated/debug handling, and modifier payload boundaries.
8. Require Drop Table, Drop/Pickup, Inventory, Equipment, UI, Save/Load, and Debug stories to consume the catalog/query contracts rather than raw rows.
9. If JSON/Resource/OpenMir2 import is introduced later, add supplemental importer tests while preserving this runtime boundary.

**Rollback plan**: If the DTO surface proves too broad for the first playable slice, keep the catalog/validation boundary and temporarily expose fewer projections. Do not roll back to raw dictionaries, Resource runtime authority, duplicated subsystem truth, or unversioned save/template data.

## Validation Criteria

- [ ] Duplicate `item_id` rows fail deterministically with ordered reasons.
- [ ] Missing required identity/version/source/lifecycle fields fail before runtime catalog publication.
- [ ] `openmir2_verified` item or field without evidence reference fails validation.
- [ ] Player-facing spawnable rows missing required display metadata or presentation tokens fail validation.
- [ ] Debug-only rows cannot appear in normal spawnable index or normal drop table validation.
- [ ] Deprecated rows remain lookup-resolvable for migration/debug but cannot be used for new normal drops.
- [ ] Blocked-unconfirmed rows cannot become implementation authority or normal spawnable loot.
- [ ] Stack validation rejects non-stackable quantity > 1, stackable max <= 1, and quantity <= 0.
- [ ] Equipment rows require equipment candidate data and non-stackable policy.
- [ ] Non-equipment rows with active equipment modifier payload fail in Phase 1.
- [ ] Modifier payload rejects unknown stat IDs, non-targetable stats, `health_current`, `mana_current`, unsupported operations, out-of-range values, and malformed source labels.
- [ ] `main_comparison_hint` and visible modifier facts are present for normal stat equipment or a structured comparison-unavailable reason blocks normal spawnability.
- [ ] Runtime catalog indexes exact `(item_id, definition_version)` lookup and rejects ambiguous latest-version policy.
- [ ] Query results distinguish missing, version mismatch, unsupported version, deprecated resolved, invalid, debug-only blocked, and normal resolved states.
- [ ] UI projection DTOs expose only the fields approved for that projection and not raw authoring rows or mutable modifier arrays.
- [ ] DTO aliasing tests mutate constructor inputs and returned arrays and prove normalized rows/query results are unchanged.
- [ ] Save/load tests or review prove saves persist references and instance fields, not copied template truth as authority.
- [ ] Drop table validation tests or review prove normal drop pools can reference only normal-spawnable definitions.
- [ ] Equipment tests or review prove equip legality and active source resolution are not decided by Item Definition alone.
- [ ] Character Attributes integration tests or review prove final effective stats/combat power are not stored in item definitions.
- [ ] Formula/contract tests do not depend on SceneTree, UI, Autoload, ResourceLoader, FileAccess, signal order, or real `.tres` files.

## GDD Requirements Addressed

| GDD Document | System | Requirement | How This ADR Satisfies It |
|-------------|--------|-------------|--------------------------|
| `design/gdd/item-definition-system.md` | Item Definition | Own stable item IDs, template contracts, display metadata, stack policy, equipment data entry, modifier payload boundary, source/evidence labels, and lifecycle states. | Defines normalized immutable runtime rows, validation stages, indexes, and ownership boundary. |
| `design/gdd/item-definition-system.md` | Item Definition | Downstream systems must consume normalized definition data, not mutable authoring graphs. | Selects runtime DTO/table authority and forbids raw dictionaries/Resources as runtime truth. |
| `design/gdd/item-definition-system.md` | Item Definition | Query/projection results must include resolution, validation, eligibility, UI availability, display/classification/equipment/presentation views, and debug reasons. | Defines status-bearing conceptual DTOs and narrowed UI projection contracts. |
| `design/gdd/item-definition-system.md` | Item Definition | Invalid definitions must not be spawnable; drop tables reject invalid references at validation time. | Defines normal-spawnable indexes and lifecycle/eligibility status gates. |
| `design/gdd/item-definition-system.md` | Item Definition | Definitions may be deprecated but not silently deleted/remapped once referenced. | Defines versioned save/load lookup and explicit migration semantics. |
| `design/gdd/item-definition-system.md` | Item Definition | Phase 1 fixture must support baseline, upgrade, sidegrade/worse, material, and optional rare/showcase validation. | Migration plan requires MVP fixture payloads; validation criteria ensure player-explainable normal spawnable items. |
| `design/gdd/item-definition-system.md` | Item Definition | UI must use display keys and projections, not raw IDs or final layout/color values. | Separates player-safe display/projection DTOs from final UI/localization presentation ownership. |
| `design/gdd/item-definition-system.md` | Item Definition | Equipment modifier payload is input only; final stats/combat power belong to Character Attributes. | Defines modifier payload boundary and cites ADR-0006/0014 ownership. |
| `design/gdd/item-definition-system.md` | Item Definition | Resource graphs must not become gameplay authority. | Applies ADR-0013-style authoring envelope/importer boundary to Item Definition. |

## Related Decisions

- `design/gdd/item-definition-system.md`
- `design/gdd/character-attributes-system.md`
- `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`
- `docs/architecture/adr-0011-attribute-fixture-config-loading-strategy-mvp-provisional-values.md`
- `docs/architecture/adr-0013-attribute-godot-resource-duplication-shared-reference-policy-if-tres-resources-are-used.md`
- `docs/architecture/adr-0014-attribute-combat-power-main-stat-display-proxy-ownership.md`
- `docs/registry/architecture.yaml` — should be updated with Item Definition state ownership, query/projection contracts, validation/loading contracts, and forbidden item-truth duplication patterns after this ADR.
