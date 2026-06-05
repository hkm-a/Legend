# ADR-0018: Inventory and Equipment Instance Modifier Transaction Boundary

## Status

Accepted

## Date

2026-06-05

## Last Verified

2026-06-05

## Decision Makers

hkm + Claude Code Game Studios; engine specialist review: godot-specialist; strategic review: technical-director / TD-ADR.

## Summary

This ADR defines ownership and transaction boundaries for Phase 1 inventory item instances, stack/container state, equipment slot occupancy, equipment legality, and equipment-derived Attribute modifier source resolution. We choose scene-tree-independent `InventoryService` and `EquipmentService` candidate/commit models where Inventory owns storage, Equipment owns equip state and source-key resolution, Character Attributes owns final aggregation/snapshots/display proxy, and cross-domain equip transactions publish one coherent result without observable partial state.

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Core / Economy / Progression / Scripting / UI Handoff |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff; this ADR avoids post-cutoff-only APIs and uses project-owned service/DTO architecture. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md`; `docs/architecture/adr-0009-attribute-atomic-source-update-and-transaction-model.md`; `docs/architecture/adr-0014-attribute-combat-power-main-stat-display-proxy-ownership.md`; `docs/architecture/adr-0015-item-definition-runtime-data-validation-query-and-versioning.md`; `docs/architecture/adr-0017-drop-table-ground-drop-and-pickup-lifecycle-boundary.md`; `design/gdd/item-definition-system.md`; `design/gdd/character-attributes-system.md` |
| **Post-Cutoff APIs Used** | None required. Core inventory/equipment authority does not use UI Control state, SceneTree, signals, Autoloads, Resources, FileAccess, or Godot networking. |
| **Verification Required** | Verify `RefCounted` DTO defensive copies, no public mutable DTO state, no Node/Resource/Object identity as item identity, no scene/signal/UI drag-drop authority, no observable cross-domain partial equip commit, and no deprecated string-based signal connections in optional adapters. |

> **Note**: If Godot 4.6.3 creates a concrete compatibility blocker during implementation, project policy allows evaluating a Godot version downgrade. Until that happens, this ADR targets the pinned Godot 4.6.3 baseline.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | ADR-0006 Attribute Data Representation and Stat ID Typing; ADR-0007 Attribute Snapshot Query API Shape and Read-Only Enforcement; ADR-0008 Attribute Event Signal Contract; ADR-0009 Attribute Atomic Source Update and Transaction Model; ADR-0014 Attribute Combat Power and Main Stat Display Proxy Ownership; ADR-0015 Item Definition Runtime Data, Validation, Query, and Versioning; ADR-0017 Drop Table, Ground Drop, and Pickup Lifecycle Boundary. |
| **Enables** | Inventory receive/stack/container implementation; pickup inventory handoff from ADR-0017; equipment equip/unequip/replace implementation; equipment preview; equipment-derived Attribute source transactions; inventory/equipment save-load boundary design; inventory/equipment UI stories. |
| **Blocks** | Any story that stores inventory items, stages pickup receive, merges/splits stacks, equips/unequips/replaces equipment, resolves equipment modifiers, sends equipment-derived Attribute transactions, previews equipment deltas, or persists inventory/equipment truth. |
| **Ordering Note** | Inventory/Equipment implementation remains blocked until dependent ADRs are Accepted. This ADR defines inventory/equipment boundaries but does not choose final UI layout, drag/drop UX, inventory capacity tuning, equipment slot count beyond MVP fixture needs, economy value, or final save-file schema. |

## Context

### Problem Statement

The Phase 1 loop needs pickup rewards to enter inventory, equipment to move from inventory into equipment slots, and equipped items to affect Character Attributes atomically. `/architecture-review` identified Inventory/Equipment as the third priority architecture gap because Item Definition and Attributes are already designed, but no ADR defines item instance identity, stack/container ownership, equipment legality, modifier source resolution, or no-partial-commit handoff to Character Attributes.

Without this decision, implementation could use UI drag/drop state as authority, store item template fields in inventory rows, use Node/Object IDs as item instance IDs, update equipment slots while Attributes reject modifiers, or let UI observe new attribute stats while inventory/equipment still show the old item.

### Current State

- Item Definition owns template truth, query/projection DTOs, and modifier payload declarations (ADR-0015).
- Character Attributes owns Attribute source transactions, snapshots, and display proxy data (ADR-0009, ADR-0014).
- Drop/Pickup expects a future Inventory staged receive contract (ADR-0017).
- No Inventory or Equipment implementation exists.

### Constraints

- Inventory/Equipment core logic must be scene-tree-independent and unit-testable.
- UI, drag/drop Control nodes, signals, scene nodes, Resources, and Autoloads must not be gameplay authority.
- Inventory owns storage/container truth; Equipment owns equipped-slot truth; Character Attributes owns final stat truth.
- GDScript `RefCounted`, `Array`, and `Dictionary` are mutable/reference-based; DTOs and staged candidates must avoid aliasing.
- Item instance identity must be project-owned and persistence-safe; Godot object identity is invalid as durable truth.
- Equipment transactions must avoid externally observable mixed states across Inventory, Equipment, and Attributes.

### Requirements

- Define stable item instance and inventory stack/entry identity.
- Define Inventory staged receive/abort/commit contract for Pickup.
- Define Equipment equip/unequip/replace candidate validation and final commit boundary.
- Define equipment-derived Attribute source key canonicalization and Attribute transaction handoff.
- Define default equipment preview path.
- Define save/load ownership boundaries for inventory, equipment, and equipment-derived Attribute sources.
- Forbid UI/Resource/Node/signal authority and copied template truth.

## Decision

We define two service authorities:

1. **InventoryService** owns inventory container state, inventory entries/stacks, item instance identity, stack quantities, receive/move/split/merge/remove transactions, container versions, and inventory persistence input.
2. **EquipmentService** owns equipment slot occupancy, equip legality, active equipped item references, replacement transactions, equipment source-key canonicalization, and equipment-derived Attribute source delta construction.

Other authorities remain separate:

- `ItemDefinitionRuntimeCatalog` owns item template truth and equipment payload declarations.
- `CharacterAttributes` owns validation, aggregation, snapshots, deltas, combat power/display proxy, and committed Attribute source state.
- `PickupService` owns pickup orchestration but uses Inventory staged receive.
- Save/Load orchestrates file/container persistence but does not override Inventory/Equipment truth.

### Item Instance, Inventory Entry, and Stack Identity

Phase 1 uses explicit identity separation:

- `item_id`: stable semantic template key owned by Item Definition.
- `definition_version`: item definition version label owned by Item Definition.
- `item_instance_id`: project-owned stable ID for non-stackable item instances, especially equipment.
- `inventory_entry_id`: project-owned stable ID for a committed inventory entry/slot/stack record.
- `stack_quantity`: quantity held by an inventory entry for stackable items.
- `container_id`: stable semantic inventory/equipment/storage container key.
- `slot_id`: Inventory-owned or Equipment-owned stable semantic slot key depending on domain.

Rules:

- Equipment may reference only valid non-stackable/equippable `item_instance_id` records in Phase 1.
- Stackable materials use `inventory_entry_id + item_id + definition_version + stack_quantity`; they do not imply one `item_instance_id` per unit unless a future ADR explicitly adds per-unit identity.
- When pickup merges a stackable grant into an existing stack, the existing `inventory_entry_id` remains authoritative and the staged grant identity is consumed into that entry.
- When pickup creates a new non-stackable equipment item, Inventory assigns or accepts a staged `item_instance_id` before final commit.
- `item_instance_id` and `inventory_entry_id` must not be derived from `Object.get_instance_id()`, NodePath, Resource path, scene node name, display name, localized text, icon path, RID, or Resource object identity.
- Numeric IDs must have documented safe range. If IDs may exceed JSON safe integer range or external tool precision, persistence stores them as strings.

### Inventory Ownership and Staged Receive Contract

InventoryService owns:

- committed inventory containers;
- inventory entries/stacks;
- item instance IDs and allowed instance fields;
- stack quantities;
- container membership and slot assignment;
- container/entry version counters;
- staged receive state;
- inventory persistence input.

Inventory stores item references and allowed instance fields only. It must query Item Definition for template facts such as display metadata, quality, stack policy, max stack, item type, and equipment payload availability. It must not copy item template truth as authoritative storage.

`InventoryStageReceiveRequest` is used by Pickup and future reward/debug sources. Staging validates and reserves the exact intended result without changing committed inventory state.

Minimum statuses:

- `COMMIT_READY`
- `REJECTED_CAPACITY`
- `REJECTED_INVALID_ITEM_REF`
- `REJECTED_INELIGIBLE_ITEM_REF`
- `REJECTED_STACK_POLICY`
- `REJECTED_QUANTITY`
- `STALE_CONTAINER_VERSION`
- `DUPLICATE_ITEM_INSTANCE_ID`
- `INVARIANT_FAILURE`

Staged receive rules:

1. Stage validates item reference through `ItemDefinitionRuntimeCatalog`.
2. Stage validates stack policy, quantity, non-stackable instance requirements, and container constraints.
3. Stage reserves the exact merge/new-entry/container outcome.
4. Stage returns a status-bearing result and a `stage_id` only on `COMMIT_READY`.
5. Final commit after `COMMIT_READY` is a synchronous no-yield final swap, not a second normal validation pass that may reject.
6. Abort releases only staged state and never removes committed inventory.
7. If final commit after `COMMIT_READY` fails, it is `INVARIANT_FAILURE`, not normal rejection.

### Equipment Ownership

EquipmentService owns:

- equipment slot occupancy;
- equip/unequip/replace legality;
- active equipped item references;
- equipment slot versions;
- equipped source-key canonicalization;
- modifier source resolution from equipped item references;
- equipment preview request construction;
- equipment persistence input.

Equipment queries Item Definition for equipment candidate data and modifier payload declarations. Equipment decides whether an item instance can be equipped in a slot and whether replacement is legal. It does not calculate final effective stats, combat power, or display proxy values.

Equipment must not invent modifiers absent from Item Definition unless a future affix/enchant system explicitly supplies instance modifiers. In Phase 1, accepted equipment-derived Attribute modifier operations are limited by ADR-0015/Character Attributes policy.

### Equipment Source Key Canonicalization

EquipmentService is the only producer of equipment-derived Attribute source keys.

Canonical key fields:

```text
namespace = "equipment"
actor_scope = actor_id or actor-local scope marker
slot_id = Equipment-owned semantic slot key
item_instance_id = stable project-owned item instance ID
item_id = Item Definition semantic item key
definition_version = Item Definition version
source_kind = "equipped_item_modifier"
```

Rules:

- Field order is fixed as listed above for debug/string serialization.
- Runtime may use a typed DTO rather than concatenated string, but semantic fields are the same.
- `slot_id` is an Equipment semantic slot key, not a UI slot index, label, NodePath, or scene name.
- Duplicate detection occurs after canonicalization.
- Replacement removal and addition use the same canonical key space.
- UI and Item Definition do not generate equipment source keys.
- Changing item instance, slot, or definition version changes the source key unless a future migration ADR defines stable remap semantics.

### Cross-Domain Equip Transaction

Equipment equip/unequip/replace is a synchronous cross-domain transaction coordinator over Inventory, Equipment, and Character Attributes.

Canonical order:

1. Validate request, actor, inventory entry, item instance, and expected inventory/equipment versions.
2. Query Item Definition for equipment candidate data and modifier payload declarations.
3. Validate equip legality and target slot.
4. Build candidate Inventory state, if the operation moves an item between inventory/equipment domains.
5. Build candidate Equipment slot state.
6. Canonicalize equipment source keys for removed and added equipped sources.
7. Build `AttributeSourceDelta` remove old/add new in one delta.
8. Pre-materialize inventory/equipment final swaps and verify they are failure-free if Attribute commits.
9. Submit `AttributeTransactionRequest` with required `expected_source_version` to Character Attributes.
10. If Attribute rejects, discard candidates and keep Inventory/Equipment unchanged.
11. If Attribute commits, final swap Inventory and Equipment candidates immediately in the same synchronous call stack before any UI/presentation/signal adapter observes the result.
12. Publish one `EquipmentTransactionResult` referencing Attribute result/status.

Cross-domain observability rules:

- No consumer may observe Attributes committed to new equipment while Equipment/Inventory still expose old state, or Equipment slots changed while Attributes still expose old committed sources without stale/failure status.
- Reads during transaction return last committed Inventory/Equipment/Attribute state, never candidates.
- Attribute event sinks/signals/UI adapters must not dispatch until the Equipment transaction coordinator has finalized Inventory and Equipment swaps or produced invariant failure.
- No `await`, timers, `call_deferred()`, SceneTree mutation, or signal emission occurs during the transaction.
- If final Inventory/Equipment swap fails after Attribute commit, this is `INVARIANT_FAILURE`; implementation must halt, quarantine actor state, or enter a repair path. It must not claim normal rollback.

### Character Attributes Handoff

Equipment never writes Character Attributes committed source sets directly. It submits only `AttributeTransactionRequest` / `AttributeSourceDelta` to Character Attributes.

Character Attributes validates target stat IDs, operation support, source labels, duplicate source keys, numeric bounds, and transaction versions. Equipment cannot infer Attribute success from local equip legality.

### Equipment Preview Default

Phase 1 default preview path is the Character Attributes pure preview query:

1. Equipment validates inventory item and equipment candidate data.
2. Equipment resolves hypothetical modifier source payloads and canonical preview source keys marked `preview_only`.
3. Equipment submits a preview request to Character Attributes with current source/snapshot version provenance.
4. Character Attributes returns `AttributePreviewResult` / display proxy data without committing sources, changing Inventory/Equipment state, incrementing authoritative versions, invoking sinks/signals, or entering committed event arrays.
5. UI consumes preview as comparison only, not committed growth.

Equipment preview must not reserve inventory slots, mutate equipped refs, create committed Attribute sources, or trigger growth celebration.

### Persistence Boundary

Authoritative persistence ownership:

- Inventory persists item instance IDs, inventory entry IDs, item references, definition versions, stack quantities, allowed instance fields, container membership, and inventory versions.
- Equipment persists equipped slot occupancy, active equipped item refs, slot IDs, item instance refs, and equipment versions.
- Character Attributes persists non-equipment/base/current/source inputs as defined by ADR-0010, but equipment-derived Attribute sources are rebuilt from Equipment + Item Definition on load.
- Save/Load envelope may store both Inventory and Equipment domains, but load validation must cross-check them.

Forbidden persistence authority:

- Inventory/Equipment must not persist copied item display names, quality, stack policy, equipment payload, modifier templates, final effective stats, combat power, or UI layout as authoritative truth.
- Character Attributes must not override Equipment slot truth with separately persisted equipment-derived modifier sources.
- If saved Equipment slot truth and saved/debug Attribute equipment-derived source evidence disagree, load fails closed or enters explicit migration/recovery; it must not silently choose one.

### Architecture

```text
PickupService / DebugGrant / Load
        │
        ▼
