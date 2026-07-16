# Ticket 0048: Origin-admission audit v0 implementation

## 1. Publication identity and exact base

```text
base: 5795ce2e62e9f2851bb6fd806079553918eed89a
reviewed PR #60 head: 8e64551e62c17e461f4954d3ab1c019a26052239
branch: agent/origin-admission-audit-v0-implementation
commit: claims: implement origin-admission audit v0
PR title: claims: implement origin-admission audit v0
PR state: draft
```

The base is the merge commit of PR #60. The reviewed PR #60 implementation
head is an ancestor of that exact base. This ticket implements the second
runtime slice ratified by Ticket 0046 after Ticket 0047 established the
same-replay carrier and complete candidate universe.

The exact classification is:

```text
correctness:
complete same-replay origin-admission audit

authority:
exact origin-group assignment only

standing:
unchanged

support:
not implemented

aggregation:
not implemented
```

## 2. Exact changed-path allowlist

Only:

```text
README.md
tickets/0048-origin-admission-audit-v0-implementation.md
crates/magpie-claims/src/lib.rs
crates/magpie-claims/src/origin_admission_replay.rs
crates/magpie-claims/src/origin_admission_audit.rs
crates/magpie-claims/src/origin_binding_verifier.rs
crates/magpie-claims/tests/origin_binding_verifier.rs
crates/magpie-claims/tests/origin_admission_audit.rs
fixtures/origin-admission-audit-v0/admitted.json
fixtures/origin-admission-audit-v0/duplicate-collapse.json
fixtures/origin-admission-audit-v0/conflict.json
fixtures/origin-admission-audit-v0/scoped-unresolved.json
fixtures/origin-admission-audit-v0/incomplete-universe.json
```

No Cargo, log, episodic, artifact-provenance, closure, replay-snapshot,
Deadbolt-context, deterministic-verifier, standing, policy, design, tool, CI
or existing-fixture path is in scope.

## 3. Central deterministic law

```text
H:
the exact completely verified Magpie log prefix carried by
OriginAdmissionReplayContextV0

P:
the exact compiled policy magpie-origin-admission-v0

M:
the exact immutable ResolutionContentClosureV0 and its complete
four-field identity

complete supported candidate universe from H
+
exact immutable closure M
+
compiled policy P
->
deterministic standing-inert origin-admission audit
```

Same `H + P + M` produces byte-identical audit output. Candidate selection
comes only from the same-replay context. Closure order, caller order,
duplicate input order and anchor occurrence multiplicity select no outcome.

The audit establishes only which candidates were examined, which bytes were
unavailable, which statements were rejected, which valid statements targeted
another policy or untrusted authority, which trusted assignments matched or
remained availability-unresolved, and which exact contribution received a
duplicate, unresolved, conflicting or admitted opaque origin-group result.

```text
verified origin binding
!= trusted authority

trusted origin assignment
!= support contribution

origin admission
!= aggregation

distinct admitted groups
!= statistical independence

candidate audit
!= standing decision

serialized audit output
!= replay authority
```

## 4. Exact compiled policy identity

The implementation defines and re-exports:

```rust
pub const ORIGIN_ADMISSION_AUDIT_SCHEMA_V0: &str =
    "magpie-origin-admission-audit-v0";

pub const ORIGIN_ADMISSION_AUDIT_CANONICALIZATION_PROFILE_V0: &str =
    "magpie-origin-admission-audit-json-v0";

pub const ORIGIN_ADMISSION_POLICY_ID_V0: &str =
    "magpie-origin-admission-v0";
```

Reviewed code selects the exact policy. There is no policy argument, registry,
alias, negotiation, environment override, implicit latest version or
bundle-selected implementation.

## 5. Exact trusted authority pair

The implementation defines and re-exports:

```rust
pub const ORIGIN_ADMISSION_TRUSTED_AUTHORITY_KIND_V0: &str =
    "human-review";

pub const ORIGIN_ADMISSION_TRUSTED_AUTHORITY_REFERENCE_V0: &str =
    "authority:origin-review-v0";
```

Both fields must match. This pair authorizes only one exact opaque
origin-group assignment for one exact contribution and namespace. It grants
no truth, support, reputation, independence, aggregation, standing or writer
authority. No second trusted class, hierarchy, score or caller table exists.

## 6. Public audit types

The crate adds and re-exports exactly:

```text
OriginAdmissionAuditCompletionV0
OriginAdmissionCandidateDispositionV0
OriginAdmissionCandidateAuditV0
AdmittedOriginV0
OriginBindingConflictV0
OriginAdmissionDecisionV0
OriginAdmissionAuditV0
```

