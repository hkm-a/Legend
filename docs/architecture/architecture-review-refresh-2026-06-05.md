# Architecture Review Refresh — 2026-06-05

Date: 2026-06-05
Engine: Godot 4.6.3
Mode: read-only refresh over Accepted ADR-0001 through ADR-0020
Baseline: `docs/architecture/architecture-review-2026-06-05.md` and `docs/architecture/traceability-index.md`

---

## 1. Loaded Counts

Loaded inputs:

- GDD top-level Markdown files: 6
  - 5 system/concept GDDs
  - 1 `design/gdd/systems-index.md`
- GDD review logs found: 3
- ADRs reviewed: 20
  - `docs/architecture/adr-0001*.md` through `docs/architecture/adr-0020*.md`
  - All 20 are currently `Accepted`
- Architecture registries reviewed: 2
  - `docs/architecture/tr-registry.yaml`
  - `docs/registry/architecture.yaml`
- Engine reference files reviewed/listed: 12
  - `VERSION.md`
  - `current-best-practices.md`
  - `breaking-changes.md`
  - `deprecated-apis.md`
  - 8 module files under `docs/engine-reference/godot/modules/`
- Previous baseline documents reviewed: 2
  - `docs/architecture/architecture-review-2026-06-05.md`
  - `docs/architecture/traceability-index.md`

---

## 2. Updated Coverage Summary

Total requirements remain: **83**

Updated coverage after ADR-0016 through ADR-0020 are Accepted:

| Status | Previous Count | Refreshed Count | Change |
|---|---:|---:|---:|
| Covered | 53 | 73 | +20 |
| Partial | 18 | 10 | -8 |
| Gaps | 12 | 0 | -12 |

### Summary

The previous review's 12 hard gaps are no longer hard architecture gaps. ADR-0016 through ADR-0020 now cover the missing governance and boundary decisions for:

- OpenMir2 evidence governance
- Drop table / ground drop / pickup lifecycle
- Inventory / equipment transaction boundary
- Map distance and movement legality
- Resource / map conversion pipeline

However, this does **not** mean implementation is ready across the whole Phase 1 slice. Several requirements remain **Partial** because the ADRs define architecture boundaries, but actual downstream contracts, concrete GDD revisions, implementation stories, tests, UX/accessibility specs, or runtime data artifacts are still absent.

### Reclassification of the Previous 12 Gaps

| Requirement ID | Previous | Refreshed | Rationale |
|---|---|---|---|
| `TR-concept-003` | Gap | Covered | ADR-0016 establishes source-first OpenMir2 evidence governance and blocks MinimalMirClient from being authoritative. |
| `TR-concept-005` | Gap | Covered | ADR-0016 keeps protocol/networking as evidence/governance/future-spike scope and explicitly avoids runtime networking authority in Phase 1. |
| `TR-systems-index-005` | Gap | Covered | ADR-0017 defines death/drop roll/ground drop/pickup/inventory receive staging boundary. |
| `TR-systems-index-008` | Gap | Covered | ADR-0020 defines resource/map conversion pipeline and validation boundary while allowing provisional Phase 1 maps. |
| `TR-openmir2-spike-001` | Gap | Partial | ADR-0016 defines governance for required mapping coverage, but the actual OpenMir2 behavior contracts for all listed domains are not yet produced/accepted. |
| `TR-openmir2-spike-002` | Gap | Covered | ADR-0016 defines source tiers and preserves OpenMir2 source as Tier 1 authority. |
| `TR-openmir2-spike-003` | Gap | Covered | ADR-0016 defines E3/E4 readiness gates before source-authentic downstream claims. |
| `TR-openmir2-spike-004` | Gap | Covered | ADR-0016 defines evidence/contract/divergence record schema and structured readiness. |
| `TR-openmir2-spike-005` | Gap | Covered | ADR-0016 defines Adopt/Simplify/Exclude/Defer and provisional contract lifecycle. |
| `TR-openmir2-spike-007` | Gap | Partial | ADR-0016 defines the contract registry model, but the actual Map Coordinate, Blocking, Movement, Attack, Damage, Death, Spawn, Drop, Ground Item, Pickup, Inventory, Equipment, and Protocol contracts are not all present as accepted evidence-backed artifacts. |
| `TR-openmir2-spike-008` | Gap | Covered | ADR-0016 defines intentional divergence record requirements. |
| `TR-map-space-016` | Gap | Covered | ADR-0019 defines canonical distance facts, movement legality, diagonal/corner-cutting fail-closed behavior, and evidence-gated policy profiles. |

### Remaining Partial Coverage Themes

The remaining 10 Partial requirements are architecture-covered enough to proceed with careful story slicing, but not complete enough for a clean pre-production gate:

