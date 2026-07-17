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
only to verify globally unique edge IDs across the complete candidate
universe, exact alignment first between every `Admitted` candidate's enclosing
replay-derived trace and its nested admitted value, then between trace-aligned
candidate admissions and top-level admitted output, plus exact
completion shape and completion/disposition consistency.

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

These global composition checks must run before duplicate candidate edge-ID
validation, admitted-candidate trace consistency, candidate-map insertion,
contribution-identity duplicate detection, admitted-set alignment, upstream
completion shape and completion/disposition consistency, per-contribution
invariants, upstream completion mapping or support projection. They must run
for complete-empty and origin-incomplete admitted audits and when both
admitted maps are empty.

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

9. duplicate candidate edge ID

10. admitted-candidate trace consistency

11. duplicate candidate-admission identity

12. duplicate top-level-admission identity

13. admitted key-set equality

14. complete admitted-value equality

15. upstream completion shape and completion/disposition consistency

16. per-contribution invariants

17. upstream completion mapping

18. support-contribution projection
```

An internally inconsistent upstream audit is rejected even when its completion
also reports incomplete origin admission.

Normative precedence is:

```text
upstream identity failure
precedes
compiled-policy failure

compiled-policy failure
precedes
duplicate candidate edge-ID failure

duplicate candidate edge-ID failure
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

The empty-unavailable-selector shape failure joins the completion/disposition
failure class. It is reported within stage 15 before any candidate-agreement
comparison, and it still precedes per-contribution invariant failure,
completion mapping and projection.

Upstream completion is mapped only after all identity, compiled-policy,
candidate-universe uniqueness, candidate-trace, alignment,
completion/disposition and per-contribution checks succeed. No partial
support-contribution vector may survive any failure.

## 13. Admitted-candidate trace and candidate/top-level alignment laws

### Candidate-universe edge-ID uniqueness

Immediately after the two fixed compiled-policy checks, validate the exact
`edge_id` of every candidate audit, regardless of disposition.

The complete candidate universe must satisfy:

```text
every candidate audit has a globally unique exact edge_id
```

This check includes candidates whose dispositions are:

```text
PolicyIneligible
OriginAdmissionIncomplete
OriginDecisionAbsent
OriginNotAdmitted
Admitted
```

It must execute before admitted-candidate trace consistency, candidate
admitted-map insertion, contribution-identity duplicate checks,
candidate/top-level alignment, completion shape and completion/disposition
validation, per-contribution invariants, completion mapping or projection.

The future implementation must derive duplicate detection structurally and
deterministically. If more than one edge ID is duplicated, it reports the
lexically first exact duplicated edge ID, independent of caller or vector
order.

It must not repair, deduplicate, prefer one candidate, silently collapse
entries or limit the check to `Admitted` candidates.

Any duplicate produces exactly:

```rust
DuplicateCandidateEdgeId {
    edge_id: String,
}
```

This is a global candidate-universe composition failure. It carries no
synthetic `ContributionIdentityV0`.

```text
duplicate candidate edge ID
->
UpstreamCompositionRejected
->
zero support contributions
```

### Admitted-candidate trace consistency

Immediately after duplicate candidate edge-ID validation, and before candidate
admitted-map insertion, duplicate candidate-admission detection, top-level
alignment, completion shape and completion/disposition validation,
per-contribution invariants, completion mapping or projection, validate every
candidate whose disposition is exactly:

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

### Upstream completion shape and completion/disposition consistency

After exact admitted key-set and complete-value equality succeed, first
validate the shape of the inherited completion value itself, then validate
the upstream completion against every candidate disposition.

Freeze this exact shape law:

```text
AdmittedContributionAuditCompletionV0::OriginAdmissionIncomplete
requires a non-empty unavailable_binding_selectors vector
```

The upstream constructor produces `Complete` whenever the
unavailable-selector vector is empty, so an inherited incomplete completion
carrying an empty vector is unrepresentable from honest upstream output. It
is malformed inherited output, and malformed inherited output is rejected,
never interpreted. The empty-vector value is never normalized to `Complete`,
never mapped to `AdmittedContributionIncomplete` and never repaired.

The shape check executes before any candidate-agreement comparison within
this stage. An empty-vector incomplete completion plus any disposition
violation reports `EmptyUnavailableBindingSelectors`.

Freeze these exact disposition laws:

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

An incomplete upstream audit must contain no:

```text
OriginDecisionAbsent
OriginNotAdmitted
Admitted
```

