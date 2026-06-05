# ADR-0019: Map Distance Facts and Movement Legality Boundary

## Status

Accepted

## Date

2026-06-05

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Core / Navigation / Physics |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff and movement/navigation APIs must be verified against engine reference docs. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md`; `docs/engine-reference/godot/modules/navigation.md`; `.claude/docs/technical-preferences.md` |
| **Post-Cutoff APIs Used** | None as authoritative gameplay APIs. Godot 4.5+ dedicated 2D navigation server behaviour is acknowledged only as a future non-authoritative helper risk. |
| **Verification Required** | Verify that movement legality tests instantiate only scene-tree-independent service cores; verify Godot Navigation, Physics, TileMapLayer, Node2D/global position, and screen/world coordinates cannot bypass logical-cell distance and MapSpaceState reservation/commit validation. |

> **Note**: If Knowledge Risk is MEDIUM or HIGH, this ADR must be re-validated if the
> project upgrades or downgrades engine versions. If a Godot version incompatibility blocks this contract, engine downgrade is an allowed technical option to evaluate, but the logical-cell authority boundary remains unchanged unless a superseding ADR says otherwise.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | ADR-0001 Map Data Representation; ADR-0002 Typed Query Result Schema; ADR-0003 Authoritative Occupancy / Reservation / Update Ordering; ADR-0005 Input Projection / Coordinate Conversion; ADR-0016 OpenMir2 Evidence Mapping Registry and Provisional Contract Governance, for evidence labels/readiness only and not hot runtime authority. |
| **Enables** | Movement implementation stories; future Pathfinding ADR; future AI route stories; Combat, Pickup, Spawn, and QA stories that consume canonical distance facts while owning their own thresholds. |
| **Blocks** | Any implementation story that decides movement adjacency, diagonal movement, corner-cutting, or shared map-cell distance semantics without this boundary. |
| **Ordering Note** | This ADR introduces no new authoritative mutable state. It defines distance facts and read-only movement legality evaluation. MapDefinition owns static facts; MapSpaceState owns occupancy/reservation mutation and deterministic conflict ordering; downstream systems own their own action ranges and final legality. |

## Context

### Problem Statement

The Map Coordinate / Blocking / Y-sort GDD requires one shared future Map Distance Contract so movement, combat, pickup, spawn, AI, and debug tools do not invent incompatible distance and neighbor semantics. The same GDD also states that diagonal movement, corner-cutting, pickup range, attack range, and spawn distance semantics are evidence-gated against OpenMir2 and must not be presented as source-authentic until accepted evidence exists. Without an ADR, movement implementation may accidentally treat Chebyshev distance, Godot navigation output, physics collisions, scene positions, or per-system local helper logic as authority.

### Constraints

- Logical `Vector2i` map cells are the canonical inputs for map-cell distance and adjacency.
- `MapDefinition` owns static passability/blocking facts under ADR-0001.
- `SpatialQueryResult`, `CellFactsSnapshot`, canonical reason/status semantics, and no-local-reason-string rules are owned by ADR-0002.
- `MapSpaceState` owns actor occupancy, item occupancy, reservations, command ordering, and conflict winner/loser resolution under ADR-0003.
- Screen/world/input projection into canonical logical cells is owned by ADR-0005.
- OpenMir2 authenticity labels and readiness gates are governed by ADR-0016, but hot gameplay systems must not query the evidence registry.
- Phase 1 must remain unit-testable without SceneTree, Autoload globals, Node lifecycle, physics callbacks, navigation agents, or visual TileMapLayer data authority.
- Combat, Pickup/Drop, Spawn, AI, and future Pathfinding systems need distance facts, but must not lose ownership of their own thresholds and action legality.

### Requirements

- Provide one canonical logical-cell distance facts contract.
- Provide one read-only movement legality query boundary for single-step/request evaluation.
- Fail closed on diagonal movement and corner-cutting until an accepted evidence-backed or explicitly provisional profile allows them.
- Preserve MapSpaceState as the only authority for occupancy/reservation mutation and deterministic update ordering.
- Preserve downstream ownership of attack range, pickup range, spawn radius, AI perception/leash radius, and pathfinding heuristics.
- Make all Phase 1 movement legality tests deterministic and independent of Godot scene/runtime helpers.

## Decision

ADR-0019 establishes a scene-tree-independent logical-cell distance and movement-legality boundary. The boundary intentionally keeps `MapDistanceService` and `MovementLegalityService` in one ADR because movement legality depends on canonical distance facts, but the services have separate ownership.

`MapDistanceService` derives canonical map-cell distance facts from two logical `Vector2i` cells only. It returns facts such as signed delta, absolute delta, Manhattan distance, Chebyshev distance, same-cell, orthogonal-neighbor, and diagonal-neighbor. These facts are not business-rule decisions: they do not define attack range, pickup range, spawn radius, AI perception, pathfinding heuristics, or movement approval.

`MovementLegalityService` is a read-only evaluator for one movement request or single-step candidate. Under the Phase 1 `mvp_provisional_orthogonal_only` profile, a movement step is eligible to submit as a movement reservation only when the target is an orthogonal neighbor (`manhattan_distance == 1`), the target is statically actor-passable, and the latest MapSpaceState snapshot shows no blocking actor or conflicting reservation. Same-cell requests return or map to `NO_MOVEMENT_REQUESTED` / `no_movement_requested`, create no reservation, and must not be treated as ordinary blocked movement. Diagonal movement and corner-cutting remain evidence-gated and fail closed under the Phase 1 profile.

`MovementLegalityService` does not mutate actor occupancy, item occupancy, reservations, command ordering, `created_sequence`, or movement conflict winners. All reservation creation, commit, cancel, cleanup, and deterministic winner/loser resolution remain owned by `MapSpaceState` under ADR-0003. A legality result based on a snapshot is advisory until the corresponding MapSpaceState command is accepted and resolved in the authoritative update phase. Legality results must not be cached as future movement authority across occupancy/reservation changes.

`MovementLegalityService` does not query `OpenMir2EvidenceRegistry` during hot gameplay. ADR-0016 governs evidence labels, accepted contract references, bootstrap validation, and story readiness. Runtime movement policy receives an already-selected profile such as `mvp_provisional_orthogonal_only` or a future accepted evidence-backed profile.

Godot Navigation, Physics, `CharacterBody2D` collision, `Area2D` callbacks, `TileMapLayer` navigation/collision/custom data, `Node2D`/global position, and screen/world coordinates are non-authoritative helpers only. Future pathfinding or navigation integrations may use their outputs as suggestions, but every executed step must be converted to logical cells and revalidated through `MapDistanceService`, `MovementLegalityService`, and MapSpaceState reservation/commit contracts.

### Architecture Diagram

```text
Screen/world input, AI intent, or scripted request
        |
        | ADR-0005 converts to canonical logical Vector2i cells
        v
