# Ticket 0060: Inline predicate attestation binding contract

## 1. Exact base and starting gate

```text
base:
3a9550034539579856f2f7d0101ca9404bc2049c

base subject:
Merge pull request #74 from noctem-o/agent/claim-inline-sha256-predicate-runtime

PR #74:
claims: implement claim-inline SHA-256 predicate v0
```

The base is the merge commit of PR #74 into `main`. Before this ticket was
started, current `origin/main` was fetched, resolved exactly to the SHA
above, and `git merge-base --is-ancestor` confirmed the merge commit is in
the branch history. The isolated worktree was clean and the new branch was
created directly from that fetched base.

## 2. Branch and proposed publication title

```text
branch:
agent/inline-predicate-attestation-binding-contract

proposed commit title / draft PR title:
docs: ratify inline predicate attestation binding v0
```

`AGENTS.md` is binding. This work does not commit, push, open or edit a PR,
mark readiness, merge, resolve comments, or otherwise mutate GitHub. The
owner retains sole publication and merge authority.

## 3. Documentation-only classification

```text
correctness:
ratifies one routing-only same-replay evidence-to-claim attestation
binding contract for the Ticket 0057/0058/0059 inline predicate family

authority:
documentation contract only; future authority may originate only from
one snapshot-only resolver that internally re-fetches the claim and
evidence

runtime:
unchanged; implementation remains the next separately reviewed slice

predicate:
unchanged; no digest relation is produced or interpreted

polarity:
none

standing:
unchanged; Bound is standing-inert

policy:
none ratified

edges:
not read and not composed

tests and fixtures:
unchanged

canonical L0 bytes:
unchanged

derived audit bytes:
one future profile and three documentation vectors ratified

dependencies:
unchanged
```

No runtime code, test, fixture, Cargo, format, policy, standing, edge,
writer, loader, CAS, filesystem, network, clock, or Deadbolt behavior is
implemented or changed.

## 4. Exact changed-path allowlist

Exactly these seven documentation paths may change:

```text
README.md
docs/design/claim-inline-subject-binding-v0.md
docs/design/claim-inline-sha256-predicate-v0.md
docs/design/inline-predicate-attestation-binding-v0.md
docs/design/standing-view-evidence-ceilings.md
docs/design/standing-aggregation-independence-groups.md
tickets/0060-inline-predicate-attestation-binding-contract.md
```

The two Ticket 0060 documents are new. The other five paths are living
status/sequence mirrors only. Their inherited laws and historical records
may not be rewritten.

Every other path is prohibited. In particular: Rust, tests, fixtures,
Cargo manifests, the lockfile, `docs/FORMAT.md`, L0 payloads, policy code,
standing code, the historical verifier, CI, release metadata, generated
files, tools, historical tickets, and GitHub metadata.

## 5. Mission and normative contract

The normative design is:

[`docs/design/inline-predicate-attestation-binding-v0.md`](../docs/design/inline-predicate-attestation-binding-v0.md)

It answers only:

```text
Does this exact replayed evidence node route and bind to this exact
replayed claim, its validated predicate identity, and its exact scope?
```

It does not answer predicate equality or inequality, support,
contradiction, contribution eligibility, admission, standing, edge
authority, contradiction debt, invalidation, supersession, or currentness.

The compact law is:

```text
one verified StandingReplaySnapshot
+ requested claim_id
+ requested evidence_id
+ internally re-resolved valid Ticket 0059 claim
+ replayed DeterministicVerification evidence
+ exact new attestation envelope and schema
+ exact predicate, claim, and scope bindings
→ Bound
```

`Bound` is a routing result only.

## 6. Decision ledger

Every mission decision is adjudicated here. The companion design note
contains the full normative detail, failure order, vectors, hostile cases,
and reviewer checks.

