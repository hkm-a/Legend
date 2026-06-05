# ADR-0020: Resource / Map Conversion Pipeline and Validation Boundary

## Status

Accepted

## Date

2026-06-05

## Engine Compatibility

| Field | Value |
|-------|-------|
| **Engine** | Godot 4.6.3 |
| **Domain** | Core / Resources / Rendering / Tooling |
| **Knowledge Risk** | HIGH — Godot 4.6.x is post-cutoff and Resource, TileMapLayer, FileAccess, editor tooling, and import behaviour must be verified against pinned docs/tests. |
| **References Consulted** | `docs/engine-reference/godot/VERSION.md`; `docs/engine-reference/godot/current-best-practices.md`; `docs/engine-reference/godot/breaking-changes.md`; `docs/engine-reference/godot/deprecated-apis.md`; `docs/architecture/adr-0001-map-data-representation.md`; `docs/architecture/adr-0013-attribute-godot-resource-duplication-shared-reference-policy-if-tres-resources-are-used.md`; `docs/architecture/adr-0016-openmir2-evidence-mapping-registry-and-provisional-contract-governance.md` |
| **Post-Cutoff APIs Used** | `TileMapLayer` is the non-deprecated 2D tile presentation layer; `Resource.duplicate_deep()` exists in Godot 4.5+ and may be used only as verified importer/tooling/test aid; Godot 4.4+ `FileAccess.store_*` methods return `bool` and must be checked. |
| **Verification Required** | Verify `MapDefinition` serialization/output; `TileMapLayer` visual output cannot override gameplay facts; `ResourceLoader` cache/object identity cannot affect conversion correctness; `ResourceSaver.save()` / `PackedScene.pack()` / `FileAccess.open()` / `FileAccess.store_*` failures produce structured report failures; editor/tool wrappers do not make SceneTree or `@tool` lifecycle conversion authority. |

> **Note**: If Knowledge Risk is MEDIUM or HIGH, this ADR must be re-validated if the project upgrades or downgrades engine versions. If Godot 4.6.3 tooling or import behaviour blocks this pipeline, engine downgrade is an allowed technical option to evaluate, but the static `MapDefinition` gameplay authority boundary remains unchanged unless superseded.

## ADR Dependencies

| Field | Value |
|-------|-------|
| **Depends On** | ADR-0001 Map Data Representation; ADR-0002 Typed Query Result Schema; ADR-0016 OpenMir2 Evidence Mapping Registry and Provisional Contract Governance. |
| **Enables** | Synthetic Phase 1 map fixture conversion; future OpenMir2/source map conversion spike; map/visual consistency validation; CI-visible map conversion evidence; implementation stories that need a small playable `MapDefinition`. |
| **Blocks** | Any story that imports, bakes, generates, or validates gameplay map data from source files, visual TileMapLayer scenes, OpenMir2/source fixtures, or generated Resource assets without a structured conversion boundary. |
| **Ordering Note** | ADR-0003, ADR-0004, ADR-0005, and ADR-0019 are related constraints, not hard dependencies for the base conversion path. ADR-0020 may validate data compatibility for MapSpaceState, Y-sort anchors, input projection, or movement policy, but it does not own those runtime behaviours. ADR-0013 is precedent only; this ADR restates map-specific Resource/source-envelope boundaries. |

## Context

### Problem Statement

The architecture review identified a remaining Foundation gap: the project needs a dedicated resource/map conversion pipeline spike before map implementation can safely proceed beyond hand-authored assumptions. ADR-0001 already defines `MapDefinition` as the authoritative static gameplay map data Resource, while `TileMapLayer` remains visual/editor presentation only. However, without a conversion and validation boundary, future work could treat raw OpenMir2/source fixtures, visual TileMapLayer custom data, scene nodes, Resource paths, ResourceLoader cache identity, or partially written generated files as gameplay truth.

The Phase 1 technical slice may use a synthetic map, but that synthetic map must still be explicitly labeled `mvp_provisional`, validated, reproducible, and generated or authored through a path that will not conflict with later OpenMir2 evidence-backed conversion work.

