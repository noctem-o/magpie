# Rust portable-history conformer contract v0

## Status and authority

Status: implementation contract for the next bounded Rust tranche.

The companion implementation handoff is
[Ticket 0077](../../tickets/0077-rust-portable-history-conformer-v0.md).
Owner merge freezes this implementation shape; runtime work remains a separate
PR and review.

This document maps the already frozen complete portable-history semantics onto
the merged Rust raw-input frontend. It is not an ADR, FORMAT, a new portable
language, a corpus revision, cross-language conformance evidence, a receipt or
trust contract, a release contract, or finding closure.

Authority remains, in order:

1. [FORMAT](../FORMAT.md) and Accepted ADRs, especially
   [ADR-0008](../adr/0008-complete-producing-coordinates.md),
   [ADR-0009](../adr/0009-verified-supplied-history-and-explicit-checkpoint-expectations.md),
   and [ADR-0010](../adr/0010-ed25519-verification-profile-and-external-trust-root-admissibility.md);
2. the frozen
   [portable input-language contract](portable-verifier-input-language-contract.md),
   [A-021 verification-semantics contract](a021-ed25519-verification-semantics-contract.md),
   and [Rust frontend contract](portable-verifier-rust-frontend-contract-v0.md);
3. the exact frozen [portable corpus](portable-verifier-corpus-v1.md) and its
   [manifest](../../fixtures/verifier-language-v1/manifest.json);
4. merged implementation and tests; and
5. this implementation mapping and living administrative documentation.

If this document conflicts with a higher layer, the higher layer wins and the
runtime tranche stops. No implementation convenience may silently create a new
verification law.

This contract introduces no new portable semantic decision. Its private
continuation-success capability is an implementation enforcement of the
already frozen first-failure law and #137's exclusive-continuation boundary,
not a new result, stage, trust rule, or ADR-level choice.

## Baseline and landed dependency

This contract is reconciled against `main`
`7e7cb5a930b1db4f2ebc1c5ad0296641cd74db6d`, the merge of PR #137. The
reviewed implementation head is
`edb25ca4a85a64ad9a328b75be8a4490b3a88d4e`. PR #136, merge
`e2a3f9dca6f23b148062a0f7c69cbd17893f1318`, froze the frontend architecture
implemented by #137.

The merged crate-internal `portable_verifier` now owns:

- explicit profile selection and the complete external-key gate before any
  history byte is inspected;
- exact-byte framing, UTF-8, line, and record-index law;
- one complete locked-`serde_json` syntax pass;
- ordered duplicate-preserving closed schema and lexical decoding;
- a private typed `FrontendRecord`; and
- a `PendingRecord` that exclusively owns the current record, the bound
  `ProfiledSignatureVerifier`, and the only continuation.

Its tests establish 324 frontend-tranche-final corpus outcomes and explicit
non-preemption accounting for the other 108 cases. They do not run the six
remaining stages or produce a complete portable-history result.

PR #135's
[`ProfiledSignatureVerifier`](../../crates/magpie-log/src/signature_profile.rs)
remains the authoritative Rust `V_sig` primitive. The legacy unprofiled
verifier remains compatibility-only and unchanged.

## Contract thesis

The complete Rust conformer consumes the merged frontend product one record at
a time:

```text
FrontendSession
    |
    v
PendingRecord(current record + bound verifier + exclusive continuation)
    |
    v
Sequence
    |
    v
PreviousLink
    |
    v
ContentHash
    |
    v
Signature
    |
    v
PayloadValidation
    |
    v
Genesis
    |
    v
commit next history state + release continuation
    |
    v
next current record, or exact complete-history ACCEPT at end-of-input
```

Any current-record failure terminates immediately. The continuation remains
unreleased, and no later record is inspected. Only complete success through all
six stages may advance traversal.

The conformer is an in-memory deterministic verifier over one supplied
immutable history. It does not perform persistence, replay projections,
checkpoint evaluation, admission, or authority policy.

