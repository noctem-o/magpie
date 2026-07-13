# Standing Aggregation and Origin Groups

## Status

Proposed design note.

The filename is preserved for link stability. “Independence group” is earlier
design shorthand. The normative terms are **origin group** for the opaque
policy key and **corroboration separation** for the relationship between
distinct admitted groups. Magpie does not prove statistical, causal,
institutional, organisational, or control independence.

Sequencing status: replayable verifier admission, standing-inert verifier
context, one direct policy-v2 `Supported` rule, and policy-v2 composition
hardening are landed. The artifact-provenance and origin-admission contract is
the next architecture slice. Genuine aggregation remains blocked until
standing-inert foreign-bundle verification, origin-admission audit, and an
admitted-contribution audit exist.

## Purpose

This note defines the future doctrine for achieved-standing aggregation,
origin groups, and corroboration separation before Magpie implements that
behavior.

Core laws:

```text
A ceiling is not achieved standing.
A contribution is not aggregation.
Aggregation is replay-derived, deterministic policy behavior.
Origin groups prevent source-count inflation.
Corroboration may help reach an allowed ceiling; it must not exceed the ceiling.

Distinct admitted origin groups are policy-recognised corroboration separation.
Distinct admitted origin groups are not proof of statistical independence.
```

This phase is docs/tickets only. It freezes vocabulary and constraints for a
later implementation phase; it does not change runtime `StandingView` behavior.

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
- No artifact, acquisition, derivation, origin-binding, or origin-admission
  implementation.
- No contradiction debt implementation.
- No invalidation implementation.
- No supersession implementation.
- No ratification admission semantics.
- No runtime `StandingView::resolved_standing()` behavior change in this
  docs-only PR.
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

`StandingResolution` v0 is the canonical governed-standing explanation
surface. It names the fixed `magpie-claims-standing-v0` policy,
parses the target typed claim's `claim_domain` from `metadata_json` fail-closed,
and reports candidate support ceilings in deterministic trace entries.

`StandingView::resolved_standing()` is then a compatibility scalar that
delegates to the governed result from `StandingResolution`; it is not a second
standing engine. V0 does not turn candidate ceilings into achieved standing.
It exposes legacy raw status separately as quarantined audit material rather
than governed truth.

Current v0 resolution does not implement aggregation, corroboration-separation
amplification, direct refutation, contradiction debt, invalidation,
supersession, or public writer admission through `EpistemicGate`.

Explicit policy v2 adds one direct deterministic `Supported` contribution.
That rule is not aggregation, and trace multiplicity is not corroboration.
This note does not mutate any current behavior.

## Definitions

### Candidate Contribution

A candidate contribution is structurally present and parseable enough for the
resolver to inspect. It may still have missing endpoints, a scope mismatch, no
permitted ceiling, or an unmet admission/verifier requirement.

A candidate contribution is not standing, policy eligibility, or admission. It
is merely material that the fold can explain deterministically.

### PolicyEligible Contribution

A policy-eligible contribution is a candidate that passes non-authority checks
for edge kind, source and target existence, exact scope, closed domain, source
standing, applicable ceiling, and graph constraints.

Policy eligibility does not mean an actor, verifier, evidence label, domain, or
origin assertion is authorized. Do not call a contribution eligible for
an effect if that effect depends on an admission or verifier decision that has
not been made.

### Admitted Contribution

An admitted contribution is policy-eligible material whose authority-dependent
claims have passed replayable admission/verifier policy or a future
`EpistemicGate` decision.

Only admitted contributions may affect achieved standing. Admission still does
not imply promotion: aggregation and ceiling rules remain separate.
For origin-sensitive aggregation, a standing-inert admitted-contribution audit
must also associate the exact contribution with its replay-derived admitted
origin result. That audit reports potential input and does not create support.

### Support Contribution

A support contribution is an admitted positive contribution capped by
`support_ceiling` and any source-standing limits.

Support contribution is not achieved support. It is one input to future
aggregation.

### Refutation Contribution

A refutation contribution is an admitted negative contribution capped by
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

### Origin Group

An origin group is an opaque exact policy key used to prevent multiple admitted
contribution paths from the same governed origin from counting repeatedly.

