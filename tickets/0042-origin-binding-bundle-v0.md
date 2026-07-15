# Ticket 0042: Exact origin-binding bundle v0 contract

## 1. Exact starting point and worker authority

This documentation-only protocol ticket starts exactly from merge commit:

```text
dcbbab061408c0a7fd924171294d49216c40e06d
```

That commit is the merge of PR #53. The branch is exactly:

```text
agent/origin-binding-bundle-v0-contract
```

The suggested future human commit and PR title are both:

```text
docs: define origin-binding bundle v0
```

`AGENTS.md` is binding. Codex may edit and validate the exact scoped
documentation, but may not commit, push, open or edit a PR, mark readiness,
merge or enable auto-merge. The human retains sole commit, publication,
PR-state and merge authority.

## 2. Goal

Ratify the first exact origin-binding foreign-bundle protocol:

```text
this exact contribution
is assigned to this opaque origin group
under this claimed authority path
for this exact target origin-admission policy
```

The protocol freezes the canonical statements a future standing-inert verifier
may validate. It creates no runtime capability and no authority trust.

```text
canonical origin-binding statement
+ exact root
+ exact SegmentAnchored occurrence
+ future profile-specific verification
-> verified origin-binding statement

verified origin-binding statement
!= trusted authority
!= admitted origin
!= support contribution
!= corroboration
!= standing
```

The bundle records a governance assertion. It does not trust itself.

## 3. Exact changed paths

Exactly these six paths are in scope:

```text
README.md
docs/design/artifact-acquisition-derivation-bundle-v0.md
docs/design/artifact-provenance-origin-admission.md
docs/design/origin-binding-bundle-v0.md
docs/design/standing-aggregation-independence-groups.md
tickets/0042-origin-binding-bundle-v0.md
```

The design document and this ticket are new. No code, fixture, manifest,
lockfile, tool, release document, changelog, CI file or historical ticket may
change.

## 4. Relationship to parent contracts

- Ticket 0037 supplies the artifact-provenance and origin-admission authority
  architecture.
- Ticket 0038 ratifies the exact acquisition/direct-derivation sibling
  protocols.
- Ticket 0039 implements their standing-inert exact verifier and closed
  `ArtifactProvenanceContextTraceV0`.
- Ticket 0040 ratifies immutable `ResolutionContentClosureV0` availability.
- Ticket 0041 implements closure construction and exact read-only lookup but
  does not wire it into a verifier.

This ticket adds only the exact origin-binding statement contract. The
acquisition and derivation protocols remain unchanged. Origin-binding
verification, origin admission, admitted-contribution audit, policy v3 and
aggregation remain future work.

## 5. Two closed sibling schemas

V0 deliberately selects two distinguishable wire families.

The direct-acquisition family binds a contribution artifact to one exact
acquisition basis. The direct-derivation family binds a contribution artifact
to one exact direct derivation whose parent is established by one exact
acquisition basis.

Two sibling schemas were selected so neither shape needs optional or nullable
selectors, a `provenance_kind` switch, provenance-edge arrays, inferred
lineage, transitive graph search, wildcard ancestry or fallback acquisition
search. Each schema is closed, exact and independently auditable.

## 6. Every exact identity

```text
direct-acquisition bundle_kind:
magpie-origin-binding-acquisition-v0

direct-acquisition schema:
magpie-origin-binding-acquisition-v0

direct-derivation bundle_kind:
magpie-origin-binding-derivation-v0

direct-derivation schema:
magpie-origin-binding-derivation-v0

canonicalization_profile:
magpie-origin-binding-json-v0

verifier_profile:
magpie-origin-binding-verifier-v0

witness_algorithm:
sha256

artifact identity algorithm:
sha256

maximum canonical bundle bytes:
16,384
```

`verifier_profile` is not a bundle field. It identifies reviewed future code
and contract selection. There is no alias, negotiation, generic registry,
implicit latest profile, caller-selected verifier or environment override.

## 7. Exact direct-acquisition wire schema and order

