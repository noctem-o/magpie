# Ticket 0063: Claim-Inline Predicate Edge Lanes v0 Implementation

## Status

Implemented as one owner-reviewable, uncommitted candidate patch on:

```text
C:\magpie
```

This ticket implements the exact standing-inert runtime ratified by Ticket
0062 and landed through merged PR #77. It does not activate policy v4 or
change achieved standing.

## Checkout and synchronization record

```text
required starting SHA:
be7c0faea94c2be2daa57e81ede7cfaae2b24937

observed starting branch:
agent/claim-inline-predicate-edge-lanes-contract

observed starting SHA:
0a082d8dde0a7f59b423b813c80a161054785191

required and observed merge subject after synchronization:
Merge pull request #77 from noctem-o/agent/claim-inline-predicate-edge-lanes-contract

checkout path:
C:\magpie

synchronized main SHA:
be7c0faea94c2be2daa57e81ede7cfaae2b24937

created implementation branch:
agent/claim-inline-predicate-edge-lanes-runtime
```

The initial index and working tree were clean. No merge, rebase,
cherry-pick, or revert operation was active. `origin` existed. `git fetch
origin --prune`, `git switch main`, and `git pull --ff-only origin main`
succeeded. Local `main` and `origin/main` were both exactly the required SHA
with divergence `0 0`. The proposed branch was absent locally (exit 1) and
remotely (exit 2) before creation.

This work used the direct checkout. No new worktree was created. Existing
worktrees were not modified.

## Implementation classification

```text
correctness:
implements one closed same-snapshot claim-inline predicate edge-lane family

authority:
one new private-construction standing-inert derived outcome

public input:
one StandingReplaySnapshot plus exact claim_id, evidence_id, and edge_id strings

predicate:
unchanged sha256_claim_inline_bytes_equals_v0 semantics

binding:
unchanged Ticket 0061 semantics and failure order

edge:
one exact replayed supports or contradicts edge

polarity:
lane eligibility classification only

candidate ceiling:
Settled for support eligibility; Refuted for refutation eligibility

standing:
unchanged

policy v2:
unchanged and not supplied by this runtime

policy v4:
absent

governed Refuted:
absent

aggregation:
absent

contradiction debt:
absent

L0 and FORMAT:
unchanged

fixtures and frozen chains:
unchanged

dependencies:
unchanged

writer, loader, CAS, filesystem, network, plugin, model, Deadbolt:
unchanged
```

## Exact changed-path allowlist

Only the following case-sensitive paths are authorized:

```text
README.md
crates/magpie-claims/src/claim_inline_sha256_predicate.rs
crates/magpie-claims/src/claim_inline_sha256_predicate/edge_lane.rs
crates/magpie-claims/src/inline_predicate_attestation.rs
crates/magpie-claims/src/lib.rs
crates/magpie-claims/tests/claim_inline_predicate_edge_lane.rs
docs/design/claim-inline-subject-binding-v0.md
docs/design/claim-inline-sha256-predicate-v0.md
docs/design/inline-predicate-attestation-binding-v0.md
docs/design/claim-inline-predicate-edge-lanes-v0.md
docs/design/standing-view-evidence-ceilings.md
docs/design/standing-aggregation-independence-groups.md
tickets/0063-claim-inline-predicate-edge-lanes-implementation.md
```

The new paths are:

```text
crates/magpie-claims/src/claim_inline_sha256_predicate/edge_lane.rs
crates/magpie-claims/tests/claim_inline_predicate_edge_lane.rs
tickets/0063-claim-inline-predicate-edge-lanes-implementation.md
```

No path outside the allowlist is part of the candidate.

## Private composition refactor

The runtime uses the privacy-preserving child-module shape:

```text
claim_inline_sha256_predicate.rs
└── claim_inline_sha256_predicate/edge_lane.rs
```

The private composition has three layers:

1. `resolve_claim_inline_subject_v0` remains the single Ticket 0059 claim
   resolver.
2. `resolve_inline_predicate_attestation_after_claim_v0` is the shared
   crate-private Ticket 0061 post-claim binding helper.
3. The child edge-lane module performs exact edge validation, one digest
   relation evaluation, matrix classification, and the corresponding
   eligible ceiling check.

