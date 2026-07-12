# Ticket 0027: Replayed standing context snapshot

## Goal

Add a public read-only `StandingReplaySnapshot` that can be returned only after
one successful `LogReader::replay` has co-derived `StandingView` and
`DeadboltAnchorIndex` through a private composite projection.

## Problem

PR #37 guarantees that one replay verifies and folds one exact record snapshot.
The two claims projections are nevertheless public, independently constructible
views. Two separate replay calls do not prove that they observed the same store
state, record vector, replay invocation, or intended verifying-key context.

## Same-replay threat

```text
replay snapshot A -> StandingView
store changes
replay snapshot B -> DeadboltAnchorIndex
```

Both replays may succeed while producing views with different provenance. A
future policy caller must not mistake that pair for one coherent context.

## Construction law

```text
StandingReplaySnapshot exists
    -> one successful LogReader::replay completed
    -> StandingView and DeadboltAnchorIndex saw the same verified events
    -> both projections came from the same record snapshot
```

Trust in the verifying key supplied to the reader remains external. The
snapshot does not prove foreign bundle verification, actor authority,
admission, achieved standing, interpretation truth, policy wisdom, or
scientific correctness.

## Public API

```text
StandingReplaySnapshot
replay_standing_context(&LogReader<S>)
standing() -> &StandingView
anchors() -> &DeadboltAnchorIndex
event_count() -> u64
canonical_bytes() -> Vec<u8>
```

The deterministic bytes frame the event count and existing projection bytes.
They are derived replay-test material, not L0 canonical encoding or a persistent
format contract.

## Private composite projection

`StandingContextProjection` is module-private and implements `Projection`. Its
fixed fold order applies each exact event reference first to `StandingView` and
then to `DeadboltAnchorIndex`. `replay_standing_context` creates it fresh, calls
`LogReader::replay` once, and moves both views into the public snapshot only
after success.

## Privacy invariants

- Snapshot fields are private.
- There is no public `new`, `Default`, `Deserialize`, or `Projection`.
- There is no constructor from separate projections.
- There are no mutable accessors, setters, replacement methods, or parts APIs.
- The private composite projection is not exported.
- Existing independent public projections remain constructible and useful, but
  do not manufacture shared-replay provenance.

## Acceptance criteria

- The public construction function calls `LogReader::replay` exactly once.
- It does not call `verify_chain`, `events`, or a store read directly.
- Both internal views receive the same event references in one replay.
- A changing store cannot make the two views observe different snapshots.
- `event_count` is the count returned by that replay, not caller input.
- Replay failure returns the existing specific error and no public snapshot.
- The portable fixture produces two events, one exact anchor, and structurally
  empty standing.
- Rebuilding from the same log yields equal snapshots and deterministic bytes.
- No standing resolution or occurrence-context resolver is invoked.
- No existing projection API is removed or restricted.

## Adversarial test

`ChangingStandingStore` returns a valid typed-claim/evidence/edge plus anchor A
snapshot on its first read, and a second valid snapshot with anchor B appended
on later reads. The test calls only `replay_standing_context` and proves one
read, the first snapshot's event count and standing tables, anchor A present,
and anchor B absent.

Two independent replay calls would request both snapshots and could diverge;
the private composite receives each event once and forwards it to both views.

## Standing-inert boundary

The snapshot exposes existing read-only views but does not call
`resolved_standing`, `resolved_standing_with_trace`, or
`resolve_deadbolt_occurrence_context`. It adds no admission, promotion,
settlement, refutation, aggregation, policy selection, trace reason, or policy
identifier.

## Frozen surfaces

No changes to `magpie-log`, `LogStore`, `Projection`, `LogReader`, `Payload`, L0
tags, canonical encoding, content hashes, signatures, fixtures, goldens,
`tools/verify_chain.py`, standing resolution fields or bytes, trace fields or
reasons, policy ceilings/context requirements, Deadbolt occurrence schemas or
matching, `MAGPIE_CLAIMS_POLICY_ID`, Deadbolt, Cogitator, MCP, or writer
surfaces.

## Non-goals

No policy v1; achieved standing; verifier-context application; settlement;
support or refutation application; aggregation; admission or `EpistemicGate`;
human ratification; deterministic-verifier context; contradiction debt;
invalidation; supersession; currentness; independence groups; corroboration;
writer or MCP surfaces; foreign bundle verification; public verified snapshot
in `magpie-log`; stored verifying key or authority boolean; typestate;
capability token; locking; transaction; streaming replay; or network service.

## Validation

```text
git diff --check
cargo fmt --all -- --check
cargo test -p magpie-claims --locked
cargo test -p magpie-log --locked
cargo test --workspace --locked
cargo clippy --locked -p magpie-claims --all-targets -- -D warnings
cargo test -p magpie-claims standing_replay_snapshot --locked
cargo test -p magpie-claims deadbolt_occurrence --locked
cargo test -p magpie-claims standing_resolution --locked
cargo test -p magpie-claims --test standing_replay_snapshot --locked
cargo test -p magpie-claims --test standing_resolution --locked
python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737
rg -n "StandingReplaySnapshot|StandingContextProjection|replay_standing_context|LogReader::replay|impl Projection|Default|new\(|from_parts|into_parts|standing_mut|anchors_mut" crates/magpie-claims/src crates/magpie-claims/tests docs tickets
rg -n "MAGPIE_CLAIMS_POLICY_ID|StandingTraceReason|governed_standing|Settled|Supported|Refuted" crates/magpie-claims/src crates/magpie-claims/tests
```

## Follow-up

Ticket 0028 implements **Deadbolt occurrence/inclusion achieved-standing policy
v1** through this snapshot. Ticket 0027 itself remains construction-only and
defines no achieved-standing rule.
