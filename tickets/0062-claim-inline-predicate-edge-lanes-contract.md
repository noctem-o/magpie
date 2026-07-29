# Ticket 0062: Claim-Inline Predicate Edge Lanes Contract

## Status

Documentation-only contract candidate complete in the direct checkout.
Runtime is absent.

```text
Ticket 0059 predicate runtime:
landed

Ticket 0061 attestation-binding runtime:
landed through merged PR #76

Ticket 0062 edge-lane contract:
ratified in this candidate documentation patch

next:
claim-inline predicate edge-lane runtime v0
```

## 1. Starting point and direct-checkout execution

The initial primary checkout state was:

```text
checkout:
C:\magpie

starting branch:
agent/inline-predicate-attestation-binding-runtime

starting branch HEAD:
aa44802cfc690d3cb3e44dfd92a8c9c4a8f83768

mode:
direct checkout; no worktree created
```

The clean starting branch was not modified. The exact synchronization
sequence fetched `origin`, switched to `main`, and fast-forwarded only.

The synchronized ticket base is:

```text
ba9f444e84fcfd3fc756fc45f816d910ced9b83c
Merge pull request #76 from noctem-o/agent/inline-predicate-attestation-binding-runtime
```

Observed after synchronization:

```text
local main:
ba9f444e84fcfd3fc756fc45f816d910ced9b83c

origin/main:
ba9f444e84fcfd3fc756fc45f816d910ced9b83c

divergence:
0 0

status:
clean
```

The proposed branch did not exist (`git show-ref --verify --quiet` exited 1),
so the exact branch was created:

```text
agent/claim-inline-predicate-edge-lanes-contract
```

Its creation point was the exact required base. No merge, rebase, reset,
force operation, branch deletion, worktree creation, stash, clean, restore,
commit, push, publication, GitHub mutation, or backup-branch change occurred.

## 2. Proposed publication title

Proposed commit and draft PR title:

```text
docs: ratify claim-inline predicate edge lanes v0
```

Publication remains owner-only. This task does not commit, push, create or
edit a pull request, mark readiness, resolve or reply to review threads,
change GitHub metadata, or merge.

## 3. Documentation-only classification

```text
correctness:
one closed same-snapshot composition contract for the claim-owned inline
SHA-256 predicate, exact attestation binding, and one replayed edge

authority:
documentation only; no runtime authority path changes

runtime:
absent; implementation is a later separately reviewed ticket

predicate:
unchanged; sha256_claim_inline_bytes_equals_v0 remains neutral

binding:
unchanged; Ticket 0061 Bound remains routing-only

edges:
one future exact edge is interpreted only by the new closed lane composer

polarity:
lane classification only; no achieved standing

policy:
no standing policy v4 ratified

standing:
unchanged

support:
no new governed support and no change to policy v2

refutation:
no governed Refuted result

aggregation:
absent

contradiction debt:
absent

tests and fixtures:
unchanged

canonical L0 bytes:
unchanged

derived audit bytes:
one future profile and documentation vectors ratified

dependencies:
unchanged
```

No Rust, test, fixture, Cargo, lockfile, CI, release, format, L0, writer,
loader, filesystem, CAS, network, plugin, model, or Deadbolt behavior is
added or changed.

## 4. Exact changed-path allowlist

Only these eight paths may change:

```text
README.md
docs/design/claim-inline-subject-binding-v0.md
docs/design/claim-inline-sha256-predicate-v0.md
docs/design/inline-predicate-attestation-binding-v0.md
docs/design/claim-inline-predicate-edge-lanes-v0.md
docs/design/standing-view-evidence-ceilings.md
docs/design/standing-aggregation-independence-groups.md
tickets/0062-claim-inline-predicate-edge-lanes-contract.md
```

Expected new paths:

```text
docs/design/claim-inline-predicate-edge-lanes-v0.md
tickets/0062-claim-inline-predicate-edge-lanes-contract.md
```

All other paths are prohibited, including `AGENTS.md`, historical Tickets
0001-0061, Rust, tests, fixtures, Cargo manifests, `Cargo.lock`,
`docs/FORMAT.md`, L0, policy/standing/verifier code, generated files, CI,
release/package metadata, tools, secrets, `.env*`, GitHub metadata, and
`backup/main-before-sync-20260729-043839`.

## 5. Architectural prerequisites verified

The required cold read established:

- **Ticket 0056 blocker.** The withdrawn policy-v4 candidate allowed a
  `contradicts` evidence node to supply arbitrary unrelated bytes. Digest
  inequality over those evidence-selected bytes manufactured a negative
  result for a true claim. No v4 identity, rule, resolver, receipt, or
  conflict blocker is active.
- **Claim-owned immutable subject.** Ticket 0057 places one exact inline byte
  subject and expected digest operand inside the claim's canonical statement.
  Exact three-way statement equality and a separate statement-content-hash
  recomputation bind the claim. Evidence cannot replace the subject.
- **Ticket 0059 relation.** The compiled
  `sha256_claim_inline_bytes_equals_v0` evaluator resolves one requested claim
  from one `StandingReplaySnapshot`, computes only over claim-owned bytes, and
  yields neutral `DigestEqual`, `DigestUnequal`, or `ResolutionFailed`.
  Failures are never inequality or falsity. Its opaque outcome and receipt
  have no public `Debug`, construction, deserialization, or authority
  reinjection.
