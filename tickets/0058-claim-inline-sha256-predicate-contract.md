# Ticket 0058: Claim-inline SHA-256 predicate contract

## 1. Exact base

```text
67a87e01575a9e9930ba3dc1ec52b95f963b6136
```

That commit is the merge of PR #72
(`docs: ratify claim-inline subject binding v0`) into `main`.

## 2. Branch, commits and PR title

Suggested branch:

```text
agent/claim-inline-sha256-predicate-contract
```

Suggested commit title and draft PR title:

```text
docs: ratify claim-inline SHA-256 predicate v0
```

`AGENTS.md` is binding. K3 does not commit, push, open or edit a PR, mark
readiness, merge, resolve review threads, or enable auto-merge. The user
retains sole publication and merge authority.

## 3. Documentation-only classification

```text
correctness:
ratifies one neutral claim-inline SHA-256 predicate contract over the
Ticket 0057 subject-binding substrate; no runtime

authority:
documentation only; no authority path changes

standing:
unchanged

policy:
none ratified (no policy identity introduced)

predicate:
one new ratified predicate identity,
sha256_claim_inline_bytes_equals_v0, contract only

runtime:
unchanged

tests:
unchanged

fixtures:
unchanged

canonical bytes:
unchanged

L0:
unchanged

dependencies:
unchanged

positive effect:
the first two-sided-decidable, polarity-neutral evaluation substrate
over claim-fixed subject bytes is frozen before any support or
refutation policy exists; the Ticket 0056 substitution class stays
closed by construction
```

This ticket implements no runtime capability. It changes no Rust, tests,
fixtures, formats, vectors, public APIs, policy matrices, standing
behavior, writer behavior, Deadbolt behavior, origin admission, or
aggregation.

## 4. Exact changed-path allowlist

Exactly these five paths may change:

```text
README.md
docs/design/claim-inline-subject-binding-v0.md  (narrow handoff/status only)
docs/design/claim-inline-sha256-predicate-v0.md  (new)
docs/design/standing-view-evidence-ceilings.md  (narrow living-status handoff)
tickets/0058-claim-inline-sha256-predicate-contract.md  (new)
```

No other path is permitted. Specifically prohibited: Rust source, tests,
fixtures, Cargo manifests, the lockfile, CI, release metadata, changelog,
`docs/FORMAT.md`, `AGENTS.md`, golden vectors, the Python verifier,
historical tickets including Tickets 0056 and 0057,
`docs/design/standing-aggregation-independence-groups.md` (see section
15), generated files, and GitHub metadata.

The subject-binding design note may receive only a narrow handoff/status
clarification; its ratified law is not altered. Making explicit that
pre-resolver upstream allocation is outside the resolver-owned
predicate-ID resource guarantee is a scope clarification of its
existing compiled-caps boundary, not a relaxation or a new
implementation law.

## 5. The predicate law

Ratified in full in
[`docs/design/claim-inline-sha256-predicate-v0.md`](../docs/design/claim-inline-sha256-predicate-v0.md);
this ticket and that note agree exactly:

```text
exact verified replay
+ one exact claim selected by claim ID
+ strict Ticket 0057 outer-envelope parsing
+ strict inline descriptor parsing
+ compiled predicate-identity equality
+ descriptor-derived canonical statement
+ exact three-way statement equality (direct byte comparison)
+ separate claim content-hash recomputation
+ subject bytes decoded only from the claim, bounded
+ one compiled predicate identity
→ one deterministic standing-inert predicate evaluation
```

The exact proposition is:

```text
SHA-256(hex_decode_strict_lowercase(
    subject_hex from the strictly resolved replayed claim))
==
hex_decode_strict_lowercase(
    expected_sha256 from that same claim's descriptor)
```

The proposition ranges over the decoded subject bytes only — never the
encoded hex characters, never the statement bytes. `subject_hex` is the
claim-owned encoded subject operand; `expected_sha256` is the
claim-owned expected terminal digest operand. The expected operand does
not independently commit to or identify the subject: a well-formed
`DigestUnequal` outcome is the legitimate relation in which it differs
from the independently computed subject digest. The claim author fixes
both operands at assertion time, the evaluator reads both from the
replayed claim, and neither is caller-supplied evaluation input. The
canonical statement injectively records the complete descriptor and
both operands. `ClaimAssertedV2.content_hash` commits to the exact UTF-8
statement bytes, is recomputed separately, and never enters the
terminal comparison.

The proposition is two-sided decidable after successful resolution:
given one valid decoded byte string and one valid 32-byte expected
digest, exactly one of equality or inequality holds. The outcome proves
only the digest relation. SHA-256 is not injective; digest equality
never becomes unique object identity, and digest inequality never
becomes falsity of anything beyond the named relation.

## 6. Exact predicate identity

```text
sha256_claim_inline_bytes_equals_v0
```

New identity, as Ticket 0056's blocker and Ticket 0057's versioning
boundary require: `sha256_bytes_equals_v0` keeps its evidence-local
positive meaning and is not reinterpreted, aliased, or extended. The
identity is 35 ASCII bytes of closed `[a-z0-9_]+`, within the compiled
`MAX_PREDICATE_ID_BYTES_V0 = 64` bound. It is compiled into the future
evaluator as a constant; a descriptor `predicate_id` that parses cleanly
but is not byte-equal to the constant fails `UnknownPredicateId` before
canonical statement construction, mirroring the existing v0 checker's
compiled-identity comparison.

## 7. Exact neutral outcome vocabulary

The closed, polarity-neutral outcome classification vocabulary is the
fieldless public enum:

```text
Sha256ClaimInlineBytesPredicateOutcomeKindV0:
    DigestEqual
    DigestUnequal
    ResolutionFailed
```

The authority-bearing outcome is an opaque public struct
`Sha256ClaimInlineBytesPredicateOutcomeV0` with a private
representation and no public constructor, no struct- or tuple-literal
construction, no `Deserialize`, no `Default`, no `From`/`TryFrom`, and
no public mutation. Its only public semantic inspection accessors are
read-only:
`kind()`, `requested_claim_id() -> &str`,
`receipt() -> Option<&Sha256ClaimInlineBytesPredicateReceiptV0>`, and
`failure() -> Option<&ClaimInlineSubjectResolutionFailureV0>`. Every
outcome privately retains the exact `claim_id` query key supplied to
the evaluator. `requested_claim_id()` borrows that exact value for all
three kinds, without normalization or replacement. For `DigestEqual`
and `DigestUnequal` it borrows the `claim_id` already retained in the
receipt; no duplicate claim ID exists outside the receipt. For
`ResolutionFailed` it borrows privately retained query attribution
stored with the failure reason. That value does not assert that a
replayed claim existed and is not replay provenance, standing
authority, or proof of successful resolution.

An outcome of kind `DigestEqual` or `DigestUnequal` carries exactly one
receipt. An outcome of kind `ResolutionFailed` carries the exact
requested claim-ID attribution plus exactly one closed failure reason,
and no receipt. The evaluator alone creates the complete outcome; no
caller can construct an outcome or failure payload, reclassify it,
replace its requested claim ID, move a receipt between outcomes,
deserialize, default, mutate, or re-present an outcome as authority. The
fieldless kind enum and the closed failure enum are caller-constructible
classification vocabulary only: they carry no receipt or query
attribution, cannot be converted into an outcome, and are never accepted
as authority. Reusing the borrowed requested ID as an ordinary query
key is permitted but never conveys claim existence or authority.
Serialization is deterministic and one-way through the exact canonical
profile below. The future `Serialize` implementation borrows the outcome
and emits audit material only; cloning or serializing any legitimately
obtained value never creates an authority input.

### Canonical outcome audit JSON v0

The canonicalization profile is exactly
`magpie-claim-inline-sha256-predicate-outcome-json-v0`. It is a
compiled future encoder identity, not a ninth receipt field. The
authority-bearing opaque outcome remains privately represented; a
private fixed-field wire view emits its audit bytes directly with
`serde_json::to_vec`, following Magpie's existing purpose-built derived
audit convention.

This deliberately reuses the exact `outcome` / `details` adjacent
tagging and direct typed `serde_json::to_vec` convention of the
origin-admission and admitted-contribution audits. Unlike a unit
variant, each outcome here has an explicit `details` payload: a receipt
for either terminal relation or exact requested claim-ID attribution
plus a reason for resolution failure.

The bytes are one compact UTF-8 JSON object with exact adjacent tagging:

```text
DigestEqual:
{"outcome":"digest_equal","details":{"receipt":{RECEIPT}}}

DigestUnequal:
{"outcome":"digest_unequal","details":{"receipt":{RECEIPT}}}

ResolutionFailed:
{"outcome":"resolution_failed","details":{"claim_id":"CLAIM_ID","reason":"FAILURE"}}
```

The outer key order is always `outcome`, then `details`. `outcome` is exactly one of
`digest_equal`, `digest_unequal`, or `resolution_failed`.

For `digest_equal` and `digest_unequal`, `details` contains exactly one
key, `receipt`. The receipt is an object whose eight keys and JSON
string values appear in this exact order:

```text
predicate_schema
predicate_id
claim_id
scope_ref
canonical_statement
claim_content_hash
expected_sha256
computed_sha256
```

For `resolution_failed`, `details` contains exactly two mandatory keys
in this exact order:

```text
claim_id
reason
```

`claim_id` is the exact query key supplied to
`evaluate_sha256_claim_inline_bytes_equals_v0`, preserved without
rejection, normalization, hashing, truncation, replacement, or other
canonicalization. It is query attribution only. In particular,
`MissingClaim` plus this field does not assert that a replayed claim
existed; the field is not replay provenance, standing authority, or
proof of resolution. `reason` is the exact snake-case encoding paired
with the closed failure vocabulary:

```text
MissingClaim                         -> "missing_claim"
MissingTypedClaim                    -> "missing_typed_claim"
MalformedMetadata                    -> "malformed_metadata"
MissingClaimDomain                   -> "missing_claim_domain"
WrongTypeClaimDomain                 -> "wrong_type_claim_domain"
DuplicateClaimMetadataKey            -> "duplicate_claim_metadata_key"
UnknownClaimDomain                   -> "unknown_claim_domain"
ClaimDomainMismatch                  -> "claim_domain_mismatch"
UnknownClaimMetadataKey              -> "unknown_claim_metadata_key"
MissingInlineSubjectDescriptor       -> "missing_inline_subject_descriptor"
WrongTypeInlineSubjectDescriptor     -> "wrong_type_inline_subject_descriptor"
DuplicateDescriptorKey               -> "duplicate_descriptor_key"
UnknownDescriptorKey                 -> "unknown_descriptor_key"
MissingSchema                        -> "missing_schema"
WrongTypeSchema                      -> "wrong_type_schema"
UnknownSchema                        -> "unknown_schema"
MissingPredicateId                   -> "missing_predicate_id"
WrongTypePredicateId                 -> "wrong_type_predicate_id"
EmptyPredicateId                     -> "empty_predicate_id"
PredicateIdTooLong                   -> "predicate_id_too_long"
InvalidPredicateId                   -> "invalid_predicate_id"
UnknownPredicateId                   -> "unknown_predicate_id"
MissingExpectedSha256                -> "missing_expected_sha256"
WrongTypeExpectedSha256              -> "wrong_type_expected_sha256"
InvalidExpectedSha256                -> "invalid_expected_sha256"
MissingSubjectHex                    -> "missing_subject_hex"
WrongTypeSubjectHex                  -> "wrong_type_subject_hex"
SubjectHexTooLong                    -> "subject_hex_too_long"
InvalidSubjectHex                    -> "invalid_subject_hex"
StatementBindingMismatch             -> "statement_binding_mismatch"
MissingClaimContentHash              -> "missing_claim_content_hash"
ClaimContentHashMismatch             -> "claim_content_hash_mismatch"
DecodedSubjectTooLarge               -> "decoded_subject_too_large"
```

