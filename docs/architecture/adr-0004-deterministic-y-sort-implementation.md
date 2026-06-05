# ADR-0004: Deterministic Y-sort Implementation

## Status

Accepted

## Date

2026-06-04

## Last Verified

2026-06-04

## Decision Makers

hkm + Claude Code Game Studios

## Summary

This ADR defines how map-space presentation ordering is computed deterministically for actors, dropped items, and sortable map objects. We choose a `MapSortCoordinator` that computes the explicit key `(anchor_y, object_type_sort_rank, stable_instance_id)` and applies presentation order in batches, while Godot CanvasItem/Node2D mechanisms remain implementation details rather than gameplay authority.

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Rendering / Core map presentation |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff and must be verified against engine reference docs. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md`; `docs/engine-reference/godot/modules/rendering.md` |
| **Post-Cutoff APIs Used** | `TileMapLayer` may be present as visual layer from ADR-0001; Godot 4.x `Node2D.y_sort_enabled` may be used only as a rendering aid. |
| **Verification Required** | Verify that the chosen Godot presentation mechanism can apply deterministic `(anchor_y, type_rank, stable_id)` ordering without deprecated `YSort`, hidden scene-tree insertion order, or unbounded `z_index` mapping. |

> **Note**: If Knowledge Risk is MEDIUM or HIGH, this ADR must be re-validated if the
> project upgrades engine versions. Flag it as "Superseded" and write a new ADR.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | ADR-0001 Map Data Representation; ADR-0002 Typed Query Result Schema; ADR-0003 Authoritative Occupancy / Reservation Update Ordering |
| **Enables** | Y-sort implementation stories, rendering/debug evidence, and loot readability validation. |
| **Blocks** | Any story that claims deterministic Y-sort ordering or map-space presentation ordering. |
| **Ordering Note** | This ADR defines deterministic presentation ordering only. It does not decide loot labels, VFX, art style, target selection priority, or gameplay legality. |

## Context

### Problem Statement

The GDD requires world objects to sort by `anchor_y`, then object type rank, then stable instance ID. Godot can assist with 2D sorting, but relying only on automatic Y-sort or incidental scene-tree order cannot guarantee the full tuple key, especially when objects share the same anchor Y. Without a project-owned sort coordinator, dropped items, actors, and props may flicker or reorder unpredictably, and QA cannot verify the exact ordering contract.

### Current State

ADR-0001 provides static Y-sort anchor metadata through `MapDefinition`. ADR-0003 provides authoritative runtime state changes and dirty events. No rendering sort implementation exists yet.

### Constraints

- `YSort` nodes are deprecated; new implementation must not depend on them.
- `Node2D.y_sort_enabled` may be used as a rendering aid, but it is not the full deterministic tuple sort authority.
- Scene-tree insertion order, spawn callback order, and arbitrary signal order must not decide tie-breaks.
- Y-sort affects visual ordering only and must not change movement, blocking, pickup, spawn, combat, or target validity.
- Missing anchors, missing type ranks, or invalid stable IDs must surface structured debug reasons instead of silently falling back.

### Requirements

- Compute `y_sort_key = (anchor_y, object_type_sort_rank, stable_instance_id)` exactly as the GDD defines.
- Use stable IDs that are comparable within an active sorted set.
- Batch dirty sort input refreshes instead of resorting every object every frame.
- Provide debug evidence containing anchor, type rank, stable ID, final order, and invalid input reasons.
- Keep presentation ordering independent of gameplay authority.

## Decision

Use a `MapSortCoordinator` or equivalent presentation-ordering service. Sortable actors, dropped items, and map objects register sortable metadata. The coordinator computes a deterministic key from the authoritative logical/render anchor data and active type-rank policy. It sorts lexicographically by `(anchor_y, object_type_sort_rank, stable_instance_id)` and applies the resulting presentation order through a Godot-specific mechanism verified during implementation.

Godot mechanisms such as `Node2D.y_sort_enabled`, `z_index`, layered `Node2D` containers, or coordinator-managed child order may be used only as final presentation application tools. They do not own the sort semantics. If child order is used, it must be actively set by the coordinator from the deterministic sorted list; incidental scene-tree insertion order is forbidden. If `z_index` is used, the implementation must prove that the key mapping fits the needed range and preserves tie-breaks.

The coordinator listens to dirty events from map load, actor movement commit, item placement/removal, registration changes, anchor changes, type-rank changes, stable ID changes, and map unload. Static sortable objects are sorted on load and only reprocessed when sort-relevant metadata changes. Dynamic objects mark dirty when their sort-relevant inputs change. Full re-sort every gameplay frame is not the normal path.

### Architecture

```text
MapDefinition Y-sort anchor metadata
        |
