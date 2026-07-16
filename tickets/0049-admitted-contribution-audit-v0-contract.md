# Ticket 0049: Admitted-contribution audit v0 contract

## 1. Publication identity and exact base

```text
base:
3419c1d17b78d777347f8a6b4526f37a5cc4e58a

reviewed PR #61 implementation head:
59db35521a1cf4c18217ab6842e95f8d0ad7ab83

branch:
agent/admitted-contribution-audit-v0-contract

commit:
docs: ratify admitted-contribution audit v0

PR title:
docs: ratify admitted-contribution audit v0

PR state:
draft
```

The base is the merge commit of PR #61. The reviewed PR #61 head is an
ancestor of that exact base.

## 2. Classification and goal

This ticket ratifies the exact standing-inert contract:

```text
verified replay structure
+
exact evidence artifact identity
+
complete origin-admission audit
+
compiled admitted-contribution policy
->
deterministic admitted-contribution audit
```

Exact classification:

```text
correctness:
complete admitted-contribution contract

authority:
exact graph contribution + exact verified artifact
+ exact admitted origin only

standing:
unchanged

support contribution:
not implemented

aggregation:
not implemented

runtime:
not implemented
```

This is documentation only. It defines the future implementation boundary
completely enough for the next runtime PR to implement mechanically without
reopening v0 policy semantics.

## 3. Exact changed-path allowlist

Only:

```text
README.md
docs/design/standing-aggregation-independence-groups.md
docs/design/admitted-contribution-audit-v0.md
tickets/0049-admitted-contribution-audit-v0-contract.md
```

No Rust, test, fixture, Cargo, lockfile, release, tool, CI, L0, existing ticket
or existing origin-admission design path is in scope.

## 4. Central deterministic law

Freeze:

```text
H:
the exact completely verified Magpie log prefix carried by
OriginAdmissionReplayContextV0

Pₒ:
the exact compiled origin-admission policy
magpie-origin-admission-v0

P꜀:
the exact compiled admitted-contribution policy
magpie-admitted-contribution-v0

M:
the exact immutable ResolutionContentClosureV0 and its complete
four-field identity

same H + same Pₒ + same P꜀ + same M
->
byte-identical admitted-contribution audit
```

No ambient filesystem, CAS, network, callback, plugin, environment, mutable
registry, second replay or caller-directed search participates.

The future resolver must internally derive `OriginAdmissionAuditV0` from the
same private-construction replay context and the same closure. It must not
accept a caller-provided origin audit as authority.

## 5. Exact policy identities

Freeze:

```text
admitted-contribution policy:
magpie-admitted-contribution-v0

audit schema:
magpie-admitted-contribution-audit-v0

audit canonicalization profile:
magpie-admitted-contribution-audit-json-v0

required origin-admission policy:
magpie-origin-admission-v0
```

Reviewed compiled code selects both policies. There is no runtime selector,
registry, alias, negotiation, environment override, bundle-selected
implementation, caller-selected implementation or implicit `latest`.

## 6. Complete candidate universe

Freeze:

```text
AdmittedContributionCandidateUniverseV0(H)
=
the complete BTreeMap-ordered justification-edge projection from H
```

The exact universe is:

```text
every exact TypedJustificationEdge retained in the accepted
StandingReplaySnapshot, ordered by exact edge ID
```

Every retained edge produces exactly one candidate audit.

Enumeration must not begin from:

- origin-admitted decisions;
- caller-provided edge IDs;
- a claim ID;
- closure contents;
- caller-selected evidence labels;
- only favourable `supports` edges; or
- only candidates that already appear admissible.

Unsupported and malformed candidates remain visible with deterministic
reasons. Closure objects must not be scanned to discover graph candidates.

## 7. Exact closed v0 lane

The first policy admits exactly:

```text
edge kind:
supports

source evidence kind:
ExternalSource

target claim domain:
ExternalReport

candidate support ceiling:
Supported

required origin-admission policy:
magpie-origin-admission-v0

artifact algorithm:
sha256
```

Positive shape:

```text
ExternalSource
× ExternalReport
× supports edge
× exact evidence artifact identity
× exact OriginAdmitted decision
->
AdmittedContributionV0
```

This lane creates no support contribution, aggregation or achieved standing.

V0 excludes:

```text
ExternalSource × Interpretation
HumanRatification
DeterministicVerification
DeadboltAnchor
ExecutionEvidence
BehavioralEvaluation
ModelSelfReport
LensReadout
refutation contributions
contradicts edges
invalidates edges
```

