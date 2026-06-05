# ADR-0010: Attribute Save Load Persistence Boundary for Base Current Modifier Sources

## Status

Accepted

## Date

2026-06-04

## Last Verified

2026-06-04

## Decision Makers

hkm + Claude Code Game Studios

## Summary

This ADR defines the persistence boundary between the Save/Load system and the Character Attributes system. We choose to persist `AttributePersistentInput` semantic source data as durable truth, not derived/effective `AttributeSnapshot` data. Save/Load owns file/container IO and migration orchestration; Character Attributes owns semantic interpretation, validation, load rebuild, current-resource correction acceptance, committed runtime state, and the final `AttributeUpdateResult` outcome.

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Core / Scripting / Persistence Boundary |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff and must be verified against engine reference docs. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md`; `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`; `docs/architecture/adr-0007-attribute-snapshot-query-api-shape-and-read-only-enforcement.md`; `docs/architecture/adr-0008-attribute-event-signal-contract-and-scene-tree-independent-core.md`; `docs/architecture/adr-0009-attribute-atomic-source-update-and-transaction-model.md` |
| **Post-Cutoff APIs Used** | None required by the Character Attributes core. If Save/Load orchestration later uses `FileAccess`, Godot 4.4+ `FileAccess.store_*` boolean return values must be checked. `Resource.duplicate_deep()` is not selected as the preferred persistence boundary; if used for nested Resource isolation in a later implementation, it requires pinned-version verification. |
| **Verification Required** | Verify `FileAccess.open()` and `store_*` failure handling in Save/Load orchestration; JSON parse/stringify primitive boundary and numeric safety if JSON is used; rejection of `store_var()` / `get_var()` object graphs as authoritative attribute payload; Resource sharing/caching and nested duplication behavior; `Array` / `Dictionary` shallow-copy behavior; `RefCounted` aliasing/cycle risks; no FileAccess/ResourceLoader/SceneTree/Autoload dependency in Character Attributes load rebuild tests. |

> **Note**: If Knowledge Risk is MEDIUM or HIGH, this ADR must be re-validated if the project upgrades engine versions. Flag it as "Superseded" and write a new ADR.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | ADR-0006 Attribute Data Representation and Stat ID Typing; ADR-0007 Attribute Snapshot Query API Shape and Read-Only Enforcement; ADR-0008 Attribute Event Signal Contract and Scene-Tree-Independent Core; ADR-0009 Attribute Atomic Source Update and Transaction Model; approved Character Attributes GDD: `design/gdd/character-attributes-system.md`. |
| **Enables** | Attribute persistent input DTO implementation; Save/Load to Character Attributes load request contract; load rebuild validation tests; current-resource load correction policy tests; migration-policy hooks for attribute semantic keys; persistence integration stories for Character Attributes AC-12. |
| **Blocks** | Any story that saves, loads, restores, migrates, or confirms Character Attributes actor identity, base stat sources, current HP/MP, equipped modifier sources, source status labels, stat registry versions, config/fixture versions, or persisted attribute debug comparison data. |
| **Ordering Note** | ADR-0006 defines runtime stat identity and semantic-key boundaries. ADR-0007 defines published snapshot truth. ADR-0008 defines result/event dispatch. ADR-0009 defines the transaction commit/swap model. This ADR binds save/load persistence to those existing contracts but does not choose the complete game save-file format, save slot UI, encryption, compression, cloud sync, or global save migration framework. |

## Context

### Problem Statement

The Character Attributes GDD requires Save/Load to persist authoritative inputs and versions, not final derived stats as truth. Loading unchanged inputs must rebuild equivalent effective values, preserve current resources where valid, and return structured failure or approved migration when config, equipment, versions, or current-resource states are invalid.

Without a persistence boundary ADR, implementation could save `StatId` enum ordinals, dense runtime vectors, mutable Godot `Resource` objects, live `AttributeSnapshot` objects, final derived stats, or event payloads as durable truth. Load code could then bypass validation, mutate runtime state directly, silently repair missing data, or present failed load results to combat/HUD as current gameplay truth. Those patterns would contradict ADR-0006 stat identity, ADR-0007 snapshot validity, ADR-0008 scene-tree-independent result semantics, and ADR-0009 atomic transaction semantics.

### Current State

No Character Attributes implementation or Save/Load implementation exists yet. The approved Character Attributes GDD lists save/load persistence boundary for base/current/modifier sources as an implementation-blocking ADR prerequisite. ADR-0009 explicitly defers load authoritative replace and `load_correction` policy to this ADR.

### Constraints

- Godot 4.6.3 is post-cutoff; persistence-adjacent API assumptions must be checked against local engine-reference docs.
- Character Attributes core must remain scene-tree-independent and unit-testable without file IO, SceneTree, Autoload, slot paths, or Godot signal processing.
- Save/Load orchestration owns file/container IO and migration orchestration; Character Attributes owns attribute semantic validation and rebuild.
- Runtime stat identity is `StatId`; durable persistence must use semantic keys or versioned migration, not enum ordinals.
- Published snapshots are read-only current truth views after rebuild; they are not durable save truth.
- Failed/rejected load rebuilds must not mutate committed source/current/snapshot state or advance versions.
- Current HP/MP values are part of Phase 1 snapshots and must be persisted/validated as current resource values, not as ordinary modifier-targetable stats.

### Requirements

- Define the minimum durable `AttributePersistentInput` boundary.
- Preserve stable semantic names for persisted stat/resource keys and source references.
- Define load rebuild through ADR-0009 transaction semantics.
- Define default failure behavior for missing config/equipment, incompatible versions, unknown keys, unsupported modifier schema, duplicate/colliding source keys, and impossible current-resource state.
- Allow migration/recovery only through explicit versioned policy with traceable correction/failure reasons.
- Keep Save/Load file IO and Character Attributes load acceptance as separate result domains.
- Forbid derived snapshots, Resource graphs, Variant object graphs, dense vectors, or enum ordinals as authoritative attribute persistence.

## Decision

Save/Load and Character Attributes use a strict ownership split.

Save/Load owns save slot/container IO, file format envelope, write success/failure, optional compression/encryption, save-slot metadata, and migration orchestration. Character Attributes owns interpretation, validation, rebuild, current-resource correction acceptance, committed runtime attribute state, source/snapshot version behavior, and `AttributeUpdateResult` outcome for attribute load requests.

Character Attributes core must not call `FileAccess`, `ResourceLoader`, slot-path APIs, `get_tree()`, Autoloads, scene signals, or any file IO during load rebuild. It receives already decoded DTO/request data from Save/Load orchestration and returns typed load/rebuild results.

The durable attribute truth is `AttributePersistentInput`, not `AttributeSnapshot`. `AttributePersistentInput` persists stable semantic strings/keys and version labels for:

- actor identity and actor type;
- player `class_id` or monster `monster_template_id` where applicable;
- level/progression payload;
- base stat table reference and/or semantic-key base stat overrides;
- current resource values by semantic resource key;
- equipped item references, source references, and slot IDs;
- persistent modifier source references where applicable;
- stat registry version;
- config/fixture version;
- attribute schema version;
- source status labels such as `mvp_provisional`, `openmir2_evidence_pending`, or `openmir2_verified`;
- optional debug snapshot hash/comparison evidence.

Durable save payloads encode semantic stat/resource keys as strings. Load normalization converts accepted strings to `StringName` and then to current registry-owned `StatId` / resource IDs through the registry and migration table. Persisted semantic keys are external boundary identifiers only. After validation/migration, runtime state must use the current registry-owned `StatId` representation defined by ADR-0006.

Attribute persistence must not store these as authoritative truth:

- `StatId` enum ordinals;
- dense runtime vectors;
- final derived/effective stats;
- mutable Godot `Resource` instances;
- `RefCounted` object identity;
- live `AttributeSnapshot` objects;
- staged candidate state;
- `AttributeUpdateResult` or domain event payloads;
- debug snapshot hashes or comparison records.

Persisted final snapshots may exist only as debug comparison/cache hints. They never bypass validation/rebuild. A debug snapshot hash or comparison record may help detect drift, but a hash match must not skip validation, migration, source normalization, current-resource validation, or candidate rebuild. A debug snapshot hash/comparison record must never be returned through ADR-0007 as the current snapshot unless the load transaction commits a newly materialized valid snapshot.

Load creates a typed load transaction/request through ADR-0009. The request kind concept is `LOAD_REPLACE_AUTHORITY` or equivalent. `LOAD_REPLACE_AUTHORITY` may bypass normal expected source/snapshot version admission checks only as an explicit policy supplied by Save/Load orchestration. It does not bypass schema validation, config validation, semantic-key migration, source validation, candidate construction, result materialization, final commit/swap, or ADR-0008 result publication.

A successful committed `LOAD_REPLACE_AUTHORITY` is a structural authority replacement. Unless a later replay/session-restore ADR supersedes this policy, it advances `source_version` exactly once and publishes a new valid `snapshot_version` exactly once, even if the rebuilt effective values equal the pre-load values. A rejected load advances neither `source_version` nor `snapshot_version`.

Any load validation failure before commit leaves committed attribute source set, current resources, current snapshot, `source_version`, and `snapshot_version` unchanged. Failed load results may expose previous valid data only as stale/display-only metadata per ADR-0007. Combat, HUD, AI, growth feedback, and save confirmation must not consume failed load rebuilds as normal gameplay truth.

Default load behavior is conservative structured failure. Missing config/equipment, incompatible schema/config/stat registry version, unknown required field, unknown stat/resource key, unknown source status, unsupported modifier schema, duplicate/colliding source key, impossible current-resource state, or source-authentic label without required evidence returns structured failure with no side effects. Optional unknown fields may be ignored only if the schema version declares a forward-compatible extension policy. Unknown required fields and unknown required semantic keys are blocking unless a versioned migration maps them.

Save/load orchestration selects and applies file/schema migration policy before submitting a typed load request. Character Attributes validates the post-migration semantic DTO, applies only explicitly authorized source/status-aware corrections, and returns `AttributeUpdateResult`. Approved migration policy may translate semantic keys, replace or drop missing sources, or clamp current resources, but it must be explicit, versioned, source-status-aware, and emit deterministic correction/failure reasons. There is no silent repair by default.

Current HP/MP values are saved as semantic resource key plus current value. On load, max resources are rebuilt first from base/config/modifier sources. Current values are then validated against the rebuilt max resource values. `load_correction` may clamp current values only if migration/recovery policy explicitly permits it. `load_correction` must emit correction reasons and must not be surfaced as damage, healing, growth, or normal gameplay mutation. If no approved correction policy exists, impossible current resource values produce structured load failure.

If Save/Load orchestration uses JSON, JSON is only a primitive container boundary. Attribute DTOs must be encoded to plain primitives: `Dictionary`, `Array`, `String`, bounded numeric values, `bool`, and `null`. `StringName`, enums, typed arrays, `RefCounted` DTOs, and `Resource` instances must be explicitly converted before serialization and explicitly reconstructed/validated after parse. Parsed JSON data is untrusted `Variant` data. Character Attributes must validate required fields, value types, semantic keys, version labels, numeric ranges, unknown fields policy, and duplicate/colliding source keys before any candidate commit. If large integer IDs or hashes are stored, their safe numeric range must be documented or encoded as strings.

Attribute persistence must not rely on `FileAccess.store_var()` / `get_var()` object graphs, `full_objects`, or engine object serialization as the authoritative durable format for attribute truth. If Save/Load orchestration uses Variant serialization for an outer container, the attribute payload inside must still be an explicit primitive DTO schema and must reject embedded Objects/Resources as authority.

Godot `.tres` Resources may be used as authoring/config inputs or save references, but save payload must not store mutable Resource instances as runtime truth. Godot `Resource` instances may be cached/shared by loaders and editor references. Attribute runtime must treat loaded Resources as read-only authoring/config inputs and normalize/copy their data into plain runtime DTOs before gameplay. Runtime mutation of a loaded `.tres` Resource is forbidden unless a later ADR explicitly creates an isolated duplicated instance policy. `duplicate()` is not sufficient for nested Resource isolation. `duplicate_deep()` is Godot 4.5+ and available in the project's 4.6.3 reference, but implementations must verify exact duplication mode/options before relying on it. The preferred persistence boundary remains normalization to plain DTOs rather than duplicated mutable Resources.

`RefCounted` DTOs are reference types and must not be serialized as object identity. Persistence encode/decode must copy scalar/container data into new DTO instances through validating factories. DTOs must avoid cyclic references and must not expose public mutable fields, mutable arrays, mutable dictionaries, Resources, or live snapshot references.

`Array` and `Dictionary` assignment is not a defensive copy. Inbound persistence containers must be copied/normalized before staging. `duplicate()` is shallow unless explicitly deep; shallow copies are acceptable only for arrays of immutable scalar values or immutable-by-contract DTO references. Nested arrays/dictionaries require explicit deep normalization. Objects/Resources inside arrays/dictionaries must not become authority through container copy. Typed arrays/dictionaries improve static checking but do not make contents immutable and do not replace runtime validation after JSON/Variant decode.

Save/Load orchestration must check `FileAccess.open()` failure and Godot 4.4+ `FileAccess.store_*` boolean return values for every authoritative write if it uses `FileAccess`. Failed writes must not be reported as successful saves. Attribute load acceptance is separate from save-file read/write success: Character Attributes may accept or reject a load transaction, while Save/Load orchestration separately owns whether the slot container was actually read or written successfully.

Save/Load orchestration may adapt a committed `AttributeUpdateResult` into persistence-level success/failure UI, but it must not reinterpret signal callback order, sink delivery, or adapter emission as load success. Returned result state is the load acceptance authority.

### Architecture Diagram

```text
Save slot / file container / migration envelope
        |
        | Save/Load orchestration owns IO, envelope decode, migration selection,
        | write success, optional compression/encryption
        v
