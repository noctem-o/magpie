# Resolution Content Closure v0

## 1. Status

Ratified architecture contract with production construction and exact lookup.
Ticket 0040 landed the contract; Ticket 0041 implements the first production,
standing-inert `ResolutionContentClosureV0` construction surface.

This document fixes the immutable closure boundary, exact Rust surface,
construction laws, complete first-failure precedence, exact v0 limits,
canonical manifest and typed identity. No loader, parser, manifest
deserializer, CAS integration, orchestrator wiring or verifier rewiring exists.
The existing artifact-provenance verifier still receives explicit optional
byte slices.

## 2. Purpose

One deterministic resolution must not change because ambient object
availability changed between calls. The closure freezes exactly:

> Which exact byte objects were supplied under which exact expected keys for
> one resolution attempt?

The frozen universe is finite, immutable and untrusted. It prevents an ambient
filesystem, CAS, callback, mutable store or network from silently changing the
inputs to a resolution. It does not decide whether a key is valid, whether
supplied bytes match that key, whether a bundle is canonical, whether a bundle
root is anchored, whether an artifact digest is correct or whether any
statement is true.

## 3. Architectural law

The following distinctions are normative:

```text
closure membership
!= object validity

closure identity
!= trust root

closure identity
!= artifact identity

closure identity
!= bundle verification

object availability
!= provenance

verified provenance
!= source identity

source identity
!= origin admission

origin admission
!= support

support
!= aggregation

aggregation
!= settlement
```

Closure construction grants no provenance, identity, truth, origin admission,
support, aggregation, settlement or writer authority. Those checks and effects
remain in the existing profile-specific verifier and later explicit policies.

## 4. Relationship to `(H, P, M)`

The deterministic input law is:

```text
verified log prefix H
+ explicit policy identities P
+ immutable resolution content closure M
-> deterministic derived result

same H + same P + same M
-> byte-identical result
```

`H` establishes the accepted replay prefix relative to an externally selected
Magpie verifying key. `P` selects explicit versioned policy behavior. `M`
freezes only the finite keyed availability universe. None substitutes for
another.

An object absent from `M` is unavailable for that resolution. A later loader
may construct a different `M2`, but it does not mutate or reinterpret the
result for `(H, P, M1)`.

## 5. Object classes and exact keys

`ResolutionContentClosureV0` contains exactly two disjoint classes of
untrusted objects.

### 5.1 Artifact objects

An artifact object is keyed by this exact two-field expected identity, in this
field order:

```text
algorithm
digest
```

The key is exact untrusted lookup material. It is not proof that the supplied
bytes match the expected algorithm or digest. Closure construction applies the
v0 key-field resource bound but does not perform artifact verification.

Historical `EvidenceRegistered.content_hash` strings are not reinterpreted as
closure keys. Filenames, URLs, paths, MIME types, titles, DOIs and database
identifiers are not artifact keys.

### 5.2 Foreign-bundle objects

A foreign-bundle object is keyed by the exact five-field anchor selector
identity, in the exact order of `ArtifactProvenanceAnchorSelectorV0`:

```text
bundle_kind
witness_root
witness_algorithm
canonicalization_profile
run_id
```

A bare root, run ID, bundle kind, filename, URL, CAS path or array index is not
an adequate key. The five strings remain untrusted lookup material. Closure
membership does not establish that the selector is valid or that a matching
anchor exists in an accepted replay.

### 5.3 Disjoint namespaces

Artifact and foreign-bundle namespaces are disjoint. An artifact key and a
foreign-bundle key cannot conflict merely because their strings or retained
bytes happen to match.

## 6. Construction and immutability

Construction consumes an explicitly supplied, already finite collection of
key/byte pairs. It performs no:

```text
filesystem lookup
directory scan
CAS lookup
remote fetch
network access
callback
plugin invocation
environment-variable lookup
database query
second Magpie replay
lazy population
cache fill
background hydration
```

A separately reviewed future loader may gather bytes before construction. That
loader is outside deterministic resolution and outside Ticket 0040.

Once constructed, a closure is immutable. Adding, deleting or replacing an
object constructs a different closure. It never mutates an existing resolution
input. An empty closure is valid.