```json
{
  "schema": "magpie-origin-binding-acquisition-v0",
  "binding_id": "...",
  "origin_admission_policy_id": "...",
  "contribution": {
    "target_claim_id": "...",
    "source_evidence_id": "...",
    "justification_edge_id": "...",
    "scope_ref": "...",
    "artifact": {
      "algorithm": "sha256",
      "digest": "..."
    }
  },
  "acquisition_selector": {
    "bundle_kind": "magpie-artifact-acquisition-v0",
    "witness_root": "...",
    "witness_algorithm": "sha256",
    "canonicalization_profile": "magpie-artifact-provenance-json-v0",
    "run_id": "..."
  },
  "origin_group": "...",
  "authority": {
    "kind": "...",
    "reference": "..."
  },
  "run_id": "..."
}
```

Exact top-level field order:

1. `schema`
2. `binding_id`
3. `origin_admission_policy_id`
4. `contribution`
5. `acquisition_selector`
6. `origin_group`
7. `authority`
8. `run_id`

## 8. Exact direct-derivation wire schema and order

```json
{
  "schema": "magpie-origin-binding-derivation-v0",
  "binding_id": "...",
  "origin_admission_policy_id": "...",
  "contribution": {
    "target_claim_id": "...",
    "source_evidence_id": "...",
    "justification_edge_id": "...",
    "scope_ref": "...",
    "artifact": {
      "algorithm": "sha256",
      "digest": "..."
    }
  },
  "acquisition_selector": {
    "bundle_kind": "magpie-artifact-acquisition-v0",
    "witness_root": "...",
    "witness_algorithm": "sha256",
    "canonicalization_profile": "magpie-artifact-provenance-json-v0",
    "run_id": "..."
  },
  "derivation_selector": {
    "bundle_kind": "magpie-artifact-derivation-v0",
    "witness_root": "...",
    "witness_algorithm": "sha256",
    "canonicalization_profile": "magpie-artifact-provenance-json-v0",
    "run_id": "..."
  },
  "origin_group": "...",
  "authority": {
    "kind": "...",
    "reference": "..."
  },
  "run_id": "..."
}
```

Exact top-level field order:

1. `schema`
2. `binding_id`
3. `origin_admission_policy_id`
4. `contribution`
5. `acquisition_selector`
6. `derivation_selector`
7. `origin_group`
8. `authority`
9. `run_id`

## 9. Exact nested field orders

Contribution:

1. `target_claim_id`
2. `source_evidence_id`
3. `justification_edge_id`
4. `scope_ref`
5. `artifact`

Artifact identity:

1. `algorithm`
2. `digest`

Acquisition and derivation selectors, matching the public
`ArtifactProvenanceAnchorSelectorV0` tuple:

1. `bundle_kind`
2. `witness_root`
3. `witness_algorithm`
4. `canonicalization_profile`
5. `run_id`

Authority claim:

1. `kind`
2. `reference`

Every field is required exactly once. No unknown, optional, nullable,
extension or arbitrary metadata member is permitted.

## 10. Validation classes

Protocol-controlled identifiers use exactly:

```text
[A-Za-z0-9][A-Za-z0-9._:/-]{0,127}
```

This grammar applies to `binding_id`, `origin_admission_policy_id`,
`origin_group`, `authority.kind`, `authority.reference` and bundle `run_id`.
The strings are ASCII only, 1-128 bytes, exact and case-sensitive. They have no
prefix, hierarchy, path, inheritance, wildcard, timestamp or authority
semantics. Selector `run_id` values retain the existing artifact-provenance
identifier contract.

Replayed references use exact UTF-8 length contracts instead:

```text
target_claim_id:       1-1,024 UTF-8 bytes
source_evidence_id:    1-1,024 UTF-8 bytes
justification_edge_id: 1-1,024 UTF-8 bytes
scope_ref:             1-1,024 UTF-8 bytes
```

There is no Unicode normalization, case folding, path interpretation, prefix
matching, wildcard or semantic rewriting. Syntactic validity does not prove a
named replay object exists.

Origin-binding replay-reference syntax must not admit a value forbidden by
all accepted replay objects it could match. An empty `contribution.scope_ref`
therefore fails schema-specific semantic validation as
`invalid replay reference length`, before any replay lookup.

