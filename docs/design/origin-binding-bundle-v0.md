# Origin-Binding Bundle v0

## 1. Status

Ratified exact protocol contract. Ticket 0042 is the documentation-only slice
that freezes the first origin-binding foreign-bundle protocol.

This document defines exact statements for the standing-inert origin-binding
verifier, whose runtime landed separately.
It implements no parser, encoder, fixture, verifier, receipt type, closure-to-
verifier wiring, origin admission, authority trust, contribution admission,
support, standing, policy v3, aggregation, loader, CAS, filesystem or network
access, writer authority, or Magpie L0 payload.

The current implementation substrate ends at:

```text
exact artifact identity
+ verified acquisition/direct-derivation context
+ immutable ResolutionContentClosureV0
```

This contract adds the exact governance statement shape, not runtime
capability.

## 2. Purpose and relationship to Tickets 0037-0041

[Ticket 0037](../../tickets/0037-artifact-provenance-origin-admission.md) and
[`artifact-provenance-origin-admission.md`](artifact-provenance-origin-admission.md)
separate artifact identity, acquisition, lineage, origin admission, support
and aggregation.

[Ticket 0038](../../tickets/0038-artifact-acquisition-derivation-bundle-v0.md)
ratified the exact acquisition and direct-derivation sibling protocols.
[Ticket 0039](../../tickets/0039-artifact-provenance-verifier-v0.md)
implemented their standing-inert parser, canonicalizer, exact replay verifier,
portable fixtures and closed `ArtifactProvenanceContextTraceV0` audit surface.

[Ticket 0040](../../tickets/0040-resolution-content-closure-v0.md) ratified
the immutable finite availability universe `M`.
[Ticket 0041](../../tickets/0041-resolution-content-closure-v0-implementation.md)
implemented its bounded construction, canonical manifest, typed identity and
exact read-only lookup. It did not wire the closure into a verifier.

Ticket 0042 now answers only:

```text
this exact contribution
is assigned to this opaque origin group
under this claimed authority path
for this exact target origin-admission policy
```

The acquisition/direct-derivation protocol remains unchanged. At Ticket 0042
ratification time, origin-binding verification and origin admission remained
future slices; those slices subsequently landed under their separately
reviewed tickets.

## 3. Architectural laws

The positive verification construction is:

```text
canonical origin-binding statement
+ exact root
+ exact SegmentAnchored occurrence
+ profile-specific verification
-> verified origin-binding statement
```

The authority boundary is:

```text
verified origin-binding statement
!= trusted authority
!= admitted origin
!= support contribution
!= corroboration
!= standing
```

The bundle records a governance assertion. It does not trust itself.

The comparison laws are:

```text
origin binding is contribution-scoped

origin-group comparison is namespace-scoped
```

The deterministic input remains:

```text
verified log prefix H
+ explicit policy identities P
+ immutable resolution content closure M
-> deterministic derived result

same H + same P + same M
-> same derived result
```

No bundle field, caller argument or ambient state may select hidden authority.

## 4. Exact two-family decision

V0 has two closed, distinguishable sibling schemas.

The direct-acquisition family binds a contribution whose artifact is expected
to equal the artifact established by one exact acquisition bundle.

The direct-derivation family binds a contribution whose artifact is expected
to equal the output of one exact direct-derivation bundle, whose direct parent
is expected to equal the artifact established by one exact acquisition
bundle.

The two-family decision forbids:

- optional or nullable selector fields;
- one schema with a `provenance_kind` switch;
- arrays of provenance edges;
- inferred or transitive lineage;
- graph search or wildcard ancestry; and
- “find any matching acquisition” semantics.

Each wire shape is closed and exact. Family selection is made by the exact
schema and bundle-kind pair, not by optional-field presence.

## 5. Exact protocol identities

The direct-acquisition family uses exactly:

```text
bundle_kind: magpie-origin-binding-acquisition-v0
schema: magpie-origin-binding-acquisition-v0
```

The direct-derivation family uses exactly:

```text
bundle_kind: magpie-origin-binding-derivation-v0
schema: magpie-origin-binding-derivation-v0
```

