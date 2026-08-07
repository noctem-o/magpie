# Admitted-Contribution Audit v0

## Status

Ratified contract.

Runtime implemented by Ticket 0050.

Standing unchanged.

Support contribution absent.

Ticket 0049 freezes, and Ticket 0050 implements, the standing-inert bridge:

```text
verified replay structure
+
exact evidence artifact identity
+
complete origin-admission audit
+
compiled admitted-contribution policy
->
deterministic admitted-contribution audit
```

This document preserves the complete ratified and implemented boundary. The
runtime resolver, public audit types, fixtures and tests are landed. They add
no support contribution, aggregation or standing change.

`AdmittedContributionV0` remains standing-inert. Ticket 0051 defines the next
positive-input boundary in
[`support-contribution-audit-v0.md`](support-contribution-audit-v0.md).

## Purpose

The landed architecture now provides:

```text
artifact-provenance verification
->
origin-binding verification
->
origin-admission audit
```

`OriginAdmissionAuditV0` answers:

```text
Was this exact contribution assigned to this exact opaque origin group?
```

It does not decide whether the replay-derived graph material is in a supported
contribution lane. In particular, an origin-admission decision does not by
itself establish that:

- the target is a complete standing claim;
- the edge is a `supports` edge;
- the evidence kind and claim domain select a permitted policy cell;
- the candidate ceiling is exactly `Supported`;
- the evidence node names an artifact digest; or
- that asserted digest is the exact artifact identity carried by the admitted
  origin result.

The admitted-contribution audit answers only:

```text
Is this exact replay-derived graph contribution structurally valid,
inside the first supported policy lane, bound to the exact verified
artifact identity, and associated with an admitted origin result?
```

The layer distinctions are normative:

```text
origin admitted
!= contribution admitted

contribution admitted
!= support contribution

support contribution
!= aggregation

aggregation
!= achieved standing
```

## Architectural laws

The central deterministic law is:

```text
H:
the exact completely verified Magpie log prefix carried by
OriginAdmissionReplayContextV0

Pₒ:
the exact compiled origin-admission policy
magpie-origin-admission-v0

P꜀:
the exact compiled admitted-contribution policy
magpie-admitted-contribution-v0

M:
the exact immutable ResolutionContentClosureV0 and its complete
four-field identity

same H + same Pₒ + same P꜀ + same M
->
byte-identical admitted-contribution audit
```

No ambient state participates. The implemented audit performs no second replay,
filesystem scan, CAS lookup, network request, callback, plugin invocation,
environment lookup, mutable-registry lookup or caller-directed search.

The implemented resolver internally derives `OriginAdmissionAuditV0` from the
same private-construction `OriginAdmissionReplayContextV0` and the same
`ResolutionContentClosureV0`. A caller-provided origin-admission audit is audit
output, not authority.

The authority law is:

```text
exact replay-derived graph contribution
+
exact policy-lane validation
+
exact equality with the verified admitted-origin contribution artifact
+
exact OriginAdmitted result
->
AdmittedContributionV0
```

That result establishes only the exact proposition named above. It establishes
no truth, source reputation, statistical independence, support, aggregation or
standing.

## Current substrate and landed runtime

The accepted `StandingReplaySnapshot` contains the unchanged `StandingView`,
whose retained graph tables include:

```text
claims:
BTreeMap<String, StandingClaim>

typed_claims:
BTreeMap<String, TypedClaimNode>

typed_evidence:
BTreeMap<String, TypedEvidenceNode>

justification_edges:
BTreeMap<String, TypedJustificationEdge>
```

The current standing v0 resolver already applies a reviewed fail-closed
candidate order for target existence, typed target existence, edge kind,
claim-domain parsing, source evidence existence, exact scope, evidence-kind
parsing and the support-policy cell. The current policy surfaces establish:

```text
support_ceiling(ExternalSource, ExternalReport)
= Supported

support_context_requirement(ExternalSource, ExternalReport)
= NoPrivilegedContext
```

