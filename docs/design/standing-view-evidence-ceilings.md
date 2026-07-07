# StandingView Evidence Ceilings

## Status

Proposed design note.

## Purpose

This note defines the ADR-0002 evidence ceiling law that later
`StandingView` semantics must implement.

The current substrate can retain typed claims, typed evidence, and typed
justification edges. It deliberately does not yet implement support promotion,
settlement, contradiction debt, invalidation, supersession, ratification,
`EpistemicGate`, or writer-facing surfaces.

This note freezes the next rule layer before those semantics are implemented.

## Non-goals

- No Rust changes.
- No tests.
- No `docs/FORMAT.md` changes.
- No `Payload` changes.
- No canonical encoding changes.
- No golden vector changes.
- No Python verifier changes.
- No support promotion.
- No settlement implementation.
- No contradiction debt.
- No invalidation or supersession semantics.
- No ratification semantics.
- No `EpistemicGate`.
- No MCP, CLI, or ordinary-agent writer surfaces.
- No lens ingestion.
- No Deadbolt changes.
- No scope inheritance.
- No probabilistic fusion.
- No Dung, JTMS, ATMS, or AGM machinery.

## Current substrate

ADR-0002 and the following implementation PRs have established the substrate:

- L0 stores signed, hash-chained events, not mutable truth.
- Tags 6-8 exist as additive ADR-0002 vocabulary:
  `ClaimAssertedV2`, `EvidenceRegistered`, and
  `JustificationEdgeRecorded`.
- `StandingView` is a deterministic projection over verified replay.
- `StandingView` retains typed claim metadata in `TypedClaimNode`.
- `StandingView` retains typed evidence in `TypedEvidenceNode`.
- `StandingView` retains typed justification edges in
  `TypedJustificationEdge`.
- `ClaimAssertedV2` creates a `Conjectured` claim.
- `EvidenceRegistered` and `JustificationEdgeRecorded` are retained but inert.
- `SegmentAnchored` remains occurrence/inclusion evidence only; it does not
  create typed evidence, promote claims, or decide interpretation truth.

## Core rule

An evidence ceiling is the maximum standing contribution a piece of evidence
may make. It is not automatic promotion.

Evidence ceilings are deterministic caps, not probabilities and not numeric
confidence. A piece of evidence may contribute less than its ceiling, or
nothing at all, if source, target, actor class, scope, edge kind, or future
admission rules do not allow contribution.

Edges do not promote claims unless deterministic fold rules explicitly allow
that promotion. Refutation, contradiction debt, invalidation, and supersession
are separate from positive ceiling rules.

## Status vocabulary

The standing vocabulary remains:

- `Open`
- `Conjectured`
- `Supported`
- `Settled`
- `Refuted`

Ceilings in this note describe maximum positive contribution toward this
vocabulary. They do not define the full current standing algorithm.

## Evidence ceiling table

| evidence kind | maximum positive contribution | rule |
| --- | --- | --- |
| `DeterministicVerification` | `Settled` | Only for exact machine-checkable predicates. |
| `HumanRatification` | `Settled` | Only under explicit human authority and exact scope. |
| `DeadboltAnchor` | `Settled` for occurrence/inclusion claims; `Supported` for interpretation claims | Proves/refuses occurrence and inclusion, not meaning. |
| `ExecutionEvidence` | `Supported` | Supports operational or empirical claims; does not settle truth. |
| `BehavioralEvaluation` | `Supported` | Supports observed behavior under stated conditions; does not settle truth. |
| `ExternalSource` | `Supported` | May remain `Conjectured` if weak, stale, or uncorroborated. |
| `ModelSelfReport` | `Conjectured` | Can seed hypotheses; cannot support or settle by itself. |
| `LensReadout` | `Conjectured` | Can seed model-internal hypotheses; cannot support or settle by itself in v1. |

### DeterministicVerification

Maximum positive contribution: `Settled`.

Only exact machine-checkable predicates qualify. Examples include:

- hash equality;
- signature validity;
- chain inclusion;
- verifier pass/fail;
- deterministic replay equivalence;
- exact parser acceptance or rejection.

Restrictions:

- Does not settle broad interpretation claims.
- Does not settle claims outside the exact verified predicate.
- Must identify the method or verifier in metadata or future policy context.
- If the predicate is not exact and machine-checkable, treat it as at most
  `Supported` or reject contribution until a later rule exists.

### HumanRatification

Maximum positive contribution: `Settled`.

Human ratification may settle only under explicit human authority and exact
scope.

Restrictions:

- Requires `actor_class = HumanRoot` or a future authority rule admitted by
  `EpistemicGate`.
- Does not escape `scope_ref`.
- Does not override deterministic contradictions without explicit later
  invalidation or supersession rules.
- Does not let ordinary agents settle claims.

### DeadboltAnchor

Maximum positive contribution: `Settled` for occurrence/inclusion claims and
`Supported` for interpretation claims.

Deadbolt can prove or refuse that a governed execution, witness, or bundle
occurred and was included. It cannot decide what that occurrence means.

Restrictions:

