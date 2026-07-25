# Ticket 0056: Standing policy v4 deterministic direct-refutation contract

## 1. Exact base

```text
5ca19a761d9ad46cd09b251796b4f4262302a1df
```

That commit is the merge of PR #69
(`docs: reconcile post-v3 architecture ledger`) into `main`.

## 2. Branch, commits and PR title

Suggested branch:

```text
agent/standing-policy-v4-direct-refutation-contract
```

Suggested commit title and draft PR title:

```text
docs: ratify standing policy v4 deterministic direct refutation rule
```

`AGENTS.md` is binding. K3 does not commit, push, open or edit a PR, mark
readiness, merge, resolve review threads, or enable auto-merge. The user
retains sole publication and merge authority.

## 3. Documentation-only classification

```text
correctness:
ratifies one closed direct-refutation policy rule; no runtime

authority:
documentation only; no authority path changes

standing:
unchanged (no v4 resolver exists)

policy:
one new ratified policy identity, magpie-claims-standing-v4,
contract only

runtime:
unchanged

tests:
unchanged

fixtures:
unchanged

canonical bytes:
unchanged

L0:
unchanged

dependencies:
unchanged

positive effect:
the first exact case in which replay-derived evidence may produce
governed Refuted is frozen before any implementation exists
```

This ticket implements no runtime capability. It changes no Rust, tests,
fixtures, formats, vectors, public APIs, policy matrices, standing
behavior, writer behavior, Deadbolt behavior, origin admission, or
aggregation.

## 4. Exact changed-path allowlist

Exactly these five paths may change:

```text
README.md
docs/design/standing-view-evidence-ceilings.md
docs/design/standing-aggregation-independence-groups.md
docs/design/standing-policy-v4-deterministic-direct-refutation-v0.md  (new)
tickets/0056-standing-policy-v4-deterministic-direct-refutation-contract.md  (new)
```

No other path is permitted. Specifically prohibited: Rust source, tests,
fixtures, `Cargo.toml`, `Cargo.lock`, CI, release metadata, changelog,
`docs/FORMAT.md`, `AGENTS.md`, golden vectors, the Python verifier,
`docs/design/replayable-deterministic-verifier-admission-v0.md` (a ratified
contract whose phase-scoped non-goals remain accurate; the v4 design note
carries the distinction instead), the v3 normative design contract,
historical tickets 0001-0055, generated files, and GitHub metadata.

## 5. Architectural layer distinctions

```text
refutation ceiling
!= automatic refutation

contradicts edge
!= proof of falsity

verifier failure
!= refutation

malformed evidence
!= refutation

missing evidence
!= refutation

unavailable witness
!= refutation

missing anchor
!= proof of non-occurrence

non-matching unrelated material
!= refutation

caller-provided receipt
!= authority

serialized audit output
!= replay authority

direct refutation
!= contradiction debt

direct refutation
!= invalidation

direct refutation
!= supersession/currentness

direct refutation
!= negative aggregation

legacy raw Refuted
!= governed replay-derived Refuted
```

And the standing invariants: one signed append-only log is the source of
truth; standing remains derived and regenerable; authority remains
private-construction and same-replay; no implicit latest-policy selection;
no caller-provided policy identity, audit, or receipt; no ambient lookup;
no probabilistic confidence; no voting or majority rule; no inference from
absence.

## 6. Central deterministic law

```text
one exact typed ExactMachineCheckable claim
+ one exact typed DeterministicVerification evidence node
+ one exact contradicts justification edge
+ exact claim/evidence/edge/scope/predicate/statement/content-hash
  bindings
+ refutation_ceiling cell == Some(Refuted)
+ one exact same-snapshot negative evaluation proving
  sha256_bytes_equals_v0 false for the exact bound witness
+ no conflicting Matched supports verification for the same claim
→ one governed direct-refutation contribution
→ governed Refuted
```

The rule never independently produces `Settled` or `Supported`. The full
normative text is
[`docs/design/standing-policy-v4-deterministic-direct-refutation-v0.md`](../docs/design/standing-policy-v4-deterministic-direct-refutation-v0.md);
this ticket and that note agree exactly.

