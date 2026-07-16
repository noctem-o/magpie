# Ticket 0050: admitted-contribution audit v0 implementation

## 1. Exact base and reviewed contract head

Implementation base:

```text
45032ae42caaabe52672157f3cf8b29d13e5b5e3
```

Reviewed Ticket 0049 / PR #62 contract head:

```text
48b9a0138f5137e7dee1d453a256a07c8ccd4194
```

PR #62 was required to be merged, its reviewed head was required to be an
ancestor of `main`, and `main` was required to resolve exactly to the
implementation base before this ticket could create a branch or edit files.
Preflight confirmed all three conditions, a clean worktree, the expected
`noctem-o/magpie` remote, authenticated `gh`, and no existing local branch,
remote branch, Ticket 0050 or equivalent implementation PR.

## 2. Branch, commit and PR title

```text
branch:
agent/admitted-contribution-audit-v0-implementation

commit:
claims: implement admitted-contribution audit v0

PR title:
claims: implement admitted-contribution audit v0
```

The pull request must be opened as a draft against `main`.

## 3. Exact changed-path allowlist

Only these paths may change:

```text
README.md
tickets/0050-admitted-contribution-audit-v0-implementation.md
crates/magpie-claims/src/lib.rs
crates/magpie-claims/src/admitted_contribution_audit.rs
crates/magpie-claims/src/origin_admission_replay.rs
crates/magpie-claims/src/origin_binding_verifier.rs
crates/magpie-claims/src/standing.rs
crates/magpie-claims/tests/admitted_contribution_audit.rs
fixtures/admitted-contribution-audit-v0/admitted.json
fixtures/admitted-contribution-audit-v0/decision-absent.json
fixtures/admitted-contribution-audit-v0/conflict.json
fixtures/admitted-contribution-audit-v0/eligible-unresolved.json
fixtures/admitted-contribution-audit-v0/incomplete-origin.json
fixtures/admitted-contribution-audit-v0/same-origin-distinct-contributions.json
```

No Cargo, L0, policy, origin-audit, closure, release, Deadbolt or existing
fixture path is in scope.

## 4. Central deterministic law

```text
exact verified replay H
+
compiled origin-admission policy Pₒ
+
compiled admitted-contribution policy P꜀
+
immutable content closure M
→
deterministic standing-inert admitted-contribution audit
```

The authority law is:

```text
exact replay-derived graph contribution
+
exact v0 policy-lane validation
+
exact evidence artifact identity
+
exact internally derived OriginAdmitted result
→
AdmittedContributionV0
```

## 5. Exact compiled constants

The implementation defines and re-exports exactly:

```rust
pub const ADMITTED_CONTRIBUTION_AUDIT_SCHEMA_V0: &str =
    "magpie-admitted-contribution-audit-v0";

pub const ADMITTED_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0: &str =
    "magpie-admitted-contribution-audit-json-v0";

pub const ADMITTED_CONTRIBUTION_POLICY_ID_V0: &str =
    "magpie-admitted-contribution-v0";

pub const ADMITTED_CONTRIBUTION_ARTIFACT_ALGORITHM_V0: &str =
    "sha256";
```

The implementation reuses `ORIGIN_ADMISSION_POLICY_ID_V0`; it does not define
another origin-policy constant. These constants disclose compiled policy
identity and do not select runtime policy.

## 6. Complete candidate universe

The resolver enumerates every `TypedJustificationEdge` retained in the accepted
`StandingReplaySnapshot`, directly from `StandingView.justification_edges` in
its `BTreeMap` edge-ID order. Every retained edge produces exactly one
`AdmittedContributionCandidateAuditV0`.

The resolver does not begin from origin decisions, origin-binding candidates,
closure objects, caller-selected edges, one claim, supports-only edges,
favourable evidence kinds or favourable claim domains. The closure remains an
availability universe rather than a graph index.

## 7. Exact v0 lane

After structural evaluation succeeds, the only admitted-contribution-v0 lane
is:

```text
supports
× ExternalSource
× ExternalReport
× Supported
× non-empty sha256 content hash
× exact OriginAdmitted decision
```

