# Support-Contribution Audit v0

## Status

Ratified contract.

Runtime implementation absent.

Support contributions absent.

Aggregation absent.

Standing unchanged.

Ticket 0051 freezes the standing-inert boundary:

```text
internally derived AdmittedContributionAuditV0
+
exact upstream composition validation
+
one compiled support-contribution policy
->
deterministic SupportContributionAuditV0
```

## Purpose

The landed architecture now provides:

```text
verified replay
->
artifact-provenance verification
->
origin-binding verification
->
origin-admission audit
->
admitted-contribution audit
```

The implemented `AdmittedContributionAuditV0` establishes only:

```text
this exact replay-derived graph contribution
is structurally valid inside one exact admitted lane,
refers to the exact verified artifact identity,
and possesses one exact admitted origin assignment
```

It does not create support.

This contract defines the next exact question:

```text
Which exact admitted contributions are valid positive,
ceiling-bounded inputs to a future aggregation policy?
```

It does not answer:

```text
How many origin groups count?

Are two groups statistically independent?

Has corroboration occurred?

Has the target claim reached Supported?

What is achieved standing?
```

The layer distinctions are normative:

```text
admitted contribution
!= support contribution

support contribution
!= corroboration

corroboration
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

Pₛ:
the exact compiled support-contribution policy
magpie-support-contribution-v0

M:
the exact immutable ResolutionContentClosureV0 and its complete
four-field identity

same H + same Pₒ + same P꜀ + same Pₛ + same M
->
byte-identical SupportContributionAuditV0
```

No ambient state participates.

The future resolver must perform:

```text
no second replay
no filesystem scan
no CAS lookup
no network request
no callback
no plugin invocation
no environment lookup
no mutable registry lookup
no caller-selected search
```

The positive authority law is:

```text
one exact internally derived AdmittedContributionV0
+
complete upstream identity, compiled-policy, admitted-candidate trace,
alignment and completion/disposition validation
+
exact per-contribution lane, inherited identity, origin-group and
supporting-selector validation
+
compiled policy magpie-support-contribution-v0
->
one standing-inert SupportContributionV0
```

That result establishes only that the exact admitted contribution is a valid
positive input to a future aggregation policy and is capped at `Supported`.
It establishes no truth, corroboration, group count, statistical independence,
aggregation result or achieved standing.

## Current substrate and remaining gap

Ticket 0050 implements:

```rust
impl OriginAdmissionReplayContextV0 {
    pub fn resolve_admitted_contribution_audit_v0(
        &self,
        closure: &ResolutionContentClosureV0,
    ) -> AdmittedContributionAuditV0;
}
```

The private-construction `OriginAdmissionReplayContextV0` carries:

```text
one accepted StandingReplaySnapshot
+
one exact VerifiedLogPrefixIdentityV0
```

The same context internally derives the complete origin-admission audit and
the complete admitted-contribution audit for one exact immutable closure.

`AdmittedContributionAuditV0` publicly exposes:

```text
schema
canonicalization_profile
policy_id
origin_admission_policy_id
verified_prefix_identity
closure_identity
completion
candidate_audits
admitted_contributions
```

Each `AdmittedContributionV0` publicly exposes:

```text
policy_id
origin_admission_policy_id
contribution
namespace
origin_group
evidence_kind
claim_domain
candidate_ceiling
supporting_origin_candidate_selectors
```

The positive admitted result is intentionally represented twice:

```text
A:
each candidate audit with disposition
Admitted { admitted_contribution }

B:
the top-level admitted_contributions vector
```

Ticket 0050 constructs both from one execution and tests ordinary alignment.
The support-contribution boundary must nevertheless validate the complete
composition before treating either representation as inherited authority.
The remaining gap is a deterministic, standing-inert layer that:

1. derives the admitted audit internally from the same context and closure;
2. validates every exact upstream identity in fixed first-failure order;
3. validates the two fixed compiled support-policy cells independently of
   admitted contribution count;
4. validates every `Admitted` candidate's enclosing replay-derived trace
   against its nested admitted value;
5. proves trace-aligned candidate admissions and top-level admissions are
   identical maps;
6. validates that upstream completion agrees with every candidate disposition;
7. validates every remaining inherited contribution invariant, including the
   copied artifact digest, origin-group key and supporting selectors;
8. maps each aligned admitted contribution one-to-one into a typed support
   contribution; and
9. produces deterministic global completion or rejection without partial
   output.

The layer must not reopen graph structure, artifact verification, origin
binding or origin admission.

## Exact deterministic inputs

The future public resolver receives exactly:

```text
self:
one private-construction OriginAdmissionReplayContextV0 carrying H

closure:
one successfully constructed immutable ResolutionContentClosureV0 carrying M
```

Reviewed compiled code supplies:

```text
Pₒ:
magpie-origin-admission-v0

P꜀:
magpie-admitted-contribution-v0

Pₛ:
magpie-support-contribution-v0
```

The future audit retains:

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

The closure identity binds the exact immutable availability input. It does not
verify an object, reconstruct bytes or grant authority.

The future resolver must not accept:

```text
AdmittedContributionAuditV0
AdmittedContributionV0
AdmittedContributionCandidateAuditV0
candidate contribution lists
claim IDs
edge IDs
origin groups
policy IDs
trusted Booleans
caller-created snapshots
caller-created anchor indexes
serialized audit bytes
callbacks
plugins
filesystem paths
CAS references
network clients
environment state
```

A cloned or serialized admitted-contribution audit remains audit output. It is
not an authority token.

## Exact compiled policy identities

Freeze exactly:

```text
support-contribution policy:
magpie-support-contribution-v0

audit schema:
magpie-support-contribution-audit-v0

audit canonicalization profile:
magpie-support-contribution-audit-json-v0

required admitted-contribution policy:
magpie-admitted-contribution-v0

required origin-admission policy:
magpie-origin-admission-v0

required artifact algorithm:
sha256
```

The future implementation must define and re-export exactly:

```rust
pub const SUPPORT_CONTRIBUTION_AUDIT_SCHEMA_V0: &str =
    "magpie-support-contribution-audit-v0";

pub const SUPPORT_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0: &str =
    "magpie-support-contribution-audit-json-v0";

pub const SUPPORT_CONTRIBUTION_POLICY_ID_V0: &str =
    "magpie-support-contribution-v0";
```

The future implementation must reuse:

