# Contributing to Magpie

Magpie is experimental. APIs, crates, and docs change without notice. The one
thing that doesn't change is the log format. Small, focused contributions are
welcome; for security problems, follow [SECURITY.md](SECURITY.md) instead of
opening an issue.

## Before you start

- Keep each pull request to one concern, small enough to review in one
  sitting.
- For anything beyond a bug fix or a test, open an issue first.
- Architecture, format, policy, and authority changes start as a proposal: an
  issue, or a pull request that adds only a design note under `docs/`. Don't
  implement them inside an unrelated change. Accepted decisions are recorded
  in [`docs/adr/`](docs/adr/).

## Rules that don't bend

**The log format is frozen.** `magpie-core-v1` is defined by
[`docs/FORMAT.md`](docs/FORMAT.md) and
[`crates/magpie-log/src/canonical.rs`](crates/magpie-log/src/canonical.rs),
and pinned by the golden vectors in
[`crates/magpie-log/tests/golden.rs`](crates/magpie-log/tests/golden.rs). Don't
change field order, encodings, tag values, domain strings, or an existing
`Payload` variant. A new payload variant is possible: it gets a new tag, a
`FORMAT.md` §3 entry, golden coverage, and an explicit arm in every `Payload`
match (no `_ => {}`).

**A failing golden test means the format broke.** Fix the cause. Never edit
the expected hashes or regenerate the fixtures to make it pass.
`crates/magpie-log/examples/regen_golden.rs` is only for reviewed,
owner-approved format changes: cutting a new profile, or appending records
for a new payload tag while every existing record stays byte-for-byte
unchanged.

**Authority stays where it is.** `LogWriter` is the only way to append, and
nothing mutates or deletes history; corrections are new events. Readers and
projections can't write. Projections are pure folds, so rebuilding from zero
must match incremental building byte for byte
([`regenerable.rs`](crates/magpie-claims/tests/regenerable.rs)). Replay and
projections run only on a completely verified history; don't add a replay or
projection path that skips verification. `LogReader::unverified_events` is a
deliberate raw-inspection exception for diagnostics, and its events can't feed
a projection.
Standing is always computed under an explicitly selected policy. A refactor
must not quietly widen what any of these grant.

**New dependencies need discussion first.** Don't hand-roll cryptography,
entropy, or security-relevant parsing to avoid one; raise it in an issue.

## Tests

Chain and security behaviour needs hostile tests, not just a happy path:
tampering, forged rehashes, wrong keys, truncation, rollback. The suite only
grows. Don't delete, skip, or weaken a test to get green.

## Before you open a pull request

CI pins Rust 1.98.1, Python 3.12.14, and Go 1.27.0. At minimum, run:

```sh
git diff --check
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

If you touch verifiers, fixtures, packaging, or the Python and Go conformers,
also run the matching conformance commands in the README's
[Development](README.md#development) section. In the pull request, say what
you ran and what you didn't.

`AGENTS.md` and `CLAUDE.md` are instructions for automated coding agents
working in this repository. Human contributors don't need to follow their
delegation workflow, but the rules above come from them.

## License

Contributions are accepted under the [Apache License 2.0](LICENSE-APACHE), as
set out in its section 5.