1. Full 30-second loot loop is still Partial because combat, death, spawn, HUD, feedback, and save/load are not all fully designed/implemented.
2. Loot feedback/audio/visual valuation remains Partial; ADRs define data boundaries, not presentation feel.
3. UI-only-read boundaries are directionally covered, but UX specs and interaction/accessibility requirements are missing.
4. Systems-index-wide layering is mostly covered by ADRs and registry stances, but several MVP systems remain Not Started.
5. Actual OpenMir2 behavior contracts remain Partial despite governance being accepted.
6. Spawn/drop/pickup map legality is improved, but spawn-specific rules remain downstream.
7. MVP concrete item set remains Partial; item definition architecture exists, but actual catalog/content is not present.
8. Accessibility metadata remains Partial because required UX/accessibility docs are missing.

---

## 3. Cross-ADR Conflicts

Verdict for conflict phase: **CONCERNS**

### Hard Conflicts

No hard mutual-exclusion conflict was found across ADR-0001 through ADR-0020.

The accepted ADR set is broadly coherent:

- Static map facts remain under `MapDefinition` / ADR-0001.
- Runtime occupancy and reservation mutation remain under `MapSpaceState` / ADR-0003.
- Query/result DTO discipline is consistent with ADR-0002 and extended into later systems.
- Character Attributes remains the authority for stat aggregation, snapshots, current-resource mutation, preview, and display proxy outputs.
- Item Definition remains template truth.
- Drop/Pickup, Inventory, Equipment, Movement, and Map Conversion each define bounded ownership rather than stealing upstream authority.
- OpenMir2 evidence governance remains offline/tooling/bootstrap, not hot gameplay authority.

### Concerns

1. **OpenMir2 governance exists, but actual contracts are still missing.**
   ADR-0016 solves the governance gap, but `TR-openmir2-spike-001` and `TR-openmir2-spike-007` remain Partial until actual Accepted contracts exist for movement, attack, damage, death, spawn, drop, pickup, inventory, equipment, and protocol.

2. **Combat/life/death/spawn remain architecture-thin.**
   ADR-0017 depends on death/reward source context but does not own combat death semantics. Future Combat, Damage, Life/Death, and Spawn ADRs/GDDs must avoid backfilling assumptions that conflict with ADR-0016 and ADR-0019.

3. **Save/Load remains boundary-scattered.**
   ADR-0010 covers Attribute persistence, ADR-0015 covers item definition versioning, and ADR-0018 covers inventory/equipment save boundaries, but there is not yet a single Save/Load system ADR covering slot schema, write atomicity, migration orchestration, and cross-system load ordering.

4. **Presentation and UX are only baseline-specified.**
   ADR-0014 and ADR-0015 provide display/projection boundaries, and baseline interaction/accessibility documents now exist, but HUD, inventory/equipment UI, loot feedback, and screen-level UX specs are still not fully formalized.

5. **Implementation story slicing must not treat Accepted ADRs as completed systems.**
   ADRs now cover architectural feasibility, but tests, fixtures, concrete catalogs, OpenMir2 evidence artifacts, and actual service implementations are still absent.

---

## 4. Engine Compatibility

Verdict for engine compatibility: **No blocker found**

Godot 4.6.3 remains high knowledge-risk because it is post-cutoff, but the ADR set largely mitigates that risk by avoiding fragile runtime dependence on post-cutoff APIs for core gameplay authority.

### Positive Findings

- Core gameplay authorities are mostly scene-tree-independent, injectable service/DTO models.
- ADRs avoid making `TileMapLayer`, physics, navigation, signals, Autoloads, Resources, or scene node identity authoritative for gameplay truth.
- ADR-0020 explicitly checks Godot 4.4+ `FileAccess.store_*` boolean write-result behavior.
- ADR-0020 uses `TileMapLayer` only as optional visual/editor output, not gameplay authority.
- ADR-0019 keeps Godot Navigation/Physics as non-authoritative helpers only.
- Resource mutability/shared-cache risk is repeatedly constrained by ADR-0013, ADR-0015, and ADR-0020.

### Engine Concerns to Carry Forward

- DTO sketches using public `var` fields must be treated as conceptual unless implementation enforces immutable-by-contract access and defensive copies.
- GDScript enum/int typing details must be verified against Godot 4.6.3 during implementation.
- Optional signal adapters must use modern typed signal/Callable patterns and must not become gameplay authority.
- GUT compatibility with Godot 4.6.3 still needs concrete test setup verification.
- File/Resource/Scene save operations in tooling must produce structured failures on failed writes, packs, saves, or temp-file replacements.

---

