# ADR-0017: Drop Table, Ground Drop, and Pickup Lifecycle Boundary

## Status

Accepted

## Date

2026-06-05

## Last Verified

2026-06-05

## Decision Makers

hkm + Claude Code Game Studios; engine specialist review: godot-specialist; strategic review: technical-director / TD-ADR.

## Summary

This ADR defines the Phase 1 reward pipeline boundary for drop tables, ground drops, and pickup lifecycle. We choose a scene-tree-independent, deterministic, staged pipeline where Drop Table owns validated roll policy, Ground Drop owns ground-drop lifecycle, Pickup owns pickup commit orchestration, `MapSpaceState` remains the only item occupancy mutation authority, Item Definition remains template truth, and Inventory remains final storage authority.

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Core / Economy / 2D Spatial Tooling / Scripting |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff; this ADR avoids post-cutoff-only gameplay APIs and uses project-owned deterministic service/DTO architecture. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md`; `docs/architecture/adr-0001-map-data-representation.md`; `docs/architecture/adr-0003-authoritative-occupancy-reservation-update-ordering.md`; `docs/architecture/adr-0015-item-definition-runtime-data-validation-query-and-versioning.md`; `docs/architecture/adr-0016-openmir2-evidence-mapping-registry-and-provisional-contract-governance.md`; `design/gdd/systems-index.md` |
| **Post-Cutoff APIs Used** | None required. This ADR does not require Godot physics, `Area2D`, `TileMapLayer`, networking, signals, ResourceLoader, FileAccess, or SceneTree APIs for core gameplay authority. |
| **Verification Required** | Verify service/DTO determinism, no SceneTree/Autoload/signal/physics authority, no Resource graph runtime authority, defensive-copy DTO behavior, deterministic RNG replay, and no-half-commit pickup transaction behavior. |

> **Note**: If Godot 4.6.3 creates a concrete compatibility blocker during implementation, project policy now allows evaluating a Godot version downgrade. Until that happens, this ADR targets the pinned Godot 4.6.3 baseline.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | ADR-0001 Map Data Representation; ADR-0002 Typed Query Result Schema; ADR-0003 Authoritative Occupancy / Reservation Update Ordering; ADR-0015 Item Definition Runtime Data, Validation, Query, and Versioning; ADR-0016 OpenMir2 Evidence Mapping Registry and Provisional Contract Governance. |
| **Enables** | Drop Table system GDD/ADR implementation; Ground Drop and Pickup system design; drop roll tests; pickup commit tests; loot feedback presentation hooks; future Inventory receive contract integration. |
| **Blocks** | Any story that rolls drops, materializes ground loot, places/removes ground item occupancy, processes pickup requests, grants pickup rewards, or claims OpenMir2-authentic drop/pickup behavior. |
| **Ordering Note** | ADR-0016 is Accepted and defines evidence readiness for source-authentic OpenMir2 claims. ADR-0018 is Accepted and owns Inventory/Equipment staged receive and storage/equipment semantics. Until relevant OpenMir2 contracts pass ADR-0016 readiness, all concrete drop-rate, quantity, pickup timing/range, ownership, and despawn behavior must remain `mvp_provisional` or `openmir2_evidence_pending`, not `openmir2_verified`. |

## Context

### Problem Statement

The 30-second loot loop needs monsters to die, roll rewards, place readable ground loot, let the player pick it up, and hand the reward to inventory without duplicating item truth or corrupting map occupancy. `/architecture-review` identified this as the second highest-priority architecture gap: existing Map ADRs define item occupancy, and Item Definition defines template truth, but no ADR connects death/drop/ground item/pickup/inventory receive into one deterministic lifecycle boundary.

Without this ADR, drop logic could copy item definitions into drop rows, create inventory instances too early, mutate map item occupancy directly, use scene nodes or `Area2D` as pickup authority, grant inventory before map cleanup succeeds, or silently delete rewards when placement fails.

### Current State

- `MapSpaceState` owns actor/item occupancy and reservations through ADR-0003 command submission.
- `ItemDefinitionRuntimeCatalog` owns item template truth through ADR-0015.
- ADR-0016 now defines evidence readiness for source-authentic OpenMir2 drop/pickup claims.
- No Drop Table, Ground Drop, Pickup, Inventory, or Equipment implementation exists yet.

### Constraints

- Phase 1 is offline and single-client; no networking authority is introduced.
- Drop/pickup core logic must be deterministic and unit/integration-testable.
- Presentation, audio, UI, and scene nodes may show loot state but never decide gameplay success.
- Ground drops must not become a second inventory or item-template truth store.
- Inventory capacity, stack merge, slot layout, final item instance identity, equipment legality, and save truth are downstream decisions, not owned here.
- OpenMir2-authentic claims require ADR-0016 readiness; provisional values must be labeled provisional.

### Requirements

- Validate drop table item references against Item Definition before normal gameplay use.
- Roll drop candidates deterministically from injected RNG/seed context.
- Materialize ground drop records with stable IDs and lifecycle state.
- Place/remove ground item occupancy only through `MapSpaceState` commands.
- Validate and resolve pickup claims deterministically.
- Stage inventory receive and map occupancy removal before final pickup commit.
- Prevent half-commit states where inventory gets an item but ground loot remains available, or ground loot disappears while inventory rejects the reward.
- Surface placement, pickup, inventory, map cleanup, evidence, and invariant failures as structured results.

## Decision

We define a three-owner reward pipeline:

1. **Drop Table Runtime Catalog** owns validated drop pools, weights, quantity policies, and deterministic roll policy.
2. **Ground Drop Service** owns ground drop records, lifecycle, placement intent, claim state, despawn policy, and provenance metadata.
3. **Pickup Service** owns pickup request validation, deterministic claim resolution, staged inventory receive orchestration, staged map removal orchestration, and final pickup result.

The pipeline explicitly depends on other authorities:

- `ItemDefinitionRuntimeCatalog` owns item template truth and item reference validation.
- `MapSpaceState` owns spatial item occupancy mutation and spatial query truth.
- Inventory owns final inventory storage, item instance/stack identity, capacity, merge, slot, and persistence semantics.
- ADR-0016 owns whether drop/pickup behavior may claim OpenMir2 authenticity.

### Architecture

```text
Monster death / reward source context
        │
        ▼
