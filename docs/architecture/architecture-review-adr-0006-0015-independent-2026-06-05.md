# Architecture Review — ADR-0006 through ADR-0015 Independent Scoped Review

Date: 2026-06-05
Engine: Godot 4.6.3
Mode: scoped independent `/architecture-review` over ADR-0006 through ADR-0015
Baseline: ADR-0001 through ADR-0005 for cross-ADR consistency; existing `architecture-review-2026-06-05.md`, `architecture-review-refresh-2026-06-05.md`, and `traceability-index.md` for historical context

---

## 1. Scope and Loaded Inputs

### Primary ADR scope

Reviewed ADRs:

- `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`
- `docs/architecture/adr-0007-attribute-snapshot-query-api-shape-and-read-only-enforcement.md`
- `docs/architecture/adr-0008-attribute-event-signal-contract-and-scene-tree-independent-core.md`
- `docs/architecture/adr-0009-attribute-atomic-source-update-and-transaction-model.md`
- `docs/architecture/adr-0010-attribute-save-load-persistence-boundary-base-current-modifier-sources.md`
- `docs/architecture/adr-0011-attribute-fixture-config-loading-strategy-mvp-provisional-values.md`
- `docs/architecture/adr-0012-attribute-formula-only-gut-test-strategy-without-scene-tree-ui-autoload.md`
- `docs/architecture/adr-0013-attribute-godot-resource-duplication-shared-reference-policy-if-tres-resources-are-used.md`
- `docs/architecture/adr-0014-attribute-combat-power-main-stat-display-proxy-ownership.md`
- `docs/architecture/adr-0015-item-definition-runtime-data-validation-query-and-versioning.md`

All primary-scope ADRs are currently `Accepted`.

### Consistency baseline

Reviewed as dependency and consistency context:

- `docs/architecture/adr-0001-map-data-representation.md`
- `docs/architecture/adr-0002-typed-query-result-schema.md`
- `docs/architecture/adr-0003-authoritative-occupancy-reservation-update-ordering.md`
- `docs/architecture/adr-0004-deterministic-y-sort-implementation.md`
- `docs/architecture/adr-0005-input-projection-coordinate-conversion.md`

All baseline ADRs are currently `Accepted`.

### GDD and registry context

Reviewed or checked as requirements context:

- `design/gdd/map-coordinate-blocking-y-sort-system.md`
- `design/gdd/character-attributes-system.md`
- `design/gdd/item-definition-system.md`
- `design/gdd/systems-index.md`
- `docs/architecture/tr-registry.yaml`
- `docs/architecture/traceability-index.md`
- `.claude/docs/technical-preferences.md`
- `docs/engine-reference/godot/VERSION.md`

Loaded count for this scoped pass: 4 GDD/context documents, 15 ADRs, 1 TR registry, 1 traceability index, and Godot 4.6.3 engine preference/reference files.

---

## 2. Traceability Summary

Verdict for scoped coverage: **PASS**

This scoped review found no new ADR coverage gaps for the systems governed by ADR-0006 through ADR-0015.

| Area | Relevant Requirements | ADR Coverage | Status |
|---|---:|---|---|
| Map-space baseline used by Attributes / Items | `TR-map-space-*` dependency context | ADR-0001 through ADR-0005 | ✅ Covered for this scoped dependency review |
| Character Attributes | `TR-attributes-001` through `TR-attributes-018` | ADR-0006 through ADR-0014 | ✅ Covered |
| Item Definition | `TR-item-definition-001` through `TR-item-definition-021` | ADR-0015 plus dependencies on ADR-0006, ADR-0013, ADR-0014 where needed | ✅ Covered at ADR-boundary level, with known downstream partials for concrete MVP content and UI/accessibility metadata |
| Cross-system UI/read-only constraints from `systems-index.md` | `TR-systems-index-001`, `TR-systems-index-002`, `TR-systems-index-006`, `TR-systems-index-007` | ADR-0001 through ADR-0015 | ✅ Covered for authority boundaries; UX baseline remains a pre-gate concern |