A complete upstream audit may contain `PolicyIneligible`,
`OriginDecisionAbsent`, `OriginNotAdmitted` and `Admitted`, but never
`OriginAdmissionIncomplete`.

An incomplete audit containing only `PolicyIneligible` candidates is valid.
The future implementation must not require at least one
`OriginAdmissionIncomplete` candidate. The shape check distinguishes shape
from content: an empty selector vector is impossible-by-construction, while
a non-empty selector vector with only `PolicyIneligible` candidates is
legitimate and maps to `AdmittedContributionIncomplete`.

Failure laws:

```text
Complete + OriginAdmissionIncomplete candidate
-> UpstreamCompositionRejected

OriginAdmissionIncomplete + OriginDecisionAbsent candidate
-> UpstreamCompositionRejected

OriginAdmissionIncomplete + OriginNotAdmitted candidate
-> UpstreamCompositionRejected

OriginAdmissionIncomplete + aligned Admitted candidate/top-level value
-> UpstreamCompositionRejected
```

The failure reports the first inconsistent candidate in exact candidate
edge-ID order. It adds no public enum and no free-form reason.

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
× four exact replay references with byte length 1..=1024
× sha256 artifact identity with an exact lowercase-hex-64 digest
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
four exact inherited replay references with byte length 1..=1024
+
exact sha256 contribution artifact with a 64-character lowercase
hexadecimal digest
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

6. contribution.target_claim_id
   satisfies the exact origin-binding replay-reference grammar

7. contribution.source_evidence_id
   satisfies the exact origin-binding replay-reference grammar

8. contribution.justification_edge_id
   satisfies the exact origin-binding replay-reference grammar

9. contribution.scope_ref
   satisfies the exact origin-binding replay-reference grammar

10. contribution.artifact_algorithm
   == sha256

11. contribution.artifact_digest
    is exactly 64 lowercase hexadecimal ASCII characters

12. namespace.origin_admission_policy_id
    == magpie-origin-admission-v0

13. namespace.target_claim_id
    == contribution.target_claim_id

14. namespace.scope_ref
    == contribution.scope_ref

15. origin_group
    satisfies the exact origin-binding protocol-identifier grammar

16. supporting_origin_candidate_selectors
    is non-empty

17. every supporting selector
    satisfies the exact origin-binding selector grammar

18. supporting selector vector
    is strictly increasing under exact derived Ord
```

Any invariant failure rejects the complete support audit globally. The future
resolver emits no partial support set.

Exact replay-reference grammar:

```text
byte length:
1..=1024

no other normalization or alphabet restriction
```

Validation is over exact UTF-8 bytes through string byte length. Empty values
and values longer than 1,024 bytes are invalid. The implementation must not
trim, normalize, truncate, reinterpret or repair any reference.

The support-contribution implementation must apply this grammar separately to:

```text
ContributionIdentityV0.target_claim_id
ContributionIdentityV0.source_evidence_id
ContributionIdentityV0.justification_edge_id
ContributionIdentityV0.scope_ref
```

The namespace target and scope equalities remain mandatory after the
corresponding contribution references have been validated. Equality does not
replace validity, and validity does not replace equality.

Exact artifact-digest grammar:

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

Exact origin-group grammar:

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

Validation uses the exact inherited bytes. No trimming, normalization, Unicode
folding, case conversion, aliasing or repair is permitted.

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

The later runtime implementation must add and use exactly these four narrow
crate-private reuse seams in
`crates/magpie-claims/src/origin_binding_verifier.rs`:

```rust
pub(crate) fn valid_origin_binding_replay_reference_v0(
    value: &str,
) -> bool {
    !value.is_empty() && value.len() <= 1_024
}

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
valid_origin_binding_replay_reference_v0
-> exact existing replay-reference byte-length grammar

existing validate_replay_reference
-> valid_origin_binding_replay_reference_v0

valid_origin_group_v0
-> existing valid_identifier

valid_origin_binding_artifact_digest_v0
-> existing is_lowercase_hex_64

valid_origin_binding_selector_v0
-> existing origin-binding constants
 + existing is_lowercase_hex_64
 + existing valid_identifier
