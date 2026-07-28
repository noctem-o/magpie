# Claim-Inline SHA-256 Predicate v0

## Status

Contract ratified by Ticket 0058. Documentation only.

```text
subject-binding substrate:
claim-invariant byte-subject identity — ratified by Ticket 0057

predicate identity:
sha256_claim_inline_bytes_equals_v0 — ratified (contract only)

evaluation outcomes:
DigestEqual / DigestUnequal / ResolutionFailed — ratified as neutral,
standing-inert vocabulary

policies, standing rules, support, refutation, attestation schemas:
none ratified

runtime:
not implemented
```

This note ratifies one neutral, claim-inline SHA-256 predicate contract
over the subject-binding substrate of Ticket 0057. It ratifies no
standing policy, no support or refutation rule, no evidence attestation
schema, no edge-polarity meaning, and no runtime. It adds no Rust,
tests, fixtures, Cargo metadata, dependency, CI, or L0 surface.

## Purpose

Ticket 0057 established claim-invariant byte-subject identity: an
`ExactMachineCheckable` claim independently commits to exactly one
immutable byte subject through claim-owned inline subject bytes, and the
complete subject descriptor is carried injectively in the canonical
statement. What Ticket 0057 deliberately did not ratify is the
predicate that evaluates over that resolved subject.

This contract ratifies exactly that predicate and nothing more:

```text
SHA-256(exact decoded claim-owned inline subject bytes)
==
the exact claim-owned expected_sha256 value
```

The predicate is polarity-neutral. The same resolution and digest
computation path produces both terminal relation outcomes; later,
separately ratified positive-support and direct-refutation policies may
consume the neutral outcome, but this contract assigns no outcome any
standing meaning, and it grants no standing itself.

## The exact proposition

The proposition an inline-subject claim asserts — and the only
proposition the predicate evaluates — is:

```text
SHA-256(hex_decode_strict_lowercase(
    subject_hex from the strictly resolved replayed claim))
==
hex_decode_strict_lowercase(
    expected_sha256 from that same claim's descriptor)
```

The proposition ranges over the **decoded subject bytes** only. It is
not SHA-256 of the encoded hex characters, and it is not SHA-256 of the
canonical statement. `subject_hex = "616263"` asserts a proposition
about the three bytes `abc`; hashing the six ASCII characters `616263`
or hashing the statement evaluates a different, unrelated proposition.
The existing v0 checker hashes decoded bytes and compares digest bytes
in exactly this way
(`crates/magpie-claims/src/deterministic_verifier_context.rs`).

The two digest roles remain distinct, exactly as in the v0 family:

```text
expected_sha256 (descriptor)
    commits to the claim-owned subject bytes

ClaimAssertedV2.content_hash
    commits to the exact UTF-8 canonical statement bytes
```

`content_hash` is never a subject digest and never enters the terminal
comparison. `expected_sha256` is **proposition data**: the claim author
chooses the subject and the expected digest at assertion time, and that
choice defines the proposition (Ticket 0057, authority and construction
boundary). The evaluator reads both operands from the replayed claim; no
caller, evidence node, edge, or audit value may supply or replace either
operand. A caller able to substitute the freshly computed digest for the
claim's expected digest could turn a false proposition into a true one;
the authority path below forecloses that by construction.

The proposition is two-sided decidable after successful resolution:
given one valid decoded byte string and one valid 32-byte expected
digest, exactly one of digest equality or digest inequality holds. This
is the property the Ticket 0056 candidate lacked: there, each evidence
node selected its own byte subject, so inequality proved nothing about
the claim. Here the subject is claim-fixed, so inequality over the same
claim-owned bytes is a genuine, reproducible relation outcome — still
without any standing meaning of its own. The outcome proves only the
digest relation. SHA-256 is not injective; digest equality never
becomes unique object identity, and digest inequality never becomes
falsity of anything beyond the named relation.

## Predicate identity

```text
sha256_claim_inline_bytes_equals_v0
```

