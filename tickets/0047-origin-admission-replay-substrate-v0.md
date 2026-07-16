# Ticket 0047: Origin-admission replay substrate v0

## 1. Publication identity and exact base

```text
base: 1ca24895d8922461edf4b7bc6d088deb3537bba9
branch: agent/origin-admission-replay-substrate-v0
commit: claims: add origin-admission replay substrate v0
PR title: claims: add origin-admission replay substrate v0
PR state: draft
```

The base is the merge commit of PR #59. The reviewed PR #59 head is
`0f2e1eb346fdd4566be18c87487c263be1e4aeac`, and it must be an ancestor of
that exact base before implementation begins.

## 2. Outcome and authority classification

This slice implements exactly two replay-derived capabilities:

```text
exact same-snapshot verified log-prefix identity
+
complete deterministic origin-binding candidate enumeration
```

Its exact classification is:

```text
correctness:
same-snapshot verified replay identity and candidate enumeration

authority:
no origin admission

standing:
unchanged

closure evaluation:
not implemented

conflict fold:
not implemented
```

The result identifies history and enumerates supported replay occurrences. It
does not decide what any binding means or trust any authority claim.

## 3. Exact changed-path allowlist

Only these paths may change:

```text
README.md
tickets/0047-origin-admission-replay-substrate-v0.md
crates/magpie-log/src/lib.rs
crates/magpie-log/src/logimpl.rs
crates/magpie-log/tests/chain.rs
crates/magpie-claims/src/lib.rs
crates/magpie-claims/src/replay_snapshot.rs
crates/magpie-claims/src/deadbolt_context.rs
crates/magpie-claims/src/origin_admission_replay.rs
crates/magpie-claims/tests/origin_admission_replay_substrate.rs
```

All historical tickets and design contracts remain unchanged evidence. No
manifest, lockfile, fixture, L0, standing, policy, verifier, closure, tool, CI
or Deadbolt path is in scope.

## 4. Central architectural law

```text
one completely verified record snapshot
-> one exact verified log-prefix identity
-> one co-replayed StandingView
-> one co-replayed DeadboltAnchorIndex
-> one complete supported origin-binding candidate universe
```

Every output shares one retained verified event vector. The critical
distinctions are:

```text
projection equality
!= exact log-prefix identity

anchor occurrence
!= verified origin-binding statement

complete candidate universe
!= completed origin-admission audit

replay substrate
!= admission authority
```

The slice establishes only which exact Magpie history was replayed and which
supported origin-binding anchor identities occurred in it. It establishes no
binding-byte availability or validity, trusted authority, admitted group,
duplicate/conflict outcome, support, standing, aggregation or independence.

## 5. `VerifiedReplaySummary`

`magpie-log` adds and re-exports:

```rust
pub struct VerifiedReplaySummary {
    event_count: u64,
    tip: ContentHash,
}
```

It derives only `Clone`, `Copy`, `Debug`, `PartialEq` and `Eq`. Its fields are
private. It has no `Default`, `Serialize`, `Deserialize` or public constructor.
It exposes exactly:

```rust
impl VerifiedReplaySummary {
    pub fn event_count(&self) -> u64;
    pub fn tip(&self) -> ContentHash;
}
```

A value is successful replay audit output. A copied value cannot authorize
application of another record set, and no replay API accepts it as input.

## 6. `LogReader::replay_with_summary`

The only additive replay method is:

```rust
pub fn replay_with_summary<P: Projection>(
    &self,
    projection: &mut P,
) -> Result<VerifiedReplaySummary, LogError>
```

Its exact control flow is:

```text
one LogStore::read_records() call
-> parse and verify the complete returned snapshot
-> retain the exact verified events and verified tip
-> apply no projection event until full verification succeeds
-> apply every retained verified event in sequence
-> return event_count and tip from that exact snapshot
```

It uses the existing retained `VerifiedSnapshot` from the shared verification
loop. It does not call `events()`, call `verify_chain()`, perform a second store
read, recompute a tip from projection state or verify one snapshot and apply
another. A late failure returns the existing first `LogError`, mutates no
projection and returns no summary.

## 7. Existing replay compatibility

The existing public signature remains exactly:

```rust
pub fn replay<P: Projection>(
    &self,
    projection: &mut P,
) -> Result<u64, LogError>
```

