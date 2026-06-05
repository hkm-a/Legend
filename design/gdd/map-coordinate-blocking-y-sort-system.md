# 地图坐标 / 阻挡 / Y-sort 系统

> **Status**: In Review
> **Author**: hkm + Claude Code Game Studios
> **Last Updated**: 2026-06-03
> **Implements Pillar**: Primary — 传奇骨架，现代皮肤; Supports — 稳刷不断流 / 爆装有戏
> **Quick reference** — Layer: `Foundation` · Priority: `MVP` · Key deps: `OpenMir2 行为映射 Spike`
> **Creative Director Review (CD-GDD-ALIGN)**: CONCERNS accepted 2026-06-03 — evidence gates and MVP provisional scope tightened after review.

## Overview

地图坐标 / 阻挡 / Y-sort 系统是 Phase 1 离线刷怪爆装切片的基础空间合同层：它定义地图逻辑格、世界/屏幕坐标关系、可通行与不可通行判断、对象占位语义，以及 2D/2.5D 表现中角色、怪物、地面物和地图元素的 Y-sort 读取规则。该系统不决定 Godot 的具体实现架构，也不发明未经 OpenMir2 行为映射 Spike 确认的移动速度、攻击距离或拾取距离数值；它的职责是产出后续点击移动、目标选择、怪物生成、战斗距离、掉落落点、拾取合法性和地图表现都能引用的 `Map Coordinate Contract` 与 `Map Blocking Contract`。没有这套合同，后续系统会各自解释“这一格能不能站、这个对象挡不挡路、这个物品落在哪里、谁应该画在谁前面”，从而破坏“传奇骨架，现代皮肤”的底层可信感。

## Player Fantasy

玩家不会直接操作坐标、阻挡或 Y-sort 系统，但应当持续感到这个世界的空间规则可信、熟悉且清晰。角色、怪物、障碍物和地面掉落之间的站位关系，应保留经典传奇的判断直觉：哪里能走、哪里被挡、谁在前谁在后，都不需要玩家反复猜测。现代化客户端的价值不是改变这套空间骨架，而是让它更稳定、更易读，让玩家把注意力放回刷怪、爆装、拾取和变强的循环上。

## Detailed Rules

### Core Rules

#### 1. Logical Grid Authority Rule

The map uses a discrete logical grid as the authoritative space for gameplay. Movement legality, blocking, occupancy, monster spawn points, combat distance inputs, drop landing cells, pickup legality, and target spatial queries must reference logical grid coordinates instead of raw pixel positions.

MVP design intent is that one logical grid cell corresponds to one OpenMir2 map cell. The exact source data origin, axis orientation, cell dimensions, and projection details are **TBD from OpenMir2 source evidence** and must not be invented in this GDD.

#### 2. Coordinate Space Separation Rule

The system recognizes three coordinate spaces:

| Coordinate Space | Purpose | Authority |
|---|---|---|
| Logical Grid Coordinate | Gameplay rules, occupancy, blocking, distance inputs, spawn/drop/pickup legality | Authoritative for gameplay |
| World / Render Position | Visual placement of actors, drops, props, effects, and Y-sort anchors | Derived from logical grid |
| Screen / Input Position | Mouse input and camera-space interaction | Converted into candidate logical grid or object query |

World and screen coordinates may be used for presentation and input conversion, but they must not override logical grid state.

#### 3. OpenMir2 Evidence Gate Rule

Rules in this GDD are classified as either **OpenMir2-derived** or **MVP provisional**. A rule may only be treated as OpenMir2-derived after the OpenMir2 behavior mapping Spike reaches E3/E4 source evidence for that behavior. Until then, the rule is an MVP provisional implementation contract for the offline Godot slice and must not be described as source-accurate.

This evidence gate applies especially to single-cell actor occupancy, movement reservation timing, actor swap rejection, dropped-item same-cell stacking, dropped-item non-blocking movement, drop-on-death-cell behavior, Y-sort type rank order, coordinate origin, axis direction, projection, diagonal movement, and distance semantics. If later E3/E4 evidence conflicts with a provisional rule, this GDD must be revised before implementation stories are generated or marked ready.

#### 4. MVP Implementation Readiness Matrix Rule

Phase 1 may proceed with a synthetic test map or technical-slice implementation only when each spatial rule is explicitly classified. Provisional rules may be implemented for the offline Godot slice, but they must not be described as OpenMir2-authentic, source-derived, or final behavior and must remain revisable if E3/E4 source evidence later conflicts.

| Rule Area | Current Classification | Phase 1 Technical Slice May Implement? | Final / Source-Authentic Gate |
|---|---|---:|---|
| Logical grid as gameplay authority | MVP provisional contract | Yes | Confirm OpenMir2 coordinate semantics before source-authentic claim. |
| One project cell maps to one OpenMir2 map cell | MVP provisional assumption | Yes for synthetic maps | E3/E4 OpenMir2 map-cell evidence required. |
| Player and ordinary monsters are single-cell blockers | MVP provisional assumption | Yes | E3/E4 actor occupancy evidence or approved intentional divergence. |
| Source occupied + target reserved movement | MVP provisional safeguard | Yes | E3/E4 movement timing evidence or approved intentional divergence. |
| Actor swap rejection | MVP provisional safeguard | Yes | E3/E4 same-tick movement / occupancy evidence required. |
| Dropped items do not block actor movement | MVP provisional assumption | Yes | E3/E4 ground-item movement evidence or approved intentional divergence. |
| Same-cell item capacity for pickup-complete Phase 1 | MVP provisional technical-slice rule: one dropped item per logical cell (`same_cell_item_capacity = 1`) | Yes for synthetic-map pickup-complete slice | E3/E4 item stacking evidence or approved drop/pickup GDD may revise. |
| Multiple dropped items sharing one cell | Evidence-gated / future or debug-only exception | No for pickup-complete Phase 1; allowed only in malformed-state tests or future rules | E3/E4 item stacking evidence or approved drop/pickup GDD decision. |
| Drop-on-death-cell placement | MVP provisional assumption | Yes for synthetic maps if the cell is item-placeable and under capacity | E3/E4 drop landing evidence or drop GDD fallback decision. |
| Pickup spatial candidate lookup | MVP provisional technical-slice rule for capacity-1 cells; evidence-gated for source authenticity | Yes for synthetic-map pickup-complete slice | E3/E4 pickup evidence or approved drop/pickup GDD. |
| Pickup distance and pickup commit legality | Evidence-gated / downstream-owned | Yes only as a local synthetic-map same-cell pickup rule if explicitly story-scoped; source-authentic completion remains blocked | E3/E4 pickup evidence or approved drop/pickup GDD. |
| Diagonal movement, corner-cutting, and distance metric | Evidence-gated | No for movement/combat/AI completion | E3/E4 OpenMir2 evidence or approved intentional divergence. |
| Y-sort type rank order | MVP provisional presentation contract | Yes, with readability validation | Rendering ADR and drop readability evidence. |

Implementation stories must state which matrix rows they rely on. A story may use provisional rows for a technical slice, but it must remain blocked for source-authentic completion if any relied-on row is still evidence-gated, and blocked for Phase 1 completion if the row's Phase 1 gate says candidate/debug only.

#### 5. Phase 1 Provisional Playable Contract Rule

This GDD may support a Phase 1 synthetic-map technical slice before OpenMir2 evidence reaches E3/E4, but only under an explicitly labeled **MVP provisional playable contract**. This contract is implementation-usable for local Godot slice stories and is **not** source-authentic OpenMir2 behavior. Any story using it must label the relied-on rules as `MVP provisional` and must not claim OpenMir2 compatibility until the corresponding evidence gate passes.

The minimum Phase 1 provisional playable contract is:

| Contract Field | Phase 1 Provisional Value | Source-Authentic Gate |
|---|---|---|
| Map scope | One loaded synthetic test map with valid logical bounds and matching cell data. | OpenMir2 map/cell evidence or resource/map conversion Spike. |
| Coordinate authority | Gameplay uses logical grid cells; render/input positions are derived or converted. | OpenMir2 coordinate origin, axis, cell, and projection evidence. |
| Actor occupancy | Player and ordinary monsters are single-cell blocking actors. | OpenMir2 actor occupancy evidence or approved divergence. |
| Movement contention | Source remains occupied; destination is reserved before commit. | OpenMir2 movement timing evidence or approved divergence. |
| Item movement blocking | Dropped items do not block actor entry. | OpenMir2 ground-item movement evidence or approved divergence. |
| Phase 1 item capacity | `same_cell_item_capacity = 1` for pickup-complete stories. | OpenMir2 item stacking evidence or drop/pickup GDD may revise. |
| Phase 1 pickup candidate order | `stable_drop_sequence` ascending, then `stable_item_instance_id`, used only if a malformed/future state contains multiple candidates. | OpenMir2 pickup ordering evidence or drop/pickup GDD. |
| Phase 1 pickup completion | A local synthetic-map story may complete same-cell pickup only when exactly one item candidate exists, the item is still available, and downstream inventory capacity is stubbed/approved for that story. | Pickup GDD and OpenMir2 pickup evidence. |
| Drop fallback | If the death cell is illegal or at capacity, return `drop_fallback_required`; do not delete, reroll, or silently move the reward. | Drop GDD fallback policy and OpenMir2 drop evidence. |
| Y-sort tie-break | Anchor Y, object type rank, stable ID; no scene-tree insertion order. | Rendering ADR and source/feel validation. |

This contract is the only path from this GDD to early implementation stories before ADR completion. If a story needs behavior outside this table, it must either wait for the relevant ADR/downstream GDD or explicitly record an approved intentional divergence.

#### 6. Map Coordinate Contract Rule

Every playable map must expose a `Map Coordinate Contract` that defines:

- The map ID or map data source.
- The legal logical grid bounds.
- The intended relationship between OpenMir2 map cells and project logical grid cells.
- The rule for converting a logical grid cell into a stable render anchor.
- The rule for converting a screen/input point into a candidate logical grid cell when possible.
- The failure behavior for invalid, ambiguous, or out-of-bounds coordinates.

If coordinate conversion cannot resolve a valid candidate cell, the query must return an explicit failure reason instead of silently choosing an unrelated cell.

#### 7. Out-of-Bounds Fail-Closed Rule

Any logical coordinate outside the current map bounds is treated as invalid for movement, spawn, drop, pickup, combat-distance input, and target-position queries. Out-of-bounds cells must fail closed and return an explicit `out_of_bounds` reason.

Out-of-bounds is distinct from missing map data. A query may only return `out_of_bounds` after the map bounds are known and valid. If map bounds, cell data, or required blocking data are unavailable, the query must return `unknown_or_unloaded` instead of masking the data failure as ordinary out-of-bounds.

#### 8. Map Blocking Contract Rule

Every playable map must expose a `Map Blocking Contract` that separates:

- **Static blocking**: terrain, walls, buildings, impassable map objects, and other map-authored passability data.
- **Dynamic blocking**: runtime occupancy by player, monsters, NPCs, or other blocking actors.
- **Item occupancy**: dropped items that have a logical grid cell but do not block actor movement in MVP.
- **Visual-only obstruction**: props or overlays that may affect readability or Y-sort but do not change gameplay passability.

MVP static passability may be represented as `passable / blocked`, but the contract must allow future expansion into blocking categories such as actor passability, projectile passability, safe-zone rules, or visual-only obstruction.

#### 9. Structured Query Result Schema Rule

Spatial queries must return structured results, not only true/false. A query result must expose at least:

| Field | Meaning |
|---|---|
| `status` | `allowed`, `blocked`, `unavailable`, or `unresolved`. |
| `primary_reason` | One canonical machine-readable reason selected by priority. |
| `secondary_reasons` | Optional debug-only reasons detected during evaluation. |
| `query_context` | One canonical query context enum such as `actor_entry`, `item_placement`, `pickup_spatial_candidate`, `reservation_create`, `reservation_commit`, `coordinate_conversion`, or `presentation_debug`; a single query must not claim multiple primary contexts. |
| `cell_facts` | Optional immutable snapshot facts such as actor occupant ID, reservation ID, item count/list, stable drop sequence, and Y-sort anchor state. The snapshot must not expose mutable runtime references. |
| `retry_hint` | Optional downstream hint such as `never_same_state`, `wait_or_backoff`, `repath_required`, `fallback_required`, or `blocked_until_evidence`. |

The canonical MVP reason registry is:

| Reason | Applies To | Status | Meaning |
|---|---|---|---|
| `walkable` | Actor entry / generic success | `allowed` | The cell is legal for the requested actor-entry context. |
| `item_placeable` | Item placement success | `allowed` | A dropped item may be placed in the cell for the requested item context. |
| `invalid_coordinate` | Coordinate conversion | `unresolved` | The input could not be resolved to a logical coordinate. |
| `unknown_or_unloaded` | Map data / cell data | `unavailable` | Required map, bounds, blocking, or cell data is missing or unavailable. |
| `out_of_bounds` | Known map bounds | `blocked` | The resolved coordinate lies outside valid map bounds. |
| `blocked_by_static_map` | Actor entry, spawn, item placement where applicable | `blocked` | Static map data forbids the requested context. |
| `blocked_by_actor` | Actor entry, actor placement, reservation | `blocked` | Another blocking actor occupies the cell. |
| `reserved` | Actor entry, actor placement, reservation | `blocked` | Another active reservation conflicts with the requested claim. |
| `actor_already_reserved` | Reservation creation | `blocked` | The requesting actor already has an active movement reservation. |
| `reservation_owner_mismatch` | Reservation commit | `blocked` | The target reservation does not belong to the committing actor/request. |
| `source_occupancy_lost` | Reservation commit | `blocked` | The moving actor no longer occupies the expected source cell. |
| `item_capacity_full` | Item placement | `blocked` | Item occupancy policy does not allow another item in the cell. |
| `no_item_present` | Pickup spatial query | `blocked` | The queried cell contains no dropped item candidate. |
| `pickup_rule_unresolved` | Pickup spatial query | `unresolved` | Pickup distance, adjacency, or selection rule is not yet approved. |
| `pickup_selection_order_unresolved` | Pickup spatial query | `unresolved` | Multiple item candidates exist but deterministic selection order is not approved. |
| `pickup_distance_failed` | Pickup spatial query | `blocked` | The approved pickup-distance rule rejects the actor/item relationship. |
| `item_not_available` | Pickup commit | `blocked` | The item was removed, expired, or otherwise unavailable before commit. |
| `drop_fallback_required` | Drop placement | `unresolved` | Initial placement failed and the drop system must apply an approved fallback policy. |
| `pickup_candidate_available` | Pickup spatial candidate query | `allowed` | One or more dropped item candidates exist in the queried cell before downstream pickup legality checks. |
| `pickup_commit_blocked_until_rule_approved` | Pickup completion / story gate | `unresolved` | Pickup completion is blocked because distance, selection order, ownership, inventory, or commit rules are not approved. |
| `cross_map_movement_undefined` | Movement / transition | `unresolved` | Cross-map movement or transition is not defined for MVP. |
| `invalid_y_sort_anchor` | Presentation / debug | `unresolved` | The object lacks a valid Y-sort anchor. |
| `invalid_y_sort_rank` | Presentation / debug | `unresolved` | The object's type lacks a configured sortable rank in the active Y-sort policy. |
| `invalid_stable_sort_id` | Presentation / debug | `unresolved` | The object lacks a comparable stable ID or mixes incomparable ID types in one sorted set. |
| `no_movement_requested` | Actor entry / reservation | `allowed` | The requester targets its own current source cell, so no movement reservation is created. |

`occupied_by_item` is not a primary blocking reason for actor movement in MVP. It may appear only as a secondary reason or as `cell_facts.item_count > 0`, because dropped items do not block actor entry.

Minimum MVP `cell_facts` snapshot fields are: `map_id`, `cell`, `static_passability_state`, `actor_occupant_id`, `reservation_id`, `reservation_owner_actor_id`, `item_count`, `item_instance_ids`, `stable_drop_sequences`, `y_sort_anchor_valid`, and `y_sort_key_preview` when applicable. Unknown or unavailable fields must be omitted or explicitly null with `primary_reason = unknown_or_unloaded`; they must not be fabricated from visual nodes.

Downstream systems may use the reason and retry hint to decide whether to retry, search a nearby cell, show feedback, wait/backoff, or fail the action. They must not invent local reason names for this system's query outputs without a GDD revision.

#### 10. Map Authoring and Validation Contract Rule

Every playable map must expose enough authoring metadata for QA and future level/map GDDs to validate that legal cells form a readable loot-loop play space. This GDD does not design the Phase 1 map layout, but map data or validation fixtures must support:

- `player_start_cell` for reachability validation.
- `intended_play_region` or `critical_loop_region` for Phase 1 test maps.
- `spawn_allowed_region` and optional `no_spawn_region` tags.
- `item_placeable_region` and `drop_readability_region` tags.
- `visual_obstruction_region` tags for props, foregrounds, canopies, or overlays that may affect readability.
- Y-sort anchor metadata for sortable map props and dynamic world objects.

Validation tools or debug evidence must be able to report unreachable passable cells, spawn cells unreachable from the player start, item-placeable cells that are not drop-readable, visual-only obstructions overlapping drop-readable or critical-loop cells, missing Y-sort anchors, and visual/logical passability contradictions. For the Phase 1 synthetic-map fixture, validation must also prove at least one complete loot-loop path: `player_start_cell -> critical_loop_region -> spawn_allowed_region -> monster_death_cell -> item_placeable/drop_readable cell -> pickup candidate query`. Concrete pacing, route shape, monster density, and encounter layout still belong to a later Phase 1 map or level GDD, but this GDD requires the minimum fixture to demonstrate that a single dynamic blocker cannot fully seal the critical loop path and that at least one drop-readable item-placement cell remains usable.

#### 11. Actor Occupancy Rule

In MVP, the player and ordinary monsters are single-cell blocking actors. A blocking actor occupies exactly one logical grid cell and prevents other blocking actors from entering that cell.

Multi-cell actors, bosses, large static objects with gameplay footprint, corpses that block movement, and temporary skill blockers are out of MVP scope unless a later GDD explicitly adds them.

#### 12. Movement Reservation Rule

When a blocking actor begins a movement claim, the source cell remains `occupied_actor` until the movement commits, and the destination cell becomes `reserved` after a successful reservation. On commit, the source cell is released and the destination cell becomes `occupied_actor`. On cancel, the reservation is released and the actor remains in the source cell.

This is an **MVP provisional implementation safeguard**, not yet an OpenMir2-derived rule. It prevents two actors from claiming the same target cell in the offline Godot slice and preserves a deterministic occupancy contract for future movement, AI, and networking work. If OpenMir2 source evidence proves different occupancy timing, this movement contract must be revised or explicitly marked as an intentional divergence before implementation stories are generated.

#### 13. Reservation Ownership and Lifecycle Rule

A movement reservation is an authoritative runtime record owned by the map-space contract, not by arbitrary actor nodes, animation callbacks, UI, or AI code. Every reservation must include at minimum:

| Field | Meaning |
|---|---|
| `reservation_id` | Stable runtime identifier unique within the active map. |
| `owner_actor_id` | Actor that requested and may commit/cancel the reservation. |
| `map_id` | Active map containing source and target cells. |
| `source_cell` | Last committed occupied cell expected at commit time. |
| `target_cell` | Reserved destination cell. |
| `purpose` | `movement` for MVP; future purposes require GDD revision. |
| `created_sequence` | Deterministic update sequence or tick used for ordering/debugging. |
| `max_age_policy` | Runtime-only stale cleanup rule; exact tick count belongs to ADR/implementation. |

MVP reservations are runtime-only and must not be saved. Map unload clears all active reservations for that map. Save/load or restore logic rebuilds only committed actor/item occupancy from persistent logical cells unless a future persistence GDD adds movement-in-progress restoration.

Only the reservation owner may commit or cancel its movement reservation. A commit is valid only when the reservation still belongs to the same actor/request, the actor still occupies `source_cell`, `target_cell` is still in bounds and actor-passable, and no other blocking actor occupies `target_cell`. If any condition fails, commit fails with a canonical structured reason, the target reservation is cleared unless a later movement GDD explicitly retains it, and the actor remains in or is restored to its last valid committed source cell.

Reservations must be released when the owner actor dies, despawns, loses map registration, transfers maps, cancels movement, fails commit, or is removed during map unload. Stale reservations must be cleaned before new movement, spawn, or actor-placement queries are resolved for the affected map.

#### 14. Authoritative Update Ordering Rule

Occupancy and reservation mutations must be serialized through a deterministic authoritative update pipeline. Implementation ADRs may choose the exact Godot process hook or service architecture, but the behavior must preserve this order or an equivalent documented ordering:

1. Validate active map data and reject unavailable maps.
2. Clean invalid actors, removed items, expired/stale reservations, and map-unload state.
3. Apply death, despawn, transfer, and removal releases for actor/item occupancy.
4. Collect movement and placement intents for the current authoritative update.
5. Resolve competing reservation/placement requests in deterministic action order.
6. Commit or cancel due movement reservations atomically.
7. Apply item placement, item removal, and pickup spatial state changes.
8. Mark affected Y-sort inputs dirty and publish debug/query snapshots.

If two actors request the same target cell in the same authoritative update, deterministic ordering decides the winner. The MVP provisional ordering source is request sequence, then stable actor ID. Frame order, scene-tree insertion order, and arbitrary signal callback order must not decide gameplay occupancy. Implementation ADR may refine the source of request sequence and stable actor ID, but it must preserve deterministic replayable ordering for tests.

#### 15. Item Occupancy Rule

In MVP, dropped items have logical grid cells, participate in pickup queries and Y-sort, and may share a cell with actors. Dropped items do **not** block actor movement.

For pickup-complete Phase 1 stories, a logical grid cell may contain at most one dropped item: `same_cell_item_capacity = 1`. This is an **MVP provisional technical-slice rule**, chosen to close the 30-second loot-loop pickup path without inventing OpenMir2 item stacking behavior. If a second item would be placed in the same cell, item placement fails with `item_capacity_full` and the drop system must receive `drop_fallback_required` rather than silently stacking, deleting, rerolling, or moving the reward.

Multiple dropped items in one cell are treated as malformed-state, debug-only, or future-rule cases until OpenMir2 evidence or the drop/pickup GDD approves a different capacity. If such a state is inspected, the system must expose item instance IDs, item count, per-item logical cell, stable drop sequence, and deterministic listing order. The provisional candidate order is `stable_drop_sequence` ascending, then `stable_item_instance_id`. If those ordering fields are unavailable, the query must return `pickup_selection_order_unresolved` and must not complete pickup or remove any item.

#### 16. Loot Readability Floor Rule

Dropped items are gameplay-spatial objects and primary reward feedback. A valid dropped item must not become completely invisible or unidentifiable in MVP because of actor overlap, same-cell stacking, visual-only props, Y-sort tie-breaks, missing labels, or foreground decoration.

This system does not own loot VFX, labels, audio, item value, or pickup priority, but it must preserve a readability floor for downstream systems:

- Dropped items sharing a cell remain individually queryable by stable item instance ID.
- A cell containing multiple dropped items exposes item count and item list facts to debug/presentation systems.
- If a dropped item shares a logical cell or anchor with an actor, gameplay legality may remain unchanged, but rendering/drop presentation must provide a visible affordance such as label, highlight, outline, elevated icon, loot beam, or equivalent before the loot-loop slice is considered presentation-complete.
- High-value or high-priority dropped items must be eligible for stronger presentation priority in downstream drop/pickup/rendering systems.
- Spatial placement failure must return a structured reason to the drop system; this system must not destroy, reroll, downgrade, or silently discard a generated reward.
- If an initial death-cell drop is illegal, the drop system must apply an approved fallback policy before the reward is considered resolved. Until that fallback policy exists, gameplay that can generate drops on illegal cells is incomplete.

#### 17. Logical Position vs. Visual Offset Rule

Actors, monsters, dropped items, and props may use visual offsets, animation offsets, or interpolation for presentation. Gameplay queries must use the object's logical grid coordinate and declared anchor, not transient sprite position.

Combat distance, pickup distance, spawn legality, blocking, and movement legality must not be calculated from current sprite pixels.

#### 18. Neighbor and Direction Rule

The contract must support orthogonal and diagonal neighbor concepts for downstream movement, combat, pickup, and AI systems. Whether diagonal movement is allowed, whether diagonal corner-cutting is allowed, and which distance formula treats diagonal adjacency as adjacent are **TBD from OpenMir2 source evidence**.

Until confirmed, downstream systems must not invent their own incompatible neighbor or distance rules.

#### 19. Unified Distance Input Rule

This system does not define attack range, pickup range, spawn radius, or movement speed. It only provides the logical grid coordinates used by those systems. Combat, pickup, spawn, and AI systems must reference a shared future `Map Distance Contract` rather than each defining separate distance semantics.

The specific formula and thresholds are **TBD from OpenMir2 source evidence** or later approved system GDDs.

#### 20. Spawn Legality Rule

Monster spawn systems must query this system before placing an actor. A spawn candidate is legal only if:

1. The coordinate is inside map bounds.
2. The static map contract allows actor placement.
3. No blocking actor occupies the cell.
4. No active reservation conflicts with the spawn.
5. Any spawn-system-specific requirements pass.

Spawn probability, spawn tables, spawn radius, respawn timing, and monster selection are owned by the monster spawn system, not this system.

#### 21. Drop Landing Rule

Drop systems must query this system before placing a dropped item. In MVP, a drop landing candidate is legal only if:

1. The coordinate is inside map bounds.
2. The static map contract allows item placement.
3. The cell is not `unknown_or_unloaded`.
4. The target cell is under the active item-capacity rule; for pickup-complete Phase 1, `item_count < same_cell_item_capacity` and `same_cell_item_capacity = 1`.

By default, a dropped item may land on the monster's death cell if that cell is legal for item placement and under capacity. If that candidate is illegal or at capacity, the drop system owns the fallback search strategy and this system returns `drop_fallback_required`. Search radius and fallback order are **TBD from OpenMir2 source evidence** or the drop system GDD.

#### 22. Pickup Spatial Query Rule

Pickup systems must query this system for the player's logical cell, the item's logical cell, and any blocking or invalid coordinate conditions. This system does not decide pickup distance, ownership, inventory capacity, or auto-pickup behavior.

For the Phase 1 provisional synthetic-map pickup-complete story only, pickup may complete when the actor and the single available item candidate share the same logical cell and downstream inventory acceptance is stubbed or approved by the story. This same-cell completion rule is not OpenMir2-authentic. Source-authentic pickup distance, adjacency, ownership, inventory, and commit legality remain **TBD from OpenMir2 source evidence** and belong to the pickup GDD.

