# Ticket 0037 — Artifact Provenance and Origin Admission Contract

## Goal

Land a docs-only architecture contract for exact artifact identity, governed
acquisition occurrence, exact derivation lineage, source descriptors,
contribution-scoped origin binding, explicit origin-comparison namespaces,
opaque origin groups, immutable resolution content closures, and
replay-derived origin-admission audit.

This ticket implements no runtime capability. It defines the authority
substrate that must exist before genuine support aggregation can be designed or
implemented.

## Architectural laws

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

Positive construction:

```text
content addressing establishes exact artifact identity

a governed acquisition bundle establishes acquisition history

an exact derivation relation establishes lineage

a governed origin-binding bundle may assign one exact contribution path
to one opaque origin group

only a later explicit standing policy may count admitted origin groups
```

Binding and comparison remain distinct:

```text
origin bindings are contribution-scoped

origin-group comparison is namespace-scoped

multiple distinct ContributionIdentity values may map to the same
origin-group key inside one OriginComparisonNamespace
```

## Current gap

L0 can retain typed claims, typed evidence, justification edges, optional
`content_hash`, opaque `metadata_json`, signed Magpie event provenance, and
exact `SegmentAnchored` identities. That substrate does not establish:

- which exact external artifact bytes were acquired;
- which governed action acquired them;
- whether one artifact derives from another;
- whether a URL, DOI, repository, publisher, author, header, or signing
  identity is authentic;
- whether two artifacts share one upstream evidentiary origin;
- whether an origin assignment was authorized; or
- whether two reports may count as separate corroborators.

The existing aggregation note rejects self-declared independence metadata but
does not yet define the replayable authority mechanism that can admit an
origin-group assignment. Aggregation remains blocked until this gap is closed
by later standing-inert implementation slices.

Existing:

```rust
Provenance {
    agent,
    source,
}
```

records who or what produced a Magpie event. It does not prove external
publisher or server identity, authorship, DOI or repository ownership,
upstream origin, independence, control separation, or artifact truth.

Existing `EvidenceRegistered.content_hash` values remain asserted material.
This ticket does not retroactively convert them into verified artifact
identities.

## Selected anchored-hierarchy decision

The first implementation must represent governed artifact acquisition,
derivation, and origin-binding authority as explicit versioned foreign bundles
that are sealed and verified at the governed boundary and then committed to
Magpie through the existing `SegmentAnchored` occurrence/inclusion seam.

This decision means:

- no new L0 payload tag in the first implementation;
- distinguishable versioned acquisition, derivation, and origin-binding bundle
  families;
- explicit canonicalization/verifier profiles for every participating family;
- final bundle-kind strings and wire schemas deferred to later contracts;
- no authority from a bare `bundle_kind` string;
- Magpie replay establishes exact anchored occurrence only;
- the bundle's verifier establishes exact bundle contents relative to its
  profile;
- origin-admission policy governs whether a verified binding may assign a
  group; and
- later standing policy governs whether admitted groups affect standing.

Only explicitly selected, versioned, root-matched, and verified families may
participate. An arbitrary anchored bundle or
`metadata_json.independence_group` cannot admit itself.

The seam remains portable and optional. Magpie core must not require live
Deadbolt infrastructure. Using the seam must not change
`SegmentAnchored` canonical fields or ADR-0001 occurrence/inclusion
semantics.

The frozen seam accepts `witness_root` only as exactly 64 lowercase hexadecimal
characters, corresponding to 32 root bytes. Any first foreign-bundle verifier
profile that reuses `SegmentAnchored` must produce that representation and name
its exact algorithm explicitly in `witness_algorithm`. This constraint does not
select SHA-256 or any other algorithm. A different root width or encoding
requires a future ADR and potentially an explicit L0 evolution decision rather
than silent reuse of this seam.

## Vocabulary to freeze

### Artifact identity

Exact bytes under an explicit digest algorithm. It is not a filename, URL,
title, DOI, MIME type, row ID, semantic document, acquisition, or source.
Byte-identical acquisitions share artifact identity but have distinct
acquisition occurrences. Distinct bytes do not imply distinct origins.