```text
ADMITTED_CONTRIBUTION_POLICY_ID_V0
ADMITTED_CONTRIBUTION_AUDIT_SCHEMA_V0
ADMITTED_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0
ADMITTED_CONTRIBUTION_ARTIFACT_ALGORITHM_V0
ORIGIN_ADMISSION_POLICY_ID_V0
```

It must not define independent duplicate constants for upstream identities.

There is no:

```text
runtime policy selector
policy registry
alias
negotiation
environment override
caller-selected policy
bundle-selected implementation
implicit latest version
```

Serialized policy IDs disclose reviewed compiled code. They do not select it.

## Internally derived source audit

Reserve exactly:

```rust
impl OriginAdmissionReplayContextV0 {
    pub fn resolve_support_contribution_audit_v0(
        &self,
        closure: &ResolutionContentClosureV0,
    ) -> SupportContributionAuditV0;
}
```

The future resolver must internally derive exactly one admitted-contribution
audit:

```rust
let admitted_audit =
    self.resolve_admitted_contribution_audit_v0(closure);
```

It may instead call one exact private implementation seam only if that seam
produces byte-identical `AdmittedContributionAuditV0` output.

The same context and closure must determine:

```text
verified-prefix identity
closure identity
origin-admission audit
admitted-contribution audit
support-contribution audit
```

There is no transposition seam where an admitted audit from another history,
closure or policy may be paired with this resolution.

## Complete support-source universe

Freeze:

```text
SupportContributionSourceUniverseV0(H, M)
=
every exact AdmittedContributionV0 in the internally derived
AdmittedContributionAuditV0, after complete upstream composition
validation, ordered by exact ContributionIdentityV0
```

This layer must not rediscover its source universe from:

```text
StandingView
justification edges
typed evidence
origin decisions
origin groups
closure contents
caller-selected contributions
candidate audits that are merely policy-eligible
```

The admitted-contribution layer already owns:

```text
graph structure
policy-lane admission
artifact identity binding
origin-decision lookup
```

The support-contribution layer must consume that result, not implement a
second admitted-contribution engine.

Candidate audits are consulted only to verify that every candidate `Admitted`
result is exactly aligned first with its enclosing replay-derived trace and
then with the top-level admitted output, and that every candidate disposition
agrees with upstream completion. Policy-ineligible, origin-incomplete,
decision-absent, conflicted and unresolved candidates are not support-source
entries.

## Upstream identity validation

Before consuming any admitted contribution, validate the internally derived
audit in this exact first-failure order:

```text
1. schema
   == magpie-admitted-contribution-audit-v0

2. canonicalization_profile
   == magpie-admitted-contribution-audit-json-v0

3. policy_id
   == magpie-admitted-contribution-v0

4. origin_admission_policy_id
   == magpie-origin-admission-v0

5. verified_prefix_identity
   == self.verified_prefix_identity()

6. closure_identity
   == closure.identity()
```

Any mismatch is a global composition rejection. The future implementation must
not reinterpret, repair, normalize or prefer mismatched upstream output.

The resulting rejected support audit still binds the expected current
`self.verified_prefix_identity()` and `closure.identity()`. It does not copy a
mismatched upstream identity into the new top-level authority surface.

## Global compiled-policy validation

Immediately after the six upstream identity checks, the future implementation
must validate these fixed compiled-policy cells exactly once:

```text
support_ceiling(
    ExternalSource,
    ExternalReport,
)
==
Some(Supported)
```

```text
support_context_requirement(
    ExternalSource,
    ExternalReport,
)
==
NoPrivilegedContext
```

These are global composition checks. They are not properties of one particular
`AdmittedContributionV0`.

They must execute before admitted-candidate trace consistency, candidate-map
insertion, duplicate detection, admitted-set alignment, upstream
completion/disposition consistency, per-contribution invariants, upstream
completion mapping or support projection. They must execute even when:

```text
the admitted audit is complete and empty

the admitted audit reports origin incompleteness

candidate_admitted is empty

top_level_admitted is empty
```

A fixed cell mismatch is a global composition rejection. The future
implementation must not attach a synthetic contribution identity, manufacture
an empty or sentinel contribution, or wrap the failure in
`ContributionInvariantMismatch`.

## Admitted-candidate trace consistency

Immediately after the two fixed compiled-policy checks, and before candidate
admitted-map insertion, duplicate candidate-admission detection, top-level
alignment, completion/disposition validation, per-contribution invariants,
completion mapping or projection, validate every candidate whose disposition
is exactly:

```text
Admitted {
    admitted_contribution
}
```

For each such candidate, validate these exact equalities in this exact field
order:

```text
1. candidate.edge_id
   ==
   admitted_contribution.contribution.justification_edge_id

2. candidate.source_evidence_id
   ==
   admitted_contribution.contribution.source_evidence_id

3. candidate.target_claim_id
   ==
   admitted_contribution.contribution.target_claim_id

4. candidate.evidence_kind
   ==
   Some(admitted_contribution.evidence_kind)

5. candidate.claim_domain
   ==
   Some(admitted_contribution.claim_domain)

6. candidate.candidate_ceiling
   ==
   Some(admitted_contribution.candidate_ceiling)

7. candidate.candidate_contribution
   ==
   Some(admitted_contribution.contribution)

8. candidate.candidate_namespace
   ==
   Some(admitted_contribution.namespace)
```

All comparisons are exact typed or exact byte-string equality. The future
implementation must not normalize strings, infer missing optional fields,
prefer either representation, repair the candidate or construct a replacement
candidate.

Only candidates with an exact `Admitted` disposition are subject to this
specific trace-to-admitted check. The first malformed admitted candidate in
exact candidate edge-ID order produces:

```rust
AdmittedCandidateTraceMismatch {
    edge_id: String,
}
```

The `edge_id` identifies the malformed candidate. The fixed eight-field
checking order remains normative even though the serialized failure does not
identify the mismatched field.

Only after one admitted candidate passes all eight checks may its nested
`AdmittedContributionV0` enter:

```text
candidate_admitted:
BTreeMap<ContributionIdentityV0, AdmittedContributionV0>
```

After this stage:

```text
candidate map membership
means
the nested admitted value is exactly aligned with its enclosing
replay-derived candidate trace
```

## Candidate-to-top-level alignment

Build privately:

```text
candidate_admitted:
BTreeMap<ContributionIdentityV0, AdmittedContributionV0>

top_level_admitted:
BTreeMap<ContributionIdentityV0, AdmittedContributionV0>
```

The candidate map includes only exact, trace-aligned:

```text
AdmittedContributionCandidateDispositionV0::Admitted
```

The top-level map includes every exact value in:

```text
AdmittedContributionAuditV0::admitted_contributions()
```

The normative alignment law is:

```text
candidate_admitted
==
top_level_admitted
```

Equality means:

```text
same exact key set
+
same complete AdmittedContributionV0 value for every key
```

The complete value includes:

```text
policy IDs
ContributionIdentityV0
OriginComparisonNamespaceV0
origin group
evidence kind
claim domain
candidate ceiling
supporting origin selectors
```

Failure laws:

```text
duplicate exact contribution identity in candidate admissions
->
global composition rejection

duplicate exact contribution identity in top-level admissions
->
global composition rejection

candidate-only identity
->
global composition rejection

top-level-only identity
->
global composition rejection

same identity but different complete admitted value
->
global composition rejection
```

The implementation must not choose:

```text
first
last
newest
oldest
lexical winner
top-level preferred
candidate preferred
```

No mismatch is locally repaired.

## Upstream completion/disposition consistency

After exact admitted key-set and complete-value equality succeed, the future
implementation must validate the upstream completion against every candidate
disposition.

Freeze these exact laws:

```text
AdmittedContributionAuditCompletionV0::Complete
->
no candidate disposition may be OriginAdmissionIncomplete
```

```text
AdmittedContributionAuditCompletionV0::OriginAdmissionIncomplete
->
every candidate disposition must be either:
- PolicyIneligible
- OriginAdmissionIncomplete
```

Therefore an incomplete upstream audit must contain no:

```text
OriginDecisionAbsent
OriginNotAdmitted
Admitted
```

A complete upstream audit may contain:

```text
PolicyIneligible
OriginDecisionAbsent
OriginNotAdmitted
Admitted
```

but never `OriginAdmissionIncomplete`.

An incomplete upstream audit containing only `PolicyIneligible` candidates is
valid. The consistency check must not require at least one
`OriginAdmissionIncomplete` candidate.

Failure laws:

```text
Complete + OriginAdmissionIncomplete candidate
->
global composition rejection

OriginAdmissionIncomplete + OriginDecisionAbsent candidate
->
global composition rejection

OriginAdmissionIncomplete + OriginNotAdmitted candidate
->
global composition rejection

OriginAdmissionIncomplete + aligned Admitted candidate/top-level value
->
global composition rejection
```

The failure reports the first inconsistent candidate in exact candidate
edge-ID order. It must not add another public enum or a free-form reason.

## Closed v0 support lane

The first support-contribution policy accepts exactly:

```text
AdmittedContributionV0
× admitted policy magpie-admitted-contribution-v0
× origin policy magpie-origin-admission-v0
× ExternalSource
× ExternalReport
× candidate ceiling Supported
× support_ceiling cell Supported
× support-context requirement NoPrivilegedContext
× sha256 artifact identity with an exact lowercase-hex-64 digest
->
SupportContributionV0
```

The normative positive shape is:

```text
exact internally aligned AdmittedContributionV0
+
exact ExternalSource × ExternalReport support cell
+
exact Supported ceiling
+
exact NoPrivilegedContext classification
+
exact sha256 contribution artifact with a 64-character lowercase
hexadecimal digest
->
SupportContributionV0
```

`NoPrivilegedContext` means only:

```text
this support-policy cell requires no HumanRoot,
deterministic-verifier or Deadbolt-verifier context
```

It does not mean:

```text
automatically admitted
trusted
true
corroborated
aggregated
Supported standing
```

The exact origin-admission and admitted-contribution paths remain mandatory.

The `support_ceiling` cell and support-context requirement are validated once
as fixed global compiled-policy prerequisites. The remaining inherited value
checks are applied to every aligned admitted contribution.

## Per-contribution invariants

For every aligned admitted contribution, validate in this exact first-failure
order:

```text
1. admitted contribution policy_id
   == magpie-admitted-contribution-v0

2. admitted contribution origin_admission_policy_id
   == magpie-origin-admission-v0

3. evidence_kind
   == ExternalSource

4. claim_domain
   == ExternalReport

5. candidate_ceiling
   == Supported

6. contribution.artifact_algorithm
   == sha256

7. contribution.artifact_digest
   is exactly 64 lowercase hexadecimal ASCII characters

8. namespace.origin_admission_policy_id
   == magpie-origin-admission-v0

9. namespace.target_claim_id
   == contribution.target_claim_id

10. namespace.scope_ref
   == contribution.scope_ref

11. origin_group
    satisfies the exact origin-binding protocol-identifier grammar

12. supporting_origin_candidate_selectors
    is non-empty

13. every supporting selector
    satisfies the exact origin-binding selector grammar

14. supporting selector vector
    is strictly increasing under exact derived Ord
```

Any invariant failure rejects the complete support-contribution audit
globally. The future implementation must not emit a partial support set from
an internally inconsistent admitted set.

The exact artifact-digest grammar is:

```text
byte length:
64

every byte:
0..9
or
a..f
```

Uppercase, a prefix or `0x` marker, whitespace, trimming, normalization,
Unicode, case conversion and repair are forbidden.

The exact origin-group grammar is:

```text
byte length:
1..=128

first byte:
ASCII alphanumeric

remaining bytes:
ASCII alphanumeric
or
. _ : / -
```

Validation is over the exact inherited bytes. No trimming, normalization,
Unicode folding, case conversion, aliasing or repair is permitted.

Every inherited supporting selector must satisfy this exact structural
origin-binding grammar:

```text
bundle_kind:
exactly one of
magpie-origin-binding-acquisition-v0
magpie-origin-binding-derivation-v0

witness_root:
exactly 64 lowercase hexadecimal ASCII characters

witness_algorithm:
exactly sha256 through
ORIGIN_BINDING_WITNESS_ALGORITHM_V0

canonicalization_profile:
exactly magpie-origin-binding-json-v0 through
ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0

run_id:
byte length 1..=128
first byte ASCII alphanumeric
remaining bytes ASCII alphanumeric or . _ : / -
```

This validation is structural and protocol-exact. It does not re-read bundle
bytes, recompute the bundle witness root, look up an anchor, reopen origin
admission, rerun the origin-binding verifier or confer standalone authority on
the selector.

After the vector is known to be non-empty and every selector is individually
valid, require:

```text
supporting_origin_candidate_selectors
is strictly increasing under
ArtifactProvenanceAnchorSelectorV0::Ord
```

