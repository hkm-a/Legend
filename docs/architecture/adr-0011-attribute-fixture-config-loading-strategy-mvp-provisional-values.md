# ADR-0011: Attribute Fixture Config Loading Strategy for MVP Provisional Values

## Status

Accepted

## Date

2026-06-04

## Last Verified

2026-06-04

## Decision Makers

hkm + Claude Code Game Studios

## Summary

This ADR defines how Phase 1 Character Attributes loads and validates MVP provisional fixture/config values before gameplay use. We choose project-owned typed fixture/config factories plus primitive semantic-key fixture payloads as the initial strategy, producing validated immutable-by-contract runtime config DTO/table objects injected into the scene-tree-independent Character Attributes core. External JSON or `.tres` authoring formats may be introduced later, but they must pass through the same normalization and validation boundary before becoming gameplay config.

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Core / Scripting / Config Loading |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff and must be verified against engine reference docs. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md`; `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`; `docs/architecture/adr-0007-attribute-snapshot-query-api-shape-and-read-only-enforcement.md`; `docs/architecture/adr-0008-attribute-event-signal-contract-and-scene-tree-independent-core.md`; `docs/architecture/adr-0009-attribute-atomic-source-update-and-transaction-model.md`; `docs/architecture/adr-0010-attribute-save-load-persistence-boundary-base-current-modifier-sources.md` |
| **Post-Cutoff APIs Used** | None required by the Phase 1 Character Attributes core. This ADR relies on standard GDScript `RefCounted`, typed methods/arrays, explicit runtime validation, and injected factory-built DTOs. `Resource.duplicate_deep()` is Godot 4.5+ and is not selected as the preferred config boundary; if a future `.tres` implementation relies on its exact modes/options, it must be re-verified against Godot 4.6.3 docs. |
| **Verification Required** | Verify `Array` / `Dictionary` shallow/deep copy behavior; typed Array/Dictionary behavior after Variant/JSON decode; JSON numeric type/range handling if JSON is used; ResourceLoader shared/cached Resource behavior if `.tres` authoring is introduced; `Resource.duplicate_deep()` exact behavior if used; `FileAccess.open()` and Godot 4.4+ `store_*` return behavior if external config files are read/written; deterministic failure ordering independent of raw Dictionary iteration; no FileAccess/ResourceLoader/SceneTree/Autoload dependency in formula-only config tests. |

> **Note**: If Knowledge Risk is MEDIUM or HIGH, this ADR must be re-validated if the project upgrades engine versions. Flag it as "Superseded" and write a new ADR.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | ADR-0006 Attribute Data Representation and Stat ID Typing; ADR-0007 Attribute Snapshot Query API Shape and Read-Only Enforcement; ADR-0008 Attribute Event Signal Contract and Scene-Tree-Independent Core; ADR-0009 Attribute Atomic Source Update and Transaction Model; ADR-0010 Attribute Save Load Persistence Boundary; approved Character Attributes GDD: `design/gdd/character-attributes-system.md`. |
| **Enables** | Attribute stat registry fixture implementation; MVP provisional class/monster/equipment fixture setup; config validation tests; formula-only GUT setup using injected fixtures; combat power provisional weighting fixtures; future external authoring pipeline migration. |
| **Blocks** | Any Character Attributes implementation story that loads or validates stat registry rows, stat/resource bounds, actor-type required stat sets, class fixtures, monster fixtures, equipment modifier fixtures, combat power weights, source status labels, OpenMir2 evidence labels, or config/schema versions. |
| **Ordering Note** | This ADR defines Phase 1 fixture/config loading and validation. It does not choose a full external data pipeline, editor tooling, localization file format, OpenMir2 evidence harvesting process, or permanent balance-data authoring workflow. Later JSON/Resource/data-pipeline ADRs must preserve the normalization/validation boundary defined here. |

## Context

### Problem Statement

The Character Attributes GDD requires base stat fixtures, stat bounds, resource bounds, monster templates, combat power weights, equipment flat modifiers, and source-status labels to load from explicit config/fixture sources or injected test data. It also forbids hardcoded gameplay values in implementation formulas and requires every value/formula label to carry `source_status`.

Before implementation stories can start, the project must decide how MVP provisional values enter the Character Attributes core. The decision must support formula-only GUT tests without scene tree, UI, Autoload, or a full external data pipeline, while avoiding raw dictionary schemas, mutable Godot Resources, enum ordinal fixtures, or scattered formula constants.

### Current State

No Character Attributes implementation exists. ADR-0006 through ADR-0010 already define stat identity, snapshot/query read-only boundaries, event/result semantics, transaction atomicity, and save/load persistence semantics. The remaining gap is how Phase 1 config and fixtures are authored, injected, normalized, validated, versioned, and labeled before formula/rebuild code uses them.

### Constraints

- Phase 1 needs fast implementation and deterministic tests; a full designer-facing external data pipeline would be premature.
- GDScript factories may be used as config/fixture sources, but gameplay formulas must not hardcode provisional values.
- Character Attributes core must remain scene-tree-independent.
- Runtime config authority must be validated and immutable-by-contract after normalization.
- External fixture keys and version labels must use semantic strings/StringName boundaries, not enum ordinals.
- Godot `Resource` / `.tres`, JSON, `Array`, `Dictionary`, and `RefCounted` all have reference/Variant/copy behavior that can leak mutable state if used carelessly.
- OpenMir2-authentic labels require evidence; provisional values must be visibly labeled as provisional or evidence-pending.

### Requirements

- Provide a Phase 1 fixture/config authoring approach that satisfies GDD AC-03 without requiring a full asset/data pipeline.
- Define the config domains required by Character Attributes.
- Normalize semantic keys to ADR-0006 runtime IDs before formula/rebuild use.
- Validate all config before gameplay snapshot publication.
- Produce deterministic ordered failure reasons for config failures.
- Keep formula-only tests independent of filesystem, ResourceLoader cache, SceneTree, Autoload, and Nodes.
- Preserve a migration path to external JSON/Resource/data-pipeline authoring later.

## Decision

Phase 1 Character Attributes uses project-owned typed fixture/config factories plus primitive semantic-key fixture payloads as the initial authoritative config-loading strategy for MVP provisional values.

These factories are centralized fixture/config sources, not formula constants. Formula/runtime code must not hardcode gameplay values such as base stats, bounds, combat power weights, monster template stats, equipment modifier values, or source-status labels. Factory output still passes through the same validation/normalization pipeline as any later external JSON or `.tres` input. Boundary-value tests may inline values only when the exact value is the subject of the test.

The fixture/config data set includes:

- stat registry rows;
- actor-type required stat sets;
- stat bounds;
- resource bounds;
- min/max pair definitions;
- class/base stat fixtures;
- monster template fixtures;
- equipment flat modifier fixtures;
- combat power weights;
- resource correction policy labels/defaults needed by structural max-resource changes, spawn initialization, and load validation where configurable;
- source status labels;
- OpenMir2 evidence labels/references where applicable;
- attribute schema version;
- stat registry version;
- fixture config version;
- source status/evidence version;
- failure reason registry order;
- visible/debug stat display metadata keys;
- visible delta summary ordering/cap metadata where player-facing summaries depend on config.

Runtime config authority is validated normalized immutable-by-contract DTO/table objects injected into Character Attributes core. Runtime config authority is not raw JSON, raw `Dictionary`, mutable `Resource` / `.tres`, `ResourceLoader` cache, enum ordinals, positional dense vector literals, hardcoded formula constants, or engine object graphs.

Dense vectors remain allowed after stat registry normalization as runtime storage keyed by validated `StatId` per ADR-0006. What this ADR forbids is dense vector literals, enum ordinal arrays, or positional fixture rows as external fixture/config truth unless they are produced by the normalizer from semantic-key input and versioned registry metadata.

Preferred Phase 1 implementation uses `RefCounted` or static GDScript factory/builder classes that construct typed config DTOs for tests and bootstrap. Factory scripts/classes must not be `Node`s, Autoloads, SceneTree-dependent, ResourceLoader-dependent, or filesystem-dependent for formula-only tests. GUT tests should directly instantiate/build DTOs and inject normalized config into Character Attributes core.

External JSON or `.tres` may be introduced later as authoring/envelope formats. If introduced, they load/decode outside formula/runtime hot paths, recursively validate, normalize/copy to plain DTO/table objects, and then cease to be runtime authority. `ResourceLoader`, `FileAccess`, and file paths are composition/bootstrap responsibilities, not Character Attributes formula/runtime hot path responsibilities.

Config validation fails deterministically before gameplay snapshot publication for:

- unknown stat/resource/source-status keys;
- missing required stat registry rows;
- missing actor-type required stat sets;
- missing class/monster/equipment fixture rows required by the test or actor under construction;
- inverted stat/resource bounds;
- invalid pair definitions;
- unsupported modifier operations;
- duplicate source keys;
- duplicate/colliding semantic keys;
- source-authentic claims without evidence;
- `openmir2_verified` without evidence ID/source reference;
- provisional values without `source_status`;
- out-of-range numeric values;
- all-zero combat power weights;
- invalid display metadata references required by visible delta output;
- version mismatch without migration.

Deterministic failure order must come from explicit validation stage order, failure reason registry order, stat/resource registry order, and sorted semantic key order where needed. Implementation must not rely on raw `Dictionary` iteration order to define cross-test or cross-version failure ordering.

Attribute config tables are validated and normalized before any actor structural transaction can commit. A config validation failure blocks structural rebuild publication and returns structured config failure through the same failure/result vocabulary used by ADR-0008 and ADR-0009. Config validation failure must not advance `source_version` or `snapshot_version`.

Config versions are explicit. External payload stores version labels as strings; runtime may normalize them to `StringName`. `attribute_schema_version` is the Character Attributes schema version exposed as `schema_version` in ADR-0010 persistence DTOs. `fixture_config_version` is the attribute config version exposed as `config_version` where Save/Load needs one compact version label. `stat_registry_version` identifies the stat registry mapping. `source_status/evidence_version` identifies the source-status and evidence-label vocabulary. If multiple sub-versions are stored, Save/Load must persist enough labels for deterministic migration.

Changing fixture values requires a version bump or documented reason. The documentation may be a fixture changelog entry, ADR revision, GDD revision, or test fixture note, depending on scope. Silent fixture value changes are forbidden because they make save/load migration, QA evidence, and balance verification ambiguous.

MVP provisional values are allowed only with `mvp_provisional` or `openmir2_evidence_pending` labels. `openmir2_verified` requires evidence ID/source reference. Config validation rejects OpenMir2-authentic claims without evidence.

Valid config is frozen-by-contract. Config DTO/table rows expose no public mutable fields, setters, mutator methods, mutable arrays/dictionaries, or Resources. Constructors/factories copy inbound containers. Getters return scalars, defensive copies, or immutable-by-contract row DTOs. `RefCounted` row DTOs avoid cyclic references and do not rely on reference identity for correctness.

`Array` and `Dictionary` assignment is not a defensive copy. `duplicate()` is shallow by default. Nested containers are recursively normalized; Object, `RefCounted`, or `Resource` values inside containers must be reconstructed through validating factories or rejected. Typed containers improve static checking but do not replace runtime validation, especially after Variant/JSON decode.

If JSON is used later, JSON is only a primitive envelope. Godot JSON parse results are untrusted `Variant` graphs. Enum values, `StringName`, typed arrays, DTOs, and Resources must be explicitly reconstructed. Numeric values must be verified for integer semantics and configured safe ranges. Large IDs/hashes should be encoded as strings where numeric precision or coercion ambiguity matters.

If `.tres` / Resources are used later, loaded Resources are read-only authoring/config inputs. Godot loaded Resources may be shared/cached by `ResourceLoader` or editor references; runtime must not mutate them to express config state. `duplicate()` is not sufficient for nested Resource isolation. `duplicate_deep()` is a Godot 4.5+ Resource isolation fallback after pinned-version verification, but the preferred boundary remains loaded Resource -> validating factory -> plain DTO/table objects.

If external files are loaded or written by bootstrap/composition tooling, IO success/failure remains separate from config validation success/failure. Character Attributes core must not call `FileAccess`. Bootstrap/composition code that uses `FileAccess.open()` must check open failure; code that writes config/cache/output with Godot 4.4+ `FileAccess.store_*` must check boolean return values.

Config reload/change notification is out of scope for Phase 1. Any future live reload that changes gameplay config must be routed through an accepted update/transaction design and must not bypass Character Attributes validation, versioning, or result semantics.

### Architecture Diagram

```text
Phase 1 fixture/config source
(GDScript factory payloads; later JSON/.tres envelope optional)
        |
        | semantic strings + primitive values + source_status + versions
        v
