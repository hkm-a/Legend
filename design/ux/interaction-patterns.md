# Interaction Patterns

## Overview

This document defines the baseline player-facing interaction patterns for the Phase 1 2D/2.5D offline loot-loop slice. The UX goal is to make movement, targeting, pickup, inventory management, and equipment changes feel immediate, readable, and low-friction while preserving an OpenMir2-inspired mouse-driven mental model.

Phase 1 prioritizes keyboard/mouse PC play. Gamepad support is secondary and partial. Touch support is out of scope.

## UX Pillars

### Direct Intent

The player should feel that clicking the world expresses intent directly: move there, attack that, pick up that item, inspect that object.

Common, reversible actions should not require confirmation. Destructive, irreversible, or ambiguous actions must require explicit confirmation or a safer alternative flow.

### Always Explain Failure

When an action cannot happen, the player should know why. Common failure examples include blocked cells, no valid path, out-of-range targets, inventory full, invalid equipment slot, unmet item requirements, target already gone, and action cooldown.

### Low-Friction Loot Loop

The player should be able to move, fight, collect, inspect, equip, and continue playing with minimal menu friction. Pickup, inventory opening, comparison, and equipment changes should require few inputs and produce immediate feedback.

### Progressive Disclosure

Default UI should show the information needed for the current decision. Deeper stats, formulas, and item details may be exposed through hover, focus, compare, or expanded views.

## Input Conventions

### Keyboard/Mouse Baseline

| Action | Primary Input | Secondary Input | UX Notes |
|---|---|---|---|
| Move to cell/world point | Left-click ground | Hold left-click to continue pathing if supported | Click target should show destination feedback. |
| Select/target enemy | Left-click enemy | Target-cycle key, if implemented | Selection ring or highlight required. |
| Attack target | Left-click hostile target | Hotkey action if target selected | If out of range, use the approved out-of-range behavior below. |
| Interact / pickup | Left-click item, NPC, or object | Interact key, such as `F` | Use consistent cursor/highlight affordance. |
| Open inventory | `I` | UI button | Inventory must be keyboard navigable once open. |
| Open character/equipment | `C` | UI button | May be combined with inventory for Phase 1. |
| Close current panel | `Esc` | Panel close button | Closes the topmost panel before opening pause/menu. |
| Use consumable / item action | Right-click item | Context menu / hotkey | Right-click performs the default safe action. |
| Equip item | Right-click equippable item | Drag to equipment slot | Default behavior should be predictable. |
| Drop item | Context menu only | Drag outside inventory if later approved | Destructive/valuable drops need confirmation. |
| Compare item | Hover/focus item while equipment context is available | Modifier key or expanded tooltip | Comparison should not require a click. |
| Show advanced tooltip | Hold modifier or toggle expanded view | Tooltip expand button | Keeps default tooltips readable. |

All core actions should be represented as named Godot input actions so remapping can be added without rewriting interaction logic.

### Gamepad Baseline — Partial Support

Gamepad support is partial in Phase 1 and should focus on coherent UI navigation plus basic interact/pickup support. Full combat targeting parity may be deferred.

| Action | Suggested Gamepad Input | UX Notes |
|---|---|---|
| Navigate UI | D-pad / left stick | Visible focus indicator required. |
| Confirm UI action | South face button | Same as click/activate. |
| Cancel / close | East face button | Mirrors `Esc` behavior. |
| Open inventory | Menu/View button | Enters focus navigation mode. |
| Context action | West/North face button | Opens item context menu or safe default. |
| Interact / pickup nearest valid object | South face button | Optional gameplay support for Phase 1. |
| Select nearest target | Shoulder button | Optional; defer if combat targeting is unstable. |

Supported gamepad screens must never hide focus or trap the player without a cancel path.

### Godot 4.6 Focus Model

Godot 4.6 separates mouse/touch focus from keyboard/gamepad focus. UI visuals must distinguish:

- Mouse hover.
- Keyboard/gamepad focus.
- Persistent selection.
- Disabled/unavailable state.

A hovered inventory item and a keyboard-focused inventory item may be different controls. UI implementation must not assume `grab_focus()` affects mouse hover, and testing must cover both mouse and keyboard/gamepad navigation.

## World Interaction Patterns

### Click-to-Move

When the player left-clicks a valid walkable cell or world point:

1. Show immediate click feedback at the destination.
2. Start movement/pathing within the same frame or next visible update.
3. If the destination is reachable, move toward it.
4. If the destination is invalid or unreachable, show blocked/invalid feedback.
5. Repeated clicks replace the current destination predictably.