#### 23. Y-sort Anchor Rule

Every world object that participates in map visual ordering must expose a stable Y-sort anchor. For actors and monsters, the anchor represents the standing point / foot point. For dropped items, it represents the item's ground anchor. For static props, it represents the designer-authored base point or visual sorting anchor.

Y-sort must use the anchor's world/render position derived from the logical grid and visual offset contract, not the sprite's geometric center.

#### 24. Y-sort Visual-Only Rule

Y-sort affects visual ordering only. It must not change blocking, movement legality, target validity, attack distance, pickup distance, spawn legality, or drop legality.

If an object is visually hidden or overlapped, target selection and UI feedback rules decide how the player interacts with it.

#### 25. Stable Y-sort Tie-Breaker Rule

If multiple sortable objects share the same Y-sort anchor value, sorting must remain deterministic. MVP uses:

1. Object type layer.
2. Stable instance ID or spawn sequence.

The exact object type layer order is:

1. Ground-level dropped items.
2. Small world decorations that participate in map sorting.
3. Blocking actors: player, monsters, NPCs.
4. Large sortable map props.
5. Overlay / canopy / roof-like visual layers, if present.

Complex overlay, roof, and canopy handling is not required in MVP; this GDD only preserves the contract field for later art and map rendering work.

The object type rank order is also **MVP provisional** and must be validated against drop readability. If dropped items become hidden under actors or props, rendering, pickup presentation, item highlights, outlines, labels, or loot beams may improve visibility while gameplay coordinates and pickup legality remain unchanged.

#### 26. Data Authority Rule

Logical map data is authoritative for gameplay passability, occupancy, and spatial queries. Visual tiles, sprites, animation frames, decorative layers, and physics/collision helpers must not contradict logical map state without an explicit ADR or design revision.

#### 27. Performance Guardrail Rule

This system is expected to be queried by movement, AI, spawn, combat, drop, pickup, target selection, rendering, and debug tools. It must therefore define bounded runtime behavior even before implementation ADRs choose data structures.

MVP guardrails are:

- Normal gameplay must not scan the full map every frame for blocking, occupancy, item, or Y-sort queries.
- Spatial queries should be event-driven, command-driven, or batched through the authoritative update rather than freely polled by every downstream system every render frame.
- Pathfinding, spawn fallback, target search, and drop fallback systems must declare bounded candidate counts or budgets in their own GDDs before implementation.
- Query APIs must return structured reasons without requiring log emission as part of normal semantics.
- Coordinate failure logs and conflicting-reason logs must be rate-limited or aggregated; repeated identical failures must not spam logs every frame.
- Debug overlays must be disabled by default, debug-only, and bounded to selected cells, visible regions, or explicitly profiled debug modes. Full-grid overlays must not be treated as normal gameplay performance.
- Y-sort dirty events must be coalesced and refreshed in a controlled update phase; static sortable objects must not be resorted every frame unless their sort-relevant data changes.
- Same-cell item inspection must support bounded or truncated debug display while preserving the total item count.
- Performance benchmark evidence must run with debug overlays and unbounded diagnostic logging disabled unless the benchmark specifically measures debug tooling.

Implementation stories must expose debug/profiling evidence sufficient to prove these guardrails. Minimum counters or equivalent evidence are: `spatial_queries_per_update`, `cells_scanned_per_query`, `cells_scanned_per_update`, `sort_inputs_marked_dirty`, `sort_inputs_refreshed`, `coordinate_failure_logs_emitted`, `conflicting_reason_logs_emitted`, and `debug_overlay_cells_drawn`. Load-time or offline validation may scan a full map, but normal gameplay runtime must not repeat full-map validation in the frame/update loop.

Exact numeric budgets belong to ADR, profiling, and later performance tests, but implementation stories must provide evidence that these guardrails are respected.

#### 28. Implementation Boundary and ADR Prerequisites Rule

This GDD does not decide Godot node structure, TileMapLayer usage, collision layer strategy, resource format, data cache layout, pathfinding algorithm, or Y-sort implementation method. Those decisions belong to architecture and ADR work after the GDD is approved.

Implementation must not treat deprecated `TileMap` as an equal option without version-verified justification. Godot 4.6.3 implementation ADRs must verify post-cutoff engine behavior before coding, including:

- Map data representation: TileMapLayer, custom Resources, imported arrays, or other data source.
- Logical map schema: bounds, passability categories, item placement flags, visual obstruction metadata, Y-sort anchors, and stable IDs.
- Screen/input → world/render → logical conversion pipeline, including camera/viewport/UI interaction and ambiguous input failure.
- Deterministic Y-sort strategy capable of honoring `anchor_y`, object type rank, and stable ID without relying on scene-tree insertion order.
- Authoritative occupancy mutation pipeline in Godot update/tick terms.
- Typed GDScript query result schema and reason enum family suitable for unit tests.
- Debug evidence output method without requiring player-facing UI ownership.

### States and Transitions

#### Map Cell State Dimensions

A logical grid cell may have multiple state dimensions at once. For example, a cell may be statically passable, contain a dropped item, and also be visually covered by a sortable prop. The dimensions below should be read as composable state facets rather than one mutually exclusive enum.

| State / Facet | Meaning | Entered When | Exited When | Gameplay Effect |
|---|---|---|---|---|
| `unloaded` | Map data is unavailable or not initialized. | Map not loaded, data missing, or map transition in progress. | Map data loads and validates. | Fail closed for movement, spawn, drop, pickup, and targeting. |
| `out_of_bounds` | Coordinate is outside legal map bounds. | Query coordinate is outside the map range. | Query changes to an in-bounds coordinate. | Always illegal for gameplay placement or movement. |
| `static_passable` | Static map data allows actor movement or placement for the requested context. | Map data marks the cell as passable. | Static map data changes, if ever. | Candidate may continue to dynamic checks. |
| `static_blocked` | Static map data forbids actor movement or placement. | Terrain, wall, building, or authored blocker marks the cell blocked. | Static map data changes, if ever. | Actor movement/spawn is illegal. |
| `occupied_actor` | A blocking actor currently owns the cell. | Player, monster, or NPC commits occupancy. | Actor moves away, despawns, dies and releases occupancy, or transfers maps. | Other blocking actors cannot enter. |
| `reserved` | A system has temporarily claimed the cell. | Movement, spawn, or another placement process reserves it. | Commit or cancel. | Other blocking claims fail until released. |
| `occupied_item` | One or more dropped items occupy the cell. | Drop system places item(s). | Item is picked up, expires, or is removed. | Does not block actor movement in MVP; pickup queries may target it. |
| `visual_only` | A visual object exists that may affect sorting/readability but not gameplay. | Map prop, decoration, or overlay is registered. | Prop removed or map unloaded. | May affect Y-sort or rendering only. |

#### Actor Occupancy Lifecycle

| State | Meaning | Entered When | Exited When |
|---|---|---|---|
| `not_registered` | Object exists but has no map-space registration. | Object is created before spawn placement. | Object receives a legal map cell. |
| `registered_idle` | Object occupies a logical cell and has no active move claim. | Spawn or movement commit completes. | Movement, despawn, death cleanup, or transfer begins. |
| `reserving_destination` | Object requests a target cell but has not claimed it yet. | Movement or placement request starts. | Reservation succeeds or fails. |
| `moving_reserved` | Source cell remains occupied and target cell is reserved. | Reservation succeeds and movement begins. | Movement commits or cancels. |
| `committing_cell` | Occupancy switches from source cell to destination cell. | Movement reaches its commit point. | Source releases and destination becomes occupied. |
| `removed` | Object no longer participates in map-space rules. | Death cleanup, pickup, despawn, or map unload occurs. | Object is re-registered if respawned or restored. |

#### Blocking Query Flow

| Step | Input | System Action | Output |
|---|---|---|---|
| 1. Normalize | Logical, world, or screen-derived coordinate | Resolve to a candidate logical grid coordinate when possible. | Candidate cell or `invalid_coordinate`. |
| 2. Bounds Check | Candidate cell | Check legal map bounds. | Continue or return `out_of_bounds`. |
| 3. Static Check | Candidate cell + context | Read map-authored passability for the requested context. | Continue or return `blocked_by_static_map`. |
| 4. Dynamic Check | Candidate cell + context | Check actor occupancy, item occupancy, and reservations. | Continue, return `blocked_by_actor` / `reserved`, or expose non-blocking item occupancy through `cell_facts`. |
| 5. Context Check | Candidate cell + requesting system | Apply context-specific legality supplied by the requesting system. | Legal or context-specific failure. |
| 6. Result | All checks | Return structured deterministic query result. | Downstream system decides retry, fallback, feedback, or failure. |

#### Y-sort Lifecycle

| State | Meaning | Entered When | Exited When |
|---|---|---|---|
| `not_sortable` | Object does not participate in map Y-sort. | UI, labels, pure logic objects, or non-map effects are created. | Object receives a valid Y-sort anchor. |
| `sortable_static` | Static map object contributes a stable sorting anchor. | Map loads sortable prop data. | Map unloads or prop is removed. |
| `sortable_dynamic` | Actor, item, or dynamic object contributes a sorting anchor. | Object registers into map space. | Object is removed or becomes non-sortable. |
| `sort_dirty` | Sorting input changed. | Object moves, spawns, despawns, changes anchor, or type layer changes. | Sorting is refreshed. |
| `sort_resolved` | Current visual ordering is deterministic for this update. | Sort refresh completes. | Next dirty event occurs. |

### Interactions with Other Systems

| System | Input to This System | Output from This System | Ownership Boundary |
|---|---|---|---|
| OpenMir2 行为映射 Spike | Source evidence for coordinates, blocking, movement legality, object occupancy, and map data semantics. | Provisional contracts and open questions that require evidence. | Spike owns evidence maturity and source references; this system owns design contracts based on that evidence. |
| 点击移动系统 | Candidate target cell, actor ID, movement context. | Passability result, blocking reason, reservation result, commit/cancel effect. | This system owns cell legality and occupancy; movement owns path choice, movement timing, and click fallback. |
| 交互目标 / 选择系统 | Screen/input position, candidate object positions, query context. | Candidate grid/object spatial data and Y-sort/readability references. | This system provides spatial facts; target selection owns final click priority and selection UX. |
| 怪物生成系统 | Spawn region, candidate cells, monster occupancy type. | Legal/illegal spawn result and blocking reason. | This system owns cell legality; spawn system owns tables, timing, and candidate selection. |
| 怪物 AI / 行为系统 | Current actor cells, requested move targets, occupancy queries. | Neighbor/candidate legality and reservation outcomes. | This system owns spatial legality; AI owns behavior choice and path/target decisions. |
| 基础战斗系统 | Attacker cell, target cell, line/distance query inputs. | Logical cell positions and shared distance-contract inputs. | This system owns coordinates; combat owns range formula, attack legality, and damage flow. |
| 掉落与拾取系统 | Death cell, item placement request, player/item cell query. | Legal item placement result, item occupancy, player/item grid positions. | This system owns placement legality; drop/pickup owns drop fallback, pickup distance, ownership, inventory checks, and item priority. |
| 地图表现 / 渲染系统 | Map visual layers, sortable object anchors, object type layers. | Stable Y-sort inputs and expected visual ordering contract. | This system owns sorting semantics; rendering/ADR owns Godot implementation. |
| 存档系统 | Map ID, actor cells, item cells, occupancy state. | Serializable spatial state fields. | This system defines spatial state; save/load owns persistence format and restore timing. |
| 资源 / 地图转换管线 Spike | Imported map data, source blocking metadata, anchor metadata. | Required coordinate/blocking fields and validation expectations. | This system defines required map contract fields; tools pipeline owns extraction/import. |
| QA / Debug Tools | Cell coordinate, object ID, query context. | Blocking reason, occupant list, reservation state, Y-sort anchor data. | This system exposes inspectable state; QA/tools own visualization. |
| Network / 最小协议系统 (Future) | Server-authoritative cells and correction events. | Client-side contract compatible with authoritative grid state. | Future networking owns authority and sync; this system preserves deterministic grid semantics. |

## Formulas

### 1. `in_bounds`

The `bounds_valid` and `in_bounds` formulas are defined as:

`bounds_valid = map_loaded AND map_width > 0 AND map_height > 0 AND cell_data_matches_dimensions`

`in_bounds = bounds_valid AND (0 <= cell_x < map_width) AND (0 <= cell_y < map_height)`

**Variables:**

| Variable | Symbol | Type | Range | Description |
|---|---|---|---|---|
| Map Loaded | `map_loaded` | bool | `{true, false}` | Whether the current map data is available for gameplay queries. |
| Map Width | `map_width` | int | `> 0` when valid | Number of logical-grid columns in the current map. |
| Map Height | `map_height` | int | `> 0` when valid | Number of logical-grid rows in the current map. |
| Cell Data Matches Dimensions | `cell_data_matches_dimensions` | bool | `{true, false}` | Whether loaded cell data matches the declared width and height. |
| Bounds Valid | `bounds_valid` | bool | `{true, false}` | Whether map bounds can be trusted for `in_bounds`. |
| Cell X | `cell_x` | int | unbounded input; valid when `0` to `map_width - 1` | Queried logical-grid X coordinate. |
| Cell Y | `cell_y` | int | unbounded input; valid when `0` to `map_height - 1` | Queried logical-grid Y coordinate. |

**Output Range:** `true` or `false`. If `bounds_valid = false`, gameplay queries fail closed with `unknown_or_unloaded`. If `bounds_valid = true` and `in_bounds = false`, movement, spawn, drop, pickup, combat-distance input, and target-position queries fail closed with `out_of_bounds`.

