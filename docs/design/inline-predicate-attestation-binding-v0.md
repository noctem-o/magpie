# Inline Predicate Attestation Binding v0

## Status

Contract ratified by Ticket 0060. Ticket 0061 implements the routing-only,
standing-inert runtime in this candidate patch; this note's ratified authority
law is unchanged.

```text
outer evidence-metadata envelope:
inline_predicate_attestation

inner schema:
magpie-inline-predicate-attestation-v0

evidence kind:
DeterministicVerification

outcome:
Bound | ResolutionFailed

authority:
one routing-only same-replay resolver over one StandingReplaySnapshot

predicate relation, edge polarity, contribution, standing:
not read, not produced, not ratified
```

This contract answers one question only:

```text
Does this exact replayed evidence node route and bind to this exact
replayed claim, its validated predicate identity, and its exact scope?
```

It does not answer whether the predicate relation is equal or unequal;
whether evidence supports or contradicts; whether a contribution is
eligible; whether a claim is `Supported`, `Settled`, or `Refuted`; whether
contradiction debt, invalidation, supersession, or currentness changes; or
whether any edge is authoritative.

## Inherited boundary

Ticket 0057 already ratified the complete routing-field boundary:

```text
schema
predicate_id
subject_claim_id
scope_ref
```

The same contract forbids an inline-family attestation from supplying an
alternate subject byte source. Ticket 0058 ratified the neutral
`sha256_claim_inline_bytes_equals_v0` predicate and the separation:

```text
attestation binding != predicate evaluation
predicate evaluation != evidence polarity
evidence polarity != standing contribution
```

Ticket 0059 implements the neutral evaluator on one
`StandingReplaySnapshot`. This contract neither changes that evaluator nor
accepts one of its outcomes as input.

The attestation is replayed asserted routing material. It is not a receipt.
`Bound` exists only when the resolver derives it from the exact claim and
evidence nodes co-retained in one snapshot.

## Exact evidence metadata

The complete `EvidenceRegistered.metadata_json` object for this family is:

```json
{
  "inline_predicate_attestation": {
    "schema": "magpie-inline-predicate-attestation-v0",
    "predicate_id": "sha256_claim_inline_bytes_equals_v0",
    "subject_claim_id": "claim-c",
    "scope_ref": "scope-s"
  }
}
```

The outer object contains exactly one key:

```text
inline_predicate_attestation
```

The nested object contains exactly four keys:

```text
schema
predicate_id
subject_claim_id
scope_ref
```

Every field is mandatory and every value is a JSON string. No optional
field, `null`, alias, alternate envelope, or sibling metadata is accepted.
The outer-only shape is intentional: unrelated evidence metadata may not
coexist in an attestation-binding attempt.

### Outer-envelope decision

`inline_predicate_attestation` is the exact v0 outer identity.

Alternatives were rejected:

- `verification_witness` is the historical byte-bearing v0 family. Reusing
  it would blur witness input with routing-only attestation.
- `predicate_attestation` is too broad: it would silently reserve an
  envelope for predicate families whose routing laws have not been
  reviewed.
- `claim_inline_predicate_attestation` repeats claim ownership in an
  evidence-side route without adding a binding discriminator. The exact
  `subject_claim_id` field already supplies claim routing.

The chosen name is a sibling of, never an alias for,
`verification_witness`.

### Inner-schema decision

The exact schema identity is:

```text
magpie-inline-predicate-attestation-v0
```

It is 38 ASCII bytes. It is compiled into the future resolver and compared
exactly after JSON unescaping. No normalization, case folding, alias,
prefix, suffix, version fallback, or registry lookup is permitted.

`magpie-verification-witness-v0` is not accepted. That historical schema
has a different proposition and carries `witness_hex`; it stays
parser-wise and semantically separate.

## Exact field law

### `schema`

- required JSON string;
- exactly `magpie-inline-predicate-attestation-v0`;
- compared from the decoded JSON string without normalization;
- no caller-selected schema object or parser registry.

Because the only accepted value is one compiled constant, this contract
adds no separate schema-length bound. A future parser can compare the
borrowed decoded token with the constant without materializing an
arbitrarily long mismatch.

### `predicate_id`

- required JSON string;
- non-empty;
- decoded UTF-8 length at most
  `MAX_PREDICATE_ID_BYTES_V0 = 64`;
- closed ASCII `[a-z0-9_]+` after the length check;
- finally byte-equal to the validated predicate identity resolved from
  the same replayed claim.

The accepted value for this v0 resolver is therefore exactly:

```text
sha256_claim_inline_bytes_equals_v0
```

The field inherits Ticket 0057 and Ticket 0058's resource law. A borrowed
count-only unescape pass rejects decoded byte 65 before any
resolver-owned allocation or copy attributable to the identifier,
character validation, binding comparison, receipt construction, or
statement construction. Before that decision, retained state is limited
to bounded non-identifier scalar decoding/counting and key-traversal
state containing no copied complete or partial identifier. Only a
successful decoded-length bound permits materializing the bounded
identifier for lexical and identity checks. Receipt construction occurs
only after every binding check succeeds.

