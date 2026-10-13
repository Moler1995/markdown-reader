# Issue #41 scope: Mermaid relationship geometry

## Requirements

- Render `note left of <state>` as a continuous link to its annotated state in
  Unicode and ASCII output.
- Render nested sequence activations as continuous, distinct bars that do not
  overwrite message labels, for same-participant nesting and overlapping
  activations on different participants, in Unicode and ASCII output.
- Keep the change renderer-only and preserve parsing and routing behavior.
- Cover the issue examples and edge cases with exact geometry assertions.

## Scope ceiling

- At most 600 changed implementation/documentation lines, excluding new test
  fixtures.
- Reclassify zero unrelated snapshots.
- Tolerate zero regressions in existing tests and rendering behavior.

## Findings and validation

The initial adjacency-only note test was a false positive: it accepted a vertical
`┆` touching a horizontal `─` without a compatible port. Replaced it with a
port-aware traversal that starts at the note border and ends at the exact target
box boundary. Both tests fail on master: the left note ends `┆` → `─╰`, and
nested activations render `██two`, `██three`, with stray blocks on arrow rows.

Plan: derive a note's terminal corner from the actual route and target border;
reserve explicit activation lanes during layout, place labels/arrows outside
those lanes, and draw continuous bars. Cover equal-span nesting, self messages,
and distinct participant activations. Correct only affected snapshots.

## Implemented behavior and verification

State note endpoints now combine the real incoming route ports with the port
pointing toward the target border. Scoped to note connectors, preserving other
routing and arrowhead behavior. Activation depths are computed once, including
equal-range LIFO nesting; layout reserves lane width, endpoints/labels stay
outside bars, and each bar remains exactly two cells wide through its full span.
Self-call loops reserve room and extend activation spans through the return leg.
Unknown participant or message indices are safely skipped by public rendering.

The exact topology, label clearance and complete lane-coordinate tests pass in
both Unicode and ASCII. The Mermaid crate suite passes after correcting nine
affected note/activation/autonumber snapshots; no unrelated snapshot is reclassified.
The initial adjacency-only note check was discarded and the directional test
failed before implementation. Full stable/MSRV workspace checks and other local CI gates pass on the final
integrated branch, including the shared Rustls advisory remediation.

## PR53 review remediation plan

Scope remains at most 600 changed implementation/documentation lines, zero
unrelated snapshot reclassification and zero regressions. The sequence-only
review fixes first establish failing tests for nested right-of and over/pair
notes, numbered labels crossing active intermediate participants, and direct
Canvas boundary/overwrite ownership.

Active widths will be indexed by participant and message-boundary events,
avoiding an activation-vector scan for every message endpoint. Layout and self
loops will measure the actual displayed autonumber labels. Right-of notes
reserve active lane widths; over and over-pair notes retain their original
participant anchoring and source event order. Notes are opaque annotations:
their exact text, interior and border intentionally occlude activation bars
where they intersect, while exposed bars above and below remain continuous.
Frame borders may similarly occlude a lane at the border intersection. This
is annotation paint order, not a gap in an exposed activation span.

The sequence Canvas will retain combining suffixes at the last display column,
update ownership only after successful writes, and release both columns of an
older wide glyph when overlapping text replaces its base. Focused geometry and
Canvas tests plus a baseline/current sparse-activation benchmark validate the
fixes; root owns the full suite, commit, push and GitHub replies.

Review reproductions ran before fixes: the expanded geometry suite returned
exit 101 (three failures), pinning exact note borders/text and the complete
numbered label at its assigned origin. The private Canvas regression filter
also exposed final-column combining loss, blocked-origin ownership updates and
overlapping wide-base ownership. Existing geometry cases remain green.

Final sequence remediation checks: geometry regressions pass (7 tests),
sequence unit regressions pass (13 tests), and CJK coverage passes (6 tests).
The existing sequence snapshots initially exposed the measured autonumber
layout width correction; only that related snapshot was accepted, increasing
the issue-specific count to nine. No unrelated snapshot changed. Sparse
activation rendering improved from 318ms to 67ms (8,000 messages/1,000 spans)
and 607ms to 116ms (16,000 messages/1,000 spans) in the same local debug
benchmark, with identical output byte counts and exact message-label counts.
The shared box-table glyph core owns Canvas text/serialization rules, keeping
the implementation within the declared ceiling. Root runs full final gates.

Final adversarial public-API fixture inserted an unknown-participant message
before a valid activation and reproduced complete loss of its eight bar cells
(exit 101). The renderer now stores optional arrow rows at the original message
indices, skips absent arrows safely, and still processes notes following a
skipped message. The exact eight-cell lane and three-row note-border fixture
passes; geometry coverage is now eight tests. Twenty existing sequence
snapshot checks and thirteen sequence unit checks pass. The added autonumber
snapshot is the only review-stage reclassification and is issue-related.