Both families use exactly:

```text
canonicalization_profile: magpie-origin-binding-json-v0
verifier_profile: magpie-origin-binding-verifier-v0
witness_algorithm: sha256
artifact identity algorithm: sha256
maximum canonical bundle bytes: 16,384
```

`verifier_profile` is not a bundle field. It identifies later reviewed code
and contract selection. There is no alias, runtime negotiation, generic schema
registry, algorithm registry, implicit latest profile, caller-selected
verifier or environment override.

The frozen `SegmentAnchored` seam requires a root represented by exactly 64
lowercase hexadecimal characters for 32 bytes. This profile therefore uses
SHA-256 over the exact canonical origin-binding bundle bytes.

## 6. Exact direct-acquisition schema

The complete logical object is:

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

Every field is required exactly once. No unknown field is permitted.

## 7. Exact direct-derivation schema

The complete logical object is:

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

Every field is required exactly once. No unknown field is permitted.

## 8. Exact nested field orders

`contribution` uses exactly:

1. `target_claim_id`
2. `source_evidence_id`
3. `justification_edge_id`
4. `scope_ref`
5. `artifact`

Every artifact identity uses exactly:

1. `algorithm`
2. `digest`

Every acquisition or derivation selector uses exactly the existing public
`ArtifactProvenanceAnchorSelectorV0` order:

1. `bundle_kind`
2. `witness_root`
3. `witness_algorithm`
4. `canonicalization_profile`
5. `run_id`

`authority` uses exactly:

1. `kind`
2. `reference`

No object has an extension member or arbitrary metadata member.

## 9. Identifier and replay-reference validation

### 9.1 Protocol-controlled identifiers

The exact grammar is:

```text
[A-Za-z0-9][A-Za-z0-9._:/-]{0,127}
```

It applies to:

- `binding_id`;
- `origin_admission_policy_id`;
- `origin_group`;
- `authority.kind`;
- `authority.reference`; and
- bundle `run_id`.

Each value is ASCII only, 1-128 bytes, exact and case-sensitive. These strings
have no prefix, hierarchy, path, inheritance, wildcard, timestamp or authority
semantics.

The `run_id` values inside acquisition and derivation selectors satisfy the
same existing artifact-provenance identifier contract. Their other fields are
validated under section 14.

### 9.2 Replayed Magpie references

These fields name existing replay material and do not use the restrictive
protocol-identifier grammar:

```text
target_claim_id:       1-1,024 UTF-8 bytes
source_evidence_id:    1-1,024 UTF-8 bytes
justification_edge_id: 1-1,024 UTF-8 bytes
scope_ref:             1-1,024 UTF-8 bytes
```

Comparison is exact. There is no Unicode normalization, case folding, path
interpretation, prefix matching, wildcard or semantic rewriting. Canonical
JSON escaping represents every permitted scalar exactly.

Origin-binding replay-reference syntax must not admit a value forbidden by
all accepted replay objects it could match. An empty `contribution.scope_ref`
therefore fails schema-specific semantic validation as
`invalid replay reference length`, before any replay lookup.

Syntactic validity does not establish existence. The verifier must
still revalidate exact same-replay structure.

## 10. Complete contribution identity

The exact binding subject is:

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

It is exact and contribution-scoped. It is not a publisher-global identity,
URL, domain, evidence-kind label, source descriptor, permanent origin identity
or aggregation lane.

From one accepted replay, the verifier must require:

```text
justification edge source_id
== contribution.source_evidence_id

justification edge target_id
== contribution.target_claim_id

typed claim scope_ref
== typed evidence scope_ref
== justification edge scope_ref
== contribution.scope_ref
```

The target claim, source evidence and justification edge must all exist in that
same replay. Naming unrelated existing IDs cannot manufacture a contribution.
The contribution artifact is the exact artifact on which this grouping
decision is grounded.

## 11. Derived origin-comparison namespace

No `origin_comparison_namespace` object is serialized.

The verifier derives exactly:

```text
OriginComparisonNamespaceV0 {
    origin_admission_policy_id
    target_claim_id
    scope_ref
}
```