There is no optional or omitted-field ambiguity. Both outer keys are
always present. A terminal relation has `receipt` and no sibling
`claim_id` or `reason`; its claim ID remains solely inside the
eight-field receipt. A resolution failure has `claim_id` then `reason`
and never `receipt`. Both failure fields are present and non-null. No
extra key is permitted, and no other key or declaration order is
canonical.

The encoder law is exact: fixed object-field declaration order; compact
UTF-8 JSON; `"` and `\` escaped as `\"` and `\\`; the short escapes
`\b`, `\t`, `\n`, `\f`, and `\r`; lowercase `\u00xx` for every other
U+0000-U+001F control; direct UTF-8 for every other Unicode scalar; no
Unicode normalization; no escaped `/`; no insignificant whitespace,
BOM, or trailing newline. The encoder serializes the private typed wire
view directly and never passes through `serde_json::Value`, a map, JCS,
RFC 8785, a generic canonical-JSON registry, or Magpie L0.

No compiled maximum for claim IDs exists in the repository:
`ClaimAssertedV2` requires only a non-empty `claim_id`. This contract
therefore invents no `MAX_CLAIM_ID_BYTES`, adds no failure reason, and
performs no query-ID rejection or normalization. The opaque outcome
privately preserves the exact already-existing query string. Canonical
failure serialization size grows linearly with that string under the
fixed escaping law; this audit attribution introduces no second subject
or predicate authority source.

The following three one-line code-block contents are the complete
canonical byte vectors; the Markdown line ending after each is not part
of the vector.

`DigestEqual` — 639 bytes, SHA-256
`d0c6a0f771600c34aad02264f722f915bb9093ac28c4489d1369585c402a6f5a`:

```json
{"outcome":"digest_equal","details":{"receipt":{"predicate_schema":"magpie-machine-predicate-inline-bytes-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","claim_id":"claim-c","scope_ref":"scope-s","canonical_statement":"magpie-machine-predicate-inline-bytes-v0:sha256_claim_inline_bytes_equals_v0:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad:616263","claim_content_hash":"aa1b8f1c339bc8e53c7db9d432e85073cc1495284279ba1b2474b74d26381223","expected_sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad","computed_sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"}}}
```

`DigestUnequal` — 641 bytes, SHA-256
`cced955ed7822888e3d2e0a3f91263ec905d2c629d5649003eaabd3360f81291`:

```json
{"outcome":"digest_unequal","details":{"receipt":{"predicate_schema":"magpie-machine-predicate-inline-bytes-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","claim_id":"claim-c","scope_ref":"scope-s","canonical_statement":"magpie-machine-predicate-inline-bytes-v0:sha256_claim_inline_bytes_equals_v0:0000000000000000000000000000000000000000000000000000000000000000:616263","claim_content_hash":"1d5c96e14bb5116ef1dcf2481fac95a8b134cfc7434dc6054e69f919ca15ff93","expected_sha256":"0000000000000000000000000000000000000000000000000000000000000000","computed_sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"}}}
```

`ResolutionFailed(MissingClaim)`, requested claim ID `claim-missing` —
95 bytes, SHA-256
`b99b96af9b35c96e6d65bea4a0fdd364670abefd21e163009737a9ef4d91969d`:

```json
{"outcome":"resolution_failed","details":{"claim_id":"claim-missing","reason":"missing_claim"}}
```

Future compatibility tests must pin all three literal vectors, their
lengths and SHA-256 values, every failure string, successful receipt
field order, failed-detail `claim_id` / `reason` order, requested-claim
attribution, and escaping edge cases. Any byte change requires a new
canonicalization-profile identity; the v0 bytes may not drift.

The candidate labels `Satisfied` / `Unsatisfied` were audited and
rejected: they sit one morpheme from the withdrawn Ticket 0056
vocabulary (`PredicateSatisfied` / `PredicateFalse`) and invite the
failure-to-falsity collapse this contract forbids. The existing v0
`Matched` / `DigestMismatch` tokens keep their evidence-local meaning
and are not reused or reinterpreted;
`DeterministicVerifierContextTraceV0::DigestMismatch` remains unchanged,
evidence-local, and non-authoritative.

Required laws:

```text
DigestEqual != Supported
DigestEqual != Settled

DigestUnequal != Refuted

ResolutionFailed != DigestUnequal

malformed != DigestUnequal
unavailable != DigestUnequal
oversized != DigestUnequal
unknown schema != DigestUnequal
unknown predicate != DigestUnequal

edge kind != predicate truth
evidence polarity != predicate truth

predicate outcome != standing
predicate outcome != admission
predicate outcome != contribution
```

One shared resolution and digest-computation path produces both
terminal outcomes; they differ only at the terminal exact digest
comparison. `DigestUnequal` is constructible only after every
resolution prerequisite succeeds and the 32 computed digest bytes
differ from the 32 strictly decoded expected digest bytes. Every
prerequisite failure is `ResolutionFailed` — never a negative result.
The empty subject is ordinary: empty bytes against the empty-byte
digest yield `DigestEqual`; against any other valid digest,
`DigestUnequal`.

## 8. Exact receipt shape

Exactly eight fields, each independently derived from the same replay
and each needed to audit the terminal comparison:

```text
predicate_schema, predicate_id, claim_id, scope_ref,
canonical_statement, claim_content_hash, expected_sha256,
computed_sha256
```

The receipt's `claim_id` is both successful query identity and replay
attribution because resolution succeeded. A failure has no receipt; its
separately retained requested claim ID is query attribution only and
proves no replayed claim existed.

`scope_ref` is routing and later policy-binding context only, never
truth. `claim_content_hash` records the rebound statement commitment,
never the subject digest.

Excluded candidates: verified-prefix identity (the snapshot owns no
such field; including it would fabricate provenance), subject length
(exactly recomputable from the retained canonical statement), raw
subject bytes (already recorded injectively by the canonical statement
and covered by its separately verified content hash), and any
`Status`, policy identity, evidence or edge ID, contribution,
admission result, or polarity (a predicate receipt is not standing
material).

## 9. Exact same-replay authority path

```text
one successful LogReader verification
→ one private composite replay
→ one StandingReplaySnapshot (private construction)
→ one same-call evaluation by one claim ID
→ one Sha256ClaimInlineBytesPredicateOutcomeV0
```

Conceptual future trusted query:

```rust
StandingReplaySnapshot::evaluate_sha256_claim_inline_bytes_equals_v0(
    &self, claim_id: &str,
) -> Sha256ClaimInlineBytesPredicateOutcomeV0
```

The evaluator privately retains the exact query string at entry for
`requested_claim_id()`. It adds no claim-ID validation, bound, failure,
normalization, hashing, truncation, or canonicalization. Successful
resolution retains that value once in the receipt. Failed resolution
retains it only as private query attribution beside the closed reason;
it does not turn the query key into evidence that a claim existed.

No caller supplies subject bytes, expected digest, predicate result,
parser output, resolved subject, descriptor, metadata, statement,
scope, receipt, outcome, authority table, policy, standing, evidence
ID, edge ID, closure, callback, content provider, snapshot fragment,
or store. The evaluator re-fetches both claim tables internally from
its one snapshot; mixing tables across snapshots is impossible by
construction. The contracted future evaluation hangs on `StandingReplaySnapshot` —
the layer that owns the existing standing-inert v0 verifier query —
because the predicate needs only replayed claim tables; it does not
consume a closure and asserts no verified-prefix provenance. The return is
total: no `Option`; a missing claim is an explicit `ResolutionFailed`
reason, and `requested_claim_id()` still returns the exact supplied
query key without asserting that the missing claim existed. Two
evaluations of the same claim on the same snapshot produce equal
outcomes and byte-identical serialized audit output. Failures with the
same reason but different requested claim IDs retain distinct query
attribution and therefore serialize to different canonical bytes.

## 10. Exact parser and evaluation order

1. Re-fetch the typed claim and the legacy claim from the same verified
   snapshot by the one claim ID.
2. Strictly parse the outer metadata envelope with one duplicate-aware
   whole-envelope parse: exactly `claim_domain` and
   `machine_predicate_inline_bytes`, no other outer keys, no duplicate
   keys at either level, no descriptor fields at the outer level;
   `claim_domain` revalidated by the existing strict claim-domain
   parser and required to equal `ExactMachineCheckable`. Duplicate
   detection is post-unescape; the canonical statement is derived from
   decoded values; raw token bytes are not identity.
3. Strictly parse the nested descriptor in Ticket 0057's ratified
   resolution order: schema; `predicate_id` (missing, wrong-type,
   empty, then beyond the compiled 64-byte bound — decided by a
   borrowed count-only unescape preflight that accumulates no complete
   or partial copied identifier and rejects decoded byte 65 before any
   resolver-owned allocation or copy attributable to the identifier,
   character validation, or statement construction — then outside
   `[a-z0-9_]+`, then
   byte-unequal to the compiled identity); `expected_sha256`
   (missing, wrong-type, then not exactly 64 lowercase hexadecimal
   characters — no prefix, no uppercase, no whitespace — validated
   before canonical statement construction); `subject_hex` (missing,
   wrong-type, then beyond 8192 encoded characters — rejected before
   decoding — then odd-length, uppercase, or non-hex). This requires a
   purpose-built borrowed two-pass inline-descriptor parser, or an
   equivalent count-only preflight.
   The existing strict claim-domain parser may be reused for the outer
   `claim_domain`, and its duplicate/unknown-key structural pattern may
   be reused, but its allocating generic descriptor-string value path
   must not be reused for the inline descriptor's resource-sensitive
   string fields. The first pass inspects and logically unescapes an
   existing upstream-owned `predicate_id` token only to count decoded
   UTF-8 bytes. Before the bound decision, the resolver may retain only
   bounded non-identifier bookkeeping: the decoded-byte count, JSON
   escape and UTF-8 decoder state, fixed scalar flags, and
   duplicate/key-traversal state containing no copied identifier bytes.
   It may not mutate or extend upstream input; allocate or extend an
   owned `String` or `Vec` as an identifier representation; construct an
   owned `Cow` containing identifier material; copy raw or decoded
   identifier bytes into a buffer; accumulate a prefix, suffix,
   complete, or partial identifier; materialise the complete identifier;
   or invoke the allocating generic descriptor-string path. Unrelated
   bounded parser bookkeeping containing no copied identifier bytes
   remains outside this predicate-ID allocation/copy guarantee. Decoded
   byte 65 yields `PredicateIdTooLong`. Allocation or copying of raw
   claim, event, metadata, statement, input-buffer, or
   storage/application representations performed before resolver entry
   by upstream replay or application code is outside this resolver-owned
   resource law and cannot be undone; that scope exclusion grants the
   resolver no additional permission. Only after the bound succeeds may
   a second pass decode, copy, or allocate the accepted bounded
   identifier; character validation and canonical statement construction
   follow.
   Escaped JSON and multibyte UTF-8 are counted by their decoded bytes
   and cannot bypass the bound. Duplicate detection remains
   post-unescape.
