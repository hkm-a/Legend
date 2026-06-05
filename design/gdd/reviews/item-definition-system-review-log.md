# 物品定义系统 — Design Review Log

## Review — 2026-06-04 — Verdict: MAJOR REVISION NEEDED

Scope signal: XL
Specialists: full design review synthesis, including systems, economy, QA, UI/UX, technical/Godot, gameplay, performance, audio/visual and senior creative/technical perspectives
Blocking items: major revision required across player-facing fixture readiness, formula semantics, UI/audio/accessibility handoff, AC evidence, performance/ADR boundaries, and traceability
Summary: Full review found the draft had a strong item identity and ownership boundary, but did not yet sufficiently guarantee a Phase 1 loot-loop item set that players can read as “this may make me stronger.” The review required a concrete MVP loot-loop fixture contract, richer player-facing display metadata, formula split between lookup/semantic validity/profile eligibility/spawn eligibility, clearer UI/audio/accessibility handoff, stronger acceptance criteria and test matrix, explicit normalized runtime truth, and implementation-blocking ADR follow-up.
Prior verdict resolved: First full review

### Blocking themes

1. MVP loot-loop fixture was underspecified: baseline, upgrade, sidegrade/weaker item and material shape needed explicit smoke coverage.
2. Player-facing metadata was too thin for loot labels, inventory cells, tooltips, comparison and fallback states.
3. Formula semantics needed separation between reference lookup, semantic validation, profile eligibility and spawn eligibility.
4. Modifier payload and stack validation needed clearer outputs, variables, safe ranges and examples.
5. UI, visual, audio and accessibility handoff needed data-side semantic metadata without giving Item Definition ownership of final presentation.
6. Runtime truth needed a normalized, indexed, read-only DTO/table boundary rather than mutable authoring Resources or copied downstream truth.
7. Acceptance criteria needed blocking evidence wording, explicit QA gates, and a minimum implementation test matrix.
8. Implementation should remain blocked until a dedicated Item Definition ADR resolves runtime data representation, validation result schema, authoring/loading, version/migration and Resource policy.

### Revision written after review

A targeted major revision was written on 2026-06-04. It adds:

- Phase 1 player-facing metadata requirements for item labels, type/quality labels, icon/world visual/audio tokens, salience and accessibility semantics.
- MVP provisional loot-loop fixture requirements covering baseline equipment/reference, clear upgrade, sidegrade/weaker equipment, stackable material and optional rare showcase.
- Split formulas for `item_reference_lookup_resolvable`, `item_definition_semantically_valid`, `definition_profile_eligible`, and `spawn_eligible_reference`.
- Expanded stack and modifier formulas with result rows, variables, output ranges and examples.
- Deterministic validator-bound note for safe integer limits, supported definition versions, modifier safe ranges and runtime profile gates.
- Runtime normalized/indexed/read-only truth contract and query/projection/fallback contract for UI and downstream systems.
- Dedicated UI/visual/audio/accessibility appendices that preserve ownership boundaries.
- AC-01 through AC-25 plus a minimum blocking test matrix.
- Registry synchronization for all Item Definition cross-system formulas.
- Systems index status update to `Designed`.

### Next step

Run `/design-review design/gdd/item-definition-system.md` in a fresh Claude Code session for independent review. If accepted or only targeted fixes remain, author the implementation-blocking Item Definition ADR before creating implementation stories.

## Review — 2026-06-04 — Verdict: DESIGNED / READY FOR FRESH REVIEW

Scope signal: XL
Specialists: post-design validation agents — systems-designer, economy-designer, technical-director
Blocking items: 0 known GDD-content blockers after targeted cleanup; ADR remains required before implementation stories are Ready
Summary: Post-design validation found the revised document has healthy ownership boundaries and strong enough item/economy/runtime separation for the next independent design review. Targeted cleanup renamed overloaded formula concepts, added output/result rows, made validator bounds deterministic, moved appendices after the required eight GDD sections, clarified multi-version edge cases, and registered formulas.
Prior verdict resolved: Yes — known major-review blockers have been addressed in the GDD draft; independent `/design-review` still required for approval.

