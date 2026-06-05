# ADR-0016: OpenMir2 Evidence Mapping Registry and Provisional Contract Governance

## Status

Accepted

## Date

2026-06-05

## Last Verified

2026-06-05

## Decision Makers

hkm + Claude Code Game Studios; engine specialist review: godot-specialist; strategic review: technical-director / TD-ADR.

## Summary

This ADR establishes the source-first governance artifact for OpenMir2 behavior evidence, evidence maturity, provisional contracts, and intentional divergence records used by Phase 1 systems. We choose an offline, deterministic, versioned `OpenMir2EvidenceRegistry` data/model boundary that is queried by design, tooling, bootstrap validation, QA, and story readiness only; it is not a Godot runtime gameplay authority, network implementation, Autoload, or scene-tree service.

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Core / Tools / Networking-reference |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-LLM-cutoff |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md`; `docs/engine-reference/godot/modules/networking.md`; `.claude/docs/technical-preferences.md` |
| **Post-Cutoff APIs Used** | Optional Godot-native tooling may use `FileAccess`; if it writes files, it must check Godot 4.4+ `store_*` boolean returns. No Godot networking API is introduced. |
| **Verification Required** | Verify deterministic parsing/normalization, explicit `FileAccess.open()` failure handling, explicit `store_*` write-failure handling, no SceneTree/Autoload/runtime gameplay dependency, no YAML parser dependency unless approved, and CI-visible structured readiness failures. |

> **Note**: Godot 4.6.3 is post-cutoff. Any Godot-native tooling that reads or writes this registry must be rechecked against the pinned engine reference if the project upgrades engine versions.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | None — this ADR establishes the root evidence-governance contract for OpenMir2-derived behavior claims. |
| **Enables** | Future ADRs for Drop Table / Ground Drop / Pickup Lifecycle; Inventory / Equipment Instance and Modifier Transaction Boundary; Map Distance and Movement Legality Contract; Resource / Map Conversion Pipeline; future protocol spikes. |
| **Blocks** | Source-authentic / `openmir2_verified` movement, combat, drop, pickup, inventory, equipment, item, attribute, map, and protocol claims until this ADR is Accepted and the referenced evidence/contract passes readiness. It does **not** block explicitly labeled `mvp_provisional` work that records evidence gaps and avoids source-authentic claims. |
| **Ordering Note** | Downstream systems may proceed with provisional Phase 1 implementation only when GDD/ADR/story text clearly labels assumptions as `mvp_provisional` or `openmir2_evidence_pending`; any `Adopt`, `Simplify`, `source-authentic`, or `openmir2_verified` claim must cite a readiness-passing Accepted contract or Accepted evidence reference from this registry. |

## Context

### Problem Statement

The project vision depends on “传奇骨架，现代皮肤”: the Godot client may modernize presentation and engineering, but core movement, combat, death, drop, pickup, inventory, equipment, and protocol-entry behavior must not be invented from memory or copied from non-authoritative clients. The OpenMir2 Behavior Mapping Spike GDD already defines evidence tiers, E0–E4 maturity, Adopt/Simplify/Exclude/Defer decisions, provisional contracts, and intentional divergence rules, but `/architecture-review` found no ADR establishing the architecture/governance artifact that makes those rules stable, queryable, and auditable.

Without this decision, every downstream GDD/ADR could invent local evidence notes, cite MinimalMirClient as if it were authoritative, treat raw E2 findings as Phase 1 rules, or call a value `openmir2_verified` without a stable evidence ID and readiness gate.

### Current State

- `design/gdd/openmir2-behavior-mapping-spike.md` defines the design rules and table fields for source evidence, evidence levels, provisional contracts, and divergence records.
- `docs/architecture/architecture-review-2026-06-05.md` reports OpenMir2 evidence governance as the highest-priority architecture gap.
- ADR-0001 through ADR-0015 already use source/evidence wording such as `openmir2_verified`, `source_status`, evidence labels, and provisional values, but they do not define the central evidence registry those labels must resolve against.
- `docs/architecture/tr-registry.yaml` now includes TR IDs that require a stable OpenMir2 evidence governance ADR.

### Constraints

- Phase 1 is an offline 2D/2.5D Godot 4.6.3 PC-native GDScript slice; this ADR must not introduce runtime networking.
- OpenMir2 source is Tier 1 behavior evidence; MirServer configuration is data context, mir2x is interpretation aid, and MinimalMirClient/client observation is verification-only.
- Evidence governance must be deterministic, versionable, and auditable across GDDs, ADRs, stories, tests, and future tools.
- Godot-native tooling must not rely on unapproved YAML parsers/addons.
- Registry validation must return structured results; GDScript `assert()` must not be the governance gate.
- Runtime gameplay hot paths must not read files, parse evidence documents, or query this registry as authority.

### Requirements

- Provide stable IDs for OpenMir2 evidence records, provisional contracts, and intentional divergence records.
- Preserve the OpenMir2 Spike GDD source hierarchy and E0–E4 evidence maturity rules.
- Distinguish raw evidence from Accepted contracts.
- Fail closed on missing evidence, low maturity, unaccepted contracts, conflicting evidence, unsupported source tier, or unversioned references.
- Allow explicitly labeled `mvp_provisional` implementation while blocking false `openmir2_verified` claims.
- Support design/tooling/QA/CI validation without SceneTree, Autoload, runtime gameplay, or Godot networking dependency.
- Integrate with downstream ADRs and runtime catalogs by stable copied evidence/contract references, not hot registry lookups.

## Decision

We will establish an offline source-first `OpenMir2EvidenceRegistry` governance model for OpenMir2 behavior evidence, provisional contracts, intentional divergences, and readiness gates.

`OpenMir2EvidenceRegistry` is a governance dataset and optional tooling/query model, not a runtime gameplay service. It owns:

- Evidence record schema and stable `evidence_id` values.
- Source authority tiers: Tier 1 OpenMir2 source, Tier 2 MirServer/config/data, Tier 3 mir2x/reference implementation, Tier 4 MinimalMirClient/client observation.
- Evidence maturity levels E0–E4.
- Mapping item lifecycle and decision lifecycle.
- Adopt / Simplify / Exclude / Defer decisions.
- Provisional contract records and stable contract IDs/names/versions.
- Intentional divergence records.
- Structured readiness validation and risk gates.
- Source-format and tooling boundaries for registry authoring and validation.

It does **not** own:

- Runtime gameplay state.
- Godot networking, socket, packet, RPC, multiplayer, or protocol implementation.
- Movement, combat, drop, pickup, inventory, equipment, item, attribute, map, UI, save, or economy runtime truth.
- SceneTree nodes, Autoload singletons, Resources as runtime authority, or hot-path file IO.

### Canonical Source, Normalized Model, and Query Layer

The registry has three explicit layers:

1. **Canonical source of record**
   - Human-authored offline evidence documents are the source of record.
   - Phase 1 may use strict Markdown tables, CSV, JSON, ConfigFile, or typed GDScript fixture payloads as authoring envelopes.
   - YAML is permitted only for external non-Godot tooling or documentation unless a YAML parser/addon dependency is explicitly approved by a later technical decision.
   - Multiple source files may exist, but one evidence ID or contract ID must not be silently overwritten by another source. Duplicate or conflicting IDs fail validation.

2. **Normalized validation model**
   - Tooling normalizes source rows into typed immutable-by-contract DTO/table rows: `OpenMir2EvidenceRecord`, `OpenMir2ProvisionalContract`, and `OpenMir2DivergenceRecord`.
   - The normalized model is deterministic: sorted input order, stable ID validation, explicit duplicate detection, explicit conflict detection, and fail-closed readiness status.
   - Internal mutable arrays/dictionaries are not exposed directly; query APIs return scalar values, copied collections, immutable-by-contract row DTOs, or status-bearing result wrappers.

3. **Query/readiness interface**
   - Design tools, review skills, story readiness, bootstrap validation, QA/debug tools, and generated documentation may query the normalized model.
   - Hot gameplay systems must not query the registry to decide movement, combat, drops, pickup, inventory, equipment, save/load, UI, or runtime item/attribute behavior.
   - Runtime catalogs may carry copied accepted evidence IDs or contract refs as metadata validated at bootstrap/import time.

### Stable Identity and Lifecycle

Evidence and contracts use stable identities:

- `evidence_id`: stable permanent ID, e.g. `OM2-EVID-MOVEMENT-001`; never renumbered or reused.
- `contract_id`: stable permanent ID, e.g. `OM2-CONTRACT-MAP-BLOCKING-001`; never renumbered or reused.
- `contract_name`: human-readable stable semantic name, e.g. `Map Blocking Contract`; renames require a migration/supersession note.
- `contract_version`: append-only semantic version or integer revision for downstream references.
- `divergence_id`: stable permanent ID, e.g. `OM2-DIV-MOVEMENT-001`.

Evidence record lifecycle:

```text
Candidate → Located → Interpreted → Cross-Checked → Decided → Contracted
```

Contract status lifecycle:

```text
Proposed → Accepted → Superseded / Deprecated
```

Rules:

- Raw E3/E4 evidence does not automatically authorize `openmir2_verified` gameplay/data claims.
- A source-authentic downstream claim must reference either an Accepted evidence record explicitly approved for that claim or an Accepted contract that passes readiness validation.
- Superseded or Deprecated contracts remain in the registry for traceability; downstream references to them must be migrated or explicitly tolerated by a later decision.
- Conflicting evidence blocks readiness even if one individual source record appears E3 or E4.

### Accepted Contract Readiness

`validate_contract_ready(contract_id_or_name, required_min_level)` returns a structured readiness result. It does not rely on GDScript `assert()` semantics.

A contract is ready only if all are true:

- Contract status is `Accepted`.
- Contract decision is `Adopt` or `Simplify`.
- Every required Phase 1 behavior covered by the contract meets the required minimum evidence level, default E3.
- The contract references Tier 1 OpenMir2 source for behavior authority.
- Tier 2/3/4 sources, if present, are labeled as data context / interpretation aid / verification-only.
- No unresolved `Conflicting` evidence state exists.
- Required contract fields are complete.
- Contract ID and version are stable.
- Any Simplify decision states preserved semantics, removed complexity, forbidden changes, and validation needs.

### MVP Provisional Work Boundary

This ADR blocks false authenticity, not all implementation.

Downstream systems may implement explicit `mvp_provisional` behavior before OpenMir2 readiness if they:

- Label the behavior as provisional or evidence-pending.
- Do not call it `openmir2_verified`, `source-authentic`, or `Adopted from OpenMir2`.
- Do not invent concrete values as OpenMir2 truth.
- Record evidence gaps and future contract dependencies.
- Include tests that validate the provisional contract as provisional, not authentic.

### Architecture

```text
OpenMir2 source / config / reference observations
        │
        ▼