The Ticket 0059 public evaluator and the new lane runtime share one
parent-private `evaluate_resolved_claim_inline_sha256_v0` function. It
computes the SHA-256 digest once for its resolved input and returns
parent-private relation/computed-byte material. Ticket 0059 continues to
construct its unchanged eight-field receipt and unchanged public outcome.

One lane call invokes `resolve_claim_inline_subject_v0(self, claim_id)`
exactly once. The resulting value is borrowed first by the Ticket 0061
post-claim helper and later by the parent-private digest evaluator. It is not
parsed or resolved a second time.

The attestation module can name and borrow
`ResolvedClaimInlineSubjectV0`, but Rust module privacy keeps its subject
bytes, expected digest, computed digest, canonical statement, claim content
hash, and relation inaccessible. The only crate-visible methods remain:

```text
claim_id
predicate_id
scope_ref
```

The shared post-claim helper returns only:

```text
predicate_id
claim_id
evidence_id
scope_ref
```

It cannot compute or observe `DigestEqual` or `DigestUnequal`.

Neither public Ticket 0059 audit outcome nor public Ticket 0061 audit
outcome/receipt is consumed as authority. The lane runtime consumes only
private resolved values derived from the same snapshot.

## Public API

The sole resolver is:

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

The exported Ticket 0062 types are exactly:

```text
ClaimInlinePredicateEdgeLaneOutcomeKindV0
ClaimInlinePredicateEdgeLaneKindV0
ClaimInlinePredicateEdgeLaneRelationV0
ClaimInlinePredicateEdgeLaneEdgeKindV0
ClaimInlinePredicateEdgeLaneIneligibleReasonV0
ClaimInlinePredicateEdgeLaneFailureV0
ClaimInlinePredicateEdgeLaneOutcomeV0
ClaimInlinePredicateEdgeLaneReceiptV0
```

No free function, `StandingView` method,
`OriginAdmissionReplayContextV0` method, generic predicate/edge composer,
caller-selected lane/policy/ceiling, raw-material resolver, alternate
snapshot input, or authority reinjection method was added.

## Derivation and failure order

One call performs:

1. preserve the exact requested claim, evidence, and edge IDs for failed or
   ineligible attribution;
2. resolve the requested claim once through Ticket 0059;
3. wrap claim failure as
   `AttestationBindingFailed(ClaimPredicateResolutionFailed(...))`;
4. run the shared Ticket 0061 post-claim helper;
5. wrap any binding failure as `AttestationBindingFailed(...)`;
6. fetch only the exact requested edge through
   `self.standing().justification_edge(edge_id)`;
7. require an existing edge;
8. require exact `supports` or `contradicts`;
9. require exact source/evidence binding;
10. require exact target/claim binding;
11. require exact edge/bound-attestation scope binding;
12. compute the claim-owned digest relation once;
13. classify the frozen matrix;
14. consult only the corresponding eligible ceiling;
15. privately construct the outcome.

The frozen top-level first-failure order is:

```text
1. AttestationBindingFailed(exact nested Ticket 0061 failure)
2. MissingEdge
3. UnsupportedEdgeKind
4. EdgeSourceBindingMismatch
5. EdgeTargetBindingMismatch
6. EdgeScopeBindingMismatch
7. SupportCeilingInvariantMismatch
8. RefutationCeilingInvariantMismatch
```

Matrix mismatches are `Ineligible`, not failures. Ineligible cells do not
consult either ceiling.

## Matrix and candidate ceilings

| Claim-owned relation | Exact edge | Outcome |
| --- | --- | --- |
| `DigestEqual` | `supports` | `SupportEligible` |
| `DigestUnequal` | `supports` | `Ineligible(DigestUnequalDoesNotSupport)` |
| `DigestEqual` | `contradicts` | `Ineligible(DigestEqualDoesNotRefute)` |
| `DigestUnequal` | `contradicts` | `RefutationEligible` |

The private pure `classify_matrix_with_ceilings_v0` seam accepts closures
only inside the module. Production supplies the compiled policy functions.
Unit tests stage unexpected cells without a public override and count
invocations to prove that:

- support eligibility requires the support cell to be exactly
  `Some(Status::Settled)`;
- refutation eligibility requires the refutation cell to be exactly
  `Some(Status::Refuted)`;
- support eligibility does not consult the refutation cell;
- refutation eligibility does not consult the support cell;
- neither ineligible cell consults either cell.

