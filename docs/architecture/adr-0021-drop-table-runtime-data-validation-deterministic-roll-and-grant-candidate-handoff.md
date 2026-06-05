# ADR-0021: Drop Table Runtime Data, Validation, Deterministic Roll, and Grant Candidate Handoff

## Status

Accepted

## Date

2026-06-05

## Last Verified

2026-06-05

## Decision Makers

hkm + Claude Code Game Studios; source design: `design/gdd/drop-table-system.md`; engine context: Godot 4.6.3 reference docs.

## Summary

This ADR defines the implementation boundary for the Phase 1 Drop Table System after its GDD was independently reviewed and approved. We choose a scene-tree-independent `DropTableRuntimeCatalog` that validates authored drop table data against Item Definition, OpenMir2 evidence governance, deterministic RNG contracts, and quantity rules, then emits replayable item grant candidate DTOs for ADR-0017 GroundDrop/Pickup handoff without owning ground placement, pickup, inventory, equipment, item template truth, or presentation.

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Core / Economy / Scripting / Testing |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff; this ADR intentionally avoids post-cutoff gameplay APIs for core authority. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md`; `docs/architecture/adr-0015-item-definition-runtime-data-validation-query-and-versioning.md`; `docs/architecture/adr-0016-openmir2-evidence-mapping-registry-and-provisional-contract-governance.md`; `docs/architecture/adr-0017-drop-table-ground-drop-and-pickup-lifecycle-boundary.md`; `design/gdd/drop-table-system.md` |
| **Post-Cutoff APIs Used** | None required. Runtime core does not depend on `TileMapLayer`, physics, navigation, signals, `ResourceLoader`, `FileAccess`, SceneTree, timers, frames, AccessKit, or Godot 4.5+ language features as gameplay authority. |
| **Verification Required** | Verify deterministic replay, exact weighted boundary selection, no global RNG/time/frame/signal/scene-order authority, no Dictionary-order nondeterminism, no hot-path OpenMir2 evidence registry query, Item Definition spawn eligibility validation, item definition version type compatibility, evidence-label validation, quantity validation, immutable-by-contract DTO boundaries, and guard-runner replacement before claiming implementation Done. |

> **Note**: Godot 4.6.3 is post-cutoff. If the project upgrades engine versions or selects a test addon with incompatible behavior, re-validate this ADR or supersede it.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | ADR-0015 Item Definition Runtime Data, Validation, Query, and Versioning; ADR-0016 OpenMir2 Evidence Mapping Registry and Provisional Contract Governance; ADR-0017 Drop Table, Ground Drop, and Pickup Lifecycle Boundary; `design/gdd/drop-table-system.md`. |
| **Enables** | Drop Table implementation stories; deterministic drop table unit tests; reward-source-to-grant-candidate integration; GroundDrop handoff tests; future Drop/Pickup, Inventory, Equipment, Loot Feedback, and QA tooling stories. |
| **Blocks** | Any story that rolls monster rewards, validates drop tables for normal gameplay, emits item grant candidates, claims drop-table probabilities, or claims OpenMir2-authentic drop behavior. |
| **Ordering Note** | This ADR intentionally remains below GroundDrop/Pickup lifecycle ownership from ADR-0017. Drop Table implementation may begin only after the real approved Godot test runner is installed/wired or the story explicitly remains design-only; implementation Done requires passing automated evidence. |

## Context

### Problem Statement

The Phase 1 loot-loop needs monsters or reward sources to produce validated item grant candidates with deterministic probability behavior. ADR-0017 already defines the broad reward pipeline and says Drop Table owns validated roll policy, but implementation needs a narrower contract: what data Drop Table owns, how validation fails, how deterministic selection works, how quantities are calculated, what DTOs are emitted, and what it must not mutate.

Without this ADR, Drop Table implementation could silently skip invalid rows, copy item definition truth, use global RNG, depend on scene order, reroll after placement failure, vary probabilities based on inventory state, or directly create ground/inventory/equipment objects.

### Current State

- `design/gdd/drop-table-system.md` is approved for ADR authoring after independent review and targeted fixes.
- `design/registry/entities.yaml` registers MVP provisional fixture item IDs and Drop Table formulas.
- ADR-0015 owns Item Definition template truth and `spawn_eligible_reference`.
- ADR-0016 owns source-first evidence governance and prevents false `openmir2_verified` claims.
- ADR-0017 owns the larger DropTable → GroundDrop → Pickup staged lifecycle boundary.
- Test infrastructure exists, but `tests/gdunit4_runner.gd` is still a failing guard until an approved GUT/GdUnit4 addon is installed and wired.

### Constraints

- Runtime logic must be deterministic, replayable, and unit-testable.
- Core authority must be scene-tree-independent and injectable.
- Drop Table must not depend on Node lifecycle, Autoloads, UI, physics, navigation, signals, timers, frames, filesystem fixtures, ResourceLoader cache, or global RNG.
- Drop Table cannot own item template truth, ground item lifecycle, map occupancy, pickup, inventory receive, equipment legality, save truth, or presentation.
- All concrete probabilities and item IDs remain `mvp_provisional` until source evidence/contracts or final content data replace them.
- Invalid normal gameplay tables fail whole-table validation instead of silently altering probabilities.

### Requirements

- Represent authored drop tables as data-driven rows with stable IDs and explicit order.
- Validate normal gameplay tables before runtime selection.
- Resolve item references through Item Definition spawn eligibility.
- Enforce ADR-0016 evidence readiness for `openmir2_verified` labels.
- Select no-drop or item rows through deterministic weighted selection with exact boundary behavior.
- Roll quantities deterministically and validate against Item Definition stack/equipment constraints.
- Emit one replayable result DTO per group attempt, or an ordered aggregate of group-attempt DTOs.
- Return structured failures with deterministic reason ordering.
- Emit item grant candidates only; never create or mutate ground drops, inventory entries, equipment state, item definitions, or presentation.

## Decision

Implement Drop Table as a scene-tree-independent runtime catalog and roll service:

1. `DropTableRuntimeCatalog` owns validated normalized drop table definitions.
2. `DropTableValidator` validates authoring/runtime data against table identity, group/row structure, weights, Item Definition spawn eligibility, quantity policy, and evidence labels.
3. `DropRollService` performs deterministic weighted selection and quantity rolls from injected RNG/fake RNG inputs.
4. `DropRollResult` / `DropGroupRollResult` DTOs report `ROLLED_DROP`, `NO_DROP`, or structured failure with replay provenance.
5. `DropGrantCandidate` DTOs contain item reference, quantity intent, definition version policy/result, and roll provenance only.

Drop Table does **not** create ground-drop records. GroundDropService consumes `DropGrantCandidate` under ADR-0017 and owns placement/lifecycle. Inventory later consumes staged pickup results, not Drop Table results directly. `drop_table_version` may be an integer table revision, but Item Definition version identity inside grant candidates must use ADR-0015/ADR-0018-compatible semantic version truth such as `StringName` or a typed value object with equivalent durable encoding.

### Architecture

```text
Monster death / reward source context
        |
        v
