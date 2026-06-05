# Systems Index: 新玛法：觉醒客户端

> **Status**: Draft / Approved for GDD authoring
> **Created**: 2026-06-03
> **Last Updated**: 2026-06-05
> **Source Concept**: design/gdd/game-concept.md

> **TD-SYSTEM-BOUNDARY Note**: CONCERNS accepted — each system GDD must define `Owns / Reads / Writes / Emits / Listens`; avoid God Object boundaries; UI reads or invokes gameplay interfaces but never owns gameplay data; network/protocol remains Spike/Future only for Phase 1.
>
> **PR-SCOPE Note**: OPTIMISTIC accepted — Phase 1 is a thin slice, not a complete Legend/Mir client MVP. MVP systems below mean minimum verifiable implementation only.

---

## Overview

`新玛法：觉醒客户端` needs systems that prove a tight 30-second offline loot loop: click-move through a small 2D/2.5D map, kill a normal monster, see an exciting drop, pick it up, compare it, equip it, and immediately see the character become stronger. The first system set therefore prioritizes classic Legend/OpenMir2 behavior boundaries, Godot map/collision/sorting rules, item/drop/equipment data, basic combat, inventory/equipment flow, and clear reward feedback. Full MMO features, networking, social systems, multi-class depth, complex equipment growth, and polished long-term content are intentionally deferred until the thin slice proves `稳刷不断流`, `爆装有戏`, `传奇骨架，现代皮肤`, and `每次登录都带走成长`.

---

## Systems Enumeration

