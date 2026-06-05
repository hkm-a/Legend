# Accessibility Requirements

## Overview

This document defines accessibility requirements for the Phase 1 2D/2.5D offline loot-loop technical slice. The goal is to ensure core gameplay and UI interactions remain usable across keyboard/mouse and partial gamepad input, with readable UI, non-color-dependent feedback, scalable text, clear error states, and safe interaction flows.

The project targets a PC-native Godot 4.6.3 client. Keyboard/mouse is primary, gamepad support is partial, and touch support is out of scope for Phase 1.

## Accessibility Baseline

The default target is WCAG 2.1 Level AA where applicable to UI, text, input, status messaging, and audiovisual presentation, adapted for a PC-native game context.

Every Phase 1 player-facing feature should be evaluated against this baseline:

- [ ] Usable with keyboard only where UI interaction is required.
- [ ] Usable with gamepad only for supported UI and mapped core actions.
- [ ] Text readable at minimum supported UI scale and resolution.
- [ ] Functional without relying on color alone.
- [ ] No unsafe flashing content.
- [ ] Subtitles/captions available for dialogue or meaningful voiced content if present.
- [ ] UI scales correctly at supported resolutions.
- [ ] Error states are perceivable through more than one channel where practical.
- [ ] Focus state is visible for all keyboard/gamepad navigable UI.
- [ ] Controls are remappable or prepared for remapping where implementation scope allows.

Accessibility requirements are release-quality constraints, not polish-only features. Blocking accessibility issues must be reported to production as release blockers.

## Godot 4.6.3 Accessibility Context

Godot 4.5+ introduced AccessKit-based screen reader support for Control nodes. Godot 4.6 also uses a dual-focus model where mouse/touch focus is separate from keyboard/gamepad focus.

Implementation consequences:

- Control-based UI should be preferred for menus, inventory, equipment, settings, and dialogs.
- Icon-only controls need visible text equivalents or accessible names where engine support permits.
- Keyboard/gamepad focus visuals must be tested separately from mouse hover visuals.
- Status messages should be structured so they can later be announced or surfaced to assistive technologies.
- Godot 4.6.3 is post-cutoff for LLM memory; implementation details must be verified against engine reference docs or official docs before coding.

## Input Accessibility

### Keyboard-Only Requirements

All menus and inventory/equipment flows must be operable without a mouse.

Required keyboard capabilities:

- Open/close inventory.
- Move focus between UI elements.
- Select inventory items.
- Activate default item action.
- Open item context actions if context menus exist.
- Equip/unequip items.
- Close panels with `Esc`.
- Recover from every UI state without mouse input.

Acceptance criteria:

- [ ] Player can open inventory, inspect an item, equip it, unequip it, and close inventory using keyboard only.
- [ ] Keyboard focus is always visible.
- [ ] Focus order is predictable and follows visual layout.
- [ ] No panel traps keyboard focus.

### Mouse Requirements

Mouse interactions should remain forgiving and readable.

Required mouse capabilities:

- Interactive targets have hover states.
- Click targets are large enough for reliable use.
- Tooltips remain on-screen.
- Drag/drop, if supported, has a non-drag alternative.
- Right-click default actions are safe and non-destructive.

Acceptance criteria:

- [ ] Every mouse-clickable UI element has hover feedback.
- [ ] Drag-to-equip has an alternate right-click, context-menu, or keyboard method.
- [ ] Dropping/destroying valuable items is never the accidental default action.

### Gamepad Requirements

Phase 1 gamepad support is partial, but supported areas must be coherent.

Required for supported UI:

- Visible focus indicator.
- D-pad/left-stick focus navigation.
- Confirm/cancel mapping.
- Inventory navigation.
- Equipment slot navigation.
- No mouse-only required action inside supported screens.

Optional for gameplay:

- Nearest-target selection.
- Interact/pickup nearest valid object.
- Open/close inventory and character panels.

Acceptance criteria:

- [ ] Player can navigate supported menus with gamepad only.
- [ ] Gamepad focus never becomes invisible.
- [ ] Player can cancel/back out of every supported gamepad UI state.
- [ ] Unsupported gamepad gameplay actions are documented rather than implied.

