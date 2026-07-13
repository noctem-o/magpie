# Artifact Acquisition and Direct Derivation Bundle v0

## 1. Status

Proposed exact protocol contract seeking human ratification.

This note pins bytes and conceptual future verifier behavior only. It does not
implement a parser, canonicalizer, hasher, closure, replay path, fixture,
standing rule, or runtime authority.

## 2. Purpose

Ticket 0037 established the architecture boundary between exact external bytes,
acquisition history, derivation lineage, origin admission, support, and
aggregation. This note pins the first exact foreign-bundle protocol inside that
boundary:

```text
untrusted artifact bytes
+ exact acquisition or derivation statement
-> canonical foreign-bundle bytes
-> SHA-256 root
-> exact SegmentAnchored occurrence
-> later same-replay profile-specific verification
-> standing-inert verified context
```

The resulting context establishes only the exact proposition checked. It does
not establish factual truth, publisher identity, locator authenticity,
authorship, transport authenticity, transformation correctness, source
independence, origin admission, support, aggregation, or settlement.

## 3. Relationship to Ticket 0037

Ticket 0037 and
[`artifact-provenance-origin-admission.md`](artifact-provenance-origin-admission.md)
remain the parent architecture contract. Ticket 0038 selects exact acquisition
and direct-derivation bundle identities, schemas, canonical bytes, root
construction, anchor matching, and standing-inert verifier outcomes for the
first implementation family.

This selection does not pin an origin-binding family. It does not change the
deterministic resolution boundary:

```text
deterministic provenance/origin resolution input
= verified log prefix H
+ explicit policy identities P
+ immutable resolution content closure M

same H + same P + same M
-> same derived result
```

The exact `ResolutionContentClosureV0` manifest encoding remains deferred.

## 4. Selected identities

The acquisition family is exactly:

```text
bundle_kind: magpie-artifact-acquisition-v0
schema: magpie-artifact-acquisition-v0
```

The direct-derivation family is exactly:

```text
bundle_kind: magpie-artifact-derivation-v0
schema: magpie-artifact-derivation-v0
```

Both families use exactly:

```text
canonicalization_profile: magpie-artifact-provenance-json-v0
verifier_profile: magpie-artifact-provenance-verifier-v0
witness_algorithm: sha256
artifact identity algorithm: sha256
```

`verifier_profile` is not a bundle field and is not supplied as trusted
authority by runtime callers. It identifies reviewed code and contract
selection. A future implementation may disclose it in derived audit material,
but only the exact reviewed `SegmentAnchored` tuple may dispatch this profile.

These values select SHA-256 only for this protocol. They do not change Magpie
or Deadbolt algorithms globally and do not create an algorithm registry,
alias table, negotiation surface, or ambient latest profile.

## 5. Shared artifact identity

Every artifact identity in both schemas is exactly:

```json
{
  "algorithm": "sha256",
  "digest": "<64 lowercase hexadecimal characters>"
}
```

The canonical nested field order is exactly:

1. `algorithm`
2. `digest`

`algorithm` must equal the ASCII string `sha256`. No alias, case variant, or
other algorithm is accepted. `digest` must be exactly 64 ASCII lowercase
hexadecimal characters representing 32 bytes. Artifact equality is exact
equality of both fields. It identifies bytes only; it does not identify a
locator, acquisition, source, author, publisher, meaning, or origin.

## 6. Acquisition schema

An acquisition bundle contains exactly one acquisition statement and has this
logical object:

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

The canonical top-level field order is exactly:

1. `schema`
2. `acquisition_id`
3. `acquisition_profile`
4. `artifact`
5. `observed_locator`
6. `run_id`

The `artifact` object uses the nested field order in section 5. Every field is
required exactly once. No unknown field or arbitrary metadata is permitted.

Field semantics are closed:

- `schema` must equal `magpie-artifact-acquisition-v0`.
- `acquisition_id` identifies this exact acquisition statement.
- `acquisition_profile` is an opaque exact process-profile identifier.
- `artifact` identifies the exact acquired bytes.
- `observed_locator` is an opaque observed or import locator descriptor.
- `run_id` must exactly equal the matching `SegmentAnchored.run_id`.

An acquisition bundle does not prove that the locator served the bytes, the
locator is authentic, its owner authored the artifact, transport was secure,
or the acquisition profile is trusted by an origin-admission policy. An opaque
`acquisition_profile` string does not authorize itself.

Later verification may establish only exact canonical statement bytes, exact
anchor inclusion, exact artifact-byte SHA-256 equality, and exact field/profile
conformance.

## 7. Direct derivation schema

A derivation bundle contains exactly one direct parent-to-derived statement and
has this logical object:

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

The canonical top-level field order is exactly:

1. `schema`
2. `derivation_id`
3. `transformation_profile`
4. `parent_artifact`
5. `derived_artifact`
6. `run_id`

Both nested artifact objects use the field order in section 5. Every field is
required exactly once. No unknown field or arbitrary metadata is permitted.

Field semantics are closed:

- `schema` must equal `magpie-artifact-derivation-v0`.
- `derivation_id` identifies one exact direct-lineage statement.
- `transformation_profile` is an opaque exact transformation-profile
  identifier.
- `parent_artifact` identifies one exact direct parent.
- `derived_artifact` identifies one exact output.
- `run_id` must exactly equal the matching `SegmentAnchored.run_id`.

The parent and derived artifact identities must differ exactly. If
`parent_artifact == derived_artifact`, verification returns
`ParentEqualsDerived`. A no-byte-change operation does not require a derivation
edge from an artifact to itself.

For a multi-input transformation, later tooling may emit multiple direct
relations. This contract defines neither a statement list nor graph-wide
transitive semantics. It does not rerun the named transformation and does not
prove transformation correctness, semantic preservation, completeness,
faithfulness, or absence of omitted inputs. An opaque
`transformation_profile` string does not authorize itself.

## 8. Identifier and locator validation

The shared identifier grammar applies to `acquisition_id`, `derivation_id`,
`run_id`, `acquisition_profile`, and `transformation_profile`:

```text
[A-Za-z0-9][A-Za-z0-9._:/-]{0,127}
```

Equivalently, each identifier is ASCII only, 1-128 bytes, begins with an ASCII
alphanumeric character, and thereafter contains only ASCII alphanumeric or
`.`, `_`, `:`, `/`, `-`. Whitespace, control characters, and non-ASCII bytes
are forbidden. Comparison is byte-exact and case-sensitive. No Unicode
normalization occurs.

These strings have no hierarchical, prefix, path, containment, inheritance,
wildcard, or authority semantics. `run_id` is an exact correlation identifier,
not authority, and must not be treated as a timestamp.

`observed_locator` must be valid UTF-8, contain 1-4096 UTF-8 bytes, and contain
neither U+0000, any ASCII C0 control U+0000-U+001F, nor DEL U+007F. It is
compared as an exact string with no normalization. It may contain a URL, path,
archive reference, or opaque import label, but the verifier does not parse,
resolve, fetch, open, or interpret it. Its syntax creates no source authority.
URL parsing is not required in v0.

## 9. Bundle size limit

The fixed limit is:

```text
MAX_ARTIFACT_PROVENANCE_BUNDLE_BYTES_V0 = 16384
```

It applies to supplied canonical bundle bytes before parsing. An over-limit
bundle returns `BundleTooLarge` before UTF-8 or JSON work. Artifact bytes are
external closure objects and are not subject to this bundle limit.

## 10. Fixed canonical JSON profile

`magpie-artifact-provenance-json-v0` is a purpose-built, fixed-schema canonical
JSON profile. It is not full RFC 8785/JCS, is not any Deadbolt interim or
candidate profile, and is not `magpie-core-v1`. It accepts only the exact two
schemas in sections 6 and 7.

