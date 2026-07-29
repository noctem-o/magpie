# Standing Policy v4 Claim-Inline Direct Refutation v0

## Status

Ratified by documentation-only Ticket 0064. The Tickets 0057-0063
standing-inert substrate is landed, including Ticket 0063 through merged
PR #78. No policy-v4 runtime exists in this patch, and current code produces
no governed `Refuted` result through this rule.

The next separately reviewed slice is the policy-v4 runtime. Contradiction
debt, invalidation, supersession/currentness, `EpistemicGate`, writers,
loaders, CAS, filesystem, and network ingestion remain future.

## Exact classification

```text
policy:
magpie-claims-standing-v4

rule:
sha256_claim_inline_bytes_direct_refutation_v0

predicate:
sha256_claim_inline_bytes_equals_v0

domain:
ExactMachineCheckable

evidence:
DeterministicVerification

edge:
contradicts

relation:
DigestUnequal

private lane:
RefutationEligible

candidate ceiling:
Some(Refuted)

permitted achieved effect:
one governed Refuted result

runtime in this ticket:
none
```

This is one closed claim-inline deterministic direct-refutation policy rule.
It does not create a general negative-evidence framework.

## Purpose

Tickets 0057-0063 close the exact subject-binding defect that blocked the
first direct-refutation candidate. They establish:

1. one immutable claim-owned inline subject and one claim-owned expected
   digest;
2. one neutral claim-only SHA-256 relation;
3. one routing-only deterministic-verification attestation binding;
4. one exact edge-binding and relation/polarity matrix; and
5. an opaque, standing-inert `RefutationEligible` lane with candidate ceiling
   `Refuted`.

This contract ratifies the smallest policy step that may turn that exact
internally derived lane into achieved governed `Refuted`. It preserves the
separation:

```text
replay substrate != admission authority
candidate classification != governed standing
public audit output != replay authority
```

## Historical boundary: Ticket 0056 remains blocked

Ticket 0056 remains a blocked historical record. Its exact counterexample was:

```text
claim:
sha256_bytes_equals_v0:
ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad

true supporting witness_hex:
616263

attacker-selected contradicting witness_hex:
756e72656c61746564206d6174657269616c

attacker-selected digest:
d42273bca408c671996c4f6eea5efefd010450655a44d742b0a2ba055b0542dc
```

The contradicting evidence selected unrelated bytes (`unrelated material`).
Its mismatch said only that those evidence-selected bytes did not equal the
claim's expected digest. The claim did not independently identify the checked
subject, so that mismatch could not bear negative authority.

Ticket 0064 does not repair or revive that rule:

- `sha256_bytes_equals_v0` retains its historical evidence-relative positive
  meaning;
- `DeterministicVerifierContextTraceV0::DigestMismatch` remains
  non-authority;
- the Ticket 0056 rule identity remains withdrawn;
- every Ticket 0056 negative trace name and conflict blocker remains
  withdrawn and is not inherited;
- the Ticket 0056 negative receipt and resolution shapes are not inherited;
  and
- no historical finding or identity is rewritten.

This contract uses only the different versioned proposition
`sha256_claim_inline_bytes_equals_v0`:

```text
SHA-256(exact decoded claim-owned inline subject bytes)
==
exact claim-owned expected_sha256
```

Both operands are resolved from the exact claim in one verified replay
snapshot. Evidence cannot supply either operand. Ticket 0061 attestation
metadata contains only:

```text
schema
predicate_id
subject_claim_id
scope_ref
```

Subject bytes, expected digest, computed digest, and relation are unknown
attestation fields. The Ticket 0056 unrelated-byte value therefore has no
input position in the new rule.

Reuse of the sequential policy identity `magpie-claims-standing-v4` is a new
ratification over a complete replacement substrate. It is not retroactive
ratification of Ticket 0056.

## Inherited laws

This contract inherits without modification:

- append-only replay and first-write-wins typed node and edge identity;
- complete governed-standing v0, v1, v2, and v3 semantics;
- legacy raw standing quarantine;
- explicit policy selection, never ambient or latest-policy selection;
- the two exact policy-v3 context resolver signatures and internal derivation
  path;
- the Ticket 0057 claim-inline subject-binding law;
- the Ticket 0059 claim-only predicate failure order, relation, opacity, and
  vectors;
- the Ticket 0061 routing-only attestation failure order, opacity, and vectors;
- the Ticket 0063 edge-lane failure order, four-cell matrix, opacity, and
  vectors;
- deterministic `BTreeMap<String, ...>` edge ordering;
- `refutation_ceiling(DeterministicVerification,
  ExactMachineCheckable) == Some(Refuted)`;
- support policy v2 and its evidence-relative `sha256_bytes_equals_v0`
  predicate;
- support policy v3 and its exact complete support-audit composition; and
- all frozen tour, chain, L0, FORMAT, release, and package behavior.

The exact landed policy-v3 public methods are:

```rust
impl OriginAdmissionReplayContextV0 {
    pub fn resolved_standing_v3(
        &self,
        claim_id: &str,
        closure: &ResolutionContentClosureV0,
    ) -> Option<Status>;

    pub fn resolved_standing_with_trace_v3(
        &self,
        claim_id: &str,
        closure: &ResolutionContentClosureV0,
    ) -> StandingResolutionV3;
}
```

The trace resolver internally derives:

```text
self.snapshot().resolved_standing_with_trace_v2(claim_id)
+
self.resolve_support_contribution_audit_v0(closure)
+
requested claim scope from the same snapshot
+
self.verified_prefix_identity()
+
closure.identity()
→ one crate-private policy-v3 composition input
```

Policy v3 preserves inherited `Settled`, governed `Refuted`, and `Supported`;
otherwise its one qualifying external-report corroboration application may
produce `Supported`, never `Settled`. Its outer identities, complete inherited
v2 object, support audit, failure, lane, application, ignored material, and
blockers remain intact inside v4.

## Normative distinctions

The following inequalities are permanent:

```text
RefutationEligible != governed Refuted
candidate Refuted ceiling != achieved Refuted
DigestUnequal != refutation
contradicts edge != refutation
binding success != refutation
malformed or missing material != refutation
public audit output != replay authority
legacy raw Refuted != governed Refuted
absence != falsity
```

`RefutationEligible` is an internally derived candidate classification. It
becomes policy-bearing only through this exact v4 rule and precedence.

## Central policy law

Ratify exactly:

```text
one complete internally derived StandingResolutionV3
+ one exact verified replay context
+ one requested claim
+ zero or more internally enumerated exact contradicts edges targeting that claim
+ one claim-owned subject resolution for the policy call
+ one claim-owned DigestEqual or DigestUnequal relation for the policy call
+ exact same-snapshot attestation binding per candidate evidence
+ exact edge source, target, and scope binding
+ RefutationEligible for at least one candidate
+ compiled refutation ceiling exactly Some(Refuted)
+ no conservative inherited-standing blocker
→ one direct-refutation policy application
→ governed Refuted
```

