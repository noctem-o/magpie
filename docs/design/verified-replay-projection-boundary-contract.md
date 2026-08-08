# Verified Replay → Projection Boundary Contract

## Status

Narrow documentation-only implementation contract for A-009 / RQ-002.

Resolved baseline:

```text
repository: noctem-o/magpie
origin/main: c2fe991d8023d237f7d589bd3f022233c1f332cc
branch: agent/verified-replay-projection-boundary-contract
```

This document freezes the smallest future Rust/API correction. It changes no
runtime, test, canonical format, fixture, ADR, or audit disposition. A-009 /
RQ-002 remains **Confirmed** until a separate implementation is merged,
independently reviewed, and later reconciled in the living audit ledger.

No new ADR is required. Accepted replay doctrine already requires complete
verification of one retained record snapshot before projection application.
This contract makes that existing doctrine structural at the public type
boundary; it does not change what verification or trust means.

## Purpose

Make this invariant true in ordinary downstream safe Rust:

> Only an event being presented from one completely verified, retained replay
> snapshot may enter the public `Projection` application boundary.

The correction must preserve all three of these legitimate capabilities:

- `SignedEvent` remains public raw/canonical historical material;
- callers retain an explicit raw inspection API; and
- downstream crates may continue to implement custom public projections.

It must not make `SignedEvent` trusted, hide hostile history, create a portable
verified-event credential, or invent a general authority system.

## Audited defect

At the resolved baseline, `SignedEvent` is public, cloneable, serializable,
deserializable, and constructible by public struct literal. That is correct for
a stored L0 record. The defect is that the same type is also the public input
to `Projection::apply`:

```rust
pub fn events(&self) -> Result<Vec<SignedEvent>, LogError>;

pub trait Projection {
    fn apply(&mut self, event: &SignedEvent);
}
```

`LogReader::events` explicitly does not verify sequence, predecessor links,
content hashes, signatures, genesis binding, or payload validity. Nevertheless
an ordinary caller can currently perform:

```text
LogReader::events
-> Vec<SignedEvent>                 parsed, explicitly unverified
-> Projection::apply
-> authority-looking derived state
```

The same bypass also begins with a deserialized, hand-constructed, cloned,
writer-returned, or correctly hand-signed `SignedEvent`. A warning contains the
hazard but does not remove the type compatibility. This is the exact A-009 /
RQ-002 defect.

## Current public capability/type graph

The current graph has two origins converging on one input type:

```text
RAW OR CALLER-CONTROLLED

LogReader::events ---------------------+
serde_json / struct literal -----------+--> SignedEvent
LogWriter::append return / clone -------+        |
correctly hand-signed isolated record --+        v
                                           Projection::apply

VERIFIED REPLAY

LogStore::read_records (one snapshot)
-> parse and retain SignedEvent vector
-> verify complete vector
-> same retained SignedEvent references
-> Projection::apply
```

Only the second origin has the required provenance, but the public trait cannot
distinguish them.

### Complete projection inventory

A repository-wide Rust scan found exactly seven implementations:

| Implementation | Path | Classification | Current role |
| --- | --- | --- | --- |
| `ClaimsView` | `crates/magpie-claims/src/lib.rs` | production public projection | legacy compatibility claim view |
| `StandingView` | `crates/magpie-claims/src/standing.rs` | production public projection | typed standing facts and policy substrate |
| `DeadboltAnchorIndex` | `crates/magpie-claims/src/deadbolt_context.rs` | production public projection | ordered exact anchor occurrences |
| `EpisodicView` | `crates/magpie-episodic/src/lib.rs` | production public projection | rebuildable SQLite/FTS derived view |
| `StandingContextProjection` | `crates/magpie-claims/src/replay_snapshot.rs` | private/composite projection | forwards one replay input to standing and anchor views |
| `RecordingProjection` | `crates/magpie-log/tests/chain.rs` | test-only projection | records payloads for replay ordering/failure tests |
| `AnchorReplay` | `crates/magpie-log/tests/deadbolt_anchor_fixture.rs` | test-only projection | inspects the portable anchor fixture through replay |

There is no example/demo-only `Projection` implementation.

### Complete direct application inventory

The same scan found five executable `.apply(...)` calls:

| Call | Classification | Meaning |
| --- | --- | --- |
| `logimpl.rs`: replay loop | verified replay path | applies retained events only after complete verification |
| `replay_snapshot.rs`: standing forward | composite forwarding | forwards the same replay input to `StandingView` |
| `replay_snapshot.rs`: anchor forward | composite forwarding | forwards the same replay input to `DeadboltAnchorIndex` |
| `episodic.rs`: genesis live apply | unit-test mechanics and raw/manual public use | directly applies output from raw `events()` |
| `episodic.rs`: appended-event live apply | unit-test mechanics and raw/manual public use | directly applies writer-returned `SignedEvent` values |

No other raw/manual, production, example, or composite application call was
found. The episodic live-versus-replay test is a direct executable example of
the compatibility that this contract removes.

### Other mechanically inventoried surfaces

- Raw `events()` has twelve Rust call expressions: five in `golden.rs`, two in
  `deadbolt_anchor_fixture.rs`, one in `chain.rs`, three in
  `regen_golden.rs`, and one in the episodic live-apply test. They are fixture,
  diagnostic, format-generation, or test-mechanics uses; no production crate
  calls it.
- `verify_chain()` is used for validation and writer/release demonstrations.
  Its count result is not passed back into replay and exposes no event witness.
- `replay()` drives all four production projection families, the two test-only
  projections, and standing tests. `replay_with_summary()` drives the same
  verified loop and the private standing/origin replay contexts.
- `VerifiedReplaySummary` is constructed only after successful replay, is
  converted into detached prefix audit identity in `magpie-claims`, and is
  never accepted as replay input.
- `serde_json::from_slice::<SignedEvent>` appears in the raw inspection
  implementation. The verifier and hostile tests also deserialize
  `SignedEvent` through inferred type annotations. All such values remain raw.

## Governing invariant

The type system must preserve this distinction:

```text
SignedEvent
= raw/canonical historical record value

SignedEvent
!= evidence that the event came from verified replay
!= trusted event
!= accepted claim
!= authority
```

And:

```text
Projection application input
= an ephemeral value obtainable by ordinary downstream safe Rust
  only while LogReader presents an event from a retained snapshot whose
  complete verification phase has already succeeded
```

A public authority-looking projection must not accept a caller-provided or
merely parsed `SignedEvent` as though the value's existence established chain
provenance.

## Raw SignedEvent semantics

`SignedEvent` remains the public L0 storage record with its current public
fields and current `Clone`, `Serialize`, and `Deserialize` behavior. It may be:

- parsed, inspected, serialized, deserialized, and cloned;
- returned by `LogWriter::append`;
- constructed for hostile fixtures and independent tooling;
- used to verify format, hashes, and signatures independently; and
- carried as portable raw historical material.

Those operations establish no verified-replay type state. Even a correctly
signed isolated event is not projection-compatible because complete replay
also verifies the retained snapshot's order, links, hashes, genesis binding,
payload validity, and every other event.

Making `SignedEvent`, `EventCore`, `Sig`, or `ContentHash` private is forbidden.
Raw record carriage and verified replay application are separate concepts.

## Complete replay semantics

The current replay ordering is normative and remains unchanged:

```text
1. call read_records() exactly once
2. retain that exact record vector
3. parse that exact vector into SignedEvent values
4. verify the complete vector under this LogReader's supplied VerifyingKey
5. retain the exact parsed event vector that was verified
6. only after complete success, present those same retained events to Projection
7. return the exact count and tip from that same snapshot
```

No event may be applied while verification is still in progress. A parse,
sequence, link, hash, signature, genesis, profile, key-binding, or payload
failure at any position means:

```text
zero Projection::apply calls
zero replay summary
```

Replay must not stream application, reparse after verification, reread the
store, or call `verify_chain()` and then obtain another snapshot.
`verify_chain()` remains summary-only validation and does not produce or mint
projection input.

## Verified replay input semantics

The selected new public type is:

```rust
pub struct VerifiedReplayEvent<'event> {
    event: &'event SignedEvent,
}

impl<'event> VerifiedReplayEvent<'event> {
    fn from_verified_snapshot(event: &'event SignedEvent) -> Self {
        Self { event }
    }

    pub fn event(&self) -> &'event SignedEvent {
        self.event
    }
}

pub trait Projection {
    fn apply(&mut self, event: &VerifiedReplayEvent<'_>);
}
```