A well-formed identifier that differs from the claim's validated
predicate identity is `PredicateBindingMismatch`. This resolver invents no
generic predicate registry and no second predicate-ID ontology.

### `subject_claim_id`

- required JSON string;
- non-empty;
- exact decoded-string equality with the resolver's requested `claim_id`;
- no normalization, prefix, namespace, wildcard, containment, or alias
  semantics.

No repository-wide claim-ID maximum exists. This contract adds none.
The parser may compare the borrowed decoded token directly with the query
key and must not materialize an arbitrarily long mismatching value merely
to compare it. An accepted value and its audit serialization may grow
linearly with the exact existing identifier.

### `scope_ref`

- required JSON string;
- non-empty;
- exact decoded-string equality with both the replayed evidence
  `scope_ref` and the replayed typed claim `scope_ref`;
- no inheritance, prefix, wildcard, containment, lattice, ontology,
  normalization, or alias semantics.

No repository-wide `scope_ref` maximum exists. This contract adds none.
As with `subject_claim_id`, a future parser may compare a borrowed decoded
token directly and need not materialize a mismatching routing string.

## Strict parsing and resource law

The future parser must validate one complete JSON document:

- the top-level value is an object;
- no trailing value or non-whitespace content;
- exact JSON string, number, literal, array, object, escape, Unicode,
  surrogate-pair, separator, and delimiter grammar;
- duplicate keys rejected at both levels;
- duplicate detection after JSON unescaping, including arbitrary unknown
  keys;
- unknown keys rejected at both levels;
- raw key spelling is never identity;
- decoded string values are compared exactly, with no Unicode
  normalization.

The parser records structural facts before selecting the frozen semantic
failure. JSON source-key order, map iteration, Serde encounter order, or
the first internal parser branch may not alter the result.

The replay substrate already owns the complete `metadata_json` string
before this resolver runs. This contract cannot retroactively cap or undo
that allocation. It adds no global event, metadata, container-depth, key
count, query-ID, claim-ID, evidence-ID, or scope limit. It also adds no
hidden depth failure. A future implementation may reuse or factor the
Ticket 0059 borrowed complete-JSON machinery, but it may not reuse the
historical allocating generic witness-string path in a way that violates
the predicate-ID pre-allocation law.

The exact field-specific resource decisions are:

| Material | v0 decision |
| --- | --- |
| `predicate_id` | inherited decoded UTF-8 maximum of 64 bytes; no complete or partial resolver-owned copy before the decoded-length bound succeeds |
| `schema` | no new bound; streaming exact comparison with one compiled constant |
| `subject_claim_id` | no new bound; required non-empty exact comparison with the requested claim ID |
| `scope_ref` | no new bound; required non-empty exact comparison with both replayed scopes |
| raw `metadata_json` | no new global bound; already replay-allocated input |
| requested `claim_id` / `evidence_id` | no new validation, normalization, truncation, hash, or bound |

## Evidence-kind decision

The exact required evidence kind is:

```text
evidence.evidence_kind == DeterministicVerification
```

Reuse is sound because `EvidenceKind` is a closed L0 vocabulary label while
actual resolver authority comes from exact same-replay parsing and binding.
The label alone grants nothing. No support or refutation policy cell is
activated by `Bound`.

No new `EvidenceKind`, L0 payload, format tag, policy-matrix cell, or golden
event is introduced.

The two metadata families remain fail-closed in both directions:

- the historical resolver accepts only outer `verification_witness` with
  schema `magpie-verification-witness-v0`; the new
  `inline_predicate_attestation` key is an unknown witness key and can
  never produce its `Matched` result;
- this resolver accepts only outer `inline_predicate_attestation` with
  schema `magpie-inline-predicate-attestation-v0`; a historical
  `verification_witness` object yields
  `UnknownAttestationMetadataKey`;
- an object carrying both outer keys is rejected by both families;
- no dispatch occurs from `DeterministicVerification` alone.

## Exact same-replay authority path

The sole future public resolver is:

```rust
impl StandingReplaySnapshot {
    pub fn resolve_inline_predicate_attestation_v0(
        &self,
        claim_id: &str,
        evidence_id: &str,
    ) -> InlinePredicateAttestationBindingOutcomeV0;
}
```

There is no edge ID and no edge input.

The legitimate construction path is:

```text
one successful LogReader verification
→ one private composite replay
→ one StandingReplaySnapshot (private construction)
→ one resolver call with claim_id and evidence_id
→ internal same-snapshot claim resolution
→ internal same-snapshot evidence re-fetch
→ strict attestation parse and exact bindings
→ one opaque InlinePredicateAttestationBindingOutcomeV0
```

