# Issue #48: WezTerm display math

## Scope

Fix display equations in `math_mode = "image"` for WezTerm, preserving the
fraction, bold symbols, and `\\tag{1}` as a visible `(1)` equation number.
Keep the normal image sizing and terminal cell geometry correct, and preserve
the Unicode fallback when image rendering is unavailable.

## Hard limits

- At most 600 changed lines, excluding test fixtures.
- Reclassify 0 unrelated snapshots.
- Tolerate 0 regressions in the existing checks.

## Evidence and validation

Before changing the implementation, add and run a strong failing regression
that exercises the reported formula, checks distinct non-empty raster output,
and verifies the actual image dimensions/cell measurement. Check the text-mode
tag fallback separately. Record platform limitations and exact check results
below. Do not create follow-up issues for in-scope gaps.

## Findings

- The reported equation already rasterises through RaTeX in the local CPU
  renderer. The regression checks PNG dimensions, verifies the tag adds width,
  and calculates the bounded cell height from the decoded image.
- The fallback renderer emitted `(dN(t))/(dt) = A(t)N(t). tag1`; the failing
  test recorded this exact malformed tag output.
- `ratatui-image` 11.0.6 documents WezTerm's stable protocol as iTerm2. Its
  picker blacklists Kitty/Sixel when `WEZTERM_EXECUTABLE` is set, but a WSL
  process with only `TERM_PROGRAM=WezTerm` can accept the Kitty probe first.
  The reproducer modeled that successful Kitty probe and failed with `Kitty`
  selected instead of `Iterm2`.
- This selection order is confirmed against the locked 11.0.6
  [`picker.rs`](https://docs.rs/crate/ratatui-image/11.0.6/source/src/picker.rs):
  queried capability precedes the environment-derived iTerm2 hint. The
  implementation therefore keeps the installed dependency version and applies
  the narrow override after a successful query.
- The upstream [compatibility matrix](https://github.com/ratatui/ratatui-image#compatibility-matrix)
  lists WezTerm's iTerm2 protocol as working and says its Kitty and Sixel
  support is not bug-free. This supports the protocol choice, while the
  upstream QA column is still unchecked for WezTerm.
- The code change explicitly chooses iTerm2 for recognized WezTerm environment
  markers and converts the fallback `\\tag{...}` argument into a
  parenthesized label. This is a protocol-routing mitigation, not direct proof
  that the malformed display in the reported GUI setup is resolved.
- The final raster regression compares decoded RGBA pixels for the tagged and
  untagged equations and for bold versus plain symbols. Its expected terminal
  height is computed from the decoded image height and the picker cell size;
  separate protocol tests check exact WezTerm detection and preserve Kitty for
  unrelated terminals. The text fallback asserts the complete expected
  equation string.
- No WezTerm GUI is available in this environment, so terminal rendering
  itself cannot be screenshot-verified here. Keep this limitation visible in
  release notes until checked on the reported WSL/WezTerm setup.

## Validation

- Initial image regression passed against existing code: the equation produced
  a substantive PNG, the tag increased width, and the image reserved a
  bounded terminal cell height using an 8×16 test cell size.
- Initial tag regression failed with the exact output shown above.
- Initial WezTerm picker regression failed: a simulated Kitty probe remained
  selected instead of iTerm2.
- Focused regressions: the image, fallback tag, WezTerm override, and
  non-WezTerm protocol preservation tests all passed.
- `cargo test --workspace`: passed across the workspace and doctests; existing
  ignored tests remain ignored.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo fmt --all -- --check`: passed after applying rustfmt.
- `cargo doc --workspace --no-deps` with `RUSTDOCFLAGS='-D warnings'`: failed
  on existing ambiguous intra-doc links in `crates/mermaid-text/src/lib.rs`;
  no unrelated docs were changed.
- `cargo doc --workspace --no-deps`: passed with pre-existing unresolved-link
  warnings in the workspace.
- Baseline `cargo deny check` failed on the existing rustls 0.23.40 advisory
  `RUSTSEC-2026-0285` (the advisory requires rustls >=0.23.45). The root
  integration handles this shared lockfile remediation; this issue branch
  records the baseline finding without duplicating the dependency change.