Canonical evidence source documents
(Markdown / CSV / JSON / ConfigFile / typed fixtures; YAML external-only unless approved)
        │
        ▼
OpenMir2EvidenceRegistry normalizer + validator
        │
        ├── OpenMir2EvidenceRecord table
        ├── OpenMir2ProvisionalContract table
        ├── OpenMir2DivergenceRecord table
        └── Structured readiness/risk results
        │
        ▼
Design/review/story/QA/bootstrap tools
        │
        ├── GDD/ADR traceability
        ├── story readiness gates
        ├── runtime catalog bootstrap validation of copied evidence refs
        └── reports / risk gates

Runtime gameplay hot path ──X── no file IO, no registry query, no Autoload authority
```

### Key Interfaces

The following GDScript-style interfaces are conceptual contracts. Implementations must use static typing, doc comments for public APIs, and status-bearing result wrappers rather than nullable or raw dictionary-only results.

```gdscript
class_name OpenMir2EvidenceLevel

enum Value {
    E0_UNVERIFIED_NOTE,
    E1_SYMBOL_LOCATED,
    E2_RESPONSIBILITY_OBSERVED,
    E3_FLOW_TRACED,
    E4_CROSS_CHECKED,
}
```

```gdscript
class_name OpenMir2SourceTier

enum Value {
    TIER_1_OPENMIR2_SOURCE,
    TIER_2_MIRSERVER_CONFIG,
    TIER_3_REFERENCE_IMPLEMENTATION,
    TIER_4_CLIENT_OBSERVATION,
}
```

```gdscript
class_name OpenMir2MappingDecision

