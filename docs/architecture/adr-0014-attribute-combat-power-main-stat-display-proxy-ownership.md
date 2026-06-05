# ADR-0014: Attribute Combat Power and Main Stat Display Proxy Ownership

## Status

Accepted

## Date

2026-06-04

## Last Verified

2026-06-04

## Decision Makers

hkm + Claude Code Game Studios

## Summary

This ADR defines ownership for Character Attributes display proxy data: `combat_power`, `combat_power_before`, `combat_power_after`, `combat_power_delta`, `primary_comparison_stat`, visible/hidden delta summaries, `growth_reason`, and `growth_salience`. Character Attributes owns formula-tested provisional display proxy computation and semantic handoff metadata derived from validated snapshots, config, transactions, and previews. Combat power is display-only and MVP-provisional: it is never damage authority, equip legality, item valuation authority, UI layout authority, OpenMir2-authentic truth, or a safe celebration trigger when stale/invalid/unavailable.

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Core / Scripting / UI Handoff |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff; this ADR avoids post-cutoff-only APIs and must preserve verified signal/DTO/testing boundaries. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md`; `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`; `docs/architecture/adr-0007-attribute-snapshot-query-api-shape-and-read-only-enforcement.md`; `docs/architecture/adr-0008-attribute-event-signal-contract-and-scene-tree-independent-core.md`; `docs/architecture/adr-0009-attribute-atomic-source-update-and-transaction-model.md`; `docs/architecture/adr-0010-attribute-save-load-persistence-boundary-base-current-modifier-sources.md`; `docs/architecture/adr-0011-attribute-fixture-config-loading-strategy-mvp-provisional-values.md`; `docs/architecture/adr-0012-attribute-formula-only-gut-test-strategy-without-scene-tree-ui-autoload.md`; `docs/architecture/adr-0013-attribute-godot-resource-duplication-shared-reference-policy-if-tres-resources-are-used.md` |
| **Post-Cutoff APIs Used** | None required. Phase 1 does not require Godot 4.5+ `@abstract`, variadic arguments, `Resource.duplicate_deep()`, SceneTree focus APIs, or UI-specific post-cutoff APIs for display proxy computation. |
| **Verification Required** | Verify `RefCounted` DTO aliasing/defensive-copy behavior; display proxy status mapping to snapshot/result status; typed signal adapters emit only after result materialization; no deprecated string-based `connect()`; no SceneTree/Autoload/UI dependency in formula/core tests; UI handoff uses semantic IDs/status and does not recompute formulas; combat/equipment/save/AI do not consume combat power as authority. |

> **Note**: If Knowledge Risk is MEDIUM or HIGH, this ADR must be re-validated if the project upgrades engine versions. Flag it as "Superseded" and write a new ADR.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | ADR-0006 Attribute Data Representation and Stat ID Typing; ADR-0007 Attribute Snapshot Query API Shape and Read-Only Enforcement; ADR-0008 Attribute Event Signal Contract and Scene-Tree-Independent Core; ADR-0009 Attribute Atomic Source Update and Transaction Model; ADR-0010 Attribute Save Load Persistence Boundary; ADR-0011 Attribute Fixture Config Loading Strategy; ADR-0012 Attribute Formula Only GUT Test Strategy; ADR-0013 Attribute Godot Resource Duplication Policy; approved Character Attributes GDD: `design/gdd/character-attributes-system.md`. |
| **Enables** | Character Attributes player-facing growth handoff implementation; formula-only tests for `combat_power`; equipment preview display proxy; visible/hidden delta summaries for equipment UI/growth feedback; downstream HUD/equipment UI/growth feedback stories once their GDDs exist. |
| **Blocks** | Any Character Attributes implementation story that exposes combat power, primary comparison stat, growth salience, visible delta summary, equipment preview display proxy, or growth handoff fields; any combat/equipment/UI story that consumes these fields before the ownership boundary is accepted. |
| **Ordering Note** | ADR-0006 through ADR-0014 are now Accepted. Implementation stories may reference these accepted Character Attributes stances, but Done remains blocked until the approved real Godot GDScript test runner executes required story/unit tests. This ADR does not replace future Combat, Equipment, Item Definition, UI/HUD, Localization, or Growth Feedback GDDs; it defines the Character Attributes side of the handoff only. |

## Context

### Problem Statement

The Character Attributes GDD requires player-facing growth handoff data so players can understand that equipment, level, or template changes made them stronger. It names `primary_comparison_stat`, `combat_power_before`, `combat_power_after`, `combat_power_delta`, visible delta summary, hidden delta summary, growth reason, and growth salience.

Without a clear ownership decision, downstream systems could recompute formulas, treat `combat_power` as damage or item value authority, persist it as save truth, celebrate stale/invalid rebuilds, or require UI code to infer before/after deltas. That would contradict the Character Attributes authority model and blur boundaries with Combat, Equipment, UI, Save/Load, and future OpenMir2 evidence mapping.

### Current State

No Character Attributes implementation exists. The GDD is approved for ADR authoring and explicitly lists combat power/main stat display proxy ownership as an implementation-blocking ADR/technical design. ADR-0006 through ADR-0013 already define stat IDs, snapshots, events/results, transactions, save/load, config fixtures, formula tests, and Resource boundaries. This ADR closes the final display proxy ownership gap.

### Constraints

- Combat power is MVP provisional and display-only.
- Combat owns final damage, hit/crit, DPS, TTK, and combat action semantics.
- Equipment owns equip legality, item/category/slot semantics, and resolved modifier sources.
- UI/HUD/equipment UI/growth feedback own presentation, accessibility, localization rendering, layout, VFX/audio, and timing.
- Character Attributes owns formula-tested attribute-derived display proxy data so downstream UI does not recompute gameplay formulas.
- Stale, invalid, unavailable, or preview-only data must not become normal growth celebration authority.
- Formula/core logic must remain scene-tree-independent and testable without UI/Autoload/signals.

### Requirements

- Provide authoritative display proxy DTOs derived from validated attribute state and config.
- Explicitly forbid combat power as damage, equip legality, item valuation, AI, save truth, or OpenMir2-authentic authority.
- Expose before/after/delta data with version provenance so UI does not infer missing fields.
- Preserve preview non-authority and committed event/result boundaries.
- Provide deterministic `primary_comparison_stat` fallback for valid growth-relevant rebuilds without item hints.
- Align delta row content with GDD AC-08.
- Fail all-zero or no-active-player-facing contribution configs deterministically.

## Decision

Character Attributes owns the provisional display proxy computation and semantic handoff metadata for:

- `combat_power_before`;
- `combat_power_after`;
- `combat_power_delta`;
- current `combat_power` when queried from one valid snapshot;
- `primary_comparison_stat`;
- `visible_delta_summary`;
- `hidden_delta_summary`;
- `growth_reason`;
- `growth_salience`;
- display proxy validity/mode/status.

This ownership applies only when those values are derived from already validated attribute snapshots, ADR-0011 config weights, source/rebuild results, current-resource mutation results where relevant, or non-authoritative preview candidate calculations.

Display proxy data is **not**:

- damage formula authority;
- hit/crit/DPS/TTK truth;
- combat AI authority;
- equip legality;
- item worth, sell value, rarity, drop value, or build optimization authority;
- OpenMir2-authentic truth;
- save/load authoritative state;
- UI layout, localization rendering, icon/color choice, VFX/audio, or animation timing;
- a valid celebration trigger when stale, invalid, preview-only, or unavailable.

### System Boundaries

Combat must never read `combat_power`, `primary_comparison_stat`, visible deltas, or growth salience to resolve damage, hit, crit, TTK, target selection, or combat legality. Combat consumes ADR-0007 snapshots and combat-specific future GDD/ADR contracts.

Equipment owns item legality, item slot/category semantics, modifier resolution, and whether an item/source may be submitted. Equipment may provide validated category/stat hints, source deltas, and preview requests. Character Attributes consumes only validated hints/config to select display proxy fields. It does not infer equip legality, item slot truth, rarity, sell value, drop value, or item valuation.

UI/HUD/equipment UI/growth feedback own presentation. They may hide, degrade, badge, localize, color, iconize, animate, or sonify display proxy data, but they must not recompute attribute formulas, aggregate modifiers, infer missing before/after values, or celebrate stale/invalid/unavailable display proxy states. Positive/negative presentation must not rely on color alone.

Save/Load persists authoritative inputs and versions, not `combat_power` or display proxy output. Display proxy values may appear in debug evidence or caches only as non-authoritative comparison data and must be recomputed after load from validated snapshots/config.

### Combat Power Formula Authority

Character Attributes computes combat power only through ADR-0011 config weights and ADR-0012 formula-tested logic. Every output carries:

- `mvp_provisional` label;
- `display_only` label;
- source status/evidence label summary;
- schema version;
- config version;
- stat registry version;
- before/after snapshot versions or preview base version;
- display proxy mode/status;
- structured failure/unavailable reasons when computation cannot produce a safe display value.

Inactive, reserved, debug-only, hidden, or non-player-facing stats contribute exactly `0` to normal combat power and are filtered out of normal visible summaries.

Config validation fails in Stage A if, after filtering inactive/reserved/debug-only/non-player-facing stats, no active player-facing combat power contribution has positive weight. Nonzero combat-power weights assigned to inactive/reserved/debug-only stats are rejected during config validation unless a later debug-only config ADR explicitly permits them for debug surfaces. If a child formula fails or an otherwise valid snapshot has no computable display contribution, display proxy returns structured unavailable/failure and must not show `0` as meaningful.

### Primary Comparison Stat Selection

`primary_comparison_stat` is a semantic display handoff field, not equipment legality or item valuation. Selection uses this order:

1. If a valid equipment/item/source/category/stat hint is supplied by Equipment or config, choose the configured display priority for that hint. Phase 1 defaults are `physical_attack_pair` for offensive hints, `health_max` / `physical_defense_pair` for survivability hints.
2. For valid accepted growth-relevant authoritative rebuilds without a valid hint, select the first non-zero visible delta by configured display priority, preferring active pair rows over scalar rows.
3. If no visible non-zero delta exists, `primary_comparison_stat` may be unavailable and `growth_salience` must be `none` or `debug_only` as appropriate.
4. For invalid, stale display-only, failed rebuild, or unavailable display proxy states, `primary_comparison_stat` is unavailable for normal celebration.

This fallback prevents UI from recomputing formulas while preserving Equipment ownership of item semantics.

### Display Proxy Status Mapping

| Display Proxy Mode | Allowed Source State | Consumer Rule |
|---|---|---|
| `authoritative` | Committed valid snapshot/result after ADR-0009 commit/swap | Safe for player-facing growth display only if reason/salience allows; not combat authority. |
| `preview` | Non-authoritative `AttributePreviewResult` candidate calculation; no source commit, no authoritative version increment, no sink/signal/event commit | Comparison only; not growth celebration unless later committed. |
| `stale_display_only` | Previous valid snapshot exposed after failed rebuild per ADR-0007/0009 | UI/debug only; no combat truth, save truth, or celebration. |
| `invalid` | Failed config/source/calculation/output validation | No normal growth display; only safe unavailable/debug presentation. |
| `unavailable` | Valid state exists but display proxy cannot be computed or compared | Show no combat power/main stat; UI may use available visible deltas if present and status allows. |

These modes are display-proxy views over existing snapshot/query/result statuses, not a replacement authority vocabulary.

### Delta Row Contract

Visible and hidden delta summary rows must carry, directly or through parent result metadata:

- stat ID or pair ID;
- before value;
- after value;
- signed delta;
- reason;
- before snapshot version;
- after snapshot version, or preview marker/base version;
- display metadata key;
- visibility classification;
- source/status labels needed for safe presentation.

Rows must not contain final localized text, UI colors, icons, layout directives, mutable Resource payloads, mutable Dictionary payloads, or mutable collection references.

### Preview Non-Authority

Display proxy data may be derived from non-authoritative `AttributePreviewResult` candidate calculations only if those calculations do not commit sources, advance authoritative versions, invoke sinks/signals, replace snapshots, or enter `Array[AttributeDomainEvent]`.

Preview display proxy results include base snapshot/source/config versions and a preview marker. They are valid for comparison presentation only and cannot trigger committed growth feedback.

### Event and Signal Boundary

`AttributeUpdateResult` and `AttributePreviewResult` may include compact display proxy IDs, deltas, statuses, and summary rows. They must not include full old+new snapshots by default. Current truth still comes from ADR-0007 snapshot/query APIs.

Godot signals are allowed only in downstream typed signal adapters after result materialization. Signal callback order must not decide transaction outcome, versions, display proxy status, combat truth, save truth, or growth eligibility. Connections use typed signal / Callable style, not deprecated string-based `connect("signal", obj, "method")`.

### Implementation Shape

Phase 1 does not require Godot 4.5+ `@abstract`. Implementations may use scene-tree-independent `RefCounted` services, static helpers, or injected strategy objects. Public DTO APIs are immutable-by-contract: constructor/factory validation, copied inbound arrays, getter-only access, defensive copies for collection output, no public mutable vars, no setters, and no mutator methods.

### Architecture Diagram

```text
Validated committed snapshot(s) / preview candidate
        |
        | ADR-0007 status + ADR-0009 version/source rules
        v