## 8. Exact first-failure order

Every edge candidate stops at the first failed check:

```text
1. target StandingClaim exists
2. target TypedClaimNode exists
3. edge kind is exactly supports
4. target claim_domain parses under the existing closed policy grammar
5. source TypedEvidenceNode exists
6. target/evidence/edge scopes are exactly equal
7. evidence_kind parses under the closed EvidenceKind vocabulary
8. evidence kind is exactly ExternalSource
9. claim domain is exactly ExternalReport
10. support_ceiling(ExternalSource, ExternalReport) is exactly Supported
11. TypedEvidenceNode.content_hash is non-empty
12. exact ContributionIdentityV0 and OriginComparisonNamespaceV0 are derived
13. origin-admission completion permits downstream admission
14. one exact origin-admission decision is located for the exact key
15. only OriginAdmitted produces AdmittedContributionV0
```

The implementation must reuse the existing reviewed claim-domain parsing and
support-candidate evaluation logic through the smallest private shared seam
that preserves all standing v0/v1/v2 bytes and behaviour.

It must not add a duplicate public standing engine or a second looser parser.

## 9. Closed candidate failure vocabulary

Freeze:

```text
MissingTargetClaim
MissingTypedTargetClaim
UnsupportedEdgeKindForV0
MalformedClaimMetadata
MissingClaimDomain
WrongTypeClaimDomain
DuplicateClaimDomainKey
UnknownClaimDomain
MissingSourceEvidence
ScopeMismatch
UnknownEvidenceKind
UnsupportedEvidenceKindForV0
UnsupportedClaimDomainForV0
NoSupportCeiling
CandidateCeilingMismatch
MissingEvidenceContentHash
```

`DuplicateClaimDomainKey` is the admitted-contribution audit's versioned name
for the existing fail-closed duplicate-metadata-key parser result that
prevents a unique claim-domain result. The new audit must not accept metadata
that standing v0 rejects.

`NoSupportCeiling` applies when the exact selected policy cell yields `None`.

`CandidateCeilingMismatch` applies when that exact cell yields a status other
than exactly `Supported`.

Both outcomes fail closed. No Boolean, free-form string, `Other` variant or
caller-defined reason is permitted.

## 10. Evidence-artifact identity bridge

`TypedEvidenceNode.content_hash` is asserted replay material. It is not
standalone artifact authority.

For v0:

```text
TypedEvidenceNode.content_hash
must be non-empty

artifact algorithm
must be exactly sha256
```

Derive:

```text
ContributionIdentityV0 {
    target_claim_id:
        edge.target_id

    source_evidence_id:
        edge.source_id

    justification_edge_id:
        exact retained edge ID

    scope_ref:
        exact shared target/evidence/edge scope

    artifact_algorithm:
        sha256

    artifact_digest:
        TypedEvidenceNode.content_hash
}
```

Derive:

```text
OriginComparisonNamespaceV0 {
    origin_admission_policy_id:
        magpie-origin-admission-v0

    target_claim_id:
        edge.target_id

    scope_ref:
        exact shared scope
}
```

Authority arises only from:

```text
asserted content_hash
+
exact equality with verified admitted-origin contribution artifact
=
graph node and governed provenance path refer to the same exact bytes
```

Fail closed:

```text
empty content_hash
->
not admitted

different digest
->
different ContributionIdentityV0
->
no exact admitted-origin match
->
not admitted
```

No partial identity matching or nearby-decision search is permitted.

Artifact identity must not be inferred from URI, locator, quote hash, evidence
summary, filename, metadata prose, acquisition locator, origin group or
publisher identity.

## 11. Exact origin-decision lookup key

For a graph-policy-eligible candidate, locate a decision only by:

```text
ContributionIdentityV0
+
OriginComparisonNamespaceV0
```

Exact outcomes:

```text
no exact decision
->
OriginDecisionAbsent

EligibleBindingUnresolved
->
OriginNotAdmitted with the exact nested decision

ConflictingBindings
->
OriginNotAdmitted with the exact nested decision

OriginAdmitted
->
Admitted
```

There is at most one terminal origin decision for one exact key.

Do not select by group, claim, evidence, edge, scope or digest alone; first or
last decision; newest occurrence; lexical winner; or caller order.

## 12. Internal origin-audit composition

The future resolver must call:

```rust
self.resolve_origin_admission_audit_v0(closure)
```

or one exact private shared implementation seam producing the same
`OriginAdmissionAuditV0`.

