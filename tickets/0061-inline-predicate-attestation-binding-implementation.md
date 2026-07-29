# Ticket 0061: Inline Predicate Attestation Binding Implementation

## Status

Implemented and validated in this uncommitted candidate patch.

```text
Ticket 0060 contract: ratified
Ticket 0061 implementation: implemented in this candidate patch
resolver: implemented and standing-inert
next: support/refutation lane contract(s)
```

This ticket records implementation only. It does not amend the ratified law in
Tickets 0057 through 0060.

## 1. Starting point and execution mode

Required and observed starting commit:

```text
6dc56e34f12928e47d37868b444ed1562fc38e99
Merge pull request #75 from noctem-o/agent/inline-predicate-attestation-binding-contract
```

Execution used the primary checkout directly:

```text
checkout: C:\magpie
starting branch: main
implementation branch: agent/inline-predicate-attestation-binding-runtime
mode: direct checkout; no implementation worktree created
```

Before branch creation, the required implementation branch name was found in a
clean stale Traycer worktree at the same required base. The owner explicitly
authorized removal of that exact clean worktree and its no-unique-commit branch.
After that bounded cleanup, the complete preflight was rerun: the primary
checkout was clean, no Git operation was active, local `main` and fetched
`origin/main` both equalled the required commit with zero divergence, the
required merge subject matched, the backup branch was not checked out, and the
implementation branch no longer existed. The exact branch was then created
from the required base in `C:\magpie`.

No commit, push, publication, pull-request mutation, GitHub mutation, reset,
clean, restore, stash, rebase, amend, credential change, Git configuration
change, backup-branch change, or unrelated destructive Git action was
performed.

## 2. Implementation classification

This is a routing-only, same-snapshot, standing-inert implementation of the
Ticket 0060 contract:

```text
one verified StandingReplaySnapshot
+ requested claim_id
+ requested evidence_id
+ internally resolved valid Ticket 0059 claim
+ replayed DeterministicVerification evidence
+ strict inline_predicate_attestation metadata
+ exact predicate, claim, and scope bindings
→ Bound
```

Every unsuccessful route returns one opaque `ResolutionFailed` outcome with
the exact requested claim and evidence IDs plus one frozen reason.

`Bound` does not prove digest equality, truth, support, contradiction,
refutation, contribution eligibility, admission, settlement, standing, edge
existence, evidence content-hash correctness, actor authority, external
artifact identity, currentness, or verified-prefix provenance.

## 3. Changed-path boundary

The exact allowlist for this ticket is:

```text
README.md
crates/magpie-claims/src/claim_inline_sha256_predicate.rs
crates/magpie-claims/src/inline_predicate_attestation.rs
crates/magpie-claims/src/lib.rs
crates/magpie-claims/tests/inline_predicate_attestation.rs
docs/design/claim-inline-subject-binding-v0.md
docs/design/claim-inline-sha256-predicate-v0.md
docs/design/inline-predicate-attestation-binding-v0.md
docs/design/standing-aggregation-independence-groups.md
docs/design/standing-view-evidence-ceilings.md
tickets/0061-inline-predicate-attestation-binding-implementation.md
```

Historical Tickets 0057 through 0060 are unchanged. Cargo manifests, the
lockfile, FORMAT/L0, fixtures, golden files, policy code, standing code,
historical deterministic-verifier production code, release metadata, CI,
tools, generated files, I/O surfaces, plugins, Deadbolt behavior, and GitHub
metadata are unchanged.

## 4. Shared claim resolution

`claim_inline_sha256_predicate.rs` now contains one crate-private
`resolve_claim_inline_subject_v0` primitive shared by the Ticket 0059 evaluator
and the Ticket 0061 resolver.

The primitive preserves the existing sequence:

1. standing claim lookup;
2. typed claim lookup from the selected snapshot;
3. complete strict metadata parsing;
4. descriptor validation and all frozen field checks;
5. canonical statement construction;
6. exact standing/typed/derived statement binding;
7. required claim content hash and SHA-256 rebinding;
8. subject decoding;
9. decoded subject size guard.

Its private result retains the terminal evaluator's material, but exposes only
crate-private borrows for resolved claim ID, validated predicate ID, and exact
claim scope. Its fields remain private. The attestation module cannot inspect
subject bytes, subject hex, expected bytes or digest, canonical statement,
claim content hash, computed digest, or terminal equality.

