# ADR-0022: Click Movement Runtime Intent, Path Query, Input Gate, and Command Result Contract

## Status

Accepted

## Date

2026-06-05

## Last Verified

2026-06-05

## Decision Makers

hkm + Claude Code Game Studios

## Summary

This ADR defines the runtime contracts required before implementing the approved Click Movement GDD: input gate results, movement intent DTOs, orthogonal path query results, observable runtime state/outcome DTOs, movement command result handling, and testable dependency boundaries. We choose typed GDScript `RefCounted` DTOs and enums that compose with ADR-0002/0003/0005/0019 instead of local strings, singleton-only controller logic, frame polling, or Godot helper authority.

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Input / Core / Navigation / Scripting |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff and input dual-focus behavior must be verified. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/modules/input.md`; ADR-0002; ADR-0003; ADR-0005; ADR-0019; `design/gdd/click-movement-system.md` |
| **Post-Cutoff APIs Used** | None as gameplay authority. Godot 4.6 dual-focus behavior is acknowledged as an input routing verification requirement. |
| **Verification Required** | Verify Godot 4.6 UI consumption and dual-focus cases; verify no `_input`/polling path can bypass UI ownership; verify headless movement core tests run without SceneTree/Autoload; verify path planner deterministic output under fixed fixtures. |

> **Note**: If Knowledge Risk is MEDIUM or HIGH, this ADR must be re-validated if the project upgrades or downgrades engine versions. If Godot input behavior changes, supersede this ADR rather than weakening the typed input gate contract.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | ADR-0001 Map Data Representation; ADR-0002 Typed Query Result Schema; ADR-0003 Authoritative Occupancy / Reservation Update Ordering; ADR-0005 Input Projection / Coordinate Conversion; ADR-0019 Map Distance Facts and Movement Legality Boundary |
| **Enables** | Click Movement implementation stories; movement input integration; orthogonal path planner tests; movement presentation handoff; future Combat/Pickup movement-to-range requests. |
| **Blocks** | Any story implementing click movement runtime intent, path query, replacement behavior, input/UI gate integration, movement command result handling, or movement-speed config consumption. |
| **Ordering Note** | This ADR is after Click Movement GDD approval and before implementation. It does not supersede ADR-0002/0003/0005/0019; it binds their contracts into a click movement runtime architecture. |

## Context

### Problem Statement

The Click Movement GDD was approved for ADR/architecture with several implementation blockers: path query DTO/schema, input/UI gate DTO/schema, actor unavailable reasons, movement speed source, same-event ownership, request ordering, deterministic path tie-breaks, command-result correction flow, and testable dependency boundaries. If implementation proceeds without this ADR, GDScript code may invent local strings, ad hoc dictionaries, `Vector2i.ZERO` sentinels, frame-polling input shortcuts, singleton-only service access, or nondeterministic path search results.

### Current State

The project has accepted architecture for typed spatial query results, authoritative map-space command mutation, input projection, and movement legality. No click movement runtime DTOs, input gate result schema, path query result schema, or command-result consumption contract exist yet.

### Constraints

- Click movement must use named input actions and the same input event whose UI ownership was evaluated.
- UI/input ownership must fail closed when unresolved, especially under Godot 4.6 mouse/touch versus keyboard/gamepad dual-focus.
- Projection success is coordinate-candidate success only; it is not walkability.
- Missing logical cells must not be represented by `Vector2i.ZERO`, previous valid cells, or nullable untyped magic.
- Pathfinding is orthogonal-only under the MVP policy and cannot use Godot Navigation/Physics/TileMap/Y-sort as authority.
- MapSpaceState owns movement reservation/commit/cancel mutation and authoritative command ordering.
- Headless movement logic must be unit-testable through injected dependencies or approved adapters, not direct singleton-only access.

### Requirements

- Define typed input gate, movement intent, path query, and movement outcome contracts.
- Provide deterministic path planner tie-break behavior or explicit test policy.
- Distinguish local correlation IDs from MapSpaceState authoritative ordering.
- Define command failure/correction minimum behavior.
- Define movement speed config source for MVP without hardcoding.
- Preserve all Click Movement GDD acceptance criteria AC-47 through AC-52.

## Decision

Create a click movement runtime contract layer with typed DTOs and enum families for:

1. `MovementInputGateResult` — same-event action match, UI ownership, input source, and world-point availability.
2. `ClickMovementIntent` — a player movement request built only after input gate and projection acceptance.
3. `ClickPathQueryResult` — orthogonal path query result, budget facts, typed reasons, unresolved policy facts, and deterministic metadata.
4. `ClickMovementRuntimeState` and `ClickMovementTransitionOutcome` — observable behavior state reporting for tests and presentation.
5. Command-result consumption rules — how click movement reacts to `RESERVE_MOVEMENT`, `COMMIT_MOVEMENT`, and `CANCEL_MOVEMENT` outcomes.
6. `ClickMovementTuning` — data-driven movement speed and path budget config for MVP.

The contracts use typed enums internally. They may expose `StringName` conversion helpers only for logs, localization, debug overlays, editor tooling, or serialization. Gameplay branching must not compare raw local strings.

### Architecture

```text
InputEvent
  |
  v
