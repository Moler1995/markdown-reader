# Scope: Mermaid display width (#52)

Hard limits: 600 changed implementation/documentation lines excluding tests;
0 unrelated snapshots; 0 regressions. Preserve public signatures and ASCII.

The initial exact regression run failed: flowchart/sequence CJK labels split,
class right borders displaced (12 columns versus 10), combining marks separated,
and quadrant marker moved from column 69 to 73. XY categories already passed.
A weak char-count assertion was replaced by terminal-column measurement before
accepting the reproduction. No implementation hypothesis preceded these tests.

Fixes reserve/skip wide continuation cells and attach zero-width suffixes to
successful bases. Sequence, quadrant and box tables share clipped text writing
and serialization; fixed-cell class/ER drawing retains its constant-time path.
Public box helpers retain inline marks without shifting subsequent coordinates.

6.1 integration probes also failed on exact-boundary marks, rejected wide writes
hiding neighbors, recompute/style stale continuations, overlapping wide glyphs,
and public helper clipping. All are resolved here with complete-string/column
assertions, including private route writers. No follow-up issue was created.

Initial focused/workspace tests, format, clippy and default docs passed; inherited
Rustls advisory is resolved in parent PR51. Final integrated gates and independent
review evidence are recorded in PR-53-review.md. No CJK snapshots changed.
