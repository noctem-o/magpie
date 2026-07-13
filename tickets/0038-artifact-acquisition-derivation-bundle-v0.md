# Ticket 0038: Artifact acquisition and direct derivation bundle v0

## Goal

Pin one exact documentation-only foreign-bundle protocol for one acquisition
statement and one direct parent-to-derived statement. The contract fixes the
two schemas, shared canonical JSON bytes, SHA-256 roots, full anchor identity,
explicit closure inputs, standing-inert conceptual verifier outcomes, and
normative examples that the following fixture/verifier PR must reproduce.

```text
This ticket pins a protocol contract only.
It implements no parser, verifier, fixture or standing behaviour.
```

## Architectural law

```text
untrusted artifact bytes
+ exact acquisition or derivation statement
-> canonical foreign-bundle bytes
-> SHA-256 root
-> exact SegmentAnchored occurrence
-> later same-replay profile-specific verification
-> standing-inert verified context
```

The authority boundaries are:

```text
canonical statement verification != truth
artifact hash match != locator authenticity
acquisition bundle match != source identity
derivation bundle match != transformation correctness
bundle occurrence != origin admission
origin admission != support
support != aggregation
```

The protocol contains two distinguishable sibling families. It contains no
mixed statement list, generic metadata map, caller-selected role, origin-binding
variant, or generic provenance envelope. Each bundle records exactly one
statement.

## Exact selected identifiers

```text
acquisition bundle_kind: magpie-artifact-acquisition-v0
acquisition schema: magpie-artifact-acquisition-v0

derivation bundle_kind: magpie-artifact-derivation-v0
derivation schema: magpie-artifact-derivation-v0

canonicalization_profile: magpie-artifact-provenance-json-v0
verifier_profile: magpie-artifact-provenance-verifier-v0
witness_algorithm: sha256
artifact identity algorithm: sha256
```

The verifier-profile identity comes from reviewed code/contract selection. It
is not a bundle field, a runtime-negotiated profile, or caller-supplied trusted
authority. SHA-256 is selected only for this first protocol. No global Magpie
or Deadbolt algorithm choice and no registry is created.

## Exact acquisition schema

```json
{
  "schema": "magpie-artifact-acquisition-v0",
  "acquisition_id": "...",
  "acquisition_profile": "...",
  "artifact": {
    "algorithm": "sha256",
    "digest": "..."
  },
  "observed_locator": "...",
  "run_id": "..."
}
```

Canonical top-level order is exactly `schema`, `acquisition_id`,
`acquisition_profile`, `artifact`, `observed_locator`, `run_id`. Canonical
artifact order is exactly `algorithm`, `digest`.

All fields are required exactly once. `schema` is the exact selected schema;
`acquisition_id` identifies this statement; `acquisition_profile` is an opaque
process-profile identifier; `artifact` identifies exact acquired bytes;
`observed_locator` is an opaque observation/import descriptor; and `run_id`
must equal the matching anchor `run_id` byte-for-byte.

The bundle proves neither that the locator served the bytes nor locator
authenticity, publisher identity, authorship, secure transport, factual truth,
or policy trust in `acquisition_profile`.

## Exact derivation schema

```json
{
  "schema": "magpie-artifact-derivation-v0",
  "derivation_id": "...",
  "transformation_profile": "...",
  "parent_artifact": {
    "algorithm": "sha256",
    "digest": "..."
  },
  "derived_artifact": {
    "algorithm": "sha256",
    "digest": "..."
  },
  "run_id": "..."
}
```

Canonical top-level order is exactly `schema`, `derivation_id`,
`transformation_profile`, `parent_artifact`, `derived_artifact`, `run_id`.
Each nested artifact order is exactly `algorithm`, `digest`.

All fields are required exactly once. `derivation_id` identifies one direct
lineage statement; `transformation_profile` is opaque; parent and derived
artifacts identify one exact input and output; and `run_id` must equal the
matching anchor `run_id` byte-for-byte.

One bundle records one direct relation. Multiple-input tooling may later emit
multiple direct relations, but v0 defines no statement list, graph-wide
transitivity, completeness, or omitted-input rule. Exact equal parent and
derived identities return `ParentEqualsDerived`. A matched statement does not
rerun or prove the transformation.

## Artifact, identifier, and locator validation

Every artifact identity contains only exact `algorithm = "sha256"` and a
`digest` of exactly 64 ASCII lowercase hexadecimal characters representing 32
bytes. Algorithm aliases, case variants, uppercase digest text, prefixes, and
other algorithms fail closed.