enum Value {
    ADOPT,
    SIMPLIFY,
    EXCLUDE,
    DEFER,
}
```

```gdscript
class_name OpenMir2EvidenceRecord
extends RefCounted

## Stable permanent evidence ID, never renumbered or reused.
func get_evidence_id() -> StringName

## Behavior domain such as Movement, Combat, Death, Drop, Pickup, Inventory, Equipment, or Protocol.
func get_behavior_domain() -> StringName

## Highest behavior-authority source tier for this evidence record.
func get_source_tier() -> int

## Evidence maturity E0-E4.
func get_evidence_level() -> int

## Stable source file/function/symbol references. Returns a defensive copy.
func get_source_refs() -> Array[String]

## True only when this record has no unresolved conflict marker.
func is_conflict_free() -> bool
```

```gdscript
class_name OpenMir2ProvisionalContract
extends RefCounted

func get_contract_id() -> StringName
func get_contract_name() -> StringName
func get_contract_version() -> int
func get_status() -> StringName
func get_decision() -> int
func get_required_evidence_ids() -> Array[StringName]
func get_preserved_semantics() -> Array[String]
func get_allowed_simplifications() -> Array[String]
func get_forbidden_changes() -> Array[String]
func get_dependent_gdds() -> Array[String]
func get_verification_needs() -> Array[String]
```

```gdscript
class_name OpenMir2ContractQueryResult
extends RefCounted

