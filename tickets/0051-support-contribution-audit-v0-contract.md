# Ticket 0051: Support-contribution audit v0 contract

## 1. Exact base and reviewed implementation head

```text
base:
62026dbcfeff06ef6c6255e5e775113ded8e94fc

reviewed PR #63 implementation head:
b9b34ee95a08345c2cb27b92f310ad5e9124af35
```

The base is the merge commit of PR #63. The reviewed PR #63 implementation
head is an ancestor of that exact base.

## 2. Branch, commit and PR title

```text
branch:
agent/support-contribution-audit-v0-contract

commit:
docs: ratify support-contribution audit v0

PR title:
docs: ratify support-contribution audit v0

PR state:
draft
```

## 3. Documentation-only classification

This ticket ratifies the exact standing-inert contract:

```text
internally derived AdmittedContributionAuditV0
+
exact upstream composition validation
+
one compiled support-contribution policy
->
deterministic SupportContributionAuditV0
```

Exact classification:

```text
correctness:
complete support-contribution audit contract

authority:
internally aligned AdmittedContributionV0 values only

positive effect:
typed support input only

standing:
unchanged

aggregation:
not implemented

runtime:
not implemented
```

This ticket changes documentation only. It adds no Rust, tests, fixtures,
Cargo metadata, dependency, release file, tool, CI, L0 surface or runtime
capability.

## 4. Exact five-path allowlist

Only:

```text
README.md
docs/design/admitted-contribution-audit-v0.md
docs/design/standing-aggregation-independence-groups.md
docs/design/support-contribution-audit-v0.md
tickets/0051-support-contribution-audit-v0-contract.md
```

No other path may change.

In particular, this ticket changes no Rust, fixture, test, Cargo, lockfile,
release, tool, CI, existing ticket, origin-admission design,
standing-view-evidence-ceilings design or `AGENTS.md` path.

## 5. Architectural layer distinctions

The landed architecture is:

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

The implemented `AdmittedContributionAuditV0` establishes:

```text
this exact replay-derived graph contribution
is structurally valid inside one exact admitted lane,
refers to the exact verified artifact identity,
and possesses one exact admitted origin assignment
```

It does not create support.

Ticket 0051 freezes:

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

The support-contribution audit answers only:

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

## 6. Central deterministic law

Freeze:

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

The future resolver performs:

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

## 7. Exact policy identities

Freeze:

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

Future public constants:

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

It must not define duplicate upstream constants.

There is no runtime selector, registry, alias, negotiation, environment
override, caller-selected policy, bundle-selected implementation or implicit
latest version. Serialized policy IDs disclose compiled code; they do not
select it.

## 8. Future resolver

Reserve exactly:

```rust
impl OriginAdmissionReplayContextV0 {
    pub fn resolve_support_contribution_audit_v0(
        &self,
        closure: &ResolutionContentClosureV0,
    ) -> SupportContributionAuditV0;
}
```

No free resolver, `StandingView` resolver, standalone
`StandingReplaySnapshot` complete resolver or alternate authority carrier is
permitted.

## 9. Internally derived source audit

The future resolver must derive exactly one admitted-contribution audit:

```rust
let admitted_audit =
    self.resolve_admitted_contribution_audit_v0(closure);
```

It may call one exact private implementation seam only if the resulting
`AdmittedContributionAuditV0` is byte-identical.

It must not accept:

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

A cloned or serialized admitted audit is output, not authority.

## 10. Complete support-source universe

Freeze:

```text
SupportContributionSourceUniverseV0(H, M)
=
every exact AdmittedContributionV0 in the internally derived
AdmittedContributionAuditV0, after complete upstream composition
validation, ordered by exact ContributionIdentityV0
```

The support layer must not rediscover its source universe from:

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

Ticket 0050 already owns graph structure, policy-lane admission, artifact
identity binding and origin-decision lookup. Candidate audits are consulted
only to verify exact alignment with top-level admitted output.

## 11. Upstream identity checks

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

Any mismatch is a global composition rejection. Mismatched upstream output is
never reinterpreted or repaired.

Immediately after those six checks, validate these fixed compiled-policy cells
independently of admitted contribution count:

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

These global composition checks must run before duplicate detection,
admitted-set alignment, per-contribution invariants, upstream completion
mapping or support projection. They must run for complete-empty and
origin-incomplete admitted audits and when both admitted maps are empty.

## 12. Exact composition-validation order

Freeze:

```text
1. upstream schema
2. upstream canonicalization profile
3. upstream admitted policy
4. upstream origin policy
5. verified-prefix identity
6. closure identity
7. compiled support-ceiling cell
8. compiled support-context cell
9. duplicate candidate-admission identity
10. duplicate top-level-admission identity
11. admitted key-set equality
12. complete admitted-value equality
13. per-contribution invariants
14. upstream completion mapping
15. support-contribution projection
```

An internally inconsistent upstream audit is rejected even when its completion
also reports incomplete origin admission.

Compiled policy-cell failure outranks duplicate or alignment failure occurring
later in the sequence. All composition failure outranks availability
incompleteness. Malformed inherited output or compiled policy drift must not be
hidden behind an availability result.

Upstream completion is mapped only after all identity, compiled-policy,
alignment and per-contribution checks succeed.

## 13. Candidate/top-level alignment law

Build privately:

```text
candidate_admitted:
BTreeMap<ContributionIdentityV0, AdmittedContributionV0>

top_level_admitted:
BTreeMap<ContributionIdentityV0, AdmittedContributionV0>
```

The candidate map includes only:

```text
AdmittedContributionCandidateDispositionV0::Admitted
```

The top-level map includes every exact value returned by:

```text
AdmittedContributionAuditV0::admitted_contributions()
```

Normative law:

```text
candidate_admitted
==
top_level_admitted
```

Equality requires:

```text
same exact key set
+
same complete AdmittedContributionV0 value for every key
```

The complete value includes policy IDs, contribution, namespace, origin group,
evidence kind, claim domain, candidate ceiling and supporting selectors.

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

No first, last, newest, oldest, lexical, top-level-preferred or
candidate-preferred rule exists. No mismatch is locally repaired.

## 14. Exact v0 support lane

The first support policy accepts exactly:

```text
AdmittedContributionV0
× admitted policy magpie-admitted-contribution-v0
× origin policy magpie-origin-admission-v0
× ExternalSource
× ExternalReport
× candidate ceiling Supported
× support_ceiling cell Supported
× support-context requirement NoPrivilegedContext
× sha256 artifact identity
->
SupportContributionV0
```

Positive shape:

```text
exact internally aligned AdmittedContributionV0
+
exact ExternalSource × ExternalReport support cell
+
exact Supported ceiling
+
exact NoPrivilegedContext classification
+
exact sha256 contribution artifact
->
SupportContributionV0
```

`NoPrivilegedContext` means only that the policy cell requires no HumanRoot,
deterministic-verifier or Deadbolt-verifier context. It does not mean
automatically admitted, trusted, true, corroborated, aggregated or achieved
`Supported`.

The complete origin-admission and admitted-contribution paths remain
mandatory.

The support-ceiling cell and support-context requirement are validated once as
fixed global compiled-policy prerequisites. The remaining inherited value
checks are applied to every aligned admitted contribution.

## 15. Per-contribution invariant order

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

7. namespace.origin_admission_policy_id
   == magpie-origin-admission-v0

8. namespace.target_claim_id
   == contribution.target_claim_id

9. namespace.scope_ref
   == contribution.scope_ref

10. supporting_origin_candidate_selectors
    is non-empty
```

Any invariant failure rejects the complete support audit globally. The future
resolver emits no partial support set.

## 16. Completion law

Freeze:

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
identity, compiled-policy cells, alignment and invariants valid
+
upstream completion Complete
->
Complete
```

```text
identity, compiled-policy cells, alignment and invariants valid
+
upstream OriginAdmissionIncomplete
->
AdmittedContributionIncomplete
->
zero support contributions
```

```text
any upstream identity, compiled-policy, alignment or invariant failure
->
UpstreamCompositionRejected
->
zero support contributions
```

Unavailable selectors are inherited exactly and in exact order from
`AdmittedContributionAuditCompletionV0::OriginAdmissionIncomplete`.

A complete aligned admitted audit containing zero admissions obeys:

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

That is not an error and not refutation.

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

