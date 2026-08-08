# Ticket 0070: Verified replay → projection boundary contract

## Status

Documentation-only implementation-contract and handoff slice for A-009 /
RQ-002.

This PR changes no Rust, tests, fixtures, FORMAT, audit ledger, ADR, dependency,
manifest, policy, replay runtime, projection runtime, or trust-root semantics.
Human owner review is required. A separate runtime PR must satisfy the
normative design before the finding can be reconsidered.

Resolved base:

```text
repository: noctem-o/magpie
origin/main: c2fe991d8023d237f7d589bd3f022233c1f332cc
baseline merge: Merge pull request #116 from
  noctem-o/docs/reconcile-audit-disposition-post-115
branch: agent/verified-replay-projection-boundary-contract
audit target: A-009 / RQ-002
```

At preflight, isolated-worktree `HEAD`, `origin/main`, their merge base, and
live `refs/heads/main` were exactly the recorded SHA and the worktree was
clean. Ticket 0070 and both allowed output paths were absent.

Normative design:

[`docs/design/verified-replay-projection-boundary-contract.md`](../docs/design/verified-replay-projection-boundary-contract.md)

Proposed commit and draft PR title:

```text
docs: define verified replay projection boundary
```

## Audit target

```text
A-009 / RQ-002
Unverified parsed events and verified replay inputs share the same public
projection-compatible type.
```

The living disposition ledger records the finding as **Confirmed** and P0.
This contract is not runtime evidence. The ledger must remain unchanged in
this PR and in the later implementation PR.

## Historical defect

Current public API:

```rust
pub fn LogReader::events(&self) -> Result<Vec<SignedEvent>, LogError>;

pub trait Projection {
    fn apply(&mut self, event: &SignedEvent);
}
```

`events()` documents that it parses its own store snapshot without verifying
chain links, hashes, signatures, genesis binding, or payload validity. Yet its
output, a deserialized raw record, a public struct literal, a clone, a
writer-returned event, or a hand-signed isolated event may all be passed to the
public trait method.

```text
raw/caller-selected SignedEvent
-> public Projection::apply
-> authority-looking derived state
```

The verified replay path uses the same type after stronger work:

```text
one read_records snapshot
-> parse and retain exact events
-> verify complete snapshot
-> only after success apply the same retained events
```

The current implementation correctly preserves one-read, complete-
verification-before-application behavior. The defect is the public input type,
not replay ordering.

## Current code evidence

At the resolved base:

- `crates/magpie-log/src/event.rs:328-335` defines public, serde-capable,
  public-field `SignedEvent` as the stored record;
- `crates/magpie-log/src/logimpl.rs:394-405` exposes explicitly unverified
  `events() -> Vec<SignedEvent>`;
- `logimpl.rs:420-450` reads once, obtains a completely verified retained
  snapshot, applies it, and returns exact count/tip;
- `logimpl.rs:466-469` makes raw `&SignedEvent` the public trait input;
- `crates/magpie-claims/src/deadbolt_context.rs:41-56` documents the manual/raw
  construction hazard but cannot prevent it; and
- the architecture audit, runtime-quality audit, and living ledger all identify
  the same type-compatibility defect.

`VerifiedReplaySummary` is useful precedent: private fields, no public
constructor, no `Deserialize`, successful-replay output only, and no input
position that authorizes another replay. It must not become an event-minting
token.

## Repository-wide surface inventory

Mechanical scans covered every Rust occurrence of `SignedEvent`, `Projection`,
`Projection::apply`, `impl Projection`, `.apply(`, the four named
`LogReader` methods, `VerifiedReplaySummary`, and typed `SignedEvent` serde
parses.

Exactly seven `Projection` implementations exist:

| Implementation | Classification |
| --- | --- |
| `ClaimsView` | production public projection |
| `StandingView` | production public projection |
| `DeadboltAnchorIndex` | production public projection |
| `EpisodicView` | production public projection |
| `StandingContextProjection` | private/composite projection |
| `RecordingProjection` | test-only projection |
| `AnchorReplay` | test-only projection |

