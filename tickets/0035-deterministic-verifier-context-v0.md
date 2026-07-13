# Ticket 0035: Standing-inert deterministic verifier context v0

## Goal and starting boundary

Starting at merge commit
`c5e24ef61f95d1b7cce819d28fe18f69fe9c431e`, implement one replay-derived,
standing-inert checker for the exact `sha256_bytes_equals_v0` proposition.

```text
signed event != verifier authority
typed evidence label != verifier authority
verification witness != successful verification
successful verification != admitted contribution
replay-derived verifier receipt != achieved standing
```

This ticket derives verifier context but grants no epistemic standing.
A `Matched` receipt is audit material. No policy consumes it in this change.

## Exact scope and public API

The only trusted query is:

```rust
StandingReplaySnapshot::resolve_deterministic_verifier_context_v0(
    &self, claim_id: &str, evidence_id: &str, edge_id: &str,
) -> Option<DeterministicVerifierContextTraceV0>
```

`None` means the exact v0 candidate lane was not selected or same-snapshot
structural revalidation disagreed. `Some(failure)` means the lane was selected
but context construction failed. `Some(Matched)` means the closed checker
matched; it is neither admission nor standing.

The exported closed constants are
`MACHINE_PREDICATE_SCHEMA_V0`, `VERIFICATION_WITNESS_SCHEMA_V0`,
`SHA256_BYTES_EQUALS_PREDICATE_V0`,
`SHA256_BYTES_EQUALS_STATEMENT_PREFIX_V0`, and
`MAX_WITNESS_BYTES_V0 = 4096`. There is no registry, alias, callback, or runtime
registration.

## Private parsing and deterministic precedence

Custom streaming serde visitors preserve duplicate keys without
`serde_json::Value`. Claim metadata permits only `claim_domain` and a complete
`machine_predicate`; witness metadata permits only one complete
`verification_witness`. All required values are strings.

Claim precedence is malformed/non-object/trailing, duplicate, unknown, missing
predicate, incomplete/wrong-type, schema, predicate ID, digest. Witness
precedence is malformed/non-object/trailing, duplicate, unknown, missing
witness, incomplete/wrong-type, schema, predicate ID binding, claim binding,
scope binding, encoded limit, strict hex, decoded limit. This order is
independent of key order.

## Statement, content hash, and checker

The sole canonical statement is
`sha256_bytes_equals_v0:` plus the exact lowercase expected digest. It must
equal the replayed statement byte-for-byte. The non-empty replayed content hash
must equal:

```text
lowercase_hex(SHA-256(UTF-8(exact replayed statement)))
```

That content hash commits to statement bytes. The predicate's
`expected_sha256` separately commits to witness bytes.

The checker rejects encoded witnesses longer than 8192 characters before
decoding, accepts only lowercase even-length hex (including empty), decodes at
most 4096 bytes, hashes exactly those bytes, compares digest bytes, and returns
`DigestMismatch` or `Matched`. It stores no witness bytes in the receipt.

## Receipt and failure vocabulary

`DeterministicVerifierReceiptV0` has private fields, read-only getters, no
constructor, and no `Deserialize`. It records predicate schema and ID, claim,
evidence and edge IDs, scope, canonical statement, claim content hash, expected
and computed witness digests, and `u64` witness length.

The adjacently tagged trace serializes as `{"outcome":"matched","receipt":...}`
or a unit form such as `{"outcome":"digest_mismatch"}`. Failures are:

```text
MissingMachinePredicate MalformedMachinePredicate DuplicatePredicateKey
UnknownPredicateKey UnknownPredicateSchema UnknownPredicateId
InvalidExpectedDigest StatementPredicateMismatch MissingClaimContentHash
ClaimContentHashMismatch MissingVerificationWitness
MalformedVerificationWitness DuplicateWitnessKey UnknownWitnessKey
UnknownWitnessSchema ClaimBindingMismatch ScopeBindingMismatch
PredicateBindingMismatch InvalidWitnessHex WitnessTooLarge DigestMismatch
```

These bytes are policy-defined derived audit bytes for deterministic
comparison, not L0 canonical encoding.

## Trusted construction and standing-inert proof

The sole construction path is successful `LogReader` verification, private
composite replay, one `StandingReplaySnapshot`, exact v0 candidate selection,
re-fetch and revalidation of the claim/evidence/edge, strict parsing and
binding, then the closed checker. Candidate selection requires the exact
`DeterministicVerification × ExactMachineCheckable` v0 trace, `Settled`
ceiling, and ordered accepted/context/candidate-only reasons. Missing nodes,
wrong structure, wrong kind/domain, and scope mismatch remain v0 trace failures
and yield no context attempt.

Compile-fail documentation proves callers cannot construct a receipt literal
and `StandingView` has no trusted query. The public snapshot query is pure,
takes `&self`, caches nothing, returns no `Status`, and is accepted by no policy.
V0, v1, raw standing, and snapshot canonical bytes remain unchanged.

## Dependencies, inventory, and tests

`magpie-claims` adds direct workspace dependencies on existing `hex` and
`sha2`. The lockfile local package dependency list gains only those names; no
registry resolution changes. The package inventory checker requires the new
module and integration test, increasing the claims inventory by two files
without claiming standalone registry packageability.

Hostile coverage includes exact and empty matches, 4096/4097-byte boundaries,
all parser and binding outcomes, duplicate/unknown precedence, statement and
content-hash attacks, actor/boolean/label authority confusion, candidate
boundary failures, changing-store one-read provenance, verification failure,
standing and byte preservation, literal matched JSON, and every literal unit
failure serialization.

## Files in scope

Only `Cargo.lock`, `README.md`, the claims manifest/module/export/test, the two
named design notes, this ticket, and the release metadata checker may change.
Policy, standing, replay snapshot, Deadbolt context, L0, fixtures, releases,
format, seams, ADRs, CI, and root manifest are frozen.

## Non-goals and follow-up

No policy v2, achieved status, contribution, aggregation, independence,
refutation, invalidation, supersession, gate, writer, payload, tag, projection,
cache, artifact access, filesystem/process/network/plugin authority, generic
registry, or proof system is implemented.

Validation runs focused tests and doctests, format, Clippy, workspace tests,
tour, metadata and inventories, standalone `magpie-log` packaging, both chain
verifiers, lockfile/standing/authority/frozen-release audits, and full diff
inspection. The next separately reviewed slice is exactly one explicit
policy-v2 direct `Supported` rule.
