# Ticket 0041: Resolution content closure v0 implementation

## 1. Exact starting point and worker authority

This implementation starts exactly from merge commit:

    f43615d49c23a723adb7950bb9d1a4349a760382

That commit is the merge of PR #52 and lands Ticket 0040. The implementation
branch is exactly:

    agent/resolution-content-closure-v0-implementation

The suggested future human commit and PR title are both:

    claims: implement resolution content closure v0

AGENTS.md is binding. Codex may edit and validate this ticket but may not
commit, push, open a PR, mark readiness, merge or enable auto-merge. The human
retains sole commit, publication, readiness and merge authority.

## 2. Goal

Implement the first production, standing-inert ResolutionContentClosureV0:

    explicit finite borrowed key/byte inputs
    -> bounded deterministic validation
    -> exact duplicate collapse or deterministic conflict
    -> immutable owned keyed objects
    -> purpose-built canonical manifest
    -> exact four-field typed closure identity
    -> exact read-only lookup

The deterministic law remains:

    verified log prefix H
    + explicit policy identities P
    + immutable resolution content closure M
    -> deterministic derived result

    same H + same P + same M
    -> byte-identical result

The closure freezes untrusted availability only. It creates no provenance,
origin, admission, support, aggregation, standing or writer authority.

## 3. Relationship to Tickets 0040 and 0039

Ticket 0040 ratified the closure contract, exact keys, limits, manifest,
identity and vectors. Ticket 0041 pins the implementation-time Rust surface
and complete first-failure algorithm before code.

Ticket 0039 implemented the separate standing-inert artifact-provenance
verifier. Its existing snapshot methods continue to receive explicit optional
byte slices. Ticket 0041 does not rewire, invoke or modify either resolver and
does not change their thirteen-stage behavior.

## 4. Exact changed paths

Exactly:

    README.md
    docs/design/artifact-provenance-origin-admission.md
    docs/design/resolution-content-closure-v0.md
    tickets/0041-resolution-content-closure-v0-implementation.md
    crates/magpie-claims/src/lib.rs
    crates/magpie-claims/src/resolution_content_closure.rs
    crates/magpie-claims/tests/resolution_content_closure.rs

No manifest, lockfile, fixture, tool, release, historical ticket, verifier,
replay, Deadbolt-context, standing, policy, tour, golden-vector or CI path may
change.

## 5. Implementation-time LengthOverflow reachability correction

Ticket 0040 historically used the conceptual term LengthOverflow. The Ticket
0041 reachability audit found no ordinary v0 production input capable of
reaching an independent result after earlier checks:

    artifact occurrences <= 1,024
    foreign-bundle occurrences <= 1,024
    artifact object bytes <= 67,108,864
    foreign-bundle object bytes <= 1,048,576

The largest structurally possible sum after those checks is approximately
65 GiB and fits in u64. A successful closure is further limited to
536,870,912 supplied bytes.

Therefore:

- checked conversion and checked addition remain mandatory;
- impossible conversion or addition failure fails closed as
  TotalSuppliedBytesExceeded with a saturated u64 audit value;
- no independent length-overflow failure exists in the current v0 vocabulary;
- Ticket 0040 remains unchanged as historical contract evidence; and
- every public Ticket 0041 failure has an ordinary production-input witness.

The conceptual Constructed outcome maps to Ok(ResolutionContentClosureV0); it
is not an error variant.