The private constructor name is illustrative; its privacy and sole call site
are normative. It must be private inside `magpie-log`'s replay implementation,
not `pub`, `pub(crate)`, or exposed through another factory. The only call site
must be the post-verification loop over `VerifiedSnapshot.events`.

`VerifiedReplayEvent` is the selected name because it says exactly where the
type state came from. Here “Verified” means only “currently presented after
complete Magpie replay verification of one retained snapshot under this
reader's supplied key.” Names such as `TrustedEvent`, `AuthoritativeEvent`,
`AcceptedEvent`, or `ValidEvent` overstate that meaning.

The wrapper is borrowed and lifetime-bound to one retained raw event. The
trait receives a shared reference to the wrapper. No raw event is cloned for
application and the wrapper cannot become a freely owned portable credential.

## Construction/privacy law

Ordinary downstream safe Rust must be able to name and observe
`VerifiedReplayEvent`, because it appears in a public implementable trait, but
must not be able to mint it.

The implementation must provide all of these properties:

- the `event` field is private;
- the replay-only constructor is private to the implementation module;
- no public or crate-public constructor, builder, factory, unchecked
  constructor, Boolean flag, or marker field exists;
- the type implements neither `Default`, `Serialize`, nor `Deserialize`;
- the minimal implementation also omits `Clone` and `Copy`; shared borrowing
  supplies all legitimate fan-out;
- no `From`, `TryFrom`, `Deref`, `AsRef`, or `Into` path converts a raw event
  into the wrapper or visually erases the wrapper at the trait boundary;
- `verify_chain()`, `VerifiedReplaySummary`, counts, tips, and copied audit
  identities cannot construct it; and
- no unsafe construction seam is added.

The only public observation method is `event() -> &SignedEvent`. A projection
may inspect every raw historical field and may clone the underlying
`SignedEvent`. Such a clone is raw material: it loses the wrapper type state and
cannot re-enter `Projection::apply`.

Rust auto traits inferred from the borrowed field are not authority surfaces.
Ordinary unsafe memory fabrication is outside the safe-Rust public API
guarantee and must not be legitimized by any Magpie helper.

## Projection extension compatibility

`Projection` remains public, unsealed, and downstream-implementable. This is
an intentional extension surface: external crates may define rebuildable
views and have `LogReader::replay` drive them.

Every implementation changes mechanically from reading `&SignedEvent` to
reading `event.event()` from `&VerifiedReplayEvent<'_>`. The trait remains
infallible; A-007 / RQ-009 owns any future projection error semantics.

The supported law is:

```text
downstream code may implement Projection
downstream code may receive a genuine witness during replay
downstream code may forward that same witness during its callback
downstream code cannot fabricate the witness from raw history
```

The public method remains callable when a caller already holds a genuine
borrowed replay input. That permits composition; it does not create a raw
bypass.

This boundary attests only input provenance. It does not attest the correctness
of an arbitrary `Projection` implementation, require a custom composite to
forward exactly once, or make arbitrary derived state authoritative. The
top-level `LogReader` replay loop remains responsible for presenting each
retained event once in verified sequence order; a custom implementation owns
its own fold semantics.

## Composite projection law

The shared-reference signature deliberately supports fan-out:

```rust
impl Projection for StandingContextProjection {
    fn apply(&mut self, event: &VerifiedReplayEvent<'_>) {
        self.standing.apply(event);
        self.anchors.apply(event);
    }
}
```

Both children receive the exact same witness from the same replay invocation.
The composite does not construct a wrapper, unwrap and re-bless a raw event,
verify again, read again, use global state, or require unsafe code. Passing the
wrapper by value would require `Copy`, cloning, or a public reconstruction seam
for fan-out and is therefore rejected in favor of the shared reference.

## Raw inspection API decision

Choose Option B:

```rust
pub fn unverified_events(&self) -> Result<Vec<SignedEvent>, LogError>;
```

Remove `events()` rather than retain a deprecated alias. The output remains
raw `SignedEvent`; only the name changes. Its documentation must continue to
state that its own read is parsed but not verified for chain linkage, hashes,
signatures, genesis binding, or payload validity.

Evidence for the deliberate pre-1.0 source break:

- `events()` and `Projection` arrived as initial scaffold shape; history shows
  no later compatibility contract for the exact method name;
- all twelve repository callers are tests, fixture inspection, or the golden
  generator; no production crate calls the method;
- the release contract explicitly says public Rust source compatibility is not
  promised before 1.0; and
