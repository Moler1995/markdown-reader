# Scope: Mermaid CJK and fullwidth display width (#52)

## Requirements

- Render wide Unicode characters as one glyph occupying two terminal columns, without emitting the continuation cell as a space or allowing later drawing passes to overwrite it.
- Apply consistent display-width placement and serialization to the shared grid, sequence canvas, box-table canvas, and any affected quadrant or XY-chart canvas.
- Preserve ASCII output and existing public APIs.
- Cover CJK/fullwidth text and combining marks in Unicode and ASCII render modes, with exact geometry assertions for affected diagrams.
- Keep existing snapshots stable unless a narrowly justified expected output correction is necessary.

## Scope ceiling and acceptance

- Maximum 600 changed implementation/documentation lines, excluding new test fixtures.
- Zero unrelated snapshots reclassified and zero unrelated regressions.
- No unrelated feature or dependency changes.

## Evidence

- Issue: https://github.com/leboiko/markdown-reader/issues/52, reported by @worldnine.

## Status

Implementation in progress.

### Reproduction evidence (before implementation)

Added `tests/cjk_display_width.rs` with exact contiguous-label checks and terminal-column measurements, then ran:

```text
CARGO_TARGET_DIR=/tmp/markdown-reader-issue-52-target cargo test -p mermaid-text --test cjk_display_width -- --nocapture
```

After replacing an invalid box-table assertion that compared display width with stored `char` count, the reproduction run failed with exit 101. It pinned split flowchart/sequence labels, combining-mark separation, class border displacement (display columns 12 vs. 10), and a quadrant marker shifted from column 69 to 73. Exact labels and geometry assertions prevent a no-op or deleted feature from passing; XY category order and canvas width passed.

### Final validation

- `cargo fmt --all -- --check` — exit 0.
- `cargo clippy --workspace --all-targets -- -D warnings` — exit 0.
- `cargo test -p mermaid-text --test cjk_display_width` — exit 0, 6 passed.
- `cargo test --workspace` — exit 0; all workspace tests and doctests passed, with existing ignored tests unchanged.
- `cargo doc --workspace --no-deps` — exit 0 with existing intra-doc warnings; adding `RUSTDOCFLAGS='-D warnings'` exits 101 on those warnings.
- `cargo deny check` — exit 1 on the existing `rustls 0.23.40` advisory `RUSTSEC-2026-0285`; root is carrying its dependency update in the integrated changes.

No snapshot files changed. The implementation and documentation diff remains below the 600-line scope ceiling, excluding tests.
