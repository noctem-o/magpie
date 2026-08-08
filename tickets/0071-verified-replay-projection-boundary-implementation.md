# Ticket 0071: Implement verified replay → projection boundary

## Status

Runtime/API remediation candidate for A-009 / RQ-002, implementing the merged
Ticket 0070 contract from PR #117.

This ticket does not edit the audit-disposition ledger and does not
administratively mark A-009 / RQ-002 Closed. The required sequence remains:

```text
runtime remediation candidate
-> independent hostile review
-> owner merge
-> later audit-disposition reconciliation
```

A-009 / RQ-002 remains **Confirmed** pending that sequence. Human owner review
and merge authority remain exclusive.

## Resolved base, branch, and worktree

```text
repository: noctem-o/magpie
worktree: C:\Users\herpe\.codex\worktrees\magpie-verified-replay-projection-boundary
branch: agent/verified-replay-projection-boundary
starting HEAD: 99512b5eddd39ecc98e31e59173252da717f2fa2
origin/main: 99512b5eddd39ecc98e31e59173252da717f2fa2
merge-base(HEAD, origin/main): 99512b5eddd39ecc98e31e59173252da717f2fa2
live refs/heads/main: 99512b5eddd39ecc98e31e59173252da717f2fa2
governing #117 merge: 99512b5eddd39ecc98e31e59173252da717f2fa2
reviewed #117 contract head: a7c4a7de6e5e051e1c34f148ea8422c5ba9b0fa6
audit target: A-009 / RQ-002
```

The isolated worktree was clean before editing. `HEAD == origin/main`, the
merge base was `origin/main`, #117 was an ancestor of live main, Ticket 0070
and the normative design were present, and Ticket 0071 was absent. Live main
had not moved beyond the expected #117 merge, so there were no intervening
commits to classify.

## Governing contract

- [`docs/design/verified-replay-projection-boundary-contract.md`](../docs/design/verified-replay-projection-boundary-contract.md)
- [`tickets/0070-verified-replay-projection-boundary-contract.md`](0070-verified-replay-projection-boundary-contract.md)
- PR #117, merged as
  `99512b5eddd39ecc98e31e59173252da717f2fa2`

## Current defect remediated

Before this slice, raw and verified paths both exposed `SignedEvent` to the
public `Projection::apply` boundary. A caller could parse, clone, deserialize,
construct, independently sign, or receive a writer-returned event and then
manually mutate any public projection with it.

The implemented topology is:

```text
raw SignedEvent
  -> serde / writer return / caller construction / unverified_events()
  -X-> Projection::apply

one LogStore::read_records snapshot
  -> parse the complete snapshot
  -> verify the complete exact snapshot
  -> retain Vec<SignedEvent> plus exact tip
  -> privately borrow each retained event as VerifiedReplayEvent<'_>
  -> Projection::apply(&VerifiedReplayEvent<'_>)
```

This is type-state/API correction only. It adds no admission, identity,
authorization, policy, trust-root, currentness, detached response, or new
epistemic mechanism.

## Public API delta

```rust
pub struct VerifiedReplayEvent<'event> {
    event: &'event SignedEvent, // private
}

impl<'event> VerifiedReplayEvent<'event> {
    pub fn event(&self) -> &'event SignedEvent;
}

pub trait Projection {
    fn apply(&mut self, event: &VerifiedReplayEvent<'_>);
}

impl<S: LogStore> LogReader<S> {
    pub fn unverified_events(&self) -> Result<Vec<SignedEvent>, LogError>;
}
```

`LogReader::events()` is removed with no deprecated alias. `SignedEvent` and
its serialization, cloning, public fields, writer return, canonical bytes,
hashes, signatures, and JSONL layout are unchanged.

### VerifiedReplayEvent privacy properties

The wrapper is public and observable through `event()`, so external crates can
implement `Projection`. Its field and constructor are private to `logimpl.rs`.
It has no public or `pub(crate)` constructor, and no `Default`, `Clone`, `Copy`,
`Serialize`, `Deserialize`, `Deref`, `AsRef`, builder, unchecked constructor,
boolean marker, unsafe helper, or event conversion implementation.

Constructor inventory after implementation:

```text
private constructor definitions: 1
post-verification replay-loop calls: 1
other production constructor calls: 0
```

The hostile struct-literal doctest mentions the private field but cannot mint
a value. Composite projections forward the same borrowed value and never call
the constructor.

## Changed paths

Production/API:

- `crates/magpie-log/src/logimpl.rs`
- `crates/magpie-log/src/lib.rs`
- `crates/magpie-claims/src/lib.rs`
- `crates/magpie-claims/src/standing.rs`
- `crates/magpie-claims/src/deadbolt_context.rs`
- `crates/magpie-claims/src/replay_snapshot.rs`
- `crates/magpie-episodic/src/lib.rs`