| # | System Name | Category | Priority | Status | Design Doc | Depends On |
|---|-------------|----------|----------|--------|------------|------------|
| 1 | OpenMir2 行为映射 Spike | Spike / Foundation | MVP | Designed | design/gdd/openmir2-behavior-mapping-spike.md | — |
| 2 | 地图坐标 / 阻挡 / Y-sort 系统 | Foundation | MVP | Needs Revision | design/gdd/map-coordinate-blocking-y-sort-system.md | OpenMir2 行为映射 Spike |
| 3 | 角色属性系统 | Foundation | MVP | Needs Revision | design/gdd/character-attributes-system.md | OpenMir2 行为映射 Spike |
| 4 | 物品定义系统 | Foundation | MVP | Approved | design/gdd/item-definition-system.md | OpenMir2 行为映射 Spike |
| 5 | 掉落表系统 | Foundation / Economy | MVP | Approved | design/gdd/drop-table-system.md | 物品定义系统 |
| 6 | 点击移动系统 | Core | MVP | Approved | design/gdd/click-movement-system.md | 地图坐标 / 阻挡 / Y-sort 系统 |
| 7 | 交互目标 / 选择系统 (inferred) | Core | MVP | Not Started | — | 点击移动系统; 地图坐标 / 阻挡 / Y-sort 系统 |
| 8 | 伤害计算系统 (inferred) | Core | MVP | Not Started | — | 角色属性系统 |
| 9 | 生命 / 死亡 / 复活规则 (inferred) | Core | MVP | Not Started | — | 角色属性系统 |
| 10 | 基础战斗系统 | Core | MVP | Not Started | — | 点击移动系统; 交互目标 / 选择系统; 伤害计算系统; 生命 / 死亡 / 复活规则 |
| 11 | 怪物生成系统 (inferred) | Core | MVP | Not Started | — | 地图坐标 / 阻挡 / Y-sort 系统; 角色属性系统 |
| 12 | 怪物 AI / 行为系统 (inferred) | Core | MVP | Not Started | — | 怪物生成系统; 基础战斗系统 |
| 13 | 掉落与拾取系统 | Core / Economy | MVP | Not Started | — | 掉落表系统; 基础战斗系统; 交互目标 / 选择系统; 物品定义系统 |
| 14 | 背包系统 | Core / Economy | MVP | Not Started | — | 物品定义系统; 掉落与拾取系统; 存档系统 |
| 15 | 装备系统 | Core / Progression | MVP | Not Started | — | 背包系统; 角色属性系统; 物品定义系统; 存档系统 |
| 16 | 极简 HUD 系统 (inferred) | Presentation / UI | MVP | Not Started | — | 角色属性系统; 基础战斗系统 |
| 17 | 掉落视觉 / 音效反馈系统 | Presentation / Audio | MVP | Not Started | — | 掉落与拾取系统; 掉落表系统; 物品定义系统 |
| 18 | 背包 / 装备 UI 系统 | Presentation / UI | MVP | Not Started | — | 背包系统; 装备系统; 物品定义系统 |
| 19 | 成长反馈系统 | Presentation / Progression | MVP | Not Started | — | 角色属性系统; 装备系统 |
| 20 | 存档系统 (inferred) | Persistence | MVP | Not Started | — | 角色属性系统; 物品定义系统 |
| 21 | 资源 / 地图转换管线 Spike | Spike / Tools | Spike | Not Started | — | — |
| 22 | 目标 / 引导系统 | Feature / UI | Vertical Slice+ | Not Started | — | 点击移动系统; 基础战斗系统; 掉落与拾取系统; 装备系统 |
| 23 | 地图推进 / 解锁系统 | Feature / Progression | Vertical Slice+ | Not Started | — | 角色属性系统; 装备系统; 目标 / 引导系统 |
| 24 | Boss / 精英怪系统 | Feature / Gameplay | Vertical Slice+ | Not Started | — | 怪物 AI / 行为系统; 掉落表系统; 地图推进 / 解锁系统 |
| 25 | 商店 / 回收系统 (inferred) | Economy | Alpha | Not Started | — | 背包系统; 物品定义系统; 存档系统 |
| 26 | 强化 / 材料成长系统 | Progression / Economy | Alpha | Not Started | — | 装备系统; 掉落表系统; 背包系统 |
| 27 | 技能系统 | Gameplay / Progression | Alpha | Not Started | — | 角色属性系统; 基础战斗系统; 存档系统 |
| 28 | NPC / 城镇功能系统 (inferred) | Feature / Narrative | Alpha | Not Started | — | 商店 / 回收系统; 目标 / 引导系统 |
| 29 | 网络 / 最小协议系统 | Future / Networking | Future / Polish | Not Started | — | OpenMir2 行为映射 Spike |
| 30 | 社交 / 组队 / 行会 / 交易系统 | Future / Social | Future / Polish | Not Started | — | 网络 / 最小协议系统; 物品定义系统; 地图推进 / 解锁系统 |
| 31 | 可访问性 / 输入辅助系统 (inferred) | Future / Accessibility | Future / Polish | Not Started | — | 极简 HUD 系统; 背包 / 装备 UI 系统; 点击移动系统 |
| 32 | 音频混音 / 奖励音效系统 (inferred) | Audio / Polish | Future / Polish | Not Started | — | 掉落视觉 / 音效反馈系统; 基础战斗系统; 背包 / 装备 UI 系统 |
| 33 | 本地化系统 (inferred) | Future / Localization | Future / Polish | Not Started | — | 极简 HUD 系统; 物品定义系统; 目标 / 引导系统 |
| 34 | 数据调试 / 开发工具 (inferred) | Tools / Meta | Future / Polish | Not Started | — | 物品定义系统; 掉落表系统; 基础战斗系统; 装备系统 |

---

## Categories

| Category | Description | Typical Systems |
|----------|-------------|-----------------|
| **Foundation** | Shared rules and data contracts that other systems depend on | map coordinates, stats, item definitions, drop tables |
| **Core** | Systems required to make the 30-second loop playable | click movement, targeting, combat, monster behavior, pickup |
| **Economy** | Resource and item creation, storage, valuation, and sinks | drops, inventory, shop/recycle, materials |
| **Progression** | How the character becomes stronger over time | equipment, stat growth, map unlocks, skills |
| **Persistence** | Save state and continuity boundaries | minimal save slots, snapshots, schema |
| **Presentation / UI** | Player-facing information and feedback | HUD, loot feedback, inventory/equipment UI, growth feedback |
| **Audio** | Reward and action sound feedback | loot stingers, pickup, equip, hit, mix polish |
| **Spike / Tools** | Technical-risk investigation and development support | OpenMir2 mapping, resource conversion, debug tools |
| **Future / Social / Networking** | Full-vision systems outside the Phase 1 thin slice | protocol, social, party, guild, trade |

