# Ticket 0045: Standing-inert origin-binding verifier v0

## 1. Publication identity and exact base

```text
base: c12ef0756a2984c99329679d8630ddec5bc4bc39
branch: agent/standing-inert-origin-binding-verifier-v0
commit: claims: implement origin-binding verifier v0
PR title: claims: implement origin-binding verifier v0
```

This slice implements deterministic verification of one exact origin-binding
governance assertion. It does not implement trust or admission.

```text
correctness:
strict deterministic verification only

authority:
no authority is admitted

standing:
unchanged

aggregation:
not implemented
```

## 2. Exact changed paths

Only these paths may change:

```text
README.md
crates/magpie-claims/src/artifact_provenance_verifier.rs
crates/magpie-claims/src/lib.rs
crates/magpie-claims/src/origin_binding_verifier.rs
crates/magpie-claims/tests/origin_binding_verifier.rs
fixtures/origin-binding-v0/README.md
fixtures/origin-binding-v0/acquisition-binding.json
fixtures/origin-binding-v0/derivation-binding.json
fixtures/origin-binding-v0/derivation-parent-acquisition-bundle.json
tickets/0045-standing-inert-origin-binding-verifier-v0.md
```

## 3. Central architectural law

```text
one immutable ResolutionContentClosureV0
+ one verified StandingReplaySnapshot
+ one exact origin-binding selector
-> one deterministic origin-binding audit trace

verified origin-binding statement
!= trusted authority
!= admitted origin
!= support contribution
!= corroboration
!= standing
```

A match means only that the supplied closure contained one strictly parsed,
canonically encoded, root-matched, same-replay-anchored assertion structurally
bound to one exact contribution and backed by exact verified provenance. The
bundle records governance content; it does not trust itself.

## 4. Exact public constants

The crate exports exactly these origin-binding protocol constants:

```rust
pub const ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0: &str =
    "magpie-origin-binding-acquisition-v0";
pub const ORIGIN_BINDING_ACQUISITION_SCHEMA_V0: &str =
    "magpie-origin-binding-acquisition-v0";
pub const ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0: &str =
    "magpie-origin-binding-derivation-v0";
pub const ORIGIN_BINDING_DERIVATION_SCHEMA_V0: &str =
    "magpie-origin-binding-derivation-v0";
pub const ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0: &str =
    "magpie-origin-binding-json-v0";
pub const ORIGIN_BINDING_VERIFIER_PROFILE_V0: &str =
    "magpie-origin-binding-verifier-v0";
pub const ORIGIN_BINDING_WITNESS_ALGORITHM_V0: &str = "sha256";
pub const ORIGIN_BINDING_ARTIFACT_ALGORITHM_V0: &str = "sha256";
pub const MAX_ORIGIN_BINDING_BUNDLE_BYTES_V0: usize = 16_384;
```

There is no registry, alias, negotiation, implicit-latest, environment override,
caller-selected verifier, or runtime policy selector.

## 5. Exact public types

The crate exports:

```text
ContributionIdentityV0
OriginComparisonNamespaceV0
OriginBindingFamilyV0
OriginBindingReceiptV0
OriginBindingContextTraceV0
```

`ContributionIdentityV0` privately retains target claim ID, source evidence
ID, justification edge ID, scope, artifact algorithm and artifact digest. It
has read-only getters, deterministic comparison and serialization traits, no
public constructor and no `Deserialize`.

`OriginComparisonNamespaceV0` is derived from exactly origin-admission policy
ID, target claim ID and scope. It excludes evidence ID, edge ID, artifact,
selectors, binding ID, binding run ID and complete contribution identity. It
has getters only and no `Deserialize`.

`OriginBindingFamilyV0` is the closed enum `Acquisition | Derivation`; there is
no `Other` or arbitrary family string.

`OriginBindingReceiptV0` is private-construction, getter-only, serializable
audit material. A private closed provenance enum makes acquisition-only and
acquisition-plus-derivation receipts the only representable family states.
There is no public constructor and no `Deserialize`.

`OriginBindingContextTraceV0` is a closed adjacently tagged serializable trace.
It has `canonical_bytes`, `matched_receipt`, `matched_acquisition` and
`matched_derivation` read-only helpers, and no `Deserialize`.

## 6. Snapshot-only resolver API

The only new resolver is:

```rust
pub fn resolve_origin_binding_context_v0(
    &self,
    binding_selector: &ArtifactProvenanceAnchorSelectorV0,
    closure: &ResolutionContentClosureV0,
) -> OriginBindingContextTraceV0
```

It exists only on `StandingReplaySnapshot`, reuses the existing five-field
selector, takes one immutable closure, performs no mutation or second replay,
and consults only same-replay private projections. There is no free resolver,
no `StandingView` or `DeadboltAnchorIndex` equivalent, no caller-created anchor
index, no caller-created receipt input, no trusted-authority Boolean, and no
serialized receipt authority input.

## 7. Two closed wire families

Direct acquisition has exact top-level order:

```text
schema
binding_id
origin_admission_policy_id
contribution
acquisition_selector
origin_group
authority
run_id
```

Direct derivation has exact top-level order:

```text
schema
binding_id
origin_admission_policy_id
contribution
acquisition_selector
derivation_selector
origin_group
authority
run_id
```

Contribution order is `target_claim_id`, `source_evidence_id`,
`justification_edge_id`, `scope_ref`, `artifact`; artifact order is
`algorithm`, `digest`; selector order is `bundle_kind`, `witness_root`,
`witness_algorithm`, `canonicalization_profile`, `run_id`; authority order is
`kind`, `reference`. Every member is required exactly once. No nullable field,
extension member, generic map, metadata object or `provenance_kind` switch is
accepted.

## 8. Immutable closure lookup

Stage 1 uses only:

```rust
closure.foreign_bundle(binding_selector)
```

Nested provenance composition also uses exact closure keys. No lookup occurs
before availability is established, and there is no filesystem, CAS, network,
environment, callback, plugin, search, alternate selector or ambient source.
Closure insertion order cannot alter the result.

## 9. Strict parsing and canonicalisation

The origin-binding module has a private duplicate-aware fixed-schema parser for
only its two schemas. It validates UTF-8, rejects BOM distinctly, requires one
top-level object, rejects trailing JSON, observes duplicate keys at every depth
including discarded unknown and wrong-type subtrees, distinguishes missing
from wrong type, and applies fixed failure precedence. It never converts the
input into `serde_json::Value`.

Protocol identifiers use `[A-Za-z0-9][A-Za-z0-9._:/-]{0,127}`. Replay
references are exact non-empty 1-1,024-byte UTF-8 strings. Artifact identity is
exactly `sha256` plus 64 lowercase hexadecimal characters. Nested selectors
must use the exact artifact-provenance family identity, `sha256`,
`magpie-artifact-provenance-json-v0`, a 64-character lowercase root, and a
valid existing-contract run ID.

The private encoder emits only schema order, compact JSON, direct UTF-8 and the
ratified escapes. It performs no sorting, Unicode normalization or JCS claim.
After semantics, exact re-encoding must equal the complete supplied bytes
before hashing. Pretty whitespace, reordering, trailing newline, unnecessary
escapes, escaped solidus, uppercase escape hex and escaped surrogate pairs are
noncanonical.

## 10. Complete fixed failure order

```text
1. exact binding-bundle lookup in ResolutionContentClosureV0
2. raw binding-bundle byte limit
3. UTF-8, BOM, JSON and top-level-object validity
4. duplicate-key detection
5. required-field, unknown-field and JSON-type validation
6. schema-specific semantic validation
7. exact canonical re-encoding equality
8. selected binding selector fields
9. computed binding-root equality
10. exact binding-anchor occurrence in accepted replay
11. exact contribution structural revalidation
12. exact acquisition prerequisite resolution
13. exact derivation prerequisite resolution for derivation family
14. exact cross-statement artifact coherence
15. one matched standing-inert origin-binding receipt
```

Within selector comparison the order is bundle kind, witness algorithm,
canonicalization profile, run ID, computed root, anchor. All exact binding
anchor occurrences are retained in replay sequence order; multiplicity is
audit visibility and never match amplification.

## 11. Closed trace vocabulary

The public trace keeps separate variants for:

```text
binding bundle unavailable
binding bundle too large
invalid UTF-8
UTF-8 BOM present
invalid JSON
top-level value not object
duplicate key
missing field
unknown field
wrong JSON type
schema mismatch
invalid protocol identifier
invalid replay reference length
unsupported artifact algorithm
invalid artifact digest
invalid acquisition selector
invalid derivation selector
noncanonical encoding
binding bundle-kind mismatch
binding witness-algorithm mismatch
binding canonicalization-profile mismatch
binding run-ID mismatch
binding witness-root mismatch
binding anchor absent
target claim absent
source evidence absent
justification edge absent
edge source mismatch
edge target mismatch
scope mismatch
acquisition prerequisite unresolved with exact nested trace
derivation prerequisite unresolved with exact nested trace
acquisition artifact differs from contribution artifact
acquisition artifact differs from derivation parent
derivation output differs from contribution artifact
matched acquisition-based origin binding
matched derivation-based origin binding
```

No Boolean, generic invalid/unresolved string, `Other`, or flattened nested
provenance outcome replaces these distinctions.

## 12. Contribution structural revalidation

The verifier re-fetches typed claim, typed evidence and justification edge from
`self.standing()` in that exact absence order. It then checks edge source, edge
target and scope in that order:

```text
edge.source_id == contribution.source_evidence_id
edge.target_id == contribution.target_claim_id
claim.scope_ref == evidence.scope_ref == edge.scope_ref == contribution.scope_ref
```

Unrelated individually existing IDs cannot manufacture a contribution.
Actor classes, rationale, metadata, labels, source descriptors and asserted
`TypedEvidenceNode.content_hash` do not establish artifact identity or
authority.

## 13. Closure-aware artifact-provenance composition

`artifact_provenance_verifier.rs` adds two crate-private closure-aware helpers.
Both the existing explicit-slice APIs and the new helpers call the same private
reviewed acquisition or derivation pipeline. The helpers look up the exact
nested bundle, parse it with the existing parser, discover artifact identities
from that canonical nested statement, and only then perform exact closure
artifact lookup.

The origin-binding module neither parses nested provenance bundles nor copies
the existing parser, canonical encoder, error vocabulary, selector/root/anchor
logic or receipt construction. The public artifact-provenance trace, receipt,
canonical bytes, thirteen-stage precedence and explicit-slice APIs remain
unchanged.

## 14. Cross-statement artifact coherence

For acquisition:

```text
verified acquisition artifact == contribution artifact
```

For derivation:

```text
verified acquisition artifact == verified derivation parent
verified derivation output == contribution artifact
```

Each mismatch has its own trace and is checked only after all applicable
prerequisite verifiers genuinely match. There is no graph traversal,
transitive lineage, alternate acquisition or selector fallback.

## 15. Receipt construction and authority boundary

The matched receipt retains verifier profile, full binding selector,
family/schema, binding ID, policy ID, complete contribution, derived namespace,
full acquisition selector, family-selected derivation selector, origin group,
authority kind/reference, binding run ID, bundle length, computed root, every
binding occurrence, successful acquisition receipt and family-selected
successful derivation receipt.

`origin_group`, `authority.kind`, `authority.reference` and
`origin_admission_policy_id` remain opaque verified statement content. They do
not select code, prove a policy, trust an authority, bootstrap a trust root,
admit a group, identify a publisher, establish independence or truth, or affect
support or standing. `"trusted": true` is an unknown field. Cloned or serialized
receipts are audit output only and cannot be supplied to any resolver.

## 16. Fixture inventory and pinned roots

```text
fixtures/origin-binding-v0/README.md
fixtures/origin-binding-v0/acquisition-binding.json
  870 bytes
  sha256 4f4bbaab87c1cfdc4dfe3b8c0d8b11a04bdc637de1e8e1481001b09543f0a4d7
fixtures/origin-binding-v0/derivation-binding.json
  1,144 bytes
  sha256 24aa77f2f601e70f6d5d6044f0b1274394f164276df07296be16ab59953a5a29
fixtures/origin-binding-v0/derivation-parent-acquisition-bundle.json
  294 bytes
  sha256 94593dd609cb44b07c2f3dd5a72226ff4c02ad4b9f4378a84d4432c433a1f7ce
```

Each begins with `0x7b`, ends with `0x7d`, has no BOM and no trailing newline.
Existing artifact-provenance bundles and artifact bytes are reused without
duplication. No signed anchor log or trust root is committed; deterministic
signed Magpie chains are built at test runtime.

## 17. Hostile test matrix

