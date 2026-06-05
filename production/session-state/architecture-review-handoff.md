# Architecture Review Handoff — Map Coordinate / Blocking / Y-sort System

Date: 2026-06-04

## Purpose

This handoff prepares a fresh Claude Code session to run `/architecture-review` without relying on the ADR authoring conversation context.

Per CCGS process, `/architecture-review` must not run in the same session that authored the ADRs. The review agent must be independent of the authoring context.

## Target Review

Run in a fresh session:

```text
/architecture-review
```

Primary scope:

- `design/gdd/map-coordinate-blocking-y-sort-system.md`
- `docs/architecture/adr-0001-map-data-representation.md`
- `docs/architecture/adr-0002-typed-query-result-schema.md`
- `docs/architecture/adr-0003-authoritative-occupancy-reservation-update-ordering.md`
- `docs/architecture/adr-0004-deterministic-y-sort-implementation.md`
- `docs/architecture/adr-0005-input-projection-coordinate-conversion.md`
- `docs/registry/architecture.yaml`

## Current CCGS State

The `地图坐标 / 阻挡 / Y-sort 系统` GDD is approved for downstream architecture planning.

Approval tracking already written:

- `design/gdd/systems-index.md` marks the system as `Approved`.
- `design/gdd/reviews/map-coordinate-blocking-y-sort-system-review-log.md` contains the approval entry.

Implementation remains gated by:

- Proposed ADRs becoming accepted after architecture review.
- OpenMir2 evidence gates where the GDD marks rules provisional or source-authenticity blocked.
- Downstream GDDs for movement, pickup/drop, map conversion, loot readability, and distance semantics where applicable.

## ADR Set to Review

### ADR-0001: Map Data Representation

Path: `docs/architecture/adr-0001-map-data-representation.md`

Status: `Proposed`

Core decision:

- `MapDefinition` Resource is the authoritative static gameplay map data source.
- Static map facts use flattened dense arrays / `Packed*Array` where practical.
- `TileMapLayer` is visual/editor presentation only and cannot override gameplay truth.
- `MapSpaceState` or equivalent runtime service owns actor occupancy, item occupancy, and reservations.

Important registry stances:

- `static_map_facts` owned by map-data-representation.
- `runtime_map_space_state` owned by map-space-runtime.
- Forbidden: visual tile data as gameplay authority.
- Forbidden: per-cell Resource/Node dense map storage.
- Forbidden: normal gameplay full-map scan.

### ADR-0002: Typed Query Result Schema

Path: `docs/architecture/adr-0002-typed-query-result-schema.md`

Status: `Proposed`

Core decision:

- Use typed GDScript enums as internal authority for status, reason, context, and retry hints.
- Use `RefCounted` DTOs such as `SpatialQueryResult` and `CellFactsSnapshot` for full semantic snapshots.
- `StringName` is only for debug/export/log/QA mapping, not internal gameplay authority.
- Hot-path helpers are allowed only if they preserve the same reason priority semantics.

Important guardrails:

- No local downstream reason strings.
- No dictionary-only authoritative query payloads.
- Snapshots must not expose mutable runtime references.

### ADR-0003: Authoritative Occupancy / Reservation Update Ordering

Path: `docs/architecture/adr-0003-authoritative-occupancy-reservation-update-ordering.md`

Status: `Proposed`

Core decision:

- `MapSpaceState` owns an authoritative command queue.
- Systems submit typed commands/intents instead of directly mutating occupancy/reservation state.
- `created_sequence` is assigned by the authoritative queue/service.
- Commands resolve deterministically by sequence and stable actor/item/request IDs.
- Mutations apply atomically during an authoritative update phase.

Important guardrails:

- No direct map-space state mutation outside `MapSpaceState`.
- Godot signal callback order must not decide gameplay authority.
- Scene-tree order, wall-clock time, node creation order, or arbitrary callback order cannot decide occupancy/reservation conflicts.

### ADR-0004: Deterministic Y-sort Implementation

Path: `docs/architecture/adr-0004-deterministic-y-sort-implementation.md`

Status: `Proposed`

Core decision:

- `MapSortCoordinator` computes explicit `(anchor_y, object_type_sort_rank, stable_instance_id)` keys.
- Godot rendering mechanisms such as `Node2D.y_sort_enabled`, `z_index`, layered containers, or coordinator-managed child order may only apply the already-computed presentation order.
- Deprecated `YSort` nodes are forbidden.
- Scene-tree insertion order is not a valid tie-break.

Important guardrails:

- Y-sort is visual-only and cannot affect gameplay legality.
- Invalid anchor/rank/stable ID returns structured debug reasons.
- Dirty batching is the normal refresh path.

### ADR-0005: Input Projection / Coordinate Conversion

Path: `docs/architecture/adr-0005-input-projection-coordinate-conversion.md`

Status: `Proposed`

Core decision:

- `MapProjection` converts UI-gated screen/viewport/world input into logical `Vector2i` candidates.
- Failed, ambiguous, or unresolved projection returns structured `invalid_coordinate` results.
- Projection must not silently fall back to `(0, 0)`, nearest arbitrary cell, visual TileMapLayer cell, or last valid cell.
- `TileMapLayer` is not gameplay coordinate authority.

Important guardrails:

- Input must pass UI/input routing before world projection.
- Godot 4.6 dual-focus means mouse/touch and keyboard/gamepad focus paths must be tested separately where applicable.
- Actor/controller-local coordinate conversion is forbidden.

## Registry State

`docs/registry/architecture.yaml` has been updated with:

- State ownership:
  - `static_map_facts`
  - `runtime_map_space_state`
- Interface contracts:
  - `map_static_facts_lookup`
  - `spatial_query_result_schema`
  - `map_space_command_submission`
  - `deterministic_y_sort_key`
  - `input_projection_to_logical_cell`
- API decisions:
  - `logical_map_data_source`
  - `visual_tile_layer`
  - `spatial_query_schema_representation`
  - `map_space_update_authority`
  - `y_sort_presentation_ordering`
  - `input_projection_service`
- Forbidden patterns:
  - `visual_tile_data_as_gameplay_authority`
  - `per_cell_resource_or_node_map_storage`
  - `normal_gameplay_full_map_scan`
  - `local_spatial_reason_strings`
  - `dictionary_only_spatial_query_payload`
  - `direct_map_space_state_mutation`
  - `signal_callback_order_as_gameplay_authority`
  - `scene_tree_order_as_sort_authority`
  - `controller_local_coordinate_conversion`
  - `ui_consumed_input_world_projection`

## Known Review Focus Areas

The architecture review should verify:

1. All GDD implementation prerequisites are covered by ADR-0001 through ADR-0005.
2. The ADR set does not contradict the approved GDD's provisional/source-authentic evidence gates.
3. The ADRs do not accidentally decide downstream-owned rules:
   - movement speed
   - pathfinding
   - diagonal movement / corner cutting
   - pickup distance / ownership / inventory
   - combat distance
   - drop fallback search radius
   - player-facing loot readability affordance
   - source-authentic OpenMir2 coordinate origin/axis/projection
4. ADR-0003 and ADR-0005 remain contract-level and do not overreach into movement/pathfinding or target-selection UX.
5. Godot 4.6.3 engine risk is consistently documented.
6. Deprecated Godot APIs are avoided:
   - no `TileMap` as new implementation basis
   - no deprecated `YSort` nodes
   - no string-based signal coupling where typed/callable patterns are expected later
7. Registry stances match ADR content and do not overstate decisions beyond the ADR text.
8. ADR statuses are currently `Proposed`; stories remain blocked until accepted according to docs/CLAUDE.md.

## Expected Outcomes

If `/architecture-review` returns PASS or minor CONCERNS:

- Mark reviewed ADRs as ready to be moved from `Proposed` to `Accepted` according to the project's ADR status process.
- Generate or update architecture review artifacts as the skill dictates.
- Then proceed toward `/create-architecture` or `/create-control-manifest` depending on project stage and skill recommendation.

If `/architecture-review` returns FAIL or blocking concerns:

- Fix only the identified ADR/GDD/registry conflicts.
- Update `production/session-state/active.md` with the blockers and revised next step.

## Do Not Do in the Review Session

- Do not implement code.
- Do not create stories before ADR acceptance/control manifest readiness.
- Do not treat MVP provisional GDD rules as OpenMir2-authentic behavior.
- Do not run `/architecture-review` from the same ADR-authoring context.