Every artifact identity is exact `algorithm = sha256` plus exactly 64
lowercase hexadecimal digest characters. Equality covers both fields and
establishes bytes only.

Acquisition and derivation selectors require their exact bundle kinds, exact
`witness_algorithm = sha256`, exact
`canonicalization_profile = magpie-artifact-provenance-json-v0`, exactly 64
lowercase hexadecimal root characters and an existing-contract `run_id`.

## 11. Complete contribution identity and structural law

```text
ContributionIdentityV0 {
    target_claim_id
    source_evidence_id
    justification_edge_id
    scope_ref
    artifact {
        algorithm
        digest
    }
}
```

The future verifier must re-fetch all three replay nodes from one accepted
replay and require:

```text
edge.source_id == contribution.source_evidence_id
edge.target_id == contribution.target_claim_id
typed claim scope_ref
== typed evidence scope_ref
== edge scope_ref
== contribution.scope_ref
```

The exact artifact is the basis of the grouping decision. The identity is not
a publisher-global identity, URL, domain, source descriptor, evidence label,
permanent origin identity or aggregation lane. Unrelated IDs cannot
manufacture a contribution.

## 12. Derived comparison namespace law

No namespace object is serialized. Derive exactly:

```text
OriginComparisonNamespaceV0 {
    origin_admission_policy_id
    target_claim_id
    scope_ref
}
```

from the bundle policy ID and contribution target claim/scope.

The namespace excludes source evidence ID, edge ID, artifact identity,
acquisition selector, derivation selector, binding ID, run ID and complete
contribution identity. Including contribution-unique values would manufacture
apparent separation.

```text
origin binding is contribution-scoped

origin-group comparison is namespace-scoped

same origin_group + same derived namespace
-> same governed origin group

same textual origin_group + different namespace
-> no defined relationship
```

## 13. Policy-targeting boundary

`origin_admission_policy_id` is required claimed target data. It does not
select code, negotiate policy, prove policy existence, prove authority trust,
authorize caller selection or become an ambient latest policy.

Future reviewed code selects one exact policy and requires field equality. A
bundle that requests a different or more favourable policy cannot change the
selected implementation. Its verified statement may remain audit-visible
while policy mismatch prevents admission.

## 14. Claimed authority and origin-group boundary

The authority object contains exactly `kind` and `reference`. It asserts only
that the grouping decision was made through that claimed path. It proves no
trust, authenticity, admission, correctness or independence.

There is no `trusted`, `verified`, `approved`, `independent`, `confidence`,
`weight`, `score`, `priority`, `quorum`, `authority_is_valid`, signer public
key, certificate or self-supplied trust root.

`origin_group` is one opaque exact policy key. It is not truth, publisher,
authorship, source quality, confidence, reputation, ownership, organisation,
statistical independence, causal independence or authority. It has no
hierarchy, prefix, wildcard, path, containment, inheritance or cross-namespace
meaning.

Only a separate explicit versioned origin-admission policy may decide whether
the exact claimed authority path is trusted and whether a group is admitted.

## 15. Provenance coherence laws

Direct acquisition:

```text
verified acquisition receipt artifact identity
== contribution artifact identity
```

Direct derivation:

```text
verified acquisition receipt artifact identity
== verified derivation receipt parent artifact identity

verified derivation receipt derived artifact identity
== contribution artifact identity
```

The bundle names exact selectors. There is no selector substitution, lineage
inference, graph traversal, closest/newest/best match or alternate provenance
path.

The future origin-binding verifier must compose through the existing reviewed
artifact-provenance context surface and preserve the complete nested
`ArtifactProvenanceContextTraceV0` for every unsuccessful prerequisite. It
must not duplicate that parser vocabulary or reinterpret failure as a match.

## 16. Canonical profile, size, root and binding anchor

`magpie-origin-binding-json-v0` is fixed-schema canonical JSON, not JCS,
`magpie-core-v1`, `magpie-artifact-provenance-json-v0` or an extensible
profile.

It requires valid UTF-8; no BOM; one top-level object; no leading/trailing
bytes, whitespace or newline; duplicate-aware parsing; every required field
exactly once; no unknown fields; no `null`, Boolean, number or array; and only
strings plus the exact nested objects.

