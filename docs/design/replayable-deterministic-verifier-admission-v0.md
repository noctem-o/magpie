# Replayable Deterministic-Verifier Admission v0

## Status and purpose

Ratified architecture contract with its first bounded policy consumer landed.
The contract, standing-inert strict checker/context, and explicit policy-v2
direct `Supported` rule are implemented. Genuine aggregation and conservative
independence remain next.

The purpose is to say exactly where deterministic-verifier authority may begin,
and to bound the first achieved-`Supported` rule that may consume it.

## Implementation mapping

- module: `crates/magpie-claims/src/deterministic_verifier_context.rs`;
- public snapshot method:
  `StandingReplaySnapshot::resolve_deterministic_verifier_context_v0`;
- public audit types: `DeterministicVerifierReceiptV0` and
  `DeterministicVerifierContextTraceV0`;
- private boundary: duplicate-aware metadata parsers, statement/hash binding,
  bounded witness decoder, and the closed SHA-256 checker;
- dependency boundary: existing workspace `hex` and `sha2` dependencies are
  added directly to `magpie-claims` only;
- hostile and literal audit fixtures:
  `crates/magpie-claims/tests/deterministic_verifier_context.rs`;
- `standing_v2.rs`: explicit policy-v2 types and resolver;
- `resolved_standing_with_trace_v2`: internally inherits v1 and invokes the
  snapshot-only verifier context;
- `StandingPolicyContextV2`: retains exact inherited Deadbolt or deterministic
  verifier traces;
- `deterministic_support_standing_v2.rs`: policy, precedence, blocker,
  non-amplification, and byte fixtures.

## Architectural law

```text
An evidence label is not verifier authority.

A witness supplied by an event is not a receipt.

A verifier receipt exists only when a closed deterministic checker
successfully re-evaluates exact replay-derived material under an
explicit policy.

A successful receipt may admit one bounded contribution.
It does not establish arbitrary semantic truth, external artifact
origin, interpretation truth, or publication authority.

A machine predicate cannot lend standing to adjacent prose.

The standing-bearing statement must be the exact canonical rendering
of the predicate, and the claim content hash must commit to that exact
statement.
```

The stages remain separate:

```text
typed evidence != verification witness
verification witness != successful verification
successful verification != admitted contribution
admitted contribution != aggregation
bounded contribution != general truth
achieved Supported != Settled
```

## Relation to tagged v0.1.0

The human-created `v0.1.0` tag names the reviewed source release at
`2bbfbd1f451e35b65e8c89aeaf39801621e484df`. This post-release design neither
rewrites that tag nor broadens its source, format, policy, replay, packaging,
publication, or absence claims. Policy v0 and policy v1 remain exactly the
released policies. The explicit post-release identity
`magpie-claims-standing-v2` is now implemented; it is selected only by
versioned snapshot methods and is not ambient policy.

## Premises and existing substrate

Provenance is not factual commitment. “Evidence E carries witness W” differs
from “claim C is accepted under policy P.” A signature, content hash,
attribution, receipt, or inclusion fact proves only its named proposition.

The current substrate provides:

- signed typed `ClaimAssertedV2`, `EvidenceRegistered`, and
  `JustificationEdgeRecorded` events;
- opaque, exactly retained `metadata_json` strings on claim and evidence nodes;
- exact IDs and opaque exact-match `scope_ref` values;
- closed `EvidenceKind`, `ClaimDomain`, and support-context classifications;
- v0 candidate traces, where `DeterministicVerification ×
  ExactMachineCheckable` has a `Settled` ceiling but still reports
  `RequiresVerifierContext` and contributes nothing;
- one private composite replay that creates `StandingReplaySnapshot` only after
  one complete `LogReader::replay` verification and co-fold;
- a policy-v1 direct Deadbolt lane that re-fetches replayed nodes and the edge
  instead of trusting candidate trace fields.

Current surfaces merely classify that deterministic verifier context is
required. They do not create it. The genuinely trusted current context is the
same-replay relationship between the private snapshot's `StandingView` and
`DeadboltAnchorIndex`, plus the exact Deadbolt context derived from them.

An `EvidenceKind::DeterministicVerification` value is producer-supplied
classification. `actor_class`, `content_hash`, prose, and metadata such as
`{"verified":true}` are also asserted data. None records an independently
computed result or proves trusted resolver provenance.