### Acquisition occurrence

A governed acquisition profile's record that it obtained or imported one exact
artifact. A later bundle must bind a versioned schema, acquisition profile,
artifact identity, acquisition/run identity, observed locator or import source,
chain-relevant occurrence identity, and any bounded transport observations.
It proves neither truth nor publisher identity.

### Derivation relation

An explicit binding among exact parent artifact identity, exact derived
artifact identity, transformation profile, and run identity. Extraction,
normalization, OCR, transcription, translation, summarization, keyframes,
filtering, and model analysis are derivations. No transitive inheritance,
fuzzy derivation, or semantic similarity is authoritative in the first
version.

### Source descriptor

Observed or asserted URL, DOI, repository/commit, publisher, claimed author,
feed, archive ID, response header, or signing identity. A descriptor may
inform governance but is not authority. No universal publisher/author/
organisation ontology is in scope.

### Contribution identity

The exact origin-admission subject: target claim identity, source evidence
identity, justification edge identity, exact scope, and relevant artifact
identity where applicable.

### Origin comparison namespace

The exact namespace in which origin-group keys from multiple distinct
contributions may be compared. The first policy's minimum
`OriginComparisonNamespace` includes exact origin-admission policy identity,
target claim identity, and `scope_ref`.

It must not include source evidence ID, justification-edge ID, artifact
identity, acquisition occurrence, or the complete `ContributionIdentity` when
that would make every contribution incomparable. A later explicit aggregation
policy may further partition it through a separately defined aggregation-lane
identity; this ticket does not define those future lane fields.

### Origin group

An opaque exact policy key that prevents repeated counting of admitted
contribution paths sharing one governed origin. It is not a truth, reputation,
quality, publisher, confidence, independence, ownership, or authority label.
Its equality is interpreted across separately bound contributions inside one
exact origin-comparison namespace:

```text
same group key + same origin-comparison namespace
= same admitted origin group

same group key + different origin-comparison namespace
= no defined relationship
```

### Origin-binding statement

A governed assignment from one exact contribution identity to one opaque group
inside one exact origin-comparison namespace under one explicit policy and
authority path. A later statement must bind its versioned schema/profile, exact
contribution identity, origin-comparison namespace, artifact/acquisition
references, group, governance scope, authority identity/reference, and
run/decision identity. Rationale is audit prose, not machine authority.

### Admitted origin result

A replay-derived result after structural revalidation, artifact-byte
verification, anchored and verified acquisition/derivation/binding bundles,
exact contribution matching, trusted authority under one explicit policy, and
absence of unresolved conflict. It is not a support contribution.

### Resolution content closure

The conceptual `ResolutionContentClosureV0` is an immutable, finite, explicitly
identified set of untrusted artifact bytes and foreign-bundle bytes supplied to
one resolution attempt. It is not a trust root, authority registry, mutable CAS
view, network namespace, evidence, or permission to fetch more material.

Artifact objects are keyed by exact `ArtifactIdentity`. Foreign bundle objects
are keyed by the full matching `SegmentAnchored` identity, or an exact derived
anchor identity containing `bundle_kind`, `witness_root`, `witness_algorithm`,
`canonicalization_profile`, and `run_id`. A filename or locator is not a closure
key. The exact type, encoding, digest profile, and manifest schema remain future
work.

## Contribution-scoped origin-admission law

```text
origin grouping is claim/contribution scoped

not:

publisher X always equals origin group Y for every claim
```

The first policy permits no prefix, wildcard, containment, inheritance,
publisher-global assignment, implicit reuse, or cross-policy meaning. The
resolver must re-fetch and exactly match the target claim, evidence, edge,
scope, and relevant artifact from one verified snapshot. One binding never
binds another contribution implicitly.

That exact-subject rule does not isolate comparison. Separately verified
bindings for distinct `ContributionIdentity` values may map to the same group
key inside one exact `OriginComparisonNamespace`:

```text
ContributionIdentity
= exact subject of one binding

OriginComparisonNamespace
= policy/claim/scope namespace for comparing assignments across contributions
```