## 7. Exact policy identities

```text
policy id:
magpie-claims-standing-v4

inheritance:
the complete explicit policy-v3 result, internally derived from the
same context and closure

rule identity:
StandingPolicyRuleV4::Sha256BytesEqualsDirectRefutationV0
(serialized: sha256_bytes_equals_direct_refutation_v0)

runtime policy parameter:
none
```

V4 is selected only through explicitly versioned resolvers on
`OriginAdmissionReplayContextV0`. It does not replace, alias, negotiate
with, or become an ambient default for v0-v3. There is no "latest" policy.
V4 never forks from v2: inheritance derives the complete v3 resolution
first, preserving corroboration in the ladder.

## 8. Future resolver surface

```rust
OriginAdmissionReplayContextV0::resolved_standing_v4(
    &self, claim_id: &str, closure: &ResolutionContentClosureV0,
) -> Option<Status>

OriginAdmissionReplayContextV0::resolved_standing_with_trace_v4(
    &self, claim_id: &str, closure: &ResolutionContentClosureV0,
) -> StandingResolutionV4
```

No free function, no `StandingReplaySnapshot` or `StandingView` v4 method,
no public sibling negative-context resolver. The negative evaluation is
crate-private inside the future v4 module.

## 9. Exact authority path

```text
one successful LogReader verification
→ one private composite replay
→ one OriginAdmissionReplayContextV0 (exact verified-prefix identity)
+ one immutable ResolutionContentClosureV0 (exact content input)
→ internally derived complete policy-v3 resolution
→ internally derived negative evaluation (crate-private)
→ one crate-private pure v4 composer
→ StandingResolutionV4
```

No caller supplies an audit, receipt, contribution, predicate outcome,
threshold, policy, lane, precedence table, snapshot, callback, serialized
output, or alternate authority table. The negative receipt is constructed
only inside the resolver, from the same verified replay, in the same call.
The immutable closure is exact content input, supplied exactly as the v3
resolvers receive it and consumed only by the inherited v3 derivation; the
digest comparison consumes no closure content. It is untrusted input, not
authority, and the contract states that cost honestly.

## 10. Exact lane and candidate universe

```text
evidence kind:  DeterministicVerification
claim domain:   ExactMachineCheckable
edge kind:      contradicts
predicate:      sha256_bytes_equals_v0
ceiling law:    refutation_ceiling == Some(Refuted)
```

Candidate eligibility, revalidated by re-fetch from the same snapshot:

1. requested claim exists in the legacy and typed claim tables;
2. the edge exists, is exactly `contradicts`, targets the claim, and names
   an existing typed evidence source;
3. evidence kind parses into the closed `EvidenceKind` vocabulary (an
   unrecognised kind fails `UnknownEvidenceKind`);
4. claim domain metadata parses strictly into the closed `ClaimDomain`
   vocabulary (an unrecognised domain fails `UnknownClaimDomain`);
5. exact three-way scope equality;
6. the (evidence kind, claim domain) pair is exactly the sole v4 lane —
   every other known pair, including the two other `Some(Refuted)` ceiling
   cells, is rejected before any ceiling check or negative evaluation;
7. the compiled refutation ceiling cell equals `Some(Refuted)`.

Structural first-failure order mirrors v0 support-candidate order with the
edge-kind check taking `contradicts`:

```text
MissingTargetClaim, MissingTypedTargetClaim, UnsupportedEdgeKind,
MalformedMetadata, MissingClaimDomain, WrongTypeClaimDomain,
DuplicateMetadataKey, UnknownClaimDomain, MissingSourceEvidence,
ScopeMismatch, UnknownEvidenceKind, UnsupportedDirectRefutationLane,
NoRefutationCeiling
```

`UnsupportedDirectRefutationLane` fires when kind and domain both parse but
the pair is not exactly `DeterministicVerification ×
ExactMachineCheckable`; `NoRefutationCeiling` then guards only the exact
lane's own compiled cell against drift.

