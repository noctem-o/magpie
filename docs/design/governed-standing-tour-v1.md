# Governed-Standing Tour v1

## Purpose

The primary `magpie-claims` example is an executable release contract for the
current governed-standing thesis:

```text
signed append-only history
→ complete verification
→ one co-replayed standing/context snapshot
→ v0 candidate explanation
→ explicit v1 direct-proof contribution lane
→ exact achieved standing
→ hostile negative controls
→ drop all derived state
→ replay from the log alone
→ byte-identical snapshots and resolutions
```

It adds no standing rule or authority. It demonstrates existing public APIs and
fails by assertion if their current architectural boundaries drift.

## Why the legacy tour was stale

The previous tour used `ClaimAsserted`, `EvidenceRecorded`,
`ClaimStatusChanged`, and `ClaimsView`. It showed a writer manually changing a
claim from `Conjectured` to `Settled`, then regenerating that compatibility
projection. That remains supported legacy vocabulary, but it no longer
represents Magpie's strongest public claim: stored information and a writer's
status declaration are not governed belief.

The front-door example now uses typed claims, typed evidence, justification
edges, one exact anchor, `replay_standing_context`, the v0 explanation surface,
and the explicit snapshot-only v1 resolver. It never uses a legacy status
mutation to create governed standing.

## Deterministic signed history

The fixed demonstration key and injected counter clock create one repeatable
11-event history:

1. genesis;
2. a typed `Occurrence/Inclusion` claim with a strict
   `segment_anchored_v1` predicate;
3. matching `DeadboltAnchor` evidence and `deadbolt_anchor_ref`;
4. an exact-scope `supports` edge;
5. a second valid occurrence candidate whose evidence reference changes only
   `run_id`;
6. its matching-scope evidence and support edge;
7. an `Interpretation` claim over the same occurrence identity;
8. its matching-scope anchor evidence and support edge;
9. one `SegmentAnchored` event containing the exact five-field identity.

The writer goes out of scope before any read. `LogReader::verify_chain` checks
the full history, and `replay_standing_context` independently performs the one
verified replay that constructs the trusted composite snapshot. The example
does not use unverified `LogReader::events()`.

## Positive exact occurrence proposition

The positive proposition is narrowly:

> The named five-field `SegmentAnchored` identity occurs in this signed Magpie
> history.

V0 preserves the candidate-only result `Conjectured`. V1 selects
`DeadboltOccurrenceInclusionV1`, resolves `Matched`, retains identity, sequence,
event hash, and provenance, contributes exactly `Settled`, and produces
governed `Settled`. `StandingView::resolved_standing` remains the v0 scalar and
therefore still returns `Conjectured`.

## Hostile mismatch control

The mismatch path remains a valid v0 `DeadboltAnchor × Occurrence/Inclusion`
candidate. Its claim predicate names the anchored identity, while its evidence
reference changes exactly one `run_id`. V1 attempts the same closed lane but
records `PredicateReferenceMismatch`, contributes nothing, leaves the claim
`Conjectured`, and retains verifier-context and candidate-only blockers.

The exact context enum is asserted rather than collapsed into a boolean.

## Interpretation control

The interpretation path cites the same anchor material under the actual closed
`Interpretation` domain. Current v0 policy exposes a `Supported` candidate
ceiling with verifier-context requirements, while governed standing remains
`Conjectured`. The occurrence-lane classifier rejects the domain, so v1 attaches
no application and cannot settle the interpretation.

Occurrence is not interpretation: the anchor establishes neither that the
action was wise nor that a statement about its meaning is true.

## Drop-and-replay proof

The first evaluation captures the existing canonical-byte surfaces for:

- the complete `StandingReplaySnapshot`;
- positive v0 and v1 resolutions;
- mismatch v0 and v1 resolutions;
- interpretation v0 and v1 resolutions.

It also captures typed scalar and matched-occurrence audit facts. The snapshot
and every full resolution then leave scope. A second
`replay_standing_context` call reconstructs the snapshot from signed history
alone and recomputes every resolution. The tour asserts identical typed facts,
identical matched audit material, byte-identical snapshot bytes, and
byte-identical bytes for all six resolutions.

No hand-built serialization wrapper replaces those component comparisons.

## Executable CI contract

CI runs the exact public command after the Rust test suite:

```text
cargo run --locked --example tour -p magpie-claims
```

The process succeeds only if all embedded verification, policy, hostile-control,
and replay assertions pass. The independent frozen-chain verifier remains a
separate CI step because L0 format verification and the governed-standing tour
prove different things.

The complete stdout transcript is not pinned. Its concise facts and ordering
are deterministic and validated by running the example twice, but semantic and
canonical-byte assertions are the stable release contract; pinning presentation
text would add noise without strengthening the trust boundary.

## Trusted-construction claim

### What this construction proves

The example's `StandingView` and `DeadboltAnchorIndex` were co-derived from one
completely verified event vector, and the explicit policy-v1 resolver settled
only the exact occurrence proposition whose typed claim predicate, typed
evidence reference, and replayed anchor identity matched.

### What it does not prove

- that the example signing key is production-secure;
- that the caller selected a socially correct real-world trust root;
- that foreign Deadbolt bundle contents were verified by this example;
- that the anchored action was safe, wise, desirable, or successful;
- that an interpretation of the occurrence is true;
- that Magpie is a general truth oracle;
- that support aggregation, admission, refutation, contradiction debt,
  invalidation, or supersession exists;
- that ordinary writer or MCP surfaces exist;
- that the deterministic example key should be reused outside tests and
  demonstrations.

### Sole legitimate origin path

One successful `LogReader::replay` through `replay_standing_context`, followed
by the explicit snapshot-only v0 and v1 resolvers.

### Manual or adversarial bypasses

This claim excludes manually constructed public trace or resolution structs;
independently constructed standing and anchor projections; two separate replay
calls treated as shared provenance; unverified event reads; actor-class or
evidence-kind labels; prose, content hashes, or `verified` booleans; treating
the anchor as foreign-bundle verification; and treating the positive occurrence
result as interpretation truth.

### Hostile regression

The example and CI fail if v0 promotes the positive fixture beyond
`Conjectured`; v1 settles the mismatch or interpretation control; the positive
path no longer settles through exact matched context; replay stops regenerating
byte-identical snapshots or resolutions; the tour returns to manual legacy
status mutation; or the primary README command stops executing the governed
standing thesis.

## Non-goals

This demonstration adds no public API, standing behaviour, policy selector,
proof family, achieved `Supported` or `Refuted`, aggregation, admission,
refutation, contradiction debt, invalidation, supersession, writer surface,
foreign bundle verification, dependency, fixture framework, CLI, or L0 change.