```

The later runtime PR must make the existing private
`validate_replay_reference` delegate to
`valid_origin_binding_replay_reference_v0`, so the replay-reference grammar has
one implementation source. The support-audit implementation must call that
helper for all four inherited replay-reference fields and must call the other
three helpers for their exact inherited values.

All four helpers remain crate-private and expose:

```text
no public API
no configurable maximum
no caller-selected grammar
no normalization
no alternate validation profile
no test-only public switch
```

They accept no configurable expected kind, profile, algorithm, length or
alphabet and change no origin-binding behaviour, output or canonical bytes.

The future implementation must not expose a generic public identifier
validator, caller-selected validation profile, runtime grammar registry, new
policy input, test-only public switch, closure lookup, anchor lookup or bundle
re-verification. It must not reuse `valid_provenance_selector`, because that
helper validates the inner artifact-provenance acquisition and derivation
selector kinds rather than the two outer origin-binding selector kinds.

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
identity, compiled-policy cells, candidate edge-ID uniqueness,
admitted-candidate trace, alignment, completion/disposition and invariants
valid
+
upstream completion Complete
->
Complete
```

```text
identity, compiled-policy cells, candidate edge-ID uniqueness,
admitted-candidate trace, alignment, completion/disposition and invariants
valid
+
upstream OriginAdmissionIncomplete
->
AdmittedContributionIncomplete
->
zero support contributions
```

```text
any upstream identity, compiled-policy, candidate edge-ID uniqueness,
admitted-candidate trace, alignment, completion/disposition or invariant
failure
->
UpstreamCompositionRejected
->
zero support contributions
```

```text
upstream OriginAdmissionIncomplete
+ empty unavailable_binding_selectors
->
UpstreamCompositionRejected {
    EmptyUnavailableBindingSelectors
}
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

    DuplicateCandidateEdgeId {
        edge_id: String,
    },

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

    EmptyUnavailableBindingSelectors,

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

These two global policy-cell failures, the candidate-universe uniqueness
failure and the empty-unavailable-selector shape failure carry no
`ContributionIdentityV0`. The future implementation must not attach a
synthetic contribution identity, manufacture an empty or sentinel
contribution, or wrap any of them in `ContributionInvariantMismatch`.

`DuplicateCandidateEdgeId` means the complete candidate-audit universe
contains more than one candidate with the exact reported `edge_id`. It reports
the lexically first exact duplicated edge ID, independent of caller or vector
order, and is checked across all candidate audits before admitted-candidate
trace validation or candidate admitted-map insertion.

`AdmittedCandidateTraceMismatch` reports the exact `edge_id` of the first
malformed admitted candidate in exact candidate edge-ID order. It carries no
free-form field name or generic reason; the fixed eight-field comparison order
is normative.

`UpstreamCompletionDispositionMismatch` reports the exact `edge_id` of the
first candidate whose disposition is inconsistent with the upstream
completion. Candidate selection uses exact edge-ID order.

`EmptyUnavailableBindingSelectors` reports an inherited incomplete completion
carrying an empty unavailable-selector vector. The upstream constructor
produces `Complete` whenever that vector is empty, so this value is
unrepresentable from honest upstream output. It is never normalized to
`Complete`, never mapped to `AdmittedContributionIncomplete` and never
repaired. It carries no `ContributionIdentityV0` and no candidate `edge_id`:
there is no candidate edge to report, and fabricating one would be a
synthetic value. It must not be folded into
`UpstreamCompletionDispositionMismatch`.

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
    InvalidTargetClaimReference,
    InvalidSourceEvidenceReference,
    InvalidJustificationEdgeReference,
    InvalidScopeReference,
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

Exact semantics:

```text
CandidateCeilingMismatch:
the admitted value does not carry exact Supported

InvalidTargetClaimReference:
the inherited contribution target_claim_id does not satisfy the exact
origin-binding replay-reference grammar

InvalidSourceEvidenceReference:
the inherited contribution source_evidence_id does not satisfy the exact
origin-binding replay-reference grammar

InvalidJustificationEdgeReference:
the inherited contribution justification_edge_id does not satisfy the exact
origin-binding replay-reference grammar

InvalidScopeReference:
the inherited contribution scope_ref does not satisfy the exact origin-binding
replay-reference grammar

InvalidArtifactDigest:
the inherited contribution artifact digest is not exactly 64 lowercase
hexadecimal ASCII characters

InvalidOriginGroup:
the exact inherited origin_group does not satisfy the existing
origin-binding protocol-identifier grammar

MissingSupportingOriginSelector:
the inherited supporting selector vector is empty

InvalidSupportingOriginSelector:
at least one inherited supporting selector fails the exact structural
origin-binding selector grammar