---

## Priority Tiers

| Tier | Definition | Target Milestone | Design Urgency |
|------|------------|------------------|----------------|
| **MVP** | Minimum systems required to verify the offline 30-second loot loop. Implement as thin-slice boundaries, not full client systems. | First playable technical slice | Design FIRST |
| **Spike** | High-risk technical investigation required before production confidence, but not necessarily part of the playable loop. | Before architecture / early prototype | Design or research FIRST where blocking |
| **Vertical Slice+** | Systems that turn the thin slice into a more complete playable area with goals and escalation. | Vertical slice / demo | Design SECOND |
| **Alpha** | Wider gameplay and economy features after the core loop is proven. | Alpha milestone | Design THIRD |
| **Future / Polish** | Full-vision, social, localization, accessibility, networking, and tooling polish. | Beta / Release / later roadmap | Design as needed |

---

## Dependency Map

### Foundation Layer (no dependencies or upstream authority only)

1. OpenMir2 行为映射 Spike — defines the original-source authority for movement, combat, drops, inventory, and equipment behavior.
2. 资源 / 地图转换管线 Spike — validates asset legality, map format, sprite format, coordinate, and occlusion risks without blocking temporary-material MVP.
3. 角色属性系统 — defines base stats and stat contracts; equipment writes modifiers, not direct God Object state.
4. 物品定义系统 — defines item IDs, types, quality, icons, equipment data, and shared item metadata.
5. 掉落表系统 — depends on item definitions and defines monster-to-reward pools and weights.
6. 地图坐标 / 阻挡 / Y-sort 系统 — depends on OpenMir2 mapping; must separate logical coordinates, blocking queries, and visual sorting.
7. 存档系统 — depends on stats and item definitions; owns save slot, schema, and serialization only.

### Core Layer (depends on foundation)

1. 点击移动系统 — depends on map coordinates, blocking, and Y-sort rules.
2. 交互目标 / 选择系统 — depends on click movement and map rules; decides whether a click targets ground, monster, loot, or UI-adjacent interaction.
3. 伤害计算系统 — depends on character stats; calculates combat output without owning health state.
4. 生命 / 死亡 / 复活规则 — depends on character stats; owns HP/life state and emits death/failure events.
5. 基础战斗系统 — depends on movement, targeting, damage calculation, and life/death rules; coordinates attack flow.
6. 怪物生成系统 — depends on map rules and character/stat templates.
7. 怪物 AI / 行为系统 — depends on monster generation and combat.
8. 掉落与拾取系统 — depends on drop tables, combat death events, targeting, and item definitions; must be split by events: monster death, drop generation, ground loot, pickup request, inventory receive.
9. 背包系统 — depends on item definitions, pickup, and persistence boundary.
10. 装备系统 — depends on inventory, stats, items, and persistence boundary.

### Feature Layer (depends on core)

1. 目标 / 引导系统 — observes movement, combat, pickup, and equipment events; does not control core gameplay flow.
2. 地图推进 / 解锁系统 — depends on stats, equipment, and goals.
3. Boss / 精英怪系统 — depends on monster AI, drop tables, and map progression.
4. 商店 / 回收系统 — depends on inventory, items, and persistence.
5. 强化 / 材料成长系统 — depends on equipment, drops, and inventory.
6. 技能系统 — depends on stats, combat, and persistence.
7. NPC / 城镇功能系统 — depends on shop/recycle and goals.

### Presentation Layer (depends on features and core)

1. 极简 HUD 系统 — depends on stats and combat.
2. 掉落视觉 / 音效反馈系统 — depends on loot, drop table, and item quality data.
3. 背包 / 装备 UI 系统 — depends on inventory, equipment, and item definitions.
4. 成长反馈系统 — depends on stats and equipment.
5. 音频混音 / 奖励音效系统 — depends on loot feedback, combat, and UI events.

### Polish / Future Layer (depends on broad architecture)

