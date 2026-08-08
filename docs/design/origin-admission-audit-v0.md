# Origin-Admission Audit v0

## 1. Status

Ratified documentation-only architecture contract. Ticket 0046 records this
decision, and Ticket 0048 implements the runtime. The implemented current v0
behavior is claimant-label compatibility behavior: its authority-pair match is
compiled compatibility label selection, not independently authenticated
authority, and its `OriginAdmitted` output is not independently authority-
bound. ADR-0007 (Accepted 2026-08-06) defines the separate authority-binding
object and `H + P + M + A` coordinate boundary for any future authority-bound
successor. A-024 remains **Confirmed** until that successor and its runtime
evidence exist.

The exact base of this contract is the post-PR-#58 main commit:

```text
0947169bdb0b6d7744bdcd4c9c12ee677cef5496
```

PR #58 landed the standing-inert origin-binding verifier v0. Its reviewed head
`95c58e7bb38441828426ad753a57e27ade4ea960` is an ancestor of this base.

## 2. Purpose

The existing resolver verifies one caller-selected origin-binding selector:

```text
one selected origin-binding selector
+ one immutable ResolutionContentClosureV0
+ one verified StandingReplaySnapshot
-> one standing-inert OriginBindingContextTraceV0
```

That is necessary verification machinery, but it cannot by itself prove that
the caller presented the complete set of potentially conflicting bindings.
Origin admission requires a complete replay-derived candidate universe, one
selected policy, one authority path and an audit bound to the exact log prefix
and immutable closure used. Under current v0 the authority path is compiled
compatibility label selection; an independently authenticated authority path
exists only in the future ADR-0007 successor.

The central laws are:

```text
verified origin binding
!= trusted authority

exact authority-label equality
!= authenticated grouping authority

trusted origin assignment
!= support contribution

origin admission
!= aggregation

distinct admitted origin groups
!= proof of statistical independence

caller-selected candidate subset
!= complete conflict audit
```

The positive construction is:

```text
complete replay-anchored candidate universe
+ exact selected policy
+ compiled authority-label selection under that policy
+ deterministic duplicate/conflict handling
+ exact verified-prefix identity
+ immutable closure identity
-> reproducible standing-inert origin-admission audit
```

The authority-path term in that construction is compatibility label selection
under current v0, not independently authenticated authority.

## 3. Deterministic input law

The complete input is:

```text
H = exact completely verified Magpie log prefix
P = exact compiled origin-admission policy identity
M = exact immutable ResolutionContentClosureV0 identity

same H + same P + same M
-> byte-identical origin-admission audit
```

No ambient state participates. In particular, the audit performs no second
store read, filesystem scan, CAS lookup, network request, callback, plugin
invocation, environment lookup, mutable-registry lookup, lazy hydration or
caller-directed search.

`H`, `P` and `M` remain separate. Projection equality is not log-prefix
identity; closure identity is not object validity; and a policy ID serialized
inside a bundle is not executable policy selection.

## 4. Exact selected policy v0

The only selected policy identity is:

```text
magpie-origin-admission-v0
```

The runtime implementation is selected by reviewed versioned code. Exact
equality is required:

```text
receipt.origin_admission_policy_id
== "magpie-origin-admission-v0"
```

The bundle field is claimant-controlled canonical statement content. It does
not select code. Any other exact value is `policy mismatch`.

A policy-mismatched binding remains visible in candidate audit output but:

- admits no origin;
- does not enter the selected policy's trusted assignment fold;
- does not enter the selected policy's conflict fold;
- does not block or veto a selected-policy assignment; and
- does not become an eligible-unresolved blocker.

V0 has no runtime policy parameter, registry, alias, implicit latest policy,
environment override, bundle-selected implementation or caller-selected
policy.

## 5. Exact compiled grouping-authority label selection v0

The selected policy matches exactly one claimed authority pair:

```text
authority.kind:
human-review

authority.reference:
authority:origin-review-v0
```

Selection comes from the compiled policy's exact equality with this pair. The
bundle merely names the pair and cannot select or authenticate itself. This is
compiled compatibility label selection: it prevents policy-label substitution,
but it does not authenticate the claimant, authorize an issuer, or
independently bind grouping authority. The matched claim remains a claimant
assertion, and admissions under this section are claimant-label compatibility
admissions. Authenticated grouping authority requires the separate ADR-0007
authority-binding object verified under explicit `H + P + M + A` coordinates.