The resolver internally re-fetches:

- the standing claim;
- the typed claim;
- the typed evidence.

No caller supplies a claim, evidence node, descriptor, parsed attestation,
predicate outcome, predicate receipt, binding outcome, binding receipt,
scope, predicate identity, authority object, callback, closure, alternate
snapshot, snapshot fragment, store, policy, edge, or content provider.

The exact query IDs are retained for audit attribution. Selection by query
ID is not authority substitution: the resolver always re-fetches the node
under that key from its own snapshot. A missing key produces a closed
failure.

## Relation to the Ticket 0059 evaluator

The future implementation must choose the private shared-resolution option:

```text
one private claim-inline resolution primitive
used internally by:
  Ticket 0059's public neutral evaluator
  the future attestation-binding resolver
```

The shared primitive receives only the selected
`&StandingReplaySnapshot` and `claim_id`. It re-fetches the standing and
typed claim, applies Ticket 0059's exact 33-reason resolution law through
the decoded-subject size guard, and returns a crate-private resolved value
or the exact `ClaimInlineSubjectResolutionFailureV0`.

The shared primitive necessarily resolves the claim-owned subject bytes and
expected operand so that Ticket 0059's exact resolution failures remain one
law. Those values are not attestation inputs or binding comparisons. The
neutral evaluator alone consumes them to perform the terminal SHA-256
comparison and create `DigestEqual` or `DigestUnequal`. The attestation
resolver consumes only the primitive's resolved claim ID, validated predicate
identity, and exact claim scope for routing; it does not inspect the resolved
subject or expected operand and does not compute, observe, branch on,
serialize, or expose the digest relation, expected digest, computed digest,
subject bytes, subject hex, canonical statement, or claim content hash.

This means:

```text
any claim that would reach DigestEqual    → equally eligible to bind
any claim that would reach DigestUnequal  → equally eligible to bind
any claim that reaches ResolutionFailed   → cannot bind
```

`DigestUnequal` is not a binding failure. `ResolutionFailed` is not
negative polarity.

The two rejected alternatives are:

1. Privately call the public evaluator and accept both terminal kinds. This
   is non-caller-substitutable, but it needlessly computes and observes the
   relation inside a routing-only resolver and creates an avoidable leakage
   branch.
2. Duplicate the Ticket 0059 parser and resolution law. This creates two
   drifting authorities and is prohibited.

Factoring the private primitive may not alter Ticket 0059's public API,
33-reason precedence, terminal behavior, canonical bytes, or pinned
vectors. If an implementation cannot preserve those surfaces exactly, it
must stop.

## Exact resolver procedure

One call performs these operations in order:

1. Preserve the exact requested `claim_id` and `evidence_id` for audit
   attribution; add no validation or normalization.
2. Invoke the private same-snapshot claim-resolution primitive. It
   re-fetches both claim tables and applies Ticket 0059's exact resolution
   law. Any failure becomes
   `ClaimPredicateResolutionFailed(<exact Ticket 0058 reason>)`.
3. Re-fetch the typed evidence by `evidence_id`; absence is
   `MissingEvidence`.
4. Require exact evidence kind `DeterministicVerification`; any other
   closed kind is `WrongEvidenceKind`.
5. Strictly parse the complete evidence metadata and apply the frozen
   outer, inner, type, schema, and lexical failure order below.
6. Compare attestation `predicate_id` with the validated claim predicate
   identity; mismatch is `PredicateBindingMismatch`.
7. Compare attestation `subject_claim_id` with the exact requested
   `claim_id`; mismatch is `ClaimBindingMismatch`.
8. Compare replayed evidence `scope_ref` with attestation `scope_ref`;
   mismatch is `EvidenceScopeBindingMismatch`.
9. Compare replayed typed claim `scope_ref` with attestation `scope_ref`;
   mismatch is `ClaimScopeBindingMismatch`.
10. Construct one private receipt from the successful replay/query basis
    and return opaque kind `Bound`.

The scope comparisons deliberately have a deterministic order:

| Attestation scope A | Evidence scope E | Claim scope C | Result |
| --- | --- | --- | --- |
| `A == E == C` | same | same | eligible for `Bound` |
| `A != E`, `A == C` | different | same as A | `EvidenceScopeBindingMismatch` |
| `A == E`, `A != C` | same as A | different | `ClaimScopeBindingMismatch` |
| `A != E`, `A != C` | different | different or equal to E | `EvidenceScopeBindingMismatch` first |

Scope failure is routing failure only. It is never falsity or polarity.

## `content_hash`, actor class, and summary

These fields are explicitly adjudicated:

```text
EvidenceRegistered.content_hash:
ignored by this resolver

EvidenceRegistered.actor_class:
no resolver-specific check; ignored after ordinary replay/L0 validity

EvidenceRegistered.summary:
ignored by this resolver
```