## 7. Duplicate and conflict semantics

Construction uses exact set semantics within each class:

```text
same class
+ same exact key
+ byte-identical content
-> idempotent duplicate
-> one retained closure object
-> no multiplicity in the canonical manifest
-> no additional authority

same class
+ same exact key
+ different content bytes
-> construction conflict
-> no closure is constructed

different exact keys
+ byte-identical content
-> two retained keyed objects
-> both keys remain visible in the canonical manifest
-> no inferred identity, provenance or corroboration relationship
```

Construction never selects first-write-wins, last-write-wins, lexical-minimum
bytes, newest input or caller ordering.

## 8. Resource bounds

The exact v0 construction limits are:

| resource | v0 maximum |
| --- | ---: |
| supplied artifact entries | 1,024 |
| supplied foreign-bundle entries | 1,024 |
| total supplied entries | 2,048 |
| UTF-8 bytes in any individual key field | 1,024 |
| bytes in one artifact object | 67,108,864 (64 MiB) |
| bytes in one foreign-bundle object | 1,048,576 (1 MiB) |
| supplied object bytes across one closure | 536,870,912 (512 MiB) |

Entry-count checks apply to supplied entries before exact-duplicate collapse.
Individual key-field and object limits likewise apply to supplied entries.

Total supplied object bytes are the checked sum of the byte length of every
supplied artifact entry and every supplied foreign-bundle entry before exact-
duplicate collapse. Every occurrence contributes its full length: three
same-key/same-byte entries count three times, and identical bytes supplied
under different keys count once per supplied entry. An empty closure supplies
zero object bytes. Entries that later conflict still contribute their full
length according to the future fixed first-failure algorithm; caller insertion
order cannot exempt an entry or choose the winning failure. The supplied-total
cap controls construction work before retention and before duplicates can
reduce the stored set.

Entry-count and total-supplied-byte checks jointly bound pre-collapse
construction work. Entry counts cap structural multiplicity; the supplied-
byte cap bounds total object payload presented before duplicate collapse.

For every successfully constructed v0 closure, the following derived invariant
holds:

```text
retained object bytes
<= supplied object bytes
<= 536,870,912
```

The retained total is the checked sum of retained objects after exact
same-key/same-byte duplicate collapse. Collapse can only remove duplicate
occurrences; it cannot add bytes. Byte-identical objects retained under
different keys each contribute their full length, but each retained occurrence
already contributed to the supplied total. V0 therefore has no independent
post-collapse byte cap or post-collapse construction failure.

A future incompatible profile may introduce a separate retained-memory budget
when its retained cap is lower than its supplied cap. That profile must ratify
its own limit and closed outcome rather than reserving an unreachable v0
failure.

All length conversion and addition use checked arithmetic. The Ticket 0041
reachability audit found no ordinary v0 production input capable of reaching
an independent length-overflow result after the earlier count and per-object
checks. At most 1,024 artifact objects of 67,108,864 bytes and 1,024 foreign
bundles of 1,048,576 bytes can reach the supplied-total stage, a structurally
possible sum of approximately 65 GiB that fits in `u64`. Successful
construction is further limited to 536,870,912 supplied bytes.

Therefore impossible conversion or addition failure fails closed as
`TotalSuppliedBytesExceeded` with a saturated `u64` audit value. Checked
arithmetic remains mandatory. No independent `LengthOverflow` outcome exists
in v0.

The 1 MiB foreign-bundle cap is an outer closure resource bound. A selected
profile-specific verifier retains every stricter bound. In particular, the
current artifact-provenance verifier still rejects bundles larger than 16,384
bytes before parsing.

These are the exact limits of this v0 profile, not permanent universal limits.
An incompatible future limit or behavior requires a new profile or schema
identity.

## 9. Canonical manifest schema

Every successfully constructed closure has one purpose-built canonical JSON
manifest. The closure schema identity is exactly:

```text
magpie-resolution-content-closure-v0
```

The exact top-level shape and field order are:

```json
{
  "schema": "magpie-resolution-content-closure-v0",
  "artifact_objects": [],
  "foreign_bundle_objects": []
}
```