`NoPrivilegedContext` means only that the standing ceiling surface requires no
existing human, deterministic-verifier or Deadbolt-verifier context for that
cell. It does not admit an origin, create a support contribution or authorize
aggregation. This contract independently requires the exact origin-admission
path defined below.

The implemented `OriginAdmissionReplayContextV0` already co-derives:

```text
one exact verified log-prefix identity
+
one StandingReplaySnapshot
+
one complete origin-binding candidate universe
```

and already exposes:

```rust
pub fn resolve_origin_admission_audit_v0(
    &self,
    closure: &ResolutionContentClosureV0,
) -> OriginAdmissionAuditV0;
```

The implemented origin-admission audit binds the exact verified prefix, the
complete four-field closure identity and the compiled
`magpie-origin-admission-v0` policy. It produces at most one terminal decision
for one exact `ContributionIdentityV0 + OriginComparisonNamespaceV0` key.

Ticket 0050 implements the deterministic graph-facing audit that:

1. enumerates every retained justification edge;
2. applies the exact first admitted-contribution lane;
3. derives an exact `ContributionIdentityV0` from graph material;
4. binds `TypedEvidenceNode.content_hash` to the exact verified artifact
   identity already carried through the origin path;
5. locates the exact internally derived origin decision; and
6. emits complete standing-inert audit output.

## Exact deterministic inputs

The implemented public resolver receives exactly:

```text
self:
one private-construction OriginAdmissionReplayContextV0 carrying H

closure:
one successfully constructed immutable ResolutionContentClosureV0 carrying M
```

The compiled code supplies exactly:

```text
Pₒ:
magpie-origin-admission-v0

P꜀:
magpie-admitted-contribution-v0
```

The implemented audit retains:

```text
self.verified_prefix_identity()

closure.identity()
```

The complete closure identity remains:

```text
ResolutionContentClosureIdentityV0 {
    schema
    canonicalization_profile
    digest_algorithm
    manifest_sha256
}
```

The closure identity binds availability input. It does not prove that any
closure object verified, reconstruct bytes or grant authority.

The resolver must not accept any additional authority input, candidate subset,
policy identity or lookup mechanism.

## Exact compiled policy identities

The implemented runtime uses exactly:

```text
admitted-contribution policy:
magpie-admitted-contribution-v0

audit schema:
magpie-admitted-contribution-audit-v0

audit canonicalization profile:
magpie-admitted-contribution-audit-json-v0

required origin-admission policy:
magpie-origin-admission-v0
```

Reviewed compiled code selects both policy implementations. There is no:

- runtime policy selector;
- policy registry;
- alias;
- negotiation;
- environment override;
- bundle-selected implementation;
- caller-selected implementation; or
- implicit `latest` policy.

An ID serialized into audit output discloses the compiled policy that produced
the result. It does not select code.

## Complete candidate universe

Define:

```text
AdmittedContributionCandidateUniverseV0(H)
=
the complete BTreeMap-ordered justification-edge projection from H
```

The candidate universe is exactly:

```text
every exact TypedJustificationEdge retained in the accepted
StandingReplaySnapshot, ordered by exact edge ID
```

Every retained edge produces exactly one
`AdmittedContributionCandidateAuditV0`, including unsupported, malformed,
incomplete and unfavourable edges.

Enumeration reads the accepted snapshot's actual replay-derived
justification-edge table. Ticket 0050 adds only the smallest crate-private read
seam required to share this table without changing `StandingView` fields,
getters or bytes.

The candidate universe must not begin from:

- origin-admitted decisions;
- caller-provided edge IDs;
- a claim ID;
- closure contents;
- caller-selected evidence labels;
- only `supports` edges;
- only favourable evidence kinds or claim domains; or
- only edges that already appear admissible.

The closure is not a graph index. The implementation must not scan closure
objects to discover contribution candidates.

## Closed v0 policy lane

The first compiled policy admits exactly one positive lane:

```text
edge kind:
supports

source evidence kind:
ExternalSource

target claim domain:
ExternalReport

candidate support ceiling:
Supported

required origin-admission policy:
magpie-origin-admission-v0

artifact algorithm:
sha256
```

The normative positive shape is:

```text
ExternalSource
× ExternalReport
× supports edge
× exact evidence artifact identity
× exact OriginAdmitted decision
->
AdmittedContributionV0
```

This is the minimum substrate for later conservative external-source
corroboration and same-origin non-amplification. It creates no support
contribution and no achieved standing.

V0 explicitly excludes:

```text
ExternalSource × Interpretation
HumanRatification
DeterministicVerification
DeadboltAnchor
ExecutionEvidence
BehavioralEvaluation
ModelSelfReport
LensReadout
refutation contributions
contradicts edges
invalidates edges
```

Those paths have different authority, direction or verifier requirements. Each
requires a separate versioned policy decision.

## Deterministic candidate evaluation

Every edge candidate stops at the first failed check in exactly this order:

```text
1. target StandingClaim exists
2. target TypedClaimNode exists
3. edge kind is exactly supports
4. target claim_domain parses under the existing closed policy grammar
5. source TypedEvidenceNode exists
6. target/evidence/edge scopes are exactly equal
7. evidence_kind parses under the closed EvidenceKind vocabulary
8. evidence kind is exactly ExternalSource
9. claim domain is exactly ExternalReport
10. support_ceiling(ExternalSource, ExternalReport) is exactly Supported
11. TypedEvidenceNode.content_hash is non-empty
12. exact ContributionIdentityV0 and OriginComparisonNamespaceV0 are derived
13. origin-admission completion permits downstream admission
14. one exact origin-admission decision is located for the exact key
15. only OriginAdmitted produces AdmittedContributionV0
```

The implementation reuses the existing reviewed claim-domain parser and
support-candidate evaluation logic through the smallest private shared seam.
The seam preserves every existing standing v0, v1 and v2 byte and behaviour.

The implementation must not add a duplicate public standing engine, a second
looser claim-domain parser or a different closed evidence-kind vocabulary.

The admitted-contribution audit exposes its own closed failure enum because its
serialized output is a separately versioned contract. The exact
`AdmittedContributionCandidateReasonV0` variants are:

```text
MissingTargetClaim
MissingTypedTargetClaim
UnsupportedEdgeKindForV0
MalformedClaimMetadata
MissingClaimDomain
WrongTypeClaimDomain
DuplicateClaimDomainKey
UnknownClaimDomain
MissingSourceEvidence
ScopeMismatch
UnknownEvidenceKind
UnsupportedEvidenceKindForV0
UnsupportedClaimDomainForV0
NoSupportCeiling
CandidateCeilingMismatch
MissingEvidenceContentHash
```

These distinctions preserve the current standing semantics while naming the
new audit's own versioned vocabulary. `DuplicateClaimDomainKey` is the audit
name for the existing fail-closed duplicate-metadata-key parser outcome that
prevents a unique claim-domain result; the private shared parser must not
silently accept metadata that standing v0 rejects.

`NoSupportCeiling` applies only when the exact selected policy cell yields
`None`.

`CandidateCeilingMismatch` applies when that exact cell yields a status other
than exactly `Supported`.

Either result indicates compiled-policy drift and fails closed. The audit must
not silently widen the lane.

No generic Boolean, free-form error string, `Other` variant or caller-defined
reason is permitted.

## Evidence-artifact identity bridge

`TypedEvidenceNode.content_hash` is asserted replay material. Its presence and
L0 validation do not establish external artifact identity by themselves.

The artifact-provenance, origin-binding and origin-admission path independently
verifies one exact artifact identity. Admitted-contribution v0 connects the
graph node to that path only through exact identity equality.

The graph-side requirements are:

```text
TypedEvidenceNode.content_hash
must be non-empty

artifact algorithm
must be exactly sha256
```

The candidate contribution is derived exactly as:

```text
ContributionIdentityV0 {
    target_claim_id:
        edge.target_id

    source_evidence_id:
        edge.source_id

    justification_edge_id:
        exact retained edge ID

    scope_ref:
        exact shared target/evidence/edge scope

    artifact_algorithm:
        sha256

    artifact_digest:
        TypedEvidenceNode.content_hash
}
```

The expected namespace is derived exactly as:

```text
OriginComparisonNamespaceV0 {
    origin_admission_policy_id:
        magpie-origin-admission-v0

    target_claim_id:
        edge.target_id

    scope_ref:
        exact shared scope
}
```

The required equality is:

```text
TypedEvidenceNode.content_hash
==
ContributionIdentityV0.artifact_digest
on the exact replay-derived OriginAdmitted decision
```

Therefore:

```text
asserted content_hash
+
exact equality with verified admitted-origin contribution artifact
=
graph node and governed provenance path refer to the same exact bytes
```

The asserted hash gains no standalone authority. Authority arises only when
the complete graph-derived contribution identity and namespace exactly match
the complete contribution identity and namespace carried by the internally
derived `OriginAdmitted` result.

Fail-closed laws:

```text
empty content_hash
->
not admitted

different digest
->
different ContributionIdentityV0
->
no exact admitted-origin match
->
not admitted
```

The implementation must not define partial identity matching. It must not
search for a nearby decision with the same claim, evidence or edge but a
different artifact digest.

Artifact identity must not be inferred from:

- source URI;
- locator;
- quote hash;
- evidence summary;
- filename;
- metadata prose;
- acquisition locator;
- origin group; or
- publisher identity.

The bridge establishes byte identity only. It does not establish that the
evidence is true, relevant, reputable, fresh or independent.

## Origin-admission composition

The implemented public entry point is exactly:

```rust
impl OriginAdmissionReplayContextV0 {
    pub fn resolve_admitted_contribution_audit_v0(
        &self,
        closure: &ResolutionContentClosureV0,
    ) -> AdmittedContributionAuditV0;
}
```

The implemented method internally calls:

```rust
self.resolve_origin_admission_audit_v0(closure)
```

or one exact private shared implementation seam that produces the same
`OriginAdmissionAuditV0`.

The resolver must not accept:

```text
OriginAdmissionAuditV0
AdmittedOriginV0
OriginAdmissionDecisionV0
candidate edge list
candidate contribution list
claim ID
policy ID
authority table
trusted Boolean
caller-created snapshot
caller-created anchor index
serialized audit
callback
plugin
filesystem path
CAS reference
network client
environment
```

A cloned or serialized `OriginAdmissionAuditV0` remains audit output. It is not
an authority token.

The same replay carrier and closure must determine:

```text
verified-prefix identity
closure identity
origin-admission audit
admitted-contribution audit
```

There is no transposition seam where an origin audit from another history,
closure or policy may be paired with this graph.

## Global completion law

The implemented completion vocabulary remains:

```rust
pub enum AdmittedContributionAuditCompletionV0 {
    Complete,

    OriginAdmissionIncomplete {
        unavailable_binding_selectors:
            Vec<ArtifactProvenanceAnchorSelectorV0>,
    },
}
```

The completion result is derived exactly from the internally resolved
`OriginAdmissionAuditCompletionV0`.

Mapping:

```text
OriginAdmissionAuditCompletionV0::Complete
->
AdmittedContributionAuditCompletionV0::Complete

OriginAdmissionAuditCompletionV0::IncompleteCandidateUniverse {
    unavailable_binding_selectors
}
->
AdmittedContributionAuditCompletionV0::OriginAdmissionIncomplete {
    unavailable_binding_selectors
}
```

The unavailable selector vector must be retained in exact origin-admission
order.

The global law is:

```text
origin admission incomplete
->
admitted_contributions is empty globally
```

Every graph candidate audit remains visible. Candidates that fail a graph,
metadata or policy check retain their earlier exact
`PolicyIneligible { reason }` disposition.

A candidate that passes every graph, lane, ceiling and content-hash check
receives `OriginAdmissionIncomplete`.