| Item | Decision | Adjudication |
| --- | --- | --- |
| A. Outer envelope | exact outer key `inline_predicate_attestation`; complete evidence metadata contains only that key | `verification_witness` is byte-bearing historical input; broader names reserve unreviewed families; the chosen sibling name is exact and cross-family fail-closed |
| B. Inner schema | exact `magpie-inline-predicate-attestation-v0` | a new schema is required because `magpie-verification-witness-v0` carries a byte source and cannot be reused |
| C. Required fields | exactly four mandatory string fields: `schema`, `predicate_id`, `subject_claim_id`, `scope_ref` | this is Ticket 0057's inherited field boundary; no optional field or alternate source is permitted |
| C. Structural parsing | one complete object; duplicate and unknown keys rejected at both levels after JSON unescaping; source order never selects failure | matches Magpie's strict derived-metadata practice while closing escaped duplicate and smuggling attacks |
| C. Identifier law | `predicate_id` inherits decoded UTF-8 maximum 64 and `[a-z0-9_]+`; claim ID and scope are non-empty opaque exact strings with no new maximum | preserves the compiled resource law without inventing a generic ID ontology or a new L0 limit |
| D. Evidence kind | reuse exact `DeterministicVerification` | the closed label describes an automated deterministic route but grants no authority by itself; exact envelopes prevent old/new cross-dispatch; no L0 or matrix expansion |
| E. Same replay | one future `StandingReplaySnapshot::resolve_inline_predicate_attestation_v0(&self, claim_id, evidence_id)` method | the resolver internally re-fetches standing claim, typed claim, and typed evidence; no edge ID or caller-created authority input |
| F. Evaluator relation | choose option 2: factor one private claim-resolution primitive shared by Ticket 0059's evaluator and the attestation resolver | avoids duplicate parser authority and avoids computing/observing the terminal digest relation in a routing-only resolver; Ticket 0059 API, behavior, failures, and vectors stay frozen |
| G. Edge/polarity | no edge is read; `Bound` has no positive or negative polarity | supports, contradicts, both, and no-edge histories are binding-equivalent; only a later lane contract may compose binding, relation, and a replayed edge |
| H. Evidence content hash | ignored | no canonical attestation statement or evidence-artifact hash law exists; binding it would invent a second contract |
| H. Actor class | no resolver-specific check; ignored after ordinary replay/L0 validity | actor labels are asserted classification, not authority |
| H. Summary | ignored | prose is not a routing or authority field |
| I. Outcome | opaque `Bound` / `ResolutionFailed`; resolver-created private representation with read-only inspection and one-way serialization | no public construction, deserialization, defaulting, conversion, mutation, reclassification, debug exposure, consuming deconstruction, or reinjection; an audit-only clone has no accepting authority sink |
| I. Receipt | exactly `schema`, `predicate_id`, `claim_id`, `evidence_id`, `scope_ref` | five fields are the complete routing audit basis; envelope and evidence kind are compiled success invariants, and no edge/relation/policy/standing field is needed |
| J. Failure relation | preserve the exact typed `ClaimInlineSubjectResolutionFailureV0` as the nested reason of `ClaimPredicateResolutionFailed` | retains all 33 Ticket 0058 reasons and precedence without duplicating or flattening claim-resolution authority |
| J. Outer failure order | nested claim resolution, evidence existence/kind, full metadata structure, outer structure, inner structure, schema/field validation, then predicate/claim/scope bindings | distinguishes missing evidence from malformed evidence, resolution failure from inequality, and scope mismatch from falsity |
| K. Canonical profile | `magpie-inline-predicate-attestation-binding-outcome-json-v0` | direct typed compact UTF-8 adjacent `outcome` / `details` encoding; audit material only, never accepted back |
| K. Pinned vectors | `Bound`: 216 bytes / `9ecd2e4c…6bc5`; `MissingEvidence`: 125 bytes / `968bf853…0fe6`; nested `MissingClaim`: 173 bytes / `2a341009…1190` | lengths and SHA-256 values were independently recomputed before inclusion; complete bytes are in the design note |

## 7. Exact ratified identities and field sets

```text
outer key:
inline_predicate_attestation

schema:
magpie-inline-predicate-attestation-v0

predicate:
sha256_claim_inline_bytes_equals_v0

evidence kind:
DeterministicVerification

required fields:
schema
predicate_id
subject_claim_id
scope_ref

outcome kinds:
Bound
ResolutionFailed

receipt:
schema
predicate_id
claim_id
evidence_id
scope_ref

canonical audit profile:
magpie-inline-predicate-attestation-binding-outcome-json-v0
```

The complete evidence metadata object carries no sibling key. The
attestation contains no bytes, digests, relation, outcome, receipt,
contribution, edge, policy, standing, contradiction, invalidation, or
caller-selected authority field.

## 8. Authority and cross-family law

The sole future public query is:

```rust
StandingReplaySnapshot::resolve_inline_predicate_attestation_v0(
    &self,
    claim_id: &str,
    evidence_id: &str,
) -> InlinePredicateAttestationBindingOutcomeV0
```

One private claim-resolution primitive is shared by the existing neutral
evaluator and the future resolver. It re-fetches both claim tables and
applies the exact Ticket 0059 33-reason law. The primitive necessarily
resolves the claim-owned subject and expected operand, but those values are
neither attestation inputs nor binding comparisons; only the existing
evaluator consumes them for the terminal digest comparison. The attestation
resolver consumes only resolved claim identity, validated predicate identity,
and claim scope, then re-fetches typed evidence from that same snapshot. No
public raw-material resolver or parsed-attestation type exists.

The two terminal predicate relations are equally eligible for binding.
The attestation resolver does not compute or expose either. Any
Ticket 0059 claim-resolution failure prevents binding and is preserved as
the exact typed nested reason.

The historical and new evidence metadata families refuse one another:

```text
verification_witness
  only with magpie-verification-witness-v0
  only in the historical edge-bearing checker

inline_predicate_attestation
  only with magpie-inline-predicate-attestation-v0
  only in the routing-only no-edge resolver
```

`DeterministicVerification` alone dispatches neither parser and grants no
standing.

## 9. Failure vocabulary and canonical bytes

The companion design note freezes:

- the complete nested 33-reason Ticket 0058 vocabulary and order;
- the complete outer 28-stage vocabulary and order;
- exact post-unescape duplicate behavior;
- exact unknown-field smuggling refusal;
- the two ordered scope failures;
- ordinary and nested failure wire payloads;
- all snake-case spellings;
- exact key and receipt field order;
- exact UTF-8/escaping law;
- one successful and two failed literal vectors with lengths and hashes.

No serialized outcome is authority. There is no `Deserialize` path and no
resolver accepts audit bytes or an outcome-shaped value.

## 10. Required non-effects

This contract changes none of:

- `magpie-core-v1`;
- `docs/FORMAT.md`;
- payload tags or fields;
- `EvidenceKind`, edge-kind, or actor-class vocabulary;
- golden vectors;
- existing deterministic-verifier traces or bytes;
- Ticket 0059 evaluator behavior, failures, receipts, or vectors;
- support or refutation ceilings;
- support-context requirements;
- standing v0/v1/v2/v3;
- aggregation or origin policy;
- contradiction debt;
- invalidation;
- supersession/currentness;
- `EpistemicGate`;
- writer, loader, CAS, filesystem, network, clock, callback, plugin, or
  Deadbolt behavior.

No policy v4 or replacement direct-refutation identity is ratified.

## 11. Explicitly unratified surfaces

No runtime implementation, parser module, public type, support/refutation
lane, contribution, admission, edge composition, policy identity, standing
rule, conflict/precedence rule, contradiction debt, invalidation,
supersession/currentness, evidence content-hash statement law, actor
authority rule, generic registry, L0 change, format tag, golden event,
dependency, I/O, or GitHub action is included.

The next separately reviewed slice is only the routing-only
attestation-binding implementation. It must stop if preserving this
contract requires defining polarity, contribution, or standing.

## 12. Independent hostile review record

Two fresh independent read-only checkers are required on the complete
candidate:

- Checker A — authority/substitution: another claim, evidence, snapshot,
  old witness, parsed data, predicate outcome, serialized receipt, edge,
  and byte-bearing-field substitution.
- Checker B — policy/polarity leakage: support/refutation derivation,
  `DigestUnequal` or failure-to-falsity collapse, label/actor/content-hash
  authority, edge-derived standing, and premature policy-cell activation.

The checkers may not edit, generate artifacts, commit, or mutate Git/GitHub.
Their exact verdicts and any adjudication are recorded here only after the
complete candidate exists.