All public structs have private fields, `Clone`, `Debug`, `PartialEq`, `Eq`
and `Serialize`, with read-only getters only. They have no `Deserialize`,
`Default` or public constructor. The closed enums have the same derives
except no struct privacy rule, and serialize with:

```rust
#[serde(
    tag = "outcome",
    content = "details",
    rename_all = "snake_case"
)]
```

The enum variant order and struct field order are exactly the Ticket 0046
contract. Optional validated contribution and namespace fields always
serialize in place as an exact object or JSON `null`. Only the top-level
`OriginAdmissionAuditV0` exposes:

```rust
pub fn canonical_bytes(&self) -> Vec<u8>;
```

## 7. Public audit entry point

The only new public resolver entry point is:

```rust
impl OriginAdmissionReplayContextV0 {
    pub fn resolve_origin_admission_audit_v0(
        &self,
        closure: &ResolutionContentClosureV0,
    ) -> OriginAdmissionAuditV0;
}
```

It accepts one private-construction same-replay context and one successfully
constructed immutable closure. It accepts no candidate list, receipt list,
candidate audit list, policy ID, authority table, trusted Boolean, callback,
plugin, filesystem path, CAS reference, network client, environment,
serialized audit, caller-created anchor index or caller-created prefix
identity. A standalone `StandingReplaySnapshot` has no complete audit method.

## 8. Private origin-binding evaluation refactor

The existing public method retains its exact signature and delegates to one
crate-private evaluator:

```rust
pub(crate) fn evaluate_origin_binding_context_v0(
    snapshot: &StandingReplaySnapshot,
    binding_selector: &ArtifactProvenanceAnchorSelectorV0,
    closure: &ResolutionContentClosureV0,
) -> OriginBindingEvaluationV0;
```

The exact private types are:

```rust
pub(crate) enum OriginBindingEvaluationClassV0 {
    BindingBytesUnavailable,
    DefinitivelyRejected,
    AvailabilityOnly,
    Matched,
}

pub(crate) struct ValidatedOriginBindingSubjectV0 {
    contribution: ContributionIdentityV0,
    namespace: OriginComparisonNamespaceV0,
    claimed_policy_id: String,
    claimed_origin_group: String,
    authority_kind: String,
    authority_reference: String,
}

pub(crate) struct OriginBindingEvaluationV0 {
    trace: OriginBindingContextTraceV0,
    class: OriginBindingEvaluationClassV0,
    validated_subject: Option<ValidatedOriginBindingSubjectV0>,
}
```

`OriginBindingEvaluationV0::into_parts()` supplies the audit execution with
the unchanged public trace, the private class and safely validated subject.
These private types are neither serialized nor deserialized.

The public method remains:

```rust
pub fn resolve_origin_binding_context_v0(
    &self,
    binding_selector: &ArtifactProvenanceAnchorSelectorV0,
    closure: &ResolutionContentClosureV0,
) -> OriginBindingContextTraceV0
```

and returns only `evaluate_origin_binding_context_v0(...).into_trace()`. There
is one parser and verifier pipeline, not a second audit parser.

## 9. Validated-subject threshold

`ValidatedOriginBindingSubjectV0` is created only after all of:

```text
valid UTF-8 and JSON
closed schema and semantic validity
canonical byte equality
exact bundle family
exact selector algorithm, profile and run identity
exact witness-root equality
same-replay binding anchor occurrence
target claim exists
source evidence exists
justification edge exists
edge source matches
edge target matches
claim, evidence and edge scope equality
```

At that point the exact contribution, namespace, claimed policy, claimed
group, authority kind and authority reference are safe audit material. The
subject is constructed before nested artifact-provenance resolution so a
pure availability failure can retain the exact validated subject.

Parser material before canonical validation, selector/root/anchor failures,
unrelated replay IDs, mismatched edge relationships and mismatched scopes are
never exposed as validated. A matched receipt is asserted to agree exactly
with the validated contribution, namespace, policy, group and authority.
Late definitive failures may retain the already-safe subject.

## 10. Global candidate-byte completeness

Candidates are derived only through:

```rust
self.origin_binding_candidates_v0()
```

The audit never enumerates closure keys. For every candidate in exact selector
order it retains the selector, all occurrences in ascending sequence, one
shared evaluator result, the complete public trace, safely validated
contribution and namespace, and one disposition.

If every candidate binding is present:

```text
Complete
```

If any exact binding bytes are absent from `closure.foreign_bundle(selector)`:

```text
IncompleteCandidateUniverse {
    unavailable_binding_selectors
}
```