Under the selected policy, a pair match admits only:

```text
one exact contribution-scoped origin-group assignment
```

The match does not authorize claim truth, evidence truth, publisher identity,
source quality, support, standing, settlement, statistical independence,
aggregation, writer access or any adjacent proposition.

Any other exact pair is `origin-binding authority untrusted`. A matched
untrusted binding remains audit-visible but admits no origin, does not enter
the trusted group partition, cannot veto a trusted assignment and cannot
manufacture a denial-of-service conflict.

No actor class, Magpie event provenance string, URL, domain, filename,
signature count, model score, metadata flag or bundle-carried `trusted` field
grants authority.

## 6. Complete replay-derived candidate universe

The implemented audit accepts no caller-supplied binding selectors, receipts
or candidate list. It derives candidates exclusively from the complete
`DeadboltAnchorIndex` co-produced by the same accepted replay as the standing
projection.

Define `OriginBindingCandidateUniverseV0(H)` as every distinct exact
five-field anchor identity in that replay satisfying all of:

```text
bundle_kind
in {
  magpie-origin-binding-acquisition-v0,
  magpie-origin-binding-derivation-v0
}

witness_algorithm
== sha256

canonicalization_profile
== magpie-origin-binding-json-v0
```

The five-field selector order is the existing exact order:

```text
bundle_kind
witness_root
witness_algorithm
canonicalization_profile
run_id
```

Distinct selector identities are ordered lexicographically by those exact
strings. Every exact repeated anchor occurrence remains attached to its one
selector in ascending event sequence. Repeated occurrences create neither
additional candidate statements nor additional authority.

Anchors using unsupported kinds, algorithms or profiles remain outside policy
v0's candidate universe. They grant no authority and do not negotiate future
support. Extra closure objects without a matching candidate anchor are outside
Magpie's accepted record and do not enter the audit.

This law makes a favourable caller-selected subset an impossible API shape.

This shape-matched, replay-derived candidate universe and the global
byte-completeness gate of section 8 are current v0 compatibility behavior.
They do not define the ADR-0007 successor's authority candidate designation,
finite designation set under `A`, assignment-key-scoped incompleteness, or
resolver-level designation-universe verification, and this section does not
retrofit those semantics into v0.

## 7. Candidate-enumeration implementation boundary

Ticket 0047 implements this boundary's crate-private deterministic
enumeration seam,
`OriginAdmissionReplayContextV0::origin_binding_candidates_v0()`, over the
same replay-derived `DeadboltAnchorIndex`. Ticket 0048 implements
`OriginAdmissionAuditV0` and the public `resolve_origin_admission_audit_v0()`
resolver that consumes that substrate. The seam must enumerate the actual
index co-derived by the same replay.

It must not:

- expose a public mutable map or iterator with mutation authority;
- accept a caller-created `DeadboltAnchorIndex` as trusted input;
- change `DeadboltAnchorIndex::canonical_bytes`;
- change existing exact occurrence lookup;
- change `StandingReplaySnapshot::canonical_bytes`;
- parse serialized anchor-index bytes to recover identities; or
- reconstruct candidates from closure contents.

Enumeration observes the private replay-derived map directly and returns the
supported exact selectors with their existing ordered occurrences.

## 8. Global binding-byte completeness gate

For every selector in `OriginBindingCandidateUniverseV0(H)`, exact binding
bytes must be available through:

```text
closure.foreign_bundle(selector)
```

If one or more candidate binding bundles are unavailable, the audit completion
is:

```text
IncompleteCandidateUniverse
```

Every unavailable selector is retained in deterministic selector order. The
top-level audit contains zero contribution decisions and zero admitted origins
globally.

This pessimistic v0 rule is required because an unavailable anchored binding
could target and conflict with any contribution. Its subject cannot be known
without the exact bytes.

There is no ambient lookup, best-effort admission, caller assertion that an
omitted selector is irrelevant, partial trusted result or admission from a
favourable available subset. Available candidates may still receive their
ordinary standing-inert verifier traces for audit visibility, but no available
trace is fed to the trusted assignment fold while completion is incomplete.

