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
- `crates/magpie-episodic` — second projection: `EpisodicView`, SQLite + FTS5 full-text search over events
- `crates/magpie-log` — also owns the contract-selected supported SQLite L0 persistence backend; L0 and projection databases remain separate
- `docs/FORMAT.md` — normative spec; a from-scratch reimplementation must reproduce the golden vectors from this page alone
- `tools/verify_chain.py` — readable independent/reference conformer for the selected portable language and profile; `tools/portable_verifier.py` remains the earlier independent complete conformer artifact

## Commands

- `cargo test` — whole workspace; must stay green (17 tests at format freeze; grow the suite, never shrink it).
- `cargo run --example tour -p magpie-claims` — 30-second end-to-end demo.

## Conventions

- Doc comments carry design rationale, not just API description. Honest caveats are house style: if something is unsound or provisional, say so plainly.
- Tests are adversarial where possible (tamper, forged rehash, wrong-key genesis, truncation). New chain-level behaviour needs a hostile test, not just a happy path.
- No new dependencies without explicit discussion. The hash path especially stays dependency-minimal. But "no new dependencies" never licenses hand-rolling entropy, cryptographic primitives, or security-relevant parsing — `getrandom` and audited equivalents are pre-approved for those purposes; when a ticket constraint forces that choice, stop and flag rather than pick a horn.
- `BTreeMap` over `HashMap` anywhere that gets serialized — determinism is the point.
- No knowledge graph. The wiki, when it exists, is a downstream projection.
- Single-threaded by design for now (`MemStore` is `Rc`/`RefCell`); don't reach for `Arc`/`Mutex` without a design conversation.

## Queued work (keep current; update as things land)

1. C3 decision — only if the advertised pre-alpha requires detachable governed
   output, add the smallest coordinate-complete verification/replay result or
   receipt; otherwise preserve the explicit exclusion.
2. C4 — harden the applicable public assurance boundary: pinned validation
   inputs, platform evidence, and release-environment reproducibility.
3. C5 — write the public pre-alpha release contract, freeze a candidate, and
   falsify it before an owner release decision.

Keep general ingestion/CAS, `EpistemicGate`, governed ordinary agent writing,
contradiction/currentness runtime, ADR-0007 authority runtime, librarian,
effectful MCP, production KMS, and ambient/Vesper agent work deferred unless a
separate owner decision moves one into the release boundary.

Landed on current `main`: PR #146's non-test public
`FileStore::verify_portable_history` acquires exact file bytes once after the
complete selected-profile/external-key gate and invokes the authoritative
portable semantics without legacy `LogStore::read_records` splitting. Blank
physical records and newline distinctions are preserved, missing files remain
operational I/O failures, existing zero-byte files are the governed empty
snapshot, and the complete result is computed internally from the same call;
no public typed receipt/result was added. The independent Python reference is
`tools/verify_chain.py` itself; it remains separate from the comparison-only
`tools/portable_verifier.py`. The standalone Go conformer still uses only its
authorized vendored `filippo.io/edwards25519 v1.2.0` dependency, and the
selected-root differential compares `verdict`, `class`, `line`,
`record_index`, `event_count`, `tip`, and `ordered_recomputed_hashes` for the
exact frozen 432 cases with isolated Python execution and repeat-two stability.
This is bounded selected-profile evidence, not trust, authority, universal
verifier equivalence, release readiness, or a release decision.

Administrative reconciliation after owner-merged PR #146: A-004, RQ-006, and
A-021 are Closed narrowly for the fixed portable language/profile after the
contract-qualified Rust/Python/Go evidence, hostile review, hosted CI, owner
merge, and separate ledger reconciliation. RQ-013 remains Partially
remediated with A-018 exact-boundary, broader platform/resource/crash, and
release/environment assurance scope open. RQ-015 remains Confirmed under its
controlled reproducibility disposition; A-011 remains Partially remediated.
Closure does not establish trust in the caller-supplied external key, key
ownership, identity, delegation, authority, permission, freshness, latest head,
canonical branch, anti-rollback, non-equivocation, global completeness,
production durability, truth, cryptographic proof, or release approval.
Historical audit records and the dated pre-alpha candidate assessment remain
untouched.

Landed: the complete crate-internal Rust portable-history conformer applies the
six ordered post-schema stages through one crate-internal implementation path
and produces the complete count/tip/hash trace used by all 432 frozen cases
(PR #139, 2026-08-24). PR #146 now reaches that authoritative semantics from
the real public file path; the crate-internal result boundary remains intact.
This remains a caller-supplied finite-history check, not trust, authority,
currentness, or global-completeness evidence.

Landed: bounded non-normative Rust fuzz/metamorphic assurance over the complete
crate-internal conformer, including focused hostile repairs (PR #140,
2026-08-25).
It is assurance evidence, not a normative oracle or proof of correctness.

Landed: the authoritative crate-internal Rust portable raw-input frontend through
`Schema`, including exact profile/external-key preflight, exact-byte framing and
UTF-8, the locked `serde_json` syntax seam, duplicate-preserving closed decoding,
and a private pending-record/exclusive-continuation boundary (PR #137,
2026-08-24). It proves 324 frontend-tranche-final outcomes and 108 later-stage
non-preemption cases, not complete history conformance or finding closure.

Landed: typed, explicit, fail-closed Rust `V_sig` profile selection and the exact
ADR-0010 relation as the namespaced `ProfiledSignatureVerifier` primitive; the
legacy unprofiled verifier remains unchanged (PR #135, 2026-08-24). This is a
cryptographic primitive landing, not complete portable conformance or finding
closure.

Landed: SQLite + FTS5 episodic projection (`magpie-episodic`, PR #5, 2026-07-02).
Landed: Deadbolt seam ratified as anchored hierarchy — ADR-0001, `SegmentAnchored`
tag 5, FORMAT.md §3 addition, extended golden vectors (six events, seqs 0-4
unchanged), tag-5 Python verifier (PR #8, 2026-07-03).
Landed: Deadbolt-side seam complete (deadbolt PRs #316–#320, 2026-07-03..05):
`cog-anchor` sole-writer crate; kernel, observation, codelet, and transaction
seal points emit fail-open with `pending-anchor.json` debt markers; `cog anchor
reconcile` re-verifies bundles then anchors idempotently via the `AnchorSet`
projection — the first consumer of anchors.
