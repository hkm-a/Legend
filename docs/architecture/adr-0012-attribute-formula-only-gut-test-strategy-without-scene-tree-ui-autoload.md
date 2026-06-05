# ADR-0012: Attribute Formula Only GUT Test Strategy Without Scene Tree UI Autoload

## Status

Accepted

## Date

2026-06-04

## Last Verified

2026-06-04

## Decision Makers

hkm + Claude Code Game Studios

## Summary

This ADR defines the blocking unit-test authority and harness boundary for Phase 1 Character Attributes formula and contract logic. We choose formula-only GUT unit tests that instantiate injectable `RefCounted` / service cores, typed DTOs, factories, normalizers, fake sinks, and fake counters directly, without making SceneTree, UI, Autoload, filesystem, Resource loading, timers, frames, or Godot signal callback order part of the Character Attributes blocking test oracle.

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Testing / Core / Scripting |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff and must be verified against engine reference docs and the selected GUT version before implementation. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md`; `.claude/docs/technical-preferences.md`; `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`; `docs/architecture/adr-0007-attribute-snapshot-query-api-shape-and-read-only-enforcement.md`; `docs/architecture/adr-0008-attribute-event-signal-contract-and-scene-tree-independent-core.md`; `docs/architecture/adr-0009-attribute-atomic-source-update-and-transaction-model.md`; `docs/architecture/adr-0010-attribute-save-load-persistence-boundary-base-current-modifier-sources.md`; `docs/architecture/adr-0011-attribute-fixture-config-loading-strategy-mvp-provisional-values.md` |
| **Post-Cutoff APIs Used** | None required by the Character Attributes formula/core test harness. The strategy intentionally avoids relying on Godot 4.5+ `@abstract`, variadic args, `Resource.duplicate_deep()`, timers, deferred calls, SceneTree APIs, UI focus behavior, or signal callback order as formula/contract test authority. |
| **Verification Required** | Verify the selected GUT runner/addon can run Godot 4.6.3 headless tests that instantiate `RefCounted` classes directly; verify `Array` / `Dictionary` shallow/deep copy behavior; typed Array behavior after Variant/untyped factory input; `RefCounted` aliasing behavior; deterministic failure ordering independent of raw `Dictionary` iteration; no `Node`, SceneTree, Autoload, `FileAccess`, `ResourceLoader`, real `.tres`, real `.tscn`, timer/frame, or UI dependency in blocking Character Attributes formula/contract unit tests; fake instrumentation can prove O(M + S) behavior without profiler/timer assertions. |

> **Note**: If Knowledge Risk is MEDIUM or HIGH, this ADR must be re-validated if the project upgrades engine versions. Flag it as "Superseded" and write a new ADR.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | ADR-0006 Attribute Data Representation and Stat ID Typing; ADR-0007 Attribute Snapshot Query API Shape and Read-Only Enforcement; ADR-0008 Attribute Event Signal Contract and Scene-Tree-Independent Core; ADR-0009 Attribute Atomic Source Update and Transaction Model; ADR-0010 Attribute Save Load Persistence Boundary; ADR-0011 Attribute Fixture Config Loading Strategy; approved Character Attributes GDD: `design/gdd/character-attributes-system.md`. |
| **Enables** | Formula-only Character Attributes GUT unit-test harness; Character Attributes blocking evidence for formulas, validation, snapshots, deltas, resource mutations, preview non-authority, transaction no-side-effect behavior, reentrancy rejection, DTO aliasing boundaries, debug trace minimal evidence, event/result DTO materialization, and O(M + S) aggregation evidence; story readiness for Character Attributes logic implementation once remaining ADR prerequisites are resolved. |
| **Blocks** | Any Character Attributes implementation story that claims completion for formula outputs, validation pipeline, config normalization, snapshot validity, snapshot/result read-only boundaries, current-resource mutation, structural transaction behavior, preview non-authority, debug trace payload, event/result DTO content, or fixture portions of equipment/monster/growth acceptance criteria without formula-only GUT evidence. |
| **Ordering Note** | This ADR defines Character Attributes blocking formula/contract unit-test authority only. It does not forbid Save/Load IO tests, external config pipeline tests, `.tres` / JSON integration tests, signal adapter tests, HUD/UI walkthroughs, scene smoke tests, screenshots, or downstream integration evidence. Those evidence types are supplemental or owned by downstream systems and cannot replace the formula/contract unit tests required here. |

## Context

### Problem Statement

The Character Attributes GDD makes many Phase 1 acceptance criteria BLOCKING and explicitly requires automated unit tests for formulas, validation, snapshot validity, deltas, resource mutation, event/version contracts, debug trace evidence, and fixture portions of equipment/monster/growth behavior. It also lists a formula-only GUT test strategy without scene tree, UI, or Autoload as an implementation-blocking ADR prerequisite.

If blocking Character Attributes tests depend on SceneTree, `Node` lifecycle, Autoload singletons, UI/Control state, filesystem fixtures, real `.tres` / `.tscn` files, ResourceLoader cache behavior, timers/frames, or Godot signal callback order, then core formula and contract defects can be hidden behind integration behavior. Such tests would also contradict ADR-0008's scene-tree-independent core, ADR-0009's result-before-dispatch transaction model, ADR-0010's no-file-IO attribute core boundary, and ADR-0011's factory/normalizer config boundary.

This ADR decides what evidence counts as blocking Character Attributes formula/contract evidence, how the test harness is constructed, and which dependencies are forbidden for that evidence.

### Current State

No Character Attributes implementation or test suite exists yet. The project has selected GUT as the GDScript unit-test framework in `.claude/docs/technical-preferences.md`, but has not yet defined the Character Attributes harness boundary. ADR-0006 through ADR-0011 define runtime stat identity, snapshot/query read-only semantics, event/result boundaries, atomic transaction behavior, save/load input boundaries, and fixture/config normalization. The remaining gap is the unit-test strategy that proves those decisions without accidentally depending on scene/runtime integration layers.

### Scope and Non-Goals

This ADR governs **Phase 1 Character Attributes blocking formula/contract GUT unit tests**.

It does not forbid:

- Save/Load file IO unit or integration tests;
- future JSON, `.tres`, CSV, YAML, generated-script, or editor import pipeline tests;
- Godot signal adapter tests;
- HUD/UI screenshots, walkthroughs, or interaction tests;
- scene smoke tests;
- visual/feel evidence;
- downstream combat, equipment, save/load, spawn, or UI integration tests.

Those tests are valid supplemental or downstream-system evidence. They cannot substitute for Character Attributes formula/contract unit tests when a Character Attributes story claims completion of logic, validation, transaction, snapshot, result, preview, debug trace, or formula behavior.

### Constraints

- The Character Attributes core must remain injectable and scene-tree-independent.
- Test setup must align with ADR-0011: config payloads enter through factories/normalizers, not scattered hardcoded formula constants.
- Runtime formula tests must use ADR-0006 `StatId` semantics after normalization.
- Snapshot/result DTO tests must respect ADR-0007 immutable-by-contract and defensive-copy boundaries.
- Event/result tests must respect ADR-0008: core results are materialized before any downstream signal/sink adapter.
- Transaction tests must respect ADR-0009: no candidate exposure, no mutate-then-rollback, no reentrant mutation acceptance in Phase 1.
- Load rebuild contract tests must use decoded DTOs or primitive payloads, not file IO inside Character Attributes core.
- GDScript immutability is by contract and tests; `RefCounted`, `Array`, `Dictionary`, typed arrays, and DTO payloads can still alias mutable data if not copied/normalized.

### Requirements

- Define the authoritative blocking test category for Character Attributes formulas and contracts.
- Keep blocking tests executable in GUT headless without constructing gameplay scenes, UI, Autoloads, or file-backed Resources.
- Require config normalizer tests that start from primitive semantic-key payloads, not only pre-normalized tables.
- Allow formula kernel tests to inject validated normalized runtime config tables where config loading is not the test subject.
- Cover every Character Attributes formula and the GDD-required structured failure semantics.
- Prove failed transactions, failed rebuilds, invalid previews, and invalid load DTOs have no committed side effects.
- Prove DTO/read-only/aliasing boundaries by mutating caller-owned or returned containers where possible.
- Prove deterministic failure ordering with explicit expected reason sequences.
- Prove aggregation O(M + S) behavior through deterministic fake counters/instrumentation rather than timers or frame counts.

## Decision

Phase 1 Character Attributes uses formula-only GUT unit tests as the blocking evidence authority for Character Attributes formula and contract behavior.

Blocking formula/contract tests must live under `tests/unit/character_attributes/` unless the test setup ADR later supersedes the global test layout. These tests directly instantiate:

- injectable Character Attributes core/service classes;
- typed `RefCounted` DTOs and request/result wrappers;
- fixture/config factories;
- config normalizers;
- validated immutable-by-contract runtime config tables;
- fake event sinks;
- fake instrumentation counters;
- no-op downstream adapters where needed.

Blocking formula/contract tests must not depend on:

- gameplay scene instantiation;
- `Node` lifecycle, `_ready()`, `_process()`, `_physics_process()`, `get_tree()`, child lookup, or scene ownership;
- Autoload singletons or global event buses;
- UI / `Control` state, HUD state, focus state, screenshots, or layout;
- real `.tscn` or `.tres` files;
- `ResourceLoader` or ResourceLoader cache behavior;
- `FileAccess`, save slot paths, filesystem fixtures, or file IO;
- timers, frames, real time, `Performance` profiler values, or frame-count assertions;
- signal callback order, listener order, deferred calls, or asynchronous event order as formula/update authority.

The GUT runner itself may run inside a Godot process. "Without SceneTree" means the **objects under test and the blocking test oracle** do not depend on SceneTree or Node lifecycle. It does not require proving the GUT runner has no internal SceneTree.

Character Attributes config tests are split into two allowed layers:

1. **Config normalization tests** start from primitive semantic-key payloads or centralized factory payload output, pass through ADR-0011 validation/normalization, and verify success/failure, source-status labels, semantic key mapping, version labels, failure order, and defensive-copy behavior.
2. **Formula kernel tests** may inject already-normalized `AttributeRuntimeConfigTables` when the subject is a formula, transaction, snapshot, preview, resource mutation, or result boundary. Such tests cannot be cited as coverage for config loading, semantic-key migration, source-status validation, or external payload validation.

Formula-only tests may use synthetic `StatId` values only for narrow math-boundary cases where registry mapping is not the subject. Any test claiming fixture/config coverage must pass through semantic-key payload normalization and must not use enum ordinal arrays, positional dense-vector fixture rows, or raw strings as runtime authority.

The following Character Attributes behavior requires blocking formula/contract GUT evidence:

- `effective_stat` success and structured failures;
- `effective_stat_pair` success and structured failures;
- `current_resource_after` success, max-resource increase policy, max-resource reduction clamp, and structured failures;
- `snapshot_valid` success/failure and deterministic ordered failure reasons;
- `attribute_delta` comparability and structured failures;
- `snapshot_delta` requested-set mode, visible summary mode, debug-full mode, inactive/reserved filtering, pair grouping, and structured failures;
- `combat_power` MVP provisional display formula, all-zero weight failure, inactive/reserved zero contribution, and propagated child failures;
- source status and OpenMir2 evidence label validation;
- config validation Stage A failure before any actor commit;
- source/input validation Stage B failure before any actor commit;
- calculation/correction Stage C success/failure behavior;
- output validation Stage D failure before snapshot publication;
- invalid modifier policy, including unknown stat, unsupported operation, inactive/reserved target, duplicate source key, expired/inactive source, and missing evidence;
- snapshot/query wrapper status, stale/display-only fallback, invalid snapshot status, and no status bypass;
- normal runtime snapshot no eager full debug trace payload;
- preview non-authority: no source commit, no version advancement, no event sink/signal adapter invocation, no committed domain event entry, and explicit preview metadata;
- structural transaction no-side-effect failure;
- load-rebuild contract against decoded DTOs/payloads without file IO;
- current-resource mutation path without structural modifier reaggregation;
- reentrant mutation rejection during transaction, result materialization, result dispatch, or fake sink invocation;
- event/result DTO content and copied event arrays after successful commits;
- debug trace minimal payload sufficient to reproduce at least one failing formula case without UI, SceneTree, or Autoload;
- DTO, snapshot, config table, request, and result aliasing/mutation boundaries;
- deterministic O(M + S) modifier aggregation evidence using fake counters or instrumentation.

Integration, adapter, UI, screenshot, manual walkthrough, Save/Load IO, external authoring, and scene smoke tests may be created later. They must be labeled as supplemental or downstream evidence and must not be used to mark the Character Attributes formula/contract acceptance criteria complete unless the formula/contract unit tests above also pass.

### Architecture Diagram

```text
GUT headless test invocation
        |
        | creates test case scripts
        v
