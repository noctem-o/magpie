# Standing Aggregation and Independence Groups

## Status

Proposed design note.

## Purpose

This note defines the future doctrine for achieved-standing aggregation and
independence groups before Magpie implements that behavior.

Core laws:

```text
A ceiling is not achieved standing.
A contribution is not aggregation.
Aggregation is replay-derived, deterministic policy behavior.
Independence groups prevent source-count inflation.
Corroboration may help reach an allowed ceiling; it must not exceed the ceiling.
```

This phase is docs/tickets only. It freezes vocabulary and constraints for a
later implementation phase; it does not change current `StandingView` behavior.

## Non-goals

- No Rust changes.
- No tests.
- No L0 payload variants.
- No canonical encoding changes.
- No `docs/FORMAT.md` changes.
- No golden vector changes.
- No `tools/verify_chain.py` changes.
- No writer-facing surfaces.
- No MCP write paths.
- No `EpistemicGate`.
- No achieved-standing aggregation implementation.
- No contradiction debt implementation.
- No invalidation implementation.
- No supersession implementation.
- No ratification admission semantics.
- No `StandingView::resolved_standing()` behavior change.
- No support-ceiling behavior change.
- No refutation-ceiling behavior change.
- No Deadbolt changes.
- No model integration.
- No librarian/navigator functionality.
- No network sync, remote services, or always-on behavior.
- No production trust, certification, attestation, or live-readiness claim.

## Current substrate

The current substrate can retain typed claims, typed evidence, and typed
justification edges:

- `ClaimAssertedV2` retains typed claim metadata.
- `EvidenceRegistered` retains typed evidence metadata.
- `JustificationEdgeRecorded` retains typed source/target relationships.
- `magpie_claims::policy::support_ceiling` defines maximum positive
  contributions by evidence kind and claim domain.
- `magpie_claims::policy::refutation_ceiling` defines whether an evidence kind
  and claim domain may directly contribute `Refuted`.

`StandingView::resolved_standing()` already exists as an older first fold slice.
It is a pure read-only query over accumulated tables. It currently handles
evidence-to-claim `supports` edges under a conservative support slice, but it
deliberately does not read `claim_domain`, does not parse `metadata_json`, and
does not implement settlement, contradiction debt, invalidation, supersession,
`EpistemicGate`, or writer surfaces.

This note does not mutate that behavior.

## Definitions

### Candidate Contribution

A candidate contribution is something that may contribute if edge kind, source,
target, scope, domain, admission, and ceiling checks pass.

A candidate contribution is not yet standing. It is merely material that the
future fold may inspect.

### Eligible Contribution

An eligible contribution is a candidate contribution that passed the relevant
checks for edge kind, source existence, target existence, exact scope, domain
classification, admission policy, source standing, and ceiling.

Eligibility means the future aggregator may consider the contribution. It does
not mean the target claim is automatically promoted or refuted.

### Support Contribution

A support contribution is an eligible positive contribution capped by
`support_ceiling` and any source-standing limits.

Support contribution is not achieved support. It is one input to future
aggregation.

### Refutation Contribution

A refutation contribution is an eligible negative contribution capped by
`refutation_ceiling`.

Refutation contribution is not contradiction debt, invalidation, or
supersession. It is one input to future negative standing aggregation.

### Ceiling

A ceiling is the maximum permitted contribution policy allows for a particular
evidence kind and claim domain.

Ceilings constrain aggregation. They are not automatic promotion and not stored
standing.

### Achieved Standing

Achieved standing is the deterministic result after replay-derived aggregation.
It is not stored in L0 and is not an authoritative mutable field.

Achieved standing must be regenerable from the event log, policy, and derived
projection rules.

### Independence Group

An independence group is an opaque policy key used to prevent multiple reports
from the same origin from counting as independent corroboration.

Independence groups are policy material, not L0 canonical material.

Future policy may use a `metadata_json` key such as `independence_group`. This
is future policy over opaque `metadata_json`. It is not a canonical encoding
change and not a new `Payload` field.

### Independent Corroboration

Independent corroboration is corroboration from eligible contributions that the
future fold admits as distinct independence groups.

Independent corroboration may help a contribution reach its allowed ceiling. It
must not exceed that ceiling.

### Same-Origin Evidence

Same-origin evidence is evidence that shares an independence group or otherwise
fails future independence checks.

Same-origin evidence may still be retained and may still be useful context. It
must not be counted as multiple independent corroborators.

### Aggregation Lane

An aggregation lane is a deterministic partition of eligible contributions that
future policy aggregates together. A lane may be separated by claim, direction
of contribution, evidence kind, claim domain, edge kind, scope, or independence
group.

Aggregation lanes prevent unrelated evidence from being combined accidentally
and give future explainability code a stable way to report why a claim reached
its achieved standing.

## Aggregation Laws

1. A ceiling is not achieved standing.
2. A contribution is not aggregation.
3. Aggregation is deterministic replay policy, not a mutable stored field.
4. Aggregation must be order-independent over the same admitted event set.
5. Aggregation must be regenerable from L0 plus deterministic policy.
6. A contribution may be lower than its ceiling or ignored if any eligibility
   check fails.
7. Corroboration may help reach an allowed ceiling; it must not exceed the
   ceiling.
8. Repeated weak evidence from the same group must not simulate independent
   corroboration.