Only `evaluate_sha256_claim_inline_bytes_equals_v0` hashes the resolved subject,
compares expected and computed digest bytes, classifies `DigestEqual` versus
`DigestUnequal`, and constructs the existing eight-field predicate receipt.
The new resolver neither calls that public evaluator nor duplicates its claim
parser.

### Ticket 0059 inertia

The public evaluator signature, types, inspection methods, 33 failures and
order, parser behavior, attribution, opacity, canonical profile, receipt, and
terminal behavior are unchanged. Existing focused tests pass, and the three
vectors remain:

| Outcome | Bytes | SHA-256 |
| --- | ---: | --- |
| `DigestEqual` | 639 | `d0c6a0f771600c34aad02264f722f915bb9093ac28c4489d1369585c402a6f5a` |
| `DigestUnequal` | 641 | `cced955ed7822888e3d2e0a3f91263ec905d2c629d5649003eaabd3360f81291` |
| `ResolutionFailed(MissingClaim)` | 95 | `b99b96af9b35c96e6d65bea4a0fdd364670abefd21e163009737a9ef4d91969d` |

The new integration suite also pins these values after the refactor.

## 5. Public API and opacity

The new snapshot-only method is:

```rust
impl StandingReplaySnapshot {
    pub fn resolve_inline_predicate_attestation_v0(
        &self,
        claim_id: &str,
        evidence_id: &str,
    ) -> InlinePredicateAttestationBindingOutcomeV0;
}
```

The exported types are exactly:

- `InlinePredicateAttestationBindingOutcomeKindV0`;
- `InlinePredicateAttestationBindingFailureV0`;
- `InlinePredicateAttestationBindingOutcomeV0`;
- `InlinePredicateAttestationBindingReceiptV0`.

Outcome inspection is limited to `kind`, `requested_claim_id`,
`requested_evidence_id`, `receipt`, `failure`, and `canonical_bytes`. Receipt
inspection is limited to `schema`, `predicate_id`, `claim_id`, `evidence_id`,
and `scope_ref`.

The outcome and receipt have private fields and private construction. They
implement only `Clone` and `Serialize`. They do not implement `Deserialize`,
`Default`, conversions, mutation, consuming accessors, `Debug`, `Display`,
`Error`, equality, ordering, or hashing. Compile-fail coverage blocks literals,
variants, failure payloads, reclassification, mutation, consuming extraction,
debug/display, deserialization/default/conversions, reinjection, alternate
snapshot inputs, `StandingView` dispatch, raw-material dispatch, and a public
parsed-attestation type.

For `Bound`, both requested-ID accessors borrow the receipt's sole ID strings.
For failure, the outcome privately owns the exact two requested strings and
one failure reason; this remains attribution rather than existence proof.

## 6. Strict borrowed parser and resource law

The new module narrowly reuses the Ticket 0059 complete-JSON scanner through
crate-private tokens and helpers kept in the existing allowlisted source file.
It does not introduce a generic JSON framework, dependency, `serde_json::Value`,
map-based encoder, recursion cap, metadata-size cap, key-count cap, or new
failure.

The parser:

- validates one complete JSON document and full JSON grammar;
- accepts valid strings, escapes, Unicode scalar values, surrogate pairs,
  containers, numbers, literals, delimiters, and trailing whitespace;
- rejects malformed strings, escapes, surrogates, values, delimiters, trailing
  values, and non-object documents as `MalformedAttestationMetadata`;
- compares decoded keys after unescaping;
- detects duplicate known and unknown keys at both levels;
- collects structural facts before selecting a semantic failure;
- rejects every unknown outer or nested key;
- performs no Unicode normalization;
- selects failures by frozen semantic order rather than source-key order.

The complete evidence metadata has exactly one outer
`inline_predicate_attestation` key and four mandatory nested string fields:
`schema`, `predicate_id`, `subject_claim_id`, and `scope_ref`.

`predicate_id` performs missing, wrong-type, and empty checks before a borrowed
count-only decoded UTF-8 byte pass. Decoded byte 65 returns
`PredicateIdTooLong`. No complete or partial identifier is copied before the
64-byte bound succeeds. Only an accepted bounded value is materialized;
alphabet validation follows the length check and binding follows alphabet
validation.

Schema, subject-claim, and scope mismatches are compared against borrowed
decoded content. Arbitrarily long mismatches are not materialized. Requested
IDs retain no new validation or bounds. The ratified absence of global
metadata, depth, and key-count caps means hostile valid JSON can consume CPU;
duplicate detection intentionally rescans members without allocating an
attacker-controlled key set.