Object order is schema-defined. Canonical string encoding is exact:

```text
"       -> \"
\       -> \\
U+0008  -> \b
U+0009  -> \t
U+000A  -> \n
U+000C  -> \f
U+000D  -> \r
other U+0000-U+001F -> lowercase \u00xx
all other Unicode scalars -> direct UTF-8
```

There is no normalization, unnecessary escape, escaped solidus or uppercase
hexadecimal escape digit. Malformed escape/surrogate structures are invalid
JSON. A valid escaped surrogate pair is `NonCanonicalEncoding` because the
scalar must be emitted directly.

The future verifier re-encodes after semantic validation and requires exact
input equality. Parseable alternate whitespace or spelling is
`NonCanonicalEncoding`.

The exact raw limit is 16,384 bytes before UTF-8/JSON work.

```text
canonical_bytes = exact magpie-origin-binding-json-v0 bytes
witness_root = lowercase_hex(SHA-256(canonical_bytes))
```

The exact selected binding anchor contains schema-selected `bundle_kind`, the
computed root, `witness_algorithm = sha256`,
`canonicalization_profile = magpie-origin-binding-json-v0` and exact bundle
`run_id`. Bundle, selector and `SegmentAnchored` run IDs must agree. All five
anchor fields match exactly. `SegmentAnchored` remains occurrence/inclusion
only.

## 17. Future verifier fixed stage order

1. exact binding-bundle availability from `ResolutionContentClosureV0`;
2. raw binding-bundle byte limit;
3. UTF-8, BOM, JSON and top-level-object validity;
4. duplicate-key detection;
5. required-field, unknown-field and JSON-type validation;
6. schema-specific semantic validation;
7. exact canonical re-encoding equality;
8. selected binding selector: bundle kind, witness algorithm,
   canonicalization profile and `run_id`;
9. computed binding-root equality;
10. exact binding-anchor occurrence in the accepted replay;
11. exact contribution structural revalidation;
12. exact acquisition prerequisite resolution;
13. exact derivation prerequisite resolution for the derivation family;
14. exact cross-statement artifact coherence; and
15. one matched standing-inert origin-binding receipt.

Caller order and ambient storage cannot alter this precedence.

## 18. Minimum closed outcome vocabulary

Binding bundle/parser distinctions:

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
```

Binding selector/replay distinctions:

```text
binding bundle-kind mismatch
binding witness-algorithm mismatch
binding canonicalization-profile mismatch
binding run-ID mismatch
binding witness-root mismatch
binding anchor absent
```

Contribution distinctions:

```text
target claim absent
source evidence absent
justification edge absent
edge source mismatch
edge target mismatch
scope mismatch
contribution artifact mismatch
```

Prerequisite distinctions:

```text
acquisition prerequisite unresolved
derivation prerequisite unresolved
acquisition artifact differs from contribution artifact
acquisition artifact differs from derivation parent
derivation output differs from contribution artifact
```

Matched distinctions:

```text
matched acquisition-based origin binding
matched derivation-based origin binding
```

Exact Rust spelling is deferred. No distinction may collapse into Boolean
`verified`; unresolved provenance retains the exact nested existing trace.

## 19. Future receipt and authority boundary

A future private-construction `OriginBindingReceiptV0` must retain verifier
profile, complete binding selector, family/schema, binding ID, target policy,
complete `ContributionIdentityV0`, derived namespace, acquisition selector,
family-selected derivation selector, origin group, both authority fields,
binding run ID, byte length, computed root, all binding occurrences, successful
acquisition receipt and family-selected successful derivation receipt.

It must have private fields, getters only, no public constructor, no
`Deserialize`, and serializable audit output only. No API accepts a caller-
created receipt as authority.

```text
matched origin-binding receipt
= verified governance assertion

