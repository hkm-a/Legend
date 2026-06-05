# ADR-0006: Attribute Data Representation and Stat ID Typing

## Status

Accepted

## Date

2026-06-04

## Last Verified

2026-06-04

## Decision Makers

hkm + Claude Code Game Studios

## Summary

This ADR defines the authoritative runtime data representation and stat ID typing rules for the Character Attributes system. We choose project-owned compact typed `StatId` integer IDs for runtime authority, stable `StringName` semantic keys only at external boundaries, dense validated runtime vectors for stat values where appropriate, and typed DTO/source payloads instead of raw string or dictionary maps.

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Core / Scripting |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff and must be verified against engine reference docs. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md` |
| **Post-Cutoff APIs Used** | None required. This ADR relies on standard GDScript enums, typed variables/arrays, `RefCounted`, `Resource`, and explicit runtime validation. It does not depend on post-cutoff-only APIs such as `@abstract`, variadic arguments, or `Resource.duplicate_deep()`. |
| **Verification Required** | Verify enum range validation, append-only/tombstone ID policy, dense vector indexing bounds, integer width/overflow behavior, `Resource` shared-reference isolation, `RefCounted` DTO aliasing behavior, `StringName` to `StatId` normalization, and unsupported modifier operation failure under Godot 4.6.3. |

> **Note**: If Knowledge Risk is MEDIUM or HIGH, this ADR must be re-validated if the project upgrades engine versions. Flag it as "Superseded" and write a new ADR.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | Approved Character Attributes GDD: `design/gdd/character-attributes-system.md`. |
| **Enables** | Attribute stat registry implementation; attribute config normalization/loading technical design; base/effective/current stat vector storage implementation; modifier DTO/source input implementation; formula unit tests for `effective_stat`, `effective_stat_pair`, `current_resource_after`, and overflow failure cases; ADR for snapshot/query API shape and read-only enforcement; ADR for event/signal contract and scene-tree-independent core. |
| **Blocks** | Any implementation story that defines, stores, mutates, serializes, or consumes authoritative character attribute stat identity, base/effective/current stat arrays, modifier target IDs, or attribute source payloads. |
| **Ordering Note** | This ADR establishes stat identity and runtime data representation only. Snapshot read-only API shape, event dispatch, transaction semantics, save file format, fixture/config loading format, formula-only GUT setup, and combat power ownership remain separate required ADRs or technical designs. |

## Context

### Problem Statement

The Character Attributes GDD requires the system to own stat semantics, validation, modifier targets, current resources, snapshots, source status labels, and formula inputs while avoiding repeated string dictionary lookups in runtime hot paths. The project must decide how stat identity remains stable across three domains: runtime hot-path calculation, external config/save/debug semantics, and future migration/versioning.

Without this decision, implementers could independently choose raw strings, loose dictionaries, Godot `Resource` objects, enum ordinals, or per-stat runtime objects. Those approaches would make formulas harder to test, invite typo-driven bugs, create shared mutable state risks, and make later save/load migration unsafe.

### Current State

No Character Attributes implementation exists yet. The approved GDD explicitly blocks implementation pending ADRs, with this decision listed as the first prerequisite. Existing map-related ADRs establish a project pattern of typed DTOs, explicit authority boundaries, and avoidance of loose dictionary-only payloads for authoritative gameplay semantics.

### Constraints

- Godot 4.6.3 is post-cutoff; engine-specific Resource, typing, and collection behavior must be verified against local engine-reference docs.
- Phase 1 is an offline 2D/2.5D loot-loop slice targeting PC and GDScript.
- Attribute formulas must be unit-testable without scene tree, UI, or Autoload.
- Runtime hot paths must not perform repeated raw string / `StringName` dictionary lookup for combat/resource stat reads.
- External config, localization, debug output, save/load, and QA evidence still require stable semantic keys readable by humans and tools.
- GDScript type annotations improve correctness but do not replace explicit runtime validation.

### Requirements

- Provide one canonical runtime identity for every Phase 1 stat and reserved stat.
- Preserve stable semantic names for config, save/load, localization, display metadata, debug, and migration.
- Support actor-type-specific validation for player and monster required fields.
- Keep current-resource values such as current HP/MP out of ordinary modifier-targetable stat policy, even if represented by flagged `StatId` entries.
- Support Phase 1 `add_flat` modifier target validation and deterministic rejection of unsupported operations.
- Support overflow-safe formula tests and technical-limit failure behavior.
- Avoid exposing mutable runtime backing arrays, dictionaries, or shared Resources as authoritative gameplay state.

## Decision

Character Attributes runtime uses project-owned compact typed integer IDs as authoritative in-memory stat identity. The canonical type is `StatId`, represented by a stable GDScript enum or equivalent generated constants. Runtime calculation, modifier aggregation, validation, and consumer-facing query APIs must use `StatId`, not raw strings, as gameplay authority.

Current HP/MP values are treated as **current-resource values**, not ordinary modifier-targetable stats. Phase 1 may represent current-resource fields with dedicated `ResourceId` identities or with `StatId` entries flagged as `resource_current`, `not_modifier_targetable`, and `mutation_only_via_resource_request`; either implementation must preserve the same boundary: current resources mutate only through the ADR-0009 current-resource mutation path and must never be targeted by item/equipment stat modifiers. Max resources such as `HEALTH_MAX` and `MANA_MAX` remain normal effective/max stat identities that may participate in formulas and allowed modifier policy.

External boundaries use stable `StringName` semantic keys. Config files, save/load payloads, localization/display metadata, debug dumps, QA evidence, and editor-facing data may refer to stats by semantic key such as `physical_attack_min`. Load/normalization code must validate each external key and map it to a current `StatId` before runtime use.

`StatId` has these compatibility rules:

- IDs are contiguous and zero-based for dense vector indexing.
- The enum or generated ID table includes a `COUNT` sentinel for array sizing and validation.
- IDs are append-only. Existing IDs must not be reordered or reused.
- Removed stats become tombstones or require an explicit versioned migration table.
- Enum ordinals are not durable config/save truth unless a versioned migration table proves compatibility.
- Every runtime index operation must validate `0 <= stat_id < StatId.COUNT` before accessing dense vectors.

Runtime stat values may use `Array[int]`, `PackedInt32Array`, or `PackedInt64Array` depending on documented numeric range and implementation needs. Phase 1 stat values are expected to fit small technical safe ranges from the GDD, but formula implementations must still detect invalid range, overflow, or truncation conditions and return structured failure rather than wrapping or silently clamping outside approved policy.

Static stat definitions may be authored as Godot `Resource` / `.tres` assets, injected fixtures, or another config format selected by a later fixture/config ADR. Regardless of authoring format, runtime initialization must normalize and copy the data into immutable-by-contract registry tables before gameplay use. Godot `Resource` objects are authoring inputs, not authoritative mutable runtime attribute state.

Attribute source payloads use typed DTO classes, preferably `RefCounted` for scene-tree-independent data. DTOs include at minimum concepts equivalent to `AttributeBaseInput`, `AttributeModifier`, `AttributeModifierSourceKey`, and `AttributePersistentInput`. Because `RefCounted` objects are reference types, implementations must avoid shared mutable aliasing by copying boundary data, using constructor/factory validation, and not exposing internal mutable arrays or dictionaries to consumers.

`ModifierOperation` and source/status labels use typed enums or equivalent validated constants. Phase 1 accepts only `ADD_FLAT` as a gameplay-active modifier operation. Unsupported operations such as percent, multiply, or override must fail deterministically during validation; they must not enter runtime aggregation as ignored, coerced, or partially supported operations unless a later accepted ADR revises the policy.

Snapshot/query API shape and read-only enforcement are deferred to the next ADR. This ADR nevertheless forbids raw `Dictionary[StringName, Variant]`, raw string maps, mutable `Resource` objects, or enum ordinals as authoritative runtime stat representation.

### Architecture

```text
External semantic data
(config/save/debug/localization/editor)
        |
        | StringName keys validated at load/normalization
        v
