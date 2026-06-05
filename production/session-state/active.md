# Active Session State

<!-- STATUS -->
Epic: Systems Design
Feature: Interaction Target / Selection System
Task: Writing 交互目标 / 选择系统 GDD; Edge Cases completed
<!-- /STATUS -->

## Current Task

Completed major-review targeted revision, autonomous Post-Design Validation, independent fresh-session review, and implementation-blocking ADR authoring for `物品定义系统`.

Independent design review result on 2026-06-04: **APPROVED FOR ADR AUTHORING** after one targeted fix. Systems review found the Open Questions MVP fixture row still said “1 normal equipment + 1 material,” which was weaker than Detailed Rules / AC-23. The row has been corrected to require: 1 baseline equipped item/reference, 1 same-category clear upgrade equipment, 1 same-category sidegrade or weaker equipment, 1 stackable material, and optional rare/showcase item.

Item Definition architecture session updates applied:

- Created `docs/architecture/adr-0015-item-definition-runtime-data-validation-query-and-versioning.md` with status `Proposed`.
- Covered runtime item definition representation, `ItemDefId`/semantic key identity, validation/query result DTO contracts, authoring/config loading, version/migration boundary, Resource authoring policy, read-only/defensive-copy rules, indexed lookup, and blocking tests.
- Updated `docs/registry/architecture.yaml` metadata and added ADR-0015 architecture stances.
- Updated `design/registry/entities.yaml` metadata and Item Definition formula registry entries including `item_definition_semantically_valid`, `item_reference_lookup_resolvable`, `definition_profile_eligible`, and `spawn_eligible_reference`.
- Updated `design/gdd/systems-index.md` row 4 to `Approved` and progress counts to 4 reviewed / 3 approved.
- Appended fresh-review approval entry to `design/gdd/reviews/item-definition-system-review-log.md`.

Next recommended steps:

1. Continue resolving the 2026-06-05 architecture review gaps by authoring the Resource / Map Conversion Pipeline ADR.
2. After this coherent ADR set is complete, run independent `/architecture-review` in a fresh Claude Code session.
3. Register concrete MVP item candidates only after first loot-loop fixture naming is approved; current example IDs remain non-canonical.

## Current Section

`物品定义系统` GDD authoring, post-design validation, independent fresh-session review, targeted review fix, and ADR-0015 authoring are complete. Current state: **Approved for implementation planning after independent architecture review; implementation blocked until ADR-0015 is reviewed/accepted**.

2026-06-05 architecture review follow-up ADRs are in progress. ADR-0016, ADR-0017, ADR-0018, ADR-0019, and ADR-0020 have been authored as Proposed and registry-synchronized. The top architecture-review gap ADR set is now covered pending an independent fresh-session `/architecture-review`.

## Active Files

- `design/gdd/item-definition-system.md`
- `design/gdd/reviews/item-definition-system-review-log.md`
- `design/gdd/systems-index.md`
- `design/registry/entities.yaml`
- `production/session-state/active.md`
- `docs/architecture/adr-0015-item-definition-runtime-data-validation-query-and-versioning.md`
- `docs/architecture/adr-0016-openmir2-evidence-mapping-registry-and-provisional-contract-governance.md`
- `docs/architecture/adr-0017-drop-table-ground-drop-and-pickup-lifecycle-boundary.md`
- `docs/architecture/adr-0018-inventory-and-equipment-instance-modifier-transaction-boundary.md`
- `docs/architecture/adr-0019-map-distance-and-movement-legality-contract.md`
- `docs/architecture/adr-0020-resource-map-conversion-pipeline-and-validation-boundary.md`
- `docs/architecture/architecture-review-2026-06-05.md`
- `docs/architecture/architecture-review-refresh-2026-06-05.md`
- `docs/architecture/traceability-index.md`
- `docs/architecture/tr-registry.yaml`
- `docs/registry/architecture.yaml`

## Progress Checklist