Decoded primitive payload / post-migration DTO data
        |
        | semantic strings + versions + source references
        v
AttributeLoadRequest / AttributePersistentInput
        |
        | Character Attributes owns validation and load rebuild
        v
Character Attributes transaction core
        |
        +--> validate schema/config/stat registry versions
        +--> migrate semantic strings -> StringName -> StatId/resource IDs
        +--> rebuild max resources before current resource validation
        +--> build candidate source/current/snapshot/result
        +--> final commit/swap only if valid
        v
AttributeUpdateResult
        |
        +--> success: new committed source_version + snapshot_version
        +--> failure: committed state unchanged; stale/display-only previous truth only
        v
ADR-0007 snapshot/query APIs for current truth
```

### Key Interfaces

```gdscript
class_name AttributePersistentInput
extends RefCounted

func get_actor_id() -> int:
    pass

func get_actor_type_key() -> StringName:
    pass

func get_identity_fields_copy() -> Dictionary:
    pass

func get_level_payload_copy() -> Dictionary:
    pass

func get_base_stat_table_key() -> StringName:
    pass

func get_base_stat_overrides_copy() -> Array[AttributePersistentStatValue]:
    pass

func get_current_resources_copy() -> Array[AttributePersistentResourceValue]:
    pass

func get_source_references_copy() -> Array[AttributePersistentSourceReference]:
    pass