### Constraints

- ADR-0001 owns what `MapDefinition` means as static runtime gameplay fact authority.
- ADR-0002 owns typed status/reason discipline for structured map-space reporting; conversion reports should follow the same fail-closed discipline without pretending to be normal gameplay `SpatialQueryResult` objects.
- ADR-0016 owns source-authentic evidence labels and readiness; OpenMir2-derived map/cell/coordinate/blocking/occlusion claims require accepted evidence/contract references.
- `TileMapLayer`, TileSet custom data, collision shapes, navigation shapes, scene tiles, transforms, offsets, z-index, and draw order are visual/editor/validation inputs only.
- Godot-native file/asset writers must not ignore write, save, pack, replace, or open failures.
- Conversion/import may full-scan maps offline, in editor tooling, CI, or bootstrap validation; normal gameplay hot paths must not run source conversion or full-map scans.
- Asset licensing/legal clearance, full art production pipeline, map layout design, streaming, save/load, movement, combat, pickup, spawn, and runtime Y-sort ordering are outside this ADR.

### Requirements

- Define the boundary from source envelopes to validated candidate `MapDefinition` outputs.
- Support Phase 1 synthetic `mvp_provisional` maps without false OpenMir2 authenticity.
- Allow future OpenMir2/source fixture conversion without making MinimalMirClient, visual similarity, or decoded visuals authoritative.
- Produce deterministic `MapConversionReport` / `MapValidationReport` artifacts with structured failure reasons.
- Keep conversion core scene-tree-independent and testable in headless/unit contexts.
- Keep optional `TileMapLayer` / scene output non-authoritative.
- Prevent partial writes, unchecked `ResourceSaver`/`FileAccess` failures, and Resource graph/cache identity from becoming hidden truth.

## Decision

ADR-0020 establishes an offline/tooling/bootstrap map source conversion boundary. The pipeline accepts explicit source envelopes such as Phase 1 synthetic fixtures, future OpenMir2/source fixtures, or future visual/art map sources; parses and normalizes whitelisted source data into `MapConversionInput`; bakes candidate `MapDefinition` Resources; optionally emits non-authoritative `TileMapLayer` / scene visual output; and produces deterministic `MapConversionReport` / `MapValidationReport` evidence.

The pipeline owns conversion, validation, provenance reporting, and candidate bake output boundaries only. ADR-0001 remains the authority for `MapDefinition` runtime static gameplay facts. ADR-0020 does not redefine `MapDefinition` schema, runtime map facts, runtime occupancy, movement legality, input projection, Y-sort ordering, pathfinding, combat, pickup, spawn, save/load, asset licensing, or map layout design.

Import and bake are offline/tooling/bootstrap activities, not normal gameplay-frame behaviour. Runtime gameplay must consume already-baked `MapDefinition` assets and ADR-owned services, never raw source envelopes, visual output, source fixtures, conversion reports, editor nodes, or generated scene metadata as gameplay authority.

### Architecture Diagram

```text
Synthetic fixture / OpenMir2 source fixture / future visual source
        |
        v
Source Input Envelope
        |
        v
Parse / decode stage
        |
        v
MapConversionInput DTO
(normalized, tooling-only, not runtime truth)
        |
        +--> ADR-0016 source/evidence label validation
        |
        v
Bake candidate MapDefinition Resource
        |
        +--> optional TileMapLayer / scene visual output
        |
        v
MapConversionReport + MapValidationReport
        |
        v
Accepted baked MapDefinition loaded by runtime under ADR-0001
```

### Key Interfaces

Exact class/file locations are implementation details. The core parser/normalizer/validator/baker must be injectable and scene-tree-independent.

```gdscript
class_name MapConversionInput
extends RefCounted

var conversion_schema_version: int
var source_kind: StringName
var source_id: StringName
var source_hash: StringName
var map_id: StringName
var width: int
var height: int
var evidence_status: StringName
var evidence_contract_id: StringName
var evidence_contract_version: int
var is_mvp_provisional: bool
```