The integration suite covers both positive families; closure absence; exact
same-replay anchor absence; all parser/shape/semantic traces; nested duplicates
including unknown and wrong-type subtrees; canonical whitespace/order/escape
hostiles; every binding selector stage; complete contribution failure order;
unrelated-node non-composition; exact nested artifact-provenance trace retention
for selector mismatch, root mismatch, anchor absence, artifact absence and
digest mismatch; all three coherence mismatches after successful prerequisites;
three-anchor multiplicity; repeated-call and closure-order determinism; receipt
non-substitution; exact fixture bytes; and standing-byte inertia.

Compile-fail module documentation proves private receipt construction, absence
of `Deserialize` for receipt/contribution/namespace, absence of a `StandingView`
resolver, absence of a free caller-index resolver, and inability to submit a
receipt to the snapshot resolver.

## 18. Standing-inert and deterministic-byte requirements

Resolving one or more times must leave byte-identical:

```text
StandingReplaySnapshot::canonical_bytes()
StandingView::canonical_bytes()
StandingResolution v0 canonical bytes
StandingResolution v1 canonical bytes
StandingResolution v2 canonical bytes
```

Governed standing, legacy raw standing, blockers and standing traces also remain
equal. Origin-binding traces are not added to standing v0, v1 or v2. Equal
`H + selector + M` yields equal traces, equal receipts and byte-identical trace
canonical bytes.

## 19. Frozen surfaces and non-goals

This ticket changes no L0 schema/tag/encoding, signature protocol, anchor
semantics or types, replay-snapshot/standing bytes or fields, standing policy
v0/v1/v2, policy ID, support/refutation ceiling, deterministic verifier,
artifact-provenance public result or fixture, closure construction/identity/API,
log store/replay, Cargo manifest, dependency, lockfile, release metadata, tour
output, writer or Deadbolt surface.

It does not implement authority trust, signer/key registry, origin admission,
runtime policy selection, conflict or duplicate fold, admitted-origin or
admitted-contribution result, support, policy v3, aggregation, corroboration,
thresholds, probability, reputation, publisher identity, statistical
independence, transitive provenance, traversal, supersession, revocation,
expiry, loader, CAS, filesystem/network/callback/plugin lookup, cache, writer,
new L0 payload or Deadbolt change.

## 20. Validation, authority, review and stop conditions

Run focused validation first:

```powershell
cargo test -p magpie-claims --locked --lib
cargo test -p magpie-claims --locked --doc
cargo test -p magpie-claims --locked --test artifact_provenance_verifier
cargo test -p magpie-claims --locked --test resolution_content_closure
cargo test -p magpie-claims --locked --test origin_binding_verifier
cargo test -p magpie-claims --locked --test origin_binding_verifier
cargo test -p magpie-claims --locked --test origin_binding_verifier
```

Then run:

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
```

Independently reproduce all three fixture bytes twice: once with an
insertion-ordered structured compact JSON emitter and once with a separately
written fixed-schema manual encoder and exact escaping function. Compare both
outputs directly with committed bytes and verify lengths, hashes, boundary
bytes, BOM absence and newline absence. Temporary scripts are not committed.

George authorizes, only after all validation passes, staging the exact paths,
one exact commit, one non-force push of the named branch and one draft PR. The
worker must not change main, mark ready, merge, enable auto-merge, force-push,
resolve reviews, modify another PR, or add commits/branches.

Reviewer checklist:

- Confirm the exact base, one-commit scope and ten-path allowlist.
- Confirm only the snapshot owns the public resolver authority boundary.
- Confirm origin parsing is fixed-schema and nested provenance parsing is not duplicated.
- Confirm both explicit-slice provenance APIs retain byte-identical behavior.
- Confirm closure lookup follows identities declared by nested statements.
- Confirm all prerequisite traces and coherence mismatches retain their exact order.
- Confirm receipts are private-construction audit output and admit no authority.
- Confirm standing v0/v1/v2 and replay/closure frozen surfaces are unchanged.
- Confirm exact fixture bytes, independent calculations and complete validation evidence.

Stop with the smallest clear failing test if implementation would require a
canonical-vector change, public provenance-result or precedence change, closure
change, dependency, standing/policy change, caller receipt authority, nested
parser duplication, ambient lookup, second replay, new L0 payload, additional
family, nullable family selector, or any path outside the allowlist.