## 6. Exact constants

    pub const RESOLUTION_CONTENT_CLOSURE_SCHEMA_V0: &str =
        "magpie-resolution-content-closure-v0";
    pub const RESOLUTION_CONTENT_CLOSURE_CANONICALIZATION_PROFILE_V0: &str =
        "magpie-resolution-content-closure-json-v0";
    pub const RESOLUTION_CONTENT_CLOSURE_DIGEST_ALGORITHM_V0: &str =
        "sha256";
    pub const MAX_RESOLUTION_CONTENT_CLOSURE_ARTIFACT_ENTRIES_V0: usize =
        1_024;
    pub const MAX_RESOLUTION_CONTENT_CLOSURE_FOREIGN_BUNDLE_ENTRIES_V0: usize =
        1_024;
    pub const MAX_RESOLUTION_CONTENT_CLOSURE_TOTAL_ENTRIES_V0: usize =
        2_048;
    pub const MAX_RESOLUTION_CONTENT_CLOSURE_KEY_FIELD_BYTES_V0: usize =
        1_024;
    pub const MAX_RESOLUTION_CONTENT_CLOSURE_ARTIFACT_OBJECT_BYTES_V0: usize =
        67_108_864;
    pub const MAX_RESOLUTION_CONTENT_CLOSURE_FOREIGN_BUNDLE_OBJECT_BYTES_V0: usize =
        1_048_576;
    pub const MAX_RESOLUTION_CONTENT_CLOSURE_SUPPLIED_OBJECT_BYTES_V0: u64 =
        536_870_912;

There is no alias, registry, negotiation API, runtime algorithm selection,
extension map, latest profile or environment override.

## 7. Exact public key and borrowed inputs

ResolutionArtifactObjectKeyV0 privately stores, in order:

    algorithm: String
    digest: String

It derives Clone, Debug, PartialEq, Eq, PartialOrd, Ord and Serialize, but not
Deserialize. Its public constructor is infallible and applies no semantic
validation. Read-only getters expose both exact strings. The key is untrusted
lookup material, not proof that bytes match either string.

ResolutionArtifactObjectInputV0<'a> stores:

    key: ResolutionArtifactObjectKeyV0
    bytes: &'a [u8]

ResolutionForeignBundleObjectInputV0<'a> stores:

    selector: ArtifactProvenanceAnchorSelectorV0
    bytes: &'a [u8]

Both have infallible constructors and read-only key/selector, bytes and byte
length getters. Neither implements Deserialize or implies authority. Their
custom Debug output includes the key or selector and byte length but never raw
payload bytes. Borrowed input deliberately permits repeated references to one
bounded allocation.

## 8. Exact key-field and construction-error enums

ResolutionContentClosureKeyFieldV0 derives Clone, Copy, Debug, PartialEq, Eq,
PartialOrd, Ord and Serialize, but not Deserialize. Its declaration and
failure-precedence order is:

    ArtifactAlgorithm
    ArtifactDigest
    BundleKind
    WitnessRoot
    WitnessAlgorithm
    CanonicalizationProfile
    RunId

It serializes with snake-case names.

ResolutionContentClosureConstructionErrorV0 derives Clone, Debug, PartialEq,
Eq and Serialize, but not Deserialize. It uses:

    #[serde(
        tag = "outcome",
        content = "details",
        rename_all = "snake_case"
    )]

It contains exactly:

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

No other variant is permitted. Error values are audit output only.

## 9. Complete first-failure algorithm

The category precedence is exact:

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

Caller insertion order never selects the result.

### Stage 1: total entry count

Count every occurrence before collapse. If the checked total exceeds 2,048,
return TotalEntryLimitExceeded. This precedes class limits and later defects.
Impossible count conversion/addition uses saturated u64 audit values.

### Stage 2: artifact entry count

If total passed but artifact occurrences exceed 1,024, return
ArtifactEntryLimitExceeded.

### Stage 3: foreign-bundle entry count

If earlier counts passed but foreign-bundle occurrences exceed 1,024, return
ForeignBundleEntryLimitExceeded.

### Stage 4: key-field byte limits

Inspect the seven categories in enum declaration order. For the first category
with any string longer than 1,024 UTF-8 bytes, return one aggregate
KeyFieldTooLarge recording the category, offending occurrence count, maximum
observed length and exact maximum. Do not sort unbounded hostile strings or
choose an entry by caller order. Apply no semantic validation, normalization
or case folding.

### Stage 5: artifact-object size

