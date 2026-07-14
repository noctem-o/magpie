# Resolution Content Closure v0

## 1. Status

Ratified architecture contract. Ticket 0040 is landed.

This document fixes the immutable `ResolutionContentClosureV0` boundary,
including its construction laws, exact v0 limits, canonical manifest and
identity. It is a documentation-only contract. No production closure type,
loader, parser, manifest deserializer, CAS integration or resolver rewiring is
implemented by Ticket 0040.

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
| retained object bytes across one closure | 536,870,912 (512 MiB) |

Entry-count checks apply to supplied entries before exact-duplicate collapse.
This prevents unlimited duplicate-input work. Individual key-field and object
limits likewise apply to supplied entries. The retained-total cap is evaluated
after exact same-key/same-byte duplicate collapse; byte-identical objects under
different keys each contribute their retained length.

All length conversion and addition use checked arithmetic conceptually.
Overflow fails closed as `LengthOverflow`.

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

Likewise, a foreign-bundle `witness_root` may disagree with
`content_sha256`. Later profile-specific verification, not closure
construction, classifies that mismatch.

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
zero except the value zero. Ticket 0040 implements no parser or manifest
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

The closure identity value is:

```text
lowercase_hex(
  SHA-256(
    exact canonical manifest bytes under
    magpie-resolution-content-closure-json-v0
  )
)
```

The conceptual typed identity binds at least:

```text
schema = magpie-resolution-content-closure-v0
canonicalization_profile = magpie-resolution-content-closure-json-v0
digest_algorithm = sha256
manifest_sha256 = the closure identity value
```

The manifest does not contain its own digest. There is no alias, negotiation
table, runtime-selected profile, generic registry or implicit latest version.
A future incompatible change requires a new profile or schema identity.

The closure identity is an exact commitment to one finite keyed availability
universe. It is not a signature, trust root, provenance receipt, archive
guarantee, proof that an external store retains the bytes, proof that object
keys match object bytes, proof that bundles are canonical, proof that anchors
occurred, proof of origin or proof of truth.

A closure identity alone cannot reconstruct the closure. The manifest contains
only object digests and lengths, not raw object bytes.

## 13. Lookup semantics

The future closure API permits only exact lookup:

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

Ticket 0040 does not alter their signatures or behavior. It does not claim to
rewire either method. Once bytes are supplied, the verifier's existing parsing,
schema, canonical equality, selector comparison, root, replay occurrence,
artifact and receipt semantics remain unchanged. Lookup success establishes no
successful verifier context.

## 15. Construction outcomes

The future construction surface must preserve this closed conceptual
vocabulary. Exact Rust spelling may be pinned by the implementation ticket, but
these distinctions cannot be collapsed:

```text
ArtifactEntryLimitExceeded
ForeignBundleEntryLimitExceeded
TotalEntryLimitExceeded
KeyFieldTooLarge
ArtifactObjectTooLarge
ForeignBundleObjectTooLarge
TotalRetainedBytesExceeded
LengthOverflow
ConflictingArtifactObject
ConflictingForeignBundleObject
Constructed
```

Exact same-key/same-byte duplicates collapse idempotently and are not failures.
There is no generic `InvalidClosure`, arbitrary primary string, `Other` variant
or permissive fallback.

When several failures are present, the future implementation must use fixed
category precedence and deterministic key ordering, never caller insertion
order. The implementation ticket must pin the complete first-failure algorithm
before code lands. This contract does not invent that algorithm prematurely.

## 16. Normative vectors

The vectors use the committed files in
`fixtures/artifact-provenance-v0/`. Two independently structured temporary
Python calculations read those bytes, recomputed every object length and
SHA-256, sorted the exact key tuples and independently encoded the manifest.
Both produced the same results.

For every vector, the first byte is `0x7b` (`{`), the final byte is `0x7d`
(`}`), no UTF-8 BOM is present and there is no trailing newline.