DropRollRequest
(source_context_id, drop_table_id, rng_stream_id, roll_sequence)
        |
        v
DropTableRuntimeCatalog
(prevalidated DropTableDefinition tables)
        |
        +--> ItemDefinitionRuntimeCatalog query
        |       (spawn_eligible_reference, stack policy, definition version)
        |
        +--> OpenMir2EvidenceReadiness query
        |       (bootstrap/import/tool/test validation only;
        |        accepted evidence/contract labels are copied into the catalog)
        |
        v
DropRollService
(weighted selection + quantity roll using injected deterministic RNG)
        |
        v
DropGroupRollResult / DropRollResult
        |
        +--> NO_DROP result
        +--> structured failure result
        +--> DropGrantCandidate(item_id, definition_version, quantity, provenance)
                 |
                 v
          GroundDropService / ADR-0017 handoff
```

### Key Interfaces

Conceptual GDScript contracts:

```gdscript
class_name DropTableRuntimeCatalog
extends RefCounted

func validate_table(table_data: DropTableDefinitionData, item_catalog: ItemDefinitionRuntimeCatalog, evidence_registry: OpenMir2EvidenceRegistry) -> DropTableValidationResult:
    # Evidence registry access is allowed only during bootstrap/import/tool/test validation,
    # never inside DropRollService.roll_table() or normal monster death roll hot paths.
    pass