- Can settle claims like "bundle X was anchored with witness root Y".
- Can settle claims like "run ID R is included in this chain".
- Cannot settle claims like "the anchored result is scientifically true".
- Cannot settle claims like "the policy decision was wise".
- May support interpretation claims only when connected through explicit
  justification edges and future fold rules.

### ExecutionEvidence

Maximum positive contribution: `Supported`.

Execution evidence records that something ran, produced output, passed a smoke
test, or behaved as observed.

Restrictions:

- Does not settle truth.
- Does not settle broad interpretation claims.
- Can support operational or empirical claims.
- If the execution evidence is also deterministic and machine-checkable, it
  should be represented as `DeterministicVerification`, not silently upgraded.

### BehavioralEvaluation

Maximum positive contribution: `Supported`.

Behavioral evaluation records a benchmark, evaluation, observed model or system
behavior, test suite result, or comparative behavior.

Restrictions:

- Does not settle truth.
- May be sample-sensitive.
- Must not be treated as universal proof.
- Can support claims about observed behavior under stated conditions.

### ExternalSource

Maximum positive contribution: `Supported`.

External sources record outside documents, pages, papers, reports, articles,
repositories, or other references.

Restrictions:

- Does not settle truth by itself.
- Source credibility, corroboration, and freshness are future policy concerns.
- May remain only `Conjectured` if weak, stale, or uncorroborated.
- Cannot exceed `Supported` in v1 without human ratification or deterministic
  verification.

### ModelSelfReport

Maximum positive contribution: `Conjectured`.

Model self-report records what a model says about itself, its reasoning, its
memory, its confidence, or a generated explanation.

Restrictions:

- Cannot support or settle claims by itself.
- Must not be treated as ground truth.
- May seed hypotheses or candidate claims.
- Requires external evidence to rise above `Conjectured`.

### LensReadout

Maximum positive contribution: `Conjectured`.

Lens readout records interpretability, lens, probe, or derived model-internal
observations.

Restrictions:

- Cannot support or settle claims by itself in v1.
- May seed hypotheses.
- May become supporting evidence in a future stronger regime, but not now.
- Requires external validation or deterministic verification to rise above
  `Conjectured`.

## Claim-domain distinction

Claim domains are not yet encoded in L0 tags. They are part of future
`StandingView` and `EpistemicGate` policy.

Future semantics should distinguish at least these domains:

- `Occurrence/Inclusion`: whether an event, run, bundle, witness root, chain
  record, or inclusion fact exists.
- `ExactMachineCheckable`: a predicate decidable by deterministic machinery.
- `OperationalObservation`: a bounded observation about execution, output,
  behavior, or environment.
- `Interpretation`: a claim about meaning, explanation, causality, scientific
  truth, or policy wisdom.
- `ExternalReport`: a claim attributed to an external source.
- `ModelIntrospection`: a claim about model-internal state, representation, or
  self-report.
- `HumanJudgment`: a scoped human acceptance, review decision, or ratification.

Domain constraints:

- `DeadboltAnchor` can settle `Occurrence/Inclusion`, not `Interpretation`.
- `DeterministicVerification` can settle `ExactMachineCheckable`, not broad
  `Interpretation`.
- `ExecutionEvidence` can support `OperationalObservation`.
- `BehavioralEvaluation` can support `OperationalObservation` or bounded
  `Interpretation`, but not settle.
- `ExternalSource` can support `ExternalReport` or interpretation claims, but
  not settle.
- `ModelSelfReport` can only conjecture `ModelIntrospection` or candidate
  hypotheses.
- `LensReadout` can only conjecture model-internal hypotheses in v1.
- `HumanRatification` can settle `HumanJudgment` or scoped interpretation only
  with explicit authority.

## Edge interaction rules

### supports

- May contribute positive standing up to the source evidence ceiling.
- Does not automatically promote.
- Requires source and target existence.
- Requires exact compatible `scope_ref` in v1.
- Future fold must be deterministic.

### ratifies

- May settle only under `HumanRatification` or a future explicit authority rule.
- Requires exact scope.
- Does not allow ordinary agents to settle claims.
- Should be gated by future `EpistemicGate` before writer surfaces exist.

### derived_from

- Preserves lineage.
- Does not itself promote standing.
- May be used by future fold rules to trace ancestry.

### contradicts

- Creates debt or blocks settlement in a future semantics PR.
- Does not automatically refute in this design note.
- Must not be implemented yet.

### invalidates

- May remove or neutralize contribution from a source in a future semantics PR.
- Does not delete events.
- Must preserve historical lineage.

### supersedes

- Marks currentness and lineage in a future semantics PR.
- Does not delete or rewrite old claims.
- Must not silently erase old standing without explicit fold rules.

## Scope rules

`scope_ref` is opaque exact-match string material in v1.

- No inheritance.
- No containment.
- No wildcard semantics.
- No prefix semantics.
- No ontology, lattice, or implied parent scope.
- A support or ratification edge cannot escape its exact scope unless a future
  ADR amends or supersedes ADR-0002.

Corrections remain new events. They are never edits or deletes.

## SegmentAnchored boundary