InventoryService staged receive / move / split / merge
        │ committed inventory entries + item_instance_id / inventory_entry_id
        ▼
EquipmentService equip/replace request
        │
        ├── query ItemDefinitionRuntimeCatalog for equipment payload
        ├── build Inventory candidate
        ├── build Equipment slot candidate
        ├── canonicalize equipment source keys
        └── build AttributeSourceDelta
                 │
                 ▼
        CharacterAttributes AttributeTransactionRequest
                 │
                 ▼
        Attribute commit/reject
                 │
                 ▼
        Equipment coordinator finalizes Inventory + Equipment swaps
                 │
                 ▼
        EquipmentTransactionResult / UI preview or committed result
```

### Key Interfaces

```gdscript
class_name InventoryStageReceiveRequest
extends RefCounted

func get_item_id() -> StringName: pass
func get_definition_version() -> StringName: pass
func get_quantity() -> int: pass
func get_source_context_id() -> StringName: pass
func get_expected_container_version() -> int: pass
```

```gdscript
class_name InventoryStageReceiveResult
extends RefCounted

enum Status {
    COMMIT_READY,
    REJECTED_CAPACITY,
    REJECTED_INVALID_ITEM_REF,
    REJECTED_INELIGIBLE_ITEM_REF,
    REJECTED_STACK_POLICY,
    REJECTED_QUANTITY,
    STALE_CONTAINER_VERSION,
    DUPLICATE_ITEM_INSTANCE_ID,
    INVARIANT_FAILURE,
}