### Targeted fixes written after post-design validation

- Renamed `item_reference_resolvable` to `item_reference_lookup_resolvable` to clarify lookup is not spawn/equip eligibility.
- Renamed overloaded semantic validity references to `item_definition_semantically_valid`.
- Renamed spawn-facing check to `spawn_eligible_reference`.
- Added explicit result rows to item formula variable tables.
- Added deterministic validation-bound policy when configured safe ranges, supported versions or profile gates are absent.
- Changed presentation metadata from `pickup_priority` to `pickup_visual_salience` to avoid gameplay pickup-priority authority confusion.
- Added multi-version `definition_version` edge case and migration/version-policy notes.
- Confirmed appendices follow Acceptance Criteria so required GDD section order remains intact.
- Registered `stack_quantity_valid`, `modifier_payload_row_valid`, `modifier_payload_valid`, `item_reference_lookup_resolvable`, `item_definition_semantically_valid`, `definition_profile_eligible`, and `spawn_eligible_reference` in `design/registry/entities.yaml`.

### Next step

Fresh-session command:

```text
/design-review design/gdd/item-definition-system.md
```

## Review — 2026-06-04 — Verdict: APPROVED FOR ADR AUTHORING

Scope signal: XL
Specialists: independent fresh-session review panel — economy-designer, systems-designer, technical-director
Blocking items: 0 remaining GDD approval blockers after targeted Open Questions fix; implementation remains blocked until a dedicated Item Definition ADR is accepted.
Summary: Independent review found the revised Item Definition GDD has strong ownership boundaries, sufficient Phase 1 loot-loop fixture requirements, validated formula separation, normalized runtime truth direction, OpenMir2 evidence gating, and downstream handoff contracts for Drop Table, Drop/Pickup, Inventory, Equipment, Character Attributes, UI, Save/Load and future economy. One targeted blocker was found: the Open Questions row for the MVP first item set still said “1 normal equipment + 1 material,” weaker than the Detailed Rules and AC-23 baseline/upgrade/sidegrade/material requirement. That row has been corrected to match the binding fixture contract.
Prior verdict resolved: Yes — the major revision blockers and the targeted fresh-review fixture inconsistency are resolved for GDD approval.

### Fresh review findings

- Economy review verdict: `APPROVE WITH CONCERNS`; no GDD approval blocker, but implementation/content readiness requires item ADR and concrete MVP item candidates later.
- Systems review verdict: `NEEDS TARGETED REVISION`; blocking item BF-01 was the MVP fixture inconsistency in Open Questions.
- Technical review verdict: `CONCERNS`; GDD is suitable for ADR authoring, but not implementation-ready without a dedicated Item Definition ADR and accepted dependency ADRs where relevant.

### Targeted fix written after fresh review

- Updated Open Questions MVP item-set assumption to require: 1 baseline equipped item/reference, 1 same-category clear upgrade equipment, 1 same-category sidegrade or weaker equipment, 1 stackable material, and optional rare/showcase item.
- Updated GDD status to `Approved for ADR Authoring — implementation blocked pending Item Definition ADR`.
- Updated Systems Index row 4 to `Approved` and progress counts to 4 reviewed / 3 approved.
- Updated `design/registry/entities.yaml` `last_updated` metadata to `2026-06-04`.

### Required follow-up before implementation stories

- Author and review a dedicated Item Definition ADR covering runtime data representation, query/projection result schema, validation result schema, version/migration boundary, authoring/config loading pipeline, Resource/.tres policy application, DTO read-only/defensive-copy rules, indexed lookup, and test requirements.
- Register concrete MVP item candidates only after first loot-loop fixture naming is approved; current example IDs remain non-canonical.
- Keep OpenMir2-authentic item, slot, stat, and instance claims blocked until E3/E4 evidence exists.
