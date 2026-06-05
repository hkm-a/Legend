# ADR-0013: Attribute Godot Resource Duplication and Shared Reference Policy If `.tres` Resources Are Used

## Status

Accepted

## Date

2026-06-04

## Last Verified

2026-06-04

## Decision Makers

hkm + Claude Code Game Studios

## Summary

This ADR defines the Godot `Resource` / `.tres` duplication and shared-reference policy for future Character Attributes authoring inputs. Phase 1 remains factory-first per ADR-0011. If `.tres` Resources are introduced later, they are schema-driven authoring envelopes only: importer/bootstrap code reads explicit whitelisted semantic data, validates and normalizes it into project-owned immutable-by-contract DTO/table data, and never treats loaded or duplicated Resource graphs as runtime authority, save truth, formula-test authority, snapshots, or transaction state.

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Core / Scripting / Resources / Config Loading |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff and Resource duplication behavior must be verified against pinned docs/tests before implementation. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md`; `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`; `docs/architecture/adr-0007-attribute-snapshot-query-api-shape-and-read-only-enforcement.md`; `docs/architecture/adr-0008-attribute-event-signal-contract-and-scene-tree-independent-core.md`; `docs/architecture/adr-0009-attribute-atomic-source-update-and-transaction-model.md`; `docs/architecture/adr-0010-attribute-save-load-persistence-boundary-base-current-modifier-sources.md`; `docs/architecture/adr-0011-attribute-fixture-config-loading-strategy-mvp-provisional-values.md`; `docs/architecture/adr-0012-attribute-formula-only-gut-test-strategy-without-scene-tree-ui-autoload.md` |
| **Post-Cutoff APIs Used** | `Resource.duplicate_deep()` exists in Godot 4.5+ and is relevant only as an optional importer/tooling test aid after pinned-version verification. It is not selected as a runtime authority boundary. No Character Attributes formula/core path depends on post-cutoff APIs. |
| **Verification Required** | Verify ResourceLoader cache/shared-reference behavior; root `.tres` Resource mutation after import cannot affect normalized DTO/tables; nested exported arrays/dictionaries cannot leak mutable authority; `Resource.duplicate()` fails or is insufficient for nested isolation; any `duplicate_deep()` use is tested against nested Resources, subresources, external references, containers, and post-normalization mutation isolation under Godot 4.6.3; formula-only tests still avoid ResourceLoader/FileAccess/real `.tres` per ADR-0012. |

> **Note**: If Knowledge Risk is MEDIUM or HIGH, this ADR must be re-validated if the project upgrades engine versions. Flag it as "Superseded" and write a new ADR.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | ADR-0006 Attribute Data Representation and Stat ID Typing; ADR-0007 Attribute Snapshot Query API Shape and Read-Only Enforcement; ADR-0008 Attribute Event Signal Contract and Scene-Tree-Independent Core; ADR-0009 Attribute Atomic Source Update and Transaction Model; ADR-0010 Attribute Save Load Persistence Boundary; ADR-0011 Attribute Fixture Config Loading Strategy; ADR-0012 Attribute Formula Only GUT Test Strategy; approved Character Attributes GDD: `design/gdd/character-attributes-system.md`. |
| **Enables** | Future `.tres` / Godot Resource authoring importer design for Character Attributes config; Resource isolation regression tests; external authoring pipeline migration that preserves ADR-0011 normalization and ADR-0012 formula-test boundaries. |
| **Blocks** | Any Character Attributes story that introduces `.tres`, `ResourceLoader`, exported Resource config, Resource-backed fixtures, Resource duplication, Resource importer/bootstrap code, or Resource-based config pipeline behavior before this policy is implemented and tested. |
| **Ordering Note** | This ADR does not select `.tres` as the Phase 1 config strategy. ADR-0011 remains the Phase 1 factory-first config authority, and ADR-0012 remains the blocking formula/contract test authority. This ADR only governs the policy if `.tres` / Godot Resources are introduced as external authoring envelopes later. |

## Context

### Problem Statement

The Character Attributes GDD lists a prerequisite decision for Godot Resource duplication/shared-reference policy if `.tres` Resources are used. Earlier ADRs already establish that Resources may author config but must not become mutable runtime state or save truth. This ADR makes that policy explicit so future `.tres` authoring cannot bypass semantic-key normalization, immutable-by-contract DTO boundaries, no-side-effect transactions, or formula-only GUT evidence.

Godot `Resource` instances are reference objects. Loaded Resources, nested subresources, exported `Array` / `Dictionary` values, and ResourceLoader cache entries can be shared and mutable. If Character Attributes retained such graphs as runtime authority, one consumer or test could mutate config, snapshots, save inputs, or formula fixtures through an alias and create hard-to-reproduce gameplay truth bugs.

### Current State

No Character Attributes implementation exists. ADR-0011 selects centralized GDScript fixture/config factories and primitive semantic-key payloads as the Phase 1 MVP strategy. This ADR is therefore a guardrail for future external `.tres` authoring/import work, not a replacement for the current fixture strategy.

### Constraints

- Character Attributes runtime authority must remain project-owned, normalized, and immutable-by-contract.
- Formula/contract tests must not use ResourceLoader, FileAccess, real `.tres`, real `.tscn`, SceneTree, UI, Autoload, timers, frames, or signal order as blocking evidence.
- Save/load truth uses primitive semantic DTO payloads and versions, not Resource or Variant object graphs.
- Importer/bootstrap integration tests may use `.tres` only to prove Resource isolation and importer behavior.
- Godot 4.6.3 Resource duplication behavior is post-cutoff risk and must be verified before relying on exact API behavior.

### Requirements

- Define whether `.tres` / Resource can become runtime Character Attributes authority.
- Define the boundary between root authoring Resource, nested Resource graphs, importer schema, and normalized DTO/table output.
- Prevent ResourceLoader cache behavior or Resource object identity from affecting gameplay correctness.
- Make `duplicate()` / `duplicate_deep()` usage safe and non-authoritative.
- Preserve ADR-0011 deterministic validation and ADR-0012 formula-only test boundaries.

## Decision

Phase 1 Character Attributes does **not** use Godot `Resource` / `.tres` instances as runtime authority. If `.tres` Resources are introduced later, they are input authoring envelopes only.

A future `.tres` path must follow this flow:

```text
Godot .tres Resource file
        |
        | ResourceLoader/bootstrap only; no formula/core dependency
        v