### 10.1 Raw input requirements

Supplied bundle bytes must:

- be valid UTF-8;
- contain no UTF-8 BOM;
- contain exactly one JSON object;
- contain no leading or trailing whitespace;
- contain no trailing newline or trailing bytes;
- contain no duplicate key at any depth;
- contain every required field exactly once;
- contain no unknown field;
- contain no `null`, Boolean, numeric, or array value; and
- use only JSON strings and the exact nested artifact objects.

A valid JSON object containing leading, trailing, or internal insignificant
whitespace is parseable but noncanonical and returns
`NonCanonicalEncoding` at canonical re-encoding equality. A second JSON value
or non-whitespace trailing bytes returns `InvalidJson`.

### 10.2 Object order and separators

Objects are emitted only in the schema-defined field orders above. Arbitrary
fields are forbidden, so arbitrary key sorting is neither required nor
permitted as a substitute for schema order.

Canonical bytes use `:` between each key and value, `,` between fields, and no
insignificant whitespace.

### 10.3 Canonical string encoding

Canonical string encoding is exact:

- strings are surrounded by `"` characters;
- `"` becomes `\"`;
- `\` becomes `\\`;
- U+0008, U+0009, U+000A, U+000C, and U+000D use `\b`, `\t`, `\n`, `\f`, and
  `\r`, respectively;
- every other U+0000-U+001F character uses lowercase `\u00xx`;
- every other Unicode scalar value is emitted directly as UTF-8;
- no Unicode normalization occurs;
- unnecessary escapes are noncanonical;
- escaped solidus `\/` is noncanonical;
- hexadecimal escape digits must be lowercase; and
- lone surrogate code points and invalid UTF-8 are rejected.

A valid escaped surrogate pair denotes one Unicode scalar value, but its
escaped spelling is noncanonical because that scalar must be emitted directly
as UTF-8. The identifier and locator validators further reject controls as
specified in section 8.

### 10.4 Canonical-input requirement

After duplicate-aware parsing and exact schema validation, the verifier must
deterministically re-encode the logical object under this profile and compare
those bytes with the complete supplied byte string. Any difference returns:

```text
NonCanonicalEncoding
```

The root commits the exact canonical bytes, not a loosely equivalent JSON
value. Pretty printing, field reordering, unnecessary escapes, a trailing
newline, or any other alternate valid encoding cannot share one accepted
identity.

## 11. Root construction

For either family, root construction is exactly:

```text
witness_root
= lowercase_hex(
    SHA-256(canonical_bundle_bytes)
  )