The existing metadata strings can honestly carry the first bounded predicate
and witness without changing L0. The proposition is solely about bytes carried
inside the exact evidence node. Anything about a file, URL, external artifact,
program output, acquisition path, or interpretation would require new artifact
identity, acquisition, origin, or semantic policy and is outside this contract.

## Candidate, witness, receipt, and contribution

- A **candidate** is v0's deterministic explanation of structurally eligible
  claim, evidence, and edge material. It is not verification.
- A **machine claim** is the jointly bound set of claim ID, exact canonical
  statement, statement content hash, and structured machine predicate. No
  element may be substituted independently or treated as adjacent annotation.
- A **verification witness** is untrusted L0-carried checker input inside the
  evidence node's opaque metadata. Being signed into Magpie preserves the
  assertion; it does not make the assertion correct.
- A **derived verifier receipt** is the successful `Matched` audit result of the
  closed checker over exact material co-replayed in one verified snapshot. A
  serializable receipt value is audit material; public construction of an
  identical value is not proof that the trusted resolver produced it.
- An **admitted contribution** is a policy-v2 decision consuming a successful
  same-snapshot receipt. The checker does not choose standing.

The witness is asserted data. The digest comparison is derived verification.
The `Supported` result is policy output. None of these establishes external
artifact origin or arbitrary truth.

## Initial closed predicate

The only predicate family is `sha256_bytes_equals_v0`:

```text
The raw byte witness carried by this exact evidence node,
under this exact claim, edge, and scope binding,
has SHA-256 digest H.
```

It does not mean an external file has digest H, a named artifact came from a
source, a URL returned the bytes, a program produced them, the bytes are safe
or semantically correct, or an interpretation is true.

The standing-bearing claim statement is not free prose. Its sole accepted
canonical rendering is:

```text
sha256_bytes_equals_v0:<expected_sha256>
```

Conceptually,
`canonical_statement(predicate) = "sha256_bytes_equals_v0:" +
predicate.expected_sha256`. The rendering uses ASCII only and has no leading,
trailing, or internal whitespace; newline; quotation marks; `0x` prefix;
uppercase hexadecimal; Unicode normalization; case folding; alias; explanatory
prefix or suffix; or equivalent-expression handling. Requiring this rendering
does not broaden the proposition checked. It prevents that exact proposition
from lending standing to unrelated prose.

## Claim metadata schema

For this predicate, the exact claim `metadata_json` object is:

```json
{
  "claim_domain": "ExactMachineCheckable",
  "machine_predicate": {
    "schema": "magpie-machine-predicate-v0",
    "predicate_id": "sha256_bytes_equals_v0",
    "expected_sha256": "<64 lowercase hexadecimal characters>"
  }
}
```

The outer object permits exactly `claim_domain` and `machine_predicate`; the
nested object permits exactly `schema`, `predicate_id`, and
`expected_sha256`. All fields are required strings. Unknown keys at either
level and duplicate keys anywhere fail closed. `expected_sha256` is exactly 64
ASCII lowercase hexadecimal characters, with no prefix or whitespace.

The existing v0 claim-domain parser remains unchanged: it reads
`claim_domain`, ignores the new non-domain key, and preserves its existing
duplicate-key failure. A later v2 predicate parser separately applies the
stricter whole-object schema. Thus v0 behaviour does not change and v2 cannot
reinterpret unrelated claim metadata as a machine predicate.

The complete claim-side representation stores three existing fields
separately:

```text
statement:
sha256_bytes_equals_v0:<expected_sha256>

content_hash:
SHA-256 over the exact UTF-8 bytes of that statement, rendered lowercase hex

metadata_json:
{
  "claim_domain": "ExactMachineCheckable",
  "machine_predicate": {
    "schema": "magpie-machine-predicate-v0",
    "predicate_id": "sha256_bytes_equals_v0",
    "expected_sha256": "<expected_sha256>"
  }
}
```

Metadata parsing alone is insufficient. Successful v2 admission requires
agreement among the structured metadata, replayed statement, and replayed
content hash. `statement` and `content_hash` remain existing
`ClaimAssertedV2` fields, not metadata keys, so this introduces no L0 shape
change. It makes the existing `docs/design/metadata-conventions.md` meaning —
`ClaimAssertedV2.content_hash` is the hash of canonical claim-statement bytes —
load-bearing only for this v2 predicate family; it does not redefine the field
globally or change L0 validation.

