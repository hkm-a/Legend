# ADR-0001: Map Data Representation

## Status

Accepted

## Date

2026-06-04

## Last Verified

2026-06-04

## Decision Makers

hkm + Claude Code Game Studios

## Summary

This ADR decides how Phase 1 authoritative logical map data is represented in Godot 4.6.3 for the map coordinate / blocking / Y-sort system. We choose a Resource-first logical map: `MapDefinition` stores static gameplay cell facts in flattened typed arrays, while `TileMapLayer` remains visual/editor presentation and cannot override gameplay truth.

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Rendering / Core map data / Scripting |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff and must be verified against engine reference docs. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md`; `docs/engine-reference/godot/modules/rendering.md` |
| **Post-Cutoff APIs Used** | `TileMapLayer` as the non-deprecated 2D tile presentation layer; Godot 4.5+ `Resource.duplicate_deep()` only for explicit mutable deep-copy tooling/tests involving nested resources. |
| **Verification Required** | Verify `MapDefinition` Resource serialization, `Packed*Array` dimensions, `Vector2i` cell indexing, TileMapLayer-vs-logical validation reports, runtime immutability, and no dependency on deprecated `TileMap` or `YSort` nodes. |

> **Note**: If Knowledge Risk is MEDIUM or HIGH, this ADR must be re-validated if the
> project upgrades engine versions. Flag it as "Superseded" and write a new ADR.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | None |
| **Enables** | Future ADRs for typed query result schema, authoritative occupancy update ordering, deterministic Y-sort implementation, and input projection / coordinate conversion. |
| **Blocks** | Implementation stories for `地图坐标 / 阻挡 / Y-sort 系统` cannot start until this ADR is Accepted. |
| **Ordering Note** | This ADR defines the static map data source only. Runtime occupancy/reservation ordering and rendering sort execution require separate ADRs before full implementation. |

## Context

### Problem Statement

The approved map coordinate / blocking / Y-sort GDD requires a Godot implementation to expose logical bounds, static passability, item placement flags, visual obstruction metadata, Y-sort anchors, and validation evidence. Without deciding where static map truth lives, downstream movement, spawn, drop, pickup, combat, rendering, and debug systems may each read different sources such as tile visuals, scene nodes, collision helpers, or ad hoc dictionaries.

### Current State

There is no implemented map data layer and no previous ADR. The GDD is approved for downstream architecture, but explicitly gates implementation on Godot 4.6.3 ADRs. The architecture registry has no existing stances.

### Constraints

- Godot 4.6.3 is post-cutoff; implementation must avoid deprecated APIs and verify engine-sensitive decisions.
- Gameplay authority must be logical-grid based; visual/world/screen state must not override logical map state.
- `TileMap` is deprecated; Godot 4.6.3 visual tile work must use `TileMapLayer` where tile layers are needed.
- Normal gameplay must not scan the full map every frame.
- Static map data must be unit-testable without requiring a loaded scene tree.
- OpenMir2 coordinate/source evidence remains incomplete; this ADR must support Phase 1 synthetic maps and later imported/source-verified maps.

### Requirements

- Store valid map bounds and detect invalid dimensions or mismatched cell data.
- Expose static actor passability, item placement, visual obstruction tags, authoring validation tags, and Y-sort anchor metadata.
- Support O(1) indexed cell lookup for dense Phase 1 maps.
- Keep static map data immutable at runtime; runtime actor occupancy, item occupancy, and reservations are owned separately.
- Provide validation reports/snapshots for QA without making full-grid overlays normal gameplay behavior.

## Decision

Use a Resource-first logical map architecture. A custom `MapDefinition` Resource is the authoritative source for static gameplay map facts. It stores `map_id`, `width`, `height`, and flattened dense per-cell arrays indexed by `index = cell.y * width + cell.x`. Dense primitive facts should use `Packed*Array` fields where practical; typed `Array[Type]` may be used for editor-facing metadata, references, or non-primitive data. `TileMapLayer` may provide visual/editor presentation and may carry matching metadata for validation, but it is not a gameplay authority for movement, blocking, drops, pickup, combat-distance inputs, or Y-sort facts.

Runtime mutable spatial state is held in `MapSpaceState` or an equivalent runtime service that reads a `MapDefinition` and owns actor occupancy, item occupancy, and reservations. `MapSpaceState` must not mutate the `MapDefinition` asset. `MapDefinition` is treated as runtime-immutable; systems must not mutate exported arrays or nested Resource references loaded from shared assets. Runtime systems should access static facts through read-only query methods where practical; direct exported array mutation is authoring/tooling-only. Any editor tool, test, or runtime experiment that requires mutation must operate on an explicit deep duplicate, using Godot 4.5+ `duplicate_deep()` where nested resources are involved, and must not save that duplicate back to the shared asset unless it is an intentional authoring operation.

Field-level schemas for `MapCellStaticFacts` and `MapValidationReport` are intentionally deferred to the typed query result schema ADR. Concrete enum or bitmask encodings for packed arrays are likewise deferred to schema/implementation specification; this ADR only fixes the ownership, storage shape, and authority boundary.

### Architecture

```text
OpenMir2/source fixture or synthetic map authoring
        |
        v