The two other `Some(Refuted)` ceiling cells (`DeterministicVerification ×
Occurrence/Inclusion`, `DeadboltAnchor × Occurrence/Inclusion`) are
deliberately not covered: no sound achieved-negative substrate exists for
them today, and absence in an open append-only prefix is not universal
exclusion. The closed lane failure rejects them before any negative
evaluation even though their compiled ceilings are `Some(Refuted)`.

## 11. Exact negative-verifier semantics

The existing `DeterministicVerifierContextTraceV0::DigestMismatch` unit
trace is permanently disqualified from negative authority: it is a
receipt-less failure token of the positive `supports` lane, and pinned v2
law keeps a `Matched` supports path `Supported` beside another path's
`DigestMismatch`. The negative path never reuses, reinterprets, wraps, or
re-exports that value, and the positive trace type stays byte-identical.

For each eligible candidate the negative evaluation performs, in frozen
order, the same strict checks as the positive checker (strict predicate
parsing; statement byte-identity; claim content-hash rebinding; strict
witness parsing; schema, predicate, claim, and scope bindings; encoded and
decoded witness bounds), then its own terminal digest comparison:

```text
computed == expected → PredicateSatisfied
                       (the alleged contradiction fails; audit-only)

computed != expected → PredicateFalse(receipt)
                       (the only authoritative negative outcome)
```

`PredicateFalse` proves exactly that the one inline witness, exactly bound
to the exact claim, predicate, and scope, does not satisfy
`sha256_bytes_equals_v0`. It proves nothing about external artifacts,
occurrence, non-occurrence, interpretation, or general truth. Every
prerequisite failure variant is audit-only and is never falsity. The
closed negative trace vocabulary contains no `DigestMismatch` and no
`Matched` variant.

## 12. Exact precedence and conflict law

Preconditions: identity-valid, failure-free inherited v3. Then:

```text
1. inherited governed Refuted → Refuted (preserved; non-amplifying)
2. inherited governed Settled → Settled (preserved; v4 demotes nothing)
3. exact conflict: Matched supports verification (inherited v2 trace)
   + PredicateFalse contradicts verification for the same claim
   → closed blocker ConflictingDeterministicVerification
   → no v4 application; inherited standing preserved
4. one or more uncontested valid direct refutations → governed Refuted
5. otherwise → inherited standing unchanged
```

Reachability notes: an `ExactMachineCheckable` claim can inherit
`Supported` only from a `Matched` supports verification — exactly the
conflict case of rule 3, so no silent override exists. Such a claim cannot
inherit `Settled` today (v1 settles only `Occurrence/Inclusion`; domains
are single-valued), so rule 2 is completeness law for future domain
changes, which must re-review this precedence. Rule 4's reachable effect
today is promotion from `Open`, `Conjectured`, or none to `Refuted`. Rule
3 is the exact boundary against contradiction debt: verified conflicts are
marked, never collapsed; debt resolution remains future.

## 13. Multiplicity and non-amplification

One valid direct refutation is sufficient. This is a direct rule, not
aggregation; no two-group threshold, count, vote, score, or quorum applies.
One or one hundred valid paths yield the same single `Refuted`; every
application is retained for audit. Duplicate occurrences, cloned receipts,
repeated edges from one evidence node, and repeated evaluation never
amplify. Failed candidates never aggregate into refutation. Each of
multiple `contradicts` edges from one evidence node is an independent
exact candidate; invalid siblings neither manufacture nor destroy
authority.

## 14. Legacy raw quarantine

Legacy raw standing stays quarantined: raw `Refuted` establishes nothing,
vetoes nothing, and participates in nothing. An inherited governed
`Refuted` remains authoritative; repeated valid refutations beneath it are
redundant audit, never recovery or revival.

## 15. Outer-composition failure law

Mirrors the v3 outer law, identically ordered: inherited claim identity;
inherited policy identity (`magpie-claims-standing-v3`); verified-prefix
identity; closure identity; inherited failure-freedom (a blocker, never an
outer failure). Identity-invalid inheritance clears top-level governed and
legacy standing and sets currentness to `Unknown`; identity-valid
inheritance with a later outer failure preserves valid inherited values
without promotion. Outer failures return one exact first failure in frozen
order, retain the inherited resolution verbatim, and expose no evaluation
material. No outer failure manufactures a refutation.