**Example:** If `map_loaded = true`, `map_width = 10`, `map_height = 5`, `cell_data_matches_dimensions = true`, and the queried cell is `(9, 4)`, then `bounds_valid = true` and `in_bounds = true`. If the queried cell is `(10, 4)`, then `in_bounds = false` and the reason is `out_of_bounds`. If `map_loaded = false`, then `bounds_valid = false` and the reason is `unknown_or_unloaded`, not `out_of_bounds`.

### 2. `actor_enterable`

The `actor_enterable` formula is defined as:

`actor_enterable = bounds_valid AND in_bounds AND terrain_actor_passable AND NOT target_occupied_by_other_actor AND NOT target_reserved_by_other_actor`

**Variables:**

| Variable | Symbol | Type | Range | Description |
|---|---|---|---|---|
| Bounds Valid | `bounds_valid` | bool | `{true, false}` | Whether map bounds and required cell data are available. |
| In Bounds | `in_bounds` | bool | `{true, false}` | Result of the `in_bounds` formula for the target cell. |
| Terrain Actor Passable | `terrain_actor_passable` | bool | `{true, false}` | Whether map data allows a blocking actor to stand in the cell. |
| Target Occupied By Other Actor | `target_occupied_by_other_actor` | bool | `{true, false}` | Whether the target cell is occupied by a blocking actor other than the requester. |
| Target Reserved By Other Actor | `target_reserved_by_other_actor` | bool | `{true, false}` | Whether the target cell is reserved by another actor's pending movement. |

**Output Range:** `true` or `false`. Dropped items do not appear in this formula because they occupy item-placement state but do not block actor movement in MVP.

**Example:** If `bounds_valid = true`, `in_bounds = true`, `terrain_actor_passable = true`, `target_occupied_by_other_actor = false`, and `target_reserved_by_other_actor = true`, then `actor_enterable = false` because the target cell is already reserved by another actor. If an actor queries its own current source cell, that request resolves through the no-movement rule and does not create a new reservation.

### 3. `item_placeable`

The `item_placeable` formula is defined as:

`item_placeable = bounds_valid AND required_item_placement_data_available AND in_bounds AND terrain_item_placeable AND item_capacity_available`

Where for the Phase 1 provisional playable contract:

`item_capacity_available = item_count < same_cell_item_capacity`

`same_cell_item_capacity = 1`

**Variables:**

| Variable | Symbol | Type | Range | Description |
|---|---|---|---|---|
| Bounds Valid | `bounds_valid` | bool | `{true, false}` | Whether map bounds and required item-placement data are available. |
| In Bounds | `in_bounds` | bool | `{true, false}` | Result of the `in_bounds` formula for the target cell. |
| Terrain Item Placeable | `terrain_item_placeable` | bool | `{true, false}` | Whether map data allows a dropped item to be placed on this cell. |
| Required Item Placement Data Available | `required_item_placement_data_available` | bool | `{true, false}` | Whether item-placement terrain data and item occupancy facts are loaded for this cell. If false, return `unknown_or_unloaded`. |
| Item Capacity Available | `item_capacity_available` | bool | `{true, false}` | Whether the cell's item-occupancy rule allows another dropped item. |
| Item Count | `item_count` | int | `>= 0` | Number of dropped item instances currently registered in the logical cell. |
| Same-Cell Item Capacity | `same_cell_item_capacity` | int | `1` for Phase 1 playable contract; future values require GDD revision | Maximum dropped item instances allowed in one logical cell for the active item-placement policy. |

**Output Range:** `true` or `false`. If `bounds_valid = false` or `required_item_placement_data_available = false`, the structured reason is `unknown_or_unloaded`, not ordinary static blocking or capacity failure. Actor occupancy and actor reservations are intentionally excluded unless OpenMir2 evidence later proves dropped items cannot be placed under actors.

**Example:** If `bounds_valid = true`, `required_item_placement_data_available = true`, `in_bounds = true`, `terrain_item_placeable = true`, `item_count = 1`, and `same_cell_item_capacity = 1`, then `item_capacity_available = false` and `item_placeable = false` because the Phase 1 capacity rule does not allow another dropped item in that cell. If item-placement data is unavailable, the result reason is `unknown_or_unloaded` before capacity is evaluated.

### 4. `reservation_commit_valid`

The `reservation_commit_valid` formula is defined as:

`reservation_commit_valid = reservation_matches_current_move_claim AND source_occupied_by_requesting_actor AND target_reserved_by_requesting_actor AND bounds_valid_target AND required_actor_entry_data_available_target AND in_bounds_target AND terrain_actor_passable_target AND NOT target_occupied_by_other_actor AND source_cell != target_cell`

**Variables:**

| Variable | Symbol | Type | Range | Description |
|---|---|---|---|---|
| Reservation Matches Current Move Claim | `reservation_matches_current_move_claim` | bool | `{true, false}` | Whether the reservation ID, owner actor, source cell, target cell, and request identity match the commit attempt. |
| Source Occupied By Requesting Actor | `source_occupied_by_requesting_actor` | bool | `{true, false}` | Whether the moving actor still occupies its expected source cell at commit time. |
| Target Reserved By Requesting Actor | `target_reserved_by_requesting_actor` | bool | `{true, false}` | Whether the target cell is still reserved by the same actor attempting to commit movement. |
| Target Bounds Valid | `bounds_valid_target` | bool | `{true, false}` | Whether target map bounds and required cell dimensions can be trusted. |
| Required Actor Entry Data Available Target | `required_actor_entry_data_available_target` | bool | `{true, false}` | Whether target actor-entry passability, occupancy, and reservation data are loaded. |
| Target In Bounds | `in_bounds_target` | bool | `{true, false}` | Result of `in_bounds` for the target cell. |
| Target Terrain Actor Passable | `terrain_actor_passable_target` | bool | `{true, false}` | Whether map blocking data still allows actor entry into the target cell. |
| Target Occupied By Other Actor | `target_occupied_by_other_actor` | bool | `{true, false}` | Whether another blocking actor occupies the target cell at commit time. |
| Source Cell | `source_cell` | logical coordinate | valid committed actor cell | The movement claim's source cell. |
| Target Cell | `target_cell` | logical coordinate | valid reserved actor cell | The movement claim's target cell. |

**Output Range:** `true` or `false`. If target bounds or required actor-entry data are unavailable, the structured reason is `unknown_or_unloaded`; if target bounds are valid but the target is outside them, the reason is `out_of_bounds`. It is evaluated at movement commit time, not only at request time, so it protects against stale reservations, actor displacement, map transitions, and conflicting occupancy updates.

**Example:** If the reservation matches the current move claim, the source is still occupied by the requesting actor, the target is still reserved by that actor, the target is in bounds and passable, no other blocking actor occupies it, and `source_cell != target_cell`, then `reservation_commit_valid = true`. If the reservation owner does not match, the commit fails with `reservation_owner_mismatch`; if the source is no longer occupied by the requester, it fails with `source_occupancy_lost`.

### 5. `y_sort_key`

The `y_sort_key` formula is defined as:

`y_sort_key_valid = anchor_y_valid AND object_type_sort_rank_valid AND stable_instance_id_valid AND stable_instance_id_comparable_with_active_set`

`y_sort_key = (anchor_y, object_type_sort_rank, stable_instance_id) IF y_sort_key_valid`

If `anchor_y_valid = false`, presentation/debug queries return `invalid_y_sort_anchor`. If `object_type_sort_rank_valid = false`, presentation/debug queries return `invalid_y_sort_rank`. If `stable_instance_id_valid = false` or `stable_instance_id_comparable_with_active_set = false`, presentation/debug queries return `invalid_stable_sort_id`. Invalid Y-sort inputs must not be silently sorted by scene-tree order or array insertion order.

**Variables:**

| Variable | Symbol | Type | Range | Description |
|---|---|---|---|---|
| Anchor Y | `anchor_y` | int or float | render-space dependent; derived from object anchor | Vertical position of the object's Y-sort anchor. |
| Anchor Y Valid | `anchor_y_valid` | bool | `{true, false}` | Whether the object exposes a valid sortable anchor. |
| Object Type Sort Rank | `object_type_sort_rank` | int | configured enum rank | Stable tie-breaker rank for object category when multiple objects share the same `anchor_y`. |
| Object Type Sort Rank Valid | `object_type_sort_rank_valid` | bool | `{true, false}` | Whether the object category has a configured rank in the active sorting policy. |
| Stable Instance ID | `stable_instance_id` | int or string | unique within the active sorted set | Deterministic final tie-breaker. |
| Stable Instance ID Valid | `stable_instance_id_valid` | bool | `{true, false}` | Whether the object has a non-empty stable ID. |
| Stable Instance ID Comparable With Active Set | `stable_instance_id_comparable_with_active_set` | bool | `{true, false}` | Whether the ID uses the same comparable type as the active sorted set. Mixed int/string comparison is invalid. |

**Output Range:** Either a valid lexicographically sortable tuple or a structured presentation/debug failure reason. The contract sorts ascending by tuple and treats later entries as visually in front unless the rendering ADR documents an equivalent mapping. Larger `anchor_y` should normally appear visually in front of smaller `anchor_y`; when `anchor_y` ties, the object type rank and stable ID determine a deterministic order. `stable_instance_id` must use one comparable type within an active sorted set; mixed int/string comparison is invalid and returns `invalid_stable_sort_id`.

**Example:** If a dropped item has `y_sort_key = (120.0, 0, 501)` and an actor has `y_sort_key = (120.0, 2, 17)`, the actor's higher rank appears in front under the MVP rank mapping, while the dropped item must still satisfy the loot readability floor through label/highlight/loot presentation if needed. If two actors share `anchor_y = 120.0` and type rank `2`, stable IDs such as `17` and `18` decide deterministic order and prevent frame-to-frame flicker.

### 6. `query_result_priority`

The `query_result_priority` formula is defined as:

`query_result_priority = first_reason_by_priority(candidate_reasons)`

Where:

`candidate_reasons = {invalid_coordinate IF NOT coordinate_resolved} ∪ {unknown_or_unloaded IF NOT bounds_valid OR NOT required_query_data_available} ∪ {out_of_bounds IF bounds_valid AND NOT in_bounds} ∪ {no_movement_requested IF bounds_valid AND required_query_data_available AND in_bounds AND query_context = actor_entry AND source_cell = target_cell} ∪ {invalid_y_sort_anchor IF query_context = presentation_debug AND NOT anchor_y_valid} ∪ {invalid_y_sort_rank IF query_context = presentation_debug AND anchor_y_valid AND NOT object_type_sort_rank_valid} ∪ {invalid_stable_sort_id IF query_context = presentation_debug AND anchor_y_valid AND object_type_sort_rank_valid AND (NOT stable_instance_id_valid OR NOT stable_instance_id_comparable_with_active_set)} ∪ {blocked_by_static_map IF terrain_blocks_query} ∪ {blocked_by_actor IF query_context IN {actor_entry, reservation_create, reservation_commit} AND target_occupied_by_other_actor} ∪ {reserved IF query_context IN {actor_entry, reservation_create, reservation_commit} AND target_reserved_by_other_actor} ∪ {item_capacity_full IF query_context = item_placement AND NOT item_capacity_available} ∪ {no_item_present IF query_context = pickup_spatial_candidate AND item_count = 0} ∪ {pickup_selection_order_unresolved IF query_context = pickup_spatial_candidate AND item_count > 1 AND NOT pickup_selection_order_available} ∪ {pickup_candidate_available IF query_context = pickup_spatial_candidate AND (item_count = 1 OR (item_count > 1 AND pickup_selection_order_available))} ∪ {walkable IF query_context = actor_entry AND no blocking reason exists} ∪ {item_placeable IF query_context = item_placement AND no item placement blocking reason exists}`

**Variables:**

| Variable | Symbol | Type | Range | Description |
|---|---|---|---|---|
| Coordinate Resolved | `coordinate_resolved` | bool | `{true, false}` | Whether the input resolved to a candidate logical cell. |
| Bounds Valid | `bounds_valid` | bool | `{true, false}` | Whether map bounds and required cell dimensions are valid. |
| Required Query Data Available | `required_query_data_available` | bool | `{true, false}` | Whether the map data needed for this query context is loaded. |
| In Bounds | `in_bounds` | bool | `{true, false}` | Result of the `in_bounds` formula. |
| Terrain Blocks Query | `terrain_blocks_query` | bool | `{true, false}` | Whether terrain blocks the current query type. Actor movement and item placement may read different terrain flags. |
| Query Context | `query_context` | enum | one canonical context | The single primary query type being evaluated; actor entry, item placement, pickup spatial candidate, reservation create, reservation commit, coordinate conversion, or presentation debug. |
| Source Cell | `source_cell` | logical coordinate | valid committed actor cell when applicable | Used to detect no-movement requests before creating reservations. |
| Target Cell | `target_cell` | logical coordinate | queried logical coordinate | The cell being evaluated. |
| Target Occupied By Other Actor | `target_occupied_by_other_actor` | bool | `{true, false}` | Whether another blocking actor occupies the queried cell. |
| Target Reserved By Other Actor | `target_reserved_by_other_actor` | bool | `{true, false}` | Whether the queried cell is reserved by another actor. |
| Item Capacity Available | `item_capacity_available` | bool | `{true, false}` | Whether item placement capacity exists for the queried cell. |
| Item Count | `item_count` | int | `>= 0` | Dropped items present in the queried cell. |
| Pickup Selection Order Available | `pickup_selection_order_available` | bool | `{true, false}` | Whether the query can deterministically order multiple item candidates. Single-item pickup candidate queries do not require a selection order. |
| Anchor Y Valid | `anchor_y_valid` | bool | `{true, false}` | Presentation/debug query input used to detect missing Y-sort anchors. |
| Object Type Sort Rank Valid | `object_type_sort_rank_valid` | bool | `{true, false}` | Presentation/debug query input used to detect missing sortable type-rank configuration. |
| Stable Instance ID Valid | `stable_instance_id_valid` | bool | `{true, false}` | Presentation/debug query input used to detect missing stable IDs. |
| Stable Instance ID Comparable With Active Set | `stable_instance_id_comparable_with_active_set` | bool | `{true, false}` | Presentation/debug query input used to detect mixed incomparable ID types. |
| Candidate Reasons | `candidate_reasons` | set | subset of configured reason enum | All reasons detected by the query before priority resolution. |