## 17. Closed composition-failure vocabulary

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

    ContributionInvariantMismatch {
        contribution: ContributionIdentityV0,
        reason: SupportContributionInvariantReasonV0,
    },
}
```

`SupportCeilingPolicyMismatch` means the compiled
`support_ceiling(ExternalSource, ExternalReport)` cell is not exactly
`Some(Supported)`.

`SupportContextRequirementMismatch` means the compiled
`support_context_requirement(ExternalSource, ExternalReport)` cell is not
exactly `NoPrivilegedContext`.

These two global composition failures carry no `ContributionIdentityV0`. The
future implementation must not attach a synthetic contribution identity,
manufacture an empty or sentinel contribution, or wrap either failure in
`ContributionInvariantMismatch`.

`candidate_only` and `top_level_only` use exact
`ContributionIdentityV0` order.

No `Other`, `UnknownFailure`, free-form reason string, Boolean result or
caller-defined variant is permitted.

## 18. Closed invariant-failure vocabulary

Freeze variants in this exact order:

```rust
pub enum SupportContributionInvariantReasonV0 {
    AdmittedPolicyMismatch,
    OriginPolicyMismatch,
    EvidenceKindMismatch,
    ClaimDomainMismatch,
    CandidateCeilingMismatch,
    ArtifactAlgorithmMismatch,
    NamespacePolicyMismatch,
    NamespaceTargetMismatch,
    NamespaceScopeMismatch,
    MissingSupportingOriginSelector,
}
```

Exact semantics:

```text
CandidateCeilingMismatch:
the admitted value does not carry exact Supported
```

The closed invariant vocabulary contains ten contribution-specific reasons.
Fixed compiled-policy drift is represented only by the two global composition
failures and fails closed.

## 19. Five future public types

Freeze exactly:

```text
SupportContributionAuditCompletionV0
SupportContributionCompositionFailureV0
SupportContributionInvariantReasonV0
SupportContributionV0
SupportContributionAuditV0
```

All future public structs have:

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

All closed public enums have:

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

## 20. Exact field order

`SupportContributionV0`:

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

Exact fixed values:

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

All remaining fields are copied exactly from the aligned admitted value.

`SupportContributionAuditV0`:

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

## 21. Deterministic ordering

Freeze:

```text
candidate admitted map:
exact ContributionIdentityV0 order

top-level admitted map:
exact ContributionIdentityV0 order

candidate_only differences:
exact ContributionIdentityV0 order

top_level_only differences:
exact ContributionIdentityV0 order

support_contributions:
exact ContributionIdentityV0 order

supporting origin selectors:
exact order inherited from AdmittedContributionV0

unavailable binding selectors:
exact order inherited from AdmittedContributionAuditV0
```

When more than one duplicate candidate identity, duplicate top-level identity,
complete-value mismatch or invariant mismatch exists, the failure reports the
first affected contribution in exact `ContributionIdentityV0` order. Within
that contribution, the exact ten-step invariant order selects the reason.

No caller order, event insertion order, group preference or write recency
selects an outcome.

## 22. Canonical serialization

Only the top-level audit exposes:

```rust
pub fn canonical_bytes(&self) -> Vec<u8>;
```

The future implementation serializes directly:

```rust
serde_json::to_vec(self)
```

Determinism comes from exact field order, exact enum tagging and exact ordered
vectors.

Do not define JCS, RFC 8785, generic canonical JSON, post-serialization map
sorting, a new Magpie canonical encoding or a new L0 format.

## 23. Multiplicity law

Freeze:

```text
two distinct admitted contribution identities
assigned to the same namespace and origin group
->
two SupportContributionV0 values
```

The support layer must not key, collapse, deduplicate, score or count by origin
group.

Layer ownership:

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

## 24. Source-standing boundary

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

The generic doctrine that future claim-to-claim support contributions may be
limited by source standing remains valid. It is not applicable to this exact
first evidence-node lane.

## 25. No aggregation-lane decision

Ticket 0051 defines no:

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

Future policy v3 owns which support contributions share a lane, how
same-origin material collapses, whether distinct groups provide
corroboration separation, what threshold applies and what achieved standing
follows.

## 26. Required fixtures

The future implementation PR must add literal canonical fixtures for at least:

```text
one-support-contribution.json
complete-empty.json
admitted-contribution-incomplete.json
same-origin-distinct-contributions.json
distinct-origin-distinct-contributions.json
```

Meanings:

```text
one-support-contribution:
one aligned admission -> Complete -> one support contribution

complete-empty:
complete aligned audit with no admissions
-> Complete -> zero support contributions

admitted-contribution-incomplete:
origin availability incomplete
-> AdmittedContributionIncomplete -> zero support contributions

same-origin-distinct-contributions:
two distinct admissions in one group
-> two support contributions -> no collapse