For entries longer than 67,108,864 bytes, choose the lexicographically
smallest exact artifact key by algorithm then digest. Report the maximum
oversized length observed for that selected key.

### Stage 6: foreign-bundle-object size

For entries longer than 1,048,576 bytes, choose the lexicographically smallest
exact selector by bundle_kind, witness_root, witness_algorithm,
canonicalization_profile and run_id. Report the maximum oversized length
observed for that selected selector.

### Stage 7: total supplied bytes

Checked-sum every supplied occurrence before collapse. Every duplicate
contributes its full length. Exactly 536,870,912 may proceed;
536,870,913 fails. Impossible conversion or arithmetic returns the same
failure with a saturated audit value.

### Stage 8: artifact conflicts

Group by exact artifact key. All equal byte slices collapse idempotently. Any
difference conflicts. If several keys conflict, return the lexicographically
smallest exact key. Exact equality is byte equality, not length or hash.

### Stage 9: foreign-bundle conflicts

Apply the same rule to the exact five-field selector. Artifact conflict always
precedes bundle conflict.

### Stage 10: construct

Only after all failure stages pass, copy one byte vector per exact key into
private BTreeMap storage. Calculate retained bytes and require:

    retained object bytes
    <= supplied object bytes
    <= 536,870,912

Duplicate collapse cannot add bytes. There is no second retained-memory limit.

## 10. Exact constructor and immutable closure