**Output Range:** Exactly one primary reason from the configured reason enum. Default priority is `invalid_coordinate` → `unknown_or_unloaded` → `out_of_bounds` → `no_movement_requested` → `invalid_y_sort_anchor` → `invalid_y_sort_rank` → `invalid_stable_sort_id` → `blocked_by_static_map` → `blocked_by_actor` → `reserved` → `item_capacity_full` → `no_item_present` → `pickup_selection_order_unresolved` → `pickup_candidate_available` → `walkable` / `item_placeable`. The success reason depends on query context.

**Example:** If `coordinate_resolved = false`, then `query_result_priority = invalid_coordinate`. If `coordinate_resolved = true` but `bounds_valid = false`, then `query_result_priority = unknown_or_unloaded`. If `bounds_valid = true` and `in_bounds = false`, then `query_result_priority = out_of_bounds`. If an actor-entry query is in bounds and terrain does not block, but `target_reserved_by_other_actor = true`, then `query_result_priority = reserved`. If a presentation/debug query has a valid anchor but no configured object type rank, then `query_result_priority = invalid_y_sort_rank`. If a pickup spatial candidate query has `item_count = 1`, then `query_result_priority = pickup_candidate_available` even when no multi-item selection order is needed. If an item-placement query is in bounds and actor-occupied but item placement ignores actor occupancy under MVP policy, actor occupancy appears only in `cell_facts`, not as `blocked_by_actor`.

## Edge Cases

### Coordinates and Map Bounds

- **If a gameplay query references a logical coordinate outside known valid map bounds**: return `status = blocked` with `primary_reason = out_of_bounds`; no movement, placement, pickup, spawn, combat-distance input, or interaction may proceed.
- **If a visual/world position cannot be converted to a valid logical cell because projection mapping is unavailable or unresolved**: reject the gameplay action with `primary_reason = invalid_coordinate`; do not infer a fallback cell from screen position.
- **If a logical cell maps to valid gameplay data but the corresponding visual tile is missing**: gameplay uses the logical cell result; rendering uses a missing-visual fallback, and the cell remains enterable or blocked according to gameplay data.
- **If a sprite visually overlaps another cell due to sprite size, offset, animation, or 2.5D projection**: blocking, pickup, attack range inputs, and occupancy are resolved only from the object’s logical cell and approved formulas; sprite overlap does not create gameplay collision.
- **If imported map width/height metadata conflicts with the available cell data**: map load fails closed; all cells on that map are treated as `unknown_or_unloaded` until the asset is corrected or re-imported.

### Blocking, Occupancy, and Reservation

- **If a target cell is outside bounds, statically blocked, occupied by a blocking actor, or reserved by another blocking actor**: `actor_enterable` returns `false` with the highest-priority structured reason from `query_result_priority`.
- **If a target cell contains only non-blocking dropped items**: `actor_enterable` may return `true` if the map cell is otherwise enterable and no blocking actor or reservation prevents entry.
- **If multiple blocking reasons apply to the same target cell**: gameplay consumes one canonical reason selected by `query_result_priority`; debug tools may expose all detected reasons.
- **If a blocking actor is removed, despawned, killed, or otherwise invalidated while occupying a cell**: its occupied cell is released during the authoritative state update, and subsequent queries no longer treat that actor as blocking.
- **If an actor has no valid current logical cell**: it cannot block movement, reserve cells, attack, be targeted by cell distance, or participate in Y-sort until assigned a valid in-bounds logical cell.
- **If a system attempts to place a single-cell blocking actor on a cell already occupied by another blocking actor**: placement fails with reason `blocked_by_actor`; no displacement or stacking occurs.
- **If future content introduces multi-cell actors or multi-cell blockers**: they must not use the MVP single-cell actor rule until a separate footprint rule set is approved.

### Movement Reservation

- **If a blocking actor begins movement**: its source cell remains occupied and its target cell becomes reserved until the movement commits or cancels.
- **If another blocking actor queries a cell reserved by a moving actor**: the query returns blocked with reason `reserved`, even if the reserving actor has not visually arrived.
- **If a moving actor’s reservation is canceled before commit**: the target reservation is released, and the actor remains occupying its source cell.
- **If a moving actor’s reservation commit is valid**: the actor releases the source cell, becomes occupied in the target cell, and the target reservation is cleared in the same authoritative update.
- **If a moving actor’s target cell becomes invalid before commit because the map unloads, the cell becomes blocked, or the reservation no longer matches the actor**: commit fails, the reservation is cleared, and the actor remains in or is restored to its last valid occupied source cell.
- **If an actor attempts to reserve a target cell while it already has an active reservation**: reject the new reservation with reason `actor_already_reserved`; the existing reservation remains unchanged unless explicitly canceled first.
- **If two actors request the same target cell in the same authoritative update**: exactly one reservation may be granted according to deterministic action ordering; all other requests fail with reason `reserved` or `blocked_by_actor` after the winning reservation is applied.
- **If two actors attempt to swap cells simultaneously**: both moves are rejected unless future OpenMir2 source evidence explicitly permits swaps; source cells remain occupied to prevent pass-through ambiguity.
- **If an actor attempts to move into its own current source cell**: the request resolves as no movement; no new reservation is created, and the actor remains occupying the source cell.
- **If an actor cancels movement after commit has already completed**: the cancel request has no effect on occupancy; any new movement must be requested as a separate action.
- **If a reservation references an actor instance that no longer exists**: the reservation is invalid and must be released during the next authoritative cleanup before new movement queries are resolved.

### Item Placement and Drops

- **If an item drop is requested on an in-bounds, item-placeable cell with available item capacity**: placement succeeds even if the cell is occupied or reserved by a blocking actor, because dropped items do not block movement in MVP.
- **If an item drop is requested outside map bounds**: placement fails with reason `out_of_bounds`; the item must not be silently moved to a nearby cell unless a separate loot-spill rule is approved.
- **If an item drop is requested on a statically blocked map cell**: `item_placeable` must return the configured result for item placement rules; if required item-placement map data is unavailable, placement fails closed with `unknown_or_unloaded`; if the initial placement is illegal but the reward has already been generated, the result must include `drop_fallback_required` for the drop system.
- **If another dropped item is placed on a logical cell that already contains one dropped item in the Phase 1 playable contract**: placement fails with `item_capacity_full` and the drop system receives `drop_fallback_required`; the item is not silently stacked, deleted, rerolled, or moved.
- **If multiple dropped items occupy the same logical cell because of malformed state, debug fixture, save migration, or future rules**: all remain individually addressable by stable item instance ID; the deterministic listing order is `stable_drop_sequence` then `stable_item_instance_id`; Y-sort tie-breaking must not merge them into one visual or gameplay entity.
- **If an item is picked up, expired, or removed while sharing a cell with other dropped items**: only that item instance is removed; the other dropped items remain in the cell and keep their own pickup eligibility.
- **If an item has a valid logical cell but no loaded visual asset**: the item remains pickup-queryable and Y-sortable; rendering uses a missing-item visual fallback or hides only the sprite, not the gameplay entity.
- **If an item has no valid logical cell**: it cannot be placed, picked up, queried, or Y-sorted; the item instance is invalid until assigned a valid in-bounds cell.

### Pickup Spatial Queries

- **If a pickup spatial query targets an out-of-bounds cell**: return no spatial candidate and reason `out_of_bounds`.
- **If a pickup spatial query targets an in-bounds cell containing no dropped items**: return no spatial candidate and reason `no_item_present`.
- **If a pickup spatial query targets one dropped item in the target cell**: return that item as the spatial candidate without deciding pickup distance, inventory eligibility, or pickup completion.
- **If a pickup spatial query targets multiple dropped items in the same cell**: under the Phase 1 capacity-1 playable contract this is a malformed/debug/future-rule state; return exactly one spatial candidate only when `stable_drop_sequence` and `stable_item_instance_id` are available for deterministic ordering, otherwise return reason `pickup_selection_order_unresolved` and do not complete pickup.
- **If pickup distance, adjacency, diagonal rules, inventory capacity, or item eligibility are required**: those checks are performed by the authoritative pickup / inventory systems after the spatial candidate is selected; this system only exposes cell occupancy facts and selected spatial candidates.
- **If a selected dropped item is removed before downstream pickup commit**: this system reports that the item is no longer present at the logical cell; inventory mutation and final commit failure reasons belong to the authoritative pickup / inventory systems.
- **If a dropped item is on the actor’s occupied cell**: this system may return the item as a same-cell spatial candidate, but same-cell visual overlap alone does not imply pickup completion.

### Y-sort and Visual Ordering

- **If two renderable world objects have different Y-sort anchor Y values**: render order is determined by `y_sort_key` using `anchor_y` as the primary key.
- **If two renderable world objects have the same Y-sort anchor Y value**: render order is determined by object type layer, then stable instance ID; frame order and scene-tree insertion order must not decide the tie.
- **If two dropped items share the same cell and same anchor Y**: they are ordered by dropped-item type layer and stable item instance ID; their order remains stable across frames.
- **If a blocking actor and a dropped item share the same logical cell and same anchor Y**: the object type layer decides their relative visual order; the dropped item does not affect blocking or actor occupancy.
- **If an object lacks a valid Y-sort anchor**: it is excluded from world Y-sort rendering and reported as invalid for presentation; gameplay state remains unchanged.
- **If an object’s visual sprite extends above or below its anchor**: Y-sort continues to use the anchor, not sprite bounds, so tall sprites do not reorder based on their full texture rectangle.
- **If an actor is visually interpolating between cells**: Y-sort may use the approved presentation anchor for the current rendered position, while gameplay queries continue to use the authoritative occupied/reserved logical cells.
- **If Y-sort presentation conflicts with gameplay hit/pickup expectations**: gameplay authority wins; interaction prompts, pickup queries, and targeting must use logical-cell rules, not apparent front/back draw order.
- **If stable instance ID ordering changes after save/load, respawn, or map reload**: the system treats this as invalid for deterministic Y-sort; stable IDs must be restored or regenerated through a deterministic documented rule before rendering order is trusted.

### Map Loading, Missing Data, and Map Changes

- **If a map is not loaded**: all gameplay queries against that map return `status = unavailable` with `primary_reason = unknown_or_unloaded`; no actor movement, item placement, pickup, or Y-sort registration occurs.
- **If map blocking data fails to load but visual tiles load**: the map remains gameplay unavailable; visual presence does not imply walkability.
- **If visual data fails to load but blocking data loads**: gameplay may run if the map is otherwise valid, but presentation must use missing-visual fallbacks and report the asset issue.
- **If a map unloads while actors or items still reference its cells**: those actors/items are removed from active gameplay queries and Y-sort until transferred to another valid map or restored after reload.
- **If an actor attempts to move across a map boundary or into another map**: reject with reason `cross_map_movement_undefined` unless a separate transition/teleport rule explicitly handles the transfer.
- **If a saved actor or item position references a cell that no longer exists in the loaded map version**: restoration fails closed for that entity; it must not be auto-snapped to a nearby valid cell without an approved migration rule.

### Downstream System Conflicts

- **If combat, AI, pathfinding, loot, UI, or networking requests a passability result**: they must consume the structured query result from this system; they must not duplicate coordinate, blocking, or reservation logic.
- **If pathfinding predicts a route through a cell that becomes occupied or reserved before movement execution**: movement execution fails at the reservation step; pathfinding must re-query rather than force entry.
- **If combat range logic and movement adjacency logic disagree because distance or diagonal rules are TBD**: both systems must block final authoritative action with their relevant `*_rule_unresolved` reason until OpenMir2 evidence resolves the shared formula.
- **If AI selects a target cell that is valid during planning but invalid during execution**: execution fails with the current structured reason, and AI must request a new plan from the latest grid state.
- **If future network/server authority provides a position that conflicts with client-local occupancy**: the authoritative gameplay state must reconcile to the server/source-of-truth position, clearing or rebuilding affected reservations deterministically.
- **If UI hover/selection uses screen-space hit testing that disagrees with logical-cell lookup**: UI may display hover affordance only after resolving the selected object/cell through the gameplay query contract.
- **If debug tools reveal visual and logical coordinates disagree**: gameplay remains governed by logical coordinates; the mismatch is logged as a coordinate-mapping defect and not corrected by runtime guessing.
- **If any downstream system needs a new blocking category, item capacity rule, actor footprint, pickup priority, or Y-sort layer not defined here**: that system must propose an explicit GDD update before implementation; it may not introduce hidden local rules.