- [x] Draft Character Attributes GDD
- [x] Register initial cross-system formulas in `design/registry/entities.yaml`
- [x] Run full `/design-review design/gdd/character-attributes-system.md`
- [x] Receive verdict: **MAJOR REVISION NEEDED**
- [x] Revise GDD for player-facing growth, validation, events, ADR gates, and AC staging
- [x] Update `systems-index.md` row 3 to `Needs Revision` during revision phase
- [x] Create review log with major-review entry
- [x] Run full re-review
- [x] Receive verdict: **NEEDS REVISION** targeted formula/registry cleanup
- [x] Apply targeted formula and AC fixes
- [x] Synchronize `design/registry/entities.yaml`, including `combat_power`
- [x] Update `systems-index.md` row 3 to `Approved`
- [x] Append review log entries for targeted re-review and final approval
- [x] Update systems-index progress tracker counts
- [x] Create ADR-0006 for attribute data representation and stat ID typing
- [x] Update `docs/registry/architecture.yaml` with ADR-0006 stances
- [x] Create ADR-0007 for snapshot/query API shape and read-only enforcement
- [x] Update `docs/registry/architecture.yaml` with ADR-0007 stances
- [x] Create ADR-0008 for event/signal contract and scene-tree-independent core
- [x] Update `docs/registry/architecture.yaml` with ADR-0008 stances
- [x] Create ADR-0009 for atomic source update / transaction model
- [x] Update `docs/registry/architecture.yaml` with ADR-0009 stances
- [x] Create ADR-0010 for save/load persistence boundary for base/current/modifier sources
- [x] Update `docs/registry/architecture.yaml` with ADR-0010 stances
- [x] Create ADR-0011 for fixture/config loading strategy for MVP provisional values
- [x] Update `docs/registry/architecture.yaml` with ADR-0011 stances
- [x] Create ADR/technical design for formula-only GUT test strategy without scene tree, UI, or Autoload
- [x] Update `docs/registry/architecture.yaml` with ADR-0012 stances
- [x] Create ADR for Godot Resource duplication/shared-reference policy if `.tres` Resources are used
- [x] Update `docs/registry/architecture.yaml` with ADR-0013 stances
- [x] Create ADR/technical design for combat power/main stat display proxy ownership
- [x] Update `docs/registry/architecture.yaml` with ADR-0014 stances
- [x] Continue GDD flow with `/design-system 物品定义系统` after prerequisite architecture is sufficiently covered
- [x] Complete `物品定义系统` GDD authoring
- [x] Complete post-design validation and apply targeted fixes
- [x] Run independent `/architecture-review` on 2026-06-05 and record FAIL coverage report
- [x] Register 83 stable TR IDs in `docs/architecture/tr-registry.yaml`
- [x] Write ADR-0016 OpenMir2 Evidence Mapping Registry and Provisional Contract Governance
- [x] Update `docs/registry/architecture.yaml` with ADR-0016 stances
- [x] Write ADR-0017 Drop Table, Ground Drop, and Pickup Lifecycle Boundary
- [x] Update `docs/registry/architecture.yaml` with ADR-0017 stances
- [x] Write ADR-0018 Inventory and Equipment Instance Modifier Transaction Boundary
- [x] Update `docs/registry/architecture.yaml` with ADR-0018 stances
- [x] Write ADR-0019 Map Distance Facts and Movement Legality Boundary
- [x] Update `docs/registry/architecture.yaml` with ADR-0019 stances
- [x] Write ADR-0020 Resource / Map Conversion Pipeline and Validation Boundary
- [x] Update `docs/registry/architecture.yaml` with ADR-0020 stances
- [x] Run independent ADR acceptance readiness review
- [x] Fix ADR-0006 current-resource identity ambiguity
- [x] Fix ADR-0002/ADR-0019 movement unresolved reason alignment
- [x] Mark ADR-0001 through ADR-0020 as Accepted
- [x] Run independent architecture refresh after ADR acceptance
- [x] Write `docs/architecture/architecture-review-refresh-2026-06-05.md`
- [x] Update `docs/architecture/traceability-index.md` summary to 73 covered / 10 partial / 0 gaps
- [x] Set up test infrastructure (`tests/unit`, `tests/integration`, `.github/workflows/tests.yml`)
- [x] Author UX/accessibility baselines (`design/ux/interaction-patterns.md`, `design/ux/accessibility-requirements.md`)
- [x] Register Item Definition formulas in `design/registry/entities.yaml`
- [x] Update systems index row 4 to `Designed`
- [x] Run independent `/design-review design/gdd/item-definition-system.md` in a fresh Claude Code session
- [x] Apply targeted Open Questions MVP fixture fix from fresh review
- [x] Update systems index row 4 to `Approved`
- [x] Create implementation-blocking Item Definition ADR after design review
- [x] Update `docs/registry/architecture.yaml` with ADR-0015 stances
- [ ] Run independent `/architecture-review` for the coherent ADR set, including ADR-0015, in a fresh Claude Code session

## ADR-0006 Summary — Attribute Data Representation and Stat ID Typing

Written: 2026-06-04

File: `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`

Decision summary:

- Character Attributes runtime uses project-owned compact typed `StatId` integer IDs as authoritative in-memory stat identity.
- External config/save/debug/localization/editor boundaries use stable `StringName` semantic keys mapped to `StatId` during load/normalization.
- `StatId` is contiguous, zero-based, append-only, includes `COUNT`, and is not durable save/config truth without migration.
- Runtime stat values may use `Array[int]`, `PackedInt32Array`, or `PackedInt64Array` only after numeric range is documented and overflow/truncation behavior is tested.
- Godot `Resource` / `.tres` objects may author config but must be normalized/copied before gameplay use.
- Typed `RefCounted` DTOs may represent attribute sources and modifiers, but implementations must guard against shared mutable aliasing.
- Phase 1 accepts only `ADD_FLAT` modifier operations; unsupported operations fail deterministically.
- Snapshot/query API shape is resolved by ADR-0007.

Validation outcomes:

- Godot specialist verdict: `CONCERNS`, resolved by adding guardrails for enum ordinal drift, packed integer width, Resource mutation/cache sharing, RefCounted DTO aliasing, StringName boundary misuse, and GDScript runtime validation limits.
- Technical Director TD-ADR verdict: `CONCERNS`, resolved by writing the same guardrails and tightening Enables/Blocks language.

Registry stances added in `docs/registry/architecture.yaml`:

- State ownership: `attribute_runtime_state`
- Interface contract: `attribute_stat_id_contract`
- API decision: `attribute_runtime_stat_identity`
- Forbidden patterns:
  - `raw_string_stat_map_as_runtime_authority`
  - `resource_as_mutable_attribute_runtime_state`
  - `enum_ordinal_as_durable_attribute_identity`
  - `unchecked_packed_attribute_integer_truncation`
  - `shared_mutable_attribute_dto_payload`