from exactly:

```text
bundle.origin_admission_policy_id
bundle.contribution.target_claim_id
bundle.contribution.scope_ref
```

The namespace deliberately excludes:

- `source_evidence_id`;
- `justification_edge_id`;
- artifact identity;
- acquisition selector;
- derivation selector;
- `binding_id`;
- `run_id`; and
- complete `ContributionIdentityV0`.

Including those values would isolate each contribution and manufacture
apparent separation.

```text
same origin_group
+ same derived OriginComparisonNamespaceV0
-> same governed origin group

same textual origin_group
+ different namespace
-> no defined relationship
```

## 12. Origin-admission policy targeting

`origin_admission_policy_id` is required canonical bundle content. It is only
a claimed target-policy identity.

It does not select runtime code, negotiate policy, authorize caller selection,
prove that the policy exists, prove that the policy trusts the authority, or
become an ambient latest policy.

The reviewed origin-admission code selects one exact policy
implementation and requires exact equality with this field. A bundle requesting
a different or more favourable policy cannot change that selected
implementation. A verified statement may exist while a selected-policy
mismatch prevents admission.

## 13. Origin group and claimed authority semantics

### 13.1 Origin group

`origin_group` is one opaque exact policy key. It is not truth, publisher
identity, authorship, source quality, confidence, reputation, ownership,
organisational identity, statistical independence, causal independence or
authority.

It has no hierarchy, prefix, wildcard, path, containment, inheritance,
cross-policy, cross-claim or cross-scope semantics. Equality has meaning only
inside the derived `OriginComparisonNamespaceV0`.

### 13.2 Claimed authority path

The authority object contains exactly:

```text
kind
reference
```

It means only:

```text
the bundle asserts that this grouping decision was made
through authority path kind/reference
```

It does not mean the authority is trusted or authentic, the assignment is
admitted, the group is correct, or the contribution is independent.

The canonical bundle contains no `trusted`, `verified`, `approved`,
`independent`, `confidence`, `weight`, `score`, `priority`, `quorum` or
`authority_is_valid` field. It contains no signer public key, certificate or
trust root capable of bootstrapping its own authority.

`authority.kind`, `authority.reference`, and `origin_group` are
claimant-supplied assertions. Their canonical form, protocol syntax, exact
equality, occurrence in anchored bytes, provenance, and presence under the
compiled origin-admission pair do not independently authenticate or authorize
their issuer. A signer or authority named inside an exact canonical anchored
bundle remains untrusted claimant data. The current origin-admission policy's
exact match of that claim is compiled compatibility label selection, not
authentication or authorization. Authenticated grouping authority requires the
separate authority-binding object and explicit authority coordinate defined by
ADR-0007.

## 14. Artifact identity and exact provenance selectors

Every artifact identity is exactly:

```json
{"algorithm":"sha256","digest":"<64 lowercase hexadecimal characters>"}
```

`algorithm` must equal ASCII `sha256`. `digest` must be exactly 64 lowercase
hexadecimal characters. Equality is exact over both fields. Artifact identity
establishes exact bytes only; it does not establish acquisition, source,
publisher, authorship, origin or truth.

The acquisition selector requires exactly:

```text
bundle_kind: magpie-artifact-acquisition-v0
witness_algorithm: sha256
canonicalization_profile: magpie-artifact-provenance-json-v0
witness_root: exactly 64 lowercase hexadecimal characters
run_id: existing artifact-provenance identifier grammar
```

The derivation selector requires exactly:

```text
bundle_kind: magpie-artifact-derivation-v0
witness_algorithm: sha256
canonicalization_profile: magpie-artifact-provenance-json-v0
witness_root: exactly 64 lowercase hexadecimal characters
run_id: existing artifact-provenance identifier grammar
```

Selectors are untrusted exact references. Their presence does not prove the
referenced bundles are available, roots match, anchors exist, artifact bytes
match, or contexts verify.

## 15. Provenance coherence laws

The direct-acquisition family requires:

```text
verified acquisition receipt artifact identity
== contribution artifact identity
```

