# Ticket 0053: Standing policy v3 external-report corroboration contract

## 1. Exact base

```text
base:
53799f92d8e7a1dc3467c24fc38896684e1f1945
```

The base is the merge commit of PR #65 (Ticket 0052, the
support-contribution audit v0 runtime).

## 2. Branch, commits and PR title

```text
branch:
agent/standing-policy-v3-external-report-corroboration-contract

commit:
docs: ratify standing policy v3 corroboration rule

PR title:
docs: ratify standing policy v3 corroboration rule

PR state:
draft
```

## 3. Documentation-only classification

This ticket ratifies the exact standing policy v3 external-report
corroboration contract:

```text
complete internally derived SupportContributionAuditV0
+
exact requested ExternalReport claim
+
at least two distinct admitted origin groups
+
one exact permitted aggregation lane
->
governed Supported
```

Exact classification:

```text
correctness:
complete standing policy v3 corroboration contract

authority:
internally derived StandingResolutionV2 and
SupportContributionAuditV0 only

positive effect:
typed contract only

standing:
unchanged

aggregation:
contract ratified; not implemented

runtime:
not implemented
```

This ticket changes documentation only. It adds no Rust, tests, fixtures,
Cargo metadata, dependency, release file, tool, CI, L0 surface or runtime
capability.

## 4. Exact six-path allowlist

Only:

```text
README.md
docs/design/standing-aggregation-independence-groups.md
docs/design/standing-view-evidence-ceilings.md
docs/design/support-contribution-audit-v0.md
docs/design/standing-policy-v3-external-report-corroboration-v0.md  (new)
tickets/0053-standing-policy-v3-external-report-corroboration-contract.md  (new)
```

No other path may change.

In particular, this ticket changes no Rust, fixture, test, Cargo,
lockfile, release, tool, CI, existing ticket, origin-admission design,
admitted-contribution design, origin-binding design, or `AGENTS.md` path.

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
->
support-contribution audit
```

Ticket 0053 freezes:

```text
support contribution
!= counted origin group

counted origin group
!= proof of independence

corroboration separation
!= truth

aggregation result
!= stored standing

Supported
!= Settled
```

The v3 rule answers only:

```text
Do at least two distinct admitted origin groups support this exact
ExternalReport claim inside one exact permitted lane?
```

It does not answer:

```text
Are two groups statistically independent?
Is any source trustworthy?
Has the claim reached Settled?
Has refutation, contradiction debt, invalidation, or supersession
occurred?
What is the standing of the source itself?
```

## 6. Central deterministic law

Freeze:

```text
H:
the exact completely verified Magpie log prefix carried by
OriginAdmissionReplayContextV0

P₂:
the exact compiled standing policy magpie-claims-standing-v2

P₃:
the exact compiled standing policy magpie-claims-standing-v3

Pₛ:
the exact compiled support-contribution policy
magpie-support-contribution-v0

M:
the exact immutable ResolutionContentClosureV0 and its complete
four-field identity

same H + same P₂ + same P₃ + same Pₛ + same M + same requested claim
->
byte-identical StandingResolutionV3
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
standing policy v3:
magpie-claims-standing-v3

single closed rule:
StandingPolicyRuleV3::ExternalReportCorroborationV0

corroboration threshold:
EXTERNAL_REPORT_CORROBORATION_MIN_DISTINCT_ORIGIN_GROUPS_V0 = 2

required inherited standing policy:
magpie-claims-standing-v2

required support-contribution policy:
magpie-support-contribution-v0

required admitted-contribution policy:
magpie-admitted-contribution-v0

required origin-admission policy:
magpie-origin-admission-v0
```

Future public constants:

```rust
pub const MAGPIE_CLAIMS_POLICY_V3_ID: &str = "magpie-claims-standing-v3";

