# Ticket 0028: Deadbolt occurrence/inclusion achieved-standing policy v1

## Goal

Implement the first narrowly achieved governed-standing rule: settle an exact
structured Deadbolt occurrence/inclusion proposition only when its accepted v0
candidate, claim predicate, evidence reference, and co-replayed anchor identity
all match.

## Exact policy law

```text
supports edge
× exact evidence/edge/target scope
× DeadboltAnchor
× Occurrence/Inclusion
× support_ceiling == Some(Settled)
× support_context_requirement == DeadboltVerifierContext
× exact segment_anchored_v1 claim predicate
× exact segment_anchored_v1 evidence reference
× exact five-field identity in the same StandingReplaySnapshot anchor index
× resolver outcome Matched
→ achieved contribution Settled
→ governed standing Settled
```

Nothing else achieves standing in this ticket.

## V0 frozen boundary

Keep `MAGPIE_CLAIMS_POLICY_ID`, both `StandingView` resolver methods,
`StandingResolution`, `StandingTraceEntry`, `StandingTraceReason`, and their
representative canonical bytes unchanged. V0 continues to return
`Conjectured` for the positive fixture.

## V1 public API and identity

Policy ID:

```text
magpie-claims-standing-v1
```

Public types:

- `StandingResolutionV1`;
- `StandingTraceEntryV1`;
- `StandingPolicyApplication`;
- `StandingPolicyRule` with exactly `DeadboltOccurrenceInclusionV1`;
- `DeadboltOccurrenceContextTrace`.

Snapshot-only methods:

- `StandingReplaySnapshot::resolved_standing_v1`;
- `StandingReplaySnapshot::resolved_standing_with_trace_v1`.

No public resolver accepts separate projections and no ambient latest-policy
surface exists.

## Layered trace design

Each v1 trace entry preserves its complete v0 candidate and optionally attaches
one rule application. Candidate evaluation, context satisfaction, and achieved
standing remain distinct layers. Eligible entries must exactly match the
deterministic v0 candidate fields and reason order before replayed nodes are
re-fetched and checked.

## Context outcome vocabulary

The serializable closed outcomes are `Matched`, `WrongEvidenceKind`,
`WrongClaimDomain`, `MissingClaimPredicate`, `MalformedClaimPredicate`,
`MissingEvidenceReference`, `MalformedEvidenceReference`,
`PredicateReferenceMismatch`, and `AnchorNotFound`.

`Matched` achieves exactly `Settled`; all other outcomes achieve nothing.

## Blocker semantics

Recompute blockers from layered trace in deterministic set order.
`AcceptedCandidate` is not a blocker. A matched path satisfies its
`RequiresVerifierContext` and `CeilingIsCandidateOnly` limitations. Unmatched
paths retain them, and unrelated blockers remain visible without vetoing an
independently matched direct proof.

## Acceptance criteria

- Positive signed replay proves v0 `Conjectured` and explicit v1 `Settled`.
- Trace retains edge, source, rule, exact identity, sequence, event hash, and
  achieved contribution.
- Missing, malformed, mismatched, and absent-anchor contexts remain exact.
- Wrong policy cells and malformed v0 candidates receive no v1 application.
- Actor classes, prose, summaries, hashes, booleans, and bare labels cannot
  settle.
- One unresolved path cannot veto one matched direct path.
- Duplicate anchors use the earliest occurrence and do not amplify.
- Rebuilt snapshots and v1 resolution bytes are identical.
- Representative v1 canonical bytes are pinned without changing v0 bytes.
- Anchor-only replay remains structurally empty with no application.

## Positive and hostile tests

Use one fully signed log with `Genesis`, `ClaimAssertedV2`,
`EvidenceRegistered`, `JustificationEdgeRecorded`, and `SegmentAnchored`.
Exercise every context failure, all five identity-field disagreements, policy
cell exclusions, missing replayed nodes, scope mismatch, unknown vocabulary,
authority confusion, non-veto behavior, duplicate anchors, regeneration, and
the portable anchor-only fixture.

## Canonical-byte pin

Pin one exact `StandingResolutionV1::canonical_bytes` value. The bytes are
derived test material, not L0 encoding or a persistent protocol.

## Non-goals

No aggregation, corroboration, independence groups, deterministic-verification
settlement, interpretation support, operational-observation support, achieved
`Supported` or `Refuted`, refutation, contradiction debt, invalidation,
supersession, human admission, `EpistemicGate`, writer/MCP surfaces, foreign
bundle verification, policy negotiation, L0/fixture/golden changes, or
Deadbolt/Cogitator changes.

## Validation

```text
git diff --check
cargo fmt --all -- --check
cargo test -p magpie-claims --locked
cargo test -p magpie-log --locked
cargo test --workspace --locked
cargo clippy --locked -p magpie-claims --all-targets -- -D warnings
cargo test -p magpie-claims deadbolt_occurrence_standing_v1 --locked
cargo test -p magpie-claims deadbolt_occurrence --locked
cargo test -p magpie-claims standing_resolution --locked
cargo test -p magpie-claims standing_replay_snapshot --locked
cargo test -p magpie-claims --test deadbolt_occurrence_standing_v1 --locked
cargo test -p magpie-claims --test standing_resolution --locked
python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737
```

## Follow-up

Deterministic achieved-standing contribution lanes without amplification.
