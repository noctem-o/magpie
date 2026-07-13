# Ticket 0039: Standing-inert artifact provenance verifier v0

## Goal

Implement the first strict Magpie-side parser, canonicalizer and verifier for
the exact acquisition/direct-derivation protocol ratified by Ticket 0038, plus
portable normative fixtures reproducing both pinned vectors byte-for-byte.

```text
untrusted selector
+ optional untrusted bundle bytes
+ optional untrusted artifact bytes
+ one verified replay snapshot
-> one closed deterministic audit outcome
```

This ticket implements verification only.
It grants no origin admission, support, aggregation, settlement,
writer authority, or standing effect.

## Exact starting contract

The implementation starts from merge commit
`e5e59295c4ce104ff647cd7f3488c8e72d358500`, which lands Ticket 0038 and
[`artifact-acquisition-derivation-bundle-v0.md`](../docs/design/artifact-acquisition-derivation-bundle-v0.md).
Every schema byte, identifier, field order, canonical string rule, root,
selector comparison, replay rule, artifact check and first-failure precedence
in that ratified contract remains exact.

The deterministic input remains explicit:

```text
one untrusted selector
+ already-bounded optional bundle/artifact byte slices
+ one StandingReplaySnapshot from one completely verified replay
```

There is no production `ResolutionContentClosureV0` in this ticket. The byte
slice options model the already-bounded result of lookup in one immutable
resolution input and do not authorize a file, network, callback, plugin, CAS or
second-store lookup.

## Public constants

The new module exports and `lib.rs` re-exports exactly:

```rust
pub const ARTIFACT_ACQUISITION_BUNDLE_KIND_V0: &str =
    "magpie-artifact-acquisition-v0";
pub const ARTIFACT_ACQUISITION_SCHEMA_V0: &str =
    "magpie-artifact-acquisition-v0";
pub const ARTIFACT_DERIVATION_BUNDLE_KIND_V0: &str =
    "magpie-artifact-derivation-v0";
pub const ARTIFACT_DERIVATION_SCHEMA_V0: &str =
    "magpie-artifact-derivation-v0";
pub const ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0: &str =
    "magpie-artifact-provenance-json-v0";
pub const ARTIFACT_PROVENANCE_VERIFIER_PROFILE_V0: &str =
    "magpie-artifact-provenance-verifier-v0";
pub const ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0: &str = "sha256";
pub const ARTIFACT_PROVENANCE_ARTIFACT_ALGORITHM_V0: &str = "sha256";
pub const MAX_ARTIFACT_PROVENANCE_BUNDLE_BYTES_V0: usize = 16_384;
```

There is no registry, alias table, negotiation API or latest-profile selector.

## Public selector

`ArtifactProvenanceAnchorSelectorV0` stores exactly `bundle_kind`,
`witness_root`, `witness_algorithm`, `canonicalization_profile` and `run_id` as
private strings. It derives `Clone`, `Debug`, `PartialEq`, `Eq`, `PartialOrd`,
`Ord` and `Serialize`, but not `Deserialize`.

Its public constructor is infallible and performs no validation. Read-only
getters expose each exact string. Hostile selectors remain untrusted comparison
and lookup material until the verifier classifies them under the fixed stage
order. No selector method returns a receipt or proves replay occurrence.

## Snapshot-only public API

Trusted construction is available only on `StandingReplaySnapshot`:

```rust
pub fn resolve_artifact_acquisition_context_v0(
    &self,
    selector: &ArtifactProvenanceAnchorSelectorV0,
    bundle_bytes: Option<&[u8]>,
    artifact_bytes: Option<&[u8]>,
) -> ArtifactProvenanceContextTraceV0
```

```rust
pub fn resolve_artifact_derivation_context_v0(
    &self,
    selector: &ArtifactProvenanceAnchorSelectorV0,
    bundle_bytes: Option<&[u8]>,
    parent_artifact_bytes: Option<&[u8]>,
    derived_artifact_bytes: Option<&[u8]>,
) -> ArtifactProvenanceContextTraceV0
```

There is no free raw-material resolver and no equivalent method on
`StandingView`, `DeadboltAnchorIndex` or a caller-created receipt. The methods
take `&self`, cache nothing, mutate nothing and consult only the snapshot's
private replay-derived anchor index.

## Closed outcome enum

`ArtifactProvenanceContextTraceV0` derives `Clone`, `Debug`, `PartialEq`, `Eq`
and `Serialize`, but not `Deserialize`. It is adjacently tagged with exact
snake-case outcomes and exposes deterministic `canonical_bytes`,
`matched_acquisition` and `matched_derivation` getters.