Different contributions from one publisher may represent an independent
investigation, copied press release, or syndicated report. A permanent
publisher label cannot decide those cases.

## Same-snapshot and authority boundary

```text
one completely verified log replay
+ exact anchored foreign-bundle identities
+ untrusted externally supplied bytes
+ profile-specific verification
= possible trusted derived context
```

Callers may supply untrusted CAS bytes only through an explicit immutable
resolution content closure. They may not supply trusted acquisition results,
origin bindings, admitted-origin results, or authority booleans. Trusted context
exists only after artifact bytes match the exact artifact identity carried by a
verified bundle and each supplied bundle root matches an exact anchor in the
same verified replay.

```text
deterministic provenance/origin resolution input
= verified log prefix H
+ explicit policy identities P
+ immutable resolution content closure M

same H + same P + same M
-> same derived result
```

A future output must disclose or bind the verified prefix identity, every
applicable policy identity, and resolution-content-closure identity. Resolution
may perform no ambient CAS lookup, filesystem scan, callback, remote fetch, or
network lookup. A separate loader may populate the closure before resolution;
once constructed, it is finite and immutable. Closure objects remain untrusted
until their exact digest/root, profile, and same-replay anchor checks pass.

An unanchored valid bundle is outside Magpie's record. A bundle anchored after
historical tip H cannot affect resolution as of H. An object absent from `M` is
unavailable for that resolution and produces the relevant explicit unavailable
outcome without ambient lookup. A later resolution over the same historical log
prefix may resolve the material only with an explicitly different closure `M2`.
That is a new input, not mutation of the result for `(H, P, M)`. If the log tip
also changes, both differences remain visible.

A signer named by a bundle cannot bootstrap its own authority. Self-signing is
integrity relative to a key, not trusted identity. Trust roots and verifier
profiles are explicit and versioned. No ambient mutable registry or
`latest` acquisition, origin-admission, or standing policy exists. Runtime
callers cannot choose a favourable policy.

Actor class, metadata, URL, domain, filename, signature count, and model
confidence grant no authority. Human authority may govern grouping but cannot
settle factual truth. Deadbolt governs execution/admission and records
evidence; it is not a truth oracle.

Existing v0/v1/v2 policy and snapshot bytes remain frozen. Later work may reuse
the same-snapshot anchor index but must separately version any new snapshot or
context surface.

## Duplicate and conflict behavior

Two exact verified bindings assigning the same exact contribution identity to
the same namespace and group under the same policy remain visible as duplicate
occurrences. They create at most one assignment, do not amplify, and need not
constitute a conflict.

Two verified bindings assigning the same exact contribution identity to
different groups under the same namespace and policy must:

- remain visible;
- produce an explicit conflict;
- admit no group;
- contribute zero corroboration separation; and
- avoid first-write-wins, last-write-wins, lexical minimum, or preference for
  newer or human-looking statements.

Supersession, revocation, expiry, and temporal ownership change remain future
work. Until those semantics exist, conflicts fail closed.

Different contributions assigned the same admitted group in the same namespace
are valid same-origin material, not duplicate binding occurrences and not a
conflict. A later aggregation policy may count their shared group at most once.

Different contributions assigned different admitted groups in the same
namespace may provide policy-recognised corroboration separation. That is not
statistical-independence proof and creates no support before a later explicit
aggregation policy. Group-key strings in different namespaces remain
incomparable even when byte-identical.

## Required audit distinctions

Future standing-inert audit must distinguish:

- no artifact identity;
- artifact bytes unavailable;
- artifact digest mismatch;
- acquisition anchor absent;
- acquisition bundle unavailable;
- acquisition bundle malformed;
- acquisition verifier/profile unsupported;
- acquisition verification failed;
- acquisition verified;
- derivation absent when required;
- derivation bundle unavailable;
- derivation bundle invalid;
- source descriptor only;
- origin binding absent;
- origin-binding anchor absent;
- origin-binding bundle unavailable;
- origin binding malformed;
- origin-binding authority untrusted;
- contribution-subject mismatch;
- exact duplicate binding;
- conflicting binding; and
- origin admitted.

