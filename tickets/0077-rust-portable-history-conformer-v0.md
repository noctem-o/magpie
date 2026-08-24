# Ticket 0077: Rust portable-history conformer v0

## Status

Documentation-only implementation handoff. This ticket and its documentation
PR do not implement the conformer or themselves authorize runtime changes.
Human owner authority remains exclusive.

Owner merge of the companion
[Rust portable-history conformer contract v0](../docs/design/portable-verifier-rust-history-conformer-contract-v0.md)
freezes the implementation boundary and makes one separate bounded Rust runtime
PR the next candidate. That runtime PR must still receive normal owner review.

## Goal

Implement the smallest crate-internal complete Rust portable-history conformer
over the raw-input frontend merged by PR #137. For each accepted
`PendingRecord`, apply exactly:

```text
Sequence
-> PreviousLink
-> ContentHash
-> Signature
-> PayloadValidation
-> Genesis
```

Only complete success through all six stages may commit the next count/tip and
release the frontend continuation. End-of-input must produce the exact frozen
complete-history ACCEPT result.

## Scope

The later runtime PR may:

- add one small crate-internal conformer module in `magpie-log`;
- consume the merged private `FrontendSession`, `PendingRecord`, and
  `FrontendRecord` rather than reparsing input;
- add the minimum private stage helpers and running state needed for the six
  frozen checks;
- narrow the successful `PendingRecord` transition so the conformer is its
  sole legitimate production authority;
- make the smallest crate-private visibility adjustment to an existing pure,
  semantically exact helper;
- remove the temporary module-level `#[allow(dead_code)]` when the production
  consumer makes it unnecessary;
- add an end-to-end frozen-corpus adapter and focused production-path tests;
  and
- expose only the minimum crate-internal count/tip result required by the
  frozen portable contract.

The implementation must process one current record at a time. Exact source
placement may follow then-current crate conventions, but the ownership and
ordering in this ticket are mandatory.

## Non-goals

Do not:

- reimplement profile selection, external-key preflight, framing, UTF-8, JSON,
  schema, lexical decoding, or coordinates;
- change `ProfiledSignatureVerifier`, the legacy unprofiled verifier, canonical
  encoding, hashing, FORMAT, an Accepted ADR, or the portable language;
- change the frozen corpus, manifest, sidecar, golden bytes, or Deadbolt
  fixtures;
- build Python or Go conformers, differential execution, fuzz/metamorphic
  assurance, or release evidence;
- implement checkpoint expectations, replay projections, persistence, or
  admission;
- create a public receipt, `G_history`, `VerifiedHistory`, or another
  detachable authority-bearing result;
- add trust, identity, custody, authority, standing, freshness, latest,
  completeness, canonical-history, rollback, or non-equivocation semantics;
  or
- close or change the controlled disposition of any audit finding.

## Governing sources

Read and apply in this order:

1. [FORMAT](../docs/FORMAT.md) and Accepted
   [ADR-0008](../docs/adr/0008-complete-producing-coordinates.md),
   [ADR-0009](../docs/adr/0009-verified-supplied-history-and-explicit-checkpoint-expectations.md),
   and
   [ADR-0010](../docs/adr/0010-ed25519-verification-profile-and-external-trust-root-admissibility.md);
2. the frozen
   [portable input-language contract](../docs/design/portable-verifier-input-language-contract.md),
   [A-021 verification-semantics contract](../docs/design/a021-ed25519-verification-semantics-contract.md),
   and merged
   [Rust frontend contract](../docs/design/portable-verifier-rust-frontend-contract-v0.md);
3. the exact [portable corpus contract](../docs/design/portable-verifier-corpus-v1.md),
   [manifest](../fixtures/verifier-language-v1/manifest.json), and frozen bytes;
4. merged frontend, canonical, hashing, payload, and
   [`ProfiledSignatureVerifier`](../crates/magpie-log/src/signature_profile.rs)
   implementation/tests; and
