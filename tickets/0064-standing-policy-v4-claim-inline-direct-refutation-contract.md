# Ticket 0064: Standing Policy v4 Claim-Inline Direct Refutation Contract

## Status

Documentation-only contract ratification. This ticket adds no Rust, tests,
fixtures, Cargo surface, policy implementation, writer, loader, or ingestion
path. A separately reviewed runtime ticket is required before any governed
`Refuted` result can be produced by policy v4.

Required base:

```text
bf81da35bda9ae5830b8a3501f9deef3a6fe55fd
Merge pull request #78 from noctem-o/agent/claim-inline-predicate-edge-lanes-runtime
```

Branch:

```text
agent/standing-policy-v4-claim-inline-direct-refutation-contract
```

Proposed commit and draft PR title:

```text
docs: ratify standing policy v4 claim-inline direct refutation
```

Normative design:

[`docs/design/standing-policy-v4-claim-inline-direct-refutation-v0.md`](../docs/design/standing-policy-v4-claim-inline-direct-refutation-v0.md)

## Contract classification

```text
correctness:
ratifies one closed claim-inline deterministic direct-refutation policy rule

authority:
documentation-only contract; no authority path changes in this PR

policy identity:
magpie-claims-standing-v4

rule identity:
sha256_claim_inline_bytes_direct_refutation_v0

inheritance:
complete internally derived policy-v3 resolution from the same replay context
and closure

public input:
one OriginAdmissionReplayContextV0, one requested claim_id, and one immutable
ResolutionContentClosureV0

candidate substrate:
private same-snapshot Ticket 0063 derivation only

eligible negative lane:
sha256_claim_inline_bytes_equals_v0
× ExactMachineCheckable
× DeterministicVerification
× contradicts
× DigestUnequal
× RefutationEligible

candidate ceiling:
Refuted

achieved effect:
one uncontested eligible direct-refutation path may produce governed Refuted

support:
no new governed-support rule; SupportEligible remains standing-inert

policy v2:
unchanged

policy v3:
unchanged and inherited completely

standing runtime:
absent in this PR

contradiction debt:
absent

invalidation:
absent

supersession/currentness:
absent

aggregation:
absent

repetition amplification:
absent

L0 and FORMAT:
unchanged

fixtures, tests, Cargo, dependencies, CI, release, verifier:
unchanged

writer, loader, CAS, filesystem, network, plugin, model, Deadbolt:
unchanged
```

## Decision

Ratify exactly one future policy rule:

```text
StandingPolicyRuleV4::Sha256ClaimInlineBytesDirectRefutationV0
serialized as sha256_claim_inline_bytes_direct_refutation_v0
```

The rule is permitted only when one exact, internally replay-derived
`contradicts` candidate targeting the requested claim is classified
`RefutationEligible` by the Ticket 0063 law, the compiled
`DeterministicVerification × ExactMachineCheckable` refutation ceiling is
exactly `Some(Refuted)`, the complete inherited v3 result is safe to extend,
and inherited governed standing is `None`, `Open`, or `Conjectured`.

The future resolver may then create one application and return governed
`Refuted`. It must never infer refutation from `DigestUnequal`, an edge,
binding success, malformed material, missing material, candidate absence,
public audit output, or a candidate ceiling alone.

## New design, not Ticket 0056 revival

Ticket 0056 remains a blocked historical record. Its evidence-relative
negative rule remains unsound because its evidence node selected the checked
bytes. Its proposed rule identity, negative trace names, conflict blocker,
resolution shape, and receipt shape remain withdrawn and are not inherited.

The historical predicate `sha256_bytes_equals_v0` is unchanged.
`DeterministicVerifierContextTraceV0::DigestMismatch` remains non-authority and
policy v2 keeps its positive evidence-relative rule unchanged.

This ticket instead uses only the different versioned proposition
`sha256_claim_inline_bytes_equals_v0`. Its exact subject bytes and expected
SHA-256 are both claim-owned. Ticket 0061 attestation metadata carries only
schema, predicate ID, subject claim ID, and scope; it cannot supply subject
bytes, an expected digest, a computed digest, or a relation. The Ticket 0056
unrelated-byte substitution therefore cannot enter this rule.

Reuse of the sequential identity `magpie-claims-standing-v4` means one new
ratification over the complete Tickets 0057-0063 substrate. It is not
retroactive ratification of Ticket 0056, and none of Ticket 0056's withdrawn
rule, trace, blocker, receipt, or resolution names is reused.

## Exact future public resolver

The only future public policy-v4 entry points are:

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

There is no free function, `StandingView` method,
`StandingReplaySnapshot` v4 method, public composer, latest-policy alias, or
caller-selected policy identity.