Origin groups are policy material, not L0 canonical material, publisher
identities, reputation scores, truth labels, or independence certificates.
Their meaning is scoped by the exact origin-admission policy, contribution
subject, and claim scope.

Earlier or future metadata such as `metadata_json.independence_group` is
asserted advisory data only. It cannot create an origin group, change canonical
encoding, or admit itself.

### Corroboration Separation

Corroboration separation means that an explicit origin-admission policy has
assigned exact contribution-scoped paths to distinct admitted origin groups.

Corroboration separation may be consumed only by a later explicit aggregation
policy and must not exceed the applicable ceiling. It is not proof of
statistical, causal, institutional, organisational, or control independence.

### Same-Origin Evidence

Same-origin evidence shares an admitted origin group or otherwise has no
admitted corroboration separation.

Same-origin evidence may still be retained and may still be useful context. It
must not be counted repeatedly.

### Aggregation Lane

An aggregation lane is a deterministic partition that may organize candidates
and policy-eligible contributions for explanation. Only admitted contributions
may affect achieved standing within a lane. A lane may be separated by claim,
direction of contribution, evidence kind, claim domain, edge kind, scope, or
origin group.

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
8. Repeated weak evidence from the same group must not simulate corroboration
   separation.
9. Support aggregation must not revive a raw `Refuted` claim without explicit
   future invalidation or supersession rules.
10. Absence of evidence is not refutation.
11. Human authority is governance/consent authority, not truth authority.
12. Aggregation must not infer stronger claim domains from prose confidence.
13. Aggregation must keep support, refutation, contradiction debt,
    invalidation, and supersession as separate policy concepts.

## Admission and Authority Boundary

Advisory metadata and vocabulary labels are never authority by themselves.
`actor_class`, `evidence_kind`, `claim_domain`, a self-declared group, and
a `DeadboltAnchor` evidence label are asserted data until admitted by
replayable policy.

The authority path is defined by
`docs/design/artifact-provenance-origin-admission.md`: exact artifact and
contribution identity, anchored and profile-verified acquisition/derivation/
origin-binding bundles, explicit origin-admission policy, and no unresolved
binding conflict. Origin admission still does not create a support
contribution.

A trace policy identifier discloses which fixed policy produced an explanation.
It is not a caller-controlled selector that may choose a more favorable outcome.

Successful Deadbolt settlement or direct refutation requires admitted verifier
context. A bare typed evidence node labelled `DeadboltAnchor` proves neither
that verification occurred nor that the label's author had Deadbolt authority.

## Origin-Group and Corroboration-Separation Laws

Origin-group keys are opaque exact strings. Their equality has meaning only
under the exact origin-admission policy and contribution subject that admitted
them.

Rules:

- No prefix semantics.
- No inheritance.
- No containment.
- No fuzzy matching.
- No URL/domain inference in v1.
- No automatic source clustering.
- No probabilistic independence scoring.
- No "same publisher but different URL" inference unless future policy
  explicitly admits it.

Absent, malformed, ambiguous, conflicting, unsupported, unverified, or
unadmitted
origin material provides zero corroboration separation and must not amplify
standing. Future folds may retain it in a non-amplifying lane or refuse
corroborating aggregation.

A self-declared unique group must not count as corroboration separation.
Evidence producers cannot manufacture separation by choosing a fresh string
for every report.

Origin groups are not source truth. They only constrain whether multiple exact
admitted contributions may be treated as separate by a later aggregation
policy. Unknown origin counts as zero corroboration separation in the first
policy. Same-origin multiplicity may count at most once later.

## External-Source Corroboration

Law:

```text
ExternalSource cannot settle truth, even with corroboration policy.
```

`ExternalSource` may support `ExternalReport` or `Interpretation` only up to
`Supported`. Corroboration may help decide whether an external-source
contribution is admitted or remains weak, but corroboration cannot turn
`ExternalSource` into `Settled`.

Multiple external reports from the same origin group do not count repeatedly.
Reposted material, syndicated text, mirrored pages, copied abstracts, or
repeated citations may be useful provenance but do not create corroboration
separation unless explicit contribution-scoped origin bindings are admitted.

