# Drop Table System Review Log

## Independent Design Review — 2026-06-05

Reviewed file: `design/gdd/drop-table-system.md`

Review agents:

- Economy Designer: **NEEDS REVISION**
- Systems Designer: **NEEDS REVISION**
- QA Lead: **APPROVE**

Composite verdict before fixes: **NEEDS REVISION**

Blocking findings:

1. Concrete MVP item IDs were used before being registered or clearly fixture-scoped.
2. MVP expected acquisition outputs were derivable but not explicitly documented.
3. Economy health / faucet-risk context was too implicit for a loot-loop slice.
4. Formula sections needed explicit output range statements and complete variable/intermediate declarations.
5. `enabled valid rows` wording could imply silent skipping of invalid rows.
6. Bidirectional dependency status needed current/future/blocker labeling.
7. RNG inclusivity, multi-group result handling, zero-weight row validation, and statistical test policy needed clarification.

Fixes applied:

- Registered MVP provisional item fixtures in `design/registry/entities.yaml`:
  - `mvp_small_material_shard`
  - `mvp_bronze_sword`
  - `mvp_iron_sword`
  - `mvp_showcase_blade`
- Added explicit expected acquisition table:
  - any drop: 40%, expected once per 2.5 kills
  - material: 25%, expected once per 4 kills
  - any equipment: 15%, expected once per 6.67 kills
  - upgrade-or-better: 3%, expected once per 33.33 kills
  - rare/showcase: 0.5%, expected once per 200 kills
- Added material faucet calculation: 0.5 material units per kill.
- Added economy health note that Phase 1 rates are validation-biased and not long-term economy balance.
- Clarified `rng_int(a, b)` as inclusive.
- Clarified one result DTO per group attempt and fail-whole-table behavior for invalid normal gameplay tables.
- Added formula output range paragraphs and missing variables/intermediates.
- Reworded total-weight formula to use “prevalidated-valid group.”
- Clarified zero-weight rows still need valid references in normal tables.
- Added deterministic failure ordering policy.
- Added statistical test policy favoring exact boundary fake-RNG tests over flaky probabilistic sampling.
- Replaced vague player-loop acceptance criteria with numeric expected acquisition targets.

Post-fix internal status: **APPROVED FOR ADR AUTHORING / IMPLEMENTATION PLANNING AFTER INDEPENDENT REVIEW REFRESH**.
