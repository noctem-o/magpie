# Ticket 0040: Resolution content closure v0 contract

## Exact starting point

This documentation-only architecture ticket starts exactly from merge commit:

```text
73efd3d7bb1d53bf74e2498a8f6268ec9c2fb656
```

That commit is the merge of PR #51. The implementation branch is:

```text
agent/resolution-content-closure-v0-contract
```

The original architecture commit and PR title are both:

```text
docs: define resolution content closure v0
```

PR #52 was opened as a draft. A human later intentionally marked it ready for
review. This review remediation preserves that ready-for-review state and adds
one follow-up commit:

```text
docs: harden closure resource and identity bounds
```

## Goal

Ratify the finite, immutable availability universe `M` used by one
deterministic resolution:

```text
verified log prefix H
+ explicit policy identities P
+ immutable resolution content closure M
-> deterministic derived result

same H + same P + same M
-> byte-identical result
```

The closure freezes only:

> Which exact byte objects were supplied under which exact expected keys for
> one resolution attempt?

It prevents ambient filesystem, CAS, callback, mutable-store and network state
from silently changing a resolution. It grants no provenance, identity, truth,
origin admission, support, aggregation, settlement or writer authority.

## Exact changed paths

Exactly these four paths are in scope:

```text
README.md
docs/design/artifact-provenance-origin-admission.md
docs/design/resolution-content-closure-v0.md
tickets/0040-resolution-content-closure-v0.md
```

The design document and this ticket are new. Ticket 0039 and every other
historical ticket remain unchanged.

## Architectural laws

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

Closure construction does not compare expected artifact keys with actual object
digests, validate bundle roots, parse or canonicalize bundles, query replay,
admit origins or affect standing.

## Exact identities

```text
closure schema:
magpie-resolution-content-closure-v0

closure canonicalization profile:
magpie-resolution-content-closure-json-v0

closure digest algorithm:
sha256
```

There is no alias, negotiation table, runtime-selected profile, generic
registry or implicit latest version. An incompatible future change requires a
new schema or profile identity.

## Object classes and exact keys

The closure has exactly two disjoint classes.

Artifact object key, exact field order:

```text
algorithm
digest
```

This is an untrusted availability key, not proof of content identity.
Historical `EvidenceRegistered.content_hash` strings are not closure keys.
Filenames, URLs, paths, MIME types, titles, DOIs and database identifiers are
not artifact keys.

Foreign-bundle object key, exact `ArtifactProvenanceAnchorSelectorV0` field
order:

```text
bundle_kind
witness_root
witness_algorithm
canonicalization_profile
run_id
```

A bare root, run ID, bundle kind, filename, URL, CAS path or array index is not
an adequate key. Membership proves neither key validity nor replay occurrence.
The two class namespaces cannot conflict with one another.

## Explicit construction and immutability

Construction accepts only an explicitly supplied, already finite collection of
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

A future loader may collect bytes before construction but remains outside
deterministic resolution and outside Ticket 0040. Once constructed, the closure
is immutable. Adding, deleting or replacing an object creates a different
closure and a different resolution input. An empty closure is valid.

## Duplicate and conflict laws

```text
same class
+ same exact key
+ byte-identical content
-> idempotent duplicate
-> one retained object
-> one manifest entry
-> no added authority

same class
+ same exact key
+ different content
-> construction conflict
-> no closure

different exact keys
+ byte-identical content
-> two retained keyed objects
-> both keys visible in manifest
-> no inferred identity, provenance or corroboration
```

First-write-wins, last-write-wins, lexical-minimum bytes, newest input and
caller ordering are forbidden.

## Exact v0 resource limits

```text
maximum supplied artifact entries:
1024

maximum supplied foreign-bundle entries:
1024

maximum total supplied entries:
2048

maximum UTF-8 bytes in any individual key field:
1024

maximum bytes in one artifact object:
67,108,864

maximum bytes in one foreign-bundle object:
1,048,576

maximum supplied object bytes across one closure:
536,870,912

maximum retained object bytes across one closure:
536,870,912
```

These are the eight exact v0 resource maxima.