Strictly increasing means exact canonical `BTreeSet`-derived order plus no
duplicate exact selectors. The future implementation must not sort,
deduplicate, repair order or select a preferred duplicate.

The landed origin-admission fold produces supporting selectors from a
`BTreeSet`. The exact derived `ArtifactProvenanceAnchorSelectorV0::Ord` field
order is:

```text
bundle_kind
witness_root
witness_algorithm
canonicalization_profile
run_id
```

The future implementation must reuse the existing origin-binding grammars
through exactly these three narrow crate-private seams in
`crates/magpie-claims/src/origin_binding_verifier.rs`:

```rust
pub(crate) fn valid_origin_group_v0(value: &str) -> bool {
    valid_identifier(value)
}

pub(crate) fn valid_origin_binding_artifact_digest_v0(
    value: &str,
) -> bool {
    is_lowercase_hex_64(value)
}

pub(crate) fn valid_origin_binding_selector_v0(
    selector: &ArtifactProvenanceAnchorSelectorV0,
) -> bool {
    let bundle_kind = selector.bundle_kind();

    (bundle_kind == ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0
        || bundle_kind == ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0)
        && is_lowercase_hex_64(selector.witness_root())
        && selector.witness_algorithm()
            == ORIGIN_BINDING_WITNESS_ALGORITHM_V0
        && selector.canonicalization_profile()
            == ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0
        && valid_identifier(selector.run_id())
}
```

Their exact delegation is:

```text
valid_origin_group_v0
-> existing valid_identifier

valid_origin_binding_artifact_digest_v0
-> existing is_lowercase_hex_64

valid_origin_binding_selector_v0
-> existing origin-binding constants
 + existing is_lowercase_hex_64
 + existing valid_identifier
```

The support-audit implementation must call these helpers. All three must remain
crate-private, add no public API, accept no configurable grammar, expected
kind, profile, algorithm, length or alphabet, and change no origin-binding
behaviour, output or canonical bytes.

The future implementation must not expose a generic public identifier
validator, caller-selected validation profile, runtime grammar registry, new
policy input, test-only public switch, closure lookup, anchor lookup or bundle
re-verification. It must not reuse `valid_provenance_selector`, because that
helper validates the inner artifact-provenance acquisition and derivation
selector kinds rather than the two outer origin-binding selector kinds.

## Global completion and failure precedence

Freeze this exact global composition-validation order:

```text
1. upstream schema

2. upstream canonicalization profile

3. upstream admitted policy

4. upstream origin policy

5. verified-prefix identity

6. closure identity

7. compiled support-ceiling cell

8. compiled support-context cell

9. admitted-candidate trace consistency

10. duplicate candidate-admission identity

11. duplicate top-level-admission identity

12. admitted key-set equality

13. complete admitted-value equality

14. upstream completion/disposition consistency

15. per-contribution invariants

16. upstream completion mapping

17. support-contribution projection
```

An internally inconsistent upstream audit is rejected even if its completion
also reports incomplete origin admission.

Normative precedence is:

```text
upstream identity failure
precedes
compiled-policy failure

compiled-policy failure
precedes
candidate trace failure

candidate trace failure
precedes
duplicate and admitted-map alignment failure

admitted-map alignment failure
precedes
completion/disposition failure

completion/disposition failure
precedes
per-contribution invariant failure

all composition and invariant failure
precedes
completion mapping

completion mapping
precedes
projection
```

Upstream completion is mapped only after all identity, compiled-policy,
candidate-trace, alignment, completion/disposition and per-contribution checks
succeed. No partial support-contribution vector may survive any failure.

The future top-level completion is:

```rust
pub enum SupportContributionAuditCompletionV0 {
    Complete,

    AdmittedContributionIncomplete {
        unavailable_binding_selectors:
            Vec<ArtifactProvenanceAnchorSelectorV0>,
    },

    UpstreamCompositionRejected {
        failure: SupportContributionCompositionFailureV0,
    },
}
```

Laws:

```text
identity, compiled-policy cells, admitted-candidate trace, alignment,
completion/disposition and invariants valid
+
upstream completion Complete
->
Complete
```

```text
identity, compiled-policy cells, admitted-candidate trace, alignment,
completion/disposition and invariants valid
+
upstream OriginAdmissionIncomplete
->
AdmittedContributionIncomplete
->
zero support contributions
```

```text
any upstream identity, compiled-policy, admitted-candidate trace, alignment,
completion/disposition or invariant failure
->
UpstreamCompositionRejected
->
zero support contributions
```

Unavailable selectors are inherited exactly from:

```text
AdmittedContributionAuditCompletionV0::
OriginAdmissionIncomplete
```

Their exact order must be preserved.

A complete admitted audit containing zero admitted contributions obeys:

```text
complete and empty admitted audit
+
valid global support-ceiling cell
+
valid global support-context cell
+
valid composition
->
Complete
+
empty support_contributions
```

That is not an error and is not refutation.

The policy-drift laws are:

```text
complete and empty admitted audit
+
support-ceiling policy drift
->
UpstreamCompositionRejected {
    SupportCeilingPolicyMismatch
}
+
zero support_contributions
```

```text
origin-incomplete admitted audit
+
valid global policy cells
+
valid composition
->
AdmittedContributionIncomplete
+
inherited unavailable selectors
+
zero support_contributions
```

```text
origin-incomplete admitted audit
+
support-context policy drift
->
UpstreamCompositionRejected {
    SupportContextRequirementMismatch
}
+
zero support_contributions
```

Complete-empty does not bypass policy validation. Availability incompleteness
does not hide compiled policy drift.

## Closed composition-failure vocabulary

Freeze variants in this exact order:

```rust
pub enum SupportContributionCompositionFailureV0 {
    UpstreamSchemaMismatch,

    UpstreamCanonicalizationProfileMismatch,

    UpstreamAdmittedPolicyMismatch,

    UpstreamOriginPolicyMismatch,

    VerifiedPrefixIdentityMismatch,

    ClosureIdentityMismatch,

    SupportCeilingPolicyMismatch,

    SupportContextRequirementMismatch,

    AdmittedCandidateTraceMismatch {
        edge_id: String,
    },

    DuplicateCandidateAdmission {
        contribution: ContributionIdentityV0,
    },

    DuplicateTopLevelAdmission {
        contribution: ContributionIdentityV0,
    },

    AdmittedSetMismatch {
        candidate_only:
            Vec<ContributionIdentityV0>,

        top_level_only:
            Vec<ContributionIdentityV0>,
    },

    AdmittedValueMismatch {
        contribution: ContributionIdentityV0,
    },

    UpstreamCompletionDispositionMismatch {
        edge_id: String,
    },

    ContributionInvariantMismatch {
        contribution: ContributionIdentityV0,
        reason: SupportContributionInvariantReasonV0,
    },
}
```