`EvidenceRegistered.content_hash` is historically asserted artifact-hash
material and may be empty. This contract does not define a canonical
attestation statement or say what artifact an attestation evidence node
must hash. Requiring or binding that field would silently invent a new
content law. Empty and populated valid values therefore produce the same
binding result and receipt when every ratified input is otherwise equal.

An actor-class label is asserted provenance classification, not resolver
authority. `AutomatedVerifier`, `HumanRoot`, or any other L0-valid actor
class neither creates nor blocks `Bound`.

Summary prose is not a routing field and cannot amend, override, or lend
meaning to the attestation.

## Edge and polarity exclusion

The resolver does not enumerate, select, fetch, parse, or inspect
`JustificationEdgeRecorded` nodes.

```text
supports edge exists       → not read
contradicts edge exists    → not read
both kinds exist           → not read
no edge exists             → not read

edge kind                  != binding
Bound                      has no positive polarity
Bound                      has no negative polarity
```

No edge ID or edge kind appears in the API, outcome, receipt, failure
payload, or canonical bytes. An edge cannot change a binding failure into
`Bound` or change `Bound` into a contribution.

Only a later separately ratified lane contract may combine:

```text
same-replay attestation binding
+ same-replay predicate relation
+ one replayed edge
```

That later lane must derive each input internally from one snapshot. It may
not accept a caller-created outcome or receipt from this contract or Ticket
0059.

## Opaque outcome and receipt

The public classification vocabulary is:

```text
InlinePredicateAttestationBindingOutcomeKindV0:
    Bound
    ResolutionFailed
```

The authority-bearing
`InlinePredicateAttestationBindingOutcomeV0` is an opaque public struct
with a private representation. The resolver alone constructs it.

Read-only inspection is limited to:

```rust
pub fn kind(
    &self,
) -> InlinePredicateAttestationBindingOutcomeKindV0;

pub fn requested_claim_id(&self) -> &str;

pub fn requested_evidence_id(&self) -> &str;

pub fn receipt(
    &self,
) -> Option<&InlinePredicateAttestationBindingReceiptV0>;

pub fn failure(
    &self,
) -> Option<&InlinePredicateAttestationBindingFailureV0>;

pub fn canonical_bytes(&self) -> Vec<u8>;
```

For `Bound`, the requested IDs are borrowed from the receipt's sole
`claim_id` and `evidence_id`; no duplicate copies exist outside the
receipt. For `ResolutionFailed`, the private outcome retains both exact
query keys as attribution only. A failed query key does not prove that a
node existed.

The successful receipt contains exactly five fields in this order:

```text
schema
predicate_id
claim_id
evidence_id
scope_ref
```

Its complete read-only inspection surface is:

```rust
pub fn schema(&self) -> &str;
pub fn predicate_id(&self) -> &str;
pub fn claim_id(&self) -> &str;
pub fn evidence_id(&self) -> &str;
pub fn scope_ref(&self) -> &str;
```

Their meanings are:

| Field | Audit meaning |
| --- | --- |
| `schema` | exact attestation schema successfully parsed |
| `predicate_id` | exact claim-validated and attestation-bound predicate identity |
| `claim_id` | successful query identity and replayed claim binding |
| `evidence_id` | successful query identity and replayed evidence binding |
| `scope_ref` | exact scope shared by attestation, evidence, and typed claim |

No sixth field is needed. The compiled resolver path already fixes the
outer envelope and evidence kind; duplicating them would not add audit
identity. The snapshot owns no verified-prefix field. Content hash,
actor class, summary, edge material, relation, subject, digest,
contribution, policy, and standing fields are excluded.

The outcome and receipt have:

- private fields and private construction;
- no public constructor;
- no struct- or tuple-literal construction;
- no `Deserialize`;
- no `Default`;
- no `From` or `TryFrom`;
- no public mutation;
- no consuming `into_receipt`, `into_failure`, or other outcome
  deconstruction/reinjection method;
- no public `Debug` representation;
- no caller-selected classification;
- no authority reinjection.

`Clone` and `Serialize` may be implemented for audit convenience only.
An outcome `Serialize` implementation must delegate to the exact canonical
wire view below and may not introduce a second outcome representation.
An owned clone is an audit copy of one legitimately obtained exact value,
not a caller-selected deconstruction or authority input; no resolver, lane,
policy, or composer accepts it. The fieldless outcome kind and closed
failure vocabulary may remain caller-constructible non-authority
classifications; they carry no receipt and cannot convert into an outcome.

No public parsed-attestation type exists. No public API accepts raw
attestation fields, a parsed descriptor, an outcome, a receipt, or
canonical bytes as authority.

## Closed failure vocabulary and exact order

The closed public failure vocabulary is:

```text
InlinePredicateAttestationBindingFailureV0:
    ClaimPredicateResolutionFailed(ClaimInlineSubjectResolutionFailureV0)
    MissingEvidence
    WrongEvidenceKind
    MalformedAttestationMetadata
    DuplicateAttestationMetadataKey
    UnknownAttestationMetadataKey
    MissingInlinePredicateAttestation
    WrongTypeInlinePredicateAttestation
    DuplicateAttestationField
    UnknownAttestationField
    MissingSchema
    WrongTypeSchema
    UnknownSchema
    MissingPredicateId
    WrongTypePredicateId
    EmptyPredicateId
    PredicateIdTooLong
    InvalidPredicateId
    MissingSubjectClaimId
    WrongTypeSubjectClaimId
    EmptySubjectClaimId
    MissingScopeRef
    WrongTypeScopeRef
    EmptyScopeRef
    PredicateBindingMismatch
    ClaimBindingMismatch
    EvidenceScopeBindingMismatch
    ClaimScopeBindingMismatch
```

One resolver call returns at most one failure, selected in the exact order
shown. Stage 1 is itself the exact Ticket 0058/Ticket 0059 33-reason order:

```text
 1. missing_claim
 2. missing_typed_claim
 3. malformed_metadata
 4. missing_claim_domain
 5. wrong_type_claim_domain
 6. duplicate_claim_metadata_key
 7. unknown_claim_domain
 8. claim_domain_mismatch
 9. unknown_claim_metadata_key
10. missing_inline_subject_descriptor
11. wrong_type_inline_subject_descriptor
12. duplicate_descriptor_key
13. unknown_descriptor_key
14. missing_schema
15. wrong_type_schema
16. unknown_schema
17. missing_predicate_id
18. wrong_type_predicate_id
19. empty_predicate_id
20. predicate_id_too_long
21. invalid_predicate_id
22. unknown_predicate_id
23. missing_expected_sha256
24. wrong_type_expected_sha256
25. invalid_expected_sha256
26. missing_subject_hex
27. wrong_type_subject_hex
28. subject_hex_too_long
29. invalid_subject_hex
30. statement_binding_mismatch
31. missing_claim_content_hash
32. claim_content_hash_mismatch
33. decoded_subject_too_large
```

The nested typed reason is preserved rather than flattened or
reinterpreted. This gives an exact audit reason while keeping all claim
resolution authority in one implementation. In particular:

- `MissingClaim` and `MissingTypedClaim` remain distinguishable;
- `ResolutionFailed` never becomes `DigestUnequal`;
- `DigestUnequal` never appears as a failure;
- the binding resolver does not invent a second spelling or ordering for
  claim-side failures.

After the nested stage, the exact outer stage order is:

```text
 2. MissingEvidence
 3. WrongEvidenceKind
 4. MalformedAttestationMetadata
 5. DuplicateAttestationMetadataKey
 6. UnknownAttestationMetadataKey
 7. MissingInlinePredicateAttestation
 8. WrongTypeInlinePredicateAttestation
 9. DuplicateAttestationField
10. UnknownAttestationField
11. MissingSchema
12. WrongTypeSchema
13. UnknownSchema
14. MissingPredicateId
15. WrongTypePredicateId
16. EmptyPredicateId
17. PredicateIdTooLong
18. InvalidPredicateId
19. MissingSubjectClaimId
20. WrongTypeSubjectClaimId
21. EmptySubjectClaimId
22. MissingScopeRef
23. WrongTypeScopeRef
24. EmptyScopeRef
25. PredicateBindingMismatch
26. ClaimBindingMismatch
27. EvidenceScopeBindingMismatch
28. ClaimScopeBindingMismatch
```

`MalformedAttestationMetadata` means the full evidence metadata is not one
complete JSON object. Duplicate wins over unknown; unknown wins over a
simultaneously missing required envelope. The same structural order applies
inside the envelope before individual field validation. This makes old
`verification_witness` metadata fail exactly as
`UnknownAttestationMetadataKey`, and makes any byte- or polarity-smuggling
field fail as an unknown field rather than being ignored.

All failures are routing-resolution failures only. None is negative
polarity, contradiction, refutation, invalidation, or evidence of falsity.

## Canonical outcome audit JSON v0

The exact profile identity is:

```text
magpie-inline-predicate-attestation-binding-outcome-json-v0
```

It is a compiled encoder identity, not a receipt field. The encoder uses a
private fixed-field wire view and direct typed `serde_json::to_vec`. It does
not pass through `serde_json::Value`, a map, JCS, RFC 8785, a generic
canonical-JSON registry, or Magpie L0.

The outer object is adjacently tagged with exact key order:

```text
outcome
details
```

`Bound` has:

```text
{"outcome":"bound","details":{"receipt":{RECEIPT}}}
```

The receipt field order is exactly:

```text
schema
predicate_id
claim_id
evidence_id
scope_ref
```

An ordinary failure has exact detail order:

```text
claim_id
evidence_id
reason
```

```text
{"outcome":"resolution_failed","details":{"claim_id":"CLAIM_ID","evidence_id":"EVIDENCE_ID","reason":"FAILURE"}}
```

A nested claim-resolution failure has exact detail order:

```text
claim_id
evidence_id
reason
claim_reason
```

```text
{"outcome":"resolution_failed","details":{"claim_id":"CLAIM_ID","evidence_id":"EVIDENCE_ID","reason":"claim_predicate_resolution_failed","claim_reason":"TICKET_0058_FAILURE"}}
```

The ordinary failure spellings are:

```text
MissingEvidence                    -> "missing_evidence"
WrongEvidenceKind                  -> "wrong_evidence_kind"
MalformedAttestationMetadata       -> "malformed_attestation_metadata"
DuplicateAttestationMetadataKey    -> "duplicate_attestation_metadata_key"
UnknownAttestationMetadataKey      -> "unknown_attestation_metadata_key"
MissingInlinePredicateAttestation  -> "missing_inline_predicate_attestation"
WrongTypeInlinePredicateAttestation
                                   -> "wrong_type_inline_predicate_attestation"
DuplicateAttestationField          -> "duplicate_attestation_field"
UnknownAttestationField            -> "unknown_attestation_field"
MissingSchema                      -> "missing_schema"
WrongTypeSchema                    -> "wrong_type_schema"
UnknownSchema                      -> "unknown_schema"
MissingPredicateId                 -> "missing_predicate_id"
WrongTypePredicateId               -> "wrong_type_predicate_id"
EmptyPredicateId                   -> "empty_predicate_id"
PredicateIdTooLong                 -> "predicate_id_too_long"
InvalidPredicateId                 -> "invalid_predicate_id"
MissingSubjectClaimId              -> "missing_subject_claim_id"
WrongTypeSubjectClaimId            -> "wrong_type_subject_claim_id"
EmptySubjectClaimId                -> "empty_subject_claim_id"
MissingScopeRef                    -> "missing_scope_ref"
WrongTypeScopeRef                  -> "wrong_type_scope_ref"
EmptyScopeRef                      -> "empty_scope_ref"
PredicateBindingMismatch           -> "predicate_binding_mismatch"
ClaimBindingMismatch               -> "claim_binding_mismatch"
EvidenceScopeBindingMismatch       -> "evidence_scope_binding_mismatch"
ClaimScopeBindingMismatch          -> "claim_scope_binding_mismatch"
```

The `claim_reason` spellings are exactly the 33 strings listed in the
nested failure order above.

The escaping law is identical to Ticket 0058's adjacent audit profile:
compact UTF-8 JSON; fixed declaration order; `"` and `\` escaped as
`\"` and `\\`; short escapes `\b`, `\t`, `\n`, `\f`, `\r`; lowercase
`\u00xx` for every other U+0000-U+001F control; direct UTF-8 for every
other Unicode scalar; no Unicode normalization; no escaped `/`; no
insignificant whitespace, BOM, or trailing newline.

The following one-line code-block contents are the complete canonical
vectors. Markdown line endings are not part of the vectors.

`Bound` — 216 bytes, SHA-256
`9ecd2e4c3c4f291fb8d4c5c6a4ceac590a5cc5a516c088766979596705d86bc5`:

```json
{"outcome":"bound","details":{"receipt":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","claim_id":"claim-c","evidence_id":"evidence-e","scope_ref":"scope-s"}}}
```

`ResolutionFailed(MissingEvidence)` — 125 bytes, SHA-256
`968bf85313eef6e2700fb9e3780bfa1b9f88c31f1d6a58898901843b20a10fe6`:

```json
{"outcome":"resolution_failed","details":{"claim_id":"claim-c","evidence_id":"evidence-missing","reason":"missing_evidence"}}
```

`ResolutionFailed(ClaimPredicateResolutionFailed(MissingClaim))` —
173 bytes, SHA-256
`2a341009683cc4bc9db94a5d631898a0ea5b1efa7701e8e23ac0f93831b91190`:

```json
{"outcome":"resolution_failed","details":{"claim_id":"claim-missing","evidence_id":"evidence-e","reason":"claim_predicate_resolution_failed","claim_reason":"missing_claim"}}
```

Any byte change requires a new profile identity. These bytes are derived
audit material only. No resolver, evaluator, lane, policy, or composer may
deserialize or accept them as authority.

## Hostile cases

Unless a row says otherwise, the claim itself is a valid Ticket 0059
claim, the requested IDs exist, and the three scopes are initially equal.

