# ADR-0002: Typed Query Result Schema

## Status

Accepted

## Date

2026-06-04

## Last Verified

2026-06-04

## Decision Makers

hkm + Claude Code Game Studios

## Summary

This ADR defines the typed GDScript schema for spatial query results used by the map coordinate / blocking / Y-sort system. We choose typed enums plus `RefCounted` DTO snapshots for full semantic results, with optional no-allocation helpers that must preserve the same canonical reason semantics.

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Core / Scripting |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff and must be verified against engine reference docs. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md` |
| **Post-Cutoff APIs Used** | None required. This ADR relies on GDScript typed classes/enums and `RefCounted`. |
| **Verification Required** | Verify enum typing, DTO allocation behaviour, defensive snapshot copying, and reason/status mapping tests under Godot 4.6.3. |

> **Note**: If Knowledge Risk is MEDIUM or HIGH, this ADR must be re-validated if the
> project upgrades engine versions. Flag it as "Superseded" and write a new ADR.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | ADR-0001 Map Data Representation |
| **Enables** | ADR-0003 Authoritative Occupancy / Reservation Update Ordering; implementation stories for map-space query APIs. |
| **Blocks** | Any implementation story that returns map-space query results to movement, spawn, drop, pickup, combat, rendering, or QA/debug tools. |
| **Ordering Note** | This ADR defines result schema only. It does not define static map storage, runtime mutation ordering, input projection, or rendering execution. |

## Context

### Problem Statement

The GDD requires spatial queries to return structured results containing `status`, `primary_reason`, `secondary_reasons`, `query_context`, immutable `cell_facts`, and optional `retry_hint`. Without a typed schema, downstream systems may use ad hoc dictionaries, booleans, or local reason strings, causing semantic drift and making tests unable to prove that every system consumes the same map-space contract.

### Current State

ADR-0001 defines `MapDefinition` as static map facts authority and `MapSpaceState` as runtime spatial state authority. No typed query result implementation exists yet.

### Constraints

- Query results must match the approved GDD reason registry and query contexts.
- Runtime snapshots must not expose mutable `MapSpaceState` or `MapDefinition` internals.
- Full DTO snapshots are required for tests, QA/debug, and integration evidence.
- Hot paths may avoid allocations, but cannot fork semantics from the canonical result path.
- New reason names or query contexts require GDD/ADR revision; downstream systems may not invent local strings.

### Requirements

- Provide typed, unit-testable result structures.
- Provide canonical status, reason, context, and retry hint families.
- Preserve deterministic `query_result_priority` semantics.
- Support immutable `cell_facts` snapshots for debug/QA evidence.
- Provide conversion to stable text keys for logs, QA reports, and serialization without making strings the internal authority.

## Decision

Use typed GDScript enums as the authoritative in-memory representation for query status, query reason, query context, and retry hint. Use `RefCounted` DTO classes for full semantic query results and snapshots. `StringName` values may be exposed only through explicit conversion helpers for debug output, QA evidence, logs, serialization, editor-facing displays, and data-driven registry lookup; they are not the internal authority for gameplay decisions.

`SpatialQueryResult` is the primary full-result DTO. `CellFactsSnapshot` is an immutable-by-contract snapshot assembled from `MapDefinition` and `MapSpaceState` facts. Arrays inside snapshots must be copied or constructed as owned values; snapshots must not expose mutable runtime arrays, dictionaries, actor nodes, item nodes, or reservation records.

Hot-path helpers may return primitive booleans, enum reasons, or compact flags to avoid DTO allocation, but they must share the same evaluation logic and reason priority table as the full DTO path. A hot-path result that would disagree with `SpatialQueryResult` is a defect.

### Architecture

```text
MapDefinition static facts + MapSpaceState runtime facts
        |
        v
Spatial query evaluator
        |
        +--> full semantic path: SpatialQueryResult + CellFactsSnapshot
        |
        +--> hot path helper: enum/flag result with identical priority semantics
        |
        v
Movement / spawn / drop / pickup / combat / rendering / QA-debug consumers
```

### Key Interfaces

```gdscript
enum SpatialQueryStatus {
    ALLOWED,
    BLOCKED,
    UNAVAILABLE,
    UNRESOLVED,
}

enum SpatialQueryContext {
    ACTOR_ENTRY,
    ITEM_PLACEMENT,
    PICKUP_SPATIAL_CANDIDATE,
    RESERVATION_CREATE,
    RESERVATION_COMMIT,
    COORDINATE_CONVERSION,
    PRESENTATION_DEBUG,
}