The displayed whitespace is explanatory. Canonical bytes contain none.

Each artifact entry has exact field order:

```json
{
  "key": {
    "algorithm": "...",
    "digest": "..."
  },
  "byte_length": 0,
  "content_sha256": "..."
}
```

Each foreign-bundle entry has exact field order:

```json
{
  "key": {
    "bundle_kind": "...",
    "witness_root": "...",
    "witness_algorithm": "...",
    "canonicalization_profile": "...",
    "run_id": "..."
  },
  "byte_length": 0,
  "content_sha256": "..."
}
```

Each entry records the exact expected lookup key, the exact supplied object
length as a `u64` decimal integer and lowercase SHA-256 over the exact supplied
object bytes. Raw object bytes are not embedded.

The actual object digest is retained even when it disagrees with the expected
key:

```text
artifact key digest D
+ supplied bytes whose SHA-256 is X
+ D != X
-> closure may still construct
-> manifest records key digest D and content_sha256 X
-> later artifact verification reports the digest mismatch
```

For every foreign-bundle entry, `content_sha256` is the closure-profile SHA-256
over the exact supplied foreign-bundle object bytes, regardless of the
selector's `witness_algorithm`:

```text
content_sha256
= lowercase_hex(SHA-256(exact supplied foreign-bundle object bytes))

content_sha256
!= generic computed witness root
```

A selected profile-specific verifier is responsible for parsing or otherwise
interpreting the supplied bytes under its fixed reviewed profile, computing
that profile's witness root under its supported witness algorithm and
canonicalization rules, and comparing that result with `witness_root`.

For the current artifact-provenance v0 profile only,
`witness_algorithm = sha256`, accepted bundle bytes are the exact canonical
bundle bytes, and the computed profile-specific witness root is SHA-256 over
those accepted bytes. Therefore, only after canonical acceptance under that
profile, the computed witness root equals `content_sha256` because both hash
the same exact bytes with SHA-256. Closure construction still performs no such
comparison. A future profile whose witness root uses another supported
algorithm or a transformed root construction has no implied equality with
`content_sha256`.

## 10. Canonical encoding

The canonicalization profile identity is exactly:

```text
magpie-resolution-content-closure-json-v0
```

This is a purpose-built fixed-schema encoder. It is not an arbitrary JSON
canonicalization algorithm, accepts no arbitrary schema, and remains separate
from `magpie-artifact-provenance-json-v0` even though it uses the same string
emission discipline.

Canonical strings use exactly:

```text
\" for quotation mark
\\ for reverse solidus
\b \t \n \f \r for those controls
lowercase \u00xx for other U+0000-U+001F controls
direct UTF-8 for every other Unicode scalar
no Unicode normalization
no escaped solidus
no unnecessary escaping
no uppercase hexadecimal escape digits
no whitespace
no BOM
no trailing newline
```

Integers use the shortest unsigned base-10 representation, with no leading
zero except the value zero. Ticket 0041 implements no parser or manifest
deserialization API.

## 11. Ordering

Artifact entries are ordered lexicographically by the exact tuple:

```text
algorithm
digest
```

Foreign-bundle entries are ordered lexicographically by:

```text
bundle_kind
witness_root
witness_algorithm
canonicalization_profile
run_id
```

Comparison is bytewise over exact UTF-8 field bytes. There is no locale, case
folding, Unicode normalization, path normalization, URL normalization or
semantic comparison. The artifact array is emitted before the foreign-bundle
array. Input order and duplicate occurrence order cannot affect manifest bytes
or closure identity.

## 12. Closure identity

The closure digest algorithm identity is exactly:

```text
sha256
```

The exact semantic type is:

```text
ResolutionContentClosureIdentityV0 {
    schema:
        "magpie-resolution-content-closure-v0"

    canonicalization_profile:
        "magpie-resolution-content-closure-json-v0"

    digest_algorithm:
        "sha256"

    manifest_sha256:
        <exactly 64 lowercase hexadecimal characters>
}
```

The displayed field order is normative for the future typed public and audit
surface. The identity has exactly these four fields. It has no alias, optional
field, extension, metadata map, negotiation value or runtime profile selector.

