# Public pre-alpha convergence reassessment — 2026-08-22

## Status and authority boundary

This is a living administrative reassessment of the smallest defensible public
pre-alpha boundary at one pinned `main` revision. It supersedes current planning
in the [2026-08 historical baseline](public-pre-alpha-convergence-baseline-2026-08.md)
without rewriting that baseline's evidence or conclusions at its own date.

This document is not an ADR, implementation contract, release contract, audit,
finding closure, or authorization to start work. It does not amend FORMAT,
Accepted ADRs, the frozen portable corpus, compatibility semantics, the audit
ledger, or the v0.1.0 release boundary.

The owner decision recorded here is narrow: on 2026-08-22, after PR #131
merged and its final hostile review completed, the repository owner explicitly
approved the exact merged `magpie-portable-verifier-corpus-v1`. That authority
comes from the explicit owner statement, not from merge or CI success alone.

## Pinned evidence

| Field | Recorded value |
| --- | --- |
| Repository | `noctem-o/magpie` |
| Reassessment date | 2026-08-22 |
| Original 2026-08-22 pinned `main` | `169a94e16dd0f6f53620302c09e4b2f041b61cb9` |
| Current reconciled `main` (2026-08-24) | `a0910cc1c4cb7cc76204b26d5eaaeee6581570b9` |
| Last formal source release | `v0.1.0` |
| Corpus identity | `magpie-portable-verifier-corpus-v1` |
| Frozen manifest SHA-256 | `7d758d3f2dac1161fe15dd064b130ccbfcdaf0437ea8ab8d7772492194801a81` |
| Corpus lifecycle | owner-approved, merged, frozen, and non-repointable |

The relevant convergence sequence is:

| PR | Merge commit | Evidence added |
| --- | --- | --- |
| #123 | `9fd723980d8dd53a7ee2f5adc4247bb357175f0a` | ADR-0008 complete-producing-coordinate proposal |
| #124 | `f55a5327c399667525db3ed0cae5328cc2f7227d` | owner ratification of ADR-0008 |
| #125 | `88975aac6f646d35016fe31cb2777a463089a340` | ADR-0009 verified-history/checkpoint proposal |
| #126 | `9b88dbe4e45b4ba060cf0abc60475e8cbef87e0c` | owner ratification of ADR-0009 |
| #127 | `2c38ffe32367bbb38d96525b938f148d40eb29d4` | typed history-checkpoint evaluation and hostile tests |
| #128 | `6afa60357d51f9996da1f5776e6d59ced70eba9f` | supported local SQLite L0 contract |
| #129 | `f3fc220faca402c15ba207b27caa8ce79cb032c7` | supported SQLite L0 runtime and hostile tests |
| #130 | `afc4ff7cf5dfe33f672723da3e4a9a9880752a67` | proposed exact additive Ed25519 verification subprofile |
| #131 | `169a94e16dd0f6f53620302c09e4b2f041b61cb9` | ADR-0010 ratification and exact portable corpus merge |
| #132 | `2751ac41748d6aba05c503c721dad84a2c89f916` | living post-#131 convergence and audit reconciliation |
| #135 | `a0910cc1c4cb7cc76204b26d5eaaeee6581570b9` | merged typed Rust `V_sig` primitive and explicit fail-closed profile selection |

The reassessment date and the evidence attributed to its original
`169a94e16dd0f6f53620302c09e4b2f041b61cb9` baseline remain historical facts.
The 2026-08-24 row records the current living baseline after #132 and #135; it
does not backdate either merge or rewrite the 2026-08-22 owner decision.

Current implementation and tests outrank PR descriptions. Accepted FORMAT and
ADRs follow; frozen narrow contracts and the exact corpus come next; historical
audits remain evidence at their recorded revisions. This reassessment adds no
new governing source.

## Frozen corpus lifecycle

The owner-approved mapping is exactly:

```text
magpie-portable-verifier-corpus-v1
-> 7d758d3f2dac1161fe15dd064b130ccbfcdaf0437ea8ab8d7772492194801a81
```

The literal manifest value
`"status": "candidate-pending-owner-review-and-merge"` is freeze-time metadata.
The approval occurred later and is recorded in living documentation. Editing
that field would change the exact approved manifest bytes and unlawfully
repoint v1, so it remains untouched.

The corpus is a 432-case exact-byte oracle with 19 ACCEPT and 413 REJECT
results. It is not a parser, verifier, conformer, proof that a host library
implements ADR-0010, or cross-language agreement. Its approval therefore does
not close A-021, A-004, RQ-006, or the wider RQ-013 assurance finding.

## Smallest candidate public claim

The smallest claim worth testing is:

> Given an externally supplied verification key, an exact supplied history,
> explicitly identified immutable verification semantics, and, when requested,
> an explicit caller-supplied checkpoint, Magpie can verify that supplied
> history under the selected semantics, distinguish history verification from
> checkpoint expectation, replay verified material deterministically, and
> reproduce governed results only within explicitly identified producing
> inputs.