matched origin-binding receipt
!= trusted governance authority
!= admitted origin group
!= support contribution
!= aggregation input
!= standing
```

## 20. Duplicate and conflict doctrine

Exact verified bindings with the same complete contribution identity, derived
namespace, group and target policy are duplicate occurrences. They remain
visible, create at most one assignment, do not amplify and need not conflict.

Verified bindings with the same complete contribution identity, derived
namespace and target policy but different groups create an explicit unresolved
conflict. Both remain visible; no group is admitted; zero corroboration
separation results. There is no first-write-wins, last-write-wins, lexical
minimum, higher-sequence, newer-looking-run-ID, human-looking-authority or
implicit-supersession rule.

Distinct contribution identities may map to the same group in one namespace.
That is valid intended same-origin grouping, not duplication or conflict.

Distinct contributions with different admitted groups in one namespace may
later provide policy-recognised corroboration separation. This contract admits
no group and creates no support.

## 21. Normative canonical vectors

These vectors freeze exact origin-binding bytes and roots. Selector roots do
not prove referenced bundles exist.

### 21.1 Direct-acquisition binding vector

The vector reuses the committed acquisition fixture selector and artifact.

Exact canonical JSON bytes, one literal line with no trailing newline:

```json
{"schema":"magpie-origin-binding-acquisition-v0","binding_id":"origin-binding-acq-0001","origin_admission_policy_id":"magpie-origin-admission-v0","contribution":{"target_claim_id":"claim-origin-0001","source_evidence_id":"evidence-origin-acq-0001","justification_edge_id":"edge-origin-acq-0001","scope_ref":"scope:origin-example","artifact":{"algorithm":"sha256","digest":"51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076"}},"acquisition_selector":{"bundle_kind":"magpie-artifact-acquisition-v0","witness_root":"4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e","witness_algorithm":"sha256","canonicalization_profile":"magpie-artifact-provenance-json-v0","run_id":"run-acq-0001"},"origin_group":"origin-group-report-0001","authority":{"kind":"human-review","reference":"authority:origin-review-v0"},"run_id":"run-origin-binding-acq-0001"}
```

```text
UTF-8 byte length: 870
witness_root: 4f4bbaab87c1cfdc4dfe3b8c0d8b11a04bdc637de1e8e1481001b09543f0a4d7
first byte: 0x7b
final byte: 0x7d
UTF-8 BOM: absent
trailing newline: absent
```

```text
bundle_kind: magpie-origin-binding-acquisition-v0
witness_root: 4f4bbaab87c1cfdc4dfe3b8c0d8b11a04bdc637de1e8e1481001b09543f0a4d7
witness_algorithm: sha256
canonicalization_profile: magpie-origin-binding-json-v0
run_id: run-origin-binding-acq-0001
```

### 21.2 Direct-derivation binding vector

Explicit hypothetical coherence:

```text
hypothetical acquisition artifact:
sha256:b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060

committed derivation fixture parent:
sha256:b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060

committed derivation fixture output:
sha256:1921b918b15842c7fdb115078e610263fac85f159c1d8e0ecec3d89a0faa4005