`MapConversionInput` is a normalized intermediate DTO used only by tooling/import/bootstrap validation. It is not save truth, runtime gameplay authority, or a substitute for `MapDefinition`.

```gdscript
class_name MapConversionReport
extends RefCounted

var status: int
var primary_reason: int
var secondary_reasons: Array[int]
var source_kind: StringName
var source_id: StringName
var source_hash: StringName
var generated_map_id: StringName
var output_artifacts: Array[StringName]
var validation_report: MapValidationReport
```

`MapConversionReport` and `MapValidationReport` follow ADR-0002's typed status/reason discipline and fail-closed semantics, but they are tooling/report DTOs, not normal gameplay `SpatialQueryResult` objects unless a later implementation story explicitly maps a validation issue into a runtime spatial query context.

### Phase 1 Minimum Implementation

Phase 1 minimum implementation is intentionally small:

1. A synthetic `mvp_provisional` map fixture envelope.
2. Deterministic normalization into `MapConversionInput`.
3. Bake to a small `MapDefinition` Resource or fixture-equivalent output matching ADR-0001.
4. `MapConversionReport` / `MapValidationReport` with structured fail-closed reasons.
5. Tests proving visual/editor data cannot override `MapDefinition` facts.

OpenMir2 decode, source-authentic map semantics, visual/art source import, scene generation, legal/licensing checks, full editor plugins, atlas/material import, and production map layout workflow remain Spike/Future unless separately approved.

### Output Contract

The required output is a validated `MapDefinition` data contract plus structured reports. Persisting that output as `.tres`, `.res`, generated GDScript fixture data, JSON/ConfigFile intermediate, or editor-only `.tscn` visual scenes is an implementation/writer choice unless a later accepted implementation ADR/story pins the asset format.

Optional `TileMapLayer` / scene output is a non-authoritative presentation artifact produced for editor visibility, visual smoke tests, QA comparison, or temporary Phase 1 presentation. It does not define static gameplay facts, coordinate truth, collision truth, movement legality, item placement, Y-sort order, or source-authentic map semantics.

If visual output grows into a full art pipeline, editor plugin, streaming scene generator, atlas/material pipeline, or production map-layout workflow, write a separate Visual Map / Scene Bake Pipeline ADR. ADR-0020 covers only optional visual output tightly coupled to map conversion validation.

### Resource and Asset Identity Policy

`MapDefinition` is authoritative as a validated static data asset and query contract, not as mutable Resource object identity. Gameplay correctness must not depend on whether `ResourceLoader` returns a cached shared instance or a fresh instance. Runtime systems may retain a loaded `MapDefinition` reference only under a read-only-by-contract access policy and should prefer query methods over direct exported array access.

Source Resources, loaded scenes, TileMapLayer nodes, imported visual assets, decoded source envelopes, Resource UIDs, import metadata, source paths, generated filenames, scene unique names, NodePaths, subresource identity, and ResourceLoader cache behaviour are not gameplay facts. Runtime map identity uses explicit validated `map_id` and version/source metadata, not file path or Resource UID.

`Resource.duplicate_deep()` exists in Godot 4.5+ and may be used only as an importer/tooling/test aid after Godot 4.6.3 verification. ADR-0020 does not rely on exact `duplicate_deep()` parameter semantics. Any implementation that calls it must verify behaviour with nested Resources, subresources, external Resource references, exported arrays/dictionaries, and post-duplicate mutation tests. Passing those tests still does not make a duplicated Resource graph runtime mutable authority.

### Godot Asset Writer Policy

Godot asset writers that emit `.tres`, `.res`, `.tscn`, or equivalent Godot-native assets must treat `ResourceSaver.save()` and `PackedScene.pack()` return values as authoritative failure signals. A non-`OK` `Error` becomes a structured conversion failure such as `resource_save_failed`, `scene_pack_failed`, or `scene_save_failed`.

Any direct `FileAccess` writer must check `FileAccess.open()` failure and `get_open_error()` where applicable. Godot 4.4+ `store_*` return values must be checked; a `false` write result becomes `file_write_failed` or a more specific structured reason. Silent partial writes are forbidden.