## 7. Exact resolver procedure

One invocation performs:

1. preserve exact requested claim and evidence IDs for any failed result;
2. invoke the same-snapshot private claim resolver;
3. wrap any exact nested failure as `ClaimPredicateResolutionFailed`;
4. re-fetch typed evidence from the same snapshot by requested evidence ID;
5. require exact `DeterministicVerification`;
6. strictly parse the complete evidence metadata;
7. apply outer structural precedence;
8. apply inner structural precedence;
9. validate schema;
10. validate predicate ID;
11. validate subject claim ID;
12. validate scope;
13. compare the parsed predicate with the claim-validated predicate;
14. compare the parsed subject claim with the requested claim;
15. compare parsed scope with replayed evidence scope;
16. compare parsed scope with resolved typed-claim scope;
17. privately construct the exact five-field receipt and return `Bound`.

Evidence scope mismatch precedes claim scope mismatch. No justification edge is
enumerated, selected, fetched, parsed, or inspected.

The complete outer failure vocabulary, in order, is:

1. `ClaimPredicateResolutionFailed(exact nested reason)`;
2. `MissingEvidence`;
3. `WrongEvidenceKind`;
4. `MalformedAttestationMetadata`;
5. `DuplicateAttestationMetadataKey`;
6. `UnknownAttestationMetadataKey`;
7. `MissingInlinePredicateAttestation`;
8. `WrongTypeInlinePredicateAttestation`;
9. `DuplicateAttestationField`;
10. `UnknownAttestationField`;
11. `MissingSchema`;
12. `WrongTypeSchema`;
13. `UnknownSchema`;
14. `MissingPredicateId`;
15. `WrongTypePredicateId`;
16. `EmptyPredicateId`;
17. `PredicateIdTooLong`;
18. `InvalidPredicateId`;
19. `MissingSubjectClaimId`;
20. `WrongTypeSubjectClaimId`;
21. `EmptySubjectClaimId`;
22. `MissingScopeRef`;
23. `WrongTypeScopeRef`;
24. `EmptyScopeRef`;
25. `PredicateBindingMismatch`;
26. `ClaimBindingMismatch`;
27. `EvidenceScopeBindingMismatch`;
28. `ClaimScopeBindingMismatch`.

Source comparison confirms the inherited enum still has exactly 33 variants in
the Ticket 0059 order and the new outer enum has exactly these 28 stages.
Unit coverage serializes every nested reason without flattening, including the
defense-in-depth decoded-size reason.

## 8. Relation neutrality and non-effects

Claims that the Ticket 0059 evaluator classifies `DigestEqual` and
`DigestUnequal` both produce identical `Bound` audit bytes when routing inputs
are otherwise identical. The new module imports no SHA-256 implementation,
never hashes subject bytes, never observes expected or computed digest
material, and never serializes or branches on terminal equality.

The resolver ignores evidence `content_hash`, `actor_class`, and `summary` for
authority. Varying only those replay-valid fields leaves classification and
canonical bytes unchanged. Histories with no edge, a `supports` edge, a
`contradicts` edge, or both edge kinds also produce identical binding bytes.

The resolver creates no polarity, support/refutation lane, contribution,
admission, standing policy, achieved standing, contradiction debt,
invalidation, supersession, currentness, writer, loader, CAS, filesystem,
network, callback, plugin, or Deadbolt behavior.

StandingResolution v0, policies v1-v3, support/refutation matrices, historical
deterministic verification, the tour, both frozen chains, and package/release
metadata remain unchanged under the observed validation gates.

## 9. Cross-family refusal

Tests prove:

- historical `verification_witness` metadata presented to the new resolver is
  `UnknownAttestationMetadataKey`;
- new `inline_predicate_attestation` metadata presented to the historical
  resolver is `UnknownWitnessKey`, never `Matched`;
- metadata containing both outer keys is rejected by both families;
- the `DeterministicVerification` kind with `{}` metadata grants neither
  family authority;
- invoking the new route against historical claim material does not change the
  historical resolver's canonical bytes.

Historical deterministic-verifier production code is unchanged.

## 10. Canonical audit JSON

The encoder is bound to:

```text
magpie-inline-predicate-attestation-binding-outcome-json-v0
```

It uses `serde_json::to_vec` over private fixed-field typed wire views. It uses
no `Value`, maps, JCS/RFC 8785, generic canonical JSON, or Magpie L0.

