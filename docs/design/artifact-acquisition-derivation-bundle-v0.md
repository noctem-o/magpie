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
- invalid UTF-8 is rejected before JSON parsing; and
- malformed JSON escapes, lone escaped surrogates, and invalid surrogate-pair
  structures are rejected as invalid JSON.

A malformed JSON escape, lone escaped surrogate, or invalid surrogate-pair
structure returns `InvalidJson` at stage 3. An invalid UTF-8 byte sequence
returns `InvalidUtf8` at stage 3. A valid escaped surrogate pair denotes one
Unicode scalar value, so parsing succeeds, but its escaped spelling returns
`NonCanonicalEncoding` at stage 7 because that scalar must be emitted directly
as UTF-8. The identifier and locator validators further reject controls as
specified in section 8. No Unicode-specific primary outcome is introduced.

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

## 12. Anchor selector, identity, occurrence, and exact matching

`ArtifactProvenanceAnchorSelectorV0` is an untrusted five-field tuple naming
one attempted bundle verification:

```text
ArtifactProvenanceAnchorSelectorV0

bundle_kind
witness_root
witness_algorithm
canonicalization_profile
run_id
```

The exact Rust type and serialization remain deferred. A selector may come from
an immutable resolution-content closure or another explicitly bounded request
surface. It is lookup material, comparison material, and untrusted input. It is
not proof of occurrence, trusted context, authority, permission to select a
different verifier profile, or a caller-created matched result.

An anchor identity is the exact equality class of one five-field
`SegmentAnchored` payload. An anchor occurrence is one replayed
`SegmentAnchored` event carrying that identity, with a sequence position and
other audit material. The accepted completely verified replay is the sole
source of replay-derived anchor identities and occurrences:

```text
selector A
!= replay-derived anchor identity
!= replay-derived anchor occurrence

only exact occurrence of A in the verified replay
establishes anchored occurrence
```

All five strings compare byte-for-byte. Root-only matching is forbidden. The
reviewed schema determines the expected bundle kind. Reviewed code fixes exact
`sha256`, `magpie-artifact-provenance-json-v0`, and
`magpie-artifact-provenance-verifier-v0`; bundle content, the selector, and
runtime callers do not negotiate them.

After canonical re-encoding equality, selector validation and replay lookup
proceed exactly as follows:

1. derive expected protocol values from the reviewed contract and parsed
   bundle;
2. compare `selector.bundle_kind` with the schema-selected bundle kind;
3. compare `selector.witness_algorithm` with `sha256`;
4. compare `selector.canonicalization_profile` with
   `magpie-artifact-provenance-json-v0`;
5. compare bundle `run_id` with `selector.run_id`;
6. recompute SHA-256 over the exact canonical bundle bytes;
7. compare that lowercase root with `selector.witness_root` byte-for-byte;
8. query the completely verified replay for exact selector identity `A`;
9. verify referenced artifact closure objects; and
10. produce the matched outcome.

The comparisons return, respectively, `BundleKindMismatch`,
`WitnessAlgorithmMismatch`, `CanonicalizationProfileMismatch`,
`RunIdMismatch`, and `WitnessRootMismatch`. Exact replay lookup occurs only
after every selector protocol-field comparison and the root comparison pass.
Only then does zero matching occurrence return `AnchorAbsent`, meaning:

> The attempted selector is internally consistent with the parsed canonical
> bundle and fixed protocol, but no exact corresponding occurrence exists in
> the accepted replay.

The selector root has no separate grammar failure. Uppercase hexadecimal,
wrong-length or non-hexadecimal text, or any other digest differs from the
recomputed lowercase SHA-256 root and returns `WitnessRootMismatch`, provided
the earlier selector fields pass. Any accepted `SegmentAnchored` occurrence
still necessarily carries the frozen 64-lowercase-hex representation.

An identity may have zero, one, or many replay occurrences. `AnchorAbsent`
applies only to zero; one or more occurrences satisfy one existential occurrence
prerequisite. Repetition follows this law:

```text
repeated identical five-field SegmentAnchored events
-> one exact anchor identity
-> multiple audit-visible occurrences
-> one existential anchor match
-> no verification amplification
-> no standing amplification
```

When a selector matches, future standing-inert audit material must retain all
matching replay occurrences in deterministic sequence order. Each retained
occurrence may include sequence, event hash, event provenance, and the
writer-supplied transaction timestamp. Those fields do not participate in
five-field identity equality and confer no additional authority by themselves.
The verifier does not choose only the newest occurrence and uses neither
first-write-wins nor last-write-wins. Five identical occurrences do not create
five matched acquisition or derivation propositions. Multiplicity does not
strengthen truth, provenance, origin admission, support, or standing. The
primary result remains one `MatchedAcquisition` or one `MatchedDerivation`.

## 13. Resolution-content-closure inputs