Known non-`ExternalSource` evidence is
`UnsupportedEvidenceKindForV0`. `ExternalSource` with a known non-
`ExternalReport` domain is `UnsupportedClaimDomainForV0`. A missing selected
policy cell is `NoSupportCeiling`; a selected non-`Supported` cell is
`CandidateCeilingMismatch`; an empty evidence `content_hash` is
`MissingEvidenceContentHash`.

## 8. Private standing evaluation seam

`standing.rs` exposes only crate-private shared structure:

```rust
pub(crate) enum SupportCandidateStructureFailureV0 {
    MissingTargetClaim,
    MissingTypedTargetClaim,
    UnsupportedEdgeKindForV0,
    MalformedMetadata,
    MissingClaimDomain,
    WrongTypeClaimDomain,
    DuplicateMetadataKey,
    UnknownClaimDomain,
    MissingSourceEvidence,
    ScopeMismatch,
    UnknownEvidenceKind,
}
```

```rust
pub(crate) struct SupportCandidateStructureEvaluationV0<'a> {
    failure: Option<SupportCandidateStructureFailureV0>,
    evidence: Option<&'a TypedEvidenceNode>,
    claim_domain: Option<ClaimDomain>,
    evidence_kind: Option<EvidenceKind>,
}
```

The shared entry point is:

```rust
StandingView::evaluate_support_candidate_structure_v0(
    target_claim_id,
    edge,
)
```

It owns the exact first seven checks. Both standing v0 and the new audit map
this private typed result into their own existing/versioned public
vocabularies. `parse_claim_domain` remains the one parser implementation. The
private evaluator has no serialization, public constructor or caller-supplied
result.

## 9. Proof of standing-v0 compatibility

Standing keeps its historical trace construction separate from the new
audit's optional-field staging. In particular, the existing standing trace
continues to pre-populate the same fields at the same historical points even
when the shared evaluator fails earlier.

Before editing, these base suites passed:

```text
cargo test -p magpie-claims --locked --lib
92 passed

cargo test -p magpie-claims --locked --test standing_resolution
14 passed

cargo test -p magpie-claims --locked --test deterministic_support_standing_v2
40 passed
```

The implementation tests pin the literal accepted standing-v0 canonical bytes
and representative historical field population for malformed metadata and an
unsupported edge. The existing standing-resolution and deterministic
standing-v2 suites are rerun after the refactor. Expected standing bytes are
not changed or re-blessed.

## 10. Private graph identity constructors

`origin_binding_verifier.rs` adds only these crate-private construction seams:

```rust
ContributionIdentityV0::from_admitted_contribution_graph_v0(
    target_claim_id,
    source_evidence_id,
    justification_edge_id,
    scope_ref,
    artifact_digest,
)
```

It hardcodes the existing SHA-256 artifact algorithm.

```rust
OriginComparisonNamespaceV0::for_admitted_contribution_v0(
    target_claim_id,
    scope_ref,
)
```

It hardcodes the existing `ORIGIN_ADMISSION_POLICY_ID_V0`. Neither type gains a
public constructor, `Deserialize`, authority or admission semantics.

## 11. Internal origin-audit composition

The resolver accepts exactly one private-construction
`OriginAdmissionReplayContextV0` and one immutable
`ResolutionContentClosureV0`. Inside the resolver it derives exactly one:

```rust
let origin_audit =
    context.resolve_origin_admission_audit_v0(closure);
```

It asserts equality of verified-prefix identity, closure identity and the
compiled origin policy. It accepts no origin audit, origin decision, admitted
origin, snapshot, anchor index, serialized output, policy selector, callback,
plugin, path, CAS or network authority.

## 12. Origin-decision index

When origin admission is complete, the implementation builds:

```rust
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct OriginDecisionKeyV0 {
    contribution: ContributionIdentityV0,
    namespace: OriginComparisonNamespaceV0,
}
```

and:

```rust
BTreeMap<
    OriginDecisionKeyV0,
    &OriginAdmissionDecisionV0,
>
```