No candidate may use an available local origin result while the complete
origin-binding candidate universe is unavailable.

## Candidate dispositions

Freeze:

```rust
pub enum AdmittedContributionCandidateDispositionV0 {
    PolicyIneligible {
        reason: AdmittedContributionCandidateReasonV0,
    },

    OriginAdmissionIncomplete,

    OriginDecisionAbsent,

    OriginNotAdmitted {
        decision: OriginAdmissionDecisionV0,
    },

    Admitted {
        admitted_contribution: AdmittedContributionV0,
    },
}
```

For a graph-policy-eligible candidate and a complete origin audit, the
implemented resolver locates a decision only by:

```text
ContributionIdentityV0
+
OriginComparisonNamespaceV0
```

Outcomes are exact:

```text
no exact decision
->
OriginDecisionAbsent

EligibleBindingUnresolved
->
OriginNotAdmitted with that exact nested decision

ConflictingBindings
->
OriginNotAdmitted with that exact nested decision

OriginAdmitted
->
Admitted
```

There is at most one terminal origin-admission decision for one exact key.

`OriginNotAdmitted` may contain only `EligibleBindingUnresolved` or
`ConflictingBindings`. It must not reinterpret either decision and must not
contain a winning origin group.

Decision selection must not use:

- group string alone;
- claim ID alone;
- evidence ID alone;
- edge ID alone;
- scope alone;
- artifact digest alone;
- first or last decision;
- newest occurrence;
- lexical winner; or
- caller order.

## Admitted contribution result

Freeze this exact conceptual field order:

```rust
pub struct AdmittedContributionV0 {
    policy_id: String,
    origin_admission_policy_id: String,
    contribution: ContributionIdentityV0,
    namespace: OriginComparisonNamespaceV0,
    origin_group: String,
    evidence_kind: String,
    claim_domain: String,
    candidate_ceiling: Status,
    supporting_origin_candidate_selectors:
        Vec<ArtifactProvenanceAnchorSelectorV0>,
}
```

Exact v0 values are:

```text
policy_id:
magpie-admitted-contribution-v0

origin_admission_policy_id:
magpie-origin-admission-v0

evidence_kind:
ExternalSource

claim_domain:
ExternalReport

candidate_ceiling:
Supported
```

`origin_group` and `supporting_origin_candidate_selectors` must come only from
the exact internally derived `AdmittedOriginV0`.

`AdmittedContributionV0` means only:

```text
this exact graph contribution
is structurally valid
inside the exact v0 lane
refers to the exact verified artifact identity
and has one admitted origin-group assignment
```

It does not mean:

```text
the evidence is true
the source is reputable
the claim is supported
the group is statistically independent
the contribution has been counted
standing has changed
```

Carrying the upstream v0 `origin_group` across this typed boundary preserves
deterministic compatibility data. It does not authenticate the group, create
grouping authority, or independently establish source identity, and the value
does not become authority-bound because it crossed a typed audit boundary.
The upstream group originates from claimant-label compatibility admission;
the future authority-bound successor requires its own additive types and
complete `H + P + M + A` coordinates under ADR-0007.

## Public audit surface

The runtime exposes exactly these six public types:

```text
AdmittedContributionAuditCompletionV0
AdmittedContributionCandidateReasonV0
AdmittedContributionCandidateDispositionV0
AdmittedContributionCandidateAuditV0
AdmittedContributionV0
AdmittedContributionAuditV0
```

All public structs have:

```text
private fields
Clone
Debug
PartialEq
Eq
Serialize
read-only getters
no Deserialize
no Default
no public constructor
```

All closed enums have:

```text
Clone
Debug
PartialEq
Eq
Serialize
no Deserialize
no Default
```

Enums serialize exactly with:

```rust
#[serde(
    tag = "outcome",
    content = "details",
    rename_all = "snake_case"
)]
```

Only the top-level audit exposes `canonical_bytes()`.

The candidate audit has this exact conceptual field order:

```rust
pub struct AdmittedContributionCandidateAuditV0 {
    edge_id: String,
    source_evidence_id: String,
    target_claim_id: String,
    evidence_kind: Option<String>,
    claim_domain: Option<String>,
    candidate_ceiling: Option<Status>,
    candidate_contribution: Option<ContributionIdentityV0>,
    candidate_namespace: Option<OriginComparisonNamespaceV0>,
    disposition: AdmittedContributionCandidateDispositionV0,
}
```

Field semantics are exact:

- `edge_id`, `source_evidence_id` and `target_claim_id` are always retained
  from the candidate edge;
- `evidence_kind` is populated only after the source evidence exists;
- `claim_domain` is populated only after exact parsing succeeds;
- `candidate_ceiling` is populated only after exact evidence-kind and
  claim-domain parsing and only when the candidate reaches the selected policy
  cell check;
- `candidate_contribution` and `candidate_namespace` are populated only after
  every structural, lane, ceiling and evidence-content-hash check passes; and
- every optional field serializes explicitly as its exact value or JSON
  `null`.

The candidate audit must not include raw metadata JSON, artifact bytes,
foreign-bundle bytes, claim statement text, evidence summary, rationale,
source URI, locator or quote text.

The top-level audit has this exact conceptual field order:

```rust
pub struct AdmittedContributionAuditV0 {
    schema: String,
    canonicalization_profile: String,
    policy_id: String,
    origin_admission_policy_id: String,
    verified_prefix_identity: VerifiedLogPrefixIdentityV0,
    closure_identity: ResolutionContentClosureIdentityV0,
    completion: AdmittedContributionAuditCompletionV0,
    candidate_audits: Vec<AdmittedContributionCandidateAuditV0>,
    admitted_contributions: Vec<AdmittedContributionV0>,
}
```

Its fixed string values are:

```text
schema:
magpie-admitted-contribution-audit-v0

canonicalization_profile:
magpie-admitted-contribution-audit-json-v0

policy_id:
magpie-admitted-contribution-v0

origin_admission_policy_id:
magpie-origin-admission-v0
```

Only this top-level type exposes:

```rust
pub fn canonical_bytes(&self) -> Vec<u8>;
```

## Deterministic ordering and canonical bytes

Freeze:

```text
candidate audits:
exact justification-edge ID order from StandingView's BTreeMap

admitted contributions:
exact ContributionIdentityV0 order

supporting origin selectors:
exact selector order inherited from OriginAdmissionAuditV0

unavailable binding selectors:
exact selector order inherited from OriginAdmissionAuditV0
```

The implementation derives admitted-contribution order structurally through a
`BTreeMap` keyed by exact `ContributionIdentityV0`; it does not repair an
order-sensitive fold after the fact.

No caller order, closure insertion order, anchor occurrence multiplicity,
origin-group lexical preference or write recency selects an outcome.

The top-level method must serialize directly from the typed structure:

```rust
serde_json::to_vec(self)
```

Determinism comes from exact field declaration order, adjacently tagged closed
enums, explicitly serialized optional fields and the ordered vectors above.

This profile is not JCS, RFC 8785, generic canonical JSON, map sorting or a new
generic Magpie canonical profile. It is purpose-built deterministic derived
audit JSON.

Raw artifact and foreign-bundle bytes must not appear in the audit.

## Multiplicity and non-amplification

The admitted-contribution audit preserves exact contribution multiplicity:

```text
two distinct admitted contributions
assigned to the same admitted origin group
->
two AdmittedContributionV0 values
```

It must preserve:

- each exact edge;
- each exact evidence node;
- each exact artifact identity;
- each exact contribution identity; and
- each exact admitted origin assignment.

It must not collapse distinct contributions by origin group.

The layer boundary is:

```text
origin-binding duplicate collapse
belongs to OriginAdmissionAuditV0

same-origin contribution non-amplification
belongs to policy v3 aggregation

AdmittedContributionAuditV0
preserves every exact admitted contribution
```

Later aggregation alone may apply:

```text
many admitted contributions
from one admitted origin group
->
at most one same-origin aggregation contribution
```

