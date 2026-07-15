# Artifact Provenance and Origin Admission

## 1. Status

Ratified architecture contract. Ticket 0037 is landed.

This note defines doctrine and implementation order only. Ticket 0037 is
documentation-only. [Ticket 0038](../../tickets/0038-artifact-acquisition-derivation-bundle-v0.md)
and
[`artifact-acquisition-derivation-bundle-v0.md`](artifact-acquisition-derivation-bundle-v0.md)
now pin the proposed first exact acquisition/direct-derivation protocol; they
also implement no runtime capability. No artifact, acquisition, derivation,
origin-binding, origin-admission, contribution, aggregation, or writer
capability is implemented by these contracts.

Ticket 0040 ratified the separate immutable resolution-content-closure
contract, and Ticket 0041 implements its bounded construction, canonical
manifest, typed identity and exact read-only lookup. That availability surface
remains standing-inert and untrusted. No loader, orchestrator wiring or
verifier integration exists.

[Ticket 0042](../../tickets/0042-origin-binding-bundle-v0.md) and
[`origin-binding-bundle-v0.md`](origin-binding-bundle-v0.md) are this slice.
They ratify the exact two-sibling-family origin-binding statement contract.
They add no parser, verifier, origin admission, authority trust, standing or
writer capability. Origin-binding verification and origin admission remain
future work.

## 2. Purpose

Magpie can retain structurally valid evidence and exact foreign-bundle anchors.
It cannot yet establish the authority path between:

```text
evidence is structurally present
```

and:

```text
this exact contribution may count as belonging to one admitted origin group
```

That missing path must be explicit before support aggregation. Otherwise a
producer could manufacture apparent corroboration with fresh metadata, URLs,
digests, signatures, publisher labels, or duplicated reports.

This contract freezes the distinctions needed to prevent that authority
transposition:

```text
Magpie event provenance
!= external artifact provenance

artifact identity
!= acquisition identity

acquisition identity
!= source identity

source descriptor
!= admitted origin

derivation lineage
!= independent corroboration

distinct bytes
!= distinct origin

distinct publishers
!= necessarily distinct origin

distinct signatures
!= necessarily distinct control

verified provenance
!= truth

origin admission
!= support contribution

support contribution
!= aggregation

aggregation
!= settlement
```

The positive construction law is:

```text
content addressing establishes exact artifact identity

a governed acquisition bundle establishes acquisition history

an exact derivation relation establishes lineage

a governed origin-binding bundle may assign one exact contribution path
to one opaque origin group

only a later explicit standing policy may count admitted origin groups
```

## 3. Current substrate

The current L0 vocabulary retains signed typed claims, typed evidence,
justification edges, optional `content_hash` strings, opaque
`metadata_json`, and exact `SegmentAnchored` identities. Verified replay
establishes Magpie event integrity, order, inclusion, and provenance relative
to the caller's externally selected Magpie verifying key. Policy v0 is
candidate-only; policy v1 settles one exact Deadbolt occurrence proposition;
policy v2 directly supports one exact inline SHA-256 proposition.

None of those surfaces establishes which external bytes were acquired, which
governed action acquired them, whether one artifact derives from another, who
controls an external source, or whether two contributions may count as
separate corroborators.

In particular, the existing event field:

```rust
Provenance {
    agent,
    source,
}
```

records who or what produced a Magpie event. It does not prove an external
publisher, remote server identity, authorship, DOI ownership, repository
ownership, upstream evidentiary origin, independence, control separation, or
artifact truth. This contract does not reinterpret event provenance as
external source provenance.

Likewise, `EvidenceRegistered.content_hash` is asserted string material at
L0. Existing values are not retroactively promoted into verified artifact
identities. The policy-v2 `sha256_bytes_equals_v0` rule checks inline witness
bytes under one exact proposition; it does not establish external artifact
identity or acquisition history.

## 4. Missing authority seam

The current aggregation note correctly rejects self-declared independence
metadata. It does not yet define the replayable mechanism that can authorize an
origin assignment.

The missing seam has four separate questions:

1. What exact artifact bytes are in question?
2. What governed acquisition or explicit transformation relates those bytes to
   Magpie's evidence?
3. What observed or asserted source descriptors are available for governance
   review?
4. Under what explicit policy and authority may one exact contribution path be
   assigned to one opaque origin group?

No answer may be inferred from the answer to another question. Exact bytes do
not prove acquisition, acquisition does not prove source identity, a source
descriptor does not admit an origin, and an admitted origin does not create a
support contribution.

## 5. Architectural decision

The first implementation will reuse the anchored hierarchy:

> Governed artifact-acquisition, derivation and origin-binding authority will
> be represented by explicit versioned foreign bundles sealed and verified at
> the governed boundary, then committed to Magpie through the existing
> `SegmentAnchored` occurrence/inclusion seam.

Consequences:

- No new Magpie L0 payload tag is proposed for the first implementation.
- Acquisition, derivation, and origin-binding statements use distinguishable,
  explicitly versioned foreign bundle families.