The resolver accepts no caller-provided v3 resolution, lane outcome, lane
receipt, predicate outcome, attestation outcome, evidence, edge, candidate
list, relation, ceiling, status, threshold, precedence table, callback,
serialized bytes, alternate snapshot, verified-prefix identity, or audit
object. It derives the complete v3 result internally through the same
`OriginAdmissionReplayContextV0` and the same immutable closure.

The closure is an explicit content input only because v4 inherits v3. The
direct-refutation lane does not read closure objects or closure bytes.

## Authority-preserving private composition

The future runtime must not consume, parse, pattern-match, or clone-and-inspect
any of these public audit values to decide policy:

- `Sha256ClaimInlineBytesPredicateOutcomeV0`;
- `Sha256ClaimInlineBytesPredicateReceiptV0`;
- `InlinePredicateAttestationBindingOutcomeV0`;
- `InlinePredicateAttestationBindingReceiptV0`;
- `ClaimInlinePredicateEdgeLaneOutcomeV0`;
- `ClaimInlinePredicateEdgeLaneReceiptV0`;
- their canonical bytes; or
- their public inspection enums.

The implementation must share or add the smallest crate-private resolved
representation. One policy call must:

1. derive the complete v3 result internally;
2. enumerate the exact replayed negative-candidate edge set;
3. resolve the claim-owned inline subject at most once;
4. evaluate the claim-owned digest relation at most once;
5. reuse that relation for every exact candidate;
6. bind each candidate evidence node separately after claim resolution;
7. validate exact source, target, and scope binding;
8. apply the Ticket 0063 matrix privately;
9. consult only the exact `Refuted` ceiling cell and only for an otherwise
   `RefutationEligible` candidate; and
10. construct v4 trace, blocker, application, and resolution values privately.

Equivalent Ticket 0059, 0061, or 0063 public audit output may be emitted beside
the private value. It must never be inspected or parsed to reach the policy
decision.

Subject bytes, expected digest, computed digest, and relation material must not
be widened into the attestation module. A runtime that needs to alter Ticket
0059, 0061, or 0063 public APIs, opacity, failure vocabularies, or vectors must
stop for a new contract.

## Candidate universe and order

Production candidates are derived only from `StandingView`'s replayed
first-write-wins justification-edge map. The exact candidate filter is:

```text
edge.edge_kind bytes == "contradicts"
and
edge.target_id bytes == requested claim_id bytes
```

For every retained edge:

```text
edge_id = exact replayed map key
evidence_id = exact edge.source_id
```

Candidate order is the existing `BTreeMap<String, ...>` raw `String::Ord`
edge-ID order. It is case-sensitive and normalization-free. There is no
newest, best, first-successful, fuzzy, aliased, namespaced, trimmed,
case-folded, or Unicode-normalized selection.

As in the established standing candidate model, unrelated target edges are
ignored rather than retained as trace entries. This negative universe also
ignores `supports` and unknown edge kinds rather than reinterpreting them.
Impossible-after-enumeration source, target, kind, or missing-edge failures
remain required private invariant tests; they are not ordinary production
trace entries.

Every production candidate trace entry records its exact edge ID, derived
evidence ID, and exactly one of:

- `RefutationEligible`;
- `Ineligible(DigestEqualDoesNotRefute)`; or
- `ResolutionFailed(exact ClaimInlinePredicateEdgeLaneFailureV0)`.

Candidate failures are audit-only. One malformed or missing candidate does not
veto an independent exact eligible candidate.

## Standing precedence

The exact precedence is:

1. An inherited claim, inherited policy-v3, verified-prefix, or closure
   identity mismatch returns the first exact outer failure, clears top-level
   governed and legacy standing to `None`, sets top-level currentness to
   `Unknown`, retains the complete inherited v3 object, and performs no
   candidate derivation or application.
2. An inherited v3 `resolution_failure`, or the first inherited v3 blocker
   `SupportAuditIncomplete`, `SupportAuditRejected`, or
   `InheritedV2ResolutionFailed`, preserves inherited standing and adds one
   closed v4 blocker. It performs no candidate derivation or application.
   `InsufficientDistinctOriginGroups` is a complete, ordinary no-v3-support
   result and is not an extension blocker.
3. Identity-valid inherited governed `Refuted` is preserved. Candidates remain
   auditable, but no v4 application or amplification is created.
4. Identity-valid inherited governed `Supported` or `Settled` plus one or more
   eligible candidates preserves inherited standing and emits
   `InheritedPositiveStandingRequiresContradictionPolicy` with the exact
   inherited status. No v4 application is created.
5. Identity-valid inherited `None`, `Open`, or `Conjectured` plus one or more
   eligible candidates yields one v4 application and governed `Refuted`.
