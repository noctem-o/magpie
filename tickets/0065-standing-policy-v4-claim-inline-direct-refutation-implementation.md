# Ticket 0065: Standing Policy v4 Claim-Inline Direct Refutation Implementation

## Status

Implementation complete; final validation and same-session review are recorded
below.

Required and synchronized base:

```text
459506fbd4f7c526138afbe1635ee7d182b540d0
Merge pull request #79 from noctem-o/agent/standing-policy-v4-claim-inline-direct-refutation-contract
```

Branch:

```text
agent/standing-policy-v4-claim-inline-direct-refutation-runtime
```

Proposed commit and draft PR title:

```text
feat(claims): implement standing policy v4 claim-inline direct refutation
```

Normative contract:

[`tickets/0064-standing-policy-v4-claim-inline-direct-refutation-contract.md`](0064-standing-policy-v4-claim-inline-direct-refutation-contract.md)

Normative design:

[`docs/design/standing-policy-v4-claim-inline-direct-refutation-v0.md`](../docs/design/standing-policy-v4-claim-inline-direct-refutation-v0.md)

## Checkout and synchronization

The initial checkout was
`agent/standing-policy-v4-claim-inline-direct-refutation-contract` at
`bf81da35bda9ae5830b8a3501f9deef3a6fe55fd`. The required preflight switched
to `main`, fetched and fast-forwarded it to
`459506fbd4f7c526138afbe1635ee7d182b540d0`, verified local and remote
divergence `0 0`, the exact merge subject above, a clean checkout, no active
Git operation, and absence of the proposed local and remote branch. It then
created the implementation branch at that exact SHA.

The continuation preflight reverified:

```text
root:
C:/magpie

branch:
agent/standing-policy-v4-claim-inline-direct-refutation-runtime

HEAD:
459506fbd4f7c526138afbe1635ee7d182b540d0

index and working tree:
clean before implementation

merge, rebase, cherry-pick, or revert:
none active
```

No second worktree was created.

## Implementation classification

```text
correctness:
implements one closed claim-inline deterministic direct-refutation policy-v4
runtime

policy:
magpie-claims-standing-v4

rule:
sha256_claim_inline_bytes_direct_refutation_v0

public resolver:
OriginAdmissionReplayContextV0 plus claim_id and immutable
ResolutionContentClosureV0 only

inheritance:
complete internally derived StandingResolutionV3 from the same replay context
and closure

candidate source:
exact replayed first-write-wins contradicts edges targeting the requested claim

predicate:
unchanged sha256_claim_inline_bytes_equals_v0

claim resolution:
at most once per extension-safe policy call with one or more candidates

digest evaluation:
at most once per extension-safe policy call with one or more candidates

attestation:
separate same-snapshot binding per candidate evidence node

eligible lane:
DigestUnequal × contradicts × RefutationEligible × Some(Refuted)

permitted promotion:
inherited None, Open, or Conjectured to governed Refuted

inherited positive standing:
Supported and Settled preserved behind
InheritedPositiveStandingRequiresContradictionPolicy

inherited governed Refuted:
preserved without another application or amplification

legacy raw Refuted:
quarantined

support:
no new support rule; SupportEligible remains standing-inert

policies v0-v3:
unchanged

contradiction debt:
absent

aggregation and repetition amplification:
absent

L0 and FORMAT:
unchanged

policy.rs and ceiling cells:
unchanged

writer, loader, CAS, filesystem, network, model, plugin, Deadbolt:
unchanged
```

## Changed-path boundary

The complete authorized allowlist and observed case-sensitive changed-path
union are:

```text
README.md
crates/magpie-claims/src/claim_inline_sha256_predicate.rs
crates/magpie-claims/src/claim_inline_sha256_predicate/edge_lane.rs
crates/magpie-claims/src/lib.rs
crates/magpie-claims/src/origin_admission_replay.rs
crates/magpie-claims/src/resolution_content_closure.rs
crates/magpie-claims/src/standing_v3.rs
crates/magpie-claims/src/standing_v4.rs
crates/magpie-claims/tests/standing_v4.rs
docs/design/claim-inline-sha256-predicate-v0.md
docs/design/claim-inline-predicate-edge-lanes-v0.md
docs/design/standing-view-evidence-ceilings.md
docs/design/standing-aggregation-independence-groups.md
docs/design/standing-policy-v4-claim-inline-direct-refutation-v0.md
tickets/0065-standing-policy-v4-claim-inline-direct-refutation-implementation.md
```