There is no example/demo projection implementation.

Exactly five executable direct applications exist:

| Family | Calls | Classification |
| --- | ---: | --- |
| `LogReader::replay_with_summary` loop | 1 | verified replay path |
| `StandingContextProjection` child forwarding | 2 | composite forwarding |
| episodic live-versus-replay test | 2 | unit mechanics plus raw/manual public bypass |

The episodic test directly applies one raw `events()` result and each
writer-returned `SignedEvent`. No other manual application call exists.

Raw `events()` has twelve Rust call expressions, all in five test/generator
files: `golden.rs` (5), log `deadbolt_anchor_fixture.rs` (2), `chain.rs` (1),
`regen_golden.rs` (3), and episodic `episodic.rs` (1). No production crate
calls it. Replay calls cover all production views and existing test-only
projections. Summary use remains output-only, including conversion to the
origin-admission verified-prefix audit identity.

## Selected architecture

Add exactly one public conceptual type:

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

The private constructor spelling may vary; these properties may not:

- the field and constructor are non-public;
- only the post-verification replay loop calls the constructor;
- the type borrows the exact retained `SignedEvent`;
- it is not `Default`, `Serialize`, `Deserialize`, `Clone`, or `Copy`;
- it has no public raw conversion, unchecked constructor, builder, marker, or
  verification flag;
- its only public observation is `event()`; and
- the trait receives a shared reference so composites can forward the same
  witness without minting or copying it.

`VerifiedReplayEvent` means only that the event is currently being presented
after complete replay verification of one retained snapshot under the reader's
supplied key. It is not trusted, authoritative, admitted, true, current,
standing, identity, a credential, or a portable proof.

Public `Projection` remains unsealed and downstream-implementable. The trait
remains infallible. A downstream custom projection can be replay-driven and
can forward a genuine callback input, but cannot fabricate one from raw
history.

## Raw inspection compatibility decision

Choose Option B:

```text
events()               remove
unverified_events()    add; returns Vec<SignedEvent>
deprecated alias       do not add
```

This intentionally breaks the exact provisional name while preserving raw
inspection. Repository history shows the name arrived as scaffold shape and
never acquired a downstream compatibility contract. All current callers are
tests/golden tooling, and the release contract explicitly does not promise
pre-1.0 Rust source compatibility.

Option A would be structurally sufficient after changing the trait but would
retain ambiguity. Option C would duplicate public raw API without evidence of
a compatibility need. Raw output remains raw and cannot enter the new trait
boundary under any option.

Compatibility conclusions are deliberately distinct:

```text
SignedEvent as public raw historical material: intentional, preserve
public custom Projection implementations: intentional, preserve
raw LogReader inspection capability: intentional, preserve
exact events() method name: provisional, remove
manual raw Projection::apply: accidental permissiveness, remove
```

## Snapshot and failure law

The implementation must retain:

```text
one store read
-> complete parse and verification
-> exact retained event vector
-> witness construction and application
-> exact count/tip summary
```

A late parse, link, hash, signature, genesis, key/profile, or payload failure
must create no wrapper and make zero projection calls. `verify_chain()` remains
summary-only and cannot mint an input. A copied `VerifiedReplaySummary` cannot
wrap or bless caller-selected records.

## Expected future runtime surface

Expected concentration:

- `crates/magpie-log/src/logimpl.rs` — wrapper, private construction, trait
  signature, post-verification presentation, and raw method rename;
- `crates/magpie-log/src/lib.rs` — public re-export and boundary rustdoc;
- `crates/magpie-claims/src/lib.rs` — `ClaimsView` implementation;
- `crates/magpie-claims/src/standing.rs` — `StandingView` implementation;
- `crates/magpie-claims/src/deadbolt_context.rs` — anchor implementation and
  now-structural warning text;