Tests/examples:

- `crates/magpie-log/tests/chain.rs`
- `crates/magpie-log/tests/deadbolt_anchor_fixture.rs`
- `crates/magpie-log/tests/golden.rs`
- `crates/magpie-log/examples/regen_golden.rs`
- `crates/magpie-claims/tests/standing_replay_snapshot.rs`
- `crates/magpie-episodic/tests/episodic.rs`

Implementation evidence:

- `tickets/0071-verified-replay-projection-boundary-implementation.md`

No extra living documentation path required synchronization. The source
rustdoc is the current public API description. Historical audits, design
contracts, tickets, bootstrap material, the tag-bound release contract, and
the audit ledger retain their historical wording.

## Projection implementation inventory

Mechanical inventory after migration still finds seven implementations:

Production public:

1. `ClaimsView`
2. `StandingView`
3. `DeadboltAnchorIndex`
4. `EpisodicView`

Private/composite:

5. `StandingContextProjection`

Test-only downstream implementations:

6. `RecordingProjection`
7. `AnchorReplay`

Every implementation accepts `&VerifiedReplayEvent<'_>`. The four production
views immediately inspect `replay_event.event()` and preserve their prior fold
semantics. `StandingContextProjection` forwards the exact same wrapper
reference to both child projections.

## Direct apply inventory

Surviving executable calls are exactly:

1. the `LogReader::replay_with_summary` post-complete-verification loop;
2. `StandingContextProjection` forwarding to `StandingView`;
3. `StandingContextProjection` forwarding to `DeadboltAnchorIndex`.

The remaining `.apply(` text in `logimpl.rs` consists of hostile
`compile_fail` examples. No executable `SignedEvent -> Projection::apply` call
remains. No public inherent raw fold/apply method exists on a built-in
projection.

## Raw inspection migration inventory

The twelve baseline Rust call expressions were disposed as follows:

- `crates/magpie-log/tests/golden.rs`: five renamed to
  `unverified_events()`;
- `crates/magpie-log/tests/deadbolt_anchor_fixture.rs`: two renamed;
- `crates/magpie-log/tests/chain.rs`: one renamed, plus a hostile raw
  diagnostic test;
- `crates/magpie-log/examples/regen_golden.rs`: three renamed;
- `crates/magpie-episodic/tests/episodic.rs`: the raw genesis call and manual
  writer-event application were removed and replaced by two independent
  genuine replays.

Historical `events()` documentation is intentionally unchanged. In
particular, `docs/releases/v0.1.0-contract.md` is tag-bound evidence for the
then-current v0.1.0 API, not living current-main API documentation.

## Negative proofs N1-N10

Permanent proofs are dependency-free rustdoc `compile_fail` cases plus exact
public API and repository scans. A temporary external probe crate reproduced
the compiler diagnostics and was removed before the scope audit.

| Proof | Permanent/probe location | Forbidden operation | Actual primary failure | Intended reason |
|---|---|---|---|---|
| N1 | `Projection` rustdoc / hostile probe | `&SignedEvent` passed to `apply` | E0308 | expected `&VerifiedReplayEvent`, found `&SignedEvent` |
| N2 | `unverified_events` rustdoc / hostile probe | parsed raw output passed to `apply` | E0308 | same exact wrapper/raw mismatch |
| N3 | `VerifiedReplayEvent` rustdoc / hostile probe | `serde_json` deserialization | E0277 | `Deserialize` is not implemented |
| N4 | `VerifiedReplayEvent` rustdoc / isolated hostile probe | downstream struct literal | E0451 | field `event` is private |
| N5 | `VerifiedReplayEvent` rustdoc / hostile probe | `&SignedEvent::into()` | E0277 | `From<&SignedEvent>` is not implemented |
| N6 | `verify_chain` rustdoc / hostile probe | use verified count to wrap raw event | E0599 | no `wrap` method on returned `u64` |
| N7 | `VerifiedReplaySummary` rustdoc / hostile probe | use genuine summary to wrap raw event | E0599 | no `wrap` method on summary |
| N8 | `Projection` rustdoc / hostile probe | writer-returned correctly signed event passed to `apply` | E0308 | signed raw event is still not replay input |
| N9 | all seven implementation signatures plus custom replay test | built-in/custom boundary divergence | common wrapper-only trait signature | no inherent raw built-in mutation surface found |
| N10 | repository public escape scan | raw/unchecked/apply adapter bypass | no match | no exported equivalent recreates raw projection mutation |