enum SpatialQueryReason {
    WALKABLE,
    ITEM_PLACEABLE,
    INVALID_COORDINATE,
    UNKNOWN_OR_UNLOADED,
    OUT_OF_BOUNDS,
    BLOCKED_BY_STATIC_MAP,
    BLOCKED_BY_ACTOR,
    RESERVED,
    ACTOR_ALREADY_RESERVED,
    RESERVATION_OWNER_MISMATCH,
    SOURCE_OCCUPANCY_LOST,
    ITEM_CAPACITY_FULL,
    NO_ITEM_PRESENT,
    PICKUP_RULE_UNRESOLVED,
    PICKUP_SELECTION_ORDER_UNRESOLVED,
    PICKUP_DISTANCE_FAILED,
    ITEM_NOT_AVAILABLE,
    DROP_FALLBACK_REQUIRED,
    PICKUP_CANDIDATE_AVAILABLE,
    PICKUP_COMMIT_BLOCKED_UNTIL_RULE_APPROVED,
    CROSS_MAP_MOVEMENT_UNDEFINED,
    MOVEMENT_RULE_UNRESOLVED,
    DIAGONAL_MOVEMENT_UNRESOLVED,
    CORNER_CUTTING_UNRESOLVED,
    INVALID_Y_SORT_ANCHOR,
    INVALID_Y_SORT_RANK,
    INVALID_STABLE_SORT_ID,
    NO_MOVEMENT_REQUESTED,
}

enum SpatialRetryHint {
    NONE,
    NEVER_SAME_STATE,
    WAIT_OR_BACKOFF,
    REPATH_REQUIRED,
    FALLBACK_REQUIRED,
    BLOCKED_UNTIL_EVIDENCE,
}

class_name CellFactsSnapshot
extends RefCounted

var map_id: StringName
var cell: Vector2i
var static_passability_state: int
var actor_occupant_id: int
var reservation_id: int
var reservation_owner_actor_id: int
var item_count: int
var item_instance_ids: Array[int]
var stable_drop_sequences: Array[int]
var y_sort_anchor_valid: bool
var y_sort_key_preview: Variant

class_name SpatialQueryResult
extends RefCounted