- `crates/magpie-claims/src/replay_snapshot.rs` — shared-wrapper composite;
- `crates/magpie-episodic/src/lib.rs` — episodic implementation; and
- affected tests/examples — trait signatures, raw-name migration, compile-fail
  cases, replay positives, and removal of manual raw application mechanics.

Likely directly affected test/tool paths include:

```text
crates/magpie-log/tests/chain.rs
crates/magpie-log/tests/deadbolt_anchor_fixture.rs
crates/magpie-log/tests/golden.rs
crates/magpie-log/examples/regen_golden.rs
crates/magpie-episodic/tests/episodic.rs
```

Other replay integration suites must remain green even when their source does
not require edits. `event.rs` should need no format-semantic change. If
persisted `SignedEvent` shape must change, stop: the selected design has been
violated.

## Negative proof matrix

| ID | Permanent proof | Expected compiler/API result |
| --- | --- | --- |
| N1 | raw `SignedEvent` calls `apply` | `E0308`-class wrapper/raw type mismatch |
| N2 | `unverified_events()` output calls `apply` | same intended type mismatch |
| N3 | serde deserializes wrapper | missing `Deserialize` bound |
| N4 | downstream struct literal constructs wrapper | private field failure |
| N5 | raw event converts to wrapper | no `From`/`TryFrom` or equivalent |
| N6 | `verify_chain()` result wraps arbitrary event | no event witness factory/API |
| N7 | copied `VerifiedReplaySummary` mints event | no blessing method/input position |
| N8 | correctly hand-signed record calls `apply` | still raw/wrapper mismatch |
| N9 | raw feeds any built-in or custom projection | common wrapper-only trait boundary |
| N10 | alternate raw/unchecked apply path | public source/API scan finds none |

The implementation must cover `ClaimsView`, `StandingView`,
`DeadboltAnchorIndex`, `EpisodicView`, and a downstream custom projection in
N9. Negative cases must fail at the intended boundary with otherwise valid
setup. A separate lifetime compile-fail case must also prove that a projection
cannot retain `&VerifiedReplayEvent` after its `apply` callback; a retained
clone of the observed `SignedEvent` is raw only.

## Positive proof matrix

| ID | Permanent proof |
| --- | --- |
| P1 | downstream custom `Projection` implements the trait and replay drives it |
| P2 | valid replay yields byte-identical `ClaimsView` |
| P3 | valid replay yields identical `StandingView` behavior/bytes |
| P4 | valid replay yields identical Deadbolt occurrence context |
| P5 | one private composite fans the same wrapper to multiple children without reread/reverification |
| P6 | valid replay yields unchanged episodic canonical derived bytes |
| P7 | replay reads exactly one snapshot |
| P8 | valid prefix plus bad signature/hash/link yields zero applications |
| P9 | replay summary count/tip exactly match the applied retained snapshot |
| P10 | explicit raw diagnostics remain through `unverified_events()` |
| P11 | golden and Deadbolt-anchor chains remain byte-for-byte unchanged |
| P12 | external trust in the supplied verifying key remains explicitly external |

## Canonical invariants

Required zero changes:

```text
docs/FORMAT.md
Payload and payload tags
EventCore
SignedEvent persisted shape and JSONL representation
ContentHash and hash preimage
Sig and signature message/domain
genesis semantics
golden fixture bytes
Deadbolt-anchor fixture bytes
```

No format/profile bump, fixture regeneration, or new event tag is permitted.

## Test migration law

Tests must conform to the production boundary. Allowed approaches are:

- construct a valid test log and use real `LogReader::replay`;
- move pure folding mechanics behind a private helper and test that helper
  inside the owning crate; or
- compare independent genuine replays.

Forbidden test conveniences include a public wrapper constructor,
`new_unchecked`, `apply_raw`, `apply_unverified`, a serde witness, a public test
factory, or a production adapter accepting `SignedEvent`.

## Non-goals and adjacent findings