DropTableRuntimeCatalog
(validate table → roll deterministic item grant candidates)
        │ item_id/version/quantity intent only
        ▼
GroundDropService
(create GroundDropRecord → request placement)
        │ PLACE_ITEM command referencing ground_drop_id
        ▼
MapSpaceState
(authoritative item occupancy mutation)
        │ post-commit spatial result
        ▼
Ground drop available on logical cell
        │
        ▼
PickupService
(validate request → spatial verify → deterministic claim
 → staged inventory receive → staged REMOVE_ITEM
 → final commit result)
        │
        ├── InventoryService staged receive / abort / commit (future ADR)
        ├── MapSpaceState REMOVE_ITEM command
        └── post-commit presentation/audio/QA result
```

### Ownership Boundary

#### Drop Table Runtime Catalog owns

- Drop table IDs, pool rows, roll groups, weights, quantity policies, eligibility labels, source/evidence labels, and validation errors.
- Deterministic roll request/result DTOs.
- Validation that all normal droppable item references resolve through `ItemDefinitionRuntimeCatalog` to active, spawn-eligible definitions.

It does not own item display names, quality truth, stack policy truth, equipment metadata, final inventory instances, ground placement, pickup state, or map occupancy.

#### Ground Drop Service owns

- `ground_drop_id` stable runtime ID.
- Ground drop lifecycle: `PENDING_PLACEMENT`, `AVAILABLE`, `CLAIMED`, `PICKED_UP`, `DESPAWNED`, `FAILED_PLACEMENT`, `INVALIDATED`.
- Ground drop item grant intent: `item_id`, optional `definition_version`, quantity, source context, roll provenance, and allowed future instance/grant input fields.
- Placement result status, claim holder/request, claim expiry policy, despawn eligibility, and debug provenance.

It does not own item template truth, inventory slots/stacks, equipped references, save truth, display metadata truth, or spatial occupancy mutation.

#### MapSpaceState owns

- Spatial item occupancy for logical cells.
- `PLACE_ITEM` / `REMOVE_ITEM` command ordering, conflict resolution, and occupancy mutation.
- The latest committed spatial query truth for pickup/drop placement.

GroundDropService and PickupService must never directly mutate MapSpaceState internal item occupancy tables.

#### Pickup Service owns

- Pickup request validation against ground drop lifecycle and map-space spatial query results.
- Deterministic claim ordering.
- Staged inventory receive orchestration.
- Staged map removal orchestration.
- Final pickup commit result and structured failure reasons.

It does not own final inventory storage, stack merge, slot placement, equipment legality, save persistence, or UI/audio feedback.

### Ground Drop Payload Boundary

A `GroundDropRecord` may store:

- `ground_drop_id`;
- `item_id` and optional `definition_version`;
- quantity / quantity unit;
- drop source context such as monster/source actor ID, drop table ID, roll sequence, and evidence/provisional labels;
- logical map placement metadata and placement result;
- lifecycle and claim state;
- deterministic roll provenance such as seed stream ID and roll sequence;
- future instance/grant input fields only if accepted by a later Inventory ADR.

It must not store copied item definition display names, icons, quality truth, stack policy truth, equipment modifier template truth, combat power, price, final inventory slot, final inventory stack ID, equipped reference, or save truth.

### Deterministic Drop Roll Contract

Drop rolls use an injected deterministic RNG stream or deterministic seed context. Drop systems must not call `randomize()`, use global RNG state, wall-clock time, frame delta, node lifecycle order, scene-tree order, dictionary iteration order, or signal order as roll authority.

A roll result must carry enough debug/test provenance to replay the result in tests:

- drop table ID/version;
- source context ID;
- seed stream ID or deterministic seed label;
- roll sequence;
- selected pool/row IDs;
- item references and quantity result;
- structured no-drop or invalid-table reasons.

### Placement Failure Policy

If a drop roll produces a reward but placement fails, the reward must not be silently deleted, rerolled, or moved by presentation code.

Placement failure must enter one deterministic path:

- `FAILED_PLACEMENT` with structured failure result and debug-visible provenance;
- explicitly configured retry/fallback candidate policy;
- source-owned deferred placement queue;
- or invariant failure requiring investigation.

Any fallback search radius, alternate cell policy, retry count, or reward conversion is a tuning/design rule for Drop/Pickup or future OpenMir2 evidence contracts. It must not be invented here as authentic behavior. Until ADR-0016 readiness exists, fallback values are `mvp_provisional`.

### Pickup Transaction and No-Half-Commit Rule

Pickup commit is a staged transaction across GroundDropService, MapSpaceState, and InventoryService. The final result is `PICKED_UP` only after all commit conditions succeed.

Canonical order:

1. Validate ground drop exists and is `AVAILABLE`.
2. Validate request actor/context and pickup preconditions.
3. Verify spatial eligibility through latest committed `MapSpaceState` query/result or an ADR-0003-compatible authoritative command result.
4. Resolve deterministic claim by authoritative request sequence, stable actor/request ID, and `ground_drop_id` tie-breakers.
5. Stage inventory receive request through the future Inventory receive contract.
6. Submit and stage/confirm `MapSpaceState REMOVE_ITEM` command for the same `ground_drop_id`.
7. If both staged inventory receive and map removal are commit-ready, commit inventory receive, mark ground drop `PICKED_UP`, and publish final pickup result.
8. Presentation/audio/UI/QA consumes only the final pickup result or structured failure result.

Failure rules:

- If inventory receive rejects before commit, release the claim and keep the ground drop available unless another deterministic lifecycle rule such as despawn applies.
- If map removal fails before final inventory commit, abort staged inventory receive, keep or invalidate the ground drop according to structured failure reason, and do not grant the item.
- If inventory commit succeeds but ground state cannot be marked picked up, this is an invariant failure; implementation must surface it and prevent normal continuation rather than silently duplicating or deleting the reward.
- Pickup success must never be inferred from claim creation, inventory staging, UI click, signal emission, audio playback, or visual disappearance alone.

### Despawn vs Pickup Ordering

If despawn eligibility and pickup claim/commit target the same ground drop in the same authoritative window, ordering is deterministic and documented by the GroundDropService update policy.

Phase 1 policy:

- Lifecycle cleanup/despawn eligibility is evaluated before accepting new pickup claims for unclaimed drops.
- A drop already in `CLAIMED` state blocks despawn until claim resolution, unless the claim expires by deterministic sequence/tick policy before inventory staging.
- If a claim expires, it releases before the next claim resolution round.
- Despawn removal from map occupancy must use `MapSpaceState REMOVE_ITEM` command.

No ordering may depend on signal callback order, scene-tree order, node creation order, wall-clock time, frame delta, or physics overlap order.

### Key Interfaces

These GDScript-style interfaces are conceptual contracts. Implementations must use static typing, public API doc comments, no public mutable vars, defensive copies for collections, and status-bearing result wrappers.

```gdscript
class_name DropRollRequest
extends RefCounted