Config factory / bootstrap builder
(RefCounted/static, no Node/Autoload/SceneTree/filesystem dependency in formula tests)
        |
        | recursive copy + semantic validation + migration if defined
        v
Attribute config normalizer
        |
        +--> String/StringName keys -> StatId/resource IDs
        +--> actor-type required sets
        +--> bounds/pairs/resources/equipment/class/monster tables
        +--> source_status/evidence labels
        +--> failure reason order/display metadata
        v
Validated immutable-by-contract runtime config tables
        |
        | injected into Character Attributes core
        v
Structural rebuild / preview / formula tests
        |
        v
Snapshot publication only if config + source + output validation pass
```

### Key Interfaces

```gdscript
class_name AttributeFixtureConfigFactory
extends RefCounted

func build_phase1_fixture_payload() -> AttributeFixtureConfigPayload:
    pass
```

```gdscript
class_name AttributeFixtureConfigPayload
extends RefCounted

func get_schema_version_key() -> StringName:
    pass

func get_stat_registry_version_key() -> StringName:
    pass

func get_fixture_config_version_key() -> StringName:
    pass

func get_source_status_version_key() -> StringName:
    pass

func get_stat_rows_copy() -> Array[AttributeStatConfigRow]:
    pass

func get_resource_rows_copy() -> Array[AttributeResourceConfigRow]:
    pass