Every candidate audit remains visible in exact candidate order, but
`contribution_decisions` is exactly empty and no available candidate enters
the fold. Extra unanchored closure bundles are ignored.

## 11. Exact candidate disposition precedence

Every candidate is classified exactly once in this order:

```text
1. BindingBytesUnavailable
2. DefinitivelyRejected
3. PolicyMismatch
4. AuthorityUntrusted
5. TrustedEligibleUnresolved
6. TrustedMatched
```

`BindingBytesUnavailable` is only exact candidate binding absence and has no
validated subject. `DefinitivelyRejected` covers every other early or late
failure and is never reclassified by parsed policy or authority.

Only `AvailabilityOnly` and `Matched` evaluations reach policy
classification. Wrong policy is visible and fold-inert. After exact policy
match, any wrong authority field is visible and fold-inert. Only selected-
policy, trusted-authority `AvailabilityOnly` becomes
`TrustedEligibleUnresolved`; only selected-policy, trusted-authority `Matched`
becomes `TrustedMatched`.

Wrong-policy and untrusted candidates cannot admit, conflict, veto or block.

## 12. Exact availability-only classification

`AvailabilityOnly` is used only after full structural subject validation when
nested resolution stops solely on:

```text
ArtifactProvenanceContextTraceV0::BundleUnavailable
ArtifactProvenanceContextTraceV0::ArtifactUnavailable
ArtifactProvenanceContextTraceV0::ParentArtifactUnavailable
ArtifactProvenanceContextTraceV0::DerivedArtifactUnavailable
```

This covers unavailable acquisition or derivation prerequisite bundles and
unavailable acquisition, parent or derived artifacts.

Malformed nested bundles, selector/root/anchor failures, digest mismatches and
artifact-coherence failures are `DefinitivelyRejected`, even if the origin-
binding subject was already safely validated.

## 13. Trusted fold inputs

The trusted fold runs only for a complete candidate universe and consumes
private validated material from the current execution. It does not parse or
deserialize candidate audit JSON, dispositions, traces, receipts or a prior
audit.

The exact private key is:

```rust
struct ContributionAdmissionKeyV0 {
    contribution: ContributionIdentityV0,
    namespace: OriginComparisonNamespaceV0,
}
```

The namespace remains exactly:

```text
origin_admission_policy_id
target_claim_id
scope_ref
```

Only `TrustedMatched` and `TrustedEligibleUnresolved` contribute. The
implementation uses a `BTreeMap` by exact key, a `BTreeMap` by matched group
and `BTreeSet` selector partitions. Every other disposition is audit-visible
and fold-inert.

## 14. Duplicate law

For one exact key and group:

```text
N trusted matched candidate selectors
->
one matched origin-group partition
->
N selector references retained
```

There is no multiplicity weight, confidence increase, majority, quorum,
anchor-count amplification or receipt-count amplification. Repeated exact
anchor occurrences remain attached to one candidate and never create
additional fold inputs.

## 15. Conflict, unresolved and admission precedence

For every exact contribution-admission key:

```text
1. two or more distinct matched trusted groups
   -> ConflictingBindings

2. otherwise one or more trusted eligible unresolved candidates
   -> EligibleBindingUnresolved

3. otherwise exactly one matched trusted group
   -> OriginAdmitted

4. otherwise
   -> no decision
```

A conflict uses lexical group order, flattens matched selectors by group then
selector order, retains unresolved selectors, and admits no group. No first,
last, newest, oldest, lexical, majority or occurrence-count winner exists.

An eligible unresolved decision has a non-empty unresolved selector vector,
at most one matched group, exact matched selector order and exact unresolved
selector order. It admits no group.

An admitted origin assigns only the exact contribution to the exact opaque
group inside the exact namespace under `magpie-origin-admission-v0`. It does
not create support, truth, independence, reputation, aggregation or standing.

## 16. Deterministic ordering

Ordering is structural:

```text
candidate audits:
exact five-field candidate-selector order

anchor occurrences:
ascending event sequence

contribution decisions:
ContributionIdentityV0 order
then OriginComparisonNamespaceV0 order

origin groups:
exact lexical string order

selectors inside one matched group:
exact selector order

conflict matched selector flattening:
group order
then selector order

unresolved selectors:
exact selector order
```

`BTreeMap` and `BTreeSet` establish the order rather than repairing it after
an order-sensitive fold.

## 17. Canonical serialization

Only the top-level audit implements:

```rust
pub fn canonical_bytes(&self) -> Vec<u8> {
    serde_json::to_vec(self)
        .expect("origin-admission audits are always serializable")
}
```