9. Support aggregation must not revive a raw `Refuted` claim without explicit
   future invalidation or supersession rules.
10. Absence of evidence is not refutation.
11. Human authority is governance/consent authority, not truth authority.
12. Aggregation must not infer stronger claim domains from prose confidence.
13. Aggregation must keep support, refutation, contradiction debt,
    invalidation, and supersession as separate policy concepts.

## Independence-Group Laws

Independence group keys are opaque exact strings.

Rules:

- No prefix semantics.
- No inheritance.
- No containment.
- No fuzzy matching.
- No URL/domain inference in v1.
- No automatic source clustering.
- No probabilistic independence.
- No "same publisher but different URL" inference unless future policy
  explicitly admits it.

Absent, ambiguous, conflicting, or unsupported independence-group metadata must
not be treated as independent corroboration. Future fold rules may either count
it as a single weak lane or refuse corroborating aggregation.

Independence groups are not source truth. They only constrain whether multiple
eligible contributions may count as independent corroborators.

## External-Source Corroboration

Law:

```text
ExternalSource cannot settle truth, even with corroboration policy.
```

`ExternalSource` may support `ExternalReport` or `Interpretation` only up to
`Supported`. Corroboration may help decide whether an external-source
contribution is admitted or remains weak, but corroboration cannot turn
`ExternalSource` into `Settled`.

Multiple external reports from the same independence group do not count as
multiple independent corroborators. Reposted material, syndicated text, mirrored
pages, copied abstracts, or repeated citations may be useful provenance but do
not create independent lanes unless future policy explicitly admits their
independence.

Human ratification of an external report may record a human judgment about the
report, but it does not settle the report's truth. A human can approve relying
on a report for a scoped decision; they cannot make the report true by decree.

Model self-reports and lens readouts do not become supporting evidence merely by
repetition. Repeated model or lens outputs may seed hypotheses, but repetition
alone must not raise them above their ceiling or simulate independent
corroboration.

## Interaction With Support Ceilings

`support_ceiling` defines the maximum positive contribution permitted for an
evidence-kind/domain pair. Future support aggregation may combine eligible
support contributions, but the result must not exceed the applicable ceiling.

Examples:

- `ExternalSource x ExternalReport` may reach at most `Supported`.
- `HumanRatification x HumanJudgment` may reach at most `Supported`.
- `DeadboltAnchor x Occurrence/Inclusion` may reach `Settled` only when future
  verifier-context and eligibility rules admit it.

Support aggregation must consider source standing. Unsupported or conjectured
source claims cannot amplify a target beyond their own derived standing.

## Interaction With Refutation Ceilings

`support_ceiling` and `refutation_ceiling` are separate policy surfaces.
Aggregation may consider both later, but this phase does not implement the
resolver that combines them.

Refutation eligibility must not be confused with contradiction debt. A direct
refutation contribution requires a `refutation_ceiling` cell that permits
`Refuted` plus future eligibility checks.

Only the refutation ceiling policy can admit direct `Refuted` contribution. It
does not make refutation automatic.

## Interaction With Contradiction Debt

A `contradicts` edge may create debt or block settlement later; it does not
automatically refute in this phase.

Contradiction debt is future fold behavior. It should remain distinct from:

- direct refutation contribution;
- source invalidation;
- supersession/currentness;
- ordinary negative interpretation;
- absence of evidence.

Invalidation and supersession are separate future semantics. Invalidation is not
refutation. Supersession is not deletion.

## Future Implementation Order

1. Add aggregation policy helper code that computes candidate and eligible
   contributions without changing writer surfaces.
2. Parse advisory `claim_domain` and future `independence_group` metadata under
   policy, not L0 validation.
3. Add deterministic aggregation lanes and explanation traces.
4. Add external-source corroboration with independence-group limits.
5. Add direct refutation aggregation only where `refutation_ceiling` permits it.
6. Implement contradiction debt.
7. Implement invalidation and supersession.
8. Implement `EpistemicGate` admission rules.
9. Add optional librarian/navigator behavior only after the policy substrate is
   explicit and reviewed.

## Future Tests

Future PRs should add tests with names such as:

- `external_sources_same_independence_group_do_not_amplify`
- `external_sources_independent_groups_may_correlate_to_supported`
- `external_source_corroboration_never_settles`
- `missing_independence_group_does_not_count_as_independent`
- `model_self_report_repetition_does_not_amplify`
- `lens_readout_repetition_does_not_amplify`
- `human_ratification_does_not_settle_external_report`
- `support_aggregation_does_not_revive_refuted_claim`
- `contradicts_edge_creates_debt_not_refutation`
- `invalidation_is_not_refutation`
- `supersession_is_not_deletion`
- `aggregation_is_order_independent`
- `aggregation_is_regenerable`

These are future tests for later PRs. This note does not add tests.

## Reviewer Checklist

- Confirm this phase is docs/tickets only.
- Confirm independence groups are opaque policy strings, not inferred source
  clusters.
- Confirm missing or ambiguous independence metadata does not count as
  independent corroboration.
- Confirm external-source corroboration cannot yield `Settled`.
- Confirm support and refutation aggregation remain separate from contradiction
  debt, invalidation, and supersession.
- Confirm no current `StandingView::resolved_standing()` behavior changes.