`SegmentAnchored` is occurrence/inclusion evidence, not interpretation truth.

Typed `EvidenceRegistered` may cite an anchor, a witness root, a run ID, or a
verifier result. The anchor alone does not create typed evidence, promote a
claim, settle interpretation, or decide what a result means.

This preserves the ADR-0001/ADR-0002 boundary:

- Deadbolt acts.
- Magpie remembers.
- `StandingView` derives standing by deterministic replay.
- Future writer surfaces remain blocked until `EpistemicGate`.

## Examples

### Valid ceiling applications

```text
DeadboltAnchor + supports + claim "run-0001 was anchored with witness_root X"
=> may reach Settled as occurrence/inclusion.
```

```text
DeadboltAnchor + supports + claim "the anchored theorem is true"
=> at most Supported, not Settled.
```

```text
ModelSelfReport + supports + claim "model says it solved the theorem"
=> at most Conjectured.
```

```text
LensReadout + supports + claim "the model internally represented causal feature F"
=> at most Conjectured in v1.
```

```text
DeterministicVerification + supports + claim "golden seqs 0-8 verify under key K"
=> may reach Settled if exact verifier predicate passes.
```

```text
ExecutionEvidence + supports + claim "command X passed on machine Y at time T"
=> may reach Supported unless represented as exact deterministic verification.
```

```text
HumanRatification + ratifies + claim "this interpretation is accepted for scope S"
=> may reach Settled only under explicit HumanRoot authority and exact scope.
```

### Invalid promotions

```text
SegmentAnchored directly settles interpretation truth.
```

Invalid.

```text
AgentProposer asserts a claim and it becomes Settled.
```

Invalid.

```text
ModelSelfReport promotes a claim to Supported by itself.
```

Invalid.

```text
LensReadout settles a claim about the world.
```

Invalid.

```text
DeadboltAnchor settles "this theorem is true" because a witness bundle exists.
```

Invalid.

```text
ExternalSource settles a claim without ratification or deterministic verification.
```

Invalid.

## Future test matrix

Future PRs should add tests with names such as:

- `model_self_report_support_does_not_promote_beyond_conjectured`
- `lens_readout_support_does_not_promote_beyond_conjectured`
- `deadbolt_anchor_settles_occurrence_not_interpretation`
- `deterministic_verification_can_settle_exact_predicate`
- `execution_evidence_supports_but_does_not_settle`
- `external_source_supports_but_does_not_settle`
- `human_ratification_requires_human_root_and_exact_scope`
- `support_requires_existing_source_and_target`
- `support_requires_exact_scope_match`
- `derived_from_preserves_lineage_without_promotion`
- `contradiction_blocks_settlement_without_auto_refuting`
- `invalidates_neutralizes_without_deleting`
- `supersedes_marks_currentness_without_deleting`
- `segment_anchor_does_not_create_evidence_or_settle_claim`

These are future tests for later PRs. This note does not add tests.

## Rollout order

1. This PR: docs-only evidence ceiling law.
2. Next PR: test-only or scaffold PR for ceiling fixtures.
3. Implement `supports` for no-promotion ceiling cases first:
   `ModelSelfReport`, `LensReadout`, and conservative `ExternalSource`.
4. Implement `supports` for `ExecutionEvidence` and `BehavioralEvaluation` to
   `Supported`.
5. Implement exact deterministic verification settlement.
6. Implement Deadbolt occurrence/inclusion settlement while preserving the
   interpretation boundary.
7. Implement HumanRoot ratification.
8. Implement contradiction debt.
9. Implement invalidation and supersession.
10. Only then design `EpistemicGate` and writer-facing surfaces.

This order keeps the lowest-ceiling no-promotion cases ahead of settlement
cases. It proves that the projection can refuse overpromotion before it learns
to settle.

## Stop conditions

Stop before implementation if:

- defining ceilings appears to require Rust changes;
- defining ceilings appears to require `docs/FORMAT.md` changes;
- defining ceilings appears to require `Payload` changes;
- defining ceilings appears to require canonical encoding changes;
- defining ceilings appears to require golden vector changes;
- defining ceilings appears to require Python verifier changes;
- evidence ceilings cannot be defined without adding claim-domain fields to L0;
- support, debt, or ratification semantics seem necessary now;
- `EpistemicGate` seems necessary now;
- writer-facing surfaces seem necessary now;
- Deadbolt changes seem necessary now.

## Reviewer checklist

- Confirm this PR is docs-only.
- Confirm every ADR-0002 evidence kind has a ceiling.
- Confirm ceilings are deterministic caps, not probabilities.
- Confirm ceilings are not automatic promotion.
- Confirm claim domains are future policy, not new L0 fields.
- Confirm `scope_ref` remains opaque exact-match string material.
- Confirm `SegmentAnchored` remains occurrence/inclusion evidence and not
  interpretation truth.
- Confirm edge rules defer promotion, contradiction debt, invalidation,
  supersession, and ratification implementation.
- Confirm future tests cover low-ceiling refusal before settlement.
- Confirm writer-facing surfaces remain blocked until `EpistemicGate`.