A later separately ratified contribution-to-selector index may narrow this
global requirement. V0 has no such index.

## 9. Shared origin-binding evaluation seam

Every available candidate must reuse the exact PR #58 parser and resolver
pipeline. The audit must not duplicate or reinterpret:

- origin-binding parsing;
- schema and semantic validation;
- canonical encoding;
- selector and witness-root checks;
- same-replay anchor lookup;
- contribution structural revalidation;
- nested artifact-provenance composition;
- artifact-coherence checks; or
- receipt construction.

The implementation may refactor the existing private pipeline to return
a crate-private evaluation result containing validated intermediate audit
material. The public surfaces remain unchanged:

```text
OriginBindingContextTraceV0
OriginBindingReceiptV0
StandingReplaySnapshot::resolve_origin_binding_context_v0
```

The internal result may expose a validated contribution, namespace, selected
policy ID, claimed authority pair and claimed group for a late-stage
availability-unresolved candidate. It must never expose unvalidated parser
material as authority. The current public resolver and trace bytes must retain
their exact behavior.

## 10. Definitive rejection and scoped unresolved eligibility

The audit distinguishes:

```text
definitively rejected under H and M
```

from:

```text
otherwise policy-eligible but unresolved because required M material
is unavailable
```

Parser, schema, canonical, selector, root, replay-structure, anchor, digest and
artifact-coherence failures are definitive rejections. They remain visible and
enter neither the trusted group partition nor the unresolved blocker set.

A candidate is a scoped unresolved eligible binding only when the shared
pipeline has already established all of:

```text
valid exact origin-binding subject
selected policy ID match
trusted authority pair match
exact ContributionIdentityV0
exact OriginComparisonNamespaceV0
```

and resolution stopped only because one required closure object was
unavailable. The v0 availability-only cases are:

```text
artifact-provenance bundle unavailable
acquisition artifact unavailable
derivation parent artifact unavailable
derivation output artifact unavailable
```

Other nested provenance failures, including malformed bytes, selector or root
mismatch, anchor absence and artifact digest mismatch, remain definitive
rejections.

A scoped unresolved eligible binding blocks admission only for its exact:

```text
ContributionIdentityV0
+ OriginComparisonNamespaceV0
```

Once that validated subject is known, it does not block unrelated
contributions. Its claimed group remains audit material and does not enter the
matched trusted group partition.

Disposition classification is deterministic:

1. missing candidate binding bytes are `BindingBytesUnavailable` and make the
   complete audit globally incomplete;
2. any definitive shared-pipeline failure is `DefinitivelyRejected`;
3. a matched or availability-only late-stage result with another policy ID is
   `PolicyMismatch`;
4. the same result with the selected policy but another authority pair is
   `AuthorityUntrusted`;
5. a selected-policy, trusted-authority availability-only result is
   `TrustedEligibleUnresolved`; and
6. a selected-policy, trusted-authority matched receipt is `TrustedMatched`.

Policy mismatch is checked before authority trust for outcomes eligible for
those classifications. Neither classification is an admission or veto.

## 11. Trusted assignment fold

Only matched `OriginBindingReceiptV0` values satisfying both exact selected
policy equality and exact trusted-authority equality enter the fold. "Trusted"
here names the compiled compatibility label selection of section 5, not
authenticated authority: assignments this fold produces are claimant-label
compatibility admissions.

The exact grouping key is:

```text
ContributionAdmissionKeyV0 {
    contribution: ContributionIdentityV0,
    namespace: OriginComparisonNamespaceV0,
}
```

This may remain a crate-private key. Its equality and order are exact. The
namespace remains exactly:

```text
origin_admission_policy_id
target_claim_id
scope_ref
```

It must not gain evidence ID, edge ID, artifact identity, selector, binding ID,
run ID or complete contribution identity as namespace members.

Within one key, matched trusted receipts are partitioned by exact opaque
`origin_group`.

## 12. Duplicate law

Multiple trusted matched receipts assigning the same exact:

```text
contribution
namespace
origin_group
```

derive one assignment only.