Attribute display proxy computation
        |
        +--> combat_power_before / after / delta
        +--> primary_comparison_stat
        +--> visible / hidden delta rows
        +--> growth_reason / growth_salience
        +--> status/mode/source labels
        v
AttributeUpdateResult / AttributePreviewResult / query wrapper
        |
        +--> Combat: must ignore display proxy for damage authority
        +--> Equipment: may use for comparison presentation only
        +--> UI/Growth: presents safely; no recompute/no stale celebration
        +--> Save/Load: does not persist as authority
```

### Key Interfaces

```gdscript
class_name AttributeDisplayProxyResult
extends RefCounted

func is_success() -> bool:
    pass

func get_mode() -> int:
    pass

func get_combat_power_before() -> AttributeDisplayNumberQueryResult:
    pass

func get_combat_power_after() -> AttributeDisplayNumberQueryResult:
    pass

func get_combat_power_delta() -> AttributeDisplayNumberQueryResult:
    pass

func get_primary_comparison_stat() -> AttributePrimaryComparisonQueryResult:
    pass

func get_visible_delta_rows_copy() -> Array[AttributeVisibleDeltaRow]:
    pass

func get_hidden_delta_rows_copy() -> Array[AttributeVisibleDeltaRow]:
    pass

func get_failure_reasons_copy() -> Array[int]:
    pass