- Each supported family has an explicit canonicalization and verifier profile.
- Ticket 0038 pins exact bundle-kind strings, wire schemas, SHA-256
  artifact/root algorithms, canonical bytes, and one standing-inert verifier
  profile for the acquisition/direct-derivation sibling families.
- Ticket 0042 separately pins exact direct-acquisition-binding and
  direct-derivation-binding sibling schemas. Neither origin-binding schema
  changes the referenced provenance schemas or makes a provenance selector
  optional.
- A bare `bundle_kind` string grants no authority.
- `SegmentAnchored` proves only that an exact foreign bundle identity occurred
  in the accepted Magpie chain at one chain position.
- `SegmentAnchored` does not verify foreign contents.
- The bundle's profile-specific verifier establishes its exact contents
  relative to the named profile and matched anchor.
- Later origin-admission policy decides whether a verified origin-binding
  statement may influence grouping.
- Later standing policy decides whether an admitted group may affect standing.

Only explicitly selected, versioned, root-matched, and verified bundle families
may participate. This decision is not permission to interpret arbitrary
anchored bundles and does not authorize a rule such as:

```text
any anchored bundle with metadata_json.origin_group
-> admitted origin
```

That would transpose occurrence authority into origin authority.

The anchored hierarchy remains optional for portable Magpie core. A governed
Deadbolt integration may seal, verify, and anchor these foreign families, but
Magpie core must remain testable with portable fixtures and without a live
Deadbolt installation. Deadbolt governs execution and admission and records
evidence; it does not become a truth oracle.

## 6. Vocabulary and identity boundaries

### 6.1 Artifact identity

An artifact identity names exact bytes under an explicit digest algorithm. It
is content identity, not a filename, URL, title, DOI, MIME type, database row,
semantic-document identity, acquisition identity, or source identity.

Different acquisitions of byte-identical material have one artifact identity
and multiple acquisition occurrences. Different byte strings are not
automatically different origins.

The first implementation must use an exact versioned representation before
using artifact identity as verified context. Ticket 0038 proposes exact
`algorithm = "sha256"` identities for only its acquisition/direct-derivation
v0 families. That local selection does not reinterpret historical
`content_hash` values or create a global algorithm registry.

### 6.2 Acquisition occurrence

An acquisition occurrence records that a governed acquisition profile obtained
or imported one exact artifact. A future acquisition bundle must bind at
minimum:

- a versioned statement schema;
- an explicit acquisition profile;
- the exact artifact identity;
- an exact acquisition or run identity;
- the observed locator or import source;
- a chain-relevant occurrence identity; and
- any bounded transport observations that the profile chooses to record.

An acquisition occurrence proves neither artifact truth nor publisher
identity. Two crawlers acquiring the same bytes create two acquisition
occurrences, not two independent sources.

### 6.3 Derivation relation

An exact derivation relation binds:

- an exact parent artifact identity;
- an exact derived artifact identity;
- an explicit transformation profile; and
- an exact run identity.

Examples include PDF text extraction, HTML normalization, OCR, transcription,
translation, summarization, keyframe extraction, dataset filtering, and model
analysis generated from source material.

A derivative does not become a new corroborating origin merely because it has
different bytes. The first version admits no transitive inheritance, fuzzy
derivation, semantic similarity, or inferred lineage. Every authoritative
relation is explicit.

### 6.4 Source descriptor

A source descriptor is observed or asserted material that may inform a future
governance decision. Examples include a URL, DOI, repository and commit,
publisher name, claimed author, feed identifier, archival identifier, response
header, or signing identity.

A descriptor is not authority by itself and does not prove ownership,
authorship, control, authenticity, independence, or truth. The first version
does not attempt a universal publisher, author, or organisation identity
ontology.

### 6.5 Contribution identity

Origin admission is scoped to one exact contribution path. The conceptual
`ContributionIdentity` binds at least:

- the exact target claim identity;
- the exact source evidence identity;
- the exact justification edge identity;
- the exact `scope_ref`; and
- the relevant exact artifact identity where applicable.

The governing law is:

```text
origin grouping is claim/contribution scoped

not:

publisher X always equals origin group Y for every claim
```

This permits governance to distinguish an organisation's independent
investigation, copied press release, and syndicated report on a
proposition-by-proposition basis. The first policy permits no prefix, wildcard,
containment, inheritance, implicit reuse, or publisher-global assignment.

### 6.6 Origin comparison namespace

An `OriginComparisonNamespace` is the exact namespace within which origin-group
keys assigned to multiple distinct contributions may be compared. It is not the
subject of a binding. The minimum namespace for the first origin-admission
policy includes:

- the exact origin-admission policy identity;
- the exact target claim identity; and
- the exact `scope_ref`.

The exact origin-binding v0 contract serializes no namespace object. It derives
`OriginComparisonNamespaceV0` from the bundle's
`origin_admission_policy_id`, `contribution.target_claim_id` and
`contribution.scope_ref`. This avoids duplicated claim or scope fields that
could disagree.

