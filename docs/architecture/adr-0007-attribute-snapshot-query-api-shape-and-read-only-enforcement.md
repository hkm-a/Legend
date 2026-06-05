# ADR-0007: Attribute Snapshot Query API Shape and Read-Only Enforcement

## Status

Accepted

## Date

2026-06-04

## Last Verified

2026-06-04

## Decision Makers

hkm + Claude Code Game Studios

## Summary

This ADR defines how Character Attributes publishes versioned snapshots and query results to consumers without exposing mutable runtime state. We choose `RefCounted` immutable-by-contract snapshot/value DTOs returned through status-bearing query result wrappers, with defensive-copy collection access, duplicated stale/invalid status metadata, no scene tree dependency, and explicit tests enforcing the read-only boundary.

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Core / Scripting |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff and must be verified against engine reference docs. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md`; `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md` |
| **Post-Cutoff APIs Used** | None required. This ADR relies on standard GDScript classes, typed methods, typed arrays, `RefCounted`, and explicit boundary validation. It does not depend on post-cutoff-only APIs such as `@abstract`, variadic arguments, or `Resource.duplicate_deep()`. |
| **Verification Required** | Verify `RefCounted` reference aliasing behavior, typed array copy behavior, shallow-vs-deep `Array`/`Dictionary` copy behavior, no public property/mutator exposure, snapshot stability after later runtime mutation, stale/invalid status propagation, and GUT tests proving consumer mutation attempts cannot alter authoritative runtime state or previously published snapshots. |

> **Note**: If Knowledge Risk is MEDIUM or HIGH, this ADR must be re-validated if the project upgrades engine versions. Flag it as "Superseded" and write a new ADR.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | ADR-0006 Attribute Data Representation and Stat ID Typing; approved Character Attributes GDD: `design/gdd/character-attributes-system.md`. |
| **Enables** | Attribute snapshot DTO implementation; attribute provider/service query interface implementation; combat read-only stat consumption; HUD read-only stat/resource display; equipment post-equip delta consumption; save/load attribute read boundary; QA/debug attribute inspection tools; GUT tests for snapshot immutability-by-contract, stale status propagation, and defensive-copy collection access; later ADR for event/signal contract and scene-tree-independent core. |
| **Blocks** | Any story where combat, HUD, equipment, AI, save/load, or debug tools consume attribute values; any story exposing `AttributeSnapshot`, `AttributeSnapshotQueryResult`, `AttributeStatQueryResult`, or `AttributeResourceQueryResult`; any story publishing snapshot versions; any story handling failed rebuild visibility/stale snapshot behavior; any story passing attribute snapshot data across system boundaries. |
| **Ordering Note** | ADR-0006 governs stat identity and storage. This ADR governs read access and publication. Event dispatch, transaction/commit ordering, save file format, fixture loading, formula-only GUT setup, and combat power ownership remain separate required ADRs or technical designs. |

## Context

### Problem Statement

The Character Attributes GDD requires a read-only, versioned `AttributeSnapshot` or equivalent so combat, HUD, equipment, AI, save/load, and QA/debug tools can consume attribute truth without mutating runtime authority. The system must publish actor ID, actor type, snapshot version, schema/config version, source status, effective stats, current resources, valid/invalid/stale state, lightweight reason/changed metadata, and safe player-facing deltas while keeping heavy debug trace pull-based.

Godot/GDScript complicates this because `RefCounted`, `Array`, `Dictionary`, and custom classes use reference semantics and GDScript does not provide language-enforced immutable objects or true private fields. Therefore the project needs an explicit API shape that enforces read-only behavior through public API design, construction-time copying, no mutable exposure, conventions, code review, and tests.

### Current State

No Character Attributes implementation exists. ADR-0006 establishes project-owned `StatId` identity, external `StringName` boundaries, normalized runtime storage, and forbidden patterns for mutable Resources and shared mutable DTO payloads. This ADR builds on that foundation to define how attribute data crosses system boundaries safely.

### Constraints

- Snapshot/query APIs must be testable without scene tree, UI, signals, or Autoload.
- Consumers must not observe mutable backing arrays, dictionaries, Resources, or runtime service internals.
- Failed rebuilds must not replace the current valid snapshot.
- Previous valid snapshots may be exposed only with stale/display-only status when a rebuild fails.
- Debug trace must be lazy/pull-based or invalid/debug-only, not part of normal runtime snapshots.
- Public APIs must preserve ADR-0006 `StatId` semantics. If implemented as enum/int constants in GDScript, raw arbitrary integers are still invalid until range/registry validation succeeds.

### Requirements

- Publish actor-local monotonic snapshot versions.
- Provide schema version, config version, validity state, and source status summary.
- Provide stat and resource query results with typed success/failure semantics.
- Support visible delta summary and changed stat metadata without exposing mutable internals.
- Preserve previous published snapshot stability after later runtime mutation.
- Prevent stale/invalid/display-only snapshots from being consumed as current combat truth.
- Provide automated tests proving consumer mutation attempts cannot alter authoritative state or published snapshots.

## Decision

Character Attributes publishes snapshots as `RefCounted` immutable-by-contract value objects created only by the character attributes runtime/provider after successful structural rebuild or approved resource mutation. Snapshots are read-only through their public API: no public mutable vars, no setters, no mutator methods, and no internal mutable `Array`/`Dictionary` references returned to consumers.

Read-only enforcement is contractual, not compiler-enforced. GDScript does not provide engine-enforced immutable objects or true private fields. This ADR therefore defines read-only enforcement as the combination of:

- public API shape with getter/query methods only;
- no public mutable fields on snapshots, query results, row DTOs, or status DTOs;
- construction-time copying from runtime mutable state;
- no exposed backing arrays, dictionaries, Resources, or mutable nested objects;
- provider-level query wrappers that carry validity/stale/failure status;
- code review against registered forbidden patterns;
- automated tests proving consumer mutation attempts cannot affect runtime authority or previously published snapshots.

Consumers should obtain snapshots through an `AttributeSnapshotQueryResult` or equivalent provider result, not by directly reaching into the runtime service. Direct snapshot references are still read-only views, but stale/invalid/display-only status must be duplicated both in the wrapper/result and in snapshot metadata so consumers cannot accidentally treat an old snapshot as current truth.

If a structural rebuild fails, the provider does not replace the current valid snapshot. It returns a failure/invalid result containing failure reasons and, if available, a previous valid snapshot marked stale/display-only. Combat and other gameplay consumers must not consume that previous snapshot as current authoritative truth; UI/debug may display it only with player-safe stale/unavailable status.

All stat/resource identity in public APIs follows ADR-0006. Interface sketches use `StatId` and `ResourceId` as conceptual project-owned identities. If GDScript implementation represents them as `int` enum constants, every public query boundary must validate that the value is a current valid project-owned ID; raw arbitrary integers are not semantically valid stat/resource identity.

Collection return rules:

- Getter methods must not return internal arrays or dictionaries.
- Arrays of scalar values may be returned as shallow defensive copies.
- Arrays containing DTO rows must either clone row DTOs at query time or guarantee those row DTOs are immutable-by-contract under the same no-public-var/no-setter/no-mutator/no-mutable-container rules.
- Normal gameplay APIs should avoid returning `Dictionary`. Debug-only dictionary output may be assembled on demand as boundary/debug data and must not become gameplay authority.
- `const`, typed arrays, and shallow `duplicate()` are not considered read-only enforcement mechanisms by themselves.

Debug traces are not embedded as unbounded formatted strings in normal snapshots. They are generated lazily, returned only through explicit debug/invalid query APIs, or included in structured invalid/failure results when needed for QA evidence.

### Architecture

```text
Character Attributes runtime authority
(normalized registry + mutable current state + version counters)
        |
        | publish after successful rebuild/resource mutation
        | copy from mutable runtime backing storage
        v