## ADR-0007 Summary — Attribute Snapshot Query API Shape and Read-Only Enforcement

Written: 2026-06-04

File: `docs/architecture/adr-0007-attribute-snapshot-query-api-shape-and-read-only-enforcement.md`

Decision summary:

- Character Attributes publishes snapshots as `RefCounted` immutable-by-contract value objects.
- Consumers obtain snapshots and stat/resource values through status-bearing query result wrappers such as `AttributeSnapshotQueryResult`, `AttributeStatQueryResult`, and `AttributeResourceQueryResult`.
- GDScript does not enforce true immutability/private fields; read-only behavior is enforced by API shape, construction-time copies, no public vars/setters/mutators, defensive collection rules, code review, and tests.
- Snapshot/provider APIs preserve ADR-0006 `StatId` semantics. If implemented as enum/int constants, raw arbitrary integers remain invalid until validated.
- Failed rebuild does not replace the current valid snapshot. Previous valid snapshots may be exposed only as stale/display-only and status must be visible in both wrapper and snapshot metadata.
- Arrays/dictionaries from runtime backing state must not be exposed. Scalar arrays may be shallow defensive copies; DTO arrays must clone rows or use immutable-by-contract row DTOs.
- Debug trace is lazy/pull-based or invalid/debug-only, not embedded as normal snapshot payload.

Validation outcomes:

- Godot specialist verdict: `CONCERNS`, resolved by clarifying immutable-by-contract semantics, shallow copy risks, public API `StatId` semantics, result DTO immutability, direct constructor safety, and stale status propagation.
- Technical Director TD-ADR verdict: `APPROVE` after those revisions.

Registry stances added in `docs/registry/architecture.yaml`:

- Interface contract: `attribute_snapshot_query_contract`
- API decision: `attribute_snapshot_read_only_representation`
- Forbidden patterns:
  - `mutable_attribute_snapshot_exposure`
  - `snapshot_status_bypass`
  - `snapshot_debug_trace_eager_payload`
  - `dictionary_only_attribute_snapshot_payload`
  - `public_mutable_attribute_result_dto`

## ADR-0008 Summary — Attribute Event Signal Contract and Scene-Tree-Independent Core

Written: 2026-06-04

File: `docs/architecture/adr-0008-attribute-event-signal-contract-and-scene-tree-independent-core.md`

Decision summary:

- Character Attributes core is scene-tree-independent and implemented as injectable `RefCounted` service/core classes, not Node, Autoload, or scene signal authority.
- Phase 1 authoritative update paths use one result envelope: `AttributeUpdateResult`.
- `AttributeRebuildEvent`, `ResourceChangedEvent`, and `AttributeInvalidatedEvent` are compact immutable-by-contract domain event DTOs.
- `AttributePreviewResult` is a non-authoritative result DTO, not an `AttributeDomainEvent`, and never enters committed event arrays.
- Optional `AttributeEventSink` is a typed `RefCounted` sink contract; sink invocation happens only after result materialization and is synchronous best-effort in Phase 1.
- Godot signals are allowed only in downstream typed signal adapters after the core result is decided; signal callback order cannot affect transaction results, state, versions, combat truth, save truth, or growth eligibility.

Validation outcomes:

- Godot specialist verdict: `APPROVE with CONCERNS`, resolved by clarifying immutable-by-contract DTO limitations, event sink implementation without `@abstract`, typed signal adapter style, preview non-authority, coalescing, and RefCounted lifecycle risks.
- Technical Director TD-ADR verdict: `CONCERNS`, resolved by standardizing on one `AttributeUpdateResult` envelope, binding event ordering, defining sink timing/failure behavior, standardizing `AttributeEventSink`, and clarifying preview is not an event.

Registry stances added in `docs/registry/architecture.yaml`:

- Interface contracts:
  - `attribute_update_result_contract`
  - `attribute_domain_event_contract`
  - `attribute_signal_adapter_contract`
- API decisions:
  - `attribute_scene_tree_independent_core`
  - `attribute_event_sink_contract`
- Forbidden patterns:
  - `attribute_core_scene_tree_dependency`
  - `attribute_signal_callback_order_authority`
  - `attribute_event_payload_as_full_state_authority`
  - `attribute_preview_as_committed_event`
  - `mutable_attribute_event_result_dto`
  - `autoload_attribute_event_bus_phase1`

## ADR-0009 Summary — Attribute Atomic Source Update and Transaction Model

Written: 2026-06-04

File: `docs/architecture/adr-0009-attribute-atomic-source-update-and-transaction-model.md`

Decision summary:

- Character Attributes structural rebuilds use actor-local synchronous transactions with copied candidate state and final commit/swap.
- External systems submit typed transaction/current-resource mutation requests; they do not directly mutate committed attribute sources or runtime state.
- `source_version` advances exactly once only on committed structural source transactions.
- `snapshot_version` advances exactly once when a new current valid published snapshot/resource observable state is created; Phase 1 snapshots include current HP/MP values.
- Failed/rejected structural transactions leave committed source set, source version, snapshot version, current snapshot, and current resources unchanged.
- Current-resource mutation means HP/MP current-value mutation, not Godot `Resource` mutation; it advances `snapshot_version`, not `source_version`, and does not reaggregate modifiers.
- Equipment replacement is one atomic source delta; consumers never observe old+new both active or neither active on success.
- Version expectations detect stale source/snapshot commands before staging or calculation.
- Phase 1 rejects nested/reentrant mutation during transaction, result materialization, or result dispatch.