6. With no eligible candidate, inherited standing is preserved.

This law covers current and defensively staged v0-v3 states. `Open`,
`Supported`, `Settled`, and governed `Refuted` may be unreachable for some
current claim-inline graphs, but the composer boundary must handle them
explicitly because the inherited policy contract permits them.

Legacy raw `Refuted` remains quarantined and cannot enter this precedence.
No v4 rule demotes `Settled`, overrides `Supported`, revives raw status,
chooses a winner, resolves a contradiction, infers currentness, or performs
invalidation or supersession.

## Multiplicity and non-amplification

One eligible path is sufficient when precedence permits application. Several
eligible paths remain separate trace entries in deterministic order, but
produce exactly one application and the same one governed `Refuted` result.
The `applications` vector has cardinality zero or one.

Repeated edges, repeated evidence nodes, repeated calls, cloned audit output,
and serialized output do not add confidence, weight, score, quorum, majority,
or standing. Failed and ineligible candidates never combine into authority.

If `supports` and `contradicts` edges coexist, the one claim-owned relation is
shared. Exactly one edge polarity can be eligible under the Ticket 0063
matrix. The other is ineligible when queried by the standing-inert lane
resolver and is outside the v4 negative candidate universe. That is not
contradiction debt.

## Exact v4 contract surface

The normative design freezes:

- `MAGPIE_CLAIMS_POLICY_V4_ID`;
- `StandingPolicyRuleV4`;
- `StandingPolicyContextV4`;
- `StandingTraceClassificationV4`;
- `StandingTraceEntryV4`;
- `StandingPolicyApplicationV4`;
- `StandingBlockerV4`;
- `StandingResolutionFailureV4`; and
- `StandingResolutionV4`.

`StandingResolutionV4` retains the requested claim ID, governed standing,
quarantined legacy raw standing, inherited currentness, policy ID,
verified-prefix identity, closure identity, complete inherited
`StandingResolutionV3`, first outer failure, exact ordered candidate trace,
zero-or-one application vector, and closed blockers.

Authority-bearing structs have private fields, private construction,
read-only getters, `Clone`, and one-way `Serialize`. They have no
`Deserialize`, `Default`, public constructor, public mutation, `From`,
`TryFrom`, consuming authority conversion, public test staging, resolver
reinjection, `Debug`, `Display`, `Error`, equality, ordering, or hashing
surface. Public enum tokens remain inert inspection vocabulary and are never
accepted by a resolver or composer.

## Outer failures and blockers

Outer failure order is:

1. `InheritedClaimMismatch`;
2. `InheritedPolicyMismatch`;
3. `VerifiedPrefixIdentityMismatch`;
4. `ClosureIdentityMismatch`.

The closed blocker vocabulary is:

```rust
pub enum StandingBlockerV4 {
    InheritedV3ResolutionFailed,
    InheritedV3NotExtensionSafe {
        blocker: StandingBlockerV3,
    },
    InheritedPositiveStandingRequiresContradictionPolicy {
        inherited_status: Status,
    },
}
```

`InheritedV3NotExtensionSafe` may contain only
`SupportAuditIncomplete`, `SupportAuditRejected`, or
`InheritedV2ResolutionFailed`. When more than one is present, v4 reports only
the first in inherited v3 blocker order because the complete inherited v3
object already retains the full blocker vector.

Outer failure, candidate failure, ineligibility, blocker, and absence of an
eligible candidate are distinct. Candidate failures never become outer
failures and do not globally veto independent candidates.

## Canonical audit

The exact future profile is:

```text
magpie-claims-standing-v4-resolution-json-v0
```

It is compact UTF-8 JSON emitted by direct typed fixed-field serde wire
representation. It is deterministic derived audit data, not L0, not an event
format, and never deserializable authority. It uses no `serde_json::Value`,
map-shaped wire object, JCS/RFC 8785, or generic canonical JSON.

The normative design freezes field order, enum spelling, inherited-v3
placement, trace/application/blocker order, optional outer-failure omission,
explicit null trace detail fields, escaping, Unicode behavior, no BOM, and no
trailing newline. It contains nine complete future-runtime vectors with exact
lengths and lowercase SHA-256 digests.

## Required future runtime tests

The separate implementation ticket must test:

- exact v4 policy and rule identities;
- complete v3 inheritance through the explicit context and closure;
- no latest-policy alias;
- one claim resolution and one digest relation per policy call;
- exact candidate enumeration, filtering, and order;
- all Ticket 0063 matrix classes and private defensive failure invariants;
- exact nested Ticket 0063, 0061, and 0059 failures;
- support-edge exclusion;
- `DigestEqual + contradicts` ineligibility;
- `DigestUnequal` without an exact `contradicts` candidate;
- wrong evidence, target, source, scope, kind, and ceiling drift;
- failure independence;
- one and several eligible paths without amplification;
- inherited `None`, `Open`, `Conjectured`, `Supported`, `Settled`, and
  governed `Refuted`;