These outcomes must not collapse into Boolean `verified` or `independent`
fields. Unknown states fail closed. `artifact bytes unavailable` means no
artifact-grounded origin admission and no origin-sensitive aggregation input;
it does not erase the structurally recorded evidence or forbid a different
future policy from using evidence that does not require artifact verification.
Every unavailable outcome is relative to the exact resolution content closure
and must not trigger ambient object lookup.

## Storage model

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

L0 alone proves anchored occurrence and chain order. Bundle interpretation
requires exact bundle bytes and the selected verifier. Missing bundle bytes
leave context unresolved. Missing artifact bytes prevent content verification
and therefore block artifact-grounded origin admission and origin-sensitive
aggregation input; the structural evidence remains visible and unresolved. A
future archive/export may package the log and referenced CAS objects, but Ticket
0037 makes no log-alone reconstruction or packaging claim. The CAS is only a
source of untrusted closure inputs, never an ambient or authoritative store for
resolution.

## Interaction with aggregation

Normative terminology:

- `origin group` is the opaque policy grouping key;
- `corroboration separation` means two admitted contributions have distinct
  origin groups inside the same origin-comparison namespace;
- “independence group” is earlier design shorthand preserved in the existing
  aggregation-note filename; and
- Magpie does not prove statistical, causal, institutional, organisational, or
  control independence.

```text
distinct admitted origin groups
= policy-recognised corroboration separation

distinct admitted origin groups
!= proof of statistical independence
```

Required sequence:

```text
artifact provenance and origin-admission contract
-> standing-inert foreign-bundle verification
-> standing-inert origin-admission audit
-> admitted-contribution audit
-> explicit aggregation policy
```

The admitted-contribution audit will be a standing-inert explanation surface
that revalidates one exact policy-eligible contribution and associates it with
its admitted origin result. It is derived from the explicit `(verified log
prefix H, policy identities P, immutable resolution content closure M)` input,
reports potential aggregation input, and creates neither support nor standing.

Policy v2 direct support is not aggregation. Origin admission is not support.
One admitted group is not aggregation. Missing, malformed, conflicting,
self-declared, unverified, or unknown origin material provides zero
corroboration separation in the first policy. Same-origin multiplicity may
count at most once later within one comparison namespace. Keys in different
namespaces are incomparable. External-source aggregation can never reach
`Settled`.

No numeric threshold, policy v3, “two sources imply Supported” rule, support
aggregation, refutation, contradiction debt, invalidation, or supersession is
defined here.

## Files in scope

Exactly:

- `README.md`
- `docs/design/artifact-provenance-origin-admission.md`
- `docs/design/standing-aggregation-independence-groups.md`
- `tickets/0037-artifact-provenance-origin-admission.md`

Historical ticket 0022 remains unchanged.

## Frozen surfaces

Do not modify or reinterpret:

- `magpie-core-v1`;
- existing payload tags and golden vectors;
- `SegmentAnchored` fields or ADR-0001 semantics;
- optional Deadbolt portability;
- policies v0, v1, and v2;
- existing snapshot and resolution bytes;
- evidence ceilings and actor classes;
- historical `content_hash` values;
- release contract, workspace version, package inventory, `Cargo.lock`, or
  CI.

## Explicit non-goals

No Rust, tests, Cargo dependency, L0 tag, canonical receipt bytes, final
bundle-kind strings, final canonicalization profile, cryptographic algorithm
choice, resolution-content-closure type or manifest encoding, key custody, CAS
implementation, crawler, downloader, filesystem or network access, callback,
plugin, writer API, MCP write path,
`EpistemicGate`, policy v3, aggregation threshold, support aggregation,
direct refutation, contradiction debt, invalidation, supersession,
currentness, reputation, confidence score, probabilistic independence,
publisher ontology, automatic source clustering, model integration, librarian
implementation, Deadbolt code, or production-readiness claim.

## Hostile examples and reviewer checks

- One wire report mirrored at 100 URLs creates no automatic additional group.
- Contribution A and Contribution B assigned `wire-report-17` under the same
  policy/claim/scope namespace are valid same-origin material, not a conflict;
  later counting is at most once.