`policy.rs` is unchanged. Candidate `Settled` is not achieved `Settled`;
candidate `Refuted` is not achieved `Refuted`.

## Exact edge law

The runtime performs one exact lookup:

```rust
self.standing().justification_edge(edge_id)
```

It then compares bytes exactly:

```text
edge.source_id == requested evidence_id
edge.target_id == requested claim_id
edge.scope_ref == bound attestation scope_ref
```

No trimming, case folding, Unicode normalization, namespace interpretation,
aliasing, enumeration, or compatible-edge selection occurs. Edge actor
class, rationale, and metadata are ignored for lane authority and do not
enter the receipt or canonical bytes. Replay retains its existing
first-write-wins duplicate-edge behavior.

## Outcome and receipt opacity

The authority-bearing outcome and receipt have private fields and private
construction. Each implements only `Clone` and one-way `Serialize`.
They implement no `Deserialize`, `Default`, `From`, `TryFrom`, `Debug`,
`Display`, `Error`, equality, ordering, or hashing traits. They expose no
public mutation or consuming extraction.

For eligible outcomes the receipt owns the sole successful claim, evidence,
and edge ID strings; requested-ID accessors borrow those strings. Ineligible
and failed outcomes privately retain the three exact query IDs plus exactly
one reason or failure.

Compile-fail doctests pin construction, deserialization, conversion,
mutation, extraction, formatting, error, equality, hashing, reinjection,
serialized-byte, raw-node, second-snapshot, free-function, alternate-context,
raw-composer, and caller-selected policy/ceiling barriers.

## Canonical audit profile

The canonical profile is:

```text
magpie-claim-inline-predicate-edge-lane-outcome-json-v0
```

Serialization uses direct typed fixed-field serde wire views. It uses no
`serde_json::Value`, map wire object, generic canonical JSON, JCS, Magpie L0,
or deserializable authority type.

The receipt order is:

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

Nested binding failures preserve `reason`, then `binding_reason`, then
`claim_reason` only for `claim_predicate_resolution_failed`.

## Ratified runtime vectors

| Outcome | Bytes | SHA-256 |
| --- | ---: | --- |
| `SupportEligible` | 362 | `3d1e241b92e8b66c7e42613231a68e4df990c910d48627415ad29db8dc67c085` |
| `RefutationEligible` | 372 | `2849f779e12e756fce866294053d89cae9413254c5edab78221e82ac2d3b3c06` |
| `Ineligible(DigestUnequalDoesNotSupport)` | 152 | `793302a14a991b45ff6a5c6c15c04d936664c29c3685f25d0e509f26c8e983b8` |
| `Ineligible(DigestEqualDoesNotRefute)` | 148 | `2f320d25d7b5d3f3c5c45185cf33abb025d2a6f7a93f5721d157c3318eac469c` |
| `ResolutionFailed(MissingEdge)` | 140 | `6ad4b261939e8289b1ee67157e74a37423888e5baf33204802f178e7c4033bc9` |
| nested `MissingClaim` | 238 | `942b5ec8232429b7301ba489d7aedf214ad32a77be751d59d80bc0707432a62b` |

The runtime integration test compares each emitted value with the exact
Ticket 0062 literal, byte length, and lowercase SHA-256.

## Compatibility vector inertia

Ticket 0059:

| Outcome | Bytes | SHA-256 |
| --- | ---: | --- |
| `DigestEqual` | 639 | `d0c6a0f771600c34aad02264f722f915bb9093ac28c4489d1369585c402a6f5a` |
| `DigestUnequal` | 641 | `cced955ed7822888e3d2e0a3f91263ec905d2c629d5649003eaabd3360f81291` |
| `MissingClaim` | 95 | `b99b96af9b35c96e6d65bea4a0fdd364670abefd21e163009737a9ef4d91969d` |

Ticket 0061:

| Outcome | Bytes | SHA-256 |
| --- | ---: | --- |
| `Bound` | 216 | `9ecd2e4c3c4f291fb8d4c5c6a4ceac590a5cc5a516c088766979596705d86bc5` |
| `MissingEvidence` | 125 | `968bf85313eef6e2700fb9e3780bfa1b9f88c31f1d6a58898901843b20a10fe6` |
| nested `MissingClaim` | 173 | `2a341009683cc4bc9db94a5d631898a0ea5b1efa7701e8e23ac0f93831b91190` |

