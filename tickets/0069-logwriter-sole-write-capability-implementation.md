# Ticket 0069: Implement LogWriter sole-write capability boundary

## Status

Runtime/API remediation evidence for A-001 / RQ-001, authorized by the merged
sole-writer contract from PR #114.

This ticket does not edit the audit-disposition ledger and does not itself
administratively mark A-001 / RQ-001 Closed. The required sequence remains:

```text
implementation evidence
-> independent hostile review
-> owner merge
-> later audit-disposition reconciliation
```

## Resolved base and branch

```text
repository: noctem-o/magpie
origin/main: 142793a47c0f5a7e26ec29fc100d2d6cde9ddb3a
baseline merge: Merge pull request #114 from
  noctem-o/agent/logwriter-sole-write-capability-contract
branch: agent/logwriter-sole-write-capability
audit target: A-001 / RQ-001
```

At preflight, `HEAD`, `origin/main`, and their merge base were exactly the
recorded SHA and the isolated worktree was clean.

## Governing contract

- [`docs/design/logwriter-sole-write-capability-contract.md`](../docs/design/logwriter-sole-write-capability-contract.md)
- [`tickets/0068-logwriter-sole-write-capability-contract.md`](0068-logwriter-sole-write-capability-contract.md)
- PR #114, merged as
  `142793a47c0f5a7e26ec29fc100d2d6cde9ddb3a`

The architecture choice is already fixed:

```text
Option B

public custom stores
-> read-only substitution

writer backends
-> closed to magpie-log

FileStore / MemStore
-> supported through LogWriter

ordinary public caller without LogWriter
-> no Magpie-provided raw L0 append
```

## Exact implementation shape

The implementation uses no new public authority concept.

### Public read abstraction

`LogStore` retains its existing public name and contains only:

```rust
fn read_records(&self) -> Result<Vec<Vec<u8>>, LogError>;
```

Downstream implementations remain usable by `LogReader`, verification,
`replay`, `replay_with_summary`, and the existing claims replay contexts.

### Private persistence seam

`store.rs` contains one crate-private `WriterStore: LogStore` trait with
`append_record`. Only the crate-owned `FileStore` and `MemStore` implementations
and crate-internal test instrumentation implement it.

The trait and its method are not re-exported. Neither concrete store has a
public inherent append method.

### Public LogWriter surface

`LogWriter<S>` has private fields and no public storage-trait bound. Public
`open`, `open_with_clock`, and typed `append(Provenance, Payload)` methods are
defined only for:

```text
LogWriter<FileStore>
LogWriter<MemStore>
```

Private generic helpers over `WriterStore` share recovery, genesis, event
construction, hashing, signing, serialization, persistence, and state-update
logic. Public `tip`, `len`, and `is_empty` observations remain generic and grant
no construction or persistence authority.

Stable Rust does not disambiguate same-named associated constructors from their
argument types across two specialized inherent impls. Current built-in writer
call sites therefore spell the supported backend explicitly:

```text
LogWriter::<MemStore>::open(...)
LogWriter::<MemStore>::open_with_clock(...)
LogWriter::<FileStore>::open(...)
LogWriter::<FileStore>::open_with_clock(...)
```

This is a source-level constructor-qualification change for both supported
backends, not a behavior or capability change. It avoids both an inaccessible
private bound in generated public rustdoc and a new public marker trait.

This avoids a private bound in a public API, an `allow(private_bounds)`, a
public sealed writer trait, a token, a callback, a registry, or a runtime mode.

## Writer sequencing and failure law

The shared private append helper preserves this exact order:

```text
validate genesis position and payload
-> bind current sequence and previous hash
-> obtain timestamp
-> construct EventCore
-> hash
-> sign
-> construct SignedEvent
-> serialize existing JSON representation
-> call crate-private persistence seam
-> update last_hash and next_seq only on Ok
```

A crate-internal instrumented backend exercises both writer recovery and an
injected persistence error. The error path must leave writer tip, length, and
stored records unchanged; the subsequent success must use the original next
sequence and predecessor.

## Source compatibility change

Before:

```rust
impl LogStore for CustomStore {
    fn append_record(&mut self, bytes: &[u8]) -> Result<(), LogError> {
        // externally supplied writer backend
    }

    fn read_records(&self) -> Result<Vec<Vec<u8>>, LogError> {
        // reader backend
    }
}

// Accidentally supported:
// LogWriter<CustomStore>
```

After:

```rust
impl LogStore for CustomStore {
    fn read_records(&self) -> Result<Vec<Vec<u8>>, LogError> {
        // reader backend
    }
}

// Intentionally unsupported:
// LogWriter<CustomStore>
```

This is an explicit pre-1.0 source break for custom writable backends. It is not
described as source-compatible. `FileStore` and `MemStore` writer behavior is
preserved, while custom reader implementations become smaller and no longer
need fake or panicking write methods.

## Custom-store migrations

Reader-only integration fixtures lose only their obsolete `append_record`
method:

- `ChangingSnapshotStore` in `crates/magpie-log/tests/chain.rs`;
- `ChangingStandingStore` in
  `crates/magpie-claims/tests/standing_replay_snapshot.rs`;
- `OneReadStore` in
  `crates/magpie-claims/tests/origin_admission_replay_substrate.rs`; and
- `ChangingStore` in
  `crates/magpie-claims/tests/deterministic_verifier_context.rs`.

`CountingStore` remains an external-style read wrapper for replay read-count
tests. Its writer-recovery role moves into the crate-internal
`InstrumentedWriterStore`, which legitimately implements the private seam.

## Hostile negative proofs

Crate-level compile-fail doctests use current imports, current constructor
arity, and valid setup before reaching the intended missing capability:

1. `FileStore` has no `append_record` method through the public API;
2. `MemStore` has no `append_record` method through the public API;
3. `S: LogStore` has no append member;
4. `LogReader` has no append method;
5. `LogReader` has no backend-extraction method;
6. `LogWriter<MyReadStore>` has no supported `open`; and
7. `SigningKey` plus `MemStore` has no persistence operation.

Each proof expects method-resolution failure `E0599`. The doctest suite and a
separate diagnostic inspection must confirm that the failure occurs at that
method/capability boundary rather than at setup, imports, ownership, syntax, or
constructor arity.

Repository-wide public API scans must additionally prove that there is no
public `append_record`, `append_raw`, `write_record`, persistence adapter,
extension-trait sink, or equivalent public backend mutation path.

## Positive proofs

- `ChangingSnapshotStore` continues to drive `LogReader::verify_chain`,
  `replay`, and `replay_with_summary` as a custom public read store.
- The three claims custom read stores continue to drive standing and origin
  replay contexts.
- `LogWriter<MemStore>` creates genesis, appends, verifies, reopens, and
  continues a valid chain.
- `LogWriter<FileStore>` creates a chain, reopens with exact count and tip,
  appends the next record, and verifies the continued chain.
- Empty open writes exactly one genesis through the private seam.
- Non-empty open verifies and recovers without a second genesis.
- Injected persistence failure leaves `tip` and `len` unchanged, and the next
  successful record uses the original expected sequence and predecessor.
- Golden and portable Deadbolt-anchor fixtures remain byte-for-byte unchanged
  and independently verifiable.

## Exact changed-path allowlist