source_cell + target_cell
        |
        v
MapDistanceService
  - delta_x / delta_y
  - manhattan_distance
  - chebyshev_distance
  - same/orthogonal/diagonal facts
        |
        v
MovementLegalityService (read-only)
  - consumes distance facts
  - consumes MapDefinition static facts
  - consumes MapSpaceState snapshot/query results
  - consumes selected movement policy profile
        |
        | eligible / no_movement_requested / blocked / unresolved
        v
Movement system submits typed command to MapSpaceState
        |
        v
MapSpaceState authoritative update phase
  - reservation ordering
  - winner/loser resolution
  - occupancy mutation
```

### Key Interfaces

Exact file paths and constructors are implementation details. Public APIs must remain injectable, scene-tree-independent, and unit-testable.

```gdscript
class_name MapDistanceFacts
extends RefCounted

var source_cell: Vector2i
var target_cell: Vector2i
var delta_x: int
var delta_y: int
var abs_delta_x: int
var abs_delta_y: int
var manhattan_distance: int
var chebyshev_distance: int
var is_same_cell: bool
var is_orthogonal_neighbor: bool
var is_diagonal_neighbor: bool
```

```gdscript
class_name MapDistanceService
extends RefCounted

func get_distance_facts(source_cell: Vector2i, target_cell: Vector2i) -> MapDistanceFacts:
    pass