contribution artifact:
sha256:1921b918b15842c7fdb115078e610263fac85f159c1d8e0ecec3d89a0faa4005
```

The hypothetical acquisition selector root is SHA-256 over this separately
calculated 294-byte canonical statement:

```json
{"schema":"magpie-artifact-acquisition-v0","acquisition_id":"acq-alpha-0001","acquisition_profile":"magpie-import-v0","artifact":{"algorithm":"sha256","digest":"b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060"},"observed_locator":"file:alpha.txt","run_id":"run-acq-alpha-0001"}
```

Its root is
`94593dd609cb44b07c2f3dd5a72226ff4c02ad4b9f4378a84d4432c433a1f7ce`.
The acquisition statement and selector are hypothetical, not fixtures.

Exact canonical origin-binding JSON bytes, one literal line with no trailing
newline:

```json
{"schema":"magpie-origin-binding-derivation-v0","binding_id":"origin-binding-der-0001","origin_admission_policy_id":"magpie-origin-admission-v0","contribution":{"target_claim_id":"claim-origin-0001","source_evidence_id":"evidence-origin-der-0001","justification_edge_id":"edge-origin-der-0001","scope_ref":"scope:origin-example","artifact":{"algorithm":"sha256","digest":"1921b918b15842c7fdb115078e610263fac85f159c1d8e0ecec3d89a0faa4005"}},"acquisition_selector":{"bundle_kind":"magpie-artifact-acquisition-v0","witness_root":"94593dd609cb44b07c2f3dd5a72226ff4c02ad4b9f4378a84d4432c433a1f7ce","witness_algorithm":"sha256","canonicalization_profile":"magpie-artifact-provenance-json-v0","run_id":"run-acq-alpha-0001"},"derivation_selector":{"bundle_kind":"magpie-artifact-derivation-v0","witness_root":"472e857153fe8c901b6c0d7d1b15bd876450b57b4743222a2fab91eb60bc76dd","witness_algorithm":"sha256","canonicalization_profile":"magpie-artifact-provenance-json-v0","run_id":"run-der-0001"},"origin_group":"origin-group-report-0001","authority":{"kind":"human-review","reference":"authority:origin-review-v0"},"run_id":"run-origin-binding-der-0001"}
```

```text
UTF-8 byte length: 1,144
witness_root: 24aa77f2f601e70f6d5d6044f0b1274394f164276df07296be16ab59953a5a29
first byte: 0x7b
final byte: 0x7d
UTF-8 BOM: absent
trailing newline: absent
```

```text
bundle_kind: magpie-origin-binding-derivation-v0
witness_root: 24aa77f2f601e70f6d5d6044f0b1274394f164276df07296be16ab59953a5a29
witness_algorithm: sha256
canonicalization_profile: magpie-origin-binding-json-v0
run_id: run-origin-binding-der-0001
```

This vector freezes origin-binding canonical bytes and root only. It is not an
end-to-end verified provenance fixture and does not claim that the
hypothetical acquisition or binding material is available or anchored.

## 22. Independent vector calculations

Two temporary, uncommitted Python calculations are mandatory:

1. an insertion-ordered structured representation with compact UTF-8 JSON
   emission; and
2. a separately written fixed-schema/manual encoder with its own exact string
   escaping.

Both must independently reproduce the two complete literal canonical lines,
870 and 1,144 byte lengths, roots
`4f4bbaab87c1cfdc4dfe3b8c0d8b11a04bdc637de1e8e1481001b09543f0a4d7`
and
`24aa77f2f601e70f6d5d6044f0b1274394f164276df07296be16ab59953a5a29`,
first byte `0x7b`, final byte `0x7d`, BOM absence and newline absence.

Both must also reproduce the 294-byte hypothetical acquisition statement root
`94593dd609cb44b07c2f3dd5a72226ff4c02ad4b9f4378a84d4432c433a1f7ce`.
No temporary script is committed.

## 23. Required hostile cases

The design and later verifier contract must cover at least:

1. `trusted: true` is unknown/wrong schema and grants no authority.
2. A self-named public key is only an opaque claim, not trust bootstrap.
3. A different target policy may yield a verified statement but cannot pass
   selected-policy equality.
4. Unrelated claim/evidence/edge IDs are a structural subject mismatch.
5. Different claim/evidence/edge scopes are a scope mismatch.
6. Direct-acquisition contribution/acquisition artifact disagreement is an
   artifact mismatch.
7. Acquisition artifact/derivation parent disagreement is a chain mismatch.
8. Contribution artifact/derivation output disagreement is a contribution
   mismatch.
9. Canonical root-correct but unanchored binding is `binding anchor absent`.
10. Anchored binding absent from closure `M` is bundle unavailable, with no
    ambient lookup.
11. Missing nested bundles retain exact unresolved provenance traces.
12. One-field nested selector mismatch has no fallback.
13. Byte-identical content under another acquisition selector is not
    substituted.
14. Same group under another policy, claim or scope is incomparable.
15. Different groups for one contribution produce later explicit conflict and
    no admission.
16. Different contributions assigned one group in one namespace are valid
    same-origin grouping.
17. Different claimed groups provide only potential separation after explicit
    admission, never support here.
18. Timestamp-looking run IDs carry no temporal authority.
19. Higher later sequence carries no implicit supersession.
20. URL, publisher, source descriptor, signature count, model confidence or
    distinct artifact digest cannot create an admitted origin without governed
    binding.
21. Empty `contribution.scope_ref` is `invalid replay reference length` during
    semantic reference validation → no replay lookup → no matched
    origin-binding receipt.

## 24. Frozen surfaces and explicit non-goals

Frozen unchanged:

- `magpie-core-v1`, all L0 payloads and `SegmentAnchored` semantics;
- acquisition/direct-derivation schemas, canonical bytes, vectors, roots and
  verifier behavior;
- `ArtifactProvenanceAnchorSelectorV0` and
  `ArtifactProvenanceContextTraceV0`;
- `ResolutionContentClosureV0` construction, identity, manifest and lookup;
- standing policies v0/v1/v2, snapshots and resolution bytes;
- Rust, tests, fixtures, Cargo manifests, dependencies, tools, release files,
  changelog, CI and historical Tickets 0037-0041.

No Rust, fixture, parser, canonical encoder, verifier, receipt, closure wiring,
origin-admission policy, trusted-authority table, conflict fold,
admitted-contribution audit, support rule, standing rule, policy v3,
aggregation rule, threshold, confidence score, source reputation, publisher or
identity ontology, supersession, revocation, expiry, temporal ownership,
loader, CAS, network/filesystem access, callback, new L0 payload, writer or
Deadbolt change is introduced.

## 25. Required validation

Run and report actual results only:

```powershell
git diff --check
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo run --locked --example tour -p magpie-claims

