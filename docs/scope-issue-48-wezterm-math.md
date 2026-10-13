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

## PR #53 review repairs

Review found that the raster tag assertion could pass when a renderer dropped
visible tag content and merely added transparent columns. The repaired test
requires tagged output to contain more visible alpha pixels than the untagged
control, retaining the bold/plain comparison and measured terminal geometry.
The genuine tagged image passes this assertion. Transparent padding
adds zero visible pixels, so a padded copy has exactly the control's visible
pixel count and fails the strict greater-than assertion. This directly rejects
the counterexample without adding a test that merely mirrors the helper.

The integrated Rust 1.92 run also failed to launch the application in
`tests/check_links.rs`: its runtime Cargo binary-path variable was absent and
its repository-local fallback did not match the custom target directory. The
same helper exists in `export_html.rs` and `section_extract.rs`. All three now
use Cargo's compile-time binary path. Focused suites passed on stable and
Rust 1.92 with the shared custom target directory and no runtime binary-path
override. The observed failing log is
`/tmp/markdown-reader-combined-checks/3.log`.

Protocol comments describe iTerm2 as the documented working primary
WezTerm choice; upstream GUI QA and the reported WSL presentation remain
unverified. These repairs stay within the 600-line scope ceiling and add no
dependency or CI job.

### Focused repair validation

- All three integration suites passed on stable and Rust 1.92: 2 check-links,
  1 HTML export and 6 section-extraction tests on each compiler. Both commands
  used `/Users/leboiko/dev/markdown-reader-issue-50/target` and explicitly
  removed `CARGO_BIN_EXE_markdown-reader` from the runtime environment.
- The reported-equation raster regression passed with the stronger visible
  ink assertion, bold/plain image control and terminal-cell geometry check.
- The WezTerm protocol-selection and non-WezTerm preservation tests passed.
- Logs: `/tmp/markdown-reader-pr53-math-tests/{0,1,2,3}.log`, all exit 0.