| vector | artifact count | foreign-bundle count | retained object bytes | manifest bytes | manifest SHA-256 / closure identity |
| --- | ---: | ---: | ---: | ---: | --- |
| empty | 0 | 0 | 0 | 99 | `95446014b15ce3e3cb63784da4898defad484c34ff29ddd09d662bd7a2b58092` |
| acquisition | 1 | 1 | 306 | 663 | `2253d66f473c8b0f71a47f308ef4c0dffbb6f56e522b58c54bfd5519f958a224` |
| complete existing fixture | 3 | 2 | 692 | 1,434 | `2e0f59ee647407c6e66ae7e05913788b92b0b4f85c739f4ed3bc231511a30a84` |

### 16.1 Empty closure

Exact canonical manifest bytes:

```json
{"schema":"magpie-resolution-content-closure-v0","artifact_objects":[],"foreign_bundle_objects":[]}
```

### 16.2 Acquisition closure

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

### 16.3 Complete existing fixture closure

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

## 17. Hostile cases

| hostile case | required result |
| --- | --- |
| The same five objects are supplied in reverse order. | The same manifest bytes and closure identity result. |
| One artifact entry is supplied three times with identical bytes. | One object is retained and the closure identity is unchanged; supplied-entry limits still count all three. |
| One artifact key is supplied with two different byte strings. | `ConflictingArtifactObject`; no closure is constructed. |
| One bundle key is supplied with two different byte strings. | `ConflictingForeignBundleObject`; no closure is constructed. |
| The same bytes are supplied under two different artifact keys. | Two entries are retained; no identity, provenance or corroboration relationship is inferred. |
| An artifact key digest disagrees with the supplied bytes. | The closure may construct and records both values; a later verifier may report digest mismatch. |
| A bundle `witness_root` disagrees with the supplied bytes. | The closure may construct; later profile-specific verification reports root mismatch. |
| One object is removed. | A different closure identity results. |
| One object byte changes. | Its `content_sha256` and the closure identity change. |
| An object is added to external CAS after closure construction. | The existing closure and its resolution remain unchanged. |
| An object is deleted from external CAS after closure construction. | The existing closure and its resolution remain unchanged. |
| An object is absent from `M1` and present in `M2`. | These are two explicit resolution inputs, not ambient mutation. |
| A closure manifest is copied or serialized. | It is audit material only and recreates neither raw bytes nor authority. |
| A caller supplies a closure identity. | The value is not accepted as trusted construction or as a substitute for key/byte inputs. |
| One wire report is stored under many URLs. | URL multiplicity is outside closure identity and creates no corroboration. |

## 18. Authority boundary

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

## 19. Implementation sequence

The explicit next sequence is:

```text
ResolutionContentClosureV0 contract
-> closure implementation and hostile tests
-> exact origin-binding bundle contract
-> origin-binding verifier
-> standing-inert origin-admission audit
-> admitted-contribution audit
-> policy-v3 contract
-> conservative aggregation
```

Every stage before policy-v3 remains standing-inert. The immediate next slice
is closure implementation and hostile tests only. Its ticket must pin the
complete deterministic construction first-failure algorithm before code lands.

## 20. Frozen surfaces and non-goals

Ticket 0040 does not change or reinterpret `magpie-core-v1`, L0 payloads,
`SegmentAnchored`, Deadbolt occurrence semantics,
`ArtifactProvenanceAnchorSelectorV0`, `StandingReplaySnapshot`, artifact-
provenance verifier behavior, policies v0/v1/v2, existing snapshot or
resolution bytes, fixtures, golden vectors, release metadata, Cargo manifests,
`Cargo.lock`, CI or the `v0.1.0` tag/release.

It adds no Rust, tests, dependencies, fixtures, manifest parser, closure
implementation, loader, writer, MCP surface, downloader, crawler, CAS,
database, filesystem/network access, callback, plugin, L0 change,
origin-binding schema, origin-admission policy, contribution audit, policy v3,
aggregation, refutation, contradiction debt, invalidation, supersession,
currentness or production-readiness claim.