pub const EXTERNAL_REPORT_CORROBORATION_MIN_DISTINCT_ORIGIN_GROUPS_V0: usize = 2;
```

The future implementation must reuse:

```text
MAGPIE_CLAIMS_POLICY_V2_ID
SUPPORT_CONTRIBUTION_AUDIT_SCHEMA_V0
SUPPORT_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0
SUPPORT_CONTRIBUTION_POLICY_ID_V0
ADMITTED_CONTRIBUTION_POLICY_ID_V0
ORIGIN_ADMISSION_POLICY_ID_V0
```

It must not define duplicate upstream constants.

There is no runtime selector, registry, alias, negotiation, environment
override, caller-selected policy, bundle-selected implementation or
implicit latest version. Serialized policy IDs disclose compiled code;
they do not select it. The threshold is a fixed compiled constant, never
a runtime argument, configuration value, metadata field, bundle field, or
caller choice.

## 8. Future resolver surface

Reserve exactly:

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

The scalar method delegates to the trace-bearing method and returns only
`governed_standing`. No free resolver, `StandingView` resolver,
standalone `StandingReplaySnapshot` v3 method, generic policy parameter,
or second standing engine is permitted. Only `StandingResolutionV3`
exposes `canonical_bytes()`, through direct typed
`serde_json::to_vec(self)`.

## 9. Exact authority path

The future resolver performs exactly:

```text
1. derive inherited StandingResolutionV2 internally from the context's
   own snapshot for the exact requested claim

2. derive SupportContributionAuditV0 internally from the same context
   and the same closure (the existing
   resolve_support_contribution_audit_v0 path)

3. compose one crate-private pure v3 evaluation

4. return StandingResolutionV3
```

The resolver and composer must not accept:

```text
SupportContributionAuditV0
SupportContributionV0 values
origin groups
contribution lists
lane IDs
thresholds
policy IDs
StandingView
standalone StandingReplaySnapshot authority
callbacks
lookup providers
serialized or cloned audit bytes
```

A cloned or serialized audit is output, not authority.

## 10. Exact aggregation-lane identity

The aggregation lane is a typed partition key, not the contribution-lane
identity, not a string, and not a second rule identifier.

Exactly one field partitions contributions before group counting:

```text
namespace:
the exact OriginComparisonNamespaceV0
(exact origin-admission policy ID, exact target claim ID, exact scope_ref)
```

Every counted contribution must satisfy these fixed membership
invariants, which do not vary within honest v0 output:

```text
support_contribution_policy_id == magpie-support-contribution-v0
admitted_contribution_policy_id == magpie-admitted-contribution-v0
evidence_kind == ExternalSource
claim_domain == ExternalReport
support_ceiling == Supported
```

The lane key must not include source evidence ID, justification-edge ID,
artifact digest, supporting selector, occurrence, or complete
contribution identity. A staged or internal value drifting in any fixed
membership field is an outer composition failure, never a second eligible
lane.

## 11. Exact threshold

Freeze:

```text
distinct_group_count
>= EXTERNAL_REPORT_CORROBORATION_MIN_DISTINCT_ORIGIN_GROUPS_V0
-> corroboration separation satisfied
```

One or zero distinct admitted origin groups produce no new standing.
Contribution count is not origin-group count. The threshold is populated
from compiled code only; it may appear in the explanation as audit
disclosure but never as caller input.

## 12. Exact claim-selection law

Only support contributions whose exact `target_claim_id` equals the
requested claim may enter the rule.

For every such contribution, `namespace.target_claim_id` must equal the
requested claim and `namespace.scope_ref` must equal the exact replayed
`TypedClaimNode.scope_ref` of the requested claim. A same-target
contribution whose namespace target or scope drifts from the replayed
values is an outer composition failure, not an ignored contribution.

Contributions for other exact targets remain visible as ignored,
non-counting audit material ordered by exact `ContributionIdentityV0`.
They can never affect the result.

The requested target must remain the exact replayed `ExternalReport`
claim in the exact shared scope. Nearby IDs, prose equality, shared
digests, shared evidence, aliases, summaries, or metadata may not
substitute. Policy v3 adds no new claim classification: an absent,
malformed, wrong-domain, duplicate-domain, or unknown-domain requested
claim is handled exactly as the inherited v2 resolution handles it, and
yields no v3 standing effect.

## 13. Exact same-origin collapse and group-counting law

Within one eligible lane:

```text
group retained contributions by exact origin_group string
-> count distinct exact group keys
-> success iff distinct_group_count >= 2
```

Freeze:

- multiple distinct contributions in one admitted group count at most
  once;
- every contribution is preserved in explanation material; no
  `SupportContributionV0` value is deleted, deduplicated, or rewritten;
- all distinct groups, not an arbitrary first two, remain counted and
  visible;
- group keys compare as exact strings with no case folding, trimming,
  normalization, Unicode folding, or aliasing;
- two textual group keys are comparable only inside the same exact
  `OriginComparisonNamespaceV0`; different namespaces never combine;
- unknown, absent, malformed, conflicting, unadmitted, incomplete, or
  rejected origin material provides no amplification;
- two distinct admitted groups sharing one artifact digest or supporting
  selector still count by admitted exact group keys; no digest or
  metadata clustering veto exists;
- a staged duplicate exact `ContributionIdentityV0` can never manufacture
  a second group.

## 14. Exact completion law

```text
Complete
-> the rule may evaluate groups

