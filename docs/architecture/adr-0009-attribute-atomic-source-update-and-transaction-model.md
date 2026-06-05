# ADR-0009: Attribute Atomic Source Update and Transaction Model

## Status

Accepted

## Date

2026-06-04

## Last Verified

2026-06-04

## Decision Makers

hkm + Claude Code Game Studios

## Summary

This ADR defines how Character Attributes applies structural source updates and current-resource mutations without exposing intermediate or partially failed state. We choose actor-local synchronous transactions that build fully validated candidate state before a final commit/swap, with expected version checks, deterministic failure/no-side-effect semantics, explicit current-resource correction policy, and reentrant mutation rejection during transaction and result dispatch.

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Core / Scripting |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff and must be verified against engine reference docs. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md`; `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`; `docs/architecture/adr-0007-attribute-snapshot-query-api-shape-and-read-only-enforcement.md`; `docs/architecture/adr-0008-attribute-event-signal-contract-and-scene-tree-independent-core.md` |
| **Post-Cutoff APIs Used** | None required. This ADR relies on standard GDScript classes, typed variables/methods, typed arrays, `RefCounted`, explicit result objects, and synchronous service calls. It does not require `@abstract`, threads, deferred calls, timers, or SceneTree APIs. |
| **Verification Required** | Verify `RefCounted` / `Array` / `Dictionary` request and staged-state copy behavior, no caller-owned mutable aliasing, no SceneTree/Autoload dependency, no `await` / timers / `call_deferred()` / signal emission before result materialization, synchronous signal/result-dispatch reentrancy guards, source/snapshot version counters, candidate state not leaking through published snapshots/results, and no-side-effect rejection on version mismatch or validation failure. |

> **Note**: If Knowledge Risk is MEDIUM or HIGH, this ADR must be re-validated if the project upgrades engine versions. Flag it as "Superseded" and write a new ADR.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | ADR-0006 Attribute Data Representation and Stat ID Typing; ADR-0007 Attribute Snapshot Query API Shape and Read-Only Enforcement; ADR-0008 Attribute Event Signal Contract and Scene-Tree-Independent Core; approved Character Attributes GDD: `design/gdd/character-attributes-system.md`. |
| **Enables** | Attribute transaction implementation; equipment replacement atomicity; progression/source update path; spawn/load/debug source update path; current-resource mutation path; rollback/no-side-effect formula tests; save/load persistence boundary ADR; fixture/config loading strategy. |
| **Blocks** | Any story that commits, rejects, rolls back, or validates structural Character Attributes source changes; any story that mutates current HP/MP through Character Attributes; any equipment/progression/load/spawn/debug story that changes attribute source sets; any story that depends on `source_version` / `snapshot_version` transaction semantics. |
| **Ordering Note** | ADR-0006 defines stat/source identity. ADR-0007 defines published snapshot/query read truth. ADR-0008 defines result/event dispatch after materialization. This ADR defines the actor-local transaction boundary that produces those snapshots/results/events. Save file serialization and fixture/config file formats remain separate ADRs. |

## Context

### Problem Statement

The Character Attributes GDD requires equipment replacement, level-up, load rebuild, spawn initialization, resource mutation, failed rebuild handling, and growth feedback to behave transaction-like. Consumers must never observe an intermediate state where old and new equipment both contribute, removed equipment still contributes after success, a failed rebuild is celebrated as growth, or combat consumes a stale/failed rebuild as current truth.

Without this decision, implementation could mutate committed source sets before validation succeeds, attempt unreliable manual rollback, let UI or signal callbacks trigger nested updates, ignore stale command versions, or treat current HP/MP mutations as full structural rebuilds. These failures would contradict ADR-0006 state ownership, ADR-0007 snapshot validity semantics, and ADR-0008 event/result ordering.

### Current State

No Character Attributes implementation exists. ADR-0006 establishes that Character Attributes owns runtime state and source/modifier representation. ADR-0007 establishes that failed rebuilds do not replace current valid snapshots and that stale/display-only status must be explicit. ADR-0008 establishes one `AttributeUpdateResult` envelope and forbids signal callback order from being gameplay authority. This ADR fills the missing commit/reject/version boundary.

### Constraints

- Character Attributes core must be scene-tree-independent and synchronous in Phase 1.
- Runtime source sets, derived/effective values, current resource state, snapshots, and version counters are authoritative only inside Character Attributes.
- GDScript has reference-semantics classes and mutable arrays/dictionaries; staged transactions must not retain caller-owned mutable references.
- GDScript does not provide transactional memory. Atomicity must be simulated through candidate state construction and final reference/value swap, not by mutating then rolling back.
- Godot signals are synchronous by default; mutation during transaction or result dispatch must be guarded deterministically.
- Current HP/MP values are part of the published attribute snapshot in Phase 1.
- Structural rebuilds must not run per frame; current-resource mutations must not reaggregate all modifiers.

### Requirements

- Apply structural source changes atomically per actor.
- Reject failed structural changes with no side effects on committed source set, source version, snapshot version, current snapshot, or current resources.
- Publish successful structural changes as one coherent new committed state and one coherent `AttributeUpdateResult`.
- Support cheap current-resource mutation without source aggregation.
- Detect stale commands using explicit version expectations.
- Prevent nested/reentrant mutation during transaction and result dispatch.
- Preserve read-only snapshot/result/event boundaries from ADR-0007 and ADR-0008.

## Decision

Character Attributes structural rebuilds use actor-local synchronous transactions. A structural transaction follows this logical flow:

1. receive typed request;
2. reject mutation if actor is not in an allowed transaction state;
3. check version expectations;
4. normalize and copy inbound request/source DTOs;
5. stage a candidate source set;
6. validate source/config/input;
7. calculate candidate derived/effective stats and resource corrections;
8. output-validate candidate snapshot/current values;
9. build immutable-by-contract candidate snapshot/result/event payloads;
10. perform final commit/swap;
11. return `AttributeUpdateResult`;
12. invoke optional sinks/signals per ADR-0008 only after result materialization and commit/reject status are decided.

External systems never mutate committed runtime source sets directly. Equipment, progression, load, spawn, and debug systems submit typed request DTOs such as `AttributeTransactionRequest`, `AttributeSourceUpdateRequest`, or `AttributeCurrentResourceMutationRequest` to the Character Attributes core. The core owns normalization, staging, validation, version checks, commit/reject, snapshot publication, and result materialization.

Runtime maintains actor-local `source_version` and `snapshot_version`:

- `source_version` advances exactly once only on a committed structural source transaction.
- `snapshot_version` advances exactly once when a new current valid published snapshot/resource observable state is created.
- Failed or rejected transactions advance neither version.
- Current-resource mutations advance `snapshot_version` exactly once in Phase 1 and never advance `source_version`.

Current HP/MP values are part of the published ADR-0007 attribute snapshot in Phase 1. Therefore damage, heal, spend, restore, and current-resource corrections publish a new snapshot version even when they do not reaggregate modifiers or change structural sources. Internally, implementations may store current resources separately from dense effective stat vectors, but published snapshots include both effective max values and current resource values.

Implementation must not write committed source sets, current resources, snapshots, or version counters before candidate state is fully built. The safe GDScript strategy is candidate construction plus final reference/value swap:

```text
copy request -> build candidate source/current/snapshot/result payloads
-> if any validation/materialization fails, return failure with committed state unchanged
-> final commit/swap committed source/current/snapshot/version references
-> return immutable result with committed post-transaction versions
-> dispatch optional sinks/signals
```

Candidate result/events may be prepared before commit, but must not be externally observable before commit. Any recoverable failure must occur before the final commit/swap boundary. After the final commit/swap boundary, Phase 1 treats failure as an internal invariant violation, not as a recoverable transaction rejection. Code must therefore construct all required immutable snapshot/result/event payloads before swapping authoritative references.

Failed structural transactions reject staged source changes. They keep the committed source set, `source_version`, `snapshot_version`, current snapshot, and current resources unchanged. They return an `AttributeUpdateResult` failure and may expose the previous valid snapshot only as stale/display-only metadata per ADR-0007. They must not emit committed-success events.

Successful structural transactions atomically publish the new committed source set, derived/effective values, any allowed current-resource corrections, current valid snapshot reference, version counters, and `AttributeUpdateResult`. Consumers never observe intermediate candidate state. Reads during transaction return the last committed snapshot only. Reads during result dispatch return the newly committed snapshot if commit succeeded, or the unchanged previous committed snapshot if the transaction failed. Reads never expose candidate staged state.

Current-resource mutation means HP/MP current-value mutation, not Godot `Resource` mutation. It uses `AttributeCurrentResourceMutationRequest` and is a lightweight transaction path:

- validate `expected_snapshot_version` against the current committed snapshot;
- validate request reason and resource ID;
- calculate new current value against latest effective max/current bounds;
- apply the request's correction policy;
- publish a new snapshot with updated current resource values;
- advance `snapshot_version` exactly once;
- do not advance `source_version`;
- emit `ResourceChangedEvent` through the ADR-0008 result/event contract;
- do not reaggregate structural modifiers.

Phase 1 current-resource correction policy is:

| Request Kind | Default Policy | Meaning |
|--------------|----------------|---------|
| `damage` | `CLAMP_TO_BOUNDS` | Negative delta may reduce current value to floor, normally `0`, but not below. |
| `heal` | `CLAMP_TO_BOUNDS` | Positive delta may restore current value up to effective max, but not above. |
| `spend` | `REJECT_IF_AFFORDABILITY_FAILS_THEN_CLAMP` | Request must satisfy affordability if configured; resulting value clamps within bounds. |
| `restore` | `CLAMP_TO_BOUNDS` | Positive restoration clamps to effective max. |
| `debug_current_resource` | Explicit per request | Debug request must declare `CLAMP_TO_BOUNDS` or `REJECT_OUT_OF_BOUNDS`. |
| `load_correction` | Deferred to save/load ADR | Until save/load ADR, load correction cannot silently become normal gameplay mutation. |
| `spawn_initialization` | `REJECT_OUT_OF_BOUNDS` unless fixture policy says clamp | Spawn fixtures should be valid; correction policy must be explicit. |

Structural max-resource changes that require current-resource clamp are part of the structural transaction, not a current-resource-only transaction. For example, max HP decreasing below current HP may clamp current HP during structural rebuild according to the GDD resource correction policy and emit resource-change data after the rebuild event, but it still advances `source_version` because structural sources changed.

Equipment replacement is represented as one atomic source delta containing removals and additions. Consumers never observe old and new equipment both active, neither active, removed equipment still contributing after success, or new equipment shown as equipped while attributes still use old modifiers without stale/error status. Replacement validates the final candidate source set, not intermediate remove/add order. Source keys must be canonicalized before duplicate detection. Adding duplicate source keys fails. Removing a missing source key fails by default unless the request explicitly declares idempotent no-op removal policy.

Requests carry version expectations. Version mismatch rejects before staging/calculation and has no side effects. Phase 1 defaults are:

| Request Kind | Requires `expected_source_version` | Requires `expected_snapshot_version` | Default Mismatch Behavior |
|--------------|-----------------------------------|-------------------------------------|---------------------------|
| Equipment equip/unequip/replace | Yes | Optional; required if command is based on displayed/calculated stats | Reject with stale version failure. |
| Progression / level-up / base stat source update | Yes | Optional | Reject with stale source failure. |
| Spawn initialization for new actor | No if actor has no prior state and policy is `INITIALIZE_NEW_ACTOR` | No | Initialize or reject if actor already exists. |
| Load authoritative replace | No if policy is `LOAD_REPLACE_AUTHORITY` and future save/load ADR permits it | Future save/load ADR decides | Replace only under explicit load/migration policy. |
| Debug structural mutation | Yes unless explicit debug override policy is supplied | Optional | Reject or audit override. |
| Damage/heal/spend/restore/current-resource mutation | No | Yes | Reject with stale snapshot failure. |
| Preview query | No structural commit | Yes for preview based on current values | Return stale/invalid preview result, no side effects. |

`request_id` is audit/traceability-only in Phase 1. The runtime does not cache completed request IDs or replay prior results. It may reject a duplicate active request ID for the same actor while a transaction is in progress. Durable command idempotency or retry semantics require a future scheduler/command ADR.

No nested or reentrant mutation is allowed in Phase 1. The transaction service has at least these internal state concepts:

- `IDLE`
- `TRANSACTION_ACTIVE`
- `MATERIALIZING_RESULT`
- `DISPATCHING_RESULT`

During `TRANSACTION_ACTIVE`, `MATERIALIZING_RESULT`, or `DISPATCHING_RESULT`, new mutation requests reject deterministically with `BUSY_TRANSACTION_ACTIVE`, `REENTRANT_MUTATION_REJECTED`, or `MUTATION_DURING_RESULT_DISPATCH_REJECTED`. This includes mutation attempts triggered by event sinks, Godot signal callbacks, UI listeners, debug listeners, or growth feedback callbacks. Future queued follow-up mutations require a separate scheduler/command queue ADR.

Phase 1 transactions are synchronous. Transaction code must not `await`, use timers, call deferred scene-tree callbacks, emit Godot signals, or yield before result materialization and final commit/reject status are complete. Validators and calculators must return typed success/failure objects; transaction code must not rely on exceptions or engine errors for rollback semantics.

Debug mutation goes through the same transaction path. It must carry debug source system, reason label, operator/test context if available, expected version policy, source status labels, and player-safe debug-only result status. Debug mutation must be auditable and distinguishable from normal gameplay source updates.

### Architecture

```text
External system request
(equipment / progression / spawn / load / debug / combat resource mutation)
        |
        v