```

```gdscript
class_name MovementLegalityPolicy
extends RefCounted

var policy_id: StringName
var source_status: StringName
var allows_diagonal: bool
var allows_corner_cutting: bool
var evidence_contract_id: StringName
var evidence_contract_version: int
```

```gdscript
class_name MovementLegalityService
extends RefCounted

func evaluate_single_step(
    actor_id: int,
    source_cell: Vector2i,
    target_cell: Vector2i,
    static_facts: MapCellStaticFacts,
    space_snapshot: CellFactsSnapshot,
    policy: MovementLegalityPolicy
) -> SpatialQueryResult:
    pass
```

The `MovementLegalityService` result uses ADR-0002 status/reason/context semantics. Under the Phase 1 `mvp_provisional_orthogonal_only` profile, diagonal requests map to `DIAGONAL_MOVEMENT_UNRESOLVED`, corner-cutting relationships map to `CORNER_CUTTING_UNRESOLVED`, and any broader unapproved movement-policy case maps to `MOVEMENT_RULE_UNRESOLVED`. These reasons use `SpatialQueryStatus.UNRESOLVED` and `SpatialRetryHint.BLOCKED_UNTIL_EVIDENCE` unless an accepted movement policy profile provides a more specific retry hint. Movement code must not invent local string reasons for these cases.

### Ownership Boundaries

`MapDistanceService` owns distance facts only. It does not own attack range, pickup range, spawn radius, leash radius, aggro radius, AI perception range, pathfinding heuristic choice, fallback search radius, or action legality.

`MovementLegalityService` owns read-only single-step/request movement evaluation only. It does not own path search, route smoothing, fallback target search, movement speed, animation timing, visual interpolation, or MapSpaceState mutation.

Combat may consume distance facts, but attack range, line/shape requirements, targeting legality, hit timing, and damage flow remain combat-owned. No distance fact is automatically an attack-legal result.

Pickup may consume same-cell or adjacency facts, but pickup range, claim resolution, inventory acceptance, ground drop availability, item removal, and success publication remain owned by ADR-0017 and downstream Pickup GDD/ADR work.

Spawn may consume candidate distance facts, but spawn radius, spawn region selection, fallback search order, spawn tables, spawn timing, and spawn-specific requirements remain spawn-owned.

### Non-Goals

This ADR does not define:

- path search, heuristics, smoothing, route fallback, AI route choice, or `NavigationAgent2D` integration;
- attack range, pickup range, spawn radius, aggro radius, leash radius, or targeting priority;
- movement speed, animation timing, interpolation, visual correction, or input click fallback;
- OpenMir2-authentic diagonal movement, corner-cutting, or distance semantics before accepted ADR-0016 evidence;
- MapSpaceState mutation ordering, reservation winner selection, or occupancy commit semantics already owned by ADR-0003;
- a mutable distance cache, movement policy hot-reload system, or live evidence registry query path.

## Alternatives Considered

### Alternative 1: Chebyshev distance as default adjacency authority

- **Description**: Treat `chebyshev_distance == 1` as the default neighbor/movement rule, allowing diagonal steps by default.
- **Pros**: Simple for eight-direction grid games; useful fact for some range checks.
- **Cons**: Implies diagonal movement authenticity without evidence; conflates distance facts with movement legality; can silently authorize corner-cutting; conflicts with the GDD's evidence-gated diagonal rule.
- **Rejection Reason**: Chebyshev distance remains a fact, not default movement approval. Phase 1 uses `mvp_provisional_orthogonal_only` until accepted evidence says otherwise.

### Alternative 2: Per-system local distance helpers

- **Description**: Let movement, combat, pickup, spawn, and AI each compute their own distance and adjacency semantics.
- **Pros**: Fast local implementation; avoids an upfront shared service.
- **Cons**: Creates semantic drift, incompatible tests, and cross-system bugs where attack/pickup/movement disagree on the same cells.
- **Rejection Reason**: The GDD explicitly calls for a shared Map Distance Contract; canonical facts are required before downstream range rules can be trusted.

### Alternative 3: Godot Navigation / Physics authority

- **Description**: Use `NavigationAgent2D`, `NavigationServer2D`, `NavigationRegion2D`, `CharacterBody2D` collision, `Area2D`, or TileMapLayer navigation/collision data as movement legality authority.
- **Pros**: Leverages engine helpers; may help future pathfinding or presentation.
- **Cons**: Ties core rules to scene/runtime state; hard to unit test; can diverge from logical map data; Godot 4.6 navigation/physics behaviours are post-cutoff and helper-oriented rather than OpenMir2 grid-rule authority.
- **Rejection Reason**: Godot helper systems may suggest or present movement, but authoritative movement legality must be logical-cell based and revalidated through MapSpaceState.

### Alternative 4: Fold movement legality into MapSpaceState

- **Description**: Make MapSpaceState decide all movement distance, policy, and static passability checks while applying occupancy mutations.
- **Pros**: One spatial authority; fewer service calls.
- **Cons**: Blurs static/distance/policy/mutation responsibilities; makes unit tests and downstream facts harder to reuse; risks turning MapSpaceState into a large policy engine.
- **Rejection Reason**: MapSpaceState should remain mutation and ordering authority; movement legality is a read-only preflight boundary using MapSpaceState snapshots.

## Consequences

### Positive

- Movement, combat, pickup, spawn, AI, and QA tools share one logical-cell fact vocabulary.
- Phase 1 movement defaults fail closed on diagonal/corner-cutting without pretending to be OpenMir2-authentic.
- Movement legality remains testable without Godot scene, navigation, physics, or visual tile helpers.
- MapSpaceState retains exclusive mutation and deterministic conflict-ordering authority.
- Future pathfinding can reuse movement legality without redefining adjacency rules.

### Negative

- A small service/DTO boundary must be implemented before movement stories proceed.
- Some downstream systems must distinguish “distance fact” from “action legal,” which is slightly more verbose than local helpers.
- Diagonal movement, corner-cutting, and exact OpenMir2 distance semantics remain unresolved until evidence work accepts the relevant contract.

### Risks

- **Risk**: Implementers treat `chebyshev_distance` as implicit movement or attack approval.
  **Mitigation**: ADR and registry forbid Chebyshev/default diagonal authority; tests must verify diagonal requests fail closed under the Phase 1 profile.
- **Risk**: MovementLegalityService grows into pathfinding or MapSpaceState mutation authority.
  **Mitigation**: Keep it read-only, single-step/request scoped, and require MapSpaceState command submission for mutation.
- **Risk**: Runtime code queries OpenMir2EvidenceRegistry on hot gameplay path.
  **Mitigation**: Runtime receives selected policy profile and copied evidence/contract references only; registry querying remains tooling/bootstrap/story-readiness.
- **Risk**: Godot navigation or physics helpers bypass logical validation.
  **Mitigation**: Any helper-produced target/path must be converted to logical cells and revalidated through the ADR-0019/ADR-0003 chain.

## GDD Requirements Addressed

| GDD System | Requirement | How This ADR Addresses It |
|------------|-------------|--------------------------|
| `map-coordinate-blocking-y-sort-system.md` | Movement, combat, pickup, spawn, AI, and QA need one shared distance/neighbor contract instead of local incompatible rules. | Defines `MapDistanceService` as canonical logical-cell distance facts provider. |
| `map-coordinate-blocking-y-sort-system.md` | Diagonal movement and corner-cutting must remain evidence-gated until OpenMir2 semantics are verified. | Sets Phase 1 `mvp_provisional_orthogonal_only` profile and fail-closed diagonal/corner-cutting behaviour. |
| `map-coordinate-blocking-y-sort-system.md` | Movement legality must not use visual, physics, or navigation state as gameplay authority. | Bans Godot Navigation/Physics/TileMapLayer/Node position authority for movement legality. |
| `map-coordinate-blocking-y-sort-system.md` | Runtime spatial mutation must remain deterministic and flow through the authoritative map-space update path. | Keeps MovementLegalityService read-only and routes reservation/commit through MapSpaceState under ADR-0003. |
| `map-coordinate-blocking-y-sort-system.md` | Pickup/attack/spawn distance rules require shared inputs but downstream systems retain their own rules and thresholds. | Provides distance facts only and explicitly preserves Combat, Pickup/Drop, Spawn, and AI ownership of thresholds/action legality. |

## Performance Implications

- **CPU**: Distance fact calculation is O(1) integer arithmetic. Movement legality is O(1) for a single step/request, assuming MapDefinition and MapSpaceState snapshot/query access remain O(1) as required by prior ADRs.
- **Memory**: Phase 1 may allocate compact immutable-by-contract DTOs for tests/debug and use no-allocation enum/flag helpers on hot paths as long as ADR-0002 reason semantics remain identical. No cache is introduced by this ADR.
- **Load Time**: No load-time cost beyond selecting or validating a movement policy profile during bootstrap/story readiness.
- **Network**: None for Phase 1 offline slice. Future networking must preserve logical-cell authority and cannot treat client navigation/physics output as authoritative movement truth.

## Migration Plan

No gameplay implementation exists yet. Implementation should proceed by:

1. Define the minimal `MapDistanceFacts`, `MapDistanceService`, `MovementLegalityPolicy`, and `MovementLegalityService` test-facing contracts.
2. Extend or reuse ADR-0002 reason/status mappings for diagonal unresolved/evidence-blocked movement without local string reasons.
3. Add GUT unit tests for same-cell, orthogonal, diagonal, blocked static cell, actor-occupied cell, reserved cell, and helper-output-bypass cases.
4. Integrate movement request submission so eligible requests create MapSpaceState commands rather than mutating occupancy directly.
5. Keep future pathfinding, AI route, combat range, pickup range, and spawn radius stories dependent on this contract but outside this ADR's implementation scope.

## Validation Criteria

- [ ] Distance facts for same-cell, orthogonal neighbor, diagonal neighbor, and non-neighbor cells are deterministic from `Vector2i` inputs only.
- [ ] Distance facts do not read `Node2D`, `TileMapLayer`, physics, navigation, screen, or world coordinates.
- [ ] Same-cell movement returns `NO_MOVEMENT_REQUESTED` / `no_movement_requested` and creates no reservation command.
- [ ] Orthogonal target with static passability and no occupancy/reservation conflict is eligible to submit a movement reservation.
- [ ] Statically blocked target fails with the ADR-0002 canonical static-block reason.
- [ ] Actor-occupied target fails with the ADR-0002 canonical actor-block reason.
- [ ] Reserved target fails with the ADR-0002 canonical reserved reason.
- [ ] Diagonal target fails closed under `mvp_provisional_orthogonal_only` with an unresolved/evidence-blocked reason.
- [ ] MovementLegalityService does not mutate MapSpaceState or assign command sequence values.
- [ ] Navigation, physics, TileMapLayer, Node position, and input projection outputs cannot bypass logical revalidation.
- [ ] Combat, Pickup, Spawn, and AI tests consume distance facts without treating them as ADR-0019-owned thresholds.

## Related Decisions

- `docs/architecture/adr-0001-map-data-representation.md`
- `docs/architecture/adr-0002-typed-query-result-schema.md`
- `docs/architecture/adr-0003-authoritative-occupancy-reservation-update-ordering.md`
- `docs/architecture/adr-0005-input-projection-coordinate-conversion.md`
- `docs/architecture/adr-0016-openmir2-evidence-mapping-registry-and-provisional-contract-governance.md`
- `docs/architecture/adr-0017-drop-table-ground-drop-and-pickup-lifecycle-boundary.md`
- `design/gdd/map-coordinate-blocking-y-sort-system.md`