func get_pair_rows_copy() -> Array[AttributePairConfigRow]:
    pass

func get_actor_type_required_sets_copy() -> Array[AttributeActorTypeRequiredSetRow]:
    pass

func get_class_fixture_rows_copy() -> Array[AttributeActorFixtureRow]:
    pass

func get_monster_fixture_rows_copy() -> Array[AttributeActorFixtureRow]:
    pass

func get_equipment_modifier_fixture_rows_copy() -> Array[AttributeModifierFixtureRow]:
    pass

func get_combat_power_config() -> AttributeCombatPowerConfig:
    pass
```

```gdscript
class_name AttributeConfigNormalizer
extends RefCounted

func normalize(payload: AttributeFixtureConfigPayload) -> AttributeConfigValidationResult:
    pass
```

```gdscript
class_name AttributeConfigValidationResult
extends RefCounted

func is_success() -> bool:
    pass

func get_config_tables() -> AttributeRuntimeConfigTables:
    pass

func get_failure_reasons_copy() -> Array[int]:
    pass
```

```gdscript
class_name AttributeRuntimeConfigTables
extends RefCounted

func get_schema_version_key() -> StringName:
    pass

func get_stat_registry_version_key() -> StringName:
    pass

func has_stat(stat_id: int) -> bool:
    pass