### Notes

- `docs/architecture/tr-registry.yaml` was checked as the stable TR-ID source. No new TR-ID, renumbering, deprecation, or wording revision is required by this scoped review.
- Existing broader project partials remain tracked in `docs/architecture/traceability-index.md` and `docs/architecture/architecture-review-refresh-2026-06-05.md`. This scoped PASS does not claim that the whole Phase 1 slice is implementation-complete.

---

## 3. Cross-ADR Conflicts

Verdict for conflict phase: **PASS**

No blocking cross-ADR conflict was found between ADR-0006 through ADR-0015 and the ADR-0001 through ADR-0005 baseline.

### Confirmed ownership boundaries

- Map logical data and map-space authority remain with ADR-0001 through ADR-0005.
- Character Attributes owns stat identity, base/current/derived value semantics, modifier aggregation, snapshots, events, transaction-like source updates, persistence boundaries for attribute inputs, fixture strategy, tests, Resource policy, and combat-power display proxy output.
- Item Definition owns item template truth, stable `item_id`, validation/query/versioning boundaries, and raw authoring-to-runtime normalization.
- Item Definition does not own equip legality, inventory mutation, drop lifecycle, combat-power authority, or final attribute aggregation.
- UI/presentation remains read-only/projection-consuming and does not own gameplay truth.

### Non-blocking concerns to carry forward

- DTO sketches using public fields are conceptual only; implementation must enforce immutable-by-contract access, construction-time copies, and defensive collection rules.
- GDScript enum/int typing and compact ID patterns need implementation-time tests against Godot 4.6.3.
- GUT compatibility with Godot 4.6.3 still requires concrete test setup verification.
- Concrete MVP item catalogs, OpenMir2 evidence-backed contracts, and UX/accessibility metadata remain downstream artifacts, not new ADR gaps in this scoped pass.

---

## 4. ADR Dependency Order

No dependency cycle was found.

Recommended implementation dependency order for this scoped set:

1. Foundation map-space baseline:
   - ADR-0001 — Map Data Representation
   - ADR-0002 — Typed Query Result Schema
   - ADR-0003 — Authoritative Occupancy / Reservation Update Ordering
   - ADR-0004 — Deterministic Y-sort Implementation
   - ADR-0005 — Input Projection / Coordinate Conversion
2. Character Attributes foundation:
   - ADR-0006 — Attribute Data Representation and Stat ID Typing
   - ADR-0007 — Attribute Snapshot Query API Shape and Read-Only Enforcement
   - ADR-0008 — Attribute Event / Signal Contract and Scene-Tree-Independent Core
3. Character Attributes mutation, persistence, fixture, and testing layer:
   - ADR-0009 — Attribute Atomic Source Update and Transaction Model
   - ADR-0010 — Attribute Save/Load Persistence Boundary
   - ADR-0011 — Attribute Fixture / Config Loading Strategy
   - ADR-0012 — Attribute Formula-Only GUT Test Strategy
4. Shared Godot Resource and display boundary layer:
   - ADR-0013 — Attribute Godot Resource Duplication / Shared Reference Policy
   - ADR-0014 — Attribute Combat Power / Main Stat Display Proxy Ownership
5. Item Definition layer:
   - ADR-0015 — Item Definition Runtime Data, Validation, Query, and Versioning

All ADRs in this scoped dependency chain are currently `Accepted`.

---

## 5. Engine Compatibility

Verdict for engine compatibility: **PASS with implementation-time verification notes**

Engine target is consistently Godot 4.6.3.

### Findings

- No stale engine version reference was found in this scoped ADR set.
- No deprecated API conflict was found in this scoped ADR set.
- No post-cutoff API contradiction was found across the scoped ADRs.
- The ADR set avoids making `TileMapLayer`, Y-sort, physics, navigation, signals, Autoloads, scene nodes, or mutable `Resource` instances authoritative for gameplay truth.
- Scene-tree-independent `RefCounted` / DTO / service-style boundaries are consistent with the project’s Godot 4.6.3 direction.