A later explicit aggregation policy may further partition comparison through a
separately defined aggregation-lane identity, such as direction or an
evidence-policy cell. This contract does not define those future lane fields.

The namespace must not include the source evidence identity, justification-edge
identity, artifact identity, acquisition occurrence, or complete
`ContributionIdentity` when doing so would place every contribution in a unique
namespace and make comparison impossible.

The governing distinction is:

```text
ContributionIdentity
= the exact subject of one origin-binding statement

OriginComparisonNamespace
= the namespace within which origin-group keys from multiple
  distinct contributions may be compared

origin bindings are contribution-scoped

origin-group comparison is namespace-scoped

multiple distinct ContributionIdentity values may map to the same
origin-group key inside one OriginComparisonNamespace
```

### 6.7 Origin group

An origin group is an opaque exact policy key used to prevent several admitted
contribution paths that share one governed origin from being counted
repeatedly.

It is not a truth label, reputation score, source-quality score, publisher
identity, confidence value, statistical proof of independence, ownership
certificate, or authority key. Equality is exact.

An origin-group key has meaning only inside one exact
`OriginComparisonNamespace`. Equality is interpreted across separately bound
contributions in that namespace, not confined to one contribution subject:

```text
same group key
+ same origin-comparison namespace
= same admitted origin group

same group key
+ different origin-comparison namespace
= no defined relationship
```

A byte-identical key has no cross-policy, cross-claim, or cross-scope meaning
when those inputs select different namespaces.

### 6.8 Origin-binding statement

A governed origin-binding statement assigns one exact contribution identity to
one opaque origin group inside one derived origin-comparison namespace under
one claimed target policy and authority path.

[`origin-binding-bundle-v0.md`](origin-binding-bundle-v0.md) now ratifies two
closed sibling shapes: direct-acquisition binding and direct-derivation
binding. Both carry complete `ContributionIdentityV0`, exact provenance
selectors, `origin_admission_policy_id`, `origin_group`, claimed
`authority.kind`/`authority.reference`, and exact binding `run_id`. The
namespace is derived rather than serialized. V0 contains no rationale,
arbitrary metadata, optional selector or trust Boolean.

Exact canonical and anchored verification will establish only a verified
governance assertion. It will not trust the claimed authority, admit the
origin group, make evidence or claims true, or create support.

### 6.9 Admitted origin result

An admitted origin result is a replay-derived policy outcome, not caller input.
A later policy may admit one group only after:

- the contribution path is structurally revalidated;
- the relevant exact artifact identity is present;
- required artifact bytes are available and hash correctly;
- the acquisition bundle is present, anchored, and profile-verified;
- every required derivation relation is present, anchored, and
  profile-verified;
- the origin-binding bundle is present, anchored, and profile-verified;
- the binding targets the exact contribution identity and origin-comparison
  namespace;
- the binding authority is trusted under an explicit origin-admission policy;
  and
- no unresolved conflicting binding exists.

Admission is still not a support contribution. It authorizes only an exact
grouping statement for one contribution inside the named comparison namespace
under the named policy.

## 7. Anchored foreign-bundle model

The first implementation separates evidence-bearing and governance-bearing
families:

```text
acquisition bundle
    records governed acquisition observations

derivation bundle
    records exact parent/derived lineage

origin-binding bundle
    records one governance assignment for one contribution
```

These families require distinguishable versioned `bundle_kind` values,
schemas, and verifier profiles. A bundle must not establish its own acquisition
history and authorize its own origin grouping without an external policy
boundary.

For every participating bundle:

1. the governed boundary seals and verifies exact bundle bytes under the named
   profile;
2. the exact root and occurrence identity are committed through
   `SegmentAnchored`;
3. Magpie replay derives the anchor identity from the accepted chain;
4. untrusted bundle bytes are root-matched and checked with the selected
   profile-specific verifier; and
5. a separately named policy decides whether the verified statement is
   relevant to acquisition, lineage, or origin admission.

Unknown bundle kinds, unsupported profiles, malformed statements, root
mismatches, and valid-but-unanchored bundles fail closed. A bundle cannot name
its own signer or profile and thereby bootstrap trust.

The existing anchor representation is a frozen compatibility constraint.
`SegmentAnchored.witness_root` accepts exactly 64 lowercase hexadecimal
characters, corresponding to 32 root bytes. Therefore:

```text
Any first foreign-bundle verifier profile reusing SegmentAnchored must
produce a root represented as exactly 64 lowercase hexadecimal characters,
corresponding to 32 root bytes.
```

Ticket 0038's proposed first profile names the exact `witness_algorithm` value
`sha256` and uses the required representation. That selection is local to the
two proposed v0 sibling families. A scheme that requires a different root width
or encoding cannot reuse the existing seam unchanged; it requires a future ADR
and potentially an explicit L0 evolution decision.