- **Ticket 0061 binding.** The snapshot-only two-ID resolver shares the private
  claim-resolution primitive, re-fetches exact
  `DeterministicVerification` evidence, and binds predicate, claim, and scope.
  `Bound` is routing-only. The outer 28-stage failure order preserves the
  inner 33-stage claim failure exactly. Evidence `content_hash`, actor class,
  summary, and every edge are non-authority.
- **Replayed edge shape.** `TypedJustificationEdge` retains exactly
  `edge_kind`, `source_id`, `target_id`, `scope_ref`, `actor_class`,
  `rationale`, and `metadata_json`. `StandingView::justification_edge` does an
  exact map lookup. Replay is first-write-wins for duplicate edge IDs.
- **Compiled ceiling cells.**
  `DeterministicVerification × ExactMachineCheckable` is
  `Some(Settled)` in `support_ceiling` and `Some(Refuted)` in
  `refutation_ceiling`. Both are candidate ceilings, not achieved standing.
- **Existing v2 positive rule.** Policy v2 remains tied to the historical
  evidence-local `sha256_bytes_equals_v0` verifier context. Only `Matched`
  creates its direct `Supported` application; `DigestMismatch` is
  non-authority. This contract does not replace or feed v2.
- **Existing support audit.** `SupportContributionAuditV0` operates on
  internally admitted `ExternalSource × ExternalReport` provenance/origin
  graph contributions. It is distinct from the new exact claim/evidence/edge
  lane and is unchanged.
- **Roadmap.** Ticket 0061 is landed through PR #76. This ticket freezes only
  the standing-inert edge-lane contract. Next is its runtime. Only after that
  may a fresh direct-refutation policy contract be considered.

The normative contract is:

[`docs/design/claim-inline-predicate-edge-lanes-v0.md`](../docs/design/claim-inline-predicate-edge-lanes-v0.md)

This ticket and the design note agree on all authority, API, matrix, failure,
receipt, canonical-byte, repetition, and non-effect laws.

## 6. Mission and central same-snapshot law

The only closed family is:

```text
predicate:
sha256_claim_inline_bytes_equals_v0

claim domain:
ExactMachineCheckable

evidence kind:
DeterministicVerification

eligible edge kinds:
supports
contradicts
```

The contract answers:

```text
Given one exact replay snapshot, one exact requested claim, one exact
requested evidence node, and one exact requested edge, does the internally
derived predicate relation align with that replayed edge strongly enough to
create one standing-inert, ceiling-bounded support or refutation lane input?
```

The compact law is:

```text
one StandingReplaySnapshot
+ exact requested claim_id
+ exact requested evidence_id
+ exact requested edge_id
+ one internally shared claim-inline resolution
+ exact same-snapshot attestation binding
+ one exact replayed edge
+ exact relation/edge matrix cell
+ exact compiled candidate-ceiling invariant
→ SupportEligible
  | RefutationEligible
  | Ineligible
  | ResolutionFailed
```

Every material value is internally re-fetched or derived from the one
snapshot. No caller-created outcome, receipt, parsed object, relation, edge,
lane, Boolean, policy result, callback, serialized bytes, or other snapshot is
accepted as authority.

The contract does not decide truth, achieved standing, policy admission,
aggregation, debt, invalidation, supersession, currentness, writer authority,
or whether a later policy should choose `Refuted`.

## 7. Closed relation × edge matrix

| Predicate relation | Edge kind | Result |
| --- | --- | --- |
| `DigestEqual` | `supports` | `SupportEligible` |
| `DigestUnequal` | `supports` | `Ineligible: DigestUnequalDoesNotSupport` |
| `DigestEqual` | `contradicts` | `Ineligible: DigestEqualDoesNotRefute` |
| `DigestUnequal` | `contradicts` | `RefutationEligible` |

Normative distinctions:

```text
supports + DigestUnequal != refutation
contradicts + DigestEqual != support
DigestUnequal without exact contradicts != negative authority
contradicts without DigestUnequal != negative authority
DigestEqual without exact supports != positive lane authority
supports without DigestEqual != positive lane authority
```

`Ineligible` is successful standing-inert classification of a valid
polarity-misaligned pair. It is not malformed and not
`ResolutionFailed`. `ResolutionFailed` is not falsity, contradiction,
refutation, or debt.

## 8. Exact future API and type vocabulary

The sole future public resolver is:

```rust
impl StandingReplaySnapshot {
    pub fn resolve_claim_inline_predicate_edge_lane_v0(
        &self,
        claim_id: &str,
        evidence_id: &str,
        edge_id: &str,
    ) -> ClaimInlinePredicateEdgeLaneOutcomeV0;
}
```

There is no free function, `StandingView` method,
`OriginAdmissionReplayContextV0` method, raw-material composer, generic
registry, caller-selected policy/lane, or overload taking authority objects.

The future non-authority vocabularies are exactly:

```rust
pub enum ClaimInlinePredicateEdgeLaneOutcomeKindV0 {
    SupportEligible,
    RefutationEligible,
    Ineligible,
    ResolutionFailed,
}

pub enum ClaimInlinePredicateEdgeLaneKindV0 {
    Support,
    Refutation,
}

pub enum ClaimInlinePredicateEdgeLaneRelationV0 {
    DigestEqual,
    DigestUnequal,
}

pub enum ClaimInlinePredicateEdgeLaneEdgeKindV0 {
    Supports,
    Contradicts,
}

pub enum ClaimInlinePredicateEdgeLaneIneligibleReasonV0 {
    DigestUnequalDoesNotSupport,
    DigestEqualDoesNotRefute,
}
```

The future opaque authority-bearing types are:

```text
ClaimInlinePredicateEdgeLaneOutcomeV0
ClaimInlinePredicateEdgeLaneReceiptV0
```

The closed failure vocabulary is:

```rust
pub enum ClaimInlinePredicateEdgeLaneFailureV0 {
    AttestationBindingFailed(
        InlinePredicateAttestationBindingFailureV0,
    ),
    MissingEdge,
    UnsupportedEdgeKind,
    EdgeSourceBindingMismatch,
    EdgeTargetBindingMismatch,
    EdgeScopeBindingMismatch,
    SupportCeilingInvariantMismatch,
    RefutationCeilingInvariantMismatch,
}
```

No additional lane, family, edge kind, threshold, generic polarity, conflict
result, or policy result is ratified.

## 9. Exact derivation, edge law, and failure order

One future invocation:

1. preserves the exact three requested IDs;
2. invokes one private same-snapshot claim-inline resolution path;
3. validates exact Ticket 0061 binding from the same snapshot;
4. preserves failure as `AttestationBindingFailed(exact nested failure)`;
5. re-fetches the exact edge through
   `StandingReplaySnapshot::standing().justification_edge(edge_id)`;
6. requires edge kind exactly `supports` or `contradicts`;
7. requires exact `edge.source_id == requested evidence_id`;
8. requires exact `edge.target_id == requested claim_id`;
9. requires exact `edge.scope_ref == already bound scope`;
10. computes the claim-owned digest relation exactly once;
11. classifies the exact matrix cell;
12. checks the compiled support ceiling only for an otherwise eligible
    support cell;
13. checks the compiled refutation ceiling only for an otherwise eligible
    refutation cell; and
14. privately constructs one standing-inert outcome.

Raw exact replay IDs are used. There is no `claim:`, `evidence:`, or `edge:`
prefix, normalization, case folding, trimming, wildcard, scope inheritance,
ontology, edge enumeration, or “best edge” selection.

The exact first-failure order is:

1. `AttestationBindingFailed(exact nested Ticket 0061 failure)`
2. `MissingEdge`
3. `UnsupportedEdgeKind`
4. `EdgeSourceBindingMismatch`
5. `EdgeTargetBindingMismatch`
6. `EdgeScopeBindingMismatch`
7. `SupportCeilingInvariantMismatch`
8. `RefutationCeilingInvariantMismatch`

Matrix mismatch is never a failure. Ineligible cells do not consult a ceiling.
Ceiling failures are reachable only for their corresponding otherwise eligible
cell.

The nested shape:

```text
AttestationBindingFailed(
    ClaimPredicateResolutionFailed(
        ClaimInlineSubjectResolutionFailureV0
    )
)
```

retains both inherited levels. No Ticket 0061 or Ticket 0059 reason is
flattened, renamed, or reordered. The design note lists every exact nested
snake-case spelling.

The future implementation may privately share primitives. It may not accept a
public Ticket 0059 outcome or public Ticket 0061 outcome/receipt as authority,
duplicate the claim parser, mix snapshots, hash evidence-supplied bytes, or
replay audit serialization.

## 10. Candidate ceilings and ineligible semantics

Support eligibility carries exactly:

```text
lane: Support
relation: DigestEqual
edge kind: Supports
candidate ceiling: Settled
```

Refutation eligibility carries exactly:

```text
lane: Refutation
relation: DigestUnequal
edge kind: Contradicts
candidate ceiling: Refuted
```

The compiled invariants remain:

```text
support_ceiling(
    DeterministicVerification,
    ExactMachineCheckable,
) == Some(Settled)

refutation_ceiling(
    DeterministicVerification,
    ExactMachineCheckable,
) == Some(Refuted)
```

```text
candidate Settled != achieved Settled
candidate Refuted != achieved Refuted
SupportEligible != governed Supported or Settled
RefutationEligible != governed Refuted
```

An `Ineligible` outcome retains the exact query IDs and one exact reason. It
proves only that all structural/binding prerequisites succeeded and the exact
relation did not align with the requested edge polarity. It grants no
opposite-polarity authority and no achieved standing.

## 11. Opaque outcome and receipt

Outcome inspection is exactly:

```rust
pub fn kind(
    &self,
) -> ClaimInlinePredicateEdgeLaneOutcomeKindV0;
pub fn requested_claim_id(&self) -> &str;
pub fn requested_evidence_id(&self) -> &str;
pub fn requested_edge_id(&self) -> &str;
pub fn receipt(
    &self,
) -> Option<&ClaimInlinePredicateEdgeLaneReceiptV0>;
pub fn ineligible_reason(
    &self,
) -> Option<ClaimInlinePredicateEdgeLaneIneligibleReasonV0>;
pub fn failure(
    &self,
) -> Option<&ClaimInlinePredicateEdgeLaneFailureV0>;
pub fn canonical_bytes(&self) -> Vec<u8>;
```