Determinism comes from exact field declaration order, exact enum tagging,
deterministic vectors, ordered folds and existing deterministic nested typed
serialization. The audit does not serialize through `serde_json::Value`,
`HashMap`, a caller map, a generic canonical-JSON registry, RFC 8785, JCS or
Magpie L0.

The bytes use fixed object-field order, exact vector order, compact UTF-8 JSON,
short control escapes, lowercase `\u00xx` for remaining controls, direct UTF-8
otherwise, no Unicode normalization, no whitespace, no BOM, no trailing
newline and shortest unsigned decimal integers. Raw artifacts and foreign
bundle bytes never appear in an audit.

## 18. Normative fixture identities

The five committed files are literal canonical audit bytes:

```text
fixtures/origin-admission-audit-v0/admitted.json
bytes: 6303
sha256: b10ca460595b9402d52e483d4ff0aa592ca6aef4996316f446cfb29cd28b8a8f

fixtures/origin-admission-audit-v0/duplicate-collapse.json
bytes: 11433
sha256: a2b4ee3559598ab970f74e841dadc2898fbdb7dd91ee4170182c1da735175131

fixtures/origin-admission-audit-v0/conflict.json
bytes: 11436
sha256: a20515271e5479c01e1a952ff1607826fa4e5de89bc4957fcef5d2ef48d92494

fixtures/origin-admission-audit-v0/scoped-unresolved.json
bytes: 8190
sha256: f2d31a951324a49c8694eb0ca5e6c914f0e31601962e920eefc97f16d45f7b41

fixtures/origin-admission-audit-v0/incomplete-universe.json
bytes: 6783
sha256: 9a5ea4489be404055193578cc88e395c1e50d34953403e99a2c7db2a0eeb7a3f
```

Every fixture begins with `0x7b`, ends with `0x7d`, is valid UTF-8, has no BOM,
surrounding whitespace or trailing newline, and retains exact field and vector
order.

Integration tests construct deterministic signed histories and closures, run
the real resolver, compare exact production bytes with each literal, parse the
literals independently and verify selectors, dispositions, groups, completion
and decisions. A temporary uncommitted insertion-ordered JSON calculation
must independently re-emit and compare all five files before publication.

## 19. Hostile and compatibility tests

The audit suite covers complete admission, global missing-binding failure,
unanchored closure bundles, repeated occurrences, duplicate collapse,
conflict, wrong-policy and untrusted non-veto, same-key unresolved blocking,
conflict precedence, scoped unresolved isolation, definitive nested failures,
disposition precedence, namespace separation, deterministic repeated
execution, closure order independence, different closure identity, different
verified history identity and standing inertia.

Compile-fail documentation proves private construction, no deserialization for
all seven types, no caller selector list, no caller anchor index, no policy or
authority parameters, no serialized or cloned audit authority, no standalone
snapshot resolver and `OriginAdmissionReplayContextV0` as the only complete
public replay carrier.

Origin-binding regression tests compare the unchanged public resolver with the
private evaluator for matched acquisition, matched derivation, binding absence,
malformed binding, selector/root mismatch, replay mismatch, unavailable nested
bundle, unavailable artifact, malformed nested provenance, artifact digest
mismatch and artifact-coherence failure. Frozen public trace byte lengths and
SHA-256 values are pinned from the required base.

## 20. Standing-inert preservation

Audit execution mutates no replay, projection, closure or policy state. Tests
compare before and after:

```text
StandingReplaySnapshot canonical bytes
StandingView canonical bytes
standing v0 canonical bytes
standing v1 canonical bytes
standing v2 canonical bytes
support ceilings
refutation ceilings
deterministic verifier traces
origin-binding public traces
```

All remain byte-for-byte unchanged. The audit is serialized output, not replay
authority, and no resolver accepts it back as authority.

## 21. Frozen surfaces

This slice changes none of:

```text
magpie-core-v1
magpie-sig-v1
Magpie L0 payload vocabulary
canonical event encoding
hashing
signatures
genesis
LogStore
LogReader
LogWriter
VerifiedReplaySummary
VerifiedLogPrefixIdentityV0 fields or bytes
OriginAdmissionReplayContextV0 construction
StandingReplaySnapshot fields or canonical bytes
StandingView canonical bytes
DeadboltAnchorIndex public API or canonical bytes
origin-binding public resolver signature
OriginBindingContextTraceV0 variants, order or canonical bytes
OriginBindingReceiptV0 fields, order or serialized bytes
artifact-provenance public APIs, trace variants or bytes
ResolutionContentClosureV0 construction
ResolutionContentClosureIdentityV0 fields or bytes
deterministic verifier APIs or receipts
standing policy v0
standing policy v1
standing policy v2
support ceilings
refutation ceilings
Cargo manifests
dependencies
lockfile
existing fixtures
Deadbolt
```

