# ADR-0003: Authoritative Occupancy / Reservation Update Ordering

## Status

Accepted

## Date

2026-06-04

## Last Verified

2026-06-04

## Decision Makers

hkm + Claude Code Game Studios

## Summary

This ADR defines how runtime actor occupancy, item occupancy, and movement reservations are mutated deterministically. We choose a `MapSpaceState` command queue with authoritative sequence assignment and atomic update phases, rejecting immediate node/controller mutation and arbitrary signal callback order.

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Core / Scripting |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff and must be verified against engine reference docs. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md` |
| **Post-Cutoff APIs Used** | None required. This ADR defines project update ordering rather than a new Godot API dependency. |
| **Verification Required** | Verify deterministic command ordering, signal isolation, atomic mutation visibility, and compatibility with chosen Godot process hook before implementation stories complete. |

> **Note**: If Knowledge Risk is MEDIUM or HIGH, this ADR must be re-validated if the
> project upgrades engine versions. Flag it as "Superseded" and write a new ADR.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | ADR-0001 Map Data Representation; ADR-0002 Typed Query Result Schema |
| **Enables** | Movement reservation stories, spawn placement stories, drop placement stories, pickup candidate stories, and deterministic integration tests. |
| **Blocks** | Any implementation story that mutates actor occupancy, item occupancy, or reservations. |
| **Ordering Note** | This ADR does not choose pathfinding, movement speed, animation timing, diagonal movement, pickup distance, combat distance, or OpenMir2-authentic movement semantics. |

## Context

### Problem Statement

The GDD requires occupancy and reservation mutations to be serialized through a deterministic authoritative update pipeline. If actor nodes, AI, movement controllers, drop systems, pickup systems, or signal callbacks mutate spatial state immediately, the result can depend on scene-tree order, callback order, frame timing, or wall-clock time. That would break deterministic tests and cause inconsistent blocking, reservation, and pickup behavior.

### Current State

ADR-0001 assigns runtime map-space state to `MapSpaceState` or an equivalent runtime service. ADR-0002 defines typed query result semantics. No authoritative mutation ordering exists yet.

### Constraints

- `MapSpaceState` is the only owner allowed to mutate actor occupancy, item occupancy, and reservations.
- Static facts come from `MapDefinition`; this ADR does not modify static map storage.
- Update order must be deterministic and unit/integration-testable.
- Normal gameplay must not depend on arbitrary signal callback order, scene-tree insertion order, node creation order, or wall-clock time.
- Source-authentic OpenMir2 timing remains evidence-gated; this ADR defines Phase 1 deterministic mutation architecture only.

### Requirements

- Collect movement, spawn, drop, pickup, cleanup, and removal intents without immediate mutation by caller systems.
- Assign deterministic `created_sequence` values from the authoritative queue/service.
- Resolve competing commands in deterministic order.
- Apply mutations atomically within one authoritative update phase.
- Publish snapshots/debug evidence only after the authoritative phase has reached a consistent state.

## Decision

Use a command queue owned by `MapSpaceState` or an equivalent map-space runtime service. Downstream systems submit typed intents/commands. They do not directly mutate occupancy maps, reservation maps, or item occupancy lists. The queue assigns monotonic `created_sequence` values and resolves commands in a deterministic order: `created_sequence`, then stable actor/item/request IDs as explicit tie-breakers where needed.

An authoritative update phase performs cleanup, command resolution, and mutation commit. During this phase, the system must not publish half-committed gameplay state to consumers. Signals may be emitted after the consistent state is reached for presentation, debug, or follow-up notifications, but signal callbacks must not recursively mutate authoritative occupancy/reservation state outside the queue.

The exact Godot hook is not finalized by this ADR. A fixed authoritative update phase may be driven by `_physics_process`, a deterministic game tick service, or another documented update loop in a future implementation plan. Regardless of hook, no system may mutate map-space state in its own `_process`, `_physics_process`, `_input`, animation callback, or signal handler outside the `MapSpaceState` command path.

### Architecture

```text
Movement / AI / spawn / drop / pickup / cleanup systems
        |
        v