- keeping both names would preserve ambiguous surface without preserving a
  distinct capability.

Retaining `events()` would still be structurally safe after the trait change,
but would preserve unnecessary ambiguity. A deprecated alias would duplicate
the raw API indefinitely. The useful compatibility promise is raw inspection,
not the accidental name.

## Snapshot/failure ordering

Wrapper construction must occur inside the existing replay loop only after
`parse_and_verify_records(..., RetainEvents)` returns the complete
`VerifiedSnapshot`:

```text
read one record snapshot
-> parse and verify complete snapshot
-> retain exact parsed vector and tip
-> for each retained raw event:
     create short-lived VerifiedReplayEvent
     call Projection::apply(&witness)
-> return exact VerifiedReplaySummary
```

The witness must never be constructed in the verification loop. Therefore a
late bad record still produces zero wrappers and zero applications. A changing
store cannot mix snapshots because replay performs no second read.

## Trust-root boundary

`VerifiedReplayEvent` is not:

- a capability token, authorization token, credential, or trust root;
- an admission or human approval decision;
- evidence by itself, standing, currentness, truth, or confidence;
- authenticated actor identity; or
- a portable proof that can bless another record.

It records one narrow type-state fact: Magpie is presenting this retained raw
event after the complete retained snapshot passed the current replay verifier
under the `VerifyingKey` supplied to this `LogReader`.

Trust in that supplied key remains external. Verification under a
caller-selected key does not make the chain socially, organizationally, or
epistemically authoritative. A-021 remains independent.

## Canonical compatibility

This is an API/type-state correction around replay. It requires zero change to:

```text
Payload
EventCore
SignedEvent persisted shape
Sig
ContentHash
canonical core bytes
hash preimage
signature message
JSONL record representation
genesis
docs/FORMAT.md
golden fixtures
Deadbolt-anchor fixtures
```

No format/profile bump, payload tag, fixture regeneration, or canonical-byte
change is permitted. `event.rs` should need no storage-semantic change.

## Rejected alternatives

| Alternative | Decision | Reason |
| --- | --- | --- |
| Documentation warning only | Reject | raw output remains type-compatible with `Projection::apply` |
| Rename `events()` only | Reject as incomplete | clearer naming does not change the trait input type |
| Retain `events()` unchanged | Reject for selected implementation | structurally safe with the wrapper, but keeps avoidable ambiguity |
| Deprecated `events()` alias | Reject | duplicates a provisional pre-1.0 raw API without a compatibility promise |
| Seal or privatize `Projection` | Reject | destroys useful downstream rebuildable-view extensibility |
| `verified: bool` in `SignedEvent` | Reject | caller-controlled and conflates record data with replay provenance |
| Public verified-event constructor | Reject | recreates the defect under a new type name |
| Owned/serializable `VerifiedEvent` | Reject | detaches verification from one replay invocation and implies portable trust |
| Make `SignedEvent` private | Reject | destroys legitimate format, fixture, tooling, and inspection use |
| `VerifiedReplaySummary::wrap(event)` | Reject | copied summary cannot bind a caller-selected event to its snapshot |
| Global registry, authority service, or token framework | Reject | far beyond the local type-state defect |
| Second projection trait, callback adapter, or mode | Reject | unnecessary public concepts and migration surface |

## Compatibility/source-break analysis

Magpie is pre-1.0. The implementation must describe these source changes
honestly rather than add escape hatches solely to avoid migration:

| Surface | Intentional compatibility | Future effect |
| --- | --- | --- |
| public raw `SignedEvent` | preserve | storage shape, construction, serde, cloning, and inspection remain |
| public custom `Projection` | preserve | downstream impls change one method parameter and remain replay-driven |
| raw inspection capability | preserve | rename to `unverified_events()`, still returns `Vec<SignedEvent>` |
| exact `events()` name | not an intentional promise | remove as an explicit pre-1.0 break |
| manual raw `Projection::apply` | accidental generic permissiveness | no longer type-checks |
| `ClaimsView` | preserve behavior/bytes | implementation reads through `event()` |
| `StandingView` | preserve behavior/bytes | implementation reads through `event()` |
| `DeadboltAnchorIndex` | preserve behavior/bytes | implementation reads through `event()` |
| `EpisodicView` | preserve behavior/derived bytes | implementation reads through `event()` |
| private composite projection | preserve | forwards the same shared wrapper to both children |
| examples and integration tests | migrate | raw inspection calls rename; manual raw apply test must use replay/private mechanics |
| direct trait callers | intentionally break | raw calls fail; genuine same-callback fan-out remains |

