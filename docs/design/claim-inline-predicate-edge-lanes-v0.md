# Claim-Inline Predicate Edge Lanes v0

## Status

Contract ratified by Ticket 0062 and landed through merged PR #77. Ticket 0063
implements its standing-inert runtime in the current candidate patch.

```text
predicate:
sha256_claim_inline_bytes_equals_v0

claim domain:
ExactMachineCheckable

evidence kind:
DeterministicVerification

eligible edge kinds:
supports
contradicts

outcome:
SupportEligible | RefutationEligible | Ineligible | ResolutionFailed

standing:
unchanged

policy v4:
not ratified
```

The exact change classification ratified by the historical Ticket 0062
documentation-only contract, before Ticket 0063 implementation, was:

```text
correctness:
one closed same-snapshot composition contract for the claim-owned inline
SHA-256 predicate, exact attestation binding, and one replayed edge

authority:
documentation only; no runtime authority path changes

runtime:
absent; implementation is a later separately reviewed ticket

predicate:
unchanged; sha256_claim_inline_bytes_equals_v0 remains neutral

binding:
unchanged; Ticket 0061 Bound remains routing-only

edges:
one future exact edge is interpreted only by the new closed lane composer

polarity:
lane classification only; no achieved standing

policy:
no standing policy v4 ratified

standing:
unchanged

support:
no new governed support and no change to policy v2

refutation:
no governed Refuted result

aggregation:
absent

contradiction debt:
absent

tests and fixtures:
unchanged

canonical L0 bytes:
unchanged

derived audit bytes:
one future profile and documentation vectors ratified

dependencies:
unchanged
```

## Purpose

This contract answers one question only:

```text
Given one exact replay snapshot, one exact requested claim, one exact
requested evidence node, and one exact requested edge, does the internally
derived predicate relation align with that replayed edge strongly enough to
create one standing-inert, ceiling-bounded support or refutation lane input?
```

It does not answer:

- whether the claim is true;
- whether achieved standing changes;
- whether a contribution is admitted by a standing policy;
- whether repeated lanes amplify;
- whether conflicts create debt;
- whether an old result is current;
- whether a claim is invalidated or superseded;
- whether any policy should choose `Refuted`; or
- whether a writer may emit new authority.

The result is a candidate lane input only. It is never achieved standing,
admission, aggregation, contradiction debt, invalidation, supersession, or
currentness.

## Inherited laws

This contract composes, without amending, these exact layers:

1. **Ticket 0057 claim ownership.** The replayed
   `ExactMachineCheckable` claim owns one immutable inline byte subject, its
   expected SHA-256 operand, its compiled predicate identity, and its exact
   scope. The canonical statement carries the complete descriptor. Binding is
   exact three-way statement equality plus a separate statement-content-hash
   recomputation. Evidence and edges cannot supply substitute subject bytes.
2. **Tickets 0058 and 0059 predicate relation.**
   `sha256_claim_inline_bytes_equals_v0` evaluates only the exact decoded
   claim-owned bytes and terminates as `DigestEqual`, `DigestUnequal`, or
   `ResolutionFailed`. Both terminal relations are neutral and standing-inert.
   Every prerequisite failure remains a failure, never inequality or falsity.
3. **Tickets 0060 and 0061 attestation binding.** Exact
   `DeterministicVerification` evidence may bind through the four-field
   `magpie-inline-predicate-attestation-v0` route. `Bound` is routing-only,
   computes no relation, reads no edge, and creates no polarity or standing.
   Its exact 28-stage failure order nests the exact 33-stage claim-resolution
   failure order.
4. **Exact replayed edge substrate.** `StandingView` retains
   `edge_kind`, `source_id`, `target_id`, `scope_ref`, `actor_class`,
   `rationale`, and `metadata_json` for each first-write-wins edge ID. The
   future lane resolver selects exactly one edge by raw requested ID through
   `StandingReplaySnapshot::standing().justification_edge(edge_id)`.
5. **Ceiling law.** The compiled cells remain:

   ```text
   support_ceiling(
       DeterministicVerification,
       ExactMachineCheckable,
   ) == Some(Settled)

   refutation_ceiling(
       DeterministicVerification,
       ExactMachineCheckable,
   ) == Some(Refuted)
   ```

   A ceiling is the maximum candidate contribution. It is not achieved
   standing and does not activate policy.
6. **Policy-v2 inertia.** The existing v2 rule remains
   `Sha256BytesEqualsDirectSupportV0` over the historical
   `sha256_bytes_equals_v0` evidence-local verifier context. That rule reaches
   governed `Supported`, never `Settled`, only from historical `Matched`.
   This contract neither replaces nor supplies that rule.
7. **Support-audit separation.** `SupportContributionAuditV0` classifies
   internally admitted `ExternalSource × ExternalReport` provenance/origin
   graph contributions for future corroboration aggregation. This contract
   instead classifies one exact claim-owned predicate, one attestation-bound
   deterministic evidence node, and one exact replayed edge.