It delegates to `replay_with_summary` and returns only
`summary.event_count()`. This preserves one store read, error type and
first-error behavior, validation order, genesis checks, payload checks,
verify-before-apply, projection event order, successful count and external
verifying-key trust.

`LogReader::verify_chain`, `LogReader::events`, writer recovery,
summary-only `VerifiedSummary`, canonical encoding, hashes, signatures and L0
validation remain unchanged.

## 8. Empty snapshot behavior

The existing empty-store law is preserved:

```text
event_count = 0
tip = ContentHash::ZERO
no projection events applied
```

No new empty-chain error exists.

## 9. `VerifiedLogPrefixIdentityV0`

`magpie-claims` adds and re-exports:

```rust
pub struct VerifiedLogPrefixIdentityV0 {
    event_count: u64,
    tip_sha256: String,
}
```

It derives `Clone`, `Debug`, `PartialEq`, `Eq`, `PartialOrd`, `Ord` and
`Serialize`. Fields are private. It has no `Deserialize`, `Default` or public
constructor. It exposes exactly:

```rust
impl VerifiedLogPrefixIdentityV0 {
    pub fn event_count(&self) -> u64;
    pub fn tip_sha256(&self) -> &str;
}
```

Private construction consumes the `VerifiedReplaySummary` returned by the
same replay used for claims projections. `tip_sha256` is exactly
`summary.tip().to_hex()`: 64 lowercase hexadecimal characters. Projection
bytes, serialized summaries and caller strings are never hashed or accepted
as prefix authority, and no second digest is introduced.

## 10. `OriginAdmissionReplayContextV0`

The separately versioned public carrier is:

```rust
pub struct OriginAdmissionReplayContextV0 {
    snapshot: StandingReplaySnapshot,
    verified_prefix_identity: VerifiedLogPrefixIdentityV0,
}
```

It derives only `Clone`, `Debug`, `PartialEq` and `Eq`. Fields are private. It
has no `Serialize`, `Deserialize`, `Default` or public constructor. It exposes
exactly:

```rust
impl OriginAdmissionReplayContextV0 {
    pub fn snapshot(&self) -> &StandingReplaySnapshot;

    pub fn verified_prefix_identity(
        &self,
    ) -> &VerifiedLogPrefixIdentityV0;
}
```

No API accepts a separately supplied snapshot and prefix identity. This makes
cross-replay authority transposition impossible through the public surface.

## 11. Trusted same-replay construction

The only new public claims construction path is:

```rust
pub fn replay_origin_admission_context_v0<S: LogStore>(
    reader: &LogReader<S>,
) -> Result<OriginAdmissionReplayContextV0, LogError>
```

It uses one private `StandingContextProjection`, calls
`reader.replay_with_summary(&mut projection)` exactly once, constructs the
unchanged `StandingReplaySnapshot` from that projection and summary count, and
constructs the prefix identity from the same summary.

```text
one retained verified event vector
-> StandingView
-> DeadboltAnchorIndex
-> event_count
-> tip_sha256
```

The private shared path is also used by `replay_standing_context`, so the old
and new constructors cannot drift in projection construction.

## 12. Existing standing replay compatibility

The existing function remains:

```rust
pub fn replay_standing_context<S: LogStore>(
    reader: &LogReader<S>,
) -> Result<StandingReplaySnapshot, LogError>
```

Its result, errors, count, standing projection, anchor projection and canonical
bytes remain unchanged. The tip is not added to `StandingReplaySnapshot`; no
field, getter, derive or serialization change is made. Exact prefix identity
exists only in the separately versioned origin-admission replay context.

## 13. Private deterministic anchor enumeration

`DeadboltAnchorIndex` adds only a crate-private read-only iterator over its
actual internal `BTreeMap`, yielding identity references and occurrence
slices. It exposes no mutation, map ownership or new public method. Existing
public lookup and canonical bytes are unchanged.

The crate-private candidate type is:

```rust
pub(crate) struct OriginBindingCandidateV0 {
    selector: ArtifactProvenanceAnchorSelectorV0,
    occurrences: Vec<DeadboltAnchorOccurrence>,
}
```

It derives `Clone`, `Debug`, `PartialEq` and `Eq`, has private fields and only
crate-private read-only getters. It is neither serialized nor deserialized and
is not re-exported.

