# ADR-0002 Tags 6–8 Implementation Plan

## Status

Proposed planning note.

Doctrine note: ticket 0019 supersedes the old HumanRatification settlement
language. Humans settle nothing epistemically; HumanRatification is
governance/judgment evidence capped below `Settled`.

## Purpose

This note plans the future ADR-0002 additive event tags:

- tag 6: `ClaimAssertedV2`
- tag 7: `EvidenceRegistered`
- tag 8: `JustificationEdgeRecorded`

It does not implement the tags. The purpose is to make the later
format-expansion PR mechanical and reviewable before touching L0 payloads,
canonical bytes, golden vectors, `docs/FORMAT.md`, the independent Python
verifier, or projection code.

The projection law comes before the new write vocabulary. ADR-0002,
`StandingView` over v0 events, and the v0 boundary tests have already landed;
this plan is the next review checkpoint before new claim-writing vocabulary
enters L0.

## Non-goals

- No Rust changes in this planning note.
- No `docs/FORMAT.md` changes.
- No canonical encoding changes.
- No golden vector changes or regeneration.
- No `tools/verify_chain.py` changes.
- No new event tags in `Payload`.
- No `StandingView` changes.
- No `EpistemicGate` implementation.
- No MCP write surfaces or other writer-facing surfaces.
- No lens ingestion.
- No scope inheritance.
- No probabilistic fusion.
- No Dung, JTMS, ATMS, or AGM machinery.
- No Deadbolt changes.

## Current foundation

The current log format is `magpie-core-v1`. Tags 0-5 are already defined and
must remain byte-for-byte unchanged:

- tag 0: `Genesis`
- tag 1: `ClaimAsserted`
- tag 2: `EvidenceRecorded`
- tag 3: `ClaimStatusChanged`
- tag 4: `Note`
- tag 5: `SegmentAnchored`

`SegmentAnchored` is occurrence/inclusion evidence, not interpretation truth.
Typed future evidence may cite an anchor, but the anchor alone must not decide
what Magpie believes about a claim.

`StandingView` now exists as a read-only derived projection over the v0 event
vocabulary. It preserves legacy v0 replay and pins the rule that anchors and
notes do not create or promote claims. Tags 6-8 must extend that projection
with explicit match arms; no wildcard match arms are allowed.

## Proposed payload schemas

All new fields are strings in v1 unless this plan explicitly says otherwise.
String fields are encoded with the existing `str` primitive: `u64` byte length
followed by the exact UTF-8 bytes. `metadata_json` is encoded as a string, not
as parsed JSON.

### Tag 6: ClaimAssertedV2

Purpose: register a scoped claim node. It does not itself settle truth.

Canonical field order:

```text
claim_id: String
statement: String
scope_ref: String
actor_class: String
content_hash: String
metadata_json: String
```

Semantic notes:

- `claim_id` is the stable claim identifier.
- `statement` is the human-readable claim text.
- `scope_ref` is opaque exact-match scope material.
- `actor_class` must be one of the ADR-0002 actor classes.
- `content_hash` is optional external content hash material. If unused, it is
  the empty string. If non-empty, the implementation PR should require 64
  lowercase hex characters.
- `metadata_json` is opaque JSON-shaped string material. L0 does not parse or
  canonicalize it as JSON.

Standing implication:

- `ClaimAssertedV2` creates a scoped claim entry if absent.
- It must not directly create `Settled`.
- The default standing should be `Conjectured` for v1. A future reviewer may
  choose stricter `Open`, but that choice must be made explicitly in the code
  PR and tests.
- Any path to `Supported`, `Settled`, or `Refuted` goes through typed evidence,
  justification edges, ceilings, and governed admission.

### Tag 7: EvidenceRegistered

Purpose: register a typed evidence node that may later connect to claims or
other evidence by justification edges.

Canonical field order:

```text
evidence_id: String
evidence_kind: String
summary: String
scope_ref: String
actor_class: String
content_hash: String
metadata_json: String
```

Semantic notes:

- `evidence_id` is the stable evidence identifier.
- `evidence_kind` must be one of the ADR-0002 evidence kinds.
- `summary` is human-readable.
- `scope_ref` is opaque exact-match scope material.
- `actor_class` identifies who or what registered the evidence.
- `content_hash` may point to external evidence content, a verifier report
  hash, or a Deadbolt witness root depending on evidence kind.
- `metadata_json` may contain method IDs, model revision IDs, calibration
  corpus hashes, source URLs, witness roots, or other structured metadata, but
  v1 canonical encoding treats it as a string.

Evidence kinds:

```text
DeterministicVerification
HumanRatification
DeadboltAnchor
ExecutionEvidence
BehavioralEvaluation
ExternalSource
ModelSelfReport
LensReadout
```

Evidence ceiling plan:

```text
DeterministicVerification -> may reach Settled for exact machine-checkable predicates.
HumanRatification -> may reach Supported for scoped human judgment or governance evidence; never Settled.
DeadboltAnchor + successful verifier context -> may reach Settled for occurrence/inclusion, Supported for interpretation.
ExecutionEvidence -> may reach Supported.
BehavioralEvaluation -> may reach Supported.
ExternalSource -> may reach Conjectured or Supported depending on policy/corroboration.
ModelSelfReport -> may reach Conjectured.
LensReadout -> may reach Conjectured.
```

These ceilings are deterministic caps, not numeric confidence.

### Tag 8: JustificationEdgeRecorded

Purpose: record a typed support, attack, lineage, invalidation, or
ratification edge between claims and evidence nodes.

Canonical field order:

```text
edge_id: String
edge_kind: String
source_id: String
target_id: String
scope_ref: String
actor_class: String
rationale: String
metadata_json: String
```

Semantic notes:

- `edge_id` is the stable edge identifier.
- `edge_kind` must be one of the ADR-0002 edge kinds.
- `source_id` identifies the source claim, evidence, or event node.
- `target_id` identifies the target claim, evidence, or event node.
- `scope_ref` is opaque exact-match scope material.
- `actor_class` identifies who or what recorded the edge.
- `rationale` is a human-readable reason.
- `metadata_json` is opaque JSON string material.

Standing implications:

- `supports` may contribute toward support subject to source standing and
  evidence ceiling.
- `derived_from` preserves lineage. It is not support by itself unless paired
  with explicit support rules.
- `contradicts` creates debt and blocks `Settled` until resolved.
- `supersedes` preserves lineage and makes old material non-current under the
  same exact scope unless future rules say otherwise.
- `invalidates` removes support contribution from the invalidated target on
  replay.
- `ratifies` can settle only when admitted under `HumanRoot` or another
  appropriate authority and explicit scope rules.

Full edge semantics may be staged. The first implementation PR may add tags
and explicit projection arms without implementing every advanced standing
transition, but tests must pin every intentionally deferred behavior.

## Closed vocabularies

### Actor classes

Allowed actor classes:

```text
HumanRoot
AgentProposer
AutomatedVerifier
DeadboltAnchorer
LensWitness
SourceImporter
```

### Evidence kinds

Allowed evidence kinds:

```text
DeterministicVerification
HumanRatification
DeadboltAnchor
ExecutionEvidence
BehavioralEvaluation
ExternalSource
ModelSelfReport
LensReadout
```

### Edge kinds

Allowed edge kinds:

```text
supports
derived_from
contradicts
supersedes
invalidates
ratifies
```

Rejected vague edge kinds:

```text
related_to
similar_to
about
probably_true
associated_with
```

The small vocabulary is intentionally restrictive to prevent relation slop.

## Validation policy

The future implementation PR should split validation into two layers:

- `Payload::validate` enforces syntactic and domain-closed facts that are part
  of the log contract.
- `EpistemicGate` enforces policy-heavy admission rules such as who is allowed
  to write a given event, whether a verifier result is acceptable, and whether
  a ratification is authorized for a scope.

Recommended `Payload::validate` rules:

- Required IDs are non-empty strings: `claim_id`, `evidence_id`, `edge_id`,
  `source_id`, and `target_id`.
- Human-facing content strings are non-empty for `statement`, `summary`, and
  `rationale`.
- `actor_class` must be one of the six closed actor classes.
- `evidence_kind` must be one of the eight closed evidence kinds.
- `edge_kind` must be one of the six closed edge kinds.
- `scope_ref` must be a non-empty string.
- `scope_ref` has no prefix, wildcard, containment, inheritance, ontology, or
  lattice semantics.
- `content_hash` may be empty. If non-empty, it must be exactly 64 lowercase
  hex characters. If a future evidence kind needs another hash algorithm, that
  algorithm name belongs in `metadata_json` or a future explicit field, not in
  an overloaded hash string.
- `metadata_json` is opaque. L0 should not parse it and should not require
  syntactically valid JSON in the first implementation.

Unknown actor classes, evidence kinds, and edge kinds should be rejected by
`Payload::validate` because they are closed v1 vocabularies. Authorization and
standing policy still belong in `EpistemicGate`.

## Scope semantics

`scope_ref` is an opaque exact-match string in v1.