## 8. Artifact and bundle storage

The storage topology is:

```text
large artifact bytes
-> external content-addressed storage

foreign acquisition/derivation/origin-binding bundles
-> external content-addressed storage

exact bundle roots and occurrence identities
-> Magpie L0 through SegmentAnchored

verified contexts, indexes and admission results
-> regenerable projections
```

The honest consequence is:

- Magpie L0 alone proves anchored occurrence and chain order.
- Interpreting a foreign bundle requires the exact bundle bytes and its
  verifier.
- Missing bundle bytes leave that context unresolved.
- Missing artifact bytes prevent artifact-grounded origin admission and provide
  no origin-sensitive aggregation input. The structurally recorded evidence
  remains visible as unresolved context and may remain relevant to a different
  future policy that does not require artifact verification.
- L0 alone cannot reconstruct foreign bundle contents.
- A later archive or export mechanism may package the log and all referenced
  content-addressed objects, but that work is outside Ticket 0037.

The external stores are not hidden sources of authority. Bytes from them are
untrusted inputs until exact identity and root checks succeed against the same
verified replay and the explicitly selected policies.

## 9. Same-snapshot verification and resolution content closure

The contract extends the existing trusted-construction pattern:

```text
one completely verified log replay
+ exact anchored foreign-bundle identities
+ untrusted externally supplied bytes
+ profile-specific verification
= possible trusted derived context
```

Callers may supply untrusted artifact and bundle bytes from content-addressed
storage. They may not supply a trusted acquisition result, trusted origin
binding, admitted origin result, or authority Boolean.

Ticket 0040 and
[`resolution-content-closure-v0.md`](resolution-content-closure-v0.md) ratify
the exact immutable `ResolutionContentClosureV0` construction, canonical
manifest and identity contract. Ticket 0041 implements production construction
from explicit borrowed key/byte inputs and exact immutable lookup. There is no
loader, parser, orchestrator wiring or resolver integration.

The closure is an immutable, finite, explicitly identified set of untrusted
artifact bytes and foreign-bundle bytes supplied to one resolution attempt. It
contains exactly two disjoint key namespaces:

- artifact objects are keyed by exact two-field expected identity
  (`algorithm`, `digest`); and
- foreign-bundle objects are keyed by the full five-field
  `ArtifactProvenanceAnchorSelectorV0` order: `bundle_kind`, `witness_root`,
  `witness_algorithm`, `canonicalization_profile`, `run_id`.

The ratified schema is `magpie-resolution-content-closure-v0`; its separate
canonicalization profile is `magpie-resolution-content-closure-json-v0`; and
its digest algorithm is exact `sha256`. The exact typed
`ResolutionContentClosureIdentityV0` contains exactly four fields in normative
order: `schema`, `canonicalization_profile`, `digest_algorithm` and
`manifest_sha256`. The final field is lowercase SHA-256 over the exact
purpose-built canonical manifest. The complete four-field value, not the bare
64-character manifest digest, is the closure identity. No second hash over the
typed identity is introduced.

That typed identity commits to exact keys, object lengths and actual object
SHA-256 values in the manifest. It is not an artifact identity, bundle
verification, trust root, authority registry, mutable CAS view, network
namespace, evidence by itself, archive guarantee or permission to fetch
additional material. The manifest omits raw object bytes, so neither the
manifest nor its typed identity can reconstruct the closure. A caller-created
well-shaped identity is not proof that reviewed construction occurred.

A bare filename, URL, locator or storage path is not a closure key. Supplying an
object does not verify it. Construction may retain expected key material that
disagrees with the actual supplied-object SHA-256; the manifest records both
without equating them. Every object remains untrusted until the existing or a
later selected verifier checks its digest or root, profile and same-replay
anchor.

For foreign-bundle objects, manifest `content_sha256` is always the closure-
profile SHA-256 commitment to the exact supplied bytes, regardless of the
selector's `witness_algorithm`. It is not a generic computed witness root. A
selected verifier independently computes its profile-specific root and
compares that result with `witness_root`. Only the current artifact-provenance
v0 profile, after accepting exact canonical bytes, computes SHA-256 over the
same byte string and therefore obtains a root equal to `content_sha256`.

Construction limits the checked sum of every supplied entry length before
exact-duplicate collapse to 536,870,912 bytes, so every duplicate occurrence
consumes the pre-collapse budget. Entry counts and supplied bytes jointly bound
pre-collapse work. Because duplicate collapse can only remove occurrences, the
derived v0 invariant is `retained object bytes <= supplied object bytes <=
536,870,912`; v0 has no independent post-collapse byte cap or post-collapse
construction failure.

The deterministic input law is:

```text
deterministic provenance/origin resolution input
= verified log prefix H
+ explicit policy identities P
+ immutable resolution content closure M

same H + same P + same M
-> same derived result
```