AttributeSnapshot immutable-by-contract value object
        |
        +--> scalar getter access
        +--> stat/resource query result DTOs
        +--> defensive-copy scalar collections
        +--> immutable-by-contract row DTO collections
        |
        v
AttributeSnapshotQueryResult wrapper
(valid/current | stale/display-only | invalid/failure)
        |
        v
Combat / equipment / HUD / save-load / AI / QA-debug consumers
```

### Key Interfaces

```gdscript
class_name AttributeSnapshotProvider
extends RefCounted

func get_current_snapshot(actor_id: int) -> AttributeSnapshotQueryResult:
    pass

func get_snapshot_at_version(actor_id: int, snapshot_version: int) -> AttributeSnapshotQueryResult:
    pass

func query_stat(actor_id: int, stat_id: StatId) -> AttributeStatQueryResult:
    pass

func query_resource(actor_id: int, resource_id: ResourceId) -> AttributeResourceQueryResult:
    pass
```

```gdscript
class_name AttributeSnapshot
extends RefCounted

func get_actor_id() -> int:
    pass

func get_actor_type() -> ActorTypeId:
    pass

func get_snapshot_version() -> int:
    pass

func get_schema_version() -> int:
    pass

func get_config_version() -> int:
    pass

func get_validity_state() -> AttributeSnapshotValidityState:
    pass