### Carry-forward verification notes

- Conceptual DTO examples must be translated into immutable-by-contract GDScript APIs with no exposed mutable authoritative dictionaries or arrays.
- Compact integer IDs / enum-like constants must be validated rather than accepting arbitrary raw integers.
- `Resource` authoring data must be duplicated/normalized before runtime gameplay use to avoid shared cache or editor-time mutation leaks.
- GUT runner compatibility with Godot 4.6.3 remains a test-infrastructure task.

---

## 6. GDD Revision Flags

No new GDD revision flag was found by this scoped ADR-0006 through ADR-0015 pass.

The earlier broader review state may still contain GDD revision work related to accepted ADR-0016 through ADR-0020 and broader Phase 1 readiness. This report does not clear those broader flags; it only records that ADR-0006 through ADR-0015 introduce no additional design-feedback blocker.

---

## 7. Architecture Document Coverage

`docs/architecture/architecture.md` does not exist yet.

Architecture-wide overview/control-manifest coverage is therefore still missing. This is a pre-gate documentation concern, not a blocking contradiction inside ADR-0006 through ADR-0015.

---

## 8. Pre-Gate Checklist

Read-only existence checks for the `/architecture-review` handoff items:

| Required Path | Status | Recommended Next Step |
|---|---|---|
| `tests/unit` | ❌ Missing | Run `/test-setup` |
| `tests/integration` | ❌ Missing | Run `/test-setup` |
| `.github/workflows/tests.yml` | ❌ Missing | Run `/test-setup` |
| `design/accessibility-requirements.md` | ❌ Missing | Run `/ux-design` or align final UX path first |
| `design/ux/interaction-patterns.md` | ❌ Missing | Run `/ux-design` |
| `docs/architecture/architecture.md` | ❌ Missing | Create architecture overview/control manifest in a dedicated architecture pass |
| `docs/consistency-failures.md` | ❌ Missing | No action required this pass because no conflict entry exists to append |

Additional test setup note: the Godot project root is `GodotMirClient`, no GUT/GdUnit4 addon is currently installed, and existing project preferences say GUT while the current `/test-setup` and `/smoke-check` templates mention GdUnit4. Resolve that framework/path choice before scaffolding tests.

---

## 9. Verdict

## Verdict: PASS for scoped ADR architecture coverage, with pre-gate CONCERNS

### PASS scope

ADR-0006 through ADR-0015 are coherent with each other, with ADR-0001 through ADR-0005, and with the documented Godot 4.6.3 direction. The scoped Attribute and Item Definition architecture coverage is sufficient to proceed into careful implementation planning once the project accepts the broader pre-gate constraints.

### CONCERNS scope

The project remains not gate-ready because required test infrastructure, CI, UX/accessibility baselines, and architecture overview documentation are missing. Those concerns should be resolved through `/test-setup`, `/ux-design`, and a dedicated architecture overview/control-manifest pass rather than by modifying this scoped ADR set.

### Blocking Issues for this scoped review

None.

### Required ADRs from this scoped review

None.

---

## 10. Handoff

Immediate actions:

1. Run `/test-setup` after deciding:
   - GUT vs GdUnit4.
   - repo-root `tests/` vs `GodotMirClient/tests/` for Godot `res://` compatibility.
   - GitHub Actions execution path for `GodotMirClient`.
2. Run `/ux-design` to create interaction and accessibility baselines.
3. Create `docs/architecture/architecture.md` or equivalent architecture overview/control manifest before pre-production gate review.

Rerun trigger: rerun `/architecture-review` after test setup, UX baselines, and architecture overview exist; run `/architecture-review rtm` later when implementation stories and test evidence files exist.