Named action + UI/input ownership gate
  -> MovementInputGateResult
       | allowed same-event world point only
       v
MapProjection (ADR-0005)
  -> SpatialQueryResult with has_logical_cell + logical_cell
       | projection accepted only
       v
ClickMovementIntent
       |
       v
ClickPathQueryService
  -> ClickPathQueryResult (orthogonal-only path or typed failure)
       |
       v
MovementLegalityService per step (ADR-0019)
       |
       v
MapSpaceState commands (ADR-0003)
  RESERVE_MOVEMENT / COMMIT_MOVEMENT / CANCEL_MOVEMENT
       |
       v
Click movement state/outcome facts for presentation, UI, audio, QA, downstream systems
```

### Key Interfaces

Exact file paths, constructors, and node/service organization are implementation details. Public logic must remain statically typed, injectable/testable, and scene-tree-independent where possible. Shared enum families should live in an approved typed namespace script such as `ClickMovementTypes` or equivalent class-name holder; DTOs may reference them as `ClickMovementTypes.MovementInputSource`-style values. Each `class_name` DTO remains one class per file under project GDScript standards unless a later ADR approves a different organization.

```gdscript
enum MovementInputSource {
    MOUSE,
    KEYBOARD,
    GAMEPAD,
    SCRIPTED,
    UNKNOWN,
}

enum MovementInputOwnershipStatus {
    WORLD_ELIGIBLE,
    UI_CONSUMED,
    BLOCKED_BY_MODAL,
    UNKNOWN_OWNERSHIP,
    NO_WORLD_TARGET,
}

enum MovementInputGateReason {
    WORLD_INPUT_ALLOWED,
    ACTION_NOT_MATCHED,
    UI_CONSUMED_INPUT,
    BLOCKED_BY_MODAL_UI,
    UNKNOWN_INPUT_OWNERSHIP,
    WORLD_POINT_UNAVAILABLE,
}

class_name MovementInputGateResult
extends RefCounted