| State | Required Feedback |
|---|---|
| Valid destination | Ground marker, movement starts, optional click sound. |
| Blocked destination | Invalid marker, short error sound, optional “Blocked” text. |
| No path | Invalid marker and “Cannot reach.” |
| Destination changed | New marker replaces old marker. |
| Movement interrupted | Character stops; reason shown if caused by failure. |

If a player clicks an item or enemy out of range, the recommended Phase 1 behavior is auto-move toward interaction range when pathing/engagement logic is stable. If that logic is not yet stable, fail clearly with “Too far away” or “Cannot reach.”

### Targeting and Combat Intent

When the player hovers over a hostile target:

- Show hostile highlight/outline.
- Show target name/health if available.
- Change cursor or show an attack affordance.

When the player clicks a hostile target:

- Select the target.
- If in range, attack.
- If out of range, auto-move into range if the movement/combat integration is stable; otherwise show “Too far away.”

| State | Required Feedback |
|---|---|
| Target hovered | Highlight/outline. |
| Target selected | Persistent selection ring/indicator. |
| Attack started | Attack animation, sound, or combat text. |
| Out of range | Range/error message or auto-approach. |
| Target invalid/dead | Selection clears safely. |
| Attack blocked/cooldown | Cooldown/invalid action feedback. |

Target categories must not rely on color alone. Use shape, outline style, iconography, nameplate treatment, or text backup.

### Pickup Interaction

Ground loot should be readable and quickly collectible.

Recommended baseline:

- Hovering loot displays item name.
- Left-clicking loot attempts pickup.
- If out of range, auto-move toward pickup range when stable; otherwise show “Too far away.”
- Pickup success shows visual feedback and updates inventory.
- Pickup failure explains why.

| Failure | Player Message |
|---|---|
| Inventory full | “Inventory full.” |
| Item no longer exists | “Item is gone.” |
| Too far away | “Too far away.” or auto-move. |
| Blocked path | “Cannot reach item.” |
| Pickup not allowed | “Cannot pick up.” |

If multiple items overlap, priority is: item directly under cursor, closest item to cursor center, then explicit item selection. Do not silently prioritize rarity/value in a way that surprises the player.

## Inventory Interaction Pattern

### Opening and Closing Inventory

- `I` opens/closes inventory.
- `Esc` closes inventory if it is the topmost panel.
- Inventory should not obscure the character or immediate loot/combat state more than necessary.
- Opening inventory does not pause gameplay unless a future design decision explicitly changes this.

| State | Requirement |
|---|---|
| Open | Grid/list visible, item slots readable. |
| Empty | Empty slots clearly shown. |
| Full | Capacity state visible before or during pickup failure. |
| Focused | Keyboard/gamepad focus indicator visible. |
| Hovered item | Tooltip appears. |
| Selected item | Persistent selected state for keyboard/gamepad. |

### Item Tooltip Pattern

Default tooltip should show:

- Item name.
- Item type.
- Equip slot if equippable.
- Core stats/effects.
- Requirement failure if relevant.
- Safe available actions.

Expanded tooltip may show full stat breakdown, source modifiers, flavor text, or advanced values.

Tooltip rules:

- Tooltips appear on mouse hover and keyboard/gamepad focus.
- Tooltips remain within screen bounds.
- Tooltips should not cover the compared equipment slot when avoidable.
- Tooltip text must be readable at supported UI scales.
- Color cannot be the only rarity/stat-change indicator.

### Item Default Actions

| Item Type | Right-Click Default |
|---|---|
| Equippable item | Equip if valid. |
| Consumable item | Use if valid. |
| Quest/key item | Inspect or no-op with explanation. |
| Material/misc item | Inspect/context menu. |
| Invalid/locked item | Show reason. |

Right-click must never perform destructive actions by default. Dropping, destroying, selling, or consuming rare/valuable items requires a deliberate command and, where appropriate, confirmation.

## Equipment Interaction Pattern

### Equip from Inventory

Primary flow:

1. Open inventory.
2. Hover or focus item to see tooltip.
3. If equipment context is available, show comparison.
4. Right-click or confirm an equippable item to equip.
5. Item moves to the correct slot.
6. Previously equipped item moves to inventory if applicable.
7. Stats update with clear feedback.

Secondary flow:

- Drag item from inventory to valid equipment slot if drag/drop is implemented.
- Valid slots highlight.
- Invalid slots reject with explanation.
- A non-drag alternative must exist.

| State | Feedback |
|---|---|
| Can equip | Valid slot highlight / tooltip says “Equip.” |
| Equipped successfully | Slot update, stat change flash, sound. |
| Cannot equip | Error text explains why. |
| Replaced item | Old item appears in inventory or swaps safely. |
| Inventory full on unequip/swap | Prevent action and explain. |
| Stat changed | Clear positive/negative comparison. |