AdmittedContributionIncomplete
-> visible through the nested audit and a closed blocker;
   no group evaluation, no v3 standing effect

UpstreamCompositionRejected
-> visible through the nested audit and a closed blocker;
   no group evaluation, no v3 standing effect
```

Incomplete and rejected audits are never repaired, normalized, retried,
partially folded, or replaced through ambient lookup. The complete
internally derived audit is nested verbatim in the resolution.

## 15. Exact v2 inheritance and precedence law

The v3 resolution retains the complete internally derived
`StandingResolutionV2` unchanged. Before any promotion, validate in this
exact first-failure order:

```text
1. inherited_v2.claim_id == requested claim_id
2. inherited_v2.policy_id == MAGPIE_CLAIMS_POLICY_V2_ID
3. inherited_v2.resolution_failure is None
```

A claim or policy mismatch is an outer composition failure. An honest
inherited v2 resolution failure is a closed blocker, never an outer
failure and never a reason to recompute v2; no promotion occurs and the
inherited governed result and failure remain visible.

Governed standing follows this exact precedence:

```text
1. inherited governed Settled   -> Settled
2. inherited governed Refuted   -> Refuted
3. inherited governed Supported -> Supported
4. otherwise, successful v3 corroboration -> Supported
5. otherwise -> the inherited v2 standing, unchanged
```

The v3 rule's achieved contribution is only `Some(Supported)`, never
`Settled`. `legacy_raw_standing` and `currentness` are copied unchanged
from the inherited v2 resolution and never consulted by the combiner.

### Outer-composition failure output law

When `resolution_failure` is `Some(_)`, freeze every remaining
`StandingResolutionV3` field exactly:

```text
claim_id:
the exact requested claim_id

governed_standing:
the inherited v2 governed standing, unchanged;
no v3 promotion occurs on any failure

legacy_raw_standing and currentness:
copied unchanged from the inherited v2 resolution

policy_id:
magpie-claims-standing-v3

verified_prefix_identity and closure_identity:
the expected current prefix and closure identities from the context
and closure; a mismatched upstream identity is never copied into the
top-level authority surface

inherited_v2 and support_contribution_audit:
nested verbatim as complete evidence, including any staged drift

lane:
None

application:
None

ignored_contributions:
empty

blockers:
empty
```

No group evaluation, claim selection, or lane construction occurs on any
outer-composition failure. The failure is the complete report.

## 16. Exact deterministic ordering

Freeze, using existing exact `Ord`, `BTreeMap`, and `BTreeSet`
semantics:

```text
lanes:
exact OriginComparisonNamespaceV0::Ord order

origin groups within one lane:
exact String::Ord order

retained contributions within one group:
exact ContributionIdentityV0::Ord order

counted groups:
exact String::Ord order

ignored, non-counting contributions:
exact ContributionIdentityV0::Ord order

blockers:
one closed enum, exact declaration order

outer composition failure:
one first-failure Option in the frozen stage order
```

No caller order, event insertion order, vector order, group lexical
preference, write recency, or arbitrary first-two selection determines an
outcome.

## 17. Closed context, blocker and failure vocabularies

Freeze:

```text
StandingPolicyContextV3:
Corroborated { distinct_group_count: usize }

StandingBlockerV3:
InsufficientDistinctOriginGroups { distinct_group_count: usize }
SupportAuditIncomplete
SupportAuditRejected
InheritedV2ResolutionFailed

StandingResolutionFailureV3 (frozen first-failure order):
InheritedClaimMismatch
InheritedPolicyMismatch
SupportAuditSchemaMismatch
SupportAuditCanonicalizationProfileMismatch
SupportAuditPolicyMismatch
SupportAuditAdmittedPolicyMismatch
SupportAuditOriginPolicyMismatch
VerifiedPrefixIdentityMismatch
ClosureIdentityMismatch
LaneInvariantMismatch { contribution: ContributionIdentityV0 }
TargetScopeMismatch { contribution: ContributionIdentityV0 }
```

## 18. Future public types and serialization

Freeze this closed type vocabulary. Every new struct has private fields
and read-only getters; every type is `Clone`, `Debug`, `PartialEq`,
`Eq`, `Serialize`; no type has `Deserialize`, `Default`, a public
constructor, or public mutation. The rule enum serializes with
`snake_case` unit variants; the context, blocker, and failure enums use
internal `kind` tagging with `snake_case` variants.

```text
StandingPolicyRuleV3
CorroborationGroupV0
  origin_group: String
  contributions: Vec<SupportContributionV0>