Its digest component is derived exactly as:

```text
manifest_sha256
= lowercase_hex(
    SHA-256(
      exact canonical manifest bytes under
      magpie-resolution-content-closure-json-v0
    )
  )
```

Equality is exact equality of all four fields:

```text
ResolutionContentClosureIdentityV0 equality
= exact equality of schema
+ exact equality of canonicalization_profile
+ exact equality of digest_algorithm
+ exact equality of manifest_sha256
```

The complete typed `ResolutionContentClosureIdentityV0`, not the bare
64-character `manifest_sha256`, is the closure identity. The bare value is only
the digest component. The manifest contains neither its own digest nor the
canonicalization-profile or digest-algorithm identity, so its digest cannot
impersonate the complete typed identity. This contract introduces no second
hash over a serialization of the four fields.

There is no alias, negotiation table, runtime-selected profile, generic
registry or implicit latest version. A future incompatible change requires a
new profile or schema identity.

The closure identity is an exact commitment to one finite keyed availability
universe. It is not a signature, trust root, provenance receipt, archive
guarantee, proof that an external store retains the bytes, proof that object
keys match object bytes, proof that bundles are canonical, proof that anchors
occurred, proof of origin or proof of truth.

A typed closure identity alone cannot reconstruct the closure. The manifest
contains only object digests and lengths, not raw object bytes. A caller-
created, well-shaped four-field value remains untrusted audit material and is
not accepted as proof that reviewed construction occurred. Trusted future
construction begins from explicit finite key/byte inputs and the reviewed
constructor path.

## 13. Lookup semantics

The implemented closure API permits only exact lookup:

```text
artifact lookup:
exact artifact key
-> exact retained bytes or unavailable

foreign-bundle lookup:
exact five-field selector key
-> exact retained bytes or unavailable
```

There is no prefix, wildcard, fallback, nearest-match, root-only, filename,
URL, path, content search, alias, case-insensitive or semantic lookup.

```text
object absent from M1
-> unavailable under M1

loader later supplies object
-> constructs M2

M1 result
-> remains the result for M1

M2 result
-> distinct resolution over distinct closure
```

## 14. Interaction with existing verifiers

The closure is an outer availability boundary. It does not change the existing
artifact-provenance verifier or its 13-stage semantics.

A future orchestrator may perform:

```text
exact closure lookup
-> optional exact retained byte slice
-> existing snapshot-only verifier invocation
```

Closure lookup is a separate stage before invocation of the current
slice-based APIs:

```rust
StandingReplaySnapshot::resolve_artifact_acquisition_context_v0
StandingReplaySnapshot::resolve_artifact_derivation_context_v0
```

Ticket 0041 does not alter their signatures or behavior and does not wire the
closure into either method. Once bytes are supplied, the verifier's existing
parsing, schema, canonical equality, selector comparison, root, replay
occurrence, artifact and receipt semantics remain unchanged. Lookup success
establishes no successful verifier context.

## 15. Exact production Rust surface

Ticket 0041 adds and re-exports these exact constants:

```rust
pub const RESOLUTION_CONTENT_CLOSURE_SCHEMA_V0: &str =
    "magpie-resolution-content-closure-v0";
pub const RESOLUTION_CONTENT_CLOSURE_CANONICALIZATION_PROFILE_V0: &str =
    "magpie-resolution-content-closure-json-v0";
pub const RESOLUTION_CONTENT_CLOSURE_DIGEST_ALGORITHM_V0: &str = "sha256";
pub const MAX_RESOLUTION_CONTENT_CLOSURE_ARTIFACT_ENTRIES_V0: usize = 1_024;
pub const MAX_RESOLUTION_CONTENT_CLOSURE_FOREIGN_BUNDLE_ENTRIES_V0: usize = 1_024;
pub const MAX_RESOLUTION_CONTENT_CLOSURE_TOTAL_ENTRIES_V0: usize = 2_048;
pub const MAX_RESOLUTION_CONTENT_CLOSURE_KEY_FIELD_BYTES_V0: usize = 1_024;
pub const MAX_RESOLUTION_CONTENT_CLOSURE_ARTIFACT_OBJECT_BYTES_V0: usize = 67_108_864;
pub const MAX_RESOLUTION_CONTENT_CLOSURE_FOREIGN_BUNDLE_OBJECT_BYTES_V0: usize = 1_048_576;
pub const MAX_RESOLUTION_CONTENT_CLOSURE_SUPPLIED_OBJECT_BYTES_V0: u64 = 536_870_912;
```