Formula/contract test factory layer
(no Node/Autoload/UI/FileAccess/ResourceLoader dependency)
        |
        +--> primitive semantic-key payloads
        |       |
        |       v
        |   ADR-0011 config normalizer tests
        |       |
        |       v
        |   validated AttributeRuntimeConfigTables
        |
        +--> typed source/request DTOs
        +--> fake sinks / fake counters
        v
Injectable Character Attributes core/service
        |
        +--> formulas
        +--> config/source/output validation
        +--> transaction staging + final swap
        +--> snapshot/result/event DTO materialization
        v
GUT assertions
        |
        +--> values and structured failures
        +--> deterministic failure reason order
        +--> versions and no-side-effect state
        +--> aliasing/mutation isolation
        +--> fake counter O(M + S) evidence
        +--> no forbidden SceneTree/UI/Autoload/IO/Resource dependency
```

### Key Interfaces

```gdscript
class_name AttributeFormulaTestFactory
extends RefCounted

func build_valid_phase1_payload() -> AttributeFixtureConfigPayload:
    pass

func build_invalid_payload_for_reason(reason_id: int) -> AttributeFixtureConfigPayload:
    pass

func build_player_source_input() -> AttributeSourceInput:
    pass

func build_monster_source_input(template_key: StringName) -> AttributeSourceInput:
    pass