The identity names the algorithm (`sha256`), the source and subject
type (`claim_inline_bytes`), the relation (`equals`), and the version
(`v0`). It is a new identity, exactly as Ticket 0056's blocker and
Ticket 0057's versioning boundary require: `sha256_bytes_equals_v0`
keeps its evidence-local positive meaning and is not reinterpreted,
aliased, or extended. The identity is 35 ASCII bytes of closed
`[a-z0-9_]+`, within the compiled `MAX_PREDICATE_ID_BYTES_V0 = 64`
bound ratified by Ticket 0057.

The identity is **compiled into the future evaluator** as a constant.
A descriptor whose `predicate_id` parses cleanly but is not byte-equal
to the compiled constant fails closed (`UnknownPredicateId`); the
evaluator never evaluates a generic or caller-selected predicate. This
mirrors the existing v0 checker, which compares the parsed
`predicate_id` against its compiled constant and fails
`UnknownPredicateId` before evaluation.

## Neutral outcome vocabulary

The closed, polarity-neutral outcome vocabulary is:

```text
Sha256ClaimInlineBytesPredicateOutcomeV0::DigestEqual { receipt }
Sha256ClaimInlineBytesPredicateOutcomeV0::DigestUnequal { receipt }
Sha256ClaimInlineBytesPredicateOutcomeV0::ResolutionFailed { reason }
```

`DigestEqual` and `DigestUnequal` each carry one privately constructed
receipt recording the exact audit basis of the terminal comparison.
`ResolutionFailed` carries one closed reason from the resolution-failure
vocabulary below and no receipt.

The names are deliberately relation names, not verdict names. The
candidate labels `Satisfied` / `Unsatisfied` were audited and rejected:
they sit one morpheme from the withdrawn Ticket 0056 vocabulary
(`PredicateSatisfied` / `PredicateFalse` — withdrawn identifiers that
must not be relabeled), and a name like `Unsatisfied` invites exactly
the failure-to-falsity collapse this contract forbids. The existing v0
`Matched` / `DigestMismatch` trace tokens keep their evidence-local
positive meaning and are not reused, wrapped, re-exported, or
reinterpreted; `DeterministicVerifierContextTraceV0::DigestMismatch`
remains unchanged, evidence-local, and non-authoritative.

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

The same subject-resolution and digest-computation path produces both
`DigestEqual` and `DigestUnequal`. They differ only at the terminal
exact digest comparison. There are no separate positive and negative
subject resolvers, and `DigestUnequal` is constructible only after
every resolution prerequisite has succeeded and the 32 computed digest
bytes differ from the 32 strictly decoded expected digest bytes. Every
prerequisite failure is `ResolutionFailed` — never `DigestUnequal`,
never a negative result, never evidence of anything.

The empty subject is ordinary and exact: `subject_hex = ""` decodes to
zero bytes, whose SHA-256 is
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
That expected digest yields `DigestEqual`; any other valid expected
digest yields `DigestUnequal`. The empty-subject canonical statement
ends with an empty final segment, and its statement content hash is
`afc617e9136762ccb1218ef474511467bad0d20bff01d8981e4cb983c18d67aa`.

## Closed resolution-failure vocabulary

`ResolutionFailed` carries exactly one variant of
`ClaimInlineSubjectResolutionFailureV0`, a closed enum evaluated in
this frozen first-failure order:

```text
 1. MissingClaim                    claim ID absent from the legacy
                                    StandingClaim table
 2. MissingTypedClaim              claim ID absent from the typed
                                    claim table
 3. MalformedMetadata              outer metadata is not one complete
                                    JSON object (malformed, trailing
                                    content, or non-object)
 4. MissingClaimDomain
 5. WrongTypeClaimDomain
 6. DuplicateClaimMetadataKey      duplicate key at the outer level
 7. UnknownClaimDomain             value outside the closed ClaimDomain
                                    vocabulary
 8. ClaimDomainMismatch            parsed domain is not exactly
                                    ExactMachineCheckable
 9. UnknownClaimMetadataKey        outer key other than claim_domain or
                                    machine_predicate_inline_bytes
10. MissingInlineSubjectDescriptor
11. WrongTypeInlineSubjectDescriptor
12. DuplicateDescriptorKey
13. UnknownDescriptorKey
14. MissingSchema
15. WrongTypeSchema
16. UnknownSchema                  not magpie-machine-predicate-inline-bytes-v0
17. MissingPredicateId
18. WrongTypePredicateId
19. EmptyPredicateId
20. PredicateIdTooLong             beyond MAX_PREDICATE_ID_BYTES_V0 = 64,
                                    checked before character validation,
                                    statement construction, or allocation
21. InvalidPredicateId             characters outside closed [a-z0-9_]+
22. UnknownPredicateId             well-formed but not byte-equal to the
                                    compiled sha256_claim_inline_bytes_equals_v0
23. MissingSubjectHex
24. WrongTypeSubjectHex
25. SubjectHexTooLong              beyond 8192 encoded characters,
                                    rejected before decoding
26. InvalidSubjectHex              odd length, uppercase, or non-hex
27. MissingExpectedSha256
28. WrongTypeExpectedSha256
29. InvalidExpectedSha256          not exactly 64 lowercase hexadecimal
                                    characters — no prefix, no uppercase,
                                    no whitespace — validated before
                                    canonical statement construction
30. StatementBindingMismatch       any of the three-way direct statement
                                    comparisons fails
31. MissingClaimContentHash        replayed claim content_hash is empty
32. ClaimContentHashMismatch       content_hash != recomputed lowercase
                                    SHA-256 of the exact UTF-8 statement
33. DecodedSubjectTooLarge         decoded subject beyond
                                    MAX_INLINE_SUBJECT_BYTES_V0 = 4096,
                                    checked after decode (unreachable when
                                    25 and 26 hold; retained as a compiled
                                    guard, never as falsity)
```

Every variant is a closed audit failure with no standing effect and no
negative meaning. The order is part of the contract: one evaluation
returns at most one reason, the first in this order. The prefix
(1-9) is this family's own frozen structural order (malformed, missing,
wrong-type, duplicate, unknown) extended with its exact two-key
envelope. It deliberately does not mirror the existing v0 claim-domain
parser, whose visitor records a duplicate key on encounter and can
therefore report a duplicate before a missing or wrong-typed
`claim_domain`; the new family freezes one total order of its own.
10-33 follow Ticket 0057's ratified resolution
procedure: descriptor fields in descriptor order (schema, predicate_id,
subject_hex, expected_sha256), then statement binding, then the separate
content-hash recomputation, then decode.

## Exact authority path

```text
one successful LogReader verification
→ one private composite replay
→ one StandingReplaySnapshot (private construction)
→ one same-call evaluation:
    re-fetch the typed claim and the legacy claim by one claim ID
    → strict outer-envelope parse
    → strict inline descriptor parse
    → compiled predicate-identity equality
    → descriptor-derived canonical statement
    → exact three-way statement equality (direct byte comparison)
    → separate claim content-hash recomputation
    → bounded subject decode from the claim only
    → terminal 32-byte digest comparison
→ one Sha256ClaimInlineBytesPredicateOutcomeV0
```

The conceptual future trusted query is:

```rust
StandingReplaySnapshot::evaluate_sha256_claim_inline_bytes_equals_v0(
    &self, claim_id: &str,
) -> Sha256ClaimInlineBytesPredicateOutcomeV0
```

The surface takes `&self` and one claim ID and nothing else. No caller
supplies subject bytes, expected digest, predicate result, parser
output, resolved subject, descriptor, metadata, statement, scope,
receipt, outcome, authority table, policy, standing, evidence ID, edge
ID, closure, callback, content provider, snapshot fragment, or store.
Claim-ID selection is not substitution: requesting claim C evaluates
C's own proposition. Replay maps are first-write-wins, so a later
duplicate `ClaimAssertedV2` cannot overwrite the resolved claim.

The contracted future evaluation hangs on `StandingReplaySnapshot` — the
same layer that owns
the existing standing-inert v0 verifier query — because the predicate
needs only the replayed claim tables and grants no standing, origin
admission, or closure consumption. `OriginAdmissionReplayContextV0` and
its `VerifiedLogPrefixIdentityV0` belong to later outer policies that
genuinely need prefix, closure, or origin composition; the neutral
predicate does not, and its receipt asserts no verified-prefix
provenance it cannot honestly own.

The return type is total: there is no `Option`. A missing or
unavailable claim is an explicit `ResolutionFailed` reason, not absence
of a value.