```text
crates/magpie-claims/examples/tour.rs
crates/magpie-claims/src/claim_inline_sha256_predicate/edge_lane.rs
crates/magpie-claims/src/origin_admission_replay.rs
crates/magpie-claims/src/origin_binding_verifier.rs
crates/magpie-claims/src/standing.rs
crates/magpie-claims/src/standing_v2.rs
crates/magpie-claims/src/standing_v3.rs
crates/magpie-claims/src/support_contribution_audit.rs
crates/magpie-claims/tests/admitted_contribution_audit.rs
crates/magpie-claims/tests/artifact_provenance_verifier.rs
crates/magpie-claims/tests/claim_inline_predicate_edge_lane.rs
crates/magpie-claims/tests/claim_inline_sha256_predicate.rs
crates/magpie-claims/tests/deadbolt_occurrence_context.rs
crates/magpie-claims/tests/deadbolt_occurrence_standing_v1.rs
crates/magpie-claims/tests/deterministic_support_standing_v2.rs
crates/magpie-claims/tests/deterministic_verifier_context.rs
crates/magpie-claims/tests/inline_predicate_attestation.rs
crates/magpie-claims/tests/origin_admission_audit.rs
crates/magpie-claims/tests/origin_admission_replay_substrate.rs
crates/magpie-claims/tests/origin_binding_verifier.rs
crates/magpie-claims/tests/regenerable.rs
crates/magpie-claims/tests/resolution_content_closure.rs
crates/magpie-claims/tests/standing_replay_snapshot.rs
crates/magpie-claims/tests/standing_resolution.rs
crates/magpie-claims/tests/standing_v3.rs
crates/magpie-claims/tests/standing_v4.rs
crates/magpie-claims/tests/support_contribution_audit.rs
crates/magpie-episodic/tests/episodic.rs
crates/magpie-log/examples/regen_golden.rs
crates/magpie-log/src/store.rs
crates/magpie-log/src/logimpl.rs
crates/magpie-log/src/lib.rs
crates/magpie-log/tests/chain.rs
crates/magpie-log/tests/golden.rs
tickets/0069-logwriter-sole-write-capability-implementation.md
```

No other `impl LogStore` or generic custom-writer surface exists on the resolved
baseline. The broad path set is mechanically required by the stable-Rust
constructor qualification above: outside the core implementation, the four
read-only fixture migrations, and this ticket, each listed source file changes only
`LogWriter::open[_with_clock]` to `LogWriter::<MemStore>::open[_with_clock]`.
Actual changed paths must equal this allowlist.

## Validation ladder

Required focused and complete checks:

```text
git diff --check
cargo fmt --all --check
cargo test -p magpie-log --locked
cargo test --doc -p magpie-log --locked
cargo test -p magpie-log --test chain --locked
cargo test -p magpie-claims --test standing_replay_snapshot --locked
cargo test -p magpie-claims --test origin_admission_replay_substrate --locked
cargo test -p magpie-claims --test deterministic_verifier_context --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked --no-fail-fast
cargo run --locked --example tour -p magpie-claims
python tools/check_release_metadata.py
cargo package -p magpie-log --locked
python tools/verify_chain.py crates/magpie-log/testdata/golden-v1.jsonl ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c
python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737
```

Also required:

- manual `E0599` diagnostic inspection for every compile-fail scenario;
- repository-wide public API and alias scans;
- exact changed-path comparison against the allowlist;
- unchanged hashes for `docs/FORMAT.md`, both frozen chain fixtures, the
  audit-disposition ledger, and the merged #114 contract/handoff; and
- a clean committed tree before the direct package check and publication.

Golden or fixture regeneration is prohibited.

## Exact non-goals

This implementation does not:

- add filesystem locking, durability, sync, journaling, framing, concurrency,
  atomicity, snapshot, or resource semantics from A-008 / RQ-007 / RQ-008;
- change `LogReader::events`, `SignedEvent`, `Projection::apply`, or the
  verified/unverified type boundary from A-009 / RQ-002;
- implement `EpistemicGate`, generic admission, rejection history, retry
  authority, Deadbolt writing, claim/evidence writers, or A-022;
- change A-004 grammar, A-014 anchor validation, A-021 root strictness, or
  A-024 authority-bound corroboration;
- change payloads, tags, canonicalization, signatures, hashes, JSON record
  representation, genesis, verifying-key semantics, standing, projection,
  fixtures, `docs/FORMAT.md`, dependencies, or release metadata; or
- edit historical audits or `docs/audits/audit-disposition-2026-08.md`.

## Administrative handoff

This PR may state that it provides runtime remediation evidence for A-001 /
RQ-001. It must not claim the living ledger is already Closed. Independent
hostile review should attack:

- alternate public raw persistence paths or aliases;
- private-bound or crate-private seam leakage into public API;
- accidental retention of external custom writer compatibility;
- loss of external custom reader compatibility;
- compile-fail cases that fail before the intended boundary;
- writer-state advancement after persistence error;
- canonical or fixture-byte drift; and
- scope leakage into A-008, A-009, A-022, or A-024.

Human owner retains sole merge authority. Publication is one commit, one push,
and one draft PR against `main`; do not mark ready, enable auto-merge, or merge.