The two global policy-cell failures carry no `ContributionIdentityV0`.
Their exact semantics are:

```text
SupportCeilingPolicyMismatch:
the compiled support_ceiling(
    ExternalSource,
    ExternalReport,
) cell is not exactly Some(Supported)

SupportContextRequirementMismatch:
the compiled support_context_requirement(
    ExternalSource,
    ExternalReport,
) cell is not exactly NoPrivilegedContext
```

`AdmittedCandidateTraceMismatch` reports the exact `edge_id` of the first
malformed admitted candidate in exact candidate edge-ID order. It carries no
free-form field name or generic reason; the fixed eight-field comparison order
is normative.

`UpstreamCompletionDispositionMismatch` reports the exact `edge_id` of the
first candidate whose disposition is inconsistent with the upstream
completion. Candidate selection uses exact edge-ID order.

Ordering requirements:

```text
candidate_only:
exact ContributionIdentityV0 order

top_level_only:
exact ContributionIdentityV0 order
```

No:

```text
Other
UnknownFailure
free-form reason string
Boolean valid/invalid
caller-defined variant
```

is permitted.

## Closed invariant-failure vocabulary

Freeze variants in this exact order:

```rust
pub enum SupportContributionInvariantReasonV0 {
    AdmittedPolicyMismatch,
    OriginPolicyMismatch,
    EvidenceKindMismatch,
    ClaimDomainMismatch,
    CandidateCeilingMismatch,
    ArtifactAlgorithmMismatch,
    InvalidArtifactDigest,
    NamespacePolicyMismatch,
    NamespaceTargetMismatch,
    NamespaceScopeMismatch,
    InvalidOriginGroup,
    MissingSupportingOriginSelector,
    InvalidSupportingOriginSelector,
    NonCanonicalSupportingOriginSelectorOrder,
}
```

Semantics:

```text
CandidateCeilingMismatch:
the admitted value does not carry exact Supported

InvalidOriginGroup:
the exact inherited origin_group does not satisfy the existing
origin-binding protocol-identifier grammar

InvalidArtifactDigest:
the inherited contribution artifact digest is not exactly 64 lowercase
hexadecimal ASCII characters

MissingSupportingOriginSelector:
the inherited supporting selector vector is empty

InvalidSupportingOriginSelector:
at least one inherited supporting selector fails the exact structural
origin-binding selector grammar

NonCanonicalSupportingOriginSelectorOrder:
the non-empty, individually valid supporting selector vector is not strictly
increasing under ArtifactProvenanceAnchorSelectorV0::Ord
```

This closed vocabulary contains fourteen contribution-specific reasons. Fixed
compiled-policy drift is represented only by the two global composition
failures and must fail closed rather than silently widening or weakening the
contract.

## SupportContributionV0 surface

Freeze this exact conceptual field order:

```rust
pub struct SupportContributionV0 {
    policy_id: String,
    admitted_contribution_policy_id: String,
    origin_admission_policy_id: String,
    contribution: ContributionIdentityV0,
    namespace: OriginComparisonNamespaceV0,
    origin_group: String,
    evidence_kind: String,
    claim_domain: String,
    support_ceiling: Status,
    supporting_origin_candidate_selectors:
        Vec<ArtifactProvenanceAnchorSelectorV0>,
}
```

Exact v0 values:

```text
policy_id:
magpie-support-contribution-v0

admitted_contribution_policy_id:
magpie-admitted-contribution-v0

origin_admission_policy_id:
magpie-origin-admission-v0

evidence_kind:
ExternalSource

claim_domain:
ExternalReport

support_ceiling:
Supported
```

All remaining fields are copied exactly from the aligned
`AdmittedContributionV0`. The admitted `candidate_ceiling` becomes the
support-contribution `support_ceiling` only after the exact compiled ceiling
and context cells are revalidated globally.

The future type exposes read-only getters for every field.

`SupportContributionV0` means only:

```text
this exact admitted contribution is a valid positive input
to the future support aggregation policy, capped at Supported
```

It does not mean:

```text
the target is currently Supported
the contribution has been counted
another origin group corroborates it
the source is reputable
the report is true
the group is independent
```

## SupportContributionAuditV0 surface

Freeze this exact conceptual field order:

```rust
pub struct SupportContributionAuditV0 {
    schema: String,
    canonicalization_profile: String,
    policy_id: String,
    admitted_contribution_policy_id: String,
    origin_admission_policy_id: String,
    verified_prefix_identity: VerifiedLogPrefixIdentityV0,
    closure_identity: ResolutionContentClosureIdentityV0,
    completion: SupportContributionAuditCompletionV0,
    support_contributions: Vec<SupportContributionV0>,
}
```

Exact fixed values:

```text
schema:
magpie-support-contribution-audit-v0

canonicalization_profile:
magpie-support-contribution-audit-json-v0

policy_id:
magpie-support-contribution-v0

admitted_contribution_policy_id:
magpie-admitted-contribution-v0

origin_admission_policy_id:
magpie-origin-admission-v0
```

Only the top-level audit exposes:

```rust
pub fn canonical_bytes(&self) -> Vec<u8>;
```

The future implementation must serialize directly from the typed structure:

```rust
serde_json::to_vec(self)
```

It must not define:

```text
JCS
RFC 8785
generic canonical JSON
map sorting after serialization
new Magpie canonical encoding
new L0 format
```

## Deterministic ordering and canonical bytes

Freeze:

```text
candidate admitted map:
exact ContributionIdentityV0 order

admitted-candidate trace mismatch:
first malformed admitted candidate in exact edge-ID order

top-level admitted map:
exact ContributionIdentityV0 order

candidate_only differences:
exact ContributionIdentityV0 order

top_level_only differences:
exact ContributionIdentityV0 order

support_contributions:
exact ContributionIdentityV0 order

supporting origin selectors:
exact order inherited from AdmittedContributionV0, accepted only when
strictly increasing under ArtifactProvenanceAnchorSelectorV0::Ord

unavailable binding selectors:
exact order inherited from AdmittedContributionAuditV0

completion/disposition mismatch:
first inconsistent candidate in exact edge-ID order
```