func get_stat_registry_version() -> StringName:
    pass

func get_config_version() -> StringName:
    pass

func get_schema_version() -> StringName:
    pass

func get_source_status() -> int:
    pass
```

```gdscript
class_name AttributePersistentStatValue
extends RefCounted

func get_stat_key() -> StringName:
    pass

func get_value() -> int:
    pass

func get_source_status() -> int:
    pass
```

```gdscript
class_name AttributePersistentResourceValue
extends RefCounted

func get_resource_key() -> StringName:
    pass

func get_current_value() -> int:
    pass

func get_source_status() -> int:
    pass
```

```gdscript
class_name AttributePersistentSourceReference
extends RefCounted

func get_source_system() -> StringName:
    pass

func get_source_instance_id() -> StringName:
    pass

func get_slot_id() -> StringName:
    pass

func get_source_status() -> int:
    pass
```

```gdscript
class_name AttributeLoadRequest
extends RefCounted

func get_actor_id() -> int:
    pass

func get_request_id() -> StringName:
    pass

func get_load_policy() -> AttributeLoadPolicy:
    pass

func get_persistent_input() -> AttributePersistentInput:
    pass

func get_migration_context() -> AttributeMigrationContext:
    pass
```

```gdscript
class_name AttributeLoadPolicy
extends RefCounted