```

No domain-separation prefix is introduced in v0. This choice is limited to
these exact schemas and canonicalization profile and must not reinterpret any
other foreign bundle root.

The result is exactly 64 lowercase hexadecimal characters representing 32
bytes, fitting the frozen `SegmentAnchored.witness_root` field. The matching
event must use exact `witness_algorithm = "sha256"`, exact
`canonicalization_profile = "magpie-artifact-provenance-json-v0"`, and the
bundle kind selected by the exact schema.

## 12. Exact anchor matching

One attempt is bound to one full expected anchor identity:

```text
bundle_kind
witness_root
witness_algorithm
canonicalization_profile
run_id
```

All five strings must match exactly. Root-only matching is forbidden. A closure
key or caller-supplied tuple is selection material only; it does not establish
that an occurrence exists. The accepted same verified replay is the sole source
of anchor occurrence.

The reviewed schema determines the only accepted bundle kind. Reviewed code
fixes `sha256`, `magpie-artifact-provenance-json-v0`, and
`magpie-artifact-provenance-verifier-v0`; bundle content and runtime callers do
not negotiate them. Bundle `run_id` must equal anchor `run_id` byte-for-byte.

The verifier must never accept a different bundle kind with the same root, a
different run ID, an unknown profile, an algorithm alias, uppercase hex, or a
caller-asserted Boolean saying the bundle was anchored. An otherwise valid
bundle without the exact same-replay anchor is outside Magpie's record.

## 13. Resolution-content-closure inputs

This contract does not define the final `ResolutionContentClosureV0` type,
manifest encoding, digest profile, or storage layout. Future verification
consumes a finite immutable closure as follows.

An acquisition attempt requires:

- bundle bytes keyed by the full expected anchor identity; and
- artifact bytes keyed by the exact artifact identity in the bundle.

A derivation attempt requires:

- bundle bytes keyed by the full expected anchor identity;
- parent artifact bytes keyed by the exact parent identity; and
- derived artifact bytes keyed by the exact derived identity.

Closure presence does not imply validity. An object absent from closure is
unavailable for that attempt. The verifier performs no ambient CAS lookup,
filesystem scan, network access, callback, plugin invocation, downloader
action, or second mutable-store read. A separate loader may construct `M`
before resolution; changing availability constructs a different closure.

## 14. Acquisition verification semantics

An acquisition context is `MatchedAcquisition` only if, in order:

1. bundle bytes are present in the closure under the full expected anchor
   identity;
2. bundle size is at most 16384 bytes;
3. raw UTF-8, BOM, JSON, duplicate-key, exact-schema, field, and canonical
   encoding checks pass;
4. the full anchor identity exists in the same completely verified replay;
5. SHA-256 over the exact canonical bundle bytes equals the anchor root;
6. bundle `run_id` equals anchor `run_id` exactly;
7. artifact bytes are present in the closure under the exact artifact identity;
   and
8. SHA-256 over those artifact bytes equals the bundle digest.

The detailed first-failure order in section 17 controls where the combined
checks above terminate.

`MatchedAcquisition` proves only:

> The accepted Magpie replay contains an exact anchor for this canonical
> acquisition statement, and the supplied artifact bytes match the statement's
> exact SHA-256 identity.

It does not prove locator authenticity, publisher or source identity,
authorship, secure transport, acquisition-profile trust, origin admission, or
truth.

## 15. Derivation verification semantics

A derivation context is `MatchedDerivation` only if, in order:

1. bundle bytes are present in the closure under the full expected anchor
   identity;
2. bundle size is at most 16384 bytes;
3. raw UTF-8, BOM, JSON, duplicate-key, exact-schema, field, and canonical
   encoding checks pass;
4. parent and derived artifact identities are distinct;
5. the full anchor identity exists in the same completely verified replay;
6. SHA-256 over the exact canonical bundle bytes equals the anchor root;
7. bundle `run_id` equals anchor `run_id` exactly;
8. parent bytes are present and match their exact SHA-256 identity; and
9. derived bytes are present and match their exact SHA-256 identity.

The detailed first-failure order in section 17 controls where the combined
checks above terminate.

`MatchedDerivation` proves only:

> The accepted Magpie replay contains an exact anchor for this canonical
> direct-lineage statement, and the supplied parent and derived bytes match the
> identities named by that statement.

It does not prove that the named transformation was executed correctly, that
the output is faithful or complete, or that no input was omitted.

Both matched contexts are standing-inert derived audit material. Neither is
caller-constructible authority. No current standing policy consumes them.

## 16. Closed conceptual outcome vocabulary

The later implementation must preserve this closed primary outcome vocabulary.
Rust spelling may be refined, but no distinction may be removed or collapsed
into `verified: bool` or arbitrary strings:

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

`ArtifactUnavailable` and `ArtifactDigestMismatch` apply to acquisition.
Parent and derived outcomes apply to derivation. `MatchedAcquisition` and
`MatchedDerivation` are separate terminal successes because the propositions
are distinct.

## 17. Deterministic first-failure precedence

The primary terminal outcome follows this exact stage order:

1. closure lookup;
2. bundle byte-size limit;
3. UTF-8, BOM, and JSON parse;
4. duplicate-key and exact-schema validation;
5. identifier, locator, and artifact-field validation;
6. derivation self-loop validation;
7. canonical re-encoding equality;
8. anchor tuple selection and exact anchor matching;
9. root recomputation and equality;
10. referenced artifact closure lookup;
11. referenced artifact digest verification; and
12. matched result.

The stage mapping is fixed:

- stage 1: `BundleUnavailable`;
- stage 2: `BundleTooLarge`;
- stage 3, in order: `InvalidUtf8`, `Utf8BomPresent`, `InvalidJson`,
  `TopLevelNotObject`;
- stage 4: `DuplicateKey`, then required-field and type/schema checks in
  canonical schema field order, producing `MissingField`, `WrongJsonType`, or
  `SchemaMismatch`; after required shapes are valid, any extra member produces
  `UnknownField`;
- stage 5: validate semantic fields in canonical schema order, producing
  `InvalidIdentifier`, `InvalidObservedLocator`,
  `UnsupportedArtifactAlgorithm`, or `InvalidArtifactDigest`;
- stage 6: `ParentEqualsDerived` for derivation only;
- stage 7: `NonCanonicalEncoding`;
- stage 8, in order: `BundleKindMismatch`, `WitnessAlgorithmMismatch`,
  `CanonicalizationProfileMismatch`, `RunIdMismatch`, then `AnchorAbsent`;
- stage 9: `WitnessRootMismatch`;
- stage 10: acquisition `ArtifactUnavailable`; derivation
  `ParentArtifactUnavailable` before `DerivedArtifactUnavailable`;
- stage 11: acquisition `ArtifactDigestMismatch`; derivation
  `ParentArtifactDigestMismatch` before `DerivedArtifactDigestMismatch`; and
- stage 12: `MatchedAcquisition` or `MatchedDerivation`.

Duplicate detection retains keys and must not use a map that silently replaces
earlier values. Because the primary duplicate result is the same regardless of
which key duplicated, map iteration cannot change it. Required fields, nested
artifact fields, semantic validation, and referenced artifacts are always
visited in the schema orders above. When validation reaches a nested artifact
object in top-level schema order, it validates that object's `algorithm` and
`digest` fields before continuing to the next top-level field. Unknown-field
diagnostics may retain the earliest input byte offset, but no unordered map
chooses the primary outcome.

The later verifier may retain subordinate diagnostics, but the primary
terminal outcome must follow this precedence.

## 18. Normative documentation vectors

These are normative protocol examples. The following fixture PR must reproduce
their bytes, lengths, identities, roots, and anchor fields byte-for-byte. The
values were calculated with local Python SHA-256 and independently reproduced
from separately written literal byte strings.

### 18.1 Minimal acquisition vector

Artifact sample is the 15 ASCII bytes displayed as `hello artifact\n`, where
`\n` is one final byte `0a`:

```text
hex: 68656c6c6f2061727469666163740a
sha256: 51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076
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