## 16. Failure and blocker behavior

Structural candidate failure → no negative-context attempt. Prerequisite
failure → application with the exact failure context and no achieved
standing. `PredicateSatisfied` → application with that terminal outcome and
no achieved standing. Uncontested `PredicateFalse` → application achieving
`Refuted`. Exact conflict → the closed blocker, no application, inherited
preserved. Inherited v3 composition failure → closed blocker, no
application, no promotion. Outer failure → the outer law. No malformed,
missing, unavailable, or unrelated material ever creates a synthetic
refutation.

## 17. Deterministic ordering

```text
candidates: lexicographic edge-ID order (the inherited v0 trace order)
applications: candidate order
structural and negative-context failures: frozen first-failure order
outer failures: frozen first-failure order
blockers: declaration order
conflict detection: claim-level set law, order-independent
```

The same verified log prefix, immutable closure, compiled policies, and
requested claim produce byte-identical v4 output.

## 18. Future public types and serialization

Mirrors the v3 boundary: `StandingResolutionV4` and the negative receipt
type have private fields and read-only getters; `Serialize` only — no
`Deserialize`, no `Default`, no public constructor, no public mutation.
Only `StandingResolutionV4` exposes direct typed JSON `canonical_bytes()`
as policy-defined derived audit bytes (not L0 canonical encoding, not a new
event format, not a persistent protocol). The receipt fields mirror the
positive receipt: predicate schema and ID, claim/evidence/edge IDs, exact
scope, canonical statement, claim content hash, expected SHA-256, computed
SHA-256 (unequal by construction), and `u64` witness length. Cloning or
serializing any value yields audit material only.

Compile-fail documentation must prove: authority-bearing public structs
(`StandingResolutionV4`, the negative receipt, any application structs)
admit no struct-literal construction, public constructor, `Deserialize`,
`Default`, or public mutation; every new type, including the closed enum
vocabularies, rejects `Deserialize` and `Default`; constructible inert enum
tokens are never accepted as authority by any resolver or composer; no
resolver accepts a caller-provided receipt, audit, context, snapshot,
policy ID, predicate outcome, threshold, lane, or precedence (the immutable
closure remains exact content input, not an authority value, exactly as in
v3); no standalone `StandingView` or `StandingReplaySnapshot` v4 method; no
production test-only staging.

## 19. Hostile cases

The design note's hostile-case table is normative and repeated here only
in summary: a `supports` edge is never a negative candidate; a bare
`contradicts` edge without `PredicateFalse` never refutes; no prerequisite
failure, wrong identity, wrong scope, wrong kind/domain/policy, malformed
or missing or oversized witness, or unavailable material ever refutes;
caller-created, cloned, deserialized, staged, or foreign-prefix receipts
never produce standing; repetition never amplifies; conflicting exact
verifications produce the conflict blocker, never a silent winner; legacy
raw `Refuted` stays quarantined; missing anchors never prove
non-occurrence; unrelated similar prose never binds; currentness and
supersession are never inferred.

## 20. Frozen surfaces

No change to: v0/v1/v2/v3 types, bytes, behavior, or fixtures;
`DeterministicVerifierContextTraceV0`; the support and refutation policy
matrices; L0 payloads or canonical encoding; `docs/FORMAT.md`; golden
vectors; the independent Python verifier; the tour (1917 bytes, SHA-256
`48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f`);
Cargo manifests or the lockfile; CI; release metadata; historical tickets;
ratified contracts.

## 21. Explicit non-goals

No Rust, tests, fixtures, Cargo, dependency, L0, canonical encoding,
golden vector, Python verifier, runtime, contradiction debt, invalidation,
supersession/currentness, `EpistemicGate`, ordinary writer, loader, CAS,
filesystem or network ingestion, model integration, librarian, Deadbolt
change, `Occurrence/Inclusion` negative rule, new predicate family,
refutation aggregation or threshold, confidence score, vote, majority,
source reputation, fuzzy matching, inference from absence, generic policy
registry, implicit latest-policy selector, caller-selected negative
authority, or production trust claim.

## 22. Future sequence