NonCanonicalSupportingOriginSelectorOrder:
the non-empty, individually valid supporting selector vector is not strictly
increasing under ArtifactProvenanceAnchorSelectorV0::Ord
```

The closed invariant vocabulary contains eighteen contribution-specific
reasons.
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

duplicate candidate edge ID:
lexically first exact duplicated edge ID across all candidate audits,
independent of caller or vector order

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

upstream completion shape:
the empty-unavailable-selector shape check executes before any
candidate-agreement comparison within the completion/disposition stage;
an empty-vector incomplete completion plus any disposition violation
reports EmptyUnavailableBindingSelectors
```

When more than one duplicated edge ID exists, the failure reports the
lexically first exact duplicated edge ID. When more than one duplicate
candidate identity, duplicate top-level identity, complete-value mismatch or
invariant mismatch exists, the failure reports the first affected contribution
in exact `ContributionIdentityV0` order. Within that contribution, the exact
eighteen-step invariant order selects the reason. Individual selector validity
is checked before selector-vector canonicality.

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

Malformed inherited-audit composition cases may use the smallest private pure
helper. The public resolver must not accept malformed caller-created audits.

No additional normative fixture file is required for malformed composition
cases.

Because the production policy cells are fixed, the future implementation may
test policy drift through that smallest private pure composition helper. It
must not add a public policy override, public test switch, runtime policy
parameter, mock policy registry or caller-selected cell.

The helper must not add public constructors, audit inputs, grammar parameters
or test switches.

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

empty-vector incomplete completion
+ only PolicyIneligible candidates
->
UpstreamCompositionRejected { EmptyUnavailableBindingSelectors }
-> zero support contributions

empty-vector incomplete completion
+ empty candidate universe
-> rejected (never Complete)

empty-vector incomplete completion
+ any disposition violation (e.g. OriginNotAdmitted candidate)
-> EmptyUnavailableBindingSelectors wins by precedence

non-empty-vector incomplete completion
+ only PolicyIneligible candidates
-> accepted completion mapping (regression guard)

the failure carries no contribution identity;
the selector vector is never defaulted, truncated, normalized or repaired

the upstream Ticket 0048 constructor yields Complete
if and only if the unavailable-selector vector is empty,
so the downstream shape check cannot silently drift from upstream behavior

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

two admitted candidates with the same edge ID
but different source evidence IDs and distinct full contribution identities
->
DuplicateCandidateEdgeId
->
zero support contributions

duplicate edge IDs among non-Admitted candidates
->
DuplicateCandidateEdgeId

one admitted and one non-Admitted candidate sharing an edge ID
->
DuplicateCandidateEdgeId

multiple duplicated edge IDs
->
lexically first exact duplicated edge ID reported

duplicate edge ID plus admitted-candidate trace mismatch
->
DuplicateCandidateEdgeId wins by precedence

duplicate edge ID
->
detected before candidate admitted-map insertion

valid unique candidate edge IDs
->
normal composition continues

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

empty contribution.target_claim_id
->
InvalidTargetClaimReference

1,025-byte contribution.target_claim_id
->
InvalidTargetClaimReference

1-byte contribution.target_claim_id
->
valid replay-reference boundary

1,024-byte contribution.target_claim_id
->
valid replay-reference boundary

empty contribution.source_evidence_id
->
InvalidSourceEvidenceReference

1,025-byte contribution.source_evidence_id
->
InvalidSourceEvidenceReference

1-byte contribution.source_evidence_id
->
valid replay-reference boundary

1,024-byte contribution.source_evidence_id
->
valid replay-reference boundary

empty contribution.justification_edge_id
->
InvalidJustificationEdgeReference

1,025-byte contribution.justification_edge_id
->
InvalidJustificationEdgeReference

1-byte contribution.justification_edge_id
->
valid replay-reference boundary

1,024-byte contribution.justification_edge_id
->
valid replay-reference boundary

empty contribution.scope_ref
->
InvalidScopeReference

1,025-byte contribution.scope_ref
->
InvalidScopeReference

1-byte contribution.scope_ref
->
valid replay-reference boundary

1,024-byte contribution.scope_ref
->
valid replay-reference boundary

equal malformed contribution and namespace target or scope values
->
corresponding invalid-reference reason
->
rejected despite equality

non-ASCII UTF-8 replay reference within the byte bound
->
valid

multibyte UTF-8 replay reference exceeding 1,024 bytes
->
corresponding invalid-reference reason