No derivation relationship is inferred or searched.

The direct-derivation family requires:

```text
verified acquisition receipt artifact identity
== verified derivation receipt parent artifact identity

verified derivation receipt derived artifact identity
== contribution artifact identity
```

No transitive lineage, graph search, alternate provenance path, closest match,
newest match, best match or first available prerequisite is accepted. The
bundle names the one acquisition selector and one direct-derivation selector
whose exact relationship must verify.

## 16. No rationale or arbitrary metadata

V0 contains no `rationale`, `notes`, `metadata`, `extensions`, `claims` or
free-form object. Rationale may exist in separate human review or
non-authoritative audit records.

The authoritative machine statement is only:

```text
which exact contribution
under which target policy
using which exact provenance basis
was assigned to which group
through which claimed authority path
under which exact binding run
```

## 17. Fixed canonical JSON profile and size limit

`magpie-origin-binding-json-v0` is a purpose-built fixed-schema profile. It is
not generic canonical JSON, RFC 8785/JCS, `magpie-core-v1`,
`magpie-artifact-provenance-json-v0`, or an extensible object profile.

Accepted bytes must:

- be valid UTF-8;
- have no UTF-8 BOM;
- contain exactly one top-level JSON object;
- have no leading or trailing bytes or whitespace;
- have no trailing newline;
- contain no duplicate key at any depth;
- contain every required field exactly once;
- contain no unknown field;
- contain no `null`, Boolean, number or array; and
- contain strings plus only the exact nested objects in sections 6-8.

Schema-defined object order is canonical; generic lexical sorting is not.
Parseable alternate whitespace reaches canonical equality and is
`NonCanonicalEncoding` rather than an accepted spelling.

Canonical string encoding is exactly:

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

There is no Unicode normalization, unnecessary escape, escaped solidus or
uppercase hexadecimal escape digit. Malformed escapes and lone or invalid
surrogate structures are invalid JSON. A valid escaped surrogate pair parses
but is noncanonical because the resulting scalar must be emitted directly as
UTF-8.

After parsing and semantic validation, the verifier re-encodes the
exact logical object and requires byte equality with the complete input. Every
alternate valid spelling is `NonCanonicalEncoding`.

The raw bundle limit is exactly 16,384 bytes, checked before UTF-8 or JSON
work. Exactly 16,384 bytes may proceed. One byte more is `binding bundle too
large`.

## 18. Root construction and selected binding anchor

For either family:

```text
canonical_bytes
= exact magpie-origin-binding-json-v0 bytes

witness_root
= lowercase_hex(SHA-256(canonical_bytes))
```

There is no v0 domain-separation prefix.

The selected binding anchor is the exact five-field tuple:

```text
bundle_kind = schema-selected origin-binding family
witness_root = computed lowercase SHA-256
witness_algorithm = sha256
canonicalization_profile = magpie-origin-binding-json-v0
run_id = exact bundle run_id
```

Bundle `run_id`, selector `run_id` and matching `SegmentAnchored.run_id` must
be exactly equal. All five selector fields must match one or more occurrences
in the accepted replay. Every exact occurrence remains audit-visible and does
not amplify verification.

`SegmentAnchored` establishes occurrence/inclusion only. It does not parse the
bundle, verify the root, revalidate the contribution, resolve provenance,
trust authority or admit a group.

## 19. Verifier deterministic precedence

This documentation-only contract implements no verifier; Ticket 0045
implements it separately. Its deterministic stage order is fixed:

1. exact binding-bundle availability from `ResolutionContentClosureV0`;
2. raw binding-bundle byte limit;
3. UTF-8, BOM, JSON and top-level-object validity;
4. duplicate-key detection;
5. required-field, unknown-field and JSON-type validation;
6. schema-specific semantic validation;
7. exact canonical re-encoding equality;
8. selected binding selector: bundle kind, witness algorithm,
   canonicalization profile, then `run_id`;