Receipt inspection is exactly:

```rust
pub fn schema(&self) -> &str;
pub fn lane(&self) -> ClaimInlinePredicateEdgeLaneKindV0;
pub fn predicate_id(&self) -> &str;
pub fn claim_id(&self) -> &str;
pub fn evidence_id(&self) -> &str;
pub fn edge_id(&self) -> &str;
pub fn scope_ref(&self) -> &str;
pub fn predicate_relation(
    &self,
) -> ClaimInlinePredicateEdgeLaneRelationV0;
pub fn edge_kind(
    &self,
) -> ClaimInlinePredicateEdgeLaneEdgeKindV0;
pub fn candidate_ceiling(&self) -> Status;
```

The receipt contains exactly, in order:

```text
schema
lane
predicate_id
claim_id
evidence_id
edge_id
scope_ref
predicate_relation
edge_kind
candidate_ceiling
```

Schema:

```text
magpie-claim-inline-predicate-edge-lane-v0
```

Eligible requested IDs borrow the receipt's sole strings. Ineligible and
failed outcomes privately retain the exact three query IDs and exactly one
reason/failure. Those retained ID strings are attribution only and, by
themselves, do not prove that any requested node existed.

Outcome and receipt have private representation/construction, no public fields
or constructor, no literal construction, `Deserialize`, `Default`,
`From`/`TryFrom`, mutation, consuming deconstruction, `Debug`, `Display`,
`Error`, equality, ordering, hashing, or reinjection. `Clone` and one-way
`Serialize` may exist for audit only. Classification values and audit bytes
are never accepted back as authority.

## 12. Canonical derived audit profile

The exact future profile is:

```text
magpie-claim-inline-predicate-edge-lane-outcome-json-v0
```

It uses direct typed `serde_json::to_vec` over private fixed-field wire views.
There is no `Value`, map, JCS/RFC 8785, generic canonical JSON, Magpie L0, or
deserializable authority form.

Outer order:

```text
outcome
details
```

Eligible outcomes:

```text
support_eligible
refutation_eligible
```

Receipt strings:

```text
lane: support | refutation
predicate_relation: digest_equal | digest_unequal
edge_kind: supports | contradicts
candidate_ceiling: settled | refuted
```

Ineligible and ordinary failure details are:

```text
claim_id
evidence_id
edge_id
reason
```

Nested binding failure details append:

```text
binding_reason
```

and, only when that binding reason is
`claim_predicate_resolution_failed`, append:

```text
claim_reason
```

The top-level nested reason is exactly `attestation_binding_failed`.
Every Ticket 0061 `binding_reason` and every Ticket 0059 `claim_reason`
retains its existing snake-case spelling.

Encoding is compact UTF-8 with fixed declaration order and standard
`serde_json` escaping, direct non-control Unicode, no normalization, no
escaped slash, no BOM, no insignificant whitespace, and no trailing newline.

## 13. Six complete documentation vectors

### A. `SupportEligible`

```json
{"outcome":"support_eligible","details":{"receipt":{"schema":"magpie-claim-inline-predicate-edge-lane-v0","lane":"support","predicate_id":"sha256_claim_inline_bytes_equals_v0","claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-support","scope_ref":"scope-s","predicate_relation":"digest_equal","edge_kind":"supports","candidate_ceiling":"settled"}}}
```

```text
UTF-8 bytes: 362
SHA-256: 3d1e241b92e8b66c7e42613231a68e4df990c910d48627415ad29db8dc67c085
```

### B. `RefutationEligible`

```json
{"outcome":"refutation_eligible","details":{"receipt":{"schema":"magpie-claim-inline-predicate-edge-lane-v0","lane":"refutation","predicate_id":"sha256_claim_inline_bytes_equals_v0","claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-refute","scope_ref":"scope-s","predicate_relation":"digest_unequal","edge_kind":"contradicts","candidate_ceiling":"refuted"}}}
```

```text
UTF-8 bytes: 372
SHA-256: 2849f779e12e756fce866294053d89cae9413254c5edab78221e82ac2d3b3c06
```

### C. `Ineligible(DigestUnequalDoesNotSupport)`

```json
{"outcome":"ineligible","details":{"claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-support","reason":"digest_unequal_does_not_support"}}
```

```text
UTF-8 bytes: 152
SHA-256: 793302a14a991b45ff6a5c6c15c04d936664c29c3685f25d0e509f26c8e983b8
```

### D. `Ineligible(DigestEqualDoesNotRefute)`

```json
{"outcome":"ineligible","details":{"claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-refute","reason":"digest_equal_does_not_refute"}}
```

```text
UTF-8 bytes: 148
SHA-256: 2f320d25d7b5d3f3c5c45185cf33abb025d2a6f7a93f5721d157c3318eac469c
```

### E. `ResolutionFailed(MissingEdge)`

```json
{"outcome":"resolution_failed","details":{"claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-missing","reason":"missing_edge"}}
```

```text
UTF-8 bytes: 140
SHA-256: 6ad4b261939e8289b1ee67157e74a37423888e5baf33204802f178e7c4033bc9
```

### F. Nested binding failure: `MissingClaim`