Exact one-line canonical JSON bytes, with no newline after the closing brace:

```json
{"schema":"magpie-artifact-acquisition-v0","acquisition_id":"acq-0001","acquisition_profile":"magpie-import-v0","artifact":{"algorithm":"sha256","digest":"51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076"},"observed_locator":"file:hello-artifact.txt","run_id":"run-acq-0001"}
```

```text
UTF-8 byte length: 291
witness_root: 4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e
```

Exact corresponding `SegmentAnchored` fields:

```text
bundle_kind: magpie-artifact-acquisition-v0
witness_root: 4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e
witness_algorithm: sha256
canonicalization_profile: magpie-artifact-provenance-json-v0
run_id: run-acq-0001
```

### 18.2 Minimal derivation vector

Parent sample is the 6 ASCII bytes displayed as `alpha\n`; derived sample is
the 6 ASCII bytes displayed as `ALPHA\n`. Each `\n` is one final byte `0a`:

```text
parent hex: 616c7068610a
parent sha256: b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060
derived hex: 414c5048410a
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

Exact one-line canonical JSON bytes, with no newline after the closing brace:

```json
{"schema":"magpie-artifact-derivation-v0","derivation_id":"der-0001","transformation_profile":"uppercase-ascii-v0","parent_artifact":{"algorithm":"sha256","digest":"b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060"},"derived_artifact":{"algorithm":"sha256","digest":"1921b918b15842c7fdb115078e610263fac85f159c1d8e0ecec3d89a0faa4005"},"run_id":"run-der-0001"}
```

```text
UTF-8 byte length: 374
witness_root: 472e857153fe8c901b6c0d7d1b15bd876450b57b4743222a2fab91eb60bc76dd
```

Exact corresponding `SegmentAnchored` fields:

```text
bundle_kind: magpie-artifact-derivation-v0
witness_root: 472e857153fe8c901b6c0d7d1b15bd876450b57b4743222a2fab91eb60bc76dd
witness_algorithm: sha256
canonicalization_profile: magpie-artifact-provenance-json-v0
run_id: run-der-0001
```

## 19. Hostile examples

| hostile input or authority claim | deterministic fail-closed result |
| --- | --- |
| Semantically valid JSON with pretty-print whitespace. | `NonCanonicalEncoding` at stage 7. |
| Canonical fields reordered. | `NonCanonicalEncoding` at stage 7. |
| A trailing newline after an otherwise canonical object. | `NonCanonicalEncoding` at stage 7. |
| A duplicate key at any depth. | `DuplicateKey` at stage 4. |
| An unknown top-level or nested field. | `UnknownField` at stage 4 after required shape checks. |
| An uppercase artifact digest. | `InvalidArtifactDigest` at stage 5. |
| Artifact algorithm alias `sha-256`. | `UnsupportedArtifactAlgorithm` at stage 5. |
| The same root is presented under the sibling bundle kind. | `BundleKindMismatch` at stage 8. |
| The same bundle is presented under another anchor `run_id`. | `RunIdMismatch` at stage 8. |
| The same root is presented under another or unknown canonicalization profile. | `CanonicalizationProfileMismatch` at stage 8. |
| A valid bundle has no exact anchor in the accepted replay. | `AnchorAbsent` at stage 8. |
| Bundle bytes exist in ambient CAS but are absent from closure `M`. | `BundleUnavailable` at stage 1; no CAS lookup occurs. |
| An acquisition artifact is absent from closure `M`. | `ArtifactUnavailable` at stage 10. |
| Supplied acquisition artifact bytes do not match the named digest. | `ArtifactDigestMismatch` at stage 11. |
| An acquisition locator is treated as publisher authority. | The bundle may reach only `MatchedAcquisition`; no publisher-authority context exists and no origin admission is produced. |
| A derivation parent identity equals its derived identity. | `ParentEqualsDerived` at stage 6. |
| Parent and output hashes match, but the claimed transformation was performed incorrectly. | At most `MatchedDerivation`; no transformation-correctness context exists. |
| A caller supplies `verified: true` inside the bundle. | `UnknownField` at stage 4. A value outside the bundle is not accepted as authority or as an outcome selector. |
| The exact bundle is anchored only after historical tip `H`. | `AnchorAbsent` at stage 8 for resolution over `H`. |
| The same canonical bytes are supplied under an anchor identity differing only in `witness_root`. | `WitnessRootMismatch` at stage 9 after exact occurrence selection; an earlier differing tuple field instead returns its stage-8 mismatch. |
| A closure key contains uppercase root hex. | No such identity can occur in a valid accepted Magpie replay; exact lookup returns `AnchorAbsent` at stage 8. |
| The anchor uses algorithm alias `sha-256`. | `WitnessAlgorithmMismatch` at stage 8. |
| Parent bytes are missing but derived bytes are present. | `ParentArtifactUnavailable` at stage 10. |
| Parent bytes match but derived bytes do not. | `DerivedArtifactDigestMismatch` at stage 11. |

## 20. Authority and non-authority claims

The protocol preserves these boundaries everywhere:

```text
canonical statement verification
!= truth