Map import / bake / authoring validation tool
        |
        v
MapDefinition Resource  <------ optional consistency check ------ TileMapLayer visuals
(static authoritative data)                                  (presentation/editor only)
        |
        v
MapSpaceState runtime
(dynamic actor/item/reservation state)
        |
        +--> movement / spawn / drop / pickup / combat queries
        +--> bounded QA snapshot / validation report
        +--> rendering receives Y-sort anchor metadata, but does not own gameplay facts
```

### Key Interfaces

```gdscript
class_name MapDefinition
extends Resource

@export var map_id: StringName
@export var width: int
@export var height: int
@export var actor_passability: PackedByteArray
@export var item_placeability: PackedByteArray
@export var visual_obstruction_tags: PackedInt32Array
@export var y_sort_anchor_ids: PackedInt32Array
@export var validation_tags: PackedInt32Array

func is_bounds_valid() -> bool:
    # width > 0, height > 0, and all required dense arrays have width * height entries.

func is_cell_in_bounds(cell: Vector2i) -> bool:
    # Returns true only when bounds are valid and 0 <= cell.x < width, 0 <= cell.y < height.

func get_cell_index(cell: Vector2i) -> int:
    # Returns cell.y * width + cell.x for in-bounds cells; returns -1 for invalid cells.

func get_cell_static_facts(cell: Vector2i) -> MapCellStaticFacts:
    # Returns a query result snapshot/value assembled from arrays, not a per-cell stored Resource.

func validate_for_playable_contract() -> MapValidationReport:
    # Returns bounded authoring/QA validation evidence for the GDD's Phase 1 playable contract.
