# ADR-0008: Attribute Event Signal Contract and Scene-Tree-Independent Core

## Status

Accepted

## Date

2026-06-04

## Last Verified

2026-06-04

## Decision Makers

hkm + Claude Code Game Studios

## Summary

This ADR defines how the Character Attributes system reports rebuild, resource, invalidation, and preview outcomes without coupling the core attribute runtime to Godot scene tree, Autoloads, or signal callback order. We choose scene-tree-independent `RefCounted` service/core classes whose authoritative update methods return a single typed `AttributeUpdateResult` envelope containing immutable-by-contract domain events; optional sinks and Godot signals are downstream notification adapters only after the transaction result is fully decided.

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Core / Scripting |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff and must be verified against engine reference docs. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md`; `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`; `docs/architecture/adr-0007-attribute-snapshot-query-api-shape-and-read-only-enforcement.md` |
| **Post-Cutoff APIs Used** | None required. This ADR relies on standard GDScript classes, typed variables/methods, typed arrays, `RefCounted`, optional `Node` signal adapters, and Callable/signal-property connection style. It does not require Godot 4.5 `@abstract`, variadic arguments, or any Godot 4.6-only API. Optional later `@abstract` use requires separate pinned-version verification and project style approval. |
| **Verification Required** | Verify `RefCounted` DTO aliasing/copy behavior, typed `Array` copy behavior, no public mutable DTO exposure, no SceneTree/Autoload dependency in core unit tests, deterministic event ordering, `AttributeEventSink` best-effort failure behavior, typed signal adapter connections, absence of deprecated string-based `connect("signal", obj, "method")`, and that listener presence/order cannot affect transaction status or snapshot/resource versions. |

> **Note**: If Knowledge Risk is MEDIUM or HIGH, this ADR must be re-validated if the project upgrades engine versions. Flag it as "Superseded" and write a new ADR.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | ADR-0006 Attribute Data Representation and Stat ID Typing; ADR-0007 Attribute Snapshot Query API Shape and Read-Only Enforcement; approved Character Attributes GDD: `design/gdd/character-attributes-system.md`. |
| **Enables** | Attribute core update/result API; `AttributeDomainEvent` DTO implementation; `AttributeEventSink` / no-op sink / fake sink test doubles; optional `AttributeSignalAdapter` Node; HUD/combat/equipment/save-load invalidation listeners; later atomic source update / transaction model ADR; formula-only GUT test strategy. |
| **Blocks** | Any story that emits or consumes `AttributeRebuildEvent`, `ResourceChangedEvent`, `AttributeInvalidatedEvent`, `AttributePreviewResult`, `AttributeUpdateResult`, or Godot signals derived from Character Attributes; any story that requires Character Attributes core to run outside SceneTree; any story that treats attribute event callback order as gameplay authority. |
| **Ordering Note** | ADR-0006 governs runtime stat identity. ADR-0007 governs snapshot/query truth. This ADR governs event/result publication and core/adapter separation. It does not decide source commit/rollback policy beyond requiring that transaction outcome, committed/rejected status, and versions be fully decided before sinks or signals are invoked. |

## Context

### Problem Statement

The Character Attributes GDD requires compact `AttributeRebuildEvent`, `ResourceChangedEvent`, `AttributeInvalidatedEvent`, and `AttributePreviewResult` semantics while also requiring the attribute core to be testable without scene tree or Autoload. HUD, combat, equipment, save/load, AI, growth feedback, and QA/debug tools need update/invalidation hints, but they must not become part of the attribute authority path.

Without this decision, implementers could put gameplay truth in Node signals, Autoload event buses, signal listener order, UI callbacks, or scene lifecycle. That would create hidden dependencies, race-like authority bugs, difficult GUT tests, and contradictory consumers that bypass ADR-0007 snapshot/query status semantics.

### Current State

No Character Attributes implementation exists yet. ADR-0006 establishes project-owned `StatId` identity, external semantic-key boundaries, typed DTO/source payload expectations, and forbidden mutable runtime representations. ADR-0007 establishes immutable-by-contract snapshots and status-bearing query result wrappers as the cross-system read boundary. The remaining gap is how attribute changes are reported to consumers without making dispatch mechanics authoritative.

### Constraints

- Godot 4.6.3 is post-cutoff; engine-sensitive signal and scripting assumptions must be checked against local reference docs.
- Core formula/update tests must instantiate Character Attributes services without adding Nodes to `SceneTree`.
- Attribute state, source validity, snapshot versions, resource versions, and failed rebuild semantics are owned by Character Attributes runtime, not listeners.
- Godot 4.x deprecates string-based connection style; adapter code must use typed signal / Callable style.
- GDScript has no engine-enforced immutable objects or traditional interfaces; contracts must be enforced by API shape, copying, code review, and tests.
- Events must be compact and coalesced; normal runtime events must not carry full old+new snapshots, mutable collections, or eager formatted debug trace.
- Consumers that need current truth must use ADR-0007 snapshot/query APIs.

### Requirements

- Return a single authoritative result envelope from every committed, rejected, no-op, or preview-like core update path where transaction status matters.
- Report data-first event DTOs only after the core transaction outcome is fully decided.
- Preserve ADR-0006 `StatId` and ADR-0007 snapshot/version/status semantics in event payloads.
- Let tests capture events deterministically without SceneTree, signals, or Autoload.
- Let Godot UI/presentation/integration layers adapt committed results into typed signals.
- Ensure listener presence, absence, order, or failure cannot affect transaction outcome, source commit/rollback outcome, or version publication.
- Keep preview results non-authoritative and outside committed event streams.

## Decision

Character Attributes core is scene-tree-independent. Its authoritative runtime services are injectable GDScript classes, normally `RefCounted` service/core objects, not `Node`, Autoload, or scene signal sources. Core code must not call `get_tree()`, depend on `NodePath`, use `_ready()` / `_process()` lifecycle, connect scene signals, or rely on Autoload globals to calculate attributes, decide validity, commit/rollback state, publish versions, or determine transaction results.

All Phase 1 authoritative update paths use one result envelope: `AttributeUpdateResult`. Structural rebuilds, resource-only mutations, invalid/rejected transactions, no-op transactions, and committed attribute updates all return `AttributeUpdateResult` with an `update_kind` / `result_state` concept. The envelope carries actor ID, transaction/update kind, result state, current or previous snapshot query metadata, source/config/version metadata, failure reasons if any, and a copied `Array[AttributeDomainEvent]`. Specialized convenience methods may exist, but they must return or wrap the same `AttributeUpdateResult` contract.

Domain events are data-first immutable-by-contract `RefCounted` DTOs. Required committed/diagnostic event categories are:

- `AttributeRebuildEvent` for successful committed structural rebuilds;
- `ResourceChangedEvent` for successful committed resource-only changes or resource corrections attached to a committed structural rebuild;
- `AttributeInvalidatedEvent` for failed rebuild/source/config failures or invalidation diagnostics.

`AttributePreviewResult` is a result DTO, not an `AttributeDomainEvent`, and must never appear in `Array[AttributeDomainEvent]`. Preview output may reuse delta/status DTO concepts, but it is non-authoritative and outside the committed event stream.

Event and result DTOs must expose getter/query methods only. They must not expose public mutable vars, setters, mutator methods, mutable `Array` / `Dictionary` references, mutable `Resource` payloads, or mutable snapshot/source collections. Collections passed into event/result DTO constructors are copied at construction. Collections returned by getters are copies. Mixed event streams use `Array[AttributeDomainEvent]`; shallow copy of the list is acceptable only because every event element also follows immutable-by-contract DTO rules. This is contractual immutability, not engine-enforced immutability, and must be verified by tests.

Event dispatch is explicit. The core always returns the fully materialized `AttributeUpdateResult`. It may also publish the same committed/diagnostic event set to an injected `AttributeEventSink` contract. Phase 1 standardizes the public sink contract as a typed `RefCounted` base class; a no-op sink and fake sink should support tests. If implementation wants Callable ergonomics, it must wrap the `Callable` in a concrete `CallableAttributeEventSink` adapter rather than changing the core contract. This ADR does not require Godot 4.5 `@abstract`.

Sink timing is binding: transaction outcome is fully decided, committed or rejected, versioned, and materialized into `AttributeUpdateResult` before any sink is invoked. Phase 1 sink publication is synchronous best-effort notification. Sink failure may be logged or reported through diagnostics, but must not alter the already-created result, committed state, snapshot version, resource version, source commit/rollback status, failure status, or event list.

Godot signals are allowed only in downstream adapter/presentation/integration layers after the core transaction has completed and returned its result. Signal adapters translate committed core results into typed Godot signals. Signals are notification/invalidation hints only. The presence, absence, order, or failure of signal callbacks must not affect attribute state, transaction status, source commit/rollback decision, published snapshot/resource version, combat truth, save truth, or growth eligibility.

Signal adapters may pass result/event DTO references only because those DTOs are immutable-by-contract. Adapters must not pass mutable core state, mutable snapshots, live source collections, or runtime backing arrays/dictionaries. Connections must use Godot 4 signal-property / Callable style such as `adapter.resource_changed.connect(_on_resource_changed)`, not deprecated string-based `connect("signal", obj, "method")`.

Consumers that need current authoritative values must read through ADR-0007 snapshot/query APIs. Events may carry compact before/after scalar values, deltas, changed masks/stat IDs, reason codes, versions, and player-safe status, but events do not replace the snapshot/query contract as full state transfer.

Event coalescing is required. One actor transaction/update point produces one compact `AttributeUpdateResult` envelope with deterministic event ordering. Coalescing reduces noise but must not hide required categories. Phase 1 ordering rules are:

1. Failed/rejected structural rebuilds or source/config failures produce failure status and at most one `AttributeInvalidatedEvent`; they must not emit committed-success events.
2. Successful structural rebuilds emit at most one `AttributeRebuildEvent` first.
3. If a successful structural rebuild also applies resource correction, such as max-resource clamp, related `ResourceChangedEvent` entries follow the rebuild event in deterministic resource registry order.
4. Resource-only transactions emit `ResourceChangedEvent` entries in deterministic resource registry order and do not emit `AttributeRebuildEvent`.
5. No-op transactions return a result envelope with no committed events unless diagnostic status is explicitly needed.

`AttributePreviewResult` is non-authoritative. It must not commit source changes, increment authoritative versions, replace snapshots, invoke sinks, or emit committed domain events. It must identify the actor ID and base snapshot/source/config version used for calculation so stale previews can be detected and recomputed.

Phase 1 does not introduce an Autoload event bus. If a later phase needs global cross-scene event routing, it requires a separate ADR and must remain downstream of this scene-tree-independent core and its result DTO contract.

### Architecture

```text
External source/resource/preview request
        |
        v