## 5. GDD Revision Flags

Verdict: **Needs Revision still present**

`design/gdd/systems-index.md` still marks the following MVP foundation GDDs as `Needs Revision`:

- `design/gdd/map-coordinate-blocking-y-sort-system.md`
- `design/gdd/character-attributes-system.md`
- `design/gdd/item-definition-system.md`

These flags should remain until the GDDs are refreshed against the now-Accepted ADR-0016 through ADR-0020 set.

Expected revision themes:

1. Update OpenMir2 evidence references to match ADR-0016 governance.
2. Reflect ADR-0017 drop/pickup lifecycle and MapSpaceState command boundaries.
3. Reflect ADR-0018 inventory/equipment transaction, preview, and save boundary.
4. Reflect ADR-0019 distance/movement legality profile and fail-closed diagonal/corner-cutting behavior.
5. Reflect ADR-0020 map conversion/report validation boundary.
6. Tighten immutable-by-contract DTO language where GDDs still imply raw mutable structures.
7. Keep `mvp_provisional` labels wherever actual OpenMir2 contracts are not yet accepted.

---

## 6. Verdict

## Verdict: CONCERNS

Reason: the architecture has materially improved and the previous 12 hard gaps are resolved at ADR-boundary level, but the project is not cleanly gate-ready.

### Why Not FAIL

- ADR-0001 through ADR-0020 are Accepted.
- No hard cross-ADR contradiction was found.
- No Godot 4.6.3 compatibility blocker was found.
- Previous hard gaps now have accepted architecture decisions.
- The authority model is coherent: map, attributes, items, drops, pickup, inventory, equipment, movement, OpenMir2 evidence, and map conversion have separable ownership.

### Why Not PASS

- 10 requirements remain Partial.
- Actual OpenMir2 behavior contracts are still missing for several required domains.
- Core combat/death/spawn/save/presentation systems remain architecture-thin or not yet started.
- `Needs Revision` flags still exist in `systems-index.md`.
- Pre-gate test and UX/accessibility baseline files now exist, but the test runner is a failing guard until the approved GUT/GdUnit4 addon is installed and real tests execute.
- There is no evidence yet that the accepted ADRs have corresponding passing unit/integration tests or fixture catalogs.

---

## 7. Pre-Gate Checklist

Read-only existence check:

| Required Path | Status |
|---|---|
| `tests/unit` | Present — directory baseline created |
| `tests/integration` | Present — directory baseline created |
| `.github/workflows/tests.yml` | Present — Godot 4.6.3 headless guard workflow created |
| `design/accessibility-requirements.md` | Not used — project UX docs standard places this under `design/ux/` |
| `design/ux/interaction-patterns.md` | Present — baseline created |
| `design/ux/accessibility-requirements.md` | Present — baseline created |

Additional note:

- `tests/gdunit4_runner.gd` is intentionally a failing guard until the approved GUT/GdUnit4 addon is installed and wired to execute real tests.

Pre-gate implication: do **not** treat this architecture refresh as a pre-production gate pass until the guard runner is replaced/delegated to a real approved test framework and the first blocking unit/integration evidence passes.

---

## 8. Required Next Actions — Top 5

1. **Produce actual OpenMir2 accepted contract artifacts.**
   Prioritize Movement, Attack, Damage, Death, Spawn, Drop, Ground Item, Pickup, Inventory, Equipment, and Protocol contract records under ADR-0016 governance. This is the main reason `TR-openmir2-spike-001` and `TR-openmir2-spike-007` remain Partial.

2. **Revise the three `Needs Revision` GDDs.**
   Update map-space, character-attributes, and item-definition GDDs to reflect ADR-0016 through ADR-0020 and clear stale architecture-review concerns.

3. **Create the test infrastructure baseline.**
   Add `tests/unit`, `tests/integration`, and `.github/workflows/tests.yml` before any gate that requires automated evidence. Attribute, item definition, drop/pickup, inventory/equipment, and movement legality stories should not proceed without test locations and runner expectations.

4. **Author missing UX/accessibility baseline docs.**
   Create interaction patterns and accessibility requirements before HUD, inventory/equipment UI, loot feedback, or item projection UI work proceeds. Resolve whether the canonical accessibility path is `design/accessibility-requirements.md`, `design/ux/accessibility-requirements.md`, or both via index/redirect.

5. **Plan the next ADR/GDD batch for remaining MVP gaps.**
   Recommended order:
   - Combat / Damage Calculation boundary
   - Life / Death / Reward Source event boundary
   - Spawn / Monster template boundary
   - Save/Load cross-system schema and load-order boundary
   - HUD / Loot Feedback / Inventory-Equipment UI handoff boundary