python tools/check_release_metadata.py
cargo package -p magpie-log --locked

python tools/verify_chain.py `
  crates/magpie-log/testdata/golden-v1.jsonl `
  ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c

python tools/verify_chain.py `
  fixtures/deadbolt-anchor-v1/anchor-log.jsonl `
  d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737

git diff --check
git status --short
git diff --stat
git diff --name-only
```

If `cargo package -p magpie-log --locked` refuses solely because intended
README changes are uncommitted, report the exact refusal, do not commit and do
not use `--allow-dirty`. Treat the clean-tree rerun as a human post-commit
publication gate.

Run the prohibited-path, exact changed-path, authority-confusion,
runtime-overclaim and schema-consistency audits required by the implementation
prompt. Review every search match manually and inspect the complete diff from
the exact base.

## 26. Reviewer checklist

- Confirm the branch starts exactly at
  `dcbbab061408c0a7fd924171294d49216c40e06d`.
- Confirm exactly the six scoped paths changed and no prohibited path changed.
- Confirm exactly two closed sibling schemas exist and no optional selector
  ambiguity appears.
- Confirm every exact identity and field order agrees between this ticket and
  the design.
- Confirm protocol identifiers and replay references use distinct validation
  classes.
- Confirm contribution identity is complete and same-replay revalidation is
  mandatory.
- Confirm the namespace is derived from policy/claim/scope, never serialized.
- Confirm policy targeting cannot select runtime code.
- Confirm authority is claimed but untrusted and cannot bootstrap itself.
- Confirm origin-group equality is namespace-local and opaque.
- Confirm provenance selectors are exact and no search/substitution exists.
- Confirm the canonical profile, size, root and five-field anchor are exact.
- Confirm verifier precedence and minimum outcome vocabulary are closed.
- Confirm nested provenance traces are retained rather than reduced to a
  Boolean.
- Confirm a future receipt is private-construction audit material only.
- Confirm duplicates, conflicts, same-origin grouping and potential separation
  are distinct.
- Confirm both vectors and the hypothetical parent acquisition reproduce under
  two independently structured calculations.
- Confirm all 21 hostile cases are explicit.
- Confirm no parser, verifier, admission, support, standing, aggregation,
  loader, writer or runtime claim appears.
- Confirm every validation result reported at handoff actually ran.

## 27. Exact next slice

The next separately reviewed slice is only the standing-inert origin-binding
verifier:

```text
exact closure lookup
+ strict two-family parser and canonical encoder
+ binding selector/root/anchor verification
+ contribution structural revalidation
+ exact existing provenance-context composition
+ artifact coherence
+ private matched receipt
+ hostile tests
```

It must not add origin admission, authority trust, conflict admission fold,
admitted-contribution audit, support, standing, policy v3, aggregation, loader,
CAS, filesystem/network access, L0 payload or writer authority.