## Exact ownership and no-reparsing law

The frontend exclusively owns:

```text
ProfileSelection (operational/configuration)
ExternalKey
Framing
JsonSyntax
Schema
```

The conformer exclusively owns:

```text
Sequence
PreviousLink
ContentHash
Signature
PayloadValidation
Genesis
complete end-of-history ACCEPT
```

The conformer must consume `PendingRecord` and its private `FrontendRecord`.
It must not:

- reread raw JSON or call `serde_json`;
- duplicate framing, UTF-8, schema, lexical, or coordinate logic;
- normalize source text or deserialize a second `SignedEvent`;
- reconstruct the external key from Genesis;
- select, default, infer, negotiate, or replace the signature profile;
- construct a second verifier or route through legacy
  `VerifyingKey::verify`;
- parse the whole history before semantic verification; or
- turn the frontend or corpus adapter into a second verifier grammar.

The frontend's decoded strings and fixed arrays are the semantic inputs. Raw
JSON spelling has completed its normative role before the conformer begins.

## Minimum production shape

Exact Rust names remain implementation details. The smallest intended shape is
one crate-internal conformer session that owns:

- a ready `FrontendSession`; and
- the running accepted-history state below.

Its step/run operation consumes itself or otherwise retains exclusive
ownership. It repeatedly obtains one `PendingRecord`, evaluates the six stages,
and either returns the exact governed rejection, returns an operational error,
or advances with the sole continuation. End-of-input returns the frozen ACCEPT
summary.

Do not add public constructors, serde implementations, an iterator of parsed
records, a detachable `ParsedRecord`, or six public typestate wrappers. A small
private helper per semantic stage is encouraged when it makes order and error
mapping directly inspectable.

## Running accepted-history state

The minimum mutable semantic state is exactly:

```text
event_count: u64
tip: ContentHash
```

Before record zero:

```text
event_count = 0
tip         = ContentHash::ZERO
```

The selected signature profile, original external-key bytes, decoded key point,
and frontend cursor are already bound inside the frontend/PendingRecord. They
are immutable session context, not duplicate history state. A separate
`genesis_seen` flag is unnecessary: `event_count == 0` is the exact
pre-Genesis state, and successful Genesis at record zero is required before
`event_count` can become one.

Each stage reads the old state. The conformer computes a candidate next state
but makes it observable only after every current-record stage succeeds:

```text
validate current record against old state
-> compute recomputed current hash
-> run all six stages
-> acquire semantic-success capability
-> consume PendingRecord and its continuation
-> return ready conformer with event_count + 1 and recomputed hash as tip
```

A rejected record never advances count, tip, Genesis state, or cursor. Checked
counter arithmetic is required. A host/internal representational failure is an
operational conformance failure, not a new `Sequence` rejection or an invisible
portable record-count limit.

The production result need not retain all accepted records or ordered hashes.
A test adapter may observe each recomputed hash while driving the frozen corpus.

## Exact stage order

For one frontend-accepted pending record, the first applicable failure wins:

| Order | Stage | Frozen responsibility |
| ---: | --- | --- |
| 1 | `Sequence` | decoded `core.seq` equals the zero-based accepted-record position |
| 2 | `PreviousLink` | decoded `core.prev_hash` equals the previous accepted recomputed hash, or zero before record zero |
| 3 | `ContentHash` | existing `magpie-core-v1` canonical `EventCore` hash equals transported stored hash |
| 4 | `Signature` | bound `ProfiledSignatureVerifier` accepts decoded signature over the exact current content hash |
| 5 | `PayloadValidation` | only the frozen L0 payload semantic rules pass |
| 6 | `Genesis` | position/kind, then profile, then declared-key lexical/equality rules pass |

All later stages remain unexecuted after a failure. The current record's frozen
one-based `line` and zero-based `record_index` are copied from `FrontendRecord`.
No byte-column coordinate exists.

## Sequence

