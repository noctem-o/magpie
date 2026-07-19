# Standing Policy V3 External-Report Corroboration v0

## Status

Contract ratified by Ticket 0053. Documentation only.

```text
standing policy v3:
contract ratified by Ticket 0053
runtime: not implemented

first rule:
ExternalReportCorroborationV0 (single closed rule)

threshold:
two distinct admitted origin groups (compiled constant)

achieved standing:
governed Supported only, never Settled
```

This note ratifies policy. It adds no Rust, tests, fixtures, Cargo metadata,
dependency, CI, L0 surface, or runtime capability.

## Purpose

Define Magpie's first explicit standing policy v3 aggregation rule: a
deliberately narrow, conservative `ExternalSource × ExternalReport`
corroboration rule.

The exact high-level law is:

```text
complete internally derived SupportContributionAuditV0
+ exact requested ExternalReport claim
+ at least two distinct admitted origin groups
+ contributions inside one exact permitted aggregation lane
->
governed Supported
```

The rule separates, exactly and permanently:

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

Distinct admitted origin groups are policy-recognised corroboration
separation only. They are not statistical, causal, organisational,
institutional, publisher, ownership, or control independence. No URL,
hostname, domain, author name, publisher name, citation, metadata, fuzzy
similarity, model judgment, or probabilistic clustering may manufacture
separation.

## Non-goals

This contract does not define or implement:

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
- writer, loader, CAS, filesystem, network, model, or librarian integration;
- L0 changes;
- policy matrix changes;
- a second standing engine;
- a free-form or caller-visible lane string;
- source-standing propagation or claim-to-claim support lanes;
- artifact-digest or selector clustering as a separation veto;
- runtime implementation of any kind.

## Exact policy identity

Freeze one fixed standing policy v3 identity:

```rust
pub const MAGPIE_CLAIMS_POLICY_V3_ID: &str = "magpie-claims-standing-v3";
```

There is no alias, no `latest`, no runtime selector, no negotiation, no
registry, no environment override, no bundle-selected policy, and no
caller-selected policy ID. The serialized policy ID discloses compiled code;
it does not select it.

Policy v3 inherits explicit policy v2 through one internally derived
`StandingResolutionV2`. It does not recreate, replace, or bypass the
Deadbolt and deterministic-verifier rules, and it does not retire the v0,
v1, or v2 policy identities.

## Exact rule identity

Freeze exactly one closed v3 rule:

```text
StandingPolicyRuleV3::ExternalReportCorroborationV0
```

The closed enum is the rule's only identity, following the contribution-lane
law that the rule enum remains the serialized identity and that a second
identifier would duplicate a closed concept and could drift. No additional
aggregation rule, second rule ID, or free-form lane string enters this
ticket.

The rule's achieved standing is only `Some(Supported)`. It can never
produce `Settled`.

## Exact threshold

Freeze the minimum corroboration separation as one compiled policy constant:

```rust
pub const EXTERNAL_REPORT_CORROBORATION_MIN_DISTINCT_ORIGIN_GROUPS_V0: usize = 2;
```

The threshold is fixed compiled policy. It is not a runtime argument,
configuration value, metadata field, bundle field, environment value, or
caller choice. It may be serialized into the explanation as audit
disclosure, but the resolver and composer populate it from compiled code
only.

One or zero distinct admitted origin groups produce no new standing.
Contribution count is not origin-group count.

## Exact aggregation-lane identity

The aggregation lane is a typed partition key. It is not the
contribution-lane identity of policy v1/v2, not a string, and not a second
rule identifier.

Exactly one field partitions contributions before group counting:

```text
namespace:
the exact OriginComparisonNamespaceV0
(exact origin-admission policy ID, exact target claim ID, exact scope_ref)
```

Every counted contribution must additionally satisfy these fixed membership
invariants, which do not vary within honest v0 output:

```text
policy_id
(the exact SupportContributionV0 field and getter)
== magpie-support-contribution-v0

admitted_contribution_policy_id
== magpie-admitted-contribution-v0

origin_admission_policy_id
(the exact SupportContributionV0 field and getter)
== magpie-origin-admission-v0

namespace.origin_admission_policy_id
== magpie-origin-admission-v0

evidence_kind
== ExternalSource

claim_domain
== ExternalReport

support_ceiling
== Supported

contribution.scope_ref
== namespace.scope_ref
```