Every element except the requested `claim_id` and immutable closure is
internally replay-derived. The closure is an explicit content input only
because policy v4 inherits the complete policy-v3 result. The direct-refutation
lane does not read closure objects, closure bytes, foreign bundle bytes, or
artifact bytes.

The exact rule is:

```rust
StandingPolicyRuleV4::Sha256ClaimInlineBytesDirectRefutationV0
```

Its serialized spelling is:

```text
sha256_claim_inline_bytes_direct_refutation_v0
```

No other v4 rule is ratified.

## Exact future public API

The only future public resolver surface is:

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

The scalar method returns exactly
`resolved_standing_with_trace_v4(...).governed_standing()`.

There is:

- no free function;
- no `StandingView` method;
- no `StandingReplaySnapshot` v4 method;
- no public policy composer;
- no caller-selected policy ID;
- no latest-policy selector; and
- no second standing engine.

The resolver accepts no caller-provided:

- `StandingResolutionV3`;
- lane outcome or receipt;
- predicate outcome or receipt;
- attestation outcome or receipt;
- evidence or edge object;
- evidence or edge ID list;
- candidate list;
- relation;
- ceiling;
- status;
- threshold;
- precedence table;
- callback;
- canonical or serialized bytes;
- second snapshot;
- verified-prefix identity; or
- audit object.

The complete v3 result is internally derived through the same
`OriginAdmissionReplayContextV0` and closure on every v4 call.

## Public audit is never policy authority

These values are output-only:

- `Sha256ClaimInlineBytesPredicateOutcomeV0`;
- `Sha256ClaimInlineBytesPredicateReceiptV0`;
- `InlinePredicateAttestationBindingOutcomeV0`;
- `InlinePredicateAttestationBindingReceiptV0`;
- `ClaimInlinePredicateEdgeLaneOutcomeV0`;
- `ClaimInlinePredicateEdgeLaneReceiptV0`;
- their canonical bytes;
- their clones; and
- their inspection enums.

The future runtime must not call a public resolver and inspect its public
outcome to make a policy decision. It must not deserialize, parse, hash-match,
or compare public audit bytes as an authority shortcut.

An implementation may produce equivalent Ticket 0059, 0061, or 0063 audit
output beside the private resolved value. The policy decision must be built
from the private value, not from that output.

## Smallest private composition seam

The future implementation may introduce only the smallest crate-private
family composition needed to:

1. derive complete v3 internally;
2. enumerate the exact replayed candidate edges;
3. call `resolve_claim_inline_subject_v0` at most once;
4. call the claim-owned SHA-256 relation evaluator at most once;
5. retain the relation in one crate-private resolved value;
6. call `resolve_inline_predicate_attestation_after_claim_v0` separately for
   each candidate evidence node;
7. validate each exact replayed edge's source, target, and scope;
8. classify each candidate with the Ticket 0063 matrix;
9. consult the exact refutation-ceiling cell only after a candidate is
   otherwise `RefutationEligible`; and
10. privately construct the v4 result.

The claim-owned relation is shared across the candidate family. Evidence
binding is not shared: each evidence node must independently bind after claim
resolution.

The attestation module must not gain access to subject bytes, expected digest,
computed digest, relation, candidate ceiling, edge polarity, or standing.

A sound implementation must stop for a new contract if it requires changing
Ticket 0059, 0061, or 0063 public APIs, opacity, failure vocabularies, or
vectors.

## Candidate universe

The negative candidate universe is derived from replayed justification edges,
never caller IDs:

```text
for each exact replayed edge in existing edge-ID order:
    retain only if edge.edge_kind bytes == "contradicts"
    retain only if edge.target_id bytes == requested claim_id bytes
    candidate.edge_id = exact replayed map key
    candidate.evidence_id = exact edge.source_id
```

The existing `StandingView` uses a first-write-wins
`BTreeMap<String, TypedJustificationEdge>`. Therefore candidate order is exact
Rust `String::Ord` order of raw edge IDs. It is deterministic,
case-sensitive, and equivalent to raw UTF-8 lexicographic order for valid
Rust strings.

There is no:

- support-edge reinterpretation;
- edge alias or namespace;
- compatible-edge search;
- newest, best, or first-successful selection;
- fuzzy identity;
- Unicode normalization;
- trimming;
- case folding; or
- caller-selected evidence.

Mirroring the established standing candidate model, unrelated target edges
are ignored and have no trace entry. `supports` and unknown edge kinds are
also outside the negative universe and have no v4 trace entry. Repeated edge
IDs retain the first replayed edge only; distinct edge IDs remain distinct
candidates.

The production filter makes `MissingEdge`, `UnsupportedEdgeKind`,
`EdgeSourceBindingMismatch`, and `EdgeTargetBindingMismatch` defensive
invariants rather than ordinary reachable production outcomes. The runtime
must still test them through crate-private staging because a refactor must
fail closed. `EdgeScopeBindingMismatch`, binding failures, the one
ineligibility, and eligible classifications remain ordinary candidate
outcomes.

## Candidate derivation and failure independence

After the v3 extension-safety gates, candidate derivation is:

1. enumerate and order the negative universe;
2. if it is empty, preserve inherited standing without resolving the
   claim-inline subject;
3. resolve the claim-inline subject once;
4. if claim resolution fails, retain one exact nested Ticket 0063 failure for
   each candidate, without treating the failure as falsity;
5. otherwise evaluate the one claim-owned relation once;
6. bind each evidence node separately;
7. validate exact edge identity and scope;
8. classify the relation/edge cell; and
9. consult the `Refuted` ceiling only for an otherwise eligible candidate.

One malformed or missing candidate does not prevent another independent
candidate from being eligible. Candidate failure is audit-only. It never
becomes an outer failure or global blocker.

For production negative candidates, the only ineligible cell is:

```text
DigestEqual × contradicts
→ Ineligible(DigestEqualDoesNotRefute)
```

The eligible cell is:

```text
DigestUnequal × contradicts
× refutation_ceiling(
    DeterministicVerification,
    ExactMachineCheckable,
  ) == Some(Refuted)
→ RefutationEligible
```

If the ceiling drifts, every otherwise eligible candidate is instead
`ResolutionFailed(RefutationCeilingInvariantMismatch)` and no application is
created.

## Complete standing precedence

Precedence runs after v3 derivation and before or after candidate derivation as
specified below.

### 1. Outer identity failure

Validate in this exact first-failure order:

1. inherited v3 `claim_id` equals requested `claim_id`;
2. inherited v3 `policy_id` equals `MAGPIE_CLAIMS_POLICY_V3_ID`;
3. inherited v3 verified-prefix identity equals the calling context's exact
   verified-prefix identity; and
4. inherited v3 closure identity equals `closure.identity()`.

The first mismatch returns the corresponding v4 outer failure. Every one is
identity-invalid for v4 and therefore:

```text
governed_standing = None
legacy_raw_standing = None
currentness = Unknown
candidate_trace = []
applications = []
blockers = []
```

The exact requested claim ID, expected context prefix identity, expected
closure identity, and complete inherited v3 audit object remain present.
Apparently valid standing from the mismatched inherited object is never
borrowed.

