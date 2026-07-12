# Ticket 0026: Replay one verified record snapshot

## Goal

Eliminate the time-of-check-to-time-of-use seam in `LogReader::replay` by
reading one record snapshot, fully parsing and verifying that exact snapshot,
and applying only the events retained from it.

## Problem

The former replay path performed two independent store reads:

```text
read snapshot A
    -> verify snapshot A
    -> read snapshot B
    -> parse and apply snapshot B
```

`LogStore` does not promise immutable or repeatable reads. A mutable, racy, or
adversarial implementation could therefore return a valid chain first and
inject an unverified event into the second snapshot consumed by `events()`.

## Security law

```text
read one snapshot
    -> parse and verify that exact snapshot completely
    -> apply the exact verified events from that snapshot
```

For every successful replay:

```text
applied_events == verified_events
```

`verify(snapshot A); apply(snapshot B)` must be impossible when `A != B`.

## Verification-before-apply invariant

The complete returned record vector must pass all existing checks before the
first `Projection::apply` call:

- record JSON parsing;
- expected sequence number;
- previous-hash linkage;
- recomputed content hash;
- Ed25519 signature;
- payload validation;
- exactly one genesis at sequence zero;
- canonicalisation profile;
- genesis-declared verifying-key binding;
- absence of a later genesis.

On any parsing or verification failure, replay returns the existing specific
error and leaves the supplied projection untouched. This atomicity guarantee
does not cover a projection whose own `apply` implementation panics or is
internally incorrect.

## Implementation

- Add a private helper that accepts an already-read `&[Vec<u8>]` and the
  verifying key.
- Parse and verify the entire slice in the existing check order.
- Retain the parsed events privately and return them only after full success,
  together with the recovered tip.
- Make `LogReader::replay` call `read_records()` exactly once and fold the
  retained events.
- Reuse the helper for `verify_chain` and writer recovery without changing
  their public signatures or semantics.
- Keep `events()` as an explicitly unverified parsing API.

No public verified wrapper or new authority-bearing construction is added.

## Adversarial regression design

A deterministic test-only `ChangingSnapshotStore` returns:

```text
first read  -> valid genesis + valid Note
second read -> the same prefix + forged SegmentAnchored signature
```

A recording projection and observable read counter prove that replay reads
exactly once, succeeds over the first snapshot, applies only genesis and Note,
and never observes the injected anchor. The test fails under the former
two-read flow because that flow requests and folds the second snapshot.

A second test supplies one snapshot with a valid prefix and a later bad
signature. It proves the verification error is returned and the recording
projection remains empty.

## Acceptance criteria

- `LogReader::replay` calls `read_records` exactly once.
- The exact returned record vector is fully parsed and verified.
- `Projection::apply` receives only events parsed from that verified vector.
- No projection event is applied before the full vector verifies.
- A second-snapshot injection store cannot smuggle an event into replay.
- A later invalid record cannot leave a valid prefix applied.
- `verify_chain` public behaviour and exact verified count remain unchanged.
- `events` remains explicitly unverified.
- Writer count/tip recovery and empty-store genesis bootstrap remain unchanged.
- Empty reader verification still reports zero events.
- Existing error variants, sequence attribution, and first-failure ordering are
  preserved.
- No public API or log-format change is introduced.

## Frozen surfaces

No changes to `LogStore`, `Projection`, `Payload`, L0 tags, `docs/FORMAT.md`,
canonical bytes, content hashes, signature domain or encoding, genesis or
`SegmentAnchored` formats, golden vectors, fixtures, `tools/verify_chain.py`,
standing types or behaviour, `DeadboltAnchorIndex` behaviour, occurrence
metadata schemas, policy ceilings or IDs, Deadbolt, Cogitator, or writer-facing
authority surfaces.

## Non-goals

No file or cross-process locking; store transactions or snapshot API; public
`VerifiedSnapshot`; typestate or capability token; verified-index wrapper;
combined standing/anchor replay; standing-policy v1; achieved `Settled`;
admission; aggregation; projection rollback; asynchronous or streaming replay;
mmap, SQLite, or network storage; Deadbolt integration; or Verus proof.

## Validation

```text
git diff --check
cargo fmt --all -- --check
cargo test -p magpie-log --locked
cargo test --workspace --locked
cargo clippy --locked -p magpie-log --all-targets -- -D warnings
cargo test -p magpie-log replay --locked
cargo test -p magpie-log chain --locked
cargo test -p magpie-log --test chain --locked
python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737
rg -n "read_records|verify_chain|events\(|replay<|Projection::apply" crates/magpie-log/src crates/magpie-log/tests
```

## Relationship to future standing policy

Ticket 0025 requires `DeadboltAnchorIndex` and future standing projections to
come from the same successful verified replay under the intended key. This
ticket supplies the necessary single-snapshot replay property: the same
immutable record vector is verified and folded unchanged.

It does not make a manually constructed `DeadboltAnchorIndex` verified, admit
evidence, integrate standing, or settle any claim. Those remain future
policy-bearing work.

Ticket 0027 consumes this guarantee by co-deriving `StandingView` and
`DeadboltAnchorIndex` through one replay invocation without changing standing.