Root Resource authoring envelope
        |
        | schema-driven importer reads explicit whitelisted fields only
        v
Primitive/value semantic payload
        |
        | ADR-0011 validation + normalization
        v
Project-owned immutable-by-contract DTO/table data
        |
        | injected into scene-tree-independent core
        v
Character Attributes formulas / transactions / snapshots / results
```

Runtime Character Attributes core, formula tests, save truth, snapshots, transactions, result DTOs, and config tables must not hold loaded Resource graphs, duplicated Resource graphs, ResourceLoader cache entries, editor-authored subresources, or mutable Resource-derived containers as authoritative state.

### Root Resource Envelope vs Nested Object Policy

The root `.tres` Resource may be accepted only as an importer input envelope. Nested `Object`, `Resource`, `RefCounted`, `Node`, `Callable`, and `Signal` values are rejected by default.

A later ADR or explicit versioned import schema may whitelist a specific nested Resource shape, but even then:

- the nested Resource remains a read-only authoring envelope;
- its fields are read through the same schema-driven importer;
- output is still reconstructed as project-owned DTO/table data;
- no nested Resource identity or mutable graph becomes gameplay authority.

### Schema-Driven Import

Resource import is schema-driven. `AttributeGodotResourceConfigImporter` or equivalent may read only fields declared by `AttributeResourceImportPolicy` or an equivalent versioned schema. The importer must not treat arbitrary exported properties, broad `get_property_list()` reflection output, editor metadata, inherited Resource properties, script references, object identity, Resource path, or `.tres` serialization order as the authoritative schema.

Unlisted fields must either fail or be ignored only according to an explicit forward-compatible policy. Unknown required fields, wrong versions, unsupported field types, duplicate semantic keys, missing source-status labels, invalid evidence labels, and invalid bounds must return deterministic validation failures.

Allowed imported value types are limited to the schema's explicit whitelist, typically primitive/value semantic data such as `bool`, `int`, `float` where integer/finite range is validated, `String`, `StringName`, and arrays/dictionaries whose recursive contents are also whitelisted. The importer rejects unsupported `Variant` values by default, especially `Object`, `Resource`, `RefCounted`, `Node`, `Callable`, `Signal`, and arbitrary script instances.

### Deterministic Failure Ordering

Resource importer failure ordering must be deterministic and independent of engine/editor traversal order. Failures are ordered by explicit import stage, schema field order, ADR-0011 failure reason registry order, stat/resource registry order, and sorted semantic keys where applicable.

The importer must not rely on raw `Dictionary` iteration order, Resource property order, `.tres` serialization order, editor display order, subresource order, or ResourceLoader cache behavior as its validation oracle.

### Duplication Policy

`Resource.duplicate()` is not accepted as nested Resource isolation for Character Attributes config, snapshots, save truth, fixtures, or runtime authority. The project must not rely on shallow duplication to protect nested Resources, arrays, dictionaries, or subresources.

`Resource.duplicate_deep()` exists in Godot 4.5+ and may be used only as an importer/tooling defensive-copy aid after Godot 4.6.3 verification. Passing `duplicate_deep()` tests does not make a duplicated Resource graph valid runtime authority. Duplicated Resources are still authoring/importer inputs only, and final authority must still be normalized DTO/table data.

`Array.duplicate(true)` and `Dictionary.duplicate(true)` may help copy plain Variant containers, but they are not an Object/Resource/RefCounted isolation boundary. Normalized runtime containers must recursively satisfy the importer whitelist and must not contain `Object`, `Resource`, `RefCounted`, `Node`, `Callable`, or `Signal` values. Public APIs return scalars, immutable-by-contract row DTOs, snapshots, or defensive copies and never expose mutable internal containers.

### ResourceLoader Cache and Object Identity

ResourceLoader cache mode, object identity, path identity, editor instance identity, or repeated-load behavior must not be a gameplay correctness premise. Tests should pass whether loader behavior returns shared instances or fresh instances, because correctness comes from schema-driven validation, copying, and normalization.

Importer/bootstrap code must not retain the original loaded Resource reference as authoritative state after import. Runtime config tables, snapshots, save payloads, and formula tests must not consult ResourceLoader cache entries.

### Test Boundary

Importer/bootstrap integration tests may use ResourceLoader, FileAccess, real `.tres`, and Resource duplication to verify authoring import behavior. These tests are supplemental pipeline evidence.

Blocking Character Attributes formula/contract tests remain governed by ADR-0012 and must use primitive semantic-key payloads, factories, normalizers, DTOs, fake sinks, fake counters, and committed-state probes without ResourceLoader, FileAccess, real `.tres`, or Resource-backed fixtures.

| Test type | `.tres` / ResourceLoader allowed? | Can substitute for blocking formula evidence? |
|---|---:|---:|
| Resource importer/bootstrap integration | Yes | No |
| Resource isolation regression | Yes | No |
| Config normalizer from primitive semantic payload | No real `.tres` | Yes, where applicable |
| Formula/kernel/transaction/snapshot tests | No | Yes |

### Architecture Diagram

```text
Future external authoring layer
(.tres root Resource envelope)
        |
        | ResourceLoader / FileAccess only in bootstrap/import tests
        v
