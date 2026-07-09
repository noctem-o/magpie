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

A belief is a replay result, not a string in memory. Standing is never stored
as an authoritative mutable field; it is always the deterministic result of
replaying L0 through the fold. Everything below caps what a replay is *allowed
to conclude* — it does not record conclusions.

An evidence ceiling is the maximum standing contribution a piece of evidence
may make. It is not automatic promotion. In short: `ceiling != achieved
standing`.

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

## Human authority is not truth authority

Corrected law:

```text
Humans settle nothing epistemically.
```

Human authority is real, but it lives on the governance and consent axis, not
the truth axis. A human may approve an action, ratify a preference, record a
judgment, accept operational risk, or provide evidence that a human judgment
occurred. That evidence may support a claim under bounded rules. It does not
make the claim true by decree.

A human can approve a risky action; they cannot make a hash match by decree.

`Settled` is not an honorific for trust in the actor. It is a standing result
reserved for claims whose truth is established by deterministic verification,
cryptographic verification, exact replay equivalence, or exact
occurrence/inclusion evidence admitted by policy. General interpretation claims
do not become `Settled` merely because a human, even the maintainer or
`HumanRoot`, ratified them.

Human ratification may support the claim "a human ratified X" as a
governance/judgment record while not settling "X is true." Later policy may
settle the occurrence of ratification if that occurrence is logged, signed, and
verified; it must not settle the truth of the ratified statement merely from
the ratification.

This mirrors the existing treatment of model self-reports and lens readouts:
they are admissible signals with strict ceilings, not privileged truth oracles.
Claims remain contestable unless they are deterministic, cryptographic, exact
replay, or exact occurrence/inclusion facts admitted by explicit policy.

The standing boundary is therefore:

```text
Deadbolt governs permission.
Magpie derives standing.
Humans approve actions/preferences/judgments.
Humans do not settle truth.
Models propose or witness.
Models do not settle truth.
```

## Evidence ceiling table

| evidence kind | maximum positive contribution | rule |
| --- | --- | --- |
| `DeterministicVerification` | `Settled` | Only for exact machine-checkable predicates. |
| `HumanRatification` | `Supported` | Governance/judgment evidence only; humans do not settle truth. |
| `DeadboltAnchor` | `Settled` for occurrence/inclusion claims with successful verifier context; `Supported` for interpretation claims | Proves/refuses verified occurrence and inclusion, not meaning. |
| `ExecutionEvidence` | `Supported` | Supports operational or empirical claims; does not settle truth. |
| `BehavioralEvaluation` | `Supported` | Supports observed behavior under stated conditions; does not settle truth. |
| `ExternalSource` | `Supported` | May remain `Conjectured` if weak, stale, or uncorroborated. |
| `ModelSelfReport` | `Conjectured` | Can seed hypotheses; cannot support or settle by itself. |
| `LensReadout` | `Conjectured` | Can seed model-internal hypotheses; cannot support or settle by itself in v1. |

The table above is the one-dimensional summary. The authoritative ceiling is
two-dimensional: the same evidence kind carries different authority in different
claim domains. The matrix below is the cross-reference the future fold and
`EpistemicGate` read; the per-kind sections that follow it are rationale.

## Evidence ceiling matrix (evidence_kind × claim_domain)

A ceiling is a function of both axes: `ceiling = f(evidence_kind, claim_domain)`.
Cell values are the maximum positive contribution and are drawn only from
`Settled` / `Supported` / `Conjectured` / `—` (no admissible contribution).

Filling principle: each evidence kind reaches its per-kind ceiling only in its
home domain(s). Where it bears on an adjacent domain without being authoritative
there, it contributes at most one juridical level lower. Where it has no
admissible bearing, the cell is `—`. No cell exceeds that kind's row maximum in
the one-dimensional table above.

| evidence kind ↓ / claim domain → | Occurrence/​Inclusion | ExactMachine​Checkable | Operational​Observation | Interpretation | ExternalReport | ModelIntro​spection | HumanJudgment |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `DeterministicVerification` | `Settled` | `Settled` | `Supported` | `—` | `—` | `—` | `—` |
| `HumanRatification` | `—`† | `—`† | `Supported`† | `Supported`† | `—`† | `—`† | `Supported`† |
| `DeadboltAnchor` | `Settled`‡ | `Supported` | `Supported` | `Supported` | `—` | `—` | `—` |
| `ExecutionEvidence` | `—` | `—` | `Supported` | `—` | `—` | `—` | `—` |
| `BehavioralEvaluation` | `—` | `—` | `Supported` | `Supported`§ | `—` | `—` | `—` |
| `ExternalSource` | `—` | `—` | `—` | `Supported` | `Supported` | `—` | `—` |
| `ModelSelfReport` | `—` | `—` | `—` | `Conjectured` | `—` | `Conjectured` | `—` |
| `LensReadout` | `—` | `—` | `—` | `Conjectured` | `—` | `Conjectured` | `—` |