func build_modifier_source(source_key: StringName, target_stat_key: StringName, value: int) -> AttributeModifierSource:
    pass
```

```gdscript
class_name AttributeFormulaTestHarness
extends RefCounted

func normalize_config(payload: AttributeFixtureConfigPayload) -> AttributeConfigValidationResult:
    pass

func make_core(config_tables: AttributeRuntimeConfigTables) -> AttributeCore:
    pass

func make_fake_event_sink() -> AttributeFakeEventSink:
    pass

func make_fake_aggregation_counter() -> AttributeFakeAggregationCounter:
    pass

func capture_committed_state(core: AttributeCore) -> AttributeCommittedStateProbe:
    pass

func assert_no_committed_side_effects(before: AttributeCommittedStateProbe, after: AttributeCommittedStateProbe, result: AttributeUpdateResult) -> void:
    pass

func assert_failure_reasons_ordered(actual: Array[int], expected: Array[int]) -> void:
    pass
```

```gdscript
class_name AttributeFakeAggregationCounter
extends RefCounted

func record_stat_visit(stat_id: int) -> void:
    pass

func record_modifier_visit(source_key: StringName) -> void:
    pass

func get_stat_visit_count() -> int:
    pass

func get_modifier_visit_count() -> int:
    pass
```

```gdscript
class_name AttributeFakeEventSink
extends RefCounted

