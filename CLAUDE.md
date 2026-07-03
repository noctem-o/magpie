@AGENTS.md

# Claude Code instructions for Magpie

Claude is the architect, planner, reviewer, and taste layer.

Use Claude for:

- architecture
- research judgement
- epistemic model design
- event-log invariants
- provenance/security boundaries
- final review of generated diffs
- deciding whether a task is safe to delegate

Use Codex through `.\scripts\codex-delegate.ps1` for bounded implementation work.
Do not modify `scripts\codex-delegate.ps1` during ordinary delegation runs. The wrapper
is part of the delegation boundary. Only edit it when the user explicitly asks to debug
or change the harness.

## Windows Codex sandbox caveat

On native Windows, Codex shell execution in `workspace-write` may intermittently fail
with `CreateProcessAsUserW failed: 5`.

For Windows write-mode tickets, prefer file-edit tasks that Codex can complete by
patching files. Do not require Codex to run `cargo`, `git`, `tar`, `pwsh`, or other
commands as hard preconditions.

Ask Codex to report validation commands for the human to run. The human or parent
reviewer runs validation outside Codex.

If shell execution fails, Codex may continue only when the requested file edits are
still safe and bounded.

## Delegation

Good Codex tasks:

- scaffolding modules
- implementing a clearly specified CLI command
- adding tests
- small refactors
- SQLite migrations
- serde/data-model plumbing
- codebase search and summarisation

Do not delegate:

- Magpie's core memory semantics
- claim lifecycle design
- provenance/trust policy
- Deadbolt integration authority
- broad architecture decisions
- anything requiring product judgement

Before delegation, write a ticket under `.agent-runs\pending\` with:

1. Goal
2. Scope
3. Non-goals
4. Constraints
5. Tests
6. Acceptance criteria

After Codex returns:

1. Read the summary.
2. Inspect the diff.
3. Run tests yourself.
4. Recommend accept, revise, or discard.
5. Do not merge without human approval.

The desired workflow is: Claude decides -> Codex implements -> Claude reviews -> human merges.

On Windows, invoke the Codex wrapper as:

`powershell.exe -ExecutionPolicy Bypass -File .\scripts\codex-delegate.ps1 read <ticket>`

or:

`powershell.exe -ExecutionPolicy Bypass -File .\scripts\codex-delegate.ps1 write <ticket>`

Do not call `codex` directly unless explicitly asked. Prefer the wrapper.

# Magpie architecture rules

Local-first epistemic memory: one signed, hash-chained, append-only log (L0) is the
sole source of truth; everything else is a derived, regenerable projection.
Architecture: docs/ (bootstrap + ledger). Normative format spec: docs/FORMAT.md.

## The rule that outranks all others: the log format is frozen

`crates/magpie-log/src/canonical.rs` + `docs/FORMAT.md` define `magpie-core-v1`,
pinned by golden vectors (`crates/magpie-log/tests/golden.rs` + `testdata/golden-v1.jsonl`).

If a golden test fails: **STOP.** Do not fix the test, regenerate the fixture, or
update expected hashes. Red golden = format broke: either revert your bug, or it's a
deliberate format break (new profile name, human decision — out of scope unless George
explicitly asks). `examples/regen_golden.rs` is only for cutting a NEW profile; never
run it to make a red test green.

Freeze consequences:

- Never change field order, encodings, tag values, magic strings, or domain strings in `canonical.rs`.
- Never change an existing `Payload` variant's fields or canonical encoding.
- NEW payload variants are fine: append a fresh tag in `canonical.rs`, document in
  `docs/FORMAT.md` §3, add golden coverage. Every projection then needs an explicit
  match arm — `Payload` matches are deliberately exhaustive; never add `_ => {}`.

## Authority is type-shaped — keep it that way

- `LogWriter` is the only write path. Never add another append path, nor any mutate/delete. Corrections are new events.
- `LogReader` and projections stay structurally unable to write.
- Projections are pure folds: rebuilt-from-zero must equal incrementally-built, byte for byte (`tests/regenerable.rs` is the thesis test of the repo).
- The genesis event (seq 0) is library-written and validated at write and read time. Don't route around it.
- Stored bytes are never the hash preimage. Store format (JSON-lines) may evolve; canonical bytes may not.

## Layout

- `crates/magpie-log` — L0: event model, `magpie-core-v1` codec, hashing, signing, chain verification, stores (`FileStore`, `MemStore`)
- `crates/magpie-claims` — first projection: `ClaimsView`, the epistemic claim store
- `crates/magpie-episodic` — second projection: `EpisodicView`, SQLite + FTS5 full-text search over events (rusqlite bundled; the only crate that may depend on SQLite)
- `docs/FORMAT.md` — normative spec; a from-scratch reimplementation must reproduce the golden vectors from this page alone
- `tools/verify_chain.py` — independent Python verifier, written from FORMAT.md alone; CI runs it against the golden fixture with a pinned trust root

## Commands

- `cargo test` — whole workspace; must stay green (17 tests at format freeze; grow the suite, never shrink it).
- `cargo run --example tour -p magpie-claims` — 30-second end-to-end demo.

## Conventions

- Doc comments carry design rationale, not just API description. Honest caveats are house style: if something is unsound or provisional, say so plainly.
- Tests are adversarial where possible (tamper, forged rehash, wrong-key genesis, truncation). New chain-level behaviour needs a hostile test, not just a happy path.
- No new dependencies without explicit discussion. The hash path especially stays dependency-minimal.
- `BTreeMap` over `HashMap` anywhere that gets serialized — determinism is the point.
- No knowledge graph. The wiki, when it exists, is a downstream projection.
- Single-threaded by design for now (`MemStore` is `Rc`/`RefCell`); don't reach for `Arc`/`Mutex` without a design conversation.

## Queued work (keep current; update as things land)

1. Deadbolt-side emitter (deadbolt repo): the gate appends one `SegmentAnchored`
   per successful seal-and-verify, per ADR-0001 (`docs/adr/0001-deadbolt-seam.md`).
   Includes the deferred reconciliation story for sealed-but-unanchored bundles
   (visible set, retry policy) — ops design; needs its own design conversation
   before any ticket is cut.
2. Only after that: typed stores, reranker, evaluator. Earn each from use.

Landed: SQLite + FTS5 episodic projection (`magpie-episodic`, PR #5, 2026-07-02).
Landed: Deadbolt seam ratified as anchored hierarchy — ADR-0001, `SegmentAnchored`
tag 5, FORMAT.md §3 addition, extended golden vectors (six events, seqs 0-4
unchanged), tag-5 Python verifier (PR #8, 2026-07-03).