`ConflictingBindings`, `EligibleBindingUnresolved` and `OriginAdmitted` keys
come only from their complete embedded contribution and namespace. Duplicate
exact keys are asserted impossible; there is no first, last, newest, lexical
or caller-order winner.

## 13. Exact 15-stage precedence

The implementation evaluates in this literal order:

1. target `StandingClaim` exists;
2. target `TypedClaimNode` exists;
3. edge kind is exactly `supports`;
4. claim metadata and closed claim-domain parsing succeed;
5. source `TypedEvidenceNode` exists;
6. target, evidence and edge scopes are equal;
7. `EvidenceKind` parses through the closed vocabulary;
8. evidence kind is exactly `ExternalSource`;
9. claim domain is exactly `ExternalReport`;
10. `support_ceiling(ExternalSource, ExternalReport)` is exactly `Supported`;
11. evidence `content_hash` is non-empty;
12. exact contribution and namespace identities are derived;
13. origin-admission completion permits downstream admission;
14. one exact origin decision is found for the complete key;
15. only `OriginAdmitted` creates `AdmittedContributionV0`.

The mappings preserve:

```text
StandingTraceReason::MalformedMetadata
→
AdmittedContributionCandidateReasonV0::MalformedClaimMetadata

StandingTraceReason::DuplicateMetadataKey
→
AdmittedContributionCandidateReasonV0::DuplicateClaimDomainKey
```

Hostile precedence tests pin unsupported evidence before unsupported domain,
malformed domain before missing evidence, and unsupported edge before malformed
domain.

## 14. Optional-field staging

Candidate fields are staged exactly:

```text
edge_id:
always the retained edge key

source_evidence_id:
always edge.source_id

target_claim_id:
always edge.target_id

evidence_kind:
the raw replay string once source evidence exists

claim_domain:
only after exact closed domain parsing succeeds

candidate_ceiling:
only after evidence-kind and claim-domain lane selection reaches
the policy-cell check

candidate_contribution and candidate_namespace:
only after structural, lane, ceiling and non-empty hash checks succeed
```

Every option serializes explicitly as a value or JSON `null`. No raw metadata,
claim statement, evidence summary, rationale, URI, locator, quote, artifact,
bundle or publisher bytes are included.

## 15. Global completion law

Completion maps exactly:

```text
OriginAdmissionAuditCompletionV0::Complete
→
AdmittedContributionAuditCompletionV0::Complete
```

```text
OriginAdmissionAuditCompletionV0::IncompleteCandidateUniverse {
    unavailable_binding_selectors
}
→
AdmittedContributionAuditCompletionV0::OriginAdmissionIncomplete {
    unavailable_binding_selectors
}
```

The selector vector is cloned without reordering. When origin admission is
incomplete, every graph candidate remains visible, all earlier graph/policy
failures remain exact, every otherwise eligible candidate terminates as
`OriginAdmissionIncomplete`, and `admitted_contributions` is globally empty.
No locally available origin decision is used.

## 16. Public audit types

The module re-exports exactly:

```text
AdmittedContributionAuditCompletionV0
AdmittedContributionCandidateReasonV0
AdmittedContributionCandidateDispositionV0
AdmittedContributionCandidateAuditV0
AdmittedContributionV0
AdmittedContributionAuditV0
```

All public structs have private fields, `Clone`, `Debug`, `PartialEq`, `Eq`,
`Serialize`, read-only getters, no `Deserialize`, no `Default` and no public
constructor. Closed public enums have `Clone`, `Debug`, `PartialEq`, `Eq`,
`Serialize`, no `Deserialize` and no `Default`, using:

```rust
#[serde(
    tag = "outcome",
    content = "details",
    rename_all = "snake_case"
)]
```

The only new public resolver is:

```rust
impl OriginAdmissionReplayContextV0 {
    pub fn resolve_admitted_contribution_audit_v0(
        &self,
        closure: &ResolutionContentClosureV0,
    ) -> AdmittedContributionAuditV0;
}
```

Only `AdmittedContributionAuditV0` exposes `canonical_bytes()`.

## 17. Deterministic ordering

Ordering is structural:

```text
candidate_audits:
StandingView justification-edge BTreeMap order

admitted_contributions:
ContributionIdentityV0 BTreeMap order

supporting origin selectors:
exact AdmittedOriginV0 order

unavailable binding selectors:
exact OriginAdmissionAuditV0 order
```

Exact duplicate admitted contribution identities and duplicate origin-decision
keys assert rather than overwrite.

## 18. Canonical serialization

The top-level audit serializes directly:

```rust
serde_json::to_vec(self)
```

Determinism comes from field declaration order, adjacent enum tagging,
explicit optional fields, `BTreeMap` traversal and inherited selector order.
There is no `serde_json::Value` production path, `HashMap`, JCS, RFC 8785,
generic canonical JSON or new Magpie encoding.

## 19. Multiplicity law

Two distinct `ContributionIdentityV0` values assigned to the same textual
origin group remain two `AdmittedContributionV0` values. The output is keyed
and ordered by exact contribution identity, never by group.

```text
origin-binding duplicate collapse
→
OriginAdmissionAuditV0

exact contribution preservation
→
AdmittedContributionAuditV0

same-origin non-amplification
→
future aggregation policy
```

This implementation does not perform the third layer.

## 20. Normative fixtures

All fixtures are compact UTF-8, begin with `0x7b`, end with `0x7d`, and have no
BOM, surrounding whitespace or trailing newline.

| Fixture | Bytes | SHA-256 |
| --- | ---: | --- |
| `fixtures/admitted-contribution-audit-v0/admitted.json` | 3495 | `886d0159fc275c8505c1d4ec634fcec7a5975f4443881e577348f890e8d00f93` |
| `fixtures/admitted-contribution-audit-v0/decision-absent.json` | 1509 | `0613b47aecf02f7d2e40e2e3016b39eefb26aa74d148ef88870b6bb311b9d13b` |
| `fixtures/admitted-contribution-audit-v0/conflict.json` | 2710 | `c35d9c4b0c59c7140f56c994637a444205e9ae0220159939d1a45cb3c8bdf118` |
| `fixtures/admitted-contribution-audit-v0/eligible-unresolved.json` | 2394 | `99a2d62d443dff44970dd51572e1a70e107581376fb5381a404a453a52af074b` |
| `fixtures/admitted-contribution-audit-v0/incomplete-origin.json` | 1815 | `bb115589fd97ffc88ce3175188216cd3213a7b862fad8031a8c9942cbe7d81e8` |
| `fixtures/admitted-contribution-audit-v0/same-origin-distinct-contributions.json` | 6485 | `929dc86024af6b37918ee3e9ea74a09aa3557cff1cd00c87a115f48102501584` |

Each test constructs deterministic signed history and closure input, calls the
real resolver, compares production canonical bytes to the committed literal,
parses the literal independently and checks schema, policy IDs, completion,
candidate order, dispositions, contribution order, groups and selector order.
A separate uncommitted insertion-ordered JSON reconstruction built all six
expected objects from hard-coded reviewed values, compared them byte-for-byte
with the fixtures, and independently checked lengths and hashes.

## 21. Hostile tests

Focused tests cover:

- complete trusted admission;
- every replay edge and exact edge-ID ordering;
- all reachable first-failure stages and optional-field staging;
- full-key content-hash mismatch with no nearby matching;
- empty content hash;
- global incomplete-origin suppression with earlier failures preserved;
- exact conflict and eligible-unresolved cloning;
- two edges over one artifact and group;
- two artifacts in one group;
- namespace isolation;
- repeated deterministic resolution;
- closure construction order and materially different closure identity;
- different verified histories with equal standing snapshots;
- standing, policy, verifier and origin-audit inertia;
- canonical fixture bytes and absence of raw objects.

The frozen L0 validator rejects unknown evidence-kind strings before accepted
replay. The private shared evaluator and audit reason mapper therefore exercise
`UnknownEvidenceKind` with synthetic crate-private unit inputs rather than
weakening L0. `NoSupportCeiling` and `CandidateCeilingMismatch` are likewise
tested through the smallest private policy-drift unit seam without public test
controls.

## 22. Compile-fail tests

Documentation tests prove:

1. `AdmittedContributionCandidateAuditV0` has no struct-literal construction;
2. `AdmittedContributionV0` has no struct-literal construction;
3. `AdmittedContributionAuditV0` has no struct-literal construction;
4. none of the six public types implements `Deserialize`;
5. no resolver accepts a caller-selected edge list;
6. no resolver accepts a caller-created `OriginAdmissionAuditV0`;
7. no resolver accepts `OriginAdmissionDecisionV0`;
8. no resolver accepts a policy ID;
9. no resolver accepts a caller-created snapshot or anchor index;
10. no resolver accepts serialized or cloned audit output;
11. `StandingView` has no admitted-contribution resolver;
12. standalone `StandingReplaySnapshot` has no complete resolver;
13. `ContributionIdentityV0` still has no public constructor;
14. `OriginComparisonNamespaceV0` still has no public constructor.

No test-only public constructor is added.

## 23. Standing and origin-audit inertia

Tests compare before and after admitted-audit execution:

```text
StandingView canonical bytes
StandingReplaySnapshot canonical bytes
standing v0 canonical bytes
standing v1 canonical bytes
standing v2 canonical bytes
support ceilings
refutation ceilings
support-context requirements
deterministic-verifier traces
origin-binding traces
OriginAdmissionAuditV0 canonical bytes
```

The admitted-contribution resolver mutates none of them. It is a deterministic,
standing-inert read over one verified replay context and one immutable closure.

## 24. Frozen surfaces

This ticket changes no Magpie L0 format, payload vocabulary, event encoding,
hashing, signatures, genesis, log API, verified-prefix fields or bytes,
standing public fields or bytes, standing reason vocabulary, standing v0/v1/v2
policy, policy ceiling, support-context rule, deterministic-verifier context,
Deadbolt context, artifact-provenance public verifier, closure public surface,
origin-binding public verifier, identity public API, origin replay-context
construction, origin-audit public type or bytes, Cargo manifest, dependency,
lockfile, release metadata or Deadbolt path.

Changes in frozen-owning files are limited to the private standing evaluator,
the two private graph identity constructors and the one exact context resolver.

## 25. Explicit non-goals

This ticket does not implement support-contribution contract, support
contribution, achieved support, standing policy v3, group counting,
corroboration thresholds, aggregation, statistical independence, reputation,
publisher identity, confidence, probability, a second lane,
`ExternalSource × Interpretation`, HumanRatification admission,
deterministic-verifier contribution admission, Deadbolt contribution admission,
refutation contribution, contradiction debt, invalidation, supersession,
currentness, quote verification, locator validation, source-standing
propagation, graph traversal, transitive provenance, `EpistemicGate`, writer
authority, loader, CAS, filesystem access, network access, callback/plugin
lookup, a new L0 payload or Deadbolt changes.

## 26. Correctness, authority and standing statement

```text
correctness:
complete replay-derived admitted-contribution audit

authority:
exact graph contribution
+ exact verified artifact identity
+ exact admitted origin only

standing:
unchanged

support contribution:
not implemented

aggregation:
not implemented
```

`AdmittedContributionV0` establishes structural validity in the one exact lane,
exact verified-artifact identity through the provenance/origin path, and one
exact admitted origin-group assignment. It does not establish truth,
relevance, freshness, reputation, publisher identity, independence,
corroboration, support contribution, aggregation, standing or writer
authority.

## 27. Stop conditions

Stop and report rather than broaden the change if implementation requires:

- changed standing trace/resolution bytes or first-failure precedence;
- a duplicate or looser claim-domain parser;
- a public identity constructor;
- caller-provided origin audits or decisions;
- partial contribution matching;
- closure-derived graph selection;
- beginning from origin decisions instead of all edges;
- admission from incomplete origin audit;
- contribution collapse by origin group;
- changed origin-audit bytes;
- changed policy ceilings, standing v1/v2, Cargo metadata or Magpie L0;
- a dependency or any path outside the allowlist.

If the private standing extraction cannot preserve exact standing output, the
correct action is a separate reviewed private-refactor PR.

## 28. Validation commands

Focused and repeated validation:

```powershell
cargo test -p magpie-claims --locked --lib
cargo test -p magpie-claims --locked --doc
cargo test -p magpie-claims --locked --test origin_admission_audit
cargo test -p magpie-claims --locked --test deterministic_support_standing_v2
cargo test -p magpie-claims --locked --test admitted_contribution_audit

cargo test -p magpie-claims --locked --test admitted_contribution_audit
cargo test -p magpie-claims --locked --test origin_admission_audit
cargo test -p magpie-claims --locked --test deterministic_support_standing_v2
cargo test -p magpie-claims --locked --test admitted_contribution_audit
```

Full validation:

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
```

Fixture endpoint, BOM, trailing-newline, byte-length and SHA-256 checks are
required for all six literal files. After the one commit, clean-tree release
validation is:

```powershell
python tools/check_release_metadata.py
cargo package -p magpie-log --locked
```

Only commands that actually run and pass may be reported as passing.

Final pre-commit evidence:

```text
cargo test -p magpie-claims --locked --lib
95 passed

cargo test -p magpie-claims --locked --doc
67 passed

cargo test -p magpie-claims --locked --test origin_admission_audit
18 passed

cargo test -p magpie-claims --locked --test deterministic_support_standing_v2
40 passed

cargo test -p magpie-claims --locked --test admitted_contribution_audit
19 passed

cargo test -p magpie-claims --locked --test standing_resolution
14 passed

required repeated high-risk order
19 + 18 + 40 + 19 passed

git diff --check
passed

cargo fmt --all --check
passed

cargo clippy --workspace --all-targets --locked -- -D warnings
passed

cargo test --workspace --locked
passed

cargo run --locked --example tour -p magpie-claims
passed

tools/verify_chain.py golden-v1
9 records verified

tools/verify_chain.py deadbolt-anchor-v1
2 records verified

six fixture endpoint, BOM, newline, length and SHA-256 checks
passed

independent hard-coded insertion-ordered JSON construction
all six byte-identical
```

## 29. Publication authority

After every required validation passes, publication authority is limited to:

- stage only the exact allowlist;
- create exactly one commit with the required message;
- push only the named branch without force;
- open exactly one draft PR with the required title.

The worker may not modify `main`, mark the PR ready, merge, enable auto-merge,
force-push, create another branch or commit, or expand runtime authority.

## 30. Reviewer checklist

- [ ] Base and reviewed contract head are exact.
- [ ] Diff contains exactly the allowlisted paths.
- [ ] Candidate enumeration begins from every retained edge in BTreeMap order.
- [ ] The shared evaluator preserves standing-v0 reason and byte behaviour.
- [ ] Claim-domain parsing has one implementation.
- [ ] The only lane is `ExternalSource × ExternalReport × Supported`.
- [ ] Content hash is asserted replay material and full-key matching is exact.
- [ ] Identity constructors are crate-private and hardcode compiled constants.
- [ ] Origin audit is internally derived exactly once from the same context and closure.
- [ ] Decision index uses full contribution plus namespace and rejects duplicates.
- [ ] Incomplete origin admission globally suppresses admitted output.
- [ ] Conflict and unresolved decisions are cloned exactly without a winner.
- [ ] Distinct same-origin contributions remain distinct.
- [ ] Public structs have private fields and no `Deserialize` or constructors.
- [ ] All six fixture lengths and hashes match this ticket.
- [ ] Standing v0/v1/v2 and `OriginAdmissionAuditV0` bytes remain unchanged.
- [ ] No support contribution, aggregation, independence or standing is claimed.
- [ ] One commit, clean worktree, normal push and draft PR state are verified.

## 31. Publication claim

The precise claim is:

```text
Magpie can now deterministically audit every replay-derived
justification edge under one exact admitted-contribution policy,
bind eligible ExternalSource × ExternalReport graph material to
the exact verified artifact and admitted-origin identity, and
report whether each contribution is ineligible, origin-incomplete,
decision-absent, origin-conflicted, origin-unresolved or admitted.

The result remains standing-inert. It creates no support
contribution, aggregation, independence or achieved standing.
```