distinct-origin-distinct-contributions:
two distinct admissions in different groups
-> two support contributions
-> no corroboration or threshold claim
```

Composition rejection may use the smallest private pure helper. The public
resolver must not accept malformed caller-created audits.

Because the production policy cells are fixed, the future implementation may
test policy drift through that smallest private pure composition helper. It
must not add a public policy override, public test switch, runtime policy
parameter, mock policy registry or caller-selected cell.

This contract PR adds no fixture or hash.

## 27. Required hostile tests

The future implementation must prove:

```text
one exact aligned admitted contribution
-> one support contribution

complete admitted audit with no admissions
-> Complete
-> empty output

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
-> inherited selector order
-> zero support contributions

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

same-origin distinct admitted contributions
-> two support contributions

distinct-origin admitted contributions
-> two support contributions
-> no corroboration claim

upstream schema mismatch
-> global rejection

upstream canonicalization-profile mismatch
-> global rejection

upstream admitted-policy mismatch
-> global rejection

upstream origin-policy mismatch
-> global rejection

verified-prefix mismatch
-> global rejection

closure-identity mismatch
-> global rejection

duplicate candidate admission
-> global rejection

duplicate top-level admission
-> global rejection

candidate-only admitted identity
-> global rejection

top-level-only admitted identity
-> global rejection

same identity with differing admitted value
-> global rejection

every per-contribution invariant drift
-> global rejection

composition rejection
-> zero support contributions

caller-provided AdmittedContributionAuditV0
-> no public API

caller-provided AdmittedContributionV0 list
-> no public API

caller-provided origin group or policy
-> no public API

same H + Pₒ + P꜀ + Pₛ + M
-> byte-identical support audit

equivalent closure construction order
-> byte-identical support audit

different H with equal admitted projections
-> different verified-prefix identity and audit bytes

support audit execution
-> OriginAdmissionAuditV0 bytes unchanged
-> AdmittedContributionAuditV0 bytes unchanged
-> standing v0/v1/v2 bytes unchanged
-> support/refutation ceilings unchanged
-> support-context requirements unchanged
```

## 28. Required compile-fail tests

The future implementation must prove:

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

No public test controls are permitted.

## 29. Frozen surfaces

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
new support-contribution fixtures and tests
```

It must not mutate existing admitted-contribution or standing surfaces.

## 30. Explicit non-goals

Do not define or implement:

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

## 31. Future sequence

The next runtime PR must:

1. add the exact constants and public audit types;
2. add the exact context resolver;
3. internally derive one admitted audit;
4. validate the six upstream identities;
5. validate the two fixed compiled support-policy cells independently of
   admitted contribution count;
6. build candidate and top-level admitted maps and reject duplicates;
7. validate exact key-set and complete-value alignment;
8. apply the ten per-contribution invariants;
9. map upstream completion only after all composition checks pass;
10. project aligned admissions one-to-one;
11. serialize the typed top-level audit; and
12. add fixtures, hostile tests and compile-fail tests.

After that:

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

## 32. Stop conditions

Stop and report rather than invent semantics if the contract or future
implementation would require:

- accepting caller-provided admitted audits;
- reopening graph or origin admission;
- selecting contributions from closure objects;
- consuming merely policy-eligible candidate audits;
- preferring candidate or top-level output on mismatch;
- repairing upstream composition locally;
- emitting a partial support set after invariant failure;
- treating `NoPrivilegedContext` as standalone admission;
- treating the support ceiling as achieved standing;
- deduplicating by origin group;
- defining an aggregation lane;
- selecting a group threshold;
- defining policy v3;
- changing admitted-contribution types or bytes;
- changing standing;
- changing policy matrices; or
- editing outside the exact allowlist.

Publication also stops for a dirty tree, wrong repository or remote,
unavailable or unauthenticated `gh`, unmerged PR #63, reviewed-head ancestry
failure, main SHA drift, occupied named branch, existing Ticket 0051, existing
design document or equivalent PR.

## 33. Validation commands

Pre-commit:

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

Authority-language audit:

```powershell
git diff | Select-String -Pattern `
  'support|standing|aggregate|corroborat|independen|trusted|authority|count|threshold|source standing'

git diff | Select-String -Pattern `
  'caller|audit|composition|alignment|NoPrivilegedContext|ceiling|origin_group'
```

Diff audit:

```powershell
git status --short
git diff --name-only
git diff --stat
git diff --check

git diff -- README.md
git diff -- docs/design/admitted-contribution-audit-v0.md
git diff -- docs/design/standing-aggregation-independence-groups.md
git diff -- docs/design/support-contribution-audit-v0.md
git diff -- tickets/0051-support-contribution-audit-v0-contract.md
```