```

`MapCellStaticFacts` is a query-result value or DTO, not one stored Resource per cell. Hot-path code may use lower-level no-allocation helpers such as `is_cell_actor_passable(cell: Vector2i) -> bool`, `is_cell_item_placeable(cell: Vector2i) -> bool`, or `get_cell_flags(cell: Vector2i) -> int` where allocation pressure matters.

### Implementation Guidelines

- Use `Vector2i` for all logical grid cells.
- Use flattened arrays for dense cell data; do not store one Resource or Node per logical cell in MVP.
- Treat packed array lengths as part of bounds validity; mismatched sizes fail validation and cause runtime queries to return unavailable/unknown through the query schema ADR.
- Keep `TileMapLayer` visual-only. TileSet custom data, TileMapLayer collision shapes, navigation shapes, and scene placement may assist authoring or validation but must not override `MapDefinition` at runtime.
- Use `Node2D.y_sort_enabled` or later rendering ADR mechanisms for presentation. Do not use deprecated `YSort` nodes, and never use scene-tree insertion order as deterministic gameplay or sort authority.
- Prefer validation reports and bounded snapshots before full-grid overlays. Full-grid overlays are debug-only and not normal performance evidence.

## Alternatives Considered

### Alternative 1: TileMapLayer-authoritative gameplay data

- **Description**: Store passability, item placement, and anchor facts directly in `TileMapLayer` / TileSet custom data and read those during gameplay.
- **Pros**: Editor-visible; fewer separate assets; intuitive for tile artists.
- **Cons**: Couples gameplay truth to visual tiles; tile art changes can alter movement or drop legality; harder to unit-test without scene setup; risks confusion between visual collision/custom data and gameplay authority.
- **Estimated Effort**: Lower initial authoring effort, higher integration and validation risk.
- **Rejection Reason**: Contradicts the GDD's logical data authority boundary and makes visual layer changes dangerous for gameplay.

### Alternative 2: Runtime scene-node / Area2D registry authority

- **Description**: Represent static blockers, props, and cells with scene nodes or Area2D objects that register facts at runtime.
- **Pros**: Flexible for handcrafted scenes; easy to attach editor gizmos; can reuse Godot scene hierarchy.
- **Cons**: Too many nodes for dense grids; scene-tree order and lifecycle become implicit data authority; harder to keep deterministic; poor fit for source map import and headless unit tests.
- **Estimated Effort**: Medium initial effort, high maintenance/performance risk.
- **Rejection Reason**: It turns a dense logical grid into scene-object state and conflicts with deterministic O(1) query requirements.

### Alternative 3: Dictionary keyed by Vector2i as the primary map store

- **Description**: Store only authored cells in `Dictionary[Vector2i, CellData]`.
- **Pros**: Flexible for sparse maps; easier to inspect ad hoc in prototypes.
- **Cons**: Less type-safe; higher lookup overhead; missing-key semantics can mask data errors; less direct validation of rectangular OpenMir2-like maps.
- **Estimated Effort**: Similar prototype effort, higher long-term correctness risk.
- **Rejection Reason**: Phase 1 and OpenMir2-like map data are dense rectangular grids; flattened arrays better support bounds validation and deterministic tests.

## Consequences

### Positive

- Gameplay systems share one static map truth independent of visuals.
- Static map queries are unit-testable without loading scenes.
- Dense arrays allow predictable O(1) lookups and straightforward bounds validation.
- Later OpenMir2 import/bake tooling can target `MapDefinition` without rewriting gameplay systems.
- Visual map iteration remains possible through `TileMapLayer` without making art data authoritative.

### Negative

- Requires an importer/baker or authoring helper instead of relying only on Godot tile editing.
- Designers cannot treat TileMapLayer edits as automatically changing gameplay unless validation/bake updates `MapDefinition`.
- Resource arrays are less comfortable to hand-edit for large maps.
- Separation creates an extra consistency-validation step between logical data and visuals.

### Neutral

- Runtime dynamic state is deliberately moved out of static map assets.
- Future rendering/Y-sort and input projection ADRs must still define their own Godot-specific execution details.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| Runtime code accidentally mutates shared `MapDefinition` Resource arrays. | Medium | High | Treat `MapDefinition` as runtime-immutable; runtime state lives in `MapSpaceState`; mutation requires explicit deep duplicate/tooling path. |
| Visual TileMapLayer and logical MapDefinition drift apart. | Medium | Medium | Require validation report comparing map_id, dimensions, expected visual coverage, and authoring tags. |
| Packed arrays reduce editor readability. | High | Medium | Use importer/baker/test fixtures; expose validation reports instead of requiring per-cell inspector editing. |
| Query result DTO allocation becomes hot-path overhead. | Medium | Medium | Provide no-allocation helper methods for hot paths; reserve rich snapshots for debug/QA or non-hot calls. |
| Future OpenMir2 evidence changes map semantics. | Medium | High | Keep source-specific assumptions outside this ADR where possible; import/bake layer can revise produced MapDefinition fields without changing logical authority architecture. |

## Performance Implications

| Metric | Before | Expected After | Budget |
|--------|--------|---------------|--------|
| CPU (cell lookup) | No implementation | O(1) index lookup for static facts; no full-map scan during normal gameplay | Must respect 60 fps / 16.6 ms frame budget and GDD query guardrails |
| Memory | No implementation | Dense primitive arrays sized by `width * height`; lower overhead than per-cell Resources/Nodes | Phase 1 client under 1 GB RAM |
| Load Time | No implementation | Resource load + array size validation; optional offline/import validation can scan full map | Full-map validation allowed at load/offline time, not every gameplay frame |
| Network | N/A | N/A | N/A |

## Migration Plan

No existing implementation requires migration.

1. Create small synthetic `MapDefinition` fixture for Phase 1.
2. Add validation for dimensions and required array lengths.
3. Add unit tests for bounds/indexing and static passability/item-placeability lookup.
4. Add TileMapLayer consistency validation only after visual test maps exist.
5. Introduce importer/baker once OpenMir2 or resource/map conversion evidence is available.

**Rollback plan**: If Resource-first proves insufficient, mark this ADR Superseded and write a replacement ADR. Keep gameplay callers behind `MapDefinition`/`MapSpaceState` interfaces so the backing store can change without rewriting downstream systems.

## Validation Criteria

- [ ] A synthetic `MapDefinition` with matching dimensions validates successfully.
- [ ] A `MapDefinition` with mismatched array lengths fails validation and produces unavailable/unknown query evidence downstream.
- [ ] `Vector2i` bounds and `index = y * width + x` behavior are covered by unit tests.
- [ ] Static actor passability and item placement facts are read from `MapDefinition`, not `TileMapLayer`.
- [ ] A TileMapLayer mismatch is reported by validation and cannot override gameplay facts.
- [ ] Hot-path lookup can use no-allocation helper methods.
- [ ] No implementation uses deprecated `TileMap` or `YSort` nodes.

## GDD Requirements Addressed

| GDD Document | System | Requirement | How This ADR Satisfies It |
|-------------|--------|-------------|--------------------------|
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Logical Grid Authority Rule | Makes `MapDefinition` logical grid data the static gameplay authority. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Map Coordinate Contract Rule | Stores `map_id`, bounds, dimensions, and indexable logical cells. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Map Blocking Contract Rule | Stores static actor passability and item placement fields separate from runtime dynamic occupancy. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Map Authoring and Validation Contract Rule | Requires `validate_for_playable_contract()` and validation reports/snapshots. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Data Authority Rule | Prevents visual tiles, sprites, TileMapLayer collision, or scene nodes from overriding gameplay map data. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Performance Guardrail Rule | Uses O(1) dense array lookup and forbids full-map scans during normal gameplay frames. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Implementation Boundary and ADR Prerequisites Rule | Resolves the required map data representation ADR while leaving query schema, Y-sort execution, update ordering, and input projection to later ADRs. |

## Related

- `design/gdd/map-coordinate-blocking-y-sort-system.md`
- Future ADR: typed query result schema
- Future ADR: authoritative occupancy update ordering
- Future ADR: deterministic Y-sort implementation
- Future ADR: input projection / coordinate conversion