Field names are the exact landed Ticket 0052 surfaces:
`SupportContributionV0.policy_id`,
`SupportContributionV0.admitted_contribution_policy_id`, and
`SupportContributionV0.origin_admission_policy_id` for the contribution-level
invariants, and `SupportContributionAuditV0.policy_id`,
`SupportContributionAuditV0.admitted_contribution_policy_id`, and
`SupportContributionAuditV0.origin_admission_policy_id` for the audit-level
outer identity checks.

Two origin-policy checks exist at separate levels and never substitute for
each other:

```text
audit-level identity:
SupportContributionAuditV0.origin_admission_policy_id
== magpie-origin-admission-v0
-> SupportAuditOriginPolicyMismatch

contribution-level lane membership:
SupportContributionV0.origin_admission_policy_id
== magpie-origin-admission-v0
-> LaneInvariantMismatch { contribution: exact ContributionIdentityV0 }

namespace identity:
OriginComparisonNamespaceV0 carries the exact origin-admission policy as
partition identity; that namespace policy component must itself equal
magpie-origin-admission-v0 or the contribution fails LaneInvariantMismatch.
A correct namespace does not excuse a drifted separately serialized
SupportContributionV0.origin_admission_policy_id, and a correct audit-level
origin policy does not excuse a drifted contribution-level origin policy.
```

The separately serialized `SupportContributionV0.origin_admission_policy_id`
is not part of the lane partition key: the exact
`OriginComparisonNamespaceV0`, including its own origin-admission policy
component, remains the sole partition. The contribution-level field is a
fixed membership invariant whose drift fails closed, not a value that
creates another lane. No value is repaired, normalized, inherited from a
nearby field, or silently ignored.

The compiled support-context requirement for
`ExternalSource × ExternalReport` remains `NoPrivilegedContext`; a complete
support audit has already revalidated both fixed policy cells globally.

The lane key must not include source evidence ID, justification-edge ID,
artifact digest, supporting selector, occurrence, or the complete
contribution identity: including any of them would isolate every
contribution and defeat comparison. Those values remain contribution
identity and explanation material.

A staged or internal value drifting in any fixed membership field is an
outer composition failure, never a second eligible lane. In honest
Ticket 0052 output every contribution shares the fixed fields, so exactly
one lane exists per requested claim.

## Exact authority path

The future public resolver performs exactly:

```text
OriginAdmissionReplayContextV0 (private construction, one verified replay)
+ one immutable ResolutionContentClosureV0 (the exact input M)
->
inherited StandingResolutionV2
  derived internally from the context's own snapshot
  for the exact requested claim
+
SupportContributionAuditV0
  derived internally from the same context and the same closure
+
one crate-private pure v3 composer
->
StandingResolutionV3
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

No ambient CAS, filesystem, callback, plugin, environment, registry,
remote, or network lookup participates. The same verified replay
context (H), the same compiled policies (P), the same immutable closure
(M), and the same requested claim produce byte-identical output on every
evaluation.

## Exact claim-selection law

Only support contributions whose exact `target_claim_id` equals the
requested claim may enter the rule.

For every such contribution, the exact
`OriginComparisonNamespaceV0.target_claim_id` must equal the requested
claim, `namespace.scope_ref` must equal the exact replayed
`TypedClaimNode.scope_ref` of the requested claim, and
`contribution.scope_ref` must equal `namespace.scope_ref`. A same-target
contribution whose namespace target or scope drifts from the replayed
values, or whose contribution scope drifts from the namespace, is an
outer composition failure reported as `TargetScopeMismatch`, not an
ignored contribution.

Contributions whose exact `target_claim_id` differs from the requested
claim remain visible as ignored, non-counting audit material ordered by
exact `ContributionIdentityV0`. They can never affect the result.

The requested target must remain the exact replayed `ExternalReport` claim
in the exact shared scope. Nearby IDs, prose equality, shared digests,
shared evidence, aliases, summaries, or metadata may not substitute.

Policy v3 adds no new claim classification. An absent, malformed,
wrong-domain, duplicate-domain, or unknown-domain requested claim is
handled exactly as the inherited policy v2 resolution already handles it;
v3 introduces no parser, no second domain lookup, and no alternate claim
authority. Such a request yields no v3 standing effect.

## Exact same-origin collapse and group-counting law

Within one eligible lane:

```text
group retained contributions by exact origin_group string
-> count the distinct exact group keys
-> success iff distinct_group_count
   >= EXTERNAL_REPORT_CORROBORATION_MIN_DISTINCT_ORIGIN_GROUPS_V0