## Exact parser and evaluation order

One evaluation performs, in this frozen order:

1. Re-fetch the typed claim and the legacy claim from the same verified
   snapshot by the one claim ID (failures 1-2).
2. Strictly parse the outer metadata envelope with one duplicate-aware
   whole-envelope parse: exactly `claim_domain` and
   `machine_predicate_inline_bytes`, no other outer keys, no duplicate
   keys at either level, no descriptor fields at the outer level;
   `claim_domain` revalidated through the existing strict claim-domain
   parser and required to equal `ExactMachineCheckable` (failures 3-11).
   Duplicate detection is post-unescape: two keys equal after JSON
   string decoding are one duplicate key. The canonical statement is
   derived from decoded values; raw token bytes are not identity. The
   existing claim-domain parser alone never constitutes descriptor
   authority.
3. Strictly parse the nested descriptor (failures 12-29): unknown
   schema rejected; `predicate_id` checked missing, wrong-type, empty,
   then beyond the compiled `MAX_PREDICATE_ID_BYTES_V0 = 64` bound —
   before character validation, canonical statement construction, or
   any allocation — then characters outside closed `[a-z0-9_]+`, then
   byte-unequal to the compiled `sha256_claim_inline_bytes_equals_v0`
   constant; `subject_hex` checked missing, wrong-type, then beyond
   8192 encoded characters — rejected before decoding — then odd
   length, uppercase, or non-hex; `expected_sha256` checked missing,
   wrong-type, then not exactly 64 lowercase hexadecimal characters —
   no prefix, no uppercase, no alternate encoding, no whitespace —
   validated before canonical statement construction.
4. Derive the canonical statement from the strictly parsed descriptor
   and require exact three-way statement equality by direct byte
   comparison — `StandingClaim.statement == TypedClaimNode.statement
   == descriptor-derived canonical statement` (failure 30). No
   normalization of whitespace, Unicode, case, or prose.
5. Require a non-empty replayed `ClaimAssertedV2.content_hash` and
   recompute it separately as the lowercase SHA-256 of the exact UTF-8
   statement bytes (failures 31-32). Collision resistance is assumed;
   hash equality is never identity and never replaces a direct
   comparison.
6. Decode the subject from the claim — never from evidence, edges,
   caller arguments, closures, stores, or any other source — enforcing
   the compiled `MAX_INLINE_SUBJECT_BYTES_V0 = 4096` decoded bound
   after decode (failure 33).
7. Compute `SHA-256` over exactly the decoded subject bytes and compare
   the 32 computed bytes with the 32 strictly decoded expected digest
   bytes: equal yields `DigestEqual(receipt)`; unequal yields
   `DigestUnequal(receipt)`.

The evaluator is deterministic, total or fail-closed, commandless,
networkless, pluginless, callback-free, environment-independent, and
free of ambient mutable policy. It performs no file, artifact, closure,
CAS, second-store, or network read. Two evaluations of the same claim
on the same snapshot produce equal outcomes and byte-identical
serialized audit output; there is no clock, nonce, counter, or
traversal-dependent field. Repetition is audit-only and never
amplifies.

## Receipt boundary

The receipt's only job is auditability of the terminal comparison.
The smallest sound field set, evaluated field-by-field against the
audit questions (independently derived? needed to audit? recomputable?
a second authority source? duplicating a nested record? implying
standing or admission?), is exactly eight fields:

```text
predicate_schema       magpie-machine-predicate-inline-bytes-v0 —
                       the versioned profile actually parsed
predicate_id           sha256_claim_inline_bytes_equals_v0 —
                       proves the exact compiled predicate ran, never a
                       generic fallback
claim_id               exact replay attribution and query identity
scope_ref              the replayed claim's exact scope — routing and
                       later policy-binding context only, never truth
canonical_statement    the exact standing-facing proposition that
                       passed direct three-way binding
claim_content_hash     the replayed statement-commitment field whose
                       independent rebinding succeeded — never the
                       subject digest
expected_sha256        the exact claim-owned terminal operand
computed_sha256        the independently derived terminal result
```

Excluded candidates and why:

- **verified-prefix identity** — `StandingReplaySnapshot` owns no such
  field; including one would fabricate provenance or require caller
  input. A later outer resolution that needs prefix identity records it
  from the context that owns it.