Tests must adapt to the boundary. Legitimate migrations are real
`LogReader::replay`, a private pure fold helper tested inside its owning crate,
or a valid test log replayed into the projection. No public
`apply_unverified`, test constructor, or production escape hatch may be added.

## Implementation proof obligations

### Negative proofs

The runtime PR must land permanent compile-fail or equivalent structural
proofs that fail at the intended boundary:

| ID | Forbidden operation | Expected failure |
| --- | --- | --- |
| N1 | raw `SignedEvent` passed to `Projection::apply` | type mismatch: wrapper reference required |
| N2 | output from `unverified_events()` applied directly | same type mismatch at `apply` |
| N3 | deserialize `VerifiedReplayEvent` | missing `Deserialize` implementation |
| N4 | construct wrapper with struct literal | private field/constructor visibility failure |
| N5 | convert `SignedEvent` or `&SignedEvent` into wrapper | no `From`, `TryFrom`, or equivalent conversion |
| N6 | use successful `verify_chain()` result to wrap an event | no event-producing or wrapper factory API |
| N7 | use copied `VerifiedReplaySummary` to mint/bless an event | no summary input or wrapping method |
| N8 | pass a correctly hand-signed isolated `SignedEvent` | still the N1 type mismatch |
| N9 | manually feed raw input to any built-in or custom projection | every implementation shares the wrapper-only trait boundary |
| N10 | use `apply_raw`, `apply_unverified`, `apply_signed_event`, unchecked constructor, or adapter | public API/source scan finds no alternate escape |

Compile-fail cases must import the current public trait, use otherwise valid
constructors and syntax, and fail for the stated reason—not a stale signature,
missing import, ownership mistake, or unrelated error.

An additional lifetime compile-fail proof must show that an implementation
cannot retain `&VerifiedReplayEvent` beyond the `apply` callback. Retaining a
clone of `event.event()` remains allowed, but the retained value is only raw
`SignedEvent`.

### Positive proofs

| ID | Required behavior |
| --- | --- |
| P1 | A downstream crate implements public `Projection` and is driven by `LogReader::replay`. |
| P2 | Valid history rebuilds byte-identical `ClaimsView` output. |
| P3 | Valid history rebuilds identical `StandingView` behavior and bytes. |
| P4 | Verified replay builds identical Deadbolt anchor occurrence context. |
| P5 | A private composite forwards the same wrapper to multiple children with one read and one verification. |
| P6 | Valid replay rebuilds `EpisodicView` with unchanged canonical derived bytes. |
| P7 | Instrumentation proves replay reads exactly one record snapshot. |
| P8 | A valid prefix plus bad signature, hash, or link causes zero projection applications. |
| P9 | `replay_with_summary` returns the exact count and tip of the applied retained snapshot. |
| P10 | `unverified_events()` retains explicit parsed raw inspection for diagnostics and tooling. |
| P11 | Golden and Deadbolt-anchor chains remain byte-for-byte unchanged and independently verify. |
| P12 | Tests/docs continue to state that trust in the supplied verifying key is external. |

## Hostile scenarios

1. **Parsed raw history.** `unverified_events()` returns `SignedEvent`; passing
   one to `StandingView::apply` fails to type-check.
2. **Fabricated record.** Public `EventCore`, hash, signature, and
   `SignedEvent` construction still cannot produce the wrapper.
3. **Correctly signed isolated event.** Cryptographic validity alone does not
   satisfy complete-snapshot provenance; raw apply fails.
4. **Copied summary.** A legitimate `VerifiedReplaySummary` has no method or
   input position that blesses another record set.
5. **Late invalid record.** No wrapper is constructed and no projection is
   called before the final verification error returns.
6. **Changing store.** Replay applies only the first retained, completely
   verified snapshot and never performs a second read.
7. **Custom downstream projection.** Its public trait impl compiles and replay
   drives it; a raw `SignedEvent` cannot drive it manually.
8. **Composite projection.** The composite forwards the same shared wrapper to
   two children without construction, reread, reverification, unsafe, or
   global state.