`standing_v3.rs` is used only for the authorized
`#[cfg(test)] pub(crate)` policy-ID staging helper needed to construct a
foreign inherited v3 identity. It has no production semantic change.

`origin_admission_replay.rs` and `resolution_content_closure.rs` each add one
owner-authorized `#[cfg(test)] pub(crate)` identity helper. Neither helper is
exported. The helpers validate exact 64-character lowercase hexadecimal
digests and construct only `VerifiedLogPrefixIdentityV0` or
`ResolutionContentClosureIdentityV0`. They cannot stage a complete replay
context, snapshot/identity pairing, closure, retained object set, or lookup
behavior.

## Public API

`lib.rs` exports exactly:

```text
MAGPIE_CLAIMS_POLICY_V4_ID
StandingPolicyRuleV4
StandingPolicyContextV4
StandingTraceClassificationV4
StandingTraceEntryV4
StandingPolicyApplicationV4
StandingBlockerV4
StandingResolutionFailureV4
StandingResolutionV4
```

The only resolver methods are:

```rust
impl OriginAdmissionReplayContextV0 {
    pub fn resolved_standing_v4(
        &self,
        claim_id: &str,
        closure: &ResolutionContentClosureV0,
    ) -> Option<Status>;

    pub fn resolved_standing_with_trace_v4(
        &self,
        claim_id: &str,
        closure: &ResolutionContentClosureV0,
    ) -> StandingResolutionV4;
}
```

The scalar delegates to the trace resolver. There is no public free resolver,
composer, candidate resolver, policy selector, latest-policy alias,
`StandingView` v4 method, or `StandingReplaySnapshot` v4 method.

The three authority-bearing structs have private fields and construction,
derive only `Clone` and one-way `Serialize`, and expose only the exact
Ticket 0064 read-only getter surface. They implement no `Deserialize`,
`Default`, mutation, conversion, extraction, `Debug`, `Display`, `Error`,
equality, ordering, or hashing surface.

## Module and private composition structure

`standing_v4.rs` contains the public closed vocabulary, private trace and
application construction, the sealed composition input, one pure crate-private
composer, candidate enumeration and derivation, exact precedence, and the two
inherent context methods.

`claim_inline_sha256_predicate.rs` keeps
`ResolvedClaimInlineSubjectV0` crate-private and exposes no subject or digest
material. Its narrow relation evaluator accepts the resolved claim and returns
only the closed crate-private equal/unequal token.

`edge_lane.rs` adds one crate-private negative-candidate resolver. It accepts
the same snapshot, already-resolved claim, already-evaluated relation, and
exact candidate IDs; invokes Ticket 0061's post-claim binding separately;
validates edge kind, source, target, and scope in Ticket 0063 order; and reuses
the existing four-cell matrix. The production wrapper supplies only the
compiled deterministic-verification/exact-machine-checkable refutation cell.
It never constructs or inspects a public Ticket 0063 outcome or receipt.

The v4 policy decision consumes a private
`StandingCandidateDerivationV4` flag produced beside the public trace. It does
not reinject or inspect public Ticket 0059, 0061, or 0063 audit output.

## One-resolution and one-relation law

The production path enumerates candidates first. An empty universe returns an
empty derivation without invoking claim resolution or relation evaluation.
For a non-empty universe, a generic private derivation seam invokes its
`FnOnce` claim resolver exactly once and, only after success, its `FnOnce`
relation evaluator exactly once. The closed relation is copied to each
candidate classifier, which performs a separate attestation binding.

Focused counter tests observe:

```text
three candidates:
claim resolutions = 1
relation evaluations = 1
candidate bindings = 3

zero candidates:
claim resolutions = 0
relation evaluations = 0

claim-resolution failure over three candidates:
claim resolutions = 1
relation evaluations = 0
candidate bindings = 0
trace failures = 3

identity or extension-safety stop:
candidate derivations = 0
```

## Candidate enumeration and order