No inherited outcome, receipt, serialized audit, trace, candidate ceiling, or
caller-visible classification becomes a new authority input.

## Terminology and closed scope

The only selected family is:

```text
predicate:
sha256_claim_inline_bytes_equals_v0

claim domain:
ExactMachineCheckable

evidence kind:
DeterministicVerification

edge kinds:
supports | contradicts
```

The future non-authority classification vocabularies are exactly:

```rust
pub enum ClaimInlinePredicateEdgeLaneOutcomeKindV0 {
    SupportEligible,
    RefutationEligible,
    Ineligible,
    ResolutionFailed,
}

pub enum ClaimInlinePredicateEdgeLaneKindV0 {
    Support,
    Refutation,
}

pub enum ClaimInlinePredicateEdgeLaneRelationV0 {
    DigestEqual,
    DigestUnequal,
}

pub enum ClaimInlinePredicateEdgeLaneEdgeKindV0 {
    Supports,
    Contradicts,
}

pub enum ClaimInlinePredicateEdgeLaneIneligibleReasonV0 {
    DigestUnequalDoesNotSupport,
    DigestEqualDoesNotRefute,
}
```

The future authority-bearing opaque types are:

```text
ClaimInlinePredicateEdgeLaneOutcomeV0
ClaimInlinePredicateEdgeLaneReceiptV0
```

The closed future failure vocabulary is exactly:

```rust
pub enum ClaimInlinePredicateEdgeLaneFailureV0 {
    AttestationBindingFailed(
        InlinePredicateAttestationBindingFailureV0,
    ),
    MissingEdge,
    UnsupportedEdgeKind,
    EdgeSourceBindingMismatch,
    EdgeTargetBindingMismatch,
    EdgeScopeBindingMismatch,
    SupportCeilingInvariantMismatch,
    RefutationCeilingInvariantMismatch,
}
```

These enums are inspection vocabulary, not authority tokens. No enum value can
be converted into an outcome or accepted by a resolver, lane, policy, or
composer as authority.

There is no second predicate family, claim domain, evidence kind, edge kind,
lane, threshold, generic polarity value, conflict result, policy result,
registry, alias, negotiation, or implicit latest version.

## Central same-snapshot law

```text
one StandingReplaySnapshot
+ exact requested claim_id
+ exact requested evidence_id
+ exact requested edge_id
+ one internally shared claim-inline resolution
+ exact same-snapshot attestation binding
+ one exact replayed edge
+ exact relation/edge matrix cell
+ exact compiled candidate-ceiling invariant
→ SupportEligible
  | RefutationEligible
  | Ineligible
  | ResolutionFailed
```

Every input is internally re-fetched or derived from the one selected
snapshot. No caller-created predicate outcome, attestation outcome, receipt,
edge, parsed metadata, relation, lane, policy result, Boolean, snapshot,
serialized audit bytes, or callback is accepted as authority.

## Exact future public API

The only future authority path is:

```rust
impl StandingReplaySnapshot {
    pub fn resolve_claim_inline_predicate_edge_lane_v0(
        &self,
        claim_id: &str,
        evidence_id: &str,
        edge_id: &str,
    ) -> ClaimInlinePredicateEdgeLaneOutcomeV0;
}
```

There is:

- no free function;
- no method on `StandingView`;
- no method on `OriginAdmissionReplayContextV0`;
- no public raw-material composer;
- no generic predicate registry;
- no caller-selected policy or lane; and
- no overload taking outcomes, receipts, claims, evidence objects, edge
  objects, relations, closures, callbacks, or another snapshot.

Requested IDs are opaque exact query strings. The method adds no namespace
prefix, normalization, case folding, trimming, alias, wildcard,
prefix/suffix matching, or inferred identity.

## Future authority path and derivation order

One future invocation must perform exactly:

1. Preserve the exact requested `claim_id`, `evidence_id`, and `edge_id`.
2. Invoke one private same-snapshot claim-inline resolution path.
3. Validate the exact Ticket 0061 attestation binding from that same
   snapshot.
4. Preserve any Ticket 0061 failure as
   `AttestationBindingFailed(exact nested failure)`.
5. Re-fetch the exact requested edge from the same snapshot through
   `StandingReplaySnapshot::standing().justification_edge(edge_id)`.
6. Require `edge.edge_kind` to be exactly `supports` or `contradicts`.
7. Require `edge.source_id` to equal the exact requested `evidence_id`.
8. Require `edge.target_id` to equal the exact requested `claim_id`.
9. Require `edge.scope_ref` to equal the exact scope already bound among the
   claim, evidence, and attestation.
10. Compute the claim-owned SHA-256 relation exactly once from the privately
    resolved claim material.