Before commit:

```powershell
git add `
  README.md `
  docs/design/admitted-contribution-audit-v0.md `
  docs/design/standing-aggregation-independence-groups.md `
  docs/design/support-contribution-audit-v0.md `
  tickets/0051-support-contribution-audit-v0-contract.md

git diff --cached --name-only
git diff --cached --stat
git diff --cached --check
git diff --cached
```

After commit:

```powershell
git show --stat --oneline HEAD
git diff-tree --no-commit-id --name-only -r HEAD
git rev-list --count 62026dbcfeff06ef6c6255e5e775113ded8e94fc..HEAD
git status --short
```

Clean-tree release validation:

```powershell
python tools/check_release_metadata.py
cargo package -p magpie-log --locked
git status --short
```

Only commands that actually run and pass may be reported as passing.

## 34. Publication authority

`AGENTS.md` remains binding except that George authorizes, only after the exact
documentation change and every prescribed validation pass:

- staging only the five allowlisted paths;
- creating exactly one commit named
  `docs: ratify support-contribution audit v0`;
- pushing only `agent/support-contribution-audit-v0-contract` without force;
  and
- opening exactly one draft PR titled
  `docs: ratify support-contribution audit v0`.

The worker may not modify `main`, create another branch or commit, mark the PR
ready, merge, enable auto-merge, force-push, resolve reviews, modify another
PR, add runtime code, add tests or fixtures, implement policy v3, count groups
or change standing.

## 35. Reviewer checklist

1. Confirm exact base, reviewed PR #63 head, branch, one-commit boundary and
   five changed paths.
2. Confirm status is ratified contract, runtime absent, support contributions
   absent, aggregation absent and standing unchanged.
3. Confirm deterministic input is exactly `H + Pₒ + P꜀ + Pₛ + M`.
4. Confirm all policy/schema/profile identities and upstream constant reuse.
5. Confirm the only public resolver is the exact context method.
6. Confirm the admitted audit is internally derived and caller-provided output
   is never authority.
7. Confirm the complete support-source universe is the aligned top-level
   admitted set in exact contribution order.
8. Confirm graph, artifact and origin admission are not reopened.
9. Confirm exact six-step upstream identity validation.
10. Confirm exact fifteen-step composition-validation order.
11. Confirm the support-ceiling and support-context cells are global checks.
12. Confirm the global policy cells are checked for empty admitted sets.
13. Confirm the global policy cells are checked before completion mapping.
14. Confirm policy drift outranks origin incompleteness.
15. Confirm the two global policy failures carry no contribution identity.
16. Confirm candidate and top-level admissions become maps with duplicate
    rejection.
17. Confirm identical keys and complete values are required.
18. Confirm no representation wins or repairs mismatch.
19. Confirm the one exact v0 lane and `NoPrivilegedContext` meaning.
20. Confirm the invariant vocabulary contains ten contribution-specific
    reasons in the exact ten-step first-failure order.
21. Confirm composition failure precedes upstream incompleteness.
22. Confirm complete-empty remains valid only when global policy validation
    succeeds.
23. Confirm all completion variants and zero-output failure laws.
24. Confirm the closed composition and invariant failure vocabularies.
25. Confirm all five future public types, traits, enum tagging and field order.
26. Confirm only the top-level audit exposes `canonical_bytes()`.
27. Confirm deterministic ordering and direct typed JSON serialization.
28. Confirm distinct same-origin admitted contributions remain distinct
    support inputs.
29. Confirm no group count, collapse, lane, threshold or corroboration result.
30. Confirm the v0 source endpoint is evidence, not a source claim with
    standing.
31. Confirm required fixtures and hostile/compile-fail tests are reserved for
    the runtime PR.
32. Confirm every frozen surface remains unchanged.
33. Confirm no support-contribution runtime, policy v3, aggregation, standing,
    writer, loader, CAS, filesystem/network, L0, Cargo, dependency, fixture or
    Deadbolt change appears.

## 36. Precise contract claim

```text
Magpie now has a ratified contract for deterministically
transforming an internally derived and completely aligned set of
AdmittedContributionV0 values into exact, ceiling-bounded positive
support inputs under one compiled policy.

No support-contribution runtime exists yet.

No origin group is collapsed or counted.

No aggregation lane or threshold is selected.

No achieved standing changes.
```