```

Freeze:

- Multiple distinct contributions assigned to the same admitted origin
  group count at most once.
- Every contribution is preserved in explanation material; no underlying
  `SupportContributionV0` value is deleted, deduplicated, or rewritten.
- All distinct groups, not an arbitrary first two, remain counted and
  visible.
- Group keys compare as exact strings: no case folding, trimming,
  normalization, Unicode folding, or aliasing. Keys differing only by
  ASCII case are distinct exact admitted keys.
- Two textual group keys are comparable only inside the same exact
  `OriginComparisonNamespaceV0`. Contributions from different namespaces
  must never be combined.
- Unknown, absent, malformed, conflicting, unadmitted, incomplete, or
  rejected origin material provides no amplification and enters no group.
- Two distinct admitted groups carrying the same artifact digest or
  supporting selector still count by their admitted exact group keys.
  This rule never invents a digest or metadata clustering veto; any such
  law would belong to the origin-admission policy, not to aggregation.
- Every selected contribution must carry a unique exact
  `ContributionIdentityV0`. A staged duplicate exact identity — in one
  origin group or across different origin groups — fails closed as
  `DuplicateContributionIdentity`, reporting the first duplicated
  identity in exact `ContributionIdentityV0` order. No group evaluation
  or counting occurs.

## Exact completion law

The internally derived support audit has exactly three completion states.
The v3 rule consumes them literally:

```text
Complete
-> the rule may evaluate groups and count corroboration separation

AdmittedContributionIncomplete
-> visible in the v3 explanation through the nested audit and a closed
   blocker; no group evaluation, no v3 standing effect

UpstreamCompositionRejected
-> visible in the v3 explanation through the nested audit and a closed
   blocker; no group evaluation, no v3 standing effect
```

Incomplete and rejected audits are never repaired, normalized, retried,
partially folded, or replaced through ambient lookup. The complete
internally derived audit is nested verbatim in the resolution so that
incomplete and rejected detail and every original support contribution
remain visible.

## Exact v2 inheritance and precedence law

The v3 resolution retains the complete internally derived
`StandingResolutionV2` unchanged as inherited audit material. Before any
promotion, the composer validates, in this exact first-failure order:

```text
1. inherited_v2.claim_id == requested claim_id
2. inherited_v2.policy_id == MAGPIE_CLAIMS_POLICY_V2_ID
3. inherited_v2.resolution_failure is None
```

A claim or policy mismatch is an outer composition failure. An honest
inherited v2 resolution failure is not an outer failure and never a
reason to recompute v2: it is reported through a closed blocker, no
promotion occurs, and the inherited governed result and failure remain
visible.

Governed standing under an identity-valid inherited v2 resolution with
no resolution failure then follows this exact precedence:

```text
1. inherited governed Settled   -> Settled
2. inherited governed Refuted   -> Refuted
3. inherited governed Supported -> Supported
4. otherwise, successful v3 corroboration -> Supported
5. otherwise -> the inherited v2 standing, unchanged
```

When `inherited_v2.resolution_failure` is `Some(_)`, this table does not
apply: the inherited standing and failure remain visible as a closed
blocker, and no promotion occurs regardless of corroboration.

The v3 rule's achieved contribution is only `Some(Supported)`, never
`Settled`.

## Legacy raw quarantine

`legacy_raw_standing` and `currentness` are copied unchanged from the
inherited v2 resolution. Legacy raw `Settled` or `Refuted` remains
quarantined compatibility and audit material: it never substitutes for
inherited governed standing, never establishes governed `Settled`, and
never vetoes a successful v3 corroboration. An inherited governed
`Refuted` remains `Refuted` until an explicit versioned invalidation,
supersession, recovery, or refutation policy changes that law; this
contract defines none of them.

## Exact outer-composition failure output law

When `resolution_failure` is `Some(_)`, freeze every remaining
`StandingResolutionV3` field exactly:

```text
claim_id:
the exact requested claim_id