The audit retains every binding selector, binding ID, binding run ID, matched
receipt and exact anchor occurrence through the ordered candidate audits and
decision references. Therefore:

```text
N duplicate assignments
-> one admitted group
```

There is no multiplicity weight, confidence increase, quorum, majority,
authority amplification or occurrence-count amplification.

## 13. Conflict, unresolved and admission precedence

For one exact contribution-admission key, terminal precedence is:

```text
1. two or more distinct matched trusted origin groups
   -> ConflictingBindings

2. otherwise, one or more scoped unresolved eligible bindings
   -> EligibleBindingUnresolved

3. otherwise, exactly one distinct matched trusted origin group
   -> OriginAdmitted

4. otherwise
   -> no admitted-origin decision
```

A known conflict remains terminal when additional scoped unresolved candidates
exist. Those candidates remain referenced by the conflict audit, but no group
is admitted.

Conflict means exactly:

```text
same exact contribution
+ same exact namespace
+ selected policy
+ trusted authority
+ two or more distinct matched origin-group strings
```

Conflict admits zero groups. The fold never uses first-write-wins,
last-write-wins, newest-wins, oldest-wins, lexical minimum, lexical maximum,
majority, anchor count, human-looking preference, binding-ID preference or
run-ID preference.

Supersession, revocation, expiry and temporal ownership are future policies.

## 14. Untrusted and policy-mismatched non-veto law

A matched binding with the wrong policy ID or an untrusted authority pair
remains visible in candidate audit output. It does not:

- enter the trusted origin-group partition;
- admit a group;
- conflict with a trusted assignment;
- veto a trusted admission; or
- count as an unresolved eligible blocker.

This non-veto law prevents untrusted statements from manufacturing
denial-of-service conflicts.

## 15. Meaning of OriginAdmitted

`OriginAdmitted` means only:

```text
under policy magpie-origin-admission-v0,
this exact contribution is assigned to this exact opaque origin group
inside this exact comparison namespace
```

It does not mean that the contribution supports the claim, the contribution is
true, the origin group is a publisher, the origin is statistically independent,
the group is reputable, another group is independent, the claim reaches any
standing, or aggregation has occurred.

Current v0 `OriginAdmitted` is a claimant-label compatibility admission. It is
not independently authority-bound, not authenticated origin-group admission,
and not authorization by an external grouping authority, and it is not
eligible for silent use in a future authority-bound path by wrapper, alias,
field resemblance, or caller assertion. The future ADR-0007 successor must
carry its own explicit coordinate set.

The audit remains standing-inert derived material.

## 16. Exact verified-prefix identity

The implemented audit binds the exact accepted Magpie prefix with:

```text
VerifiedLogPrefixIdentityV0 {
    event_count
    tip_sha256
}
```

Field order is normative. `event_count` is the exact verified record count as
`u64`. `tip_sha256` is the final verified `ContentHash` rendered as exactly 64
lowercase hexadecimal characters. Empty-chain behavior remains the existing
log behavior and is not redefined here.

Both values must come from the exact same record snapshot used for projection
application:

```text
one read_records() snapshot
-> complete parsing and chain verification
-> retained verified events and verified tip
-> projection application over those retained events
-> exact event_count and tip identity
```

The implementation must not obtain the tip by calling verification and replay
separately. No second `read_records()` call is permitted.

The existing private retained `VerifiedSnapshot` already carries verified
events and tip. Ticket 0047 exposes that information through the additive
`LogReader::replay_with_summary` API and the separately versioned
`OriginAdmissionReplayContextV0` origin-resolution context. That exposure
preserves unchanged:

```text
LogReader::replay
StandingReplaySnapshot public fields
StandingReplaySnapshot::canonical_bytes
existing replay behavior
existing public verification summaries
```

Hashing `StandingReplaySnapshot::canonical_bytes` is forbidden as a substitute.
Two exact log prefixes may derive equal standing and anchor projections while
having different event histories and tips.

## 17. Closure identity binding

The audit retains the complete typed:

```text
ResolutionContentClosureIdentityV0 {
    schema
    canonicalization_profile
    digest_algorithm
    manifest_sha256
}
```

in that existing field order. It never retains only the bare manifest digest
as the identity.