Human ratification of an external report may record a human judgment about the
report, but it does not settle the report's truth. A human can approve relying
on a report for a scoped decision or govern how a contradiction is handled;
they cannot make the report true by decree or erase truth-bearing contradiction
debt.

Model self-reports and lens readouts do not become supporting evidence merely by
repetition. Repeated model or lens outputs may seed hypotheses, but repetition
alone must not raise them above their ceiling or simulate corroboration
separation.

## Interaction With Support Ceilings

`support_ceiling` defines the maximum positive contribution permitted for an
evidence-kind/domain pair. Future support aggregation may combine admitted
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
`Refuted`, the non-authority policy checks, and admitted verifier context.

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

Future policy must define explicit precedence between direct refutation and
contradiction debt. Human ratification may govern action or handling, but it
must not erase truth-bearing contradiction debt or settle factual truth.

## Future Implementation Order

1. Ratify Ticket 0037's artifact-provenance and origin-admission contract.
2. Pin one exact versioned acquisition/derivation foreign-bundle schema and
   verifier profile.
3. Add portable bundle fixtures and standing-inert verification.
4. Pin one exact origin-binding bundle schema and explicit origin-admission
   policy.
5. Add a standing-inert origin-admission audit.
6. Add an admitted-contribution audit surface with no aggregation.
7. Pin one explicit standing policy v3 aggregation rule.
8. Implement conservative aggregation.
9. Add direct refutation through admitted verifier context.
10. Add contradiction debt with explicit precedence against direct refutation.
11. Add invalidation and supersession/currentness.
12. Add the capability-bearing `EpistemicGate` and writer-facing surfaces.
13. Add governed acquisition tooling and librarian proposals.

The direct policy-v2 rule consumes one successful closed verifier context and
contributes `Supported` without combining evidence. It introduces no numeric
threshold, source count, corroboration, or independence claim. Origin admission
is not support, one admitted group is not aggregation, and steps 2-6 must remain
standing-inert. Step 7 is future work: this note neither defines a threshold nor
creates policy v3.

## Future Tests

Future PRs should add tests with names such as:

- `external_sources_same_origin_group_do_not_amplify`
- `distinct_admitted_origin_groups_are_only_corroboration_separation`
- `external_source_corroboration_never_settles`
- `unknown_origin_provides_zero_corroboration_separation`
- `model_self_report_repetition_does_not_amplify`
- `lens_readout_repetition_does_not_amplify`
- `human_ratification_does_not_settle_external_report`
- `support_aggregation_does_not_revive_refuted_claim`
- `contradicts_edge_creates_debt_not_refutation`
- `invalidation_is_not_refutation`
- `supersession_is_not_deletion`
- `aggregation_is_order_independent`
- `aggregation_is_regenerable`
- `self_declared_origin_group_does_not_amplify`
- `conflicting_origin_bindings_admit_zero_groups`
- `bare_deadbolt_label_is_not_admitted_verifier_context`
- `trace_policy_id_is_not_an_outcome_selector`
- `human_ratification_does_not_erase_contradiction_debt`

These are future tests for later PRs. This note does not add tests.

## Reviewer Checklist

- Confirm this phase is docs/tickets only.
- Confirm origin groups are opaque policy strings, not inferred source
  clusters, truth labels, or publisher identities.
- Confirm the filename preserves earlier “independence group” shorthand while
  normative prose uses origin group and corroboration separation.
- Confirm missing, ambiguous, conflicting, or unknown origin material provides
  zero corroboration separation.
- Confirm self-declared or unadmitted origin groups never amplify.
- Confirm external-source corroboration cannot yield `Settled`.
- Confirm support and refutation aggregation remain separate from contradiction
  debt, invalidation, and supersession.
- Confirm `StandingResolution` v0 is described as the canonical governed
  explanation surface and `resolved_standing()` only as its compatibility
  scalar.
- Confirm artifact/bundle verification, origin-admission audit, and
  admitted-contribution audit all precede aggregation.
- Confirm origin admission is not support and one admitted group is not
  aggregation.
- Confirm no numeric threshold or implemented policy v3 appears.
- Confirm no runtime `StandingView` behavior changes are made by this docs PR.