AttributeTransactionRequest / AttributeCurrentResourceMutationRequest
        |
        v
Character Attributes transaction core
        |
        +--> state guard + version expectation check
        +--> normalize/copy inbound DTOs
        +--> build candidate source/current state
        +--> validate source/config/input
        +--> calculate candidate effective stats/resource corrections
        +--> build candidate snapshot/result/events
        |
        v
Final commit/swap boundary
        |
        +--> success: swap committed source/current/snapshot/version refs
        +--> failure before swap: committed state unchanged
        |
        v
AttributeUpdateResult
        |
        v
ADR-0008 sinks/signals after result materialization
```

### Key Interfaces

```gdscript
class_name AttributeTransactionRequest
extends RefCounted

func get_actor_id() -> int:
    pass

func get_request_id() -> StringName:
    pass

func get_request_kind() -> int:
    pass

func get_version_expectation() -> AttributeVersionExpectation:
    pass

func get_source_delta() -> AttributeSourceDelta:
    pass

func get_reason_code() -> int:
    pass

func get_source_system() -> StringName:
    pass
```

```gdscript
class_name AttributeSourceDelta
extends RefCounted

func get_removals_copy() -> Array[AttributeModifierSourceKey]:
    pass

func get_additions_copy() -> Array[AttributeSourcePayload]:
    pass