The N10 scan covered `apply_raw`, `apply_unverified`,
`apply_signed_event`, `apply_event`, `fold_raw`, `push_event`, `accept_event`,
`verified_event_unchecked`, `from_verified`, projection/replay adapters,
conversion/wrap helpers, and public inherent apply methods.

## Lifetime non-detachment proof

The permanent `VerifiedReplayEvent` doctest implements a projection with:

```rust,ignore
retained: Option<&'event VerifiedReplayEvent<'event>>
```

and attempts `self.retained = Some(event)` inside `apply`. The direct external
probe failed because both the callback-reference lifetime and the wrapper's
borrowed-event lifetime would have to outlive the projection's stored
lifetime. A projection may instead call `event.event().clone()`; that result is
only `SignedEvent` and cannot re-enter `apply`.

## Positive proofs P1-P12

| Proof | Evidence |
|---|---|
| P1 downstream custom projection | `chain.rs::downstream_custom_projection_observes_genuine_replay_inputs`; external `RecordingProjection` implements the public trait, calls `event()`, and is driven by real replay |
| P2 ClaimsView equivalence | `regenerable::projection_is_byte_identical_after_dropping_all_derived_state` and golden workspace tests pass unchanged |
| P3 StandingView equivalence | `standing_resolution`, `standing_replay_snapshot`, and full standing suites preserve governed results and canonical bytes |
| P4 Deadbolt occurrence equivalence | log and claims Deadbolt fixture suites preserve identity, sequence, hash, provenance, ordering, and canonical bytes |
| P5 composite fan-out | `standing_replay_snapshot_fans_out_one_verified_replay_to_both_views` plus the same-reference forwarding implementation and changing-store test |
| P6 EpisodicView equivalence | `projection_is_byte_identical_across_independent_verified_replays`, drop/rebuild, schema, row, and canonical-byte tests |
| P7 exactly one snapshot read | `replay_uses_one_verified_snapshot_when_store_changes_on_read`, summary read-count tests, and `standing_replay_snapshot_uses_one_changing_store_snapshot` |
| P8 late failure means zero application | dedicated late signature, content-hash, and previous-link replay/summary tests plus hostile parity and existing genesis/payload tests |
| P9 exact replay summary | `replay_with_summary_uses_one_snapshot_and_returns_exact_count_tip_and_sequence` and replay/summary equivalence tests |
| P10 raw diagnostics remain | `unverified_events_returns_parsed_raw_records_without_verifying_them` returns parsed values from a bad-signature record while `verify_chain` rejects it |
| P11 frozen histories unchanged | blob/SHA checks, golden/deadbolt fixture tests, and independent Python verifiers |
| P12 external key trust explicit | `VerifiedReplayEvent`, replay, `DeadboltAnchorIndex`, and standing snapshot rustdoc preserve the external-key-trust/non-authority distinction |

## Replay ordering and summary evidence

`replay_with_summary` still performs exactly one `read_records()`, passes that
exact byte snapshot once to `parse_and_verify_records(..., RetainEvents)`, and
creates no wrapper until that function has returned the completely verified
`VerifiedSnapshot`. A late parse, sequence, link, hash, signature, genesis,
profile, key-binding, or payload failure therefore returns before the replay
loop and applies zero callbacks. The summary count and tip are derived from the
same retained snapshot whose events are applied.

Changing-store tests report one read. Dedicated late bad-signature,
content-hash, and previous-link tests report zero projection payloads and no
summary. The exact-summary test reports the expected three events, exact final
hash, and identical application sequence.

## Canonical and historical preservation

Pre-edit identities:

| Path | Git blob | SHA-256 |
|---|---|---|
| `docs/FORMAT.md` | `e957b5dbec950be83b00c9aa6beb605ae518465c` | `a777875cc745ddcefc3891701ce930ddda06c5fa8b4eb046198d41116a3184ec` |
| `docs/releases/v0.1.0-contract.md` | `dfd8e9312cec0c4b3f4827e70f142e7e5e4ac176` | `7b810a6e10d0d8c165dd466f8cec4f05d49d73bb0ff7acd1f006b7647f9a2f95` |
| `crates/magpie-log/testdata/golden-v1.jsonl` | `c764eab3e41b357714c1b441522e9bd5b98c9f04` | `767ef86b7e2ad1d65e8315ae28bac682bceee29223b5f1a0612ec7ef062cffb4` |
| `fixtures/deadbolt-anchor-v1/anchor-log.jsonl` | `5809743f816fe5e26b3f1aa6bd476267791f10f2` | `2c715006e3e5bef571b3d31b95f47d3c4bbb3ea68fa9750365b4a62b66cd41e5` |