## Dependencies

### Upstream Dependencies

| Dependency | Type | Required Input | Status | Notes |
|---|---|---|---|---|
| OpenMir2 行为映射 Spike | Hard | Source evidence for map coordinates, blocking, object occupancy, movement legality, drop landing, pickup legality, and map data semantics. | Existing GDD; several source questions still open. | This system may define provisional contracts, but final coordinate origin, axis direction, cell projection, diagonal rules, distance semantics, and source-accurate blocking behavior remain **TBD from OpenMir2 source evidence**. |
| 资源 / 地图转换管线 Spike | Hard for validation | Map data, blocking metadata, coordinate metadata, visual anchor metadata, and import validation hooks. | Planned; no GDD yet. | The design contract can be written now, but implementation validation requires a map data source or converted test map. |
| Godot 4.6.3 engine reference | Hard for implementation, Soft for design | Verified capabilities and constraints for 2D rendering, TileMap/TileMapLayer, CanvasItem sorting, collision helpers, and input projection. | Engine reference exists. | This GDD does not choose Godot APIs; ADRs must verify post-cutoff implementation details before coding. |

### Downstream Dependencies

| Dependent System | Type | What It Needs From This System | Ownership Boundary |
|---|---|---|---|
| 点击移动系统 | Hard | Candidate logical cell, passability result, blocking reason, reservation/commit/cancel contract. | Movement owns click fallback, path choice, movement timing, animation, and movement feedback. |
| 交互目标 / 选择系统 | Hard | Screen/input to spatial query support, object logical cells, Y-sort/readability references. | Target selection owns click priority, hover rules, final selected target, and UI feedback. |
| 怪物生成系统 | Hard | Spawn cell legality, actor occupancy, static blocking, reservation checks. | Spawn system owns spawn tables, spawn timing, spawn regions, and fallback candidate search. |
| 怪物 AI / 行为系统 | Hard | Current actor cells, neighbor/candidate legality, reservation outcomes. | AI owns behavior choice, target choice, and path planning strategy. |
| 基础战斗系统 | Hard | Attacker and target logical cells, shared distance-contract inputs, blocking facts if needed. | Combat owns attack range formula, attack legality, damage, hit timing, and combat feedback. |
| 掉落与拾取系统 | Hard | Death cell, item placement legality, item occupancy, player/item logical cells. | Drop/pickup owns loot selection, fallback drop search, pickup distance, item priority, ownership, and inventory checks. |
| 地图表现 / 渲染系统 | Soft for gameplay, Hard for presentation | Y-sort anchors, object type sort ranks, visual-only obstruction semantics. | Rendering owns Godot implementation, layers, sprites, animation offsets, and visual fallbacks. |
| 存档系统 | Soft in MVP, Hard once persistence begins | Map ID, actor logical cells, item logical cells, occupancy/reservation state needed for restoration. | Save/load owns persistence format, versioning, and restore/migration behavior. |
| QA / Debug Tools | Soft but recommended | Inspectable cell state, blocking reason, occupant list, reservation state, Y-sort anchors. | QA/tools own debug overlay and reporting presentation. |
| Network / 最小协议系统 | Future | Deterministic grid state, reservation/commit semantics, stable coordinate contract. | Networking owns server authority, reconciliation, and protocol synchronization. |

### Provisional Assumptions

These assumptions are **MVP provisional** unless and until OpenMir2 behavior mapping reaches E3/E4 evidence for the same behavior:

- MVP uses single-map, single-layer logical grid rules unless a later GDD expands scope.
- Player and ordinary monsters are single-cell blocking actors.
- Movement uses source-cell occupied plus target-cell reservation as a deterministic Godot slice safeguard.
- Dropped items occupy logical cells but do not block actor movement.
- Pickup-complete Phase 1 uses `same_cell_item_capacity = 1`; multiple dropped items sharing a logical cell are future/evidence-gated or debug-only until drop/pickup design and OpenMir2 evidence approve a different rule.
- Drop-on-death-cell placement is provisional until source evidence confirms ground-item placement behavior; failed initial placement must not silently delete generated rewards.
- Y-sort type rank order is provisional and must be validated against drop readability.
- Complex overlay, roof, canopy, multi-cell actor, portal, safe-zone, and dynamic terrain rules are outside MVP unless later GDDs add them.

### Dependency Risks

- If OpenMir2 coordinate or blocking evidence conflicts with this provisional contract, this GDD must be revised before implementation stories are generated.
- If the map conversion pipeline cannot provide logical bounds, static passability, and Y-sort anchors, this system cannot be verified.
- If downstream systems invent their own local distance, blocking, or occupancy rules, the project will lose cross-system consistency and should fail design review.

## Tuning Knobs

### Policy Knobs

| Knob | Type | MVP Default | Safe Range / Values | Purpose | Risk if Misused |
|---|---|---|---|---|---|
| `query_reason_priority_order` | ordered enum list | `invalid_coordinate` → `unknown_or_unloaded` → `out_of_bounds` → `no_movement_requested` → `invalid_y_sort_anchor` → `invalid_y_sort_rank` → `invalid_stable_sort_id` → `blocked_by_static_map` → `blocked_by_actor` → `reserved` → `item_capacity_full` → `no_item_present` → `pickup_selection_order_unresolved` → `pickup_candidate_available` → success reason | Must include all primary structured reasons for the relevant query context exactly once. | Chooses the canonical reason when multiple reasons apply. | Wrong ordering can hide map data failures or mislead downstream retry/fallback logic. |
| `actor_blocks_actor_movement` | bool | `true` | `{true, false}` | Controls whether blocking actors prevent other blocking actors from entering the same cell. | Setting to `false` may simplify prototypes but causes overlapping actors and can invalidate combat/AI assumptions. |
| `movement_reservation_required` | bool | `true` | `{true, false}` | Controls whether movement must reserve the target cell before commit. | Setting to `false` allows same-frame cell contention and makes occupancy less deterministic. |
| `item_blocks_actor_movement` | bool | `false` | `{true, false}` | Controls whether dropped items block actors. | Setting to `true` can break loot-loop flow by letting drops clog paths. |
| `same_cell_item_policy` | enum | `single_item_phase1_provisional` | `single_item_phase1_provisional`, `capacity_limited_future`, `source_verified_rule` | Controls whether multiple dropped items can share a cell. | Changing this affects drop fallback, pickup ordering, and item visualization. |
| `same_cell_item_capacity` | int | `1` | `1` for Phase 1 pickup-complete stories; higher values require GDD revision or source-verified rule | Maximum dropped item instances allowed in one logical cell. | Higher values reopen pickup selection, readability, and query performance risks. |
| `missing_map_data_policy` | enum | `fail_closed` | `fail_closed` only for MVP | Determines behavior when passability or bounds data is missing. | Any non-fail-closed mode risks walk-through-walls or invalid spawns. |
| `y_sort_type_rank_order` | ordered enum list | dropped items → small decorations → blocking actors → large props → overlay/canopy | Must remain deterministic. | Resolves same-anchor visual ordering. | Bad ordering can hide drops, obscure actors, or cause confusing front/back presentation. |
| `stable_id_required_for_sorting` | bool | `true` | `{true, false}` | Requires stable instance IDs for final Y-sort tie-breaks. | If disabled, sort order may flicker or vary after save/load. |

### Debug and Validation Knobs

| Knob | Type | MVP Default | Safe Range / Values | Purpose | Risk if Misused |
|---|---|---|---|---|---|
| `show_grid_coordinates_overlay` | bool | `false` | `{true, false}` | Displays logical grid coordinates for debugging. | Should not ship as normal player-facing UI. |
| `show_blocking_reason_overlay` | bool | `false` | `{true, false}` | Displays passability and structured blocking reason per inspected cell. | Can clutter screen; must remain debug-only. |
| `show_occupancy_overlay` | bool | `false` | `{true, false}` | Displays actor occupancy, item occupancy, and reservations. | Debug-only; may reveal internal state if exposed to players. |
| `show_y_sort_anchor_overlay` | bool | `false` | `{true, false}` | Displays object anchors used for Y-sort. | Debug-only; useful for diagnosing sorting defects. |
| `log_coordinate_mapping_failures` | bool | `true` | `{true, false}` with rate limiting | Logs screen/world to logical cell conversion failures. | If unbounded, repeated hover/path failures can spam logs and distort performance evidence. |
| `log_conflicting_query_reasons` | bool | `true` | `{true, false}` with rate limiting | Logs all detected reasons when `query_result_priority` returns one canonical reason. | If unbounded, hidden multi-reason bugs become log noise and performance risk. |
| `debug_overlay_update_mode` | enum | `selected_cell_or_visible_region` | `selected_cell`, `visible_region`, `full_grid_debug_only` | Controls how much debug overlay information is drawn. | Full-grid overlays can distort performance and readability if treated as normal gameplay. |
| `y_sort_dirty_refresh_mode` | enum | `batched_update` | `batched_update`, `immediate_debug_only` | Controls how dirty sort inputs are resolved. | Immediate resort per object can create frame-time spikes. |

### Future / Expansion Knobs

| Knob | Type | MVP Default | Safe Range / Values | Purpose | Risk if Enabled Too Early |
|---|---|---|---|---|---|
| `passability_category_set` | enum set | `actor_passable`, `item_placeable` | Future categories may include projectile, sight, safe-zone, flying, water, door/portal. | Allows richer blocking rules after MVP. | Adding categories before downstream systems are ready can split behavior contracts. |
| `multi_cell_footprint_enabled` | bool | `false` | `{true, false}` | Enables actors/props that occupy more than one logical cell. | Requires new formulas, edge cases, spawn/drop validation, and Y-sort footprint rules. |
| `diagonal_corner_rule` | enum | `TBD from OpenMir2 source evidence` | `allow`, `forbid`, `source_verified_rule` | Defines whether diagonal movement can cut between two blocked orthogonal neighbors. | Premature choice can break OpenMir2 behavior alignment. |
| `distance_metric` | enum | `TBD from OpenMir2 source evidence` | `manhattan`, `chebyshev`, `euclidean`, `source_verified_rule` | Shared distance semantics for combat, pickup, spawn, and AI. | If systems choose different metrics, movement/combat/pickup will conflict. |
| `overlay_layer_enabled` | bool | `false` | `{true, false}` | Enables complex roof/canopy/foreground overlay handling. | Adds rendering and asset complexity beyond MVP needs. |
| `cross_map_transition_enabled` | bool | `false` | `{true, false}` | Enables movement or transfer across map boundaries. | Requires transition, save/load, spawn, and future networking rules. |

### Non-Tunable Values

The following must not be tuned in this GDD because they require OpenMir2 evidence or downstream system ownership:

- Movement speed.
- Attack range.
- Pickup range.
- Spawn radius.
- Drop search radius.
- Tile pixel size.
- Projection formula.
- Coordinate origin and axis direction.
- Exact diagonal movement and corner-cutting behavior.

## Visual/Audio Requirements

This system has no direct player-facing audio requirement in MVP. Its visual responsibility is to provide reliable spatial presentation contracts for downstream rendering and QA validation.

### Visual Requirements

- All world objects that participate in map depth ordering must expose a visible or debug-inspectable Y-sort anchor.
- Player, ordinary monsters, dropped items, and sortable map props must preserve stable front/back ordering according to `y_sort_key`.
- Missing map visuals must not change gameplay passability; if visual data is absent but logic data is valid, the renderer should use a missing-visual fallback or report the missing asset through debug evidence.
- Dropped items sharing a cell must remain individually identifiable in debug evidence, and the loot-loop slice must provide a player-facing readability affordance before same-cell drops are considered presentation-complete.
- Complex overlay, roof, canopy, and tall-object masking are not required for MVP, but the contract must not prevent later foreground/overlay layers from being added.

### Audio Requirements

- No spatial audio, movement audio, blocking audio, pickup audio, or drop audio is owned by this system.
- If a downstream system plays feedback for blocked movement, pickup failure, drop spawn, or interaction ambiguity, that system owns the audio cue and should reference this system’s structured reason only as an input.

## UI Requirements

This system has no normal player-facing UI in MVP. It does require debug and QA-facing UI/evidence support.

### Debug / QA UI Requirements

- A debug view or overlay should be able to display the selected cell’s logical coordinate.
- The same debug view should display in-bounds status, static blocking status, actor occupancy, reservation state, dropped item count/list, and latest structured query reason.
- A Y-sort debug view should display each sortable object’s anchor, object type rank, stable instance ID, and final ordering.
- Debug overlays must be disabled by default and must not be treated as player-facing UI.
- Debug overlays should default to selected-cell or visible-region inspection; full-grid overlays are debug-only and must not be used as normal gameplay performance evidence.
- If debug UI is not implemented in the first story, equivalent logs or exported debug evidence must be available for QA to verify the acceptance criteria.

## Acceptance Criteria

### Acceptance Scope Layers

Acceptance criteria are divided by implementation scope:

- **Phase 1 Blocking**: Must pass for the first implementation story or the 30-second offline loot-loop slice cannot be trusted.
- **Phase 1 Integration**: Must pass before the map coordinate / blocking / Y-sort system is considered complete for the MVP slice.
- **Evidence Gate**: Must remain blocked until OpenMir2 source evidence or an approved design decision resolves the TBD behavior.
- **Future / Post-MVP**: Documents required behavior boundaries for later systems; these must not be scheduled as first-sprint implementation scope unless explicitly promoted by production planning.