Direct `FileAccess` writers must use a temporary file plus validated replacement strategy for authoritative generated outputs. The pipeline must not overwrite the last known good asset/report until parse, normalize, validation, bake, write, close, and replace steps have succeeded. `ResourceSaver`-based outputs must check returned `Error`; if atomicity is required for a given output type, that writer must save to a temporary path and replace only after successful validation.

The pipeline is an offline/editor/import tool boundary. It may write project-controlled `res://` generated assets only when running in editor/offline tooling or CI with source-tree write access. Exported gameplay runtime must not write `res://` and must not depend on generating `MapDefinition` assets at runtime. Any runtime diagnostic output, if later allowed, must target `user://` and remain non-authoritative.

### Editor / Tooling Boundary

The conversion core must be scene-tree-independent. `EditorPlugin`, `EditorImportPlugin`, `@tool` scripts, and TileMapLayer preview scenes are adapters only. They must call the same deterministic parser, normalizer, validator, and baker used by headless tests.

No conversion result may depend on `_ready()` order, editor selection state, node ownership quirks, NodePath identity, scene-tree insertion order, live TileMapLayer node state, filesystem traversal order, Resource property order, `.tres` serialization order, or raw `Dictionary` iteration order.

TileSet custom data, TileMapLayer collision, navigation, scene tiles, tile rotation, transforms, offsets, z-index, and visual draw order are validation/presentation inputs only. They must not be read during gameplay as blocking, movement, drop, pickup, combat-distance, or Y-sort truth. A mismatch against `MapDefinition` produces `visual_logical_mismatch`; it does not silently patch either side.

### Evidence and Provisional Labels

OpenMir2-derived map, cell, coordinate, blocking, occlusion, or visual matching claims may be labeled `openmir2_verified` only if ADR-0016 has an Accepted readiness-passing evidence/contract reference for that specific claim. MinimalMirClient observation, visual similarity, or successful visual map decode can never authorize gameplay semantics by itself.

Phase 1 synthetic maps are `mvp_provisional`. They may support the 30-second loot-loop technical slice but must not be described as source-authentic OpenMir2 behaviour.

### Deterministic Failure Ordering

Validation failures must be deterministic and ordered by:

1. conversion stage;
2. source envelope identity;
3. schema field order;
4. `map_id`;
5. cell index order `y * width + x`;
6. stable source row/tile/object ID where applicable.

Validation must not rely on dictionary iteration order, scene-tree order, TileMapLayer child order, Resource property order, editor display order, or filesystem traversal order.

### Failure Reasons

The conversion/report reason registry must include or map to structured reasons for at least:

- `parse_failed`
- `unsupported_source_format`
- `unknown_or_unloaded`
- `invalid_bounds`
- `array_length_mismatch`
- `visual_logical_mismatch`
- `missing_required_region`
- `missing_y_sort_anchor`
- `file_open_failed`
- `file_write_failed`
- `resource_save_failed`
- `scene_pack_failed`
- `scene_save_failed`
- `temp_write_failed`
- `replace_failed`
- `output_path_not_allowed`
- `tileset_missing_or_unloaded`
- `unsupported_tilemaplayer_feature`
- `editor_only_dependency_in_core`
- `evidence_not_accepted`
- `provisional_only`

## Alternatives Considered

### Alternative 1: TileMapLayer-authoritative conversion

- **Description**: Treat a visual TileMapLayer or TileSet custom data as the source of gameplay map facts during import and runtime.
- **Pros**: Easy to inspect in editor; artists/designers can edit visuals directly.
- **Cons**: Visual art changes could silently alter gameplay; TileMapLayer collision/navigation/custom data would conflict with ADR-0001; harder to prove OpenMir2 source semantics.
- **Rejection Reason**: TileMapLayer may assist validation and presentation, but `MapDefinition` remains gameplay static fact authority.

### Alternative 2: Runtime source parsing