MapSpaceCommand submission API
        |
        v
MapSpaceState authoritative queue
        |
        +--> assign created_sequence
        +--> cleanup invalid actors/items/reservations/map unload state
        +--> sort commands by deterministic order
        +--> resolve conflicts
        +--> atomically apply occupancy/item/reservation mutations
        +--> publish query snapshots / dirty Y-sort notifications / debug counters
```

### Key Interfaces

```gdscript
class_name MapSpaceCommand
extends RefCounted

enum CommandKind {
    RESERVE_MOVEMENT,
    COMMIT_MOVEMENT,
    CANCEL_MOVEMENT,
    PLACE_ACTOR,
    REMOVE_ACTOR,
    PLACE_ITEM,
    REMOVE_ITEM,
    PICKUP_SPATIAL_VERIFY,
    CLEANUP_MAP_STATE,
}

var command_kind: CommandKind
var created_sequence: int
var actor_id: int
var item_instance_id: int
var request_id: int
var map_id: StringName
var source_cell: Vector2i
var target_cell: Vector2i

class_name MapSpaceState
extends RefCounted

func submit_command(command: MapSpaceCommand) -> int:
    # Assigns authoritative created_sequence and queues the command.

func run_authoritative_update() -> void:
    # Cleans stale state, resolves queued commands, applies mutations atomically, then publishes post-update evidence.

func get_cell_query_result(cell: Vector2i, context: SpatialQueryContext) -> SpatialQueryResult:
    # Reads the latest consistent state and returns typed query results.
