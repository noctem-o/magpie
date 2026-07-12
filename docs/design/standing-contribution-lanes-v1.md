# Standing Contribution Lanes v1

## Purpose

This note records a behaviour-preserving refactor of
`magpie-claims-standing-v1`.

The architectural law is:

```text
Every achieved-standing path is represented as one deterministic,
closed contribution lane, while current v1 results, serialized traces,
canonical bytes, and authority boundaries remain unchanged.
```

This change makes the existing exact Deadbolt occurrence proof path reusable
as a code shape. It does not make any additional candidate achieve standing.

## Latent structure from PR #39

PR #39 already contained the stages of one contribution lane, but the lane was
expressed as a one-off method call:

1. `StandingView` classified replayed graph material into deterministic v0
   candidate traces.
2. Policy v1 recognized one exact Deadbolt candidate trace.
3. The snapshot resolver re-fetched and revalidated the replayed edge, claim,
   and evidence nodes.
4. Exact co-replayed Deadbolt context produced one bounded direct contribution
   or none.
5. The claim-level resolver used successful direct contribution to produce the
   existing v1 result and blocker explanation.

The refactor makes steps 2 and 3 explicit as closed lane selection followed by
closed lane execution. The checks and results within every stage stay the same.

## Implemented shape

The private policy-v1 flow is:

```text
v0 StandingTraceEntry
→ classify_contribution_lane(candidate) -> Option<StandingPolicyRule>
→ apply_contribution_lane(snapshot, claim_id, candidate, selected rule)
→ exhaustive match on StandingPolicyRule
→ re-fetch and revalidate replayed nodes
→ resolve exact co-replayed Deadbolt context
→ existing StandingPolicyApplication or no application
→ existing claim-level v1 result and blockers
```

`classify_contribution_lane` accepts only the existing exact candidate shape:

- an edge identity and source identity are present;
- evidence kind is exactly `DeadboltAnchor`;
- claim domain is exactly `Occurrence/Inclusion`;
- candidate ceiling is exactly `Settled`;
- reasons are exactly, and in order, `AcceptedCandidate`,
  `RequiresVerifierContext`, and `CeilingIsCandidateOnly`.

That candidate selects exactly
`StandingPolicyRule::DeadboltOccurrenceInclusionV1`. Every other candidate
selects no lane.

Lane execution is a separate private function with an exhaustive match over
`StandingPolicyRule`. There is no wildcard arm. The sole arm calls the existing
Deadbolt-specific revalidation and context attempt. Adding a future enum
variant therefore creates a compile-time obligation to implement its lane
dispatch; merely adding the variant grants no authority.

No private duplicate lane enum is introduced. The public
`StandingPolicyRule` already supplies the one-to-one closed identity the
implementation needs.

## Separation of stages

The stages retain distinct meanings:

- candidate classification is v0 explanation, not achieved standing;
- lane selection recognizes one exact candidate structure, but contributes
  nothing;
- context attempt re-fetches replayed nodes and evaluates co-replayed context;
- bounded direct contribution is the existing `Some(Settled)` only for
  `DeadboltOccurrenceContextTrace::Matched`;
- claim-level result preserves the existing rule that any successful direct
  path may settle the proposition while unrelated unresolved paths remain
  visible.

The lane never trusts candidate trace fields alone. Execution still re-fetches
the edge, target, and evidence from the same private
`StandingReplaySnapshot`; rechecks source, target, `supports`, exact scope,
evidence kind, ceiling, and context requirement; and then invokes the exact
Deadbolt occurrence resolver over that snapshot's anchor index.

## Contribution-lane laws

1. A candidate maps to zero or one policy-v1 contribution lane.
2. Lane selection is deterministic and closed.
3. A selected lane may produce one bounded direct contribution or none.
4. A lane application is not aggregation.
5. The number of lane applications does not itself affect standing.
6. Repetition creates no threshold, vote, confidence score, or corroboration
   effect.
7. A contribution may not exceed its existing policy ceiling.
8. Only replay-derived context from the same `StandingReplaySnapshot` may
   satisfy the current direct-proof lane.
9. A manually constructed public trace or resolution value does not prove
   trusted resolver provenance.
10. A future rule receives no authority merely by being added to an enum. It
    must define exact structural eligibility, replayed-node revalidation,
    replayable context, a bounded contribution, and deterministic tracing.

## Direct contribution is not aggregation

The current lane proves one exact proposition directly. It does not combine
weaker inputs to reach a result. Claim-level resolution observes whether any
application produced the existing direct `Settled` contribution; it does not
count applications.

Two independently represented candidate paths may each retain their own
deterministic trace and successful application. The result is still exactly
`Settled`. Trace multiplicity is audit material. It is not a stronger status,
a vote, a score, a threshold, or a claim that there are "two proofs."