Entry counts are checked before exact-duplicate collapse. Total supplied
object bytes are the checked sum of every supplied artifact-entry and foreign-
bundle-entry byte length before collapse. Three same-key/same-byte occurrences
count three times; identical bytes supplied under several keys count once per
supplied entry; an empty closure supplies zero bytes. Entries later classified
as conflicting still contribute their full lengths according to the future
fixed first-failure algorithm; caller order cannot exempt an entry or select a
failure. Entry-count and total-supplied-byte checks jointly bound pre-collapse
construction work: counts cap structural multiplicity and the byte cap bounds
object payload presented before duplicates can reduce retention.

Total retained object bytes are the checked sum after exact same-key/same-byte
duplicate collapse; byte-identical objects retained under different keys count
separately. The supplied-total and retained-total caps deliberately share the
same v0 value but govern different stages. Individual entry limits apply before
collapse. All conceptual conversion and addition use checked arithmetic and
overflow fails closed.

The closure-level 1 MiB foreign-bundle limit is an outer bound. Existing
profile-specific limits remain stricter where specified. The current
artifact-provenance verifier retains its exact 16,384-byte bundle cap.

These are exact v0 limits, not universal permanent limits.

## Canonical manifest schema

The canonical manifest is generated only from a successfully constructed
closure. It is purpose-built fixed-schema JSON, not arbitrary JSON
canonicalization.

Exact top-level shape and order:

```json
{
  "schema": "magpie-resolution-content-closure-v0",
  "artifact_objects": [],
  "foreign_bundle_objects": []
}
```

Exact artifact-entry shape and order:

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

Exact foreign-bundle-entry shape and order:

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

`byte_length` is the exact object length as a checked `u64` decimal integer.
`content_sha256` is lowercase SHA-256 over the exact supplied object bytes. Raw
object bytes are not embedded. Actual object length and digest remain recorded
even when an expected key disagrees.

For a foreign-bundle object, `content_sha256` is always closure-profile
SHA-256 over the exact supplied bytes, regardless of the selector's
`witness_algorithm`. It is not a generic computed witness root. A selected
profile-specific verifier independently interprets the bytes, computes its
root under its reviewed algorithm and canonicalization rules, and compares
that result with `witness_root`.

The current artifact-provenance v0 verifier uses SHA-256 over exact accepted
canonical bundle bytes, so only after that profile's canonical acceptance does
its computed root equal `content_sha256`. Closure construction still performs
no comparison. Future profiles using another supported algorithm or transformed
root construction have no implied equality with `content_sha256`.

## Canonical encoding and ordering

Artifact entries sort bytewise over exact UTF-8 field bytes by:

```text
algorithm, digest
```

Foreign-bundle entries sort bytewise by:

```text
bundle_kind, witness_root, witness_algorithm,
canonicalization_profile, run_id
```

The artifact array precedes the foreign-bundle array. There is no locale, case
folding, Unicode normalization, path normalization, URL normalization or
semantic comparison. Input and duplicate occurrence order do not affect bytes.

The separate `magpie-resolution-content-closure-json-v0` profile uses exactly:

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

Unsigned integers use shortest base-10 form with no leading zero except zero.
No parser or manifest deserializer is introduced.

## Closure identity

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
surface. The identity has exactly these four fields, with no aliases, optional
fields, extensions, metadata maps, negotiation values or runtime selector.

```text
manifest_sha256
= lowercase_hex(
    SHA-256(
      exact canonical manifest bytes under
      magpie-resolution-content-closure-json-v0
    )
  )

ResolutionContentClosureIdentityV0 equality
= exact equality of all four fields
```

The complete typed identity, not the bare 64-character `manifest_sha256`, is
the closure identity. The bare value is only its digest component. The
manifest contains neither its own digest nor the profile and algorithm
identity fields. No second hash over a serialized four-field identity is
introduced.

The identity is an exact commitment to one finite keyed availability universe.
It is not a signature, trust root, provenance receipt, archive guarantee,
external-store retention proof, key/content match, bundle verification, anchor
occurrence, origin proof or truth proof. It cannot reconstruct raw objects. A
caller-created four-field value remains untrusted audit material and is not
proof that reviewed construction occurred.

## Exact lookup semantics

```text
exact artifact key
-> exact retained artifact bytes or unavailable

exact five-field bundle key
-> exact retained bundle bytes or unavailable
```

No prefix, wildcard, fallback, nearest match, root-only, filename, URL, path,
content search, alias, case-insensitive or semantic lookup exists.

An object absent from `M1` remains unavailable for `M1`. A loader that later
supplies it constructs `M2`. The `M1` result remains fixed; an `M2` result is a
distinct resolution over a distinct closure.

