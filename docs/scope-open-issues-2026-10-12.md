# Parallel open-issue fixes — 2026-10-12

The three independent fixes (#52, #41, #48) were implemented in isolated
worktrees with smaller models, then integrated on top of contributor PR #51.
The original @worldnine commit is retained. Versions are 1.35.3 for the
application and 0.57.4 for mermaid-text; no release/publish is requested.

Each issue declared a 600 implementation/documentation line ceiling, zero
unrelated snapshot reclassification and zero regressions. Separate scope notes
record failing reproductions, discarded weak assertions and final evidence.
Only nine related note/activation/autonumber snapshots changed. Shared `sequence.rs` merged
cleanly; changelog insertion conflicts were resolved into one coherent entry.

GPT-6.1 review-pr and fix-review-findings cover runtime/correctness,
security/config/performance and tests/reliability independently. Review reports
and findings remain committed, with GitHub-visible findings and resolutions.
No new follow-up issue was created.

#48 has a verified text-tag fix and protocol-selection mitigation, but the
reported terminal GUI is unavailable here. This is an external validation
limit; the PR must not claim the visual symptom is directly reproduced/fixed.
All local raster/geometry and protocol regressions still must pass.

Initial Rust1.92 testing exposed an existing custom-target binary lookup bug:
runtime Cargo binary env was absent and the fallback path did not exist. The
three helpers now use Cargo's compile-time binary path; full tests pass without
an override. 6.1 review also reproduced canvas/continuation, note, autonumber,
activation-index/performance and raster-assertion gaps. All nine findings are
resolved in this PR and independent runtime/tests cross-review is clean.

Final integrated gates all exit 0: fmt, workspace/all-target clippy with warnings
denied, full stable and Rust1.92 workspace tests, deny, audit and default docs.
Existing strict rustdoc link warnings remain; no Linux/WezTerm GUI check is
claimed. The stacked base is outside CI's master-only PR filter; these are local
results, not GitHub CI claims. Per-issue implementation and scope notes remain
within their 600-line ceilings after sharing glyph primitives; independent
review/integration records document the cross-cutting audit separately.