Validation outcomes:

- Godot specialist verdict: `APPROVE`, with recommended guardrails incorporated for candidate commit/swap, reentrant dispatch state, staged copy/deep-copy boundaries, version token defaults, current-resource naming, and no async/yield in transaction path.
- Technical Director TD-ADR verdict: `CONCERNS`, resolved by clarifying final commit/result ordering, making current resources part of published snapshots, defining resource correction policy, defining Phase 1 `request_id` semantics, adding a version expectation table, and specifying read behavior during transaction/dispatch.

Registry stances added in `docs/registry/architecture.yaml`:

- Interface contracts:
  - `attribute_transaction_request_contract`
  - `attribute_current_resource_mutation_contract`
  - `attribute_version_expectation_contract`
- API decisions:
  - `attribute_atomic_candidate_commit_model`
  - `attribute_current_resource_snapshot_version_policy`
- Forbidden patterns:
  - `direct_attribute_source_set_mutation`
  - `attribute_mutate_then_rollback_transaction`
  - `attribute_reentrant_mutation`
  - `attribute_candidate_state_exposure`
  - `attribute_resource_mutation_reaggregates_modifiers`
  - `attribute_versionless_source_update`
  - `attribute_debug_mutation_bypass`

## Approved Character Attributes Status

- `design/gdd/character-attributes-system.md`: `Approved for ADR Authoring — implementation blocked pending ADRs`.
- `design/gdd/systems-index.md`: row 3 `角色属性系统` status is `Approved`.
- `design/gdd/reviews/character-attributes-system-review-log.md`: approval entry appended.

## Key Review Findings Resolved in GDD

The approved GDD now includes:

- Player-facing growth handoff rule.
- Phase 1 stat registry with actor type, visibility, combat activity, modifier targetability, and source status.
- Provisional `combat_power` display formula.
- Equipment preview boundary.
- Attribute layers separating identity, base stats, current resources, derived stats, modifiers, snapshots, and debug trace.
- Structural rebuild vs resource mutation update paths.
- Atomic source update rule.
- Four-stage validation pipeline.
- Resource correction policy by reason.
- Default invalid modifier policy.
- Snapshot/version/query contract.
- Compact domain event categories.
- UI/presentation handoff including local-player HUD binding, stale/invalid player-safe states, and localization/display metadata.
- Save/load `AttributePersistentInput` contract.
- Implementation-blocking ADR / technical design gates.
- Performance guardrails.
- Revised formulas for `effective_stat`, `effective_stat_pair`, `current_resource_after`, `snapshot_valid`, `attribute_delta`, `snapshot_delta`, and MVP provisional `combat_power`.
- Formula-wide structured failure propagation and numeric safety conventions.
- Tuning knobs with safe Phase 1 ranges and gameplay impact.
- Rewritten AC-01 through AC-17 with clearer staging and evidence wording.
- Appendix open questions.

## Registry Sync Written 2026-06-04

Updated `design/registry/entities.yaml` formula entries for:

- `effective_stat`
- `effective_stat_pair`
- `current_resource_after`
- `snapshot_valid`
- `attribute_delta`
- `snapshot_delta`
- `combat_power`

Updated `docs/registry/architecture.yaml` for ADR-0006, ADR-0007, ADR-0008, ADR-0009, ADR-0010, ADR-0011, ADR-0012, ADR-0013, and ADR-0014 as listed above.

## ADR-0012 Summary — Attribute Formula Only GUT Test Strategy Without Scene Tree UI Autoload

Written: 2026-06-04

File: `docs/architecture/adr-0012-attribute-formula-only-gut-test-strategy-without-scene-tree-ui-autoload.md`

Decision summary:

- Phase 1 Character Attributes blocking formula/contract evidence comes from GUT unit tests under `tests/unit/character_attributes/`.
- Blocking tests directly instantiate injectable `RefCounted` / service cores, typed DTOs, config factories/normalizers, validated runtime config tables, fake event sinks, fake counters, and committed-state probes.
- Blocking formula/contract tests must not depend on gameplay scenes, Node lifecycle, SceneTree APIs, Autoloads, UI/Control state, real `.tscn`/`.tres` files, ResourceLoader, FileAccess, filesystem fixtures, timers/frames, profiler timing, or signal callback order.
- Scope is intentionally limited to Character Attributes formula/contract blocking unit tests; Save/Load IO tests, `.tres`/JSON pipeline integration tests, signal adapter tests, HUD/UI evidence, scene smoke tests, and downstream integration tests remain allowed but cannot substitute for formula/contract evidence.
- Config-normalizer tests must start from primitive semantic-key payloads or factory payload output; formula kernel tests may inject normalized runtime config only when not claiming config-loading coverage.
- Required coverage includes `effective_stat`, `effective_stat_pair`, `current_resource_after`, `snapshot_valid`, `attribute_delta`, `snapshot_delta`, `combat_power`, source-status/evidence validation, invalid modifier policy, preview non-authority, failed rebuild stale/display-only fallback, no-side-effect failed transactions, current-resource-only mutation, reentrant mutation rejection, event/result DTOs, debug trace minimal payload, aliasing boundaries, and O(M + S) fake instrumentation evidence.

Validation outcomes:

- Godot specialist verdict: `APPROVE`, with risks incorporated around GUT runner SceneTree wording, `RefCounted`/Array/Dictionary aliasing, shallow/deep copy, typed containers after Variant input, non-substitutive integration tests, and fake counter performance evidence.
- Technical Director TD-ADR verdict: `CONCERNS`, resolved by strengthening scope, engine compatibility, config-normalizer coverage, combat power/snapshot delta/source-status/preview/max-resource coverage, reentrancy tests, integration-test non-substitution wording, and registry stances.

Registry stances added in `docs/registry/architecture.yaml`:

- Interface contract: `attribute_formula_blocking_test_contract`
- API decision: `attribute_formula_unit_test_harness`
- Forbidden patterns:
  - `scene_tree_dependent_attribute_blocking_tests`
  - `filesystem_resource_attribute_formula_fixtures`
  - `attribute_integration_evidence_substitutes_formula_tests`
  - `signal_order_attribute_test_oracle`

## ADR-0013 Summary — Attribute Godot Resource Duplication and Shared Reference Policy If `.tres` Resources Are Used

Written: 2026-06-04

File: `docs/architecture/adr-0013-attribute-godot-resource-duplication-shared-reference-policy-if-tres-resources-are-used.md`

Decision summary:

- Phase 1 Character Attributes remains factory-first and does not use Godot `Resource` / `.tres` instances as runtime authority.
- Future `.tres` Resources may be used only as schema-driven authoring envelopes loaded by bootstrap/importer code.
- Importers must read explicit whitelisted fields only, reject unsupported Object/Resource/RefCounted/Callable/Signal payloads by default, and route output through ADR-0011 normalization.
- Runtime core, formula tests, save truth, snapshots, transactions, results, and config tables must not retain loaded or duplicated Resource graphs or ResourceLoader cache entries as authority.
- `Resource.duplicate()` is not accepted as nested isolation. `Resource.duplicate_deep()` may be tested as an importer/tooling aid only and never makes Resource graphs runtime authority.
- Importer/bootstrap Resource tests are supplemental and cannot substitute for ADR-0012 formula/contract GUT evidence.

Validation outcomes:

- Godot specialist verdict: `APPROVE`, with notes incorporated around `duplicate_deep()`, Array/Dictionary copy limits, Variant whitelist, DTO/table copy rules, ResourceLoader cache policy, and Resource isolation tests.
- Technical Director TD-ADR verdict: `CONCERNS`, resolved by adding schema-driven import, deterministic failure ordering, root-envelope vs nested-object policy, narrower `duplicate_deep()` wording, post-normalization mutation validation, and reflection-schema forbidden pattern.

Registry stances added in `docs/registry/architecture.yaml`:

- API decision: `attribute_resource_authoring_boundary`
- Forbidden patterns:
  - `mutable_resource_graph_attribute_runtime_authority`
  - `resource_duplicate_as_deep_isolation`
  - `resource_loader_cache_attribute_authority`
  - `resource_backed_formula_unit_fixtures`
  - `resource_reflection_as_attribute_schema`

## ADR-0014 Summary — Attribute Combat Power and Main Stat Display Proxy Ownership

Written: 2026-06-04

File: `docs/architecture/adr-0014-attribute-combat-power-main-stat-display-proxy-ownership.md`

Decision summary:

- Character Attributes owns provisional display-only computation and semantic handoff metadata for `combat_power_before`, `combat_power_after`, `combat_power_delta`, `primary_comparison_stat`, visible/hidden delta summaries, `growth_reason`, and `growth_salience`.
- Combat power is `mvp_provisional` and `display_only`; it is not damage/TTK authority, equip legality, item valuation, save truth, OpenMir2-authentic truth, UI layout, localization rendering, or celebration authority when stale/invalid/unavailable.
- Combat never reads display proxy DTOs for damage/hit/crit/TTK. Equipment owns legality/category/modifier resolution. UI/growth owns presentation and must not recompute Attribute formulas.
- Valid accepted growth-relevant rebuilds without equipment hints use deterministic primary-stat fallback from non-zero visible deltas by configured priority.
- Visible/hidden delta rows must carry AC-08 before/after/delta/reason/version/display metadata fields.
- Config validation rejects no active player-facing positive combat power contribution and nonzero inactive/reserved weights unless a later debug-only ADR permits them.
- Preview display proxy data remains non-authoritative and never enters committed events or signals.

Validation outcomes:

- Godot specialist verdict: `APPROVE`, with notes incorporated around Proposed dependencies, config failure split, equipment hint boundaries, no `@abstract` requirement, UI stale behavior, signal/result boundary, DTO aliasing, compact event payloads, and display metadata vs localization.
- Technical Director TD-ADR verdict: `CONCERNS`, resolved by adding deterministic missing-hint fallback, explicit `combat_power_before/after`, AC-08 delta row fields, active player-facing contribution validation, display proxy status mapping, and preview non-authority wording.

Registry stances added in `docs/registry/architecture.yaml`:

- Interface contract: `attribute_growth_display_proxy_contract`
- API decision: `attribute_display_proxy_authority`
- Forbidden patterns:
  - `combat_power_as_damage_authority`
  - `ui_recomputed_attribute_display_formula`
  - `stale_invalid_growth_celebration`
  - `equipment_ui_direct_attribute_formula_access`
  - `display_proxy_as_item_legality_or_value_authority`