The closure identity binds availability input `M`. It does not prove that
objects verified successfully, prove a key matched its bytes, reconstruct raw
objects, or grant authority.

## 18. Public audit surface

All field and variant orders in this section are normative for the derived
audit JSON emitted by the landed Ticket 0048 runtime.

### 18.1 VerifiedLogPrefixIdentityV0

```text
VerifiedLogPrefixIdentityV0 {
    event_count: u64
    tip_sha256: String
}
```

### 18.2 OriginAdmissionAuditCompletionV0

Closed variants, in declaration order:

```text
Complete

IncompleteCandidateUniverse {
    unavailable_binding_selectors:
        Vec<ArtifactProvenanceAnchorSelectorV0>
}
```

Unavailable selectors use exact candidate-selector order.

### 18.3 OriginAdmissionCandidateDispositionV0

Closed variants, in declaration order:

```text
BindingBytesUnavailable

DefinitivelyRejected

PolicyMismatch {
    claimed_policy_id: String
}

AuthorityUntrusted {
    authority_kind: String
    authority_reference: String
}

TrustedEligibleUnresolved {
    claimed_origin_group: String
}

TrustedMatched {
    origin_group: String
}
```

Only validated material may populate variant details.

### 18.4 OriginAdmissionCandidateAuditV0

```text
OriginAdmissionCandidateAuditV0 {
    selector: ArtifactProvenanceAnchorSelectorV0
    anchor_occurrences: Vec<DeadboltAnchorOccurrence>
    trace: OriginBindingContextTraceV0
    disposition: OriginAdmissionCandidateDispositionV0
    validated_contribution: Option<ContributionIdentityV0>
    validated_namespace: Option<OriginComparisonNamespaceV0>
}
```

Both optional fields are always serialized in place, using JSON `null` when
the shared validated pipeline cannot safely derive them. The complete existing
origin-binding trace is retained without flattening or reinterpretation.

### 18.5 AdmittedOriginV0

```text
AdmittedOriginV0 {
    policy_id: String
    contribution: ContributionIdentityV0
    namespace: OriginComparisonNamespaceV0
    origin_group: String
    supporting_candidate_selectors:
        Vec<ArtifactProvenanceAnchorSelectorV0>
}
```

Every duplicate trusted matched receipt for the one group is referenced by its
selector in exact selector order. Complete receipts and occurrences remain in
the corresponding candidate audits.

### 18.6 OriginBindingConflictV0

```text
OriginBindingConflictV0 {
    policy_id: String
    contribution: ContributionIdentityV0
    namespace: OriginComparisonNamespaceV0
    origin_groups: Vec<String>
    matched_candidate_selectors:
        Vec<ArtifactProvenanceAnchorSelectorV0>
    unresolved_candidate_selectors:
        Vec<ArtifactProvenanceAnchorSelectorV0>
}
```

`origin_groups` is exact lexical string order. Matched selector references are
flattened by ordered group partition: origin group first, then selector. The
candidate audits provide the exact selector-to-group and complete-receipt
mapping. Unresolved references use selector order and remain blocking audit
material without entering any matched group.

### 18.7 OriginAdmissionDecisionV0

Closed variants are declared in terminal-precedence order:

```text
ConflictingBindings {
    conflict: OriginBindingConflictV0
}

EligibleBindingUnresolved {
    policy_id: String
    contribution: ContributionIdentityV0
    namespace: OriginComparisonNamespaceV0
    matched_candidate_selectors:
        Vec<ArtifactProvenanceAnchorSelectorV0>
    unresolved_candidate_selectors:
        Vec<ArtifactProvenanceAnchorSelectorV0>
}

OriginAdmitted {
    admitted_origin: AdmittedOriginV0
}
```

At most one matched group exists in `EligibleBindingUnresolved`; its selector
references are ordered exactly. The unresolved list is non-empty. No decision
entry is emitted for terminal case 4. An incomplete candidate universe emits
no decision entries at all.

### 18.8 OriginAdmissionAuditV0