This is not yet a release claim. In particular, current evidence does not
establish that all portable conformers agree. Success also does not establish
key ownership or trust, checkpoint freshness, latest state, a canonical branch,
global completeness, absence of an unseen suffix or fork, non-equivocation,
truth, authority, currentness, standing as truth, or permission to act.

## C0 — constitutional convergence

**Achieved.** ADR-0008 fixes the constitutional law that a governed detached
result must retain or be immutably bound to its complete producing context.
ADR-0009 separates verified supplied history from explicit caller checkpoint
expectations. ADR-0010 fixes the exact additive signature-verification
subprofile while retaining unprofiled v1 behavior as compatibility-only.
ADR-0007 fixes future authority-bound corroboration doctrine, and current
claimant-label behavior is explicitly quarantined as compatibility semantics.

**Remaining.** Existing runtime outputs do not all realize ADR-0008. The
portable corpus intentionally does not mint a complete `G_history`; the raw
portable frontend, complete-history conformer, and complete result context
remain distinct implementation work at the C2/C3 boundary. This is not
evidence that another constitutional owner choice is currently needed before
the frontend tranche.

**Excluded.** Authority-bound corroboration runtime, generic admission,
currentness, withdrawal identity/delegation, librarian/MCP authority, and
ambient policy selection are outside the smallest pre-alpha. Exclusion does
not close their findings or ratify implementation.

**Next gate.** Preserve the Accepted decisions while implementing their exact
selected semantics. Reopen C0 only if implementation exposes a genuine
constitutional ambiguity rather than inventing one administratively.

## C1 — history and persistence

**Achieved.** PR #127 supplies typed exact/contains checkpoint evaluation over
one completely verified supplied history, with explicit Satisfied and
NotSatisfied outcomes. PRs #128/#129 select and implement `SqliteL0Store` as
the supported local persistence path: explicit create and verified reopen,
finite database/count/record/total-byte limits, record-at-a-time stable-snapshot
verification, checkpoint-relative reopen, atomic successor append, persisted
terminal revalidation, stale-writer rejection, and conservative poisoning/
reopen on unknown commit state.

**Remaining.** `FileStore` remains a small JSONL compatibility, development,
fixture, export, and inspection backend; it is not the supported durability or
concurrency boundary. Broader resource behavior through compatibility readers
and projection fallibility remain separate debt.

**Excluded.** Secure retained checkpoints, trusted remembered heads,
whole-system anti-rollback, freshness/latest, canonical-branch selection,
global completeness, non-equivocation, distributed consensus, network-filesystem
guarantees, and production KMS are not established.

**Next gate.** No new C1 runtime is required for the narrow supported-local-L0
claim. Preserve the exclusions and reassess only if the advertised storage or
rollback claim expands.

## C2 — portable verification

**Achieved.** The reference JSONL input-language contract fixes transport,
first-failure, coordinate, and result semantics. Accepted ADR-0010 fixes
`V_sig = "magpie-ed25519-canonical-prime-subgroup-v1"`. PR #131 adds the exact
432-case oracle, clean-checkout integrity checker, hostile coverage inventories,
independent manifest digest, repeated adversarial review, and the subsequent
owner approval recorded here. PR #135 adds the authoritative namespaced Rust
`ProfiledSignatureVerifier`: profile selection is typed, explicit, and
fail-closed; external-key admissibility is bound at construction; and the exact
uncofactored ADR-0010 relation is implemented without changing the legacy
unprofiled verifier.

**Remaining.** The merged cryptographic primitive is not a portable conformer.
Rust still lacks the exact raw-input frontend and complete portable-history
conformer. Python still lacks a complete profile-aware conformer, no independent
Go conformer exists, and no three-way differential run exists. The corpus is
expected output, not executable conformance evidence.

**Excluded.** No implementation may silently tighten legacy v1, infer a
profile by default, fall back, negotiate `latest`, substitute a library's
ordinary/strict API for Magpie doctrine, or turn structural key admissibility
into key trust.

**Next gate.** Implement the exact Rust portable raw-input frontend against the
frozen input-language contract. It must preserve record-by-record first-failure
ordering and use the merged `ProfiledSignatureVerifier` only as the downstream
signature primitive. Complete Rust history semantics, independent Python and Go
conformers, differential execution, and hostile review remain later gates.

## C3 — producing coordinates and derived outputs

**Achieved.** ADR-0008 now supplies one constitutional coordinate law. Verified
replay has a typed non-forgeable input boundary, and checkpoint evaluations
retain their selected profile, external key bytes, verified-history summary,
checkpoint, relation, and outcome. Several newer policy/audit paths carry
stronger private prefix/closure coordinates.