## Remaining Implementation Blockers / Required ADRs

The Character Attributes GDD-listed prerequisite ADR/technical-design set is now authored. Note: all ADRs are still `Proposed` until an independent `/architecture-review` and acceptance pass.

Architecture review follow-up status on 2026-06-05:

- ✅ ADR-0016 OpenMir2 Evidence Mapping Registry and Provisional Contract Governance authored.
- ✅ ADR-0017 Drop Table, Ground Drop, and Pickup Lifecycle Boundary authored.
- ✅ ADR-0018 Inventory and Equipment Instance Modifier Transaction Boundary authored.
- ✅ ADR-0019 Map Distance Facts and Movement Legality Boundary authored.
- ✅ ADR-0020 Resource / Map Conversion Pipeline and Validation Boundary authored.

The coherent architecture-review follow-up ADR set is now authored. Independent TD/Godot readiness review found no remaining engine blocker and identified two acceptance blockers, both fixed: ADR-0006 current-resource identity wording and ADR-0002/ADR-0019 movement unresolved reason alignment. ADR-0001 through ADR-0020 are now marked `Accepted`.

## Next Recommended CCGS Step

Item Definition GDD post-design validation is complete. Next recommended step in a fresh independent session:

```text
/design-review design/gdd/item-definition-system.md
```

Important review note remains: run `/architecture-review` only in a fresh Claude Code session after completing this coherent set of ADRs, not in the same authoring session.

Important review note:

Run `/architecture-review` only in a fresh Claude Code session after completing a coherent set of ADRs. Do not run `/architecture-review` in the same session as `/architecture-decision`, because the reviewing agent should be independent of the authoring context.


## Session Extract — /architecture-review 2026-06-05

- Verdict: FAIL
- Requirements: 83 total — 53 covered, 18 partial, 12 gaps
- New TR-IDs registered: 83
- GDD revision flags: map-coordinate-blocking-y-sort-system.md, character-attributes-system.md, item-definition-system.md
- Top ADR gaps: OpenMir2 Evidence Mapping Governance; Drop Table / Ground Drop / Pickup Lifecycle; Inventory / Equipment Instance and Modifier Transaction Boundary
- Report: docs/architecture/architecture-review-2026-06-05.md
- Traceability Index: docs/architecture/traceability-index.md
- TR Registry: docs/architecture/tr-registry.yaml


## Session Extract — /architecture-decision ADR-0016 2026-06-05

- ADR authored: `docs/architecture/adr-0016-openmir2-evidence-mapping-registry-and-provisional-contract-governance.md`
- Status: Proposed
- Purpose: resolves the highest-priority `/architecture-review` gap for OpenMir2 evidence governance, source tiers, E0-E4 readiness, Accepted contracts, intentional divergence, and false `openmir2_verified` prevention.
- Engine review: godot-specialist returned CONCERNS; incorporated YAML external-only rule, no GDScript assert gate, FileAccess bool/open-error checks, no networking/Autoload/runtime authority, typed result wrappers, and immutable-by-contract guidance.
- TD-ADR review: technical-director returned CONCERNS; incorporated canonical source vs normalized model, stable IDs/version/lifecycle, raw evidence vs Accepted contract distinction, and mvp_provisional non-blocking boundary.
- Registry updated: added ADR-0016 state ownership, interface contracts, API decisions, and forbidden patterns to `docs/registry/architecture.yaml`.
- User preference recorded: Godot version downgrade is acceptable if 4.6.3 compatibility becomes a blocker; default remains current pinned 4.6.3 until a concrete incompatibility triggers evaluation.
- Next recommended step: continue missing ADR sequence with Drop Table / Ground Drop / Pickup Lifecycle Boundary, then Inventory / Equipment Instance and Modifier Transaction Boundary.


## Session Extract — /architecture-decision ADR-0017 2026-06-05

- ADR authored: `docs/architecture/adr-0017-drop-table-ground-drop-and-pickup-lifecycle-boundary.md`
- Status: Proposed
- Purpose: resolves the second-priority `/architecture-review` gap for monster death/drop roll/ground drop/pickup request/inventory receive lifecycle boundary.
- Engine review: godot-specialist returned CONCERNS; incorporated GroundDropService vs MapSpaceState no-double-authority rule, staged pickup commit sequence, stable IDs, deterministic RNG, DTO defensive-copy, and scene/physics/signal non-authority guardrails.
- TD-ADR review: technical-director returned CONCERNS; incorporated no-half-commit invariant, inventory staging boundary, ground drop payload vs final inventory instance truth, placement failure policy, despawn vs pickup ordering, and ADR-0016 readiness/provisional labeling.
- Registry updated: added ADR-0017 drop table, ground drop, pickup state ownership, interface contracts, API decisions, and forbidden patterns to `docs/registry/architecture.yaml`.
- Next recommended step: continue missing ADR sequence with Inventory and Equipment Instance/Modifier Transaction Boundary.


## Session Extract — /architecture-decision ADR-0018 2026-06-05