The future output must disclose or bind the verified log-tip or prefix identity,
every applicable policy identity, and the resolution-content-closure identity.
No ambient CAS lookup, filesystem scan, remote fetch, callback, network lookup,
plugin invocation, database query, second Magpie replay, lazy population,
cache fill or background hydration may occur in closure construction or inside
deterministic resolution. A separate future loader may populate a closure
before resolution; once constructed, the closure is finite and immutable.

An object absent from closure `M` is unavailable for that resolution. The
resolver emits the relevant explicit unavailable outcome and does not search
ambient storage. Adding an object constructs a different closure `M2`; `M2`
may resolve material that remained unresolved under `M1`. That is a new
resolution input, not mutation or rewriting of the result for `(H, P, M1)`.
CAS bytes remain untrusted inputs, so freezing availability does not make the
external store a second authority system.

Trusted context may be constructed only after artifact bytes match the exact
artifact identity carried by a verified bundle and each supplied bundle root
matches an exact anchor in the same completely verified replay. A foreign
bundle that is otherwise valid but unanchored is outside Magpie's record. A
bundle anchored only after historical tip H cannot influence a resolution as
of H. A later resolution over the same historical log prefix may resolve
previously unavailable material only by using an explicitly different
resolution content closure. If the verified log tip also changes, both the new
prefix and the new closure remain visible in the resolution input. Neither case
rewrites the result for an earlier `(H, P, M)` tuple.

Existing v0, v1, and v2 policies and snapshot/resolution bytes remain frozen.
The first implementation may reuse the existing same-snapshot anchor index, but
must not mutate or silently widen those derived-byte surfaces. Any new snapshot
or context type required later must be separately versioned. Its public API
shape is future work.

## 10. Contribution-scoped origin admission

An origin-admission attempt begins with one exact `ContributionIdentity`, not
with a publisher, domain, URL, or artifact collection. The policy re-fetches
the target claim, source evidence, edge, scope, and artifact references from
one verified snapshot and compares them exactly with the origin-binding
statement. It also derives and matches the exact `OriginComparisonNamespace`
selected by the policy, target claim, and `scope_ref`.

The conceptual flow is:

```text
exact contribution identity
+ exact origin-comparison namespace
+ verified artifact identity
+ verified acquisition and required derivation lineage
+ exact verified origin-binding statement
+ explicit trusted authority under one origin-admission policy
+ no unresolved conflict
-> one admitted origin group
```

One binding never implicitly binds another contribution across claims, scopes,
edges, artifacts, policies, or time. That exact-subject rule does not isolate
group comparison: separately verified bindings for multiple distinct
`ContributionIdentity` values may assign the same origin-group key inside one
exact `OriginComparisonNamespace`.

The coherent derivation chain is:

```text
exact contribution
+ exact origin-comparison namespace
+ verified binding
-> one policy-scoped group assignment

many contribution assignments in one namespace
-> comparable origin-group keys

verified log prefix
+ policy identities
+ immutable content closure
-> reproducible origin-admission audit
```

Supersession, revocation, expiry, and temporal ownership change are future
semantics. Until those semantics exist, a conflict cannot be repaired by
preferring a newer-looking statement.

## 11. Trust roots and authority

Foreign verifier profiles and trust roots must be explicit and versioned. No
ambient mutable trust registry participates.

The following laws apply:

- A signer or key named inside a bundle cannot bootstrap its own authority.
- Self-signing proves integrity relative to that key, not trusted identity.
- A bundle kind, actor class, metadata field, URL, domain, filename, signature
  count, or model confidence grants no authority.
- There is no `latest` acquisition, origin-admission, or standing policy.
- Runtime callers cannot select a more favourable policy.
- A profile name selects only explicitly reviewed verification behavior; it
  does not confer source authority.
- Human authority may govern contribution grouping but cannot settle factual
  truth.
- Deadbolt may govern execution/admission and record evidence, but does not
  decide content truth or source independence.
- Verified provenance establishes only the exact proposition verified under
  the selected roots and profiles.

Policy choice must be explicit in versioned code or a separately ratified
contract. Log-carried data and caller inputs cannot negotiate authority.

## 12. Duplicate and conflict handling

### Exact duplicate binding

Two exact verified bindings that assign the same exact contribution identity to
the same exact origin-comparison namespace and origin group under the same
policy:

- remain visible as duplicate occurrences;
- do not create two groups;
- do not amplify; and
- do not necessarily create a conflict.

The later audit may retain every occurrence while deriving one exact assignment.

### Conflicting binding

Two verified bindings that assign the same exact contribution identity to
different origin groups under the same policy and origin-comparison namespace:

- both remain visible;
- produce an explicit conflict outcome;
- admit no origin group;
- contribute zero corroboration separation; and
- never use first-write-wins, last-write-wins, lexical minimum, or silent
  preference for the newer or more human-looking statement.

Supersession, revocation, expiry, and temporal ownership change are future work.
Until then, unresolved conflict fails closed.

### Intended same-origin grouping

