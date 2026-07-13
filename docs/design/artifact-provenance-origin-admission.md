# Artifact Provenance and Origin Admission

## 1. Status

Proposed architecture contract seeking human ratification.

This note defines doctrine and implementation order only. Ticket 0037 is
documentation-only. No artifact, acquisition, derivation, origin-binding,
origin-admission, contribution, aggregation, or writer capability is
implemented by this contract.

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
- Exact bundle-kind strings, wire schemas, digest algorithms, and verifier
  profiles are deferred to later implementation contracts.
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

The first implementation must pin an exact versioned representation before
using artifact identity as authority. This contract selects no final digest
algorithm and does not reinterpret historical `content_hash` values.

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

### 6.6 Origin group

An origin group is an opaque exact policy key used to prevent several admitted
contribution paths that share one governed origin from being counted
repeatedly.

It is not a truth label, reputation score, source-quality score, publisher
identity, confidence value, statistical proof of independence, ownership
certificate, or authority key. Equality is exact.

An origin group's meaning is scoped by the exact origin-admission policy, the
contribution subject, and the relevant claim scope. A group identifier has no
cross-policy meaning unless a later explicit contract says otherwise.

### 6.7 Origin-binding statement

A governed origin-binding statement assigns one exact contribution identity to
one opaque origin group under one explicit policy and authority path. A future
statement must bind at minimum:

- a versioned schema and verifier profile;
- the exact contribution identity;
- relevant exact artifact and acquisition references;
- the origin-group key;
- the exact governance scope;
- the authority identity or authority reference; and
- an exact run or decision identity.

The bundle may retain rationale for audit. Prose rationale is not machine
authority. An origin-binding statement governs grouping; it does not make
evidence or claims true and does not create a support contribution.

### 6.8 Admitted origin result

An admitted origin result is a replay-derived policy outcome, not caller input.
A later policy may admit one group only after:

- the contribution path is structurally revalidated;
- the relevant exact artifact identity is present;
- required artifact bytes are available and hash correctly;
- the acquisition bundle is present, anchored, and profile-verified;
- every required derivation relation is present, anchored, and
  profile-verified;
- the origin-binding bundle is present, anchored, and profile-verified;
- the binding targets the exact contribution identity;
- the binding authority is trusted under an explicit origin-admission policy;
  and
- no unresolved conflicting binding exists.

Admission is still not a support contribution. It authorizes only an exact
grouping statement under the named policy.

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
- Missing artifact bytes prevent content verification and evidential use.
- L0 alone cannot reconstruct foreign bundle contents.
- A later archive or export mechanism may package the log and all referenced
  content-addressed objects, but that work is outside Ticket 0037.

The external stores are not hidden sources of authority. Bytes from them are
untrusted inputs until exact identity and root checks succeed against the same
verified replay.

## 9. Same-snapshot verification

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

Trusted context may be constructed only after artifact bytes match the exact
artifact identity carried by a verified bundle and each supplied bundle root
matches an exact anchor in the same completely verified replay. A foreign
bundle that is otherwise valid but unanchored is outside Magpie's record. A
bundle anchored only after historical tip H cannot influence a resolution as
of H. A later snapshot may resolve and admit previously unresolved evidence
without rewriting historical outcomes.

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
statement.

The conceptual flow is:

```text
exact contribution identity
+ verified artifact identity
+ verified acquisition and required derivation lineage
+ exact verified origin-binding statement
+ explicit trusted authority under one origin-admission policy
+ no unresolved conflict
-> one admitted origin group
```

There is no implicit reuse across claims, scopes, edges, artifacts, policies,
or time. Supersession, revocation, expiry, and temporal ownership change are
future semantics. Until those semantics exist, a conflict cannot be repaired
by preferring a newer-looking statement.

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
the same exact origin group under the same policy:

- remain visible as duplicate occurrences;
- do not create two groups;
- do not amplify; and
- do not necessarily create a conflict.

The later audit may retain every occurrence while deriving one exact assignment.

### Conflicting binding

Two verified bindings that assign the same exact contribution identity to
different origin groups under the same policy:

- both remain visible;
- produce an explicit conflict outcome;
- admit no origin group;
- contribute zero corroboration separation; and
- never use first-write-wins, last-write-wins, lexical minimum, or silent
  preference for the newer or more human-looking statement.

Supersession, revocation, expiry, and temporal ownership change are future work.
Until then, unresolved conflict fails closed.

## 13. Conceptual audit outcomes

The future audit surface must use a closed, stage-specific vocabulary. Exact
Rust enum names are deferred, but the following distinctions may not be
collapsed into Boolean `verified` or `independent` fields.

| stage | required outcome | meaning and fail-closed effect |
| --- | --- | --- |
| artifact | no artifact identity | no exact subject; no admission |
| artifact | artifact bytes unavailable | identity may be asserted, but content cannot be checked; no evidential use or admission |
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

## 14. Interaction with standing and aggregation

This contract corrects the implementation sequence:

```text
artifact provenance and origin-admission contract
-> standing-inert foreign-bundle verification
-> standing-inert origin-admission audit
-> admitted-contribution audit
-> explicit aggregation policy
```

