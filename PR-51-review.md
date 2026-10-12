# PR #51 review — GPT-6.1

Reviewed contributor commit `3b896f7a1d6e56377da0320cf96bddc0502c3899` for issue #50. The shared keyword helper uses checked UTF-8 slicing and preserves the prior case-insensitive ASCII keyword/whitespace behavior. Contributor authorship is retained.

## Findings

1. Minor — tests/reliability — `crates/mermaid-text/tests/utf8_multibyte.rs:226`: the colon-shorthand rejection test accepts every error. An unrelated rejection could satisfy it. Pin `Error::ParseError` and the exact unsupported-colon-shorthand diagnostic for both Unicode and ASCII sources.

## Architecture and testing assessment

The implementation removes a duplicate helper with no public API change or allocation overhead. Read the full class parser, canonical helper, UTF-8 integration tests, and parser unit tests. Existing shared helper tests cover keyword case and delimiter behavior. UTF-8 suite: 12 passed. Both contributor regressions were also run against the unfixed master parser to establish failure before restoration.

## Red-Team Checklist

- Data safety: checked — no persisted data; original string byte slices are now validated with `str::get`, all subsequent slices require an ASCII keyword match.
- Concurrency: checked — pure synchronous parser with no shared mutable state, retries or resources.
- Runtime degradation: checked — unsupported source still returns explicit parse errors; tested Unicode rejection and panic paths.
- Security/config: checked — untrusted diagram text; checked prefix matching cannot panic at non-boundary offsets. No process/config/credential changes.
- Performance: checked — no lowercasing/allocation, unchanged bounded prefix probes, fewer duplicated functions.
- Tests: checked — valid Japanese/Cyrillic relations preserve labels, malformed shorthand rejects safely; negative diagnostic assertion is the finding above.

## Recommendations and residual risk

Strengthen the rejection test in this PR and run the full local CI checks before pushing. Wide-character drawing alignment (#52) is independently being fixed in this same work session; it is not caused by the parser fix. No architecture work is deferred.

## Fix-review-findings — GPT-6.1

The negative regression now matches `Error::ParseError` and its exact diagnostic
for Japanese and ASCII shorthand. The positive test also pins both parsed
classes and the single inheritance relation, so removing parsing cannot pass.
Added root/crate CHANGELOG credit to @worldnine while retaining their commit.

The first full local gates found a pre-existing security blocker: Rustls 0.23.40
is affected by RUSTSEC-2026-0285. Updated only Rustls to 0.23.45 and webpki to
0.103.15, the latest compatible versions selected by Cargo. Primary advisory:
https://rustsec.org/advisories/RUSTSEC-2026-0285.html. Final stable gates were green after the lockfile update. Independent GPT-6.1
security/config and tests/reliability passes then discovered and reproduced
additional class-method panics; the variant sweep found both ER whitespace
splitters too. All are fixed in this PR, with failing-before/passing-after tests
and detailed findings in `docs/audit-utf8-parser-variants-2026-10-12.md`. No
finding is deferred. Final gates rerun after these code changes.

## Final independent review and validation

Separate GPT-6.1 runtime/correctness, security/config/performance and
tests/reliability passes are complete. Critical pre-existing variants found by
review were posted on GitHub and fixed in this PR; the original minor assertion
finding is fixed as well. No finding was deferred. Independent adversarial
probes covered 23 whitespace separators and 20,000 malformed class/ER inputs,
with no remaining panic. The final ER regression pins endpoints, cardinalities,
line style, label, and relationship count.

All final gates passed (exit 0): fmt, workspace clippy with denied warnings,
stable workspace tests, Rust 1.92 workspace tests, cargo-deny bans/licenses/
sources/advisories, default workspace docs, and cargo-audit. Default docs retain
existing intra-doc link warnings. No release/publish was triggered.