## Existing verifier boundary

The closure is an outer availability boundary. It does not alter the existing
artifact-provenance verifier or its exact 13 stages.

A future orchestrator may:

```text
perform exact closure lookup
-> pass resulting optional byte slice
-> invoke existing snapshot-only verifier
```

Ticket 0040 does not rewire or change:

```text
resolve_artifact_acquisition_context_v0
resolve_artifact_derivation_context_v0
```

Those APIs continue to receive explicit optional byte slices. Once supplied,
existing parsing, selector comparison, root, replay, artifact and receipt
semantics remain unchanged. Closure lookup alone creates no verifier context.

## Closed conceptual construction outcomes

```text
ArtifactEntryLimitExceeded
ForeignBundleEntryLimitExceeded
TotalEntryLimitExceeded
KeyFieldTooLarge
ArtifactObjectTooLarge
ForeignBundleObjectTooLarge
TotalSuppliedBytesExceeded
TotalRetainedBytesExceeded
LengthOverflow
ConflictingArtifactObject
ConflictingForeignBundleObject
Constructed
```

Exact duplicates are idempotent, not failures. There is no generic
`InvalidClosure`, arbitrary error string, `Other` branch or permissive fallback.

A future implementation must use fixed category precedence and deterministic
key ordering when several failures exist. Caller insertion order cannot choose
the result. The implementation ticket must pin the complete first-failure
algorithm before code lands.

## Normative vectors

Two independently structured temporary Python calculations read the committed
`fixtures/artifact-provenance-v0/` bytes, recomputed every object length and
SHA-256, sorted by the exact key tuples and independently emitted the fixed
manifest. They agreed exactly:

| vector | artifact count | bundle count | retained bytes | manifest bytes | manifest SHA-256 |
| --- | ---: | ---: | ---: | ---: | --- |
| empty | 0 | 0 | 0 | 99 | `95446014b15ce3e3cb63784da4898defad484c34ff29ddd09d662bd7a2b58092` |
| acquisition | 1 | 1 | 306 | 663 | `2253d66f473c8b0f71a47f308ef4c0dffbb6f56e522b58c54bfd5519f958a224` |
| complete existing fixture | 3 | 2 | 692 | 1,434 | `2e0f59ee647407c6e66ae7e05913788b92b0b4f85c739f4ed3bc231511a30a84` |

Each listed hash is the `manifest_sha256` component. The following table gives
the complete typed identity for every vector; its columns preserve the
normative field order:

| vector | schema | canonicalization_profile | digest_algorithm | manifest_sha256 |
| --- | --- | --- | --- | --- |
| empty | `magpie-resolution-content-closure-v0` | `magpie-resolution-content-closure-json-v0` | `sha256` | `95446014b15ce3e3cb63784da4898defad484c34ff29ddd09d662bd7a2b58092` |
| acquisition | `magpie-resolution-content-closure-v0` | `magpie-resolution-content-closure-json-v0` | `sha256` | `2253d66f473c8b0f71a47f308ef4c0dffbb6f56e522b58c54bfd5519f958a224` |
| complete existing fixture | `magpie-resolution-content-closure-v0` | `magpie-resolution-content-closure-json-v0` | `sha256` | `2e0f59ee647407c6e66ae7e05913788b92b0b4f85c739f4ed3bc231511a30a84` |

Every vector starts with `0x7b`, ends with `0x7d`, has no BOM and has no
trailing newline.

Exact empty manifest:

```json
{"schema":"magpie-resolution-content-closure-v0","artifact_objects":[],"foreign_bundle_objects":[]}
```

Exact acquisition manifest:

```json
{"schema":"magpie-resolution-content-closure-v0","artifact_objects":[{"key":{"algorithm":"sha256","digest":"51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076"},"byte_length":15,"content_sha256":"51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076"}],"foreign_bundle_objects":[{"key":{"bundle_kind":"magpie-artifact-acquisition-v0","witness_root":"4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e","witness_algorithm":"sha256","canonicalization_profile":"magpie-artifact-provenance-json-v0","run_id":"run-acq-0001"},"byte_length":291,"content_sha256":"4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e"}]}
```

The complete-vector sorted field table is normative in
[`resolution-content-closure-v0.md`](../docs/design/resolution-content-closure-v0.md).
Its five exact object facts are:

```text
artifact sha256|1921b918...faa4005 -> 6 bytes -> 1921b918...faa4005
artifact sha256|51bc0fc1...e8f8076 -> 15 bytes -> 51bc0fc1...e8f8076
artifact sha256|b6a98d9c...0b51060 -> 6 bytes -> b6a98d9c...0b51060
bundle acquisition five-field selector -> 291 bytes -> 4a5cdc95...ffb093e
bundle derivation five-field selector -> 374 bytes -> 472e8571...0bc76dd
```

The complete exact digests and key tuples appear in the normative design table;
the abbreviations above are descriptive only.

## Hostile cases

| case | exact contract result |
| --- | --- |
| Same five objects supplied in reverse order. | Same manifest and typed identity. |
| Same artifact entry supplied three times. | One retained object and same typed identity; all three entries and all three lengths count before collapse. |
| 1,024 identical 64 MiB artifact entries. | `TotalSuppliedBytesExceeded`; duplicate collapse cannot rescue construction. |
| Individually valid supplied objects total exactly 536,870,912 bytes. | May proceed past the inclusive supplied-total bound, subject to every other check. |
| The same supplied set plus one byte. | `TotalSuppliedBytesExceeded`. |
| Duplicates exceed 512 MiB supplied but would retain only 64 MiB. | `TotalSuppliedBytesExceeded`; post-collapse size does not rescue construction. |
| A test-only boundary or future profile keeps supplied bytes within its cap while distinct-key retention exceeds a lower retained cap. | `TotalRetainedBytesExceeded`. This is not reachable under the equal v0 caps because retained bytes cannot exceed supplied bytes; the outcome remains a distinct required stage. |
| Same artifact key with two different byte strings. | `ConflictingArtifactObject`; no closure. |
| Same bundle key with two different byte strings. | `ConflictingForeignBundleObject`; no closure. |
| Same bytes under two artifact keys. | Two retained entries; no inferred identity. |
| Artifact key digest disagrees with supplied bytes. | Closure may construct; later verifier may report digest mismatch. |
| Foreign-bundle selector names an unsupported witness algorithm. | Closure may construct and computes `content_sha256` using closure SHA-256; a later verifier rejects or classifies the unsupported profile. |
| Selector `witness_root` differs from `content_sha256` under current artifact-provenance v0 fields. | Closure may construct; the current verifier later reports `WitnessRootMismatch` when its independently computed SHA-256 root differs from the selector root. |
| A future profile computes a non-SHA-256 or transformed witness root. | `content_sha256` remains an available-object commitment only; no generic equality is inferred. |
| One object removed. | Different manifest SHA-256 component and typed identity. |
| One byte changed. | Different `content_sha256`, manifest SHA-256 component and typed identity. |
| Object added to external CAS after construction. | Existing closure unchanged. |
| Object deleted from external CAS after construction. | Existing closure unchanged. |
| Object absent from `M1`, present in `M2`. | Two explicit resolution inputs, not ambient mutation. |
| Closure manifest copied or serialized. | Audit material only; no raw-byte reconstruction or authority. |
| Same bare `manifest_sha256` with different schema/profile/algorithm fields. | Different four-field values are not equal typed identities; non-v0 constants are not valid v0 identities. This is semantic domain scoping, not a collision claim. |
| Caller supplies a well-shaped `ResolutionContentClosureIdentityV0`. | Not accepted as proof of trusted construction. |
| Caller copies `manifest_sha256` alone. | Digest component only, not the complete typed identity. |
| One wire report stored under many URLs. | URL multiplicity is outside closure identity and creates no corroboration. |

## Authority boundary and later origin work

The closure does not know and must not encode:

```text
ContributionIdentity
OriginComparisonNamespace
origin group
binding authority
origin-admission policy
corroboration component
polarity
aggregation lane
```

The sequence is exactly:

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

No origin binding or admission has landed through this ticket.

## Frozen surfaces and non-goals

No `Cargo.toml`, `Cargo.lock`, `.github/`, Rust, tests, fixtures, tools,
`docs/FORMAT.md`, release file, changelog, license, historical ticket, golden
vector or existing canonical fixture changes are permitted.

No policy v0/v1/v2, `StandingReplaySnapshot`,
`ArtifactProvenanceAnchorSelectorV0`, artifact-provenance verifier, Deadbolt
anchor identity/occurrence semantics, release metadata or `v0.1.0` tag/release
is modified or reinterpreted.