This ticket does not implement, redesign, close, or narrow:

- A-007 / RQ-009 projection/storage failure semantics;
- A-003, A-019 / RQ-011, or A-023 detached projection/response coordinates;
- A-021 verifying-key trust or key strictness;
- A-022 admission, `EpistemicGate`, actor identity, roles, or permissions;
- A-024 independently authority-bound origin corroboration; or
- any FORMAT, policy, claim, evidence, standing, MCP, librarian, or writer
  semantics.

The wrapper is not a generic capability or authority framework. It introduces
no token, credential, registry, policy, snapshot object, callback abstraction,
adapter, mode, or second projection trait.

## ADR decision

No ADR is required. This is enforcement of already-declared one-snapshot,
verify-before-apply doctrine and the confirmed audits. If implementation would
require sealing `Projection`, changing complete verification, changing
trust-root semantics, or making detached verified events authoritative, it
must stop for owner ratification rather than invent ADR-0008.

## Validation performed

Preflight performed the owner-specified fetch/status/branch/SHA/merge-base/log
and live-remote checks. Results:

```text
isolated worktree:
C:\Users\herpe\.codex\worktrees\magpie-verified-replay-projection-boundary-contract

branch:
agent/verified-replay-projection-boundary-contract

HEAD:
c2fe991d8023d237f7d589bd3f022233c1f332cc

origin/main:
c2fe991d8023d237f7d589bd3f022233c1f332cc

merge-base:
c2fe991d8023d237f7d589bd3f022233c1f332cc

live remote main:
c2fe991d8023d237f7d589bd3f022233c1f332cc

initial isolated status:
clean
```

All mandated audit, ADR, design, log, claims, and episodic files were read in
full. Additional replay/projection tickets, the release contract, Deadbolt
occurrence contract, origin-admission replay contract, and prior sole-writer
contract were inspected for governing compatibility and authority rules.

Temporary untracked Rust 1.94.1 probes established:

| Question | Outcome |
| --- | --- |
| public trait names a private-field/private-constructor input | metadata compile passed |
| separate downstream crate implements the public trait | metadata compile passed |
| composite forwards one shared witness to two child projections | metadata compile passed |
| wrapper borrows the retained event and exposes `event()` | metadata compile passed |
| rustdoc shows private fields, public accessor, and public trait signature but no private constructor | passed |
| raw `SignedEvent` calls `apply` | failed as intended with `E0308` |
| downstream struct literal constructs wrapper | failed as intended with `E0451` |
| `&SignedEvent` converts into wrapper | failed as intended with `E0277` |
| serde deserializes wrapper | failed as intended with `E0277` |
| projection detaches a borrowed witness | failed as intended with lifetime errors |

The probes used no unsafe, registry, marker trait, private-bound lint
suppression, or global state. Executable linking was unavailable because this
host's Visual Studio installation lacks the MSVC library directory and
`msvcrt.lib` (`LNK1104`), even after `VsDevCmd.bat`; metadata compilation and
rustdoc performed the relevant type/privacy/lifetime checks. All probe sources
and outputs were removed before documentation edits.

Documentation-safe repository validation passed:

```text
cargo fmt --all --check
python tools/check_release_metadata.py
python tools/verify_chain.py crates/magpie-log/testdata/golden-v1.jsonl \
  ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c
python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl \
  d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737
git diff --check
git status --short --untracked-files=all
git diff --name-only origin/main --
```

Release metadata reported consistent inventories of 19 `magpie-log` files,
48 `magpie-claims` files, and 9 `magpie-episodic` files. The independent
verifiers reported all nine golden records and both Deadbolt-anchor records
`OK`. The pre-commit diff contained exactly the two allowed paths.

Protected-path `git diff --quiet origin/main -- <path>` checks reported
unchanged for `crates`, `docs/audits`, `docs/adr`, `docs/FORMAT.md`,
`fixtures`, `Cargo.toml`, `Cargo.lock`, and `.github`. Full linked Rust tests,
Clippy, the tour, and package assembly were not rerun locally for this
documentation-only slice; the missing local MSVC library noted above prevents
linked execution, and draft-PR Linux CI remains the publication gate.

