# ADR-0005: Input Projection / Coordinate Conversion

## Status

Accepted

## Date

2026-06-04

## Last Verified

2026-06-04

## Decision Makers

hkm + Claude Code Game Studios

## Summary

This ADR defines the architecture for converting allowed world-interaction input into logical map cells. We choose a `MapProjection` service that converts screen/viewport input through camera/world coordinates into a logical `Vector2i` candidate, returning structured `invalid_coordinate` failures for ambiguous or unresolved input instead of guessing.

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Input / Rendering / Core map data |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff and must be verified against engine reference docs. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md`; `docs/engine-reference/godot/modules/input.md`; `docs/engine-reference/godot/modules/rendering.md` |
| **Post-Cutoff APIs Used** | No new post-cutoff input API is required, but Godot 4.6 dual-focus behaviour must be tested because mouse/touch focus is separate from keyboard/gamepad focus. |
| **Verification Required** | Verify screen-to-viewport, camera/world transform, viewport stretch/zoom, UI-consumed input gating, and invalid/ambiguous coordinate failure behaviour under Godot 4.6.3. |

> **Note**: If Knowledge Risk is MEDIUM or HIGH, this ADR must be re-validated if the
> project upgrades engine versions. Flag it as "Superseded" and write a new ADR.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | ADR-0001 Map Data Representation; ADR-0002 Typed Query Result Schema |
| **Enables** | Click-to-move stories, pickup candidate input stories, target selection spatial lookup, and coordinate debug evidence. |
| **Blocks** | Any implementation story that converts screen, mouse, camera, or render positions into gameplay logical cells. |
| **Ordering Note** | This ADR defines coordinate conversion only. It does not decide click priority, target selection UX, pickup priority, movement path choice, or OpenMir2-authentic projection values. |

## Context

### Problem Statement

The GDD separates logical grid coordinates, world/render positions, and screen/input positions. Without a single projection contract, controllers, UI, movement, pickup, combat, and target selection may each convert mouse or screen positions differently. That would create inconsistent behavior such as clicks walking to one cell, pickup querying another, or UI-covered clicks still becoming world movement.

### Current State

ADR-0001 defines `MapDefinition` logical map data and forbids `TileMapLayer` as gameplay authority. ADR-0002 defines structured result schema. No projection/conversion implementation exists yet.

### Constraints

- Gameplay uses logical `Vector2i` cells as authority.
- Screen and world coordinates are inputs to conversion, not gameplay truth.
- `TileMapLayer` may assist visual/editor presentation or validation, but must not become gameplay coordinate authority.
- Actor/controller-local conversion logic is forbidden because it fragments semantics.
- Godot 4.6 dual-focus means mouse/touch focus and keyboard/gamepad focus are separate; UI/input routing must gate world projection before conversion.
- OpenMir2 coordinate origin, axis direction, tile size, and projection semantics remain evidence-gated. Phase 1 synthetic projection is MVP provisional.

### Requirements

- Convert allowed world-interaction input through a testable pipeline.
- Output logical cells as `Vector2i`.
- Return structured `invalid_coordinate` for unresolved, UI-blocked, or ambiguous projection cases.
- Never silently fallback to `(0, 0)`, nearest arbitrary cell, TileMap visual cell, or last valid cell.
- Support tests for camera zoom, viewport transform, out-of-bounds input, UI-covered input, and ambiguous projection.

## Decision

Use a `MapProjection` service or equivalent coordinate-conversion boundary. Input routing decides whether an event is eligible for world projection. Only events that pass UI/input ownership gating are submitted to `MapProjection`. `MapProjection` converts screen or viewport points through the active viewport/camera/world transform and then maps the resulting world/render position to a logical `Vector2i` candidate according to the active map projection contract.

If conversion cannot produce exactly one valid candidate, it returns a `SpatialQueryResult` with `query_context = COORDINATE_CONVERSION` and `primary_reason = INVALID_COORDINATE`. The projection layer does not guess fallback cells and does not decide final target selection priority. Target selection may use the projected candidate and other object hit facts, but it owns click priority and UX decisions.

`MapProjection` must not read `TileMapLayer` as gameplay truth. Temporary editor/debug helpers may compare projection output to visual `TileMapLayer` locations for validation, but final logical coordinates must be validated against `MapDefinition`/map contract data.

### Architecture

```text
InputEvent / cursor position
        |
        v
