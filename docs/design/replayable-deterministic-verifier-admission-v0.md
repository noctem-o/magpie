# Replayable Deterministic-Verifier Admission v0

## Status and purpose

Proposed architecture contract. Nothing in this document is implemented or
active. It defines a later standing-inert verifier-context implementation and a
separate later policy-v2 implementation. It adds no authority by documentation.

The purpose is to say exactly where deterministic-verifier authority may begin,
and to bound the first achieved-`Supported` rule that may consume it.

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
released policies. This contract proposes the new identity
`magpie-claims-standing-v2`; it does not activate it.

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
```

Candidate trace identities are routing hints, not authority. Lane execution
must re-fetch the claim, evidence, and edge by ID from the same private
snapshot, then revalidate every condition above.

## Strict parsing and checker algorithm

JSON parsing must be deterministic and duplicate-aware. It must require one
complete JSON object, reject trailing data, reject duplicate keys before value
selection, reject unknown/missing/wrong-type fields, and perform no Unicode,
identifier, scope, hash, or hexadecimal normalization. Unknown schemas and
predicate identifiers never fall through to a generic checker.

The closed checker is:

1. Receive only the private snapshot plus exact `claim_id`, `evidence_id`, and
   `edge_id` selected by the closed v2 lane.
2. Re-fetch all three objects and validate supports/source/target, evidence
   kind, claim domain, and four-way scope equality.
3. Strictly parse the claim predicate and witness schemas.
4. Validate exact schema, predicate, claim, and scope bindings.
5. Validate and decode strict witness hex while enforcing the fixed 4096-byte
   decoded limit before allocation beyond that bound.
6. Compute SHA-256 over exactly the decoded bytes.
7. Compare the 32 computed bytes to the strictly decoded expected digest.
8. Return `Matched` with audit material only on equality; otherwise return one
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

Existing v0 candidate tracing owns missing replayed nodes, unsupported edge
kind, source/target structure, wrong evidence kind, wrong/missing claim domain,
and node/edge/claim scope mismatch. Therefore those checks are revalidated by
the v2 executor but are not duplicated as verifier-context variants. If
revalidation disagrees with the candidate, no context attempt is emitted and
the v2 lane records a closed `CandidateRevalidationFailed` classification.

Verifier-context tracing owns only strict predicate/witness parsing, their
bindings, bounded decoding, and digest comparison. V2 lane classification owns
whether the exact v0 deterministic-verification candidate selects the one v2
rule. Claim-level v2 blockers are deterministically derived from unresolved v0
blockers, `CandidateRevalidationFailed`, or the exact unsuccessful context
outcome; they do not repeat the check as a second authority source. One
successful application satisfies only its own `RequiresVerifierContext` and
`CeilingIsCandidateOnly` limitations.

## Trusted construction and bypasses

The sole legitimate origin is:

```text
one successful LogReader verification
→ one private composite replay
→ one StandingReplaySnapshot containing all required projections
→ exact candidate lane classification
→ replayed-node and edge revalidation
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
- the bytes decoded from that evidence node's witness have the expected SHA-256
  digest under `sha256_bytes_equals_v0`.

It does not prove correct trust-root choice, external artifact identity or
origin, acquisition, file contents, URL response, program execution, producer
identity beyond recorded provenance, semantic correctness, safety,
interpretation truth, arbitrary claim truth, independent corroboration, or
publication authority.

## Proposed policy-v2 boundary

The future policy identity is exactly `magpie-claims-standing-v2`. It preserves
all v1 behaviour and adds exactly one direct-support family:

```text
DeterministicVerification
× ExactMachineCheckable
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
edge ID, exact scope, predicate schema and ID, expected and computed digests,
witness byte length, exact verifier-context outcome, and achieved contribution.
The complete witness is not copied into the resolution; it remains in the
replayed evidence projection.

A Boolean or count loses exact attribution, prevents precise replay comparison
and later invalidation, obscures hostile failures, and invites amplification.

A later implementation must:

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

## End-to-end flow

```text
ClaimAssertedV2 machine predicate
+ EvidenceRegistered witness
+ supports edge
+ exact scope
+ verified one-snapshot replay
+ closed sha256_bytes_equals_v0 checker
→ replay-derived verifier receipt
→ one bounded policy-v2 Supported contribution
```

## Non-goals and questions requiring new infrastructure

This contract adds no payload, tag, projection, receipt type, checker, policy
v2, achieved standing, aggregation, independence, refutation, contradiction
debt, invalidation, supersession, currentness, `EpistemicGate`, writer surface,
artifact acquisition, file/shell/network/plugin/callback authority, proof
system, formal verification, or model/lens ingestion.

It cannot honestly decide external artifact identity, acquisition provenance,
remote responses, filesystem contents, program execution origin, or semantic
interpretation. Those questions require separately reviewed L0 or artifact
infrastructure and separately named predicates. They must not be smuggled into
`sha256_bytes_equals_v0`.

## Implementation sequence

1. Land this contract only.
2. Add a standing-inert strict parser, closed checker, context trace, and
   private-snapshot construction with hostile tests.
3. Review its audit surface and same-snapshot provenance without standing
   changes.
4. Add explicit v2 types and the one direct `Supported` rule, preserving exact
   v0/v1 bytes and precedence.
5. Only then consider genuine aggregation and independence.

The later code PR must stop if exact parsing requires L0 changes; metadata
cannot express unambiguous bindings; a new payload or artifact store is needed;
the checker needs file, shell, network, plugin, callback, or ambient authority;
v0/v1 semantics or bytes would change; the contribution cannot remain exactly
`Supported`; or accurate documentation would require claiming external origin
or general truth.