- **Description**: Load and parse synthetic/OpenMir2/source map files during normal gameplay startup or gameplay frames and use source envelopes directly.
- **Pros**: Avoids generated assets; potentially faster iteration.
- **Cons**: Makes source formats runtime dependencies; encourages full-map scans and hot file IO; bypasses validation reports; complicates export/runtime error handling.
- **Rejection Reason**: Conversion is offline/tooling/bootstrap. Runtime consumes baked validated outputs.

### Alternative 3: Fixed `.tres` / `.tscn` output now

- **Description**: Require the pipeline to always emit `.tres` MapDefinition assets and `.tscn` TileMapLayer scenes.
- **Pros**: Concrete and editor-visible; aligns with Godot asset workflows.
- **Cons**: Over-specifies Phase 1; visual output may not be needed for synthetic fixtures; scene generation requires additional TileSet, texture, material, ownership, import, and directory policies.
- **Rejection Reason**: ADR-0020 defines output contracts and writer rules, while concrete file format choices can be pinned by implementation stories or future tool ADRs.

### Alternative 4: Treat conversion reports as runtime fallback authority

- **Description**: Let runtime inspect conversion reports to decide whether to patch, override, or recover map facts.
- **Pros**: Could make runtime resilient to generated-data errors.
- **Cons**: Creates double authority; lets reports override `MapDefinition`; may hide invalid generated assets.
- **Rejection Reason**: Reports can reject, warn, or annotate candidates before acceptance, but cannot override loaded runtime gameplay facts.

## Consequences

### Positive

- Closes the architecture review gap for resource/map conversion without building a full art pipeline.
- Makes Phase 1 synthetic maps reproducible, labeled, and validation-backed.
- Preserves `MapDefinition` static gameplay authority and TileMapLayer visual-only boundary.
- Prevents Resource graph/cache identity and partial file writes from becoming hidden gameplay truth.
- Provides a future path for OpenMir2 map/source conversion that respects ADR-0016 evidence gates.

### Negative

- Adds a tooling/reporting contract before map implementation stories can rely on generated map assets.
- Requires explicit writer error handling and deterministic validation tests even for small fixtures.
- Does not solve full OpenMir2 decoding, art conversion, legal clearance, or production map workflow.

### Risks

- **Risk**: ADR-0020 is misread as owning runtime map facts.
  **Mitigation**: ADR states that ADR-0001 owns `MapDefinition` runtime static fact semantics; ADR-0020 owns candidate production/validation only.
- **Risk**: Optional visual output grows into a hidden art pipeline.
  **Mitigation**: Visual output is optional and non-authoritative; full visual pipeline requires a future ADR.
- **Risk**: Tool wrappers depend on editor/SceneTree state.
  **Mitigation**: Core conversion must be scene-tree-independent and callable from headless tests; editor/tool scripts are adapters only.
- **Risk**: Generated assets are partially written or stale.
  **Mitigation**: Check all save/write/pack errors; use temp/replace for direct FileAccess outputs; include source hash/version/generator metadata.
- **Risk**: Synthetic maps become false OpenMir2 truth.
  **Mitigation**: Require `mvp_provisional` labels and ADR-0016 accepted evidence for `openmir2_verified` claims.

## GDD Requirements Addressed

| GDD System | Requirement | How This ADR Addresses It |
|------------|-------------|--------------------------|
| `systems-index.md` | Resource / map conversion pipeline must be an independent Spike; Phase 1 may use temporary assets. | Defines a standalone conversion/validation boundary with a synthetic `mvp_provisional` Phase 1 minimum path. |
| `game-concept.md` | Legend maps, resources, coordinates, occlusion, and Y-sort need early spike/architecture verification. | Establishes source-to-MapDefinition conversion, optional visual output, and validation report evidence. |
| `map-coordinate-blocking-y-sort-system.md` | Phase 1 map scope may use one synthetic test map with valid bounds and matching cell data. | Requires synthetic fixture normalization, bake, and validation into `MapDefinition`. |
| `map-coordinate-blocking-y-sort-system.md` | OpenMir2 coordinate/cell/source authenticity remains evidence-gated. | Requires ADR-0016 accepted contract references for source-authentic map claims and labels synthetic maps provisional. |
| `map-coordinate-blocking-y-sort-system.md` | TileMapLayer/visual data must not override gameplay logical map facts. | Makes TileMapLayer optional visual/editor output and reports mismatches rather than overriding MapDefinition. |
| `map-coordinate-blocking-y-sort-system.md` | Normal gameplay must not full-scan maps for ordinary spatial queries. | Allows full scans only offline/import/editor/CI/bootstrap validation, not normal gameplay hot paths. |