The admitted-contribution audit is a future standing-inert explanation surface.
It will revalidate one exact policy-eligible contribution and associate it with
its admitted origin result while preserving every unresolved prerequisite. It
reports potential aggregation input; it does not create support or standing.

Policy v2 direct support is not aggregation. Origin admission is not support.
One admitted group is not aggregation. Missing, malformed, conflicting,
self-declared, or unverified origin material does not amplify. Unknown origin
counts as zero corroborating separation in the first policy. Same-origin
multiplicity may count at most once under a later aggregation policy.

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
| Two governed crawlers acquire the same exact bytes. | One artifact identity, two acquisition occurrences, and no duplicate corroboration. |
| A paper exists as PDF, publisher HTML, and extracted text. | Possibly several artifacts with explicit lineage; no automatic separate origin. |
| Several outlets repeat one company press release. | Distinct publishers do not establish distinct origin. Without admitted bindings, contributions remain non-amplifying. |
| Reuters and AP publish different reports about one event. | Different publishers and bytes are insufficient. Separation requires admitted contribution-scoped bindings; unresolved material remains visible but non-amplifying. |
| One model summarizes a paper and another paraphrases the summary. | Both are derived analytical artifacts. Neither becomes independent external corroboration, and model-provider diversity grants no separation. |
| Two ML papers share a dataset, benchmark harness, or checkpoint. | Different papers may share upstream dependencies. Lineage may expose overlap, but the first policy infers no causal independence. |
| Two statements use different signing keys controlled by one actor. | Signature diversity does not prove control separation and creates no automatic additional group. |
| A valid acquisition or origin-binding bundle exists in CAS without a matching anchor. | It is outside Magpie's record and cannot be admitted. |
| An exact anchor exists but the bundle bytes are unavailable. | Occurrence remains recorded; bundle interpretation is unresolved; no admission. |
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

1. Ratify Ticket 0037 and this architecture contract.
2. Pin one exact versioned acquisition/derivation foreign-bundle schema and
   verifier profile.
3. Add portable bundle fixtures and standing-inert verification.
4. Pin one exact origin-binding bundle schema and explicit origin-admission
   policy.
5. Add a standing-inert origin-admission audit.
6. Add an admitted-contribution audit surface with no aggregation.
7. Pin one explicit standing policy v3 aggregation rule.
8. Implement conservative aggregation.
9. Add direct refutation through admitted verifier context.
10. Add contradiction debt.
11. Add invalidation and supersession/currentness.
12. Add `EpistemicGate` and writer-facing surfaces.
13. Add governed acquisition tooling and librarian proposals.

Step 7 names a future policy slot only. Ticket 0037 does not define or
implement policy v3, an aggregation threshold, or any achieved-standing change.
Every step before aggregation remains standing-inert.

## 19. Frozen surfaces and explicit non-goals

This contract does not modify or reinterpret `magpie-core-v1`, existing
payload tags, golden vectors, `SegmentAnchored` canonical fields, ADR-0001
occurrence/inclusion semantics, the optional Deadbolt boundary, policies
v0/v1/v2, existing snapshot or resolution bytes, evidence ceilings, actor
classes, historical `content_hash` events, the release contract, workspace
version, package inventory, `Cargo.lock`, or CI.

It adds no Rust, tests, dependency, payload tag, canonical receipt bytes, final
bundle-kind string, final canonicalization profile, cryptographic algorithm,
key custody, CAS implementation, crawler, downloader, filesystem/network
access, callback, plugin, writer API, MCP write path, gate, aggregation,
refutation, contradiction debt, invalidation, supersession, currentness,
reputation, confidence score, probabilistic independence, identity ontology,
automatic clustering, model integration, librarian implementation, Deadbolt
code, or production-readiness claim.

## 20. Reviewer checklist

- Confirm the note remains a proposed docs-only contract.
- Confirm Magpie event `Provenance` is not external source provenance.
- Confirm existing `content_hash` values are not retroactively verified
  artifact identities.
- Confirm artifact, acquisition, derivation, descriptor, contribution, group,
  binding, and admission identities remain distinct.
- Confirm the first implementation reuses `SegmentAnchored` without changing
  its fields or occurrence/inclusion meaning.
- Confirm only selected versioned and profile-verified bundle families may
  participate.
- Confirm a bare anchor, signature, bundle kind, URL, DOI, publisher, actor
  class, metadata field, or model score grants no origin authority.
- Confirm all trusted context is same-snapshot, root-matched, and
  profile-verified.
- Confirm origin assignment is exact and contribution-scoped with no wildcard
  or publisher-global reuse.
- Confirm duplicates do not amplify and conflicts admit zero groups.
- Confirm missing bundle or artifact bytes remain unresolved rather than being
  reconstructed from L0.
- Confirm origin admission remains distinct from support, aggregation, and
  settlement.
- Confirm “origin group” and “corroboration separation” do not claim
  statistical independence.
- Confirm no aggregation threshold or implemented policy v3 appears.
- Confirm every implementation step before aggregation is standing-inert.