func get_stat_bounds(stat_id: int) -> AttributeStatBoundsQueryResult:
    pass

func get_required_stats_for_actor_type(actor_type_id: int) -> Array[int]:
    pass

func get_combat_power_config() -> AttributeCombatPowerConfig:
    pass
```

The interfaces are conceptual. Exact filenames, enum ownership, constructors, and whether factories are static helpers or `RefCounted` classes may be refined during implementation. Each `class_name` should live in its own `.gd` file where applicable.

## Alternatives Considered

### Alternative 1: Direct `.tres` Resources as runtime config authority

- **Description**: Author stat config, actor fixtures, and equipment modifier fixtures as Godot Resources and keep those Resources as runtime config authority.
- **Pros**: Editor-friendly; convenient inspector editing; natural Godot workflow.
- **Cons**: Resources are reference objects and may be shared/cached; exported fields are mutable; nested duplication is engine-sensitive; formula-only tests may depend on ResourceLoader/filesystem; conflicts with ADR-0006 Resource guardrails.
- **Rejection Reason**: `.tres` may be a future authoring input, but runtime authority must be normalized plain DTO/table data.

### Alternative 2: Loose JSON/Dictionary maps as runtime schema

- **Description**: Store fixtures in JSON or dictionaries and pass those dictionaries directly into formulas/rebuild logic.
- **Pros**: Easy to inspect and edit; quick to prototype; no custom DTOs required.
- **Cons**: Variant-heavy, weak typing, typo-prone, mutable aliasing risks, uncertain failure order, and repeated string-key lookup in hot paths.
- **Rejection Reason**: JSON/Dictionary may be an input envelope, not authoritative runtime config schema.

### Alternative 3: Hardcoded gameplay constants in formulas

- **Description**: Put MVP bounds, base stats, weights, and template values directly in formula/rebuild implementation code.
- **Pros**: Fastest initial coding; minimal factory/schema boilerplate.
- **Cons**: Violates GDD AC-03; hides source_status/evidence labels; makes balance changes and tests brittle; prevents save/load/config migration reasoning.
- **Rejection Reason**: Factories are centralized fixture/config sources; formula/runtime code must not hardcode gameplay values.

### Alternative 4: Enum ordinal arrays / dense vector fixtures as external truth

- **Description**: Define fixture rows as positional arrays ordered by `StatId` enum ordinal.
- **Pros**: Compact; fast to normalize; resembles runtime dense vectors.
- **Cons**: Enum/order changes corrupt fixture meaning; hard to review; conflicts with ADR-0006 and ADR-0010 semantic-key boundaries.
- **Rejection Reason**: Dense vectors are allowed only after semantic-key normalization; fixture/config truth uses semantic keys and versions.

### Alternative 5: Full external data pipeline before Phase 1

- **Description**: Build a complete editor/data pipeline with external files, import validation, designer tooling, and generated runtime config before Character Attributes implementation.
- **Pros**: More scalable and designer-friendly long term.
- **Cons**: Over-scoped for Phase 1; delays formula/test implementation; increases pipeline risk before core rules are proven.
- **Rejection Reason**: Factory-first fixtures are less designer-friendly but acceptable for Phase 1 to keep tests deterministic and avoid premature pipeline work. Later external authoring can preserve this ADR's normalization boundary.

## Consequences

### Positive

- Formula-only GUT tests can inject deterministic config without SceneTree, Autoload, ResourceLoader, or filesystem.
- GDD-required MVP provisional values are data/config-owned rather than scattered in formula code.
- Source-status and OpenMir2 evidence labels are enforced before gameplay publication.
- Runtime formulas use validated normalized tables and ADR-0006 `StatId` identities.
- Later JSON/.tres/data-pipeline authoring can be introduced without changing formula/runtime authority.
- Deterministic failure ordering supports stable tests and debug evidence.

### Negative

- GDScript factory fixtures are less designer-friendly than external JSON/.tres authoring and require programmer involvement for value changes in Phase 1.
- More DTOs, validation code, and boilerplate are required than hardcoded constants or loose dictionaries.
- Fixture changes need version bumps or documented reasons, adding process overhead.
- External authoring still requires a later pipeline/design pass.
- Immutable-by-contract DTOs rely on code discipline and tests rather than engine-enforced immutability.

### Neutral

- This ADR does not forbid future `.tres`, JSON, YAML, CSV, generated scripts, or editor tooling as authoring envelopes.
- This ADR does not decide final balance values or OpenMir2 authenticity.
- This ADR does not decide localization file format for display labels.
- This ADR does not define live config reload behavior beyond forbidding authority bypass in Phase 1.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| Factory fixtures are mistaken for formula hardcoding. | Medium | High | Factories are centralized config sources; formulas cannot reference gameplay constants directly; validation pipeline applies to factory output. |
| Designers cannot easily edit values in Phase 1. | Medium | Medium | Accept for Phase 1; later external authoring envelope can preserve normalization boundary. |
| Dense vector fixture literals corrupt meaning after registry changes. | Medium | High | External fixture truth uses semantic keys; dense vectors only after normalization. |
| Config DTOs expose mutable containers. | Medium | High | No public mutable fields/setters/mutators; constructors copy; getters return scalars/copies/immutable rows; tests mutate inputs/returned collections. |
| Raw Dictionary iteration causes nondeterministic failure order. | Medium | Medium | Failure order comes from explicit validation stage, registry, and sorted-key order. |
| ResourceLoader returns shared/cached Resources that mutate config authority. | Medium | High | Resources are read-only authoring inputs; normalize/copy to DTOs; runtime does not hold Resource authority. |
| JSON parse produces loose Variant data with unsafe numeric types. | Medium | Medium | Treat JSON as untrusted primitive envelope; validate types/ranges/integer semantics and encode large IDs/hashes as strings if needed. |
| Config validation failure partially initializes actor state. | Low | High | Validation must complete before actor structural transaction commit; failures do not advance versions or publish snapshots. |
| Provisional values get mislabeled as OpenMir2 verified. | Medium | Medium | `openmir2_verified` requires evidence ID/source reference; validation rejects missing evidence. |
| Future live reload bypasses transactions. | Low | High | Live reload out of scope; future config changes require accepted update/transaction design. |

## Performance Implications

| Metric | Before | Expected After | Budget |
|--------|--------|---------------|--------|
| CPU (frame time) | No implementation. | Config normalization is O(number of config rows) at bootstrap/test setup. Structural rebuild uses normalized tables and remains O(M + S). No per-frame full config reload. | Overall project frame budget 16.6 ms at 60 fps. |
| Memory | No implementation. | Runtime config tables allocate row DTOs/scalar arrays once per fixture/config set. No retained raw JSON/Resource authority. | Phase 1 client under 1 GB RAM. |
| Load Time | No implementation. | Bootstrap/test setup pays validation/normalization cost; acceptable for Phase 1. | Profile after implementation if external files are introduced. |
| Network | Not applicable for Phase 1 offline slice. | Not applicable. | None. |

## Migration Plan

No existing Character Attributes implementation needs migration.

1. Define config row DTOs for stat registry, resource bounds, pair definitions, actor-type required sets, actor fixtures, modifier fixtures, combat power weights, source status/evidence labels, and display metadata keys.
2. Define `AttributeFixtureConfigFactory` or equivalent centralized GDScript factory/builder for Phase 1 MVP provisional fixture payloads.
3. Implement `AttributeConfigNormalizer` to validate semantic keys, versions, bounds, pairs, source labels, evidence, numeric ranges, duplicate keys, and failure ordering.
4. Implement immutable-by-contract `AttributeRuntimeConfigTables` injected into Character Attributes core.
5. Add formula-only GUT tests that instantiate factories and normalizer without Node, SceneTree, Autoload, ResourceLoader, or filesystem.
6. Add mutation/aliasing tests for payload arrays/dictionaries/DTOs after normalization.
7. Add deterministic failure-order tests for multi-failure config cases.
8. If later JSON or `.tres` authoring is introduced, route it through the same payload/normalizer boundary and add engine-specific IO/Resource tests.

**Rollback plan**: If factory fixtures become too cumbersome, supersede this ADR with an external authoring envelope while preserving semantic-key input, validation/normalization, immutable runtime config tables, source-status/evidence enforcement, and no runtime Resource/Dictionary authority.

## Validation Criteria

- [ ] Phase 1 fixture/config values are supplied by centralized factory/payload sources or injected test data, not hardcoded in formula/runtime code.
- [ ] Factory output passes through the same config validation/normalization pipeline required for future external inputs.
- [ ] Valid config produces immutable-by-contract runtime config tables injected into Character Attributes core.
- [ ] Unknown keys, missing required rows, inverted bounds, invalid pairs, unsupported modifier operations, duplicate/colliding semantic keys, invalid source_status, out-of-range numeric values, all-zero combat power weights, and version mismatch without migration fail deterministically.
- [ ] Failure reason ordering is stable and does not depend on raw Dictionary iteration.
- [ ] `openmir2_verified` without evidence ID/source reference is rejected.
- [ ] Provisional fixture values without `source_status` are rejected.
- [ ] Config validation failure blocks structural rebuild publication and does not advance `source_version` or `snapshot_version`.
- [ ] Mutating primitive fixture payload arrays/dictionaries after normalization cannot alter validated config tables, staged rebuild behavior, or published snapshots.
- [ ] Getters on runtime config tables do not expose mutable arrays/dictionaries/Resources.
- [ ] Formula-only tests instantiate config factories/normalizer without FileAccess, ResourceLoader, SceneTree, Autoload, Nodes, signals, or filesystem fixtures.
- [ ] If JSON is introduced, parsed Variant data is recursively validated and normalized before use.
- [ ] If `.tres` / Resource input is introduced, mutating the loaded Resource after normalization cannot mutate runtime config authority.
- [ ] Fixture value changes include version bump or documented reason.
- [ ] Dense runtime vectors are produced only after semantic-key normalization and are not external fixture truth.

## GDD Requirements Addressed

| GDD Document | System | Requirement | How This ADR Satisfies It |
|-------------|--------|-------------|--------------------------|
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-03 requires base stat fixtures, stat bounds, resource bounds, monster template fixtures, combat power weights, and equipment modifier fixtures to load from explicit config/fixture sources or injected test data. | Defines centralized typed factory/payload fixtures plus validation/normalization as the Phase 1 config-loading strategy. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Gameplay implementation must not hardcode provisional or OpenMir2-authentic stat values. | Separates centralized fixture/config sources from formula/runtime code and forbids hardcoded gameplay constants in formulas. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Every value/formula label has `source_status`; `openmir2_verified` requires evidence ID/source reference. | Requires source-status/evidence labels in config and deterministic validation failure for missing evidence or unlabeled provisional values. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Validation pipeline Stage A requires all required stat definitions, bounds, resource bounds, pair config, stat statuses, modifier policies, and source-authentic labels to be valid before rebuild. | Defines config domains and validation failures that block snapshot publication before actor transactions commit. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Performance rules require no per-frame structural rebuild/config reload and runtime hot paths to use compact IDs, not string lookups. | Normalizes semantic fixture keys into runtime config tables and forbids per-frame config reload/hot-path ResourceLoader/FileAccess usage. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Combat power formula requires provisional non-negative weights and failure when all active player-facing weights are zero. | Includes combat power weights in config and validates all-zero combat power weights as deterministic config failure. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Implementation-blocking ADR item 6: fixture/config loading strategy for MVP provisional values. | Directly resolves the fixture/config loading strategy prerequisite while leaving full external data pipeline design to later ADRs. |

## Related Decisions

- `design/gdd/character-attributes-system.md`
- `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`
- `docs/architecture/adr-0007-attribute-snapshot-query-api-shape-and-read-only-enforcement.md`
- `docs/architecture/adr-0008-attribute-event-signal-contract-and-scene-tree-independent-core.md`
- `docs/architecture/adr-0009-attribute-atomic-source-update-and-transaction-model.md`
- `docs/architecture/adr-0010-attribute-save-load-persistence-boundary-base-current-modifier-sources.md`
- `docs/registry/architecture.yaml` — should be updated with fixture config loading, config validation, source status, normalization boundary, and forbidden config anti-patterns after this ADR.