This contract defines no origin-group count, threshold, quorum, score or
achieved status.

## Authority and serialization boundary

Every value in the admitted-contribution audit is derived audit output.
No serialized, cloned or caller-assembled value may substitute for the replay
context and closure.

The resolver authority boundary remains:

```text
LogReader
->
replay_origin_admission_context_v0
->
OriginAdmissionReplayContextV0
->
resolve_admitted_contribution_audit_v0(closure)
```

Private construction prevents a caller from transposing:

- one snapshot onto another prefix identity;
- one origin audit onto another closure;
- one admitted origin onto another contribution;
- one policy output onto another policy; or
- one serialized audit onto live replay authority.

Ticket 0050 includes compile-fail documentation proving:

- no public struct literal construction;
- no `Deserialize` for any public audit type;
- no resolver on `StandingView`;
- no complete resolver on a standalone `StandingReplaySnapshot`;
- no caller candidate list;
- no caller origin audit or decision input;
- no caller policy or authority input; and
- no serialized or cloned audit substitution.

## Implemented fixtures

Ticket 0050 adds literal canonical fixtures for:

```text
admitted external-source contribution

exact graph candidate with no origin decision

origin-binding conflict

origin-binding eligible unresolved

globally incomplete origin-admission universe

two distinct contributions assigned to the same admitted origin group
```

The final fixture contains two distinct `AdmittedContributionV0` values.
Ticket 0049 itself added no fixture files, canonical byte lengths or hashes.

## Implemented hostile tests

Ticket 0050 proves:

```text
valid ExternalSource × ExternalReport supports edge
+ non-empty evidence content hash
+ exact OriginAdmitted result
->
Admitted

same graph path with no origin binding
->
OriginDecisionAbsent
->
not admitted

origin conflict
->
exact nested ConflictingBindings
->
not admitted

origin eligible unresolved
->
exact nested EligibleBindingUnresolved
->
not admitted

origin audit globally incomplete
->
every graph candidate remains visible
->
zero admitted contributions globally

wrong edge kind
->
PolicyIneligible

ExternalSource × Interpretation
->
outside v0

non-ExternalSource evidence
->
outside v0

missing target claim
->
exact failure

missing typed target
->
exact failure

missing source evidence
->
exact failure

scope mismatch
->
exact failure

malformed, missing, wrong-type, duplicate or unknown claim_domain
->
distinct exact failures

empty evidence content_hash
->
not admitted

evidence content_hash differing from the verified origin contribution artifact
->
no exact contribution match
->
not admitted

two distinct edges over the same artifact and same admitted group
->
two admitted contributions

two distinct artifacts assigned to the same admitted group
->
two admitted contributions

same textual group in different namespaces
->
no relationship

serialized or cloned OriginAdmissionAuditV0
->
cannot substitute for replay and closure

caller-selected candidate list
->
no public API

same H + Pₒ + P꜀ + M
->
byte-identical audit

equivalent closure input order
->
byte-identical audit

different H with equal graph/origin projections
->
different verified-prefix identity and audit

audit execution
->
standing v0/v1/v2 bytes unchanged
->
support and refutation ceilings unchanged
->
origin-admission audit bytes unchanged
```

Tests also pin the exact first-failure order and verify that candidate
optional fields become non-null only at the specified stages.

## Frozen surfaces

Ticket 0049 freezes unchanged:

```text
Magpie L0 format
payload tags
canonical event encoding
signatures
hashing
genesis
LogStore
LogReader
LogWriter
VerifiedReplaySummary
StandingView fields and bytes
StandingReplaySnapshot fields and bytes
standing v0
standing v1
standing v2
standing policy IDs
support ceilings
refutation ceilings
support-context requirements
deterministic verifier contexts
Deadbolt verifier contexts
artifact-provenance verifier
resolution-content closure
origin-binding verifier
OriginAdmissionReplayContextV0
OriginAdmissionAuditV0
OriginAdmissionDecisionV0
origin-admission fixture bytes
Cargo manifests
dependencies
lockfile
release metadata
Deadbolt
```