1. 网络 / 最小协议系统 — depends on OpenMir2 mapping; Phase 1 Spike/Future only.
2. 社交 / 组队 / 行会 / 交易系统 — depends on networking, item definitions, and map progression.
3. 可访问性 / 输入辅助系统 — depends on HUD, inventory/equipment UI, and click movement.
4. 本地化系统 — depends on UI, items, and goals.
5. 数据调试 / 开发工具 — depends on item, drop, combat, and equipment data.

---

## Recommended Design Order

| Order | System | Priority | Layer | Agent(s) | Est. Effort |
|-------|--------|----------|-------|----------|-------------|
| 1 | OpenMir2 行为映射 Spike | MVP | Foundation / Spike | technical-director, gameplay-programmer | M |
| 2 | 地图坐标 / 阻挡 / Y-sort 系统 | MVP | Foundation | godot-specialist, gameplay-programmer | L |
| 3 | 角色属性系统 | MVP | Foundation | systems-designer, gameplay-programmer | M |
| 4 | 物品定义系统 | MVP | Foundation | economy-designer, systems-designer | M |
| 5 | 掉落表系统 | MVP | Foundation | economy-designer, systems-designer | M |
| 6 | 点击移动系统 | MVP | Core | gameplay-programmer, godot-specialist | M |
| 7 | 交互目标 / 选择系统 | MVP | Core | gameplay-programmer, ux-designer | M |
| 8 | 伤害计算系统 | MVP | Core | systems-designer, gameplay-programmer | M |
| 9 | 生命 / 死亡 / 复活规则 | MVP | Core | systems-designer, gameplay-programmer | M |
| 10 | 基础战斗系统 | MVP | Core | game-designer, gameplay-programmer | L |
| 11 | 怪物生成系统 | MVP | Core | systems-designer, gameplay-programmer | M |
| 12 | 怪物 AI / 行为系统 | MVP | Core | ai-programmer, gameplay-programmer | M |
| 13 | 掉落与拾取系统 | MVP | Core | economy-designer, gameplay-programmer | L |
| 14 | 背包系统 | MVP | Core | economy-designer, gameplay-programmer | M |
| 15 | 装备系统 | MVP | Core | systems-designer, gameplay-programmer | M |
| 16 | 极简 HUD 系统 | MVP | Presentation | ux-designer, godot-specialist | S |
| 17 | 掉落视觉 / 音效反馈系统 | MVP | Presentation | technical-artist, sound-designer | M |
| 18 | 背包 / 装备 UI 系统 | MVP | Presentation | ux-designer, godot-specialist | M |
| 19 | 成长反馈系统 | MVP | Presentation | systems-designer, technical-artist | M |
| 20 | 存档系统 | MVP | Foundation / Persistence | technical-director, gameplay-programmer | M |
| 21 | 资源 / 地图转换管线 Spike | Spike | Foundation / Tools | technical-director, tools-programmer | L |
| 22 | 目标 / 引导系统 | Vertical Slice+ | Feature | game-designer, ux-designer | M |
| 23 | 地图推进 / 解锁系统 | Vertical Slice+ | Feature | systems-designer, level-designer | M |
| 24 | Boss / 精英怪系统 | Vertical Slice+ | Feature | game-designer, ai-programmer | L |
| 25 | 商店 / 回收系统 | Alpha | Feature / Economy | economy-designer, gameplay-programmer | M |
| 26 | 强化 / 材料成长系统 | Alpha | Feature / Progression | economy-designer, systems-designer | L |
| 27 | 技能系统 | Alpha | Feature / Gameplay | game-designer, gameplay-programmer | L |
| 28 | NPC / 城镇功能系统 | Alpha | Feature | narrative-director, gameplay-programmer | M |
| 29 | 网络 / 最小协议系统 | Future / Polish | Future | network-programmer, technical-director | L |
| 30 | 社交 / 组队 / 行会 / 交易系统 | Future / Polish | Future | network-programmer, economy-designer | L |
| 31 | 可访问性 / 输入辅助系统 | Future / Polish | Polish | accessibility-specialist, ux-designer | M |
| 32 | 音频混音 / 奖励音效系统 | Future / Polish | Polish | audio-director, sound-designer | S |
| 33 | 本地化系统 | Future / Polish | Polish | localization-lead, ux-designer | M |
| 34 | 数据调试 / 开发工具 | Future / Polish | Tools | tools-programmer, systems-designer | M |