### Remapping and Motor Accessibility

Required:

- Core actions are represented as named Godot input actions, not hardcoded key checks.
- Future input remapping must be possible without redesigning the interaction layer.
- Required simultaneous button presses are prohibited unless an alternative exists.
- Hold actions must have configurable duration or a toggle alternative where relevant.
- Double-click must not be the only way to perform progression-critical actions.

Acceptance criteria:

- [ ] Core gameplay and UI actions have named input action identifiers.
- [ ] No required progression action depends exclusively on a multi-button chord.
- [ ] Hold or repeated-input interactions have a documented accessibility alternative if they become progression-critical.

## Visual Accessibility

### Text Readability

Required:

- Gameplay UI text must be readable at minimum supported resolution.
- Tooltip text must remain legible at minimum supported resolution.
- Text should support UI scaling.
- Avoid dense stat blocks in default tooltip views.
- Avoid all-caps for long body text.
- Use adequate line spacing for dialogue, tutorials, and item descriptions.

Recommended Phase 1 baseline:

- Body/UI text: no smaller than 16 px equivalent at 100% scale.
- Tooltip/stat text: no smaller than 14–16 px equivalent at 100% scale.
- Plan for text scaling up to 200% before release gates.

Acceptance criteria:

- [ ] Inventory item names and tooltips are readable at minimum supported resolution.
- [ ] UI scaling does not clip key labels, tooltips, buttons, or item stats.
- [ ] Text contrast is sufficient against its panel background.

### Contrast Requirements

Required:

- Normal text contrast target: at least 4.5:1.
- Large text contrast target: at least 3:1.
- UI component and focus indicator contrast target: at least 3:1.
- Disabled controls should remain legible enough to understand unavailable options.
- In-world labels must maintain readability against variable map art through outline, drop shadow, backplate, or adaptive contrast treatment.

Acceptance criteria:

- [ ] Text, tooltips, disabled states, and focus outlines pass contrast checks against their intended backgrounds.
- [ ] In-world item labels remain readable on representative Phase 1 map backgrounds.

### Color Independence

Color may reinforce meaning but must not be the only carrier of meaning.

| Information Type | Required Non-Color Support |
|---|---|
| Item rarity | Text label, border pattern, icon, rarity name, or frame style. |
| Stat increase/decrease | Arrow/symbol and numeric delta. |
| Valid/invalid action | Shape, icon, text, pattern, or animation. |
| Enemy/friendly state | Label, outline shape, cursor state, or nameplate. |
| Health/damage state | Numeric/textual indication where needed. |
| Equipment slot compatibility | Slot icon, text reason, or shape cue. |

Acceptance criteria:

- [ ] Player can understand item comparison without relying on red/green.
- [ ] Invalid actions are distinguishable without relying only on red.
- [ ] Rarity or item importance is communicated through text/iconography as well as color.

### Motion, Flashing, and VFX

Required:

- No rapid flashing above seizure safety thresholds.
- Avoid repeated high-contrast flashes for errors, hits, or loot.
- Screen shake, if used, should be subtle and configurable where feasible.
- UI feedback animations should not block interaction.
- Essential feedback must remain readable if animation is reduced or missed.

Acceptance criteria:

- [ ] No UI or combat feedback flashes at unsafe rates.
- [ ] Essential feedback is still readable when animation is reduced.
- [ ] Error feedback does not depend solely on a quick animation.

## Audio and Caption Accessibility

If dialogue, voice barks, or meaningful audio cues exist in Phase 1:

Required:

- Subtitles for all dialogue.
- Captions or visual equivalents for important audio-only cues.
- Separate volume controls where audio settings exist: Master, Music, SFX, Voice if applicable, and UI if applicable.

For non-dialogue Phase 1 feedback:

- Pickup success has visual feedback, not only sound.
- Equip success/failure has visual feedback, not only sound.
- Error states have visual feedback, not only sound.
- Combat/action readiness does not rely only on audio cues.

Acceptance criteria:

- [ ] All voiced dialogue has subtitles if dialogue exists.
- [ ] Pickup, equip, invalid action, and combat feedback remain understandable with audio muted.
- [ ] Audio cues supplement visual feedback rather than replacing it.