This contract does not define the final `ResolutionContentClosureV0` type,
manifest encoding, digest profile, or storage layout. Future verification
consumes a finite immutable closure as follows.

An acquisition attempt with selector `A` requires:

- bundle bytes keyed by the complete untrusted selector `A`; and
- artifact bytes keyed by the exact artifact identity in the bundle.

A derivation attempt with selector `A` requires:

- bundle bytes keyed by the complete untrusted selector `A`;
- parent artifact bytes keyed by the exact parent identity; and
- derived artifact bytes keyed by the exact derived identity.

Stage-1 lookup uses the complete selector as its key. Bundle bytes absent under
selector `A` return `BundleUnavailable`. Lookup success neither proves that the
selector occurs in replay nor validates the bytes; a closure may deliberately
carry hostile bytes under a hostile selector for testing. An artifact object
absent from closure is unavailable for that attempt. The verifier performs no
ambient CAS lookup, filesystem scan, network access, callback, plugin
invocation, downloader action, or second mutable-store read. A separate loader
may construct `M` before resolution; changing availability constructs a
different closure.

## 14. Acquisition verification semantics

An acquisition context is `MatchedAcquisition` only if these exact stages pass
in order:

1. bundle bytes are present under the complete untrusted selector;
2. bundle size is at most 16384 bytes;
3. UTF-8, BOM, and JSON parsing pass;
4. duplicate-key and exact-schema validation pass;
5. identifier, locator, and artifact-field validation pass;
6. the derivation-only self-loop stage is inapplicable;
7. canonical re-encoding equals the supplied bytes;
8. selector bundle kind, witness algorithm, canonicalization profile, and
   `run_id` comparisons pass in that order;
9. the recomputed root equals `selector.witness_root`;
10. at least one exact occurrence of the selector exists in the same completely
    verified replay, with all matching occurrences retained in sequence order;
11. artifact bytes are present under the exact artifact identity;
12. SHA-256 over those artifact bytes equals the bundle digest; and
13. one `MatchedAcquisition` primary outcome is produced.

`MatchedAcquisition` proves only:

> The accepted Magpie replay contains an exact anchor for this canonical
> acquisition statement, and the supplied artifact bytes match the statement's
> exact SHA-256 identity.

It does not prove locator authenticity, publisher or source identity,
authorship, secure transport, acquisition-profile trust, origin admission, or
truth.

## 15. Derivation verification semantics

A derivation context is `MatchedDerivation` only if these exact stages pass in
order:

1. bundle bytes are present under the complete untrusted selector;
2. bundle size is at most 16384 bytes;
3. UTF-8, BOM, and JSON parsing pass;
4. duplicate-key and exact-schema validation pass;
5. identifier and artifact-field validation pass;
6. parent and derived artifact identities are distinct;
7. canonical re-encoding equals the supplied bytes;
8. selector bundle kind, witness algorithm, canonicalization profile, and
   `run_id` comparisons pass in that order;
9. the recomputed root equals `selector.witness_root`;
10. at least one exact occurrence of the selector exists in the same completely
    verified replay, with all matching occurrences retained in sequence order;
11. parent and then derived artifact bytes are present under their exact
    identities;
12. parent and then derived artifact bytes match their exact SHA-256 identities;
    and
13. one `MatchedDerivation` primary outcome is produced.

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

1. bundle closure lookup;
2. bundle byte-size limit;
3. UTF-8, BOM, and JSON parsing;
4. duplicate-key and exact-schema validation;
5. identifier, locator, and artifact-field validation;
6. derivation self-loop validation;
7. canonical re-encoding equality;
8. selector protocol-field validation;
9. witness-root recomputation and equality;
10. exact selector lookup in the verified replay;
11. referenced artifact closure lookup;
12. referenced artifact digest verification; and
13. matched result.

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
  `CanonicalizationProfileMismatch`, then `RunIdMismatch`;
- stage 9: `WitnessRootMismatch`;
- stage 10: `AnchorAbsent` only when the selector is internally consistent with
  the parsed canonical bundle and fixed protocol but has zero exact occurrences
  in the verified replay;
- stage 11: acquisition `ArtifactUnavailable`; derivation
  `ParentArtifactUnavailable` before `DerivedArtifactUnavailable`;
- stage 12: acquisition `ArtifactDigestMismatch`; derivation
  `ParentArtifactDigestMismatch` before `DerivedArtifactDigestMismatch`; and
- stage 13: `MatchedAcquisition` or `MatchedDerivation`.