var action_matched: bool
var input_source: MovementInputSource
var ownership_status: MovementInputOwnershipStatus
var has_screen_position: bool
var screen_position: Vector2
var has_viewport_position: bool
var viewport_position: Vector2
var has_world_position: bool
var world_position: Vector2
var primary_reason: MovementInputGateReason
var secondary_reasons: Array[MovementInputGateReason]
```

`MovementInputGateResult` is produced from the same `InputEvent` whose named action and UI ownership were evaluated. Frame-level polling of action state alone cannot create click movement intents. Godot implementation should use `_unhandled_input(event)` or an equivalent UI-gated input router. If `_input(event)` is used, it must explicitly consult the approved input gate before projection and must not bypass Control-owned events.

The input gate owns event eligibility, not logical coordinate conversion. `screen_position` and `viewport_position` are the normal event-position facts passed onward to `MapProjection`. `world_position` may be populated only by an approved input router/projection adapter for cases that already have a world/render point; it still must pass through `MapProjection.project_world_to_logical()` before becoming a logical cell. `MovementInputGateResult` must never output a logical cell and must never decide walkability.

Input gate reasons are not spatial query reasons. They are mapped outward as follows: `WORLD_INPUT_ALLOWED` permits projection; `ACTION_NOT_MATCHED`, `UI_CONSUMED_INPUT`, `BLOCKED_BY_MODAL_UI`, `UNKNOWN_INPUT_OWNERSHIP`, and `WORLD_POINT_UNAVAILABLE` stop before projection and may be converted to UI/debug localization keys by presentation code. They must not be forced into ADR-0002 `SpatialQueryReason` unless a later ADR explicitly extends the spatial reason family for input ownership.

```gdscript
enum ClickMovementOrigin {
    PLAYER_CLICK,
    COMBAT_REQUEST,
    PICKUP_REQUEST,
    SCRIPTED_REQUEST,
}

enum ActorMovementAvailabilityReason {
    AVAILABLE,
    ACTOR_UNKNOWN,
    MISSING_AUTHORITATIVE_CELL,
    DEAD,
    DESPAWNED,
    IMMOBILIZED,
    CONTROL_LOCKED,
    MAP_TRANSITIONING,
    ACTIVE_MAP_UNAVAILABLE,
}

class_name ClickMovementIntent
extends RefCounted

var local_request_id: int
var origin: ClickMovementOrigin
var actor_id: int
var map_id: StringName
var source_cell: Vector2i
var target_cell: Vector2i
var actor_availability_reason: ActorMovementAvailabilityReason
```

`local_request_id` is for correlation only. Authoritative ordering, reservation conflict winners, and command resolution order are assigned by `MapSpaceState` under ADR-0003. Click movement may not decide same-tick conflict winners.

`ActorMovementAvailabilityReason` is a movement-intent-specific reason family. It does not replace `SpatialQueryReason`. Spatial legality failures remain ADR-0002/ADR-0019 reasons; actor availability failures remain in click movement intent/transition outcomes and map to player-facing localization keys through presentation. If a future command/query result requires actor-availability reasons as spatial reasons, ADR-0002 must be revised rather than using local strings.

```gdscript
enum ClickPathQueryReason {
    PATH_WALKABLE,
    NO_ORTHOGONAL_PATH,
    PATH_SEARCH_NODE_LIMIT_EXCEEDED,
    PATH_STEP_LIMIT_EXCEEDED,
    INVALID_PATH_REQUEST,
    UNRESOLVED_MOVEMENT_POLICY,
}

enum ClickPathQueryStatus {
    PATH_FOUND,
    NO_PATH,
    PATH_LIMIT_EXCEEDED,
    INVALID_REQUEST,
    UNRESOLVED_POLICY,
}

enum ClickPathUnresolvedPolicy {
    NONE,
    DIAGONAL_STEP_REQUIRED,
    CORNER_CUTTING_REQUIRED,
    CROSS_MAP_UNDEFINED,
    MOVEMENT_POLICY_MISSING,
}

class_name ClickPathQueryResult
extends RefCounted