governed_standing:
None when the failure is InheritedClaimMismatch or
InheritedPolicyMismatch: identity-invalid inherited authority is
visible only as nested evidence. Otherwise the inherited v2 governed
standing, unchanged; no v3 promotion occurs on any failure

legacy_raw_standing and currentness:
None and Unknown respectively when the failure is an inherited
identity mismatch: foreign raw values never surface under the
requested claim. Otherwise copied unchanged from the inherited v2
resolution

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

## Exact deterministic ordering

Freeze canonical ordering, using existing exact `Ord`, `BTreeMap`, and
`BTreeSet` semantics:

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
one first-failure Option in the frozen stage order,
never an unordered list

competing same-stage failures:
the first affected contribution in exact ContributionIdentityV0::Ord
order, independent of vector order
```

No caller order, event insertion order, vector order, group lexical
preference, write recency, or arbitrary first-two selection determines an
outcome. The same verified prefix, immutable closure, compiled
policies, and requested claim produce byte-identical resolutions and
byte-identical canonical bytes under every input permutation.

## Exact future resolver surface

Reserve exactly two public methods on `OriginAdmissionReplayContextV0`:

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

Freeze:

- The scalar method delegates to the trace-bearing method and returns
  only `governed_standing`; it therefore returns None whenever the
  trace-bearing resolution reports an inherited claim or policy identity
  mismatch, with no second scalar failure policy.
- The replay context is the sole authority carrier: it alone co-holds the
  privately constructed snapshot (for v2 inheritance) and the verified
  prefix identity (for support-audit derivation).
- No free resolver, no `StandingView` resolver, no
  `StandingReplaySnapshot` v3 method, no generic policy parameter, and no
  second standing engine.
- Only `StandingResolutionV3` exposes `canonical_bytes()`, implemented by
  direct typed `serde_json::to_vec(self)` exactly as the v2 standing
  resolution does. These bytes are deterministic derived audit material,
  not L0 canonical encoding, not authority, and never an accepted input.

## Exact future public types and serialization

Freeze this closed type vocabulary. Every new struct has private fields
and read-only getters; every type is `Clone`, `Debug`, `PartialEq`, `Eq`,
`Serialize`; no type has `Deserialize`, `Default`, a public constructor,
or public mutation. Enum serialization follows the standing-family
conventions: the rule enum uses `snake_case` unit variants; the context,
blocker, and failure enums use internal `kind` tagging with `snake_case`
variants.

```text
StandingPolicyRuleV3
  closed rule vocabulary; exactly one variant:
  ExternalReportCorroborationV0

CorroborationGroupV0
  origin_group: String
  contributions: Vec<SupportContributionV0>
  (all retained contributions, exact ContributionIdentityV0 order;
   each exact group counts at most once)

ExternalReportCorroborationLaneV0
  namespace: OriginComparisonNamespaceV0
  groups: Vec<CorroborationGroupV0>
  (exact String::Ord group order)

StandingPolicyContextV3
  Corroborated { distinct_group_count: usize }

StandingBlockerV3
  closed blocker vocabulary:
  InsufficientDistinctOriginGroups { distinct_group_count: usize }
  SupportAuditIncomplete
  SupportAuditRejected
  InheritedV2ResolutionFailed

StandingPolicyApplicationV3
  rule: StandingPolicyRuleV3
  context: StandingPolicyContextV3
  achieved_standing: Option<Status>  (only Some(Supported))