**Remaining.** A-003's public v0-v2 standing outputs, A-019/RQ-011 episodic rows
and search results, and A-023 direct matched traces remain detachable and
coordinate-poor. ADR-0008 does not retroactively repair those shapes, and the
portable corpus does not define a complete history-derivation identity.

**Excluded.** Those compatibility surfaces must not be advertised as
coordinate-complete, reproducible, authority-bearing detached results. A
general query/librarian/MCP response architecture is outside this pre-alpha.

**Next gate.** After C2 conformance is real, decide from the exact advertised
surface whether exclusion is sufficient. If a detached governed result is
required, add one narrow coordinate-complete verification/replay result or
receipt; do not redesign every legacy output in this tranche.

## C4 — public assurance

**Achieved.** The repository now has the exact corpus/digest binding, hostile
byte and coverage inventories, a clean-checkout CI integrity check, multiple
adversarial corpus-review rounds, typed checkpoint tests, and extensive SQLite
resource, schema, tamper, stale/concurrent-writer, transaction, crash-recovery,
and unknown-commit tests. PR #135 also adds focused hostile cryptographic-
boundary evidence for canonical encodings, subgroup membership, identity
asymmetry, scalar boundaries, challenge binding, and the exact uncofactored
equation.

**Remaining.** There are no independent portable conformers or differential CI
runs. GitHub Actions, Rust `stable`, `ubuntu-latest`, and the Python crypto
installation remain moving inputs. Wider platform/release-environment evidence,
compatibility-reader limits, and the stale-boundary compile-fail examples in
A-018 remain open.

**Excluded.** Routine green CI is not release assurance, corpus integrity is
not verifier correctness, and local SQLite evidence is not a network or
distributed-storage guarantee.

**Next gate.** Complete the bounded Rust frontend and history conformer, then a
separate non-normative Rust fuzz/metamorphic tranche before treating Rust as
cross-language comparison evidence. After independent Python and Go conformers
and full differential evidence exist, pin the applicable toolchain/action/
Python inputs and collect only the additional platform and release-environment
evidence required by the selected boundary.

## C5 — release falsification

**Achieved.** Corpus semantics have undergone hostile falsification and the
exact approved oracle is frozen. That is one component of future release
evidence.

**Remaining.** There is no pre-alpha release contract, frozen release
candidate, complete boundary-specific assurance record, hostile whole-candidate
architecture/runtime/release audit, or owner release decision. v0.1.0 remains
the latest formal release.

**Excluded.** Corpus falsification must not be described as whole-pre-alpha
release-candidate falsification. Final review must not expand the architecture
while attempting to close the release contract.

**Next gate.** Only after the applicable C2/C3/C4 gates: write the pre-alpha
release contract, freeze one candidate, conduct hostile architecture/runtime/
release review, remediate demonstrated contract violations, and seek an
explicit owner release decision.

## Current critical path

The evidence supports this order:

```text
exact Rust portable raw-input frontend using merged profiled V_sig downstream
-> complete Rust portable-history conformer
-> bounded non-normative Rust fuzz/metamorphic assurance
-> independent Python complete conformer from governing documents and corpus
-> independent Go complete conformer from governing documents and corpus
-> Rust/Python/Go differential execution over the exact frozen 432 cases
-> hostile review of disagreements and evidence
-> separate A-021 / A-004 / RQ-006 ledger reconciliation
-> minimal coordinate-complete detached result/receipt, only if the advertised
   boundary requires one
-> boundary-specific public assurance hardening
-> pre-alpha release contract
-> frozen-candidate hostile falsification
-> explicit owner release decision
```

This order does not reintroduce completed constitutional decisions as pending,
does not authorize the next tranche, and does not make roadmap priority equal
to audit disposition. A finding may remain Confirmed while its capability is
excluded from the public boundary; a programme may be sufficient for the
narrow boundary while compatibility debt remains.

## Administrative consequences

- The corpus lifecycle is now owner-approved, merged, frozen, and
  non-repointable. The manifest's embedded freeze-time candidate string remains
  byte-identical.
- PR #135 merged the authoritative Rust `V_sig` primitive and explicit
  fail-closed profile selection. It did not add raw portable parsing, complete
  history conformance, cross-language evidence, or finding closure.
- A-021 remains Confirmed implementation/conformance debt, now narrowed to the
  absent complete portable pipeline and qualifying cross-language evidence.
- A-004 and RQ-006 remain interoperability debt until independently executed
  conformers agree.
- RQ-013 gains substantial corpus and SQLite evidence but retains wider
  assurance scope.
- Supported SQLite L0 evidence changes the current storage assessment without
  upgrading FileStore or claiming secure anti-rollback.
- ADR-0007 remains Accepted doctrine with no authority-bound successor runtime;
  that capability remains excluded from this smallest boundary.

The narrowest next executable tranche is therefore the exact Rust portable
raw-input frontend against the frozen input-language contract, with merged
profiled `V_sig` retained as a downstream primitive rather than reimplemented.