9. **Raw diagnostics.** Malformed or unverified history remains inspectable as
   raw `SignedEvent` values where parsing itself succeeds.
10. **Serialization.** Only raw historical material survives serde; the replay
    wrapper is not serializable authority and is not deserializable.

## Non-goals

This contract does not:

- make projections intrinsically authoritative or solve detached-result
  provenance;
- change projection fallibility or storage error handling;
- change verification algorithms, trust-root selection, or key strength;
- authorize writers, humans, agents, roles, admissions, or claims;
- add policy, query, closure, MCP, librarian, or response coordinates;
- add a general capability, registry, mode, adapter, callback, or second
  projection abstraction; or
- close the audit finding administratively.

The public concept budget is exactly one new concept:
`VerifiedReplayEvent<'event>`. No token, credential, snapshot object, registry,
policy object, or authorization framework is required.

## Adjacent findings kept separate

- A-007 / RQ-009 owns fallible projection/storage semantics; `apply` remains
  infallible here.
- A-003, A-019 / RQ-011, and A-023 own exact coordinates for detached derived
  results and responses; the ephemeral wrapper is not that package.
- A-021 owns external trust in and strictness of the supplied verification
  root.
- A-022 owns admission and `EpistemicGate`; no admission concept appears here.
- A-024 owns independently authority-bound origin corroboration.

This contract also does not implement A-007, A-019, A-021, or A-022 under a
different name.

## Implementation handoff

The expected future runtime concentration is:

- `crates/magpie-log/src/logimpl.rs`: define the wrapper, keep its constructor
  private, rename raw inspection, change the trait input, and mint witnesses
  only in the post-verification loop;
- `crates/magpie-log/src/lib.rs`: re-export and document the one new public
  type and the structural boundary;
- `crates/magpie-claims/src/lib.rs`, `standing.rs`,
  `deadbolt_context.rs`, and `replay_snapshot.rs`: observe or forward the
  wrapper without changing fold behavior;
- `crates/magpie-episodic/src/lib.rs`: observe the wrapper without changing
  derived rows or bytes; and
- affected tests/examples: update trait signatures, raw inspection naming,
  compile-fail proofs, replay positives, and the episodic manual-apply test.

`crates/magpie-log/src/event.rs` requires no format-semantic change. Any need
to change persisted `SignedEvent` representation is a major red flag and must
stop the implementation for owner review.

The implementation PR must not edit the audit disposition ledger. A later
administrative slice may reconsider A-009 / RQ-002 only after runtime evidence,
independent hostile review, and owner merge.

## Reviewer checklist

- [ ] Raw `SignedEvent` remains a legitimate public historical value.
- [ ] Raw `SignedEvent` proves no verification and cannot enter `apply`.
- [ ] Only the post-verification retained-snapshot loop constructs the wrapper.
- [ ] Ordinary downstream safe Rust cannot construct or deserialize it.
- [ ] The wrapper is borrowed, non-portable, and exposes only `event()`.
- [ ] Public custom projections remain supported and unsealed.
- [ ] Shared-reference fan-out works for the private composite.
- [ ] Replay remains one-read and all-or-nothing on late failure.
- [ ] Raw inspection remains available under the explicit new name.
- [ ] External trust in the supplied key remains external.
- [ ] No canonical bytes, fixtures, or stored `SignedEvent` shape change.
- [ ] A-007 fallibility, detached provenance, admission, and identity work stay
      out of scope.
- [ ] Exactly one necessary public conceptual type is added.
- [ ] N1–N10 and P1–P12 can be implemented without another architecture
      decision.

## Target law

```text
RAW HISTORY
SignedEvent
  may be parsed, inspected, serialized, deserialized and cloned
  but confers no verified-replay status.

VERIFICATION
LogReader
  reads one record snapshot,
  parses and verifies that complete snapshot under its supplied verifying key,
  and retains the exact parsed events from that snapshot.

PROJECTION
Only magpie-log's post-verification replay loop may create the ephemeral
VerifiedReplayEvent presented to Projection::apply.

Therefore:
raw SignedEvent
  -X-> Projection::apply
LogReader::unverified_events output
  -X-> Projection::apply
successful complete snapshot verification inside LogReader replay
  --> ephemeral VerifiedReplayEvent
  --> Projection::apply

failure anywhere before complete snapshot verification
  -> zero VerifiedReplayEvent values
  -> zero Projection::apply calls
```