Production reads only the replayed
`StandingView.justification_edges: BTreeMap<String, ...>` and retains entries
whose edge-kind bytes equal `contradicts` and target bytes equal the requested
claim ID. The map key is the trace edge ID and the replayed source ID is the
evidence ID. Existing raw `String::Ord` order is retained.

Public real-replay tests cover support, unknown-kind, and unrelated-target
exclusion; raw lexical ordering; first-write-wins duplicate IDs; distinct IDs;
complete processing after an eligible candidate; and repeated resolver
byte-identity.

## Identity and extension safety

After internally deriving v3, the pure composer checks, in order:

1. inherited claim ID;
2. inherited policy ID;
3. verified-prefix identity; and
4. closure identity.

The first mismatch clears top-level standing and currentness authority,
retains the complete inherited object, emits the exact outer failure, and
performs no candidate derivation.

An inherited v3 resolution failure produces only
`InheritedV3ResolutionFailed`. The first inherited
`SupportAuditIncomplete`, `SupportAuditRejected`, or
`InheritedV2ResolutionFailed` blocker produces only
`InheritedV3NotExtensionSafe`, while the nested v3 object retains its complete
blocker vector. `InsufficientDistinctOriginGroups` is skipped as
extension-safe.

## Standing precedence and multiplicity

Identity-valid and extension-safe inherited governed `Refuted` is preserved
after complete candidate audit with no application or blocker. Inherited
`Supported` or `Settled` plus eligibility is preserved with exactly one
`InheritedPositiveStandingRequiresContradictionPolicy` blocker carrying the
exact status. Inherited `None`, `Open`, or `Conjectured` plus at least one
eligible candidate creates exactly one application and governed `Refuted`.
Without eligibility every inherited value is preserved.

Legacy raw values are copied for audit only and never inspected by
precedence. Several eligible candidates remain separate ordered trace entries
but create one application and one standing result. Failed and ineligible
candidates do not aggregate into authority, and an independent failed
candidate does not veto an eligible sibling.

## Canonical profile and vectors

`StandingResolutionV4::canonical_bytes()` directly invokes
`serde_json::to_vec(self)` under
`magpie-claims-standing-v4-resolution-json-v0`. The typed declaration order is
the wire order; only absent `resolution_failure` is omitted, while trace reason
and failure fields serialize as explicit nulls.

The nine canonical tests construct actual `StandingResolutionV4` values
through the real private composition seam, then call the actual
`StandingResolutionV4::canonical_bytes()` method. They do not use mirror wire
structs or parse Markdown at runtime. The two staged identity helpers are used
only in crate unit tests to reproduce the ratified cryptographic identities.
Public integration tests instead use real signed replay and real
`ResolutionContentClosureV0::construct` calls.

Observed runtime vectors:

| Vector | Bytes | Lowercase SHA-256 |
| --- | ---: | --- |
| A | 2,197 | `3292f6ceafc1dbd16666cce56f2bdac6de7d8de5faeb36cc4495fe272e243d9a` |
| B | 2,466 | `755aa9761363346847d54a8250cf9c82a5d0c917cf40fe250e611cc42d8e57c5` |
| C | 2,340 | `02b8aec71af0b4e21370ad8ba01b1c5796600e6702df1881bbf36ddd58f7e5be` |
| D | 2,366 | `8ae9ce1ca675a954e8b01e315a2be6f94f393db4defa88aae87bbd410c816637` |
| E | 2,593 | `b1a77cb96b628a30e5e8b1d70a075835d69abbe9f07e6275b6cf43e30dc79db5` |
| F | 2,437 | `cab622ea9e552e14e49a5b302d0cbfca0cae778644841c64fd42343f8b049e09` |
| G | 2,429 | `40b633838e23754522e51a3e31f9f915537eb109b4c14087772ebceec09dfadd` |
| H | 2,332 | `59e179505b97022dd4e85b49f5a24d922f39fcbfa91bc56feca861389e50f71b` |
| I | 2,258 | `ac1dd8a308aaf4f93dec01152409350d47f8b5008174d998de52962f4a52c800` |

Every Rust assertion checks the complete literal, byte length, lowercase
SHA-256, compact framing, BOM/newline absence, outer-failure omission law,
always-present arrays, and typed key order. The environment-gated unit-test
emitter exists only to feed an independent Python `hashlib.sha256` check and
does not retain output files.

