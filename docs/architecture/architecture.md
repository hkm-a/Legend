# Architecture Overview

Last Updated: 2026-06-05
Engine: Godot 4.6.3
Stage: Systems Design / Phase 1 technical slice preparation

## Purpose

This document summarizes the current accepted architecture for the Phase 1 2D/2.5D offline loot-loop slice. It is a navigation and implementation-planning document, not a replacement for the detailed Architecture Decision Records.

Authoritative decisions live in:

- `docs/architecture/adr-0001-map-data-representation.md` through `docs/architecture/adr-0020-resource-map-conversion-pipeline-and-validation-boundary.md`
- `docs/registry/architecture.yaml`
- `docs/architecture/tr-registry.yaml`
- `docs/architecture/traceability-index.md`

## Current Verdict Snapshot

The latest architecture refresh found:

- 83 traceability requirements registered.
- 73 covered.
- 10 partial.
- 0 hard architecture gaps.
- No blocking cross-ADR conflict across Accepted ADR-0001 through ADR-0020.
- No Godot 4.6.3 engine compatibility blocker found.

The project is not yet a clean pre-production gate pass because several domains remain partial and implementation evidence is not yet present.

## Phase 1 Target

The Phase 1 technical slice validates the 30-second offline loot loop:

1. Move on an OpenMir2-inspired 2D/2.5D map.
2. Target and attack a simple enemy.
3. Resolve death/reward context.
4. Generate a drop.
5. Place a readable ground item.
6. Pick it up.
7. Receive it into inventory.
8. Equip or compare it.
9. Show attribute/combat-power feedback.

Networking/protocol behavior remains spike/future scope for Phase 1. OpenMir2 behavior is source-first evidence, not runtime networking authority.

## System Ownership Boundaries

### Map Definition and Map Space

Accepted ADRs:

- ADR-0001 Map Data Representation
- ADR-0002 Typed Query Result Schema
- ADR-0003 Authoritative Occupancy Reservation Update Ordering
- ADR-0004 Deterministic Y-Sort Implementation
- ADR-0005 Input Projection Coordinate Conversion
- ADR-0019 Map Distance and Movement Legality Contract
- ADR-0020 Resource / Map Conversion Pipeline and Validation Boundary

Ownership rules:

- Logical grid is gameplay authority.
- Static map facts belong to map definition data.
- Runtime occupancy and movement reservation belong to `MapSpaceState`.
- Visual tiles, sprites, physics helpers, and screen coordinates cannot override logical gameplay truth.
- Movement legality and distance facts use explicit typed query/result contracts.
- Diagonal/corner-cutting and OpenMir2-authentic distance behavior remain evidence-gated.
- Y-sort is deterministic visual ordering only; it must not affect gameplay legality.

### Character Attributes

Accepted ADRs:

- ADR-0006 Attribute Data Representation and Stat ID Typing
- ADR-0007 Attribute Snapshot Query API Shape and Read-Only Enforcement
- ADR-0008 Attribute Event Signal Contract and Scene-Tree-Independent Core
- ADR-0009 Attribute Atomic Source Update and Transaction Model
- ADR-0010 Attribute Save Load Persistence Boundary
- ADR-0011 Attribute Fixture Config Loading Strategy
- ADR-0012 Attribute Formula-Only GUT Test Strategy
- ADR-0013 Attribute Resource Duplication / Shared Reference Policy
- ADR-0014 Attribute Combat Power and Main Stat Display Proxy Ownership

Ownership rules:

- Character Attributes owns stat identity, aggregation, validation, snapshots, current-resource mutation, display proxy outputs, and attribute event/result DTOs.
- Equipment, combat, HUD, save/load, scene nodes, and UI must not directly write final attributes, derived stats, or snapshot internals.
- Runtime stat identity uses compact typed IDs after normalization; external authoring/save/debug boundaries use stable semantic keys.
- Snapshots are immutable-by-contract and returned through status-bearing query wrappers.
- Structural source updates are actor-local transactions with staged candidate state and final commit/swap.
- Formula/contract evidence must be tested directly through scene-tree-independent GUT-style unit tests.

### Item Definition

Accepted ADR:

- ADR-0015 Item Definition Runtime Data Validation Query and Versioning

Ownership rules:

- Item Definition owns template truth: stable item IDs, item type, quality metadata, display metadata, stack policy, equipment data block, modifier payload, source/evidence labels, lifecycle, validation, query/projection, and version/migration boundary.
- Drop, inventory, equipment, UI, save/load, and attribute systems must not duplicate item definition fields as truth.
- Runtime truth is normalized/indexed data with status-bearing query results.
- `.tres` or other Godot Resources may be authoring envelopes only, not mutable runtime authority.
- Quality is presentation/classification metadata, not power, drop rate, price, or combat-power authority.

### OpenMir2 Evidence Governance

Accepted ADR:

- ADR-0016 OpenMir2 Evidence Mapping Registry and Provisional Contract Governance

Ownership rules:

- OpenMir2 source is Tier 1 behavior evidence.
- MinimalMirClient, MirServer config, mir2x, and other references cannot independently define authoritative behavior.
- Downstream systems may claim source-authentic behavior only when evidence reaches the required readiness level.
- Provisional MVP behavior must be labeled and must not be misrepresented as OpenMir2-verified.
- Intentional divergence requires an explicit divergence record.