The exact grammar for `acquisition_id`, `derivation_id`, `run_id`,
`acquisition_profile`, and `transformation_profile` is:

```text
[A-Za-z0-9][A-Za-z0-9._:/-]{0,127}
```

Identifiers are ASCII only, 1-128 bytes, byte-exact, and case-sensitive. They
contain no whitespace or control character and have no Unicode normalization,
hierarchy, prefix, path, containment, inheritance, or authority semantics.
`run_id` is correlation only.

`observed_locator` is valid UTF-8 of 1-4096 UTF-8 bytes, contains no U+0000,
ASCII C0 control, or DEL, and is compared exactly without normalization. It is
not parsed, fetched, or interpreted. URL/path/archive-like syntax grants no
authority.

## Bundle size limit

```text
MAX_ARTIFACT_PROVENANCE_BUNDLE_BYTES_V0 = 16384
```

The limit applies to supplied canonical bundle bytes before parsing.
Over-limit input returns `BundleTooLarge`. External artifact bytes are not
subject to this bundle limit.

## Fixed canonical JSON profile

`magpie-artifact-provenance-json-v0` is a purpose-built fixed-schema profile,
not full RFC 8785/JCS, not a Deadbolt profile, and not `magpie-core-v1`. It
accepts only the two exact schemas above.

Raw bytes must be valid UTF-8 with no BOM; contain exactly one JSON object and
no leading/trailing whitespace, trailing newline, or trailing bytes; contain
no duplicate or unknown key; contain every field exactly once; contain no
`null`, Boolean, number, or array; and use only strings plus the exact nested
artifact objects.

Objects emit in schema-defined order. Separators are `:` and `,` with no
insignificant whitespace. Canonical string encoding is exact:

- surround strings with `"`;
- encode `"` as `\"` and `\` as `\\`;
- encode U+0008, U+0009, U+000A, U+000C, U+000D as `\b`, `\t`, `\n`, `\f`,
  `\r`;
- encode other U+0000-U+001F as lowercase `\u00xx`;
- emit every other Unicode scalar directly as UTF-8;
- perform no Unicode normalization;
- reject unnecessary escapes, escaped solidus `\/`, uppercase hexadecimal
  escape digits, lone surrogates, and invalid UTF-8.

After duplicate-aware parsing and exact validation, the future verifier must
re-encode the object and compare it with the complete supplied bytes. Any
difference is `NonCanonicalEncoding`. The root therefore commits exact
canonical bytes, not a loosely equivalent JSON value.

## Root construction

```text
witness_root
= lowercase_hex(SHA-256(canonical_bundle_bytes))
```

There is no domain-separation prefix in v0. This formula applies only to this
exact foreign-bundle protocol and does not reinterpret other roots. The output
is exactly 64 lowercase hexadecimal characters representing 32 bytes.

## Exact anchor tuple

An attempt is matched against all five exact `SegmentAnchored` fields:

```text
bundle_kind
witness_root
witness_algorithm
canonicalization_profile
run_id
```

Root-only matching is forbidden. Schema selects the exact bundle kind;
reviewed code selects exact `sha256`,
`magpie-artifact-provenance-json-v0`, and
`magpie-artifact-provenance-verifier-v0`; bundle `run_id` selects only the
identical anchor `run_id`. Different kind, run, algorithm, profile, case, or a
caller-provided `verified` Boolean cannot match. A valid bundle with no exact
same-replay anchor is outside Magpie's record.

## Resolution-content-closure interaction

This ticket does not pin `ResolutionContentClosureV0` manifest encoding. It
pins only the future verifier's inputs.

Acquisition requires bundle bytes keyed by the full expected anchor identity
and artifact bytes keyed by the exact artifact identity in the bundle.
Derivation requires bundle bytes keyed by the full expected anchor identity,
parent bytes keyed by exact parent identity, and derived bytes keyed by exact
derived identity.

Closure presence is not validity. Missing objects produce explicit unavailable
outcomes. Verification performs no ambient CAS lookup, filesystem scan,
network access, callback, plugin, downloader, or second mutable-store read.

## Standing-inert verifier semantics

`MatchedAcquisition` requires bundle presence, size compliance, all raw and
canonical JSON checks, schema and field validation, the exact full anchor in
the same verified replay, computed bundle root equality, exact `run_id`,
artifact presence in closure, and artifact SHA-256 equality. It proves only:

> The accepted Magpie replay contains an exact anchor for this canonical
> acquisition statement, and the supplied artifact bytes match the statement's
> exact SHA-256 identity.

`MatchedDerivation` requires the corresponding bundle checks, distinct parent
and derived identities, the exact full same-replay anchor, computed root and
`run_id` equality, both artifact objects in closure, and both SHA-256 matches.
It proves only:

> The accepted Magpie replay contains an exact anchor for this canonical
> direct-lineage statement, and the supplied parent and derived bytes match the
> identities named by that statement.

Neither matched result proves locator authenticity, source identity,
transformation correctness, truth, origin admission, support, aggregation, or
settlement. Both are standing-inert audit material; callers cannot construct a
trusted result from its serialized shape.

## Closed failure vocabulary

The future primary outcome vocabulary is closed:

```text
BundleUnavailable
BundleTooLarge
InvalidUtf8
Utf8BomPresent
InvalidJson
TopLevelNotObject
DuplicateKey
MissingField
UnknownField
WrongJsonType
SchemaMismatch
InvalidIdentifier
InvalidObservedLocator
UnsupportedArtifactAlgorithm
InvalidArtifactDigest
ParentEqualsDerived
NonCanonicalEncoding
AnchorAbsent
BundleKindMismatch
WitnessAlgorithmMismatch
CanonicalizationProfileMismatch
RunIdMismatch
WitnessRootMismatch
ArtifactUnavailable
ArtifactDigestMismatch
ParentArtifactUnavailable
ParentArtifactDigestMismatch
DerivedArtifactUnavailable
DerivedArtifactDigestMismatch
MatchedAcquisition
MatchedDerivation
```

The distinctions may receive Rust-style spelling later but may not be removed,
converted to arbitrary strings, or collapsed to `verified: bool`.

## Deterministic precedence

Primary first failure uses exactly:

1. closure lookup;
2. bundle byte-size limit;
3. UTF-8/BOM/JSON parse;
4. duplicate-key and exact-schema validation;
5. identifier, locator and artifact-field validation;
6. derivation self-loop validation;
7. canonical re-encoding equality;
8. anchor tuple selection and exact anchor matching;
9. root recomputation and equality;
10. referenced artifact closure lookup;
11. referenced artifact digest verification;
12. matched result.

Within schema and semantic validation, fields are visited in their canonical
schema order. Duplicate detection precedes field selection and retains all
members. Within anchor stage 8, order is bundle kind, witness algorithm,
canonicalization profile, run ID, then anchor absence. For derivation artifact
stages, parent precedes derived. When a nested artifact object is reached in
top-level schema order, its `algorithm` and `digest` fields are validated before
the next top-level field. Map iteration order never selects the first failure.
Subordinate diagnostics may be retained, but the primary outcome follows this
order.

## Normative vectors

The design note contains the human-readable objects and exact anchor tuples.
The exact bytes and independently calculated values pinned here are:

### Acquisition

Artifact bytes are the 15 ASCII bytes `hello artifact\n` with final byte `0a`:

```text
artifact sha256: 51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076
```

Human-readable logical object:

```json
{
  "schema": "magpie-artifact-acquisition-v0",
  "acquisition_id": "acq-0001",
  "acquisition_profile": "magpie-import-v0",
  "artifact": {
    "algorithm": "sha256",
    "digest": "51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076"
  },
  "observed_locator": "file:hello-artifact.txt",
  "run_id": "run-acq-0001"
}
```

Exact canonical bytes, with no trailing newline:

```json
{"schema":"magpie-artifact-acquisition-v0","acquisition_id":"acq-0001","acquisition_profile":"magpie-import-v0","artifact":{"algorithm":"sha256","digest":"51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076"},"observed_locator":"file:hello-artifact.txt","run_id":"run-acq-0001"}
```

```text
UTF-8 byte length: 291
witness_root: 4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e
bundle_kind: magpie-artifact-acquisition-v0
witness_algorithm: sha256
canonicalization_profile: magpie-artifact-provenance-json-v0
run_id: run-acq-0001
```

### Derivation

Parent bytes are the 6 ASCII bytes `alpha\n`; derived bytes are the 6 ASCII
bytes `ALPHA\n`, each with final byte `0a`:

```text
parent sha256: b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060
derived sha256: 1921b918b15842c7fdb115078e610263fac85f159c1d8e0ecec3d89a0faa4005
```

Human-readable logical object:

```json
{
  "schema": "magpie-artifact-derivation-v0",
  "derivation_id": "der-0001",
  "transformation_profile": "uppercase-ascii-v0",
  "parent_artifact": {
    "algorithm": "sha256",
    "digest": "b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060"
  },
  "derived_artifact": {
    "algorithm": "sha256",
    "digest": "1921b918b15842c7fdb115078e610263fac85f159c1d8e0ecec3d89a0faa4005"
  },
  "run_id": "run-der-0001"
}
```

Exact canonical bytes, with no trailing newline:

```json
{"schema":"magpie-artifact-derivation-v0","derivation_id":"der-0001","transformation_profile":"uppercase-ascii-v0","parent_artifact":{"algorithm":"sha256","digest":"b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060"},"derived_artifact":{"algorithm":"sha256","digest":"1921b918b15842c7fdb115078e610263fac85f159c1d8e0ecec3d89a0faa4005"},"run_id":"run-der-0001"}
```

```text
UTF-8 byte length: 374
witness_root: 472e857153fe8c901b6c0d7d1b15bd876450b57b4743222a2fab91eb60bc76dd
bundle_kind: magpie-artifact-derivation-v0
witness_algorithm: sha256
canonicalization_profile: magpie-artifact-provenance-json-v0
run_id: run-der-0001
```

Independent local Python snippets reproduced all artifact digests, byte
lengths, and roots. The next fixture PR must reproduce them byte-for-byte.

## Hostile contract checks

The design note pins exact outcomes for pretty whitespace, reordered fields,
trailing newline, duplicate/unknown fields, uppercase digest, algorithm alias,
wrong kind/run/profile, absent anchor, ambient-CAS-only bytes, missing or
mismatched artifacts, locator-authority transposition, derivation self-loop,
incorrect transformation claim, caller-provided verification, post-`H` anchor,
and reuse under another anchor identity.

The important bounded cases are: alternate semantic JSON is
`NonCanonicalEncoding`; wrong anchor fields use their exact stage-8 mismatch;
root mismatch is `WitnessRootMismatch`; absent same-replay occurrence is
`AnchorAbsent`; absent closure objects use exact unavailable outcomes; and a
hash-correct derivation can establish at most `MatchedDerivation`, never
transformation correctness.

## Timestamp and signing decisions

The v0 schemas contain no timestamp. Signed Magpie anchor time and order record
chain inclusion only; they are not acquisition or transformation time. V0
makes no such time claim. A time-bearing statement requires a later explicit
schema version, and IDs or locators must not smuggle time authority.

The v0 schemas contain no embedded signature or self-declared signer. Integrity
and occurrence derive from exact canonical bytes, SHA-256 root, exact
same-replay `SegmentAnchored`, and Magpie chain verification. A named key could
not bootstrap its own authority. A future external export may use DSSE/in-toto
under a separate contract.

## Files in scope

Exactly:

- `README.md`
- `docs/design/artifact-acquisition-derivation-bundle-v0.md`
- `docs/design/artifact-provenance-origin-admission.md`
- `tickets/0038-artifact-acquisition-derivation-bundle-v0.md`

Ticket 0037 and the existing aggregation note remain unchanged.

## Frozen surfaces

Do not modify or reinterpret `magpie-core-v1`, payload tags,
`SegmentAnchored`, anchor semantics, golden vectors, Deadbolt code, policies
v0/v1/v2, snapshot or resolution bytes, evidence ceilings, historical
`content_hash`, Ticket 0037, release contract, workspace version, package
inventories, `Cargo.lock`, or CI.

## Explicit non-goals

No Rust, tests, fixtures, Cargo dependency, parser, canonicalization or hashing
implementation, closure manifest encoding, runtime CAS, filesystem/network
access, callback, plugin, L0 tag, format change, golden-chain change,
origin-binding schema, origin-admission policy, origin groups, aggregation
lanes, policy v3, support aggregation, refutation, contradiction debt,
invalidation, supersession, currentness, `EpistemicGate`, writer, crawler,
downloader, model/librarian integration, or production-readiness claim.

## Validation

Validate both normative examples independently, then run formatting, Clippy,
workspace tests, governed tour, release metadata, `magpie-log` packaging,
independent golden-chain verification, diff whitespace, exact changed-path
scope, protocol-identity searches, stale/prohibited ambiguity searches, and a
full authority audit. Every reported pass must have actually run.

## Next slice

1. Ratify Ticket 0038.
2. Add portable acquisition and derivation bundle fixtures reproducing the
   normative vectors.
3. Add standing-inert parser, canonicalizer and verifier implementation.
4. Add immutable closure construction or an explicit test-only closure surface
   as separately reviewed.
5. Pin origin-binding bundle schema and origin-admission policy.
6. Add standing-inert origin-admission audit.
7. Add admitted-contribution audit.
8. Pin policy-v3 aggregation.
9. Implement conservative aggregation.

The immediate next PR combines only portable fixture files, standing-inert
verification, and hostile tests. It has no standing effect.
