# Architecture Review — Coherent ADR Set Synchronization

Date: 2026-06-05
Engine: Godot 4.6.3
Scope: ADR-0015, ADR-0021, related boundary ADRs, TR registry, traceability index, and architecture registry synchronization.

## Verdict

**CONCERNS before remediation; PASS for architecture-boundary coherence after remediation.**

The independent architecture and Godot reviews found no blocking ownership conflict between Item Definition, Drop Table, GroundDrop/Pickup, Inventory/Equipment, Character Attributes, or Map/Movement boundaries. The concern was synchronization quality: `traceability-index.md` still showed stale `❌ Gap` rows while its summary claimed zero gaps, Character Attributes status text still referenced pending ADRs after ADR-0006 through ADR-0014 were Accepted, and selected `docs/registry/architecture.yaml` `referenced_by` lists did not fully reflect downstream ADR dependencies.

## Requirements Summary After Remediation

- Total requirements: 99
- Covered: 91
- Partial: 8
- Gaps: 0
- New TR IDs registered: None
- TR IDs renumbered: None

## Independent Review Inputs

- Technical Director review verdict: `CONCERNS` due to traceability/status synchronization, not due to ADR ownership conflict.
- Godot specialist review verdict: `CONCERNS`; no Godot 4.6.3 incompatibility blocker, with implementation guards for immutable-by-contract DTOs and real GUT/GdUnit4 runner evidence.

## Cross-ADR Coherence Findings

No blocking ownership conflicts were found.

- ADR-0015 preserves Item Definition as template truth: stable `item_id`, template metadata, lifecycle/evidence labels, stack policy, equipment candidate declarations, modifier payload declarations, validation, query/projection DTOs, and version/migration boundary.
- ADR-0021 limits Drop Table to validated normalized drop table definitions, deterministic roll policy, quantity policy, roll results, and `DropGrantCandidate` handoff data.
- ADR-0017 owns GroundDrop/Pickup lifecycle, placement handoff, pickup claim state, and staged pickup commit orchestration; it does not own inventory truth or item template truth.
- ADR-0018 owns Inventory/Equipment container, item instance, equip transaction, and modifier source handoff boundaries; it consumes Item Definition and Character Attributes contracts without copying their authority.
- ADR-0006 through ADR-0014 own Character Attributes stat identity, snapshots, events, transaction, save/load, fixture config, formula test strategy, Resource policy, and display proxy handoff.
- ADR-0001 through ADR-0005 plus ADR-0019 and ADR-0022 preserve MapDefinition, MapSpaceState, projection, distance, movement legality, and click movement boundaries.

## Remediation Applied

- Updated `docs/architecture/traceability-index.md` so stale `❌ Gap` rows are now reclassified to the accepted ADRs that cover them.
- Recomputed traceability summary to 99 total / 91 covered / 8 partial / 0 gaps.
- Preserved all stable TR IDs; no registry renumbering or deletion was performed.
- Updated `docs/registry/architecture.yaml` `state_ownership.referenced_by` for:
  - `runtime_map_space_state`
  - `item_definition_template_truth`
  - `openmir2_behavior_evidence_registry`
- Updated `design/gdd/character-attributes-system.md` status language to reflect that ADR-0006 through ADR-0014 are Accepted and implementation Done is now blocked by real Godot test runner/story evidence rather than pending ADRs.
- Updated ADR-0014 ordering note to remove stale `Proposed` wording.

## Remaining Partial Coverage

The remaining partial rows are not hard architecture gaps; they are content, implementation, or future-system coverage that cannot be fully closed by the current ADR set alone:

1. `TR-concept-001` — the full 30-second loop is only partially complete until combat, pickup, inventory, equipment, HUD/feedback, and save stories are designed/implemented.
2. `TR-concept-006` — reward feedback remains partial until presentation/audio/UI GDDs and evidence exist.
3. `TR-concept-007` — backpack/equipment comparison remains partial until Inventory, Equipment, and UI GDDs/stories exist.
4. `TR-systems-index-004` — MVP priority system coverage remains partial because several systems are still Not Started.
5. `TR-openmir2-spike-001` — ADR-0016 governs evidence mapping, but accepted evidence-backed contracts are not complete for every listed gameplay domain.
6. `TR-openmir2-spike-007` — the contract registry model exists, but all required domain contracts are not yet accepted artifacts.
7. `TR-item-definition-016` — concrete MVP item catalog/content remains pending.
8. `TR-item-definition-021` — UI/accessibility projection details remain pending UX/UI implementation evidence.

## Engine Compatibility Findings

No Godot 4.6.3 API or node-pattern blocker was found.

Guardrails that must flow into implementation stories:

1. **DTO immutability guard** — conceptual snippets with public `var` fields must not be copied as published runtime/query/result DTOs. Implementations must use validating construction, getter-only API shape, and defensive-copy collection rules unless the object is explicitly tooling-only mutable input/builder data.
2. **Test runner guard** — implementation Done for Character Attributes, Item Definition, Drop Table, Inventory/Equipment, and related systems requires a real approved Godot GDScript test runner. Guard-only runner output is not passing evidence.

## Final Assessment

The coherent ADR set is fit to continue CCGS design flow. Architecture-boundary coherence is PASS after synchronization cleanup, with implementation readiness still guarded by real test-runner setup and story-level evidence.

## Recommended Next Actions

1. Continue MVP design order with `交互目标 / 选择系统`.
2. Before implementation Done claims, install/wire the real Godot GDScript test runner and prove one smoke/unit test executes under Godot 4.6.3 headless.
3. Keep refreshing traceability after each approved GDD/ADR set; do not allow matrix row status and summary counts to diverge.