var status: ClickPathQueryStatus
var primary_reason: ClickPathQueryReason
var secondary_reasons: Array[ClickPathQueryReason]
var source_cell: Vector2i
var destination_cell: Vector2i
var path_cells: Array[Vector2i]
var visited_node_count: int
var path_length: int
var max_path_search_nodes: int
var max_click_move_path_steps: int
var unresolved_policy: ClickPathUnresolvedPolicy
var first_blocking_cell_has_value: bool
var first_blocking_cell: Vector2i
```

`path_cells` contains ordered logical cells after `source_cell`, including the destination when successful. It must contain orthogonal-only steps. Same-cell requests are consumed before `ClickPathQueryService` and produce `SpatialQueryReason.NO_MOVEMENT_REQUESTED`; they do not call path query. `ClickPathQueryStatus.PATH_FOUND` requires `path_length > 0`. `NO_PATH` means no legal orthogonal-only route was found within the approved policy. `PATH_LIMIT_EXCEEDED` means budget stopped the query. `UNRESOLVED_POLICY` is reserved for explicitly detected unapproved policy requirements such as cross-map movement, missing movement profile, a requested executable diagonal step, or a directly detected corner-cutting dependency. Ordinary orthogonal search failure must return `NO_PATH`, not infer diagonal/corner-cutting need.

Allowed path status/reason pairs:

| `ClickPathQueryStatus` | Required / Allowed `ClickPathQueryReason` | Spatial mapping for downstream feedback |
|---|---|---|
| `PATH_FOUND` | `PATH_WALKABLE` | May map to `SpatialQueryReason.WALKABLE` for presentation/debug summaries. |
| `NO_PATH` | `NO_ORTHOGONAL_PATH` | Path-specific reason; do not collapse to local string. Presentation may display “Cannot reach target.” |
| `PATH_LIMIT_EXCEEDED` | `PATH_SEARCH_NODE_LIMIT_EXCEEDED` or `PATH_STEP_LIMIT_EXCEEDED` | Path-specific reason; indicates data/config budget failure, not static blocking. |
| `INVALID_REQUEST` | `INVALID_PATH_REQUEST` | Path-specific reason for missing actor/source/destination policy inputs. |
| `UNRESOLVED_POLICY` | `UNRESOLVED_MOVEMENT_POLICY` | Must carry `ClickPathUnresolvedPolicy`; maps to ADR-0002 `MOVEMENT_RULE_UNRESOLVED`, `DIAGONAL_MOVEMENT_UNRESOLVED`, `CORNER_CUTTING_UNRESOLVED`, or `CROSS_MAP_MOVEMENT_UNDEFINED` where applicable. |

Path query reasons are click-path-specific typed reasons. They do not replace ADR-0002 spatial reasons; when a path failure originates from a `SpatialQueryResult`, the original `SpatialQueryReason` should be preserved in separate spatial debug/secondary evidence or in the movement transition outcome.

```gdscript
class_name ClickMovementTuning
extends RefCounted

var movement_speed_cells_per_second: float
var max_click_move_path_steps: int
var max_path_search_nodes: int
var invalid_click_cancels_current_path: bool
var auto_fallback_to_nearest_walkable: bool
var auto_reroute_on_dynamic_block: bool
```

MVP tuning defaults are loaded from data/config fixtures or an approved movement profile during bootstrap/config validation, not hardcoded in controller logic. Missing config may use the approved MVP default fixture in tests/prototypes; player-build runtime must have a validated movement tuning profile before movement enables.

- `movement_speed_cells_per_second = 4.0`
- `max_click_move_path_steps = 48`
- `max_path_search_nodes = 512`
- `invalid_click_cancels_current_path = false`
- `auto_fallback_to_nearest_walkable = false`
- `auto_reroute_on_dynamic_block = false`

Invalid tuning such as missing player-build profile, non-finite speed, zero/negative speed, `max_click_move_path_steps < 1`, `max_path_search_nodes < max_click_move_path_steps`, or values above the approved performance/balance profile fails validation and blocks movement configuration rather than silently clamping in gameplay.


```gdscript
enum ClickMovementRuntimeState {
    IDLE,
    PLANNING,
    RESERVING_STEP,
    MOVING_STEP,
    COMMITTING_STEP,
    ARRIVED,
    BLOCKED,
    UNRESOLVED,
    CANCELLING,
    CORRECTION_BLOCKED,
    DISABLED,
}

class_name ClickMovementTransitionOutcome
extends RefCounted