5. this ticket's
   [complete conformer contract](../docs/design/portable-verifier-rust-history-conformer-contract-v0.md).

Higher-authority sources win. Existing whole-history/legacy verifiers are
implementation evidence only and must not replace the portable composition.

## Required architecture

The production path must be:

```text
FrontendSession
-> PendingRecord(current record + bound verifier + sole continuation)
-> Sequence
-> PreviousLink
-> ContentHash
-> Signature
-> PayloadValidation
-> Genesis
-> atomically commit next count/tip and release continuation
-> next current record or exact end-of-input ACCEPT
```

The conformer must not reread JSON, call `serde_json`, duplicate framing or
schema logic, normalize source text, reconstruct the external key from Genesis,
reselect a profile, construct a second verifier, route through a legacy whole-
history path, or inspect the next record before the current semantic result.

Prefer one crate-internal conformer session and small private stage helpers.
Do not add six public typestate wrappers, an iterator of records, or a
detachable parsed-record API.

## Exact stage ownership and order

The frontend remains the sole owner of:

```text
ProfileSelection (operational/configuration)
ExternalKey
Framing
JsonSyntax
Schema
```

This conformer owns only:

```text
Sequence
PreviousLink
ContentHash
Signature
PayloadValidation
Genesis
complete end-of-history ACCEPT
```

Each stage runs only if every prior stage passed. A current-record failure
terminates immediately and must outrank every later current-record defect and
all defects in later physical records.

## Running state

The minimum accepted-history state is:

```text
event_count: u64
tip: ContentHash
```

Initialize it to count zero and `ContentHash::ZERO`. The profile, external key,
bound verifier, and cursor already belong to the frontend session and must not
be duplicated as mutable history state. `event_count == 0` is the pre-Genesis
state; no separate `genesis_seen` flag is required.

For each record, validate against the old state and compute the candidate next
state. Count, tip, Genesis-established state, and continuation become available
only after all six checks pass. Use checked arithmetic. Internal overflow or an
unreachable representation failure is operational, not a new portable class.

## Sequence

Require `record.core.seq == old_state.event_count`.

- Record zero expects sequence zero.
- Later records expect the number of completely accepted prior records.
- Schema-valid high `u64` values reach this stage.
- Rejection does not advance any state.
- Empty input invokes no Sequence check.

Use the current frontend record's frozen coordinates. Do not derive stage
semantics from `record_index` or inspect a later record.

## PreviousLink

Require exact byte equality between decoded `core.prev_hash` and
`old_state.tip`.

- Before record zero, the expected predecessor is `ContentHash::ZERO`.
- Thereafter, the expected predecessor is the recomputed hash of the last
  completely accepted record.
- An unchecked transported hash and a failed record never become chain state.

Update the tip only after complete current-record success.

## ContentHash

Use the existing `magpie-core-v1` canonical `EventCore` encoder and
`EventCore::hash()` path. Compare the recomputed hash bytes exactly with the
decoded transported stored hash.

Do not hash JSON source bytes, introduce a serializer or hash function, reorder
fields, or normalize strings. Retain the recomputed hash for Signature and the
candidate next tip. An unexpected canonical-encoding failure after frontend
acceptance is an internal/operational conformance defect, not `ContentHash`.

## Signature

Invoke only the `ProfiledSignatureVerifier` already bound in the pending record
and verify the decoded 64-byte signature over the exact recomputed content
hash. After ContentHash succeeds, that hash is byte-identical to the transported
stored hash.

Map a typed relation rejection to `Signature` at the current coordinates. Do
not inspect private diagnostics, parse key/signature bytes again, use the legacy
verifier, or let Genesis select the key/profile. Success proves only the exact
ADR-0010 relation, not trust or authority.

## PayloadValidation

Apply only the frozen L0 semantic rules left deliberately downstream by the
frontend:

| Payload | Conformer rule |
| --- | --- |
| `Genesis`, `ClaimAsserted`, `EvidenceRecorded`, `ClaimStatusChanged`, `Note` | no additional PayloadValidation rule |
| `SegmentAnchored` | `witness_root` is exactly 64 lowercase ASCII hex |
| `ClaimAssertedV2` | `claim_id`, `statement`, `scope_ref` non-empty; actor closed; optional `content_hash` empty or exact lowercase 64-hex |
| `EvidenceRegistered` | `evidence_id`, `summary`, `scope_ref` non-empty; evidence kind and actor closed; optional `content_hash` empty or exact lowercase 64-hex |
| `JustificationEdgeRecorded` | `edge_id`, `source_id`, `target_id`, `scope_ref`, `rationale` non-empty; edge kind and actor closed |

Use the exact case-sensitive actor/evidence/edge vocabularies frozen by FORMAT
and the input-language contract. `metadata_json` remains opaque. The current
pure `Payload::validate()` may be reused only if it remains semantically exact;
its diagnostic prose is not portable vocabulary.

Do not pull Genesis rules into this stage or add application/business,
standing, evidence-admission, or truth semantics.

## Genesis

Apply the exact sub-order:

```text
position/kind
-> canonicalization_profile == "magpie-core-v1"
-> declared verifying-key lexical validity and external-key equality
```

At count zero the payload must be Genesis. After count zero it must not be
Genesis. For record-zero Genesis, require the exact canonicalization profile,
then require `verifying_key` to be exact lowercase 64-hex, decode `[u8; 32]`,
and compare those bytes with the frontend-bound verifier's original external
key bytes.

Genesis cannot choose or override the external key or signature profile.
Equality records self-description only and does not establish key trust.

## Continuation authority

The production conformer must become the sole legitimate authority for
successful `PendingRecord` continuation release. Use the smallest type-shaped
mechanism, expected to be a private, non-constructible
`SemanticSuccess`-equivalent capability created only after the ordered
six-stage helper returns success.

The transition must consume both the capability and `PendingRecord`. Remove or
narrow the generic production `complete_semantics(Ok(()))` seam so arbitrary
crate code cannot assert semantic success. A conspicuous `#[cfg(test)]` bypass
may remain only for isolated frontend tests. Do not expose a public token or
create six typestate layers.

An equivalent consuming design is allowed only if code review can directly
verify the same sole-construction authority.

## Complete result and failure boundary

At end-of-input, return the current state:

- zero-byte history after successful key preflight: ACCEPT count zero, zero
  tip;
- non-empty history: ACCEPT exact event count and recomputed last-record tip.

The minimum production result is crate-internal count/tip. The manifest's
`ordered_recomputed_hashes` are required test observations, not an automatic
production-result field. Do not create a receipt or `G_history`.

Governed failure is exact frozen REJECT class plus the current record's frozen
line/index. Profile selection, internal inconsistency, unreachable canonical
failure, allocation/resource failure, and outer I/O remain operational or
configuration errors outside portable ACCEPT/REJECT. No semantic dispatch may
use `Display` or `Debug` strings.

## Corpus requirement

Execute all 432 frozen cases end to end and match every expected field:

| Final result/class | Cases |
| --- | ---: |
| `ACCEPT` | 19 |
| `ExternalKey` | 21 |
| `Framing` | 13 |
| `JsonSyntax` | 18 |
| `Schema` | 272 |
| `Sequence` | 4 |
| `PreviousLink` | 5 |
| `ContentHash` | 3 |
| `Signature` | 16 |
| `PayloadValidation` | 46 |
| `Genesis` | 15 |
| **Total** | **432** |

Preserve the 324 frontend-tranche-final results. The conformer supplies the 19
ACCEPT and 89 later-stage REJECT outcomes that #137 intentionally did not own.
For every REJECT match verdict, class, line, and index; for every ACCEPT match
count, tip, and test-observed ordered recomputed hashes. Do not mutate bytes,
expectations, manifest, or sidecar.

Passing 432/432 is complete authoritative Rust evidence against this frozen
corpus, subject to hostile review. It is not cross-language conformance or
finding closure.