The public types are exactly:

```rust
pub struct ResolutionArtifactObjectKeyV0
pub struct ResolutionArtifactObjectInputV0<'a>
pub struct ResolutionForeignBundleObjectInputV0<'a>
pub enum ResolutionContentClosureKeyFieldV0
pub enum ResolutionContentClosureConstructionErrorV0
pub struct ResolutionContentClosureIdentityV0
pub struct ResolutionContentClosureV0
```

Artifact keys privately store `algorithm` then `digest`. Foreign-bundle input
keys reuse the exact five-field `ArtifactProvenanceAnchorSelectorV0`. Both
borrowed input types expose their untrusted exact key, byte slice and byte
length, and their `Debug` output omits raw payload bytes. Key and selector
constructors are infallible and perform no semantic validation.

The key-field enum declaration order and field-failure precedence are exactly:

```text
ArtifactAlgorithm
ArtifactDigest
BundleKind
WitnessRoot
WitnessAlgorithm
CanonicalizationProfile
RunId
```

The public construction error is adjacently tagged with `outcome` and
`details`, uses snake-case variant names, and contains exactly nine failures:

```rust
TotalEntryLimitExceeded {
    supplied_entries: u64,
    maximum_entries: u64,
}
ArtifactEntryLimitExceeded {
    supplied_artifact_entries: u64,
    maximum_entries: u64,
}
ForeignBundleEntryLimitExceeded {
    supplied_foreign_bundle_entries: u64,
    maximum_entries: u64,
}
KeyFieldTooLarge {
    field: ResolutionContentClosureKeyFieldV0,
    offending_entries: u64,
    maximum_observed_bytes: u64,
    maximum_bytes: u64,
}
ArtifactObjectTooLarge {
    key: ResolutionArtifactObjectKeyV0,
    maximum_observed_bytes_for_key: u64,
    maximum_bytes: u64,
}
ForeignBundleObjectTooLarge {
    selector: ArtifactProvenanceAnchorSelectorV0,
    maximum_observed_bytes_for_key: u64,
    maximum_bytes: u64,
}
TotalSuppliedBytesExceeded {
    supplied_object_bytes: u64,
    maximum_bytes: u64,
}
ConflictingArtifactObject {
    key: ResolutionArtifactObjectKeyV0,
}
ConflictingForeignBundleObject {
    selector: ArtifactProvenanceAnchorSelectorV0,
}
```

`Ok(ResolutionContentClosureV0)` is the conceptual `Constructed` result. It is
not an error variant. There is no independent length-overflow or retained-byte
failure, generic invalid result, arbitrary error string or fallback variant.

The only production constructor is the associated function:

```rust
pub fn construct<'a>(
    artifact_entries: &[ResolutionArtifactObjectInputV0<'a>],
    foreign_bundle_entries: &[ResolutionForeignBundleObjectInputV0<'a>],
) -> Result<
    ResolutionContentClosureV0,
    ResolutionContentClosureConstructionErrorV0,
>
```

It starts from explicit borrowed key/byte inputs, mutates no caller data,
copies only successfully retained objects, and performs no lookup, replay or
artifact-provenance verification. The closure owns two private deterministic
`BTreeMap` values, canonical manifest bytes, the typed identity, supplied byte
total and retained byte total. It is not serializable or deserializable and
has no mutable accessor. Its custom `Debug` discloses identity, counts and byte
totals but not raw objects.

Read-only getters expose identity, canonical manifest bytes, both counts,
emptiness, both byte totals, exact artifact lookup and exact five-field bundle
lookup. Lookup returns `Option<&[u8]>`. No iterator or approximate, partial,
fallback or hydrating lookup is part of v0.

## 16. Complete first-failure algorithm

The constructor applies this exact category precedence before retaining bytes:

```text
1. TotalEntryLimitExceeded
2. ArtifactEntryLimitExceeded
3. ForeignBundleEntryLimitExceeded
4. KeyFieldTooLarge
5. ArtifactObjectTooLarge
6. ForeignBundleObjectTooLarge
7. TotalSuppliedBytesExceeded
8. ConflictingArtifactObject
9. ConflictingForeignBundleObject
10. Ok(ResolutionContentClosureV0)
```

Stages 1-3 count every occurrence before duplicate collapse. Total count wins
over either class count. Impossible count conversion or addition returns the
total-entry failure with saturated `u64` values.

Stage 4 visits the seven field categories in enum declaration order. For the
first category containing a value longer than 1,024 UTF-8 bytes it reports one
aggregate result: the offending occurrence count and maximum observed byte
length. This avoids sorting unbounded hostile strings. There is no validation,
normalization or case folding.

Stage 5 chooses the lexicographically smallest exact two-field artifact key
among oversized artifact entries and reports the maximum oversized length
observed for that selected key. Stage 6 applies the same law to the exact
five-field bundle selector. These comparisons occur only after every key field
is bounded.

Stage 7 checked-sums every supplied occurrence before collapse. The inclusive
maximum proceeds; one byte more fails. Impossible conversion or addition
returns `TotalSuppliedBytesExceeded` with a saturated audit value.

Stages 8 and 9 group by the exact class key. Same-key byte-identical entries
are idempotent. Any differing byte slice conflicts, and the lexicographically
smallest conflicting key wins independently of caller order. Artifact conflict
always precedes bundle conflict. Equality is exact bytes, not length or hash.

Only stage 10 copies one owned byte vector per exact key into deterministic
maps, computes retained bytes, creates the purpose-built manifest and derives
the four-field identity. It enforces the derived invariant `retained <=
supplied <= 536,870,912` without adding another retained-memory limit.

## 17. Normative vectors

The vectors use the committed files in
`fixtures/artifact-provenance-v0/`. Two independently structured temporary
Python calculations read those bytes, recomputed every object length and
SHA-256, sorted the exact key tuples and independently encoded the manifest.
Both produced the same results.

For every vector, the first byte is `0x7b` (`{`), the final byte is `0x7d`
(`}`), no UTF-8 BOM is present and there is no trailing newline.

| vector | artifact count | foreign-bundle count | retained object bytes | manifest bytes | manifest SHA-256 |
| --- | ---: | ---: | ---: | ---: | --- |
| empty | 0 | 0 | 0 | 99 | `95446014b15ce3e3cb63784da4898defad484c34ff29ddd09d662bd7a2b58092` |
| acquisition | 1 | 1 | 306 | 663 | `2253d66f473c8b0f71a47f308ef4c0dffbb6f56e522b58c54bfd5519f958a224` |
| complete existing fixture | 3 | 2 | 692 | 1,434 | `2e0f59ee647407c6e66ae7e05913788b92b0b4f85c739f4ed3bc231511a30a84` |

The listed hashes are the `manifest_sha256` components, not complete typed
identities. Because the other three fields are fixed v0 constants, each full
identity is uniquely determined as follows:

```text
empty:
ResolutionContentClosureIdentityV0 {
    schema: "magpie-resolution-content-closure-v0"
    canonicalization_profile: "magpie-resolution-content-closure-json-v0"
    digest_algorithm: "sha256"
    manifest_sha256: "95446014b15ce3e3cb63784da4898defad484c34ff29ddd09d662bd7a2b58092"
}

acquisition:
ResolutionContentClosureIdentityV0 {
    schema: "magpie-resolution-content-closure-v0"
    canonicalization_profile: "magpie-resolution-content-closure-json-v0"
    digest_algorithm: "sha256"
    manifest_sha256: "2253d66f473c8b0f71a47f308ef4c0dffbb6f56e522b58c54bfd5519f958a224"
}

complete existing fixture:
ResolutionContentClosureIdentityV0 {
    schema: "magpie-resolution-content-closure-v0"
    canonicalization_profile: "magpie-resolution-content-closure-json-v0"
    digest_algorithm: "sha256"
    manifest_sha256: "2e0f59ee647407c6e66ae7e05913788b92b0b4f85c739f4ed3bc231511a30a84"
}
```