† `HumanRatification` contributes only through an `EpistemicGate` admission rule;
a bare `actor_class = HumanRoot` string is not standing authority. Human
ratification may support `HumanJudgment`, scoped `Interpretation`, or bounded
`OperationalObservation`; it does not settle them. It has no direct positive
standing contribution for exact machine-checkable, occurrence/inclusion,
external-report, or model-introspection truth. Those domains require their own
evidence policy.

‡ `DeadboltAnchor` settles `Occurrence/Inclusion` only with successful verifier
context; a node merely labelled `DeadboltAnchor` is not enough.

§ `BehavioralEvaluation` may support only *bounded* interpretation under stated
conditions; it never settles interpretation.

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

Matrix note: `DeterministicVerification` × `ExactMachineCheckable` = `Settled`;
`DeterministicVerification` × `OperationalObservation` = `Supported`. If an
operational observation is expressed as an exact machine-checkable predicate,
classify it as `ExactMachineCheckable`. If it depends on environment,
interpretation, sampling, or empirical context, classify it as
`OperationalObservation` and cap at `Supported`.

### HumanRatification

Maximum positive contribution: `Supported`.

Human ratification is governance/judgment evidence. It can record that a human
approved an action, accepted a preference, made a scoped judgment, or ratified
an operational decision. It cannot make the ratified proposition true.

Restrictions:

- Requires `actor_class = HumanRoot` or a future authority rule admitted by
  `EpistemicGate`.
- The support rule must not be implemented before the corresponding
  `EpistemicGate` admission rule exists; an L0 `actor_class` string is not
  standing authority by itself.
- Does not escape `scope_ref`.
- Does not override deterministic contradictions without explicit later
  invalidation or supersession rules.
- Does not let ordinary agents settle claims.
- Does not let humans settle claims by decree.

Matrix note: `HumanRatification` may support `HumanJudgment`, scoped
`Interpretation`, and bounded `OperationalObservation`. It does not settle
machine-decidable domains (`Occurrence/Inclusion`, `ExactMachineCheckable`) by
override. A human ratification must not override deterministic contradiction;
exact machine predicates are settled by `DeterministicVerification`, not by
human assertion. Occurrence/inclusion settlement belongs to exact Deadbolt
anchor or deterministic verification policy, not human assertion.

### DeadboltAnchor

Maximum positive contribution: `Settled` for occurrence/inclusion claims only
when paired with successful verifier context, and `Supported` for
interpretation claims.

Deadbolt can prove or refuse that a governed execution, witness, or bundle
occurred and was included. It cannot decide what that occurrence means.

Restrictions:

- Requires successful verifier context before any occurrence/inclusion
  settlement. A typed `EvidenceRegistered` node that merely labels itself
  `DeadboltAnchor` is not enough.
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

Matrix note: `ExecutionEvidence` × `Interpretation` = no direct contribution.
Operational evidence may support operational observations; interpretive bearing
must travel through explicit claims and justification edges, not a direct matrix
cell. For example, "command X passed on machine Y" may be `Supported`, but
"therefore the architecture is correct" requires a separate interpretive claim
and supporting rationale.

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
- Cannot exceed `Supported` in v1. Human ratification may record a judgment
  about the source, but it does not settle the external report's truth.

### ModelSelfReport

Maximum positive contribution: `Conjectured`.

Model self-report records what a model says about itself, its reasoning, its
memory, its confidence, or a generated explanation.

Restrictions:

- Cannot support or settle claims by itself.
- Must not be treated as ground truth.
- May seed hypotheses or candidate claims.
- Requires external evidence to rise above `Conjectured`.

Law: model self-report is a hypothesis seed, not support.

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

Law: lens/probe output is a hypothesis seed in v1 unless externally validated.

## Claim-domain distinction

Claim domains are a **closed v1 vocabulary**, exactly like the closed
`actor_class`, `evidence_kind`, and `edge_kind` sets. The admissible domains are
the seven listed below and no others; adding one is an amendment to this note, to
the metadata convention (`docs/design/metadata-conventions.md`), and to any
future `EpistemicGate` policy. An open-ended domain vocabulary would reintroduce
the relation-slop the ADR rejected for edge kinds.

Claim domains are not yet encoded in L0 tags. *Where* a claim carries its domain
is defined by the metadata convention, not here: in v1 it is a `claim_domain` key
inside `metadata_json`, and reading it stays advisory until `EpistemicGate`
enforces it.

The `claim_domain` vocabulary is closed by future `StandingView` / `EpistemicGate`
policy. It is not currently enforced by L0 payload validation because
`claim_domain` is not yet a dedicated L0 field. This PR does not mutate tags 6-8
and does not add a `claim_domain` field to `ClaimAssertedV2`; the vocabulary is
introduced as semantic policy and metadata convention, never by changing shipped
payload fields.

The seven admissible v1 claim domains are:

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