The two digest roles are distinct:

```text
machine_predicate.expected_sha256
    hashes the evidence witness bytes

ClaimAssertedV2.content_hash
    hashes the canonical machine-claim statement bytes
```

A matching witness digest cannot compensate for a missing or mismatched claim
content hash. The two digests are equal only by coincidence.

## Evidence witness schema

For this predicate, the exact evidence `metadata_json` object is:

```json
{
  "verification_witness": {
    "schema": "magpie-verification-witness-v0",
    "predicate_id": "sha256_bytes_equals_v0",
    "subject_claim_id": "<exact claim id>",
    "scope_ref": "<exact scope>",
    "witness_hex": "<strict lowercase even-length hexadecimal>"
  }
}
```

The outer object permits only `verification_witness`; the nested object permits
only the five shown keys. All are required strings. Duplicate or unknown keys
at either level fail closed. `witness_hex` contains only ASCII `0-9a-f`, has an
even number of characters, has no prefix or whitespace, and decodes to at most
`MAX_WITNESS_BYTES_V0 = 4096`. The empty string is allowed and means the empty
byte sequence; its digest is checked normally. The policy limit is compiled
into the checker and is never caller-controlled.

Unrelated evidence metadata may not coexist in a verifier attempt. A claimed
result field such as `verified`, `admitted`, `trusted`, or `result` is an
unknown key and rejects the witness rather than being ignored.

These schemas are policy-defined interpretations of existing opaque
`metadata_json`. They add no L0 field, payload tag, event encoding, or metadata
authority.

## Exact binding and candidate boundary

A verifier attempt requires all of the following from one private
`StandingReplaySnapshot`:

```text
edge_kind == "supports"
edge.source_id == evidence_id
edge.target_id == claim_id
evidence.evidence_kind == "DeterministicVerification"
claim_domain == "ExactMachineCheckable"
evidence.scope_ref == edge.scope_ref == claim.scope_ref == witness.scope_ref
witness.subject_claim_id == claim_id
witness.predicate_id == claim.machine_predicate.predicate_id
claim.machine_predicate.schema == "magpie-machine-predicate-v0"
witness.schema == "magpie-verification-witness-v0"
predicate_id == "sha256_bytes_equals_v0"
claim.statement == "sha256_bytes_equals_v0:" + expected_sha256
claim.statement == canonical_statement(claim.machine_predicate)
claim.content_hash is non-empty
claim.content_hash == sha256_lower_hex(UTF-8(claim.statement))
```

Candidate trace identities are routing hints, not authority. Lane execution
must re-fetch the claim, evidence, and edge by ID from the same private
snapshot, including the actual claim statement and content hash, then
revalidate every condition above.

`StandingView` currently retains `StandingClaim` and `TypedClaimNode` in
separate first-write-wins maps. Exact deterministic-verifier claim binding
therefore additionally requires:

```text
StandingClaim.statement
    == TypedClaimNode.statement
    == canonical_statement(machine_predicate)
```

The ordinary `StandingClaim` is part of replayed-node revalidation. A valid
typed predicate and witness cannot produce `Matched` when the standing-facing
claim statement differs. That eligible context attempt returns
`StatementPredicateMismatch`; no normalization or semantic equivalence is
permitted. The receipt schema need not grow because successful equality
collapses every consumed claim representation to its existing canonical
statement field.

## Strict parsing and checker algorithm

JSON parsing must be deterministic and duplicate-aware. It must require one
complete JSON object, reject trailing data, reject duplicate keys before value
selection, reject unknown/missing/wrong-type fields, and perform no Unicode,
identifier, scope, hash, or hexadecimal normalization. Unknown schemas and
predicate identifiers never fall through to a generic checker.

The closed checker is:

1. Receive only the private snapshot plus exact `claim_id`, `evidence_id`, and
   `edge_id` selected by the closed v2 lane.
2. Re-fetch the claim, evidence, and edge; validate supports/source/target
   structure, evidence kind, claim domain, and three-way replay scope equality:
   `claim.scope_ref == evidence.scope_ref == edge.scope_ref`.