The closed primary vocabulary is exactly:

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
MatchedAcquisition(ArtifactAcquisitionReceiptV0)
MatchedDerivation(ArtifactDerivationReceiptV0)
```

Every attempt returns one primary trace. There is no Boolean result, arbitrary
primary error string, `Other` branch or `Option<Trace>`.

## Receipt authority boundary

`ArtifactAcquisitionReceiptV0` and `ArtifactDerivationReceiptV0` have private
fields, no public constructor and no `Deserialize`. They derive `Clone`,
`Debug`, `PartialEq`, `Eq` and `Serialize` and expose only read-only getters.

The acquisition receipt retains the verifier profile, full selector, schema,
acquisition ID/profile, artifact algorithm/digest, observed locator, bundle
length, computed root, artifact length, computed artifact SHA-256 and every
matching `DeadboltAnchorOccurrence`.

The derivation receipt retains the verifier profile, full selector, schema,
derivation ID/transformation profile, both artifact algorithms/digests, bundle
length, computed root, both byte lengths and computed SHA-256 values, and every
matching occurrence.

Module-level compile-fail examples prove:

1. acquisition receipt literals are impossible;
2. derivation receipt literals are impossible;
3. `StandingView` has no artifact resolver;
4. no free resolver accepts a caller-created `DeadboltAnchorIndex`; and
5. receipt deserialization is unsupported.

Cloned or serialized receipt material is audit output only. No resolver or
standing policy accepts it as authority.

## Duplicate-aware private parser

Acquisition and direct derivation use distinct private envelope and parsed
types, with a separate private nested artifact identity. No generic statement
list, metadata map or public parser API is exposed.

Custom serde visitors consume `serde_json` parser events without converting a
bundle to `serde_json::Value`. Every object tracks its encountered keys with a
deterministic set before value selection. A private recursive discard visitor
detects duplicates even inside unknown or wrong-type subtrees. Thus duplicates
at any depth remain observable and precede missing, wrong-type and unknown-field
classification without relying on map replacement or hash-map iteration.

Required field and semantic checks are hard-coded in canonical schema order.
Unknown-field state is considered only after every required field and nested
shape is present and correctly typed.

## Exact canonical encoder

The private encoder supports only the two fixed schemas. It emits their exact
top-level and nested field order, `:` and `,` separators, no whitespace and no
trailing newline. It is not RFC 8785/JCS and performs no arbitrary key sorting.

The string encoder emits `\"`, `\\`, `\b`, `\t`, `\n`, `\f`, `\r`, lowercase
`\u00xx` for other controls, and direct UTF-8 for every other Unicode scalar.
It performs no normalization, escaped-solidus substitution or unnecessary
escaping. Parsed logical content is re-encoded and compared with the complete
supplied bytes before hashing. Only exact canonical bytes are hashed.

Both normative fixture encodings reproduce the ratified 291-byte and 374-byte
vectors and their exact SHA-256 roots.

## Exact 13-stage precedence

1. Bundle lookup: absent bytes produce `BundleUnavailable`.
2. Raw byte limit: more than 16,384 bytes produces `BundleTooLarge`.
3. Raw parsing: `InvalidUtf8`, then `Utf8BomPresent`, then `InvalidJson`, then
   `TopLevelNotObject`.
4. Shape: any-depth `DuplicateKey`; then required missing/type/schema checks in
   canonical order; only after all required shapes, `UnknownField`.
5. Semantics: acquisition ID, profile, artifact algorithm/digest, locator,
   run ID; or derivation ID, profile, parent algorithm/digest, derived
   algorithm/digest, run ID.
6. Direct derivation only: equal parent and derived identities produce
   `ParentEqualsDerived`.
7. Complete supplied bytes must equal purpose-built canonical re-encoding or
   `NonCanonicalEncoding` results.
8. Selector fields: `BundleKindMismatch`, `WitnessAlgorithmMismatch`,
   `CanonicalizationProfileMismatch`, then `RunIdMismatch`.
9. Lowercase SHA-256 over exact canonical bytes must equal the selector root or
   `WitnessRootMismatch` results.
10. Exact five-field replay lookup occurs only now; zero occurrences produce
    `AnchorAbsent`.
11. Acquisition checks artifact availability. Derivation checks parent before
    derived availability.
12. Acquisition checks artifact digest. Derivation checks parent before derived
    digest.
13. One `MatchedAcquisition` or `MatchedDerivation` primary receipt results.

The decisive tail is:

```text
selector mismatches
-> root mismatch
-> replay absence
-> artifact availability
-> artifact digest
-> matched context
```

## Replay occurrence retention