## Cognitive Accessibility

### Progressive Disclosure

Required:

- Default tooltips show decision-critical information first.
- Advanced stats can be hidden behind a modifier key, expanded view, or details panel.
- Error messages use consistent phrasing.
- Icons should have labels or explanation where first encountered.
- Tutorial/help messages should be clear, concise, and replayable when implemented.

Acceptance criteria:

- [ ] Player can identify what an item is and whether it is useful from the default tooltip.
- [ ] Advanced details do not crowd out basic item readability.
- [ ] Similar failures use similar message structure.

### Consistency

Required:

- Same input should produce the same category of action across contexts.
- `Esc` consistently closes/cancels the topmost UI.
- Right-click consistently performs safe item default actions.
- Invalid actions use consistent feedback language.
- Destructive actions are separated from default actions.

Acceptance criteria:

- [ ] Inventory and equipment screens use consistent confirm/cancel behavior.
- [ ] Player can predict whether right-click will equip, use, inspect, or do nothing.
- [ ] Destructive actions require deliberate confirmation or are disabled until designed.

## Error Accessibility

Required:

- Error messages must be visible long enough to read.
- Error feedback should be near the player’s focus when possible.
- Repeated identical errors should be throttled to avoid spam.
- Disabled UI controls should explain why if hovered/focused.
- Important failures should not be communicated by sound or color alone.

Common message standards:

| Situation | Message |
|---|---|
| Inventory full | “Inventory full.” |
| Cannot equip due to level | “Cannot equip: requires Level X.” |
| Cannot equip due to class/stat | “Cannot equip: requirement not met.” |
| Target unreachable | “Cannot reach target.” |
| Item unreachable | “Cannot reach item.” |
| Action on cooldown | “Not ready.” |
| Invalid slot | “Wrong slot.” |
| Item gone | “Item is gone.” |

Acceptance criteria:

- [ ] Every common failed action has a specific, readable message.
- [ ] Disabled controls expose a reason through tooltip/focus text where relevant.
- [ ] Error messages do not rely only on sound or color.

## Inventory and Equipment Accessibility

Required:

- Inventory supports mouse and keyboard operation.
- Gamepad operation is supported for inventory/equipment if gamepad UI support is claimed in Phase 1.
- Item tooltips are readable and remain on-screen.
- Equipment changes communicate success/failure.
- Stat comparison does not use color alone.
- Item movement never silently deletes or overwrites items.
- Drag/drop has non-drag alternatives.

Acceptance criteria:

- [ ] Player can inspect inventory items without mouse.
- [ ] Player can equip/unequip without drag-and-drop.
- [ ] Player can compare an inventory item to equipped gear.
- [ ] Inventory-full prevents unsafe unequip/swap and explains why.
- [ ] Tooltips are readable and not clipped at supported resolutions.

## Resolution and Scaling Requirements

Required:

- UI supports the project’s minimum and target PC resolutions.
- Panels anchor predictably.
- Tooltips clamp to screen bounds.
- Inventory/equipment UI avoids critical information near unsafe screen edges.
- Text and icons remain legible after scaling.
- Localization expansion and text scaling should be tested together before release gates.

Acceptance criteria:

- [ ] Inventory, equipment, tooltip, and error messages remain visible at minimum resolution.
- [ ] UI scaling does not overlap core controls.
- [ ] Tooltip placement adjusts near screen edges.
- [ ] Focus indicators remain visible at all supported UI scales.

## Screen Reader and Assistive Technology Baseline

Implementation will be scoped separately after validating Godot 4.6.3 AccessKit behavior, but UI must not be authored in a way that blocks future assistive technology support.

Required planning constraints:

- UI controls should have meaningful names where engine support permits.
- Icon-only controls need text equivalents or accessibility labels.
- Inventory items should expose structured data for future accessibility: item name, type, rarity, equipped/unequipped state, requirements, main stats, and available actions.
- Status messages should be structured for future announcement.
- Custom-drawn UI must justify why standard Control nodes are not sufficient and must preserve an accessibility path.

Acceptance criteria:

- [ ] New UI specs identify text labels or accessible names for icon-only controls.
- [ ] Tooltip/item data is structured rather than only painted into images.
- [ ] Status messages have stable message IDs or categories for future announcement/localization.

## Accessibility Settings Baseline

The following settings are planned baseline candidates. They may be phased by milestone, but UI architecture should not make them impossible.

Visual:

- Text size.
- UI scale.
- High-contrast UI.
- Colorblind palette or non-color reinforcement mode.
- Reduce motion.
- Disable/reduce screen shake.
- Reduce flashes.
- Subtitle size.
- Subtitle background opacity.

Audio:

- Master volume.
- Music volume.
- SFX volume.
- Voice volume if voice exists.
- UI volume if UI sound exists.
- Mono audio.
- Dynamic range or loud sound reduction if needed.

Input:

- Input remapping.
- Hold/toggle alternatives.
- Hold duration.
- Repeat delay.
- Mouse sensitivity if applicable.
- Gamepad sensitivity if applicable.
- Target assist if gamepad combat support is added.

Cognitive/gameplay:

- Tutorial replay.
- Objective reminders.
- Reduce HUD clutter.
- Adjustable difficulty or pacing options if action intensity requires it.

## Testing and Audit Requirements

Every new UI screen should receive an accessibility checklist pass.

Checklist:

- [ ] Text meets minimum size and contrast requirements.
- [ ] Color is not the sole information carrier.
- [ ] Interactive elements are keyboard navigable.
- [ ] Focus order is logical.
- [ ] Focus state is visible.
- [ ] Hover-only information is also available by focus/selection.
- [ ] Subtitles/captions exist for critical audio if audio/dialogue exists.
- [ ] Inputs are represented as remappable actions.
- [ ] No required simultaneous button presses without alternatives.
- [ ] Motion-sensitive content can be reduced or disabled where applicable.
- [ ] UI remains usable at planned high text/UI scale.
- [ ] Tooltips and dialogs remain within screen bounds.
- [ ] Gamepad-supported screens are verified with gamepad navigation.

## Severity Definitions

### Blocking

- Prevents progression for keyboard-only players in required UI.
- Critical UI text below readability or contrast requirements.
- Color-only critical information.
- Missing captions/subtitles for story-critical dialogue when dialogue exists.
- Required simultaneous input without alternative.
- Unsafe flashing/seizure-risk content without mitigation.
- Item loss or destructive action caused by inaccessible or ambiguous UI.

### High

- Important but non-blocking UI lacks keyboard/gamepad support.
- Focus order confusing but usable.
- Tooltip/secondary information inaccessible by keyboard.
- Gamepad support absent from a screen that claims support.
- Missing visual equivalent for important gameplay audio.

### Medium

- Minor readability issue in noncritical UI.
- Inconsistent focus styling.
- Optional content has incomplete accessibility support.
- Advanced tooltip or secondary UI is usable but difficult to parse.

### Low

- Polish issue, inconsistency, or future improvement that does not block access.

## Phase 1 Accessibility Acceptance Criteria

### Input

- [ ] Core UI screens are usable with keyboard only.
- [ ] Supported UI screens are usable with gamepad only.
- [ ] Mouse interactions have hover/focus feedback.
- [ ] Drag-and-drop actions have non-drag alternatives.
- [ ] `Esc` or cancel reliably exits topmost UI states.

### Visual

- [ ] Text is readable at minimum supported resolution and UI scale.
- [ ] Tooltips remain within screen bounds.
- [ ] UI does not rely on color alone for rarity, validity, or stat comparison.
- [ ] Focus indicators are clearly visible.

### Audio

- [ ] Important feedback is understandable with audio muted.
- [ ] Dialogue has subtitles if dialogue exists.
- [ ] Pickup/equip/error states have visual feedback.

### Cognitive

- [ ] Default item tooltips are concise.
- [ ] Advanced details are progressively disclosed.
- [ ] Error messages are consistent and specific.
- [ ] Destructive actions require deliberate confirmation or are not default actions.

### Safety

- [ ] Items are never lost through failed equip/unequip/swap flows.
- [ ] Inventory-full edge cases are handled before changing equipment state.
- [ ] Invalid actions fail safely and explain why.
- [ ] Repeated error feedback is throttled.