- Contribution A assigned `wire-report-17` and Contribution C assigned
  `field-report-4` in the same namespace may provide corroboration separation,
  not statistical-independence proof or support by themselves.
- The key `wire-report-17` under another claim or policy namespace has no
  relationship to the byte-identical key above.
- Two crawlers fetching the same bytes create one artifact identity and two
  acquisition occurrences, not two corroborators.
- PDF, HTML, and extracted text require explicit lineage and create no
  automatic separation.
- Repeated press releases remain non-amplifying without admitted bindings.
- Reuters/AP differences in bytes and publisher labels do not prove separation.
- Model summaries and paraphrases remain derived analytical artifacts.
- Shared datasets, benchmarks, and checkpoints may preserve shared origin.
- Different keys do not prove distinct control.
- Valid unanchored bundles remain outside Magpie's record.
- Anchored but unavailable bundles remain unresolved.
- For the same `H` and `P`, closure `M1` lacking bundle B produces an unavailable
  outcome; closure `M2` containing exact B may proceed to verification. The
  result change is attributable to `M1 != M2`.
- Ambient CAS additions or removals do not change an existing closure result;
  a new closure must be constructed, and resolution performs no implicit
  lookup.
- Conflicting bindings admit zero groups.
- Circular citations do not amplify.

Reviewers must also confirm that event provenance is not publisher identity;
artifact digest is not origin; URL/DOI is not authority; a bare anchor does not
verify bundle content; a bare signature is not trusted identity; CAS bytes are
root-matched; trusted context is not caller-created; assignments are not
publisher-global; unknown origin is not a weak independent group; conflicts do
not use write order; no support contribution or threshold is implied; L0 is not
said to reconstruct foreign contents; cryptography is not said to prove factual
correctness; no hidden source of truth or ambient latest policy appears; and a
docs-only contract makes no runtime claim.

Reviewers must confirm separately that bindings remain exact and contribution-
scoped; group equality compares distinct contributions only in one explicit
origin-comparison namespace; same-group distinct contributions are not a
conflict; duplicate occurrence, conflict, same-origin grouping, and potential
separation remain distinct; `(H, P, M)` fully identifies reproducibility input;
the closure is finite, immutable, and untrusted; no ambient CAS/filesystem/
network lookup occurs; unavailable artifact bytes block artifact-grounded origin
admission without erasing structural evidence; and first bundle profiles fit
the frozen 64-lowercase-hex, 32-byte root representation without selecting an
algorithm.

## Validation requirements

Run:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo run --locked --example tour -p magpie-claims
python tools/check_release_metadata.py
cargo package -p magpie-log --locked
python tools/verify_chain.py crates/magpie-log/testdata/golden-v1.jsonl ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c
git diff --check
git status --short
git diff --stat
git diff --name-only
```

Also run the task's broad authority audit and focused link, terminology, and
stale-sequencing searches. Review every authority-audit match in changed
documents. Confirm exactly four changed paths and no prohibited surface.

## Next implementation sequence

1. Ratify Ticket 0037.
2. Pin one exact versioned acquisition/derivation foreign-bundle schema and
   verifier profile.
3. Add portable bundle fixtures and standing-inert verification.
4. Pin one exact origin-binding bundle schema and explicit origin-admission
   policy.
5. Add standing-inert origin-admission audit.
6. Add an admitted-contribution audit surface with no aggregation.
7. Pin one explicit standing policy v3 aggregation rule.
8. Implement conservative aggregation.
9. Add direct refutation through admitted verifier context.
10. Add contradiction debt.
11. Add invalidation and supersession/currentness.
12. Add `EpistemicGate` and writer-facing surfaces.
13. Add governed acquisition tooling and librarian proposals.

The next implementation slice is step 2 only. It must remain standing-inert,
pin one exact acquisition/derivation bundle family and verifier profile, and
stop if it requires changing `SegmentAnchored`, adding an L0 payload, making
Deadbolt mandatory, introducing a universal identity ontology, widening
v0/v1/v2 bytes, or selecting aggregation behavior.