It must not accept:

```text
OriginAdmissionAuditV0
AdmittedOriginV0
OriginAdmissionDecisionV0
candidate edge list
candidate contribution list
claim ID
policy ID
authority table
trusted Boolean
caller-created snapshot
caller-created anchor index
serialized audit
callback
plugin
filesystem path
CAS reference
network client
environment
```

A cloned or serialized origin audit is audit output only. The same replay
carrier and closure determine prefix identity, closure identity, origin audit
and admitted-contribution audit.

## 13. Global completion law

Freeze:

```rust
pub enum AdmittedContributionAuditCompletionV0 {
    Complete,

    OriginAdmissionIncomplete {
        unavailable_binding_selectors:
            Vec<ArtifactProvenanceAnchorSelectorV0>,
    },
}
```

Map the internally derived completion exactly:

```text
OriginAdmissionAuditCompletionV0::Complete
->
Complete

OriginAdmissionAuditCompletionV0::IncompleteCandidateUniverse {
    unavailable_binding_selectors
}
->
OriginAdmissionIncomplete {
    unavailable_binding_selectors
}
```

The selector vector retains exact origin-admission order.

Global law:

```text
origin admission incomplete
->
admitted_contributions is empty globally
```

All graph candidates remain visible. Earlier graph-policy failures retain their
exact failure. A candidate reaching the origin stage receives
`OriginAdmissionIncomplete`.

No available local origin result may be used while the complete origin
candidate universe is unavailable.

## 14. Future public resolver

Reserve exactly:

```rust
impl OriginAdmissionReplayContextV0 {
    pub fn resolve_admitted_contribution_audit_v0(
        &self,
        closure: &ResolutionContentClosureV0,
    ) -> AdmittedContributionAuditV0;
}
```

No free resolver, `StandingView` resolver, standalone
`StandingReplaySnapshot` complete resolver or alternate authority carrier is
permitted.

## 15. Future public types

Freeze:

```text
AdmittedContributionAuditCompletionV0
AdmittedContributionCandidateReasonV0
AdmittedContributionCandidateDispositionV0
AdmittedContributionCandidateAuditV0
AdmittedContributionV0
AdmittedContributionAuditV0
```

All future public structs have private fields, `Clone`, `Debug`, `PartialEq`,
`Eq`, `Serialize`, read-only getters, no `Deserialize`, no `Default` and no
public constructor.

Closed enums have `Clone`, `Debug`, `PartialEq`, `Eq`, `Serialize`, no
`Deserialize` and no `Default`.

Enums serialize with:

```rust
#[serde(
    tag = "outcome",
    content = "details",
    rename_all = "snake_case"
)]
```

Only the top-level audit exposes `canonical_bytes()`.

## 16. Exact candidate disposition and field order

Freeze:

```rust
pub enum AdmittedContributionCandidateDispositionV0 {
    PolicyIneligible {
        reason: AdmittedContributionCandidateReasonV0,
    },

    OriginAdmissionIncomplete,

    OriginDecisionAbsent,

    OriginNotAdmitted {
        decision: OriginAdmissionDecisionV0,
    },

    Admitted {
        admitted_contribution: AdmittedContributionV0,
    },
}
```

`OriginNotAdmitted` may contain only `EligibleBindingUnresolved` or
`ConflictingBindings`. It must not reinterpret either or contain a winning
origin group.

Candidate audit field order:

```rust
pub struct AdmittedContributionCandidateAuditV0 {
    edge_id: String,
    source_evidence_id: String,
    target_claim_id: String,
    evidence_kind: Option<String>,
    claim_domain: Option<String>,
    candidate_ceiling: Option<Status>,
    candidate_contribution: Option<ContributionIdentityV0>,
    candidate_namespace: Option<OriginComparisonNamespaceV0>,
    disposition: AdmittedContributionCandidateDispositionV0,
}
```

Semantics:

- edge, source and target IDs are always retained from the candidate edge;
- evidence kind is populated only after source evidence exists;
- claim domain is populated only after exact parsing succeeds;
- candidate ceiling is populated only after exact kind/domain parsing and
  reaching the selected policy-cell check;
- contribution and namespace are populated only after every structural, lane,
  ceiling and content-hash check passes; and
- optional fields serialize explicitly as an exact value or JSON `null`.

Raw metadata, artifact bytes, foreign-bundle bytes, claim text, evidence
summary, rationale, source URI, locator and quote text are excluded.