## Focused tests required of the runtime PR

### Sequence and PreviousLink

- correct first sequence, skip, duplicate, regression, and high valid `u64`;
- first-record zero link, correct chain, and mismatches;
- state not advanced after rejection; and
- a failed record never becomes a predecessor.

### ContentHash and Signature

- exact canonical preimage, representative one-bit mutation, stored mismatch,
  and proof that JSON bytes are not hashed;
- canonical encoder totality expectation after Schema;
- valid exact profile, invalid signature, altered content hash, no legacy path,
  and no Genesis-driven key/profile selection.

### PayloadValidation and Genesis

- positive/negative witness for every frozen payload rule and vocabulary;
- schema-valid but payload-invalid inputs reach PayloadValidation;
- valid one-record/multi-record histories;
- wrong first kind, later Genesis, wrong profile, malformed/unequal declared
  key, exact external-key equality, and empty-history behavior.

### First failure, state, and completion

- Sequence beats all later current-record defects;
- PreviousLink beats hash/signature/payload/Genesis;
- ContentHash beats Signature;
- Signature beats PayloadValidation;
- PayloadValidation beats Genesis;
- current semantic failure beats malformed record N+1 without inspecting it;
- zero-, one-, and multi-record completion returns exact count/tip;
- continuation releases only after all six stages pass; and
- production code cannot construct the success capability through another
  route.

## Protected boundaries

The runtime PR must leave unchanged:

- FORMAT and Accepted ADR semantics;
- portable input/result/A-021/frontend contracts and Ticket 0076;
- corpus, manifest, sidecar, golden, Deadbolt, and canonical bytes;
- `ProfiledSignatureVerifier` relation and legacy verifier behavior;
- Python and Go behavior;
- audit finding dispositions; and
- trust, authority, receipt, checkpoint, release, and cross-language policy.

No new dependency package or invisible normative resource limit is authorized.
Process one current record at a time; host resource failures stay operational.

## Stop conditions

Stop and request owner review if:

1. higher-authority sources disagree on any of the six stage laws;
2. the frozen corpus conflicts with governing prose;
3. exact first-failure ordering or coordinates are ambiguous;
4. exact ACCEPT result is ambiguous;
5. the #137 frontend requires fundamental redesign;
6. `ProfiledSignatureVerifier` semantics must change;
7. canonical encoding, FORMAT, an Accepted ADR, or portable language must
   change;
8. corpus, manifest, sidecar, golden, or Deadbolt bytes must change;
9. legacy verifier behavior must change;
10. new trust/authority semantics are needed;
11. new producing-coordinate, receipt, or `G_history` semantics are needed;
12. a new portable rejection class is needed;
13. a hidden normative resource ceiling is needed;
14. a new ADR-level constitutional decision is needed; or
15. any controlled audit disposition would need to change.

Do not solve a stop condition by widening the runtime tranche.

## Acceptance criteria

The later implementation PR is acceptable only when:

- production conformer consumes merged `PendingRecord` without reparsing;
- stage order is exactly Sequence, PreviousLink, ContentHash, Signature,
  PayloadValidation, Genesis;
- current failure prevents all later-stage and later-record inspection;
- state changes only after complete current-record success;
- existing canonical/hash path and bound `ProfiledSignatureVerifier` are used
  at their exact stages;
- later rules remain at their frozen failure classes;
- the conformer is sole intended production authority for continuation success;
- empty-history and final count/tip semantics are exact;
- all 432 cases match all expected result fields;
- no protected boundary or public receipt/authority object changes;
- validation is green; and
- hostile review finds no precedence, transactional-state, continuation,
  canonical-hash, key/profile, or legacy-path bypass defect.

## Explicit non-closures

Even after a future implementation lands, do not automatically close A-021,
A-004, RQ-006, or RQ-013. Complete Rust execution is major same-language
evidence only. Independent Python/Go conformers, exact differential execution,
wider assurance, hostile review, and separate controlled reconciliation remain
required where the ledger specifies them.