```text
Ticket 0056: this contract
→ direct-refutation runtime (implements this contract)
→ contradiction debt and precedence
→ invalidation
→ supersession/currentness
→ EpistemicGate
→ ordinary claim-bearing writers
→ governed acquisition/loading
→ librarian integration
```

This ticket ratifies only the contract.

## 23. Stop conditions

Stop rather than proceeding if the contract appears to require: Rust,
tests, fixtures, Cargo, or dependency changes; a new payload variant or
canonical byte; a second rule, lane, threshold, or predicate family; an
`Occurrence/Inclusion` negative rule; reinterpreting the existing
`DigestMismatch` trace; a public sibling negative resolver; caller-provided
authority of any kind; silently resolving conflicting exact verifications;
demoting inherited `Settled`; contradiction-debt, invalidation,
supersession, `EpistemicGate`, writer, loader, storage, network, model, or
librarian design; editing any path outside the five-path allowlist; or
claiming any runtime capability exists.

## 24. Validation commands

Run and report actual results only:

```powershell
git diff --check
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --doc --locked
cargo run --locked --example tour -p magpie-claims
python tools/check_release_metadata.py
cargo package -p magpie-log --locked
git status --short
git diff --name-only
git diff --stat
```

Because this change is documentation-only, cargo outputs must remain
identical to the exact base. The release-metadata checker may refuse the
intentionally dirty worktree; the unchanged script is then run in a
temporary VCS-free mirror. No `--allow-dirty`. No golden or fixture
regeneration. A known `CreateProcessAsUserW failed: 5` or linker
environment failure is reported, not thrashed.

## 25. Publication authority and record

At coordinator handoff, the contract candidate was uncommitted and
unpushed, and K3 performed no commit, push, or GitHub metadata action. The
user subsequently exercised sole publication authority: committed the
candidate as `379bbb5` on branch
`agent/standing-policy-v4-direct-refutation-contract`, pushed it, and
opened draft PR #71. The user retains sole publication, readiness, and
merge authority.

## 26. Sol review adjudication record

Two independent, fresh, read-only `gpt-5.6-sol` (xHigh) audits ran before
drafting. Sol A (architecture audit) and Sol B (feasibility/adversarial
audit) each returned `PROCEED WITH CONDITIONS`. They received no proposed
solution, no maker self-review, and no shared inputs. Their verdicts are
review evidence, not epistemic authority; K3 independently verified every
material claim against the repository and adjudicated as follows.

| Finding | Source | Decision | Repository evidence | Effect on contract |
| --- | --- | --- | --- | --- |
| Explicit policy v4 inheriting the complete v3 result | A, B | accepted | v0-v3 identifier series; v3 composes v2; forking would drop corroboration | §7, §8, §9 |
| Single closed lane: `DeterministicVerification × ExactMachineCheckable × contradicts × sha256_bytes_equals_v0` | A, B | accepted | refutation ceiling cell; checker prerequisites; ADR edge vocabulary | §6, §10, §11 |
| Existing `DigestMismatch` trace never authoritative | A, B | accepted | unit failure variant, no identities; pinned v2 law keeps a `Matched` path `Supported` beside another path's mismatch | §11 |
| New private-construction negative receipt inside the v4 resolver; no public sibling resolver; no generalized public predicate-evaluation API | A, B | accepted | receipt/private-construction doctrine in verifier context and v3 | §8, §9, §18 |
| One exact negative contribution suffices; non-amplifying | A, B | accepted | refutation ceiling is a direct-contribution surface; v2 direct-support doctrine | §13 |
| No `Occurrence/Inclusion` negative rule; anchor absence is not non-occurrence | A, B | accepted | no negative substrate; open append-only history | §10 |
| Legacy raw `Refuted` stays quarantined | A, B | accepted | PR #56 pinned laws | §14 |
| Inherited `Settled` preserved (Sol A/B leaned toward refutation outranking Settled) | A, B | modified | domains are single-valued; v1 settles only `Occurrence/Inclusion`, so the case is unreachable; v4 creates no demotion law | §12 |
| Conflicting exact positive + negative verifications: Sol A/B leaned toward `Refuted` outranking `Supported` | A, B | modified | Sol A's own subject-identity stop condition; ADR-0002 debt doctrine; the conflict is genuine verified disagreement | closed `ConflictingDeterministicVerification` blocker, inherited preserved, debt stays future (§12) |
| Changed-path boundary of six paths incl. the verifier-admission contract doc | A | modified | that doc's non-goals are phase-scoped and accurate; smallest-set discipline; the v4 note carries the distinction | five-path allowlist (§4) |
| Resolver signature shape `(claim_id, closure)` | A | accepted with exact v3 mirroring | v3 resolvers take `(&self, claim_id: &str, closure: &ResolutionContentClosureV0)`; the closure is explicit content input, not context-owned | §8 |
| Evidence `content_hash` not validated by the checker | B | accepted | mirrors the positive checker boundary; witness bytes are the checked content | stated in §11 scope |
| Closure participates only via v3 inheritance | A, B | accepted | the closure is explicit content input to the v3/v4 resolvers; the digest comparison does not consume it | §9 |