- Empty `scope_ref` should be rejected for tags 6-8.
- A broad/global scope must use an explicit string such as `global` or a
  project-defined scope reference.
- There is no inheritance, containment, prefix matching, wildcard matching,
  ontology lookup, or lattice semantics.
- If scope later acquires containment or inheritance semantics, ADR-0002 must
  be amended or superseded before the fold expands.

## Canonical encoding plan

The future format PR must preserve all current encodings:

- tags 0-5 unchanged;
- tag 6 appended after tag 5;
- tag 7 appended after tag 6;
- tag 8 appended after tag 7;
- no existing field ordering changes;
- no existing magic strings or signature-domain strings change;
- no existing status encoding changes;
- new variants use explicit fixed field order;
- every new field is encoded as an existing length-prefixed `str`;
- `metadata_json` is encoded as a string, not as parsed JSON;
- golden vectors 0-5 remain byte-for-byte unchanged;
- new golden events are appended after existing coverage.

Do not introduce wildcard match arms. Every `Payload` consumer must explicitly
decide what tags 6-8 mean.

## Golden vector plan

The implementation PR should stage golden work in this order:

1. Snapshot current golden hashes for existing seqs 0-5 before making format
   changes.
2. Add one minimal valid event for each new tag:
   `ClaimAssertedV2`, `EvidenceRegistered`, and `JustificationEdgeRecorded`.
3. Assert existing seqs and hashes are unchanged.
4. Append new golden coverage after the existing fixture events.
5. Run Rust golden tests.
6. Run the independent Python verifier against the updated fixture.
7. Do not regenerate old vectors to make tests green.

Stop rule:

> If existing golden vectors for tags 0-5 change, stop. Do not accept the
> change without a separate format-break ADR.

The fixture should prove the additive shape: old bytes stay old, new bytes are
covered, and Rust/Python agree on every tag.

## Rust implementation plan

The future implementation PR should include these mechanical changes:

1. Add `ClaimAssertedV2`, `EvidenceRegistered`, and
   `JustificationEdgeRecorded` to `crates/magpie-log/src/event.rs`.
2. Extend `Payload::validate` with the validation policy above.
3. Update `crates/magpie-log/src/canonical.rs` with tags 6, 7, and 8 in fixed
   field order.
4. Update `docs/FORMAT.md` with the new tags, fields, validation notes, and
   vocabulary history.
5. Append golden vectors in `crates/magpie-log/testdata/golden-v1.jsonl`.
6. Update `crates/magpie-log/tests/golden.rs` and any golden-regeneration
   example used only to cut the additive fixture.
7. Update `tools/verify_chain.py`.
8. Update `ClaimsView` with explicit ignore or compatibility arms.
9. Update `StandingView` with explicit match arms.
10. Update the episodic projection with explicit payload mapping arms.
11. Run cargo tests and the Python verifier.

No `_ => {}` arms should be introduced in projections or canonical encoding.
Compiler exhaustiveness is part of the safety boundary.

## Python verifier plan

`tools/verify_chain.py` currently implements canonical payload handling in
`payload_bytes(payload)`, with helpers for `s`, `u8`, `u64`, `status`, and
lowercase hex validation.

The future verifier PR should:

- add closed vocabulary constants for actor classes, evidence kinds, and edge
  kinds;
- add helper validation for non-empty strings where the Rust implementation
  requires them;
- add helper validation for optional lowercase-hex `content_hash`;
- add tag 6 handling in `payload_bytes` with this order:
  `claim_id`, `statement`, `scope_ref`, `actor_class`, `content_hash`,
  `metadata_json`;
- add tag 7 handling in `payload_bytes` with this order:
  `evidence_id`, `evidence_kind`, `summary`, `scope_ref`, `actor_class`,
  `content_hash`, `metadata_json`;
- add tag 8 handling in `payload_bytes` with this order:
  `edge_id`, `edge_kind`, `source_id`, `target_id`, `scope_ref`,
  `actor_class`, `rationale`, `metadata_json`;
- keep unknown `payload.kind` values failing loudly;
- keep old tag handling for tags 0-5 unchanged;
- align every validation error class with Rust's `Payload::validate` contract
  where possible.

Verifier proof commands should include:

```powershell
python .\tools\verify_chain.py .\crates\magpie-log\testdata\golden-v1.jsonl <VERIFYING_KEY_HEX>
cargo test -p magpie-log --locked
```

The Python verifier must still accept the old tag 0-5 portion unchanged and
must verify the appended tag 6-8 fixture events.

## StandingView evolution plan