enum Status {
    FOUND,
    MISSING_CONTRACT,
    DUPLICATE_CONTRACT_ID,
    INVALID_CONTRACT_VERSION,
    SUPERSEDED,
    DEPRECATED,
    INVALID_REGISTRY,
}

func get_status() -> int
func is_found() -> bool
func get_contract() -> OpenMir2ProvisionalContract
func get_failure_reasons() -> Array[StringName]
```

```gdscript
class_name OpenMir2ContractReadinessResult
extends RefCounted

enum Status {
    READY,
    MISSING_CONTRACT,
    NOT_ACCEPTED,
    BELOW_REQUIRED_EVIDENCE_LEVEL,
    MISSING_TIER_1_SOURCE,
    CONFLICTING_EVIDENCE,
    INCOMPLETE_CONTRACT_FIELDS,
    UNVERSIONED_REFERENCE,
    UNSUPPORTED_SOURCE_TIER,
    INVALID_REGISTRY,
}

func get_status() -> int
func is_ready() -> bool
func get_contract_id() -> StringName
func get_contract_version() -> int
func get_failure_reasons() -> Array[StringName]
func get_blocked_downstream_systems() -> Array[StringName]
```

```gdscript
class_name OpenMir2EvidenceRegistry
extends RefCounted

## Returns a status-bearing result; never returns null for missing contracts.
func query_contract(contract_id_or_name: StringName) -> OpenMir2ContractQueryResult

