# Deadbolt Occurrence Achieved Standing v1

## Purpose

This note defines Magpie's first policy-bearing achieved-standing rule. It
consumes the fail-closed `StandingResolution` v0 candidate trace and exact
Deadbolt occurrence context from one `StandingReplaySnapshot`.

The rule settles only the structured proposition that one precise
`SegmentAnchored` identity occurs in the accepted Magpie chain. It does not
settle anything inferred from claim prose or foreign bundle contents.

## Exact rule

```text
supports edge
× exact scope match
× DeadboltAnchor evidence
× Occurrence/Inclusion claim
× exact structured claim predicate
× exact structured evidence reference
× exact anchor identity present in the co-replayed index
→ Settled
```

The full five-field identity is `bundle_kind`, `witness_root`,
`witness_algorithm`, foreign `canonicalization_profile`, and `run_id`. Claim
and evidence metadata must agree exactly, and the same identity must occur in
the anchor index co-derived with standing through the same successful replay.

This is a direct exact-proof rule. It is not confidence scoring,
corroboration, majority voting, aggregation, or evidence amplification.

## Policy identity

The explicit v1 identity is:

```text
magpie-claims-standing-v1
```

Callers choose it only through
`StandingReplaySnapshot::resolved_standing_v1` or
`StandingReplaySnapshot::resolved_standing_with_trace_v1`. There is no
`latest` alias, runtime default, environment selector, mutable policy, or
version negotiation.

## V0 preservation

`magpie-claims-standing-v0`, `StandingResolution`, `StandingTraceEntry`,
`StandingTraceReason`, and both `StandingView` resolver methods remain
unchanged. V0 still reports candidate ceilings, requires verifier context for
Deadbolt candidates, never applies achieved standing, and returns
`Conjectured` for a typed positive fixture.

The v1 scalar delegates to the v1 traced method. It does not change the
compatibility scalar on `StandingView`.

## Candidate and application layers

Each `StandingTraceEntryV1` preserves the exact v0 `StandingTraceEntry` as its
`candidate`. An optional `StandingPolicyApplication` records whether the one
closed v1 rule was attempted, its exact context outcome, and its achieved
contribution. The rule now executes through the private closed contribution-lane
path described in `standing-contribution-lanes-v1.md`; this is a control-flow
refactor, not a new rule or result.

```text
candidate evaluation
    !=
context satisfaction
    !=
achieved-standing application
```

V1 applies only when the candidate trace exactly identifies the accepted
`DeadboltAnchor × Occurrence/Inclusion` cell with ceiling `Settled` and reasons,
in order, `AcceptedCandidate`, `RequiresVerifierContext`, and
`CeilingIsCandidateOnly`. It then re-fetches the exact replayed edge, target,
and evidence nodes and rechecks source, target, edge kind, scopes, evidence
kind, ceiling, and context requirement before invoking the existing resolver.
Any disagreement fails closed without panic.

## Context outcomes

The serializable trace vocabulary is closed:

- `Matched`, carrying the exact `DeadboltAnchorOccurrence`;
- `WrongEvidenceKind`;
- `WrongClaimDomain`;
- `MissingClaimPredicate`;
- `MalformedClaimPredicate`;
- `MissingEvidenceReference`;
- `MalformedEvidenceReference`;
- `PredicateReferenceMismatch`;
- `AnchorNotFound`.

Only `Matched` produces `Some(Settled)`. Every other outcome produces no
achieved contribution. Failures remain distinguishable rather than being
collapsed into generic missing verifier context.

## Direct-proof semantics

The settled proposition is exactly:

> The structured five-field `SegmentAnchored` identity named by this claim and
> this typed evidence reference occurs in the accepted Magpie chain.

It does not establish interpretation truth, the correctness of statements in
the foreign bundle, policy wisdom, safety, desirability, scientific truth,
actor authority, human approval, generic Deadbolt trust, generic cryptographic
truth, or execution success beyond the exact occurrence proposition. Magpie
does not parse claim prose and does not replace the foreign bundle verifier.

## Blocker semantics

Top-level v1 blockers describe unresolved paths. `AcceptedCandidate` is never
a blocker. For a successfully matched path, `RequiresVerifierContext` and
`CeilingIsCandidateOnly` are satisfied by the v1 application and are omitted
for that path. An eligible but unmatched path retains them, and all other v0
blockers remain.

Blockers are deduplicated and sorted deterministically. One independently
matched direct path settles the claim even when an unrelated candidate remains
blocked. The unresolved path stays fully visible in trace and blockers; it
cannot become a denial-of-standing mechanism.

## Audit trace

`Matched` carries the exact anchor identity, sequence, event hash, and
provenance recorded by the co-replayed index. Sequence, hash, and provenance
are audit material rather than additional identity fields or authority.

`StandingResolutionV1::canonical_bytes` serializes the deterministic derived
resolution. These bytes are test material, not L0 encoding or a persistent
protocol. A representative value is pinned by integration tests.

## Authority limitations

No actor class, claim statement, evidence summary, `content_hash`, top-level
`verified` or `admitted` boolean, bare evidence-kind label, or caller prose can
manufacture settlement. Only the exact structured three-way identity match in
the same replay snapshot can satisfy the rule.

## Duplicate anchors

Repeated identical anchors remain one identity with multiple occurrences. The
existing resolver returns the earliest sequence deterministically. Repetition
does not create a second application, amplify epistemic strength, or produce a
status above `Settled`.

## Non-goals

No general support aggregation, corroboration, independence amplification,
deterministic-verification settlement, Deadbolt interpretation or operational
support, achieved `Supported` or `Refuted`, refutation application,
contradiction debt, invalidation, supersession, currentness, human admission,
ratification, `EpistemicGate`, writer surface, MCP surface, foreign bundle
verification, graph traversal, policy negotiation, chain-tip persistence, L0
event, fixture, or Deadbolt change is part of v1.

An anchor-only log creates no proposition and therefore no policy application.

## Contribution-lane structure

The deterministic contribution-lane refactor is specified in
`standing-contribution-lanes-v1.md`. `StandingPolicyRule` remains the serialized
lane identity, and the exact direct-proof semantics in this note remain
unchanged.