ExternalReportCorroborationLaneV0
  namespace: OriginComparisonNamespaceV0
  groups: Vec<CorroborationGroupV0>
StandingPolicyContextV3
StandingBlockerV3
StandingPolicyApplicationV3
  rule: StandingPolicyRuleV3
  context: StandingPolicyContextV3
  achieved_standing: Option<Status>  (only Some(Supported))
StandingResolutionFailureV3
StandingResolutionV3
  claim_id: String
  governed_standing: Option<Status>
  legacy_raw_standing: Option<Status>
  currentness: StandingCurrentness
  policy_id: &'static str
  verified_prefix_identity: VerifiedLogPrefixIdentityV0
  closure_identity: ResolutionContentClosureIdentityV0
  inherited_v2: StandingResolutionV2
  support_contribution_audit: SupportContributionAuditV0
  resolution_failure: Option<StandingResolutionFailureV3>
    (skipped from serialization when None)
  lane: Option<ExternalReportCorroborationLaneV0>
  application: Option<StandingPolicyApplicationV3>
  ignored_contributions: Vec<SupportContributionV0>
  blockers: Vec<StandingBlockerV3>
```

Field declaration order is canonical byte order. Serialized output is
audit material, not reusable authority; no resolver accepts any v3 output
type back as input.

## 19. Legacy raw quarantine

Legacy raw `Settled` or `Refuted` remains quarantined compatibility and
audit material. It never substitutes for inherited governed standing,
never establishes governed `Settled`, and never vetoes a successful v3
corroboration. An inherited governed `Refuted` remains `Refuted` until an
explicit versioned invalidation, supersession, recovery, or refutation
policy changes that law; this ticket defines none of them.

## 20. Required hostile tests

The future runtime ticket must implement tests covering at least:

```text
group-count boundaries:
zero contributions; one admitted group; two distinct contributions in
one group; many contributions in one group; exactly two distinct
groups; more than two distinct groups

determinism:
vector permutation invariance; canonical bytes deterministic and
regenerable; group lexical order and contribution order pinned

namespace and selection:
same textual group in different namespaces; contributions for
different claims; contributions for different scopes; case-distinct
group keys; empty requested claim ID; missing typed claim node

completion:
incomplete support audit; rejected support audit; inherited governed
Supported with incomplete or rejected support audit

identity and invariants:
each support-audit identity drift separately (schema, canonicalization
profile, support policy, admitted policy, origin policy, prefix,
closure), including on complete-empty input; each fixed lane
membership field drift; same-target namespace scope drift; staged
duplicate exact ContributionIdentityV0

authority impossibility:
caller-selected threshold impossible; caller-created audit
substitution impossible; caller-selected policy, lane, group, or
contribution list impossible

inherited precedence:
inherited Settled; inherited governed Refuted; inherited Supported;
inherited v2 resolution failure with two available groups;
simultaneous inherited v2 failure and incomplete or rejected support
audit first-failure and blocker order

legacy raw quarantine:
legacy raw Settled never settles; legacy raw Refuted does not veto
successful v3 corroboration

ceiling law:
external-source aggregation never produces Settled

multiplicity:
same-origin multiplicity remains visible but counts once; all groups,
not an arbitrary first two, remain counted

clustering law:
two distinct admitted groups sharing one artifact digest or selector
still count by admitted exact group keys