```text
OriginAdmissionAuditV0 {
    schema: String
    canonicalization_profile: String
    policy_id: String
    verified_prefix_identity: VerifiedLogPrefixIdentityV0
    closure_identity: ResolutionContentClosureIdentityV0
    completion: OriginAdmissionAuditCompletionV0
    candidate_audits: Vec<OriginAdmissionCandidateAuditV0>
    contribution_decisions: Vec<OriginAdmissionDecisionV0>
}
```

The fixed values are:

```text
schema:
magpie-origin-admission-audit-v0

canonicalization_profile:
magpie-origin-admission-audit-json-v0

policy_id:
magpie-origin-admission-v0
```

The top-level audit alone exposes `canonical_bytes()`.

## 19. Construction and authority properties of the public types

Every public type above has private fields, read-only getters,
deterministic equality and deterministic `Serialize`. None has `Deserialize`
or a public constructor from serialized material. Trusted construction occurs
only through the same-snapshot replay and immutable-closure audit path.

Enum serialization is adjacently tagged with exact fields:

```text
tag: outcome
content: details
variant spelling: snake_case
```

Serialized audits, decisions, admitted-origin values, conflicts, candidate
audits, receipts and identities are audit output only. No resolver accepts one
as authority or as a substitute for replay and closure inputs. No current v0
audit, decision, or admitted-origin value becomes authority-bound by its field
shape, label equality, or later convention; the ADR-0007 authority-bound
successor requires its own additive types and complete `H + P + M + A`
coordinates.

## 20. Deterministic ordering

The complete ordering law is:

```text
candidate selectors:
lexicographic exact five-field selector order

anchor occurrences:
ascending event sequence

contribution-admission keys:
lexicographic ContributionIdentityV0, then OriginComparisonNamespaceV0

origin groups:
lexicographic exact string order

receipts inside one group:
lexicographic binding-selector order
```

Candidate audits use candidate-selector order. Contribution decisions use
contribution-admission-key order. Caller order, closure insertion order and
anchor occurrence multiplicity never select the outcome.

## 21. Audit serialization

The exact audit identities are:

```text
audit schema:
magpie-origin-admission-audit-v0

audit canonicalization profile:
magpie-origin-admission-audit-json-v0
```

This is derived deterministic audit JSON. It is not Magpie L0 encoding,
`magpie-core-v1`, RFC 8785 or JCS. The encoder is purpose-built for the closed
types in section 18.

Top-level and nested object fields emit in their declared contract order.
Arrays use section 20 ordering. Strings use the existing purpose-built Magpie
JSON discipline: exact `\"` and `\\`, short control escapes, lowercase
`\u00xx` for other U+0000-U+001F controls, direct UTF-8 otherwise, no Unicode
normalization, no escaped solidus, no unnecessary escape, no whitespace, no
BOM and no trailing newline. Unsigned integers use shortest decimal form.

No raw artifact or foreign-bundle bytes are serialized into the audit.

Ticket 0046's documentation-only contract added no fixtures. The Ticket 0048
implementation pins literal byte fixtures for:

- one admitted origin;
- duplicate trusted bindings collapsing to one assignment;
- trusted conflict;
- scoped unresolved eligible binding; and
- globally incomplete candidate universe.

## 22. Standing-inert boundary

Origin-admission audit execution must leave byte-identical and semantically
unchanged:

```text
StandingView
StandingReplaySnapshot::canonical_bytes
standing policy v0
standing policy v1
standing policy v2
support and refutation ceilings
deterministic verifier context and receipts
origin-binding verifier public traces and receipts
```

An origin-admission audit changes no standing and is not consumed by policy
v2. It creates no support contribution, aggregation input accepted by a
standing policy, settlement or writer authority.

## 23. Required hostile implementation tests

The implementation is required to preserve, and the landed Ticket 0048 hostile
suite proves across the runtime and serialization integration tests in
`crates/magpie-claims/tests/origin_admission_audit.rs` and the compile-fail
API-shape doctests in
`crates/magpie-claims/src/origin_admission_audit.rs`, at least:

```text
caller supplies only favourable selector
-> impossible API shape

one replay-anchored binding bundle absent from closure
-> incomplete universe
-> zero admissions globally

extra unanchored closure bundle
-> ignored

three identical anchor occurrences
-> one candidate selector
-> all occurrences visible
-> no amplification

two trusted matched bindings, same group
-> one admitted assignment
-> both receipts visible

two trusted matched bindings, different groups
-> explicit conflict
-> zero admitted groups

trusted match plus untrusted conflicting match
-> trusted group admitted
-> untrusted statement visible
-> no veto

trusted match plus policy-mismatched conflicting match
-> trusted group admitted
-> mismatch visible
-> no veto

trusted match plus same-subject eligible unavailable prerequisite
-> eligible binding unresolved
-> no admission

known trusted conflict plus unresolved candidate
-> conflict remains terminal
-> unresolved candidate retained

same textual group in different namespaces
-> no relationship

same H + P + M
-> byte-identical audit

different M
-> visibly different closure identity and audit

different H with same derived standing projection
-> different verified-prefix identity

serialized audit or admitted-origin value
-> cannot substitute for replay and closure

audit execution
-> no standing v0/v1/v2 byte change
```

## 24. Explicit non-goals

This contract does not implement or ratify semantics for:

- admitted contribution;
- support contribution;
- standing policy v3;
- aggregation;
- corroboration thresholds;
- statistical independence;
- probability or confidence;
- source reputation;
- publisher ontology;
- trust scoring;
- authority hierarchy or wildcard authority;
- multiple trusted authority classes;
- supersession, revocation, expiry or temporal ownership;
- loader, CAS, filesystem or network access;
- writer authority;
- a new Magpie L0 payload; or
- Deadbolt changes.

It does not change existing standing, replay, closure, origin-binding,
artifact-provenance, deterministic-verifier, serialization, dependency,
fixture, Cargo, release, CI or L0 surfaces.

## 25. Recommended implementation sequence

At ratification time, the recommended implementation sequence was:

```text
Slice 1:
same-snapshot verified-prefix identity
+ deterministic replay-anchor candidate enumeration

Slice 2:
standing-inert OriginAdmissionAuditV0 implementation
+ candidate completeness
+ trusted duplicate/conflict fold

Then:
admitted-contribution audit
-> policy-v3 contract
-> conservative aggregation
```

Slice 1 landed separately as Ticket 0047 (origin-admission replay substrate)
and Slice 2 as Ticket 0048 (audit runtime), keeping the verified-prefix,
candidate-universe, authority and fold boundaries and their tests
independently reviewable. The later stages landed as Tickets 0049-0054.

## 26. Reviewer checklist

- Confirm the selected policy is exactly `magpie-origin-admission-v0` and
  bundle content cannot select code.
- Confirm the only compiled authority-label pair is exact `human-review` plus
  `authority:origin-review-v0`, that its match is compatibility label
  selection rather than authenticated grouping authority, and that it admits
  only one exact contribution-scoped origin-group assignment.
- Confirm candidates come from the complete same-replay anchor index and no
  caller candidate list or closure enumeration substitutes for it.
- Confirm unsupported anchor families remain outside v0 without negotiating
  future support.
- Confirm one unavailable candidate binding bundle makes completion globally
  incomplete and produces zero decisions.
- Confirm the PR #58 parser, verifier, trace and receipt pipeline is reused
  rather than duplicated.
- Confirm only availability-only nested failures may become scoped unresolved
  eligible bindings after exact subject, policy and authority validation.
- Confirm definitive failures, wrong-policy bindings and untrusted bindings
  enter neither the trusted conflict fold nor the unresolved blocker set.
- Confirm duplicate trusted assignments collapse without losing receipts or
  occurrences.
- Confirm conflict precedes unresolved, unresolved precedes admission and no
  write-order or multiplicity rule selects a group.
- Confirm untrusted and policy-mismatched statements remain visible but cannot
  veto trusted admission.
- Confirm verified-prefix count and tip come from one retained verified record
  snapshot with no second `read_records()` call.
- Confirm projection canonical bytes are not treated as log-prefix identity.
- Confirm the complete four-field closure identity is retained.
- Confirm every public value is private-construction, serialization-only
  audit material with deterministic order and no `Deserialize`.
- Confirm the audit remains standing-inert and policy v2 does not consume it.
- Confirm no runtime capability, fixture, code, L0, standing, closure or
  origin-binding serialized-surface change is claimed by this contract.