- ADR authored: `docs/architecture/adr-0018-inventory-and-equipment-instance-modifier-transaction-boundary.md`
- Status: Proposed
- Purpose: resolves the third-priority `/architecture-review` gap for Inventory / Equipment instance identity, stack/container state, equip legality, modifier source resolution, Attribute transaction handoff, and pickup staged receive.
- Engine review: godot-specialist returned CONCERNS; incorporated three-domain commit ordering, equipment slot persistence owner, item_instance_id range/persistence safety, immutable DTO rules, and source key canonical form.
- TD-ADR review: technical-director returned CONCERNS; incorporated cross-domain equip transaction observability, item instance vs stack identity, full Inventory staged receive contract, persistence authority split, Attribute source mutation prohibition, and partial-commit validation criteria.
- Registry updated: added ADR-0018 Inventory/Equipment state ownership, interface contracts, API decisions, and forbidden patterns to `docs/registry/architecture.yaml`.
- Next recommended step: continue missing ADR sequence with Map Distance and Movement Legality Contract.

## Session Extract — Test Infrastructure and UX/Accessibility Baselines 2026-06-05

- Created test infrastructure directories: `tests/unit/` and `tests/integration/`.
- Created guard runner: `tests/gdunit4_runner.gd`.
- Created CI workflow: `.github/workflows/tests.yml`.
- Test runner policy: fail loudly until an approved GUT/GdUnit4 addon is installed and wired; never report a false zero-test success.
- Created UX interaction baseline: `design/ux/interaction-patterns.md`.
- Created accessibility baseline: `design/ux/accessibility-requirements.md`.
- Godot 4.6.3 UI constraints recorded: AccessKit-aware future screen reader path and dual-focus separation between mouse hover and keyboard/gamepad focus.
- Checklist items for test setup and UX/accessibility baselines are complete.
- Next recommended CCGS step: run independent architecture/gate review, then begin implementation planning for the first Phase 1 logic slice with blocking test evidence.

## Session Extract — Architecture Overview 2026-06-05

- Created `docs/architecture/architecture.md`.
- Purpose: concise implementation-planning overview linking Accepted ADR-0001 through ADR-0020, system ownership boundaries, Godot 4.6.3 constraints, test infrastructure baseline, UX/accessibility baseline, and remaining partial domains.
- This resolves the previous pre-gate concern that no architecture overview existed.
- Remaining blocker: guard test runner must be replaced/delegated to an approved GUT/GdUnit4 runner and real blocking tests must pass before implementation stories can claim Done.

## Session Extract — Stop Hook JSON Validation Fix 2026-06-05

- Updated `.claude/hooks/session-stop.sh` to emit minimal valid JSON `{}` on stdout after preserving session-log side effects.
- Verified hook exits `0` and stdout parses with `python3 -m json.tool`.
- Verified `.claude/settings.json` remains valid JSON.
- Rationale: this Claude Code build reported Stop hook JSON validation failure when the hook emitted no accepted JSON payload or an unsupported field; `{}` is the minimal safe accepted object.

## Session Extract — Drop Table System GDD 2026-06-05

- Created `design/gdd/drop-table-system.md` with the 8 required GDD sections.
- Status: `Designed — pending independent design review`.
- Purpose: defines Phase 1 Drop Table ownership for reward source to drop table mapping, roll groups, row weights, no-drop policy, quantity policy, deterministic roll results, validation, and QA replay provenance.
- Boundary: Drop Table emits item grant candidates only; it does not own Item Definition truth, ground placement, pickup lifecycle, inventory receive, equipment legality, UI/VFX/audio feedback, or final item instance identity.
- Updated `design/gdd/systems-index.md`: row 5 `掉落表系统` now points to `design/gdd/drop-table-system.md` and status is `Designed`; progress tracker now shows 5 MVP systems designed.
- Updated `design/registry/entities.yaml`: registered Drop Table formulas and added `design/gdd/drop-table-system.md` as a reference for `spawn_eligible_reference`.
- Next recommended CCGS step: independent design review for `design/gdd/drop-table-system.md`, then continue MVP system design with `点击移动系统` unless review finds blocking revisions.

## Session Extract — Drop Table Independent Design Review 2026-06-05

- Ran independent review of `design/gdd/drop-table-system.md` with economy-designer, systems-designer, and qa-lead.
- Initial verdicts: Economy Designer `NEEDS REVISION`; Systems Designer `NEEDS REVISION`; QA Lead `APPROVE`.
- Applied targeted fixes for all blocking findings:
  - registered provisional MVP item fixtures in `design/registry/entities.yaml`;
  - added expected acquisition outputs and material faucet calculation;
  - added economy health / faucet risk note;
  - clarified formula output ranges, variables, deterministic RNG inclusivity, multi-group result handling, fail-whole-table behavior, zero-weight row validation, and deterministic failure ordering;
  - clarified ADR-0016 Accepted evidence/contract readiness wording;
  - replaced vague player-loop acceptance criteria with numeric expected acquisition targets.
- Wrote review log: `design/gdd/reviews/drop-table-system-review-log.md`.
- Updated `design/gdd/drop-table-system.md` status to `Approved for ADR Authoring — implementation blocked pending Drop Table ADR / test runner`.
- Updated `design/gdd/systems-index.md` row 5 to `Approved`; progress tracker now shows 5 reviewed / 1 approved.
- Next recommended CCGS step: author Drop Table ADR, then continue MVP design order with `点击移动系统`.

## Session Extract — ADR-0021 Drop Table Runtime Boundary 2026-06-05