AttributeGodotResourceConfigImporter
(schema-driven, deterministic, whitelist-based)
        |
        +--> rejects unsupported Object/Resource/RefCounted payloads
        +--> rejects unknown fields according to versioned policy
        +--> orders failures by schema/registry/sorted key order
        v
Primitive semantic payload / importer DTO
        |
        v
ADR-0011 AttributeConfigNormalizer
        |
        v
AttributeRuntimeConfigTables
(immutable-by-contract, no Resource graph)
        |
        v
ADR-0008/0009 scene-tree-independent core
        |
        v
ADR-0007 snapshots/results/events
```

### Key Interfaces

```gdscript
class_name AttributeGodotResourceConfigImporter
extends RefCounted

func import_resource(resource: Resource, policy: AttributeResourceImportPolicy) -> AttributeConfigValidationResult:
    pass
```

```gdscript
class_name AttributeResourceImportPolicy
extends RefCounted

func get_expected_resource_schema_version() -> StringName:
    pass

func get_allowed_root_class_names_copy() -> Array[StringName]:
    pass

func get_import_field_specs_copy() -> Array[AttributeResourceImportFieldSpec]:
    pass

func get_unknown_field_policy() -> int:
    pass
```

```gdscript
class_name AttributeResourceImportFieldSpec
extends RefCounted