Focused Ticket 0059 and Ticket 0061 integration suites pass with these
values unchanged. Historical `verification_witness` behavior and old/new
metadata-family mutual refusal are also exercised.

## Test coverage

Focused unit and integration coverage includes:

- all four matrix cells;
- every top-level failure, including both ceiling-drift failures through the
  private pure seam;
- adjacent first-failure boundaries and representative double/triple faults;
- all 28 Ticket 0061 binding reasons and all 33 Ticket 0059 claim reasons,
  with enum retention and exact ordered canonical nesting;
- representative runtime claim, evidence, parser, predicate, claim, evidence
  scope, and claim scope failures;
- exact ID behavior, escaped JSON bindings, controls, direct Unicode,
  decomposed combining marks, no slash escaping, no BOM, and no newline;
- exact edge selection, all four replay-valid unsupported edge kinds, and
  first-write-wins duplicate-edge behavior;
- one-at-a-time invariance for claim actor class, evidence content hash,
  evidence actor class, evidence summary, edge actor class, edge rationale,
  and edge metadata;
- Ticket 0056 subject-substitution closure and evidence smuggling refusal;
- equal/unequal mutual exclusion, multiple evidence nodes, and distinct edge
  IDs without aggregation or amplification;
- standing v0/v1/v2/v3, policy-cell, snapshot, and
  `SupportContributionAuditV0` inertia;
- Ticket 0059, Ticket 0061, and historical verifier compatibility;
- compile-fail opacity and authority barriers.

## Temporary mutation check

After the correct focused suite passed, the
`DigestEqual + contradicts` matrix arm was temporarily changed from:

```text
Ineligible(DigestEqualDoesNotRefute)
```

to:

```text
RefutationEligible
```

The smallest affected command was:

```text
cargo test -p magpie-claims --test claim_inline_predicate_edge_lane claim_inline_predicate_edge_lane_all_four_matrix_cells_are_exact --locked
```

It failed as intended with:

```text
left: RefutationEligible
right: Ineligible
```

The ratified ineligible arm was restored immediately with a direct patch.
The same command then passed: one test passed, zero failed, fourteen
filtered out. No mutation code or generated mutation artifact remains.

## Validation record

All required validation was run. No validation command was skipped.

Focused implementation and compatibility commands:

- `cargo check -p magpie-claims --all-targets --locked`: pass;
- `cargo fmt --all --check`: pass;
- `cargo test -p magpie-claims claim_inline_predicate_edge_lane --locked`:
  pass, including all 15 edge-lane integration tests;
- `cargo test -p magpie-claims claim_inline_sha256_predicate --locked`:
  pass;
- `cargo test -p magpie-claims inline_predicate_attestation --locked`: pass;
- `cargo test -p magpie-claims --lib edge_lane --locked`: pass, 6 private
  unit tests;
- `cargo test -p magpie-claims --test claim_inline_sha256_predicate --locked`:
  pass, 19 tests;
- `cargo test -p magpie-claims --test inline_predicate_attestation --locked`:
  pass, 15 tests;
- the explicit edge-lane integration target: pass, 15 tests.

Full validation commands:

- `cargo clippy --workspace --all-targets --locked -- -D warnings`: pass;
- `cargo test --workspace --locked`: pass;
- `cargo test --doc --locked`: pass. The `magpie-claims` compile-fail
  surface ran 197 tests with zero failures; the existing `magpie-log`
  doctests ran 3 tests with zero failures.

Preservation commands:

- `cargo run --locked --example tour -p magpie-claims`: pass; raw stdout was
  exactly 1,917 bytes with SHA-256
  `48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f`;
- `python tools/verify_chain.py
  crates/magpie-log/testdata/golden-v1.jsonl
  ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c`:
  pass, all 9 records `OK`;
- `python tools/verify_chain.py
  fixtures/deadbolt-anchor-v1/anchor-log.jsonl
  d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737`:
  pass, both records `OK`;
- direct-checkout `python tools/check_release_metadata.py`: blocked by the
  intentional dirty `README.md`, exit 1;
- direct-checkout `cargo package -p magpie-log --locked`: blocked by the
  same intentional dirty file, exit 101;
