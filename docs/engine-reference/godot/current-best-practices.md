# Godot — Current Best Practices

Last verified: 2026-06-02 | Engine: Godot 4.6.3

Practices that are **new or changed** since the model's training data (~4.3).
This supplements (not replaces) the agent's built-in knowledge.

## Version Pinning (4.6.3)

- Use Godot 4.6.3 for editor, export templates, local verification, and CI once CI exists.
- Treat 4.6.3 as a 4.6-branch maintenance/stable patch: do not assume new APIs over 4.6 unless official docs say so.
- Before implementation, verify touched APIs against stable docs when they involve post-cutoff areas: rendering, UI focus, resource duplication, navigation, shader pipeline, export, and physics.

## GDScript (4.5+)

- **Variadic arguments**: Functions can accept arbitrary parameter counts
  ```gdscript
  func log_values(prefix: String, values: Variant...) -> void:
      for v in values:
          print(prefix, ": ", v)
  ```

- **Abstract classes and methods**: Use `@abstract` to enforce inheritance
  ```gdscript
  @abstract
  class_name BaseEnemy extends CharacterBody2D

  @abstract
  func get_attack_pattern() -> Array[String]:
      pass  # Subclasses MUST override
  ```

- **Script backtracing**: Detailed call stacks available even in Release builds.

## GDScript Project Rules for Legend

- Prefer statically typed GDScript for gameplay systems.
- Use `class_name` only for globally reusable gameplay/resource types; avoid polluting global class names with one-off UI components.
- Use signals for cross-system notifications such as `item_picked_up`, `equipment_changed`, `stats_changed`, and `loot_dropped`.
- Avoid string-based signal connections; use callable connections.
- Cache node references with `@onready` instead of repeated path lookup in `_process()`.

## 2D / 2.5D Client Practices

- Use `Node2D.y_sort_enabled` or well-defined CanvasItem ordering rules; do not use old `YSort` nodes.
- Prefer `TileMapLayer` over deprecated `TileMap` for any Godot-native map experiments.
- For OpenMir2-style maps, first define a conversion/mapping layer instead of forcing original map semantics directly into Godot scene nodes.
- Keep Phase 1 rendering simple: one small test map, player, monster, loot item, and visible ordering/occlusion validation before full asset pipeline work.

## Physics (4.6)

- **Jolt Physics is the default 3D engine** for new projects.
  - This project is primarily 2D/2.5D; 2D physics remains separate.
  - If any 3D spike is introduced, verify Jolt-specific behavior before using old GodotPhysics assumptions.
  - Switch: Project Settings → Physics → 3D → Physics Engine.

## Rendering (4.6)

- **D3D12 is the default backend on Windows** (was Vulkan) — for better driver compatibility.
- **Glow now processes before tonemapping** with screen blending mode — existing glow setups may look different.
- **SSR overhauled** — significant improvement in realism, stability, and performance.
- **AgX tonemapper** — new white point and contrast controls.

## Rendering (4.5)

- **Shader Baker**: Pre-compile shaders to eliminate startup hitching.
- **SMAA 1x**: New AA option — sharper than FXAA, cheaper than TAA.
- **Stencil buffer**: Available for advanced masking/portal effects.
- **Bent normal maps**: Directional occlusion in normal map textures.
- **Specular occlusion**: Ambient occlusion now affects reflections.

## Accessibility (4.5+)

- **Screen reader support**: Control nodes integrate with accessibility tools via AccessKit.
- **Live translation preview**: Test GUI layouts in different languages directly in-editor.
- **FoldableContainer**: New accordion-style UI node for collapsible sections.
- **Recursive Control disable**: Disable mouse/focus interactions for entire node hierarchies with a single property.

## Animation (4.5+)

- **BoneConstraint3D**: Bind bones to other bones with modifiers.
  - AimModifier3D, CopyTransformModifier3D, ConvertTransformModifier3D.

## Animation (4.6)

- **IK system fully restored**: Complete inverse kinematics reintroduced for 3D.
  - Available modifiers: CCDIK, FABRIK, Jacobian IK, Spline IK, TwoBoneIK.
  - Applied via `SkeletonModifier3D` nodes.

## Resources (4.5+)

- **`duplicate_deep()`**: Explicit deep duplication for nested resource trees.
  - Old `duplicate()` behavior retained for backward compatibility.
  - Use `duplicate_deep()` when you need per-instance copies of nested resources.

## Navigation (4.5+)

- **Dedicated 2D navigation server**: No longer proxied through 3D NavigationServer.
  - Reduces export binary size for 2D-only games.

## UI (4.6)

- **Dual-focus system**: Mouse/touch focus is now separate from keyboard/gamepad focus.
  - Visual feedback differs depending on input method.
  - Important for this PC client because primary input is mouse/keyboard but partial gamepad support is planned.
  - Consider separate visual states for hover, mouse focus, keyboard focus, and gamepad focus.

## Editor Workflow (4.6)

- Flexible dock drag-and-drop with blue outline preview (including bottom panel).
- Most panels support floating windows (except Debugger).
- New keyboard shortcuts: Alt+O (Output), Alt+S (Shader).
- Export variable auto-generation: drag resource from FileSystem into script editor.
- Live preview in Quick Open dialog when "Live Preview" enabled.
- New "Select Mode" (v key) prevents accidental transforms; old mode renamed "Transform Mode" (q key).

## Tooling

- **ripgrep has no `gdscript` type**: `*.gd` is registered under `gap` (GAP programming language).
  `rg --type gdscript` is a hard error — the search never executes.
  Always use `rg --glob "*.gd"` (shell) or `glob: "*.gd"` (Grep tool) to filter GDScript files.

## Platform (4.5+)

- **visionOS export**: First new platform since open-sourcing (windowed app mode).
- **SDL3 gamepad driver**: Better cross-platform gamepad support.
- **Android**: Edge-to-edge display, camera feed access, 16KB page support (Android 15+).
- **Linux**: Wayland subwindow support for multi-window capability.