func get_table_result(drop_table_id: StringName, requested_version: int) -> DropTableQueryResult:
    pass
```

```gdscript
class_name DropRollService
extends RefCounted

func roll_table(request: DropRollRequest, catalog: DropTableRuntimeCatalog, rng: DropDeterministicRng) -> DropRollResult:
    pass
```

```gdscript
class_name DropRollRequest
extends RefCounted

# Immutable-by-contract DTO.
# Required conceptual fields:
# - source_context_id: StringName
# - drop_table_id: StringName
# - drop_table_version: int
# - rng_stream_id: StringName
# - roll_sequence: int
# - source_tags: Array[StringName]
```

```gdscript
class_name DropGroupRollResult
extends RefCounted

# Immutable-by-contract DTO.
# Required conceptual fields:
# - status: DropRollStatus
# - drop_table_id: StringName
# - drop_table_version: int
# - source_context_id: StringName
# - rng_stream_id: StringName
# - roll_sequence: int
# - group_id: StringName
# - row_id: StringName or empty for no-drop/failure
# - grant_candidate: DropGrantCandidate or null
# - primary_reason: DropRollReason
# - secondary_reasons: Array[DropRollReason]
```

```gdscript
class_name DropGrantCandidate
extends RefCounted

# Immutable-by-contract DTO consumed by GroundDropService.
# Required conceptual fields:
# - item_id: StringName
# - item_definition_version: StringName
# - definition_version_policy: int
#   Policy enum/int is local validation policy only; it is not durable item version truth.
# - quantity: int
# - drop_table_id: StringName
# - drop_table_version: int
# - group_id: StringName
# - row_id: StringName
# - source_context_id: StringName
# - rng_stream_id: StringName
# - roll_sequence: int
# - source_status: int
# - evidence_ref: StringName or empty
```

```gdscript
class_name DropDeterministicRng
extends RefCounted

func next_int_inclusive(min_value: int, max_value: int, context: DropRngDrawContext) -> DropRngIntResult:
    pass