4. Derive the canonical statement and require exact three-way
   statement equality by direct byte comparison —
   `StandingClaim.statement == TypedClaimNode.statement ==
   descriptor-derived canonical statement`. No normalization.
5. Require a non-empty `content_hash` and recompute it separately as
   the lowercase SHA-256 of the exact UTF-8 statement bytes. Collision
   resistance is assumed; hash equality is never identity.
6. Decode the subject from the claim only, enforcing the compiled
   `MAX_INLINE_SUBJECT_BYTES_V0 = 4096` decoded bound after decode.
7. Compute SHA-256 over the decoded subject bytes and compare the 32
   computed bytes with the 32 strictly decoded expected digest bytes.
   Equality yields an opaque outcome of kind `DigestEqual`; inequality
   yields one of kind `DigestUnequal`. In either case `receipt()`
   borrows the evaluator-bound receipt.

The evaluator is deterministic, total or fail-closed, commandless,
networkless, pluginless, callback-free, environment-independent, and
free of ambient mutable policy.

## 11. Exact failure order

`ResolutionFailed` carries the exact requested claim-ID attribution
plus exactly one variant of `ClaimInlineSubjectResolutionFailureV0`, a
closed 33-reason enum evaluated in this frozen first-failure order:

```text
 1. MissingClaim
 2. MissingTypedClaim
 3. MalformedMetadata
 4. MissingClaimDomain
 5. WrongTypeClaimDomain
 6. DuplicateClaimMetadataKey
 7. UnknownClaimDomain
 8. ClaimDomainMismatch
 9. UnknownClaimMetadataKey
10. MissingInlineSubjectDescriptor
11. WrongTypeInlineSubjectDescriptor
12. DuplicateDescriptorKey
13. UnknownDescriptorKey
14. MissingSchema
15. WrongTypeSchema
16. UnknownSchema
17. MissingPredicateId
18. WrongTypePredicateId
19. EmptyPredicateId
20. PredicateIdTooLong
21. InvalidPredicateId
22. UnknownPredicateId
23. MissingExpectedSha256
24. WrongTypeExpectedSha256
25. InvalidExpectedSha256
26. MissingSubjectHex
27. WrongTypeSubjectHex
28. SubjectHexTooLong
29. InvalidSubjectHex
30. StatementBindingMismatch
31. MissingClaimContentHash
32. ClaimContentHashMismatch
33. DecodedSubjectTooLarge
```

The exact per-variant definitions are ratified in the design note's
failure-vocabulary section. Every variant is a closed audit failure
with no standing effect and no negative meaning. One evaluation returns
at most one reason, the first in this order. `DecodedSubjectTooLarge`
is unreachable when variants 28-29 hold and is retained as a compiled
guard, never as falsity.

## 12. Exact serialization boundary

Every future authority-bearing type in this family — the outcome, the
receipt, any application type — uses private construction, private
fields, read-only getters, `Serialize` only, no `Deserialize`, no
`Default`, no public constructor, no struct-literal construction, and
no public mutation. The outcome is opaque beyond this: no caller can
select or replace its classification, move a receipt between outcomes,
or construct it through any public enum-variant, struct-, or
tuple-literal syntax; the fieldless kind enum and the closed failure
enum are caller-constructible classification vocabulary only, carry no
receipt, and are never accepted as authority. No resolver, evaluator,
or composer ever accepts such a value back as authority input. Clones
and serialized forms are audit material only; the authority-bearing
types themselves expose no `Deserialize` path, and parsing lookalike
bytes into generic caller-owned data confers no authority. Borrowing
`requested_claim_id()` confers no authority and provides no mutation or
replacement path. The internal decoded-subject value is private,
consumed in the same call, and never published as a second authority
source. Existing v0-v3
standing-resolution types keep their frozen public fields and APIs;
caller constructibility never confers authority. Canonical outcome
bytes are exclusively the exact adjacent-tagged compact UTF-8 JSON
profile and three pinned vectors in §7; generic serialization with any
other serializer or presentation is not canonical.

## 13. Versioning boundary and compatibility invariants

`magpie-machine-predicate-v0`, `sha256_bytes_equals_v0`,
`magpie-verification-witness-v0` and its `witness_hex` field,
`DeterministicVerifierContextTraceV0` (including `Matched` and
`DigestMismatch`), and standing policies v0-v3 remain byte- and
behavior-identical. The boundary is fail-closed in both directions: an
inline-subject claim presented to the v0 predicate parser fails closed
as an unknown outer key; a v0-envelope claim presented to the inline
parser fails `ResolutionFailed`; a hybrid naming the new schema with
the v0 predicate identity fails `UnknownPredicateId`; a hybrid naming
the v0 schema fails `UnknownSchema`. No parser dispatches on
`claim_domain` alone. No L0 payload, tag, canonical encoding,
`docs/FORMAT.md`, golden vector, or Python verifier change is required
or permitted.

## 14. Polarity neutrality and standing inertia

No standing policy, support rule, refutation rule, conflict or
precedence rule, contradiction debt, invalidation, supersession or
currentness rule, evidence-kind assumption, edge-kind interpretation,
or attestation schema is ratified. The evaluation reads no evidence
node, edge, closure, or policy surface, so nothing about `supports` or
`contradicts` can alter the evaluated subject or the truth outcome.
`ResolutionContentClosureV0`, artifact-provenance verification, origin
admission, and the standing resolvers are not consumed. No ambient
lookup, second store read, network, filesystem, CAS, callback, or
loader is introduced. Later positive-support and direct-refutation
policies must invoke this same snapshot-only path internally and may
not accept caller-created outcomes or receipts.

## 15. Attestation decision and adjudication

Sol C compared three options — (1) predicate only, attestation deferred
entirely; (2) predicate plus routing-only attestation in this PR;
(3) predicate now, the predicate evaluator implementation immediately
next, and attestation as the next separately ratified contract
thereafter — and recommended option 3. K3 verified the material claims
against the repository and accepted:

- The predicate is a complete claim-local decision; an attestation is
  an independent evidence-format and binding decision (Ticket 0057
  ratified only its boundary, not a schema).
- Two load-bearing questions — the exact outer evidence-metadata
  envelope identity and the accepted evidence kind — lack enough
  evidence to freeze cleanly now.
- Bundling would ratify two independent decisions in one PR, against
  the smallest-sound-surface doctrine this repository applied to
  Tickets 0053 and 0057 themselves.

The ratified sequence:

```text
claim-inline SHA-256 predicate contract (this ticket)
→ claim-inline SHA-256 predicate evaluator implementation
→ routing-only attestation-binding contract
→ routing-only attestation-binding implementation
→ support/refutation lane contract(s)
→ corresponding standing-inert lane/input implementation(s)
→ direct-refutation policy contract
→ direct-refutation runtime
```

No later direct-refutation or support/refutation contract may bundle
the attestation schema with polarity and standing rules in one PR. What
is binding for that later attestation-binding contract is only what is
already ratified
elsewhere: Ticket 0057's boundary — routing and binding fields only,
named there as `schema`, `predicate_id`, `subject_claim_id`,
`scope_ref`, with `predicate_id` equal to the already validated claim
predicate identity, and no alternate subject byte source consumed by
the inline path — and, under Ticket 0057's authority boundary, one
same-replay, non-caller-substitutable binding path whose serialized
output is audit material only — and this contract's neutrality laws:
attestation binding is not predicate evaluation, predicate evaluation
is not evidence polarity, and evidence polarity is not standing
contribution. That inherited field boundary is Ticket 0057's ratified
text; changing it would require a separately reviewed amendment of
Ticket 0057, not silent reshaping. What remains genuinely open for the
attestation contract — the schema and outer-envelope identities,
duplicate and unknown-key handling with failure precedence, the exact
binding mechanics within the already-fixed same-replay path, field
requiredness, and the accepted evidence kind — is explicitly
**non-binding candidate material** recorded in the design note's
attestation section. This ticket ratifies none of that candidate
material.