- the exact unchanged release-metadata and package commands in the
  VCS-free `<TEMP_ROOT>\magpie-t0063-mirror-<GUID>` mirror: pass.
  Release inventories were `magpie-log` 18 files, `magpie-claims` 45 files,
  and `magpie-episodic` 8 files; `magpie-log` packaged 18 files and verified
  successfully. The mirror was removed.

Canonical and compatibility bytes were obtained from runtime outcomes.
The integration suite compared all six Ticket 0062 literals, lengths,
lowercase hashes, key order, spellings, Unicode behavior, control escaping,
slash behavior, BOM absence, whitespace absence, and trailing-newline
absence. A separate environment-gated runtime emitter produced those six
byte streams plus all six Ticket 0059/0061 compatibility streams. Python
`hashlib.sha256` independently decoded and hashed all 12 emitted streams:
every observed length and hash matched the tables above.

Diff and path commands:

- `git diff --check
  be7c0faea94c2be2daa57e81ede7cfaae2b24937 --`: pass;
- `git diff --cached --check`: pass;
- `git diff --check`: pass;
- the case-sensitive changed-path union audit: pass, exactly 13 allowed
  paths, with 0 staged paths, 0 non-normal index flags, 0 unmerged entries,
  and 0 unexpected paths.

Two attempted PowerShell cleanup command shapes were blocked before
execution by the shell safety gate because they contained recursive
`Remove-Item`. The first block occurred before mirror creation; the second
occurred after the exact literal mirror path had been returned. After
verifying that exact absolute path and leaf beneath the temporary root, the
same mirror was removed through
`System.IO.Directory.Delete(exact_path, true)`, and absence was confirmed.

An earlier runtime integration run compiled and ran 14 tests, with 12
passing and two failing because the new test fixtures used replay-invalid
actor-class and edge-kind vocabulary. The fixtures were corrected to use
replay-valid ADR-0002 values; the complete 14-test rerun passed. No runtime
law changed in that remediation.

## Same-session self-review

The three checks were sequential same-session read-only passes, not
independent review.

```text
PASS 1 — authority and soundness:
PASS

The claim is resolved exactly once in one lane call; subject and digest
material remain parent-private; the attestation module sees only claim,
predicate, and scope identifiers; no public audit value is consumed; all
derivation uses one snapshot and one exact edge; no standing, policy v4,
aggregation, amplification, conflict, or debt behavior exists.

PASS 2 — failure and canonical contract:
PASS

The source order matches the frozen first-failure order; matrix mismatch is
Ineligible; ineligible cells cannot invoke a ceiling closure; 28 binding and
33 claim reasons retain exact nested typed serialization; receipt and detail
orders, spellings, Unicode, escaping, candidate ceilings, and all six vectors
match the contract.

PASS 3 — compatibility and scope:
PASS after remediation

The first pass found stale preliminary-validation and pending-review labels
in this implementation record. This section replaced them with observed
results before the pass was rerun. On rerun, Ticket 0059/0061 vectors,
historical verifier behavior, policies v1/v2/v3, standing v0/v1/v2/v3,
support audit, tour, chains, dependencies, L0/FORMAT, and the exact path
allowlist were unchanged.
```

## Known risks

- The private composition has three layers: claim resolution, post-claim
  binding, and child-module lane derivation. Its narrowness should be reviewed
  alongside its additional control-flow complexity.
- The crate-private surface widens by one resolved-binding type, four
  read-only getters, and one post-claim helper. It carries only routing
  material.
- Ticket 0061's strict complete borrowed JSON parse cost remains inherited
  for every lane call.
- Future callers could mistake lane eligibility or a candidate ceiling for
  achieved standing unless they preserve the standing-inert boundary.
- No required validation remains blocked or skipped after the prescribed
  VCS-free mirror fallback. There is no unresolved implementation
  uncertainty. The uncommitted patch is ready for owner review.

## Publication boundary

No commit, push, pull request, review request, GitHub metadata change, merge,
review-thread action, or publication is authorized or performed by this
ticket. The final deliverable is an uncommitted patch for owner review.

## Next slice

The next separately reviewed slice is:

```text
a fresh deterministic direct-refutation policy contract
```

It is not automatically authorized by this implementation. Policy v4 does
not exist in this patch.