```

> Naming note: exact enum names and DTO fields may be refined during implementation, but public contracts must preserve status-bearing results, immutable-by-contract behavior, deterministic replay fields, and no-authority boundaries.

### Validation Stages

Drop table validation must use deterministic stages:

1. **Table identity stage**: table ID, version, source status, evidence reference.
2. **Group stage**: group IDs, explicit group order, selection mode, roll count, total weight bounds.
3. **Row stage**: row IDs, explicit row order, weight bounds, item reference syntax, source labels.
4. **Item reference stage**: Item Definition lookup and `spawn_eligible_reference` for selectable rows; syntactic reference validity for zero-weight normal rows.
5. **Quantity stage**: integer min/max/fixed values, equipment quantity `1`, material stack limits.
6. **Evidence stage**: `openmir2_verified` requires Accepted evidence or Accepted contract readiness under ADR-0016.
7. **Output stage**: normalized immutable table rows and stable indexes.

Failure reasons are ordered by validation stage, then table/group/row authoring order, then reason enum order. Dictionary iteration order must never affect failures.

### Deterministic Roll Rules

- `rng_int(a, b)` is inclusive at both ends.
- Weighted selection uses candidates in stable order: explicit no-drop candidate first, then rows by `row_order` unless a later ADR supersedes this order.
- Selection uses `roll_value = rng_int(0, drop_group_total_weight - 1)` and chooses the first candidate with `cumulative_weight_candidate > roll_value`.
- Quantity uniform range uses `quantity_min + rng_int(0, quantity_max - quantity_min)`.
- Blocking tests use fake RNG values directly, not seed-hunting.
- Statistical sampling tests are advisory only and must not be blocking unless deterministic and non-flaky.

### Runtime Failure Policy

- Missing table returns `MISSING_TABLE`.
- Invalid table returns `INVALID_TABLE` or a more specific validation result.
- Invalid RNG context returns `RNG_STREAM_INVALID`.
- No-drop returns `NO_DROP`, not failure.
- A valid roll whose downstream ground placement later fails remains a valid Drop Table grant candidate; GroundDrop handles placement failure.
- Inventory full cannot alter Drop Table probabilities or retroactively change a roll.

### Implementation Guidelines

- Use `RefCounted` services/DTOs for core runtime and tests.
- Prefer typed arrays and stable arrays over dictionaries for authored order after normalization.
- If dictionaries are accepted as authoring input, normalize them into sorted/stable arrays before validation output.
- Do not use `randomize()`, global RNG, `Time`, `Performance`, frame delta, scene node order, signal order, or `get_tree()` for roll authority.
- Do not use Godot `Resource` graphs as mutable runtime authority. If `.tres` authoring is later used, import to normalized runtime DTOs first.
- Public getters returning arrays/dictionaries must return defensive copies or immutable-by-contract row DTOs.
- DTOs must expose no public mutable vars, setters, mutators, Nodes, Resources, Callables, Signals, scene objects, physics objects, or caller-owned mutable collection references.
- Nested arrays/dictionaries are either forbidden in runtime DTO payloads or recursively normalized/copied during construction; shallow copies are not enough for nested mutable containers or shared RefCounted payloads.
- OpenMir2 evidence registry queries are bootstrap/import/tool/test validation only. `DropRollService.roll_table()` must use the prevalidated catalog and must not query the evidence registry during normal monster death rolls.
- If seed-based replay is required beyond unit tests, use a project-owned deterministic integer PRNG or record the draw sequence; do not treat Godot's built-in RNG sequence as a cross-version replay guarantee.
- Keep MVP provisional rates in data/config, not hardcoded in service logic.

## Alternatives Considered

### Alternative 1: Roll directly inside monster death code

- **Description**: Monster death handler contains drop weights and returns ground/inventory items directly.
- **Pros**: Fastest short-term implementation.
- **Cons**: Couples combat/death to economy, item definitions, ground drops, inventory, and presentation; hard to test; violates ADR-0017 boundary.
- **Estimated Effort**: Low initially, high rework.
- **Rejection Reason**: The first loot-loop slice needs clean boundaries and deterministic evidence; monster death should emit reward source context, not own drops.

### Alternative 2: Scene/Node-based drop table components

- **Description**: Each monster or scene node stores a drop table script/resource and rolls through node lifecycle callbacks.
- **Pros**: Familiar Godot workflow; easy to wire in scenes.
- **Cons**: Risks scene-tree order, node lifecycle, Resource cache mutation, and presentation coupling becoming gameplay authority.
- **Estimated Effort**: Medium.
- **Rejection Reason**: Core roll authority must be scene-tree-independent; scenes may reference table IDs but not own roll truth.

### Alternative 3: Inventory-first reward grants

- **Description**: Drop Table directly grants inventory items; ground drops are visual only.
- **Pros**: Simplifies pickup and placement for early prototype.
- **Cons**: Breaks the classic ground loot fantasy, bypasses ADR-0017 no-half-commit pipeline, and prevents pickup/ground item validation.
- **Estimated Effort**: Low.
- **Rejection Reason**: Phase 1 explicitly needs visible ground loot and pickup as part of the 30-second loop.

### Alternative 4: Statistical-only probability validation

- **Description**: Validate drops by running many random samples and checking approximate distribution.
- **Pros**: Catches gross probability mistakes.
- **Cons**: Flaky, slow, seed-dependent, weak at off-by-one boundaries, and unsuitable as blocking unit evidence.
- **Estimated Effort**: Medium.
- **Rejection Reason**: Blocking tests should check exact deterministic boundaries with fake RNG; optional statistical smoke tests may remain advisory.

## Consequences

### Positive

- Drop tables become deterministic, replayable, and unit-testable.
- Item Definition truth remains centralized.
- GroundDrop/Pickup lifecycle remains the only ground/pickup state owner.
- Economy tuning can change data without changing roll service code.
- OpenMir2 authenticity claims are evidence-gated.
- QA can test exact roll boundaries and quantity outputs without relying on chance.

### Negative

- Requires more DTOs, validators, fake RNG, and fixture data than an inline prototype.
- MVP provisional item IDs must be kept synchronized with Item Definition and registry data.
- Implementation cannot claim Done until the real test runner is installed and automated tests pass.
- Multi-system handoff requires integration tests beyond unit tests.

### Neutral

- Provisional rates are validation-biased and not final long-term economy balance.
- Future dry-streak/pity systems require a separate state owner and ADR/GDD.
- Future OpenMir2-authentic tables may supersede provisional project-local tables.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| Implementer silently filters invalid rows and changes probabilities. | Medium | High | Fail whole-table validation in normal mode; test invalid-row cases. |
| Off-by-one selection bug at cumulative boundaries. | Medium | High | Fake RNG boundary tests for every MVP range edge. |
| Global RNG/time/scene order sneaks into roll authority. | Medium | High | Constructor injection of RNG; tests with fake RNG; forbid global sources in review checklist. |
| Provisional item IDs drift from Item Definition catalog. | Medium | Medium | Registry entries now exist; implementation must validate through Item Definition catalog before normal gameplay. |
| Economy rates produce too much equipment/material for long-term balance. | High | Medium | Document validation-biased rates; require later economy pass after sinks/inventory pressure exist. |
| GUT/GdUnit4 runner remains guard-only. | High | High | Keep implementation Done blocked until approved runner is installed and real tests execute. |
| DTOs expose mutable arrays/dictionaries. | Medium | Medium | Immutable-by-contract DTOs, defensive copies, aliasing tests. |
| Future multi-group tables partially emit results after failure. | Low | Medium | Invalid normal tables fail as a whole; aggregate results preserve ordered child outcomes only after successful validation. |

## Performance Implications

| Metric | Before | Expected After | Budget |
|--------|--------|---------------|--------|
| CPU (frame time) | No Drop Table runtime. | Roll work happens on kill/reward event, not per frame. MVP tables are tiny; expected O(G + R) per table validation/roll where G = groups and R = rows. | Keep gameplay under 16.6 ms/frame; drop rolls must not scan all item definitions per roll after validation. |
| Memory | No runtime catalog. | Small normalized table rows, indexes, result DTOs, and QA provenance. | Phase 1 client under 1 GB RAM. |
| Load Time | No validation catalog. | Bootstrap/content-load validation cost for drop tables and item references. | Validate once per config load/profile; avoid full validation per kill unless debug mode. |
| Network | Not applicable. | Not applicable for Phase 1 offline slice. | None. |

## Migration Plan

No existing Drop Table implementation exists.

1. Keep `tests/gdunit4_runner.gd` as guard until approved test addon is installed.
2. Install/wire the approved GUT/GdUnit4 runner.
3. Create `tests/unit/drop_table/` with first smoke test proving real runner execution.
4. Implement DTOs/enums for drop table definitions, validation results, roll requests, group results, grant candidates, and RNG draw results.
5. Implement fake deterministic RNG test helper.
6. Implement validation stages and exact failure reason ordering.
7. Implement weighted selection and quantity roll formulas.
8. Add tests for MVP table boundaries, invalid rows, item reference gating, evidence gating, quantity constraints, and no-authority boundary.
9. Add integration test for reward source context → Drop Table result → GroundDrop handoff without placement mutation by Drop Table.
10. Only then allow Drop Table implementation stories to claim Done.

**Rollback plan**: If this approach is too heavy for the first playable proof, keep the same DTO/validation boundary but temporarily author only one small table and one group. Do not collapse rolls into monster death or inventory code; that would violate accepted ADR boundaries.

## Validation Criteria

- [ ] A real approved Godot GDScript test runner executes Drop Table tests; guard-only runner is not used as passing evidence.
- [ ] `drop_group_total_weight` tests cover valid total, zero total, negative weight, zero-weight row, and overflow.
- [ ] `selected_drop_entry` tests cover exact MVP boundaries: `0`, `5999`, `6000`, `8499`, `8500`, `9699`, `9700`, `9949`, `9950`, `9999`.
- [ ] `drop_row_probability`, `expected_attempts_for_row`, and `expected_attempts_for_tier` tests cover documented MVP values.
- [ ] `drop_quantity_result` tests cover fixed equipment quantity `1`, material uniform range `1–3`, illegal zero/negative/fractional/exceeds-stack cases, and max boundary selection.
- [ ] Validation rejects missing table ID, duplicate table IDs, unsupported version, duplicate group/row IDs, missing row order, invalid source label, missing item reference, ineligible item reference, and `openmir2_verified` without accepted readiness.
- [ ] Zero-weight normal rows still require syntactically valid item reference/source/quantity data; missing placeholders are debug/test-profile only.
- [ ] Same request/table/RNG context produces identical result across repeated runs.
- [ ] Different dictionary insertion/iteration order cannot affect normalized row/group order or failure ordering.
- [ ] `DropGrantCandidate` item definition version fields round-trip into ADR-0017 GroundDrop and ADR-0018 Inventory staged receive contracts without converting semantic version truth into integer-only identity.
- [ ] Result DTOs include required provenance fields for `NO_DROP`, `ROLLED_DROP`, and structured failures.
- [ ] Drop Table never creates ground drops, mutates `MapSpaceState`, grants inventory, equips items, plays presentation, or copies item definition display/icon/quality/modifier truth.
- [ ] Ground placement failure and inventory full do not reroll or alter Drop Table probabilities.

## GDD Requirements Addressed

| GDD Document | System | Requirement | How This ADR Satisfies It |
|-------------|--------|-------------|--------------------------|
| `design/gdd/drop-table-system.md` | Drop Table | Own reward source to drop table mapping, roll groups, weights, no-drop policy, quantity policy, validation, deterministic roll results, and QA replay provenance. | Defines `DropTableRuntimeCatalog`, validation stages, deterministic roll service, result DTOs, and provenance contract. |
| `design/gdd/drop-table-system.md` | Drop Table | Must not own item template truth, ground drop lifecycle, map placement, pickup, inventory receive, equipment legality, UI/VFX/audio, or final item instance identity. | Explicitly limits output to grant candidates and records forbidden authority boundaries. |
| `design/gdd/drop-table-system.md` | Drop Table | Validate every item row through Item Definition `spawn_eligible_reference` and stack/equipment quantity constraints. | Requires Item Definition catalog dependency and validation stages for item references and quantities. |
| `design/gdd/drop-table-system.md` | Drop Table | Use deterministic weighted selection, inclusive RNG convention, and exact expected acquisition formulas. | Defines deterministic RNG injection, exact selection formula, quantity formula, probability and expected-attempt validation criteria. |
| `design/gdd/drop-table-system.md` | Drop Table | Label non-evidence-backed rates as `mvp_provisional`; block false `openmir2_verified` labels. | Requires ADR-0016 Accepted evidence/contract readiness and structured `OPENMIR2_EVIDENCE_NOT_READY` failures. |
| `design/gdd/drop-table-system.md` | Drop Table | Phase 1 QA requires exact boundary tests and deterministic fixtures instead of relying on raw rare chance. | Makes fake RNG boundary tests and guard-runner replacement implementation-blocking validation criteria. |
| `design/gdd/item-definition-system.md` | Item Definition | Drop Table validates item references and owns drop chance/quantity, but Item Definition remains template truth. | Preserves ADR-0015 ownership and uses `spawn_eligible_reference` without copying item fields. |
| `docs/architecture/adr-0017-drop-table-ground-drop-and-pickup-lifecycle-boundary.md` | Reward Pipeline | Drop Table emits validated roll/grant candidates; GroundDrop and Pickup own placement/lifecycle/commit. | Narrows ADR-0017 Drop Table portion and keeps GroundDrop/Pickup authority intact. |

## Related

- `design/gdd/drop-table-system.md`
- `design/gdd/reviews/drop-table-system-review-log.md`
- `docs/architecture/adr-0015-item-definition-runtime-data-validation-query-and-versioning.md`
- `docs/architecture/adr-0016-openmir2-evidence-mapping-registry-and-provisional-contract-governance.md`
- `docs/architecture/adr-0017-drop-table-ground-drop-and-pickup-lifecycle-boundary.md`
- `docs/registry/architecture.yaml`
- `design/registry/entities.yaml`