Character Attributes scene-tree-independent core
(RefCounted service; normalized registry; mutable authority)
        |
        | validate + calculate + commit/reject + publish versions
        v
AttributeUpdateResult / AttributePreviewResult
(immutable-by-contract DTOs; copied event arrays)
        |
        +--> optional AttributeEventSink
        |    (synchronous best-effort test/integration sink)
        |
        +--> optional AttributeSignalAdapter Node
        |    (typed Godot signals after result is materialized)
        |
        v
Consumers treat events/signals as hints
        |
        v
ADR-0007 snapshot/query APIs for current authoritative truth
```

### Key Interfaces

```gdscript
class_name AttributeDomainEvent
extends RefCounted

func get_actor_id() -> int:
    pass

func get_event_kind() -> int:
    pass

func get_actor_snapshot_version() -> int:
    pass

func get_reason_code() -> int:
    pass
```

```gdscript
class_name AttributeRebuildEvent
extends AttributeDomainEvent

func get_changed_stat_ids_copy() -> Array[int]:
    pass

func get_visible_delta_summary_copy() -> Array[AttributeDeltaSummaryRow]:
    pass

func get_hidden_delta_summary_copy() -> Array[AttributeDeltaSummaryRow]:
    pass

func get_growth_salience() -> int:
    pass