MapSpaceState runtime object/item/actor state ---- dirty events
        |
        v
MapSortCoordinator
        |
        +--> validate anchor/type-rank/stable-ID inputs
        +--> compute y_sort_key tuple
        +--> sort dirty/active sortable set deterministically
        +--> apply presentation order via verified Godot rendering mechanism
        +--> export debug/QA ordering snapshot
```

### Key Interfaces

```gdscript
class_name SortableRegistration
extends RefCounted

var sortable_id: int
var map_id: StringName
var object_type_sort_rank: int
var stable_instance_id: int
var anchor_y: float
var node: Node2D

class_name YSortKey
extends RefCounted

var anchor_y: float
var object_type_sort_rank: int
var stable_instance_id: int

class_name MapSortCoordinator
extends RefCounted

func register_sortable(registration: SortableRegistration) -> void:
    # Registers a sortable presentation object and marks it dirty.

func unregister_sortable(sortable_id: int) -> void:
    # Removes a sortable object and clears its dirty state.

func mark_sort_dirty(sortable_id: int) -> void:
    # Coalesces dirty refresh requests.

func compute_y_sort_key(sortable_id: int) -> SpatialQueryResult:
    # Returns a presentation/debug result containing valid key facts or invalid_y_sort_* reason.

func refresh_dirty_sort_order() -> void:
    # Applies deterministic presentation order for dirty sortable sets.

func export_sort_debug_snapshot() -> SpatialQueryResult:
    # Exposes bounded debug evidence for QA/manual review.