Two distinct contribution identities assigned the same admitted origin-group
key inside the same origin-comparison namespace are not duplicates and are not
in conflict. They remain two exact contribution assignments but are treated as
sharing one origin. A later aggregation policy may count that admitted group at
most once.

For example:

```text
Contribution A -> group wire-report-17
Contribution B -> group wire-report-17
```

under the same policy, target claim, and `scope_ref` namespace is valid
same-origin grouping.

### Potential corroboration separation

Two distinct contribution identities assigned different admitted origin groups
inside the same origin-comparison namespace may provide policy-recognised
corroboration separation. This is not proof of statistical independence, and
only a later explicit aggregation policy may consume it.

For example:

```text
Contribution A -> group wire-report-17
Contribution C -> group field-report-4
```

under the same namespace describes potential separation but creates no support
by itself. If `wire-report-17` appears under another claim, scope, or policy
namespace, the byte-identical group key has no defined relationship to either
assignment above.

## 13. Conceptual audit outcomes

The future audit surface must use a closed, stage-specific vocabulary. Exact
Rust enum names are deferred, but the following distinctions may not be
collapsed into Boolean `verified` or `independent` fields.

| stage | required outcome | meaning and fail-closed effect |
| --- | --- | --- |
| artifact | no artifact identity | no exact subject; no admission |
| artifact | artifact bytes unavailable | identity may be asserted, but content cannot be checked; structural evidence remains visible and unresolved, with no artifact-grounded origin admission or origin-sensitive aggregation input |
| artifact | artifact digest mismatch | supplied bytes are not the named artifact; no admission |
| acquisition | acquisition anchor absent | bundle is outside Magpie's record; no admission |
| acquisition | acquisition bundle unavailable | occurrence remains visible; interpretation unresolved; no admission |
| acquisition | acquisition bundle malformed | statement cannot be interpreted; no admission |
| acquisition | acquisition verifier/profile unsupported | no selected checker; no admission |
| acquisition | acquisition verification failed | root, signature, schema, or profile check failed; no admission |
| acquisition | acquisition verified | exact acquisition statement is verified; this is not origin admission |
| derivation | derivation absent when required | lineage prerequisite unresolved; no admission |
| derivation | derivation bundle unavailable | occurrence may remain visible; lineage unresolved; no admission |
| derivation | derivation bundle invalid | lineage rejected; no admission |
| descriptor | source descriptor only | asserted or observed descriptor remains audit material; no admission |
| binding | origin binding absent | no grouping authority; no admission |
| binding | origin-binding anchor absent | binding is outside Magpie's record; no admission |
| binding | origin-binding bundle unavailable | occurrence remains visible; interpretation unresolved; no admission |
| binding | origin binding malformed | exact subject/group/authority cannot be derived; no admission |
| authority | origin-binding authority untrusted | integrity may hold, but the authority is not admitted; no admission |
| subject | contribution-subject mismatch | binding targets different claim/evidence/edge/scope/artifact material; no admission |
| binding | exact duplicate binding | occurrences remain visible; one assignment at most; no amplification |
| binding | conflicting binding | explicit conflict; zero admitted groups |
| admission | origin admitted | one exact policy-scoped group is admitted; still no support contribution |

Unknown or newly encountered states must not fall through to `origin admitted`.
Later implementation may use a structured audit with several stage outcomes,
but every terminal decision must remain deterministic and attributable.
Unavailable outcomes are evaluated against the exact resolution content closure;
they do not authorize ambient lookup and do not erase structurally recorded
evidence.

## 14. Interaction with standing and aggregation

Tickets 0040 and 0041 land the first two steps of the corrected implementation
sequence:

```text
ResolutionContentClosureV0 contract — landed
ResolutionContentClosureV0 implementation — landed
exact origin-binding bundle contract — this slice
origin-binding verifier — next
standing-inert origin-admission audit
admitted-contribution audit
policy-v3 contract
conservative aggregation
```

Closure construction and exact lookup are implemented without a loader or
verifier wiring. The origin-binding wire contract is ratified by Ticket 0042;
its verifier, origin admission and every later step remain future work. The
admitted-contribution audit is a future standing-inert
explanation surface.
It will revalidate one exact policy-eligible contribution and associate it with
its admitted origin result while preserving every unresolved prerequisite. It
is derived deterministically from `(verified log prefix H, explicit policy
identities P, immutable resolution content closure M)`, reports potential
aggregation input, and does not create support or standing.

Policy v2 direct support is not aggregation. Origin admission is not support.
One admitted group is not aggregation. Missing, malformed, conflicting,
self-declared, or unverified origin material does not amplify. Unknown origin
counts as zero corroborating separation in the first policy. Same-origin
multiplicity within one origin-comparison namespace may count at most once under
a later aggregation policy. Group keys from different namespaces are
incomparable.

The normative terminology is:

```text
distinct admitted origin groups
= policy-recognised corroboration separation

distinct admitted origin groups
!= proof of statistical independence
```