11. Classify the exact relation/edge matrix cell.
12. For an otherwise eligible support cell only, require:

    ```text
    support_ceiling(
        DeterministicVerification,
        ExactMachineCheckable,
    ) == Some(Settled)
    ```

13. For an otherwise eligible refutation cell only, require:

    ```text
    refutation_ceiling(
        DeterministicVerification,
        ExactMachineCheckable,
    ) == Some(Refuted)
    ```

14. Privately construct one opaque standing-inert outcome.

The implementation may refactor private shared primitives so the Ticket 0059
evaluator, Ticket 0061 binding resolver, and this composer retain one claim
parser and exact inherited failure laws. It must not:

- call the public Ticket 0059 evaluator and accept its outcome as an
  authority input;
- call the public Ticket 0061 resolver and accept its outcome or receipt
  through an authority-bearing public composer;
- duplicate the claim parser;
- accept a caller-provided relation or binding result;
- evaluate against two snapshots;
- hash evidence-supplied substitute bytes; or
- treat audit serialization as replay authority.

The public authority path remains the single three-ID snapshot method.

## Closed relation × edge matrix

| Predicate relation | Edge kind | Result |
| --- | --- | --- |
| `DigestEqual` | `supports` | `SupportEligible` |
| `DigestUnequal` | `supports` | `Ineligible: DigestUnequalDoesNotSupport` |
| `DigestEqual` | `contradicts` | `Ineligible: DigestEqualDoesNotRefute` |
| `DigestUnequal` | `contradicts` | `RefutationEligible` |

The distinctions are normative:

```text
supports + DigestUnequal
!= refutation

contradicts + DigestEqual
!= support

DigestUnequal without an exact contradicts edge
!= negative authority

a contradicts edge without DigestUnequal
!= negative authority

DigestEqual without an exact supports edge
!= positive lane authority

a supports edge without DigestEqual
!= positive lane authority
```

`Ineligible` is a successful, standing-inert classification of a valid but
polarity-misaligned pair. It is not malformed and is not `ResolutionFailed`.

`ResolutionFailed` is not support, contradiction, falsity, refutation, or
debt.

## Exact edge law

The requested edge is internally re-fetched only from:

```rust
StandingReplaySnapshot::standing().justification_edge(edge_id)
```

Bindings use raw exact replayed IDs:

```text
edge.source_id == requested evidence_id
edge.target_id == requested claim_id
edge.scope_ref == exact bound scope
```

This lane introduces no `claim:`, `evidence:`, or `edge:` namespace prefixes,
normalization, case folding, trimming, prefix or suffix matching, wildcard
semantics, scope inheritance, ontology semantics, caller-supplied edge data,
edge enumeration, or ambient “best edge” selection.

The only eligible exact replay values are:

```text
supports
contradicts
```

An unknown or otherwise unsupported edge kind is
`UnsupportedEdgeKind`, never `Ineligible` and never negative evidence.

The following edge fields are ignored for authority:

```text
actor_class
rationale
metadata_json
```

They do not enter the receipt or canonical bytes. Changing only one of them
cannot create, block, reverse, strengthen, or weaken a lane.

## Failure vocabulary and exact first-failure order

The first-failure order is:

1. `AttestationBindingFailed(exact nested Ticket 0061 failure)`
2. `MissingEdge`
3. `UnsupportedEdgeKind`
4. `EdgeSourceBindingMismatch`
5. `EdgeTargetBindingMismatch`
6. `EdgeScopeBindingMismatch`
7. `SupportCeilingInvariantMismatch`
8. `RefutationCeilingInvariantMismatch`

Relation/edge mismatch outcomes are not failures. They occur only after every
structural and binding check passes and they never consult a ceiling.

Required precedence:

- malformed claim plus missing edge →
  `AttestationBindingFailed` with the exact nested claim failure;
- missing evidence plus missing edge →
  `AttestationBindingFailed(MissingEvidence)`;
- wrong evidence kind plus missing edge →
  `AttestationBindingFailed(WrongEvidenceKind)`;
- malformed attestation plus missing edge →
  `AttestationBindingFailed(exact metadata failure)`;
- missing edge defeats every edge-field issue;
- unsupported edge kind defeats source, target, and scope mismatch;
- source mismatch defeats target and scope mismatch;
- target mismatch defeats scope mismatch;
- scope mismatch occurs before matrix classification;
- a ceiling failure is reachable only for its corresponding otherwise
  eligible matrix cell; and
- an ineligible cell does not consult or fail a ceiling invariant.

Nested Ticket 0061 failures are never flattened or reinterpreted. In
particular:

```text
AttestationBindingFailed(
    ClaimPredicateResolutionFailed(
        ClaimInlineSubjectResolutionFailureV0
    )
)
```

retains both nested levels.

## Candidate ceiling invariants

An eligible support receipt carries:

```text
lane:
Support

predicate relation:
DigestEqual

edge kind:
Supports

candidate ceiling:
Settled
```