### 2. Inherited v3 not safe to extend

If the identity-valid inherited v3 object has `resolution_failure.is_some()`:

```text
preserve inherited governed standing, legacy raw standing, and currentness
candidate_trace = []
applications = []
blockers = [InheritedV3ResolutionFailed]
```

Otherwise inspect inherited v3 blockers in their existing order.
`SupportAuditIncomplete`, `SupportAuditRejected`, and
`InheritedV2ResolutionFailed` are extension-unsafe. Preserve inherited values,
perform no candidate derivation, and emit one
`InheritedV3NotExtensionSafe { blocker }` containing the first such blocker.
The complete inherited v3 object retains any later blocker.

`InsufficientDistinctOriginGroups` is not extension-unsafe. It means the
complete v3 support audit had too few groups for the v3 support rule. It
cannot veto this independent exact negative lane.

### 3. Inherited governed Refuted

For identity-valid, extension-safe inherited governed `Refuted`, derive and
retain candidate audit but preserve `Refuted`. Create no v4 application and
no new blocker. This avoids a second achieved result and any amplification.

### 4. Inherited positive standing

For identity-valid, extension-safe inherited governed `Supported` or
`Settled`:

- with no eligible candidate, preserve inherited standing;
- with one or more eligible candidates, preserve inherited standing, create
  no application, and emit exactly:

```text
InheritedPositiveStandingRequiresContradictionPolicy {
    inherited_status: Supported | Settled
}
```

This is conservative deferral. It does not assert that contradiction debt
already exists and does not choose a winner.

### 5. Permitted direct refutation

For identity-valid, extension-safe inherited `None`, `Open`, or `Conjectured`
and at least one eligible candidate:

```text
applications = [
  Sha256ClaimInlineBytesDirectRefutationV0
  → Some(Refuted)
]
governed_standing = Some(Refuted)
```

There is exactly one application regardless of eligible-candidate count.

### 6. No eligible candidate

Preserve inherited standing. Candidate failures and ineligible candidates do
not aggregate, vote, or synthesize a result.

### Reachability notation

The current claim-inline graph normally inherits `Conjectured` from v0.
`None`, `Open`, `Supported`, `Settled`, and governed `Refuted` are nevertheless
required composer cases because the inherited v3 contract permits or can
defensively stage them. Tests must label staged-only cases rather than imply
that current ordinary replay reaches every state.

Legacy raw `Refuted` is separately reachable but remains quarantined. It is
copied only into the audit field and never consulted by precedence.

## Multiplicity and mutual exclusion

One eligible path is sufficient. Several eligible paths:

- remain separate trace entries in edge-ID order;
- yield one application;
- yield the same single governed `Refuted`;
- do not increase confidence, score, weight, or standing;
- do not create quorum, majority, or aggregation; and
- do not create contradiction debt.

Repeated resolver calls and cloned or serialized audit output do not amplify.
Ineligible and failed candidates never combine into authority.

`supports` and `contradicts` edges may coexist. The policy call resolves one
claim-owned relation:

| Relation | `supports` lane if separately queried | v4 `contradicts` candidate |
| --- | --- | --- |
| `DigestEqual` | `SupportEligible` | `Ineligible(DigestEqualDoesNotRefute)` |
| `DigestUnequal` | `Ineligible(DigestUnequalDoesNotSupport)` | `RefutationEligible` |

Both polarities cannot be eligible for the same claim-fixed relation. This is
matrix mutual exclusion, not contradiction debt.

## Exact public contract types

The future implementation must expose this semantic surface and field order.
Equivalent Rust syntax is permitted only if it preserves every name, field,
getter, serde shape, invariant, and opacity rule below.

```rust
pub const MAGPIE_CLAIMS_POLICY_V4_ID: &str =
    "magpie-claims-standing-v4";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StandingPolicyRuleV4 {
    Sha256ClaimInlineBytesDirectRefutationV0,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StandingPolicyContextV4 {
    UncontestedRefutationEligible,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StandingTraceClassificationV4 {
    RefutationEligible,
    Ineligible,
    ResolutionFailed,
}

#[derive(Clone, Serialize)]
pub struct StandingTraceEntryV4 {
    edge_id: String,
    evidence_id: String,
    classification: StandingTraceClassificationV4,
    ineligible_reason:
        Option<ClaimInlinePredicateEdgeLaneIneligibleReasonV0>,
    failure: Option<ClaimInlinePredicateEdgeLaneFailureV0>,
}

#[derive(Clone, Serialize)]
pub struct StandingPolicyApplicationV4 {
    rule: StandingPolicyRuleV4,
    context: StandingPolicyContextV4,
    achieved_standing: Option<Status>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StandingBlockerV4 {
    InheritedV3ResolutionFailed,
    InheritedV3NotExtensionSafe {
        blocker: StandingBlockerV3,
    },
    InheritedPositiveStandingRequiresContradictionPolicy {
        inherited_status: Status,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StandingResolutionFailureV4 {
    InheritedClaimMismatch,
    InheritedPolicyMismatch,
    VerifiedPrefixIdentityMismatch,
    ClosureIdentityMismatch,
}

#[derive(Clone, Serialize)]
pub struct StandingResolutionV4 {
    claim_id: String,
    governed_standing: Option<Status>,
    legacy_raw_standing: Option<Status>,
    currentness: StandingCurrentness,
    policy_id: &'static str,
    verified_prefix_identity: VerifiedLogPrefixIdentityV0,
    closure_identity: ResolutionContentClosureIdentityV0,
    inherited_v3: StandingResolutionV3,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolution_failure: Option<StandingResolutionFailureV4>,
    candidate_trace: Vec<StandingTraceEntryV4>,
    applications: Vec<StandingPolicyApplicationV4>,
    blockers: Vec<StandingBlockerV4>,
}
```

### Trace invariants

Private construction permits exactly:

| `classification` | `ineligible_reason` | `failure` |
| --- | --- | --- |
| `RefutationEligible` | `None` | `None` |
| `Ineligible` | `Some(DigestEqualDoesNotRefute)` | `None` |
| `ResolutionFailed` | `None` | `Some(exact Ticket 0063 failure)` |

`DigestUnequalDoesNotSupport` is a valid Ticket 0063 inspection token but is
not constructible in a production v4 negative trace because support edges are
outside the negative universe.

### Application invariants

Every v4 application is exactly:

```text
rule = Sha256ClaimInlineBytesDirectRefutationV0
context = UncontestedRefutationEligible
achieved_standing = Some(Refuted)
```

`applications.len()` is zero or one. It never names a winning edge; the
complete candidate trace is the ordered audit basis.

### Blocker invariants

`InheritedV3NotExtensionSafe` may contain only:

```text
StandingBlockerV3::SupportAuditIncomplete
StandingBlockerV3::SupportAuditRejected
StandingBlockerV3::InheritedV2ResolutionFailed
```

It must never contain `InsufficientDistinctOriginGroups`.

`InheritedPositiveStandingRequiresContradictionPolicy` may contain only
`Supported` or `Settled`.