### 17.1 Empty closure

Exact canonical manifest bytes:

```json
{"schema":"magpie-resolution-content-closure-v0","artifact_objects":[],"foreign_bundle_objects":[]}
```

### 17.2 Acquisition closure

Objects:

- artifact key `sha256`,
  `51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076`;
  15 bytes; actual SHA-256 equals the key digest;
- bundle key `magpie-artifact-acquisition-v0`,
  `4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e`,
  `sha256`, `magpie-artifact-provenance-json-v0`, `run-acq-0001`;
  291 bytes; actual SHA-256 equals the key root.

Exact canonical manifest bytes:

```json
{"schema":"magpie-resolution-content-closure-v0","artifact_objects":[{"key":{"algorithm":"sha256","digest":"51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076"},"byte_length":15,"content_sha256":"51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076"}],"foreign_bundle_objects":[{"key":{"bundle_kind":"magpie-artifact-acquisition-v0","witness_root":"4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e","witness_algorithm":"sha256","canonicalization_profile":"magpie-artifact-provenance-json-v0","run_id":"run-acq-0001"},"byte_length":291,"content_sha256":"4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e"}]}
```

### 17.3 Complete existing fixture closure

The exact sorted manifest field table is:

| class | exact key tuple | byte length | content SHA-256 |
| --- | --- | ---: | --- |
| artifact | `sha256` \| `1921b918b15842c7fdb115078e610263fac85f159c1d8e0ecec3d89a0faa4005` | 6 | `1921b918b15842c7fdb115078e610263fac85f159c1d8e0ecec3d89a0faa4005` |
| artifact | `sha256` \| `51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076` | 15 | `51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076` |
| artifact | `sha256` \| `b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060` | 6 | `b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060` |
| foreign bundle | `magpie-artifact-acquisition-v0` \| `4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e` \| `sha256` \| `magpie-artifact-provenance-json-v0` \| `run-acq-0001` | 291 | `4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e` |
| foreign bundle | `magpie-artifact-derivation-v0` \| `472e857153fe8c901b6c0d7d1b15bd876450b57b4743222a2fab91eb60bc76dd` \| `sha256` \| `magpie-artifact-provenance-json-v0` \| `run-der-0001` | 374 | `472e857153fe8c901b6c0d7d1b15bd876450b57b4743222a2fab91eb60bc76dd` |

The table plus the exact schema, entry field orders, canonical encoding and
ordering rules above is machine-checkable and reproduces the pinned 1,434-byte
manifest and its exact digest.

## 18. Hostile cases