The existing filename
`standing-aggregation-independence-groups.md` remains stable. “Independence
group” is earlier design shorthand; the normative opaque key is now
`origin group`, and the policy-recognised relationship between distinct
admitted groups is `corroboration separation`. Magpie does not prove
statistical, causal, institutional, organisational, or control independence.

No numeric aggregation threshold is selected here. No policy v3 is created or
implemented. “Two sources imply Supported” is not a rule. A later explicit
standing policy must define the exact eligible evidence cell, grouping law,
threshold, trace, precedence, and ceiling behavior.

External-source aggregation can never reach `Settled`. Support, refutation,
contradiction debt, invalidation, and supersession/currentness remain separate
policy concepts.

## 15. Hostile examples

| example | future fail-closed result |
| --- | --- |
| One wire report appears at 100 URLs. | Many acquisitions and byte-identical or derivative artifacts may exist. URLs create no corroboration and there is not automatically more than one origin group. |
| The syndicated report is represented by Contribution A and Contribution B, both admitted as `wire-report-17` under the same policy/claim/scope namespace. | This is valid same-origin grouping, not a conflict. The contributions remain distinct, and a later aggregation policy may count their shared group at most once. |
| Contribution A is admitted as `wire-report-17` and Contribution C as `field-report-4` in the same namespace. | The different admitted groups may provide corroboration separation, not statistical-independence proof or support without a later aggregation policy. |
| The key `wire-report-17` also appears under another claim or policy namespace. | The byte-identical key has no implied cross-namespace relationship. |
| Two governed crawlers acquire the same exact bytes. | One artifact identity, two acquisition occurrences, and no duplicate corroboration. |
| A paper exists as PDF, publisher HTML, and extracted text. | Possibly several artifacts with explicit lineage; no automatic separate origin. |
| Several outlets repeat one company press release. | Distinct publishers do not establish distinct origin. Without admitted bindings, contributions remain non-amplifying. |
| Reuters and AP publish different reports about one event. | Different publishers and bytes are insufficient. Separation requires admitted contribution-scoped bindings; unresolved material remains visible but non-amplifying. |
| One model summarizes a paper and another paraphrases the summary. | Both are derived analytical artifacts. Neither becomes independent external corroboration, and model-provider diversity grants no separation. |
| Two ML papers share a dataset, benchmark harness, or checkpoint. | Different papers may share upstream dependencies. Lineage may expose overlap, but the first policy infers no causal independence. |
| Two statements use different signing keys controlled by one actor. | Signature diversity does not prove control separation and creates no automatic additional group. |
| A valid acquisition or origin-binding bundle exists in CAS without a matching anchor. | It is outside Magpie's record and cannot be admitted. |
| An exact anchor exists but the bundle bytes are unavailable. | Occurrence remains recorded; bundle interpretation is unresolved; no admission. |
| For the same `H` and `P`, closure `M1` lacks bundle B while closure `M2` contains the exact bundle. | `M1` produces bundle-unavailable; verification may proceed under `M2`. The change is attributable to `M1 != M2`, not ambient storage drift. |
| The external CAS gains or loses objects after closure construction. | An existing closure result does not change. A loader must construct a new closure; resolution performs no implicit CAS, filesystem, or network lookup. |
| The same exact contribution is bound to two groups. | Explicit conflict, zero admitted groups, and no aggregation benefit. |
| Artifact A cites B while B cites or copies A. | Citation topology does not create two origins. Cycles do not amplify; explicit lineage and origin admission remain required. |

## 16. Alternatives considered

### Metadata-only grouping

`metadata_json.independence_group` is rejected as authority. Evidence
producers could manufacture fresh groups. Such metadata may remain asserted
audit material but cannot admit itself.

### URL or domain grouping

Rejected for the first policy. Mirrors, CDNs, redirects, domain changes, and
copied or syndicated material make locator identity insufficient.

### Unique artifact digest equals unique origin

Rejected. Distinct byte strings may be transformations or presentations of one
upstream source.

### Publisher-global grouping

Rejected. Origin is contribution- and claim-scoped, not a permanent global
property of a publisher label.

### New embedded L0 receipt payloads immediately

Deferred. The existing anchored hierarchy already supplies a kind-agnostic
foreign-bundle occurrence seam. A future ADR may add L0 vocabulary only if
implementation evidence proves that the anchor seam is inadequate without
changing its frozen semantics.

### Unanchored CAS receipts

Rejected as authoritative history. A valid unanchored bundle is outside
Magpie's record.

### Probabilistic independence score

Rejected for the first origin-admission policy. Confidence, reputation,
Bayesian weight, model judgment, and numeric thresholds do not become
authority.

### One generic mixed-authority bundle

Rejected for the first implementation. Acquisition/derivation evidence and
governance origin binding require distinguishable bundle families and verifier
profiles. A bundle must not prove its own acquisition history and authorize its
own grouping without an external policy boundary.

## 17. Non-normative design influences

These are conceptual influences, not dependencies or compatibility claims:

- W3C PROV motivates distinguishing entities, activities, and agents.
- in-toto and DSSE motivate typed statements, envelopes, and verifier
  boundaries.
- TUF motivates explicit versions, explicit trust roots, and the absence of
  ambient “latest” authority.
- SCITT and C2PA reinforce that verifiable provenance and transparency do not
  establish content truth.

This contract introduces no dependency on, protocol compatibility with, or
implementation promise for those standards.

## 18. Future implementation sequence

1. `ResolutionContentClosureV0` contract — landed by Ticket 0040.
2. `ResolutionContentClosureV0` implementation and hostile tests — landed by
   Ticket 0041.
3. Exact two-family origin-binding bundle contract — this slice, ratified by
   Ticket 0042.
4. Standing-inert origin-binding verifier — next.
5. Standing-inert origin-admission audit.
6. Admitted-contribution audit.
7. Policy-v3 contract.
8. Conservative aggregation.

The immediate next PR is step 4 only. It must verify the exact origin-binding
contract without adding a loader, origin-admission rule, standing effect or
writer authority. Steps 5-8 remain future work. Every step before policy-v3
remains standing-inert; this contract still defines no aggregation threshold
or achieved-standing change.

## 19. Frozen surfaces and explicit non-goals

This contract does not modify or reinterpret `magpie-core-v1`, existing
payload tags, golden vectors, `SegmentAnchored` canonical fields, ADR-0001
occurrence/inclusion semantics, the optional Deadbolt boundary, policies
v0/v1/v2, existing snapshot or resolution bytes, evidence ceilings, actor
classes, historical `content_hash` events, the release contract, workspace
version, package inventory, `Cargo.lock`, or CI.

Outside Ticket 0041's separate closure module and hostile tests, it adds no
dependency, payload tag, canonical receipt bytes, key custody, CAS
implementation, crawler, downloader, filesystem/network access, callback,
plugin, writer API, MCP write path, gate, aggregation,
refutation, contradiction debt, invalidation, supersession, currentness,
reputation, confidence score, probabilistic independence, identity ontology,
automatic clustering, model integration, librarian implementation, Deadbolt
code, or production-readiness claim. Ticket 0038's exact proposed bundle kinds,
canonicalization/verifier profiles, SHA-256 selection, and canonical bytes apply
only to its acquisition/direct-derivation v0 contract and do not add runtime
behavior.

Ticket 0042 adds only the exact origin-binding documentation contract. It adds
no parser, fixture, verifier, receipt, origin-admission policy, trusted-
authority table, conflict fold, support rule, standing rule or runtime wiring.

## 20. Reviewer checklist

- Confirm the origin-binding slice remains a ratified docs-only contract with
  no runtime capability.
- Confirm Magpie event `Provenance` is not external source provenance.
- Confirm existing `content_hash` values are not retroactively verified
  artifact identities.
- Confirm artifact, acquisition, derivation, descriptor, contribution, group,
  binding, and admission identities remain distinct.
- Confirm `ContributionIdentity` is the exact binding subject while
  `OriginComparisonNamespace` is the policy/claim/scope namespace for comparing
  group keys across distinct contributions.
- Confirm the v0 namespace is derived from policy/claim/scope rather than
  serialized, and the two binding families remain closed and distinguishable.
- Confirm the same group in one namespace means same-origin material, different
  groups in one namespace may provide only corroboration separation, and keys
  in different namespaces are incomparable.
- Confirm the first implementation reuses `SegmentAnchored` without changing
  its fields or occurrence/inclusion meaning.
- Confirm only selected versioned and profile-verified bundle families may
  participate.
- Confirm a bare anchor, signature, bundle kind, URL, DOI, publisher, actor
  class, metadata field, or model score grants no origin authority.
- Confirm all trusted context is same-snapshot, root-matched, profile-verified,
  and reproducible from explicit `(H, P, M)` inputs.
- Confirm `ResolutionContentClosureV0` is finite, immutable, untrusted, bound in
  future audit output, and never supplemented by ambient CAS, filesystem, or
  network lookup.
- Confirm origin assignment is exact and contribution-scoped with no wildcard
  or publisher-global reuse.
- Confirm duplicates do not amplify and conflicts admit zero groups.
- Confirm missing bundle or artifact bytes remain unresolved rather than being
  reconstructed from L0.
- Confirm missing artifact bytes block artifact-grounded origin admission and
  origin-sensitive aggregation input without erasing structural evidence.
- Confirm Ticket 0038's proposed first bundle profiles fit the frozen
  64-lowercase-hex, 32-byte root representation, select SHA-256 only for those
  exact families, and leave other widths to a future ADR/L0 evolution decision.
- Confirm origin admission remains distinct from support, aggregation, and
  settlement.
- Confirm “origin group” and “corroboration separation” do not claim
  statistical independence.
- Confirm no aggregation threshold or implemented policy v3 appears.
- Confirm every implementation step before aggregation is standing-inert.