- **subject length** — exactly recomputable as `subject_hex.len()/2`
  from the retained canonical statement. The v0 receipt needed
  `witness_len` because the witness bytes are not in its receipt; here
  the complete subject is already visible in the statement, so a length
  field is derived redundancy, not audit necessity.
- **raw subject bytes** — already committed by the canonical statement;
  a second copy adds no audit value and tempts byte-source confusion.
- **any `Status`, policy identity, evidence or edge ID, contribution,
  admission result, or polarity** — a predicate receipt is not standing
  material and carries none.

Any future authority-bearing outcome, receipt, or application type in
this family is privately constructed: private fields, read-only
getters, `Serialize` only, no `Deserialize`, no `Default`, no public
constructor, no struct-literal construction, and no public mutation.
No resolver, evaluator, or composer ever accepts such a value back as
authority input. Cloning, serializing, or deserializing any value
yields audit material only. The internal decoded-subject value used
mid-evaluation is private, consumed in the same call, and never
published as a second authority source.

## Versioning boundary

`magpie-machine-predicate-v0`, `sha256_bytes_equals_v0`,
`magpie-verification-witness-v0` and its `witness_hex` field,
`DeterministicVerifierContextTraceV0` (including `Matched` and
`DigestMismatch`), and standing policies v0-v3 remain byte- and
behavior-identical. The new predicate identity is new; nothing in this
contract reinterprets the v0 family's evidence-local proposition.

The boundary is fail-closed in both directions:

- The v0 strict predicate parser rejects the inline schema's outer key
  `machine_predicate_inline_bytes` as an unknown key, so an
  inline-subject claim presented to the v0 path fails closed there; the
  v2 lane's behavior is unchanged by this contract.
- The inline family's strict parser rejects the v0 envelope
  (`machine_predicate`), a missing `machine_predicate_inline_bytes`
  key, and the v0 schema; a hybrid descriptor naming the new schema but
  the v0 predicate identity fails `UnknownPredicateId`; a hybrid naming
  the v0 schema fails `UnknownSchema`. No parser dispatches on
  `claim_domain` alone, and no family adapts the other's statement or
  synthesizes missing fields.

No L0 payload, tag, canonical encoding, `docs/FORMAT.md`, golden
vector, or Python verifier change is required or permitted: claim
`metadata_json` remains opaque to L0.

## Polarity neutrality and standing inertia

This contract ratifies no standing policy, no support rule, no
refutation rule, no conflict or precedence rule, no contradiction debt,
no invalidation, no supersession or currentness rule, no evidence-kind
assumption, no edge-kind interpretation, and no attestation schema.
Nothing in the evaluation reads an evidence node, an edge, a closure,
or any policy surface, so nothing about `supports` or `contradicts`
can alter the evaluated subject or the truth outcome. A bare edge of
any kind changes nothing.

Positive-support and direct-refutation policies may later be ratified
to consume this neutral outcome; each must invoke this same
snapshot-only path internally and may not accept caller-created
outcomes or receipts. `ResolutionContentClosureV0`,
artifact-provenance verification, origin admission, and the standing
resolvers are not consumed. No ambient lookup, second store read,
network, filesystem, CAS, callback, or loader is introduced.

## Attestation decision: deferred

Independent audit (Sol C, adjudicated in Ticket 0058) compared three
options — predicate only; predicate plus a routing-only attestation
schema in this PR; predicate now with attestation committed as the next
separate contract — and recommended the third. K3 accepted.

The predicate is a complete claim-local decision: it resolves one
replayed claim, hashes its claim-owned bytes, and compares. An evidence
attestation is an independent evidence-format and binding decision that
introduces an outer evidence-metadata envelope, an evidence-kind
assumption, and a same-replay binding path. Two of its load-bearing
questions — the exact outer envelope identity and the accepted evidence
kind — do not yet have enough evidence to freeze cleanly, so this PR
ratifies no attestation schema and no field set beyond Ticket 0057's
already-ratified routing boundary, and assigns no meaning to `supports`
or `contradicts`.

The ratified sequence is:

```text
claim-inline SHA-256 predicate contract (this contract)
→ routing-only attestation-binding contract
→ support/refutation lane contract(s)
→ direct-refutation runtime
```

No later direct-refutation or support/refutation contract may bundle
the attestation schema with polarity and standing rules in one PR.

What is binding for that next contract is only what is already ratified
elsewhere: Ticket 0057's boundary — any future evidence attestation
consumed by the inline path may carry routing and binding fields only,
named there as `schema`, `predicate_id`, `subject_claim_id`,
`scope_ref`, with `predicate_id` equal to the already validated claim
predicate identity, and may supply no alternate subject byte source
(`witness_hex` or any other byte-carrying field is rejected by that
path). The same "routing and binding fields only" law excludes every
non-routing field by construction: no subject bytes, no expected-digest
override, no computed digest, no evaluation result, no receipt, no
standing contribution, and no policy identity may appear in such an
attestation. Binding itself must follow one same-replay,
non-caller-substitutable path whose serialized output is audit material
only — the standing doctrine of Ticket 0057's authority boundary. This
contract's neutrality laws add that attestation binding is not
predicate evaluation, predicate evaluation is not evidence polarity,
and evidence polarity is not standing contribution.
That inherited field boundary is Ticket 0057's ratified text: adding,
removing, or renaming a routing field would require its own separately
reviewed amendment of Ticket 0057, not silent reshaping here or there.

Everything further is recorded as **non-binding candidate material**
that the attestation contract owns and may reshape: the candidate
schema identity `magpie-inline-predicate-attestation-v0`; the distinct
outer evidence-metadata envelope identity; strict
duplicate/unknown-key handling with frozen failure precedence; the
exact claim-identity and scope binding mechanics within the
already-fixed same-replay path among attestation, replayed evidence,
and replayed claim; requiredness of individual
fields; and one explicit evidence-kind assumption. The two unresolved
load-bearing questions — the exact
outer envelope identity and the accepted evidence kind — belong to
that contract alone. If its freeze cannot be completed without
defining a support or refutation lane, the attestation contract must
stop rather than proceed. This contract ratifies none of that
candidate material.

## Hostile cases

The reference claim C used below: `subject_hex = "616263"` (the exact
bytes `abc`),
`expected_sha256 = ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad`,
canonical statement
`magpie-machine-predicate-inline-bytes-v0:sha256_claim_inline_bytes_equals_v0:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad:616263`,
`content_hash = aa1b8f1c339bc8e53c7db9d432e85073cc1495284279ba1b2474b74d26381223`,
any scope S.