An eligible refutation receipt carries:

```text
lane:
Refutation

predicate relation:
DigestUnequal

edge kind:
Contradicts

candidate ceiling:
Refuted
```

The distinctions are normative:

```text
candidate ceiling Settled
!= achieved Settled

candidate ceiling Refuted
!= achieved Refuted

SupportEligible
!= governed Supported

SupportEligible
!= governed Settled

RefutationEligible
!= governed Refuted
```

The contract does not decide how any later policy maps a candidate ceiling to
achieved standing. It changes no policy matrix.

## Opaque outcome and receipt

Outcome inspection is exactly:

```rust
pub fn kind(
    &self,
) -> ClaimInlinePredicateEdgeLaneOutcomeKindV0;

pub fn requested_claim_id(&self) -> &str;

pub fn requested_evidence_id(&self) -> &str;

pub fn requested_edge_id(&self) -> &str;

pub fn receipt(
    &self,
) -> Option<&ClaimInlinePredicateEdgeLaneReceiptV0>;

pub fn ineligible_reason(
    &self,
) -> Option<ClaimInlinePredicateEdgeLaneIneligibleReasonV0>;

pub fn failure(
    &self,
) -> Option<&ClaimInlinePredicateEdgeLaneFailureV0>;

pub fn canonical_bytes(&self) -> Vec<u8>;
```

Receipt inspection is exactly:

```rust
pub fn schema(&self) -> &str;
pub fn lane(&self) -> ClaimInlinePredicateEdgeLaneKindV0;
pub fn predicate_id(&self) -> &str;
pub fn claim_id(&self) -> &str;
pub fn evidence_id(&self) -> &str;
pub fn edge_id(&self) -> &str;
pub fn scope_ref(&self) -> &str;
pub fn predicate_relation(
    &self,
) -> ClaimInlinePredicateEdgeLaneRelationV0;
pub fn edge_kind(
    &self,
) -> ClaimInlinePredicateEdgeLaneEdgeKindV0;
pub fn candidate_ceiling(&self) -> Status;
```

The eligible receipt contains exactly these ten fields in this order:

```text
schema
lane
predicate_id
claim_id
evidence_id
edge_id
scope_ref
predicate_relation
edge_kind
candidate_ceiling
```

The schema is exactly:

```text
magpie-claim-inline-predicate-edge-lane-v0
```

For `SupportEligible` and `RefutationEligible`, requested IDs borrow the
receipt's sole ID strings. No duplicate successful query IDs exist outside
the receipt.

For `Ineligible` and `ResolutionFailed`, the private outcome retains:

- the exact requested claim ID;
- the exact requested evidence ID;
- the exact requested edge ID; and
- exactly one ineligible reason or failure.

The retained query-ID strings are attribution only and, by themselves, do not
prove that any requested node existed.

The outcome and receipt have:

- private representation and construction;
- no public fields;
- no public constructor;
- no struct- or tuple-literal construction;
- no `Deserialize`;
- no `Default`;
- no `From` or `TryFrom`;
- no mutation;
- no consuming deconstruction;
- no `Debug`;
- no `Display`;
- no `Error`;
- no equality, ordering, or hashing traits; and
- no authority reinjection.

`Clone` and one-way `Serialize` may exist for audit only. The fieldless
classification enums are non-authority and may derive ordinary inspection
traits. No resolver, policy, lane, or composer accepts the outcome, receipt,
clone, classification, failure, or serialized bytes as authority.

## Canonical derived audit JSON

The exact future profile is:

```text
magpie-claim-inline-predicate-edge-lane-outcome-json-v0
```

The encoder uses direct typed `serde_json::to_vec` over private fixed-field
wire views. It does not use `serde_json::Value`, maps, generic canonical JSON,
JCS/RFC 8785, Magpie L0, or a deserializable authority representation.

Outer key order is:

```text
outcome
details
```

Eligible spellings and shapes are:

```text
support_eligible
refutation_eligible
```

```text
{"outcome":"support_eligible","details":{"receipt":{RECEIPT}}}
{"outcome":"refutation_eligible","details":{"receipt":{RECEIPT}}}
```

Receipt string spellings are:

```text
lane:
support | refutation

predicate_relation:
digest_equal | digest_unequal

edge_kind:
supports | contradicts

candidate_ceiling:
settled | refuted
```

An ineligible outcome has exact detail order:

```text
claim_id
evidence_id
edge_id
reason
```

and one exact reason:

```text
digest_unequal_does_not_support
digest_equal_does_not_refute
```

Its only two shapes are:

```text
{"outcome":"ineligible","details":{"claim_id":"...","evidence_id":"...","edge_id":"...","reason":"digest_unequal_does_not_support"}}
{"outcome":"ineligible","details":{"claim_id":"...","evidence_id":"...","edge_id":"...","reason":"digest_equal_does_not_refute"}}
```

An ordinary resolution failure has exact detail order:

```text
claim_id
evidence_id
edge_id
reason
```

The ordinary top-level failure spellings are:

```text
missing_edge
unsupported_edge_kind
edge_source_binding_mismatch
edge_target_binding_mismatch
edge_scope_binding_mismatch
support_ceiling_invariant_mismatch
refutation_ceiling_invariant_mismatch
```

Every failure uses the exact outer outcome spelling `resolution_failed`.
An ordinary failure has shape:

```text
{"outcome":"resolution_failed","details":{"claim_id":"...","evidence_id":"...","edge_id":"...","reason":"..."}}
```

An attestation-binding failure has exact detail order:

```text
claim_id
evidence_id
edge_id
reason
binding_reason
```

where:

```text
reason:
attestation_binding_failed
```

Its shape is:

```text
{"outcome":"resolution_failed","details":{"claim_id":"...","evidence_id":"...","edge_id":"...","reason":"attestation_binding_failed","binding_reason":"..."}}
```

`binding_reason` preserves the exact Ticket 0061 snake-case spelling:

```text
claim_predicate_resolution_failed
missing_evidence
wrong_evidence_kind
malformed_attestation_metadata
duplicate_attestation_metadata_key
unknown_attestation_metadata_key
missing_inline_predicate_attestation
wrong_type_inline_predicate_attestation
duplicate_attestation_field
unknown_attestation_field
missing_schema
wrong_type_schema
unknown_schema
missing_predicate_id
wrong_type_predicate_id
empty_predicate_id
predicate_id_too_long
invalid_predicate_id
missing_subject_claim_id
wrong_type_subject_claim_id
empty_subject_claim_id
missing_scope_ref
wrong_type_scope_ref
empty_scope_ref
predicate_binding_mismatch
claim_binding_mismatch
evidence_scope_binding_mismatch
claim_scope_binding_mismatch
```

When `binding_reason` is `claim_predicate_resolution_failed`, append:

```text
claim_reason
```

after `binding_reason`. Its shape is:

```text
{"outcome":"resolution_failed","details":{"claim_id":"...","evidence_id":"...","edge_id":"...","reason":"attestation_binding_failed","binding_reason":"claim_predicate_resolution_failed","claim_reason":"..."}}
```

`claim_reason` preserves exactly one Ticket 0059 snake-case reason:

```text
missing_claim
missing_typed_claim
malformed_metadata
missing_claim_domain
wrong_type_claim_domain
duplicate_claim_metadata_key
unknown_claim_domain
claim_domain_mismatch
unknown_claim_metadata_key
missing_inline_subject_descriptor
wrong_type_inline_subject_descriptor
duplicate_descriptor_key
unknown_descriptor_key
missing_schema
wrong_type_schema
unknown_schema
missing_predicate_id
wrong_type_predicate_id
empty_predicate_id
predicate_id_too_long
invalid_predicate_id
unknown_predicate_id
missing_expected_sha256
wrong_type_expected_sha256
invalid_expected_sha256
missing_subject_hex
wrong_type_subject_hex
subject_hex_too_long
invalid_subject_hex
statement_binding_mismatch
missing_claim_content_hash
claim_content_hash_mismatch
decoded_subject_too_large
```

No nested reason is flattened, renamed, omitted, or reordered.

The byte law is: compact UTF-8 JSON, fixed declaration order, standard
`serde_json` escaping, direct non-control Unicode, no normalization, no
escaped slash, and no BOM, insignificant whitespace, or trailing newline.

## Documentation vectors

Each code block contains the complete literal bytes. Its Markdown line ending
is not part of the vector.

### A. `SupportEligible`

362 bytes, SHA-256
`3d1e241b92e8b66c7e42613231a68e4df990c910d48627415ad29db8dc67c085`:

```json
{"outcome":"support_eligible","details":{"receipt":{"schema":"magpie-claim-inline-predicate-edge-lane-v0","lane":"support","predicate_id":"sha256_claim_inline_bytes_equals_v0","claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-support","scope_ref":"scope-s","predicate_relation":"digest_equal","edge_kind":"supports","candidate_ceiling":"settled"}}}
```

### B. `RefutationEligible`

372 bytes, SHA-256
`2849f779e12e756fce866294053d89cae9413254c5edab78221e82ac2d3b3c06`:

```json
{"outcome":"refutation_eligible","details":{"receipt":{"schema":"magpie-claim-inline-predicate-edge-lane-v0","lane":"refutation","predicate_id":"sha256_claim_inline_bytes_equals_v0","claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-refute","scope_ref":"scope-s","predicate_relation":"digest_unequal","edge_kind":"contradicts","candidate_ceiling":"refuted"}}}
```

### C. `Ineligible(DigestUnequalDoesNotSupport)`

152 bytes, SHA-256
`793302a14a991b45ff6a5c6c15c04d936664c29c3685f25d0e509f26c8e983b8`:

```json
{"outcome":"ineligible","details":{"claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-support","reason":"digest_unequal_does_not_support"}}
```

### D. `Ineligible(DigestEqualDoesNotRefute)`

148 bytes, SHA-256
`2f320d25d7b5d3f3c5c45185cf33abb025d2a6f7a93f5721d157c3318eac469c`:

```json
{"outcome":"ineligible","details":{"claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-refute","reason":"digest_equal_does_not_refute"}}
```

### E. `ResolutionFailed(MissingEdge)`

140 bytes, SHA-256
`6ad4b261939e8289b1ee67157e74a37423888e5baf33204802f178e7c4033bc9`:

```json
{"outcome":"resolution_failed","details":{"claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-missing","reason":"missing_edge"}}
```

### F. Nested binding failure: `MissingClaim`

238 bytes, SHA-256
`942b5ec8232429b7301ba489d7aedf214ad32a77be751d59d80bc0707432a62b`:

```json
{"outcome":"resolution_failed","details":{"claim_id":"claim-missing","evidence_id":"evidence-e","edge_id":"edge-e","reason":"attestation_binding_failed","binding_reason":"claim_predicate_resolution_failed","claim_reason":"missing_claim"}}
```

Any byte drift requires a new profile identity.

## Mutual exclusion and repetition

For one exact valid claim-inline predicate in one exact snapshot, the terminal
relation is uniquely:

```text
DigestEqual
or
DigestUnequal
```

Therefore:

- `SupportEligible` and `RefutationEligible` cannot both arise from the same
  terminal relation;
- both a `supports` and a `contradicts` edge may exist, but only the edge
  aligned with the one relation can be eligible;
- the other edge is `Ineligible`, not conflict debt;
- multiple evidence nodes may independently bind to the same claim;
- multiple edges may independently resolve to the same lane;
- repeated eligible lanes do not amplify;
- the same evidence repeated under distinct edge IDs does not amplify; and
- no count, vote, threshold, majority, weight, confidence, or independence
  rule exists here.

An edge ID is first-write-wins in the replay map, so one exact ID cannot
represent multiple replay entries. Distinct edge IDs remain distinct audit
queries but do not acquire aggregation meaning.

This contract ratifies no contradiction debt or conflict blocker. It makes no
claim that a later append-only snapshot cannot contain a changed or
superseding claim; currentness remains future.

## Non-authority fields

The lane grants no authority to:

```text
ClaimAssertedV2.actor_class

EvidenceRegistered.content_hash
EvidenceRegistered.actor_class
EvidenceRegistered.summary

JustificationEdgeRecorded.actor_class
JustificationEdgeRecorded.rationale
JustificationEdgeRecorded.metadata_json
```

These remain replayed provenance or prose. Changing only one of them cannot
change the future lane outcome or canonical bytes when all ratified inputs are
equal.

Attestation `metadata_json` is not generally ignored: its exact Ticket 0061
four-field route is a binding prerequisite. Evidence may not smuggle subject
bytes through that metadata; unknown fields fail under Ticket 0061 before any
edge is read.

## Ticket 0056 blocker closure without policy activation

Ticket 0056 remains a blocked historical candidate. No identifier, policy,
rule, resolver, receipt, or conflict blocker from the withdrawn v4 candidate
becomes active here.

Its counterexample was:

```text
claim expected digest H
+ arbitrary contradicts witness bytes B
+ digest(B) != H
→ manufactured negative result
```

This lane closes that substitution path because:

- the claim owns the immutable inline subject bytes;
- the terminal relation is derived from those claim-owned bytes;
- attestation metadata carries no byte source;
- evidence cannot select substitute bytes; and
- exact binding and exact edge alignment are separately required.

Nevertheless:

```text
RefutationEligible
!= governed Refuted
```

This contract fixes the missing negative-input substrate only. It does not
ratify the policy consequence.

## Distinction from `SupportContributionAuditV0`

`SupportContributionAuditV0` concerns admitted provenance/origin graph
contributions and future corroboration aggregation.

This lane concerns one exact claim-owned machine predicate, one
attestation-bound deterministic evidence node, and one replayed edge.

```text
ClaimInlinePredicateEdgeLaneV0
!= SupportContributionAuditV0

ClaimInlinePredicateEdgeLaneV0
!= origin admission

ClaimInlinePredicateEdgeLaneV0
!= corroboration

ClaimInlinePredicateEdgeLaneV0
!= independence grouping
```

The existing support-contribution contract and runtime remain unchanged.

## Compatibility and non-effects

This contract creates no:

- evidence admission;
- origin admission;
- support aggregation;
- refutation aggregation;
- achieved standing;
- policy v4;
- governed direct-refutation rule;
- conflict precedence;
- contradiction debt;
- invalidation;
- supersession;
- currentness;
- implicit latest policy;
- generic predicate family;
- generic edge composer;
- writer capability;
- `EpistemicGate`;
- loader;
- CAS;
- filesystem or network acquisition;
- callback, plugin, or model authority; or
- Deadbolt change.