Each implementation story derived from these criteria must label its ACs with scope, story type, gate level, and evidence path. Broad criteria below may be split into smaller story-level ACs during story creation, but they must not lose the required behavior.

### Phase 1 Provisional Playable Contract

- **Scope**: Phase 1 Blocking · **Story Type**: Integration / Config/Data · **Gate**: Blocking
  **GIVEN** an implementation story uses this GDD before OpenMir2 E3/E4 evidence is complete, **WHEN** the story defines its design basis, **THEN** it explicitly labels each relied-on spatial rule as `MVP provisional`, `OpenMir2-derived`, or `intentional divergence`, and source-authentic completion remains blocked for any provisional rule.
  **Evidence**: Story readiness checklist or QA gate note that lists every relied-on readiness matrix row, each row's classification, whether it is source-authentic or MVP provisional, unresolved gate owner, and the approval path for any intentional divergence.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic · **Gate**: Blocking
  **GIVEN** the Phase 1 pickup-complete item-placement policy is active, **WHEN** one dropped item already occupies a logical cell, **THEN** another item-placement request for the same cell returns `status = blocked`, `primary_reason = item_capacity_full`, and exposes `drop_fallback_required` to the drop system without mutating existing item occupancy.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic / Integration · **Gate**: Blocking for pickup-complete stories
  **GIVEN** an actor and exactly one available dropped item share a valid logical cell under the synthetic-map provisional pickup rule, **WHEN** a pickup spatial query is made, **THEN** the query returns that item as the candidate using stable item identity and does not rely on screen-space overlap or Y-sort order.
  **Evidence**: Automated Unit Test or Integration Test.

### Coordinate Bounds and Logical Authority

- **Scope**: Phase 1 Blocking · **Story Type**: Logic · **Gate**: Blocking
  **GIVEN** a map with `map_loaded = true`, width `W > 0`, height `H > 0`, and matching cell data, **WHEN** a query uses coordinate `(x, y)` where `0 <= x < W` and `0 <= y < H`, **THEN** `bounds_valid = true` and `in_bounds(x, y)` returns `true`.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic · **Gate**: Blocking
  **GIVEN** a map with valid bounds, **WHEN** a query uses boundary coordinates such as `(-1, 0)`, `(0, -1)`, `(W, 0)`, or `(0, H)`, **THEN** `in_bounds(x, y)` returns `false`, dependent gameplay queries fail closed, and the primary reason is `out_of_bounds`.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic · **Gate**: Blocking
  **GIVEN** map bounds, cell data, or required query data are unavailable or invalid, **WHEN** a spatial query is evaluated, **THEN** it fails closed with `primary_reason = unknown_or_unloaded` rather than `out_of_bounds`.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic / Integration · **Gate**: Blocking
  **GIVEN** an object whose visual position differs from its logical grid cell due to animation, offset, or interpolation, **WHEN** gameplay queries blocking, occupancy, pickup, or enterability, **THEN** the result is based on the object’s logical grid state and not its transient screen/render position.
  **Evidence**: Integration Test.

### Coordinate Conversion

- **Scope**: Evidence Gate / Phase 1 Blocking when converter exists · **Story Type**: Logic · **Gate**: Blocking for input-driven stories
  **GIVEN** an approved coordinate converter or test double with a declared deterministic mapping contract, **WHEN** conversion is performed repeatedly under unchanged state, **THEN** it returns the same unique logical cell each time.
  **Evidence**: Automated Unit Test using approved converter/test double.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic · **Gate**: Blocking
  **GIVEN** an input coordinate cannot be converted to a valid logical cell, **WHEN** enterability, placement, pickup, or blocking is queried from it, **THEN** the query fails closed, does not default to `(0, 0)` or any arbitrary fallback, and returns `primary_reason = invalid_coordinate`.
  **Evidence**: Automated Unit Test.

### Actor Enterability and Blocking

- **Scope**: Phase 1 Blocking · **Story Type**: Logic · **Gate**: Blocking
  **GIVEN** a logical cell occupied by the player actor, **WHEN** another blocking actor queries whether it can enter that cell, **THEN** `actor_enterable(cell)` returns `false` and the reason identifies actor occupancy.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic · **Gate**: Blocking
  **GIVEN** a logical cell occupied by an ordinary monster actor, **WHEN** the player or another ordinary monster queries whether it can enter that cell, **THEN** `actor_enterable(cell)` returns `false` and the reason identifies actor occupancy.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic · **Gate**: Blocking
  **GIVEN** an in-bounds logical cell with no static blocking, no actor occupancy, and no active reservation, **WHEN** a blocking actor queries whether it can enter that cell, **THEN** `actor_enterable(cell)` returns `true`.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic · **Gate**: Blocking
  **GIVEN** an in-bounds logical cell marked as statically blocked by map data, **WHEN** a blocking actor queries whether it can enter that cell, **THEN** `actor_enterable(cell)` returns `false` and the reason identifies static map blocking.
  **Evidence**: Automated Unit Test.

### Movement Reservation and Commit

- **Scope**: Phase 1 Blocking · **Story Type**: Logic / Integration · **Gate**: Blocking
  **GIVEN** an actor moving from source cell `A` to target cell `B`, **WHEN** movement has reserved `B` but has not committed, **THEN** source cell `A` remains occupied and other actors cannot enter `A`.
  **Evidence**: Integration Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic / Integration · **Gate**: Blocking
  **GIVEN** an actor moving from source cell `A` to target cell `B`, **WHEN** movement has reserved `B` but has not committed, **THEN** target cell `B` is reserved and other actors cannot enter `B`.
  **Evidence**: Integration Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic / Integration · **Gate**: Blocking
  **GIVEN** a target cell already occupied by a blocking actor, **WHEN** another actor attempts to reserve or commit entry into that cell, **THEN** `reservation_commit_valid` returns `false` and the reason identifies actor occupancy.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic / Integration · **Gate**: Blocking
  **GIVEN** a target cell already reserved by another actor, **WHEN** a different actor attempts to reserve or commit entry into that cell, **THEN** reservation creation or commit validation fails and the reason identifies reservation conflict.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic / Integration · **Gate**: Blocking
  **GIVEN** an actor has reserved target cell `B`, **WHEN** `B` becomes invalid before commit, **THEN** `reservation_commit_valid` returns `false`, the actor is not committed into `B`, and a structured failure reason is returned.
  **Evidence**: Integration Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic / Integration · **Gate**: Blocking
  **GIVEN** an actor has a valid reservation from source cell `A` to target cell `B`, **WHEN** movement commit succeeds, **THEN** `A` is released, `B` becomes occupied by that actor, and the reservation on `B` is cleared in the same authoritative update.
  **Evidence**: Integration Test.

### Item Placement and Drop Occupancy

- **Scope**: Phase 1 Blocking · **Story Type**: Logic / Integration · **Gate**: Blocking
  **GIVEN** an in-bounds logical cell that allows item placement, **WHEN** the system queries whether a dropped item can be placed there, **THEN** `item_placeable(cell)` returns `true`.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic / Integration · **Gate**: Blocking
  **GIVEN** an out-of-bounds coordinate, **WHEN** the system queries whether a dropped item can be placed there, **THEN** `item_placeable(cell)` returns `false` and the reason identifies `out_of_bounds`.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic / Integration · **Gate**: Blocking
  **GIVEN** a logical cell containing one or more dropped items but no static blocking, actor occupancy, or reservation, **WHEN** a blocking actor queries whether it can enter the cell, **THEN** `actor_enterable(cell)` returns `true` and dropped items are not returned as an actor-blocking reason.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic · **Gate**: Blocking
  **GIVEN** a valid logical cell already containing a dropped item and `same_cell_item_capacity = 1`, **WHEN** another dropped item is placed into the same cell, **THEN** placement fails with `item_capacity_full`, the original item remains separately addressable, and the new reward is returned to the drop system for approved fallback handling.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic / Integration · **Gate**: Blocking
  **GIVEN** a logical cell containing only dropped items, **WHEN** an actor attempts to reserve that cell as a movement target, **THEN** the reservation can be created if no other blocking condition exists.
  **Evidence**: Integration Test.

### Pickup Spatial Queries

- **Scope**: Phase 1 Blocking · **Story Type**: Logic / Integration · **Gate**: Blocking for pickup candidate stories
  **GIVEN** a logical cell containing one dropped item, **WHEN** a pickup spatial query targets that cell, **THEN** the query returns that item as a spatial candidate and does not decide pickup completion.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic / Integration · **Gate**: Blocking for pickup candidate stories
  **GIVEN** a pickup spatial query targets an out-of-bounds cell, **WHEN** the query is evaluated, **THEN** no spatial candidate is returned and the reason is `out_of_bounds`.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic / Integration · **Gate**: Blocking for pickup candidate stories
  **GIVEN** a pickup spatial query targets an in-bounds cell containing no dropped items, **WHEN** the query is evaluated, **THEN** no spatial candidate is returned and the reason is `no_item_present`.
  **Evidence**: Automated Unit Test.

- **Scope**: Evidence Gate / Future or malformed-state handling · **Story Type**: Logic · **Gate**: Blocking only if multiple same-cell items are permitted by a story
  **GIVEN** a pickup spatial query targets multiple dropped items in the same cell, **WHEN** stable drop sequence and stable item instance ID are unavailable, **THEN** no item is selected, no item is removed, and the primary reason is `pickup_selection_order_unresolved`.
  **Evidence**: Automated Unit Test.

- **Scope**: Future / Debug or post-revision rule · **Story Type**: Logic / Integration · **Gate**: Blocking only if multiple same-cell items are permitted by a story
  **GIVEN** multiple dropped items occupy the same logical cell in a malformed-state test or future approved rule, **WHEN** a pickup spatial query requests a single candidate and stable ordering fields exist, **THEN** exactly one spatial candidate is returned by `stable_drop_sequence` ascending, then `stable_item_instance_id`, before downstream pickup / inventory validation runs.
  **Evidence**: Automated Unit Test once rule is in scope.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic / Integration · **Gate**: Blocking for pickup candidate stories
  **GIVEN** a selected item is removed before downstream pickup commit, **WHEN** this system is asked to verify that item’s logical cell presence, **THEN** it reports that the item is no longer present and performs no inventory mutation.
  **Evidence**: Integration Test.

### Structured Reasons and Query Priority

- **Scope**: Phase 1 Blocking · **Story Type**: Logic · **Gate**: Blocking
  **GIVEN** any failed enterability, placement, reservation, or pickup spatial query, **WHEN** the system returns the result, **THEN** the result contains a machine-readable structured reason and does not rely only on human-readable text.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic · **Gate**: Blocking
  **GIVEN** a query simultaneously matches multiple failure reasons, **WHEN** `query_result_priority` is applied, **THEN** the returned primary reason follows the configured priority order.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic · **Gate**: Blocking
  **GIVEN** the same multi-reason query is repeated under identical state, **WHEN** the result is generated multiple times, **THEN** the primary reason remains stable; any secondary reasons do not change the primary priority.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Logic · **Gate**: Blocking
  **GIVEN** an actor movement query targets a cell containing only dropped items and no static block, actor occupant, or reservation, **WHEN** the result is generated, **THEN** `status = allowed`, `primary_reason = walkable`, and item occupancy appears only in `cell_facts` or secondary debug information.
  **Evidence**: Automated Unit Test.

### Map Loading and Missing Data

- **Scope**: Phase 1 Blocking · **Story Type**: Integration · **Gate**: Blocking
  **GIVEN** map logical data, blocking data, or required coordinate data is missing, **WHEN** actor enterability, item placement, reservation, or pickup queries depend on that data, **THEN** queries fail closed and return a map-data-missing reason.
  **Evidence**: Integration Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Integration · **Gate**: Blocking
  **GIVEN** visual map tiles load but blocking data fails to load, **WHEN** gameplay queries run, **THEN** the map remains gameplay unavailable and visual presence does not imply walkability.
  **Evidence**: Integration Test.

- **Scope**: Phase 1 Blocking · **Story Type**: Integration · **Gate**: Blocking
  **GIVEN** blocking data loads but visual data fails to load, **WHEN** gameplay queries run, **THEN** gameplay may proceed from logical data while presentation reports or displays missing-visual fallback evidence.
  **Evidence**: Integration Test + Manual/Debug Evidence.

### Y-sort

- **Scope**: Phase 1 Integration · **Story Type**: Logic / Integration / Visual Evidence · **Gate**: Blocking for deterministic ordering; Advisory for visual evidence unless loot-loop story depends on it
  **GIVEN** a sortable object, **WHEN** its `y_sort_key` is calculated, **THEN** the key includes and orders by `anchor_y`, `object_type_sort_rank`, and `stable_instance_id`.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Integration · **Story Type**: Logic / Integration / Visual Evidence · **Gate**: Blocking for deterministic ordering; Advisory for visual evidence unless loot-loop story depends on it
  **GIVEN** two sortable objects with different `anchor_y` values, **WHEN** their `y_sort_key` values are compared, **THEN** ordering is determined by the anchor difference before type rank or stable ID.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Integration · **Story Type**: Logic / Integration / Visual Evidence · **Gate**: Blocking for deterministic ordering; Advisory for visual evidence unless loot-loop story depends on it
  **GIVEN** two sortable objects with the same `anchor_y` and different object type ranks, **WHEN** their `y_sort_key` values are compared, **THEN** ordering is determined by object type rank.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Integration · **Story Type**: Logic / Integration / Visual Evidence · **Gate**: Blocking for deterministic ordering; Advisory for visual evidence unless loot-loop story depends on it
  **GIVEN** two sortable objects with the same `anchor_y` and object type rank but different stable IDs, **WHEN** their `y_sort_key` values are compared repeatedly, **THEN** ordering is determined by stable ID and remains deterministic.
  **Evidence**: Automated Unit Test.