Receipt order is `schema`, `predicate_id`, `claim_id`, `evidence_id`,
`scope_ref`. Ordinary failure detail order is `claim_id`, `evidence_id`,
`reason`. Nested failure detail order adds `claim_reason` after `reason`.

Independent .NET UTF-8/SHA-256 recomputation and runtime tests agree on:

| Outcome | Bytes | SHA-256 |
| --- | ---: | --- |
| `Bound` | 216 | `9ecd2e4c3c4f291fb8d4c5c6a4ceac590a5cc5a516c088766979596705d86bc5` |
| `MissingEvidence` | 125 | `968bf85313eef6e2700fb9e3780bfa1b9f88c31f1d6a58898901843b20a10fe6` |
| nested `MissingClaim` | 173 | `2a341009683cc4bc9db94a5d631898a0ea5b1efa7701e8e23ac0f93831b91190` |

Tests also pin compact UTF-8 JSON, fixed key order, standard control escaping,
lowercase `\u00xx`, direct non-control Unicode, no normalization, no escaped
slash, no BOM, and no trailing newline.

## 11. Tests and mutation check

Focused observed results:

| Command | Result |
| --- | --- |
| `cargo test -p magpie-claims --lib claim_inline_sha256_predicate --locked` | pass; 12 passed |
| `cargo test -p magpie-claims --lib inline_predicate_attestation --locked` | pass; 9 passed |
| `cargo test -p magpie-claims --test claim_inline_sha256_predicate --locked` | pass; 19 passed |
| `cargo test -p magpie-claims --test inline_predicate_attestation --locked` | pass; 15 passed |

The new coverage includes all outer reasons, nested preservation, double-fault
precedence, complete hostile JSON, post-unescape duplicate keys, field-order
permutations, wrong types/nulls, smuggled authority keys, identifier byte
boundaries and alphabet order, long borrowed mismatches, equality neutrality,
substitutions, all scope-order classes, cross-family refusal, repeated
resolution, exact attribution and escaping, non-authority fields, edge
invariance, canonical vectors, historical verifier inertia, standing/policy
inertia, and compile-fail authority barriers.

A temporary uncommitted mutation reversed evidence-scope and claim-scope
precedence. The targeted integration test failed for the intended reason:
actual `ClaimScopeBindingMismatch` versus required
`EvidenceScopeBindingMismatch`. The exact correct implementation was restored,
and the same one-test command then passed. No mutation artifact remains.

## 12. Validation record

Observed results:

| Gate | Result |
| --- | --- |
| direct-checkout preflight, fetch, base subject, ancestry, zero divergence | pass |
| `cargo fmt --all --check` | pass |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | pass |
| `cargo test --workspace --locked` | pass; all unit, integration, and documentation targets completed with zero failures |
| `cargo test --doc --locked` | pass; 162 `magpie-claims`, 0 `magpie-episodic`, and 3 `magpie-log` doctests |
| tour | pass; 1,917 stdout bytes, SHA-256 `48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f` |
| `golden-v1.jsonl` chain | pass; all 9 records `OK` |
| Deadbolt anchor chain | pass; both records `OK` |
| six independent canonical recomputations | pass |
| 33 nested / 28 outer source-order comparison | pass |
| receipt and ordinary/nested detail field-order comparison | pass |
| all three required `git diff --check` forms | pass |
| exact changed-path and index audit | pass; 8 unstaged tracked paths, 3 untracked paths, 0 staged paths, 0 extra or missing allowlist paths, 0 non-normal index flags, and 0 unmerged entries |
| direct `python tools/check_release_metadata.py` | blocked solely by intentional dirty `README.md` |
| direct `cargo package -p magpie-log --locked` | blocked solely by intentional dirty `README.md` |
| unchanged release checker in exact VCS-free mirror | pass; inventories 18/43/8 |
| unchanged `cargo package -p magpie-log --locked` in mirror | pass; 18 files packaged and crate verified |
| VCS-free mirror cleanup | pass; mirror removed |

No validation gate was skipped. A first temporary-mirror wrapper emitted a
non-terminating PowerShell parameter error before the same two underlying gates
completed successfully; the mirror was removed. The mirror run was repeated
cleanly with terminating error handling, both gates passed again, and that
mirror was also removed.

An initial path-audit helper printed the correct union but also emitted
PowerShell method/parenthesis errors and was not counted as a pass. A clean
terminating-error rerun performed ordinal case-sensitive comparisons and
produced the passing result above.