Duplicate identical anchors retain the existing earliest-occurrence selection
and likewise do not amplify standing.

## Serialized identity and exact byte preservation

`StandingPolicyRule` remains the serialized lane identity through the existing
`StandingPolicyApplication.rule` field. A second identifier would duplicate a
closed concept and could drift, so this change adds no serialized `lane` field.

The public serialized shapes remain unchanged:

- `StandingResolutionV1`;
- `StandingTraceEntryV1`;
- `StandingPolicyApplication`;
- `StandingPolicyRule`;
- `DeadboltOccurrenceContextTrace`.

Field names, field order, enum serialization, variant names, optional-field
behaviour, trace ordering, blocker ordering, matched occurrence identity,
sequence, event hash, and provenance remain unchanged. The literal positive
fixture assertion for `StandingResolutionV1::canonical_bytes()` is unchanged
and is named as a contribution-lane preservation regression.

Policy v0 is also frozen. Its types, policy ID, candidate behaviour,
representative canonical bytes, and `StandingView` scalar delegation are
unchanged. The positive fixture remains v0 `Conjectured` and v1 `Settled`.

## Trusted-construction claim

### What this construction proves

The policy-v1 resolver selected and evaluated every achieved-standing attempt
through exactly one closed contribution lane over the v0 candidate and the
co-replayed context contained in the same `StandingReplaySnapshot`.

### What it does not prove

- that the caller selected the socially correct Magpie verifying key;
- that the foreign Deadbolt bundle contents are valid;
- that the anchored action was wise, safe, successful, or desirable;
- that any interpretation of the occurrence is true;
- that actor-class or evidence-kind labels are authoritative;
- that a manually constructed `StandingResolutionV1` came from the trusted
  resolver;
- that any aggregation, corroboration, admission, or refutation rule exists.

### Sole legitimate origin path

One successful `LogReader::replay` into the private composite projection used
by `replay_standing_context`, followed by the explicit snapshot-only policy-v1
resolver.

### Manual or adversarial bypasses

The trusted-construction claim excludes:

- caller-created resolution, application, or trace structs;
- independently constructed `StandingView` and `DeadboltAnchorIndex`
  projections;
- two independent replay calls, even over apparently identical stores;
- a mutable store returning different snapshots to different replay calls;
- self-asserted authority metadata or actor classes;
- bare evidence-kind labels;
- caller-provided `verified` or `admitted` booleans;
- unverified foreign bundle contents;
- adding a `StandingPolicyRule` variant without exact classification,
  exhaustive lane execution, replayed-node revalidation, and bounded context.

The public context and trace structs remain useful audit values, but their mere
construction does not prove they came from the trusted resolver.

### Hostile regression

This PR prevents a future contributor from adding another achieved-standing
path as an ad hoc side branch that bypasses closed lane classification,
replayed-node revalidation, exact context resolution, bounded contribution,
deterministic tracing, or snapshot provenance.

## Adding a future direct-proof family safely

A future direct-proof family must be a separate reviewed policy change. It must:

1. add an explicit `StandingPolicyRule` variant;
2. define one exact candidate classifier mapping to that rule;
3. add an exhaustive dispatch arm;
4. re-fetch and revalidate all authoritative replayed material;
5. use context derived from the same `StandingReplaySnapshot`;
6. cap its contribution at an existing, explicitly reviewed ceiling;
7. preserve deterministic trace and blocker behaviour or version them
   explicitly;
8. add hostile tests showing that labels, prose, metadata booleans, repetition,
   and independently constructed projections cannot manufacture the result.

No rule is activated by registration, callback, runtime selection, or enum
membership alone.

## Tests

Private unit tests pin exact closed classification and reject wrong evidence
kind, wrong claim domain, wrong ceiling, missing identities, altered reason set
or order, and unknown vocabulary. Because classification returns
`Option<StandingPolicyRule>`, one candidate can select at most one lane.

The exhaustive execution match has no wildcard; a new rule variant cannot
compile without an explicit dispatch decision. Integration tests retain every
existing positive, hostile, blocker, duplicate-anchor, anchor-only, replay, and
canonical-byte check. A new two-path regression proves that both successful
applications remain in trace while the claim result remains exactly `Settled`
without an amplification field or result.

## Explicit non-goals

This refactor adds no achieved-standing rule, achieved `Supported`, achieved
`Refuted`, deterministic-verification settlement, aggregation, threshold,
evidence count, vote, confidence score, corroboration, independence group,
source-count behaviour, admission, `EpistemicGate`, contradiction debt,
invalidation, supersession, currentness, runtime policy selection, writer
surface, MCP path, graph traversal, retrieval, Deadbolt change, or L0 change.