The v0.1.0 tag commit is
`2bbfbd1f451e35b65e8c89aeaf39801621e484df`. Its release-contract blob and
current-main release-contract blob are both
`dfd8e9312cec0c4b3f4827e70f142e7e5e4ac176`. The complete pre-edit audits tree
is `409675c93e6a0ed173d703fda9bbb64c4a919e19`.

Post-edit/final verification must reproduce these identities. The historical
v0.1.0 `LogReader::events()` wording is deliberately preserved because it
describes the tagged release.

Independent verification results:

```text
golden-v1.jsonl: 9/9 OK
deadbolt anchor-log.jsonl: 2/2 OK
```

Neither fixture was regenerated.

## Public rustdoc/API inspection

Generated `magpie-log` rustdoc shows:

- public `VerifiedReplayEvent<'event>` with `/* private fields */`;
- public `event()` observer;
- no visible `from_verified_snapshot` constructor;
- no `Clone`, `Serialize`, or `Deserialize` implementation;
- `Projection::apply(&VerifiedReplayEvent<'_>)`;
- public `LogReader::unverified_events()`;
- no `LogReader::events()` method anchor.

The generated type page exposes only `event()` as an inherent method. Blanket
identity conversions do not provide a `SignedEvent` conversion.

## Adjacent findings explicitly unchanged

This slice does not remediate, narrow, or close:

- A-003;
- A-007 / RQ-009 (projection fallibility/storage failures);
- A-019 / RQ-011 (detached response provenance);
- A-021 (trust in the supplied verification key);
- A-022;
- A-023;
- A-024 (source independence/origin groups).

No admission, identity, authorization, currentness, detached provenance,
standing-v3/v4 policy, origin-group, or source-independence semantics were
added. The audit-disposition ledger remains unchanged.

## Validation results

All Rust build commands below were run in the Visual Studio 2022 x64 developer
environment. A first ordinary `cargo check` and a probe using the incomplete
Visual Studio 18 environment failed before compiling Magpie with
`LNK1104: cannot open file 'msvcrt.lib'`; the correctly configured VS 2022
environment then passed.

Passed:

```text
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test -p magpie-log --locked
cargo test --doc -p magpie-log --locked
cargo test -p magpie-claims --locked --test regenerable
cargo test -p magpie-claims --locked --test standing_resolution
cargo test -p magpie-claims --locked --test standing_replay_snapshot
cargo test -p magpie-claims --locked --test deadbolt_anchor_fixture
cargo test -p magpie-claims --locked --test deadbolt_occurrence_context
cargo test -p magpie-claims --locked --test origin_admission_replay_substrate
cargo test -p magpie-episodic --locked --test episodic
cargo test --workspace --locked --no-fail-fast
cargo run --locked --example tour -p magpie-claims
cargo doc -p magpie-log --locked --no-deps
python tools/verify_chain.py crates/magpie-log/testdata/golden-v1.jsonl ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c
python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737
cargo package -p magpie-log --locked --allow-dirty
python tools/check_release_metadata.py
cargo package -p magpie-log --locked
```

`cargo test -p magpie-log --locked` passed all unit, integration, fixture,
golden, and 19 doctests. The focused suites and full workspace suite completed
with zero failures. On the clean committed tree, release metadata reported
consistent inventories for 19 `magpie-log`, 48 `magpie-claims`, and 9
`magpie-episodic` files; the normal `magpie-log` package/verification check
passed. No warning suppression was added.

The first release-metadata invocation on the dirty implementation tree was
expectedly rejected because `cargo package --list --locked` requires committed
inputs. It is not a product failure; the required clean-tree rerun passed.

## Known limitations

- Trust in the caller-supplied `VerifyingKey` remains external (A-021).
- The wrapper is an ephemeral callback input, not a portable proof or detached
  provenance coordinate.
- Projection correctness, admission, actor authority, truth, standing policy,
  and storage fallibility remain separate concerns.
- The intentional pre-1.0 source breaks require downstream callers to rename
  raw inspection and update custom projection signatures.

## Publication state and handoff

A coherent implementation commit exists locally. Publication remains pending
push of `agent/verified-replay-projection-boundary` and one draft PR. The PR
must remain draft with no auto-merge and must not be merged or marked ready by
this implementation agent.

Independent hostile review should attack safe wrapper construction, private
constructor leakage, serde/conversion minting, summary or `verify_chain`
blessing, raw event application, inherent raw projection methods, alternate
adapters, lifetime detachment, custom projection extensibility, composite
fan-out, second-read snapshot mixing, late-failure partial mutation, canonical
drift, historical release-contract mutation, external-key authority inflation,
and adjacent-scope leakage.