func get_field_key() -> StringName:
    pass

func get_allowed_value_kind() -> int:
    pass

func is_required() -> bool:
    pass

func get_schema_order() -> int:
    pass
```

```gdscript
class_name AttributeResourceIsolationTestFactory
extends RefCounted

func build_nested_resource_fixture() -> Resource:
    pass

func mutate_loaded_resource_after_import(resource: Resource) -> void:
    pass

func mutate_nested_containers_after_import(resource: Resource) -> void:
    pass
```

The interface names are conceptual. Implementation may refine exact class names. The important contract is that importer results contain normalized primitive payloads, DTOs, or ADR-0011 runtime config tables, never the original Resource or Resource-derived object graph.

## Alternatives Considered

### Alternative 1: Keep duplicated Resource graphs as runtime config authority

- **Description**: Load `.tres`, call `duplicate_deep()` or equivalent, and keep the duplicated Resource tree as Character Attributes runtime config.
- **Pros**: Editor-friendly; less DTO boilerplate; direct use of Godot authoring assets.
- **Cons**: Makes gameplay correctness depend on engine object graph behavior; can expose mutable exported fields; complicates formula-only tests; risks shared references, subresource aliasing, ResourceLoader cache coupling, and save/migration fragility.
- **Rejection Reason**: Even deep-duplicated Resource graphs remain engine object graphs. ADR-0006, ADR-0010, ADR-0011, and ADR-0012 require normalized DTO/table authority and formula tests independent of Resource loading.

### Alternative 2: Permanently ban `.tres` authoring for Character Attributes

- **Description**: Require all future attribute config to remain GDScript factories, JSON, or generated code; never allow Godot Resources.
- **Pros**: Simplest aliasing boundary; avoids ResourceLoader/cache/import concerns.
- **Cons**: Prematurely blocks editor-friendly authoring and future tooling; unnecessary if Resources are treated as input envelopes only.
- **Rejection Reason**: `.tres` can be allowed later as an authoring envelope if it preserves schema-driven import, normalization, and non-authoritative Resource graph policy.

### Alternative 3: Rely on `Resource.duplicate()` shallow copying

- **Description**: Load Resource config and call `duplicate()` to avoid shared mutation.
- **Pros**: Simple; familiar API; lower implementation effort.
- **Cons**: Local engine reference flags nested-resource duplication as requiring `duplicate_deep()`; shallow copy does not reliably isolate nested Resources, arrays, dictionaries, or subresources.
- **Rejection Reason**: `duplicate()` is explicitly insufficient for nested Resource isolation and must not be treated as a correctness boundary.

### Alternative 4: Use JSON-only external authoring instead of `.tres`

- **Description**: Avoid Resource concerns by using JSON or another primitive external file format only.
- **Pros**: Primitive envelope; clearer object graph boundary; easier diff/review.
- **Cons**: Still requires Variant validation, numeric validation, semantic-key normalization, and deterministic failure ordering; does not resolve policy if `.tres` is introduced later.
- **Rejection Reason**: JSON remains a viable future envelope, but this ADR specifically closes the `.tres` Resource policy gap from the GDD prerequisite.

### Alternative 5: Reflection-based Resource importer

- **Description**: Use Resource exported properties or `get_property_list()` broadly to discover fields and convert them to config.
- **Pros**: Flexible; less schema boilerplate; adapts automatically to new exported fields.
- **Cons**: Imports editor metadata or unintended fields; validation order can depend on engine/editor property order; unsupported Object/Resource values can slip through; weak versioning and migration control.
- **Rejection Reason**: Character Attributes config import must be explicit, versioned, and deterministic. Broad reflection is a loose schema and contradicts ADR-0011 validation discipline.

## Consequences

### Positive

- Future `.tres` authoring can be added without changing Character Attributes runtime authority.
- ResourceLoader cache and shared-reference behavior cannot corrupt formulas, snapshots, save truth, or transactions.
- Formula-only GUT evidence remains clean and deterministic.
- Importer/bootstrap tests can still verify Godot-specific Resource behavior where it belongs.
- The policy aligns Resource authoring with existing semantic-key, DTO, snapshot, transaction, and persistence boundaries.

### Negative

- `.tres` authoring requires importer schema, validation, and copy/normalization boilerplate.
- Editor-authored nested Resource graphs are limited or rejected unless explicitly modeled.
- Resource isolation requires extra integration tests beyond formula tests.
- Designers cannot assume editing a Resource instance directly updates runtime gameplay truth.
- Future hot reload must build an explicit validated swap flow rather than relying on live Resource mutation.

### Neutral

- This ADR does not require `.tres` for Phase 1.
- This ADR does not choose final external config format, editor plugin tooling, or hot reload strategy.
- This ADR does not define exact Godot `duplicate_deep()` parameters; implementation must verify pinned Godot 4.6.3 docs/tests before specifying them.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| Implementers treat passing `duplicate_deep()` tests as permission to retain Resource graphs. | Medium | High | ADR states duplicated Resources are still importer inputs only; final authority is normalized DTO/table data. |
| Reflection-based importer accepts unintended fields or Object payloads. | Medium | High | Importer is schema-driven and reads only explicit field specs; unlisted fields follow explicit policy. |
| Resource property or serialization order creates nondeterministic failures. | Medium | Medium | Failure ordering uses import stages, schema order, registry order, and sorted semantic keys, never engine/editor order. |
| Root Resource vs nested Resource policy is misunderstood. | Medium | Medium | Root Resource is envelope; nested Objects/Resources rejected by default unless later schema/ADR whitelists them and still normalizes them. |
| ResourceLoader cache/shared references mutate multiple consumers. | Medium | High | Importer output is independent of loaded Resource identity; post-import mutation isolation tests required. |
| `Array` / `Dictionary` deep copy is mistaken for Object isolation. | Medium | High | Normalized containers recursively whitelist value types and reject Object/Resource/RefCounted/Callable/Signal. |
| Importer tests are used to replace formula tests. | Medium | High | ADR-0012 remains blocking formula evidence authority; importer tests are supplemental only. |
| Future live reload mutates Resource graph as gameplay config. | Low | High | Live reload must reload, validate/normalize, and atomically swap DTO/table data through a later accepted design. |

## Performance Implications

| Metric | Before | Expected After | Budget |
|--------|--------|---------------|--------|
| CPU (frame time) | No `.tres` pipeline. | No normal gameplay frame cost. Resource import/normalization occurs in bootstrap/importer paths or explicit future reload, never per-frame formula paths. | Overall project frame budget 16.6 ms at 60 fps; no per-frame ResourceLoader or config normalization. |
| Memory | No `.tres` pipeline. | Temporary loaded Resources and importer DTOs may allocate during bootstrap/tests; runtime retains normalized DTO/table data only, not duplicate Resource graphs. | Phase 1 client under 1 GB RAM. |
| Load Time | No `.tres` pipeline. | Future `.tres` import pays Resource load plus schema validation and normalization. Acceptable only at bootstrap/import time or explicit reload flow. | Profile after external authoring is introduced. |
| Network | Not applicable for Phase 1 offline slice. | Not applicable. | None. |

## Migration Plan

No existing Character Attributes implementation needs migration.

1. Continue Phase 1 with ADR-0011 factory-first fixture/config payloads.
2. If `.tres` authoring is introduced later, define Resource classes as authoring envelopes only.
3. Implement `AttributeResourceImportPolicy` or equivalent versioned schema before loading `.tres` into Character Attributes config.
4. Implement `AttributeGodotResourceConfigImporter` that reads only explicit schema fields and reconstructs primitive semantic payloads or DTO rows.
5. Route importer output through ADR-0011 `AttributeConfigNormalizer`.
6. Add importer/bootstrap integration tests for ResourceLoader cache behavior, root/nested mutation isolation, unsupported payload rejection, failure ordering, `duplicate()` insufficiency, and any `duplicate_deep()` behavior used.
7. Keep formula/contract GUT tests using primitive payloads and DTO/table injections without ResourceLoader/FileAccess/real `.tres`.

**Rollback plan**: If `.tres` importer complexity exceeds Phase 1 needs, do not introduce `.tres`; retain GDScript factories or choose another primitive envelope in a later ADR while preserving this ADR's non-authoritative Resource graph policy.

## Validation Criteria

- [ ] Runtime Character Attributes core, snapshots, transactions, result DTOs, save truth, and runtime config tables do not retain loaded or duplicated Resource graphs as authoritative state.
- [ ] `AttributeGodotResourceConfigImporter` reads only fields explicitly declared by a versioned import policy/schema.
- [ ] Unknown exported fields fail or are ignored only according to explicit forward-compatible policy.
- [ ] Importer rejects unsupported `Object`, `Resource`, `RefCounted`, `Node`, `Callable`, `Signal`, and arbitrary script object payloads by default.
- [ ] Importer distinguishes root Resource envelope from nested Resource/Object payloads.
- [ ] Importer failures are ordered by explicit import stage, schema field order, registry order, and sorted semantic keys, not Resource property order, `.tres` serialization order, dictionary iteration, or editor display order.
- [ ] Mutating the loaded root Resource after import cannot change normalized runtime config tables.
- [ ] Mutating exported arrays/dictionaries or nested containers after import cannot change normalized runtime config tables.
- [ ] `Resource.duplicate()` is covered by a regression test proving it is insufficient or not used for nested isolation.
- [ ] Any `Resource.duplicate_deep()` use is covered by Godot 4.6.3 tests for nested Resources, subresources, external references, exported arrays/dictionaries, and post-normalization isolation.
- [ ] ResourceLoader cache/object identity behavior does not affect importer output or gameplay correctness.
- [ ] Formula-only Character Attributes tests do not load `.tres`, call ResourceLoader, call FileAccess, or use Resource-backed fixtures.
- [ ] Importer/bootstrap integration tests are labeled supplemental and cannot substitute for ADR-0012 formula/contract evidence.
- [ ] Importer output contains primitive payload/DTO/table data only and no original Resource or Resource-derived object graph.

## GDD Requirements Addressed

| GDD Document | System | Requirement | How This ADR Satisfies It |
|-------------|--------|-------------|--------------------------|
| `design/gdd/character-attributes-system.md` | Character Attributes | Implementation-blocking ADR item 8: Godot Resource duplication/shared-reference policy if `.tres` Resources are used. | Defines `.tres` as optional authoring envelope only and specifies Resource duplication, cache, nested payload, importer, and test boundaries. |
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-01 Immutable Authoritative Snapshot requires attempts to mutate exposed snapshot data to be impossible, rejected, or proven not to change authoritative state. | Forbids Resource graphs and mutable Resource-derived containers from snapshots/result authority and requires immutable-by-contract DTO/table output. |
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-03 Data-Driven Provisional Fixtures require fixture/config values to load from explicit sources or injected test data without hardcoding gameplay values. | Preserves future `.tres` as explicit authoring input while requiring ADR-0011 validation/normalization before gameplay use. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Formula-only GUT strategy must avoid scene tree, UI, Autoload, filesystem, ResourceLoader, and `.tres` for blocking evidence. | Separates Resource importer/bootstrap tests from ADR-0012 blocking formula/contract tests. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Save/load persists authoritative inputs and versions, not final derived stats or engine object graphs as truth. | Forbids Resource graphs, ResourceLoader cache entries, and duplicated Resources as save truth; importer output must be primitive semantic DTO/table data. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Performance rules require no per-frame structural rebuild/config reload and no hot-path file/config loading. | Restricts `.tres` loading/import to bootstrap/importer paths or future explicit reload design, never formula/runtime hot paths. |

## Related Decisions

- `design/gdd/character-attributes-system.md`
- `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`
- `docs/architecture/adr-0007-attribute-snapshot-query-api-shape-and-read-only-enforcement.md`
- `docs/architecture/adr-0008-attribute-event-signal-contract-and-scene-tree-independent-core.md`
- `docs/architecture/adr-0009-attribute-atomic-source-update-and-transaction-model.md`
- `docs/architecture/adr-0010-attribute-save-load-persistence-boundary-base-current-modifier-sources.md`
- `docs/architecture/adr-0011-attribute-fixture-config-loading-strategy-mvp-provisional-values.md`
- `docs/architecture/adr-0012-attribute-formula-only-gut-test-strategy-without-scene-tree-ui-autoload.md`
- `docs/registry/architecture.yaml` — should be updated with Resource authoring boundary, Resource runtime authority bans, duplicate/deep-copy policy, ResourceLoader cache authority ban, formula fixture boundary, and schema/reflection importer guardrails after this ADR.