## Exhaustive failure retention and private invariants

Unit tests enumerate all 33 Ticket 0059 claim-resolution failures in frozen
order and verify each survives inside:

```text
AttestationBindingFailed(
    ClaimPredicateResolutionFailed(exact failure)
)
```

They enumerate all 28 Ticket 0061 binding failures in frozen order, verify
enum identity, and pin the exact nested canonical spelling. The crate-private
edge seam defensively stages and verifies `MissingEdge`,
`UnsupportedEdgeKind`, `EdgeSourceBindingMismatch`,
`EdgeTargetBindingMismatch`, `EdgeScopeBindingMismatch`, and
`RefutationCeilingInvariantMismatch`. Its equal/contradicts test proves the
ineligible cell never consults the ceiling.

## Compile-fail authority barriers

Doctests separately pin the three authority-bearing structs against public
construction, constructors, deserialization, defaults, mutation, consuming
extraction, `From`, `TryFrom`, `Debug`, `Display`, `Error`, equality, ordering,
and hashing. Every new enum is pinned against `Deserialize` and `Default`.

Separate resolver-signature doctests reject inherited v3, v4 trace,
application, blocker and failure audit, all Ticket 0059/0061/0063
outcomes/receipts, relation tokens, candidate lists, edge/evidence values,
status, ceiling, threshold, policy ID, precedence, callbacks, serialized
bytes, alternate snapshots, prefix identities, and audit objects. Other
doctests pin the absence of a free resolver, `StandingView` or snapshot
method, latest alias, public composer, raw candidate resolver, and production
staging API.

The existing verified-prefix identity and closure-identity compile-fail
families remain intact. They continue to prove that callers cannot construct
or deserialize either identity, pair a snapshot with a supplied identity, or
construct a closure from an identity.

## Temporary mutation

The decisive temporary precedence mutation and restored result are recorded
in the final validation table.

## Validation record

The final command-by-command results, independent vector check, tour and
frozen-chain observations, mutation result, diff checks, and path audit are
recorded here after the final restored patch:

```text
FOCUSED BUILD

PASS  cargo check -p magpie-claims --all-targets --locked
PASS  cargo fmt --all --check

FOCUSED TESTS

PASS  cargo test -p magpie-claims standing_v4 --locked
      15 matching unit tests passed
PASS  cargo test -p magpie-claims --test standing_v4 --locked
      8 integration tests passed
PASS  cargo test -p magpie-claims claim_inline_sha256_predicate --locked
      19 matching tests passed
PASS  cargo test -p magpie-claims inline_predicate_attestation --locked
      9 matching tests passed
PASS  cargo test -p magpie-claims claim_inline_predicate_edge_lane --locked
      15 matching integration tests passed
PASS  cargo test -p magpie-claims standing_v3 --locked
      19 matching unit tests and 1 matching integration test passed

EXPLICIT COMPATIBILITY TARGETS

PASS  cargo test -p magpie-claims --test claim_inline_sha256_predicate --locked
      19 tests passed
PASS  cargo test -p magpie-claims --test inline_predicate_attestation --locked
      15 tests passed
PASS  cargo test -p magpie-claims --test claim_inline_predicate_edge_lane --locked
      15 tests passed
PASS  cargo test -p magpie-claims --test deterministic_support_standing_v2 --locked
      40 tests passed
PASS  cargo test -p magpie-claims --test standing_v3 --locked
      13 tests passed

FULL VALIDATION

FAIL  initial cargo clippy --workspace --all-targets --locked -- -D warnings
      found three unneeded unit expressions and one clone-on-Copy expression;
      all four were mechanically corrected
PASS  final cargo clippy --workspace --all-targets --locked -- -D warnings
PASS  final cargo test --workspace --locked
PASS  final cargo test --doc --locked
      281 magpie-claims and 3 magpie-log doctests passed
PASS  cargo test -p magpie-claims --doc --locked origin_admission_replay
      all 6 verified-prefix identity compile-fail doctests passed
PASS  cargo test -p magpie-claims --doc --locked resolution_content_closure
      all 6 closure-identity compile-fail doctests passed

TOUR AND FROZEN CHAINS

PASS  cargo run --locked --example tour -p magpie-claims
      raw stdout = 1,917 bytes
      SHA-256 =
      48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f
PASS  python tools/verify_chain.py
      crates/magpie-log/testdata/golden-v1.jsonl
      ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c
PASS  python tools/verify_chain.py
      fixtures/deadbolt-anchor-v1/anchor-log.jsonl
      d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737

RELEASE AND PACKAGE

BLOCKED  python tools/check_release_metadata.py in the intentional dirty
         checkout: its unchanged package-list subprocess refused README.md
BLOCKED  cargo package -p magpie-log --locked in the intentional dirty
         checkout: Cargo refused README.md
PASS     unchanged python tools/check_release_metadata.py in <TEMP_ROOT>
PASS     unchanged cargo package -p magpie-log --locked in <TEMP_ROOT>
PASS     no --allow-dirty was used; the VCS-free mirror contained the exact
         patch overlay and was verified removed

INDEPENDENT CANONICAL-VECTOR CHECK

PASS  the environment-gated unit-test emitter produced all 9 actual
      StandingResolutionV4::canonical_bytes streams
PASS  Python hashlib.sha256 independently reproduced every ratified length
      and lowercase digest in vectors A-I
PASS  no emitter-output files were created or retained

TEMPORARY MUTATION

FAIL AS EXPECTED
      cargo test -p magpie-claims
      standing_v4::tests::complete_standing_precedence_and_legacy_raw_quarantine_are_exact
      --locked
      after temporarily preserving Conjectured instead of promoting to
      Refuted; decisive assertion observed left Some(Conjectured), right
      Some(Refuted), with 0 passed and 1 failed
PASS  the exact command after restoring the ratified precedence law;
      1 passed and 0 failed
PASS  no mutation code or artifact remains

DIFF AND PATH AUDIT

PASS  git diff --check 459506fbd4f7c526138afbe1635ee7d182b540d0 --
PASS  git diff --cached --check
PASS  git diff --check
PASS  case-sensitive changed-path union: 15 authorized paths
PASS  12 tracked unstaged paths, 3 untracked paths, 0 staged paths
PASS  0 rename endpoints, 0 non-normal index flags, 0 unmerged entries,
      and 0 paths outside the allowlist

SKIPPED COMMANDS

none
```

## Same-session self-review

These are same-session self-review passes, not independent external review.
Their final verdicts are recorded after the restored patch and full validation:

```text
PASS 1 — soundness and authority:
PASS; no Ticket 0056 substitution, evidence-selected subject, audit
reinjection, caller-selected authority, mixed replay, repeated claim/relation
evaluation, shared evidence binding, or absence-as-falsity path found

PASS 2 — policy, inheritance, and precedence:
PASS; complete v3 inheritance, outer identity order, extension safety,
positive-standing preservation, inherited-Refuted non-amplification, raw
standing quarantine, failure independence, full candidate audit, and
zero-or-one application law are exact

PASS 3 — contract, API, bytes, and scope:
PASS; exact public names/getters/derives/serde order, authority barriers,
nine canonical byte streams, v0-v3 and Ticket 0059/0061/0063 inertia,
allowlisted paths, historical/living documentation boundary, and absence of
machine-specific or mutation material verified
```

## Explicit non-effects

This implementation does not revive Ticket 0056 or reinterpret
`sha256_bytes_equals_v0` and `DigestMismatch`. It does not change
`policy.rs`, any ceiling cell, policies v0-v3, L0, FORMAT, existing vectors,
fixtures, Cargo metadata, dependencies, the Python verifier, tour behavior,
or frozen chains. It adds no support policy; `SupportEligible` remains
standing-inert.

It adds no contradiction debt, conflict winner, invalidation,
supersession/currentness, absence-as-falsity, aggregation, voting, scoring,
confidence, repetition amplification, `EpistemicGate`, writer, loader, CAS,
filesystem acquisition, network acquisition, model integration, plugin, or
Deadbolt authority.

## Publication boundary

This is one owner-reviewable uncommitted patch. No commit, push, pull request,
review request, thread mutation, merge, auto-merge, rebase, amend, reset,
restore, stash, clean, branch deletion, force update, Git configuration
change, or GitHub mutation is part of Ticket 0065.