3. Strictly parse the claim predicate.
4. Derive the exact canonical statement from that predicate.
5. Compare it byte-for-byte with the replayed claim statement.
6. Require a non-empty replayed claim content hash.
7. Compute SHA-256 over the exact UTF-8 bytes of the replayed statement.
8. Compare its lowercase hexadecimal rendering byte-for-byte with
   `claim.content_hash`.
9. Strictly parse the evidence witness.
10. Validate exact witness schema, predicate, and subject-claim bindings; then
    compare `witness.scope_ref` with the already matched replay scope, thereby
    completing four-way scope equality.
11. Validate and decode strict witness hex while enforcing the fixed 4096-byte
    decoded limit before allocation beyond that bound.
12. Compute SHA-256 over exactly the decoded witness bytes.
13. Compare the 32 computed witness-digest bytes to the strictly decoded
    `expected_sha256`.
14. Return `Matched` with audit material only if every prior check succeeds;
    otherwise return one
   closed failure and no partial successful context.

The checker is deterministic, total or fail-closed, commandless, networkless,
pluginless, callback-free, environment-independent, and free of ambient mutable
policy. It performs no file or artifact read.

## Closed verifier-context outcomes

The proposed derived trace is:

```text
DeterministicVerifierContextTraceV0::Matched {
    predicate_id,
    claim_id,
    evidence_id,
    edge_id,
    scope_ref,
    canonical_statement,
    claim_content_hash,
    expected_sha256,
    computed_sha256,
    witness_len,
}
```

Its failure variants are:

```text
MissingMachinePredicate
MalformedMachinePredicate
DuplicatePredicateKey
UnknownPredicateKey
UnknownPredicateSchema
UnknownPredicateId
InvalidExpectedDigest
StatementPredicateMismatch
MissingClaimContentHash
ClaimContentHashMismatch
MissingVerificationWitness
MalformedVerificationWitness
DuplicateWitnessKey
UnknownWitnessKey
UnknownWitnessSchema
ClaimBindingMismatch
ScopeBindingMismatch
PredicateBindingMismatch
InvalidWitnessHex
WitnessTooLarge
DigestMismatch
```

`StatementPredicateMismatch` means the replayed statement is not the exact
canonical rendering of the successfully parsed predicate.
`MissingClaimContentHash` means the existing claim content-hash field is empty.
`ClaimContentHashMismatch` means it differs from the lowercase SHA-256 digest
recomputed over the exact UTF-8 statement bytes. These are closed byte-binding
failures; there is no generic semantic-equivalence outcome.

Existing v0 candidate tracing owns missing replayed nodes, unsupported edge
kind, source/target structure, wrong evidence kind, wrong/missing claim domain,
and node/edge/claim scope mismatch. Therefore those checks are revalidated by
the v2 executor but are not duplicated as verifier-context variants. If
revalidation disagrees with the candidate, no context attempt is emitted and
the v2 lane records a closed `CandidateRevalidationFailed` classification.

Verifier-context tracing owns strict predicate and witness parsing, exact
statement derivation, statement/content-hash binding, predicate/witness
bindings, bounded decoding, and digest comparison. Statement and claim-hash
mismatches do not belong to v0 candidate tracing because v0 does not interpret
machine-predicate statement semantics. V2 lane classification owns whether the
exact v0 deterministic-verification candidate selects the one v2 rule.
Claim-level v2 blockers are deterministically derived from baseline blockers
and the complete v2 trace; they do not repeat the check as a second authority
source. Exact context failure remains in the application trace. Generic
unresolved `RequiresVerifierContext` and `CeilingIsCandidateOnly` remain in
blockers, while a successful application clears only its own generic
limitations. One unresolved path does not veto a successful path, and no new
blocker vocabulary is required.

## Trusted construction and bypasses

The sole legitimate origin is:

```text
one successful LogReader verification
→ one private composite replay
→ one StandingReplaySnapshot containing all required projections
→ exact candidate lane classification
→ replayed-node and edge revalidation
→ exact predicate, canonical statement, and claim content-hash binding
→ closed deterministic predicate checker
→ derived Matched verifier context
```

A successful receipt cannot authoritatively originate from a caller-created
public struct, separate projections or replay calls, a second store read, a
bare node or metadata object, a claimed result, function pointer, plugin,
subprocess, shell, network response, ambient verifier, or mutable global policy.

The trace may be serializable for audit. Serialization proves only that a value
has bytes. Trusted origin is a property of the resolver construction path, not
of the value's shape. Later APIs must not accept a public receipt as authority;
the snapshot-only resolver must invoke the checker internally.