The crate-private enumeration method accepts no selector list, closure,
receipt list, caller-created anchor index, policy argument or callback. It
derives candidates only from `context.snapshot().anchors()`.

## 14. Exact candidate filter, selector, ordering and multiplicity

An identity is included only when all of these exact comparisons succeed:

```text
bundle_kind
in {
  magpie-origin-binding-acquisition-v0,
  magpie-origin-binding-derivation-v0
}

witness_algorithm == sha256

canonicalization_profile == magpie-origin-binding-json-v0
```

Implementation reuses the Ticket 0045 constants. It does not additionally
validate root syntax, run-ID syntax, closure availability, bundle contents,
authority claims, policy claims or contribution identities.

Each included identity constructs one exact
`ArtifactProvenanceAnchorSelectorV0` in this field order:

```text
bundle_kind
witness_root
witness_algorithm
canonicalization_profile
run_id
```

There is no normalization, trimming, case folding, aliasing or fallback. The
candidate vector preserves the existing `DeadboltAnchorIdentity` `BTreeMap`
order, which is exact five-field lexical order. It is not event, run-ID,
root-only, family-only, first-occurrence, closure or caller order.

One distinct identity produces exactly one candidate. Every exact occurrence
remains attached in ascending replay sequence:

```text
three identical SegmentAnchored events
-> one candidate selector
-> three retained occurrences
-> no candidate or authority amplification
```

## 15. Public and private authority boundaries

The complete new public claims construction path is:

```text
LogReader
-> replay_origin_admission_context_v0
-> OriginAdmissionReplayContextV0
```

Compile-fail documentation proves:

1. `VerifiedLogPrefixIdentityV0` cannot be built with a struct literal;
2. it cannot be deserialized;
3. `OriginAdmissionReplayContextV0` cannot be built with a struct literal;
4. it cannot be deserialized;
5. no public constructor accepts a snapshot plus prefix identity;
6. no public candidate enumerator accepts a caller-created anchor index;
7. `VerifiedReplaySummary` cannot be built with a struct literal outside
   `magpie-log`; and
8. it cannot be deserialized.

No test-only public constructor exists. Serialized identity material cannot be
fed back as replay authority.

## 16. Hostile test matrix

`magpie-log` tests prove:

- `replay_with_summary` reads exactly one record snapshot;
- count and tip equal the exact retained verified chain;
- the projection observes the same complete event order;
- legacy replay and summary replay have equal results and hostile errors;
- both paths use one store read;
- a late bad signature applies no prefix and returns no summary;
- an empty store returns count zero and `ContentHash::ZERO`; and
- no API accepts a copied summary to authorize another record set.

Claims integration tests prove:

- typed standing material, supported anchors and unrelated events co-derive
  exact count, tip, standing and anchor results;
- the old snapshot path is equal and byte-identical;
- equal projections from different note histories have different prefix
  identities;
- origin-context construction performs one store read;
- a late invalid record yields no context; and
- snapshot, standing, v0, v1 and v2 canonical bytes are unchanged.

Crate-private unit tests prove both supported families, separate exclusion of
wrong kind/algorithm/profile, three-occurrence retention, exact identity
separation, five-field lexical ordering and absence of caller/closure
influence. Unsupported anchors remain in the underlying index but outside the
v0 candidate universe.

## 17. Frozen surfaces

Change none of:

```text
magpie-core-v1
magpie-sig-v1
canonical event encoding
ContentHash representation
payload tags or validation
genesis rules
signature verification
LogStore API
LogWriter API
LogReader::events
LogReader::verify_chain
existing LogReader::replay signature and errors
StandingView fields or canonical bytes
StandingReplaySnapshot fields, getters or canonical bytes
DeadboltAnchorIdentity field order
DeadboltAnchorOccurrence representation
DeadboltAnchorIndex public API or canonical bytes
origin-binding verifier public API, trace bytes or receipt bytes
artifact-provenance verifier
ResolutionContentClosureV0
standing policies v0/v1/v2
support and refutation ceilings
Cargo manifests, dependencies, lockfile and fixtures
```

