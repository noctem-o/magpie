# Ticket 0036 — Deterministic Direct Support Standing Policy v2

## Goal

Activate exactly one already-reviewed deterministic verifier rule under the
fixed explicit policy identity `magpie-claims-standing-v2`, starting from merge
commit `8022f0244b2bf9fa01e84ebfbb915e6a12833daa`.

## Architectural law

```text
verifier receipt != standing by itself

verifier receipt + explicit policy-v2 rule
    = one bounded direct Supported contribution

one Supported contribution != aggregation

many identical or distinct successful verifier paths != Settled
```

The exact rule is:

```text
DeterministicVerification
× ExactMachineCheckable
× exact canonical predicate statement
× matching claim statement content hash
× sha256_bytes_equals_v0
× successful same-snapshot deterministic verifier context
→ Some(Status::Supported)
```

The existing `Some(Status::Settled)` cell is a maximum ceiling, not an
automatic outcome. This reviewed first policy contribution deliberately stops
at `Supported` and creates no second settlement family.

## Public boundary

Add the explicit `StandingPolicyRuleV2`, `StandingPolicyContextV2`,
`StandingPolicyApplicationV2`, `StandingTraceEntryV2`, and
`StandingResolutionV2` type family. Add only
`StandingReplaySnapshot::resolved_standing_v2` and
`resolved_standing_with_trace_v2`. V2 is explicitly selected, never an ambient,
latest, negotiated, registry-backed, or runtime-selected policy.

Policy v2 consumes only verifier context derived internally from the same
StandingReplaySnapshot.

It accepts no caller-provided receipt or trace as authority.

V1 is inherited internally by resolving v0 and v1 from the snapshot, preserving
candidate order, and privately converting every exact Deadbolt v1 application
into the v2 type family. V0 and v1 public types, methods, behavior, and literal
serialized bytes remain frozen.

## Deterministic rule application

Classification requires present edge/source IDs, exact
`DeterministicVerification`, exact `ExactMachineCheckable`, exact `Settled`
candidate ceiling, and exactly these ordered reasons:

```text
AcceptedCandidate
RequiresVerifierContext
CeilingIsCandidateOnly
```

Any boundary change selects no rule. For a selected candidate, v2 calls only
`resolve_deterministic_verifier_context_v0(claim_id, source_id, edge_id)`.
`Matched` contributes `Supported`; every exact failure contributes nothing.
If candidate tracing and replayed-node revalidation disagree and the method
returns `None`, the application records `CandidateRevalidationFailed` and
contributes nothing. V2 does not parse metadata, decode witnesses, rerun SHA-256,
or accept independently constructed material.

## Precedence and non-amplification

Inherited `Settled` remains `Settled`; inherited `Refuted` remains `Refuted`;
otherwise at least one deterministic supported application yields `Supported`
for an existing inherited claim; otherwise inherited v1 standing is retained.
Only Boolean existence is used. Counts, scores, weights, quorum, thresholds,
source counts, and independence groups do not participate.

This policy establishes direct support, not aggregation.

Multiple successful applications retain multiple audit traces but cannot
produce Settled or any independence claim.

A failed path does not veto a successful direct path. Its exact failure remains
in its application context, and unresolved generic blockers remain.

## Blockers and serialization

`StandingTraceReason` remains the only blocker vocabulary. `AcceptedCandidate`
is never a blocker. `Supported` or `Settled` satisfies only that trace entry's
generic verifier-context and candidate-ceiling limitations. A generic reason is
cleared claim-wide only when all corresponding entries succeed. Other v0
blockers remain, deduplicated in deterministic enum order. Blocker derivation
never invokes the checker again.

Freeze rule names `deadbolt_occurrence_inclusion_v1` and
`sha256_bytes_equals_direct_support_v0`; context kinds
`deadbolt_occurrence_v1`, `deterministic_verifier_v0`, and
`candidate_revalidation_failed`; and the required resolution, trace, and
application field order. V2 types serialize but do not deserialize. Canonical
fixtures cover success, exact failure, inherited Deadbolt settlement, mixed
paths, candidate revalidation fallback, blocker order, and replay equality.
These are deterministic derived audit bytes, not L0 encoding or a network
protocol.

## Tour and package inventory

Extend the deterministic public tour with the exact `abc` witness and canonical
SHA-256 statement using only one `ClaimAssertedV2`, one `EvidenceRegistered`,
and one `JustificationEdgeRecorded`. Preserve all existing Deadbolt and hostile
facts, then prove v0/v1 remain `Conjectured`, v2 reaches only `Supported`, and
all snapshot and resolution bytes replay identically.

Add `src/standing_v2.rs` and
`tests/deterministic_support_standing_v2.rs` to the `magpie-claims` package
inventory. Do not claim independent registry packageability.

## Files in scope

- `README.md`
- `crates/magpie-claims/examples/tour.rs`
- `crates/magpie-claims/src/lib.rs`
- `crates/magpie-claims/src/standing_v2.rs`
- `crates/magpie-claims/tests/deterministic_support_standing_v2.rs`
- `docs/design/replayable-deterministic-verifier-admission-v0.md`
- `docs/design/standing-aggregation-independence-groups.md`
- `tickets/0036-deterministic-support-standing-v2.md`
- `tools/check_release_metadata.py`

## Frozen surfaces and non-goals

Cargo manifests, `Cargo.lock`, v0, v1, L0, replay snapshot construction, both
existing context checkers, existing literal tests, payloads, projections,
fixtures, release documents, and format surfaces are frozen. No generic
verifier, other predicate, deterministic settlement, aggregation, evidence
counting, independence, provenance grouping, quorum, confidence, reputation,
refutation, contradiction debt, invalidation, supersession, currentness,
`EpistemicGate`, writer, receipt persistence/ingestion, filesystem, artifact,
shell, network, callback, plugin, registry, or model/lens behavior is in scope.

## Hostile tests and validation

Cover exact success, representative verifier failures unchanged, authority
confusion, every candidate-classification boundary, candidate revalidation
fallback, Deadbolt inheritance and precedence, `Refuted` preservation,
non-amplification, mixed success/failure, exact blocker order, literal bytes,
unchanged v0/v1/snapshot bytes, and two verified replays. Run focused v2 tests,
doctests, the tour, full formatting/lint/workspace tests, metadata/package
inventories, both independent chain verifications, frozen-surface audits, and
release-boundary audits.

## Next slice

Genuine aggregation with conservative provenance-derived independence is next.
It must preserve this direct-support boundary and may not reinterpret trace
multiplicity as corroboration.