Only after selector and root agreement does the verifier construct the existing
`DeadboltAnchorIdentity` and query `self.anchors().occurrences(&identity)`.
Zero matches are `AnchorAbsent`. One or more matches satisfy one existential
prerequisite. Every matching existing `DeadboltAnchorOccurrence` is cloned into
the receipt in deterministic sequence order.

Three repeated exact anchors therefore produce one matched primary trace and
three audit occurrences. The verifier selects neither first, last, newest nor
any single occurrence, and multiplicity creates no verification or standing
amplification.

## Portable fixture inventory

`fixtures/artifact-provenance-v0/` contains:

| path | exact bytes | SHA-256 |
| --- | ---: | --- |
| `acquisition-artifact.txt` | 15, final LF | `51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076` |
| `acquisition-bundle.json` | 291, no trailing newline | `4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e` |
| `derivation-parent.txt` | 6, final LF | `b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060` |
| `derivation-derived.txt` | 6, final LF | `1921b918b15842c7fdb115078e610263fac85f159c1d8e0ecec3d89a0faa4005` |
| `derivation-bundle.json` | 374, no trailing newline | `472e857153fe8c901b6c0d7d1b15bd876450b57b4743222a2fab91eb60bc76dd` |

The fixture README records exact selectors and independent Python
recalculations. Tests load every byte with `include_bytes!`, assert both roots,
all content hashes, exact LF behavior, absence of BOM and absence of portable
machine-marker hazards. No signed anchor log is checked in.

## Complete hostile test matrix

Every primary failure variant is reached by the integration suite. Coverage is
grouped as follows:

| boundary | hostile cases |
| --- | --- |
| lookup/size | missing bundle; exact 16,384-byte input proceeds; over-limit plus invalid UTF-8 is `BundleTooLarge` |
| UTF-8/JSON | invalid UTF-8; BOM; malformed escape; lone surrogate; invalid surrogate pair; second JSON value; valid non-object |
| duplicates | duplicate top-level key; duplicate nested artifact key; duplicate inside unknown subtree; duplicate plus missing |
| required shape | missing required field; missing plus unknown; wrong string/object type; schema wrong type; exact schema mismatch; unknown field |
| semantic acquisition | invalid acquisition ID; invalid locator; unsupported algorithm; invalid digest; invalid ID plus whitespace |
| semantic derivation | equal parent/derived identity |
| canonical equality | pretty whitespace; reordered fields; trailing newline; unnecessary escape; escaped solidus; uppercase hexadecimal escape digit; valid escaped surrogate pair |
| selector | wrong kind plus later hostility; wrong algorithm plus later hostility; wrong profile; wrong run ID |
| root/replay | uppercase/wrong root before replay; fully consistent absent anchor; absent anchor plus absent artifact |
| acquisition artifacts | missing artifact; digest mismatch; exact match |
| derivation artifacts | both absent; derived absent; both digests wrong; derived digest wrong; exact match |
| multiplicity | three acquisition anchors; three derivation anchors; one receipt and all ordered occurrences |
| determinism | repeated calls over the same snapshot/inputs yield equal traces and canonical bytes |
| authority | cloned/serialized receipts fail to substitute for an absent replay occurrence |

Compound first-failure checks pin:

```text
over-limit + invalid UTF-8 -> BundleTooLarge
duplicate + missing -> DuplicateKey
missing + unknown -> MissingField
schema wrong JSON type -> WrongJsonType before schema comparison
invalid identifier + noncanonical whitespace -> InvalidIdentifier
wrong kind + wrong algorithm + wrong root + absent anchor -> BundleKindMismatch
correct kind + wrong algorithm + wrong root + absent anchor -> WitnessAlgorithmMismatch
correct protocol fields + wrong root + absent anchor -> WitnessRootMismatch
fully consistent selector + absent anchor -> AnchorAbsent
anchor absent + artifact absent -> AnchorAbsent
parent absent + derived absent -> ParentArtifactUnavailable
parent digest wrong + derived digest wrong -> ParentArtifactDigestMismatch
```

No test depends on unordered map iteration or the process current directory.

## Standing-inertness proof

Library verifier code imports only the replay snapshot and exact Deadbolt anchor
types. It does not import standing policy modules, call a standing resolver,
append an event, create a claim/evidence/contribution, or mutate snapshot bytes.

The integration suite captures and compares before/after:

- `StandingReplaySnapshot::canonical_bytes`;
- policy-v0 standing output;
- policy-v1 standing output; and
- policy-v2 standing output.

Both matched resolvers leave every value unchanged and the test claim remains
`Conjectured`. No current policy consumes either receipt.

The source audit is:

```powershell
rg -n "resolved_standing|support_ceiling|support_context|policy_v3|origin.admission|LogWriter|append\(" crates/magpie-claims/src/artifact_provenance_verifier.rs
```