```

```gdscript
class_name AttributeVisibleDeltaRow
extends RefCounted

func get_stat_id() -> int:
    pass

func get_pair_id() -> int:
    pass

func is_pair_row() -> bool:
    pass

func get_before_value() -> int:
    pass

func get_after_value() -> int:
    pass

func get_signed_delta() -> int:
    pass

func get_reason_id() -> int:
    pass

func get_before_snapshot_version() -> int:
    pass

func get_after_snapshot_version() -> int:
    pass

func is_preview() -> bool:
    pass

func get_display_metadata_key() -> StringName:
    pass
```

```gdscript
class_name AttributeGrowthHandoff
extends RefCounted

func get_growth_reason() -> int:
    pass

func get_growth_salience() -> int:
    pass

func get_display_proxy() -> AttributeDisplayProxyResult:
    pass
```

```gdscript
class_name AttributeCombatPowerFormula
extends RefCounted

func compute(snapshot: AttributeSnapshot, config: AttributeRuntimeConfigTables) -> AttributeDisplayNumberQueryResult:
    pass

func compare(before_snapshot: AttributeSnapshot, after_snapshot: AttributeSnapshot, config: AttributeRuntimeConfigTables) -> AttributeDisplayProxyResult:
    pass
```

```gdscript
class_name AttributePrimaryStatSelector
extends RefCounted