StandingResolutionFailureV3
  closed outer-composition failure vocabulary, in frozen
  first-failure order:
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
  DuplicateContributionIdentity { contribution: ContributionIdentityV0 }

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
    (present only when the support audit is Complete and the
     requested claim selected eligible contributions)
  application: Option<StandingPolicyApplicationV3>
    (present only on Corroborated)
  ignored_contributions: Vec<SupportContributionV0>
  blockers: Vec<StandingBlockerV3>
```

Inside the failure vocabulary, `LaneInvariantMismatch` reports any fixed
membership field drifting from its required constant, while
`TargetScopeMismatch` reports any target or scope binding drift: the
namespace target or scope departing from the replayed claim, or the
contribution scope departing from the namespace.

Serialized output is audit material, not reusable authority. No resolver
ever accepts any v3 output type back as input. Field declaration order is
canonical byte order; direct typed `serde_json::to_vec(self)` produces
deterministic bytes without any new canonical encoding.

## Exact hostile-test reservation

The future runtime ticket must implement tests covering at least:

```text
group-count boundaries:
zero contributions; one admitted group; two distinct contributions in
one group; many contributions in one group; exactly two distinct
groups; more than two distinct groups

determinism:
vector permutation invariance; canonical bytes deterministic and
regenerable; group lexical order and contribution order pinned, not
only whole-vector permutation; same H, P, M with two different
requested claim IDs binds each output to its request and yields
different canonical bytes, with no cross-claim cache equivalence

inherited identity failure standing:
foreign inherited claim with governed Settled, Refuted, or Supported
-> InheritedClaimMismatch -> top-level governed None, legacy raw
None, currentness Unknown, nested inherited retained byte-for-byte;
wrong inherited v2 policy identity with governed Settled, Refuted, or
Supported -> InheritedPolicyMismatch -> top-level governed None,
legacy raw None, currentness Unknown, nested inherited retained
byte-for-byte; simultaneous claim and policy drift reports the claim
mismatch first; resolved_standing_v3 returns None on either mismatch

later-failure preservation:
identity-valid inherited v2 with governed Supported plus any later
outer-composition failure -> inherited standing preserved unchanged,
no v3 promotion

namespace and selection:
same textual group in different namespaces is incomparable;
contributions for different claims; contributions for different
scopes; case-distinct group keys remain distinct; empty requested
claim ID and missing typed claim node allow no substitution

completion:
incomplete support audit yields no counting; rejected support audit
yields no counting; inherited governed Supported with incomplete or
rejected support audit remains Supported

identity and invariants:
every support-audit identity drift separately (schema, canonicalization
profile, support policy, admitted policy, origin policy, prefix,
closure), including on complete-empty input; every fixed lane
membership field drift, including namespace origin-policy drift
reported as LaneInvariantMismatch and contribution scope_ref drift
reported as TargetScopeMismatch; same-target namespace target or scope drift is
an outer failure reported as TargetScopeMismatch, not an ignore; staged
duplicate exact
ContributionIdentityV0, in one group or across different groups, fails
closed as DuplicateContributionIdentity with the first duplicated
identity in exact order

origin-policy drift at two levels:
drifted SupportContributionV0.origin_admission_policy_id with
otherwise valid audit, namespace, contribution identity, group and
lane fields -> LaneInvariantMismatch carrying the exact contribution
identity -> no lane construction, no group counting, no v3 promotion;
drifted SupportContributionAuditV0.origin_admission_policy_id
-> SupportAuditOriginPolicyMismatch -> no claim selection, lane
construction, counting, or promotion; the two cases use separate
failure vocabularies and never collapse into one

authority impossibility:
caller-selected threshold impossible; caller-created audit
substitution impossible; caller-selected policy, lane, group, or
contribution list impossible

inherited precedence:
inherited Settled precedence; inherited governed Refuted precedence;
inherited Supported stability; inherited v2 resolution failure with
two available groups yields no promotion; simultaneous inherited v2
failure and incomplete or rejected support audit follows the frozen
first-failure and blocker order

legacy raw quarantine:
legacy raw Settled with inherited governed non-Settled never settles;
legacy raw Refuted does not veto successful v3 corroboration

ceiling law:
external-source aggregation never produces Settled