invalid replay reference plus artifact algorithm, artifact digest or namespace
equality failure
->
invalid replay-reference reason wins by precedence

invalid replay reference
->
zero support contributions

invalid replay reference
->
no trimming, truncation, normalization or repair

empty group
-> InvalidOriginGroup

129-byte group
-> InvalidOriginGroup

non-ASCII group
-> InvalidOriginGroup

group beginning with punctuation
-> InvalidOriginGroup

group containing whitespace or unsupported punctuation
-> InvalidOriginGroup

1-byte ASCII-alphanumeric group
-> valid

128-byte valid group
-> valid

allowed . _ : / - characters after the first byte
-> valid

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
four exact crate-private origin-binding grammar reuse helpers:
- valid_origin_binding_replay_reference_v0
- valid_origin_group_v0
- valid_origin_binding_artifact_digest_v0
- valid_origin_binding_selector_v0
new support-contribution fixtures and tests
```

These four crate-private helpers and the required delegation of the existing
private `validate_replay_reference` implementation are the only permitted
origin-binding module changes. They must delegate exactly as frozen above and
must not change origin-binding behaviour, output, canonical bytes or public
API. The later implementation must not otherwise mutate existing
origin-binding, admitted-contribution or standing surfaces.

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

The next runtime PR must implement this contract in this exact order:

1. add exact constants and five public audit types;
2. add the exact context resolver;
3. internally derive one admitted-contribution audit;
4. validate the six upstream audit identities;
5. validate the two fixed compiled support-policy cells;
6. validate unique edge IDs across every candidate audit;
7. validate every `Admitted` candidate's enclosing trace against its nested
   `AdmittedContributionV0`;
8. build candidate and top-level admitted maps and reject duplicate identities;
9. validate exact admitted key-set and complete-value equality;
10. validate upstream completion shape and completion/disposition
    consistency;
11. apply the eighteen per-contribution invariants;
12. map upstream completion only after every composition and invariant check
    succeeds;
13. project aligned admissions one-to-one in exact contribution order;
14. serialize the typed top-level audit directly; and
15. add fixtures, hostile tests and compile-fail tests.

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

The future loader contract must validate the inherited unavailable-selector
universe before consuming it operationally. This v0 contract inherits that
vector opaquely; the first layer that loads against it becomes its first
validating consumer.

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

For the PR #64 review-remediation follow-ups, explicit task authority
additionally permitted only editing
`docs/design/support-contribution-audit-v0.md` and
`tickets/0051-support-contribution-audit-v0-contract.md`, pushing the
existing `agent/support-contribution-audit-v0-contract` branch without
force, updating the existing PR #64 body to describe the amended contract,
and creating these remediation commits in order:

1. `ab5de3e` — `docs: harden support-policy composition`;
2. `7014ef5` — `docs: harden inherited support inputs`;
3. `c62a33f` — `docs: close support-input validation gaps`;
4. `9313122` — `docs: close support replay-identity gaps`;
5. `8349955a` — `docs: reject empty unavailable-selector incompletions`;
   and
6. `docs: align stage-15 narrative and ticket authority record` (this
   commit).

This follow-up authority does not permit replying to or resolving review
threads. Those GitHub writes require separate authorization.

## 35. Reviewer checklist

1. Confirm exact base, reviewed PR #63 head, branch, original five-path scope
   and the six review-remediation commits enumerated in §34, each limited to
   the two amended contract paths.
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
10. Confirm exact eighteen-stage composition-validation order.
11. Confirm the support-ceiling and support-context cells are global checks.
12. Confirm the global policy cells are checked for empty admitted sets.
13. Confirm the global policy cells are checked before completion mapping.
14. Confirm policy drift outranks origin incompleteness.
15. Confirm the two global policy failures carry no contribution identity.
16. Confirm candidate and top-level admissions become maps with duplicate
    rejection.
17. Confirm identical keys and complete values are required.
18. Confirm no representation wins or repairs mismatch.
19. Confirm upstream completion agrees with every candidate disposition.
20. Confirm the first inconsistent edge is deterministic in exact edge-ID
    order.
21. Confirm incomplete audits cannot contain terminal origin decisions or
    admissions.
22. Confirm complete audits cannot contain `OriginAdmissionIncomplete`
    candidates.
23. Confirm the one exact v0 lane and `NoPrivilegedContext` meaning.
24. Confirm `origin_group` is validated under the exact existing
    protocol-identifier grammar.
25. Confirm the implementation permits exactly the four crate-private reuse
    helpers `valid_origin_binding_replay_reference_v0`,
    `valid_origin_group_v0`,
    `valid_origin_binding_artifact_digest_v0` and
    `valid_origin_binding_selector_v0`.
26. Confirm the invariant vocabulary contains eighteen contribution-specific
    reasons in the exact eighteen-step first-failure order.
27. Confirm composition failure precedes upstream incompleteness.
28. Confirm complete-empty remains valid only when global policy validation
    succeeds.
29. Confirm all completion variants and zero-output failure laws.
30. Confirm the closed composition and invariant failure vocabularies.
31. Confirm all five future public types, traits, enum tagging and field order.
32. Confirm only the top-level audit exposes `canonical_bytes()`.
33. Confirm deterministic ordering and direct typed JSON serialization.
34. Confirm distinct same-origin admitted contributions remain distinct
    support inputs.
35. Confirm no group count, collapse, lane, threshold or corroboration result.
36. Confirm the v0 source endpoint is evidence, not a source claim with
    standing.
37. Confirm required fixtures and hostile/compile-fail tests are reserved for
    the runtime PR.
38. Confirm every frozen surface remains unchanged.
39. Confirm no support-contribution runtime, policy v3, aggregation, standing,
    writer, loader, CAS, filesystem/network, L0, Cargo, dependency, fixture or
    Deadbolt change appears.
40. Confirm every `Admitted` candidate's enclosing trace agrees with its nested
    `AdmittedContributionV0` across all eight exact fields.
41. Confirm candidate trace validation occurs before candidate-map insertion.
42. Confirm candidate trace mismatch reports the first exact edge ID.
43. Confirm the artifact digest is exactly 64 lowercase hexadecimal ASCII
    characters.
44. Confirm artifact-digest grammar reuses the exact crate-private
    `valid_origin_binding_artifact_digest_v0` helper.
45. Confirm every supporting selector is structurally valid.
46. Confirm selector bundle kind is one of the two exact origin-binding kinds.
47. Confirm selector witness root is exact lowercase hexadecimal-64.
48. Confirm selector witness algorithm and canonicalization profile use the
    exact fixed origin-binding constants.
49. Confirm selector run ID uses the exact protocol-identifier grammar.
50. Confirm supporting selectors are non-empty.
51. Confirm supporting selectors are strictly ordered under exact derived
    `Ord` and duplicate-free.
52. Confirm malformed selector vectors are never sorted, deduplicated or
    repaired.
53. Confirm the global composition sequence contains eighteen exact stages.
54. Confirm the invariant vocabulary contains eighteen exact reasons.
55. Confirm duplicate edge IDs are rejected across every candidate disposition.
56. Confirm duplicate candidate-edge rejection reports the lexically first
    exact duplicated edge ID independent of caller or vector order.
57. Confirm duplicate candidate-edge rejection carries no contribution
    identity, precedes candidate trace validation and candidate admitted-map
    insertion, and produces zero support contributions.
58. Confirm all four inherited replay references use exact UTF-8 byte length
    `1..=1024` with no normalization or alphabet restriction.
59. Confirm replay-reference validation precedes artifact algorithm/digest and
    namespace-equality checks.
60. Confirm namespace target/scope equality remains mandatory after reference
    validity succeeds.
61. Confirm the existing private `validate_replay_reference` implementation
    delegates to `valid_origin_binding_replay_reference_v0`.
62. Confirm every invalid replay reference rejects the complete audit with zero
    support contributions.
63. Confirm the composition-validation order remains exactly eighteen stages;
    stage 15 is upstream completion shape and completion/disposition
    consistency, and no nineteenth stage was added.
64. Confirm the empty-unavailable-selector shape check executes before any
    candidate-agreement comparison within stage 15.
65. Confirm an inherited incomplete completion with an empty
    unavailable-selector vector is rejected as
    `EmptyUnavailableBindingSelectors` — never normalized to `Complete`,
    never mapped to `AdmittedContributionIncomplete` and never repaired.
66. Confirm `EmptyUnavailableBindingSelectors` carries no contribution
    identity and no edge ID, and joins the completion/disposition failure
    class.
67. Confirm the non-empty-vector, only-`PolicyIneligible` case remains valid
    and maps to `AdmittedContributionIncomplete`.
68. Confirm the hostile-test reservations cover the empty-vector rejection,
    the precedence win over disposition violations, the non-empty regression
    guard and the upstream Ticket 0048 constructor invariant pin.

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