### Final review rounds

After the two concurrent architecture audits, seven fresh, read-only
`gpt-5.6-sol` (xHigh) hostile reviews of the candidate diff ran. Each
received no prior reviewer's report and no maker self-review, and K3
independently verified every finding against the repository before
remediating. No round returned `PASS`; every round returned numbered
must-fix findings, all remediated:

| Round | Verdict | Findings | Remediation |
| --- | --- | --- | --- |
| Sol C (18 falsification targets) | FAIL | 2 — closure signatures/context-ownership; edge-ID versus replay ordering | v3-exact signatures with explicit closure parameter; lexicographic edge-ID order |
| Closure 1 | FAIL | 2 — `Refutation Contribution` definition; ceilings-doc rollout step 5 | remediated as reported |
| Closure 2 | FAIL | 1 — ceilings-doc pre-v1 Status/Purpose framing | time-bounded; same-law Non-goals framing |
| Closure 3 | FAIL | 2 — unsatisfiable compile-fail reservation; test-matrix/rollout framing | narrowed to struct-literal boundary (constructible inert enums never accepted as authority); time-bounded framing |
| Closure 4 (convergence) | FAIL | 1 — lane-enforcement hole for the two other `Some(Refuted)` cells | closed `UnsupportedDirectRefutationLane` added |
| Closure 5 (convergence) | FAIL | 1 — eligibility items 3/4 exact-value gates | remediation prescribed; initially not applied (K3 process error, caught by the next round) |
| Closure 6 (final) | FAIL | 1 — same items 3/4 issue plus hostile-row ambiguity | remediated and grep-verified; self-verified by K3 with no further round per user directive |

The task's four-agent routing budget was exceeded (nine fresh agents) with
the user's explicit, repeated authorization in favor of the mandatory
fresh-closure rule. The final two-sentence amendment (parse-then-compare in
eligibility items 3/4 of both documents) implements the final reviewer's
own prescribed remediation and is self-verified; it has no independent
re-review.

## 27. Suggested reviewer checklist

1. Exactly the five authorised paths changed; no runtime, test, fixture,
   Cargo, L0, format, golden, verifier, or CI change.
2. The only selected lane is `DeterministicVerification ×
   ExactMachineCheckable × contradicts × sha256_bytes_equals_v0`.
3. The existing `DigestMismatch` trace is disqualified from negative
   authority and unchanged.
4. `PredicateFalse` requires every exact prerequisite and is the only
   authoritative negative outcome.
5. The conflict law produces the closed blocker, never a silent override;
   contradiction debt remains future.
6. Inherited `Settled` and `Refuted` are preserved; repetition is
   non-amplifying; legacy raw stays quarantined.
7. V4 inherits the complete internally derived v3 result; no ambient or
   caller-selected policy path exists.
8. Absence never becomes falsity; no `Occurrence/Inclusion` negative rule
   appears.
9. Serialization boundaries hold (`Serialize` only; no `Deserialize`,
   `Default`, or public constructors; private fields; deterministic bytes).
10. The ticket and the design note agree exactly.
11. Every reported validation actually ran.