```text
Checker A verdict:
VERDICT: PASS
No findings. The same-snapshot two-ID API, internal re-fetches, exact
predicate/claim/scope bindings, private construction, cross-family refusal,
unknown-field smuggling failures, no-edge surface, and one-way audit bytes
closed every attempted authority or substitution path.

Checker B verdict:
VERDICT: PASS
No findings. Bound, DigestUnequal, ResolutionFailed, the reused evidence-kind
label, actor class, content hash, summary, and every edge remained unable to
create polarity, contribution, policy activation, or achieved standing.

Coordinator adjudication:
PASS / PASS; no remediation finding remained. The unbounded replay-owned
metadata and opaque claim/scope/query strings remain an implementation-sensitive
resource seam, but the contract makes them non-authorizing, requires borrowed
exact comparisons, and preserves the inherited 64-decoded-byte predicate-ID
bound. A new-envelope supports edge can still appear as a candidate-only
ceiling or failed historical-v2 attempt; it cannot satisfy the historical
witness resolver and cannot yield achieved standing.
```

## 13. Validation contract

Run and report observed results only:

```powershell
git diff --check 3a9550034539579856f2f7d0101ca9404bc2049c HEAD --
git diff --cached --check
git diff --check
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --doc --locked
cargo run --locked --example tour -p magpie-claims
python tools/verify_chain.py crates/magpie-log/testdata/golden-v1.jsonl ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c
python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737
python tools/check_release_metadata.py
cargo package -p magpie-log --locked
git status --short
git diff --name-only
git diff --stat
```

The tour is preserved only if exact stdout is 1,917 bytes with SHA-256:

```text
48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f
```

The release checker and package command may reject the intentionally dirty
worktree. If so, each unchanged gate must pass in a temporary VCS-free mirror
without `--allow-dirty`, and every mirror must be removed. No fixture or golden
regeneration is permitted.

The changed-path audit must union:

- exact base-to-working-tree changes;
- staged paths;
- unstaged paths;
- untracked paths;
- non-normal `git ls-files -v` index flags.

It must compare exact case-sensitive Git path strings with the seven-path
allowlist and disable rename detection so a source path cannot hide.

The validation record must also include:

- an independent recomputation of every literal canonical vector's UTF-8
  length and SHA-256;
- a repository-wide coherence search that distinguishes frozen historical
  progression text from living status mirrors;
- exact terminology searches for attestation, witness, evidence kind,
  predicate outcome, polarity, support, refutation, and standing; and
- a prohibited-surface audit proving that no Cargo, FORMAT, fixture, Rust,
  policy, standing, CI, release-metadata, generated, or GitHub path changed.

Observed results are recorded after all review remediation:

| Gate | Observed result |
| --- | --- |
| exact base, branch, and ancestry | pass; `HEAD` and current fetched `origin/main` are exact base `3a9550034539579856f2f7d0101ca9404bc2049c`; branch is `agent/inline-predicate-attestation-binding-contract`; merge-base ancestry check passed |
| `git diff --check` variants | pass for pinned-base-to-`HEAD`, cached, and working-tree forms |
| `cargo fmt --all --check` | pass |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | pass on the completed rerun; the first short wrapper invocation timed out before producing a clippy verdict |
| `cargo test --workspace --locked` | pass; all workspace unit, integration, and documentation targets completed with zero failures |
| `cargo test --doc --locked` | pass; 137 `magpie-claims` plus 3 `magpie-log` documentation tests, zero failures |
| exact binary tour gate | pass; stdout is 1,917 bytes with SHA-256 `48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f` |
| independent `golden-v1.jsonl` verification | pass; all nine records `OK` under the pinned verifying key |
| independent Deadbolt anchor-chain verification | pass; both records `OK` under the pinned verifying key |
| three Ticket 0060 canonical vectors | pass; literal UTF-8 recomputation returned 216 / `9ecd2e4c3c4f291fb8d4c5c6a4ceac590a5cc5a516c088766979596705d86bc5`, 125 / `968bf85313eef6e2700fb9e3780bfa1b9f88c31f1d6a58898901843b20a10fe6`, and 173 / `2a341009683cc4bc9db94a5d631898a0ea5b1efa7701e8e23ac0f93831b91190` |
| failure-vocabulary/order comparison | pass; the nested design list is exactly the 33 source variants in order, and the outer enum/order contains exactly 28 matching stages |
| `python tools/check_release_metadata.py` | direct invocation reached Cargo and refused only the intentional uncommitted `README.md`; the unchanged checker passed without `--allow-dirty` in an exact VCS-free mirror with inventories 18/41/8; cleanup verified |
| `cargo package -p magpie-log --locked` | direct invocation refused only the intentional uncommitted `README.md`; the unchanged command passed without `--allow-dirty` in an exact VCS-free mirror, packaged 18 files, and verified the crate; cleanup verified |
| exact changed-path/index audit | pass; exact union is the seven allowlisted paths: five unstaged tracked mirrors and two untracked new documents; zero staged paths, unauthorized paths, prohibited paths, or non-normal index flags |
| coherence and terminology search | pass; all five living mirrors say the contract is ratified and implementation remains next; remaining older future-contract wording is frozen Ticket 0057/0058/0059 history or an explicitly preserved historical decision; attestation, witness, evidence-kind, predicate-outcome, polarity, support, refutation, and standing hits introduce no contrary authority claim |
| prohibited-surface audit | pass; no Rust, Cargo, lockfile, FORMAT, L0, fixture, policy-code, standing-code, CI, release-metadata, generated candidate, tool, or GitHub path changed |