| Case | Expected result under this contract |
| --- | --- |
| the Ticket 0056 attack recreated: a well-formed `contradicts` evidence node E- carries `witness_hex = 756e72656c61746564206d6174657269616c` ("unrelated material", digest `d42273bca408c671996c4f6eea5efefd010450655a44d742b0a2ba055b0542dc`), same `subject_claim_id = C`, same scope | `DigestEqual(receipt)` for C — evaluation selects only C, decodes only C's `616263`, computes `ba7816…15ad`, and never reads E- or the edge; removing all support evidence changes nothing. The manufactured-negative class is closed by construction |
| evidence metadata offering `subject_hex` or any byte field to the inline path | never consumed — the evaluator has no evidence input; a later attestation path must reject byte-bearing fields |
| edge metadata or rationale carrying subject bytes | never consumed — the evaluator has no edge input and does not traverse `justification_edges` |
| a `contradicts` edge targets C | edge polarity does not alter the evaluated subject or the truth outcome; the edge is not an evaluator input |
| a `supports` edge targets C | same as above; no edge kind is interpreted |
| C exactly as reference (subject `abc`, matching expected digest) | `DigestEqual(receipt)` |
| C's subject is `abc` and the claim-owned expected digest is any other valid 64-hex value | `DigestUnequal(receipt)` — a well-formed false proposition authored at assertion time; neutral, never `Refuted` |
| empty subject (`subject_hex = ""`) with the empty-byte digest `e3b0c4…b855` | `DigestEqual(receipt)` |
| empty subject with any other valid expected digest | `DigestUnequal(receipt)`, never `Refuted` |
| duplicate outer or nested metadata key (including post-unescape duplicates such as `"pred\u0069cate_id"`) | `ResolutionFailed(DuplicateClaimMetadataKey)` or `ResolutionFailed(DuplicateDescriptorKey)` |
| unknown outer or nested key | `ResolutionFailed(UnknownClaimMetadataKey)` or `ResolutionFailed(UnknownDescriptorKey)` |
| descriptor schema is `magpie-machine-predicate-v0` or any other value | `ResolutionFailed(UnknownSchema)` — no cross-family reinterpretation |
| descriptor names the new schema but `predicate_id = "sha256_bytes_equals_v0"` | `ResolutionFailed(UnknownPredicateId)` — well-formed syntax, wrong compiled identity |
| a v0-family claim (`machine_predicate` envelope) presented to the inline parser | `ResolutionFailed` — unknown outer key and missing descriptor; fail closed |
| an inline-subject claim presented to the v0 predicate parser | fail closed there as unknown outer key — v0 behavior unchanged |
| `expected_sha256` uppercase, `0x`-prefixed, whitespace-padded, or 63/65 characters | `ResolutionFailed(InvalidExpectedSha256)` before canonical statement construction |
| `subject_hex` odd-length, uppercase, non-hex, `0x`-prefixed, or beyond 8192 encoded characters | `ResolutionFailed` (`SubjectHexTooLong` or `InvalidSubjectHex`), never `DigestUnequal`; the 8192-character bound is enforced before any decode or allocation |
| `subject_hex` of 8194 encoded characters (4097 decoded bytes) | `ResolutionFailed(SubjectHexTooLong)` — the encoded bound rejects before any decode or allocation; `DecodedSubjectTooLarge` is an internal defense-in-depth guard, unreachable after failures 25-26, never caller-reachable, and never falsity |
| `StandingClaim.statement` differs from the descriptor-derived canonical statement | `ResolutionFailed(StatementBindingMismatch)` — exact direct comparison, no normalization |
| `TypedClaimNode.statement` differs from `StandingClaim.statement` | `ResolutionFailed(StatementBindingMismatch)` |
| `content_hash` differs from the recomputed statement hash despite statement agreement | `ResolutionFailed(ClaimContentHashMismatch)` |
| matching `content_hash` offered in place of the direct three-way statement comparison | rejected architecture — hash equality is never identity; the direct comparisons are never omitted |
| caller-created or deserialized resolved-subject value presented to a resolver | audit material only; never accepted as authority |
| caller-created, cloned, or deserialized evaluation receipt or outcome | unconstructible by struct literal; no `Deserialize`, `Default`, or public constructor; never accepted by any resolver, evaluator, or composer |
| a later attestation carrying `witness_hex` or any byte source | rejected by the inline path under Ticket 0057's inherited routing-only boundary (no alternate subject byte source); deferred to the attestation contract are exactly the six open dimensions — schema identity, envelope identity, duplicate/unknown-key handling and failure precedence, binding mechanics within the already-fixed same-replay path, field requiredness, and evidence kind; historical v0 `witness_hex` handling unchanged |
| an attestation `predicate_id` differing from the claim's predicate identity | rejected under Ticket 0057's inherited boundary — the attestation `predicate_id` must equal the already validated claim predicate identity; deferred to the attestation contract are exactly the six open dimensions — schema identity, envelope identity, duplicate/unknown-key handling and failure precedence, binding mechanics within the already-fixed same-replay path, field requiredness, and evidence kind |
| a resolver mixing `StandingClaim` from one snapshot with typed metadata from another | impossible by construction — one `&self` snapshot re-fetches both tables internally; no API accepts separate pieces |
| the same claim evaluated twice in one replay | equal outcomes and byte-identical serialized audit output; repetition never amplifies |

## What a receipt proves — and does not prove

Relative to the verified replay and the compiled predicate identity, a
receipt proves only that:

- the exact claim was co-replayed in one verified snapshot and strictly
  resolved under Ticket 0057's law;
- the descriptor named the exact compiled predicate identity;
- the canonical statement passed exact three-way direct binding and the
  claim content hash was independently rebound; and
- the computed SHA-256 of the exact decoded claim-owned subject bytes
  equals (`DigestEqual`) or differs from (`DigestUnequal`) the exact
  claim-owned expected digest.

