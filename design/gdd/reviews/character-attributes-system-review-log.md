# 角色属性系统 — Design Review Log

## Review — 2026-06-04 — Verdict: MAJOR REVISION NEEDED

Scope signal: XL
Specialists: game-designer, systems-designer, qa-lead, ux-designer, ui-programmer, godot-specialist, gameplay-programmer, performance-analyst, creative-director
Blocking items: 22 | Recommended: 10+
Summary: Full review found that the draft was strong as a backend/stat-authority contract but did not yet guarantee the core player fantasy of “我穿了，我涨了，我打得更快了.” Senior review required a Phase 1 player-facing growth contract, combat power/main stat comparison, visible/hidden/reserved stat separation, clearer validation/clamp semantics, compact event payloads, implementation-blocking ADRs, and rewritten acceptance criteria.
Prior verdict resolved: First review

### Blocking themes

1. Missing player-facing growth contract: no `combat_power`, `main_stat`, `growth_salience`, or equipment comparison summary.
2. Too many inactive/reserved stats risked polluting quick equipment value judgment.
3. Max HP/MP increase policy could make upgrades appear visually negative.
4. Validation and clamp semantics were ambiguous: raw invalid input vs corrected output vs config failure.
5. Actor-type required stat sets were unclear for player vs ordinary monster snapshots.
6. Invalid modifier policy was left as a tuning knob instead of a Phase 1 default.
7. Event payloads were too loose and potentially too heavy for UI/performance.
8. Read-only snapshot, stat ID typing, event/signal contract, fixture/config loading, and save/load boundary required ADRs before implementation.
9. Acceptance criteria mixed unit and downstream integration gates and were not independently testable enough.
10. Data/config fixture ownership was unresolved.

### Revision written after review

A revised draft was written on 2026-06-04. It adds:

- Player-facing growth handoff rule.
- Phase 1 stat registry with actor type, visibility, combat activity, modifier targetability, and source status.
- Provisional `combat_power` display formula.
- Equipment preview boundary.
- Structural rebuild vs resource mutation split.
- Atomic source update rule.
- Four-stage validation pipeline.
- Default invalid modifier policy.
- Snapshot/version/query contract.
- Compact event categories.
- UI/presentation handoff and localization metadata requirements.
- Save/load persistent input contract.
- Implementation-blocking ADR gate list.
- Performance rules.
- Revised formulas and ACs.

### Next step

Run `/design-review design/gdd/character-attributes-system.md` again. Recommended depth: full, because the revision materially changed player-facing growth, validation, events, and AC staging.

## Review — 2026-06-04 — Verdict: NEEDS REVISION

Scope signal: XL
Specialists: game-designer, systems-designer, qa-lead, ux-designer, ui-programmer, godot-specialist, gameplay-programmer, performance-analyst, creative-director
Blocking items: 8 targeted cleanup items | Recommended: 5+
Summary: Full re-review found the revised GDD had resolved the major experiential, UI, performance, Godot, and implementation-contract blockers, but systems/QA review still required targeted formula and registry cleanup before approval. Senior review classified this as targeted NEEDS REVISION rather than major revision: add formula examples, fully specify `combat_power`, synchronize registry formulas, add nested failure propagation, clarify resource correction policy and `snapshot_delta` semantics, tighten AC evidence wording, then approve without another full specialist re-review.
Prior verdict resolved: Partially — prior MAJOR REVISION was reduced to targeted NEEDS REVISION.

### Targeted fixes written after re-review

- Added formula-wide conventions for structured failure propagation, safe numeric ranges, overflow behavior, and display-only rounding.
- Added output ranges and worked examples for `effective_stat`, `effective_stat_pair`, `current_resource_after`, `snapshot_valid`, `attribute_delta`, `snapshot_delta`, and `combat_power`.
- Fully specified `combat_power`: config validity, child formula preconditions, `secondary_contribution`, output type, rounding, all-zero-weight failure, inactive stat behavior, and examples.
- Added `config_version` to `attribute_delta` comparability.
- Added resource correction policy by reason for damage/heal/spend/restore/max-resource/load/spawn/debug changes.
- Clarified `snapshot_delta` requested-set, visible summary, and debug-full semantics.
- Updated AC-04, AC-07B, AC-10, AC-11, AC-12, and AC-17 evidence wording.
- Synchronized `design/registry/entities.yaml` formulas, including adding `combat_power`.

## Review — 2026-06-04 — Verdict: APPROVED

Scope signal: XL
Specialists: focused senior synthesis after full re-review; prior specialist findings from game-designer, systems-designer, qa-lead, ux-designer, ui-programmer, godot-specialist, gameplay-programmer, performance-analyst, creative-director
Blocking items: 0 | Recommended: ADRs required before implementation
Summary: Targeted formula, registry, and AC evidence blockers from the re-review have been addressed. The GDD is approved as a foundation gameplay contract and is ready for ADR / technical design authoring; implementation remains blocked until the listed Character Attributes ADRs or technical designs are accepted.
Prior verdict resolved: Yes — MAJOR REVISION and targeted NEEDS REVISION items resolved.