Input routing / UI ownership gate
        |
        |         +--> UI consumes event: no world projection
        v
MapProjection
        |
        +--> screen position
        +--> viewport-local position
        +--> camera/world position
        +--> active map projection transform
        +--> logical Vector2i candidate
        +--> typed SpatialQueryResult for success/failure
        |
        v
Movement / pickup / target selection / debug consumers
```

### Key Interfaces

```gdscript
class_name MapProjection
extends RefCounted

func project_screen_to_logical(screen_position: Vector2) -> SpatialQueryResult:
    # Converts an allowed screen-space point into a logical cell or invalid_coordinate.

func project_viewport_to_logical(viewport_position: Vector2) -> SpatialQueryResult:
    # Converts an allowed viewport-local point into a logical cell or invalid_coordinate.

func project_world_to_logical(world_position: Vector2) -> SpatialQueryResult:
    # Converts a world/render point into a logical Vector2i candidate or invalid_coordinate.

func logical_to_world_anchor(cell: Vector2i) -> SpatialQueryResult:
    # Converts a logical cell into the stable world/render anchor defined by the active projection contract.
```

A successful conversion result includes `cell_facts.cell` or equivalent candidate data in the typed snapshot. Exact Phase 1 synthetic map projection values are implementation/test fixture choices, not source-authentic OpenMir2 claims.

### Implementation Guidelines

- Use `Vector2i` for logical cell output.
- Do not use raw `InputEvent.position` as world position; account for viewport, camera, zoom, stretch, and subviewport transforms as applicable.
- Prefer `_unhandled_input(event)` or an equivalent UI-gated event path for world mouse interaction; do not project every `_input` event unconditionally.
- Treat keyboard/gamepad focus and mouse hover/click as separate Godot 4.6 paths; test both when a story includes both input methods.
- If UI overlay captures or consumes a mouse event, world projection should not run, or should return a structured no-world-candidate failure through the input routing layer.
- `MapProjection` does not own UI business logic, click priority, movement fallback, or target selection priority.
- `TileMapLayer.local_to_map()` or visual helper APIs may be used only for editor/debug validation unless a future ADR explicitly proves and approves them as part of the projection implementation without violating ADR-0001.
- Ambiguous 2D/2.5D projection must return `invalid_coordinate` or an approved unresolved reason, not an arbitrary first candidate.

## Alternatives Considered

### Alternative 1: Controller-local coordinate conversion

- **Description**: Player controller, movement controller, pickup logic, and target selection each convert screen/world positions locally.
- **Pros**: Fast to prototype; each system can implement only what it needs.
- **Cons**: Conversion semantics diverge; hard to test consistently; UI and camera edge cases get duplicated.
- **Estimated Effort**: Low initial effort, high long-term integration cost.
- **Rejection Reason**: The GDD requires a shared coordinate contract across movement, pickup, combat, targeting, and debug systems.

### Alternative 2: TileMapLayer-authoritative lookup

- **Description**: Use visual `TileMapLayer` APIs such as local-to-map conversion as the primary gameplay coordinate authority.
- **Pros**: Convenient in Godot scenes; visually intuitive for tile-based maps.
- **Cons**: Violates ADR-0001 visual-only boundary; couples gameplay coordinates to visual tile layout; weak for non-tile visual layers or imported/source maps.
- **Estimated Effort**: Low initial effort with high architectural conflict.
- **Rejection Reason**: `TileMapLayer` may assist presentation/validation, but cannot override `MapDefinition` gameplay authority.

### Alternative 3: Physics/raycast-only projection

- **Description**: Use physics queries or Area2D hit detection as the main way to turn clicks into map cells.
- **Pros**: Useful for object hit testing and some target selection flows.
- **Cons**: Physics collision helpers become coordinate authority; misses purely logical cells; harder to prove logical-grid consistency.
- **Estimated Effort**: Medium effort with limited map contract coverage.
- **Rejection Reason**: Physics/object hit testing may support target selection, but logical cell projection must remain a map-space contract.

## Consequences

### Positive

- Movement, pickup, targeting, and debug tools share one conversion path.
- UI-consumed input is prevented from becoming accidental world actions.
- Camera/viewport transforms are testable in isolation.
- Invalid or ambiguous input fails explicitly instead of creating hidden movement bugs.

### Negative

- Requires an input routing boundary before projection.
- Requires test fixtures for camera, viewport, and synthetic map projection.
- Phase 1 projection remains provisional until OpenMir2 coordinate evidence is available.

### Neutral

- This ADR does not define final click priority or target selection UX.
- Gamepad support remains partial and may require later focus/navigation UX work.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| UI clicks leak into world actions. | Medium | High | Require UI/input routing gate before projection and test Godot 4.6 dual-focus paths. |
| Controllers reimplement private conversion logic. | Medium | High | Register forbidden pattern and centralize conversion through `MapProjection`. |
| Visual TileMapLayer drift affects gameplay clicks. | Medium | High | Validate projection against `MapDefinition`; keep TileMapLayer visual-only. |
| Ambiguous 2.5D projection guesses wrong cell. | Medium | Medium | Return `invalid_coordinate` unless a later ADR/GDD approves disambiguation. |
| Camera/viewport scaling bugs cause off-by-one cells. | Medium | Medium | Unit/integration tests for zoom, stretch, subviewport, and edge coordinates. |

## Performance Implications

| Metric | Before | Expected After | Budget |
|--------|--------|---------------|--------|
| CPU | No implementation | Per-input conversion cost; no full-map scan during normal projection | Must respect 60 fps / 16.6 ms and GDD query guardrails |
| Memory | No implementation | Minimal DTO/snapshot allocation for conversion results | Phase 1 client under 1 GB RAM |
| Load Time | N/A | Projection fixture/config load if required | N/A |
| Network | N/A | N/A | N/A |

## Migration Plan

No existing implementation requires migration.

1. Define `MapProjection` interface and Phase 1 synthetic projection fixture.
2. Define input routing gate for world interactions.
3. Add tests for screen/viewport/world/logical conversion.
4. Add invalid/ambiguous/UI-consumed input tests.
5. Integrate movement/pickup/target selection consumers through `MapProjection` instead of private conversion logic.
6. Update projection contract when OpenMir2 coordinate evidence is available.

**Rollback plan**: If the Phase 1 synthetic projection is superseded by OpenMir2 evidence, keep the `MapProjection` interface and replace the transform implementation. If the architecture boundary fails, write a superseding ADR rather than allowing controller-local conversion.

## Validation Criteria

- [ ] Valid screen/viewport/world points convert to deterministic `Vector2i` logical cells under the Phase 1 projection fixture.
- [ ] Unresolved or ambiguous inputs return `invalid_coordinate` and do not fallback to arbitrary cells.
- [ ] UI-consumed mouse events do not create world projection actions.
- [ ] Mouse world interaction and keyboard/gamepad focus paths are tested separately when both are in scope.
- [ ] Camera zoom, viewport stretch, and edge coordinates are covered by tests.
- [ ] TileMapLayer visual data does not override projection output or logical map facts.
- [ ] Movement, pickup, and target selection consumers use `MapProjection` rather than private conversion code.

## GDD Requirements Addressed

| GDD Document | System | Requirement | How This ADR Satisfies It |
|-------------|--------|-------------|--------------------------|
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Coordinate Space Separation Rule | Defines the boundary between screen/input, world/render, and logical grid coordinates. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Map Coordinate Contract Rule | Provides the conversion service required for screen/input and logical-grid candidates. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Out-of-Bounds Fail-Closed Rule | Requires invalid/unresolved conversion to return structured failure rather than guessing. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Logical Position vs. Visual Offset Rule | Prevents transient sprite position or visual tile state from becoming gameplay coordinate authority. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | UI Requirements / Debug Requirements | Supports debug evidence for coordinate conversion failures and logical cell inspection. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Implementation Boundary and ADR Prerequisites Rule | Resolves the input projection / coordinate conversion prerequisite. |

## Related

- `docs/architecture/adr-0001-map-data-representation.md`
- `docs/architecture/adr-0002-typed-query-result-schema.md`
- `design/gdd/map-coordinate-blocking-y-sort-system.md`
- Future click movement, target selection, pickup, UI/input routing, and OpenMir2 coordinate evidence work