When more than one duplicate candidate identity, duplicate top-level identity,
complete-value mismatch or contribution-invariant mismatch exists, the global
failure reports the first affected contribution in exact
`ContributionIdentityV0` order. Within that contribution, invariant reasons
use the exact fourteen-step first-failure order. Individual selector validity
is checked before selector-vector canonicality.

No caller order, event insertion order, group lexical preference or write
recency selects an outcome.

Deterministic canonical bytes arise from:

```text
exact struct field declaration order
+
exact enum declaration and adjacent tagging
+
exact ordered vectors
+
direct serde_json::to_vec(self)
```

The audit is purpose-built deterministic derived JSON. It is not Magpie L0
canonical encoding.

## Multiplicity and non-amplification

Preserve:

```text
two distinct admitted contribution identities
assigned to the same namespace and origin group
->
two SupportContributionV0 values
```

The future resolver must not key, collapse, deduplicate, score or count by
origin group.

The layer ownership is:

```text
origin-binding duplicate collapse
->
OriginAdmissionAuditV0

graph/artifact/origin admission
->
AdmittedContributionAuditV0

positive support-input classification
->
SupportContributionAuditV0

same-origin collapse and distinct-group counting
->
future standing policy v3
```

This contract does not implement the fourth layer.

It does not define:

```text
AggregationLaneV0
lane key
lane partition
group count
distinct-group threshold
quorum
majority
weight
score
minimum reports
corroboration result
```

Future policy v3 alone owns:

```text
which support contributions share a lane
how same-origin material collapses
whether distinct groups produce corroboration separation
what threshold applies
what achieved standing follows
```

Ticket 0051 preserves the exact material that policy v3 may later consume.

## Source-standing boundary

The v0 source endpoint is one `TypedEvidenceNode` classified as
`ExternalSource`. It is not a source claim with governed standing.

Therefore v0 performs no:

```text
claim-to-claim standing propagation
source-claim threshold
transitive support
recursive graph evaluation
publisher standing
document reputation
```

The generic doctrine that future support contributions may be constrained by
source standing remains valid for future claim-to-claim lanes.

It is not applicable to this exact first evidence-node lane.

The support-contribution audit does not reinterpret a source URI, locator,
publisher label, origin group or artifact digest as source standing.

## Authority and serialization boundary

Freeze exactly these five future public types:

```text
SupportContributionAuditCompletionV0
SupportContributionCompositionFailureV0
SupportContributionInvariantReasonV0
SupportContributionV0
SupportContributionAuditV0
```

All future public structs must have:

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

All closed public enums must have:

```text
Clone
Debug
PartialEq
Eq
Serialize
no Deserialize
no Default
```

All enums serialize exactly with:

```rust
#[serde(
    tag = "outcome",
    content = "details",
    rename_all = "snake_case"
)]
```

Only `SupportContributionAuditV0` exposes `canonical_bytes()`.

Every future support-audit value is derived audit output. No serialized,
cloned or caller-assembled value may substitute for the replay context and
closure.

The resolver authority boundary remains:

```text
LogReader
->
replay_origin_admission_context_v0
->
OriginAdmissionReplayContextV0
->
resolve_support_contribution_audit_v0(closure)
```

Private construction prevents a caller from transposing:

```text
one admitted audit onto another history
one admitted audit onto another closure
one admitted contribution onto another candidate
one policy output onto another policy
one serialized support audit onto live replay authority
```

## Required implementation fixtures

The future implementation PR must add literal canonical fixtures for at least:

```text
one-support-contribution.json

complete-empty.json

admitted-contribution-incomplete.json

same-origin-distinct-contributions.json

distinct-origin-distinct-contributions.json
```

Expected meanings:

```text
one-support-contribution:
one aligned admitted contribution
->
Complete
->
one support contribution

complete-empty:
complete aligned admitted audit with no admissions
->
Complete
->
zero support contributions

admitted-contribution-incomplete:
origin availability incomplete
->
AdmittedContributionIncomplete
->
zero support contributions

same-origin-distinct-contributions:
two distinct admitted contributions in one group
->
two support contributions
->
no collapse

distinct-origin-distinct-contributions:
two distinct admitted contributions in different groups
->
two support contributions
->
no corroboration or threshold claim
```

Malformed inherited-audit composition cases may be tested through the smallest
private pure helper because the public same-context resolver must not accept
caller-created malformed audits.

No additional normative fixture file is required for malformed composition
cases.

Because the production policy cells are fixed, the future implementation may
test policy drift through that smallest private pure composition helper. It
must not add a public policy override, public test switch, runtime policy
parameter, mock policy registry or caller-selected cell.

The helper must not add public constructors, audit inputs, grammar parameters
or test switches.

Ticket 0051 adds no fixture files, canonical byte lengths or hashes.

## Required hostile tests

The future implementation must prove:

```text
one exact aligned admitted contribution
->
one support contribution

complete admitted audit with no admissions
->
Complete
->
empty output

complete-empty admitted audit
+
compiled support-ceiling drift
->
UpstreamCompositionRejected {
    SupportCeilingPolicyMismatch
}
->
zero support contributions

upstream origin incompleteness
->
inherited selector order
->
zero support contributions

origin-incomplete admitted audit
+
compiled support-context drift
->
UpstreamCompositionRejected {
    SupportContextRequirementMismatch
}
->
zero support contributions

compiled support-policy drift
is checked even when both admitted maps are empty

compiled policy-cell rejection
precedes upstream completion mapping

valid complete-empty admitted audit
->
Complete
->
empty output

valid origin-incomplete admitted audit
->
AdmittedContributionIncomplete
->
zero output

Complete
+
OriginAdmissionIncomplete candidate
->
global composition rejection

OriginAdmissionIncomplete
+
OriginDecisionAbsent candidate
->
global composition rejection

OriginAdmissionIncomplete
+
OriginNotAdmitted candidate
->
global composition rejection

OriginAdmissionIncomplete
+
aligned Admitted candidate and top-level value
->
global composition rejection

multiple inconsistent candidates
->
first edge in exact edge-ID order reported

valid incomplete audit containing only PolicyIneligible
and OriginAdmissionIncomplete dispositions
->
accepted completion mapping

same-origin distinct admitted contributions
->
two support contributions

distinct-origin admitted contributions
->
two support contributions
->
no corroboration claim

upstream schema mismatch
->
global rejection

upstream canonicalization-profile mismatch
->
global rejection

upstream admitted-policy mismatch
->
global rejection

upstream origin-policy mismatch
->
global rejection

verified-prefix mismatch
->
global rejection

closure-identity mismatch
->
global rejection

otherwise aligned admitted candidate
+
separately mutate:
- edge_id
- source_evidence_id
- target_claim_id
- evidence_kind
- claim_domain
- candidate_ceiling
- candidate_contribution
- candidate_namespace
->
UpstreamCompositionRejected {
    failure:
        AdmittedCandidateTraceMismatch {
            edge_id: exact candidate edge ID
        }
}

two malformed admitted candidates
->
first edge in exact edge-ID order reported

candidate trace mismatch
->
detected before duplicate candidate-map insertion

candidate trace mismatch
->
zero support contributions

duplicate candidate admission
->
global rejection

duplicate top-level admission
->
global rejection

candidate-only admitted identity
->
global rejection

top-level-only admitted identity
->
global rejection

same identity with differing admitted value
->
global rejection

every per-contribution invariant drift
->
global rejection

empty group
->
InvalidOriginGroup

129-byte group
->
InvalidOriginGroup

non-ASCII group
->
InvalidOriginGroup

group beginning with punctuation
->
InvalidOriginGroup

group containing whitespace or unsupported punctuation
->
InvalidOriginGroup

1-byte ASCII-alphanumeric group
->
valid

128-byte valid group
->
valid

allowed . _ : / - characters after the first byte
->
valid

empty artifact digest
->
InvalidArtifactDigest

63-character lowercase hexadecimal artifact digest
->
InvalidArtifactDigest

65-character lowercase hexadecimal artifact digest
->
InvalidArtifactDigest

64-character uppercase hexadecimal artifact digest
->
InvalidArtifactDigest

64-character mixed-case artifact digest
->
InvalidArtifactDigest

64-character non-hexadecimal artifact digest
->
InvalidArtifactDigest

artifact digest with 0x prefix
->
InvalidArtifactDigest

artifact digest with whitespace
->
InvalidArtifactDigest

exactly 64 lowercase hexadecimal ASCII characters
->
valid artifact digest

supporting selector with unknown bundle kind
->
InvalidSupportingOriginSelector

supporting selector with artifact-provenance acquisition bundle kind
instead of origin-binding acquisition kind
->
InvalidSupportingOriginSelector

supporting selector with empty witness root
->
InvalidSupportingOriginSelector

supporting selector with 63-character witness root
->
InvalidSupportingOriginSelector

supporting selector with 65-character witness root
->
InvalidSupportingOriginSelector

supporting selector with uppercase witness root
->
InvalidSupportingOriginSelector

supporting selector with non-hexadecimal witness root
->
InvalidSupportingOriginSelector

supporting selector with wrong witness algorithm
->
InvalidSupportingOriginSelector

supporting selector with wrong canonicalization profile
->
InvalidSupportingOriginSelector

supporting selector with empty run ID
->
InvalidSupportingOriginSelector

supporting selector with 129-byte run ID
->
InvalidSupportingOriginSelector

supporting selector with run ID beginning with punctuation
->
InvalidSupportingOriginSelector

supporting selector with run ID containing whitespace
->
InvalidSupportingOriginSelector

supporting selector with run ID containing unsupported punctuation
->
InvalidSupportingOriginSelector

supporting selector with non-ASCII run ID
->
InvalidSupportingOriginSelector

magpie-origin-binding-acquisition-v0 selector
->
valid

magpie-origin-binding-derivation-v0 selector
->
valid

supporting selector run ID at 1 byte
->
valid

supporting selector run ID at 128 bytes
->
valid

supporting selector run ID with allowed internal . _ : / - characters
->
valid

empty supporting selector vector
->
MissingSupportingOriginSelector

one valid supporting selector
->
valid

two valid supporting selectors in strict Ord order
->
valid

duplicate exact supporting selector
->
NonCanonicalSupportingOriginSelectorOrder

two valid supporting selectors in reverse Ord order
->
NonCanonicalSupportingOriginSelectorOrder

individually invalid selector in an otherwise ordered vector
->
InvalidSupportingOriginSelector

invalid-selector failure
->
precedes order failure

composition rejection
->
zero support contributions

caller-provided AdmittedContributionAuditV0
->
no public API

caller-provided AdmittedContributionV0 list
->
no public API

caller-provided origin group or policy
->
no public API

same H + Pₒ + P꜀ + Pₛ + M
->
byte-identical support audit

equivalent closure construction order
->
byte-identical support audit

different H with equal admitted projections
->
different verified-prefix identity and audit bytes

support audit execution
->
OriginAdmissionAuditV0 bytes unchanged
->
AdmittedContributionAuditV0 bytes unchanged
->
standing v0/v1/v2 bytes unchanged
->
support/refutation ceilings unchanged
->
support-context requirements unchanged
```

The future implementation must add compile-fail tests proving:

```text
SupportContributionV0 has no public constructor

SupportContributionAuditV0 has no public constructor

none of the five public types implements Deserialize

no resolver accepts an admitted audit

no resolver accepts an admitted-contribution list

no resolver accepts a claim ID

no resolver accepts an origin group

no resolver accepts a policy ID

StandingView has no support-contribution resolver

StandingReplaySnapshot has no complete support-contribution resolver
```

The implementation must not add public test controls.

## Frozen surfaces

Ticket 0051 freezes and changes none of:

```text
Magpie L0 format
payload vocabulary
canonical event encoding
hashing
signatures
genesis
LogStore
LogReader
LogWriter
VerifiedReplaySummary
VerifiedLogPrefixIdentityV0
StandingView fields and bytes
StandingReplaySnapshot fields and bytes
standing v0
standing v1
standing v2
standing policy IDs
standing trace vocabularies
support ceilings
refutation ceilings
support-context requirement matrix
deterministic verifier contexts
Deadbolt verifier contexts
artifact-provenance verifier
resolution-content closure
origin-binding verifier
ContributionIdentityV0
OriginComparisonNamespaceV0
OriginAdmissionReplayContextV0 construction
OriginAdmissionAuditV0
OriginAdmissionDecisionV0
AdmittedContributionAuditV0
AdmittedContributionV0
admitted-contribution fixtures
Cargo manifests
dependencies
lockfile
release metadata
Deadbolt
```

The later implementation may add only:

```text
three exact compiled constants
five standing-inert public audit types
one exact context resolver method
the smallest private composition helper needed for hostile tests
three exact crate-private origin-binding grammar reuse helpers:
- valid_origin_group_v0
- valid_origin_binding_artifact_digest_v0
- valid_origin_binding_selector_v0
new support-contribution fixtures and tests
```

These three crate-private helpers are the only permitted origin-binding module
changes. They must delegate exactly as frozen above and must not change
origin-binding behaviour, output, canonical bytes or public API. The later
implementation must not otherwise mutate existing origin-binding,
admitted-contribution or standing surfaces.

## Explicit non-goals

This contract does not define or implement:

- support-contribution runtime;
- aggregation lane;
- same-origin collapse;
- origin-group counting;
- distinct-group threshold;
- quorum;
- majority;
- weighting;
- scoring;
- probability;
- confidence;
- source reputation;
- publisher identity;
- statistical independence;
- corroboration result;
- achieved `Supported`;
- standing policy v3;
- standing change;
- `ExternalSource × Interpretation`;
- any non-`ExternalSource × ExternalReport` support lane;
- claim-to-claim source-standing propagation;
- transitive graph support;
- refutation contribution;
- direct refutation;
- contradiction debt;
- invalidation;
- supersession/currentness;
- quote verification;
- locator validation;
- `EpistemicGate`;
- writer authority;
- loader;
- CAS;
- filesystem access;
- network access;
- callback or plugin lookup;
- new L0 payload; or
- Deadbolt changes.

## Future implementation sequence

The next runtime PR must implement this contract mechanically in this exact
order:

1. add exact constants and five public audit types;
2. add the exact context resolver;
3. internally derive one admitted-contribution audit;
4. validate the six upstream audit identities;
5. validate the two fixed compiled support-policy cells;
6. validate every `Admitted` candidate's enclosing trace against its nested
   `AdmittedContributionV0`;
7. build candidate and top-level admitted maps and reject duplicate identities;
8. validate exact admitted key-set and complete-value equality;
9. validate upstream completion/disposition consistency;
10. apply the fourteen per-contribution invariants;
11. map upstream completion only after every composition and invariant check
    succeeds;
12. project aligned admissions one-to-one in exact contribution order;
13. serialize the typed top-level audit directly; and
14. add fixtures, hostile tests and compile-fail tests.

After that runtime PR:

```text
standing policy v3 contract
->
conservative aggregation
->
direct refutation
->
contradiction debt
->
invalidation and supersession/currentness
->
EpistemicGate and writer surfaces
```

No later stage is ratified or implemented by Ticket 0051.

## Reviewer checklist

- Confirm status is ratified contract, runtime absent, support contributions
  absent, aggregation absent and standing unchanged.
- Confirm the deterministic input is exactly
  `H + Pₒ + P꜀ + Pₛ + M`.
- Confirm the admitted audit is derived internally from the same context and
  closure and cannot be caller-provided.
- Confirm the complete support-source universe is every aligned top-level
  `AdmittedContributionV0`, ordered by exact contribution identity.
- Confirm graph, artifact and origin admission are not reopened.
- Confirm upstream identity validation uses the exact six-step order.
- Confirm the exact seventeen-stage composition-validation order.
- Confirm the support-ceiling and support-context cells are global checks.
- Confirm the global policy cells are checked for empty admitted sets.
- Confirm the global policy cells are checked before completion mapping.
- Confirm policy drift outranks origin incompleteness.
- Confirm the two global policy failures carry no contribution identity.
- Confirm every `Admitted` candidate's enclosing trace agrees with its nested
  `AdmittedContributionV0` across all eight exact fields.
- Confirm candidate trace validation occurs before candidate-map insertion.
- Confirm candidate trace mismatch reports the first exact edge ID.
- Confirm duplicate candidate admission is checked before duplicate top-level
  admission.
- Confirm candidate and top-level maps require identical keys and complete
  values.
- Confirm no candidate or top-level representation wins a mismatch.
- Confirm upstream completion agrees with every candidate disposition.
- Confirm the first inconsistent edge is deterministic in exact edge-ID order.
- Confirm incomplete audits cannot contain terminal origin decisions or
  admissions.
- Confirm complete audits cannot contain `OriginAdmissionIncomplete`
  candidates.
- Confirm composition failure precedes upstream availability incompleteness.
- Confirm the only positive lane is exact
  `ExternalSource × ExternalReport × Supported × NoPrivilegedContext × sha256`.
- Confirm `NoPrivilegedContext` grants no standalone admission or support.
- Confirm the artifact digest is exactly 64 lowercase hexadecimal ASCII
  characters.
- Confirm artifact-digest grammar reuses the one exact crate-private
  `valid_origin_binding_artifact_digest_v0` helper.
- Confirm `origin_group` is validated under the exact existing
  protocol-identifier grammar.
- Confirm every supporting selector is structurally valid.
- Confirm selector bundle kind is one of the two exact origin-binding kinds.
- Confirm selector witness root is exact lowercase hexadecimal-64.
- Confirm selector witness algorithm and canonicalization profile use the
  exact fixed origin-binding constants.
- Confirm selector run ID uses the exact protocol-identifier grammar.
- Confirm supporting selectors are non-empty.
- Confirm supporting selectors are strictly ordered under exact derived `Ord`
  and duplicate-free.
- Confirm malformed selector vectors are never sorted, deduplicated or
  repaired.
- Confirm the implementation permits exactly the three crate-private reuse
  helpers `valid_origin_group_v0`,
  `valid_origin_binding_artifact_digest_v0` and
  `valid_origin_binding_selector_v0`.
- Confirm the invariant vocabulary contains fourteen contribution-specific
  reasons in the exact fourteen-step first-failure order.
- Confirm any composition or invariant failure produces zero support
  contributions globally.
- Confirm complete-empty remains valid only when global policy validation
  succeeds and is not refutation.
- Confirm all five future public types, traits, field order and enum tagging.
- Confirm only the top-level audit exposes `canonical_bytes()`.
- Confirm deterministic ordering and direct typed serialization.
- Confirm distinct same-origin contributions remain distinct support inputs.
- Confirm no origin group is counted or collapsed.
- Confirm no aggregation lane, threshold, quorum, weight or corroboration
  result appears.
- Confirm the v0 source endpoint is evidence, not a source claim with governed
  standing.
- Confirm no standing, policy matrix, admitted-contribution, origin-admission,
  L0, Cargo, dependency, fixture or Deadbolt surface changes in this
  documentation-only PR.