func get_drop_table_id() -> StringName: pass
func get_source_context_id() -> StringName: pass
func get_rng_stream_id() -> StringName: pass
func get_roll_sequence() -> int: pass
```

```gdscript
class_name DropRollResult
extends RefCounted

enum Status {
    ROLLED_DROP,
    NO_DROP,
    INVALID_TABLE,
    INVALID_ITEM_REFERENCE,
    INELIGIBLE_ITEM_REFERENCE,
    OPENMIR2_EVIDENCE_NOT_READY,
    RNG_STREAM_INVALID,
}

func get_status() -> int: pass
func get_item_id() -> StringName: pass
func get_definition_version() -> StringName: pass
func get_quantity() -> int: pass
func get_failure_reasons_copy() -> Array[StringName]: pass
func get_debug_provenance_copy() -> Dictionary: pass
```

```gdscript
class_name GroundDropRecord
extends RefCounted

enum LifecycleStatus {
    PENDING_PLACEMENT,
    AVAILABLE,
    CLAIMED,
    PICKED_UP,
    DESPAWNED,
    FAILED_PLACEMENT,
    INVALIDATED,
}

func get_ground_drop_id() -> int: pass
func get_item_id() -> StringName: pass
func get_definition_version() -> StringName: pass
func get_quantity() -> int: pass
func get_lifecycle_status() -> int: pass
func get_map_id() -> StringName: pass
func get_cell() -> Vector2i: pass
func get_claim_request_id() -> int: pass
```

```gdscript
class_name PickupCommitResult
extends RefCounted