## Contract self-review

| Check | Answer |
| --- | --- |
| A — raw `SignedEvent` remains legitimate public history | Yes. |
| B — raw `SignedEvent` itself proves verification | No. |
| C — raw `SignedEvent` can enter future `Projection::apply` | No. |
| D — only completed snapshot verification inside replay creates the input | Yes. |
| E — ordinary downstream safe Rust can construct the input | No. |
| F — the input is deserializable | No. |
| G — the input is ephemeral and lifetime-bound | Yes. |
| H — public custom projections remain supported | Yes. |
| I — one input can fan out through a composite | Yes, by shared reference. |
| J — late verification failure produces zero applications | Yes. |
| K — replay remains one-snapshot | Yes. |
| L — raw diagnostic inspection remains possible | Yes, through `unverified_events()`. |
| M — trust in the supplied verification key remains external | Yes. |
| N — canonical bytes and persisted `SignedEvent` shape remain unchanged | Yes. |
| O — A-007 fallibility work remains separate | Yes. |
| P — A-019/A-023 detached-response coordinates remain separate | Yes. |
| Q — admission, identity, and `EpistemicGate` remain absent | Yes. |
| R — public concept count added | One: `VerifiedReplayEvent<'event>`. |

## Changed-path allowlist

This PR changes exactly:

```text
docs/design/verified-replay-projection-boundary-contract.md
tickets/0070-verified-replay-projection-boundary-contract.md
```

No third path is required. In particular, do not change:

```text
crates/**
docs/FORMAT.md
docs/adr/**
docs/audits/**
fixtures/**
Cargo.toml
Cargo.lock
.github/**
```

## Implementation handoff state

After this contract PR:

```text
A-009 / RQ-002: Confirmed
runtime remediation: not implemented
audit-disposition ledger: unchanged
next evidence: one separately authorized Rust/API implementation satisfying
  this design and N1-N10/P1-P12
```

The later runtime slice must preserve one-snapshot all-or-nothing replay,
change only the necessary API/implementations/tests/examples, add structural
negative proofs, and leave audit reclassification to a later administrative
slice after independent review and owner merge.

## Reviewer checklist

- [ ] The wrapper proves only same-invocation post-verification presentation.
- [ ] Raw `SignedEvent` remains public and never becomes trusted by value.
- [ ] No caller minting, serde, conversion, summary-blessing, or unchecked seam
      exists.
- [ ] Public custom projections remain supported.
- [ ] Composite fan-out needs no construction, clone, second read, or unsafe.
- [ ] Late failure still means zero application and no summary.
- [ ] Raw inspection is deliberately renamed, not hidden or duplicated.
- [ ] All four production projection families and both test-only projections
      are accounted for.
- [ ] Canonical records, hashes, signatures, fixtures, and FORMAT are untouched.
- [ ] A-007, A-019, A-021, A-022, A-023, and A-024 remain separate.
- [ ] A-009 / RQ-002 remains Confirmed and the audit ledger is unchanged.

## Publication authority

The owner instruction authorizes one commit on the task branch, one push, and
one **draft** PR against `main`. Human owner retains sole merge authority. Do
not merge, enable auto-merge, or mark the PR ready for review.

## Stop conditions

Stop and report rather than broadening scope if:

- live main acquires another Ticket 0070 or A-009 contract/implementation;
- a third changed path becomes objectively necessary;
- raw `SignedEvent`, public custom projection, or raw inspection preservation
  proves impossible with the selected one-type design;
- one-read all-or-nothing replay cannot be preserved;
- implementation would require new trust, admission, detached provenance, or
  projection-failure doctrine; or
- validation requires a runtime, FORMAT, ADR, audit, dependency, fixture, or
  manifest change.