```

Exact command subclasses and field sets may be refined during implementation, but all mutation requests must pass through this authority boundary.

### Implementation Guidelines

- `created_sequence` is assigned by `MapSpaceState`, not caller nodes.
- Do not derive ordering from `Time.get_ticks_msec()`, frame delta, node creation order, scene-tree insertion order, or signal callback order.
- Stable actor/item/request IDs must be one comparable type within an active set; mixed int/string comparisons are invalid and must surface through structured debug reasons where applicable.
- Apply command resolution before mutation commit where possible, then commit all accepted changes in a consistent phase.
- Death, despawn, item removal, map unload, stale reservation cleanup, and invalid actor cleanup must happen before resolving new movement/spawn/drop claims for affected cells.
- Signals are allowed for post-update presentation/debug notification, not as authoritative mutation paths.

## Alternatives Considered

### Alternative 1: Immediate mutation by caller systems

- **Description**: Movement, AI, spawn, drop, and pickup systems directly update occupancy and reservations when their logic runs.
- **Pros**: Simple to prototype; fewer queue classes; direct call flow is easy to read locally.
- **Cons**: Ordering depends on system update order and callbacks; conflicting moves become nondeterministic; tests cannot reliably reproduce race-like cases.
- **Estimated Effort**: Low initial effort, high integration risk.
- **Rejection Reason**: Violates the GDD requirement for deterministic authoritative update ordering.

### Alternative 2: Signal/callback order authority

- **Description**: Systems emit signals and whichever callback runs first mutates state first.
- **Pros**: Godot signals are convenient; presentation systems can react naturally.
- **Cons**: Callback ordering becomes hidden gameplay authority; recursion and re-entrant mutation are likely; difficult to debug or replay.
- **Estimated Effort**: Medium initial effort, high debugging cost.
- **Rejection Reason**: Arbitrary signal order must not decide occupancy or reservation outcomes.

### Alternative 3: Per-system local occupancy maps

- **Description**: Movement, AI, spawn, and pickup systems maintain their own caches or occupancy maps.
- **Pros**: Each system can optimize for its own queries.
- **Cons**: State divergence, stale caches, conflicting ownership, and unclear write authority.
- **Estimated Effort**: Medium effort with high synchronization cost.
- **Rejection Reason**: Conflicts with the registry stance that `runtime_map_space_state` is owned by `MapSpaceState`.

## Consequences

### Positive

- Occupancy and reservation results are reproducible in tests.
- Cross-system conflicts are resolved by one authority.
- Debug evidence can show command order and conflict resolution reasons.
- Future networking or replay work has a deterministic foundation.

### Negative

- More infrastructure than immediate mutation.
- Systems must adapt to command submission instead of direct state writes.
- Care is needed to prevent re-entrant signal callbacks from bypassing the queue.

### Neutral

- This ADR does not force a final Godot process hook.
- Visual interpolation can still run separately from authoritative logical occupancy.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| Queue grows complex and becomes a god object. | Medium | Medium | Keep this ADR scoped to map-space mutations; movement/pathfinding/pickup ownership remains downstream. |
| Commands observe half-committed state. | Low | High | Resolve and commit inside one authoritative phase; publish snapshots only after commit. |
| Caller systems bypass queue for convenience. | Medium | High | Register forbidden pattern; code review against state ownership registry. |
| Stable IDs are inconsistent or incomparable. | Medium | Medium | Require one comparable ID type per active set and test invalid ID cases. |

## Performance Implications

| Metric | Before | Expected After | Budget |
|--------|--------|---------------|--------|
| CPU | No implementation | Queue sort and conflict resolution proportional to commands per update, not full map size | Must respect 60 fps / 16.6 ms and GDD bounded query guardrails |
| Memory | No implementation | Transient command objects and compact runtime maps/lists | Phase 1 client under 1 GB RAM |
| Load Time | N/A | N/A | N/A |
| Network | N/A | Deterministic ordering supports future networking/replay, but no network traffic in MVP | N/A |

## Migration Plan

No existing implementation requires migration.

1. Define typed command kinds and submission API.
2. Implement monotonic sequence assignment in `MapSpaceState`.
3. Implement cleanup-first update phase.
4. Implement movement reservation create/commit/cancel commands.
5. Add item placement/removal command paths.
6. Add integration tests for competing commands, stale cleanup, and atomic commit.

**Rollback plan**: If the command queue is too heavy for Phase 1, write a superseding ADR that preserves single-authority mutation and deterministic ordering. Do not revert to caller-direct mutation without replacing the determinism guarantee.

## Validation Criteria

- [ ] Two actors requesting the same target cell in one update resolve deterministically.
- [ ] Movement reservation creation, commit, cancel, and cleanup all mutate state only through `MapSpaceState`.
- [ ] Source cell release, target occupancy, and reservation clearing occur atomically for successful commits.
- [ ] Death/despawn/map unload cleanup releases stale occupancy/reservations before new commands resolve.
- [ ] Signal callbacks cannot mutate authoritative occupancy/reservation state directly.
- [ ] Command order does not depend on scene-tree insertion order, node creation order, wall-clock time, or signal callback order.
- [ ] Debug evidence can report command sequence, resolved winner/loser, and structured failure reasons.

## GDD Requirements Addressed

| GDD Document | System | Requirement | How This ADR Satisfies It |
|-------------|--------|-------------|--------------------------|
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Movement Reservation Rule | Defines queued reservation create/commit/cancel mutations under `MapSpaceState`. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Reservation Ownership and Lifecycle Rule | Makes `MapSpaceState` the runtime owner and defines cleanup before new claims. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Authoritative Update Ordering Rule | Implements deterministic command ordering and atomic authoritative update phases. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Actor Occupancy Rule | Ensures actor occupancy changes through one authority. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Item Occupancy Rule | Ensures item placement/removal changes through one authority. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Performance Guardrail Rule | Resolves queued commands rather than scanning full maps or polling every render frame. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Cross-System Integration and Definition of Done | Prevents downstream systems from bypassing occupancy/reservation state directly. |

## Related

- `docs/architecture/adr-0001-map-data-representation.md`
- `docs/architecture/adr-0002-typed-query-result-schema.md`
- `design/gdd/map-coordinate-blocking-y-sort-system.md`
- Future movement, AI, spawn, drop, and pickup ADRs/GDDs