## 17. Exact admitted-contribution and top-level field order

Admitted contribution:

```rust
pub struct AdmittedContributionV0 {
    policy_id: String,
    origin_admission_policy_id: String,
    contribution: ContributionIdentityV0,
    namespace: OriginComparisonNamespaceV0,
    origin_group: String,
    evidence_kind: String,
    claim_domain: String,
    candidate_ceiling: Status,
    supporting_origin_candidate_selectors:
        Vec<ArtifactProvenanceAnchorSelectorV0>,
}
```

Exact values:

```text
policy_id:
magpie-admitted-contribution-v0

origin_admission_policy_id:
magpie-origin-admission-v0

evidence_kind:
ExternalSource

claim_domain:
ExternalReport

candidate_ceiling:
Supported
```

Origin group and supporting selectors come only from the exact internally
derived `OriginAdmitted` result.

Top-level audit:

```rust
pub struct AdmittedContributionAuditV0 {
    schema: String,
    canonicalization_profile: String,
    policy_id: String,
    origin_admission_policy_id: String,
    verified_prefix_identity: VerifiedLogPrefixIdentityV0,
    closure_identity: ResolutionContentClosureIdentityV0,
    completion: AdmittedContributionAuditCompletionV0,
    candidate_audits: Vec<AdmittedContributionCandidateAuditV0>,
    admitted_contributions: Vec<AdmittedContributionV0>,
}
```

Only the top-level type exposes:

```rust
pub fn canonical_bytes(&self) -> Vec<u8>;
```

## 18. Deterministic ordering

Freeze:

```text
candidate audits:
exact justification-edge ID order from StandingView's BTreeMap

admitted contributions:
exact ContributionIdentityV0 order

supporting origin selectors:
exact selector order inherited from OriginAdmissionAuditV0

unavailable binding selectors:
exact selector order inherited from OriginAdmissionAuditV0
```

No caller order, closure insertion order, anchor occurrence multiplicity,
group preference or write recency selects an outcome.

## 19. Canonical serialization

The future implementation serializes directly from the typed top-level
structure:

```rust
serde_json::to_vec(self)
```

Determinism comes from exact field order, adjacently tagged closed enums,
explicit optional fields and deterministic vectors.

Do not define JCS, RFC 8785, generic canonical JSON, map sorting or another
generic Magpie canonical profile.

No raw artifact or foreign-bundle bytes appear in the audit.

## 20. Multiplicity and non-amplification law

Freeze:

```text
two distinct admitted contributions
assigned to the same admitted origin group
->
two AdmittedContributionV0 values
```

The audit preserves every exact edge, evidence node, artifact identity,
contribution identity and admitted origin assignment.

It must not collapse distinct contributions by origin group.

Layer boundary:

```text
origin-binding duplicate collapse
belongs to OriginAdmissionAuditV0

same-origin contribution non-amplification
belongs to future policy v3 aggregation

AdmittedContributionAuditV0
preserves every exact admitted contribution
```

Later aggregation alone may reduce many same-group contributions to at most one
same-origin aggregation contribution. This ticket defines no count, threshold,
quorum, score or achieved status.

## 21. Required future fixtures

The runtime implementation must add literal canonical fixtures for:

```text
admitted external-source contribution

exact graph candidate with no origin decision

origin-binding conflict

origin-binding eligible unresolved

globally incomplete origin-admission universe

two distinct contributions assigned to the same admitted origin group
```

The final fixture must retain two admitted contributions.

This contract PR adds no fixture files or hashes.

## 22. Required hostile tests

The future runtime PR must prove:

```text
valid ExternalSource × ExternalReport supports edge
+ non-empty evidence content hash
+ exact OriginAdmitted result
->
Admitted

same graph path with no origin binding
->
OriginDecisionAbsent
->
not admitted

origin conflict
->
exact nested ConflictingBindings
->
not admitted

origin eligible unresolved
->
exact nested EligibleBindingUnresolved
->
not admitted

origin audit globally incomplete
->
every graph candidate remains visible
->
zero admitted contributions globally

wrong edge kind
->
PolicyIneligible

ExternalSource × Interpretation
->
outside v0

non-ExternalSource evidence
->
outside v0

missing target claim
->
exact failure

missing typed target
->
exact failure

missing source evidence
->
exact failure

scope mismatch
->
exact failure

malformed, missing, wrong-type, duplicate or unknown claim_domain
->
distinct exact failures

empty evidence content_hash
->
not admitted

evidence content_hash differing from the verified origin contribution artifact
->
no exact contribution match
->
not admitted

two distinct edges over the same artifact and same admitted group
->
two admitted contributions

two distinct artifacts assigned to the same admitted group
->
two admitted contributions

same textual group in different namespaces
->
no relationship

serialized or cloned OriginAdmissionAuditV0
->
cannot substitute for replay and closure

caller-selected candidate list
->
no public API

same H + Pₒ + P꜀ + M
->
byte-identical audit

equivalent closure input order
->
byte-identical audit

different H with equal graph/origin projections
->
different verified-prefix identity and audit

audit execution
->
standing v0/v1/v2 bytes unchanged
->
support and refutation ceilings unchanged
->
origin-admission audit bytes unchanged
```