### Unequip Flow

- Right-click or confirm an equipped item to unequip if inventory has space.
- Drag equipped item to inventory slot if drag/drop is implemented.
- If inventory is full, do not unequip and show “Inventory full.”
- Never delete, drop, or overwrite equipment as a side effect of failed unequip/swap.

### Comparison Display

| Stat Change | Display |
|---|---|
| Better | Up indicator plus numeric delta. |
| Worse | Down indicator plus numeric delta. |
| Unchanged | Neutral indicator or omitted. |
| Requirement not met | Requirement line with non-color indicator. |
| Different item type | “Cannot compare” or type mismatch notice. |

Do not rely on red/green alone. Use symbols, text labels, icons, or numeric deltas in addition to color.

## Error Handling and Player Messaging

Error messages should be short, specific, actionable where possible, non-punitive in tone, and consistent across systems.

| Poor | Better |
|---|---|
| “Failed.” | “Inventory full.” |
| “Invalid.” | “Cannot equip: requires Level 5.” |
| “No.” | “Cannot reach target.” |
| “Error 102.” | “Item is no longer available.” |

Use layered feedback:

1. Immediate visual feedback near the action location.
2. Optional short text near cursor/character.
3. Optional sound for invalid action.
4. Persistent message only for important/systemic failures.

Repeated identical errors should be throttled to avoid spam.

## Feedback State Matrix

| Player Action | Success Feedback | Failure Feedback | Persistent State |
|---|---|---|---|
| Click ground | Destination marker, movement. | Invalid marker / sound. | Current path/destination. |
| Click enemy | Highlight, attack/move. | Out-of-range/no-path message. | Selected target. |
| Pick up item | Item toast, sound, inventory update. | Inventory/full/reach error. | Item removed from ground. |
| Open inventory | Panel opens, focus set. | N/A | Inventory visible. |
| Hover/focus item | Tooltip. | N/A | Hover/focus/selection state. |
| Equip item | Slot update, stat flash. | Cannot equip reason. | Equipment changed. |
| Unequip item | Item moves to inventory. | Inventory full. | Equipment changed. |
| Drag item | Slot highlights. | Invalid drop feedback. | Drag preview. |
| Press Esc | Top panel closes. | N/A | UI stack changes. |

## UI Navigation Rules

### Keyboard

- Arrow keys or Tab move focus between UI controls according to screen-specific rules.
- Enter/Space confirms focused control.
- Esc cancels/closes topmost panel.
- Focus order must match visual/logical reading order.

### Gamepad

- D-pad/left stick navigates focus.
- South face button confirms.
- East face button cancels.
- Focus must be visible at all times.
- No UI screen may trap focus without a cancel path.

### Mouse

- Hover states should be immediate.
- Click targets must be large enough for reliable use.
- Important actions should not require pixel-perfect clicking.

## Acceptance Criteria

### Movement

- [ ] Left-clicking a valid walkable destination moves the character or starts pathing.
- [ ] Left-clicking an invalid/blocked destination provides immediate invalid feedback.
- [ ] Repeated movement clicks update the destination without UI confusion.
- [ ] Movement failure explains whether the issue is blocked terrain, no path, or invalid target.

### Targeting and Combat Intent

- [ ] Hostile targets visibly highlight on hover.
- [ ] Selected targets have a persistent visible indicator.
- [ ] Clicking a valid hostile target produces attack intent or clear out-of-range feedback.
- [ ] If a target disappears/dies, selection clears safely.

### Pickup

- [ ] Hovering ground loot identifies the item.
- [ ] Clicking reachable loot attempts pickup.
- [ ] Successful pickup updates inventory and gives visible/audio feedback.
- [ ] Pickup failure explains the reason, especially inventory full or unreachable item.

### Inventory

- [ ] Inventory opens/closes with keyboard and UI controls.
- [ ] Items show readable tooltips.
- [ ] Inventory can be navigated without a mouse.
- [ ] Inventory full state is visible or clearly messaged.

### Equipment

- [ ] Equippable items can be equipped through a low-friction action.
- [ ] Invalid equipment attempts explain why.
- [ ] Unequipping never deletes or drops items unexpectedly.
- [ ] Stat changes are communicated through more than color alone.

### Error Handling

- [ ] Common failed actions have specific messages.
- [ ] Repeated invalid actions do not spam the player.
- [ ] Error messages are short, consistent, and actionable.

### Accessibility Alignment

- [ ] Hover-only tooltip information is also available by focus/selection.
- [ ] Focus, hover, selection, and disabled states are visually distinct.
- [ ] All core interactions use named input actions suitable for future remapping.
- [ ] Drag/drop interactions have non-drag alternatives.