- legacy raw `Refuted` quarantine;
- v3 resolution-failure and extension-blocker behavior;
- exact outer failure precedence;
- construction, deserialization, mutation, conversion, and reinjection
  barriers;
- no alternate snapshot;
- all v4 canonical vectors;
- v0-v3 compatibility inertia;
- Ticket 0059, 0061, and 0063 vector inertia;
- tour and frozen-chain inertia; and
- a temporary decisive mutation that fails the intended test, is restored,
  and passes again.

Ticket 0064 itself adds no tests or Rust.

## Hostile cases

The normative design contains the required row-by-row hostile table. The
governing summary is:

- unrelated or evidence-smuggled bytes cannot reach the claim relation;
- caller-created relations, lane classes, receipts, clones, and bytes are not
  resolver inputs;
- historical predicate and evidence metadata fail closed across versions;
- bare edges, absence, malformed material, and failures never synthesize
  falsity;
- exact mismatches remain ineligible or failed audit candidates;
- multiplicity never amplifies;
- inherited positive standing defers to future contradiction policy;
- inherited governed `Refuted` is preserved without a second application;
- legacy raw `Refuted` stays quarantined; and
- contradiction debt, invalidation, supersession/currentness,
  Occurrence/Inclusion negative policy, Deadbolt absence, writers, loaders,
  CAS, filesystem, and network ingestion remain absent.

## Changed-path allowlist

Only these exact paths may change in this ticket:

```text
README.md
docs/design/claim-inline-sha256-predicate-v0.md
docs/design/claim-inline-predicate-edge-lanes-v0.md
docs/design/standing-view-evidence-ceilings.md
docs/design/standing-aggregation-independence-groups.md
docs/design/standing-policy-v4-deterministic-direct-refutation-v0.md
docs/design/standing-policy-v4-claim-inline-direct-refutation-v0.md
tickets/0064-standing-policy-v4-claim-inline-direct-refutation-contract.md
```

The Ticket 0056 file itself must not change. The old design may receive only
the narrow successor-status note authorized by this ticket.

## Explicit non-effects

This ticket does not:

- implement policy v4 runtime;
- change policies v0-v3;
- change `policy.rs` or any ceiling;
- make `SupportEligible` standing-bearing;
- reinterpret `sha256_bytes_equals_v0` or `DigestMismatch`;
- create contradiction debt, invalidation, supersession, or currentness;
- infer falsity from absence;
- add an Occurrence/Inclusion negative rule;
- aggregate, vote, score, or amplify;
- change L0, FORMAT, Rust, tests, fixtures, vectors, Cargo, dependencies, CI,
  release metadata, package inventory, the Python verifier, or tour output;
- add `EpistemicGate`, writers, loaders, CAS, filesystem, network, plugin,
  model, or Deadbolt authority; or
- commit, push, publish, or mutate GitHub.

## Validation contract

The documentation patch must run the exact validation ladder requested by the
owner, including all diff checks, formatting, Clippy, workspace tests,
doctests, tour byte/hash check, both frozen-chain checks, release metadata,
package validation (through a temporary VCS-free mirror if the dirty checkout
is refused), independent vector length/hash verification, and a
case-sensitive changed-path union audit.

The requested cold-read path
`crates/magpie-claims/tests/standing_v2.rs` does not exist at the required
base. The actual v2 integration suite is
`crates/magpie-claims/tests/deterministic_support_standing_v2.rs`; it is the
suite read for this contract. This documentation ticket does not rename or
otherwise modify it.

## Same-session review contract

Before handoff, perform three sequential read-only self-review passes:

1. soundness and authority;
2. policy and inheritance; and
3. contract and scope.

These are same-session self-reviews, not independent review. Any valid finding
must be remediated and the affected pass rerun.

## Publication boundary and next slice

Leave this as one complete, validated, uncommitted documentation patch. Do not
commit, push, create or alter a pull request, request reviewers, resolve
threads, merge, or change GitHub metadata.

Ticket 0063 and PR #78 are landed. The standing-inert substrate chain is
complete. Ticket 0064 ratifies only the future policy contract: no policy-v4
runtime exists and no governed `Refuted` result is currently produced by
code. The next separately reviewed slice is policy-v4 runtime.

Contradiction debt, invalidation, supersession/currentness, `EpistemicGate`,
writer, loader, CAS, filesystem, and network ingestion remain future.