enum Status {
    PICKED_UP,
    MISSING_GROUND_DROP,
    NOT_AVAILABLE,
    CLAIM_LOST,
    SPATIAL_VERIFY_FAILED,
    INVENTORY_RECEIVE_REJECTED,
    MAP_REMOVE_FAILED,
    DESPAWNED_BEFORE_CLAIM,
    OPENMIR2_EVIDENCE_NOT_READY,
    INVARIANT_FAILURE,
}

func get_status() -> int: pass
func is_committed() -> bool: pass
func get_ground_drop_id() -> int: pass
func get_inventory_receive_result_id() -> int: pass
func get_map_remove_command_sequence() -> int: pass
func get_failure_reasons_copy() -> Array[StringName]: pass
```

```gdscript
class_name PickupService
extends RefCounted

func request_pickup(request: PickupRequest) -> PickupCommitResult:
    # Orchestrates validation, deterministic claim, staged inventory receive,
    # staged MapSpaceState REMOVE_ITEM, and final commit/failure result.
    pass
```

### Immutable DTO Rule

Drop, ground, pickup, and result DTOs must:

- copy inbound arrays/dictionaries during construction;
- not expose public mutable vars, setters, mutators, Nodes, Resources, Callables, Signals, physics objects, or scene references;
- return defensive copies for arrays/dictionaries;
- not retain caller-owned mutable collections;
- be tested by mutating constructor inputs and getter-returned copies.

### Godot Scene / Physics / Signal Boundary

- `Area2D`, physics bodies, collision callbacks, labels, sprites, loot nodes, audio cues, animations, and UI widgets may provide presentation or candidate affordances only.
- Actual pickup eligibility and success come from PickupService + MapSpaceState + Inventory staged result DTOs.
- Signals may publish post-commit results; signal callback order must not decide drop rolls, placement, claim winner, pickup success, or inventory grant.
- `await`, timers, `call_deferred()`, `_process`, `_physics_process`, `_input`, and scene-tree insertion order must not be gameplay authority for drop/pickup state changes.

## Alternatives Considered

### Alternative 1: Drop system directly grants inventory on monster death

- **Description**: Monster death roll immediately creates inventory items without ground drops.
- **Pros**: Simple to implement; avoids pickup edge cases.
- **Cons**: Fails `爆装有戏` ground loot fantasy, skips map occupancy/readability, and does not validate pickup flow.
- **Rejection Reason**: Phase 1 requires visible ground loot and pickup.

### Alternative 2: Ground loot scene nodes as authority

- **Description**: Instantiate loot `Node2D`/`Area2D` scenes; node state decides availability and pickup.
- **Pros**: Familiar Godot workflow; easy visual prototyping.
- **Cons**: Scene lifecycle, physics callbacks, and signal order become gameplay authority; hard to test deterministically.
- **Rejection Reason**: Violates existing ADR scene-tree-independent and MapSpaceState authority boundaries.

### Alternative 3: Drop tables copy full item definition rows

- **Description**: Drop table rows include display, quality, stack, and equipment data so no catalog lookup is needed.
- **Pros**: Fewer runtime queries.
- **Cons**: Duplicates item template truth and creates drift from ADR-0015.
- **Rejection Reason**: ItemDefinitionRuntimeCatalog is the single template truth.

### Alternative 4: One monolithic LootSystem owns drop, ground, pickup, inventory grant, and presentation

- **Description**: A single system owns the whole reward pipeline.
- **Pros**: Centralized code path.
- **Cons**: Becomes a god object and conflicts with MapSpaceState, Item Definition, Inventory, UI, Audio, and Save ownership boundaries.
- **Rejection Reason**: The project requires separated authorities with explicit handoff contracts.

## Consequences

### Positive

- Prevents duplicate or lost loot through no-half-commit pickup semantics.
- Keeps map occupancy, item template truth, and inventory storage authorities separate.
- Provides deterministic roll, placement, claim, and pickup testability.
- Allows visible ground loot while preserving gameplay correctness.
- Keeps provisional drop/pickup work moving without false OpenMir2 authenticity.

### Negative

- Requires staged handoff infrastructure before pickup can be fully implemented.
- Requires future Inventory ADR compatibility for final receive/abort/commit semantics.
- More DTO/result types and failure cases than a direct grant prototype.

### Neutral

- Does not choose final drop rates, pickup range, despawn time, loot ownership rules, fallback radius, inventory capacity, or stack merge order.
- Does not implement presentation, audio, UI, or final Save/Load behavior.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| Pickup half-commit duplicates or deletes loot | Medium | High | Staged inventory + staged map remove + final commit invariant; invariant failure status. |
| GroundDropService and MapSpaceState become double occupancy authorities | Medium | High | GroundDrop stores lifecycle only; MapSpaceState mutates spatial occupancy only through commands. |
| Ground drops become shadow inventory instances | Medium | Medium | Store grant intent only; final inventory instance/stack truth belongs to future Inventory ADR. |
| Presentation or Area2D callback grants item early | Medium | High | Explicitly forbid presentation/physics/signal authority; tests/codereview gate. |
| RNG is nondeterministic | Medium | Medium | Inject deterministic RNG stream/seed; forbid `randomize()`, wall clock, and global RNG state. |
| ADR-0016 not Accepted but systems claim authenticity | Medium | High | Fail `openmir2_verified` claims until Accepted evidence/contract readiness exists; otherwise label provisional. |

## Performance Implications

| Metric | Before | Expected After | Budget |
|--------|--------|---------------|--------|
| CPU (frame time) | No implementation | Drop rolls and pickup commits are event-driven; no normal gameplay full-map or full-catalog scan | Must remain within 60 fps / 16.6 ms; reward pipeline should run only on death/pickup events |
| Memory | No implementation | GroundDropRecord and transient result DTOs proportional to active ground drops and requests | Phase 1 client under 1 GB RAM |
| Load Time | No drop table validation | Bootstrap validation of drop tables against ItemDefinition catalog | Acceptable during content/bootstrap; not per frame |
| Network | None | None | 0 KB/s in Phase 1 |

## Migration Plan

No existing drop/pickup implementation requires migration.

1. Create provisional Drop Table GDD or technical spec aligned with this ADR.
2. Implement fixture-backed `DropTableRuntimeCatalog` and validation against `ItemDefinitionRuntimeCatalog`.
3. Implement deterministic drop roll tests with injected RNG streams.
4. Implement `GroundDropService` lifecycle records and MapSpaceState `PLACE_ITEM` integration.
5. Define minimal Inventory staged receive interface in the future Inventory ADR.
6. Implement `PickupService` no-half-commit tests.
7. Add presentation/audio post-commit listeners only after result DTOs exist.

**Rollback plan**: If staged inventory integration is too heavy before Inventory ADR, pickup stories must remain blocked or use a clearly labeled test fake inventory receive contract. Do not switch to direct inventory mutation or scene-node pickup authority without superseding this ADR.

## Validation Criteria

- [ ] Drop table validation rejects missing, deprecated, ineligible, or non-spawnable item references.
- [ ] Same seed stream + same death context + same catalog produces identical drop roll results.
- [ ] Drop roll does not call `randomize()`, global RNG, wall clock, frame delta, scene order, or dictionary iteration order as authority.
- [ ] Ground drop placement uses MapSpaceState `PLACE_ITEM`; no service directly mutates item occupancy.
- [ ] Placement failure returns structured status and does not silently delete, reroll, or teleport reward.
- [ ] Two pickup requests for one `ground_drop_id` resolve deterministically.
- [ ] Inventory receive rejection releases claim and leaves ground drop available unless a deterministic lifecycle rule applies.
- [ ] Map remove failure aborts staged inventory and does not grant item.
- [ ] Final pickup success is published only after ground lifecycle, map occupancy removal, and inventory receive commit conditions succeed.
- [ ] Despawn vs pickup same-window behavior follows the documented deterministic ordering.
- [ ] DTO mutation/aliasing tests prove constructor inputs and getter-returned collections cannot mutate authority.
- [ ] Presentation/audio/UI cannot mark pickup as successful before `PickupCommitResult.PICKED_UP`.
- [ ] OpenMir2-authentic claims fail without ADR-0016 Accepted readiness; provisional claims remain allowed when labeled.

## GDD Requirements Addressed

| GDD Document | System | Requirement | How This ADR Satisfies It |
|-------------|--------|-------------|--------------------------|
| `design/gdd/systems-index.md` | Systems Index | `TR-systems-index-005`: 掉落与拾取必须拆分事件链：死亡、掉落生成、地面物、拾取请求、背包接收。 | Defines the split pipeline and ownership boundary from drop roll through pickup commit and inventory handoff. |
| `design/gdd/systems-index.md` | Systems Index | `TR-systems-index-004`: MVP includes drop table, pickup, inventory/equipment chain. | Provides the architecture seam needed to begin Drop Table and Drop/Pickup design without stealing Inventory/Equipment ownership. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | Map Coordinate / Blocking / Y-sort | `TR-map-space-013`: dropped items occupy logical cells, participate in pickup/Y-sort, and are non-blocking in MVP. | Requires ground drops to place/remove item occupancy through `MapSpaceState` and keep lifecycle separate from spatial truth. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | Map Coordinate / Blocking / Y-sort | `TR-map-space-014`: loot readability floor; placement failure cannot delete/reroll/silently move rewards. | Defines structured placement failure and forbids silent deletion, reroll, or presentation-driven relocation. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | Map Coordinate / Blocking / Y-sort | `TR-map-space-017`: spawn/drop/pickup use map-space legality queries while downstream systems own their rules. | Uses `MapSpaceState` spatial verification and commands while GroundDrop/Pickup own lifecycle. |
| `design/gdd/item-definition-system.md` | Item Definition | `TR-item-definition-017`: Drop Table, Drop/Pickup, Inventory, Equipment, Attributes, UI, Save/Load boundaries must respect Item Definition source-of-truth. | Drop tables store item refs and validate through ItemDefinition; they do not copy template truth. |
| `design/gdd/item-definition-system.md` | Item Definition | `TR-item-definition-015`: deprecated definitions need explicit save/debug/migration semantics and cannot silently remap. | Drop table validation fails closed for deprecated/ineligible references unless a later migration/debug policy allows them. |
| `design/gdd/openmir2-behavior-mapping-spike.md` | OpenMir2 Behavior Mapping Spike | Drop/Pickup behavior requires source evidence before OpenMir2-authentic claims. | Requires ADR-0016 Accepted readiness for `openmir2_verified`; otherwise labels behavior provisional. |
| `design/gdd/game-concept.md` | Game Concept | Phase 1 30-second loop must include visible drop and pickup before equipment/growth. | Establishes deterministic visible-ground-loot and pickup handoff architecture. |

## Related

- `docs/architecture/adr-0001-map-data-representation.md`
- `docs/architecture/adr-0002-typed-query-result-schema.md`
- `docs/architecture/adr-0003-authoritative-occupancy-reservation-update-ordering.md`
- `docs/architecture/adr-0015-item-definition-runtime-data-validation-query-and-versioning.md`
- `docs/architecture/adr-0016-openmir2-evidence-mapping-registry-and-provisional-contract-governance.md`
- `docs/architecture/architecture-review-2026-06-05.md`
- `design/gdd/systems-index.md`
- `design/gdd/map-coordinate-blocking-y-sort-system.md`
- `design/gdd/item-definition-system.md`
- Future Inventory / Equipment Instance and Modifier Transaction Boundary ADR