func get_status() -> int: pass
func is_commit_ready() -> bool: pass
func get_stage_id() -> int: pass
func get_planned_inventory_entry_id() -> StringName: pass
func get_planned_item_instance_id() -> StringName: pass
func get_failure_reasons_copy() -> Array[StringName]: pass
```

```gdscript
class_name EquipmentTransactionRequest
extends RefCounted

func get_actor_id() -> int: pass
func get_request_id() -> StringName: pass
func get_inventory_entry_id() -> StringName: pass
func get_item_instance_id() -> StringName: pass
func get_target_slot_id() -> StringName: pass
func get_expected_inventory_version() -> int: pass
func get_expected_equipment_version() -> int: pass
func get_expected_attribute_source_version() -> int: pass
```

```gdscript
class_name EquipmentSourceKey
extends RefCounted

func get_namespace() -> StringName: pass
func get_actor_scope() -> StringName: pass
func get_slot_id() -> StringName: pass
func get_item_instance_id() -> StringName: pass
func get_item_id() -> StringName: pass
func get_definition_version() -> StringName: pass
func get_source_kind() -> StringName: pass
```

```gdscript
class_name EquipmentTransactionResult
extends RefCounted

enum Status {
    EQUIPPED,
    UNEQUIPPED,
    REPLACED,
    REJECTED_INVALID_ITEM_REF,
    REJECTED_NOT_IN_INVENTORY,
    REJECTED_NOT_EQUIPMENT,
    REJECTED_SLOT_INCOMPATIBLE,
    REJECTED_ATTRIBUTE_TRANSACTION,
    STALE_INVENTORY_VERSION,
    STALE_EQUIPMENT_VERSION,
    STALE_ATTRIBUTE_VERSION,
    INVARIANT_FAILURE,
}