## What a successful receipt proves

Relative to the externally selected Magpie verifying key and named checker
policy, `Matched` proves only that:

- the exact claim, evidence, and supports edge were co-replayed in one verified
  snapshot and satisfied the stated identity and scope bindings; and
- the replayed claim statement was the exact canonical rendering of the
  structured `sha256_bytes_equals_v0` predicate;
- the replayed claim content hash committed to the exact UTF-8 bytes of that
  statement; and
- the bytes decoded from that evidence node's witness have the expected SHA-256
  digest under `sha256_bytes_equals_v0`.

It does not prove correct trust-root choice, external artifact identity or
origin, acquisition, file contents, URL response, program execution, producer
identity beyond recorded provenance, semantic correctness, safety,
interpretation truth, arbitrary claim truth, independent corroboration, or
publication authority.

## Implemented policy-v2 boundary

The policy identity is exactly `magpie-claims-standing-v2`. It preserves
all v1 behaviour and adds exactly one direct-support family:

```text
DeterministicVerification
× ExactMachineCheckable
× exact canonical predicate statement
× matching claim statement content hash
× sha256_bytes_equals_v0
× successful same-snapshot deterministic verifier context
→ Some(Status::Supported)
```

A ceiling is a maximum. A reviewed policy may deliberately contribute less
than its ceiling. Although the existing cell permits `Settled`, the first rule
is intentionally capped at `Supported` to establish admission machinery without
creating a second settlement family.

Claim-level precedence is:

```text
existing successful Deadbolt occurrence rule → Settled
otherwise one or more successful deterministic-verifier rules → Supported
otherwise → v0 baseline
```

Multiple successful deterministic applications retain multiple audit traces
but do not create a stronger status, count, vote, score, quorum, independence
claim, corroboration result, or `Settled`. One unresolved path does not veto an
independently successful direct path, and one successful verifier path cannot
downgrade the inherited Deadbolt `Settled` result.

## Direct support is not aggregation

One successful machine-checkable verifier context directly contributes
`Supported` under one named rule. It does not combine weak evidence, count
sources, infer independence, or implement corroboration. Trace multiplicity is
audit material only. Genuine aggregation remains a later phase with its
existing independence laws intact.

## Versioned types and serialization

Extending the existing v1 public types would reduce duplication, but it would
broaden a serialized rule vocabulary whose policy identity currently names
exactly one v1 rule. Caller-created v1 values could then carry v2 identities,
and the relationship among policy ID, rule vocabulary, and canonical fixtures
would blur.

The recommendation is explicit v2 types:

```text
StandingPolicyRuleV2
StandingPolicyContextV2
StandingPolicyApplicationV2
StandingTraceEntryV2
StandingResolutionV2
```

`StandingPolicyRuleV2` contains both an inherited
`DeadboltOccurrenceInclusionV1` dispatch and a new, explicitly named
`Sha256BytesEqualsDirectSupportV0` dispatch under the v2 policy identity.
Conversions may reuse private logic, but v0 and v1 public serialized types,
variant sets, methods, behaviour, and literal canonical bytes remain unchanged.
No ambient latest-policy selector is added.

## Deterministic basis and canonical bytes

Every successful v2 application records rule identity, claim ID, evidence ID,
edge ID, exact scope, predicate schema and ID, exact canonical statement, claim
content hash, explicit successful statement-binding outcome, expected and
computed witness digests, witness byte length, exact verifier-context outcome,
and achieved contribution. The complete witness is not copied into the
resolution; it remains in the replayed evidence projection.

A claim ID plus predicate digest is insufficient audit basis because it does
not reveal whether the displayed standing-bearing proposition matched the
checked predicate. The canonical statement is small and makes that attribution
explicit.

A Boolean or count loses exact attribution, prevents precise replay comparison
and later invalidation, obscures hostile failures, and invites amplification.

The implementation:

- retain the literal v0 and v1 canonical-byte fixtures byte-for-byte;
- add literal positive and failure v2 fixtures covering field names, order,
  enum names, optional fields, blocker order, and all audit fields;
- prove v2 resolution and snapshot replay are byte-identical for identical
  signed histories; and
- treat these resolution bytes as policy-pinned derived test material, not L0
  canonical encoding or a new persistent protocol.