9. computed binding-root equality;
10. exact binding-anchor occurrence in the accepted replay;
11. exact contribution structural revalidation;
12. exact acquisition prerequisite resolution;
13. exact derivation prerequisite resolution for the derivation family;
14. exact cross-statement artifact coherence; and
15. one matched standing-inert origin-binding receipt.

Caller order, closure insertion order, replay occurrence multiplicity and
ambient storage do not change this order.

Prerequisite resolution must use exact closure lookup followed by the existing
reviewed artifact-provenance context surface. The verifier must preserve
the complete nested `ArtifactProvenanceContextTraceV0` whenever acquisition or
derivation is unavailable, malformed, unanchored, mismatched or otherwise
unsuccessful. It must neither duplicate the existing parser/failure vocabulary
nor reinterpret a failed nested trace as a match.

## 20. Minimum closed audit distinctions

Exact Rust enum spelling was deferred to the verifier implementation (Ticket
0045), but the audit must preserve at least these primary distinctions and may
not collapse them into a Boolean `verified` result.

### Binding bundle and parser

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

### Binding selector and replay occurrence

```text
binding bundle-kind mismatch
binding witness-algorithm mismatch
binding canonicalization-profile mismatch
binding run-ID mismatch
binding witness-root mismatch
binding anchor absent
```

### Contribution subject

```text
target claim absent
source evidence absent
justification edge absent
edge source mismatch
edge target mismatch
scope mismatch
contribution artifact mismatch
```

### Provenance prerequisites

```text
acquisition prerequisite unresolved
derivation prerequisite unresolved
acquisition artifact differs from contribution artifact
acquisition artifact differs from derivation parent
derivation output differs from contribution artifact
```

An unresolved prerequisite retains the exact nested existing provenance trace.

### Matched contexts

```text
matched acquisition-based origin binding
matched derivation-based origin binding
```

A matched context proves only an exact canonical, anchored, structurally and
provenance-coherent origin-binding assertion.

## 21. Receipt boundary

The private-construction `OriginBindingReceiptV0` must retain at least:

- verifier profile;
- exact binding anchor selector;
- exact schema/family;
- `binding_id`;
- `origin_admission_policy_id`;
- complete `ContributionIdentityV0`;
- derived `OriginComparisonNamespaceV0`;
- exact acquisition selector;
- the exact derivation selector for the derivation family and no derivation
  selector for the acquisition family;
- `origin_group`;
- `authority.kind` and `authority.reference`;
- binding `run_id`;
- bundle byte length;
- computed binding witness root;
- all exact binding-anchor occurrences;
- successful acquisition receipt; and
- successful derivation receipt for the derivation family.

The receipt must have private fields, getters only, no public constructor, no
`Deserialize`, and serializable audit output only. No API may accept a caller-
created receipt as authority. A receipt-level optional derivation member may
represent the already-selected family; it does not make either bundle wire
schema optional or nullable.

Prominent authority law:

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

Only a separately selected explicit versioned origin-admission policy may
decide whether `authority.kind + authority.reference` is trusted. Under the
current v0 policy that decision is compiled compatibility label selection: it
authenticates no issuer and grants no corroboration authority. Corroboration
authority requires the separate ADR-0007 authority boundary — a distinctly
owned authority-binding object verified under explicit `H + P + M + A`
coordinates.

## 22. Duplicate and conflict doctrine

### Exact duplicate assignment

Two exact verified bindings with the same complete `ContributionIdentityV0`,
same derived `OriginComparisonNamespaceV0`, same `origin_group` and same target
origin-admission policy are duplicate occurrences. They remain individually
audit-visible, produce at most one assignment, do not amplify and need not
constitute a conflict.

### Conflicting assignment

Two verified bindings with the same complete `ContributionIdentityV0`, same
derived `OriginComparisonNamespaceV0` and same target policy but different
`origin_group` values produce an explicit unresolved conflict.

Both remain visible. No group is admitted and they provide zero corroboration
separation. There is no first-write-wins, last-write-wins, lexical-minimum
group, higher-chain-sequence preference, later-looking-run-ID preference,
human-looking-authority preference or inferred supersession.

Supersession, revocation, expiry and temporal ownership remain future
contracts.