No dependency, L0 change, closure implementation, loader, downloader, crawler,
CAS, database, network, callback, plugin, writer, MCP surface, origin-binding
schema, origin-admission policy, contribution audit, policy v3 or aggregation
is introduced.

## Validation requirements

Before commit, run and report only observed results for:

```powershell
git diff --check
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo run --locked --example tour -p magpie-claims
python tools/check_release_metadata.py
cargo package -p magpie-log --locked
python tools/verify_chain.py crates/magpie-log/testdata/golden-v1.jsonl ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c
python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737
git status --short
git diff --stat
git diff --name-only
```

Also run the prohibited-path diff, authority-transposition audit, exact-path
audit and both independent inline vector calculations required by the ticket
prompt. Review every authority-audit match manually. Before commit, review the
entire diff from the exact base.

### Review-remediation validation record

On 2026-07-14, before the follow-up commit, the final remediation text passed:

- `git diff --check`;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets --locked -- -D warnings`;
- `cargo test --workspace --locked` with 256 passed and zero failed;
- `cargo run --locked --example tour -p magpie-claims`;
- `python tools/check_release_metadata.py`, with all three inventories
  consistent;
- `cargo package -p magpie-log --locked`, packaging and verifying 19 files;
- the independent golden-chain verifier, with all nine records `OK`;
- the portable Deadbolt chain verifier, with both records `OK`;
- the prohibited-path audit, with no output; and
- the focused stale-wording and authority/identity searches after manual
  review of every match.

Two independently structured inline Python calculations reread all five
committed artifact-provenance fixture objects. Both reproduced the exact object
lengths and SHA-256 values and the unchanged empty, acquisition and complete
manifest results: 99 / `95446014...b58092`, 663 /
`2253d66f...58a224`, and 1,434 / `2e0f59ee...30a84`. Every manifest began with
`0x7b`, ended with `0x7d`, had no BOM and had no trailing newline. The complete
digests remain normative in the table above; abbreviations here are only a
compact validation record.

The required clean-tree post-commit rerun remains a publication gate and is
reported in the PR body and final handoff; this pre-commit record does not
claim it occurred early.

## Reviewer checklist

- Confirm the branch starts exactly at
  `73efd3d7bb1d53bf74e2498a8f6268ec9c2fb656`.
- Confirm exactly the four scoped paths changed and no historical ticket did.
- Confirm the three schema/profile/digest identities are exact and versioned.
- Confirm there are exactly two disjoint untrusted object classes and key field
  order matches the landed selector.
- Confirm construction performs no lookup, replay, callback or hydration.
- Confirm counts and total supplied bytes occur before duplicate collapse, and
  retained bytes after it.
- Confirm all eight v0 resource maxima are exact.
- Confirm duplicate, conflict and same-bytes/different-key cases remain
  distinct and order-independent.
- Confirm expected key values and actual object digests remain distinct.
- Confirm canonical field order, UTF-8 bytewise ordering, string escapes,
  integer form, BOM absence and no trailing newline are exact.
- Confirm both independent calculations reproduce the three exact lengths,
  digests, endpoint bytes and BOM results.
- Confirm `ResolutionContentClosureIdentityV0` has exactly four fields in the
  pinned order, the vector hashes are only `manifest_sha256` components, and
  identity is commitment only that cannot reconstruct objects or grant trust.
- Confirm foreign-bundle `content_sha256` is always closure SHA-256 over exact
  supplied bytes and is not a generic profile-specific witness root.
- Confirm lookup is exact only and `M1` never changes ambiently.
- Confirm existing slice-based verifier APIs and 13-stage semantics are not
  claimed to change.
- Confirm the closed construction outcome vocabulary is complete and no
  caller-order first-failure rule is invented.
- Confirm origin-binding fields are absent and later origin/admission/
  aggregation layers remain future work.
- Confirm the PR remains open, ready for review by intentional human action,
  unmerged, and without auto-merge.

## Exact next slice

The next separately reviewed slice is only:

```text
implement ResolutionContentClosureV0 construction
+ pin complete deterministic first-failure precedence
+ add resource-bound, duplicate/conflict, ordering, identity and ambient-state
   hostile tests
```

It must not add an origin-binding schema, origin-admission policy, contribution
audit, policy v3, aggregation, loader, downloader, crawler, CAS, database,
network surface or writer authority.