```json
{"outcome":"resolution_failed","details":{"claim_id":"claim-missing","evidence_id":"evidence-e","edge_id":"edge-e","reason":"attestation_binding_failed","binding_reason":"claim_predicate_resolution_failed","claim_reason":"missing_claim"}}
```

```text
UTF-8 bytes: 238
SHA-256: 942b5ec8232429b7301ba489d7aedf214ad32a77be751d59d80bc0707432a62b
```

Markdown line endings are not vector bytes.

## 14. Independent vector recomputation

Both mechanisms used the complete literals above, not generated runtime code.

Python command/script executed from `C:\magpie`:

```powershell
@'
import hashlib, json
vectors = [
("A_support_eligible", '{"outcome":"support_eligible","details":{"receipt":{"schema":"magpie-claim-inline-predicate-edge-lane-v0","lane":"support","predicate_id":"sha256_claim_inline_bytes_equals_v0","claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-support","scope_ref":"scope-s","predicate_relation":"digest_equal","edge_kind":"supports","candidate_ceiling":"settled"}}}'),
("B_refutation_eligible", '{"outcome":"refutation_eligible","details":{"receipt":{"schema":"magpie-claim-inline-predicate-edge-lane-v0","lane":"refutation","predicate_id":"sha256_claim_inline_bytes_equals_v0","claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-refute","scope_ref":"scope-s","predicate_relation":"digest_unequal","edge_kind":"contradicts","candidate_ceiling":"refuted"}}}'),
("C_ineligible_unequal_support", '{"outcome":"ineligible","details":{"claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-support","reason":"digest_unequal_does_not_support"}}'),
("D_ineligible_equal_refute", '{"outcome":"ineligible","details":{"claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-refute","reason":"digest_equal_does_not_refute"}}'),
("E_missing_edge", '{"outcome":"resolution_failed","details":{"claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-missing","reason":"missing_edge"}}'),
("F_nested_missing_claim", '{"outcome":"resolution_failed","details":{"claim_id":"claim-missing","evidence_id":"evidence-e","edge_id":"edge-e","reason":"attestation_binding_failed","binding_reason":"claim_predicate_resolution_failed","claim_reason":"missing_claim"}}'),
]
for name, literal in vectors:
    parsed = json.loads(literal)
    rebuilt = json.dumps(parsed, ensure_ascii=False, separators=(",", ":"))
    raw = literal.encode("utf-8")
    assert rebuilt == literal
    assert not raw.startswith(b"\xef\xbb\xbf")
    assert not raw.endswith((b"\n", b"\r"))
    assert b"\/" not in raw
    print(name, literal, len(raw), hashlib.sha256(raw).hexdigest(), sep="\n")
'@ | python -
```

Independent PowerShell/.NET mechanism:

```powershell
$vectors = @(
  @('A_support_eligible', '{"outcome":"support_eligible","details":{"receipt":{"schema":"magpie-claim-inline-predicate-edge-lane-v0","lane":"support","predicate_id":"sha256_claim_inline_bytes_equals_v0","claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-support","scope_ref":"scope-s","predicate_relation":"digest_equal","edge_kind":"supports","candidate_ceiling":"settled"}}}'),
  @('B_refutation_eligible', '{"outcome":"refutation_eligible","details":{"receipt":{"schema":"magpie-claim-inline-predicate-edge-lane-v0","lane":"refutation","predicate_id":"sha256_claim_inline_bytes_equals_v0","claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-refute","scope_ref":"scope-s","predicate_relation":"digest_unequal","edge_kind":"contradicts","candidate_ceiling":"refuted"}}}'),
  @('C_ineligible_unequal_support', '{"outcome":"ineligible","details":{"claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-support","reason":"digest_unequal_does_not_support"}}'),
  @('D_ineligible_equal_refute', '{"outcome":"ineligible","details":{"claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-refute","reason":"digest_equal_does_not_refute"}}'),
  @('E_missing_edge', '{"outcome":"resolution_failed","details":{"claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-missing","reason":"missing_edge"}}'),
  @('F_nested_missing_claim', '{"outcome":"resolution_failed","details":{"claim_id":"claim-missing","evidence_id":"evidence-e","edge_id":"edge-e","reason":"attestation_binding_failed","binding_reason":"claim_predicate_resolution_failed","claim_reason":"missing_claim"}}')
)
$utf8 = [System.Text.UTF8Encoding]::new($false, $true)
$sha = [System.Security.Cryptography.SHA256]::Create()
try {
  foreach ($vector in $vectors) {
    $bytes = $utf8.GetBytes([string]$vector[1])
    $digest = $sha.ComputeHash($bytes)
    $hex = -join ($digest | ForEach-Object { $_.ToString('x2') })
    "$($vector[0])`n$($vector[1])`nbytes=$($bytes.Length)`nsha256=$hex"
  }
}
finally {
  $sha.Dispose()
}
```

Observed agreement:

| Vector | Python bytes/hash | .NET bytes/hash | Agreement |
| --- | --- | --- | --- |
| A | 362 / `3d1e241b92e8b66c7e42613231a68e4df990c910d48627415ad29db8dc67c085` | same | yes |
| B | 372 / `2849f779e12e756fce866294053d89cae9413254c5edab78221e82ac2d3b3c06` | same | yes |
| C | 152 / `793302a14a991b45ff6a5c6c15c04d936664c29c3685f25d0e509f26c8e983b8` | same | yes |
| D | 148 / `2f320d25d7b5d3f3c5c45185cf33abb025d2a6f7a93f5721d157c3318eac469c` | same | yes |
| E | 140 / `6ad4b261939e8289b1ee67157e74a37423888e5baf33204802f178e7c4033bc9` | same | yes |
| F | 238 / `942b5ec8232429b7301ba489d7aedf214ad32a77be751d59d80bc0707432a62b` | same | yes |

Python compact JSON round-trip preserved every literal and its insertion/key
order. Both mechanisms observed no BOM, trailing newline, or escaped slash.
The literals use exact snake-case spellings and contain no insignificant
whitespace.

## 15. Mutual-exclusion theorem and repetition law

For one exact valid claim-inline predicate in one exact snapshot, the terminal
relation is uniquely `DigestEqual` or `DigestUnequal`.

Therefore:

- the same terminal relation cannot yield both `SupportEligible` and
  `RefutationEligible`;
- if both exact edge kinds exist, only the edge aligned with the one relation
  is eligible;
- the other exact edge is `Ineligible`, not conflict debt;
- several evidence nodes may independently bind one claim;
- several distinct edge IDs may independently resolve to one lane;
- repeated eligible lanes do not amplify;
- the same evidence under distinct edge IDs does not amplify; and
- no counting, threshold, vote, majority, weight, confidence, origin-group, or
  independence rule exists here.

No contradiction debt or conflict blocker is ratified. Append-only
currentness, changed claims, invalidation, and supersession remain future.

## 16. Ignored fields and non-effects

The future lane grants no authority to:

```text
ClaimAssertedV2.actor_class