The required associated function is:

    pub fn construct<'a>(
        artifact_entries: &[ResolutionArtifactObjectInputV0<'a>],
        foreign_bundle_entries: &[ResolutionForeignBundleObjectInputV0<'a>],
    ) -> Result<
        ResolutionContentClosureV0,
        ResolutionContentClosureConstructionErrorV0,
    >

It mutates no caller data, performs no ambient lookup, copies only retained
objects after validation, caches no global state, invokes no replay or standing
resolver, and invokes no artifact-provenance verifier.

ResolutionContentClosureV0 privately owns:

    BTreeMap<ResolutionArtifactObjectKeyV0, Vec<u8>>
    BTreeMap<ArtifactProvenanceAnchorSelectorV0, Vec<u8>>
    canonical manifest bytes
    ResolutionContentClosureIdentityV0
    supplied object byte total
    retained object byte total

It implements neither Serialize, Deserialize nor Default. It exposes no field,
mutable accessor, insertion method, iterator or constructor from maps,
manifest, identity, serialized material, storage, replay or callbacks.

Its custom Debug may show identity, both counts and both totals but no raw
payload.

## 11. Exact typed identity

ResolutionContentClosureIdentityV0 privately stores exactly, in order:

    schema
    canonicalization_profile
    digest_algorithm
    manifest_sha256

It derives Clone, Debug, PartialEq, Eq, PartialOrd, Ord and Serialize, but not
Deserialize. It has no public constructor. Read-only getters expose all four
fields. The first three values are the exact constants and manifest_sha256 is
exactly 64 lowercase hexadecimal characters produced by reviewed construction.

Identity equality is exact equality of all four fields. There is no second hash
over an identity serialization. Identity-shaped serialized material is audit
material and cannot construct a closure.

## 12. Exact getters and lookup

Read-only methods expose:

- identity;
- canonical_manifest_bytes;
- artifact_count;
- foreign_bundle_count;
- is_empty;
- supplied_object_bytes;
- retained_object_bytes;
- artifact bytes by exact ResolutionArtifactObjectKeyV0; and
- bundle bytes by exact ArtifactProvenanceAnchorSelectorV0.

Both lookups return Option<&[u8]>. There is no partial, prefix, root-only,
filename, URL, alias, case-insensitive, fuzzy, content-search, fallback or
hydrating lookup.

## 13. Purpose-built canonical encoder

The private encoder lives only in resolution_content_closure.rs. It is
fixed-schema and intentionally separate from the artifact-provenance encoder.
It uses no arbitrary JSON value map and creates no generic canonical-JSON
authority surface.

Top-level field order:

    {"schema":"magpie-resolution-content-closure-v0",
     "artifact_objects":[],
     "foreign_bundle_objects":[]}

The displayed line breaks are explanatory; canonical bytes contain none.

Artifact entry order:

    key { algorithm, digest }
    byte_length
    content_sha256

Foreign-bundle entry order:

    key {
      bundle_kind,
      witness_root,
      witness_algorithm,
      canonicalization_profile,
      run_id
    }
    byte_length
    content_sha256

Raw objects are omitted. content_sha256 is lowercase SHA-256 over exact
retained bytes. For a bundle it is not a generic witness root and construction
does not invoke or emulate a profile-specific verifier.

Artifacts sort by exact UTF-8 bytes over algorithm then digest. Bundles sort by
the exact five-field selector tuple. The artifact array precedes the bundle
array.

Strings emit quotation and reverse-solidus escapes; short escapes for
backspace, tab, newline, form feed and carriage return; lowercase \u00xx for
other U+0000-U+001F controls; and direct UTF-8 for every other scalar. There is
no normalization, escaped solidus, unnecessary escape, whitespace, BOM or
trailing newline. Unsigned integers use shortest base-10 form.

## 14. Identity derivation

    manifest_sha256 =
      lowercase_hex(SHA-256(exact canonical manifest bytes))

The successful constructor creates:

    ResolutionContentClosureIdentityV0 {
        schema: "magpie-resolution-content-closure-v0",
        canonicalization_profile:
            "magpie-resolution-content-closure-json-v0",
        digest_algorithm: "sha256",
        manifest_sha256: computed digest,
    }

The bare digest is only one component, not the typed identity.

## 15. Normative vectors

Tests use include_bytes! over the existing committed fixture files and pin
complete literal expected manifest bytes, never production-generated expected
bytes.

| vector | artifacts | bundles | retained bytes | manifest bytes | manifest SHA-256 |
| --- | ---: | ---: | ---: | ---: | --- |
| empty | 0 | 0 | 0 | 99 | 95446014b15ce3e3cb63784da4898defad484c34ff29ddd09d662bd7a2b58092 |
| acquisition | 1 | 1 | 306 | 663 | 2253d66f473c8b0f71a47f308ef4c0dffbb6f56e522b58c54bfd5519f958a224 |
| complete five-object closure | 3 | 2 | 692 | 1,434 | 2e0f59ee647407c6e66ae7e05913788b92b0b4f85c739f4ed3bc231511a30a84 |

Every manifest begins with 0x7b, ends with 0x7d, has no BOM and has no trailing
newline. Every vector asserts all four identity fields.

## 16. Complete hostile-test matrix

The integration suite must cover:

- total, artifact and bundle count witnesses and total-count precedence;
- every key-field category, inclusive/exclusive byte boundaries, aggregate
  counts/maxima, category precedence, Unicode byte lengths and reversed input;
- inclusive/exclusive artifact and bundle object limits, smallest offending
  key selection, selected-key maximum and reversed order;
- exact 512 MiB supplied duplicate boundary using repeated references to one
  1 MiB slice, 513 MiB failure, and total-byte precedence over both conflicts;
- same-reference and distinct-allocation exact duplicates;
- artifact and bundle conflicts, smallest conflicting keys, and artifact
  conflict precedence;
- same bytes under different artifact keys and bundle selectors;
- supplied/retained accounting and the invariant on every successful helper;
- forward, reverse and differently interleaved input order;
- caller-buffer independence after input borrows end;
- exact positive and absent lookup and one-field mismatch;
- repeated lookup immutability and absence of partial lookup APIs;
- every canonical string escape, direct Unicode, no normalization, no escaped
  solidus, no whitespace, no BOM and no trailing newline;
- mismatched artifact digest, mismatched bundle root and unusual witness
  algorithm remaining constructible without verification;
- all three complete normative manifests, lengths and identities;
- typed-identity four-field equality and identity-shaped material remaining
  non-authoritative;
- closure and input Debug omitting a distinctive raw-byte sentinel; and
- snapshot, policy-v0/v1/v2 and existing verifier behavior unchanged.

Every successful closure test asserts retained <= supplied <= 536,870,912.
Large allocations are sequential and dropped promptly.

## 17. Production witnesses for every public error

| error | ordinary witness |
| --- | --- |
| TotalEntryLimitExceeded | 1,024 artifact plus 1,025 bundle occurrences |
| ArtifactEntryLimitExceeded | 1,025 artifact occurrences only |
| ForeignBundleEntryLimitExceeded | 1,025 bundle occurrences only |
| KeyFieldTooLarge | a 1,025-byte value in each of the seven field categories |
| ArtifactObjectTooLarge | one 67,108,865-byte artifact slice |
| ForeignBundleObjectTooLarge | one 1,048,577-byte bundle slice |
| TotalSuppliedBytesExceeded | 513 borrowed occurrences of one 1 MiB bundle |
| ConflictingArtifactObject | one artifact key with two differing slices |
| ConflictingForeignBundleObject | one selector with two differing slices |

There is no public failure without an ordinary production witness.

## 18. Module-level authority examples and source audits

Compile-fail documentation proves:

1. identity struct literals are impossible;
2. closure struct literals are impossible;
3. identity deserialization is unsupported;
4. closure deserialization is unsupported;
5. no identity-only substitute constructor exists; and
6. no mutable insertion method exists.

The source audit must find no filesystem, environment, process, network, log
reader/writer, standing/policy, Deadbolt index, artifact receipt/resolver or
append surface. The only existing provenance type imported by the module is
ArtifactProvenanceAnchorSelectorV0.

The stale-outcome audit must find no normative independent length-overflow or
retained-total result. This ticket may name rejected historical terminology
only to document its removal.

## 19. Validation requirements

Run and report actual results only:

    git diff --check
    cargo fmt --all --check
    cargo clippy --workspace --all-targets --locked -- -D warnings
    cargo test -p magpie-claims --test resolution_content_closure --locked
    cargo test --doc -p magpie-claims --locked
    cargo test --workspace --locked
    cargo run --locked --example tour -p magpie-claims
    python tools/check_release_metadata.py
    cargo package -p magpie-log --locked
    python tools/verify_chain.py crates/magpie-log/testdata/golden-v1.jsonl ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c
    python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737

Also run both source audits, the exact changed-path and prohibited-path audits,
two independently structured temporary Python calculations over committed
fixture bytes, final diff/status/stat checks and full diff review.

If release metadata or package verification refuses solely because the seven
intended paths are uncommitted, report the exact refusal and leave it as a
human post-commit publication gate. Do not commit or use --allow-dirty.

## 20. Frozen surfaces, reviewer checklist and exact next slice

Frozen:

- magpie-core-v1, L0 payloads and SegmentAnchored semantics;
- policies v0, v1 and v2 and all snapshot/resolution bytes;
- ArtifactProvenanceAnchorSelectorV0 and the artifact verifier;
- Cargo.toml, Cargo.lock, dependencies and package metadata;
- fixtures, golden vectors, tools, release documents and CI;
- historical Tickets 0037-0040; and
- tour, writer and all admission/aggregation surfaces.

Reviewers must confirm:

- exactly seven paths changed;
- the nine errors and ten-stage precedence are exact and order-independent;
- every error has a production witness and no unreachable outcome remains;
- key fields are bounded before hostile-key sorting;
- duplicate collapse and accounting are pre/post collapse as specified;
- raw bytes appear in neither closure nor input Debug output;
- no Deserialize or caller-created identity authority exists;
- all three manifests and typed identities reproduce the pinned vectors;
- lookup is exact and immutable;
- no loader, ambient lookup, verifier wiring or standing effect exists; and
- every reported validation result actually ran.

The exact next slice is:

    exact origin-binding bundle contract

It may not be anticipated here with origin fields, admission rules, policy v3,
aggregation or writer behavior.
