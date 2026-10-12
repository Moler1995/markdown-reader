# Parallel open-issue fixes — 2026-10-12

The three independent fixes (#52, #41, #48) were implemented in isolated
worktrees with smaller models, then integrated on top of contributor PR #51.
The original @worldnine commit is retained. Versions are 1.35.3 for the
application and 0.57.4 for mermaid-text; no release/publish is requested.

Each issue declared a 600 implementation/documentation line ceiling, zero
unrelated snapshot reclassification and zero regressions. Separate scope notes
record failing reproductions, discarded weak assertions and final evidence.
Only eight #41 note/activation snapshots changed. Shared `sequence.rs` merged
cleanly; changelog insertion conflicts were resolved into one coherent entry.

GPT-6.1 review-pr and fix-review-findings cover runtime/correctness,
security/config/performance and tests/reliability independently. Review reports
and findings remain committed, with GitHub-visible findings and resolutions.
No new follow-up issue was created.

#48 has a verified text-tag fix and protocol-selection mitigation, but the
reported terminal GUI is unavailable here. This is an external validation
limit; the PR must not claim the visual symptom is directly reproduced/fixed.
All local raster/geometry and protocol regressions still must pass.

Initial integrated gates: format, clippy, stable workspace tests, deny, audit
and default docs passed. Rust 1.92 exposed an existing integration-test binary
lookup bug when using a custom target directory: runtime Cargo binary env was
absent and the fallback path did not exist. The suite passed with an explicit
binary-path environment override; the review will fix the three helper copies
to use Cargo’s compile-time binary path, then rerun without that override.

GPT-6.1 boundary probes also reproduced combining-mark clipping, stale wide
continuations, public box-table geometry, activation/note collision and new
per-message activation scans. These findings must be resolved before marking
the PR ready. Final integrated gates: pending. Hardware/Linux GUI checks are
not claimed.