The governing law is the [input-language contract's chain semantics and
first-failure law](portable-verifier-input-language-contract.md#chain-semantics),
supported by FORMAT's chain rules and the frozen N21/U64 cases.

The exact check is:

```text
record.core.seq == old_state.event_count
```

Therefore:

- the first record expects `seq = 0`;
- each subsequent record expects the exact count of completely accepted prior
  records;
- in-range high `u64` spellings have already passed `Schema` and fail here if
  they do not equal that position;
- no sequence state advances before whole-record success; and
- an empty input invokes no Sequence check.

`FrontendRecord.record_index` supplies the failure coordinate. It is not a
second caller-controlled sequence source. An internal disagreement between the
frontend cursor position and accepted-state count is a conformance defect, not
a new portable rejection reason.

## PreviousLink

The governing law is FORMAT's exact chain relation and the frozen N22/N29
cases. Compare the decoded `core.prev_hash` bytes exactly with
`old_state.tip`:

```text
record 0: core.prev_hash == ContentHash::ZERO
record n: core.prev_hash == recomputed hash of completely accepted record n-1
```

The running tip is the prior recomputed hash. The prior record's transported
stored hash is byte-identical after its `ContentHash` gate, but an unchecked
transport value never becomes state. A failed current record cannot become a
predecessor, and tip changes only after all six stages succeed.

## ContentHash

The governing sources are
[FORMAT sections 2-4](../FORMAT.md), the existing
[`EventCore::canonical_bytes` and `EventCore::hash`](../../crates/magpie-log/src/event.rs),
and the frozen ContentHash/ordered-hash expectations.

The stage is exactly:

```text
canonical_bytes = existing magpie-core-v1 encoder(record.core)
recomputed_hash = SHA-256(canonical_bytes)
require recomputed_hash.bytes == record.stored_hash
```

Use the existing `EventCore::hash()`/canonical encoder. Do not hash JSON bytes,
add a serializer, reorder fields, normalize strings, select another profile, or
duplicate SHA-256 logic. The recomputed value is retained for Signature and the
candidate next tip.

Once `Schema` has accepted a record, the frozen encoder is total for the typed
input. An unexpected inability to encode/hash is an operational or internal
conformance defect. It is not a `ContentHash` mismatch and does not create a
new portable rejection class.

## Signature

The governing sources are
[ADR-0010](../adr/0010-ed25519-verification-profile-and-external-trust-root-admissibility.md#normative-relation),
the [portable input contract](portable-verifier-input-language-contract.md#chain-semantics),
and merged
[`ProfiledSignatureVerifier::verify`](../../crates/magpie-log/src/signature_profile.rs).

After ContentHash equality succeeds, call the verifier already owned by the
pending record:

```text
pending.signature_verifier().verify(
    recomputed_hash.as_bytes(),
    pending.record().signature(),
)
```

The recomputed hash and stored hash are now byte-identical. Passing the
recomputed bytes makes the ADR-0010 `content_hash` dependency explicit without
changing the message.

Any `SignatureRejected` returned by this call maps to `Signature` at the
current record coordinate. Do not inspect its private rejection reason or
`Debug`/`Display` text. Do not parse key/signature bytes again, reconstruct
`A`, invoke a legacy verifier, clear a cofactor, infer a Genesis-selected
profile, or add a host-library verification mode.

Success establishes only the exact ADR-0010 relation for this message and
external key. It establishes no key trust, identity, custody, authority,
freshness, admission, or truth.

## PayloadValidation

The governing sources are
[FORMAT's L0 payload validity rules](../FORMAT.md), the
[portable input contract's required-fields and payload boundary](portable-verifier-input-language-contract.md#required-fields-and-emptiness),
the exact 46 frozen `PayloadValidation` cases, and current pure
[`Payload::validate`](../../crates/magpie-log/src/event.rs).

The frontend has already proved the exact object shape, JSON types, required
fields, payload discriminator, statuses, and decoded strings. This stage owns
only the following semantic rules:

| Payload variant | Rules at `PayloadValidation` | Frontend already guarantees |
| --- | --- | --- |
| `Genesis` | none; Genesis-specific rules remain later | closed shape and strings |
| `ClaimAsserted` | none | closed shape, exact status |
| `EvidenceRecorded` | none | closed shape and strings |
| `ClaimStatusChanged` | none | closed shape, exact statuses |
| `Note` | none | closed shape and string |
| `SegmentAnchored` | `witness_root` is exactly 64 lowercase ASCII hex characters | closed shape; all fields decoded as strings |
| `ClaimAssertedV2` | `claim_id`, `statement`, and `scope_ref` are non-empty; `actor_class` is closed; `content_hash` is empty or exactly 64 lowercase ASCII hex | closed shape and strings |
| `EvidenceRegistered` | `evidence_id`, `summary`, and `scope_ref` are non-empty; `evidence_kind` and `actor_class` are closed; `content_hash` is empty or exactly 64 lowercase ASCII hex | closed shape and strings |
| `JustificationEdgeRecorded` | `edge_id`, `source_id`, `target_id`, `scope_ref`, and `rationale` are non-empty; `edge_kind` and `actor_class` are closed | closed shape and strings |

The exact closed vocabularies are:

```text
actor_class:
  HumanRoot
  AgentProposer
  AutomatedVerifier
  DeadboltAnchorer
  LensWitness
  SourceImporter

evidence_kind:
  DeterministicVerification
  HumanRatification
  DeadboltAnchor
  ExecutionEvidence
  BehavioralEvaluation
  ExternalSource
  ModelSelfReport
  LensReadout

edge_kind:
  supports
  derived_from
  contradicts
  supersedes
  invalidates
  ratifies
```

Comparison is exact and case-sensitive, with no trim, normalization, or case
folding. `metadata_json` remains opaque decoded string data and is never parsed.
Legacy/provenance/tag-5 non-root strings receive no new non-empty rule.
`Payload::validate()` currently implements this exact pure law and may be
reused; its diagnostic strings are not portable vocabulary.

This stage does not validate a foreign bundle, interpret claims, establish
standing, authenticate actors, admit evidence, or decide truth.

## Genesis

The governing sources are [FORMAT section 5](../FORMAT.md), the
[portable chain/first-failure law](portable-verifier-input-language-contract.md#first-failure-law),
and frozen N25-N27 plus Genesis-lexical cases.

Within `Genesis`, the exact sub-order is:

```text
position/kind
-> canonicalization_profile equality
-> declared-key lexical validity and equality
```

### Position and kind

- When `old_state.event_count == 0`, the payload must be `Genesis`.
- When `old_state.event_count > 0`, the payload must not be `Genesis`.
- A later non-Genesis record passes this substage and performs no Genesis field
  checks.

### Canonicalization profile

Record-zero Genesis must declare exactly `magpie-core-v1`. This is the frozen
canonical EventCore profile, not the ADR-0010 signature-profile identity.
Genesis cannot select, replace, or negotiate `V_sig`.

### Declared external key

Only after the preceding substages pass:

1. require Genesis `verifying_key` to be exactly 64 lowercase ASCII hex
   characters;
2. decode exactly `[u8; 32]`; and
3. compare those bytes exactly with
   `pending.signature_verifier().external_key_bytes()`.

The portable external-key transport has already passed the same unique exact
lowercase-hex law. Therefore byte equality after both lexical checks is exactly
equivalent to equality with the original external transport text; the
conformer need not retain or reconstruct raw input text. A future
implementation may reuse the frontend's private lowercase-hex helper through
the smallest crate-private visibility adjustment.

Equality proves only that Genesis self-describes the externally supplied key.
It does not bootstrap trust or give Genesis authority to choose that key.

### Empty input

After a successful external-key preflight, exact zero bytes reaches frontend
end-of-input before any pending record. It is ACCEPT with count zero and zero
tip. No Genesis, signature, or key-equality check occurred, and the result makes
no Genesis/key-binding claim.

## Semantic-success continuation authority

PR #137 intentionally left a temporary crate-internal generic transition:
`PendingRecord::complete_semantics(Ok(()))`. The frontend could not prove that
the later six stages ran because no production history consumer existed.

The Ticket 0077 runtime must make the complete conformer the sole legitimate
production authority for successful release. The selected smallest mechanism
is a private, non-constructible semantic-success capability:

- the conformer owns a zero-sized/private `SemanticSuccess`-equivalent whose
  constructor is visible only after its ordered six-stage helper returns
  success;
- `PendingRecord` consumes that capability and itself to return the sole ready
  continuation;
- the current generic production `Result<(), E>` success-report seam is
  removed or narrowed so arbitrary crate production code cannot say “trust me”;
  and
- a conspicuously named `#[cfg(test)]` bypass may remain solely for isolated
  frontend tests and 324/108 accounting.

This requires only a narrow crate-private visibility/type adjustment. It does
not reopen the frontend contract, add six typestate wrappers, expose a public
success token, or create a receipt. An equivalent consuming design is allowed
only if it preserves the same construction authority visibly in the type
boundary.

The conformer should return the next ready frontend session and candidate next
history state together, so continuation release and state commit form one
unforgeable successful transition.

## Complete first-failure law

The composed order is exactly:

```text
profile identity selection (operational/configuration)
-> ExternalKey
-> current Framing
-> current JsonSyntax
-> current Schema
-> current Sequence
-> current PreviousLink
-> current ContentHash
-> current Signature
-> current PayloadValidation
-> current Genesis
-> commit current record and release continuation
-> next physical record
-> final ACCEPT at end-of-input
```

For the current pending record, each stage returns immediately on failure. The
conformer must not call the continuation transition before all six pass. A
semantic failure on record N therefore outranks malformed or invalid bytes in
record N+1 without inspecting those bytes.

All six governed failures use the current frontend record's `line` and
`record_index`. Frontend and external-key coordinates remain as already
implemented. There is no byte-column law and no collection of later errors.

## Complete result and operational boundary

The production semantic result is exactly one of:

```text
ACCEPT {
    event_count: u64,
    tip: ContentHash / 32-byte lowercase hex at the portable result boundary
}

REJECT {
    class: one of the ten frozen classes,
    line: one-based integer or absent,
    record_index: zero-based integer or absent
}
```

At end-of-input, return the current accepted state:

- empty history: `event_count = 0`, `tip = ContentHash::ZERO`;
- non-empty history: exact accepted record count and the recomputed hash of the
  last completely accepted record.

The frozen manifest additionally records `ordered_recomputed_hashes` for every
ACCEPT case. That is a conformance-test observation collected as each record
passes ContentHash; it need not enlarge the minimum production ACCEPT product.

The result remains crate-internal. It is not `VerifiedReplaySummary`,
`G_history`, a detachable ADR-0008-complete receipt, a trust result, or an
authority object. If a later C3 tranche requires a public detachable result,
it must separately supply complete producing coordinates.

Unsupported profile selection, frontend operational inconsistency,
unreachable canonical-encoder failure, allocation/resource failure, I/O around
an outer caller, and internal invariant failure are operational/configuration
results outside portable ACCEPT/REJECT. Do not collapse them into a governed
class, dispatch on `Display`/`Debug`, or return acceptance after incomplete
evaluation.

## Existing helper reuse

Pure existing helpers may be reused only when their semantics and ordering are
exact:

- `EventCore::hash()` / the existing canonical encoder at ContentHash;
- `ProfiledSignatureVerifier::verify()` at Signature;
- `Payload::validate()` at PayloadValidation;
- `ContentHash::ZERO`, fixed-array accessors, and the frontend's exact
  lowercase-hex helper where visibility permits.

Do not wrap `IncrementalVerifier`, `parse_and_verify_records`,
`verify_event_integrity`, `LogReader::verify_chain`, or another legacy
whole-history path. Those paths deserialize differently, use the legacy
unprofiled verifier, expose different errors, and do not own the portable
first-failure composition.

The implementation may make the smallest crate-private visibility adjustment
to a semantically exact pure helper. It must not change legacy behavior.

## Corpus requirement

The complete Rust conformer must execute all 432 frozen cases end to end:

| Final result/class | Cases | Owner after Ticket 0077 |
| --- | ---: | --- |
| `ACCEPT` | 19 | complete conformer |
| `ExternalKey` | 21 | merged frontend, preserved exactly |
| `Framing` | 13 | merged frontend, preserved exactly |
| `JsonSyntax` | 18 | merged frontend, preserved exactly |
| `Schema` | 272 | merged frontend, preserved exactly |
| `Sequence` | 4 | complete conformer |
| `PreviousLink` | 5 | complete conformer |
| `ContentHash` | 3 | complete conformer |
| `Signature` | 16 | complete conformer |
| `PayloadValidation` | 46 | complete conformer |
| `Genesis` | 15 | complete conformer |
| **Total** | **432** | |

The implementation must match exact verdict, first class, line, and
`record_index` for every REJECT, and event count, tip, and ordered recomputed
hashes for every ACCEPT. Corpus bytes, expectations, manifest, and sidecar are
immutable.

Passing 432/432 establishes complete authoritative Rust conformance to the
frozen corpus, subject to hostile review. It is not Rust/Python/Go equivalence,
general release evidence, or finding closure.

## Focused test matrix for the runtime tranche

The manifest-driven runner is necessary but not sufficient. Add named tests
that exercise the production conformer.

### Sequence

- first sequence zero;
- skip, duplicate, and regression;
- full-width valid `u64` reaching Sequence;
- count not advanced after rejection; and
- current Sequence failure outranks every later current-record defect.

### PreviousLink

- first-record zero predecessor;
- correct multi-record chain;
- first and later mismatch;
- a failed record never becomes the next predecessor; and
- PreviousLink outranks hash, signature, payload, and Genesis.

### ContentHash

- exact existing canonical preimage and hash;
- one-bit mutation in each representative canonical field;
- well-shaped stored-hash mismatch;
- proof that JSON source bytes are never hashed;
- total canonical encoder expectation after Schema; and
- ContentHash outranks Signature and later semantics.

### Signature

- valid exact profile/signature;
- invalid signature and altered content hash;
- identity-`R` positive and other frozen ADR-0010 boundaries through the real
  conformer path;
- no legacy verifier call;
- no Genesis-driven key/profile selection; and
- Signature outranks PayloadValidation and Genesis.

### PayloadValidation

- one positive and negative witness for every table rule and every closed
  vocabulary;
- schema-valid but payload-invalid material reaches this stage;
- empty optional tag-6/tag-7 `content_hash` passes; and
- PayloadValidation outranks Genesis.

### Genesis

- valid one-record and multi-record histories;
- non-Genesis at zero and Genesis after zero;
- wrong canonicalization profile;
- malformed/uppercase/empty declared key assigned to Genesis;
- lexically valid external-key mismatch;
- exact byte equality with the preflight-bound external key;
- no inference of trust; and
- exact empty-history special case.

### Continuation and completion

- malformed record N+1 remains uninspected when record N fails at each semantic
  stage;
- only complete six-stage success can mint the continuation capability;
- the production generic “semantic success” bypass no longer exists;
- any retained bypass is compiled only for tests;
- zero-byte history returns exact zero count/tip;
- one- and multi-record histories return exact count/tip; and
- state and continuation remain unavailable after rejection.

## Resource and performance boundary

Process one current record at a time. Do not materialize the whole history or
retain accepted records merely to compute count/tip.

The frozen language defines no new maximum history bytes, record count, record
size, payload size, execution time, or memory ceiling. This contract adds none.
Host/resource failures remain operational. A needed normative limit is a stop
condition requiring separate owner review; it must not be smuggled in as a
portable rejection.

No speculative lifetime/borrowing rewrite is required. Record-local owned
strings in the merged frontend are not a semantic defect and no normative
resource ceiling justifies widening this tranche.

## Carried #137 review notes

The runtime tranche should carry three non-blocking maintenance observations:

1. once the conformer is the production consumer, remove the temporary
   module-level `#[allow(dead_code)]` if it is no longer needed;
2. if `framing.rs` is otherwise touched, a small comment may distinguish raw
   byte `0x0d` from JSON escape bytes `5c 72`; and
3. do not undertake a speculative String/lifetime rewrite.

These are maintenance guidance, not new language law or required reasons to
touch otherwise unrelated frontend files.

## Explicit non-goals and non-closures

This contract does not include or establish:

- new parsing, framing, schema, portable-language, FORMAT, or ADR semantics;
- Python or Go conformers, cross-language differential execution, or an
  independent release oracle;
- the following non-normative Rust fuzz/metamorphic assurance tranche;
- checkpoint expectation evaluation, replay projections, receipts,
  `G_history`, or ADR-0008 runtime completion;
- legacy verifier migration or a change to `ProfiledSignatureVerifier`;
- key trust, custody, identity, authority, admission, standing, or truth;
- latest state, freshness, completeness, canonical history, rollback
  resistance, or non-equivocation;
- release hardening, release evidence, or release readiness; or
- closure or disposition change for A-021, A-004, RQ-006, RQ-013, or any other
  finding.

Even after a future 432/432 Rust implementation, those findings are not
automatically closed. Cross-language and wider assurance gates remain separate.

## Runtime-tranche stop conditions

Stop for owner review rather than widening scope if:

1. higher-authority sources disagree on any of the six stage laws;
2. the frozen corpus conflicts with governing prose;
3. exact first-failure ordering or coordinates are ambiguous;
4. exact ACCEPT count/tip semantics are ambiguous;
5. the merged frontend must be fundamentally redesigned;
6. `ProfiledSignatureVerifier` semantics must change;
7. canonical encoding, FORMAT, an Accepted ADR, or the portable language must
   change;
8. corpus, manifest, sidecar, golden, or Deadbolt bytes must change;
9. legacy verifier behavior must change;
10. a new portable rejection class is needed;
11. new trust, authority, producing-coordinate, receipt, or `G_history`
    semantics are needed;
12. a hidden normative resource ceiling appears necessary;
13. a new ADR-level constitutional decision appears necessary; or
14. any controlled audit disposition would need to change.

## Acceptance criteria for Ticket 0077

The later runtime PR is acceptable only when:

- the production conformer consumes merged `PendingRecord` without reparsing;
- stage order is exactly Sequence, PreviousLink, ContentHash, Signature,
  PayloadValidation, Genesis;
- a current failure prevents later-stage and later-record inspection;
- history state changes only after complete current-record success;
- existing canonical encoding/hash and the bound `ProfiledSignatureVerifier`
  are used at their exact stages;
- later semantic rules remain in their frozen classes;
- the conformer is the sole production authority for successful continuation;
- empty-history and final count/tip semantics are exact;
- all 432 frozen cases match every expected field, including test-observed
  ordered recomputed hashes;
- no protected boundary or public authority/receipt surface changes;
- validation is green; and
- hostile review finds no precedence, transactional-state, continuation, key,
  or legacy-path bypass.

## Target architecture law

> The complete Rust portable-history conformer consumes exactly one merged
> frontend `PendingRecord`, applies Sequence, PreviousLink, ContentHash,
> Signature, PayloadValidation, and Genesis in that order, and commits count,
> recomputed tip, and the sole continuation only after all six succeed. It
> never reparses input, reselects the key/profile, invokes the legacy verifier,
> or inspects a later record early. End-of-input returns exactly the frozen
> count/tip ACCEPT result, without minting trust, authority, a receipt,
> cross-language conformance, or finding closure.