Related boundary decision: `docs/design/standing-aggregation-independence-groups.md`
is a living doctrine note whose two status statements ("a predicate
contract ... remain[s] future") become stale on ratification. The
mission rule permits editing it only with explicit user approval of an
expanded allowlist; the user declined to grant that approval, so the
file stays untouched in this PR and the staleness fix is deferred to
the immediate predicate-evaluator implementation PR. Its historical
Ticket 0056 statements remain true regardless.

## 16. Candidate adjudication record

Three independent fresh read-only `gpt-5.6-sol` (xHigh) audits ran with
no shared inputs and no maker preference: Sol A (proposition and
semantics), Sol B (authority and substitution), Sol C (surface and
contract boundary). Sol A and Sol C returned `PROCEED WITH CONDITIONS`;
Sol B returned `SOUND WITH CONDITIONS`. K3 independently verified every
material claim against the repository, including recomputing every hash
vector quoted below. Their verdicts are review evidence, not epistemic
authority.

| Finding | Source | Decision | Repository evidence | Effect |
| --- | --- | --- | --- | --- |
| Proposition ranges over decoded claim-owned subject bytes only; encoded-hex and statement-byte readings explicitly excluded | A | accepted | v0 checker hashes decoded bytes and compares digest bytes; 0057 descriptor law | §5 formula; design note proposition |
| `subject_hex` and `expected_sha256` are separate claim-owned operands read from the replayed claim, never caller-supplied evaluation inputs; the expected operand does not independently identify or commit to the subject | A, B | accepted | 0057 authority boundary; both author-fixed at assertion time; `DigestUnequal` remains a legitimate terminal relation | §5, §9 |
| Two-sided decidability holds after successful resolution; outcome proves only the digest relation | A | accepted | 0057 chose inline bytes as decidable in both directions; 0056 lacked claim-fixed subjects | §5 |
| Accept `sha256_claim_inline_bytes_equals_v0` (35 bytes, closed alphabet, new identity) | A | accepted | naming consistent with `sha256_bytes_equals_v0`; within `MAX_PREDICATE_ID_BYTES_V0 = 64`; absent repo-wide before this contract | §6 |
| Replace `Satisfied`/`Unsatisfied` with relation names `DigestEqual`/`DigestUnequal` plus `ResolutionFailed` | A | accepted | withdrawn 0056 vocabulary (`PredicateSatisfied`/`PredicateFalse`) is inactive; `Matched`/`DigestMismatch` stay evidence-local | §7 |
| Closed failure enum in frozen first-failure order covering every 0057 prerequisite; `DigestUnequal` only from the terminal unequal comparison | A, B | accepted | 0057 resolution procedure order; the family's own frozen structural order (the v0 claim-domain parser's encounter-order duplicate precedence is deliberately not mirrored) | §10, §11 |
| Evaluator takes only `&self` + one claim ID on `StandingReplaySnapshot`; total return, no `Option` | A, B | accepted | v0 verifier query lives on the snapshot; snapshot is private-construction; replay maps first-write-wins | §9 |
| Compiled predicate-identity equality before statement construction; mismatch is `UnknownPredicateId` | B | accepted | v0 checker compares parsed ID to its compiled constant and fails `UnknownPredicateId` | §6, §10, §11 |
| Receipt = exactly 8 fields; no verified-prefix identity (snapshot owns none), no subject length (recomputable from the statement), no raw subject copy, no standing/policy/evidence/edge/polarity fields | A, B | accepted | snapshot carries no prefix identity; v0 receipt needs `witness_len` only because witness bytes are absent from it; doctrine: claim ID plus digest is insufficient audit basis | §8 |
| Opaque private-construction, `Serialize`-only outcome and receipt; closed `Serialize`-only failure classification; every serialized form is audit material only | A, B | accepted | the failure vocabulary may be caller-constructible but carries no receipt, cannot create an outcome, and is never authority; v0 receipt and v3 private-construction doctrine | §7, §12 |
| One duplicate-aware whole-envelope parse; post-unescape duplicate detection; canonical statement derived from decoded values | B | accepted | existing strict parser machinery (structural duplicate/unknown-key mechanics only; its allocating descriptor value path is not reused for the inline descriptor family's resource-sensitive fields); the lenient claim-domain parser alone is never descriptor authority | §10 |
| Bidirectional fail-closed versioning boundary; hybrids fail `UnknownSchema`/`UnknownPredicateId` | B | accepted | v0 parser rejects unknown outer keys; 0057 versioning boundary | §13 |
| Recreated Ticket 0056 arbitrary-byte attack yields `DigestEqual` for the true claim; evidence and edges are never evaluator inputs | B | accepted | the evaluator has no evidence/edge input; 0056 blocker record | §16 hostile table, design note hostile table |
| Deterministic repeated evaluation: equal outcomes, byte-identical serialized audit output | B | accepted | no candidate collection, clock, nonce, or traversal-dependent field exists | §9, §10 |
| Attestation deferred to a separate later contract (option 3); sequence predicate contract → predicate evaluator implementation → routing-only attestation-binding contract → attestation-binding implementation → support/refutation lane contract(s) → standing-inert lane/input implementation(s) → direct-refutation policy contract → direct-refutation runtime; do-not-bundle law | C | accepted | 0057 ratified the attestation boundary but no schema; outer envelope identity and accepted evidence kind unresolved | §15 |
| Ceilings note needs both stale living-status spots updated; aggregation note left untouched without explicit user approval | C | accepted with narrowing | user declined allowlist expansion; staleness recorded as deferred debt | §4, §15, README and ceilings handoffs |
| 0057 design note's aside that `sha256_bytes_equals_v0` is 21 characters (it is 22) | A | accepted as recorded observation | counted directly; harmless rationale typo in a ratified document | not repeated in 0058 documents; 0057 text untouched per ratified-law rule |

Sol B's report quoted the zero-digest variant statement hash truncated
by two characters; K3 recomputed every hash vector quoted in these
documents independently (reference claim, empty-subject, and
unrelated-material vectors all confirmed). The verified full form of
that variant is
`1d5c96e14bb5116ef1dcf2481fac95a8b134cfc7434dc6054e69f919ca15ff93`,
recorded here for completeness; it is not itself a frozen vector in
either document.

### Hostile contract review (Sol D)

One fresh, read-only `gpt-5.6-sol` (xHigh) hostile review of the
candidate ran with no maker summary and no prior reviewers' reports,
with ten falsification targets. It returned `FAIL` with four findings;
K3 independently verified each against the repository and remediated
all four:

| Finding | Verdict | Remediation |
| --- | --- | --- |
| 1. The design note's hostile table presented `DecodedSubjectTooLarge` as caller-reachable, contradicting the frozen first-failure contract (any valid ≤8192-char input decodes to ≤4096 bytes) | valid, must-fix | row replaced: 8194 encoded characters yield `SubjectHexTooLong`; variant 33 is documented as an internal defense-in-depth guard, unreachable after the subject-hex validation variants 28-29 |
| 2. README entry 28 used present tense implying the evaluator exists (`compiled into the evaluator`, `Evaluation hangs on`) while the slice ships no runtime | valid, must-fix | entry 28, ticket §9, and the design note's authority path now use contracted-future tense |
| 3. The attestation deferral was self-contradictory: it denied ratifying a schema, then froze a candidate identity and field set as non-reshapable | valid, must-fix | both documents now distinguish what is already binding (Ticket 0057's routing-only/no-byte-source boundary and this contract's neutrality laws) from explicitly non-binding candidate material the attestation contract owns and may reshape |
| 4. The adjudication record claimed the full zero-digest hash form was used "in both documents"; it appeared in neither | valid, should-fix | sentence corrected; the verified full form is recorded in this section only, and the empty-subject statement content hash (`afc617e9…d67aa`) is now quoted in the design note |

All other falsification targets survived Sol D's attack unchanged:
proposition identity, subject identity, one-path polarity neutrality,
bidirectional version separation, failure-never-falsity (including
double-failure winner pinning), standing inertia, changed-path scope,
constants, and ticket/design agreement. Sol D independently recomputed
every quoted hash vector and confirmed each one.

### Closure review round 1

A fresh, read-only `gpt-5.6-sol` (xHigh) closure review of the
remediated candidate ran cold, verifying the Sol D remediations and
re-attacking all ten falsification targets. It returned `FAIL` with two
findings; K3 independently verified each against the repository and
remediated both:

| Finding | Verdict | Remediation |
| --- | --- | --- |
| 1. The Sol D remediation overshot: calling the four-field attestation set "freely reshapable" contradicts Ticket 0057's ratified boundary text, which names `schema`, `predicate_id`, `subject_claim_id`, `scope_ref` as the routing/binding carriers | valid, must-fix | both documents now treat the four-field routing boundary (with `predicate_id` equal to the validated claim predicate identity, and no alternate subject byte source) as inherited Ticket 0057 law, changeable only by a separately reviewed 0057 amendment; only the envelope identity, schema identity, failure rules, binding mechanics, requiredness, and evidence kind remain non-binding candidate material |
| 2. The claim that the failure-order prefix "mirrors the existing v0 outer-metadata order" is false for multi-fault metadata: the v0 claim-domain visitor records a duplicate key on encounter, so it can report a duplicate before a missing or wrong-typed `claim_domain` (`crates/magpie-claims/src/standing.rs`) | valid, should-fix | the frozen order is kept (it is deterministic and sound) but the mirror claim is removed from the design note and from this table's evidence column; both now state the prefix is the family's own frozen structural order that deliberately does not mirror the v0 parser |

The closure review verified the four Sol D remediations as landed,
mechanically confirmed the 33 failure variants are identical in name,
number, and order across both documents, and independently recomputed
every quoted hash vector and constant, including the tour's 1917 bytes
and pinned SHA-256.

### Closure review round 2

A second fresh, read-only `gpt-5.6-sol` (xHigh) closure review ran cold
on the round-1-remediated candidate. It returned `FAIL` with one
must-fix finding in two parts; K3 verified both parts against Ticket
0057's ratified text and remediated:

| Finding | Verdict | Remediation |
| --- | --- | --- |
| 1a. The design note said this PR ratifies "no field set" (contradicting the inherited 0057 routing boundary named two paragraphs later) and classified the exclusion of `witness_hex`, subject bytes, digest overrides, results, receipts, standing contributions, and policy identities as reshapable candidate material — though those exclusions are exactly what 0057's "routing and binding fields only" law already forbids | valid, must-fix | the design note now says "no field set beyond Ticket 0057's already-ratified routing boundary"; the non-routing-field exclusions and the same-replay non-caller-substitutable binding-path doctrine are stated as inherited law; the non-binding list is narrowed to exactly the ticket's open dimensions (schema identity, envelope identity, duplicate/unknown-key handling and failure precedence, binding mechanics, field requiredness, evidence kind); the reviewer checklist is aligned |
| 1b. Both hostile tables said a mismatching attestation `predicate_id` is deferred/unratified — but Ticket 0057 already ratified that the attestation `predicate_id` is the already validated claim predicate identity | valid, must-fix | both hostile tables now state the mismatch is rejected under 0057's inherited boundary; only the exact failure vocabulary, precedence, and binding mechanics are deferred |

The review verified all six prior remediations as landed, confirmed
the failure-enum identity and order across both documents, and
independently recomputed every quoted vector and constant.

### Closure review round 3

A third fresh, read-only `gpt-5.6-sol` (xHigh) closure review ran cold
on the round-2-remediated candidate. It returned `FAIL` with one
must-fix finding; K3 verified it against Ticket 0057's ratified text
and remediated:

| Finding | Verdict | Remediation |
| --- | --- | --- |
| Both hostile tables attributed the `witness_hex` rejection to "the deferred attestation boundary" — misattributing inherited 0057 law to the deferred material — and neither checklist pinned the inherited/open split | valid, must-fix | both hostile tables now attribute the byte-source rejection to Ticket 0057's inherited routing-only boundary (only failure vocabulary, precedence, and mechanics deferred); both checklists now pin the exact split: inherited four routing fields, predicate-ID equality, no alternate/non-routing fields, same-replay binding path, versus the six open dimensions |

Every other falsification target survived the round unchanged.

### Closure review round 4

A fourth fresh, read-only `gpt-5.6-sol` (xHigh) closure review ran cold
on the round-3-remediated candidate. It returned `FAIL` with one
must-fix finding; K3 verified it against Ticket 0057's authority
boundary and remediated:

| Finding | Verdict | Remediation |
| --- | --- | --- |
| The ticket's §15 inherited list omitted the one same-replay, non-caller-substitutable binding path (present in the design note and both checklists), and both documents used unqualified "binding mechanics" as an open dimension — readable as reopening the same-replay requirement itself and permitting cross-prefix or caller-produced attestation binding | valid, must-fix | §15 now lists the same-replay, non-caller-substitutable binding path (audit-material serialized output) as inherited under Ticket 0057's authority boundary; every open-dimension and hostile-table use of "binding mechanics" in both documents is qualified as the exact mechanics within that already-fixed path |

Every other falsification target survived the round unchanged,
including the eight prior remediations, which the reviewer audited as
landed.

### Closure review round 5

A fifth fresh, read-only `gpt-5.6-sol` (xHigh) closure review ran cold
on the round-4-remediated candidate. It returned `FAIL` with one
must-fix finding; K3 verified it and remediated:

| Finding | Verdict | Remediation |
| --- | --- | --- |
| Both checklists still listed bare `binding mechanics` among the genuinely open dimensions — the round-4 qualification had been applied to §15, the design-note candidate list, and all four hostile rows, but not to the two checklist surfaces, and the round-4 record's claim that every open-dimension use was qualified was therefore premature | valid, must-fix | both checklists now read "binding mechanics within that already-fixed same-replay path"; a full-document sweep confirms every normative open-dimension list and hostile row is now qualified (remaining bare uses are historical narratives inside this review record) |

Every other falsification target survived the round unchanged.

### Closure review round 6

A sixth fresh, read-only `gpt-5.6-sol` (xHigh) closure review ran cold
on the round-5-remediated candidate. It returned `FAIL` with one
must-fix finding; K3 verified it and remediated:

| Finding | Verdict | Remediation |
| --- | --- | --- |
| All four hostile rows said "only the exact failure vocabulary, precedence, and binding mechanics ... are deferred", narrowing the deferred set to three items and contradicting the six open dimensions ratified in §15 and both checklists | valid, must-fix | all four hostile rows now defer exactly the six open dimensions — schema identity, envelope identity, duplicate/unknown-key handling and failure precedence, binding mechanics within the already-fixed same-replay path, field requiredness, and evidence kind — while preserving the inherited rejection sentences for `witness_hex` and mismatching `predicate_id` |

This six-dimension phrasing supersedes the narrower deferred-list
wording quoted in the round-2 and round-4 narratives above, which
describe each round's state at the time. Every other falsification
target survived the round unchanged, including all ten prior
remediations, which the reviewer audited as landed.

### Closure review round 7 (final)

A seventh fresh, read-only `gpt-5.6-sol` (xHigh) closure review ran
cold on the round-6-remediated candidate and returned `PASS` with no
findings. All ten falsification targets survived: proposition and
subject identity; one-path polarity neutrality; bidirectional version
separation and hybrid isolation; failure-never-falsity with the frozen
33-variant order; standing inertia and prefix-provenance honesty; the
six-surface attestation agreement closing the cross-prefix and
caller-produced binding attack; exact five-path scope; every quoted
hash vector and constant (independently recomputed); exact
ticket/design-note agreement; and all eleven prior remediations
verified landed. No further review rounds were run, per the stop rule
of one genuine PASS.

### External review remediation (CodeRabbit, PR #73)

After publication as PR #73, CodeRabbit reported a validation-enforcement
finding: the ticket's validation commands were report-only, with no hard
tour byte/hash gate and no changed-path allowlist enforcement. Its
autofix commit (`9ae4b03`, ticket-only) added both gates as a narrow
first remediation. K3 verified the autofix against the repository and
found the enforcement real but three residual defects still valid:

1. **Ticket/design disagreement** — the autofix changed only this
   ticket; the design note still carried the earlier report-only
   commands, so the two documents no longer agreed exactly.
2. **Non-exact tour capture** — `cargo run ... > tour_output.txt`
   routes stdout through PowerShell native-output redirection, whose
   encoding differs across PowerShell versions (UTF-16LE with BOM in
   5.1, UTF-8 in 7+), so the hashed bytes were not guaranteed to be the
   executable's exact stdout bytes; the temporary file was also left
   behind whenever a gate failed.
3. **Vacuous path gate** — the check parsed `git status --short` with a
   fixed `Substring(3)` (fragile for rename records and quoted paths)
   and never inspected the committed PR diff against the exact base,
   staged changes, or untracked files, so a clean committed branch
   passed with an empty changed set.

K3 remediated within the existing two-file documentation scope, with no
architectural change: the hardened validation contract now lives
identically in this ticket (§22) and the design note (validation
section). The tour gate captures exact stdout bytes through Python's
binary `subprocess` interface (no PowerShell redirection, no temporary
file, actual length and digest printed, nonzero subprocess exit fails
the gate). The changed-path gate unions the committed PR diff against
the exact base, staged, unstaged, and untracked paths without parsing
porcelain output, requires every changed path inside the five-path
allowlist, and requires the committed PR diff to be exactly those five
paths. Negative tests confirmed the path gate fails on an unauthorised
untracked file and the tour gate fails on an altered expected digest or
length, with all test artifacts removed afterwards. Validation results
are recorded in the handoff.

A fresh read-only `gpt-5.6-sol` (xHigh) review of that hardened
candidate returned `FAIL` with three findings, all verified by K3 and
remediated within the same two-file scope:

1. **PowerShell 5.1 native-argument mangling** — the reviewer executed
   the block on Windows PowerShell 5.1 and showed that passing the
   Python checker as a `python -c` argument loses its embedded double
   quotes during native argument marshalling, so the program never
   parses. The checker now reaches Python on standard input
   (`@' ... '@ | python -`), which no PowerShell version's argument
   marshalling can re-encode or mangle.
2. **Ungated native exit codes** — a failed cargo or git command could
   be followed by a later success, and a failed path collector could
   silently yield empty output. Every native command in the block is
   now followed by an immediate exit-code check (the release-metadata
   checker keeps its documented dirty-worktree mirror exception).
3. **Rename-source invisibility** — `git diff --name-only` reports
   only rename destinations, so an out-of-allowlist source path could
   hide behind a rename into an allowlisted path. All three diff
   collectors now use `--no-renames`, exposing a rename as its
   old-path deletion plus new-path addition.

A fresh read-only `gpt-5.6-sol` (xHigh) closure review of the
remediated block returned `FAIL` with two findings, both verified by
K3 and remediated within the same two-file scope:

1. **Case-insensitive path comparisons** — the reviewer replayed the
   gate on Windows PowerShell 5.1 with a case-variant path
   (`readme.md`) and showed PowerShell's case-insensitive
   `Sort-Object -Unique`, `-notin`, and `-ne` collapsed it against the
   allowlisted `README.md`, letting an exact unauthorised Git pathname
   pass. The gate now uses `Sort-Object -CaseSensitive -Unique`,
   `-cnotin`, and `-cne` throughout, matching Git's exact path bytes.
2. **Ungated release-metadata and report commands** — a release-check
   refusal could be overwritten by later success, contradicting the
   every-command-gated claim. The release check is now gated: a
   refusal stops validation until the unchanged script passes in an
   in-block temporary VCS-free mirror that is removed afterwards even
   on failure, and the three report commands are exit-checked like
   every other native command.

A fresh read-only `gpt-5.6-sol` (xHigh) closure review of that state
returned `FAIL` with three findings, all verified by K3 and remediated
within the same two-file scope:

1. **Index-flag hole** — an `assume-unchanged` or `skip-worktree`
   index entry could hide a tracked modification from both diff
   collectors while never appearing as untracked. An exit-gated
   `git ls-files -v` check now fails on any non-normal (`H`) index
   tag; the current index is clean.
2. **`.git` file copied into the "VCS-free" mirror** — this worktree's
   `.git` is a 52-byte linked-worktree pointer file, and robocopy
   `/XD` excludes directories only, so the mirror stayed VCS-connected
   (K3's tar-based mirrors excluded it by name, which is why they
   worked). The mirror copy now uses `/XF .git` as well and hard-gates
   `Test-Path` for a `.git` in the mirror before running; verified
   against the real robocopy: mirror is VCS-free and the checker
   passes there.
3. **Mirror cleanup not guaranteed on every failure path** — a
   robocopy failure threw before `Remove-Item`, and the location
   cmdlets lacked terminating-error handling. The whole mirror
   lifecycle now runs in `try/finally` with the mirror removed even on
   failure and the native exit preserved for the final throw.

A fresh read-only `gpt-5.6-sol` (xHigh) closure review of that state
returned `FAIL` with one finding, verified by K3 and remediated within
the same two-file scope:

1. **Command-resolution failures pass the exit gates** — a missing
   `python` (or `git`/`cargo`) raises a non-terminating
   `CommandNotFoundException` in PowerShell and never sets
   `$LASTEXITCODE`, so the gates could pass without the checker ever
   running (reproduced by the reviewer on PS 5.1 and 7.6). The block
   now preflights `git`, `cargo`, and `python` — and `robocopy` on the
   mirror fallback path — with `Get-Command -ErrorAction Stop`, making
   resolution failure terminating before any gate runs.

A fresh read-only `gpt-5.6-sol` (xHigh) closure review of that state
returned `FAIL` with five findings, all verified by K3 and remediated
within the same two-file scope:

1. **Case-insensitive index-flag match** — `git ls-files -v` marks an
   assume-unchanged entry with a lowercase tag, and PowerShell
   `-match` folds case, so `'^[^H]'` missed it. The check now uses
   `-cmatch`.
2. **Preflight/invocation shadowing** — validating `python.exe` as an
   Application did not stop a shadowing `function python` from being
   what actually ran (reproduced on both shells). Every application is
   now resolved once to its concrete path (`Get-Command ... .Source`)
   and invoked by that path everywhere.
3. **PS 7 native-exit promotion** — with
   `$PSNativeCommandUseErrorActionPreference = $true` and
   `ErrorActionPreference = 'Stop'`, a nonzero native exit throws
   before the `$LASTEXITCODE` gates, and robocopy's success codes 1-7
   would throw before the `-ge 8` threshold. The block now neutralizes
   that preference for its duration and restores it afterwards.
4. **Location restoration not guaranteed** — a terminating error after
   `Push-Location` skipped `Pop-Location`, and cleanup then ran from
   inside the mirror with its failure swallowed. The mirror lifecycle
   now nests the push in its own `try/finally` (pop always runs) and
   verifies cleanup explicitly, throwing if the mirror remains.
5. **`.cmd` cargo shim invisible to Python** — a shim-resolved cargo
   passed the preflight but `subprocess.run(["cargo"])` fails with
   WinError 2 on Windows. The resolved cargo path is now handed to
   both Python consumers through `CARGO` (the release checker honors
   it natively; the tour gate reads it), and `CARGO` is restored on
   exit.

A fresh read-only `gpt-5.6-sol` (xHigh) closure review of that state
returned `FAIL` with two findings, both verified by K3 and remediated
within the same two-file scope:

1. **Non-scalar application resolution** — with more than one matching
   Application on PATH (for example an installed Python beside the
   Windows App Execution Alias), `Get-Command ... .Source` returns an
   array and the invocation fails (reproduced on both shells). All
   four resolutions now take the first Application match in PATH
   precedence via `(@(Microsoft.PowerShell.Core\Get-Command ...)[0]).Source`.
2. **StrictMode-unsafe preference probe and mutation ordering** —
   reading the PS 7-only preference variable throws under
   `Set-StrictMode` on shells where it does not exist, and `CARGO`
   was mutated before the protecting `try` began, so a throw there
   would have left it changed. The probe now uses `Get-Variable
   -ErrorAction SilentlyContinue`, `CARGO` is read and written through
   the .NET environment API, every mutation happens inside `try`, and
   both values are restored in `finally`.

A fresh read-only `gpt-5.6-sol` (xHigh) closure review of that state
returned `FAIL` with three findings, all verified by K3 and remediated
within the same two-file scope:

1. **Live `PSVariable` restore** — `Get-Variable` returns a live
   object, so `$nativePref.Value` read at restore time was the
   just-mutated `$false`, not the original value. The block now
   captures the scalar value before mutating and restores that scalar.
2. **Nearer-scope preference defeats the guard** — probing only the
   global scope missed a function- or script-local
   `$PSNativeCommandUseErrorActionPreference = $true`, which would
   still promote native exits (including robocopy's success codes).
   The block now sets a current-scope override after recording the
   local variable's presence and scalar value, and in `finally`
   restores the prior local value or removes the temporary override,
   leaving outer scopes untouched.
3. **Absent `CARGO` not deleted on PS 7** — .NET
   `SetEnvironmentVariable(name, null, Process)` sets an empty string
   rather than deleting on PS 7, leaving a present-but-empty `CARGO`
   that would also defeat the Python fallback. The block now records
   presence separately; when `CARGO` was absent it is explicitly
   removed in `finally` with a verification check, and otherwise the
   saved value is restored through the .NET API.

### External review closure (final)

A fresh read-only `gpt-5.6-sol` (xHigh) closure review of the fully
remediated candidate returned `PASS` with no findings. It executed the
complete state matrix on Windows PowerShell 5.1 and PowerShell 7.6,
under `Set-StrictMode -Version Latest`: local/global/absent
native-exit preference × set/absent `CARGO` × normal completion and
injected mid-block throw — every case restored the exact prior state.
Both shells' tour gates and the existing binary independently produced
exactly 1917 stdout bytes with the pinned SHA-256; the mirror fallback
passed from both ordinary and linked-worktree checkouts with cleanup
verified; the case-sensitive exact-five path gate, index-flag
rejection, and all prior remediations were verified landed. No further
review rounds were run, per the stop rule of one genuine PASS.

### External review remediation (CodeRabbit, second round)

Two further CodeRabbit findings on the published head were verified as
valid and remediated surgically:

1. **Whitespace coverage** — the block invoked only the unstaged
   worktree form of `git diff --check`. The pinned `$base` is now
   declared once, early, and the standard checks run three
   hard-failing invocations: the exact pinned base tree against
   `HEAD`, the staged index, and the unstaged worktree, each with its
   own native-exit check.
2. **Merge-base range** — the committed changed-path collector used
   a merge-base range rather than comparing the exact pinned tree
   directly with `HEAD`. It now uses a direct
   two-tree comparison `$base HEAD --`, with the duplicate later
   `$base` declaration removed.

### External review remediation (Codex P2, seven threads)

Seven Codex connector P2 threads on the published head formed six
substantive remediation groups: the validation mirror was reported
once for missing committed/staged whitespace checks and separately for
the merge-base-versus-exact-base collector. Four independent read-only
`gpt-5.6-sol` (xHigh) checkers (Rust authority/API,
semantics/inherited law, parser/resource law, sequencing/boundary)
verified all six groups valid; all seven threads are remediated:

1. **Outcome reclassification** — a public enum's variants are public,
   so a caller could move a receipt from `DigestEqual` into
   `DigestUnequal` (the move-and-rewrap attack). The outcome is now an
   opaque public struct with a private representation: no public
   constructor, struct-/tuple-literal construction, `Deserialize`,
   `Default`, `From`/`TryFrom`, or mutation; read-only `kind()`,
   `requested_claim_id()`, `receipt()`, and `failure()` borrows. The
   fieldless kind enum and the closed failure enum are
   caller-constructible classification vocabulary only — they carry no
   receipt or query attribution, cannot convert into an outcome, and
   are never accepted as authority. Serialization stays deterministic
   and one-way under the exact adjacent-tagged canonical profile,
   failure strings, field order, encoder law, and pinned vectors in §7.
2. **Descriptor precedence** — the failure enum had `subject_hex`
   checks before `expected_sha256`, contradicting Ticket 0057's
   ratified resolver procedure (schema → predicate_id →
   expected_sha256 → subject_hex). The 33-variant order is corrected
   in both documents (expected at 23-25, subject at 26-29), with all
   numeric dependents, both parser procedures, and a frozen
   expected-wins hostile reservation; no implementation may order by
   JSON source-key, map-iteration, or Serde encounter order.
3. **Pre-allocation bound** — the contract required
   `PredicateIdTooLong` before resolver-owned identifier allocation or
   copy while permitting exact reuse of the allocating existing
   descriptor machinery; both could not hold. The final law preserves
   Ticket 0057 unchanged and makes the Ticket 0058 implementation path
   explicit: a borrowed first pass logically unescapes and counts
   decoded UTF-8 bytes with bounded non-identifier state only and
   without copying any complete or partial identifier; decoded byte 65
   rejects; only after success may a second pass decode, copy, or
   allocate the accepted bounded identifier, followed by character
   validation and statement construction. Pre-existing upstream
   replay/application allocation or copying before resolver entry is
   outside this resolver-owned law and cannot be undone; that
   clarification neither relaxes the law nor permits resolver mutation,
   extension, or identifier copying. The existing claim-domain parser
   and structural duplicate/unknown-key pattern may be reused, but the
   allocating generic descriptor-string value path is not sufficient
   for this family's resource-sensitive fields.
4. **Operand terminology** — `expected_sha256` was described as
   committing to the subject bytes, false for every well-formed
   `DigestUnequal` claim. It is now defined everywhere as the
   claim-owned expected terminal digest operand (proposition data);
   the canonical statement injectively records both operands and
   `ClaimAssertedV2.content_hash` commits to the statement bytes.
5. **Rollout gap** — the sequence authorized no step that implements
   the evaluator later consumers need. The ratified progression is now
   explicit in all five files: predicate contract → predicate
   evaluator implementation (the immediate next slice) → routing-only
   attestation-binding contract → attestation-binding implementation →
   support/refutation lane contract(s) → standing-inert lane/input
   implementation(s) → direct-refutation policy contract →
   direct-refutation runtime. The aggregation-note staleness fix moved
   to the evaluator-implementation PR.
6. **Validation mirror (threads 6-7)** — the design note's validation
   block lacked the committed/staged whitespace gates and separately
   used a merge-base range instead of the exact pinned-base two-tree
   comparison. It now carries the identical hardened block as this
   ticket: one early `$base`, three whitespace checks (base-vs-HEAD,
   staged, unstaged), and the direct two-tree collector.

### External review remediation (post-`e9fa2bf`, three findings)

Three fresh review findings on `e9fa2bf` were adjudicated as valid and
remediated without widening the five-path documentation boundary:

1. **Ticket 0057 law restoration** — the subject-binding design note had
   been changed from its ratified pre-bound prohibition on
   resolver-owned `predicate_id` allocation or copy to a weaker
   streaming formulation. Every resource-law change in that
   prerequisite note is restored to its pre-remediation meaning. The
   clarification that upstream allocations completed before resolver
   entry are out of scope neither relaxes that law nor grants the
   resolver permission to copy any identifier bytes. Ticket 0058 owns
   the implementable mechanism: a borrowed count-only unescape pass
   retains bounded non-identifier state only and copies no complete or
   partial identifier, decoded byte 65 rejects, and only a successful
   bound decision permits a second pass to decode/copy/allocate the
   accepted identifier before character validation and statement
   construction.
2. **Canonical outcome serialization** — the former tag-only promise
   left several byte encodings possible. Both contract documents now
   freeze the exact
   `magpie-claim-inline-sha256-predicate-outcome-json-v0` adjacent-tagged
   compact UTF-8 JSON profile, all key and field orders, all 33 failure
   strings, the escaping law, absence rules, and three literal vectors:
   unchanged 639-byte `DigestEqual` and 641-byte `DigestUnequal`, plus
   the 95-byte attributed `ResolutionFailed(MissingClaim)` vector.
3. **Evaluator-slice scope** — the implementation boundary now opens
   with one complete permitted-scope list covering the bounded parser,
   same-snapshot resolution, snapshot-only method, opaque
   outcome/receipt types, closed failures, canonical audit JSON, hostile
   and compatibility tests, and documentation. The existing
   attestation, polarity, standing, support, refutation, and
   contradiction exclusions remain unchanged.

### External review remediation (post-`8abdb1d`, one finding)

One fresh Codex P2 found that reason-only failure bytes could not be
attributed to the requested query key. The opaque outcome now privately
retains the exact evaluator `claim_id` for every kind and exposes only
the read-only `requested_claim_id()` borrow. Successful outcomes reuse
the receipt's sole `claim_id` and preserve both successful vectors
byte-for-byte. `ResolutionFailed` has no receipt; it retains the query
key solely as attribution beside one of the unchanged 33 closed
reasons, without asserting that a replayed claim existed or creating
replay provenance, standing authority, or proof of resolution.

Canonical failed `details` is exactly `claim_id` then `reason`; the
`claim-missing` / `MissingClaim` vector is 95 bytes with SHA-256
`b99b96af9b35c96e6d65bea4a0fdd364670abefd21e163009737a9ef4d91969d`.
Repository inspection found no compiled claim-ID maximum:
`ClaimAssertedV2` requires only a non-empty identifier. This contract
therefore adds no claim-ID rejection, normalization, truncation, hash,
maximum, or 34th failure; serialized failure size grows linearly with
the exact query string under the fixed JSON escaping law.

## 17. Hostile cases

The design note's hostile table is normative and agrees exactly with
this summary. Reference claim C: subject `616263` (`abc`), expected
`ba7816bf…15ad`, canonical statement
`magpie-machine-predicate-inline-bytes-v0:sha256_claim_inline_bytes_equals_v0:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad:616263`,
content hash `aa1b8f1c339bc8e53c7db9d432e85073cc1495284279ba1b2474b74d26381223`:

1. **Ticket 0056 arbitrary-byte attack recreated** — a well-formed
   `contradicts` evidence node carrying `witness_hex =
   756e72656c61746564206d6174657269616c` (digest `d42273bc…2dc`),
   same claim, same scope: an opaque outcome of kind `DigestEqual` for
   C, with `receipt()` borrowing its receipt; the evidence bytes are
   never consumed; removing all support evidence changes nothing.
2. **`contradicts` edge on C** — no alteration of subject or outcome;
   the edge is not an evaluator input.
3. **`supports` edge on C** — same; no edge kind is interpreted.
4. **Subject `abc`, matching expected digest** — `DigestEqual`.
5. **Subject `abc`, different valid expected digest** —
   `DigestUnequal`, never `Refuted`.
6. **Empty subject, empty-byte digest** — `DigestEqual`.
7. **Empty subject, other digest** — `DigestUnequal`, never `Refuted`.
8. **Duplicate outer or nested key (including post-unescape
   duplicates)** — `ResolutionFailed`.
9. **Unknown outer or nested key** — `ResolutionFailed`.
10. **Wrong schema, including historical v0 schema** — kind
    `ResolutionFailed`, with `failure()` borrowing `UnknownSchema`; no
    cross-family reinterpretation in either direction.
11. **Uppercase, prefixed, spaced, or wrong-length expected digest** —
    kind `ResolutionFailed`, with `failure()` borrowing
    `InvalidExpectedSha256`, before statement
    construction. When both the expected digest and the subject hex are
    invalid, the expected-digest failure wins — Ticket 0057's ratified
    resolution order, never JSON source-key order, map iteration, or
    Serde encounter order.
12. **Odd-length, uppercase, non-hex, or oversized subject hex** —
    `ResolutionFailed`, never `DigestUnequal`; the encoded bound is
    enforced before decoding or allocating the decoded subject output.
13. **Statement mismatch (either direct comparison)** — kind
    `ResolutionFailed`, with `failure()` borrowing
    `StatementBindingMismatch`.
14. **Content-hash mismatch despite statement agreement** — kind
    `ResolutionFailed`, with `failure()` borrowing
    `ClaimContentHashMismatch`.
15. **Matching content hash used instead of direct statement
    comparison** — rejected architecture; hash equality is never
    identity.
16. **Caller-created or deserialized resolved-subject value** — audit
    material only; never resolver authority.
17. **Caller-created, cloned, or deserialized receipt or outcome** —
    caller-created values are unconstructible; cloned or serialized
    copies of legitimately obtained values are audit material only;
    none is ever accepted as authority.
18. **Attestation carrying `witness_hex` or any byte source** —
    rejected by the inline path under Ticket 0057's inherited
    routing-only boundary (no alternate subject byte source); deferred
    to the attestation contract are exactly the six open dimensions —
    schema identity, envelope identity, duplicate/unknown-key handling
    and failure precedence, binding mechanics within the already-fixed
    same-replay path, field requiredness, and evidence kind.
    Historical v0 `witness_hex` handling is unchanged.
19. **Attestation `predicate_id` differing from the claim's** —
    rejected under Ticket 0057's inherited boundary: the attestation
    `predicate_id` must equal the already validated claim predicate
    identity. Deferred to the attestation contract are exactly the six
    open dimensions — schema identity, envelope identity,
    duplicate/unknown-key handling and failure precedence, binding
    mechanics within the already-fixed same-replay path, field
    requiredness, and evidence kind. The claim-side analogue —
    descriptor `predicate_id` unequal to the compiled identity — fails
    `UnknownPredicateId`.
20. **Same claim evaluated twice in one replay** — equal outcomes and
    byte-identical serialized audit output.
21. **Resolver mixing claim tables across snapshots** — impossible by
    construction; one `&self` snapshot re-fetches both tables
    internally.
22. **Alternate outcome tagging or payload shape** — external or
    internal tags, `receipt` beside `outcome`, reversed keys, omitted
    `details`, a failure without `claim_id`, failure-detail keys other
    than `claim_id` then `reason`, `null`, or any extra key are
    noncanonical; only the exact adjacent `outcome` / `details` profile
    is v0.
23. **Alternate JSON spelling** — pretty printing, trailing newline,
    Unicode normalization, escaped `/`, uppercase `\u00XX`, or a
    non-short control escape where the profile fixes a short escape is
    noncanonical and must differ from the pinned byte hashes.
24. **Missing `claim-a` and missing `claim-b`** — both borrow
    `MissingClaim` from `failure()`, but `requested_claim_id()` returns
    the exact respective query key and their canonical bytes differ.
25. **`MissingClaim` for `claim-missing`** — the retained ID attributes
    the query only; it does not assert claim existence, replay
    provenance, standing, or successful resolution.
26. **Caller replacement or serialized replay of failure attribution**
    — impossible as authority: no public outcome or failure-payload
    constructor, mutation, deserialization, or requested-ID replacement
    exists, and serialized bytes are never accepted back.
27. **Empty, long, or escapable requested claim ID** — preserve it
    exactly as supplied with no query-ID rejection, normalization, hash,
    truncation, maximum, or new failure; canonical size grows linearly
    under the fixed escaping law.

## 18. Frozen surfaces

No change to: v0/v1/v2/v3 types, bytes, behavior, or fixtures;
`DeterministicVerifierContextTraceV0`; the support and refutation
policy matrices; L0 payloads or canonical encoding; `docs/FORMAT.md`;
golden vectors; the Python verifier; the tour (1917 bytes, SHA-256
`48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f`);
Cargo manifests or the lockfile; CI; release metadata; historical
tickets; ratified contracts, including Ticket 0057's design-note law.

## 19. Explicit non-goals

No Rust, tests, fixtures, Cargo, dependency, L0, canonical encoding,
golden vector, Python verifier, runtime, standing rule, support or
refutation lane, policy v4 or later identity, attestation schema,
evidence-kind assumption, edge-polarity meaning, conflict or
precedence rule, contradiction debt, invalidation,
supersession/currentness, `EpistemicGate`, ordinary writer, loader,
CAS, filesystem or network ingestion, model integration, librarian,
Deadbolt change, or production trust claim. The predicate evaluator and
the subject resolver are not implemented in this slice. Contract and
runtime are never combined. The predicate evaluator implementation is
the immediate next separately reviewed slice. Its complete permitted
scope is only: the borrowed two-pass or equivalent count-only bounded
inline parser; exact same-snapshot claim resolution; the snapshot-only
evaluator method; opaque outcome and receipt types with exact privately
retained requested claim-ID attribution and the read-only
`requested_claim_id()` accessor; the closed failure vocabulary; neutral
digest comparison; the exact canonical outcome audit JSON profile;
hostile and compatibility tests; and documentation.
It may add no attestation, evidence-kind assumption, edge polarity,
support, refutation, standing, or contradiction handling.

## 20. Future sequence

```text
Ticket 0057: claim-inline subject binding contract — ratified
Ticket 0058: this predicate contract
→ claim-inline SHA-256 predicate evaluator implementation
→ routing-only attestation-binding contract
→ routing-only attestation-binding implementation
→ support/refutation lane contract(s)
→ corresponding standing-inert lane/input implementation(s)
→ direct-refutation policy contract
→ direct-refutation runtime
→ contradiction debt, invalidation, supersession/currentness, ...
```

This ticket ratifies none of those steps.

## 21. Stop conditions

Stop rather than proceeding if the contract appears to require: Rust,
tests, fixtures, Cargo, or dependency changes; a new L0 payload, tag,
canonical byte, or golden vector; reinterpreting
`sha256_bytes_equals_v0` or the v0 trace; any evidence-, edge-,
closure-, caller-, or audit-supplied subject or expected digest; any
alternate subject byte source consumed by the inline path; any standing
rule, policy identity, attestation schema, support or refutation
semantics, conflict handling, or edge-polarity meaning; separate
positive and negative subject paths; verified-prefix identity claimed
from `StandingReplaySnapshot`; closure, filesystem, CAS, network, or
ambient lookup; or editing any path outside the five-path allowlist.

## 22. Validation commands

Run the following commands and verify explicit pass/fail gates:

```powershell
# Preflight: resolve each required application once to its concrete
# path — the first Application match in PATH precedence, so multiple
# installed matches cannot produce an uninvokable array. A
# command-resolution failure is not a native exit and never sets
# $LASTEXITCODE, so the exit gates below cannot catch it; resolving
# once also prevents function/alias shadowing and bare-name shim
# failures from substituting a different command.
$gitExe = (@(Microsoft.PowerShell.Core\Get-Command git -CommandType Application -ErrorAction Stop)[0]).Source
$cargoExe = (@(Microsoft.PowerShell.Core\Get-Command cargo -CommandType Application -ErrorAction Stop)[0]).Source
$pythonExe = (@(Microsoft.PowerShell.Core\Get-Command python -CommandType Application -ErrorAction Stop)[0]).Source

# The exact pinned base, declared once and used by both the whitespace
# checks and the changed-path collectors below.
$base = "67a87e01575a9e9930ba3dc1ec52b95f963b6136"

# CARGO is handed to the Python consumers (the release checker honors
# it; the tour gate reads it) and restored on exit (or removed when it
# was absent). Native-exit promotion (the PS 7+ opt-in preference) is
# neutralized for the block by a current-scope override; the prior
# value (or absence) is restored on exit. Reads use Get-Variable, the
# .NET environment API, and scalar captures so the block stays safe
# under Set-StrictMode and live-reference traps on every supported
# shell.
$prevCargoEnv = [System.Environment]::GetEnvironmentVariable("CARGO", "Process")
$cargoEnvExisted = ($null -ne $prevCargoEnv)
$nativePrefVar = Get-Variable -Name PSNativeCommandUseErrorActionPreference -Scope Local -ErrorAction SilentlyContinue
$nativePrefExisted = ($null -ne $nativePrefVar)
$nativePrefValue = if ($nativePrefExisted) { $nativePrefVar.Value } else { $null }
try {
    [System.Environment]::SetEnvironmentVariable("CARGO", $cargoExe, "Process")
    $PSNativeCommandUseErrorActionPreference = $false
    # Standard checks (must pass): whitespace coverage across the exact
    # pinned base tree vs HEAD, the staged index, and the unstaged
    # worktree, then format, lint, and tests.
    & $gitExe diff --check $base HEAD --
    if ($LASTEXITCODE -ne 0) { throw "git diff BASE HEAD --check failed" }
    & $gitExe diff --cached --check
    if ($LASTEXITCODE -ne 0) { throw "git diff --cached --check failed" }
    & $gitExe diff --check
    if ($LASTEXITCODE -ne 0) { throw "git diff --check failed" }
    & $cargoExe fmt --all --check
    if ($LASTEXITCODE -ne 0) { throw "cargo fmt failed" }
    & $cargoExe clippy --workspace --all-targets --locked -- -D warnings
    if ($LASTEXITCODE -ne 0) { throw "cargo clippy failed" }
    & $cargoExe test --workspace --locked
    if ($LASTEXITCODE -ne 0) { throw "cargo test (workspace) failed" }
    & $cargoExe test --doc --locked
    if ($LASTEXITCODE -ne 0) { throw "cargo test (doc) failed" }

    # Tour invariant — hard gate over the exact stdout bytes. The
    # program reaches Python on stdin, never as a native argument, so
    # no PowerShell-version argument marshalling can re-encode or
    # mangle it; PowerShell redirection (>, Out-File, string variables)
    # is likewise never used. Cargo diagnostics stay on stderr and are
    # not part of the tour artifact. No temporary file is created.
    @'
import hashlib, os, subprocess, sys
r = subprocess.run(
    [os.environ.get("CARGO", "cargo"), "run", "--locked", "--example", "tour", "-p", "magpie-claims"],
    stdout=subprocess.PIPE,
)
if r.returncode != 0:
    sys.exit(f"FAIL: tour exited with code {r.returncode}")
b = r.stdout
h = hashlib.sha256(b).hexdigest()
print(f"tour bytes: {len(b)}")
print(f"tour sha256: {h}")
if len(b) != 1917:
    sys.exit(f"FAIL: expected 1917 tour bytes, got {len(b)}")
if h != "48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f":
    sys.exit("FAIL: tour SHA-256 mismatch")
'@ | & $pythonExe -
    if ($LASTEXITCODE -ne 0) { throw "tour gate failed" }

    # Release metadata — the direct check may refuse the intentionally
    # dirty worktree; a refusal stops validation until the unchanged
    # script passes in a temporary VCS-free mirror (removed afterwards,
    # even on failure, with cleanup verified).
    & $pythonExe tools/check_release_metadata.py
    if ($LASTEXITCODE -ne 0) {
        $mirror = Join-Path $env:TEMP ("magpie-relcheck-" + [guid]::NewGuid().ToString("N"))
        $mirrorExit = 0
        try {
            $robocopyExe = (@(Microsoft.PowerShell.Core\Get-Command robocopy -CommandType Application -ErrorAction Stop)[0]).Source
            & $robocopyExe . $mirror /MIR /XD .git target /XF .git | Out-Null
            if ($LASTEXITCODE -ge 8) { throw "mirror copy failed (robocopy $LASTEXITCODE)" }
            if (Test-Path -LiteralPath (Join-Path $mirror '.git')) {
                throw "mirror is not VCS-free (.git present)"
            }
            Push-Location $mirror -ErrorAction Stop
            try {
                & $pythonExe tools/check_release_metadata.py
                $mirrorExit = $LASTEXITCODE
            }
            finally {
                Pop-Location -ErrorAction SilentlyContinue
            }
            if ($mirrorExit -ne 0) { throw "release-metadata mirror check failed ($mirrorExit)" }
        }
        finally {
            Remove-Item -Recurse -Force $mirror -ErrorAction SilentlyContinue
            if (Test-Path -LiteralPath $mirror) {
                throw "mirror cleanup failed: $mirror still present"
            }
        }
    }

    # Changed-path allowlist — hard gate over committed and local
    # changes. Union of: the committed PR diff against the exact pinned
    # base (direct two-tree comparison, never a merge-base range),
    # unstaged tracked changes, staged changes, and untracked files.
    # --no-renames exposes a rename as its old-path deletion plus
    # new-path addition, so an out-of-allowlist source path cannot
    # hide behind a rename. Index flags are rejected outright: an
    # assume-unchanged or skip-worktree entry could hide a tracked
    # modification from the diff collectors. Every collector is
    # exit-checked before its output is used. Membership and equality
    # comparisons are case-sensitive, matching Git's exact path bytes.
    # Porcelain-free collection; `git status --short` is printed for
    # review, never parsed.
    $allowlist = @(
        "README.md",
        "docs/design/claim-inline-subject-binding-v0.md",
        "docs/design/claim-inline-sha256-predicate-v0.md",
        "docs/design/standing-view-evidence-ceilings.md",
        "tickets/0058-claim-inline-sha256-predicate-contract.md"
    )
    $committed = @(& $gitExe diff --name-only --no-renames $base HEAD --)
    if ($LASTEXITCODE -ne 0) { throw "git diff BASE HEAD failed" }
    $unstaged = @(& $gitExe diff --name-only --no-renames)
    if ($LASTEXITCODE -ne 0) { throw "git diff failed" }
    $staged = @(& $gitExe diff --cached --name-only --no-renames)
    if ($LASTEXITCODE -ne 0) { throw "git diff --cached failed" }
    $untracked = @(& $gitExe ls-files --others --exclude-standard)
    if ($LASTEXITCODE -ne 0) { throw "git ls-files failed" }
    $indexTags = @(& $gitExe ls-files -v)
    if ($LASTEXITCODE -ne 0) { throw "git ls-files -v failed" }
    $flagged = @($indexTags | Where-Object { $_ -cmatch '^[^H]' })
    if ($flagged.Count -gt 0) {
        throw "FAIL: assume-unchanged or skip-worktree index flags present — clear them before validating: $($flagged -join '; ')"
    }
    $all = @($committed) + @($unstaged) + @($staged) + @($untracked) |
        ForEach-Object { $_ -replace '\\', '/' } |
        Where-Object { $_ -ne "" } |
        Sort-Object -CaseSensitive -Unique
    $outside = @($all | Where-Object { $_ -cnotin $allowlist })
    if ($outside.Count -gt 0) {
        throw "FAIL: unauthorised changed path(s): $($outside -join ', ')"
    }
    $committedSorted = @($committed |
        ForEach-Object { $_ -replace '\\', '/' } |
        Where-Object { $_ -ne "" } |
        Sort-Object -CaseSensitive -Unique)
    $allowSorted = @($allowlist | Sort-Object -CaseSensitive)
    if (($committedSorted -join "`n") -cne ($allowSorted -join "`n")) {
        throw "FAIL: committed PR diff is not exactly the five allowlisted paths: $($committedSorted -join ', ')"
    }

    # Report for human review (not parsed)
    & $gitExe status --short
    if ($LASTEXITCODE -ne 0) { throw "git status failed" }
    & $gitExe diff --name-only
    if ($LASTEXITCODE -ne 0) { throw "git diff --name-only failed" }
    & $gitExe diff --stat
    if ($LASTEXITCODE -ne 0) { throw "git diff --stat failed" }
}
finally {
    if ($cargoEnvExisted) {
        [System.Environment]::SetEnvironmentVariable("CARGO", $prevCargoEnv, "Process")
    } else {
        Remove-Item Env:CARGO -ErrorAction SilentlyContinue
        if (Test-Path Env:CARGO) { throw "CARGO restore failed" }
    }
    if ($nativePrefExisted) {
        $PSNativeCommandUseErrorActionPreference = $nativePrefValue
    } else {
        Remove-Variable -Name PSNativeCommandUseErrorActionPreference -Scope Local -ErrorAction SilentlyContinue
    }
}
```

Because this change is documentation-only, cargo outputs must remain
identical to the exact base `67a87e01575a9e9930ba3dc1ec52b95f963b6136`.
Required applications (`git`, `cargo`, `python`, and `robocopy` on the
mirror fallback path) are resolved once to their first concrete
Application path with terminating checks and invoked by that path: a
command-resolution failure never sets `$LASTEXITCODE`, multiple
installed matches cannot produce an uninvokable array, and a function
or alias shadow cannot substitute a different command. Native-exit
promotion (the PS 7+ opt-in preference) is neutralized for the block
by a current-scope override — probed and restored (or removed) safely
under `Set-StrictMode` — so interpreted exit codes always reach the
gates. Every native command in the block is followed by an immediate
exit-code check, so no failed check can be overwritten silently by a
later success. The tour gate captures the executable's exact stdout
bytes through Python's binary `subprocess` interface, with the checker
program delivered to Python on standard input — never as a native
argument, whose quoting rules differ across PowerShell versions, and
never through PowerShell redirection or a string variable, which can
re-encode output — and requires exactly 1917 bytes with SHA-256
`48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f`;
it prints the actual byte length and digest on success and creates no
temporary file, so nothing is left behind on success or failure. The
changed-path gate collects the committed PR diff against the exact
base plus staged, unstaged, and untracked local paths with rename
detection disabled, so an out-of-allowlist source path cannot hide
behind a rename, and rejects assume-unchanged and skip-worktree index
flags outright, so a tracked modification cannot hide behind a flag;
membership and equality comparisons are
case-sensitive, matching Git's exact path bytes. It requires every
changed path to be inside the five-path allowlist and requires the
committed PR diff to be exactly those five paths, so a clean committed
branch cannot pass vacuously and a missing authorised file cannot pass
silently. Any violated invariant stops validation immediately and
reports the actual offending value. The release-metadata checker may
refuse the intentionally dirty worktree; validation then stops until
the unchanged script passes in a temporary VCS-free mirror, which is
removed afterwards even on failure. No `--allow-dirty`. No golden or
fixture regeneration.

## 23. Publication authority

At coordinator handoff the contract candidate is uncommitted and
unpushed; K3 performs no commit, push, or GitHub metadata action. The
user retains sole publication, readiness, and merge authority.

## 24. Suggested reviewer checklist

1. Exactly the five authorised paths changed; the subject-binding note
   carries only a narrow handoff/status clarification with its ratified
   law untouched; no runtime, test, fixture, Cargo, L0, format, golden,
   verifier, or CI change.
2. The proposition is exactly SHA-256 of the decoded claim-owned
   subject bytes compared with the claim-owned expected digest;
   encoded-hex and statement-byte readings are explicitly excluded.
3. `subject_hex` and `expected_sha256` are separate claim-owned
   operands from the replayed claim; the expected operand does not
   independently identify or commit to the subject, and the evaluator
   accepts only `&self` and one claim ID.
4. The outcome classification vocabulary is exactly `DigestEqual`,
   `DigestUnequal`, `ResolutionFailed` with the neutrality laws; the
   authority-bearing outcome is an opaque privately constructed struct
   with read-only inspection and no reclassification or receipt move,
   and no v0 or withdrawn 0056 outcome name is reused.
5. The failure enum is closed and ordered exactly as ratified;
   `DigestUnequal` is reachable only from the terminal unequal
   comparison.
6. The receipt carries exactly the eight ratified fields and no
   prefix identity, subject length, raw subject copy, or
   standing/policy/evidence/edge/polarity field.
7. Private-construction doctrine holds; no public failure-payload
   constructor or requested-claim-ID replacement exists, and serialized
   values and borrowed requested IDs are audit material only.
8. The exact adjacent `outcome` / `details` canonical profile,
   lowercase tags, terminal payload shapes, successful receipt field
   order, failed-detail `claim_id` / `reason` order, all 33 failure
   strings, escaping law, and three literal vector lengths/hashes are
   pinned; both successful vectors are unchanged and no alternate
   representation is canonical.
9. `requested_claim_id()` returns the exact evaluator query key for all
   three kinds; success uses the receipt's sole `claim_id`, while
   failure retains private query attribution that proves no claim
   existence. No claim-ID maximum, normalization, rejection, or new
   failure was invented, and failure serialization grows linearly with
   the exact query string.
10. Ticket 0057's resource law remains unamended: the bound precedes
   every resolver-owned identifier allocation or copy. Pre-existing
   upstream replay/application allocation or copying before resolver
   entry is outside that law and cannot be undone, but gives the
   resolver no permission to mutate/extend input or copy any complete or
   partial identifier into fixed-capacity or heap storage. Ticket 0058's
   borrowed
   count-only first pass retains bounded non-identifier scalar state
   only, rejects decoded byte 65, and permits accepted-value
   decoding/copy/allocation only in the second pass.
11. The versioning boundary is fail-closed in both directions; hybrids
   fail `UnknownSchema` or `UnknownPredicateId`.
12. The Ticket 0056 arbitrary-byte attack is answered by construction
   and reproduced as the central hostile case.
13. The attestation deferral states exactly: inherited from Ticket
    0057 — the four routing/binding fields (`schema`, `predicate_id`,
    `subject_claim_id`, `scope_ref`), `predicate_id` equal to the
    validated claim predicate identity, no alternate subject byte
    source and no non-routing fields, one same-replay
    non-caller-substitutable binding path; genuinely open — schema
    identity, envelope identity, duplicate/unknown-key handling and
    failure precedence, binding mechanics within that already-fixed
    same-replay path, field requiredness, evidence
    kind. No attestation schema or edge-polarity meaning is ratified;
    the sequence and do-not-bundle law are stated; the aggregation note
    is untouched and its staleness is recorded as deferred.
14. The evaluator boundary has one complete permitted-scope list and
    retains every attestation, polarity, support, refutation, standing,
    and contradiction exclusion.
15. The ticket and the design note agree exactly.
16. Every reported validation actually ran.