func get_status() -> int: pass
func is_committed() -> bool: pass
func get_attribute_update_result() -> AttributeUpdateResult: pass
func get_failure_reasons_copy() -> Array[StringName]: pass
```

### Immutable DTO Rule

Inventory, Equipment, source-key, request, candidate, and result DTOs must:

- copy inbound arrays/dictionaries during construction;
- expose no public mutable vars, setters, or mutators;
- return defensive copies for collections;
- avoid storing caller-owned mutable references;
- avoid Nodes, Resources, Callables, Signals, scene objects, physics objects, or UI controls;
- use nested DTOs that follow the same immutable-by-contract rules;
- include tests that mutate constructor inputs and getter-returned collections.

### UI / Signal / Scene Boundary

- UI drag/drop, hover, focus, Control state, icon movement, scene node visibility, audio cue, VFX, or signal callback is never inventory/equipment authority.
- UI submits typed requests and consumes status-bearing preview/final results.
- UI must not compute stack merge, equip legality, equipment source keys, Attribute deltas, combat power, or final effective stats.
- Optional signals are downstream adapters after final result materialization and must use typed signal/Callable style, not deprecated string-based `connect()`.
- Autoload/global singleton Inventory/Equipment authority is forbidden in Phase 1 unless a future ADR approves it.

## Alternatives Considered

### Alternative 1: UI-driven inventory/equipment state

- **Description**: Inventory grid and equipment slots are Control nodes; drag/drop state decides item movement and equip.
- **Pros**: Fast visual prototyping.
- **Cons**: UI lifecycle, focus, scene order, and signals become gameplay authority; impossible to unit-test core rules cleanly.
- **Rejection Reason**: UI must not own gameplay data or transaction truth.

### Alternative 2: Equipment directly mutates Character Attributes source set

- **Description**: Equipment writes modifier sources into Attribute runtime state when slots change.
- **Pros**: Fewer request/result types.
- **Cons**: Violates ADR-0009 transaction boundary, bypasses version checks and validation, and creates rollback hazards.
- **Rejection Reason**: Character Attributes owns source commit/reject semantics.

### Alternative 3: Inventory stores copied item template fields

- **Description**: Inventory entries copy display, stack, quality, equipment, and modifier fields from Item Definition.
- **Pros**: Faster local UI and equip checks.
- **Cons**: Duplicates item template truth and creates drift from ItemDefinitionRuntimeCatalog.
- **Rejection Reason**: ADR-0015 requires Item Definition as template truth.

### Alternative 4: Persist equipment-derived Attribute sources as independent authority

- **Description**: Save both Equipment slot state and final equipment-derived Attribute source rows as authoritative save truth.
- **Pros**: Easier Attribute load rebuild in isolation.
- **Cons**: Creates dual authority if slots and sources disagree.
- **Rejection Reason**: Equipment slot truth should rebuild equipment-derived Attribute sources; saved Attribute source rows may be debug evidence only.

## Consequences

### Positive

- Prevents Inventory/Equipment/Attributes partial-observation bugs.
- Defines Pickup-to-Inventory staging contract needed by ADR-0017.
- Keeps Item Definition, Inventory, Equipment, and Attributes ownership separate.
- Provides deterministic equipment source keys and version checks.
- Enables formula-correct equipment preview without committed side effects.

### Negative

- Requires staged candidate infrastructure for Inventory and Equipment.
- Requires careful coordination so Attribute sinks/signals wait for cross-domain finalization.
- More identity concepts than a simple item list.

### Neutral

- Does not define final inventory capacity tuning, UI layout, drag/drop UX, equipment slot count, item valuation, or save file syntax.
- Does not implement affixes/enchantments; future instance modifiers require a new ADR or extension.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| Attribute commits new modifiers while UI still sees old equipment | Medium | High | Cross-domain transaction coordinator finalizes Inventory/Equipment before result dispatch. |
| Stack identity conflates item instance identity | Medium | Medium | Separate `inventory_entry_id` from `item_instance_id`; stackables use entry+quantity. |
| Save/load duplicates equipment truth in Attributes | Medium | High | Equipment-derived Attribute sources rebuild from Equipment; saved Attribute rows debug-only unless future ADR says otherwise. |
| DTO aliasing mutates staged candidates | Medium | Medium | Defensive copies, no public mutable vars, aliasing tests. |
| UI drag/drop becomes authority | Medium | High | UI request/result-only boundary and forbidden pattern. |
| Final swap fails after Attribute commit | Low | High | Treat as invariant failure with halt/quarantine/repair, not normal rollback. |

## Performance Implications

| Metric | Before | Expected After | Budget |
|--------|--------|---------------|--------|
| CPU (frame time) | No implementation | Inventory/equipment transactions event-driven; no per-frame scan | Must fit 60 fps / 16.6 ms; not per-frame logic |
| Memory | No implementation | Inventory entries, staged candidates, and result DTOs proportional to item count and active transactions | Phase 1 client under 1 GB RAM |
| Load Time | No load validation | Future load rebuild validates Inventory/Equipment refs and rebuilds equipment-derived Attribute sources | Acceptable at load/bootstrap; not hot path |
| Network | None | None | 0 KB/s in Phase 1 |

## Migration Plan

No existing implementation requires migration.

1. Implement minimal `InventoryService` with committed entries, versions, and staged receive/abort/commit.
2. Implement item instance and inventory entry ID generation/validation with persistence-safe encoding.
3. Implement `EquipmentService` slot state, equip legality, and source-key canonicalization.
4. Implement Equipment → Character Attributes transaction handoff using `AttributeTransactionRequest`.
5. Implement pure preview path through Character Attributes preview.
6. Add save/load design that persists Inventory/Equipment truth and rebuilds equipment-derived Attribute sources.
7. Add UI only after request/result DTOs exist.

**Rollback plan**: If cross-domain equip coordination is too heavy, defer equip implementation or use a test-only fake Attribute service. Do not switch to UI-owned equipment state or direct Attribute source mutation without superseding this ADR.

## Validation Criteria

- [ ] Inventory staged receive rejects invalid item refs, invalid quantity, stack policy violations, and stale container versions before committed state changes.
- [ ] Inventory final commit after `COMMIT_READY` is a no-yield final swap; failure is invariant failure.
- [ ] Pickup abort releases staged inventory state without changing committed inventory.
- [ ] Stackable item merge keeps existing `inventory_entry_id` and updates quantity deterministically.
- [ ] Equipment equip validates item instance exists in Inventory and resolves Item Definition equipment data.
- [ ] Equipment source keys are canonical and duplicate detection occurs after canonicalization.
- [ ] Attribute rejection leaves Inventory and Equipment committed state unchanged.
- [ ] Attribute commit plus Inventory/Equipment final swaps publish one coherent result; no UI/signal observes mixed old/new state.
- [ ] Simulated final Inventory/Equipment swap failure after Attribute commit produces `INVARIANT_FAILURE` and does not claim normal rollback.
- [ ] Preview does not mutate Inventory, Equipment, or Character Attributes committed state; no source/snapshot version advances.
- [ ] Save/load rebuild from Inventory/Equipment produces the same equipment-derived Attribute sources.
- [ ] DTO aliasing tests prove constructor inputs and getter-returned collections cannot mutate authority.
- [ ] No item identity uses `Object.get_instance_id()`, NodePath, Resource path, scene name, display name, localized text, or icon path.

## GDD Requirements Addressed

| GDD Document | System | Requirement | How This ADR Satisfies It |
|-------------|--------|-------------|--------------------------|
| `design/gdd/systems-index.md` | Systems Index | MVP loop requires pickup → inventory → equipment → attributes growth chain. | Defines Inventory receive, Equipment equip, and Attribute handoff boundaries. |
| `design/gdd/item-definition-system.md` | Item Definition | Inventory receives item instance/stack requests and owns capacity, slot placement, merge/split, drag/drop, and item selection. | Assigns InventoryService ownership of container/entry/stack/item instance state while querying Item Definition for template facts. |
| `design/gdd/item-definition-system.md` | Item Definition | Equipment owns equip legality, slot selection, replacement, transaction order, and active equipped source set. | Assigns EquipmentService ownership of slot occupancy, legality, replacement, and source-key canonicalization. |
| `design/gdd/item-definition-system.md` | Item Definition | Equipment resolves item definition modifier rows plus instance identity into active modifier sources; Attributes validates and aggregates. | Defines Equipment → AttributeSourceDelta / AttributeTransactionRequest handoff. |
| `design/gdd/item-definition-system.md` | Item Definition | Inventory/Equipment/Save must not copy item template truth as authority. | Forbids copied template fields and requires ItemDefinitionRuntimeCatalog queries. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Equipment replacement must be atomic from the Attribute consumer perspective. | Defines one cross-domain synchronous equip transaction and prevents observable mixed equipment/attribute state. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Preview must not mutate authoritative source state or increment snapshot version. | Sets pure AttributePreviewResult as Phase 1 default preview path. |
| `docs/architecture/adr-0017-drop-table-ground-drop-and-pickup-lifecycle-boundary.md` | Drop/Pickup | Pickup requires staged Inventory receive/abort/commit to avoid half commit. | Defines the Inventory staged receive contract referenced by ADR-0017. |

## Related

- `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`
- `docs/architecture/adr-0007-attribute-snapshot-query-api-shape-and-read-only-enforcement.md`
- `docs/architecture/adr-0008-attribute-event-signal-contract-and-scene-tree-independent-core.md`
- `docs/architecture/adr-0009-attribute-atomic-source-update-and-transaction-model.md`
- `docs/architecture/adr-0014-attribute-combat-power-main-stat-display-proxy-ownership.md`
- `docs/architecture/adr-0015-item-definition-runtime-data-validation-query-and-versioning.md`
- `docs/architecture/adr-0017-drop-table-ground-drop-and-pickup-lifecycle-boundary.md`
- `design/gdd/item-definition-system.md`
- `design/gdd/character-attributes-system.md`