Future `StandingView` work should preserve v0 compatibility while adding typed
tables or equivalent deterministic structures for claims, evidence, and edges.

Required implications:

- v0 `ClaimAsserted`, `EvidenceRecorded`, and `ClaimStatusChanged` semantics
  remain compatible.
- `ClaimAssertedV2` creates scoped claim nodes.
- `EvidenceRegistered` creates evidence nodes or an evidence side table.
- `JustificationEdgeRecorded` creates edges, debt, lineage, invalidation, and
  ratification material as implemented by staged fold rules.
- `SegmentAnchored` remains occurrence/inclusion evidence only. Typed
  `EvidenceRegistered` may cite an anchor, but the anchor alone does not
  interpret itself.
- `LensReadout` is capped at `Conjectured`.
- `ModelSelfReport` is capped at `Conjectured`.
- `DeadboltAnchor` can settle occurrence/inclusion when paired with successful
  verifier context, but cannot settle interpretation truth.
- Contradiction, debt, supersession, invalidation, ratification, and cyclic
  support handling may be staged, but every implemented or deferred behavior
  must have explicit tests.

## Test matrix

### Format/canonical tests

- Existing golden vectors 0-5 unchanged.
- Tag 6 canonical bytes stable.
- Tag 7 canonical bytes stable.
- Tag 8 canonical bytes stable.
- Rust and Python verifier agree on all tags.
- Invalid actor classes rejected according to `Payload::validate`.
- Invalid evidence kinds rejected according to `Payload::validate`.
- Invalid edge kinds rejected according to `Payload::validate`.
- Invalid non-empty `content_hash` rejected.
- Unknown payload kinds still fail in the Python verifier.

### Projection tests

- `ClaimsView` has explicit arms for tags 6-8.
- `StandingView` has explicit arms for tags 6-8.
- Tag 6 creates a claim entry but does not settle ordinary agent claims.
- Tag 7 alone does not create a claim unless the future fold explicitly says
  it should.
- Tag 8 `supports` connects evidence to claim under ceiling.
- `LensReadout` cannot settle.
- `ModelSelfReport` cannot settle.
- `DeadboltAnchor` cannot settle interpretation truth.
- `contradicts` creates debt and blocks `Settled`.
- `invalidates` removes support contribution.
- `supersedes` preserves lineage.
- Cyclic or self-supporting edge sets do not self-promote.

### Replay tests

- Replay is deterministic.
- Dropping and rebuilding `StandingView` yields identical projection bytes.
- Legacy v0 chains still replay.

### Boundary tests

- No wildcard match arms.
- No format update without verifier update.
- No verifier update without golden coverage.
- No writer surface in the tag implementation PR.

## Rollout order

1. Land this docs-only planning PR.
2. Open a format-expansion implementation PR that updates `Payload`,
   canonical encoding, `docs/FORMAT.md`, golden vectors, verifier support, and
   explicit projection arms together.
3. In that same implementation PR, prove tags 0-5 are unchanged and tags 6-8
   are covered by Rust and Python verifier tests.
4. Add or extend `StandingView` tests for every implemented tag 6-8 behavior
   and every intentionally deferred behavior.
5. Only after the format/projection/verifier surface is green, design
   `EpistemicGate` admission rules.
6. Only after `EpistemicGate`, expose writer-facing surfaces.

## Stop conditions

Stop before merging the future implementation PR if:

- any existing tag 0-5 canonical bytes or golden hashes change;
- `docs/FORMAT.md` and canonical encoding disagree;
- Rust and Python verifier canonicalization disagree;
- a new tag lacks golden coverage;
- a projection uses `_ => {}` or otherwise avoids explicit arms;
- `SegmentAnchored` starts determining interpretation truth by itself;
- `metadata_json` parsing creates a second canonicalization surface;
- scope inheritance, probabilistic fusion, or credulous/multiple accepted
  standings are needed before ADR-0002 is amended;
- adding tags requires writer-facing surfaces in the same PR.

## Reviewer checklist

- Confirm this planning PR is docs-only.
- Confirm proposed tag fields and canonical order are explicit.
- Confirm tags 0-5 are preserved unchanged.
- Confirm validation policy separates L0 syntax from `EpistemicGate` policy.
- Confirm `scope_ref` is opaque exact-match only.
- Confirm `metadata_json` is opaque string material.
- Confirm `content_hash` policy is explicit.
- Confirm golden-vector and Python verifier work are required.
- Confirm `StandingView` evolution preserves the `SegmentAnchored` boundary.
- Confirm the test matrix covers format, verifier, projection, replay, and
  boundary behavior.
