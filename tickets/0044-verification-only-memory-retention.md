# Ticket 0044: Verification-only parsed-event retention optimisation

## 1. Exact resolved base and publication metadata

This implementation starts from the exact post-PR #56 `main` commit:

```text
351333ac17cb1d7b82d0ee144568d48a7406f80a
```

That base contains the reviewed PR #56 head:

```text
0b1562f9ceb6405e9dacae5ee7a013c5b9a09def
```

The branch is exactly:

```text
agent/verification-only-memory-retention
```

The commit title and PR title are both exactly:

```text
log: avoid retaining parsed events during verification
```

## 2. Purpose and current debt

The current private verifier serves trusted replay, public chain verification,
and writer recovery through one `VerifiedSnapshot` containing every parsed
`SignedEvent`. Verification-only callers read and materialise the complete raw
record snapshot, parse and completely verify it, retain a second complete
parsed-event vector, extract only its length and tip, and then discard it.

This ticket removes only that unnecessary parsed-event retention. The intended
verification-only peak retention is conceptually:

```text
complete raw-record snapshot
+ one transient parsed SignedEvent
+ count and tip
```

rather than:

```text
complete raw-record snapshot
+ complete parsed SignedEvent vector
```

The architectural classification is:

```text
correctness: unchanged
authority boundaries: unchanged
availability and long-run scalability: improved
```

The precise scope boundary is:

```text
summary-only verification
!= streaming store verification

discarding parsed events
!= weakening validation
```

## 3. Two explicit private modes

One shared private record-verification engine has two closed retention modes.

### Verification-only summary

```text
verification-only mode
→ read one record snapshot
→ parse and completely verify every record
→ retain only exact event count and chain tip
```

This mode is used by `LogReader::verify_chain` and by `LogWriter::open` /
`LogWriter::open_with_clock` recovery. Its result contains no event collection,
and the loop drops each parsed `SignedEvent` after that event has passed every
validation check and advanced count and tip.

### Retained replay snapshot

```text
replay mode
→ read one record snapshot
→ parse and completely verify every record
→ retain the exact verified events from that snapshot
→ apply them only after the complete snapshot succeeds
```

This mode is used only by `LogReader::replay`. Its private result contains the
retained event vector and verified tip. Replay derives its returned count from
that exact retained sequence.

No retention selector or result type becomes public. Callers cannot substitute
a summary for a replay snapshot, and no caller-provided callback exists.

## 4. One shared verification engine and validation order

Both modes must use one common private loop. Independent summary and replay
verification implementations are prohibited because their validation and error
behaviour could drift.

The shared loop preserves this order for every record:

```text
deserialize SignedEvent
→ verify expected sequence
→ verify previous-hash linkage
→ recompute and verify content hash
→ verify signature
→ validate payload
→ enforce genesis rules and verifying-key binding
→ advance count and tip
→ optionally retain the verified event
```

All existing `LogError` variants, failing sequence reports, dependency order,
and fail-closed behaviour remain unchanged. Discarding the parsed value after
successful validation creates no alternate trust path.

## 5. One-read and verify-before-apply laws

The trusted replay law remains:

```text
one snapshot read
→ complete parsing and verification
→ only then projection mutation
```

Each operation calls `LogStore::read_records()` exactly once. Replay retains
the events parsed from that exact returned snapshot, completes verification of
every record, and only then calls `Projection::apply`. A late parse, chain,
hash, signature, payload, genesis-profile, or key-binding failure applies no
prefix.

No production callback may receive verified prefix events while later records
remain unchecked.

## 6. Empty chains, genesis, and writer recovery

An empty record snapshot verifies to count zero and `ContentHash::ZERO`. Opening
a writer on that empty store still appends exactly one valid genesis event.

Opening a writer on a non-empty store uses summary-only verification once,
recovers the exact count and tip, does not append another genesis, and continues
at the next sequence. The existing canonicalisation-profile and externally
provided verifying-key binding checks remain mandatory.

## 7. Exact allowed paths

Exactly these paths may change:

```text
tickets/0044-verification-only-memory-retention.md
crates/magpie-log/src/logimpl.rs
crates/magpie-log/tests/chain.rs
```

Ticket 0044 is new. Completed tickets and the historical remediation ledger
remain unchanged. No manifest, lockfile, format document, fixture, verifier, or
store implementation is in scope.

## 8. Frozen public and canonical surfaces

Change none of:

- `LogStore`, `FileStore`, or `MemStore` behaviour;
- the public `LogReader`, `LogWriter`, or `Projection` APIs;
- `SignedEvent`, `EventCore`, `Payload`, payload tags or encodings,
  `Provenance`, or `ContentHash`;
- canonical byte encoding, `magpie-core-v1`, `magpie-sig-v1`, signature or hash
  preimages;
- genesis format, trust-root rules, or payload validation;
- the public `LogError` vocabulary or sequence reporting;
- golden vectors, fixture bytes, `docs/FORMAT.md`, or the independent Python
  chain verifier; and