### Read-only getters

The exact getter surface is:

```rust
impl StandingTraceEntryV4 {
    pub fn edge_id(&self) -> &str;
    pub fn evidence_id(&self) -> &str;
    pub fn classification(&self) -> StandingTraceClassificationV4;
    pub fn ineligible_reason(
        &self,
    ) -> Option<ClaimInlinePredicateEdgeLaneIneligibleReasonV0>;
    pub fn failure(&self)
        -> Option<&ClaimInlinePredicateEdgeLaneFailureV0>;
}

impl StandingPolicyApplicationV4 {
    pub fn rule(&self) -> &StandingPolicyRuleV4;
    pub fn context(&self) -> &StandingPolicyContextV4;
    pub fn achieved_standing(&self) -> Option<Status>;
}

impl StandingResolutionV4 {
    pub fn claim_id(&self) -> &str;
    pub fn governed_standing(&self) -> Option<Status>;
    pub fn legacy_raw_standing(&self) -> Option<Status>;
    pub fn currentness(&self) -> StandingCurrentness;
    pub fn policy_id(&self) -> &str;
    pub fn verified_prefix_identity(
        &self,
    ) -> &VerifiedLogPrefixIdentityV0;
    pub fn closure_identity(
        &self,
    ) -> &ResolutionContentClosureIdentityV0;
    pub fn inherited_v3(&self) -> &StandingResolutionV3;
    pub fn resolution_failure(
        &self,
    ) -> Option<&StandingResolutionFailureV4>;
    pub fn candidate_trace(&self) -> &[StandingTraceEntryV4];
    pub fn applications(&self) -> &[StandingPolicyApplicationV4];
    pub fn blockers(&self) -> &[StandingBlockerV4];
    pub fn canonical_bytes(&self) -> Vec<u8>;
}
```

There are no public constructors, setters, mutable getters, consuming
extractors, or staging helpers.

### Opacity and one-way audit

`StandingTraceEntryV4`, `StandingPolicyApplicationV4`, and
`StandingResolutionV4` have private fields and private construction. They
derive only `Clone` and one-way `Serialize`. They have no:

- `Deserialize`;
- `Default`;
- public struct literal construction;
- public constructor;
- `From` or `TryFrom` authority conversion;
- `Into`/consuming authority extraction;
- public mutation;
- public test-only staging;
- resolver reinjection;
- `Debug`, `Display`, or `Error`;
- `PartialEq`, `Eq`, `PartialOrd`, or `Ord`; or
- `Hash`.

The public closed enums may be caller-constructed, cloned, compared, and
serialized as inert inspection vocabulary. No resolver or composer accepts
them.

## Outer failure and blocker law

The exact outer failure order and wire spellings are:

| Order | Variant | `kind` |
| --- | --- | --- |
| 1 | `InheritedClaimMismatch` | `inherited_claim_mismatch` |
| 2 | `InheritedPolicyMismatch` | `inherited_policy_mismatch` |
| 3 | `VerifiedPrefixIdentityMismatch` | `verified_prefix_identity_mismatch` |
| 4 | `ClosureIdentityMismatch` | `closure_identity_mismatch` |

The blocker spellings are:

| Variant | Wire |
| --- | --- |
| `InheritedV3ResolutionFailed` | `{"kind":"inherited_v3_resolution_failed"}` |
| `InheritedV3NotExtensionSafe { blocker }` | `{"kind":"inherited_v3_not_extension_safe","blocker":<exact StandingBlockerV3>}` |
| `InheritedPositiveStandingRequiresContradictionPolicy { inherited_status }` | `{"kind":"inherited_positive_standing_requires_contradiction_policy","inherited_status":"Supported"}` or `{"kind":"inherited_positive_standing_requires_contradiction_policy","inherited_status":"Settled"}` |

The complete inherited v3 object is the source of detailed inherited failure
and blocker audit. V4 does not duplicate those nested facts into parallel
failure payloads.

The categories remain distinct:

```text
outer failure
!= candidate failure
!= ineligible candidate
!= blocker
!= no eligible candidate
```

## Canonical policy audit profile

The exact profile identity is:

```text
magpie-claims-standing-v4-resolution-json-v0
```

It is deterministic derived audit data. It is not L0, an event format, a
portable generic canonical-JSON claim, or deserializable authority.

The future encoder uses direct typed fixed-field serde representation and
`serde_json::to_vec`. It must not use:

- `serde_json::Value`;
- `BTreeMap` or `HashMap` wire objects;
- JCS/RFC 8785;
- a generic canonical JSON abstraction;
- Magpie L0; or
- a deserializable policy type.

### Exact top-level field order

```text
claim_id
governed_standing
legacy_raw_standing
currentness
policy_id
verified_prefix_identity
closure_identity
inherited_v3
resolution_failure (omitted when None)
candidate_trace
applications
blockers
```

The complete inherited v3 object is nested at `inherited_v3` in its existing
canonical field order. No inherited-v3 fact is flattened or duplicated.

### Trace, application, and blocker order

- `candidate_trace` is always present and follows exact candidate edge-ID
  order.
- Every trace entry field order is `edge_id`, `evidence_id`,
  `classification`, `ineligible_reason`, `failure`.
- `ineligible_reason` and `failure` are always present and serialize as
  explicit `null` when absent.
- `applications` is always present and has length zero or one.
- Application field order is `rule`, `context`, `achieved_standing`.
- `blockers` is always present. V4 emits at most one blocker under this
  contract.
- Outer-failure results have empty candidate, application, and blocker arrays.

### Scalar and escaping law