The tests must also pin first-failure precedence and optional-field population.

## 23. Frozen surfaces

Freeze unchanged:

```text
Magpie L0 format
payload tags
canonical event encoding
signatures
hashing
genesis
LogStore
LogReader
LogWriter
VerifiedReplaySummary
StandingView fields and bytes
StandingReplaySnapshot fields and bytes
standing v0
standing v1
standing v2
standing policy IDs
support ceilings
refutation ceilings
support-context requirements
deterministic verifier contexts
Deadbolt verifier contexts
artifact-provenance verifier
resolution-content closure
origin-binding verifier
OriginAdmissionReplayContextV0
OriginAdmissionAuditV0
OriginAdmissionDecisionV0
origin-admission fixture bytes
Cargo manifests
dependencies
lockfile
release metadata
Deadbolt
```

The future implementation may add only the six standing-inert audit types, the
exact compiled constants, the new context method, and the smallest private
shared seams required to preserve existing semantics.

## 24. Explicit non-goals

Do not define or implement:

- support contribution;
- achieved support;
- standing policy v3;
- origin-group counting;
- corroboration thresholds;
- statistical independence;
- source reputation;
- publisher identity;
- trust scores;
- numeric confidence;
- probabilistic aggregation;
- `ExternalSource × Interpretation`;
- any non-`ExternalSource` admitted-contribution lane;
- `HumanRatification` admission;
- deterministic-verifier contribution admission;
- Deadbolt contribution admission;
- refutation contribution;
- contradiction debt;
- invalidation;
- supersession or currentness;
- quote-hash verification;
- source-locator validation;
- source-standing propagation;
- graph traversal;
- transitive provenance;
- `EpistemicGate`;
- ordinary claim-bearing writer;
- loader;
- CAS;
- filesystem lookup;
- network lookup;
- callback or plugin lookup;
- new L0 payload; or
- Deadbolt changes.

## 25. Future implementation sequence

The next runtime PR must:

1. add the six exact private-construction public audit types and constants;
2. create the smallest private shared claim-domain/candidate seam without
   changing standing v0/v1/v2 results or bytes;
3. enumerate every replay-derived justification edge in exact edge-ID order;
4. internally resolve `OriginAdmissionAuditV0` from the same context and
   closure;
5. apply the exact first-failure order and content-hash bridge;
6. locate decisions by exact contribution plus namespace;
7. apply the global completion law;
8. order admitted contributions by exact `ContributionIdentityV0`;
9. serialize the typed top-level audit with `serde_json::to_vec`; and
10. add the required literal fixtures, hostile tests and compile-fail tests.

After that implementation:

```text
support-contribution contract
->
standing policy v3 contract
->
conservative aggregation
```

Every later stage remains future.

## 26. Stop conditions

Stop and report instead of inventing semantics if the contract or later
implementation would require:

- treating `content_hash` as standalone authority;
- partial contribution matching;
- accepting caller-provided `OriginAdmissionAuditV0`;
- selecting candidates from closure objects;
- beginning from only admitted origins;
- hiding unsupported edges from the candidate universe;
- admitting from incomplete origin admission;
- collapsing distinct contributions by origin group;
- including more than the one exact v0 lane;
- changing existing origin-admission types or bytes;
- changing standing v0/v1/v2;
- defining support contribution or aggregation; or
- editing outside the exact allowlist.

Publication also stops for a dirty tree, wrong repository or remote,
unavailable or unauthenticated `gh`, unmerged PR #61, reviewed-head ancestry
failure, main SHA drift, occupied named branch, existing Ticket 0049 or
equivalent PR.

## 27. Validation commands

Pre-commit validation:

```powershell
git diff --check
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo run --locked --example tour -p magpie-claims

python tools/verify_chain.py `
  crates/magpie-log/testdata/golden-v1.jsonl `
  ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c

python tools/verify_chain.py `
  fixtures/deadbolt-anchor-v1/anchor-log.jsonl `
  d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737

python tools/check_release_metadata.py
cargo package -p magpie-log --locked
```

Authority-language audit:

```powershell
git diff | Select-String -Pattern `
  'independent|independence|trusted|authority|support|standing|admit|aggregate|content_hash|artifact'
```

Diff audit:

```powershell
git status --short
git diff --name-only
git diff --stat
git diff --check

git diff -- README.md
git diff -- docs/design/standing-aggregation-independence-groups.md
git diff -- docs/design/admitted-contribution-audit-v0.md
git diff -- tickets/0049-admitted-contribution-audit-v0-contract.md
```

The changed paths must be exactly the four paths in section 3.

Before commit:

```powershell
git add `
  README.md `
  docs/design/standing-aggregation-independence-groups.md `
  docs/design/admitted-contribution-audit-v0.md `
  tickets/0049-admitted-contribution-audit-v0-contract.md

git diff --cached --name-only
git diff --cached --stat
git diff --cached --check
git diff --cached
```

After commit:

```powershell
git show --stat --oneline HEAD
git diff-tree --no-commit-id --name-only -r HEAD
git rev-list --count 3419c1d17b78d777347f8a6b4526f37a5cc4e58a..HEAD
git status --short
```

Every reported pass must have actually run. If validation requires an edit
outside the allowlist, stop rather than broadening the PR.

## 28. Publication authority

`AGENTS.md` remains binding except that George authorizes, only after the exact
documentation change and validation pass:

- staging only the four allowlisted paths;
- creating exactly one commit named
  `docs: ratify admitted-contribution audit v0`;
- pushing only `agent/admitted-contribution-audit-v0-contract` without force;
  and
- opening exactly one draft PR titled
  `docs: ratify admitted-contribution audit v0`.

The worker must not modify `main`, create another branch or commit, mark the PR
ready, merge, enable auto-merge, force-push, resolve reviews or modify another
PR.

## 29. Reviewer checklist

1. Confirm the exact base, reviewed PR #61 head, branch, one-commit boundary
   and four changed paths.
2. Confirm status is ratified contract, runtime absent and standing unchanged.
3. Confirm `H + Pₒ + P꜀ + M` is the complete deterministic input.
4. Confirm both policies are exact compiled identities with no runtime
   selector.
5. Confirm every retained edge is audited in exact `BTreeMap` edge-ID order.
6. Confirm no caller subset, closure scan or origin-admitted-first universe.
7. Confirm the only positive lane is
   `supports × ExternalSource × ExternalReport × Supported × sha256`.
8. Confirm the exact first-failure order and closed reason vocabulary.
9. Confirm the existing parser/candidate logic is shared privately without
   changing standing v0/v1/v2 behaviour or bytes.
10. Confirm `content_hash` remains asserted and gains authority only through
    exact equality with the verified origin contribution artifact.
11. Confirm complete contribution and namespace equality is the only lookup
    key.
12. Confirm the future resolver internally derives origin admission from the
    same replay context and closure.
13. Confirm caller-provided, cloned or serialized origin audit values cannot
    substitute.
14. Confirm incomplete origin admission yields zero admitted contributions
    globally while preserving all graph candidate audits.
15. Confirm only exact `OriginAdmitted` yields an admitted contribution.
16. Confirm all six future public types, field order, traits and enum tagging.
17. Confirm only the top-level audit has `canonical_bytes()`.
18. Confirm deterministic order and direct typed `serde_json::to_vec`
    serialization.
19. Confirm distinct contributions sharing one origin group remain distinct.
20. Confirm same-origin non-amplification belongs only to future aggregation.
21. Confirm all required fixtures and hostile tests are reserved for the
    runtime PR.
22. Confirm every frozen surface remains unchanged.
23. Confirm no support contribution, aggregation, standing, writer, loader,
    CAS, filesystem/network, L0, Cargo, dependency, fixture or Deadbolt change
    appears.

The precise contract claim is:

```text
Magpie now has a ratified contract for deriving a complete,
deterministic and standing-inert audit of which replay-derived
ExternalSource × ExternalReport graph contributions are structurally
eligible, refer to the exact verified artifact identity, and possess
an admitted origin-group assignment.

No runtime admitted-contribution resolver, support contribution,
aggregation or standing change exists yet.
```