- dependency manifests and `Cargo.lock`.

This ticket adds no streaming, cursor, live iterator, `BufRead`, mmap, SQLite,
async I/O, parallel verification, cache, checkpoint trust, partial
verification, prefix application, second store read, or public retention
selector.

## 9. Required hostile tests

The implementation must add focused coverage proving:

1. `summary_only_verification_returns_count_and_tip_without_events`: a valid
   multi-event snapshot returns exact count and tip in a structural summary
   variant with no event collection; retained mode over the same records
   returns the same count and tip plus the exact event sequence.
2. `verify_chain_uses_one_record_snapshot`: a changing store returns one valid
   first snapshot and a different invalid hypothetical second snapshot;
   `verify_chain` returns the first count and reads once.
3. `writer_recovery_verifies_once_and_recovers_exact_count_and_tip`: a
   non-empty counting store is read once; writer length and tip are exact;
   reopening adds no genesis; and the next append has the correct sequence and
   previous hash.
4. Verification-only and replay over equivalent late-bad-signature snapshots
   report `BadSignature` at the same sequence, while replay applies no prefix.
5. Existing hostile replay tests continue proving one snapshot read and no
   prefix application before a later failure.
6. Existing genesis, count, recovery, file-store, payload-tampering, forged
   rehash, truncation, second-genesis, profile, key-binding, and payload
   validation tests remain green.

Tests must not expose the private retention mode through a public accessor.

## 10. Validation commands

Run focused checks first:

```powershell
cargo test -p magpie-log --locked --lib
cargo test -p magpie-log --locked --test chain
```

Then run the complete required baseline:

```powershell
git diff --check
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo run --locked --example tour -p magpie-claims
python tools/check_release_metadata.py
cargo package -p magpie-log --locked
```

Run both independent chain verifications:

```powershell
python tools/verify_chain.py `
  crates/magpie-log/testdata/golden-v1.jsonl `
  ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c

python tools/verify_chain.py `
  fixtures/deadbolt-anchor-v1/anchor-log.jsonl `
  d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737
```

Every command must actually pass before publication. A known
`CreateProcessAsUserW failed: 5` shell failure is reported without repeated
retries, and publication is prohibited unless all required validation runs
successfully through a functioning shell.

## 11. Deferred `LogStore` boundary work

```text
LogStore::read_records
still materialises the complete raw-record snapshot
```

Avoiding that cost requires a separate design for a snapshot-safe store
interface. `summary-only verification != streaming store verification`.

A live iterator over an append-only file raises unresolved questions about
exact snapshot boundaries, concurrent append visibility, failure and retry
semantics, ownership and borrowing, replay retention, and deterministic
end-of-snapshot recognition. This ticket does not casually replace
`read_records()` with an iterator or claim streaming verification.

## 12. Worker and publication authority

`AGENTS.md` remains binding except for the publication actions George
explicitly authorises for this ticket after the exact scope passes every
required validation. The worker may stage only the three permitted paths,
create one commit with the exact title, push only the named branch without
force, and open one draft PR with the exact title.

The worker may not mark the PR ready, merge, enable auto-merge, resolve review
threads, dismiss reviews, modify another PR, force-push, modify `main`, or
publish any additional branch or commit. Validation failure or a broader
changed-path list prohibits publication.

## 13. Reviewer questions

### Does summary mode skip payload, signature, or genesis validation?

No. Both result modes use the same loop and validation order. Only the final
retention action differs.

### Can replay apply an event as soon as that event verifies?

No. Replay receives the retained snapshot only after the complete record
snapshot verifies and starts projection mutation afterwards.

### Does verification read once and replay read twice?

No. Each operation reads exactly one record snapshot. Replay retains parsed
events specifically to avoid snapshot skew or a second read.

### Is this streaming verification?

No. The raw-record snapshot remains fully materialised by
`LogStore::read_records()`.

### Does the optimisation change correctness or authority?

No. Correctness and authority boundaries are unchanged. Availability and
long-run scalability improve because verification-only callers no longer
retain the complete parsed `SignedEvent` vector.

### How is the memory claim tested?

The private summary result structurally contains only count and tip, and the
summary path never constructs a `Vec<SignedEvent>`. Tests prove the closed
result shape and behavioural parity. They do not claim a direct peak-memory
measurement.

## 14. Stop conditions

Stop after recording the smallest clear failing test and report the divergence
if implementation would require any of:

- a second `read_records()` call;
- projection mutation during verification or prefix application;
- a `LogStore` trait or other public API change;
- a new error variant or changed sequence report;
- altered canonical, hash, signature, genesis, payload, golden-byte, or
  fixture behaviour;
- a dependency addition;
- a production callback capable of applying events early;
- separate verification logic for summary and replay modes; or
- edits outside the three allowed paths.

Reviewer stop conditions before publication are any failed required command,
any fourth changed path, any semantic frozen-surface diff match, more than one
commit, a non-draft PR, or branch/base/head metadata that differs from this
ticket.