- Authored `docs/architecture/adr-0021-drop-table-runtime-data-validation-deterministic-roll-and-grant-candidate-handoff.md`.
- Status: Accepted.
- Purpose: implementation-blocking ADR for Drop Table runtime catalog, validation stages, deterministic weighted roll, quantity roll, result DTOs, and grant candidate handoff.
- Boundaries preserved: Drop Table validates/rolls/emits grant candidates only; it does not own Item Definition truth, GroundDrop lifecycle, MapSpaceState occupancy, Pickup, Inventory, Equipment, Save/Load, UI/VFX/audio, or item instances.
- Updated `docs/registry/architecture.yaml`: moved Drop Table catalog/roll/validation/RNG stances to ADR-0021 references and added forbidden patterns for silent invalid-row filtering, direct ground/inventory mutation, and inventory/placement-state probability mutation.
- Verified `docs/registry/architecture.yaml` parses as YAML.
- Next recommended CCGS step: independent architecture review for ADR-0021, then continue MVP system design with `点击移动系统`.

## Session Extract — ADR-0015 / ADR-0021 Traceability Refresh 2026-06-05

- Ran targeted independent architecture review focused on ADR-0015, ADR-0021, `docs/architecture/tr-registry.yaml`, `docs/architecture/traceability-index.md`, and `docs/registry/architecture.yaml`.
- Verdict before remediation: FAIL for traceability/status staleness, not for ADR ownership conflict.
- Confirmed ADR-0015 and ADR-0021 are coherent: Item Definition remains template truth; Drop Table owns validated roll policy and grant candidates only; GroundDrop/Pickup, Inventory/Equipment, and Attributes boundaries remain separate.
- Registered six Drop Table TR IDs in `docs/architecture/tr-registry.yaml` (`TR-drop-table-001` through `TR-drop-table-006`); registry version is now 3.
- Updated `docs/architecture/traceability-index.md` to 89 total requirements — 79 covered, 10 partial, 0 gaps — and added Drop Table matrix rows mapped to ADR-0021.
- Updated `TR-item-definition-017` coverage to ADR-0015, ADR-0017, ADR-0018, and ADR-0021.
- Updated GDD/status tracking: `design/gdd/item-definition-system.md`, `design/gdd/drop-table-system.md`, and `design/gdd/systems-index.md` no longer say implementation is pending the already Accepted ADRs.
- Updated `docs/registry/architecture.yaml` with Item Definition `referenced_by` hygiene and explicit `drop_grant_candidate_handoff_contract`.
- Updated stale ADR-0017 ordering note to reflect ADR-0016 and ADR-0018 Accepted status.
- Wrote report: `docs/architecture/architecture-review-adr-0015-0021-traceability-refresh-2026-06-05.md`.
- Verified edited YAML files parse and `git diff --check` reports no whitespace errors for edited review files.
- Remaining implementation blocker: real approved Godot GDScript test runner is still required before Item Definition or Drop Table stories can claim Done.
- Next recommended CCGS step: continue MVP design order with `点击移动系统`; before implementation Done, run test-runner setup.


## Session Extract — /architecture-review coherent ADR set 2026-06-05
- Verdict: CONCERNS before remediation; PASS for architecture-boundary coherence after remediation.
- Requirements: 99 total — 91 covered, 8 partial, 0 gaps.
- New TR-IDs registered: None.
- GDD revision flags: None from engine reality; stale Character Attributes status text was refreshed.
- Top ADR gaps: None at architecture-boundary level; remaining partials are content/implementation/test-evidence coverage.
- Report: docs/architecture/architecture-review-coherent-adr-set-2026-06-05.md
- Remediation: refreshed traceability-index stale Gap rows, updated architecture.yaml referenced_by for map/item/evidence stances, fixed Character Attributes and ADR-0014 stale status wording.
- Next recommended CCGS step: continue MVP design order with `交互目标 / 选择系统`; before implementation Done, install/wire real Godot GDScript test runner.


## Session Extract — Interaction Target / Selection GDD 2026-06-05
- Continued `/design-system 交互目标 / 选择系统` after coherent ADR-set cleanup.
- Existing file `design/gdd/interaction-target-selection-system.md` already contained Overview, Player Fantasy, Detailed Rules, and Formulas.
- Spawned `systems-designer` for Edge Cases review.
- Wrote Edge Cases with 18 explicit fail-closed/priority/tie-break/hover-selection cases.
- Current next section: Dependencies.


## Session Extract — Interaction Target / Selection Dependencies and Tuning 2026-06-05
- Wrote Dependencies section for `design/gdd/interaction-target-selection-system.md`, mapping upstream Input/UI Gate, MapProjection, MapSpaceState, Click Movement, candidate snapshots, and downstream Combat/Pickup/UI/Feedback/QA boundaries.
- Wrote Tuning Knobs section with data-driven near-hit radii, fallback toggles, selection clearing policy, hover throttle, tie-break evidence, invalid feedback cooldown, and non-color cue requirement.
- Current next section: Acceptance Criteria.


## Session Extract — Interaction Target / Selection Acceptance Criteria 2026-06-05
- Spawned `qa-lead` for Acceptance Criteria coverage.
- Wrote Acceptance Criteria section with 40 Given-When-Then criteria and evidence categories.
- Criteria cover UI/input gate, projection failure, fresh click query, hover-only behavior, candidate priority/tie-breaks, no silent fallback, stale selection, ground/Combat/Pickup handoff boundaries, typed result rules, tuning validation, immutable snapshots, accessibility cues, save/load runtime-only behavior, and QA smoke evidence.
- Current next section: Visual/Audio Requirements.