Ticket 0050 adds only the six standing-inert audit types, the exact compiled
constants, the context method, fixtures and tests, and the smallest private
shared seams required to preserve existing semantics.

It must not mutate the existing origin-admission audit or any standing surface.

## Explicit non-goals

This contract does not define or implement:

- support contribution;
- achieved support;
- standing policy v3;
- origin-group counting;
- corroboration thresholds;
- statistical independence;
- source reputation;
- publisher identity;
- trust scores;
- numeric confidence;
- probabilistic aggregation;
- `ExternalSource × Interpretation`;
- any non-`ExternalSource` admitted-contribution lane;
- `HumanRatification` admission;
- deterministic-verifier contribution admission;
- Deadbolt contribution admission;
- refutation contribution;
- contradiction debt;
- invalidation;
- supersession or currentness;
- quote-hash verification;
- source-locator validation;
- source-standing propagation;
- graph traversal;
- transitive provenance;
- `EpistemicGate`;
- ordinary claim-bearing writer;
- loader;
- CAS;
- filesystem lookup;
- network lookup;
- callback or plugin lookup;
- new L0 payload; or
- Deadbolt changes.

## Implemented sequence and next boundary

Ticket 0050 implements this contract in this order:

1. add the six private-construction public audit types and exact constants;
2. expose the smallest crate-private shared claim-domain/candidate evaluation
   seam that preserves standing v0/v1/v2 results and bytes;
3. enumerate the complete replay-derived justification-edge universe in exact
   edge-ID order;
4. internally resolve `OriginAdmissionAuditV0` from the same context and
   closure;
5. apply the exact first-failure order and graph-to-artifact identity bridge;
6. index terminal origin decisions by exact contribution plus namespace;
7. apply the global completion law and exact candidate dispositions;
8. order admitted contributions by exact `ContributionIdentityV0`;
9. serialize only the top-level typed audit with `serde_json::to_vec`; and
10. add the required literal fixtures, hostile tests and compile-fail boundary
    tests.

The remaining sequence is:

```text
support-contribution contract ratified by Ticket 0051
->
support-contribution audit runtime
->
standing policy v3 contract
->
conservative aggregation
```

Ticket 0049 did not ratify or implement a later stage. Ticket 0051 separately
ratifies only the next positive-input boundary.

## Reviewer checklist

- Confirm status is ratified contract, runtime implemented by Ticket 0050,
  support contribution absent and standing unchanged.
- Confirm the deterministic input is exactly `H + Pₒ + P꜀ + M`.
- Confirm both policies are compiled exact identities with no runtime selector.
- Confirm every retained justification edge produces exactly one audit in
  exact edge-ID order.
- Confirm no caller subset, closure scan or admitted-origin-first enumeration
  exists.
- Confirm the only positive lane is
  `supports × ExternalSource × ExternalReport × Supported × sha256`.
- Confirm the first-failure order and closed reason vocabulary are exact.
- Confirm the existing claim-domain parser is shared privately rather than
  duplicated or loosened.
- Confirm `content_hash` remains asserted material and gains authority only
  through exact equality with the verified admitted-origin contribution
  artifact.
- Confirm contribution and namespace matching are complete and exact.
- Confirm the implemented resolver internally derives origin admission from
  the same context and closure.
- Confirm a caller-provided or serialized origin audit cannot substitute.
- Confirm global incomplete origin admission yields zero admitted
  contributions while preserving all candidate audits.
- Confirm only exact `OriginAdmitted` yields `AdmittedContributionV0`.
- Confirm the six public types, field order, traits and enum tagging are exact.
- Confirm canonical bytes come directly from the typed top-level structure.
- Confirm distinct contributions sharing one origin group remain distinct.
- Confirm same-origin non-amplification remains policy v3 aggregation
  behavior.
- Confirm no support, aggregation, standing, writer, loader, CAS, network,
  filesystem, L0, Cargo, dependency or Deadbolt change is claimed.