Duplicate detection retains keys and must not use a map that silently replaces
earlier values. Because the primary duplicate result is the same regardless of
which key duplicated, map iteration cannot change it. Required fields, nested
artifact fields, semantic validation, and referenced artifacts are always
visited in the schema orders above. When validation reaches a nested artifact
object in top-level schema order, it validates that object's `algorithm` and
`digest` fields before continuing to the next top-level field. Unknown-field
diagnostics may retain the earliest input byte offset, but no unordered map
chooses the primary outcome. Closure order, replay iteration order, and
occurrence multiplicity likewise cannot choose or multiply the primary result.

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
| A malformed JSON escape. | `InvalidJson` at stage 3. |
| A lone escaped surrogate or invalid surrogate-pair structure. | `InvalidJson` at stage 3. |
| An invalid UTF-8 byte sequence. | `InvalidUtf8` at stage 3. |
| A valid escaped surrogate pair representing a Unicode scalar. | Parsing succeeds, then `NonCanonicalEncoding` at stage 7 because the scalar must be emitted directly as UTF-8. |
| Canonical acquisition bytes are supplied under a derivation selector kind. | `BundleKindMismatch` at stage 8. |
| The selector uses witness algorithm alias `sha-256`. | `WitnessAlgorithmMismatch` at stage 8. |
| The selector uses another or unknown canonicalization profile. | `CanonicalizationProfileMismatch` at stage 8. |
| Bundle `run_id` differs from `selector.run_id`. | `RunIdMismatch` at stage 8. |
| `selector.witness_root` differs from the recomputed canonical-bundle root. | `WitnessRootMismatch` at stage 9, before replay occurrence lookup. |
| The selector is fully consistent with the bundle, fixed protocol, and recomputed root, but has no exact occurrence in the accepted replay. | `AnchorAbsent` at stage 10. |
| Bundle bytes exist in ambient CAS but are absent from closure `M`. | `BundleUnavailable` at stage 1; no CAS lookup occurs. |
| An acquisition artifact is absent from closure `M`. | `ArtifactUnavailable` at stage 11. |
| Supplied acquisition artifact bytes do not match the named digest. | `ArtifactDigestMismatch` at stage 12. |
| An acquisition locator is treated as publisher authority. | The bundle may reach only `MatchedAcquisition`; no publisher-authority context exists and no origin admission is produced. |
| A derivation parent identity equals its derived identity. | `ParentEqualsDerived` at stage 6. |
| Parent and output hashes match, but the claimed transformation was performed incorrectly. | At most `MatchedDerivation`; no transformation-correctness context exists. |
| A caller supplies `verified: true` inside the bundle. | `UnknownField` at stage 4. A value outside the bundle is not accepted as authority or as an outcome selector. |
| The exact bundle is anchored only after historical tip `H`. | For resolution over `H`, a fully consistent selector returns `AnchorAbsent` at stage 10. |
| A selector root is uppercase, wrong-length, non-hexadecimal, or another digest. | `WitnessRootMismatch` at stage 9, provided earlier selector fields pass. |
| The same canonical bytes are present under a different closure selector. | Validation proceeds against that selector; the first exact stage-8 or stage-9 mismatch above wins. Closure-key presence is not authority. |
| Parent bytes are missing but derived bytes are present. | `ParentArtifactUnavailable` at stage 11. |
| Parent bytes match but derived bytes do not. | `DerivedArtifactDigestMismatch` at stage 12. |
| The same exact selector occurs three times in the verified replay. | One matched primary outcome is produced; three audit occurrences are retained in deterministic sequence order; there is no verification or standing amplification. |

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

Neither v0 schema contains a timestamp. A `SegmentAnchored` event carries a
signed writer-supplied transaction timestamp and a verified sequence position
in the accepted chain. Chain verification protects the recorded timestamp value
from undetected alteration; it does not externally attest that the writer's
clock was accurate. Sequence establishes accepted-chain total order. V0 makes
no trusted wall-clock, acquisition-time, transformation-time, or valid-time
claim. Repeated-occurrence timestamps are audit material only. Adding a
time-bearing statement requires a later explicit schema version; IDs and
locator syntax must not smuggle in time authority.

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
- Confirm the untrusted selector is distinct from replay-derived anchor identity
  and occurrence and never proves either.
- Confirm selector protocol-field mismatches precede root comparison, root
  comparison precedes replay lookup, and only a fully consistent absent selector
  returns `AnchorAbsent`.
- Confirm exact anchor matching uses all five fields and never root alone.
- Confirm repeated identical anchors retain every occurrence in deterministic
  sequence order while producing one existential match and no amplification.
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
- Confirm the signed writer transaction timestamp is audit material, not trusted
  wall-clock, acquisition, transformation, or valid time.
- Confirm malformed escapes, lone escaped surrogates, and invalid surrogate-pair
  structures are `InvalidJson`, invalid UTF-8 is `InvalidUtf8`, and valid escaped
  surrogate pairs are `NonCanonicalEncoding`.
- Confirm timestamps and embedded signers gain no hidden authority.
- Confirm no runtime verifier, fixture, standing, origin-admission, support, or
  aggregation claim appears.