func get_validity_state() -> int:
    pass
```

```gdscript
class_name ResourceChangedEvent
extends AttributeDomainEvent

func get_resource_id() -> int:
    pass

func get_value_before() -> int:
    pass

func get_value_after() -> int:
    pass

func get_resource_max() -> int:
    pass

func get_delta() -> int:
    pass

func get_resource_reason() -> int:
    pass
```

```gdscript
class_name AttributeInvalidatedEvent
extends AttributeDomainEvent

func get_previous_valid_snapshot_version() -> int:
    pass

func get_failed_source_version() -> int:
    pass

func get_failure_reasons_copy() -> Array[int]:
    pass

func get_player_safe_status() -> int:
    pass
```

```gdscript
class_name AttributeUpdateResult
extends RefCounted

func is_success() -> bool:
    pass

func get_actor_id() -> int:
    pass

func get_update_kind() -> int:
    pass

func get_result_state() -> int:
    pass

func get_current_snapshot_query_result() -> AttributeSnapshotQueryResult:
    pass

func get_failure_reasons_copy() -> Array[int]:
    pass

func get_events_copy() -> Array[AttributeDomainEvent]:
    pass
```

```gdscript
class_name AttributePreviewResult
extends RefCounted

func is_success() -> bool:
    pass

func get_actor_id() -> int:
    pass