EvidenceRegistered.content_hash
EvidenceRegistered.actor_class
EvidenceRegistered.summary

JustificationEdgeRecorded.actor_class
JustificationEdgeRecorded.rationale
JustificationEdgeRecorded.metadata_json
```

Changing only one of those fields cannot change a lane or its canonical bytes
when all ratified inputs remain equal. Attestation metadata is still strictly
parsed under Ticket 0061; unknown byte- or polarity-smuggling fields fail
before edge lookup.

The contract creates no evidence/origin admission, support/refutation
aggregation, achieved standing, policy v4, direct-refutation rule, conflict
precedence, contradiction debt, invalidation, supersession, currentness,
implicit policy, generic predicate/edge composer, writer, `EpistemicGate`,
loader, CAS, filesystem/network acquisition, callback/plugin/model authority,
or Deadbolt change.

## 17. Ticket 0056 blocker closure without policy activation

Ticket 0056 remains a blocked historical candidate. No withdrawn identifier,
policy, rule, resolver, receipt, or conflict blocker becomes active.

The old counterexample was:

```text
claim expected digest H
+ arbitrary contradicts witness bytes B
+ digest(B) != H
→ manufactured negative result
```

The new lane closes substitution because the claim owns the immutable subject
bytes, relation derives only from those bytes, attestation carries no byte
source, evidence cannot select substitute bytes, and exact binding plus edge
alignment are required.

```text
RefutationEligible != governed Refuted
```

Only the missing negative-input substrate is fixed. Policy consequence is not
ratified.

## 18. Distinction from `SupportContributionAuditV0`

`SupportContributionAuditV0` concerns admitted provenance/origin graph
contributions and future corroboration aggregation. The new lane concerns one
claim-owned predicate, one attestation-bound deterministic evidence node, and
one replayed edge.

```text
ClaimInlinePredicateEdgeLaneV0 != SupportContributionAuditV0
ClaimInlinePredicateEdgeLaneV0 != origin admission
ClaimInlinePredicateEdgeLaneV0 != corroboration
ClaimInlinePredicateEdgeLaneV0 != independence grouping
```

No support-contribution contract or runtime is modified.

## 19. Hostile cases

The normative design note contains the complete hostile table. It freezes at
least:

- missing claim/evidence plus missing edge → exact nested binding failure;
- wrong evidence kind → `AttestationBindingFailed(WrongEvidenceKind)`;
- syntactically malformed attestation JSON →
  `AttestationBindingFailed(MalformedAttestationMetadata)`;
- badly bound attestation → its exact Ticket 0061 binding failure before edge
  lookup;
- missing, unsupported, wrong-source, wrong-target, or wrong-scope edge →
  exact ordered lane failure;
- all four exact matrix cells;
- both edge kinds for each terminal relation;
- several evidence nodes and distinct repeated edges without amplification;
- edge-rationale and evidence-summary invariance;
- fake predicate/binding outcomes and serialized audit reinjection rejected;
- mixed snapshots impossible;
- a nested byte-smuggling field is
  `AttestationBindingFailed(UnknownAttestationField)`;
- historical witness metadata is
  `AttestationBindingFailed(UnknownAttestationMetadataKey)`;
- absence and unknown edge kind never falsity;
- ceiling drift reachable only for an otherwise eligible corresponding cell.

All cases preserve:

```text
absence != falsity
malformed != falsity
ineligible != refuted
eligible != achieved standing
audit bytes != authority
```

## 20. Validation record

No result is reported as passed unless the command actually ran and exited
successfully. The post-record documentation and path checks are rerun after
this section is written.

| Gate | Final observed result |
| --- | --- |
| `git diff --check ba9f444e84fcfd3fc756fc45f816d910ced9b83c --` | pass; exit 0, no output |
| `git diff --cached --check` | pass; exit 0, no output |
| `git diff --check` | pass; exit 0, no output |
| `cargo fmt --all --check` | pass; exit 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | pass; exit 0, no warnings |
| `cargo test --workspace --locked` | pass; exit 0 |
| `cargo test --doc --locked` | pass; exit 0; 162 `magpie-claims` and 3 `magpie-log` doctests passed, with 0 failures |
| exact `cargo run --locked --example tour -p magpie-claims` argv captured as raw stdout | pass; exit 0; 1,917 bytes; SHA-256 `48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f` |
| `python tools/verify_chain.py crates/magpie-log/testdata/golden-v1.jsonl ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c` | pass; records 0-8 `OK` |
| `python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737` | pass; records 0-1 `OK` |
| `python tools/check_release_metadata.py` in the direct checkout | blocked solely by the intentional dirty `README.md`; exit 1 |
| exact unchanged `python tools/check_release_metadata.py` in a temporary VCS-free mirror | pass; all three package inventories consistent (`magpie-log` 18, `magpie-claims` 43, `magpie-episodic` 8 files) |
| `cargo package -p magpie-log --locked` in the direct checkout | blocked solely by the intentional dirty `README.md`; exit 1 |
| exact unchanged `cargo package -p magpie-log --locked` in the same VCS-free mirror | pass; 18 files packaged and package verification completed |
| VCS-free mirror cleanup | pass; exact resolved target `<TEMP_ROOT>\magpie-ticket0062-vcsfree-codex-0062` was verified under the temp root and removed; final existence check false |
| six vectors through Python | pass; exact literals, lengths, hashes, compact JSON, key order, snake case, no BOM/newline/escaped slash |
| six vectors through PowerShell/.NET | pass; exact independent literals, lengths, hashes, key order, and byte law |
| vector-mechanism comparison | pass; all six results agree exactly |
| exact changed-path/index/unmerged audit | pass; final case-sensitive union is exactly the eight-path allowlist; no staged, unmerged, or non-normal index entries |

## 21. Same-session self-review

Two sequential read-only same-session passes are required after the complete
draft. They are self-review, not independent review.

### Pass 1 — soundness / negative authority

```text
PASS
```

The sequential read-only pass found no recurrence of arbitrary-byte
substitution or evidence-selected subject bytes. `DigestUnequal` requires an
exact `contradicts` edge for refutation eligibility, and `contradicts`
requires `DigestUnequal`; the support polarity is symmetric. Missing,
malformed, and misbound inputs remain failure, relation/edge mismatch remains
`Ineligible`, ceilings remain candidates, policy v4/debt remain absent,
repetition does not amplify, one snapshot owns the call, and no
caller-created outcome, receipt, or serialization can be reinjected.

### Pass 2 — contract / API / canonicalization

```text
FINDING:
- exact location:
  docs/design/claim-inline-predicate-edge-lanes-v0.md,
  Opaque outcome and receipt;
  tickets/0062-claim-inline-predicate-edge-lanes-contract.md, section 11