## 13. Same-session self-review

### Pass 1: parser / failure / resource

```text
PASS
```

The pass checked source-order independence, decoded duplicate detection,
complete JSON grammar and surrogate handling, identifier allocation order,
long borrowed mismatches, outer/inner and nested failure ordering,
attacker-controlled panic paths, cross-family dispatch, and accidental limits.
No finding required remediation.

### Pass 2: authority / API / serialization

```text
PASS
```

The pass checked public construction/reclassification, traits and methods,
debug/display exposure, duplicate successful IDs, reinjection, alternate
snapshot/caller-object inputs, edge access, digest leakage, evidence
non-authority fields, policy/standing effects, receipt fields, wire/reason
order, and Ticket 0059 vector drift. No finding required remediation.

These were sequential read-only self-review passes in this same session. They
were not independent reviews and used no subagent or external checker.

## 14. Known risks

- The strict scanner honors the ratified absence of global metadata, nesting,
  and key-count caps. Valid adversarial JSON can therefore consume substantial
  CPU. Duplicate-key detection is allocation-free but quadratic in members.
- The complete-JSON scanner is deliberately hand-written and non-recursive;
  hostile grammar, escape, surrogate, delimiter, number, literal, and deep
  container cases reduce but cannot eliminate parser implementation risk.
- The shared scanner seam is crate-private and increases internal coupling
  between the claim and attestation modules. The seam is narrow and covered by
  Ticket 0059 inertia tests.
- `DecodedSubjectTooLarge` remains a defense-in-depth nested reason whose public
  reachability is constrained by the earlier subject-hex bound; its guard and
  exact outer serialization remain directly tested.

No check is skipped and no unresolved contract uncertainty is known. Subject
to owner review, the uncommitted candidate patch is ready for review.

## 15. Publication boundary

No commit, push, pull request, GitHub metadata, review thread, issue, comment,
ready-state, merge, or other publication action is authorized or performed.

Proposed commit and draft PR title:

```text
feat(claims): implement inline predicate attestation binding v0
```

Proposed draft PR body:

```markdown
## Summary

- factor Ticket 0059 claim-inline resolution through one private
  same-snapshot primitive without changing its public evaluator or vectors
- add the snapshot-only routing resolver for exact
  `inline_predicate_attestation` metadata
- add opaque `Bound` / `ResolutionFailed` audit values, exact canonical
  vectors, hostile parser tests, and compile-fail authority barriers

## Authority boundary

The resolver accepts only one `StandingReplaySnapshot`, a requested claim ID,
and a requested evidence ID. It internally re-fetches both claim tables and
typed evidence. It does not accept edges, parsed metadata, claims, evidence
objects, outcomes, receipts, callbacks, other snapshots, or audit bytes.

`Bound` proves only exact routing and predicate/claim/scope binding. The
resolver neither computes nor observes `DigestEqual` / `DigestUnequal`, assigns
polarity, creates a contribution, invokes policy, nor changes standing.
Evidence content hash, actor class, summary, and all justification edges are
non-authority.

## Compatibility

- Ticket 0059's 33 failures, public API, behavior, and three canonical vectors
  remain byte-identical
- historical `verification_witness` behavior and canonical bytes are unchanged
- old and new metadata families reject one another in both directions
- StandingResolution v0, policies v1-v3, support/refutation matrices, tour,
  frozen chains, release metadata, and package inventory remain unchanged

## Validation

- formatting and clippy pass
- focused claim and attestation unit/integration suites pass
- full workspace tests and doctests pass
- tour remains 1,917 bytes with SHA-256
  `48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f`
- both frozen chains verify
- all six Ticket 0059/0060 vectors independently recompute
- release metadata and `magpie-log` packaging pass in a VCS-free mirror
- scope-precedence mutation is caught and the restored test passes

## Non-effects and next slice

No L0, FORMAT, Cargo, dependency, fixture, policy, standing, writer, loader,
CAS, filesystem, network, plugin, Deadbolt, release, CI, or GitHub surface
changes. The next separately reviewed slice is support/refutation lane
contract(s), not lane implementation, policy v4, direct-refutation runtime, the
pre-alpha demo, or ingestion.
```

## 16. Next slice

The next separately reviewed slice is:

```text
support/refutation lane contract(s)
```

It is not lane implementation, policy v4, direct-refutation runtime, the
pre-alpha demonstration, writer behavior, or ingestion.