func get_request_kind() -> int:
    pass

func permits_authority_replace() -> bool:
    pass

func permits_current_resource_load_correction() -> bool:
    pass

func get_unknown_field_policy() -> int:
    pass

func get_missing_source_policy() -> int:
    pass
```

```gdscript
class_name AttributeLoadRebuildResult
extends RefCounted

func get_update_result() -> AttributeUpdateResult:
    pass

func get_persistence_failure_reasons_copy() -> Array[int]:
    pass

func get_migration_corrections_copy() -> Array[AttributeMigrationCorrection]:
    pass
```

The interfaces above are conceptual. Exact enum ownership, file paths, constructor shapes, and whether `AttributeLoadRebuildResult` is a distinct wrapper or an `AttributeUpdateResult` specialization may be refined during implementation as long as the boundary decisions remain intact. Each `class_name` should live in its own `.gd` file.

### Implementation Guidelines

- Keep Character Attributes load rebuild injectable and unit-testable without SceneTree, Autoload, file IO, or signals.
- Treat all decoded persistence data as untrusted until validated.
- Convert semantic strings to current registry IDs only through validated registry/migration helpers.
- Never calculate formulas directly from persisted string keys.
- Validate schema/config/stat registry versions before expensive candidate construction unless migration policy explicitly maps them.
- Rebuild effective max resources before validating persisted current HP/MP.
- Keep load correction reasons distinct from damage/heal/growth reasons.
- Do not expose load candidate state through snapshot/query APIs, events, debug tools, or save confirmation.
- Save debug snapshot comparison data only as drift evidence, never as authoritative restore data.
- Tests must prove rejected load has no committed side effects.
- Tests must prove file write failure and attribute load acceptance are separate result domains.

## Alternatives Considered

### Alternative 1: Persist full derived/effective `AttributeSnapshot` as save truth

- **Description**: Save the published snapshot, including derived stats/current resources, and restore it directly on load.
- **Pros**: Fast apparent load; simple comparison against last session; less rebuild work.
- **Cons**: Bypasses config/equipment/source validation; stale derived data can survive changed formulas or item definitions; conflicts with ADR-0007 snapshot status and GDD save/load rules; hard to migrate safely.
- **Rejection Reason**: GDD AC-12 requires authoritative inputs and versions, not derived snapshot truth. Snapshots may be debug comparison hints only.

### Alternative 2: Save/Load directly mutates Character Attributes runtime state

- **Description**: Save/Load code writes base stats, current resources, modifiers, source sets, and versions directly into runtime objects during load.
- **Pros**: Fewer request/result DTOs; implementation initially looks straightforward.
- **Cons**: Violates ADR-0006 state ownership and ADR-0009 transaction model; bypasses candidate validation and no-side-effect failure; risks partial load state.
- **Rejection Reason**: Load rebuild must enter Character Attributes through a typed transaction/request and final commit/swap boundary.

### Alternative 3: Persist `StatId` enum ordinals and dense vectors

- **Description**: Save compact arrays keyed by current enum ordinal and restore directly into dense runtime storage.
- **Pros**: Small files; fast decode; convenient for dense vectors.
- **Cons**: Enum reorder/delete/reuse corrupts persisted identity; unreadable; hard migration; directly conflicts with ADR-0006.
- **Rejection Reason**: Runtime IDs are implementation details. Durable data uses semantic strings plus versioned migration.

### Alternative 4: Automatic safe repair by default

- **Description**: If load references missing equipment/config or invalid values, silently drop missing sources, clamp current resources, and continue.
- **Pros**: Fewer player-facing load failures during content iteration; easier to keep a save playable.
- **Cons**: Hides data corruption; can erase player progress; makes QA evidence ambiguous; can misrepresent invalid load as normal gameplay.
- **Rejection Reason**: Default behavior is structured failure. Repair requires explicit versioned migration/recovery policy with correction reasons.

### Alternative 5: Persist Godot `Resource` or Variant object graphs as attribute truth

- **Description**: Use `.tres` Resources, `store_var()` / `get_var()`, `full_objects`, or serialized `RefCounted` object graphs for attribute persistence.
- **Pros**: Godot-native; quick to prototype; less manual encode/decode code.
- **Cons**: Engine object identity and layout are poor durable contracts; Resources may be shared/cached/mutable; `RefCounted` aliasing/cycles create correctness and leak risks; migration and security/trust boundaries become unclear.
- **Rejection Reason**: Authoritative attribute payload must be explicit primitive semantic DTO data plus validation. Object graphs may not be save truth.

### Alternative 6: Never persist current HP/MP; always refill on load

- **Description**: Save only max/stat sources and restore current resources to max or another configured default on load.
- **Pros**: Simpler persistence; avoids invalid current resource cases.
- **Cons**: Loses actor-state continuity; contradicts GDD minimum `AttributePersistentInput` contract for current resource values; may create player-visible state jumps.
- **Rejection Reason**: Phase 1 saves current HP/MP values and validates them after max-resource rebuild. Refill policies require separate game-mode design.

## Consequences

### Positive

- Save data remains migration-friendly and human-auditable through semantic keys and version labels.
- Runtime authority remains inside Character Attributes.
- Load rebuilds reuse ADR-0009 atomic candidate commit/swap semantics.
- Failed loads are structured and side-effect-free.
- Derived snapshots cannot bypass validation after formula/config/equipment changes.
- Current-resource load correction is explicit and cannot be confused with damage/heal/growth.
- Godot Resource and object-serialization pitfalls are kept outside the authoritative attribute payload.

### Negative

- Save files become more verbose and less Godot-native than dense ordinal arrays or Resource/Variant object graphs.
- Load failures may be more common during content iteration because missing config/equipment and unknown keys fail by default unless explicit migration exists.
- Save/Load orchestration must implement encode/decode, migration, and file IO result handling separately from Character Attributes acceptance.
- Character Attributes needs additional DTOs, validators, migration hooks, and tests.
- Current resource load behavior requires careful policy design for each migration/recovery case.
- JSON or primitive container formats require explicit type/range validation after parse.

### Neutral

- This ADR does not choose JSON, binary, `.tres`, or another full save-file container format.
- This ADR does not choose global save slot UI, backup strategy, compression, encryption, or cloud sync.
- Persisted debug snapshot hash/comparison data remains allowed for QA drift checks but not authority.
- A later Save/Load system ADR may define whole-game save slot atomic write patterns.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| Implementers treat persisted snapshot/debug hash as current truth. | Medium | High | Explicitly forbid snapshot/hash bypass; tests prove load rebuild happens even with matching debug comparison data. |
| Enum ordinal or dense vector sneaks into save payload. | Medium | High | Persistence schema validation rejects ordinal/vector authority; registry maps semantic strings to `StatId` only after load. |
| Missing content during iteration causes many failed loads. | Medium | Medium | Accept as safer default; add explicit migration policy when needed. |
| Save/Load silently repairs invalid resources or missing sources. | Medium | High | Default structured failure; repair only with explicit versioned migration/recovery policy and correction reasons. |
| `FileAccess` write failure is reported as successful save. | Medium | High | Save/Load orchestration checks `open()` and Godot 4.4+ `store_*` boolean return values. |
| JSON parse coerces typed DTOs into loose `Variant` data. | Medium | Medium | Treat parsed data as untrusted; validate fields/types/ranges/versions before DTO reconstruction. |
| `store_var()` / `get_var()` object graphs become accidental truth. | Medium | High | Ban object graphs as authoritative attribute payload; require primitive DTO schema. |
| Resource sharing/caching mutates runtime or persisted truth. | Medium | High | Resources are read-only authoring/config inputs; normalize/copy into DTOs before gameplay. |
| `Array` / `Dictionary` shallow copies preserve mutable nested aliases. | Medium | High | Deep normalize nested containers; scalar shallow copies only where safe; tests mutate caller containers after submission. |
| `RefCounted` DTO cycles or aliasing create leaks or mutation bypass. | Medium | Medium | Encode/decode into fresh validating DTOs; avoid cycles; expose no public mutable fields/collections. |
| Successful load version behavior is misunderstood. | Low | Medium | Define `LOAD_REPLACE_AUTHORITY` as structural authority replacement that advances `source_version` and `snapshot_version` once on commit. |

## Performance Implications

| Metric | Before | Expected After | Budget |
|--------|--------|---------------|--------|
| CPU (frame time) | No implementation. | Load rebuild validates and reconstructs candidate state at load/spawn/restore points, not per frame. Normal gameplay remains unaffected. | Overall project frame budget 16.6 ms at 60 fps. Structural load rebuilds must not run per frame. |
| Memory | No implementation. | Persistence DTOs and migration/candidate data allocate during load/save operations. No unbounded snapshot/event history is persisted as authority. | Phase 1 client under 1 GB RAM. |
| Load Time | No implementation. | Semantic decode, migration, validation, and rebuild add load-time work but keep runtime state safe. | Acceptable for Phase 1; profile once save/load system exists. |
| Network | Not applicable for Phase 1 offline slice. | Not applicable. Semantic/versioned payload may help future migration but selects no network behavior. | None. |

## Migration Plan

No existing Character Attributes or Save/Load implementation needs migration.

1. Define `AttributePersistentInput`, persistent stat/resource/source reference DTOs, load policy DTOs, and migration context DTOs.
2. Define primitive encode/decode schema for attribute persistent input once the Save/Load system selects its container format.
3. Implement semantic string to `StringName` to `StatId` / resource ID normalization and migration helpers.
4. Implement load request path through ADR-0009 `LOAD_REPLACE_AUTHORITY` transaction semantics.
5. Implement current-resource load validation after max-resource rebuild.
6. Implement explicit migration/recovery policies for any allowed source drop/replace/key translation/current-resource clamp.
7. Implement debug snapshot comparison/hash as non-authoritative drift evidence if needed.
8. Implement Save/Load orchestration tests for file write/read result handling separately from Character Attributes load acceptance.
9. Implement GUT tests for valid round trip, rejected load no-side-effect, migration/correction reasons, ordinal/vector/object-graph rejection, JSON primitive validation if JSON is selected, and Resource/RefCounted/container aliasing boundaries.

**Rollback plan**: If `AttributePersistentInput` proves too verbose, a later ADR may compact the wire/file encoding while preserving the semantic-key durable boundary, load transaction semantics, and ban on derived snapshot/dense-vector/object-graph authority.

## Validation Criteria

- [ ] Valid unchanged `AttributePersistentInput` rebuilds equivalent actor identity, effective values, current resources, source labels, schema/config versions, and validity state; exact snapshot object identity and version need not match the previous session.
- [ ] A successful committed `LOAD_REPLACE_AUTHORITY` advances `source_version` exactly once and publishes a new valid `snapshot_version` exactly once.
- [ ] Rejected load leaves committed source set, current resources, current snapshot, `source_version`, and `snapshot_version` unchanged.
- [ ] Persisted semantic stat/resource keys migrate through the registry before runtime use.
- [ ] Persisted `StatId` ordinals, dense vectors, live snapshots, staged candidate state, event payloads, Resource instances, RefCounted object identities, and Variant object graphs are rejected as authoritative input.
- [ ] Missing config/equipment, incompatible schema/config/stat registry version, unknown required fields, unknown required semantic keys, unsupported modifier schema, duplicate/colliding source keys, and impossible current-resource states produce structured failure unless an explicit migration policy handles them.
- [ ] Optional unknown fields are ignored only when the schema version declares forward-compatible extension policy.
- [ ] Current HP/MP load validates after max-resource rebuild.
- [ ] `load_correction` clamp requires explicit versioned migration/recovery policy, emits correction reasons, and is not surfaced as damage, healing, growth, or normal gameplay mutation.
- [ ] Debug snapshot hashes/comparison records cannot skip validation/rebuild and are never returned as current truth unless a rebuilt snapshot commits.
- [ ] Character Attributes load rebuild tests instantiate the core without FileAccess, ResourceLoader, SceneTree, Autoload, signals, or slot-path APIs.
- [ ] Save-file read/write success is tested separately from Character Attributes load acceptance.
- [ ] If `FileAccess` is used by Save/Load orchestration, tests or implementation evidence show `open()` failure and Godot 4.4+ `store_*` boolean return values are checked.
- [ ] If JSON is selected, parsed data is treated as untrusted primitives and validated for required fields, types, numeric ranges, semantic keys, versions, unknown fields, and duplicate/colliding source keys.
- [ ] Resource-backed config references cannot mutate authoritative runtime state after load normalization.
- [ ] Mutating caller-owned arrays/dictionaries/DTOs after load request submission cannot alter staged or committed load behavior.

## GDD Requirements Addressed

| GDD Document | System | Requirement | How This ADR Satisfies It |
|-------------|--------|-------------|--------------------------|
| `design/gdd/character-attributes-system.md` | Character Attributes | Save/load must persist authoritative inputs, not final derived stats as truth. | Defines `AttributePersistentInput` as durable truth and forbids derived/effective snapshots as authoritative save data. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Minimum `AttributePersistentInput` includes actor identity/type, class/template, level/progression, base stat payload/table reference, current resource values, equipped item/source refs, modifier source refs, registry/config versions, and source status labels. | Enumerates the durable semantic fields and version labels that must be persisted. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Load rebuild must return a rebuild result; missing config/equipment, incompatible versions, impossible current resource state, or unsupported modifier schema produce structured failure or approved migration. | Routes load through ADR-0009 typed transaction and defines default structured failure plus explicit migration/recovery policy. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Persisted final snapshots may be used only for debug comparison. | Allows debug snapshot hash/comparison evidence only as non-authoritative drift data and forbids hash/snapshot bypass of rebuild. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Current resources are persistent for player and actor-state for monsters; load correction must be policy-approved. | Saves current HP/MP by semantic resource key and validates them after rebuilt max resources; `load_correction` requires explicit policy. |
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-12 — Save/Load Rebuild Contract: unchanged inputs rebuild equivalent effective/current values; failure is structured; failed rebuild is not consumed by combat/HUD as normal data. | Defines load equivalence criteria, failure no-side-effect behavior, stale/display-only fallback, and validation criteria for AC-12. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Attribute Layers separate identity, base stats, current resources, derived stats, modifiers, snapshots, and debug trace. | Persists only identity/base/current/source inputs and version labels; derived stats/snapshots/debug hashes remain non-authoritative. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Implementation-blocking ADR item 5: save/load persistence boundary for base/current/modifier sources. | Directly resolves the persistence boundary prerequisite while leaving whole-game save-file format to the Save/Load system. |

## Related Decisions

- `design/gdd/character-attributes-system.md`
- `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`
- `docs/architecture/adr-0007-attribute-snapshot-query-api-shape-and-read-only-enforcement.md`
- `docs/architecture/adr-0008-attribute-event-signal-contract-and-scene-tree-independent-core.md`
- `docs/architecture/adr-0009-attribute-atomic-source-update-and-transaction-model.md`
- `docs/registry/architecture.yaml` — should be updated with persistent input, load rebuild, semantic key boundary, load authority replace, migration policy, save write result boundary, and forbidden persistence anti-patterns after this ADR.