Effort estimates: S = 1 session, M = 2–3 sessions, L = 4+ sessions.

---

## Circular Dependencies

No blocking circular dependencies found.

- 装备系统 ↔ 角色属性系统: equipment changes stats, and stats drive equipment value display. Resolution: character stats own the base stat interface; equipment only contributes `StatModifier` data; combat and UI read aggregated derived stats.
- 背包 / 装备 UI ↔ 背包 / 装备系统: UI needs inventory/equipment data, but rules must not be shaped by UI widgets. Resolution: design gameplay rules first; UI listens to events and invokes interfaces only.
- 地图坐标 / 阻挡 / Y-sort ↔ 资源 / 地图转换管线 Spike: conversion findings may reshape map assumptions. Resolution: define a temporary Godot map contract first, then use the spike to validate whether real resources can adapt.

---

## High-Risk Systems

| System | Risk Type | Risk Description | Mitigation |
|--------|-----------|------------------|------------|
| OpenMir2 行为映射 Spike | Technical / Design | Original behavior is the authority; guessing movement, combat, loot, inventory, or equipment rules could shift the game away from classic Legend feel. | Read original OpenMir2 source first; produce a mapping table before detailed GDDs rely on behavior rules. |
| 地图坐标 / 阻挡 / Y-sort 系统 | Technical | Godot adaptation of grid coordinates, blocked cells, Y-sort, occlusion, and loot visibility can break movement flow and item readability. | Prototype temporary Godot map contract early; keep logical coordinate, blocking, and visual sorting responsibilities separate. |
| 角色属性系统 | Architecture / Scope | It can become a God Object if every combat, equipment, progression, and UI rule writes directly into it. | Define stat ownership and `StatModifier` boundaries; derived combat power is display/read-only. |
| 物品定义系统 | Architecture / Scope | Drops, inventory, equipment, UI, persistence, and economy all depend on item identity and metadata. | Use data-driven item definitions and validation; avoid UI-owned item data. |
| 掉落与拾取系统 | Design / Technical | This carries the `爆装有戏` pillar; if drops are unclear, unexciting, or hard to pick up, the core loop fails. | Split event chain; prototype ground loot visibility, pickup range, and feedback before expanding content. |
| 背包 / 装备 UI 系统 | Scope / UX | Looks small but can expand into complex UI, comparison logic, drag/drop rules, and data coupling. | MVP only supports minimal inventory, equipment slots, and power/main-stat comparison. |
| 资源 / 地图转换管线 Spike | Legal / Technical | Asset legality, map format, sprite format, coordinate conversion, and occlusion may be harder than gameplay code. | Treat as a separate spike; Phase 1 may use temporary assets until legality and conversion path are proven. |

---

## Progress Tracker

| Metric | Count |
|--------|-------|
| Total systems identified | 34 |
| Design docs started | 6 |
| Design docs reviewed | 6 |
| Design docs approved | 3 |
| MVP systems designed | 6/20 |
| Vertical Slice+ systems designed | 0/3 |

---

## Next Steps

- [ ] Run `/gate-check systems-design` for formal CD-SYSTEMS and TD-SYSTEM-BOUNDARY sign-off before locking the system set across many GDDs.
- [ ] Design MVP-tier systems first with `/design-system [system-name]`.
- [ ] Start with `/design-system OpenMir2 行为映射 Spike` or `/design-system 地图坐标 / 阻挡 / Y-sort 系统`, depending on whether the next session should focus on behavior authority or Godot map contract.
- [ ] Run `/design-review` on each completed GDD.
- [ ] Run `/review-all-gdds` after a related MVP batch is authored.
- [ ] Run `/gate-check pre-production` when MVP systems are designed and reviewed.
- [ ] Validate the highest-risk systems with `/prototype 30秒刷怪爆装循环` or `/vertical-slice` before committing to Production.