```

Exact storage and node ownership may change during implementation, but the tuple key and invalid-input semantics must remain testable without relying on scene-tree insertion order.

### Implementation Guidelines

- Use the GDD order: `anchor_y`, then `object_type_sort_rank`, then `stable_instance_id`.
- Treat missing anchor as `invalid_y_sort_anchor`.
- Treat missing/unconfigured type rank as `invalid_y_sort_rank`.
- Treat missing or mixed incomparable stable IDs as `invalid_stable_sort_id`.
- Do not silently sort invalid inputs by array insertion order, child order, or signal order.
- Use dirty batching and coalesced refreshes; immediate resort is debug-only unless a story proves it is bounded.
- Avoid every-frame discovery of sortable nodes.
- Handle node freed/tree-exit/map unload by unregistering or invalidating registrations before refresh.
- Loot readability affordances are owned by downstream rendering/drop/pickup presentation stories; this ADR only preserves deterministic ordering and evidence.

## Alternatives Considered

### Alternative 1: Godot automatic Y-sort only

- **Description**: Enable `Node2D.y_sort_enabled` and let Godot order child nodes by Y position.
- **Pros**: Simple, idiomatic for many 2D scenes, low custom code.
- **Cons**: Does not define full tie-break by object type rank and stable ID; equal-Y ordering can fall back to hidden presentation details.
- **Estimated Effort**: Low initial effort, insufficient determinism.
- **Rejection Reason**: The GDD requires explicit deterministic tuple ordering, not only Y-position sorting.

### Alternative 2: Manual scene-tree reorder by spawn order

- **Description**: Move child nodes in the scene tree or rely on insertion/spawn order for tie-breaks.
- **Pros**: Directly affects draw order; easy to observe in the editor.
- **Cons**: Scene-tree order becomes hidden gameplay/presentation authority; spawn/callback order can vary; save/load or respawn can change results.
- **Estimated Effort**: Medium effort with high correctness risk.
- **Rejection Reason**: Scene-tree insertion order is explicitly forbidden as a deterministic tie-break authority.

### Alternative 3: Per-node z_index updates without a coordinator

- **Description**: Each actor/item/prop computes and sets its own `z_index` or presentation order.
- **Pros**: Local implementation is simple; avoids central service at first.
- **Cons**: Tie-break policy fragments across classes; range mapping may lose rank/ID information; difficult to debug globally.
- **Estimated Effort**: Medium effort; high maintenance risk.
- **Rejection Reason**: The project needs one shared sort policy and debug snapshot across all sortable object types.

## Consequences

### Positive

- Stable, testable visual ordering for actors, items, and sortable props.
- Clear separation between gameplay legality and presentation ordering.
- Debug evidence can show why an object is sorted or invalid.
- Future rendering implementation can change Godot presentation mechanics without changing sort semantics.

### Negative

- Requires a custom coordinator and registration lifecycle.
- Requires careful integration with Node2D/CanvasItem presentation details.
- Invalid sort inputs must be handled explicitly instead of hidden by Godot defaults.

### Neutral

- This ADR does not decide final loot visibility affordances.
- This ADR does not require complex roof/canopy overlay support in MVP.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| Godot presentation mechanism cannot directly encode full tuple. | Medium | Medium | Keep tuple computation independent; choose verified application mechanism during implementation. |
| Dirty batching misses object changes. | Medium | Medium | Register dirty events for movement commit, item placement/removal, anchor/type/id changes, and map unload. |
| Stable IDs are inconsistent after save/load or respawn. | Medium | High | Treat invalid/mixed IDs as structured failures and test ID restoration/regeneration rules. |
| Sorting code drifts into gameplay authority. | Low | High | Enforce that movement, pickup, combat, and spawn queries read logical state, not draw order. |

## Performance Implications

| Metric | Before | Expected After | Budget |
|--------|--------|---------------|--------|
| CPU | No implementation | Sort cost proportional to dirty sortable set or active visible/sortable set, not full map scan every frame | Must respect 60 fps / 16.6 ms and GDD dirty-refresh guardrails |
| Memory | No implementation | Sort registrations and debug snapshots for active sortable objects | Phase 1 client under 1 GB RAM |
| Load Time | No implementation | Static sort registrations processed on map load | Full static validation allowed at load/offline time |
| Network | N/A | N/A | N/A |

## Migration Plan

No existing implementation requires migration.

1. Define object type rank policy from the GDD MVP order.
2. Define sortable registration lifecycle.
3. Implement key computation and invalid-input query results.
4. Implement dirty batching and refresh.
5. Choose and verify Godot presentation application mechanism.
6. Add unit tests for key comparison and integration/debug evidence for scene ordering.

**Rollback plan**: If the chosen presentation mechanism fails, keep `MapSortCoordinator` and replace only the application mechanism. If the tuple policy changes, revise the GDD or write a superseding ADR.

## Validation Criteria

- [ ] `y_sort_key` includes `anchor_y`, `object_type_sort_rank`, and `stable_instance_id` in that order.
- [ ] Equal anchor values are resolved by type rank, then stable ID.
- [ ] Invalid anchor, rank, or stable ID returns the correct structured reason.
- [ ] Scene-tree insertion order is not used as a tie-break.
- [ ] Dirty refreshes are coalesced and do not require full-map scans during normal gameplay.
- [ ] QA/debug snapshot shows anchor, type rank, stable ID, final ordering, and invalid-input reasons.
- [ ] Movement, blocking, pickup, spawn, and combat query results are unchanged by visual ordering.
- [ ] No implementation uses deprecated `YSort` nodes.

## GDD Requirements Addressed

| GDD Document | System | Requirement | How This ADR Satisfies It |
|-------------|--------|-------------|--------------------------|
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Y-sort Anchor Rule | Requires sortable registrations to expose anchors used by the coordinator. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Y-sort Visual-Only Rule | Keeps presentation ordering separate from gameplay legality. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Stable Y-sort Tie-Breaker Rule | Implements tuple ordering by anchor Y, type rank, and stable ID. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | `y_sort_key` Formula | Defines explicit key computation and invalid-input reasons. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Loot Readability Floor Rule | Preserves deterministic ordering evidence needed for downstream loot readability validation. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Performance Guardrail Rule | Uses dirty batching rather than every-frame full-map sorting. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Implementation Boundary and ADR Prerequisites Rule | Resolves the deterministic Y-sort strategy prerequisite while leaving art/presentation affordances to later stories. |

## Related

- `docs/architecture/adr-0001-map-data-representation.md`
- `docs/architecture/adr-0002-typed-query-result-schema.md`
- `docs/architecture/adr-0003-authoritative-occupancy-reservation-update-ordering.md`
- `design/gdd/map-coordinate-blocking-y-sort-system.md`
- Future rendering, loot presentation, and debug overlay stories