## Performance Implications

- **CPU**: Conversion and validation may scan full maps offline, in editor tooling, CI, or explicit load/bootstrap validation. Normal gameplay must use baked `MapDefinition` O(1) lookups and must not run source conversion or full-map scans for ordinary queries.
- **Memory**: Conversion may allocate intermediate DTOs/reports/tooling buffers outside gameplay hot paths. Runtime memory impact is the baked `MapDefinition` already governed by ADR-0001.
- **Load Time**: Bootstrap validation may check output metadata and report status. Expensive parse/decode/bake should be offline/editor/CI unless an implementation story explicitly accepts load-time cost.
- **Network**: None for Phase 1 offline slice.

## Migration Plan

No production map conversion implementation exists yet.

1. Define minimal synthetic source envelope and `MapConversionInput` fixture for one small test map.
2. Bake or construct a candidate `MapDefinition` matching ADR-0001.
3. Emit a `MapConversionReport` / `MapValidationReport` with deterministic structured status/reasons.
4. Add tests for invalid bounds, array length mismatch, missing required regions, missing Y-sort anchors, visual/logical mismatch, unchecked visual override attempts, and provisional/evidence label failure.
5. Add Godot integration tests only if `.tres`, `.res`, `.tscn`, ResourceSaver, PackedScene, TileMapLayer, or FileAccess writers are implemented.
6. Leave OpenMir2 decode and production visual source conversion as future spike tasks unless separately approved.

## Validation Criteria

- [ ] A synthetic `mvp_provisional` source envelope normalizes deterministically into `MapConversionInput`.
- [ ] A valid source bakes to a `MapDefinition` whose dimensions and dense arrays satisfy ADR-0001 validation.
- [ ] Invalid bounds, array length mismatch, missing required region, missing Y-sort anchor, and visual/logical mismatch produce structured fail-closed report reasons.
- [ ] TileMapLayer/TileSet custom data/collision/navigation/visual order cannot override `MapDefinition` facts.
- [ ] OpenMir2/source-authentic labels fail unless ADR-0016 accepted evidence/contract references are present.
- [ ] Conversion core tests run without SceneTree, EditorPlugin, `@tool` lifecycle, live TileMapLayer nodes, or ResourceLoader cache identity assumptions.
- [ ] Any implemented FileAccess writer checks open failure and every `store_*` bool result.
- [ ] Any implemented ResourceSaver/PackedScene writer checks `Error` returns and reports failures.
- [ ] Runtime gameplay does not parse source envelopes, run conversion, inspect conversion reports for action legality, or full-scan maps for normal queries.

## Related Decisions

- `docs/architecture/adr-0001-map-data-representation.md`
- `docs/architecture/adr-0002-typed-query-result-schema.md`
- `docs/architecture/adr-0003-authoritative-occupancy-reservation-update-ordering.md`
- `docs/architecture/adr-0004-deterministic-y-sort-implementation.md`
- `docs/architecture/adr-0005-input-projection-coordinate-conversion.md`
- `docs/architecture/adr-0013-attribute-godot-resource-duplication-shared-reference-policy-if-tres-resources-are-used.md`
- `docs/architecture/adr-0016-openmir2-evidence-mapping-registry-and-provisional-contract-governance.md`
- `docs/architecture/adr-0019-map-distance-and-movement-legality-contract.md`
- `design/gdd/game-concept.md`
- `design/gdd/systems-index.md`
- `design/gdd/map-coordinate-blocking-y-sort-system.md`
