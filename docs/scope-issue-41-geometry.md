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
both Unicode and ASCII. The Mermaid crate suite passes after correcting eight
affected note/activation snapshots; no unrelated snapshot is reclassified.
The initial adjacency-only note check was discarded and the directional test
failed before implementation. Stable workspace CI gates will be run again on
the final integrated branch, including the shared Rustls advisory remediation.