Attribute stat registry normalizer
        |
        | produces append-only StatId table + immutable-by-contract runtime registry
        v
Character Attributes runtime
        |
        +--> dense stat vectors keyed by StatId
        +--> typed source/modifier DTOs
        +--> deterministic validation/failure reason sets
        |
        v
Later ADR-defined snapshot/query/event APIs
        |
        v
Combat / equipment / HUD / save-load / AI / QA-debug consumers
```

### Key Interfaces

```gdscript
enum StatId {
    ACTOR_ID,
    ACTOR_TYPE,
    CLASS_ID,
    MONSTER_TEMPLATE_ID,
    LEVEL,
    HEALTH_MAX,
    # Optional current-resource StatId form. If used, these IDs must be
    # flagged resource_current + not_modifier_targetable +
    # mutation_only_via_resource_request. A separate ResourceId table may
    # replace these entries if ADR-0007/0009 implementation chooses that path.
    HEALTH_CURRENT,
    MANA_MAX,
    MANA_CURRENT,
    PHYSICAL_ATTACK_MIN,
    PHYSICAL_ATTACK_MAX,
    PHYSICAL_DEFENSE_MIN,
    PHYSICAL_DEFENSE_MAX,
    MAGIC_DEFENSE_MIN,
    MAGIC_DEFENSE_MAX,
    ACCURACY,
    EVASION,
    COUNT,
}