func select_primary_stat(delta_rows: Array[AttributeVisibleDeltaRow], hint: AttributeDisplayHint, config: AttributeRuntimeConfigTables) -> AttributePrimaryComparisonQueryResult:
    pass
```

The interface names are conceptual. Exact filenames, enums, result wrapper names, and constructor/factory APIs may be refined during implementation while preserving ownership, status, version, and read-only boundaries.

## Alternatives Considered

### Alternative 1: UI recomputes combat power and primary deltas from snapshots

- **Description**: HUD/equipment UI reads snapshots and locally computes combat power, main stat, and visible summary rows.
- **Pros**: UI can iterate quickly; fewer Character Attributes DTOs.
- **Cons**: Duplicates formulas; risks stale/invalid celebration; can diverge from config/version/source labels; pushes gameplay formula logic into presentation code.
- **Rejection Reason**: The GDD requires Character Attributes to provide enough structured data for growth handoff without downstream recomputing gameplay rules.

### Alternative 2: Combat owns combat power

- **Description**: Combat system calculates a power score from damage/TTK assumptions and exposes it to UI/equipment.
- **Pros**: Could align later with actual DPS/TTK; combat is the final source of damage behavior.
- **Cons**: Phase 1 combat GDD is not authored; combat power in current GDD is display-only/provisional; would confuse damage authority with growth presentation; blocks attributes on combat design.
- **Rejection Reason**: Combat owns damage/hit/crit/TTK, not this provisional display proxy. Future combat-derived scoring can supersede or supplement this ADR later.

### Alternative 3: Equipment owns all comparison math

- **Description**: Equipment/item system calculates combat power, main stat selection, and visible deltas from item modifiers and current snapshot.
- **Pros**: Equipment understands item category/slot/legality; could tailor item-specific comparisons.
- **Cons**: Equipment would need to recompute attribute formulas or duplicate config logic; risks bypassing Attribute transaction/snapshot status; violates Character Attributes formula ownership.
- **Rejection Reason**: Equipment owns item legality and modifier resolution, but Character Attributes owns attribute-derived formula/delta/proxy computation.

### Alternative 4: No combat power in Phase 1

- **Description**: Expose only raw stat deltas and omit combat power until Combat/Equipment/UI systems mature.
- **Pros**: Avoids misleading power score; less implementation work.
- **Cons**: Weakens MVP growth readability; contradicts GDD combat_power formula and AC-16 fixture evidence; loses a simple quick-judgment tool for loot loop validation.
- **Rejection Reason**: Phase 1 needs provisional display aid, but it must be strongly labeled and bounded as display-only.

### Alternative 5: Combat power as item valuation authority

- **Description**: Use combat power delta to rank item worth, equip recommendations, sell value, rarity, or build optimization.
- **Pros**: Simple one-number comparison; easy UX.
- **Cons**: Misrepresents sidegrades, future builds, skills, rarity/economy, and OpenMir2 authenticity; turns a provisional display proxy into broad item authority.
- **Rejection Reason**: Item valuation, rarity, sell value, equip recommendations, and build optimization belong to Equipment/Item/Economy/UI systems, not Character Attributes combat power.

## Consequences

### Positive

- Players get clear growth handoff without UI recomputing formulas.
- Combat power remains explicitly display-only and provisional.
- Combat, Equipment, UI, Save/Load, and Character Attributes boundaries stay separated.
- Stale/invalid/unavailable states are modeled before presentation.
- AC-08 delta fields and AC-16 growth handoff become implementable as DTO contracts.
- Formula-only tests can prove combat power and delta behavior without scenes/UI.

### Negative

- Character Attributes owns more DTO/result surface area.
- Display proxy computation adds validation and test burden.
- Provisional combat power may still be misunderstood unless labels and forbidden patterns are enforced.
- Equipment/UI must provide or consume semantic hints instead of directly calculating everything.
- Future combat-derived item scoring may require superseding or extending this ADR.

### Neutral

- This ADR does not define final UI layout, text, color, icons, sounds, animation, accessibility treatment, or localization file format.
- This ADR does not define damage, TTK, equip legality, item rarity, economy value, or OpenMir2-authentic scoring.
- This ADR does not prevent future Combat or Equipment systems from adding their own non-authoritative comparison aids if they do not contradict this boundary.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| Combat power is misused as damage/TTK authority. | Medium | High | Register forbidden pattern; combat tests/code review prove damage formulas do not read display proxy DTOs. |
| UI celebrates stale/invalid/unavailable growth. | Medium | High | DTO modes/statuses gate celebration; UI walkthrough/tests verify safe presentation. |
| Missing equipment/category hint causes valid growth to lose primary stat. | Medium | Medium | Deterministic fallback selects first non-zero visible delta by configured priority. |
| Delta rows omit before/after/version metadata. | Medium | High | Row contract includes AC-08 fields directly or through parent metadata. |
| Config with hidden nonzero weights passes but visible power has no contribution. | Medium | High | Stage A rejects no active player-facing positive contribution and nonzero inactive/reserved weights unless a later debug-only ADR permits them. |
| `RefCounted` DTOs leak mutable arrays/rows. | Medium | High | Constructors copy; getters return defensive copies; mutation tests prove isolation. |
| Preview data is treated as committed result/event. | Medium | High | Preview mode carries marker/base versions and never commits, increments versions, invokes sinks/signals, or enters committed event arrays. |
| Salience is interpreted as a command to play VFX/audio. | Medium | Medium | Salience is semantic eligibility hint only; UI/growth feedback owns presentation and may choose no effect. |
| Combat power becomes item valuation authority. | Medium | Medium | ADR forbids sell value, rarity, item worth, drop value, equip legality, and build optimization authority. |
| Display proxy computation is polled every frame by UI. | Low | Medium | Proxy is materialized through update/preview/query results; UI caches DTOs and refreshes on events/requests. |

## Performance Implications

| Metric | Before | Expected After | Budget |
|--------|--------|---------------|--------|
| CPU (frame time) | No implementation. | Display proxy computation happens during update result materialization, preview calculation, or explicit query; no per-frame UI recomputation. | Overall project frame budget 16.6 ms at 60 fps; no per-frame full attribute rebuild/display proxy scan. |
| Memory | No implementation. | Additional immutable-by-contract DTOs for display proxy and delta rows per result/preview; no full old+new snapshot payloads by default. | Phase 1 client under 1 GB RAM; keep event payload compact. |
| Load Time | No implementation. | No direct load-time impact beyond config validation for combat power weights and display priority tables. | Acceptable for Phase 1. |
| Network | Not applicable for Phase 1 offline slice. | Not applicable. | None. |

## Migration Plan

No existing Character Attributes implementation needs migration.

1. Define display proxy status/mode enums and failure reason IDs aligned with ADR-0007/0008/0009 statuses.
2. Define `AttributeDisplayProxyResult`, `AttributeVisibleDeltaRow`, `AttributeGrowthHandoff`, display number query result, and primary comparison query result DTOs.
3. Extend ADR-0011 config fixtures with combat power weights, visible display priority, pair/scalar display metadata keys, and primary selector priorities.
4. Implement formula-tested `combat_power` compute/compare logic with before/after/delta and structured failure propagation.
5. Implement primary stat selector using validated hints and deterministic visible-delta fallback.
6. Attach compact display proxy data to `AttributeUpdateResult` / `AttributePreviewResult` where relevant.
7. Add formula-only GUT tests per ADR-0012 for combat power, delta row completeness, stale/invalid handling, preview non-authority, DTO aliasing, and forbidden consumer misuse.
8. Downstream UI/equipment/growth stories consume the DTOs without recomputing formulas.

**Rollback plan**: If provisional combat power proves misleading during playtests, supersede this ADR with a raw-delta-only or combat-derived scoring ADR while preserving the rule that UI must not recompute Attribute formulas and Combat power must not become hidden damage authority unless explicitly redesigned.

## Validation Criteria

- [ ] Valid attack-up equipment fixture produces `combat_power_before`, `combat_power_after`, and positive `combat_power_delta` with `mvp_provisional` and `display_only` labels.
- [ ] Defense/health contribution works through configured weights.
- [ ] Inactive/reserved/debug-only stats contribute 0 and do not appear in normal visible summary.
- [ ] Stage A config validation rejects no active player-facing positive combat power contribution and nonzero inactive/reserved weights unless explicitly debug-only in a later ADR.
- [ ] Child `effective_stat` / `effective_stat_pair` failures propagate to structured display proxy failure/unavailable.
- [ ] Display proxy never shows `0` as meaningful when computation is unavailable/failing.
- [ ] Valid growth-relevant rebuild without an external hint selects first non-zero visible delta by configured priority.
- [ ] If no visible non-zero delta exists, `primary_comparison_stat` is unavailable and `growth_salience` is `none` or `debug_only`.
- [ ] Visible/hidden delta rows include stat or pair ID, before value, after value, signed delta, reason, before/after versions or preview marker, display metadata key, and visibility classification.
- [ ] `AttributePreviewResult` display proxy carries preview marker/base versions, does not commit sources, does not advance authoritative versions, does not invoke committed sinks/signals, and does not enter committed domain event arrays.
- [ ] `AttributeUpdateResult` may include compact display proxy IDs/deltas/status but does not include full old+new snapshots by default.
- [ ] UI/growth presentation tests or walkthroughs prove stale/invalid/unavailable/preview-only states do not trigger normal positive celebration.
- [ ] Combat formula tests or static review prove damage/hit/crit/TTK do not read combat power/display proxy DTOs.
- [ ] Equipment legality tests or static review prove equip legality and modifier resolution occur before Attribute preview/result consumption.
- [ ] Save/load tests or review prove combat power/display proxy is not persisted as authoritative truth.
- [ ] DTO aliasing tests mutate constructor inputs and returned arrays and prove display proxy state is unchanged.
- [ ] Signal adapter tests prove callback order cannot change result, versions, display proxy status, or growth eligibility.

## GDD Requirements Addressed

| GDD Document | System | Requirement | How This ADR Satisfies It |
|-------------|--------|-------------|--------------------------|
| `design/gdd/character-attributes-system.md` | Character Attributes | Player-facing growth handoff requires `growth_reason`, `growth_salience`, `primary_comparison_stat`, `combat_power_before/after/delta`, visible/hidden deltas, and invalid/stale status. | Defines Character Attributes ownership of semantic display proxy DTOs and required fields while keeping presentation in UI/growth systems. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Recommended `combat_power` formula is MVP provisional, display-only, and not damage authority. | Labels combat power as `mvp_provisional` / `display_only`, computes it through config/formula tests, and forbids Combat consumption as damage/TTK authority. |
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-08 requires deltas to include before, after, signed delta, stat/pair ID, reason, and version or preview marker. | Makes visible/hidden delta rows carry the AC-08 fields directly or through parent metadata. |
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-15 requires compact event/result payloads and no full old+new snapshots by default. | Allows compact display proxy IDs/deltas/status in result/event payloads while keeping current truth in snapshot/query APIs. |
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-16 requires positive equipment or level changes to expose growth reason, salience, primary comparison stat, visible summary, and combat power delta if enabled. | Defines accepted authoritative growth handoff fields and deterministic primary stat fallback for valid growth-relevant rebuilds without hints. |
| `design/gdd/character-attributes-system.md` | Character Attributes | AC-17 requires automated formula/contract evidence for combat power and growth fixture portions. | Adds ADR-0014-specific validation criteria compatible with ADR-0012 formula-only GUT tests. |
| `design/gdd/character-attributes-system.md` | Character Attributes | UI/presentation handoff says UI owns layout and must not show invalid/stale data as normal growth. | Separates semantic proxy ownership from UI rendering and forbids stale/invalid/unavailable celebration. |
| `design/gdd/character-attributes-system.md` | Character Attributes | Implementation-blocking ADR item 9: combat power/main stat display proxy ownership. | Directly resolves the ownership prerequisite. |

## Related Decisions

- `design/gdd/character-attributes-system.md`
- `docs/architecture/adr-0006-attribute-data-representation-and-stat-id-typing.md`
- `docs/architecture/adr-0007-attribute-snapshot-query-api-shape-and-read-only-enforcement.md`
- `docs/architecture/adr-0008-attribute-event-signal-contract-and-scene-tree-independent-core.md`
- `docs/architecture/adr-0009-attribute-atomic-source-update-and-transaction-model.md`
- `docs/architecture/adr-0010-attribute-save-load-persistence-boundary-base-current-modifier-sources.md`
- `docs/architecture/adr-0011-attribute-fixture-config-loading-strategy-mvp-provisional-values.md`
- `docs/architecture/adr-0012-attribute-formula-only-gut-test-strategy-without-scene-tree-ui-autoload.md`
- `docs/architecture/adr-0013-attribute-godot-resource-duplication-shared-reference-policy-if-tres-resources-are-used.md`
- `docs/registry/architecture.yaml` — should be updated with growth display proxy contract, display proxy authority, and forbidden display-proxy misuse patterns after this ADR.