func get_base_snapshot_version() -> int:
    pass

func get_base_source_version() -> int:
    pass

func get_preview_deltas_copy() -> Array[AttributeDeltaSummaryRow]:
    pass

func get_invalid_reasons_copy() -> Array[int]:
    pass
```

```gdscript
class_name AttributeEventSink
extends RefCounted

func publish_attribute_events(actor_id: int, events: Array[AttributeDomainEvent]) -> void:
    pass
```

```gdscript
class_name AttributeSignalAdapter
extends Node

signal attribute_transaction_completed(actor_id: int, result: AttributeUpdateResult)
signal resource_changed(actor_id: int, event: ResourceChangedEvent)
signal attribute_invalidated(actor_id: int, event: AttributeInvalidatedEvent)

func publish_result(result: AttributeUpdateResult) -> void:
    pass
```

The `int` return/argument types above represent project-owned typed enum/ID concepts from ADR-0006 and ADR-0007, such as `StatId`, `ResourceId`, event kind, result state, failure reason, and validity state. If GDScript implements them as enum/int constants, boundaries must still validate raw integers before treating them as valid IDs.

Each `class_name` should live in its own `.gd` file. Interface sketches show conceptual contracts, not a required single-file layout.

### Implementation Guidelines

- Keep core classes instantiable in GUT without `SceneTree`, Nodes, Autoload, `_ready()`, `_process()`, or signal processing.
- Return `AttributeUpdateResult` from every authoritative structural rebuild, resource mutation, invalid/rejected transaction, and no-op transaction.
- Return `AttributePreviewResult` from preview queries; do not include preview output in committed event arrays.
- Copy incoming event/failure/delta arrays at DTO construction.
- Return copies from every collection getter, including `get_events_copy()` and failure/delta summary getters.
- Use `AttributeEventSink` as the single core sink contract; provide no-op and fake sinks for tests.
- Invoke sinks only after the result is fully materialized; sink publication is synchronous best-effort in Phase 1.
- Emit adapter signals only after the core returns a committed/rejected result.
- Do not connect signals in per-frame code.
- Do not retain unbounded event history in adapters or debug tools.
- Preserve deterministic event ordering by transaction kind, event category, resource registry order, and changed-stat registry order.
- Treat deferred UI signal emission as presentation-only. Deferred callbacks may carry committed result DTOs but must not alter transaction outcome.

## Alternatives Considered

### Alternative 1: Node-owned attribute component emits Godot signals as authority

- **Description**: Implement Character Attributes as a `Node` component that owns runtime state and emits Godot signals directly during rebuild/resource mutation.
- **Pros**: Natural Godot scene workflow; inspector-friendly; simple for UI to connect.
- **Cons**: Scene lifecycle, tree placement, signal callback order, and listener failures can become hidden dependencies; formula tests require SceneTree setup; hard to guarantee no listener mutates state mid-update.
- **Estimated Effort**: Low initial effort, high correctness/testing cost.
- **Rejection Reason**: The GDD requires scene-tree-independent core and explicitly warns that Godot signals may adapt events later but must not make core logic depend on scene tree or Autoload.

### Alternative 2: Autoload global event bus

- **Description**: Publish all attribute updates through a global Autoload event bus that consumers subscribe to.
- **Pros**: Convenient cross-system routing; easy for UI/debug tools to find; centralizes subscriptions.
- **Cons**: Global coupling, load-order risk, hidden dependencies, harder isolated tests, unnecessary for Phase 1 offline slice, tempting path for systems to bypass snapshot/query APIs.
- **Estimated Effort**: Medium initial effort.
- **Rejection Reason**: Phase 1 does not need global cross-scene event routing. If a later phase needs it, it must remain downstream of the core result DTO contract and be covered by a separate ADR.

### Alternative 3: Core emits Godot signals directly

- **Description**: Use a signal-capable object or Node-like service as the Character Attributes core and emit signals directly from update methods.
- **Pros**: Straightforward notifications; fewer result DTOs; idiomatic for many Godot gameplay scripts.
- **Cons**: Conflates transaction result with dispatch mechanism; complicates unit tests; risks listener-order authority; makes it harder to support no-listener deterministic tests.
- **Estimated Effort**: Low to medium.
- **Rejection Reason**: Returned result DTOs make transaction status testable and deterministic. Godot signals are still allowed, but only in adapters after result materialization.

### Alternative 4: Poll snapshots every frame with no events

- **Description**: Consumers ignore events and poll `AttributeSnapshotProvider` every frame or action tick.
- **Pros**: Minimal event API; no listener ordering issues.
- **Cons**: Wastes CPU; delays explicit invalidation handling; causes HUD/combat/save systems to invent local polling policies; contradicts GDD requirement for compact rebuild/resource/invalidation events.
- **Estimated Effort**: Low initial effort, medium runtime/integration cost.
- **Rejection Reason**: Consumers need compact update hints while ADR-0007 remains authoritative for truth. Event hints and snapshot queries serve different roles.

### Alternative 5: Full snapshot payload in every event

- **Description**: Include full old and new snapshots or full stat maps in every event so consumers never need to query current state.
- **Pros**: Self-contained events; simple listener code; useful for debugging.
- **Cons**: Memory/allocation bloat; stale-status bypass risk; normal runtime payload becomes debug-heavy; duplicates ADR-0007 snapshot/query role; makes event history tempting as state authority.
- **Estimated Effort**: Medium.
- **Rejection Reason**: The GDD requires compact events and lazy/pull-based debug trace. Events carry versions/deltas/status; consumers query snapshots when they need full truth.

### Alternative 6: Multiple specialized result envelope types

- **Description**: Return separate result classes such as `AttributeRebuildResult`, `AttributeResourceResult`, and `AttributeInvalidationResult` from different update methods.
- **Pros**: Narrower type names; each result can expose only relevant fields.
- **Cons**: Fragmented adapter/test logic; weak interface enforcement in GDScript; consumers must handle multiple parallel contracts; harder to guarantee identical event collection and status semantics.
- **Estimated Effort**: Medium.
- **Rejection Reason**: Phase 1 standardizes on one `AttributeUpdateResult` envelope for all authoritative update outcomes to keep adapters, tests, and downstream consumers consistent.

## Consequences

### Positive

- Core attribute logic remains testable without SceneTree, Autoload, or signal processing.
- Event semantics are deterministic, data-first, and aligned with the GDD.
- Godot signals remain idiomatic at UI/integration boundaries without owning gameplay truth.
- Consumers get efficient invalidation/update hints while ADR-0007 snapshot/query APIs stay authoritative.
- Fake/no-op sinks make unit tests straightforward.
- A single `AttributeUpdateResult` reduces API fragmentation.

### Negative

- More DTOs, sink classes, and adapter classes are required.
- GDScript cannot enforce interfaces or immutability fully; tests and code review must enforce the contract.
- Signal adapters must be carefully scoped to avoid becoming hidden global dependencies.
- Event coalescing and deterministic ordering add implementation/test work.
- Copying small arrays at DTO boundaries adds some allocation overhead.

### Neutral

- Node adapters may exist for UI, debug, and scene integration, but they depend on the core service.
- This ADR does not decide source commit/rollback policy, snapshot retention policy, save file schema, or global event bus architecture.
- Deferred presentation signal emission may be used later, but only with already-materialized committed/rejected result DTOs.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| Event/result DTO collection aliasing exposes mutable internals. | Medium | High | Copy incoming arrays/dictionaries at construction; getters return copies; event elements are immutable-by-contract; tests mutate source and returned arrays. |
| Sink or adapter failure is mistaken for transaction failure. | Medium | High | Transaction result is materialized before sink/signal publication; sink publication is best-effort; failure cannot alter committed state or result. |
| Consumers use events as full authoritative state. | Medium | High | Register event contract as hint/notification; current truth must come from ADR-0007 snapshot/query APIs; events carry versions/status only. |
| Signal callback order influences gameplay. | Medium | High | Signals emit only after commit/reject result; callback order cannot alter versions/state; tests cover zero/one/multiple/reordered listeners. |
| `AttributePreviewResult` is mistaken for committed event data. | Medium | Medium | Preview is explicitly not `AttributeDomainEvent`, never appears in event arrays, does not invoke sinks/signals, and carries base version for staleness checks. |
| Unbounded event/debug history causes memory growth. | Low | Medium | Normal adapters do not retain history; any debug history must be bounded or separately designed. |
| Future `@abstract` use creates post-cutoff dependency. | Low | Medium | ADR does not require `@abstract`; optional adoption requires pinned-version verification. |
| Multiple result types reappear during implementation. | Medium | Medium | Phase 1 binds all authoritative outcomes to `AttributeUpdateResult`; specialized helpers must wrap/return that contract. |

## Performance Implications

| Metric | Before | Expected After | Budget |
|--------|--------|---------------|--------|
| CPU (frame time) | No implementation. Poll-only approaches would push consumers toward repeated checks. | Event/result construction is O(changed stats/resources) per transaction; no per-frame rebuild or full stat polling required. Sink/signal adapters run only after update points. | Overall project frame budget 16.6 ms at 60 fps. Structural rebuilds must not run per frame; HUD should use events/cached snapshots. |
| Memory | No implementation. Full-event snapshots would allocate heavily. | Compact DTOs and copied small arrays per update; no full old+new snapshot payload; no unbounded debug trace/history. | Phase 1 client under 1 GB RAM. Debug history must be bounded if added. |
| Load Time | No implementation. | No meaningful load-time impact beyond events produced by load rebuild after future save/load ADR. | Acceptable for Phase 1. |
| Network | Not applicable for Phase 1 offline slice. | Not applicable. | None. |

## Migration Plan

No existing Character Attributes implementation needs migration.

1. Define event kind, update kind, result state, failure reason, and resource reason enum ownership in the same typed-ID style as ADR-0006.
2. Implement one `class_name` per DTO file: `AttributeDomainEvent`, `AttributeRebuildEvent`, `ResourceChangedEvent`, `AttributeInvalidatedEvent`, `AttributeUpdateResult`, and `AttributePreviewResult`.
3. Implement construction-time copy and getter-copy rules for all collection fields.
4. Implement `AttributeEventSink`, `NoOpAttributeEventSink`, `FakeAttributeEventSink`, and optional `CallableAttributeEventSink` adapter.
5. Make authoritative core update methods return `AttributeUpdateResult` for structural rebuild, resource mutation, invalid/rejected, and no-op outcomes.
6. Make preview methods return `AttributePreviewResult` and prove they do not enter committed event streams.
7. Implement optional `AttributeSignalAdapter` Node with typed signals and Callable/signal-property connections.
8. Add GUT tests for scene independence, copy/immutability, deterministic ordering, sink timing/failure behavior, listener-order independence, preview non-authority, and no deprecated connect usage.

**Rollback plan**: If sink dispatch is unnecessary, remove sink publication while preserving returned `AttributeUpdateResult` and event DTOs. If signal adapters are too early for Phase 1, defer `AttributeSignalAdapter` without changing the core result/event contract.

## Validation Criteria

- [ ] GUT tests instantiate Character Attributes core without Node, SceneTree, Autoload, `_ready()`, `_process()`, or signal processing.
- [ ] The same transaction result, state mutation, snapshot version, and resource version occur with zero, one, multiple, and reordered listeners/sinks.
- [ ] `AttributeUpdateResult` is the single Phase 1 authoritative result envelope for structural rebuilds, resource mutations, invalid/rejected outcomes, and no-op transactions.
- [ ] Sink publication occurs only after result materialization; sink failure does not change the result, committed state, versions, or failure status.
- [ ] `get_events_copy()` returns a copy; mutating it does not mutate the result.
- [ ] Mutating source arrays after DTO construction does not mutate event/result DTO state.
- [ ] Event/result DTOs expose no public mutable vars, setters, mutators, mutable `Dictionary` payloads, mutable `Resource` payloads, or mutable collection references.
- [ ] Fake sink captures deterministic event order; no-op sink is supported.
- [ ] Failed/rejected structural transactions emit no committed-success events.
- [ ] Successful structural rebuild/resource correction/resource-only event ordering matches this ADR's ordering rules.
- [ ] `AttributePreviewResult` does not increment authoritative version, replace snapshot, invoke sinks, or appear in `Array[AttributeDomainEvent]`.
- [ ] Signal adapters use typed signals and Callable/signal-property connections, not string-based `connect("signal", obj, "method")`.
- [ ] Consumers that need current truth use ADR-0007 snapshot/query APIs rather than relying on event payloads as full state.

## GDD Requirements Addressed

| GDD Document | System | Requirement | How This ADR Satisfies It |
|-------------|--------|-------------|--------------------------|
| `design/gdd/character-attributes-system.md` | Character Attributes | Events are data-first domain events; Godot signals may adapt them later, but core attribute logic must be testable without scene tree or Autoload. | Defines returned `AttributeUpdateResult` / domain event DTOs as the core contract; permits Godot signals only in downstream adapters after result materialization. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Required event categories include `AttributeRebuildEvent`, `ResourceChangedEvent`, `AttributeInvalidatedEvent`, and `AttributePreviewResult`. | Defines rebuild/resource/invalidation domain events and clarifies preview as a non-authoritative result DTO outside committed event streams. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Events must be coalesced per actor transaction/update point; UI, combat, AI, and debug tools must not rely on callback order as gameplay authority. | Requires one compact result envelope per actor update point, deterministic event ordering, and forbids listener/signal order from changing state, versions, or transaction results. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Resource-only HP/MP changes emit compact `ResourceChangedEvent` without full structural rebuild. | Defines resource-only transactions as `AttributeUpdateResult` with `ResourceChangedEvent` entries and no `AttributeRebuildEvent`. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Failed rebuilds do not replace current valid snapshot and expose structured failure evidence. | Defines failed/rejected outcomes with failure status and at most one `AttributeInvalidatedEvent`, with no committed-success events. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Full old+new snapshot payloads and formatted debug traces are debug-only or pull-based; HUD should update from events or cached snapshots, not rebuild/poll full stats every frame. | Keeps events compact, forbids full old+new snapshots/eager debug trace in normal events, and directs current truth reads to ADR-0007 snapshot/query APIs. |
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-15 requires monotonic versions, one rebuild event after successful rebuild, compact resource event without full rebuild, failed rebuild not replacing current version, and no full old+new snapshots in normal runtime path. | Provides the event/result API, coalescing rules, ordering rules, and validation criteria needed to implement AC-15. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Implementation-blocking ADR item 3: event/signal contract and scene-tree-independent core. | Directly resolves the event/signal contract and scene-tree-independent core prerequisite while leaving transaction commit/rollback, save/load, fixture loading, and test strategy as separate ADRs/designs. |

## Related

- `design/gdd/character-attributes-system.md`
- `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`
- `docs/architecture/adr-0007-attribute-snapshot-query-api-shape-and-read-only-enforcement.md`
- `docs/registry/architecture.yaml` — updated with event/result contracts, scene-tree-independent core API stance, and forbidden patterns after this ADR.