- **Scope**: Phase 1 Integration · **Story Type**: Logic / Integration / Visual Evidence · **Gate**: Blocking for deterministic ordering; Advisory for visual evidence unless loot-loop story depends on it
  **GIVEN** two objects change visual front/back ordering through Y-sort, **WHEN** gameplay queries blocking, occupancy, reservation, or pickup, **THEN** gameplay results are unchanged by visual ordering.
  **Evidence**: Integration Test.

- **Scope**: Phase 1 Integration · **Story Type**: Logic / Integration / Visual Evidence · **Gate**: Blocking for deterministic ordering; Advisory for visual evidence unless loot-loop story depends on it
  **GIVEN** a debug scene containing player, monster, dropped item, and sortable map object, **WHEN** Y-sort debug evidence is captured, **THEN** QA can inspect each object’s anchor, type rank, stable ID, and final ordering.
  **Evidence**: Manual/Debug Evidence.

- **Scope**: Phase 1 Integration · **Story Type**: Visual/Feel + Integration · **Gate**: Advisory for rendering, Blocking for loot-loop presentation completion
  **GIVEN** a dropped item shares a logical cell or Y-sort anchor with an actor or prop, **WHEN** the loot-loop presentation is evaluated for a story that claims presentation completion, **THEN** the dropped item is not completely invisible and has a visible affordance confirmed by screenshot, debug capture, or equivalent manual evidence with lead sign-off.
  **Evidence**: Screenshot, debug capture, or manual evidence artifact in `production/qa/evidence/`.

### Performance and Debug Guardrails

- **Scope**: Phase 1 Blocking · **Story Type**: Logic / Performance · **Gate**: Blocking
  **GIVEN** this system resolves actor-entry, item-placement, pickup-candidate, reservation, or Y-sort dirty queries during normal gameplay with debug overlays disabled, **WHEN** a query executes, **THEN** the system does not scan the full map unless the story explicitly invokes load-time/offline validation, and the evidence reports `cells_scanned_per_query` and `cells_scanned_per_update`.
  **Evidence**: Integration Test or profiling/debug counter evidence.

- **Scope**: Phase 1 Integration · **Story Type**: Performance · **Gate**: Blocking for any story claiming performance handoff
  **GIVEN** a story consumes this spatial contract, **WHEN** QA reviews performance evidence, **THEN** the story records `spatial_queries_per_update`, `cells_scanned_per_query`, `cells_scanned_per_update`, `sort_inputs_marked_dirty`, and `sort_inputs_refreshed` against an ADR-defined or story-declared candidate budget.
  **Evidence**: Profiling/debug counter evidence linked from the story.

- **Scope**: Phase 1 Integration · **Story Type**: Performance · **Gate**: Blocking before performance handoff
  **GIVEN** repeated coordinate failures or conflicting query reasons occur, **WHEN** logs are emitted, **THEN** logging is rate-limited or aggregated and does not emit unbounded duplicate messages every frame.
  **Evidence**: Automated Test, debug counter, or profiling note.

- **Scope**: Phase 1 Integration · **Story Type**: Performance / Debug UI · **Gate**: Advisory unless debug tooling story is in scope
  **GIVEN** debug overlays are enabled, **WHEN** QA inspects cells or Y-sort anchors, **THEN** overlays are bounded to selected cells, visible regions, or explicit full-grid debug mode and are not used as normal gameplay performance evidence.
  **Evidence**: Manual/Debug Evidence.

### Map Authoring and Validation

- **Scope**: Phase 1 Integration · **Story Type**: Integration / Config/Data · **Gate**: Blocking for map validation handoff
  **GIVEN** a Phase 1 synthetic test map exposes authoring metadata, **WHEN** validation runs, **THEN** it can report player start, critical loop region, spawn region, item-placeable cells, drop-readability cells, visual obstruction regions, missing Y-sort anchors, and at least one complete player-start-to-spawn-to-drop-readable-item-placement path.
  **Evidence**: Integration Test or validation report.

- **Scope**: Phase 1 Integration · **Story Type**: Integration / Config/Data · **Gate**: Blocking for map validation handoff
  **GIVEN** spawn, item-placement, or drop-readability cells are unreachable from the player start or overlap incompatible visual obstruction regions, **WHEN** validation runs, **THEN** the issue is reported rather than silently accepted as a valid loot-loop map.
  **Evidence**: Integration Test or validation report.

### TBD Rules and Evidence Gates

- **Scope**: Evidence Gate · **Story Type**: QA Gate / Logic · **Gate**: Blocking when story claims source-authentic behavior
  **GIVEN** distance, speed, diagonal movement, corner-cutting, coordinate origin, axis direction, or projection rules are still **TBD from OpenMir2 source evidence**, **WHEN** an implementation story attempts to mark those rules complete, **THEN** the story must include approved source evidence or remain blocked/incomplete.
  **Evidence**: QA Gate.

- **Scope**: Evidence Gate · **Story Type**: QA Gate / Logic · **Gate**: Blocking when story claims source-authentic behavior
  **GIVEN** OpenMir2 evidence later defines movement speed or step timing, **WHEN** implementation uses that rule, **THEN** automated tests must cover normal values, boundary values, and invalid inputs before the relevant story can be marked complete.
  **Evidence**: Automated Unit Test once rule is defined.

- **Scope**: Evidence Gate · **Story Type**: QA Gate / Logic · **Gate**: Blocking when story claims source-authentic behavior
  **GIVEN** OpenMir2 evidence later defines diagonal or corner-cutting rules, **WHEN** actor diagonal movement is implemented, **THEN** automated tests must cover legal diagonal movement, out-of-bounds target, statically blocked target, actor-occupied target, reserved target, and any source-required orthogonal-neighbor blocking cases.
  **Evidence**: Automated Unit Test once rule is defined.

### Cross-System Integration and Definition of Done

- **Scope**: Phase 1 Integration · **Story Type**: Integration / QA Gate · **Gate**: Blocking for system completion
  **GIVEN** movement, AI, pickup, combat, loot, UI, or networking requests spatial legality, **WHEN** this system returns a failed structured query result, **THEN** the downstream system must not perform a conflicting state change or bypass occupancy/reservation state directly.
  **Evidence**: Integration Test.

- **Scope**: Phase 1 Integration · **Story Type**: Integration / QA Gate · **Gate**: Blocking for system completion
  **GIVEN** a test scene containing player, ordinary monster, dropped item, static blocker, reservation state, and sortable objects, **WHEN** movement reserve, movement commit, item placement, pickup query, and Y-sort are exercised together, **THEN** each behavior matches its contract, dropped items do not block movement, Y-sort does not alter gameplay, and occupied/reserved state does not leak or conflict.
  **Evidence**: Integration Test.

- **Scope**: Phase 1 Integration · **Story Type**: Integration / QA Gate · **Gate**: Blocking for system completion
  **GIVEN** QA opens a debug view or captures debug output for a logical cell, **WHEN** a cell is selected or inspected, **THEN** the evidence shows in-bounds status, map blocking state, actor occupancy, reservation state, dropped item list/count, and latest structured query reason.
  **Evidence**: Manual/Debug Evidence.

- **Scope**: Phase 1 Integration · **Story Type**: QA Gate · **Gate**: Blocking for system completion
  **GIVEN** this system enters completion review, **WHEN** QA checks formula coverage, **THEN** the evidence package lists unit test paths for `in_bounds`, `actor_enterable`, `item_placeable`, `reservation_commit_valid`, `y_sort_key`, and `query_result_priority`.
  **Evidence**: QA sign-off checklist with automated test paths.

- **Scope**: Phase 1 Integration · **Story Type**: QA Gate / Integration · **Gate**: Blocking for system completion
  **GIVEN** this system enters completion review, **WHEN** QA checks cross-system coexistence, **THEN** the evidence package includes at least one integration test path covering movement reservation, dropped item placement, pickup candidate query, and Y-sort coexistence in the same fixture.
  **Evidence**: QA sign-off checklist with integration test path.

- **Scope**: Evidence Gate · **Story Type**: QA Gate · **Gate**: Blocking for source-authentic completion
  **GIVEN** this system enters completion review, **WHEN** QA checks OpenMir2 evidence status, **THEN** the evidence package lists each unresolved source-evidence gate and marks whether the story claims only MVP provisional behavior or source-authentic behavior.
  **Evidence**: QA sign-off checklist with source-evidence status.

- **Scope**: Phase 1 Integration · **Story Type**: QA Gate / Manual Evidence · **Gate**: Blocking when Y-sort/debug visibility is in story scope
  **GIVEN** this system enters completion review for a story that includes Y-sort or debug visibility, **WHEN** QA checks manual/debug evidence, **THEN** the evidence package includes the relevant screenshot, debug capture, or exported debug report path.
  **Evidence**: QA sign-off checklist with manual/debug artifact path.

## Open Questions

| Question | Domain | Current Status | Needed Evidence / Decision | Blocks Implementation? | Owner |
|---|---|---|---|---|---|
| What are the authoritative OpenMir2 map coordinate origin, axis direction, and cell data semantics? | Coordinate Contract | TBD from source | E3/E4 source trace from OpenMir2 map files and object position code. | Yes, for final coordinate conversion. | technical-director / gameplay-programmer |
| Does one project logical grid cell exactly correspond to one OpenMir2 map cell in all required MVP contexts? | Coordinate Contract | Provisional yes | Source confirmation from OpenMir2 map and movement code. | Yes, for source-accurate implementation. | technical-director |
| What projection and render anchor should Godot use for the Phase 1 map? | Rendering / ADR | TBD | Architecture decision after engine reference and test map validation. | Yes, for implementation, not for GDD contract. | godot-specialist / technical-director |
| Is diagonal movement allowed in OpenMir2, and does diagonal movement allow corner-cutting? | Movement / Blocking | TBD from source | OpenMir2 movement legality chain. | Yes, for movement and AI. | gameplay-programmer |
| Which distance metric should combat, pickup, spawn, and AI use? | Distance Contract | TBD from source | OpenMir2 attack/pickup/movement evidence; later system GDD decisions. | Yes, for combat/pickup implementation. | systems-designer / gameplay-programmer |
| What are the exact attack range, pickup range, movement speed, spawn radius, and drop fallback radius values? | Downstream Values | Not owned by this GDD | OpenMir2 evidence and downstream system GDDs. | Yes, for those downstream systems. | relevant system owners |
| Can dropped items be placed on statically blocked cells in OpenMir2? | Drop Placement | TBD | OpenMir2 drop/ground item source trace. | Yes, for drop system. | economy-designer / gameplay-programmer |
| Does OpenMir2 allow multiple dropped items in one cell, and if so how are they selected for pickup? | Drop/Pickup | Phase 1 provisional pickup-complete contract uses `same_cell_item_capacity = 1`; multiple same-cell items are malformed/debug/future only until approved. | Source evidence or drop/pickup GDD decision. | Yes, for source-authentic item stacking or any future multi-item pickup ordering. | economy-designer / gameplay-programmer |
| Are player, monster, and NPC dynamic blocking semantics identical in OpenMir2? | Occupancy | MVP assumes player and ordinary monsters block other actors | Source evidence for actor collision/occupancy. | Yes, for movement/AI. | gameplay-programmer / ai-programmer |
| Should corpses, dead monsters, doors, portals, NPCs, or temporary skill objects block movement? | Future Occupancy | Out of MVP | Later GDDs and OpenMir2 evidence. | No for MVP; yes for future systems. | systems-designer |
| How should high static props, roofs, trees, and canopy layers interact with Y-sort? | Rendering | Contract reserved; MVP does not require complex overlay | Art bible, map asset spec, and rendering ADR. | No for MVP unless Phase 1 map requires it. | art-director / godot-specialist |
| What is the Godot 4.6.3 implementation approach for TileMapLayer or non-TileMap map data, logical map schema, collision helpers, input projection, and deterministic Y-sort? | Architecture | TBD | ADR after GDD approval; must verify post-cutoff Godot 4.6.3 behavior and avoid deprecated TileMap assumptions. | Yes, before coding. | technical-director / godot-specialist |
| What is the authoritative structured reason enum / typed query result schema for implementation? | Implementation Contract | Drafted in this GDD; needs ADR/API confirmation | Typed GDScript schema and test fixtures. | Yes, before coding. | gameplay-programmer / godot-specialist / qa-lead |
| What exact reservation timeout / max-age policy should the movement implementation use? | Movement / AI / Performance | GDD requires stale cleanup but does not set tick count. | Movement ADR or movement GDD. | Yes, before movement implementation. | gameplay-programmer / ai-programmer |
| What minimum player-facing affordance makes same-cell or actor-overlapped drops readable in Phase 1? | Loot Presentation | GDD requires readability floor; exact presentation TBD. | Drop/pickup GDD, rendering ADR, or visual feedback spec. | Yes, for loot-loop presentation completion. | economy-designer / technical-artist / godot-specialist |
| What authoring metadata can the resource/map conversion pipeline preserve or generate? | Map Authoring / Tools | Required fields defined provisionally. | Resource / map conversion pipeline Spike. | Yes, for source-like map validation. | tools-programmer / technical-director / level-designer |