- consequence:
  "attribution proves no node existed" inverted the required query-only
  non-proof law and could be read as authoritative proof of absence
- remediation:
  state that retained query-ID strings, by themselves, do not prove that any
  requested node existed

FINDING:
- exact location:
  tickets/0062-claim-inline-predicate-edge-lanes-contract.md, section 12
- consequence:
  "no whitespace" could be misread as rejecting exact string contents rather
  than prohibiting only non-canonical JSON formatting whitespace
- remediation:
  align with the normative design's "no insignificant whitespace" law

FINDING:
- exact location:
  docs/design/claim-inline-predicate-edge-lanes-v0.md, immediately after
  Compatibility and non-effects;
  README.md, ledger entry 30
- consequence:
  the normative non-goals were present but lacked their required explicit
  heading, while one living sentence still described all support/refutation
  lanes as future
- remediation:
  add an explicit non-goals section and distinguish the Ticket 0062 contract
  from its future runtime

FINDING:
- exact location:
  docs/design/claim-inline-predicate-edge-lanes-v0.md,
  Exact future public API
- consequence:
  an unrequested "no bound" phrase broadened the raw-ID law beyond the exact
  normalization and identity prohibitions
- remediation:
  remove that extra phrase

FINDING:
- exact location:
  docs/design/claim-inline-predicate-edge-lanes-v0.md,
  Hostile contract cases
- consequence:
  malformed JSON, nested byte smuggling, and historical witness metadata
  named only a generic inherited metadata or unknown-field failure
- remediation:
  freeze the exact nested Ticket 0061 reasons as
  MalformedAttestationMetadata, UnknownAttestationField, and
  UnknownAttestationMetadataKey