It changes no Rust, tests, fixtures, Cargo metadata, dependencies, L0 payload,
canonical L0 bytes, `docs/FORMAT.md`, golden chain, Python verifier, existing
canonical audit bytes, policy matrix, standing resolver, release metadata, CI,
package metadata, or GitHub metadata.

The historical v2 rule stays tied to `sha256_bytes_equals_v0`. This lane
neither replaces, aliases, migrates, nor silently becomes a v2 input.

## Explicit non-goals

The compatibility absences above are normative non-goals. In particular, this
contract does not generalize predicate or edge composition, admit evidence or
origins, aggregate support or refutation, interpret repetition, activate
policy v4, achieve standing, create contradiction debt, decide currentness,
or add acquisition, ingestion, writer, plugin, model, or Deadbolt authority.

It ratifies only the one standing-inert closed family and its future derived
audit representation.

## Hostile contract cases

Unless stated otherwise, the claim, evidence, attestation, edge, and scopes
are structurally valid.

| Case | Expected future result |
| --- | --- |
| missing claim plus missing edge | `ResolutionFailed(AttestationBindingFailed(ClaimPredicateResolutionFailed(MissingClaim)))`; absence is not falsity |
| missing evidence plus missing edge | `ResolutionFailed(AttestationBindingFailed(MissingEvidence))`; edge is not consulted |
| wrong evidence kind plus missing edge | `ResolutionFailed(AttestationBindingFailed(WrongEvidenceKind))` |
| syntactically malformed attestation JSON plus missing edge | `ResolutionFailed(AttestationBindingFailed(MalformedAttestationMetadata))` |
| attestation bound to another claim | `ResolutionFailed(AttestationBindingFailed(ClaimBindingMismatch))` |
| evidence-scope mismatch | `ResolutionFailed(AttestationBindingFailed(EvidenceScopeBindingMismatch))` |
| missing edge | `ResolutionFailed(MissingEdge)` |
| unsupported `ratifies` edge | `ResolutionFailed(UnsupportedEdgeKind)`; never `Ineligible` or negative evidence |
| edge source points to another evidence node | `ResolutionFailed(EdgeSourceBindingMismatch)` |
| edge target points to another claim | `ResolutionFailed(EdgeTargetBindingMismatch)` |
| edge scope differs from the bound scope | `ResolutionFailed(EdgeScopeBindingMismatch)` |
| `DigestEqual` plus `supports` | `SupportEligible`, candidate ceiling `Settled`, achieved standing unchanged |
| `DigestUnequal` plus `supports` | `Ineligible(DigestUnequalDoesNotSupport)`, never refutation |
| `DigestEqual` plus `contradicts` | `Ineligible(DigestEqualDoesNotRefute)`, never support |
| `DigestUnequal` plus `contradicts` | `RefutationEligible`, candidate ceiling `Refuted`, achieved standing unchanged |
| both edge kinds for one `DigestEqual` claim | exact `supports` query is `SupportEligible`; exact `contradicts` query is `Ineligible`; no debt |
| both edge kinds for one `DigestUnequal` claim | exact `contradicts` query is `RefutationEligible`; exact `supports` query is `Ineligible`; no debt |
| several evidence nodes bind the same claim | each exact three-ID query resolves independently; no count or amplification |
| repeated edge ID in replay | first-write-wins map identity; later duplicate cannot overwrite |
| distinct edge IDs repeat the same evidence and lane | distinct audit outcomes; no amplification |
| only edge rationale changes | no lane or canonical-byte change |
| only evidence summary changes | no lane or canonical-byte change |
| only a listed actor/content-hash/edge-metadata field changes | no lane or canonical-byte change |
| caller provides a fake predicate outcome | no accepting API |
| caller provides a fake binding receipt | no accepting API |
| caller serializes and replays an outcome | audit bytes only; no deserializer or accepting API |
| composer mixes snapshots | impossible through the one-snapshot API; private implementation must not do it |
| otherwise valid nested attestation adds a subject-byte field | `ResolutionFailed(AttestationBindingFailed(UnknownAttestationField))`; no relation or edge classification |
| historical `verification_witness` metadata | `ResolutionFailed(AttestationBindingFailed(UnknownAttestationMetadataKey))`; historical v2 behavior unchanged |
| absence of an edge | `MissingEdge`, not falsity |
| unknown edge kind | `UnsupportedEdgeKind`, not ineligible or negative |
| support ceiling cell drifts unexpectedly | only an otherwise eligible support cell yields `SupportCeilingInvariantMismatch` |
| refutation ceiling cell drifts unexpectedly | only an otherwise eligible refutation cell yields `RefutationCeilingInvariantMismatch` |
| ineligible pair plus a drifted ceiling cell | remains `Ineligible`; no ceiling is consulted |