enum ModifierOperation {
    ADD_FLAT,
    PERCENT,
    MULTIPLY,
    OVERRIDE,
}

enum SourceStatus {
    MVP_PROVISIONAL,
    OPENMIR2_EVIDENCE_PENDING,
    OPENMIR2_VERIFIED,
}

class_name AttributeModifierSourceKey
extends RefCounted

var actor_id: int
var source_system: StringName
var source_instance_id: StringName
var slot_id: StringName
var modifier_row_id: StringName
var target_stat: int
var stacking_group: StringName

class_name AttributeModifier
extends RefCounted

var source_key: AttributeModifierSourceKey
var target_stat: int
var operation: int
var value: int
var active: bool
var source_status: int
```

Exact file locations, constructors, factories, and whether enum ownership lives in `AttributeTypes`, `AttributeStatIds`, or a generated registry script are implementation details. Before implementation, the project should standardize enum ownership so all DTO files reference the same `StatId` namespace. Public classes should follow Godot's one `class_name` per file convention.

### Implementation Guidelines

- Validate all external/config/fixture stat keys before runtime use.
- Provide explicit conversion helpers such as `stat_id_to_key()` and `key_to_stat_id()` for boundaries.
- Do not compare gameplay stats by raw string or dynamically constructed `StringName` inside combat/resource loops.
- Prefer dense vectors keyed by `StatId` for active scalar stats. Use sparse debug maps only for non-hot debug evidence when needed.
- Document numeric range before choosing `PackedInt32Array`; use `PackedInt64Array` or `Array[int]` where 32-bit truncation is possible.
- Runtime registry tables must not expose mutable backing arrays/dictionaries to consumers.
- `Resource` config must be treated as read-only authoring data and copied/normalized before gameplay use.
- DTO constructors/factories must validate enum ranges, source key completeness, operation support, numeric finite-ness/range, and source status labels.
- Modifier target validation must reject IDs or resource identities flagged `resource_current`, `not_modifier_targetable`, or `mutation_only_via_resource_request`.
- Unsupported modifier operations must produce structured failure using canonical reason IDs.
- Tests must cover enum bounds, missing key mapping, unknown key rejection, unsupported operation rejection, duplicate source keys, modifier order independence, and technical-limit/overflow behavior.

## Alternatives Considered

### Alternative 1: Raw `Dictionary[StringName, Variant]` stat maps everywhere

- **Description**: Store base stats, current resources, derived stats, modifiers, and snapshots in dictionaries keyed by semantic stat names.
- **Pros**: Fast to prototype; easy to inspect; easy to serialize; flexible when adding new stats.
- **Cons**: Repeated key lookup in hot paths; typo-prone; weak refactor safety; difficult to guarantee required coverage; values become `Variant`-heavy; consumers can invent local keys.
- **Estimated Effort**: Low initial effort, high QA and integration cost.
- **Rejection Reason**: The GDD requires compact stat IDs for runtime hot paths and deterministic validation. Raw dictionary maps are acceptable only as external/debug inputs after validation, not runtime authority.

### Alternative 2: Godot `Resource` objects as authoritative runtime stat state

- **Description**: Represent stat definitions, actor stat values, modifiers, and snapshots directly as mutable `Resource` objects.
- **Pros**: Editor-friendly; `.tres` authoring support; Godot-native data workflow; convenient inspector visibility.
- **Cons**: `Resource` instances are reference objects and may be shared/cached; accidental mutation can affect multiple consumers; deep-copy behavior must be explicit; runtime authority would be harder to isolate and unit-test.
- **Estimated Effort**: Medium initial effort, high correctness risk.
- **Rejection Reason**: Resources are appropriate as authoring/config inputs, but mutable shared Resources must not become authoritative runtime attribute state.

### Alternative 3: Hardcoded string constants without compact IDs

- **Description**: Define stat names as `const` strings or `StringName` constants and use them directly in all formulas and consumers.
- **Pros**: Human-readable; avoids enum migration concerns; simple for debugging.
- **Cons**: Still string-keyed at runtime; weak static checking; consumers can bypass registry validation; hot loops rely on key lookup; localization/display keys can be confused with semantic keys.
- **Estimated Effort**: Low initial effort.
- **Rejection Reason**: Stable semantic names remain necessary at boundaries, but runtime authority needs compact validated IDs.

### Alternative 4: Per-stat class or Resource objects in runtime vectors

- **Description**: Represent each stat as an object containing value, metadata, bounds, source breakdown, visibility, and validation status.
- **Pros**: Expressive; convenient for debug; each stat can carry rich metadata.
- **Cons**: More allocations; harder to keep hot paths compact; risks carrying debug-heavy data in normal snapshots; excessive for Phase 1 formula needs.
- **Estimated Effort**: Medium to high.
- **Rejection Reason**: Phase 1 needs simple, testable, low-overhead scalar formulas and compact resource updates. Rich per-stat objects can remain a debug/presentation layer if needed.

### Alternative 5: Godot enum ordinal as durable save/config identity

- **Description**: Serialize enum integer values directly in config/save files and rely on enum order remaining stable.
- **Pros**: Compact; fast to load; simple dense vector mapping.
- **Cons**: Reordering or deleting enum values corrupts persisted data; difficult to debug; migration requires reconstructing historical enum layouts.
- **Estimated Effort**: Low initial effort, high long-term migration risk.
- **Rejection Reason**: Runtime ordinals are implementation details. Durable external data must use stable semantic keys or a versioned migration table.

## Consequences

### Positive

- Attribute formulas and consumers use one canonical runtime stat identity.
- Combat/resource hot paths can avoid repeated string dictionary lookups.
- External data remains human-readable and migration-friendly through semantic keys.
- Unsupported modifier operations and unknown stat IDs can fail deterministically.
- Runtime registry normalization isolates Godot `Resource` sharing/mutation risks.
- This decision creates a clean foundation for snapshot, event, transaction, save/load, and formula-test ADRs.

### Negative

- Adding or removing stats now requires registry discipline, append-only IDs, and migration thinking.
- Tombstoned IDs may accumulate over time.
- Dense vectors are less convenient for sparse debug inspection than dictionaries.
- Runtime normalization adds load-time validation code and test burden.
- DTO copying and immutability guardrails add boilerplate.
- Implementers must understand the allowed boundary between `StringName` semantic keys and `StatId` runtime authority.

### Neutral

- `.tres` Resources remain viable for config authoring but are not selected as runtime authority.
- Snapshot/query immutability is intentionally deferred to a later ADR.
- This ADR does not choose exact file paths, class names, constructors, or fixture format.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| Enum ordinal drift corrupts dense vectors or persisted data. | Medium | High | `StatId` is append-only; include `COUNT`; use tombstones or versioned migration; external data uses semantic keys. |
| Packed integer width truncates large values. | Medium | High | Document numeric range before choosing packed storage; test overflow/technical limits; use `PackedInt64Array` or `Array[int]` when 32-bit is unsafe. |
| Godot `Resource` shared references mutate runtime state. | Medium | High | Treat Resources as authoring inputs only; normalize/copy into runtime tables; do not expose mutable Resource internals. |
| `RefCounted` DTO aliasing allows accidental mutation after submission. | Medium | Medium | Use validated constructors/factories; copy boundary arrays/dictionaries; avoid exposing mutable internal collections. |
| `StringName` semantic keys leak into hot-path authority. | Medium | Medium | Convert at load/normalization; expose explicit helpers; tests reject unvalidated stat keys in runtime APIs. |
| GDScript type annotations are mistaken for full validation. | Medium | Medium | Validate enum ranges, array lengths, numeric ranges, operation support, duplicate source keys, and source status at runtime boundaries. |
| Later snapshot/save ADRs need richer schema than this representation anticipates. | Low | Medium | Keep this ADR limited to identity and representation; use semantic keys and versioned registry to support later migration. |

## Performance Implications

| Metric | Before | Expected After | Budget |
|--------|--------|---------------|--------|
| CPU (frame time) | No implementation. Raw string dictionary approach would add repeated key lookup in stat consumers. | Dense vector `StatId` lookup is O(1); structural rebuild aggregation remains O(M + S) as required by GDD. | Overall project frame budget 16.6 ms at 60 fps. Structural rebuilds must not run per frame. |
| Memory | No implementation. | Dense vectors allocate fixed storage per actor for registered stats; DTOs allocate per source/modifier/result. Avoids per-stat runtime objects. | Phase 1 client under 1 GB RAM. |
| Load Time | No implementation. | Config normalization adds validation pass over stat registry, fixtures, and semantic keys. | Acceptable for Phase 1; ordinary gameplay must not full-scan or re-normalize per frame. |
| Network | Not applicable for Phase 1 offline slice. | Not applicable. | None. |

## Migration Plan

No existing Character Attributes implementation needs migration.

1. Define the `StatId`, `ModifierOperation`, `SourceStatus`, and failure reason ownership namespace.
2. Create runtime registry normalization that maps semantic `StringName` keys to `StatId` and validates append-only coverage.
3. Implement typed source/modifier DTOs with constructor/factory validation.
4. Implement dense stat vector storage for formula tests, selecting `Array[int]`, `PackedInt32Array`, or `PackedInt64Array` based on documented numeric range.
5. Add tests for key normalization, enum bounds, unsupported operation failure, Resource isolation, DTO aliasing boundaries, and overflow/technical-limit failure.
6. Defer public snapshot/query read-only API to the next ADR.

**Rollback plan**: If dense vector representation proves too rigid during early implementation, supersede this ADR with a hybrid representation that preserves `StatId` runtime authority and semantic-key boundary rules while changing only internal storage.

## Validation Criteria

- [ ] Every Phase 1 GDD stat has exactly one current `StatId` or an explicit tombstone/future status.
- [ ] Runtime APIs reject unknown, out-of-range, tombstoned, or inactive stat IDs according to the GDD policy.
- [ ] External semantic keys map deterministically to `StatId`, and unknown keys produce structured failure.
- [ ] Save/config/debug paths do not treat enum ordinal as durable truth unless a migration table is present.
- [ ] Runtime stat vector indexing validates `StatId` bounds before access.
- [ ] Unsupported modifier operations fail deterministically in Phase 1.
- [ ] Tests cover overflow/technical-limit behavior without wrap or silent truncation.
- [ ] Resource-backed config fixtures cannot mutate authoritative runtime state after normalization.
- [ ] DTO boundary tests prove consumer mutation cannot alter authoritative staged/runtime state.

## GDD Requirements Addressed

| GDD Document | System | Requirement | How This ADR Satisfies It |
|-------------|--------|-------------|--------------------------|
| `design/gdd/character-attributes-system.md` | Character Attributes | Phase 1 stat registry distinguishes required, active, visible, debug-only, reserved, unsupported, modifier-targetable, and actor-type-specific fields. | Establishes `StatId` as canonical runtime identity and requires full registry normalization/validation before gameplay use. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Attribute layers separate identity, base stats, current resources, derived stats, modifiers, snapshots, and debug trace. | Defines runtime stat vectors and typed source/modifier DTOs while preserving snapshot/query as a later explicit API layer. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Runtime hot paths should use ADR-approved compact stat IDs, not repeated string dictionary lookups in combat/resource loops. | Chooses compact typed `StatId` integer IDs and dense vectors for runtime authority. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Unsupported modifier operations and unknown stat targets must fail with structured semantics. | Defines typed `ModifierOperation`; Phase 1 accepts only `ADD_FLAT` and rejects unsupported operations deterministically. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Formula inputs must be finite integers and overflow-safe; invalid technical ranges return structured failure. | Requires documented numeric storage width, bounds validation, and overflow/truncation tests before packed storage is accepted. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Save/load persists authoritative inputs and versions, not final derived stats as truth. | Requires external durable data to use stable semantic keys or versioned migration tables rather than enum ordinals. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Implementation-blocking ADR item 1: attribute data representation and stat ID typing. | Directly resolves the representation and stat ID typing prerequisite while leaving later ADR gates scoped separately. |

## Related

- `design/gdd/character-attributes-system.md`
- `docs/architecture/adr-0002-typed-query-result-schema.md` — precedent for typed enum/DTO contracts instead of loose dictionary authority.
- `docs/registry/architecture.yaml` — updated with attribute runtime state, stat ID contract, API decision, and forbidden patterns after this ADR.