### Same-origin grouping across contributions

Two distinct contribution identities may map to the same group in the same
namespace. This is the intended same-origin mechanism, not duplication or
conflict.

### Potential corroboration separation

Two distinct contributions with different admitted groups in one namespace
may later provide policy-recognised corroboration separation. This contract
admits neither group and creates no support. Under the current v0 path those
groups originate from claimant-label compatibility admission, so any
separation they supply is claimant-label compatibility corroboration, not
authenticated organisational, causal, statistical, or editorial independence.
Authority-bound corroboration separation requires the additive ADR-0007
successor path.

## 23. Normative canonical vectors

These vectors freeze exact origin-binding canonical bytes and roots. A root or
selector never proves that referenced bundle bytes exist.

Two independently structured local calculations produced each vector: one
used an insertion-ordered structured object with compact UTF-8 JSON emission;
the other used a separately written fixed-schema encoder and manual string
escaping. They reproduced identical byte strings, lengths, roots, endpoint
bytes, BOM absence and newline absence.

### 23.1 Direct-acquisition binding vector

This vector reuses the committed acquisition fixture selector and exact
artifact identity.

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

Exact resulting binding anchor selector:

```text
bundle_kind: magpie-origin-binding-acquisition-v0
witness_root: 4f4bbaab87c1cfdc4dfe3b8c0d8b11a04bdc637de1e8e1481001b09543f0a4d7
witness_algorithm: sha256
canonicalization_profile: magpie-origin-binding-json-v0
run_id: run-origin-binding-acq-0001
```

### 23.2 Direct-derivation binding vector

The explicit hypothetical coherence relationship is:

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
calculated 294-byte canonical acquisition statement:

```json
{"schema":"magpie-artifact-acquisition-v0","acquisition_id":"acq-alpha-0001","acquisition_profile":"magpie-import-v0","artifact":{"algorithm":"sha256","digest":"b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060"},"observed_locator":"file:alpha.txt","run_id":"run-acq-alpha-0001"}
```

Its root is
`94593dd609cb44b07c2f3dd5a72226ff4c02ad4b9f4378a84d4432c433a1f7ce`.
The acquisition statement and selector are hypothetical and are not committed
fixtures.

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

Exact resulting binding anchor selector:

```text
bundle_kind: magpie-origin-binding-derivation-v0
witness_root: 24aa77f2f601e70f6d5d6044f0b1274394f164276df07296be16ab59953a5a29
witness_algorithm: sha256
canonicalization_profile: magpie-origin-binding-json-v0
run_id: run-origin-binding-der-0001
```

This vector freezes the origin-binding bundle bytes and root. It is not an
end-to-end verified provenance fixture and does not assert that the
hypothetical acquisition bundle, its anchor, the binding bundle, or its anchor
exists in any accepted replay or closure.

## 24. Hostile examples

| hostile case | required fail-closed result |
| --- | --- |
| A bundle names `trusted: true`. | Unknown field or wrong schema; no authority. |
| A bundle names its own public key as authority. | At most an opaque claimed reference; no trust bootstrap. |
| A bundle targets a policy ID different from the reviewed selected policy. | A verified statement may exist, but policy mismatch prevents admission. |
| A contribution names unrelated claim, evidence and edge IDs. | Structural subject mismatch. |
| Claim, evidence and edge exist with different scopes. | Scope mismatch. |
| `contribution.scope_ref` is empty. | `invalid replay reference length` during semantic reference validation → no replay lookup → no matched origin-binding receipt. |
| A direct-acquisition contribution artifact differs from the verified acquisition artifact. | Acquisition artifact differs from contribution artifact. |
| A derivation binding's verified acquisition artifact differs from the derivation parent. | Acquisition artifact differs from derivation parent. |
| A derivation binding's contribution artifact differs from the derivation output. | Derivation output differs from contribution artifact. |
| The binding bundle is canonical and root-correct but unanchored. | Binding anchor absent. |
| The binding anchor exists but exact bytes are absent from closure `M`. | Binding bundle unavailable; no ambient lookup. |
| A nested acquisition or derivation bundle is absent from `M`. | Exact nested `ArtifactProvenanceContextTraceV0` remains unresolved. |
| A nested selector differs by one field. | Exact lookup or profile verification fails; no fallback. |
| A valid binding references a different acquisition that happens to name byte-identical content. | No selector substitution or provenance search. |
| The same group key appears under another policy, claim or scope. | No defined cross-namespace relationship. |
| Two exact bindings assign one contribution to different groups. | Later explicit conflict and no admission. |
| Two exact bindings assign different contributions to one group in the same namespace. | Valid same-origin grouping, not conflict. |
| Two different groups are claimed in one namespace. | Only potential future separation after explicit admission; no support now. |
| A run ID looks like a timestamp. | Exact identifier only; no temporal authority. |
| A higher chain sequence appears later. | No implicit supersession. |
| A source descriptor, URL, publisher, signature count, model confidence or distinct artifact digest is used as an origin group without governed binding. | No admitted origin. |