func get_source_status_summary() -> AttributeSourceStatusSummary:
    pass

func has_stat(stat_id: StatId) -> bool:
    pass

func get_stat_value(stat_id: StatId) -> AttributeStatQueryResult:
    pass

func get_resource_current(resource_id: ResourceId) -> AttributeResourceQueryResult:
    pass

func get_changed_stat_ids_copy() -> Array[StatId]:
    pass

func get_visible_delta_summary_copy() -> Array[AttributeDeltaSummaryRow]:
    pass
```

```gdscript
class_name AttributeSnapshotQueryResult
extends RefCounted

func is_success() -> bool:
    pass

func get_snapshot() -> AttributeSnapshot:
    pass

func get_result_state() -> AttributeSnapshotQueryState:
    pass

func get_failure_reason() -> AttributeQueryFailureReason:
    pass

func get_previous_valid_snapshot() -> AttributeSnapshot:
    pass
```

```gdscript
class_name AttributeStatQueryResult
extends RefCounted

func is_success() -> bool:
    pass

func get_stat_id() -> StatId:
    pass

func get_value() -> int:
    pass

func get_failure_reason() -> AttributeQueryFailureReason:
    pass

func get_snapshot_version() -> int:
    pass
```

The `StatId`, `ResourceId`, `ActorTypeId`, `AttributeSnapshotValidityState`, `AttributeSnapshotQueryState`, and `AttributeQueryFailureReason` names are conceptual project-owned typed identities. GDScript may implement them as enum/int constants, but the public contract requires validation and must not treat arbitrary integers as valid IDs.

Each `class_name` should live in its own `.gd` file. Interface sketches in this ADR show multiple classes for clarity, not as a single script layout.

### Implementation Guidelines

- Construct snapshots only from validated runtime state after successful rebuild/resource mutation or as explicit stale/display-only previous snapshot references in failure results.
- Snapshot construction must copy mutable runtime arrays/dictionaries. Published snapshots must not reference mutable runtime backing storage.
- Do not use Godot `Resource` objects as runtime snapshots.
- Do not expose snapshot/result DTO public mutable vars.
- Do not provide setters or mutator methods on snapshot/result/row DTOs.
- Treat `_init()` and static factory methods as convenience, not security boundaries. Direct construction must still be safe or fail validation.
- Avoid public gameplay `Dictionary` outputs. If debug tooling needs a dictionary, assemble a new one on demand.
- Apply the same no-mutation rules to `AttributeStatQueryResult`, `AttributeResourceQueryResult`, `AttributeSnapshotQueryResult`, `AttributeDeltaSummaryRow`, source status summary DTOs, and any other snapshot-owned result object.
- Tests must verify that mutating returned copies, row DTOs, or result DTOs cannot alter authoritative runtime state or previously published snapshots.

## Alternatives Considered

### Alternative 1: Expose mutable runtime state or snapshot dictionaries directly

- **Description**: Return `Dictionary` / `Array` snapshots or direct runtime storage maps to consumers.
- **Pros**: Fast to prototype; easy for UI/debug inspection; low boilerplate.
- **Cons**: Consumers can mutate authoritative state or old snapshots; weak typing; easy to bypass validity/stale rules; contradicts ADR-0006 and the GDD snapshot immutability requirement.
- **Estimated Effort**: Low initial effort, high long-term correctness cost.
- **Rejection Reason**: It fails the core read-only boundary and would make combat/UI/save bugs hard to isolate.

### Alternative 2: Godot `Resource` snapshots

- **Description**: Represent each snapshot as a Godot `Resource` with exported fields, arrays, and nested resources.
- **Pros**: Inspector-friendly; serializable; familiar Godot data format.
- **Cons**: Resources are reference objects and may be shared/cached; exported fields are mutable; deep-copy behavior must be explicit; snapshots could become confused with authoring/config assets.
- **Estimated Effort**: Medium.
- **Rejection Reason**: ADR-0006 forbids mutable Resource objects as runtime attribute authority; snapshots need isolated runtime value semantics, not shared authoring resources.

### Alternative 3: Raw snapshot reference with no status wrapper

- **Description**: Consumers ask for `AttributeSnapshot` directly and inspect its fields/methods.
- **Pros**: Simple call sites; fewer DTOs.
- **Cons**: Consumers can ignore stale/invalid/failure context; failed rebuild fallback may be mistaken for current truth; error handling becomes ad hoc.
- **Estimated Effort**: Low.
- **Rejection Reason**: The GDD requires invalid/stale player-safe states and failed rebuild semantics. Status-bearing query wrappers make those states explicit and testable.

### Alternative 4: Full deep clone everything on every query

- **Description**: Every query returns a complete deep clone of the snapshot and all nested rows/results.
- **Pros**: Strong isolation from aliasing; simple mental model for consumers.
- **Cons**: Excess allocations; unnecessary for scalar hot-path reads; debug-heavy data risks becoming normal runtime payload; harder to keep 60 fps frame budget if queried frequently.
- **Estimated Effort**: Medium implementation effort, high runtime overhead.
- **Rejection Reason**: Defensive copying is required only at boundaries where mutable exposure is possible. Scalar getters and immutable-by-contract DTOs avoid unnecessary full deep clones.

### Alternative 5: Scene-tree Node or Autoload snapshot provider

- **Description**: Publish snapshots through a Node, Autoload singleton, or scene-tree signal source.
- **Pros**: Easy global access; integrates naturally with Godot scenes and signals.
- **Cons**: Breaks formula-only tests; introduces hidden dependencies and load-order issues; couples core attribute logic to scene tree; event contract belongs to a later ADR.
- **Estimated Effort**: Low initially, high testing/coupling cost.
- **Rejection Reason**: The GDD requires scene-tree-independent core logic and formula-only tests. Query API must be injectable/testable without Autoload.

## Consequences

### Positive

- Consumers get a clear, testable read-only access boundary.
- Stale/invalid/display-only states are explicit and cannot be silently dropped by ordinary provider calls.
- Published snapshots remain stable after later runtime mutation.
- The API supports combat/HUD/equipment/save/debug consumers without forcing them to know runtime storage internals.
- Debug trace remains pull-based and does not bloat normal snapshot payloads.

### Negative

- More DTOs and boilerplate are required.
- GDScript cannot compiler-enforce immutability; the project relies on API discipline, tests, code review, and registry forbidden-pattern checks.
- A careless internal implementer can still violate the contract by adding public vars, mutators, or exposed backing references.
- Defensive-copy and row DTO rules add test burden.
- Provider wrappers make call sites more explicit but more verbose than direct snapshot access.

### Neutral

- Exact source file paths and enum ownership are implementation details.
- Event/signal dispatch remains deferred to the next ADR.
- Snapshot query API does not decide save file serialization format.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| GDScript immutability is only contractual, not language-enforced. | Medium | High | Ban public vars/setters/mutators; code review against forbidden patterns; tests mutate returned data and prove authority/snapshot stability. |
| `RefCounted` snapshots/results are shared by reference. | Medium | High | Treat objects as immutable-by-contract; keep mutable authority outside snapshots; do not expose mutable nested objects. |
| `Array`/`Dictionary` copies are shallow by default. | Medium | Medium | Scalar arrays may shallow-copy; DTO arrays require cloned rows or immutable-by-contract row DTOs; debug dictionaries are assembled on demand. |
| Factory methods are mistaken for enforced private construction. | Medium | Medium | Make `_init()` safe or validating; do not rely on factories as the only enforcement boundary. |
| Result DTOs become mutable even if snapshots are read-only. | Medium | High | Apply no-public-var/no-setter/no-mutator rules to all snapshot/query/result/row DTOs. |
| Consumers bypass wrapper status and use stale snapshot as current truth. | Medium | High | Carry stale/invalid/display-only state in both wrapper and snapshot metadata; tests cover failed rebuild fallback behavior. |
| Debug trace grows into normal runtime snapshot payload. | Low | Medium | Keep debug trace lazy/pull-based or invalid/debug-only; register eager payload as forbidden pattern. |

## Performance Implications

| Metric | Before | Expected After | Budget |
|--------|--------|---------------|--------|
| CPU (frame time) | No implementation. Direct dictionary snapshots would be cheap initially but unsafe. | Scalar getters and stat/resource query DTOs are O(1). Defensive-copy collections allocate only when collection APIs are called. | Overall project frame budget 16.6 ms at 60 fps. HUD/combat should use cached snapshot references/results rather than rebuilding or polling full stats per frame. |
| Memory | No implementation. | One snapshot object per published version retained by policy; compact scalar/vector data; result DTO allocations per query where needed. No unbounded debug trace in normal snapshots. | Phase 1 client under 1 GB RAM. Snapshot retention policy must be bounded by later transaction/event design. |
| Load Time | No implementation. | No direct load-time impact beyond snapshot construction after load rebuild. | Acceptable for Phase 1. |
| Network | Not applicable for Phase 1 offline slice. | Not applicable. | None. |

## Migration Plan

No existing Character Attributes implementation needs migration.

1. Define snapshot/query/result/row DTO class files with getter-only public APIs.
2. Implement snapshot factory/construction from copied validated runtime state.
3. Implement `AttributeSnapshotProvider` or equivalent injectable service interface.
4. Implement stat/resource query results with ADR-0006 `StatId` validation.
5. Implement stale/invalid/display-only wrapper semantics for failed rebuild fallback.
6. Add tests proving returned scalar collections are defensive copies and DTO rows/results cannot mutate authority.
7. Add tests proving later runtime mutation does not alter previously published snapshots.
8. Defer event dispatch and snapshot retention policy to event/transaction ADRs.

**Rollback plan**: If `RefCounted` DTO overhead proves too high, supersede this ADR with a more compact no-allocation query path while preserving status-bearing wrappers, immutable-by-contract snapshot views, and no mutable backing exposure.

## Validation Criteria

- [ ] Public snapshot/query/result/row DTOs expose no public mutable vars, setters, or mutator methods.
- [ ] Snapshot construction copies mutable runtime arrays/dictionaries and does not reference runtime backing storage.
- [ ] Returned scalar arrays are defensive copies; mutating them does not affect the snapshot or runtime authority.
- [ ] Returned DTO arrays either clone rows or use immutable-by-contract row DTOs proven safe by tests.
- [ ] Failed rebuild keeps previous valid snapshot from becoming current truth and returns stale/display-only status when exposing it.
- [ ] Snapshot and wrapper both carry enough validity/stale status for consumers to avoid bypassing status semantics.
- [ ] Debug trace is absent from normal runtime snapshot payload and available only through explicit debug/invalid query path.
- [ ] GUT tests verify consumer mutation attempts cannot alter authoritative runtime state or previously published snapshots.

## GDD Requirements Addressed

| GDD Document | System | Requirement | How This ADR Satisfies It |
|-------------|--------|-------------|--------------------------|
| `design/gdd/character-attributes-system.md` | Character Attributes | `AttributeSnapshot` is an immutable/read-only semantic object; mutable nested `Dictionary` / `Array` snapshots passed directly to consumers are not acceptable. | Defines immutable-by-contract `RefCounted` snapshots, status-bearing query wrappers, getter-only APIs, and defensive-copy/row DTO collection rules. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Snapshot must include or be queryable for actor ID/type, version, schema/config version, source status, stats, resources, valid/invalid/stale state, and lightweight reason/changed metadata. | Defines required snapshot/provider getters and query result DTOs carrying version, validity, source status, stat/resource values, and changed/delta metadata. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Failed rebuild must not replace current valid snapshot; UI may show previous valid values only with stale/unavailable status; combat must not consume failed rebuild as current truth. | Requires failed rebuild query results to preserve previous valid snapshots only as stale/display-only and carry status in both wrapper and snapshot metadata. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Full debug source breakdown must be lazy or invalid/debug-only; normal runtime snapshots must not carry heavy formatted debug strings or unbounded history. | Makes debug trace pull-based/debug-only and registers eager debug payload as forbidden. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Implementation-blocking ADR item 2: snapshot/query API shape and read-only enforcement. | Directly resolves the query/snapshot API and read-only enforcement prerequisite while leaving events, transactions, save/load, and test setup as separate ADRs. |

## Related

- `design/gdd/character-attributes-system.md`
- `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`
- `docs/registry/architecture.yaml` — updated with snapshot query interface, read-only representation API decision, and forbidden patterns after this ADR.