func get_replacements_copy() -> Array[AttributeSourceReplacement]:
    pass

func get_removal_idempotency_policy() -> int:
    pass
```

```gdscript
class_name AttributeCurrentResourceMutationRequest
extends RefCounted

func get_actor_id() -> int:
    pass

func get_request_id() -> StringName:
    pass

func get_resource_id() -> int:
    pass

func get_expected_snapshot_version() -> int:
    pass

func get_delta_value() -> int:
    pass

func get_target_value() -> int:
    pass

func get_mutation_reason() -> int:
    pass

func get_correction_policy() -> int:
    pass
```

```gdscript
class_name AttributeVersionExpectation
extends RefCounted

func get_expected_source_version() -> int:
    pass

func get_expected_snapshot_version() -> int:
    pass

func get_policy() -> int:
    pass
```

```gdscript
class_name AttributeTransactionStateIds
extends RefCounted

enum TransactionState {
    IDLE,
    TRANSACTION_ACTIVE,
    MATERIALIZING_RESULT,
    DISPATCHING_RESULT,
}
```

```gdscript
class_name AttributeTransactionFailureReasonIds
extends RefCounted

enum FailureReason {
    STALE_SOURCE_VERSION,
    STALE_SNAPSHOT_VERSION,
    BUSY_TRANSACTION_ACTIVE,
    REENTRANT_MUTATION_REJECTED,
    MUTATION_DURING_RESULT_DISPATCH_REJECTED,
    INVALID_SOURCE_DELTA,
    DUPLICATE_SOURCE_KEY,
    UNKNOWN_SOURCE_KEY,
    UNKNOWN_STAT_ID,
    UNSUPPORTED_MODIFIER_OPERATION,
    CALCULATION_FAILED,
    OUTPUT_VALIDATION_FAILED,
    CURRENT_RESOURCE_OUT_OF_BOUNDS,
    SNAPSHOT_PUBLICATION_FAILED,
}
```

All request/result/source DTOs follow ADR-0006, ADR-0007, and ADR-0008 immutable-by-contract copy rules. Interface sketches are conceptual; exact enum ownership and filenames may be refined during implementation as long as the contract remains one-class-per-file and typed.

### Implementation Guidelines

- Normalize all inbound request DTOs into internally owned staged DTO/value containers before validation or calculation.
- Do not retain references to caller-owned mutable arrays, dictionaries, Resources, or nested DTOs unless the nested DTO is already immutable-by-contract and safe to share.
- Copy source/modifier collections element-by-element into staged payloads or scalar vectors.
- Validate version expectations before building expensive candidate state.
- Build candidate snapshot/result/event payloads before final commit/swap.
- Do not expose candidate state through snapshot/query APIs, events, signals, debug tools, or logs that consumers treat as authority.
- Reads during transaction return last committed state only; reads during dispatch return post-commit state only.
- Use typed success/failure returns from validators/calculators; do not rely on exceptions for rollback.
- Keep the transaction service scene-tree-independent; do not call `get_tree()`, `call_deferred()`, timers, or signals during active transaction.
- Guard mutation entry points with transaction state checks.

## Alternatives Considered

### Alternative 1: Immediate source mutation with manual rollback on failure

- **Description**: Apply source changes directly to committed runtime state, run validation/calculation, then restore old state if something fails.
- **Pros**: Simple initial code; fewer candidate data structures.
- **Cons**: GDScript lacks transactional memory; rollback paths are easy to miss; events/debug reads may observe partial state; nested reference aliasing makes restoration fragile.
- **Estimated Effort**: Low initial effort, high correctness risk.
- **Rejection Reason**: The GDD requires consumers to never observe intermediate states. Candidate construction plus final swap is safer and testable.

### Alternative 2: Equipment/progression systems own commit; attributes only calculate

- **Description**: Equipment, progression, load, and spawn mutate their own source state and ask attributes to calculate snapshots from whatever is currently equipped/loaded.
- **Pros**: Source systems keep their domain data close; fewer requests into attributes.
- **Cons**: Contradicts registered `attribute_runtime_state` ownership; creates cross-system source-of-truth ambiguity; makes failed rebuild rollback and stale status inconsistent.
- **Estimated Effort**: Medium.
- **Rejection Reason**: Attribute runtime source inputs and modifier aggregation state are owned by Character Attributes per ADR-0006 registry stance.

### Alternative 3: Allow callback-triggered follow-up mutations immediately after commit

- **Description**: Once commit succeeds, allow event sinks, Godot signal listeners, UI callbacks, or debug listeners to synchronously submit another mutation in the same call stack.
- **Pros**: Convenient chained behavior; responsive UI-driven follow-ups.
- **Cons**: Reentrant call stacks are hard to debug; callback order can influence mutation ordering; contradicts ADR-0008's signal-order-not-authority stance.
- **Estimated Effort**: Medium.
- **Rejection Reason**: Phase 1 favors deterministic rejection. Follow-up mutation queues require a separate scheduler/command ADR.

### Alternative 4: Global queue or Autoload transaction scheduler now

- **Description**: Route all attribute source/resource commands through a global queue or Autoload scheduler that serializes transactions.
- **Pros**: Natural place for idempotency, retries, delayed commands, and cross-actor ordering.
- **Cons**: Over-scope for Phase 1; adds global coupling; conflicts with scene-tree-independent core goals; requires separate ordering semantics.
- **Estimated Effort**: High.
- **Rejection Reason**: Actor-local synchronous transactions are sufficient for Phase 1. Scheduler/queue design can be added later if needed.

### Alternative 5: Ignore version expectations because Phase 1 is single-threaded

- **Description**: Accept any command against the actor's current state and rely on single-threaded execution order.
- **Pros**: Fewer fields in requests; simpler callers.
- **Cons**: Stale UI/equipment/save/debug commands can apply to unexpected state; harder deterministic replay/debug; future async/network evolution becomes riskier.
- **Estimated Effort**: Low.
- **Rejection Reason**: Expected version tokens are cheap and provide deterministic stale command rejection even in single-threaded Phase 1.

### Alternative 6: Separate resource version instead of snapshot version for HP/MP changes

- **Description**: Keep `snapshot_version` only for structural rebuilds and introduce `resource_version` for current HP/MP changes.
- **Pros**: Avoids frequent snapshot version increments; separates structural and resource change identity.
- **Cons**: Adds another version axis before needed; complicates ADR-0007 snapshot truth; consumers must compare two versions.
- **Estimated Effort**: Medium.
- **Rejection Reason**: Phase 1 snapshots include current resources, so HP/MP changes legitimately publish new snapshot versions. A separate `resource_version` can be introduced later if profiling or network replication needs it.

## Consequences

### Positive

- Consumers never observe old+new equipment, neither-equipment, partial source commits, or failed rebuilds as successful growth.
- Failed structural transactions leave committed source/runtime/snapshot state unchanged.
- Current-resource mutations are cheaper than structural rebuilds and do not reaggregate modifiers.
- Stale UI/equipment/debug/save commands fail deterministically.
- Debug mutations remain auditable and follow the same validation path.
- The model stays compatible with ADR-0008 event/result dispatch and ADR-0007 read-only snapshots.

### Negative

- More request/version/source-delta DTOs are required.
- Callers must carry expected version data.
- Reentrant callback-driven follow-up mutation is rejected until a future scheduler exists.
- Candidate copying and snapshot/result pre-materialization add allocation overhead.
- Some load/spawn/debug policies remain explicit and cannot silently bypass validation.

### Neutral

- `request_id` is traceability-only in Phase 1.
- Current-resource mutations advance `snapshot_version` in Phase 1; a separate resource version may be added later if needed.
- Save/load authoritative replace policy is intentionally deferred to the save/load ADR.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| Staged DTO aliasing lets caller mutation affect candidate state. | Medium | High | Normalize/copy inbound DTOs and collections; tests mutate caller DTOs after submission. |
| Partial commit occurs before snapshot/result materialization failure. | Medium | High | Build candidate snapshot/result before final commit/swap; recoverable failures must occur before commit boundary. |
| Current-resource policy diverges across damage/heal/spawn/load/debug. | Medium | Medium | Bind Phase 1 correction policy table in this ADR; defer load migration only to save/load ADR. |
| Version expectation rules confuse callers. | Medium | Medium | Provide request-kind table and validation tests for each request type. |
| Reentrant mutation rejection blocks useful chained behavior. | Medium | Low | Accept for Phase 1 determinism; future scheduler ADR can add queued follow-ups. |
| Snapshot version churn on frequent HP changes. | Medium | Low for Phase 1 | Phase 1 simplicity favors one snapshot version; revisit only after profiling or network requirements. |
| Debug override bypasses normal validation. | Medium | Medium | Debug mutation uses same transaction path and must carry audit labels/version policy. |
| Reads accidentally expose candidate state. | Low | High | Snapshot/query APIs read only committed references; tests read during fake transaction states. |

## Performance Implications

| Metric | Before | Expected After | Budget |
|--------|--------|---------------|--------|
| CPU (frame time) | No implementation. Immediate mutation could be cheap but unsafe. | Structural transactions are O(M + S) for active modifiers/stats and must not run per frame. Current-resource mutations are O(1) or O(number of changed resources) and do not reaggregate modifiers. | Overall frame budget 16.6 ms at 60 fps. Structural rebuilds must be event/update-point driven, not per-frame. |
| Memory | No implementation. | Candidate source/current/snapshot/result DTOs allocate during transactions; committed state uses reference/value swap. Events remain compact. | Phase 1 client under 1 GB RAM. No unbounded transaction/result history. |
| Load Time | No implementation. | Spawn/load initialization runs validation and candidate construction; acceptable for Phase 1. | Load/save policy refined in later ADR. |
| Network | Not applicable for Phase 1 offline slice. | Not applicable. Version tokens may help future networking but no network behavior is selected. | None. |

## Migration Plan

No existing Character Attributes implementation needs migration.

1. Define request kind, version policy, correction policy, transaction state, and failure reason enum ownership.
2. Implement immutable-by-contract request DTOs: `AttributeTransactionRequest`, `AttributeSourceDelta`, `AttributeVersionExpectation`, and `AttributeCurrentResourceMutationRequest`.
3. Implement staged source/current state builders that copy inbound data and canonicalize source keys.
4. Implement structural transaction flow with version expectation check, candidate validation/calculation, candidate snapshot/result materialization, final commit/swap, and ADR-0008 result dispatch.
5. Implement current-resource mutation flow with explicit correction policies and snapshot version publication.
6. Implement transaction state/reentrancy guard.
7. Add tests for no-side-effect failure, version mismatch, equipment replacement atomicity, current-resource mutation, debug mutation audit labels, read behavior during transaction/dispatch, and request DTO aliasing.

**Rollback plan**: If the single `snapshot_version` for current-resource changes becomes too noisy, supersede this ADR with a separate `resource_version` model while preserving atomic source commit, no-side-effect failure, and staged candidate construction.

## Validation Criteria

- [ ] Failed structural transaction leaves committed source set, `source_version`, `snapshot_version`, current snapshot, and current resources unchanged.
- [ ] Successful structural transaction advances `source_version` exactly once and `snapshot_version` exactly once.
- [ ] Successful current-resource mutation advances `snapshot_version` exactly once and never advances `source_version`.
- [ ] Current-resource mutation publishes current HP/MP values through the ADR-0007 snapshot/query read model.
- [ ] Version mismatch rejects before staging/calculation and has no side effects.
- [ ] `request_id` is recorded for traceability and does not imply completed-request replay/idempotency in Phase 1.
- [ ] Caller mutation of request/source DTOs after submission cannot alter staged transaction behavior.
- [ ] Published snapshot, `AttributeUpdateResult`, and event DTOs do not reference staged mutable buffers.
- [ ] Equipment replacement delta never exposes old+new active or neither active to consumers.
- [ ] Duplicate source keys fail after source key canonicalization.
- [ ] Remove-missing behavior follows explicit policy; Phase 1 default is failure.
- [ ] Reentrant mutation during transaction/materialization/dispatch returns deterministic failure.
- [ ] Reads during transaction expose only last committed state; reads during dispatch expose only post-commit or unchanged committed state, never candidate state.
- [ ] No `await`, timer, deferred scene-tree callback, or signal emission occurs before result materialization.
- [ ] Debug mutation uses the same transaction validation path and carries debug/audit labels.
- [ ] Structural max-resource clamp is tested as part of structural transaction, not current-resource-only mutation.

## GDD Requirements Addressed

| GDD Document | System | Requirement | How This ADR Satisfies It |
|-------------|--------|-------------|--------------------------|
| `design/gdd/character-attributes-system.md` | Character Attributes | Atomic Source Update Rule: equipment replacement, level-up, load rebuild, and spawn initialization must be transaction-like; consumers must never observe intermediate old/new modifier states. | Defines actor-local staged structural transactions with final commit/swap and no candidate-state exposure. |
| `design/gdd/character-attributes-system.md` | Character Attributes | If rebuild fails after a staged change, source commit/rollback policy must be defined; combat must not consume failed rebuild as current truth. | Failed transactions reject staged source changes, do not advance versions, do not replace snapshots, and return failure results with stale/display-only previous snapshot metadata. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Structural rebuild path stages inputs, validates, aggregates, computes, validates output, publishes one snapshot/event if successful, and keeps previous snapshot on failure. | Formalizes the structural transaction phases and commit boundary that implement this path. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Resource mutation path validates HP/MP requests, uses latest valid max, clamps by reason policy, publishes resource state/version, and does not reaggregate modifiers. | Defines `AttributeCurrentResourceMutationRequest`, Phase 1 correction policy, snapshot version publication, and no `source_version` advancement for current-resource mutations. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Equipment Modifier Boundary and Atomic Rebuild AC-09: accepted equipment transaction applies removed/added modifiers as one staged source update and publishes exactly one valid rebuild event. | Models equipment replacement as one atomic source delta with final-set validation and no intermediate consumer-visible state. |
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-15: published snapshots are actor-local monotonic versions; accepted structural changes emit one rebuild event; failed rebuilds do not replace current valid snapshot version. | Defines `source_version` and `snapshot_version` advancement rules and failure no-advance semantics. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Edge cases: equipment switching failure, pending initialization, save/load rebuild mismatch, and layer pollution. | Defines stale version checks, initialization/replace policies, same-path debug mutation, and forbids direct external runtime source mutation. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Implementation-blocking ADR item 4: atomic source update / transaction model. | Directly resolves the transaction prerequisite while leaving save/load serialization and fixture/config loading to later ADRs. |

## Related

- `design/gdd/character-attributes-system.md`
- `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`
- `docs/architecture/adr-0007-attribute-snapshot-query-api-shape-and-read-only-enforcement.md`
- `docs/architecture/adr-0008-attribute-event-signal-contract-and-scene-tree-independent-core.md`
- `docs/registry/architecture.yaml` — updated with transaction contracts, version expectations, and forbidden direct/reentrant mutation patterns after this ADR.