## 25. Explicit non-goals and frozen surfaces

This contract implements no Rust, fixture file, parser, encoder, verifier,
receipt type, origin-admission policy, trusted-authority table, conflict fold,
support rule, standing rule, policy v3, aggregation rule, threshold, confidence
score, source reputation, publisher ontology, identity ontology, supersession,
revocation, expiry, temporal ownership, loader, CAS, network or filesystem
lookup, callback, new L0 payload, writer or Deadbolt code change.

It does not change or reinterpret `magpie-core-v1`, existing payload tags,
`SegmentAnchored`, Deadbolt occurrence semantics, acquisition or derivation
schemas/vectors/roots/verifier semantics, `ArtifactProvenanceAnchorSelectorV0`,
`ArtifactProvenanceContextTraceV0`, `ResolutionContentClosureV0`, policies
v0/v1/v2, snapshots, standing resolution bytes, fixtures, release metadata,
Cargo manifests, dependencies, CI or historical Tickets 0037-0041.

## 26. Reviewer checklist

- Confirm exactly two distinguishable closed binding families exist.
- Confirm every identity string and every top-level and nested field order is
  exact.
- Confirm replay-reference syntax cannot admit a value forbidden by every
  accepted replay object it could match.
- Confirm no selector is optional or nullable in either wire schema.
- Confirm `origin_admission_policy_id` is target data, not runtime selection.
- Confirm `ContributionIdentityV0` is exact and same-replay structural
  revalidation is mandatory.
- Confirm `OriginComparisonNamespaceV0` is derived, not serialized, and omits
  contribution-unique values.
- Confirm origin-group equality has meaning only inside that namespace.
- Confirm authority fields remain claimed and untrusted.
- Confirm no rationale, metadata, extension or self-trusting field exists.
- Confirm nested provenance selectors use the existing exact five-field tuple
  and no substitution or search.
- Confirm canonical string rules, byte limit, root formula and five-field
  binding anchor are exact.
- Confirm the verifier preserves nested
  `ArtifactProvenanceContextTraceV0` values.
- Confirm a matched receipt is a verified assertion only, not admission,
  support, aggregation input or standing.
- Confirm duplicate, conflict, same-origin grouping and potential separation
  remain distinct.
- Confirm both literal vectors reproduce exact lengths, roots, endpoint bytes,
  BOM absence and newline absence under two independent calculations.
- Confirm the derivation vector is not described as an end-to-end fixture.
- Confirm no runtime capability or historical-contract change is claimed.

## 27. Historical next slice

Ticket 0042 selected as its exact next slice only:

```text
standing-inert origin-binding verifier
+ exact ResolutionContentClosureV0 lookup
+ exact binding parser/canonicalizer/root/anchor checks
+ exact contribution revalidation
+ composition through existing artifact-provenance context traces
+ private matched receipt and hostile tests
```

That slice had to add no origin-admission policy, trusted-authority decision,
conflict fold, admitted-contribution audit, support, standing, policy v3,
aggregation, loader, CAS, filesystem/network access, L0 payload or writer
authority. It subsequently landed separately as Ticket 0045 (PR #58).