Expected result: no match.

## Files in scope

Exactly:

- `README.md`
- `docs/design/artifact-acquisition-derivation-bundle-v0.md`
- `tickets/0039-artifact-provenance-verifier-v0.md`
- `crates/magpie-claims/src/lib.rs`
- `crates/magpie-claims/src/artifact_provenance_verifier.rs`
- `crates/magpie-claims/tests/artifact_provenance_verifier.rs`
- `fixtures/artifact-provenance-v0/README.md`
- `fixtures/artifact-provenance-v0/acquisition-bundle.json`
- `fixtures/artifact-provenance-v0/acquisition-artifact.txt`
- `fixtures/artifact-provenance-v0/derivation-bundle.json`
- `fixtures/artifact-provenance-v0/derivation-parent.txt`
- `fixtures/artifact-provenance-v0/derivation-derived.txt`

## Frozen surfaces

No `Cargo.toml`, `Cargo.lock`, `docs/FORMAT.md`, `magpie-log`,
`deadbolt_context.rs`, `replay_snapshot.rs`, standing module/policy, existing
fixture/golden vector, CI workflow or release metadata changes. The existing
`DeadboltAnchorIdentity`, `DeadboltAnchorOccurrence`, `DeadboltAnchorIndex` and
`StandingReplaySnapshot` fields/canonical bytes are unchanged.

There is no dependency, L0 payload, format, golden chain, anchor-index,
snapshot-byte, standing-policy, release or CI surface change.

## Explicit non-goals

No final `ResolutionContentClosureV0` manifest; persistent CAS; filesystem,
network, callback or plugin lookup; downloader or crawler; generic bundle
registry; algorithm negotiation; arbitrary metadata; embedded signature;
origin-binding schema; origin-admission policy; admitted-contribution audit;
policy v3; aggregation; direct refutation; contradiction debt; invalidation;
supersession; currentness; `EpistemicGate`; writer API; MCP tool; L0 payload;
snapshot-field change; anchor-index change; dependency; production key; or
production-readiness claim.

## Validation commands

Run and report only actual results:

```powershell
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test -p magpie-claims --test artifact_provenance_verifier --locked
cargo test --doc -p magpie-claims --locked
cargo test --workspace --locked
cargo run --locked --example tour -p magpie-claims
python tools/check_release_metadata.py
cargo package -p magpie-log --locked
python tools/verify_chain.py crates/magpie-log/testdata/golden-v1.jsonl ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c
python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737
git diff --check
git status --short
git diff --stat
git diff --name-only
```

Also run two independently written temporary Python fixture calculations, the
standing-inertness source audit and the full protocol audit from the
implementation prompt. Temporary calculations are not committed.

## Reviewer checklist

- Confirm all twelve and only the twelve scoped paths changed.
- Confirm every public constant and profile identity is exact and no latest or
  negotiation surface exists.
- Confirm the selector is untrusted, infallibly constructed and cannot return a
  receipt.
- Confirm only `StandingReplaySnapshot` exposes the two resolvers.
- Confirm no `serde_json::Value` or silent map replacement loses duplicates.
- Confirm duplicates at any depth precede required shape and unknown fields.
- Confirm required shape and semantics follow canonical schema order.
- Confirm the encoder is fixed-schema, preserves direct Unicode scalar output,
  performs no normalization or arbitrary sorting and hashes only exact accepted
  canonical bytes.
- Confirm selector comparisons precede root, root precedes replay and replay
  precedes artifacts.
- Confirm exact anchor lookup uses all five fields and retains every occurrence
  without amplification.
- Confirm both receipts are private-construction, serialization-only audit
  material and no current policy consumes them.
- Confirm every primary outcome, compound precedence case, malformed-surrogate
  case and alternate-canonical-spelling case is exercised.
- Confirm both bundle lengths, artifact hashes, roots and EOF bytes reproduce
  Ticket 0038 exactly.
- Confirm library code performs no ambient filesystem, network, callback, CAS,
  environment, mutable-global, second-store or write access.
- Confirm v0/v1/v2 standing and snapshot canonical bytes remain unchanged.
- Confirm no authority language equates acquisition with source identity,
  derivation with transformation correctness, or either match with truth,
  origin admission, support, aggregation or standing.

## Next slice

1. Ratify and merge Ticket 0039.
2. Design the immutable production `ResolutionContentClosureV0` construction
   boundary.
3. Pin the origin-binding bundle schema.
4. Pin explicit origin-admission policy.
5. Implement standing-inert origin-admission audit.
6. Implement admitted-contribution audit.
7. Pin policy-v3 aggregation.
8. Implement conservative aggregation.

Do not implement step 2 or later in this ticket.