It does not prove standing, support, refutation, settlement, admission,
evidence polarity, edge meaning, external artifact identity or origin,
acquisition, file contents, semantic correctness, interpretation truth,
unique object identity (SHA-256 is not injective), or publication
authority.

## Exact future implementation boundary

A future implementation ticket may add only additive, separately
reviewed surfaces: the strict whole-envelope parser (or exact reuse of
the existing strict machinery), the snapshot-only evaluator method, the
private-construction outcome/receipt/failure types, hostile tests, and
documentation. It must not change `sha256_bytes_equals_v0`, its parser,
trace, fixtures, policies v0-v3, L0, `docs/FORMAT.md`, golden vectors,
the Python verifier, Cargo manifests, CI, or any historical ticket. No
standing rule, support lane, refutation lane, or attestation schema may
be implemented in the same slice as the predicate evaluator.

## Validation commands (for this documentation change)

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
    # Standard checks (must pass)
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
    # changes. Union of: the committed PR diff against the exact base,
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
    $base = "67a87e01575a9e9930ba3dc1ec52b95f963b6136"
    $allowlist = @(
        "README.md",
        "docs/design/claim-inline-subject-binding-v0.md",
        "docs/design/claim-inline-sha256-predicate-v0.md",
        "docs/design/standing-view-evidence-ceilings.md",
        "tickets/0058-claim-inline-sha256-predicate-contract.md"
    )
    $committed = @(& $gitExe diff --name-only --no-renames "$base...HEAD")
    if ($LASTEXITCODE -ne 0) { throw "git diff BASE...HEAD failed" }
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

## Reviewer checklist

- Confirm only the authorised documentation paths changed.
- Confirm the proposition is exactly SHA-256 of the decoded claim-owned
  subject bytes compared with the claim-owned expected digest, and that
  the encoded-hex and statement-byte interpretations are explicitly
  excluded.
- Confirm `expected_sha256` is treated as proposition data read from
  the replayed claim, and the evaluator accepts only `&self` and one
  claim ID.
- Confirm the outcome vocabulary is exactly `DigestEqual`,
  `DigestUnequal`, `ResolutionFailed`, with the required neutrality
  laws, and that no v0 or withdrawn 0056 outcome name is reused.
- Confirm the resolution-failure enum is closed, ordered exactly as
  ratified, and that `DigestUnequal` is reachable only from the
  terminal unequal comparison after all prerequisites succeed.
- Confirm the receipt carries exactly the eight ratified fields and no
  verified-prefix identity, subject length, raw subject copy, status,
  policy, evidence/edge, contribution, admission, or polarity field.
- Confirm private-construction doctrine: no public constructor,
  struct literal, `Deserialize`, `Default`, or mutation; serialized
  values are audit material only.
- Confirm the bidirectional versioning boundary: new family fails
  closed in v0, v0 family fails closed in the new parser, hybrids fail
  `UnknownSchema` or `UnknownPredicateId`.
- Confirm the Ticket 0056 arbitrary-byte attack is answered by
  construction and reproduced as the central hostile case.
- Confirm the attestation deferral states exactly: inherited from
  Ticket 0057 — the four routing/binding fields (`schema`,
  `predicate_id`, `subject_claim_id`, `scope_ref`), `predicate_id`
  equal to the validated claim predicate identity, no alternate subject
  byte source and no non-routing fields, one same-replay
  non-caller-substitutable binding path; genuinely open — schema
  identity, envelope identity, duplicate/unknown-key handling and
  failure precedence, binding mechanics within that already-fixed
  same-replay path, field requiredness, evidence
  kind. Confirm no attestation schema and no edge-polarity meaning is
  ratified, and the sequence and do-not-bundle law are stated.
- Confirm `sha256_bytes_equals_v0`, its parser and trace, policies
  v0-v3, L0, FORMAT, fixtures, and the tour are untouched.
- Confirm the ticket and this note agree exactly.
- Confirm every reported validation actually ran.

## Next slice

The next separately reviewed slice is the routing-only
attestation-binding contract recorded in the attestation decision
above, followed only later by separately ratified support/refutation
lane contracts and any direct-refutation runtime. This contract
ratifies none of those.