RERUN:
PASS
```

After remediation, the sequential read-only rerun found no generic framework
creep or `SupportContributionAuditV0` collision. It confirmed the one snapshot
method, opaque non-reinjectable construction boundary, exact ten-field
receipt with sole successful IDs, edge source/target/scope order, both nested
failure levels, all 28 Ticket 0061 and 33 Ticket 0059 source spellings,
identical design/ticket vector literals and key order, unambiguous
UTF-8/escaping law, unchanged historical tickets, exact path scope, and no
runtime or policy activation claim.

## 22. Known risks

- A future implementation must privately compose the claim resolver,
  attestation resolver, relation computation, and edge composer without
  accepting their public outcomes as authority or drifting inherited
  failures and vectors.
- The shared private seam increases coupling among Tickets 0059, 0061, and the
  future edge-lane module.
- Ticket 0061 inherits an unbounded raw `metadata_json`, nesting, and key-count
  input. Its strict allocation-free parser can consume quadratic CPU on valid
  adversarial JSON; this documentation contract adds no new cap.
- A later implementation could accidentally feed `SupportEligible` or
  `RefutationEligible` into policy. Explicit standing-inertia, v2-inertia,
  compile-fail, and hostile tests are required.
- No runtime exists, so private-composition feasibility remains an
  implementation risk even though the current crate-private subject resolver,
  exact snapshot tables, and strict binding procedure provide a sound path.

No required validation is skipped. The two direct dirty-tree
release/package invocations are recorded as blocked and their exact unchanged
VCS-free-mirror reruns pass. With the known future implementation risks above,
the uncommitted documentation patch is ready for owner review.

## 23. Publication boundary and next slice

No commit, push, pull request, GitHub metadata, review-thread response,
ready-state change, merge, auto-merge, label, issue, or publication action is
authorized or performed.

The next separately reviewed slice is:

```text
claim-inline predicate edge-lane runtime v0
```

It is not:

- policy v4;
- direct-refutation policy runtime;
- contradiction debt;
- aggregation;
- the pre-alpha demo;
- writer behavior; or
- ingestion.

## 24. Proposed draft PR body

```markdown
## Summary

- ratify one closed, standing-inert
  `sha256_claim_inline_bytes_equals_v0 × ExactMachineCheckable ×
  DeterministicVerification × (supports | contradicts)` edge-lane contract
- freeze one three-ID `StandingReplaySnapshot` API that internally composes
  claim-owned predicate resolution, exact Ticket 0061 binding, and one exact
  replayed edge
- freeze the four-cell relation/edge matrix, exact failure precedence, opaque
  ten-field receipt, canonical audit profile, and six literal vectors

## Authority boundary

The future resolver accepts only one snapshot plus requested claim, evidence,
and edge IDs. It internally re-fetches or derives every claim, evidence,
binding, relation, edge, scope, and ceiling value. It accepts no caller-created
outcome, receipt, parsed metadata, relation, edge object, Boolean, policy,
callback, serialized bytes, or second snapshot.

`DigestEqual + supports` is only `SupportEligible`;
`DigestUnequal + contradicts` is only `RefutationEligible`. The two
polarity-misaligned cells are `Ineligible`, not failures or opposite-polarity
authority. Eligible results carry candidate ceilings only:
`Settled != achieved Settled` and `Refuted != governed Refuted`.

## Soundness boundary

The Ticket 0056 arbitrary-byte substitution path stays closed: subject bytes
come only from the claim, attestation metadata carries no byte source, and
evidence cannot select substitute bytes. Exact binding and exact edge
source/target/scope alignment are separately required. Ticket 0056 remains
blocked; no policy v4, direct-refutation rule, conflict blocker, contradiction
debt, or achieved standing is ratified.

## Compatibility

- Ticket 0059's neutral evaluator and Ticket 0061's routing-only resolver stay
  unchanged
- policy v2 remains tied to historical `sha256_bytes_equals_v0`; the new lane
  neither replaces nor feeds it
- `ClaimInlinePredicateEdgeLaneV0` is distinct from
  `SupportContributionAuditV0`, origin admission, corroboration, and
  independence grouping
- repetition does not amplify, and both edge kinds yield one eligible plus one
  ineligible result for a fixed terminal relation, never conflict debt

## Non-effects

Documentation only. No Rust, tests, fixtures, Cargo, dependencies, L0, FORMAT,
policy matrix, standing, aggregation, writer, loader, CAS, filesystem,
network, plugin, model, Deadbolt, release, CI, package, or GitHub metadata
change.

## Validation

- PASS: `cargo fmt --all --check`
- PASS: `cargo clippy --workspace --all-targets --locked -- -D warnings`
- PASS: `cargo test --workspace --locked`
- PASS: `cargo test --doc --locked`
- PASS: tour stdout is 1,917 bytes with SHA-256
  `48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f`
- PASS: both frozen chains
- PASS: exact unchanged release-metadata and `magpie-log` package gates in a
  temporary VCS-free mirror; direct checkout attempts were blocked only by
  the intentional documentation-dirty tree
- PASS: all six literal vectors agree through Python and PowerShell/.NET
- PASS: final case-sensitive changed-path union is exactly the eight-path
  allowlist, with no staged, unmerged, renamed, or non-normal index entries
- self-review pass 1: PASS
- self-review pass 2: PASS after documented remediation and rerun

## Next slice

The next separately reviewed slice is claim-inline predicate edge-lane runtime
v0, not policy v4, direct-refutation policy runtime, contradiction debt,
aggregation, the pre-alpha demo, writer behavior, or ingestion.
```
