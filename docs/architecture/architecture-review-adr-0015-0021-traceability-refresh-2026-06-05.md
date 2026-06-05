# Architecture Review — ADR-0015 / ADR-0021 Traceability Refresh

Date: 2026-06-05
Engine: Godot 4.6.3
Scope: Targeted refresh for Item Definition and Drop Table architecture tracking after ADR-0021 acceptance.

## Verdict

**FAIL before remediation; CONCERNS after remediation.**

The failure was not an ADR ownership or engine-compatibility conflict. ADR-0015 and ADR-0021 are coherent and implementable. The blocking issue was traceability/status staleness: the approved Drop Table GDD and Accepted ADR-0021 had not been represented in `docs/architecture/tr-registry.yaml` or the full matrix in `docs/architecture/traceability-index.md`, which made the previous `0 gaps` summary a false negative for story readiness.

## Requirements Summary After Refresh

- Total requirements: 89
- Covered: 79
- Partial: 10
- Gaps: 0
- New TR IDs registered: 6 (`TR-drop-table-001` through `TR-drop-table-006`)

## Blocking Issues Found

1. **Drop Table TR IDs missing.**
   - `design/gdd/drop-table-system.md` was approved and ADR-0021 was Accepted, but no `TR-drop-table-*` entries existed.
   - Impact: story creation/readiness could not reference stable Drop Table requirements.

2. **Traceability index omitted Drop Table.**
   - The matrix ended at Item Definition and explicitly noted that it needed regeneration.
   - Impact: the global `0 gaps` summary was not reliable for pre-production gating.

3. **GDD status text was stale.**
   - Item Definition still said implementation was blocked pending Item Definition ADR, despite ADR-0015 being Accepted.
   - Drop Table still said implementation was blocked pending Drop Table ADR / test runner, despite ADR-0021 being Accepted.

4. **ADR-0017 ordering note was stale.**
   - It still described ADR-0016 as Proposed and Inventory/Equipment ADR as future, despite ADR-0016 and ADR-0018 now being Accepted.

## Remediation Applied

- Updated `docs/architecture/tr-registry.yaml` from version 2 to version 3 and appended six stable Drop Table TR IDs.
- Updated `docs/architecture/traceability-index.md` summary to 89 total / 79 covered / 10 partial / 0 gaps.
- Added Drop Table matrix rows covering ADR-0021.
- Updated `TR-item-definition-017` coverage to include ADR-0015, ADR-0017, ADR-0018, and ADR-0021.
- Updated `design/gdd/item-definition-system.md` status to implementation planning blocked by real test runner and story/test slicing.
- Updated `design/gdd/drop-table-system.md` status and dependency references to include ADR-0016, ADR-0017, and ADR-0021.
- Updated `design/gdd/systems-index.md` Item Definition row to `Approved` and progress tracker approved count to 2.
- Updated `docs/registry/architecture.yaml` referenced_by hygiene for Item Definition contracts.
- Added explicit `drop_grant_candidate_handoff_contract` registry stance.
- Updated ADR-0017 ordering note to reflect ADR-0016 and ADR-0018 Accepted status.

## Cross-ADR Coherence Findings

No blocking ownership conflicts were found.

- Item Definition remains template truth under ADR-0015.
- Drop Table owns validated roll policy and grant candidates only under ADR-0021.
- GroundDrop/Pickup own ground lifecycle and pickup commit under ADR-0017.
- Inventory/Equipment own staged receive, storage/equipment state, and modifier source handoff under ADR-0018.
- Character Attributes owns final attribute aggregation under ADR-0006 through ADR-0014.

## Engine Specialist Findings

Godot 4.6.3 compatibility is acceptable for ADR-0015 and ADR-0021. Both avoid post-cutoff Godot APIs as gameplay authority and use scene-tree-independent DTO/service boundaries.

Remaining blocker: the project still lacks a real approved Godot GDScript test runner. `tests/gdunit4_runner.gd` remains a guard runner and cannot serve as passing evidence. Implementation stories for Item Definition or Drop Table cannot claim Done until a real GUT/GdUnit4 runner is installed, wired, and running tests.

## Remaining Concerns

1. Real Godot test runner is still required before implementation Done.
2. `TR-item-definition-016` remains Partial until concrete MVP item catalog/content exists.
3. `TR-item-definition-021` remains Partial until UI/accessibility projection details are validated by UX/UI specs and implementation evidence.
4. OpenMir2 behavior contracts remain partial in broader project scope; any source-authentic drop/item claims must follow ADR-0016 readiness.

## Recommended Next Actions

1. Install/wire the approved Godot GDScript test runner and prove one smoke test executes.
2. Continue MVP design order with `点击移动系统`.
3. When story creation begins, reference the newly registered `TR-drop-table-*` IDs for Drop Table stories.