artifact hash match
!= locator authenticity

acquisition bundle match
!= source identity

derivation bundle match
!= transformation correctness

bundle occurrence
!= origin admission

origin admission
!= support

support
!= aggregation
```

The verifier proves the exact statement, exact anchor, and exact named content
identities only. `acquisition_profile` and `transformation_profile` are opaque
descriptors. A later explicit origin-admission policy may select trusted
profiles; this verifier contract does not.

`SegmentAnchored` retains occurrence/inclusion semantics only. The bundle does
not create a claim, admission result, support contribution, aggregation input,
or standing effect by itself. A caller-created context with identical serialized
shape is not trusted resolver provenance.

## 21. Timestamp and signing decisions

Neither v0 schema contains a timestamp. The signed Magpie anchor event records
chain inclusion time and total order. That timestamp is not automatically an
acquisition time or transformation time. V0 makes no acquisition-time or
derivation-time claim. Adding a time-bearing statement requires a later
explicit schema version; IDs and locator syntax must not smuggle in time
authority.

Neither v0 bundle contains an embedded signature or self-declared signer.
Integrity and occurrence derive from exact canonical bytes, their SHA-256 root,
the exact same-replay `SegmentAnchored` event, and Magpie chain verification.
No embedded signature is needed for this internal v0 contract, and a bundle
could not bootstrap authority by naming its own key anyway. A future external
export may use DSSE/in-toto under a separately reviewed contract.

## 22. Future implementation sequence

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

The immediate next PR after Ticket 0038 combines only portable fixture files,
standing-inert verification, and hostile tests, with no standing effect.

## 23. Frozen surfaces and explicit non-goals

This contract does not modify or reinterpret `magpie-core-v1`, existing payload
tags, `SegmentAnchored` fields or semantics, existing golden vectors, Deadbolt
code, policies v0/v1/v2, snapshot bytes, resolution bytes, evidence ceilings,
existing `content_hash` values, Ticket 0037, the release contract, workspace
version, package inventories, `Cargo.lock`, or CI.

It adds no Rust, tests, fixture files, Cargo dependency, parser,
canonicalization implementation, hashing implementation, closure manifest
encoding, runtime CAS, filesystem or network access, callback, plugin,
downloader, L0 payload tag, origin-binding schema, origin-admission policy,
origin group, aggregation lane, policy v3, support aggregation, direct
refutation, contradiction debt, invalidation, supersession, currentness,
`EpistemicGate`, writer authority, crawler, model/librarian integration, or
production-readiness claim.

The sibling families must not be replaced with a mixed statement list, generic
metadata map, caller-selected authority role, origin-binding variant, or generic
provenance envelope.

## 24. Reviewer checklist

- Confirm the status remains a proposed exact protocol contract.
- Confirm there are exactly two distinguishable single-statement families.
- Confirm all schema, bundle-kind, canonicalization, verifier, and algorithm
  identifiers are exact.
- Confirm every field and nested field has one exact canonical order.
- Confirm strings, controls, escapes, UTF-8, and no-normalization behavior are
  unambiguous.
- Confirm unknown and duplicate fields fail closed and canonical input equality
  is mandatory.
- Confirm roots are SHA-256 over exact canonical bytes with no prefix.
- Confirm anchor matching uses all five fields and never root alone.
- Confirm bundle and artifact closure inputs are explicit and immutable, with no
  ambient lookup.
- Confirm primary failure precedence is deterministic and independent of map
  iteration.
- Confirm the two normative examples reproduce the independently calculated
  lengths, digests, roots, and anchor tuples.
- Confirm acquisition does not create publisher, locator, transport, truth, or
  origin authority.
- Confirm derivation does not prove transformation correctness or graph-wide
  lineage.
- Confirm timestamps and embedded signers gain no hidden authority.
- Confirm no runtime verifier, fixture, standing, origin-admission, support, or
  aggregation claim appears.