Every case preserves:

```text
absence != falsity
malformed != falsity
ineligible != refuted
eligible != achieved standing
audit bytes != authority
```

## Future implementation requirements

The next separately reviewed runtime ticket may add only:

- the one snapshot method;
- the smallest private shared-composition seams needed to reuse the exact
  Ticket 0059 and Ticket 0061 laws;
- the opaque outcome and receipt;
- the closed classification and failure vocabularies;
- direct typed canonical serialization;
- focused hostile, vector, compatibility, and compile-fail tests; and
- narrow living documentation handoffs.

It must test:

- every matrix cell;
- every top-level failure and exact precedence;
- all 28 nested binding reasons, including all 33 nested claim reasons;
- edge source, target, and scope binding independently and in double faults;
- raw exact ID behavior and absence of namespace interpretation;
- ceiling drift only through the smallest private pure seam, with no public
  policy override;
- all six canonical vectors and all reason spellings;
- Unicode and JSON escaping;
- no BOM, whitespace, escaped slash, or trailing newline;
- ignored-field invariance;
- both-edge mutual exclusion;
- repetition without amplification;
- Ticket 0059/0061 and policy-v2 byte/behavior inertia;
- support-contribution audit inertia; and
- compile-fail barriers for construction, traits, reinjection, alternate
  snapshots, free functions, `StandingView`, raw-material composers, outcomes,
  receipts, relations, and serialized bytes.

If private sharing cannot preserve the exact Ticket 0059 and Ticket 0061
public APIs, failures, vectors, and opacity, implementation must stop.

## Reviewer checklist

1. The family is exactly
   `sha256_claim_inline_bytes_equals_v0 × ExactMachineCheckable ×
   DeterministicVerification × (supports | contradicts)`.
2. The only public authority path is the three-ID
   `StandingReplaySnapshot` method.
3. Only `claim_id`, `evidence_id`, and `edge_id` may be caller-supplied; the
   derived claim, evidence, and edge objects, and all relation, binding,
   policy, and audit values, cannot be caller-supplied.
4. Claim-owned subject bytes close the Ticket 0056 substitution path; evidence
   supplies no bytes.
5. The four relation/edge cells are exact and symmetric; mismatches are
   `Ineligible`, not failure or opposite-polarity authority.
6. Edge lookup and source/target/scope equality use exact raw replayed strings
   in the frozen order.
7. Nested Ticket 0061 and Ticket 0059 failures are retained at both levels.
8. Ceiling invariants are checked only for the corresponding eligible cell.
9. Candidate `Settled`/`Refuted` is not achieved standing.
10. The outcome and ten-field receipt are opaque and non-reinjectable, with no
    duplicate successful IDs or extra fields.
11. Canonical keys, strings, nesting, order, escaping, vectors, lengths, and
    hashes are exact.
12. Repetition does not amplify; both edge kinds create one eligible and one
    ineligible result, not conflict debt.
13. Policy v2 remains tied to historical `sha256_bytes_equals_v0`.
14. `ClaimInlinePredicateEdgeLaneV0` does not collide with
    `SupportContributionAuditV0`, origin admission, corroboration, or
    independence grouping.
15. Ignored claim/evidence/edge fields cannot affect outcome or bytes.
16. No runtime, policy v4, achieved standing, aggregation, L0, Cargo, fixture,
    dependency, I/O, writer, plugin, or Deadbolt behavior appears.

## Stop conditions

Stop rather than implement or widen this contract if:

- Rust, tests, fixtures, Cargo, L0, CI, release, dependency, or FORMAT changes
  are required in this documentation slice;
- evidence-supplied bytes are required;
- the contract cannot remain snapshot-only;
- a caller-created outcome or receipt must be accepted;
- a generic predicate or edge framework is necessary;
- achieved standing or policy v2 must change;
- policy v4, contradiction debt, conflict precedence, or amplification is
  required;
- `SupportContributionAuditV0` must change;
- a new payload, evidence kind, or edge kind is required;
- any path outside Ticket 0062's allowlist must change;
- vectors cannot be reproduced independently; or
- Ticket 0060/0061 opacity prevents sound private composition without
  amending a ratified contract.

## Implementation handoff and next slice

Ticket 0063 implements this contract's exact three-ID snapshot resolver,
one-claim-resolution private composition, four-cell matrix, eligible-only
candidate-ceiling checks, opaque outcomes and receipts, nested failures, and
canonical vectors. `SupportEligible` and `RefutationEligible` remain
standing-inert. No policy v4 or governed `Refuted` result exists.

The next possible separately reviewed slice is:

```text
a fresh deterministic direct-refutation policy contract
```

It is not automatically authorized by the Ticket 0063 implementation.
Contradiction debt, aggregation, the pre-alpha demo, writer behavior, and
ingestion remain absent.