The only additive log API is `VerifiedReplaySummary` plus
`LogReader::replay_with_summary`. The only additive public claims API is
`VerifiedLogPrefixIdentityV0`, `OriginAdmissionReplayContextV0` and
`replay_origin_admission_context_v0`. Candidate enumeration remains
crate-private.

## 18. Explicit non-goals

This slice implements no closure lookup, binding-bundle availability,
origin-binding parsing change, candidate disposition, policy matching,
authority trust, origin admission, duplicate collapse, conflict fold,
unresolved blocking, `OriginAdmissionAuditV0`, audit serialization, admitted
origin, admitted contribution, support contribution, policy v3, aggregation,
statistical independence, threshold, reputation, supersession, revocation,
expiry, loader, CAS, filesystem/network/callback/plugin access, cache, writer,
new L0 payload or Deadbolt change.

## 19. Validation commands

Focused validation:

```powershell
cargo test -p magpie-log --locked --lib
cargo test -p magpie-log --locked --test chain

cargo test -p magpie-claims --locked --lib
cargo test -p magpie-claims --locked --doc
cargo test -p magpie-claims --locked --test standing_replay_snapshot
cargo test -p magpie-claims --locked --test origin_admission_replay_substrate
cargo test -p magpie-claims --locked --test origin_admission_replay_substrate
cargo test -p magpie-claims --locked --test origin_admission_replay_substrate
```

Complete validation:

```powershell
git diff --check
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo run --locked --example tour -p magpie-claims

python tools/verify_chain.py `
  crates/magpie-log/testdata/golden-v1.jsonl `
  ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c

python tools/verify_chain.py `
  fixtures/deadbolt-anchor-v1/anchor-log.jsonl `
  d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737
```

After the one commit on a clean tree:

```powershell
python tools/check_release_metadata.py
cargo package -p magpie-log --locked
```

Every reported pass must have actually run. Any generated change or required
out-of-allowlist metadata edit stops publication.

## 20. Worker and publication authority

`AGENTS.md` remains binding except that George explicitly authorizes, only
after every required validation and exact scope audit pass:

- staging only the ten allowlisted paths;
- creating exactly one commit with the exact title in section 1;
- pushing only the named branch without force; and
- opening exactly one draft PR with the exact title in section 1.

The worker may not modify `main`, mark ready, merge, enable auto-merge,
force-push, resolve or dismiss reviews, modify another PR, or create any
additional branch or commit.

## 21. Stop conditions

Stop with the smallest clear failing test and report the divergence if the
implementation would require:

- a second `read_records()` call;
- changed replay errors, validation order or early projection application;
- adding the tip to `StandingReplaySnapshot` or changing its canonical bytes;
- a public constructor for prefix identity or replay context;
- public mutable anchor-index access;
- caller-supplied candidate selectors or closure-derived enumeration;
- changing origin-binding verifier or closure code;
- changing standing or policy code;
- a dependency, L0 or Cargo change; or
- any edit outside the exact allowlist.

Before editing, also stop for a dirty tree, wrong repository/remote, missing or
unauthenticated `gh`, an unmerged PR #59, a non-ancestor reviewed head, a main
SHA other than the exact base, or an occupied equivalent branch/open PR.

## 22. Reviewer checklist

1. Does `replay_with_summary` perform one store read and return the tip from
   the same retained verified snapshot it applies?
2. Does legacy `replay` delegate without changing any success or error result?
3. Are empty-store, late-failure, genesis, payload and external-key behaviors
   unchanged?
4. Can either public identity/context be fabricated, deserialized or assembled
   from cross-replay pieces?
5. Do standing, anchors, count and tip share one verified replay origin?
6. Do the old and new standing snapshot paths remain equal and byte-identical?
7. Does equal projection state from different histories produce distinct exact
   prefix identities?
8. Does enumeration read only the actual same-replay anchor map and remain
   crate-private?
9. Is the candidate filter exactly two kinds plus exact `sha256` and
   `magpie-origin-binding-json-v0`, with no extra validation?
10. Is selector and candidate order exact five-field lexical order?
11. Do repeated anchors produce one candidate with every ordered occurrence
    and no amplification?
12. Are closure evaluation, authority trust, origin admission, duplicate and
    conflict handling, support, standing and aggregation absent?
13. Are the changed paths exactly the ten allowlisted paths and all frozen
    surfaces untouched?
