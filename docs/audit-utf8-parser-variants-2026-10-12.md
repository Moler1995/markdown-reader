# UTF-8 parser variants reviewed with PR #51

## Root cause and scope

Untrusted diagram source reaches string slices whose offsets are inferred from
ASCII widths or independent delimiter searches. Character-boundary and ordering
invariants must hold before slicing. The contributor's checked keyword helper
fix is retained; these findings extend it without replacing their authorship.

## Search methodology

| Pattern | Matches | Confirmed | Safe |
| --- | ---: | ---: | ---: |
| `before_paren[last_space + 1..]` | 1 | 1 | 0 |
| `find/rfind(char::is_whitespace)` across repository | 4 | 3 | 1 |
| `.find(')')` in Mermaid parsers | 4 | 1 | 3 |

The whitespace search expands one operation at a time. Class and both ER token
splitters skip one byte of a multi-byte separator; pie slices at the separator
itself and is safe. Architecture requires a leading `(`; sequence requires
`rgb(`/`rgba(` before searching its close, so their ordering is established.
Broad searches for `pos + 1` slices mostly find ASCII punctuation with guarded
prefixes, exceeding the noise threshold; those were read and the expansion was
stopped. No further confirmed variants were found in that pattern family.

## Confirmed findings and fixes

- Critical, high confidence: class typed-before methods with U+3000 whitespace
  panic at byte 7 within separator bytes 6..9. Slice at the matched whitespace
  boundary and trim it, preserving the method name and return type.
- Critical, high confidence: malformed `+吠える)(` constructs range 11..9.
  Reject a close before the open and search for a close only after the open.
- Critical, high confidence: ER left/right relationship separators use the
  same one-byte skip. Slice at the separator boundary and trim it; preserve
  both entities, cardinalities, and relationship label.

All three regression tests failed before fixes (exit 101). Tests pin successful
AST structure and exact malformed-method diagnostic, so parser deletion or a
catch-all error cannot satisfy them. UTF-8 2/3/4-byte keyword boundary and ASCII
mixed-case/delimiter probes also passed in independent GPT-6.1 reviews.

## Regression guard

The existing workspace test gate includes `utf8_parser_variants.rs`. The
focused guard is `cargo test -p mermaid-text --test utf8_parser_variants`;
no additional CI job or dependency is introduced. Full local fmt, clippy,
stable/MSRV tests, deny, audit, and docs are required before pushing.