| hostile case | required result |
| --- | --- |
| The same five objects are supplied in reverse order. | The same manifest bytes and typed closure identity result. |
| One artifact entry is supplied three times with identical bytes. | One object is retained and the typed closure identity is unchanged; supplied-entry counts and all three object lengths count before collapse. |
| 1,024 identical 64 MiB artifact entries are supplied. | `TotalSuppliedBytesExceeded`; each object is individually valid and the entry count is within its class maximum, but exact-duplicate collapse cannot rescue construction. |
| Several individually valid objects have supplied lengths summing to exactly 536,870,912 bytes. | The input may proceed past the inclusive supplied-total bound, subject to every other deterministic construction check. |
| The same supplied set gains one additional byte. | `TotalSuppliedBytesExceeded`. |
| Duplicate entries exceed 512 MiB supplied but would retain only one 64 MiB object. | `TotalSuppliedBytesExceeded`; the smaller post-collapse retention does not rescue construction. |
| Supplied bytes remain within the 512 MiB cap and distinct keys retain separate objects. | The retained total remains at most the supplied total; v0 has no independent retained-total failure. |
| One artifact key is supplied with two different byte strings. | `ConflictingArtifactObject`; no closure is constructed. |
| One bundle key is supplied with two different byte strings. | `ConflictingForeignBundleObject`; no closure is constructed. |
| The same bytes are supplied under two different artifact keys. | Two entries are retained; no identity, provenance or corroboration relationship is inferred. |
| An artifact key digest disagrees with the supplied bytes. | The closure may construct and records both values; a later verifier may report digest mismatch. |
| A foreign-bundle selector names an unsupported witness algorithm. | The closure may construct and still computes `content_sha256` with closure SHA-256; a later selected verifier rejects or classifies the unsupported profile. |
| A selector `witness_root` differs from `content_sha256` under the current artifact-provenance v0 fields. | The closure may construct; `content_sha256` remains the closure byte commitment, and the current verifier later returns `WitnessRootMismatch` when its independently computed SHA-256 root differs from the selector root. |
| A future profile computes a non-SHA-256 or transformed witness root. | `content_sha256` remains an available-object commitment only; no generic equality with the profile-specific root is inferred. |
| One object is removed. | A different manifest SHA-256 component and typed closure identity result. |
| One object byte changes. | Its `content_sha256`, the manifest SHA-256 component and the typed closure identity change. |
| An object is added to external CAS after closure construction. | The existing closure and its resolution remain unchanged. |
| An object is deleted from external CAS after closure construction. | The existing closure and its resolution remain unchanged. |
| An object is absent from `M1` and present in `M2`. | These are two explicit resolution inputs, not ambient mutation. |
| A closure manifest is copied or serialized. | It is audit material only and recreates neither raw bytes nor authority. |
| The same bare `manifest_sha256` is paired with different schema, profile or algorithm fields. | The four-field values are not equal typed identities; values whose constants differ are not valid v0 identities. No collision claim is implied. |
| A caller supplies a well-shaped `ResolutionContentClosureIdentityV0`. | The value is not accepted as proof that construction occurred or as a substitute for key/byte inputs. |
| A caller copies `manifest_sha256` without the other fixed identity fields. | It has only the digest component, not the complete typed closure identity. |
| One wire report is stored under many URLs. | URL multiplicity is outside closure identity and creates no corroboration. |

## 19. Authority boundary

Closure construction answers only availability. It establishes none of:

```text
valid artifact identity
valid selector identity
canonical foreign-bundle bytes
matching foreign-bundle root
accepted-replay anchor occurrence
artifact digest correctness
provenance
source identity
origin binding
origin admission
support contribution
aggregation input
standing
settlement
writer capability
```

The closure does not know `ContributionIdentity`,
`OriginComparisonNamespace`, origin group, binding authority,
origin-admission policy, corroboration component, polarity or aggregation lane.
No such field belongs in the v0 manifest.

The typed identity boundary is equally strict:

```text
typed closure identity
!= trusted construction

typed closure identity
!= raw closure reconstruction

typed closure identity
!= object verification

typed closure identity
!= signature

typed closure identity
!= provenance

typed closure identity
!= origin authority
```

## 20. Implementation sequence

The implementation ledger is:

```text
ResolutionContentClosureV0 contract — landed
ResolutionContentClosureV0 implementation — landed
exact origin-binding bundle contract — next
origin-binding verifier
standing-inert origin-admission audit
admitted-contribution audit
policy-v3 contract
conservative aggregation
```

Every stage before policy-v3 remains standing-inert. No later origin,
admission, contribution, policy or aggregation behavior is implemented here.

## 21. Frozen surfaces and non-goals

Tickets 0040 and 0041 do not change or reinterpret `magpie-core-v1`, L0 payloads,
`SegmentAnchored`, Deadbolt occurrence semantics,
`ArtifactProvenanceAnchorSelectorV0`, `StandingReplaySnapshot`, artifact-
provenance verifier behavior, policies v0/v1/v2, existing snapshot or
resolution bytes, fixtures, golden vectors, release metadata, Cargo manifests,
`Cargo.lock`, CI or the `v0.1.0` tag/release.

Ticket 0041 adds only the closure module, exports and hostile tests. It adds no
dependencies, fixtures, manifest parser, loader, writer, MCP surface,
downloader, crawler, CAS,
database, filesystem/network access, callback, plugin, L0 change,
origin-binding schema, origin-admission policy, contribution audit, policy v3,
aggregation, refutation, contradiction debt, invalidation, supersession,
currentness or production-readiness claim.