## Validates source-authentic readiness. This is not GDScript assert().
func validate_contract_ready(
        contract_id_or_name: StringName,
        required_min_level: int = OpenMir2EvidenceLevel.Value.E3_FLOW_TRACED
) -> OpenMir2ContractReadinessResult
```

### Implementation Guidelines

- Prefer offline validation tools, generated reports, test fixtures, or bootstrap validation over runtime services.
- Do not register `OpenMir2EvidenceRegistry` as an Autoload.
- Do not call `get_tree()`, depend on `_ready()`, or use scene signals inside evidence validation logic.
- If Godot-native tooling writes registry/report files, it must check `FileAccess.open()` failure and `FileAccess.get_open_error()`, and must check every post-4.4 `store_*` boolean return.
- Tool writes must be atomic or explicitly safe against partial files; silent partial output is invalid.
- If an external tool uses YAML, the Godot project must not become dependent on YAML parsing unless a dependency ADR approves it.
- CI/tool failures must be structured and visible; `assert()` may be used as an additional developer check only after readiness result generation, not as the authority.
- Runtime systems may embed copied accepted contract IDs/versions in config metadata, but gameplay decisions come from their own accepted ADR/runtime catalog state.

## Alternatives Considered

### Alternative 1: GDD-only narrative evidence notes

- **Description**: Keep evidence tiers and source notes only inside the OpenMir2 Spike GDD prose and downstream GDD text.
- **Pros**: Lowest immediate documentation overhead; easy to write during design.
- **Cons**: Not machine-queryable, hard to gate in stories/CI, easy to fork names, and weak against false `openmir2_verified` claims.
- **Estimated Effort**: Lower than chosen approach initially; higher over time due to manual review cost.
- **Rejection Reason**: `/architecture-review` identified this as an architecture gap; source authenticity needs stable IDs and readiness gates.

### Alternative 2: Per-system local evidence notes

- **Description**: Each downstream system keeps its own OpenMir2 source notes and decides its own readiness.
- **Pros**: Lets systems move independently; notes can be tailored to each domain.
- **Cons**: Creates contradictory authority, duplicated evidence IDs, inconsistent evidence thresholds, and cross-system drift for shared behavior such as death → drop → pickup.
- **Estimated Effort**: Similar initial effort; much higher conflict-resolution cost.
- **Rejection Reason**: Violates the source-first governance need and makes cross-system behavior contracts unreliable.

### Alternative 3: Runtime Godot service / Autoload evidence registry

- **Description**: Implement `OpenMir2EvidenceRegistry` as a Godot Autoload or scene service queried by gameplay systems.
- **Pros**: Easy global access; runtime debug overlays could query it directly.
- **Cons**: Turns design evidence into runtime authority, adds SceneTree/global coupling, risks file IO hot-path behavior, and contradicts Phase 1 offline slice boundaries.
- **Estimated Effort**: Higher than chosen approach.
- **Rejection Reason**: Existing ADRs consistently forbid hidden global runtime authority, scene-tree-dependent cores, and hot-path config/evidence reloads.

### Alternative 4: External YAML-only toolchain

- **Description**: Store all evidence in YAML and require a YAML parser/toolchain for validation.
- **Pros**: Human-readable and flexible for nested records.
- **Cons**: Godot has no approved built-in YAML parser; dependency approval would be required; parser behavior could diverge between external and Godot-native tools.
- **Estimated Effort**: Moderate, but requires dependency governance.
- **Rejection Reason**: The project currently has no approved YAML dependency. YAML may be external-only until explicitly approved.

## Consequences

### Positive

- Downstream systems get one stable authority for OpenMir2 evidence readiness.
- `openmir2_verified` claims become auditable and gateable.
- Provisional Phase 1 work remains possible without pretending to be source-authentic.
- Evidence conflicts fail closed before infecting GDDs, ADRs, stories, or tests.
- MinimalMirClient/client observation is safely demoted to verification-only.
- The registry can feed architecture review, story readiness, test planning, and QA/debug reports.

### Negative

- Adds documentation and validation overhead before source-authentic claims can be made.
- Requires stable ID/version discipline similar to TR IDs and item/attribute catalogs.
- Some downstream GDDs may need to stay `mvp_provisional` longer while evidence reaches E3/E4.
- Requires future tooling discipline if registry validation is automated.

### Neutral

- This ADR does not choose movement distance metrics, attack ranges, drop rates, inventory capacity, equipment slots, or protocol implementation.
- This ADR does not require Godot-native tools immediately; manual document review remains valid if it follows the schema and readiness rules.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| Evidence registry becomes a gameplay service | Medium | High | Explicitly forbid Autoload, SceneTree, hot-path registry queries, and runtime authority. |
| Raw E3 record is treated as final OpenMir2 truth | Medium | High | Require Accepted evidence/contract readiness, not raw evidence alone, for source-authentic claims. |
| Multiple source formats cause conflicting truth | Medium | Medium | Define source documents as authoring envelopes, normalize into one schema, and fail duplicate/conflicting IDs. |
| YAML parser dependency sneaks into Godot tooling | Medium | Medium | YAML is external-only unless a dependency ADR approves it. |
| `assert()` hides readiness failure behavior | Low | Medium | Readiness returns structured result DTOs; assert is non-authoritative developer aid only. |
| File writes silently fail or produce partial reports | Medium | Medium | Check `FileAccess.open()` errors and all `store_*` bool returns; surface deterministic tool errors. |
| Provisional work is over-blocked | Medium | Medium | ADR blocks source-authentic claims only; labeled `mvp_provisional` implementation may continue with evidence-gap notes. |

## Performance Implications

| Metric | Before | Expected After | Budget |
|--------|--------|---------------|--------|
| CPU (frame time) | No registry checks | No gameplay-frame cost; registry validation is offline/tooling/bootstrap only | 0 ms in hot gameplay paths |
| Memory | No central normalized evidence model | Tooling/bootstrap may allocate DTO/tables proportional to evidence rows | Not part of runtime memory budget unless a debug/editor tool is explicitly loaded |
| Load Time | No evidence validation | Optional bootstrap/import validation cost only when checking copied evidence refs | Must not block normal gameplay loads unless a story explicitly requires validation mode |
| Network | None | None; no Godot networking API is introduced | 0 KB/s |

## Migration Plan

1. Create the ADR and registry stances.
2. Author or normalize an initial OpenMir2 evidence registry source document using stable `evidence_id`, `contract_id`, and `contract_version` fields.
3. Update downstream GDDs/ADRs that use `openmir2_verified`, evidence refs, or source-status labels to cite registry IDs instead of prose-only evidence.
4. Add story-readiness or review checks that fail source-authentic claims without Accepted readiness results.
5. Keep `mvp_provisional` systems explicitly labeled until evidence reaches readiness.

**Rollback plan**: If this ADR proves too heavy for Phase 1, supersede it with a leaner source-governance ADR that still preserves stable IDs, source tiers, E3/E4 readiness, and false-authenticity prevention. Do not delete existing evidence IDs; mark them Deprecated or Superseded.

## Validation Criteria

- [ ] A registry source row can be normalized into `OpenMir2EvidenceRecord` without exposing mutable internal arrays/dictionaries.
- [ ] Duplicate `evidence_id` or `contract_id` fails validation deterministically.
- [ ] A missing contract returns `MISSING_CONTRACT`, not null or a fabricated fallback.
- [ ] A Proposed, Superseded, Deprecated, E2, conflicting, or non-Tier-1-backed contract fails readiness.
- [ ] An Accepted E3/E4 Tier-1-backed Adopt/Simplify contract with complete fields passes readiness.
- [ ] A raw E3/E4 evidence record alone cannot authorize `openmir2_verified` without Accepted evidence/contract readiness.
- [ ] MinimalMirClient/client observation cannot become behavior authority.
- [ ] Godot-native write tooling checks `FileAccess.open()` and all `store_*` boolean returns.
- [ ] No runtime gameplay system queries `OpenMir2EvidenceRegistry` in movement/combat/drop/pickup/inventory/equipment hot paths.
- [ ] A downstream intentional divergence record includes affected contract, reason, preserved semantics, sacrificed semantics, affected systems, and verification plan.

## GDD Requirements Addressed

| GDD Document | System | Requirement | How This ADR Satisfies It |
|-------------|--------|-------------|--------------------------|
| `design/gdd/game-concept.md` | Concept | `TR-concept-003`: OpenMir2 behavior/protocol alignment must be source-first; MinimalMirClient is not authoritative. | Defines Tier 1 OpenMir2 source authority and forbids MinimalMirClient as behavior authority. |
| `design/gdd/game-concept.md` | Concept | `TR-concept-005`: Phase 1 does not depend on full networking; protocol is only spike/future reference. | Allows protocol evidence as source mapping only and forbids Godot networking implementation creep. |
| `design/gdd/openmir2-behavior-mapping-spike.md` | OpenMir2 Behavior Mapping Spike | `TR-openmir2-spike-001`: cover map, movement, combat, death, spawn, drop, ground item, pickup, inventory, equipment, and minimal protocol entry. | Establishes one registry for all behavior-domain evidence and contracts. |
| `design/gdd/openmir2-behavior-mapping-spike.md` | OpenMir2 Behavior Mapping Spike | `TR-openmir2-spike-002`: OpenMir2 source is Tier 1; MirServer, mir2x, MinimalMirClient cannot decide behavior alone. | Makes source tiers a registry-level validation rule. |
| `design/gdd/openmir2-behavior-mapping-spike.md` | OpenMir2 Behavior Mapping Spike | `TR-openmir2-spike-003`: Phase 1 Required behavior must reach E3 before downstream contract use. | Defines readiness checks with default E3 minimum and fail-closed behavior. |
| `design/gdd/openmir2-behavior-mapping-spike.md` | OpenMir2 Behavior Mapping Spike | `TR-openmir2-spike-004`: mapping items need structured source, symbol, evidence, trigger, precondition, state change, failure, decision, confidence, and contract fields. | Defines normalized evidence/contract/divergence records and stable IDs. |
| `design/gdd/openmir2-behavior-mapping-spike.md` | OpenMir2 Behavior Mapping Spike | `TR-openmir2-spike-005`: Adopt/Simplify/Exclude/Defer decisions must be explicit. | Defines decision enum and accepted contract readiness semantics. |
| `design/gdd/openmir2-behavior-mapping-spike.md` | OpenMir2 Behavior Mapping Spike | `TR-openmir2-spike-006`: do not invent behavior values below E3/E4. | Blocks source-authentic claims and `openmir2_verified` labels without readiness. |
| `design/gdd/openmir2-behavior-mapping-spike.md` | OpenMir2 Behavior Mapping Spike | `TR-openmir2-spike-007`: output provisional contracts for core behavior domains. | Defines stable contract IDs, versions, lifecycle, and readiness validation. |
| `design/gdd/openmir2-behavior-mapping-spike.md` | OpenMir2 Behavior Mapping Spike | `TR-openmir2-spike-008`: downstream divergence from E3/E4 contracts must be intentional and documented. | Defines `OpenMir2DivergenceRecord` and divergence contract. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | Map Coordinate / Blocking / Y-sort | `TR-map-space-003`: OpenMir2-derived and MVP provisional rules must be labeled; source-authentic behavior requires E3/E4. | Provides readiness gating and provisional/authentic labeling rules. |
| `design/gdd/map-coordinate-blocking-y-sort-system.md` | Map Coordinate / Blocking / Y-sort | `TR-map-space-016`: diagonal, corner-cutting, distance metric, pickup/attack range remain evidence-gated. | Blocks source-authentic distance/movement claims until an Accepted contract passes readiness. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Source/evidence labels must distinguish provisional and OpenMir2 verified values. | Defines accepted evidence/contract requirements for `openmir2_verified` attribute-related claims. |
| `design/gdd/item-definition-system.md` | Item Definition | Source/evidence labels are required and `openmir2_verified` requires E3/E4 evidence ref. | Defines the central evidence registry that item definitions can reference during validation/bootstrap. |
| `design/gdd/systems-index.md` | Systems Index | MVP systems depend on OpenMir2 behavior mapping as upstream authority. | Makes the upstream authority stable, versioned, and queryable for downstream systems. |

## Related

- `design/gdd/openmir2-behavior-mapping-spike.md`
- `design/gdd/systems-index.md`
- `docs/architecture/architecture-review-2026-06-05.md`
- `docs/architecture/tr-registry.yaml`
- `docs/architecture/adr-0001-map-data-representation.md`
- `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`
- `docs/architecture/adr-0011-attribute-fixture-config-loading-strategy-mvp-provisional-values.md`
- `docs/architecture/adr-0015-item-definition-runtime-data-validation-query-and-versioning.md`