multiplicity:
same-origin multiplicity remains fully visible but counts once; all
groups, not an arbitrary first two, remain counted

clustering law:
two distinct admitted groups sharing one artifact digest or selector
still count by admitted exact group keys

canonical fixtures:
corroborated happy path; one insufficient-groups case; one incomplete
case; one rejected case; one inherited-v2-failure case; one
representative of each outer-failure class (global identity, lane
invariant, target scope)
```

Compile-fail reservations must pin: no struct-literal construction of any
new public type, no `Deserialize`, and no resolver accepting a support
audit, support contributions, origin groups, contribution lists, lane
IDs, thresholds, policy IDs, a `StandingView`, or a standalone
`StandingReplaySnapshot`.

## Exact implementation allowlist (future runtime ticket)

The later runtime ticket may change only:

```text
crates/magpie-claims/src/standing_v3.rs              (new)
crates/magpie-claims/src/origin_admission_replay.rs  (two thin resolver methods)
crates/magpie-claims/src/lib.rs                      (mod + re-exports)
crates/magpie-claims/src/support_contribution_audit.rs  (#[cfg(test)]
                                                         staging constructors only)
crates/magpie-claims/src/origin_binding_verifier.rs  (#[cfg(test)]
                                                      namespace staging constructor only)
crates/magpie-claims/tests/standing_v3.rs            (new)
fixtures/standing-policy-v3-corroboration-v0/*       (new)
docs/design/standing-aggregation-independence-groups.md  (step 14 flip)
README.md                                            (item flip)
its own ticket file                                  (new)
```

The `#[cfg(test)] pub(crate)` staging constructors are the only permitted
changes to `support_contribution_audit.rs` and
`origin_binding_verifier.rs`: they let hostile tests stage drifted support
audits, contributions, and namespaces, exactly as Ticket 0052 staged
admitted audits. Production bytes, public API, and canonical fixtures of
both modules remain untouched.

No Cargo manifest, lockfile, dependency, L0, existing production file
beyond the named seams, existing ticket, or existing fixture may
change.

## Validation commands

The future runtime ticket must run and report, as observed:

```text
git diff --check
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --doc --locked
cargo run --locked --example tour -p magpie-claims
python tools/verify_chain.py <golden logs>
python tools/check_release_metadata.py
git status --short
```

## Reviewer checklist

- Confirm the policy identity is exactly `magpie-claims-standing-v3` with
  no alias, selector, negotiation, registry, environment override, or
  caller-selected policy.
- Confirm exactly one closed rule, `ExternalReportCorroborationV0`, and no
  second rule or lane string identity.
- Confirm the threshold is one compiled constant equal to two distinct
  admitted origin groups, never a caller value.
- Confirm the lane partitions only by exact `OriginComparisonNamespaceV0`
  and that every fixed membership field drift fails closed.
- Confirm the authority path derives both the inherited v2 resolution and
  the support audit internally from the same context and closure.
- Confirm only `Complete` audits evaluate groups, and incomplete and
  rejected audits are visible but never repaired or folded.
- Confirm the five-step precedence preserves inherited governed
  `Settled`, `Refuted`, and `Supported` when inherited v2 identity is
  valid, and that v3 never yields `Settled`.
- Confirm legacy raw values remain quarantined and never veto or settle.
- Confirm same-origin contributions remain visible but count once, and
  that no digest or metadata clustering veto exists.
- Confirm deterministic ordering for lanes, groups, contributions,
  ignored contributions, blockers, and first failures.
- Confirm private fields, getters, `Serialize`-only boundaries, no
  `Deserialize`, no `Default`, no public constructors, and direct typed
  `serde_json` canonical bytes on `StandingResolutionV3` only.
- Confirm no runtime implementation claim appears anywhere in this
  contract.

## Next-slice handoff

This contract completes doctrine step 13 (ratify one explicit standing
policy v3 aggregation rule). Doctrine step 14 (implement conservative
aggregation) remains future runtime work under the exact implementation
allowlist above. Refutation aggregation, contradiction debt, invalidation,
supersession, `EpistemicGate`, and writer surfaces remain unscheduled
future doctrine.