var previous_state: ClickMovementRuntimeState
var next_state: ClickMovementRuntimeState
var local_request_id: int
var actor_id: int
var map_id: StringName
var current_committed_cell: Vector2i
var has_reserved_target_cell: bool
var reserved_target_cell: Vector2i
var has_pending_destination_cell: bool
var pending_destination_cell: Vector2i
var spatial_primary_reason: SpatialQueryReason
var spatial_secondary_reasons: Array[SpatialQueryReason]
var path_primary_reason: ClickPathQueryReason
var input_primary_reason: MovementInputGateReason
```

`ClickMovementTransitionOutcome` is the observable test/presentation outcome contract. It does not make presentation authoritative. Optional cell fields use `has_*` booleans and must not use sentinel coordinates. `CORRECTION_BLOCKED` means local movement state is untrusted after command failure; no new reservation, arrival publication, or queued replacement consumption may occur until the authoritative reservation/committed-cell state is known.

```gdscript
enum MapSpaceCommandResultStatus {
    ACCEPTED,
    REJECTED,
    STALE_OR_MISMATCHED,
    AUTHORITY_UNAVAILABLE,
}
```

### Command Result Consumption

`MapSpaceState` remains the command authority. Click movement consumes command results and snapshots; it does not own mutation. The implementation may refine command-result DTO names, but each command outcome consumed by click movement must expose:

- command kind;
- command id / authoritative `created_sequence`;
- local request id, if supplied;
- actor id;
- source and target cells;
- `MapSpaceCommandResultStatus` accepted/rejected/stale/unavailable status;
- primary and secondary typed reasons;
- whether the actor has an active reservation after the result;
- last committed logical cell after the authoritative update.

Required behavior:

- `RESERVE_MOVEMENT` accepted → enter `MovingStep` and allow presentation to move toward the reserved target.
- `RESERVE_MOVEMENT` rejected → do not move; keep last committed cell; emit typed failure.
- `COMMIT_MOVEMENT` accepted → update local committed cell from authoritative result and continue path/arrival logic.
- `COMMIT_MOVEMENT` rejected → treat local step state as untrusted; correct presentation to last committed cell; suppress arrival. If an active reservation remains, enter `CORRECTION_BLOCKED`; do not start another reservation, publish arrival, or consume queued replacement. Implementation may submit `CANCEL_MOVEMENT` or wait for authoritative cleanup, but leaving `CORRECTION_BLOCKED` requires a post-update authoritative result/snapshot proving the reservation is cleared or correctly owned.
- `CANCEL_MOVEMENT` accepted → clear local active reservation/path according to whether a queued replacement exists.
- `CANCEL_MOVEMENT` rejected → do not assume reservation was released; query authoritative state and enter `CORRECTION_BLOCKED` until authoritative state proves the reservation status.

### Deterministic Path Planning Rules

The MVP path query uses orthogonal-only logical neighbors and read-only facts. If exact path assertions are required, neighbor expansion order must be documented and fixed. The default deterministic cardinal order is:

1. East: `Vector2i(1, 0)`
2. South: `Vector2i(0, 1)`
3. West: `Vector2i(-1, 0)`
4. North: `Vector2i(0, -1)`

If a future implementation changes this order, it must update tests and this ADR or state that tests assert path validity rather than exact path sequence. The planner may not use wall-clock time, random ordering, scene-tree order, node order, dictionary iteration order, Godot Navigation output, Physics callbacks, TileMapLayer visual data, or Y-sort order as tie-break authority.

### Implementation Guidelines

- Keep headless click movement core independent of SceneTree, Control nodes, Node2D transform state, physics callbacks, and Autoload-only globals.
- Inject or provide approved adapters for projection, movement legality, map state, path query, actor/status facts, and tuning.
- Use `has_*` booleans plus typed values for optional fields; do not use sentinel coordinates such as `(0, 0)`.
- A planned path is advisory. Every executable step must be revalidated before reservation and then confirmed by MapSpaceState command results.
- During `MovingStep`, valid replacement clicks become pending destinations; path validation is deferred until the current step commits and replans from the newly committed cell.
- UI/player messages map typed reasons to localization keys outside core movement logic.
- Presentation may use markers, cursor changes, audio, interpolation, and Y-sort updates, but those never decide movement authority.

## Alternatives Considered

### Alternative 1: Implement click movement directly in player controller

- **Description**: Handle input, projection, pathfinding, movement state, and presentation inside a player node/controller.
- **Pros**: Fastest prototype; fewer DTOs.
- **Cons**: Encourages `_input` shortcuts, Node2D position authority, singleton coupling, and hard-to-test scene dependencies.
- **Estimated Effort**: Low initial effort, high correction cost.
- **Rejection Reason**: Violates the GDD's testability, authority-boundary, and UI/input gate requirements.

### Alternative 2: Use dictionaries and local strings for path/input results

- **Description**: Return flexible `Dictionary` objects with string status and reason names.
- **Pros**: Quick to author and serialize.
- **Cons**: Typos and local reason drift; weak GDScript static typing; harder to satisfy ADR-0002.
- **Estimated Effort**: Low upfront, high review/test cost.
- **Rejection Reason**: The project requires typed query schemas and no local reason strings for gameplay authority.

### Alternative 3: Use Godot NavigationAgent2D as path authority

- **Description**: Let NavigationAgent2D generate and authorize movement paths.
- **Pros**: Engine-supported path helper.
- **Cons**: Violates logical-grid authority, ADR-0019, and SceneTree-independent testability; diagonal/corner/physics semantics may not match MVP policy.
- **Estimated Effort**: Medium.
- **Rejection Reason**: Navigation may be a future non-authoritative helper only; every executed step must be logical-cell validated and MapSpaceState-commanded.

### Alternative 4: Treat movement speed as hardcoded controller constant

- **Description**: Put a speed value directly in movement controller code.
- **Pros**: Simple.
- **Cons**: Violates data-driven gameplay values and makes tuning/story tests brittle.
- **Estimated Effort**: Low.
- **Rejection Reason**: The GDD requires external config or approved movement profile/attribute feed.

## Consequences

### Positive

- Implementation stories can proceed with typed DTO expectations and fewer local inventions.
- Input, projection, pathing, command results, and presentation are separated for tests.
- Godot 4.6 dual-focus risks are isolated in a gate contract.
- Path planner tests can be deterministic.
- Click movement remains aligned with existing accepted ADR authority boundaries.

### Negative

- More DTO and enum boilerplate before the first playable movement prototype.
- Some fields may later be folded into broader input/path/command schemas if more systems need them.
- Proposed status means implementation remains blocked until review/acceptance.

### Neutral

- This ADR defines click movement contracts, not final player-facing cursor art, audio assets, or full UI widget layout.
- Combat and pickup still own target interpretation and post-arrival business validation.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| DTO proliferation creates complexity. | Medium | Medium | Keep DTOs scoped; merge into broader schemas only after repeated cross-system use is proven. |
| Input gate still leaks UI events. | Medium | High | Require same-event ownership tests and `_unhandled_input`/router evidence. |
| Planner tie-break creates flaky tests. | Medium | Medium | Fix default cardinal order or assert path validity instead of exact route. |
| Implementers use sentinel coordinates for absent data. | Medium | High | Require `has_*` fields and tests for `(0, 0)` not being treated as absent. |
| Movement speed hardcoded during prototyping. | Medium | Medium | Config validation and code review against data-driven requirement. |
| Command failure correction under-specified in implementation. | Medium | High | Require command result DTO fields and correction tests for reservation/commit/cancel failure. |

## Performance Implications

| Metric | Before | Expected After | Budget |
|--------|--------|---------------|--------|
| CPU | No click movement runtime | Path query bounded by `max_path_search_nodes`; per-step legality/command O(1) excluding queue sort | 60 fps / 16.6 ms; Phase 1 path query must be profiled |
| Memory | No click movement runtime | DTO allocations for input/path/state facts; path arrays bounded by `max_click_move_path_steps` | Phase 1 client under 1 GB RAM |
| Load Time | N/A | Tuning/config fixture load | Minimal; covered by data validation |
| Network | N/A | None for offline Phase 1 | N/A |

## Migration Plan

No existing runtime implementation requires migration.

1. Review and accept this ADR.
2. Add/adjust typed enum families and DTO files for input gate, movement intent, path query, tuning, and command result adapters.
3. Add headless tests for input gate result consumption, projection-accepted intent creation, path query results, replacement policy, and command-result correction.
4. Add integration tests or walkthrough evidence for Godot 4.6 UI dual-focus and event routing.
5. Implement click movement core using injected/adapted dependencies.
6. Implement presentation integration after headless logic passes.

**Rollback plan**: If DTO scope proves too narrow or too broad, supersede this ADR with a broader Input/Path/Command Result schema ADR. Do not rollback to local strings, dictionary authority, or controller-local movement rules.

## Validation Criteria

- [ ] Input-gate tests prove UI-consumed and unknown ownership events never call `MapProjection` or alter movement.
- [ ] Same-event tests prove frame polling alone cannot create movement intents.
- [ ] Projection success uses `has_logical_cell` and never treats `(0, 0)` or previous cell as absent fallback.
- [ ] Path query tests cover `PATH_FOUND`, `NO_PATH`, `PATH_LIMIT_EXCEEDED`, `UNRESOLVED_POLICY`, deterministic neighbor order, and budget facts.
- [ ] Command result tests cover reserve rejection, commit failure with active reservation, cancel failure, and last-committed-cell correction.
- [ ] Tuning validation fails missing/non-finite/zero/negative movement speed and out-of-policy path budgets.
- [ ] Headless click movement core tests run without SceneTree, Control nodes, physics, navigation, TileMapLayer authority, or direct singleton-only dependencies.
- [ ] Story Done for logic/integration criteria is blocked until the approved Godot GDScript test runner executes real tests; guard-only or placeholder test infrastructure is not passing evidence.
- [ ] Presentation tests or walkthrough prove marker/cursor/audio feedback does not become movement authority.

## GDD Requirements Addressed

| GDD Document | System | Requirement | How This ADR Satisfies It |
|-------------|--------|-------------|--------------------------|
| `design/gdd/click-movement-system.md` | 点击移动系统 | AC-47 — path-found/no-path/path-limit results must be typed before implementation Done. | Defines `ClickPathQueryResult`, `ClickPathQueryStatus`, budget fields, unresolved policy fields, and deterministic path metadata. |
| `design/gdd/click-movement-system.md` | 点击移动系统 | AC-48 — input ownership/gate results must be approved for Godot 4.6 dual-focus testing. | Defines `MovementInputGateResult`, ownership statuses, same-event rule, and `_unhandled_input`/router boundary. |
| `design/gdd/click-movement-system.md` | 点击移动系统 | AC-49 — actor disabled/dead/map-transition/control-lock failures must map to approved typed reasons or DTO fields. | Defines `ActorMovementAvailabilityReason` for movement denial facts consumed by click movement. |
| `design/gdd/click-movement-system.md` | 点击移动系统 | AC-50 — real Godot GDScript tests are required before story Done. | Requires validation criteria for headless and Godot integration tests; does not accept placeholder-only test evidence. |
| `design/gdd/click-movement-system.md` | 点击移动系统 | AC-51 — click movement logic must use testable dependencies/adapters, not direct singleton-only logic. | Requires injectable/adapted dependencies for projection, legality, map state, planner, tuning and actor/status facts. |
| `design/gdd/click-movement-system.md` | 点击移动系统 | AC-52 — event routing evidence must prevent `_input`/polling bypass of UI consumption. | Requires same-event input ownership and approved event router evidence. |
| `design/gdd/click-movement-system.md` | 点击移动系统 | Repeated click and current-step behavior must be deterministic and testable. | Defines pending destination behavior during `MovingStep` and post-commit replan from the newly committed cell. |
| `design/gdd/click-movement-system.md` | 点击移动系统 | Movement speed and path budgets must be data-driven. | Defines `ClickMovementTuning` and validation failure behavior for invalid config. |

## Related

- ADR-0001: Map Data Representation
- ADR-0002: Typed Query Result Schema
- ADR-0003: Authoritative Occupancy / Reservation Update Ordering
- ADR-0005: Input Projection / Coordinate Conversion
- ADR-0019: Map Distance Facts and Movement Legality Contract (`docs/architecture/adr-0019-map-distance-and-movement-legality-contract.md`)
- `design/gdd/click-movement-system.md`