- UTF-8, compact, no insignificant whitespace;
- no BOM;
- no trailing LF or CRLF;
- valid non-ASCII scalar values remain UTF-8;
- `"` and `\` are escaped;
- JSON control characters use serde_json's exact short or `\u00xx` escapes;
- `/` is not escaped;
- no Unicode normalization;
- no key sorting beyond declared typed field order; and
- `Option<Status>::None` is `null`, while absent
  `resolution_failure` is omitted.

## Frozen future-runtime vectors

These are contract vectors over the ratified future wire design. They are not
observed runtime output, because Ticket 0064 adds no runtime.

Every vector uses a complete, identity-valid staged v3 result except vector I.
The common verified-prefix identity is event count `7` with a 64-character
`1` tip. The common closure identity uses a 64-character `2` manifest digest.
The v3 support audit is complete and empty, so the inherited v3 object retains
`InsufficientDistinctOriginGroups { distinct_group_count: 0 }`; that blocker
is intentionally extension-safe. The inherited v2 status equals the
inherited v3 status. These staged objects are valid under the complete v3
composer law.

Each literal is exactly the single line inside its code fence, excluding the
fence newline.

| Vector | Bytes | Lowercase SHA-256 |
| --- | ---: | --- |
| A. no candidate, inherited `Open` | 2,197 | `3292f6ceafc1dbd16666cce56f2bdac6de7d8de5faeb36cc4495fe272e243d9a` |
| B. one eligible, inherited `Open` | 2,466 | `755aa9761363346847d54a8250cf9c82a5d0c917cf40fe250e611cc42d8e57c5` |
| C. `DigestEqual + contradicts` | 2,340 | `02b8aec71af0b4e21370ad8ba01b1c5796600e6702df1881bbf36ddd58f7e5be` |
| D. nested binding failure | 2,366 | `8ae9ce1ca675a954e8b01e315a2be6f94f393db4defa88aae87bbd410c816637` |
| E. repeated eligible candidates | 2,593 | `b1a77cb96b628a30e5e8b1d70a075835d69abbe9f07e6275b6cf43e30dc79db5` |
| F. inherited `Supported` | 2,437 | `cab622ea9e552e14e49a5b302d0cbfca0cae778644841c64fd42343f8b049e09` |
| G. inherited `Settled` | 2,429 | `40b633838e23754522e51a3e31f9f915537eb109b4c14087772ebceec09dfadd` |
| H. inherited governed `Refuted` | 2,332 | `59e179505b97022dd4e85b49f5a24d922f39fcbfa91bc56feca861389e50f71b` |
| I. outer claim mismatch | 2,258 | `ac1dd8a308aaf4f93dec01152409350d47f8b5008174d998de52962f4a52c800` |

### A. No candidate, inherited `Open`, unchanged

```json
{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v4","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v3":{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v3","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v2":{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[],"blockers":[]},"support_contribution_audit":{"schema":"magpie-support-contribution-audit-v0","canonicalization_profile":"magpie-support-contribution-audit-json-v0","policy_id":"magpie-support-contribution-v0","admitted_contribution_policy_id":"magpie-admitted-contribution-v0","origin_admission_policy_id":"magpie-origin-admission-v0","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"completion":{"outcome":"complete"},"support_contributions":[]},"lane":null,"application":null,"ignored_contributions":[],"blockers":[{"kind":"insufficient_distinct_origin_groups","distinct_group_count":0}]},"candidate_trace":[],"applications":[],"blockers":[]}
```

Exact byte length: `2,197`.

Lowercase SHA-256:
`3292f6ceafc1dbd16666cce56f2bdac6de7d8de5faeb36cc4495fe272e243d9a`.

### B. One `RefutationEligible` candidate, inherited `Open`, governed `Refuted`

```json
{"claim_id":"claim-c","governed_standing":"Refuted","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v4","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v3":{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v3","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v2":{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[],"blockers":[]},"support_contribution_audit":{"schema":"magpie-support-contribution-audit-v0","canonicalization_profile":"magpie-support-contribution-audit-json-v0","policy_id":"magpie-support-contribution-v0","admitted_contribution_policy_id":"magpie-admitted-contribution-v0","origin_admission_policy_id":"magpie-origin-admission-v0","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"completion":{"outcome":"complete"},"support_contributions":[]},"lane":null,"application":null,"ignored_contributions":[],"blockers":[{"kind":"insufficient_distinct_origin_groups","distinct_group_count":0}]},"candidate_trace":[{"edge_id":"edge-a","evidence_id":"evidence-a","classification":"refutation_eligible","ineligible_reason":null,"failure":null}],"applications":[{"rule":"sha256_claim_inline_bytes_direct_refutation_v0","context":{"kind":"uncontested_refutation_eligible"},"achieved_standing":"Refuted"}],"blockers":[]}
```

Exact byte length: `2,466`.

Lowercase SHA-256:
`755aa9761363346847d54a8250cf9c82a5d0c917cf40fe250e611cc42d8e57c5`.

### C. `DigestEqual + contradicts`, ineligible, inherited unchanged

```json
{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v4","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v3":{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v3","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v2":{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[],"blockers":[]},"support_contribution_audit":{"schema":"magpie-support-contribution-audit-v0","canonicalization_profile":"magpie-support-contribution-audit-json-v0","policy_id":"magpie-support-contribution-v0","admitted_contribution_policy_id":"magpie-admitted-contribution-v0","origin_admission_policy_id":"magpie-origin-admission-v0","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"completion":{"outcome":"complete"},"support_contributions":[]},"lane":null,"application":null,"ignored_contributions":[],"blockers":[{"kind":"insufficient_distinct_origin_groups","distinct_group_count":0}]},"candidate_trace":[{"edge_id":"edge-a","evidence_id":"evidence-a","classification":"ineligible","ineligible_reason":"digest_equal_does_not_refute","failure":null}],"applications":[],"blockers":[]}
```

Exact byte length: `2,340`.

Lowercase SHA-256:
`02b8aec71af0b4e21370ad8ba01b1c5796600e6702df1881bbf36ddd58f7e5be`.

### D. Nested binding failure, inherited unchanged

```json
{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v4","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v3":{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v3","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v2":{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[],"blockers":[]},"support_contribution_audit":{"schema":"magpie-support-contribution-audit-v0","canonicalization_profile":"magpie-support-contribution-audit-json-v0","policy_id":"magpie-support-contribution-v0","admitted_contribution_policy_id":"magpie-admitted-contribution-v0","origin_admission_policy_id":"magpie-origin-admission-v0","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"completion":{"outcome":"complete"},"support_contributions":[]},"lane":null,"application":null,"ignored_contributions":[],"blockers":[{"kind":"insufficient_distinct_origin_groups","distinct_group_count":0}]},"candidate_trace":[{"edge_id":"edge-a","evidence_id":"evidence-a","classification":"resolution_failed","ineligible_reason":null,"failure":{"attestation_binding_failed":"missing_evidence"}}],"applications":[],"blockers":[]}
```

Exact byte length: `2,366`.

Lowercase SHA-256:
`8ae9ce1ca675a954e8b01e315a2be6f94f393db4defa88aae87bbd410c816637`.

### E. Repeated eligible candidates, one application

```json
{"claim_id":"claim-c","governed_standing":"Refuted","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v4","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v3":{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v3","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v2":{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[],"blockers":[]},"support_contribution_audit":{"schema":"magpie-support-contribution-audit-v0","canonicalization_profile":"magpie-support-contribution-audit-json-v0","policy_id":"magpie-support-contribution-v0","admitted_contribution_policy_id":"magpie-admitted-contribution-v0","origin_admission_policy_id":"magpie-origin-admission-v0","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"completion":{"outcome":"complete"},"support_contributions":[]},"lane":null,"application":null,"ignored_contributions":[],"blockers":[{"kind":"insufficient_distinct_origin_groups","distinct_group_count":0}]},"candidate_trace":[{"edge_id":"edge-a","evidence_id":"evidence-a","classification":"refutation_eligible","ineligible_reason":null,"failure":null},{"edge_id":"edge-z","evidence_id":"evidence-z","classification":"refutation_eligible","ineligible_reason":null,"failure":null}],"applications":[{"rule":"sha256_claim_inline_bytes_direct_refutation_v0","context":{"kind":"uncontested_refutation_eligible"},"achieved_standing":"Refuted"}],"blockers":[]}
```

Exact byte length: `2,593`.

Lowercase SHA-256:
`b1a77cb96b628a30e5e8b1d70a075835d69abbe9f07e6275b6cf43e30dc79db5`.

### F. Inherited `Supported` plus eligible candidate, conservative blocker

```json
{"claim_id":"claim-c","governed_standing":"Supported","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v4","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v3":{"claim_id":"claim-c","governed_standing":"Supported","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v3","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v2":{"claim_id":"claim-c","governed_standing":"Supported","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[],"blockers":[]},"support_contribution_audit":{"schema":"magpie-support-contribution-audit-v0","canonicalization_profile":"magpie-support-contribution-audit-json-v0","policy_id":"magpie-support-contribution-v0","admitted_contribution_policy_id":"magpie-admitted-contribution-v0","origin_admission_policy_id":"magpie-origin-admission-v0","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"completion":{"outcome":"complete"},"support_contributions":[]},"lane":null,"application":null,"ignored_contributions":[],"blockers":[{"kind":"insufficient_distinct_origin_groups","distinct_group_count":0}]},"candidate_trace":[{"edge_id":"edge-a","evidence_id":"evidence-a","classification":"refutation_eligible","ineligible_reason":null,"failure":null}],"applications":[],"blockers":[{"kind":"inherited_positive_standing_requires_contradiction_policy","inherited_status":"Supported"}]}
```

Exact byte length: `2,437`.

Lowercase SHA-256:
`cab622ea9e552e14e49a5b302d0cbfca0cae778644841c64fd42343f8b049e09`.

### G. Inherited `Settled` plus eligible candidate, conservative blocker

```json
{"claim_id":"claim-c","governed_standing":"Settled","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v4","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v3":{"claim_id":"claim-c","governed_standing":"Settled","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v3","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v2":{"claim_id":"claim-c","governed_standing":"Settled","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[],"blockers":[]},"support_contribution_audit":{"schema":"magpie-support-contribution-audit-v0","canonicalization_profile":"magpie-support-contribution-audit-json-v0","policy_id":"magpie-support-contribution-v0","admitted_contribution_policy_id":"magpie-admitted-contribution-v0","origin_admission_policy_id":"magpie-origin-admission-v0","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"completion":{"outcome":"complete"},"support_contributions":[]},"lane":null,"application":null,"ignored_contributions":[],"blockers":[{"kind":"insufficient_distinct_origin_groups","distinct_group_count":0}]},"candidate_trace":[{"edge_id":"edge-a","evidence_id":"evidence-a","classification":"refutation_eligible","ineligible_reason":null,"failure":null}],"applications":[],"blockers":[{"kind":"inherited_positive_standing_requires_contradiction_policy","inherited_status":"Settled"}]}
```

Exact byte length: `2,429`.

Lowercase SHA-256:
`40b633838e23754522e51a3e31f9f915537eb109b4c14087772ebceec09dfadd`.

### H. Inherited governed `Refuted`, preserved and non-amplifying

```json
{"claim_id":"claim-c","governed_standing":"Refuted","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v4","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v3":{"claim_id":"claim-c","governed_standing":"Refuted","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v3","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v2":{"claim_id":"claim-c","governed_standing":"Refuted","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[],"blockers":[]},"support_contribution_audit":{"schema":"magpie-support-contribution-audit-v0","canonicalization_profile":"magpie-support-contribution-audit-json-v0","policy_id":"magpie-support-contribution-v0","admitted_contribution_policy_id":"magpie-admitted-contribution-v0","origin_admission_policy_id":"magpie-origin-admission-v0","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"completion":{"outcome":"complete"},"support_contributions":[]},"lane":null,"application":null,"ignored_contributions":[],"blockers":[{"kind":"insufficient_distinct_origin_groups","distinct_group_count":0}]},"candidate_trace":[{"edge_id":"edge-a","evidence_id":"evidence-a","classification":"refutation_eligible","ineligible_reason":null,"failure":null}],"applications":[],"blockers":[]}
```

Exact byte length: `2,332`.

Lowercase SHA-256:
`59e179505b97022dd4e85b49f5a24d922f39fcbfa91bc56feca861389e50f71b`.

### I. Representative outer identity failure

```json
{"claim_id":"claim-c","governed_standing":null,"legacy_raw_standing":null,"currentness":"unknown","policy_id":"magpie-claims-standing-v4","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v3":{"claim_id":"claim-other","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v3","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v2":{"claim_id":"claim-other","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[],"blockers":[]},"support_contribution_audit":{"schema":"magpie-support-contribution-audit-v0","canonicalization_profile":"magpie-support-contribution-audit-json-v0","policy_id":"magpie-support-contribution-v0","admitted_contribution_policy_id":"magpie-admitted-contribution-v0","origin_admission_policy_id":"magpie-origin-admission-v0","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"completion":{"outcome":"complete"},"support_contributions":[]},"lane":null,"application":null,"ignored_contributions":[],"blockers":[{"kind":"insufficient_distinct_origin_groups","distinct_group_count":0}]},"resolution_failure":{"kind":"inherited_claim_mismatch"},"candidate_trace":[],"applications":[],"blockers":[]}
```

Exact byte length: `2,258`.

Lowercase SHA-256:
`ac1dd8a308aaf4f93dec01152409350d47f8b5008174d998de52962f4a52c800`.

## Vector validation requirement

The future runtime must reproduce each literal byte-for-byte. Ticket 0064
validation independently counts and hashes the exact literals with Python and
PowerShell/.NET. JSON parsing may be used only as a validation check; parsing
and reserialization are not the canonical source.

Any field reorder, omitted array, changed null, enum rename, changed inherited
v3 placement, extra newline, BOM, or escaping drift changes the vector and
must fail.

## Hostile contract cases

The table states production behavior. “Outside universe” means no v4 candidate
trace entry. “Preserve” means preserve identity-valid inherited governed
standing, legacy raw standing, and currentness unless an outer-failure row
states otherwise.

| Hostile case | Candidate classification | Failure or blocker | Application | Standing and why no synthetic refutation |
| --- | --- | --- | --- | --- |
| Exact Ticket 0056 unrelated-byte attack | Relation remains claim-owned; unrelated evidence bytes are unread | none if routing metadata is otherwise exact; historical witness metadata fails as below | only if the actual claim-owned relation and exact new lane qualify | Attacker bytes have no input position, so they cannot manufacture inequality |
| Evidence attempts to smuggle `subject_hex` or bytes | `ResolutionFailed` | `AttestationBindingFailed(UnknownAttestationField)` | none | preserve; attestation cannot select a subject |
| Caller-created `DigestUnequal` token | no candidate effect | none | none | preserve; token is not a resolver input |
| Caller-created `RefutationEligible` token | no candidate effect | none | none | preserve; token is inert inspection vocabulary |
| Caller-created lane receipt | no candidate effect | none | none | preserve; receipt is not accepted |
| Serialized or cloned audit output | no candidate effect | none | none | preserve; bytes and clones are output-only |
| Public predicate, binding, or lane outcome reinjection | no candidate effect | compile-time/API rejection | none | preserve; no resolver overload accepts it |
| Wrong predicate family on claim | per candidate `ResolutionFailed` | nested claim `UnknownPredicateId` or `UnknownClaimMetadataKey` | none | preserve; versions fail closed |
| Historical `verification_witness` evidence metadata | per candidate `ResolutionFailed` | `AttestationBindingFailed(UnknownAttestationMetadataKey)` | none | preserve; old and new evidence families do not cross-dispatch |
| Bare exact `contradicts` edge | failed or ineligible until every claim, binding, relation, edge, and ceiling condition succeeds | exact first nested failure if material is incomplete | none unless independently eligible | an edge alone is not refutation |
| `DigestEqual + contradicts` | `Ineligible(DigestEqualDoesNotRefute)` | none | none | preserve; equality cannot refute its own proposition |
| `DigestUnequal + supports` | outside v4 negative universe; standing-inert Ticket 0063 query is `Ineligible(DigestUnequalDoesNotSupport)` | none | none | preserve; support is never negative authority |
| Missing claim with one exact candidate edge | `ResolutionFailed` | `AttestationBindingFailed(ClaimPredicateResolutionFailed(MissingClaim))` | none | preserve; missing is not false |
| Missing evidence | `ResolutionFailed` | `AttestationBindingFailed(MissingEvidence)` | none | preserve; missing evidence is not false |
| Malformed claim metadata | `ResolutionFailed` | exact nested Ticket 0059 failure, beginning with `MalformedMetadata` where applicable | none | preserve; parse failure is not inequality |
| Malformed attestation metadata | `ResolutionFailed` | `AttestationBindingFailed(MalformedAttestationMetadata)` | none | preserve |
| Predicate mismatch | `ResolutionFailed` | `AttestationBindingFailed(PredicateBindingMismatch)` | none | preserve |
| Claim mismatch in attestation | `ResolutionFailed` | `AttestationBindingFailed(ClaimBindingMismatch)` | none | preserve |
| Evidence-scope mismatch | `ResolutionFailed` | `AttestationBindingFailed(EvidenceScopeBindingMismatch)` | none | preserve |
| Claim-scope mismatch | `ResolutionFailed` | `AttestationBindingFailed(ClaimScopeBindingMismatch)` | none | preserve |
| Edge-source mismatch | unreachable after exact production derivation; staged private classification is `ResolutionFailed` | `EdgeSourceBindingMismatch` | none | preserve; derivation uses `edge.source_id` exactly |
| Edge-target mismatch | outside production universe; staged private classification is `ResolutionFailed` | `EdgeTargetBindingMismatch` | none | preserve; wrong target is never borrowed |
| Edge-scope mismatch | `ResolutionFailed` | `EdgeScopeBindingMismatch` | none | preserve |
| Unknown edge kind | outside production universe; staged private classification is `ResolutionFailed` | `UnsupportedEdgeKind` in defensive test | none | preserve; no compatible-kind search |
| Missing edge after enumeration | unreachable after production enumeration; staged private classification is `ResolutionFailed` | `MissingEdge` | none | preserve; no caller edge ID exists |
| Repeated candidate edge ID | first replayed edge only | none beyond its one classification | at most one | first-write-wins; repetition cannot amplify |
| Distinct repeated candidate edges | separate ordered entries | exact per-candidate result | zero or one total | candidate count does not change achieved standing |
| Multiple evidence nodes | separate per-edge binding | one failure does not veto another | zero or one total | no aggregation or majority |
| Inherited `Supported` plus eligible | `RefutationEligible` retained | `InheritedPositiveStandingRequiresContradictionPolicy(Supported)` | none | preserve `Supported`; contradiction policy is future |
| Inherited `Settled` plus eligible | `RefutationEligible` retained | `InheritedPositiveStandingRequiresContradictionPolicy(Settled)` | none | preserve `Settled`; never demote |
| Inherited governed `Refuted` plus eligible | `RefutationEligible` retained | none | none | preserve one `Refuted`; no second result or amplification |
| Legacy raw `Refuted` only | classify independently of raw value | none from raw value | only if governed precedence independently permits | raw value remains quarantined |
| Inherited v3 resolution failure | no candidate derivation | `InheritedV3ResolutionFailed` | none | preserve inherited v3 result; incomplete inheritance cannot authorize |
| Inherited v3 insufficient groups | derive candidates normally | no v4 blocker | per ordinary precedence | v3 no-support is complete and does not veto an independent negative lane |
| Inherited v3 support audit incomplete | no candidate derivation | `InheritedV3NotExtensionSafe(SupportAuditIncomplete)` | none | preserve; incomplete composition cannot authorize extension |
| Inherited v3 support audit rejected | no candidate derivation | `InheritedV3NotExtensionSafe(SupportAuditRejected)` | none | preserve |
| Inherited v2 failure recorded by v3 | no candidate derivation | `InheritedV3NotExtensionSafe(InheritedV2ResolutionFailed)` | none | preserve |
| Verified-prefix mismatch | no candidate derivation | outer `VerifiedPrefixIdentityMismatch` | none | top-level governed and raw standing clear to `None`; mixed replays cannot authorize |
| Closure mismatch | no candidate derivation | outer `ClosureIdentityMismatch` | none | top-level governed and raw standing clear to `None`; closure is v3-only explicit input |
| Inherited claim mismatch | no candidate derivation | outer `InheritedClaimMismatch` | none | top-level authority clears; mismatched achieved standing is not borrowed |
| Inherited policy mismatch | no candidate derivation | outer `InheritedPolicyMismatch` | none | top-level authority clears; no latest-policy coercion |
| Unsupported claim domain | per candidate `ResolutionFailed` | nested `ClaimDomainMismatch` or exact earlier Ticket 0059 failure | none | preserve; this rule is only `ExactMachineCheckable` |
| Unsupported evidence kind | `ResolutionFailed` | `AttestationBindingFailed(WrongEvidenceKind)` | none | preserve |
| Refutation-ceiling drift | otherwise eligible candidate becomes `ResolutionFailed` | `RefutationCeilingInvariantMismatch` | none | preserve; candidate ceiling is a checked invariant |
| Candidate absence | no trace entry | none | none | preserve; absence is not falsity |
| Inference from missing material | failed or absent, never eligible | exact failure if a candidate exists | none | preserve; open-world absence has no negative meaning |
| Occurrence/Inclusion negative claim | not eligible; claim resolver fails domain binding for this rule | `ClaimDomainMismatch` where queried | none | preserve; no Occurrence/Inclusion negative rule is ratified |
| Deadbolt anchor absence | outside this rule | none from anchor absence | none | preserve; absence of an anchor does not refute |
| Contradiction-debt proposal | no additional classification | positive-standing blocker when applicable | none | preserve; debt is not implemented |
| Invalidation proposal | outside rule | none | none | preserve; no invalidation authority |
| Supersession/currentness proposal | outside rule | none | none | preserve inherited currentness; no currentness inference |

## Future runtime test matrix

The separately reviewed runtime ticket must add focused tests for:

- exact `MAGPIE_CLAIMS_POLICY_V4_ID` and rule spelling;
- exact context, trace-classification, blocker, and outer-failure spellings;
- complete nested v3 retention and exact getters;
- the two explicit context resolver signatures;
- absence of any latest-policy or ambient-policy alias;
- one subject resolution and one digest evaluation per policy call;
- zero subject resolution when the candidate universe is empty;
- exact edge filter and raw edge-ID order;
- first-write-wins duplicate edge IDs;
- support, unrelated-target, and unknown-kind exclusion;
- every Ticket 0063 relation/edge cell through the standing-inert resolver;
- the production negative cells through the v4 private family seam;
- every Ticket 0063 top-level failure through private defensive staging;
- all 28 Ticket 0061 reasons and all 33 Ticket 0059 reasons nested exactly;
- `DigestEqual + contradicts` ineligibility;
- `DigestUnequal` without exact `contradicts` candidate;
- wrong evidence, source, target, scope, and kind;
- refutation-ceiling drift;
- candidate failure independence;
- one eligible path;
- several eligible paths with one application and no amplification;
- repeated evidence and repeated calls;
- inherited `None`, `Open`, `Conjectured`, `Supported`, `Settled`, and governed
  `Refuted`;
- explicit staged-state reachability labels;
- legacy raw `Refuted` quarantine;
- inherited v3 resolution-failure gate;
- each extension-unsafe v3 blocker and their first-blocker order;
- `InsufficientDistinctOriginGroups` remaining extension-safe;
- exact four-stage outer failure precedence, including adjacent double faults;
- outer failure clearing top-level governed/raw authority;
- no `Deserialize`, `Default`, public construction, mutation, consuming
  conversion, debug, equality, ordering, hash, or resolver reinjection for
  authority-bearing structs;
- inert constructible enum tokens not being accepted;
- no second snapshot;
- no closure bytes in the direct lane;
- all nine canonical vectors;
- UTF-8, escaping, normalization, BOM, and newline behavior;
- v0, v1, v2, and v3 output inertia;
- Ticket 0059, 0061, and 0063 vector inertia;
- policy-ceiling matrix inertia;
- tour output inertia; and
- both frozen-chain vectors.

The implementation ticket must also perform one temporary mutation that
changes a decisive eligibility or precedence law, demonstrates the intended
test failure, restores the exact contract, and passes again. Acceptable
mutations include allowing `DigestEqual + contradicts`, overriding inherited
`Supported`, or producing one application per eligible candidate. The
mutation must not be committed.

Ticket 0064 itself adds no Rust or tests.

## Compatibility and explicit non-effects

This contract preserves:

- `MAGPIE_CLAIMS_POLICY_ID`, v1, v2, and v3 identities and behavior;
- v2 `Sha256BytesEqualsDirectSupportV0`;
- the historical evidence-relative predicate and `DigestMismatch` law;
- v3 complete inheritance, outer failure order, blockers, grouping, and
  candidate ordering;
- Ticket 0059 claim-owned predicate public API and vectors;
- Ticket 0061 attestation public API and vectors;
- Ticket 0063 lane public API and vectors;
- `SupportEligible` as standing-inert;
- all existing ceiling values;
- raw standing quarantine;
- L0 and FORMAT;
- fixtures, tests, Cargo, dependency, CI, release, package, tour, and verifier
  surfaces; and
- writer, loader, CAS, filesystem, network, plugin, model, and Deadbolt
  behavior.

This contract does not:

- implement policy v4;
- alter `policy.rs`;
- introduce a generic policy composer;
- consume public audit values as authority;
- expose claim subject or relation material to attestation;
- change closure construction or read closure bytes in the negative lane;
- create governed support;
- make `SupportEligible` standing-bearing;
- create contradiction debt;
- resolve a contradiction;
- demote `Settled` or override `Supported`;
- revive a legacy raw `Refuted`;
- infer falsity from failure or absence;
- add an Occurrence/Inclusion negative rule;
- infer currentness;
- invalidate or supersede;
- aggregate, vote, score, rank, or amplify;
- add `EpistemicGate`;
- add writers, loading, CAS, filesystem, or network ingestion; or
- change GitHub metadata.

## Implementation stop conditions

Stop for a new contract if implementation would require:

- evidence-selected subject bytes;
- reinterpretation of `sha256_bytes_equals_v0`;
- consuming any public outcome, receipt, inspection enum, clone, or bytes as
  authority;
- a caller-provided candidate list;
- a caller-provided v3 result;
- mixed replay contexts or an alternate snapshot;
- nondeterministic candidate enumeration;
- policy changes in `policy.rs`;
- changes to Ticket 0059, 0061, or 0063 public opacity or vectors;
- closure material in the direct-refutation lane;
- overriding `Supported` or demoting `Settled`;
- designing contradiction debt to apply the rule;
- an Occurrence/Inclusion negative rule;
- absence-as-falsity;
- aggregation, voting, scoring, or confidence;
- a public constructor or deserializable authority type;
- an unspecified canonical byte shape; or
- runtime, tests, fixtures, Cargo, L0, or FORMAT changes in Ticket 0064.

## Reviewer checklist

- [ ] Ticket 0056 remains blocked and its exact unrelated-byte attack has no
      input path.
- [ ] The new predicate and rule identities are fresh.
- [ ] `sha256_bytes_equals_v0` and `DigestMismatch` remain unchanged.
- [ ] Subject and expected digest are claim-owned.
- [ ] One claim resolution and relation are shared per policy call.
- [ ] Each evidence binding remains separate.
- [ ] No public audit outcome, receipt, clone, enum, or bytes are policy input.
- [ ] V3 is internally derived and completely retained from the same context
      and closure.
- [ ] Candidate filtering and raw edge-ID order match existing replay.
- [ ] Supports, unrelated targets, and unknown kinds are not negative
      candidates.
- [ ] `RefutationEligible` remains distinct from governed `Refuted`.
- [ ] Candidate ceiling remains distinct from achieved standing.
- [ ] Candidate failures are independent and never outer failures.
- [ ] Incomplete/rejected v3 composition is extension-blocking, while
      insufficient groups is not.
- [ ] `Supported` and `Settled` use conservative deferral.
- [ ] Governed `Refuted` is preserved without a second application.
- [ ] Legacy raw `Refuted` remains quarantined.
- [ ] Several eligible candidates create one application and no amplification.
- [ ] No contradiction debt, invalidation, supersession/currentness, or
      absence-as-falsity appears.
- [ ] Public type fields, getters, opacity, and exact enum spellings are fully
      specified.
- [ ] All nine literals, lengths, and SHA-256 values reproduce independently.
- [ ] Only the Ticket 0064 changed-path allowlist is touched.

## Runtime handoff

Ticket 0064 ratifies policy only. The future runtime slice may add the exact
context methods, private composition seam, closed types, tests, and canonical
vectors specified here. It must be separately reviewed.

Until that runtime lands:

```text
policy-v4 runtime:
absent

governed Refuted from claim-inline direct refutation:
absent

SupportEligible standing policy:
absent

contradiction debt and conflict resolution:
absent
```