canonical fixtures:
corroborated happy path; one insufficient-groups case; one incomplete
case; one rejected case; one inherited-v2-failure case; one
representative of each outer-failure class (global identity, lane
invariant, target scope)
```

## 21. Required compile-fail tests

Reserve doctests or equivalent compile-fail pins proving:

- no struct-literal construction of any new public type;
- no `Deserialize` on any new type;
- no resolver accepting a support audit, support contributions, origin
  groups, contribution lists, lane IDs, thresholds, policy IDs, a
  `StandingView`, or a standalone `StandingReplaySnapshot`;
- no caller-selected threshold or policy identity.

## 22. Frozen surfaces

This contract does not change:

- the v0, v1, or v2 standing policies, their rule vocabularies, their
  resolution types, or their canonical bytes;
- the support-contribution audit v0 contract, runtime, fixtures, or
  canonical bytes;
- the origin-admission, admitted-contribution, origin-binding,
  artifact-provenance, or deterministic-verifier surfaces;
- the evidence ceiling or support-context matrices;
- any L0 format, golden vector, tool, or CI gate.

## 23. Explicit non-goals

This ticket does not define or implement:

- `Settled` from external-source aggregation;
- weights, probabilities, confidence scores, voting, majority, quorum,
  reputation, ranking, source quality, or freshness;
- statistical-independence claims;
- cross-namespace counting;
- cross-domain aggregation;
- interpretation aggregation;
- refutation aggregation;
- contradiction debt;
- invalidation;
- supersession or currentness changes;
- `EpistemicGate`;
- writer, loader, CAS, filesystem, network, model, or librarian
  integration;
- L0 changes;
- policy matrix changes;
- a second standing engine;
- a free-form or caller-visible lane string;
- source-standing propagation or claim-to-claim support lanes;
- artifact-digest or selector clustering as a separation veto;
- runtime implementation of any kind.

## 24. Future sequence

```text
done:
step 11 — support-contribution audit v0 contract (Ticket 0051)
step 12 — support-contribution audit v0 runtime (Ticket 0052)
step 13 — ratify one explicit standing policy v3 aggregation rule
          (this ticket)

future:
step 14 — implement conservative aggregation (the v3 runtime ticket,
          under the exact implementation allowlist in the design note)
step 15+ — refutation, contradiction debt, invalidation, supersession,
          EpistemicGate, writer surfaces (unscheduled doctrine)
```

## 25. Stop conditions

Stop and escalate rather than proceeding if any of the following is
required:

- a second aggregation rule, threshold, or lane definition;
- any change to the v0/v1/v2 standing surfaces or the support audit;
- any runtime code, fixture, Cargo, L0, or CI change;
- any relaxation of the authority path (caller-provided audits, groups,
  thresholds, policies, snapshots, or callbacks);
- any digest-, metadata-, or similarity-based separation law;
- any `Settled` outcome for external-source aggregation.

## 26. Validation commands

The contract ticket itself is documentation-only; run:

```text
git diff --check
git status --short
git diff --stat
```

The future runtime ticket must run and report, as observed:

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --doc --locked
cargo run --locked --example tour -p magpie-claims
python tools/verify_chain.py <golden logs>
python tools/check_release_metadata.py
```

## 27. Publication authority

`AGENTS.md` remains binding. The human retains sole authority for
creating the branch, committing, pushing, opening or marking ready the
draft PR, merging, and resolving review threads. The worker may not
modify `main`, create any other branch, force-push, or widen the
allowlist.

## 28. Reviewer checklist

1. Confirm the base is the PR #65 merge commit and the branch carries
   exactly the commits listed in §2.
2. Confirm only the six allowlisted paths changed, and the two new paths
   are the design note and this ticket.
3. Confirm the policy identity, rule identity, and threshold constants
   match §7 with no selector, alias, or caller path.
4. Confirm the lane partitions only by exact `OriginComparisonNamespaceV0`
   and every fixed membership drift fails closed.
5. Confirm the authority path derives both inputs internally and accepts
   no caller-provided authority.
6. Confirm only `Complete` audits evaluate groups; incomplete and
   rejected audits are visible but never repaired or folded.
7. Confirm the five-step precedence, the never-`Settled` law, and the
   legacy raw quarantine.
8. Confirm same-origin contributions remain visible but count once, and
   no clustering veto exists.
9. Confirm deterministic ordering and the closed vocabularies.
10. Confirm private fields, getters, `Serialize`-only boundaries, no
    `Deserialize`, no `Default`, no public constructors, and canonical
    bytes only on `StandingResolutionV3`.
11. Confirm the hostile-test and compile-fail reservations cover §20-21.
12. Confirm no runtime implementation claim appears anywhere.

## 29. Precise contract claim

```text
This ticket ratifies one exact standing policy v3 rule:

for one exact requested ExternalReport claim,
inside one exact OriginComparisonNamespaceV0 lane,
with fixed membership invariants,
at least two distinct admitted origin groups
among the complete internally derived support contributions
->
governed Supported,
unless inherited governed standing already preserves
Settled, Refuted, or Supported.

Everything else is unchanged, future, or out of scope.
```