func publish_result(result: AttributeUpdateResult) -> void:
    pass

func get_published_results_copy() -> Array[AttributeUpdateResult]:
    pass

func request_reentrant_mutation_if_configured(core: AttributeCore) -> void:
    pass
```

The interfaces above are conceptual. Exact filenames, constructors, assertion style, and whether helpers are instance methods or static helpers may be refined during implementation. The core constraint is that the helper layer remains test-only, deterministic, and free of SceneTree/UI/Autoload/filesystem/resource-loader authority.

### Implementation Guidelines

- Keep each public test helper method unit-testable and free of hidden global state.
- Prefer test fixture factories that expose intent through names such as `build_valid_attack_upgrade_payload()` rather than opaque raw dictionaries.
- When a formula test injects normalized config, cite separate config-normalizer tests for semantic-key and source-status coverage.
- Use fake counters for O(M + S) aggregation evidence; do not use elapsed time or frame counts as blocking unit-test proof.
- Use fake event sinks to capture result/event DTOs and to attempt reentrant mutation where needed.
- Assert no committed side effects by capturing committed source version, snapshot version, current snapshot identity/status, current resources, and event/sink counts before and after rejected requests.
- Mutate inbound payload containers, returned arrays, and DTO-like values after submission where possible to prove defensive-copy and immutable-by-contract behavior.
- Verify failed rebuild keeps previous valid truth only as stale/display-only metadata and never as current combat/save truth.
- Verify `AttributePreviewResult` never appears in committed domain event arrays.
- Do not make tests pass by reaching into private backing state unless the test is explicitly white-box instrumentation for O(M + S) and the instrumentation is test-only.

## Alternatives Considered

### Alternative 1: Scene-level GUT integration tests as primary evidence

- **Description**: Instantiate scenes, Nodes, Autoloads, and gameplay adapters in GUT, then verify attributes through scene behavior.
- **Pros**: Exercises more real wiring; closer to gameplay runtime; useful for smoke tests.
- **Cons**: Can hide formula/contract defects behind scene setup; depends on lifecycle/order; contradicts the scene-tree-independent core requirement if used as blocking formula evidence; slower and harder to localize failures.
- **Estimated Effort**: Similar initial effort, higher long-term maintenance.
- **Rejection Reason**: Scene tests are allowed as supplemental integration evidence, but formula/contract authority must stay direct, deterministic, and scene-tree-independent.

### Alternative 2: UI/manual evidence for growth and snapshot behavior

- **Description**: Use HUD/equipment UI screenshots or walkthroughs to prove resource display, deltas, and growth feedback.
- **Pros**: Necessary for presentation correctness; verifies player-visible output.
- **Cons**: Cannot prove formula correctness, failure semantics, no-side-effect transactions, aliasing boundaries, or deterministic failure ordering; UI may mask stale/invalid status bugs.
- **Estimated Effort**: Lower for visual proof, insufficient for logic proof.
- **Rejection Reason**: UI/manual evidence remains required for UI stories but cannot replace Character Attributes blocking formula/contract tests.

### Alternative 3: File-backed `.tres` / JSON fixtures in blocking formula tests

- **Description**: Load real fixture files through `ResourceLoader`, `FileAccess`, or import paths in formula tests.
- **Pros**: Closer to future data pipeline; catches path/import issues.
- **Cons**: Introduces filesystem and Resource cache behavior into formula tests; conflicts with ADR-0011 factory/normalizer boundary; makes failures sensitive to import state and engine object sharing.
- **Estimated Effort**: Higher setup complexity.
- **Rejection Reason**: External authoring tests are allowed later as integration/pipeline tests. Blocking formula/contract tests use primitive payloads and injected DTOs, not real files.

### Alternative 4: Mock Autoload service for test convenience

- **Description**: Register test Autoloads for config, event buses, save data, or global attribute access during tests.
- **Pros**: Mirrors common Godot project patterns; easy global access.
- **Cons**: Reintroduces hidden coupling and load-order dependencies; conflicts with ADR-0008's ban on Phase 1 Autoload attribute event bus and scene-tree-independent core; weakens dependency injection.
- **Estimated Effort**: Lower short-term, higher debugging cost.
- **Rejection Reason**: Test code should use explicit dependency injection and fake sinks/services instead of Autoloads.

### Alternative 5: Timing/profiler-based performance tests

- **Description**: Use elapsed time, frame counts, timers, or `Performance` profiler metrics to prove formula performance.
- **Pros**: Measures real runtime cost in some environments.
- **Cons**: Flaky in headless CI; depends on hardware/editor state; poor at proving algorithmic O(M + S) behavior for small fixtures.
- **Estimated Effort**: Medium and unstable.
- **Rejection Reason**: Blocking unit evidence uses deterministic fake counters/instrumentation. Separate performance profiling may be added later.

## Consequences

### Positive

- Character Attributes formula and contract failures are localized to the core instead of scene wiring.
- Tests directly enforce ADR-0006 through ADR-0011 boundaries before implementation stories proceed.
- GUT evidence remains deterministic, headless-friendly, and suitable for CI.
- Config, snapshot, transaction, event/result, preview, persistence DTO, and debug-trace contracts can be proven without UI or gameplay scenes.
- Future integration tests remain allowed but cannot dilute logic-test requirements.
- Fake counters provide stable evidence for aggregation complexity without timing flakiness.

### Negative

- More test-only factories, DTO builders, fake sinks, fake counters, and state probes are required.
- Some tests are less representative of full gameplay wiring and must be supplemented by integration/UI evidence in downstream stories.
- Immutable-by-contract and aliasing boundaries require deliberate mutation tests and defensive-copy boilerplate.
- Config-normalizer and formula-kernel tests must be kept distinct, which adds test organization overhead.
- The selected GUT version still needs explicit Godot 4.6.3 verification before implementation.

### Neutral

- This ADR does not choose the exact GUT addon version, runner script, CI workflow, or test discovery command beyond the project standard that GUT is the unit-test framework.
- This ADR does not define whole-project testing strategy for UI, rendering, save-slot IO, external data import, or playtest evidence.
- This ADR does not require all Character Attributes tests to be black-box; carefully scoped test-only instrumentation is allowed for deterministic complexity evidence.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| Implementers interpret "without SceneTree" as impossible because the GUT runner itself uses a Godot process. | Medium | Medium | Define the boundary as objects under test and blocking oracle not depending on SceneTree/Node lifecycle, not the runner internals. |
| Normalized-table formula tests bypass config validation coverage. | Medium | High | Require separate normalizer tests from primitive semantic-key payloads; formula-kernel tests cannot claim config coverage. |
| Future integration tests are incorrectly banned. | Medium | Medium | Scope ADR to Character Attributes blocking formula/contract tests only and explicitly allow downstream/integration evidence as supplemental. |
| `RefCounted` DTOs, Arrays, or Dictionaries leak mutable aliases. | Medium | High | Constructors copy, getters return defensive copies/immutable rows, and tests mutate inputs/returned collections after submission. |
| `Array.duplicate()` / `Dictionary.duplicate()` shallow copies fail nested isolation. | Medium | High | Require recursive normalization/deep-copy tests for nested containers; scalar shallow copies only where proven safe. |
| Typed arrays give false confidence after Variant or untyped factory input. | Medium | Medium | Normalizers perform explicit runtime type/range validation and deterministic failures. |
| Tests accidentally rely on signal/listener/timer/frame order. | Medium | High | Ban callback order/timer/frame oracle in blocking tests; use returned result DTOs and fake sinks after result materialization. |
| Fake counters drift from real aggregation code. | Low | Medium | Instrument the actual aggregation path or inject test-only counters into the same loop rather than counting fixture construction. |
| GUT version is incompatible or behaves differently under Godot 4.6.3. | Low | High | Verify GUT runner before implementation stories; document the runner command and evidence path in test setup. |
| Private-state probes overfit implementation. | Medium | Medium | Limit white-box probes to committed-state side-effect and complexity evidence; value/failure tests should use public APIs/results. |

## Performance Implications

| Metric | Before | Expected After | Budget |
|--------|--------|---------------|--------|
| CPU (frame time) | No implementation. | No gameplay frame-time cost. Tests run outside gameplay frames and verify structural rebuild paths are not per-frame and aggregation behaves as O(M + S). | Overall project frame budget 16.6 ms at 60 fps; tests must not rely on frame timing as proof. |
| Memory | No implementation. | Test factories and DTOs allocate during test runs only. Runtime code may add optional test instrumentation hooks or fake-injectable counters if kept test-only/no-op in production. | Phase 1 client under 1 GB RAM; no unbounded debug/test payload in runtime snapshots. |
| Load Time | No implementation. | No gameplay load-time decision except that config normalizer tests validate bootstrap-style normalization separately from formula kernels. | Profile later if external config files are introduced. |
| Network | Not applicable for Phase 1 offline slice. | Not applicable. | None. |

## Migration Plan

No existing Character Attributes tests or implementation need migration.

1. Select and verify the GUT runner/addon version against Godot 4.6.3 headless execution.
2. Create `tests/unit/character_attributes/` and the Character Attributes test helper/factory organization.
3. Implement minimal DTO/factory builders for primitive semantic-key payloads and source/request fixtures.
4. Implement config-normalizer tests from ADR-0011 primitive payloads before formula kernel tests claim config coverage.
5. Implement formula tests for `effective_stat`, `effective_stat_pair`, `current_resource_after`, `snapshot_valid`, `attribute_delta`, `snapshot_delta`, and `combat_power`.
6. Implement transaction/resource/preview/event/result tests for no-side-effect failure, version behavior, resource-only mutation, preview non-authority, reentrancy rejection, and event/result DTO materialization.
7. Implement snapshot/query/read-only tests for stale/invalid/display-only status, wrapper status, defensive-copy behavior, and debug trace payload boundaries.
8. Implement aliasing/mutation tests for inbound payloads, returned arrays, DTOs, config tables, snapshots, and result/event DTOs.
9. Implement fake counter/instrumentation tests proving aggregation visits active modifiers and enabled stats once according to O(M + S) expectations.
10. Keep downstream integration/UI/Save/Load IO evidence in separate test/evidence locations and mark it supplemental for Character Attributes formula contracts.

**Rollback plan**: If GUT proves unsuitable for direct formula/contract testing under Godot 4.6.3, supersede this ADR with an alternate GDScript-capable unit-test runner while preserving the no SceneTree/UI/Autoload/filesystem/signal-order blocking oracle boundary and the same coverage requirements.

## Validation Criteria

- [ ] GUT runs Godot 4.6.3 headless Character Attributes unit tests that instantiate `RefCounted` / service classes directly.
- [ ] Blocking formula/contract tests do not instantiate gameplay scenes, UI/Control nodes, Autoloads, real `.tscn` files, real `.tres` files, ResourceLoader paths, FileAccess paths, timers, frame waits, or signal-order oracles.
- [ ] Config-normalizer tests start from primitive semantic-key payloads or centralized factory payloads and verify semantic key mapping, source-status labels, evidence labels, versions, deterministic failure ordering, and defensive-copy behavior.
- [ ] Formula kernel tests may inject normalized runtime config tables only when they do not claim config-loading coverage.
- [ ] `effective_stat` tests cover no modifier, one modifier, multiple modifiers, inactive modifier exclusion, min clamp, max clamp, invalid stat ID, invalid bounds, unsupported operation, order independence, duplicate source handling, out-of-range values, and numeric overflow/technical-limit structured failure.
- [ ] `effective_stat_pair` tests cover valid, equal, invalid, missing, config-incoherent, and propagated child-failure cases.
- [ ] `current_resource_after` tests cover damage/heal/spend/restore clamp, max-resource reduction clamp, max-resource increase default `keep_current_with_feedback`, invalid bounds, propagated max-stat failure, and combat-ready `health_max > 0` enforcement.
- [ ] `snapshot_valid` tests cover invalid config, missing identity, missing actor-type stats, unknown active stat IDs, invalid pairs, invalid resource bounds, invalid source labels, combat readiness failures, and multiple failure reasons sorted by registry order.
- [ ] `attribute_delta` tests cover actor mismatch, schema/config mismatch, missing stat, invalid snapshots, stale/reversed authoritative versions, preview comparability, and signed success values.
- [ ] `snapshot_delta` tests cover requested-set mode, visible summary mode, debug-full mode, inactive/reserved filtering, pair grouping without double-counting, empty identical deltas, and invalid/incomparable structured failures.
- [ ] `combat_power` tests cover valid attack upgrade, defense/health contribution, inactive/reserved stat zero contribution, all-zero active player-facing weights failure, invalid config, and propagated child formula failures.
- [ ] Source-status and OpenMir2 evidence tests reject unlabeled provisional values and `openmir2_verified` claims without evidence ID/source reference.
- [ ] Invalid modifier tests cover unknown stat target, unsupported operation, inactive/reserved target, duplicate source key, inventory-only/inactive exclusion, expired/inactive source exclusion, and missing authenticity evidence.
- [ ] Failed structural transaction tests prove committed source set, current resources, current snapshot, `source_version`, `snapshot_version`, and sink/event counts remain unchanged.
- [ ] Failed rebuild tests prove previous valid snapshot may be exposed only as stale/display-only metadata and cannot become current combat/save truth through query result wrapper bypass.
- [ ] Resource-only mutation tests prove HP/MP current-value changes advance snapshot publication according to ADR-0009 and do not reaggregate structural modifiers or advance `source_version`.
- [ ] Preview tests prove `AttributePreviewResult` does not commit source changes, increment authoritative versions, invoke sinks/signals, or appear in committed domain event arrays.
- [ ] Reentrant mutation tests prove requests submitted during transaction, result materialization, dispatch, or fake sink invocation reject deterministically and do not affect committed state.
- [ ] Event/result DTO tests prove copied arrays, expected reason/version metadata, compact payloads, and no full old+new snapshot payload by default.
- [ ] Debug trace tests prove minimal structured failure payload can reproduce at least one failing formula case without SceneTree, UI, Autoload, filesystem, or heavy eager snapshot debug strings.
- [ ] Aliasing tests mutate inbound payload containers, caller-owned DTOs, returned arrays, and debug/result collections after submission/read and prove committed/runtime/snapshot/config state is unchanged.
- [ ] Fake instrumentation tests prove modifier aggregation visits active modifier rows and enabled stats according to O(M + S), without elapsed-time, timer, profiler, frame, or performance-singleton assertions.
- [ ] Save/load load-rebuild contract tests use decoded DTOs or primitive payloads injected into Character Attributes core; FileAccess read/write tests are separate Save/Load evidence.

## GDD Requirements Addressed

| GDD Document | System | Requirement | How This ADR Satisfies It |
|-------------|--------|-------------|--------------------------|
| `design/gdd/character-attributes-system.md` | Character Attributes | Implementation-blocking ADR item 7: formula-only GUT test strategy without scene tree, UI, or Autoload. | Defines the blocking Character Attributes formula/contract test harness boundary and forbids SceneTree/UI/Autoload/filesystem/resource/signal-order dependencies for that evidence. |
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-04 Effective Stat Formula requires automated unit tests for modifiers, clamps, invalid IDs, invalid bounds, unsupported operations, order independence, and O(M + S) behavior. | Makes `effective_stat` coverage and fake O(M + S) instrumentation blocking validation criteria. |
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-05 Min/Max Pair Validation requires tests for valid/equal/invalid/missing/config-incoherent pairs. | Requires `effective_stat_pair` formula-only tests and propagated child failure coverage. |
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-06 Current Resource Mutation and Clamp requires resource-only damage/heal/spend/restore tests, max-resource clamp semantics, max increase policy, and health readiness enforcement. | Requires `current_resource_after`, resource-only mutation, max increase/reduction, and no structural reaggregation tests. |
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-07A Snapshot Validity Formula requires automated tests for invalid config/source/output states and deterministic failure ordering. | Requires `snapshot_valid` and validation Stage A-D formula/contract tests with ordered failure reasons. |
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-08 Attribute and Snapshot Delta requires comparability checks and structured delta failures. | Requires `attribute_delta` and `snapshot_delta` tests for modes, filtering, pair grouping, version/schema/config comparability, and failure cases. |
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-09 fixture portion requires accepted equipment transactions apply modifiers atomically and failed rebuilds expose structured evidence without intermediate snapshots. | Requires staged source transaction tests, no intermediate committed exposure, failed rebuild stale fallback, and no-side-effect assertions. |
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-13 fixture portion requires valid ordinary monster templates and invalid monster templates not creating combat-ready actors. | Requires factory/normalizer tests for monster actor-type payloads and combat readiness validation without scenes. |
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-14 Debug Traceability Payload requires structured failure evidence reproducible without HUD, scene tree, or Autoload. | Requires formula-only debug trace tests for minimal structured payload and forbids heavy eager debug strings in normal snapshots. |
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-15 Snapshot Versioning and Event Contract requires monotonic versions, rebuild/resource events, failed rebuild no replacement, and compact payloads. | Requires version, event/result DTO, resource mutation, failed rebuild, copied event array, and no full old+new snapshot payload tests. |
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-16 fixture portion requires growth reason, salience, visible delta summary, and provisional combat power delta for positive equipment/level changes. | Requires visible delta summary, inactive/reserved filtering, preview/growth fixture, and `combat_power` formula tests. |
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-17 Blocking Test Evidence Rollup expects automated test evidence under `tests/unit/character_attributes/`. | Sets the unit-test location and states formula/contract tests are required before Character Attributes logic stories can complete. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Formulas define `combat_power` as display-only with all-zero weight failure and inactive/reserved zero contribution. | Adds `combat_power` as blocking formula coverage even though final combat-power ownership remains a later ADR prerequisite. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Validation pipeline and source-status rules reject invalid config, missing source status, and OpenMir2-authentic claims without evidence. | Requires config-normalizer tests from primitive semantic-key payloads and source-status/evidence failure coverage. |

## Related Decisions

- `design/gdd/character-attributes-system.md`
- `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`
- `docs/architecture/adr-0007-attribute-snapshot-query-api-shape-and-read-only-enforcement.md`
- `docs/architecture/adr-0008-attribute-event-signal-contract-and-scene-tree-independent-core.md`
- `docs/architecture/adr-0009-attribute-atomic-source-update-and-transaction-model.md`
- `docs/architecture/adr-0010-attribute-save-load-persistence-boundary-base-current-modifier-sources.md`
- `docs/architecture/adr-0011-attribute-fixture-config-loading-strategy-mvp-provisional-values.md`
- `docs/registry/architecture.yaml` — should be updated with formula blocking test contract, formula unit test harness API decision, and forbidden test evidence anti-patterns after this ADR.