### Drop, Ground Item, Pickup, Inventory, and Equipment

Accepted ADRs:

- ADR-0017 Drop Table, Ground Drop, and Pickup Lifecycle Boundary
- ADR-0018 Inventory and Equipment Instance Modifier Transaction Boundary

Ownership rules:

- Drop lifecycle separates death/reward source context, drop roll, ground drop placement, pickup request, and inventory receive.
- Ground-drop placement failure must not delete, reroll, or silently move rewards.
- Pickup is staged: ground state and inventory receive must not half-commit.
- Inventory owns item instances, stack/container state, and staged receive.
- Equipment owns equip legality, equipment slots, equipment instance binding, and modifier-source handoff.
- Character Attributes owns aggregation of accepted modifier sources, not equipment legality or inventory container truth.

## Data and DTO Rules

Across systems:

- Use typed/status-bearing result DTOs for queries and failed operations.
- Include status, primary reason, secondary reasons where needed, context, and retry/debug hints as appropriate.
- Do not expose mutable arrays/dictionaries as runtime authority.
- Use defensive copies and immutable-by-contract DTOs for snapshots, query results, and event/result payloads.
- Treat public `var` examples in ADRs as conceptual unless implementation enforces read-only API shape.
- Avoid stringly typed gameplay authority after normalization; stable semantic keys remain external boundary identifiers.

## Godot 4.6.3 Constraints

Project constraints:

- Engine: Godot 4.6.3.
- Language: GDScript.
- Rendering: 2D / CanvasItem pipeline.
- Primary platform: PC.
- Primary input: keyboard/mouse.
- Partial gamepad support.
- No touch support.

Implementation guardrails:

- Verify post-cutoff Godot APIs against `docs/engine-reference/godot/` before coding.
- Prefer statically typed GDScript.
- Prefer scene-tree-independent `RefCounted` service/core classes for gameplay authority.
- Avoid Autoload singletons as authoritative gameplay dependencies in Phase 1 logic.
- Use typed signals / Callable connections only in downstream adapters, not as state authority.
- Treat Resources and `.tres` as authoring/import envelopes unless an ADR explicitly permits broader use.
- Godot 4.6 dual-focus UI behavior requires separate testing of mouse hover and keyboard/gamepad focus.

## Test Architecture Baseline

Current files:

- `tests/unit/`
- `tests/integration/`
- `tests/gdunit4_runner.gd`
- `.github/workflows/tests.yml`

Policy:

- GUT is the preferred GDScript unit test framework per project technical preferences.
- The current runner is an intentional guard: it fails until an approved GUT/GdUnit4 addon is installed and wired.
- CI must not falsely report success when no real tests are running.
- Blocking logic stories require automated test evidence in the appropriate `tests/unit/[system]/` or `tests/integration/[system]/` path.
- Character Attributes formula/contract tests must follow ADR-0012: no gameplay scenes, UI, Autoloads, filesystem fixtures, timers/frames, or signal-order authority in the blocking oracle.

Next test setup work:

1. Approve and install the selected Godot GDScript test addon/version.
2. Replace/delegate `tests/gdunit4_runner.gd` to the real runner.
3. Add first smoke test that proves the runner executes real tests.
4. Add first blocking unit tests for the selected implementation slice.

## UX and Accessibility Baselines

Current files:

- `design/ux/interaction-patterns.md`
- `design/ux/accessibility-requirements.md`

Baseline rules:

- Core interactions are mouse-first but must have keyboard-accessible UI alternatives.
- Inventory and equipment flows must not depend exclusively on drag/drop.
- Focus, hover, selection, and disabled states must be visually distinct.
- Color must not be the only carrier for rarity, stat deltas, validity, target category, or error state.
- Hover-only tooltip information must also be available by keyboard/gamepad focus or selection.
- Critical feedback must not be audio-only.
- Item loss/destructive actions require safe, deliberate flows.

## Remaining Partial Domains

The following domains still need further design/ADR/spec work before a clean gate pass:

1. Actual OpenMir2 behavior contracts for all required domains.
2. Combat, damage, life/death, and spawn system design/architecture.
3. Save/Load system-level schema, write atomicity, migration orchestration, and load ordering.
4. HUD, loot feedback, inventory/equipment screen-level UX specs, audio/VFX feel, and evidence.
5. Concrete MVP item catalog and fixture naming.
6. Passing automated tests and playtest/UI evidence.
7. GDD refresh against Accepted ADR-0016 through ADR-0020.

## Implementation Planning Guidance

When creating implementation stories:

1. Link each story to the relevant GDD acceptance criteria and ADRs.
2. Identify the owning system and forbidden cross-system writes before coding.
3. Write or wire blocking tests first for logic stories.
4. Keep gameplay authority outside scene nodes unless an ADR explicitly permits otherwise.
5. Use DTO/result wrappers for failed operations rather than booleans or null-only failure semantics.
6. Keep gameplay values data-driven and provisional values labeled.
7. Update `production/session-state/active.md` after each significant milestone.

## References

- `docs/architecture/architecture-review-refresh-2026-06-05.md`
- `docs/architecture/traceability-index.md`
- `docs/architecture/tr-registry.yaml`
- `docs/registry/architecture.yaml`
- `design/gdd/systems-index.md`
- `design/ux/interaction-patterns.md`
- `design/ux/accessibility-requirements.md`
- `docs/engine-reference/godot/VERSION.md`