The only origin-binding change is the crate-private evaluation seam preserving
every existing public result and first-failure precedence.

## 22. Explicit non-goals

This ticket does not implement admitted-contribution audit, support
contribution, standing policy v3, support or refutation aggregation,
statistical independence, corroboration thresholds, confidence, probability,
publisher ontology, source reputation, trust scoring, authority hierarchy,
multiple trusted authority classes, supersession, revocation, expiry,
temporal ownership, loader, CAS, filesystem or network access, callback or
plugin lookup, mutable policy registry, writer authority, new L0 payloads or
Deadbolt changes.

## 23. Stop conditions

Stop and report contract divergence if implementation requires changing public
origin-binding trace or receipt bytes, public origin-binding failure
precedence, duplicating the parser, exposing unvalidated material, treating a
malformed or digest-mismatched prerequisite as availability-only, accepting
caller-selected candidates, enumerating closure keys, admitting from an
incomplete universe, allowing wrong-policy or untrusted veto, choosing a
conflict winner, modifying standing or policy code, changing snapshot bytes,
adding a dependency, changing Cargo metadata or L0, or editing outside the
exact allowlist.

If the private evaluator cannot preserve public bytes cleanly, the audit stops
for a separately reviewed private-refactor PR.

## 24. Validation commands

Focused:

```powershell
cargo test -p magpie-claims --locked --lib
cargo test -p magpie-claims --locked --doc
cargo test -p magpie-claims --locked --test origin_binding_verifier
cargo test -p magpie-claims --locked --test origin_admission_replay_substrate
cargo test -p magpie-claims --locked --test origin_admission_audit
cargo test -p magpie-claims --locked --test origin_binding_verifier
cargo test -p magpie-claims --locked --test origin_admission_audit
cargo test -p magpie-claims --locked --test origin_admission_audit
```

Full:

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

Fixture endpoint, BOM, newline, byte-length and SHA-256 checks run against all
five new files. After the one exact commit, clean-tree release validation is:

```powershell
python tools/check_release_metadata.py
cargo package -p magpie-log --locked
git status --short
```

Any generated-file, inventory or out-of-allowlist release mismatch is a stop.

## 25. Publication authority

After every pre-commit validation passes, stage only the thirteen permitted
paths, create exactly one commit named:

```text
claims: implement origin-admission audit v0
```

Push only:

```text
agent/origin-admission-audit-v0-implementation
```

without force and open exactly one draft PR titled:

```text
claims: implement origin-admission audit v0
```

The PR remains open and draft with base `main`. This ticket authorizes no
second branch, second commit, ready transition, merge, auto-merge, force push,
review resolution or modification of another PR.

## 26. Reviewer checklist

- Confirm the exact base, branch, one-commit boundary and thirteen changed
  paths.
- Confirm candidates come only from the same-replay carrier and extra closure
  keys are ignored.
- Confirm the public origin-binding signature, trace variants, receipt fields,
  first-failure precedence and pinned bytes are unchanged.
- Confirm validated subject material appears only after the exact structural
  threshold.
- Confirm only the four nested availability outcomes are `AvailabilityOnly`.
- Confirm definitive failure precedes policy and authority classification.
- Confirm policy mismatch precedes authority classification for late eligible
  outcomes.
- Confirm wrong-policy and untrusted candidates cannot veto, conflict, block or
  admit.
- Confirm one missing candidate binding globally empties all decisions.
- Confirm duplicate selectors are retained without amplification.
- Confirm conflict, unresolved and admission precedence and all ordering laws.
- Confirm the five literal fixture lengths, hashes, endpoints and independent
  JSON calculation.
- Confirm no raw artifact or foreign-bundle bytes enter audit serialization.
- Confirm standing v0, v1 and v2 bytes, ceilings and verifier outputs remain
  unchanged.
- Confirm no Cargo, dependency, L0, writer, loader, network, filesystem,
  artifact-provenance, closure, standing, policy or Deadbolt change.

The precise implementation claim is:

```text
Magpie can now examine the complete supported origin-binding
candidate universe from one exact verified history and one exact
immutable content closure, classify every candidate under one
compiled policy and one trusted authority path, and deterministically
derive duplicate, unresolved, conflicting or admitted origin-group
decisions.

The audit remains standing-inert. It creates no support,
aggregation, independence or claim standing.
```
