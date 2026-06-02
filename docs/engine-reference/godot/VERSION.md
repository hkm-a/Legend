# Godot Engine — Version Reference

Last verified: 2026-06-02

| Field | Value |
|-------|-------|
| **Engine Version** | Godot 4.6.3 |
| **Release Date** | 2026-05-20 |
| **Project Pinned** | 2026-06-02 |
| **Last Docs Verified** | 2026-06-02 |
| **LLM Knowledge Cutoff** | May 2025 |
| **Risk Level** | HIGH — Godot 4.6.x is post-cutoff |

## Knowledge Gap Warning

The LLM's training data likely covers Godot up to ~4.3. Versions 4.4, 4.5,
and 4.6 introduced significant changes that the model does NOT know about.
Always cross-reference this directory before suggesting Godot API calls.

Godot 4.6.3 is a stable maintenance release on the 4.6 branch. Treat it as the
project's pinned version for all architecture, GDScript, UI, rendering, export,
and testing decisions.

## Post-Cutoff Version Timeline

| Version | Release | Risk Level | Key Theme |
|---------|---------|------------|-----------|
| 4.4 | ~Mid 2025 | MEDIUM | Jolt physics option, FileAccess return types, shader texture type changes |
| 4.5 | ~Late 2025 | HIGH | Accessibility (AccessKit), variadic args, @abstract, shader baker, SMAA |
| 4.6 | 2026-01-26 | HIGH | Jolt default, glow rework, D3D12 default on Windows, IK restored |
| 4.6.3 | 2026-05-20 | HIGH | Stable maintenance patch on 4.6 branch; 86 fixes from 41 contributors reported in official release summaries |

## Project-Specific Notes

- This project targets a Godot 4.6.3 PC-native GDScript client.
- Phase 1 is a 2D/2.5D offline loot-loop technical slice; prioritize 2D rendering,
  Control UI, GDScript, input, resource import, and export behavior.
- The project's gameplay behavior should align with OpenMir2 original source
  where relevant. Engine reference docs only cover Godot API/version risk, not
  OpenMir2 behavior semantics.
- Avoid relying on memory for Godot APIs introduced after 4.3. Verify against
  official docs or these reference files before implementing.

## Verified Sources

- Official Godot homepage: https://godotengine.org/ — showed Download Latest 4.6.3 in search result.
- Official Godot download archive: https://godotengine.org/download/archive/ — listed Godot 4.6.3-stable dated 2026-05-20 in search result.
- Official 4.6.3 release notes: https://godotengine.org/article/maintenance-release-godot-4-6-3/
- Godot GitHub releases: https://github.com/godotengine/godot/releases
- Official 4.5→4.6 migration guide: https://docs.godotengine.org/en/stable/tutorials/migrating/upgrading_to_godot_4.6.html
- Official release policy: https://docs.godotengine.org/en/stable/about/release_policy.html
- Changelog: https://github.com/godotengine/godot/blob/master/CHANGELOG.md
