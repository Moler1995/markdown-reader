# GPT-6.1 review — PR #53

PR: https://github.com/leboiko/markdown-reader/pull/53
Base: `review/issue-50-class-utf8`, identical to contributor PR #51 at `6be5629`.
Initial reviewed head: `61b2973`. Branch: `fix/open-issues-oct2026`.

## Summary

Three independent 6.1 passes reviewed runtime/correctness, security/configuration/
performance, and tests/reliability. The smaller-model issue implementations were
reviewed adversarially after integration. Nine findings were posted in
[the GitHub review](https://github.com/leboiko/markdown-reader/pull/53#pullrequestreview-5494835651).
They are resolved in this PR. Final independent runtime and tests cross-reviews
found no unresolved actionable findings. All integrated gates pass.
The [additional index finding](https://github.com/leboiko/markdown-reader/pull/53#pullrequestreview-5494930045) was posted during re-review.
No follow-up issues were created. #48 remains open because its reported GUI
requires the external WSL/WezTerm environment; this PR ships the verified tag fix
and narrowly supported protocol mitigation, without claiming that visual proof.

## Findings and remediation

| ID | Severity | Finding / failing evidence | Resolution |
|---|---|---|---|
| 1 | Critical | Final-column `é`/`界́` loses accent; quadrant right axis becomes plain `e`. | Attach zero-width suffix before visible-column bounds; share clipped writers/serialization across boxes, sequence and quadrant. |
| 2 | Critical | Blocked write hides adjacent A; recompute/style and overlapping replacement leave stale wide continuations. | Centralize Grid mutations through ownership-aware `set`; update metadata only after accepted writes. Pin exact neighboring columns and private route writers. |
| 3 | Critical | Public box `éX` body measures 7 columns instead of 8; width-one row emits width-two glyph. | Preserve public signatures, recover inline marks to display cells for writes, clip full glyphs, and retain a fast fixed-cell path for class/ER. Batch legacy rules to avoid per-column row conversion. |
| 4 | Critical | Activation overlay destroys note borders/text, including nested right-of and over notes. | Reserve right-of lanes; keep over/over-pair notes opaque at their original anchors and event positions. Activation lanes are continuous on exposed rows and intentionally hidden behind annotations/frame borders. |
| 5 | Major | Per-endpoint full activation scans add O(messages × activations); 16k messages/1k sparse spans ~45ms baseline→607ms. | Build sparse per-participant boundary changes and query by binary search. Same debug benchmark improved 607ms→116ms; exact label counts and output bytes are unchanged. Existing quadratic depth calculation is unchanged. |
| 6 | Critical | Autonumber prefix omitted from sizing; active intermediate participant destroys numbered label suffix. | Compute displayed labels once and use the same strings for gap/self-loop sizing and painting, including numbering resets. |
| 7 | Major | Tag test accepts transparent padding with no equation-number ink. | Require strictly more visible alpha pixels in tagged raster than untagged, retaining bold and geometry controls. Transparent padding has unchanged visible count and cannot pass. |
| 8 | Major | Rust1.92 custom-target full suite fails OS NotFound in three CLI integration helpers. | Use Cargo compile-time binary paths; focused suites pass on stable and 1.92 without runtime overrides. |

| 9 | Major | Skipping an unknown-participant message shifts arrow-row indices and silently deletes eight activation cells. | Keep arrow rows indexed by original message position and preserve note/event boundaries around skipped entries. |

Reproductions ran failing before each correctness diagnosis/fix. Original #41
adjacency-only connector test was rejected and replaced by directional-port
connectivity; original #52 char-count assertion was corrected to display width.
The already-passing formula raster control was never called a failing repro.
Private route-writer assertions additionally cover all direct Grid mutation paths.

## Architecture and test assessment

Canvas ownership remains local. Shared text primitives eliminate three separate
clipping/suffix implementations, while legacy public char-grid APIs retain their
signatures. No new dependency or protocol execution path is introduced; parent
PR51 handles the mandatory Rustls advisory update. No persistence/migration or
public configuration format changes are needed. Reverting these commits restores
previous rendering without data conversion.

Exact fixtures pin terminal columns, full note rectangles, activation spans and
numbered label origins in Unicode/ASCII modes. Nine related note/activation/
autonumber snapshots changed; no unrelated reclassification is
accepted. Ordinary CJK fixtures require no snapshot updates. Raster/protocol tests
validate local behavior but cannot validate a remote terminal GUI.

## Red-Team Checklist

- Data safety: checked. No database/schema/versioned persistence applies. Adversarial canvas-boundary, continuation ownership, combining marks and subsequent writes were tested against exact output.
- Concurrency: checked. Render state remains local, raster/cache task completion paths unchanged, no new coalescing/retry/shared-state path. Protocol setup stays synchronous.
- Runtime degradation: checked. Missing graphics, query error and tmux fallbacks inspected; clipping/out-of-range/unknown participant handling reviewed. Existing raster bounds/error fallback retained. GUI unavailable explicitly recorded.
- Security/config: checked. WezTerm env values are boolean markers, never executed. No secrets, new network fetches, unsafe blocks or raw terminal escapes added; locked protocol encoder retained. Manifest/lock diff reviewed.
- Performance: checked. Repeated activation scans were measured and replaced with sparse indexed events; main class/ER fixed-grid writes remain constant-time. Legacy rules batch row conversion and cap allocation to canvas size.
- Tests: checked. Complete strings, exact port topology and column/bar coordinates reject no-op implementations; visible raster ink rejects tag deletion via transparent padding. Focused custom-target integration tests cover the discovered harness failure. Focused stable/MSRV checks and independent cross-review are complete; full gates below.

## Validation

Final integrated `cargo fmt --all -- --check`, workspace/all-target clippy with
`-D warnings`, complete workspace tests on stable and Rust1.92, `cargo deny check`,
`cargo audit`, and `cargo doc --no-deps --workspace`: all exit 0. No runtime
`CARGO_BIN_EXE_markdown-reader` override was used. Focused suites include 29
boundary/CJK/geometry fixtures, 13 sequence unit tests, 20 sequence snapshots and
all three CLI integration suites on both toolchains. Logs were recorded in
`/tmp/markdown-reader-pr53-final-checks/0.log` through `6.log`. Existing strict rustdoc unresolved-link warnings
are inherited; the actual default docs gate is required. No release/publish was
triggered. This stacked base does not match CI's `master` pull-request filter, so
local checks must not be presented as GitHub CI results.

## Evidence inspected

Actual PR metadata/files/head/base and complete GitHub diff were fetched by all
three reviewers. Scope documents, complete changed modules, failing probes,
original contributor PR51 and the locked ratatui-image picker were inspected.
Primary references: [locked picker source](https://docs.rs/crate/ratatui-image/11.0.6/source/src/picker.rs)
and [upstream protocol matrix](https://github.com/ratatui/ratatui-image#compatibility-matrix).
The three independent reports and remediation plans are incorporated here rather
than leaving the findings only in temporary files. GitHub comment replies and a
fix status table identify the final verified commit.