var status: SpatialQueryStatus
var primary_reason: SpatialQueryReason
var secondary_reasons: Array[SpatialQueryReason]
var query_context: SpatialQueryContext
var cell_facts: CellFactsSnapshot
var retry_hint: SpatialRetryHint
```

Field names mirror the GDD. Exact file locations and constructor/factory style are implementation details, but public APIs must use typed values and must be unit-testable.

The movement evidence-gate extensions added for ADR-0019 map as follows: `MOVEMENT_RULE_UNRESOLVED`, `DIAGONAL_MOVEMENT_UNRESOLVED`, and `CORNER_CUTTING_UNRESOLVED` all map to `SpatialQueryStatus.UNRESOLVED` with `SpatialRetryHint.BLOCKED_UNTIL_EVIDENCE` unless an accepted movement policy profile provides a more specific retry hint. They are canonical reasons, not local movement-system strings.

### Implementation Guidelines

- Use typed enums as internal authority; never compare gameplay reasons by raw string.
- Provide explicit `reason_to_string_name()`, `status_to_string_name()`, `context_to_string_name()`, and inverse validation helpers for debug/export paths.
- Ensure every GDD canonical reason maps to exactly one status unless the GDD is revised.
- Defensive-copy snapshot arrays such as `item_instance_ids` and `stable_drop_sequences`.
- Do not place actor nodes, item nodes, reservation objects, or mutable dictionaries inside `CellFactsSnapshot`.
- Full DTO creation is acceptable for tests, QA/debug, and non-hot query paths; hot-path helpers must preserve identical reason priority semantics.

## Alternatives Considered

### Alternative 1: Dictionary payloads

- **Description**: Return `Dictionary` values with string keys for status, reason, context, and facts.
- **Pros**: Fast to prototype; easy to serialize; flexible when fields change.
- **Cons**: Weak typing, typo-prone keys, poor refactor safety, harder to prove reason coverage in tests.
- **Estimated Effort**: Lower initial effort, higher long-term QA and integration cost.
- **Rejection Reason**: The GDD requires a canonical machine-readable contract shared across systems; dictionaries invite semantic drift.

### Alternative 2: Boolean plus reason

- **Description**: Return only `allowed: bool` plus one reason value.
- **Pros**: Lightweight and simple for movement checks.
- **Cons**: Cannot represent `cell_facts`, secondary reasons, retry hints, query context, or QA/debug evidence required by the GDD.
- **Estimated Effort**: Low initial effort, insufficient feature coverage.
- **Rejection Reason**: It fails the approved structured query result schema rule.

### Alternative 3: StringName-only reason registry

- **Description**: Use `StringName` constants as the internal reason/context/status authority.
- **Pros**: Stable for logs, editor display, and data-driven lookup; avoids enum migration if names change.
- **Cons**: Weaker static typing than enums; typo or invalid combination errors are easier; tests cannot rely on compiler help.
- **Estimated Effort**: Similar implementation effort.
- **Rejection Reason**: `StringName` is useful for external mapping, but typed enums are safer as gameplay authority.

## Consequences

### Positive

- Downstream systems consume one canonical query result contract.
- Unit tests can cover every reason/status/context combination.
- Debug/QA evidence can include rich immutable snapshots without exposing runtime state.
- Hot paths remain possible without losing semantics.

### Negative

- More boilerplate than dictionaries or booleans.
- Enum evolution requires coordinated GDD/ADR updates.
- DTO allocation must be managed carefully in frequent query paths.

### Neutral

- This ADR defines schema, not the algorithms that generate results.
- Exact class file locations and constructor patterns remain implementation choices.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| DTO allocation creates hot-path pressure. | Medium | Medium | Provide no-allocation helpers that share canonical logic; reserve full snapshots for debug/QA/non-hot paths where needed. |
| Downstream systems invent local reasons. | Medium | High | Register canonical enum family; require GDD/ADR revision for new reasons. |
| Snapshot accidentally exposes mutable state. | Medium | High | Defensive-copy arrays; store stable IDs and scalar facts, not object references. |
| Reason/status mapping drifts from GDD. | Medium | High | Unit-test every reason mapping and priority branch. |

## Performance Implications

| Metric | Before | Expected After | Budget |
|--------|--------|---------------|--------|
| CPU | No query schema | Enum evaluation and optional DTO construction; hot helpers available | Must respect GDD query guardrails and 16.6 ms frame budget |
| Memory | No query schema | DTO allocations for full results; no-allocation helpers for hot paths | Phase 1 client under 1 GB RAM |
| Load Time | N/A | N/A | N/A |
| Network | N/A | N/A | N/A |

## Migration Plan

No existing implementation requires migration.

1. Define enum families and conversion helpers.
2. Define `SpatialQueryResult` and `CellFactsSnapshot` DTOs.
3. Add reason/status/context unit tests.
4. Implement full-result query factory methods.
5. Add no-allocation helpers only after full semantic tests exist.

**Rollback plan**: If enum DTOs prove too heavy, write a superseding ADR that preserves the same semantic contract while changing representation. Do not downgrade to ad hoc strings without preserving canonical tests.

## Validation Criteria

- [ ] Every GDD canonical reason exists in `SpatialQueryReason`.
- [ ] Every query context from the GDD exists in `SpatialQueryContext`.
- [ ] Every reason maps to the expected status.
- [ ] `query_result_priority` returns deterministic primary reasons under multi-reason conditions.
- [ ] `CellFactsSnapshot` does not expose mutable runtime references.
- [ ] Hot-path helpers return results consistent with full DTO queries.
- [ ] Debug/export conversion uses stable `StringName` values without making strings internal gameplay authority.

## GDD Requirements Addressed

| GDD Document | System | Requirement | How This ADR Satisfies It |
|-------------|--------|-------------|--------------------------|
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Structured Query Result Schema Rule | Defines typed `SpatialQueryResult`, `CellFactsSnapshot`, status, reason, context, and retry hint families. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | `query_result_priority` Formula | Requires canonical reason priority tests and identical hot/full semantics. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Performance Guardrail Rule | Allows no-allocation hot helpers while preserving structured semantics. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Debug / QA UI Requirements | Provides immutable snapshot DTOs suitable for bounded debug evidence. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | 地图坐标 / 阻挡 / Y-sort 系统 | Implementation Boundary and ADR Prerequisites Rule | Resolves the typed GDScript query result schema prerequisite. |

## Related

- `docs/architecture/adr-0001-map-data-representation.md`
- `design/gdd/map-coordinate-blocking-y-sort-system.md`
- Future implementation stories for map-space queries and QA/debug evidence