No validation command was skipped. Both temporary VCS-free mirrors were
removed, and the final candidate contains no generated validation artifact.

## 14. Publication boundary

No commit, push, draft PR creation/edit, readiness change, comment
resolution, merge, auto-merge, label change, or other GitHub mutation is
authorized.

Proposed commit title and draft PR title:

```text
docs: ratify inline predicate attestation binding v0
```

Proposed draft PR body:

```markdown
## Summary

- ratify the routing-only `inline_predicate_attestation` evidence metadata
  envelope and `magpie-inline-predicate-attestation-v0` schema
- bind one replayed `DeterministicVerification` evidence node to one
  same-snapshot claim, its validated inline predicate identity, and exact
  scope
- freeze an opaque `Bound` / `ResolutionFailed` audit surface, exact
  failure precedence, cross-family refusal, and canonical vectors

## Authority boundary

This contract reads no edge and assigns no predicate relation, polarity,
contribution, admission, or standing. Evidence `content_hash`, actor class,
and summary are not binding authority. The future resolver must internally
re-fetch claim and evidence from one `StandingReplaySnapshot` and share one
private claim-resolution primitive with the Ticket 0059 evaluator; no
caller-created outcome or receipt is accepted.

## Non-effects

No Rust, tests, fixtures, Cargo, L0, FORMAT, golden, policy behavior,
standing behavior, verifier-trace, release, CI, or GitHub metadata change.

## Validation

See Ticket 0060 for the observed review and validation record.
```

## 15. Stop conditions

Stop rather than proceed if:

- PR #74 is not in the exact base history;
- the worktree or path audit contains an unauthorized change;
- `DeterministicVerification` reuse would require a new policy cell or let
  either metadata family cross-dispatch;
- a binding decision requires bytes, a digest relation, an edge, polarity,
  contribution, admission, or standing;
- content-hash binding would require inventing an unratified evidence
  statement;
- the future resolver would accept caller-created parsed data, outcomes,
  receipts, callbacks, alternate snapshots, or authority objects;
- one private shared claim-resolution primitive cannot preserve Ticket
  0059's API, behavior, failures, and vectors exactly;
- any L0, format, fixture, Cargo, runtime, policy, standing, CI, release,
  or GitHub change appears necessary.

## 16. Suggested reviewer checklist

1. The base is exactly the PR #74 merge commit and the branch descends from
   it.
2. Exactly the seven documentation paths in section 4 changed.
3. The outer key, schema, four mandatory fields, and
   `DeterministicVerification` decision are exact.
4. Historical witness and new attestation families refuse each other in
   both directions.
5. The resolver is one-snapshot, re-fetches both claim tables and evidence,
   and takes no edge ID or authority object.
6. One private shared claim-resolution primitive is chosen; no duplicate
   parser authority and no caller-supplied evaluator output.
7. `DigestEqual` and `DigestUnequal` are equally binding-eligible;
   `ResolutionFailed` remains a non-polar failure.
8. Content hash, actor class, summary, edges, polarity, contribution, and
   standing have explicit non-authority decisions.
9. The opaque outcome and exact five-field receipt have no construction,
   reclassification, deserialization, mutation, debug, consuming
   deconstruction, or reinjection path; audit-only cloning is not authority.
10. The complete failure vocabulary/order and canonical profile/vectors
    agree with the design note.
11. The five mirrors change living status/sequence only and preserve
    historical wording.
12. Both independent hostile checkers returned explicit final verdicts and
    every finding was adjudicated before validation.
13. Every claimed validation was actually observed.
