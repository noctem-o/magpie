# Ticket 0029: Deterministic standing contribution lanes v1

## Goal

Refactor the existing policy-v1 achieved-standing implementation so every
attempt travels through one deterministic, closed contribution lane without
changing any epistemic behaviour or observable output.

Architectural law:

```text
Every achieved-standing path is represented as one deterministic,
closed contribution lane, while current v1 results, serialized traces,
canonical bytes, and authority boundaries remain unchanged.
```

## Implementation landed

The private resolver now performs these explicit steps:

1. obtain the unchanged v0 candidate trace;
2. call `classify_contribution_lane`, which returns
   `Option<StandingPolicyRule>`;
3. call `apply_contribution_lane` only for the selected rule;
4. dispatch through an exhaustive `StandingPolicyRule` match with no wildcard;
5. re-fetch and revalidate the same replayed nodes;
6. attempt the existing exact Deadbolt occurrence context;
7. emit the existing `StandingPolicyApplication` or no application;
8. derive the unchanged claim-level result and blockers.

The only classification is the exact existing `DeadboltAnchor ×
Occurrence/Inclusion × Settled` v0 candidate with its exact three reasons in
their existing order. It selects only
`StandingPolicyRule::DeadboltOccurrenceInclusionV1`.

No private duplicate lane enum, public type, public method, serialized field, or
dependency is added. `StandingPolicyRule` remains the lane identity.

## Lane laws

1. A candidate maps to zero or one policy-v1 contribution lane.
2. Lane selection is deterministic and closed.
3. A selected lane may produce one bounded direct contribution or none.
4. A lane application is not aggregation.
5. Lane count does not affect standing.
6. Repetition creates no threshold, vote, score, confidence, or corroboration.
7. A contribution cannot exceed its existing ceiling.
8. Only context co-replayed in the same `StandingReplaySnapshot` may satisfy the
   current lane.
9. Public trace construction is not trusted resolver provenance.
10. Adding an enum variant alone grants no authority.

## Behaviour and serialization preservation

For every replay-valid snapshot, the refactor preserves the complete
`StandingResolutionV1`: governed and legacy standing, currentness, policy ID,
trace entries and order, application presence, rule, context outcome, matched
occurrence and audit fields, achieved standing, blockers and order, and
canonical bytes.

The serialized shapes of `StandingResolutionV1`, `StandingTraceEntryV1`,
`StandingPolicyApplication`, `StandingPolicyRule`, and
`DeadboltOccurrenceContextTrace` are unchanged. The existing literal v1
canonical JSON remains unchanged and is retained as an explicit
contribution-lane refactor regression.

V0 types, policy ID, resolver methods, candidate semantics, scalar delegation,
positive `Conjectured` result, and representative canonical bytes remain
unchanged.

## Direct contribution is not aggregation

The lane produces the existing direct `Some(Settled)` only when the exact
Deadbolt context is `Matched`. All other context outcomes produce no
contribution. Claim-level policy observes existence of that direct result; it
does not count applications.

Two successful candidate paths each retain a deterministic trace and
application, while the claim result remains exactly `Settled`. Trace
multiplicity is audit material, not amplification.

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

The claim excludes caller-created resolution or trace structs; independently
constructed projections; two independent replay calls; mutable stores returning
different snapshots; self-asserted authority metadata; bare evidence-kind
labels; caller-provided verified or admitted booleans; unverified foreign bundle
contents; and any rule variant added without an exact lane implementation.

### Hostile regression

A future contributor must not be able to add another achieved-standing path as
an ad hoc side branch that bypasses closed lane classification, replayed-node
revalidation, exact context resolution, bounded contribution, deterministic
tracing, or snapshot provenance.

## Tests added

- exact candidate selects exactly `DeadboltOccurrenceInclusionV1`;
- wrong evidence kind, domain, ceiling, missing edge/source, changed reason set
  or order, and unknown vocabulary select no lane;
- the classifier's `Option` result pins the zero-or-one lane law;
- exhaustive rule dispatch remains a separate step with no wildcard arm;
- two successful direct-proof paths retain two applications without
  amplification;
- v0 positive result, scalar delegation, policy ID, and literal canonical bytes
  remain frozen;
- the existing literal v1 canonical bytes remain unchanged under a clearly
  named contribution-lane preservation test.

All existing context failure, identity mismatch, missing-node, scope,
authority-confusion, non-veto, duplicate-anchor, anchor-only, replay, blocker,
and canonical-byte tests remain.

## Non-goals

No new achieved-standing rule, deterministic-verification settlement, achieved
`Supported` or `Refuted`, aggregation, threshold, counting, voting, confidence,
corroboration, independence groups, source-count behaviour, admission,
`EpistemicGate`, refutation, contradiction debt, invalidation, supersession,
currentness, policy selection, writer surface, MCP path, retrieval, Deadbolt
change, L0 change, dependency change, or broad cleanup is included.

## Files in scope

- `crates/magpie-claims/src/standing_v1.rs`
- `crates/magpie-claims/tests/deadbolt_occurrence_standing_v1.rs`
- `docs/design/deadbolt-occurrence-standing-v1.md`
- `docs/design/standing-contribution-lanes-v1.md`
- `tickets/0029-standing-contribution-lanes-v1.md`

## Validation

Run the ticket's full Rust, canonical-byte, portable-verifier, policy-surface,
and frozen-path validation suite. The frozen-path audit must report no paths.