| Case | Exact result |
| --- | --- |
| historical `verification_witness` metadata presented to the new resolver | `ResolutionFailed(UnknownAttestationMetadataKey)`; old family never cross-dispatches |
| new attestation metadata presented to the historical resolver | never historical `Matched`; its exact outer parser rejects `inline_predicate_attestation` as an unknown witness key when the old lane reaches parsing |
| metadata carries both `verification_witness` and `inline_predicate_attestation` | rejected by both exact outer parsers |
| nested attestation carries `witness_hex` | `UnknownAttestationField` |
| nested attestation carries `subject_hex` or raw subject bytes | `UnknownAttestationField` |
| nested attestation carries `expected_sha256` or `computed_sha256` | `UnknownAttestationField` |
| nested attestation carries a digest relation, predicate outcome, or predicate receipt | `UnknownAttestationField` |
| nested attestation carries support/refutation contribution, edge kind/ID, policy identity, standing, contradiction, invalidation, or caller-selected authority data | `UnknownAttestationField` |
| attestation schema is historical or unknown | `UnknownSchema` |
| attestation predicate ID differs from the validated claim predicate | `PredicateBindingMismatch` |
| attestation subject claim differs from the requested claim | `ClaimBindingMismatch` |
| evidence scope differs, claim scope equals attestation scope | `EvidenceScopeBindingMismatch` |
| evidence scope equals attestation scope, claim scope differs | `ClaimScopeBindingMismatch` |
| evidence and claim scopes both differ from attestation scope | `EvidenceScopeBindingMismatch` first |
| evidence and claim scopes equal each other but differ from attestation scope | `EvidenceScopeBindingMismatch` first |
| evidence kind is any closed value other than `DeterministicVerification` | `WrongEvidenceKind` before metadata parsing |
| one or more `supports` edges exist | unchanged; edges are not read |
| one or more `contradicts` edges exist | unchanged; edges are not read |
| both edge kinds exist | unchanged; edges are not read |
| no edge exists | unchanged; no edge is required |
| the same claim would evaluate `DigestEqual` | equally eligible for `Bound`; relation not computed or exposed here |
| the same claim would evaluate `DigestUnequal` | equally eligible for `Bound`; never `Refuted` |
| claim resolution produces any Ticket 0059 `ResolutionFailed` reason | `ClaimPredicateResolutionFailed` with that exact nested typed reason; no evidence parsing |
| caller supplies a predicate outcome from this or another snapshot | no such API; caller outcome is never an input |
| caller supplies a parsed attestation, binding outcome, or receipt | no such API; private resolver construction only |
| caller serializes and replays an attestation result | bytes are audit material; no deserializer or authority-input API |
| keys differ only before JSON unescaping, such as `predicate_id` and `pred\u0069cate_id` | post-unescape duplicate failure at the applicable outer/inner duplicate stage |
| duplicate unknown keys plus an unknown field | duplicate failure first |
| unknown fields attempt to smuggle bytes or polarity | exact outer or inner unknown failure; never ignored |
| same evidence ID and claim ID resolved repeatedly in one snapshot | equal outcomes and byte-identical canonical audit output |
| same valid evidence is routed to another otherwise valid claim | `ClaimBindingMismatch` |
| same claim is routed through two distinct valid evidence nodes | each may independently be `Bound` with its own receipt `evidence_id`; no count, amplification, contribution, or standing effect |
| evidence `content_hash` is empty | no effect |
| evidence `content_hash` is valid and populated | no effect; not copied into receipt |
| actor class changes among L0-valid values | no resolver effect |
| summary prose changes | no resolver effect |
| escaped `subject_claim_id` or `scope_ref` decodes exactly to the replayed value | accepted as the same decoded string; canonical output uses the replay/query value under the fixed escaping law |
| long `subject_claim_id`, scope, or query ID | no new bound; exact comparison and linear audit size |
| decoded `predicate_id` is 65 bytes | `PredicateIdTooLong` before identifier copy or binding |
| decoded `predicate_id` is within 64 bytes but outside `[a-z0-9_]+` | `InvalidPredicateId` |
| metadata is malformed, trailing, empty, an array, scalar, or `null` | `MalformedAttestationMetadata` |
| metadata contains only an unknown outer key | `UnknownAttestationMetadataKey` before the simultaneously missing-envelope failure |

## What `Bound` proves

Relative to the externally selected Magpie verifying key and this compiled
resolver contract, `Bound` proves only:

- the exact claim and evidence IDs resolved from one private verified
  snapshot;
- the claim successfully resolved through Ticket 0059's exact claim-only
  resolution law;
- the evidence kind was exactly `DeterministicVerification`;
- the exact new attestation envelope and schema parsed strictly;
- the attestation predicate ID equalled the validated claim predicate;
- the attestation subject claim ID equalled the requested claim; and
- attestation, evidence, and typed claim scopes were exactly equal.

It does not prove or report:

- `DigestEqual` or `DigestUnequal`;
- support, contradiction, refutation, settlement, or standing;
- contribution eligibility or admission;
- any edge's existence, kind, target, scope, or authority;
- evidence content-hash correctness;
- actor authority;
- summary truth;
- external artifact identity, acquisition, origin, or contents;
- verified-prefix provenance not owned by the snapshot;
- currentness, supersession, invalidation, or contradiction debt;
- publication authority.

## Frozen non-effects

This contract changes none of:

- `magpie-core-v1`;
- `docs/FORMAT.md`;
- payload tags or field order;
- `EvidenceKind` vocabulary;
- edge-kind vocabulary;
- actor classes;
- golden vectors;
- existing deterministic-verifier traces or canonical bytes;
- Ticket 0059 evaluator behavior, failures, receipts, or vectors;
- support ceilings;
- refutation ceilings;
- support-context requirements;
- standing v0, v1, v2, or v3;
- aggregation or origin-group policy;
- contradiction debt;
- invalidation;
- supersession or currentness;
- `EpistemicGate`;
- writer, loader, CAS, filesystem, network, clock, plugin, callback, or
  Deadbolt behavior.

`Bound` is standing-inert.

## Explicitly unratified

This contract ratifies no:

- Rust implementation or public runtime surface beyond the future API
  contract above;
- public parsed-attestation type;
- predicate relation;
- support or refutation lane;
- contribution or admission rule;
- edge composition;
- policy identity or policy v4;
- standing result;
- conflict or precedence law;
- contradiction debt, invalidation, supersession, or currentness rule;
- canonical evidence statement or evidence `content_hash` law;
- actor-class authority requirement;
- verified-prefix receipt field;
- new evidence kind, payload, L0 tag, L0 format profile, or golden event;
- generic predicate, attestation, or authority registry;
- external-object, filesystem, CAS, loader, network, or acquisition
  authority.

If a later implementation cannot preserve routing-only binding without
simultaneously defining polarity, contribution, or standing, it must stop.

## Implemented runtime boundary

Ticket 0061 implements only:

- one private shared claim-resolution primitive factored without changing
  Ticket 0059 behavior;
- one strict borrowed attestation parser;
- the snapshot-only resolver method;
- opaque private-construction outcome and receipt types;
- the closed failure vocabulary and exact canonical audit encoder;
- focused hostile, compile-fail, vector, compatibility, and standing-inertia
  tests;
- narrow documentation status handoffs.

It does not add an edge input, lane, support/refutation meaning, contribution,
standing policy, policy identity, conflict rule, L0 change, format change,
fixture rewrite, dependency, I/O, or ambient authority.

The remaining sequence is:

```text
Ticket 0060: routing-only attestation-binding contract — ratified
→ Ticket 0061: routing-only attestation-binding implementation —
  implemented in this candidate patch
→ support/refutation lane contract(s)
→ corresponding standing-inert lane/input implementation(s)
→ direct-refutation policy contract
→ direct-refutation runtime
→ contradiction debt, invalidation, supersession/currentness, ...
```

## Reviewer checklist

1. The complete evidence metadata object has only outer
   `inline_predicate_attestation`.
2. The inner schema is exactly
   `magpie-inline-predicate-attestation-v0`; the historical witness schema
   remains separate.
3. All four inherited routing fields are mandatory strings and no fifth
   field is accepted.
4. Duplicate and unknown keys fail at both levels after JSON unescaping,
   with the exact first-failure order.
5. `predicate_id` inherits the 64-decoded-byte pre-allocation law and
   closed alphabet; no generic ID ontology is invented.
6. Claim IDs and scopes remain non-empty opaque exact-match strings with
   no new limits or normalization.
7. Evidence kind is exactly `DeterministicVerification`, but the label
   alone grants no authority and no policy cell activates.
8. Old and new metadata families refuse each other in both directions.
9. One `StandingReplaySnapshot` internally re-fetches both claim tables and
   typed evidence; no caller supplies authority objects or another
   snapshot.
10. The future implementation factors one private shared claim-resolution
    primitive; it neither calls a caller-supplied evaluator output nor
    duplicates Ticket 0059's parser law.
11. `DigestEqual` and `DigestUnequal` are equally binding-eligible;
    `ResolutionFailed` prevents binding and is never negative polarity.
12. No edge is an input or receipt field, and `Bound` has no polarity.
13. Evidence `content_hash`, actor class, and summary have their explicit
    non-authority decisions.
14. The opaque outcome and five-field receipt expose no construction,
    reclassification, deserialization, mutation, consuming deconstruction,
    debug, or reinjection path; audit-only cloning has no accepting authority
    sink.
15. The nested Ticket 0058 failure is preserved exactly and the outer
    28-stage order is complete.
16. The canonical profile, shapes, key orders, strings, escaping law,
    lengths, and SHA-256 vectors are exact.
17. Every hostile case has a closed routing-only answer.
18. No runtime, policy, polarity, standing, L0, format, fixture, Cargo,
    release, CI, or GitHub surface changes.