- `DeadboltAnchor` can settle `Occurrence/Inclusion` only with successful
  verifier context, not `Interpretation`.
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
- `HumanRatification` can support `HumanJudgment` or scoped interpretation only
  with explicit authority; it does not settle truth.

## Domain assignment is admitted policy material

Domain assignment is admitted policy material, not an inference from text.

- `StandingView` must not infer a stronger domain merely because a claim is
  phrased confidently.
- Domain assignment is an assertion-time or gate-time classification, not a truth
  discovered from prose alone.
- If a claim has absent, ambiguous, conflicting, or unsupported domain
  classification, future fold rules must choose the weaker applicable ceiling or
  refuse promotion.
- A broad `Interpretation` claim must not be automatically reclassified as
  `ExactMachineCheckable` just because it contains a checkable subclaim.

This prevents future ceiling escalation by classification drift.

## Edge interaction rules

### supports

- May contribute positive standing only up to both the source standing and the
  source evidence ceiling.
- Source claims cannot amplify a target beyond their own derived standing.
  Unsupported or conjectured source claims cannot promote a target past that
  standing.
- Evidence sources use their evidence ceiling. Claim sources use their derived
  standing plus any applicable ceiling rules.
- Does not automatically promote.
- Requires source and target existence.
- Requires exact compatible `scope_ref` in v1.
- Future fold must be deterministic.

### ratifies

- Records governance, consent, preference, or scoped human judgment.
- May support a claim only up to the `HumanRatification` ceiling.
- Requires exact scope.
- Does not allow ordinary agents to settle claims.
- Does not allow human actors to settle truth by decree.
- Must not treat `actor_class = HumanRoot` as authority unless the event has
  passed the applicable `EpistemicGate` admission rule.
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

### edge targeting and objections

`JustificationEdgeRecorded` source/target references may later target claims,
evidence, or edges by policy. The future interpretation convention over the
opaque string refs is namespaced:

- `claim:<claim_id>`
- `evidence:<evidence_id>`
- `edge:<edge_id>`

This is a future interpretation convention over opaque string refs, not a format
change; the L0 encoding of `source_id` / `target_id` stays an opaque string.

Edge targeting lets an objection be represented without a new event type: an
objection is an `invalidates` or `contradicts` edge targeting a support edge. For
example, `edge:e2 invalidates edge:e1 because e1 has a scope mismatch`.

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

## BeliefChangeLedger

`BeliefChangeLedger` is not a new source of truth and not a second persistent
store. It is a materialized explanation trace of the deterministic `StandingView`
fold — a replay trace that answers "Why does Magpie believe X?" by replaying the
same events and rules.

- It is derived, regenerable, and holds no authority L0 does not already hold.
- It records why standing changed (which evidence, which edges, which ceiling
  rule), never a conclusion that L0 did not produce.
- It obeys the derived-views-only rule: a demo surface, not a write path.

## Examples

### Valid ceiling applications

```text
DeadboltAnchor + successful verifier context + supports + claim "run-0001 was anchored with witness_root X"
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
=> may support that scoped human judgment under explicit HumanRoot authority and exact scope; it does not settle the interpretation's truth.
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
DeadboltAnchor settles occurrence/inclusion without successful verifier context.
```

Invalid.

```text
ExternalSource settles a claim without deterministic verification or explicit
corroboration policy.
```

Invalid.

```text
HumanRatification settles "this interpretation is true" because HumanRoot ratified it.
```

Invalid.

```text
HumanRatification settles "hash H matches artifact A" because a human approved it.
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
- `human_ratification_supports_judgment_but_does_not_settle_truth`
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
6. Implement Deadbolt occurrence/inclusion settlement only with successful
   verifier context while preserving the interpretation boundary.
7. Define explicit policy enums and exhaustive support ceilings with no
   wildcard catch-all.
8. Define separate `support_ceiling`, `refutation_ceiling`, and achieved
   standing aggregation. `support_ceiling` is maximum attainable support, not
   automatic promotion.
9. Add independence-group corroboration for external sources.
10. Implement contradiction debt.
11. Implement invalidation and supersession.
12. Design and implement the `EpistemicGate` admission slice for governed
    writes, including human ratification as governance/judgment evidence.
13. Only then expose writer-facing surfaces.

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
- evidence ceiling correction seems to require achieved-standing aggregation;
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
- Confirm `HumanRatification` cannot yield `Settled` in any claim domain.
- Confirm `Settled` is reserved for deterministic, cryptographic, exact replay,
  or exact occurrence/inclusion facts admitted by policy.
- Confirm edge rules defer promotion, contradiction debt, invalidation,
  supersession, and ratification implementation.
- Confirm future code guidance keeps `support_ceiling`, `refutation_ceiling`,
  and achieved standing separate.
- Confirm future tests cover low-ceiling refusal before settlement.
- Confirm writer-facing surfaces remain blocked until `EpistemicGate`.