No `magpie-core-v1`, event encoding, golden vector, independent verifier, or
`docs/FORMAT.md` change is required or permitted by this design.

## Future hostile-test matrix

```text
bare_deterministic_verification_label_does_not_support
actor_class_does_not_create_verifier_authority
verified_boolean_does_not_create_verifier_authority
caller_constructed_receipt_does_not_prove_trusted_origin
arbitrary_statement_cannot_inherit_machine_predicate_support
legacy_statement_cannot_differ_from_typed_machine_statement
statement_predicate_mismatch_fails_closed
missing_claim_content_hash_fails_closed
claim_content_hash_mismatch_fails_closed
statement_whitespace_variant_fails_closed
statement_uppercase_digest_variant_fails_closed
statement_trailing_newline_fails_closed
witness_match_cannot_override_statement_mismatch
witness_match_cannot_override_claim_content_hash_mismatch
matched_trace_binds_canonical_statement_and_content_hash

sha256_witness_exact_match_supports_only_under_v2
sha256_witness_digest_mismatch_does_not_support
sha256_witness_wrong_claim_binding_does_not_support
sha256_witness_wrong_scope_does_not_support
sha256_witness_wrong_predicate_id_does_not_support

sha256_witness_malformed_hex_fails_closed
sha256_witness_uppercase_hex_fails_closed
sha256_witness_odd_length_hex_fails_closed
sha256_witness_too_large_fails_closed
sha256_witness_duplicate_keys_fail_closed
sha256_witness_unknown_schema_fails_closed
sha256_witness_unknown_predicate_fails_closed

independently_constructed_views_do_not_create_receipt
changing_store_cannot_mix_predicate_and_witness_snapshots
verification_failure_returns_no_partial_context

one_successful_receipt_reaches_supported_not_settled
multiple_successful_receipts_do_not_amplify
deadbolt_settled_precedes_deterministic_supported
v0_bytes_remain_unchanged
v1_bytes_remain_unchanged
v2_replay_is_byte_identical
```

Additional fixtures must cover empty witness success, the exact 4096-byte
boundary, 4097-byte rejection, unknown keys at both levels, trailing JSON,
wrong field types, missing nodes, edge source/target mismatch, all four scope
positions, expected-digest case/length errors, and deterministic blocker order.
The positive fixture must set `statement` to exactly
`sha256_bytes_equals_v0:<expected_sha256>` and recompute the correct claim
content hash over those exact UTF-8 bytes.

## End-to-end flow

```text
ClaimAssertedV2 structured machine predicate
+ exact canonical claim statement
+ matching claim statement content hash
+ EvidenceRegistered witness
+ supports edge
+ exact scope
+ verified one-snapshot replay
+ closed sha256_bytes_equals_v0 checker
→ replay-derived verifier receipt
→ one bounded policy-v2 Supported contribution
```

## Non-goals and questions requiring new infrastructure

This policy slice adds no payload, tag, projection, receipt type, second
checker, aggregation, independence, refutation, contradiction debt,
invalidation, supersession, currentness, `EpistemicGate`, ordinary
claim-bearing writer surface,
artifact acquisition, file/shell/network/plugin/callback authority, proof
system, formal verification, or model/lens ingestion.

It cannot honestly decide external artifact identity, acquisition provenance,
remote responses, filesystem contents, program execution origin, or semantic
interpretation. Those questions require separately reviewed L0 or artifact
infrastructure and separately named predicates. They must not be smuggled into
`sha256_bytes_equals_v0`.

## Implementation sequence

1. Contract — landed.
2. Standing-inert parser/checker/context — landed.
3. Review audit surface and hostile tests — landed.
4. Explicit policy-v2 direct `Supported` rule — landed.
5. Genuine aggregation and independence — next.

The later code PR must stop if exact parsing requires L0 changes; metadata
cannot express unambiguous bindings; a new payload or artifact store is needed;
the checker needs file, shell, network, plugin, callback, or ambient authority;
v0/v1 semantics or bytes would change; the contribution cannot remain exactly
`Supported`; or accurate documentation would require claiming external origin
or general truth. It must also stop if it cannot bind the standing-bearing
statement exactly to the predicate; would require semantic natural-language
equivalence; would permit arbitrary prose beside the machine predicate; cannot
recompute and check the claim content hash; or could display a proposition that
differs from the checked proposition.
