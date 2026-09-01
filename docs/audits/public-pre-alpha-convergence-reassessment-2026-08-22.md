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
| Current reconciled `main` (2026-08-24) | `7e7cb5a930b1db4f2ebc1c5ad0296641cd74db6d` |
| Current reconciled `main` (2026-09-01) | `cb7d2d89f3f02293e4552db3944e9940b9be15eb` |
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
| #136 | `e2a3f9dca6f23b148062a0f7c69cbd17893f1318` | Rust portable raw-input frontend contract and Ticket 0076 |
| #137 | `7e7cb5a930b1db4f2ebc1c5ad0296641cd74db6d` | authoritative crate-internal Rust portable frontend through `Schema` |
| #139 | `684661353a18528e62b2bfa3c3f4746a4201879e` | complete crate-internal Rust portable-history conformer |
| #140 | `d7ee7101c83e5a3c0b1b23447deef291f4e303c5` | bounded Rust fuzz/metamorphic assurance |
| #141 | `8ae50f2b093ce33de4dee3cb9421e3f5ee5eb0f4` | README refresh and pre-alpha evidence boundary |
| #142 | `caa946728d7cf8f859766fd67e4225919cb0678f` | independent complete Python conformer |
| #143 | `b7970ef4994bff6dc82761829277bf894ec4bd62` | independent Go conformer and strict three-way differential |
| #144 | `d65cba1ab4eb9e3d8b8d9c9f0848bb3c643f7506` | hosted CI and final candidate evidence |
| #145 | `f2729b32e60a3df5808b2824342eddc264d9a482` | administrative reconciliation that retained the three path findings open |
| #146 | `cb7d2d89f3f02293e4552db3944e9940b9be15eb` | contract-qualified Rust production/file path, `tools/verify_chain.py`, and rerun differential |

The reassessment date and the evidence attributed to its original
`169a94e16dd0f6f53620302c09e4b2f041b61cb9` baseline remain historical facts.
The 2026-08-24 rows record the living baseline at that date after #132, #135,
#136, and #137; the 2026-09-01 row and #139-#146 sequence record the later current
state without backdating those merges or rewriting the 2026-08-22 owner
decision.

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
results. It is not itself a parser, verifier, conformer, proof that a host
library implements ADR-0010, or proof of universal cross-language agreement.
The separately reviewed and owner-merged #146 differential now establishes
bounded agreement for this frozen corpus; corpus approval alone did not close
A-021, A-004, RQ-006, or the wider RQ-013 assurance finding.

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
portable corpus intentionally does not mint a complete `G_history`; PR #146
completes the selected portable-history semantics and real file/public path,
while a coordinate-complete detached result context remains a separate C3
decision. This is not evidence that another constitutional owner choice is
currently needed for the completed conformer tranche.

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

## C2 — portable verification (historical 2026-08-24 assessment)

**Achieved.** The reference JSONL input-language contract fixes transport,
first-failure, coordinate, and result semantics. Accepted ADR-0010 fixes
`V_sig = "magpie-ed25519-canonical-prime-subgroup-v1"`. PR #131 adds the exact
432-case oracle, clean-checkout integrity checker, hostile coverage inventories,
independent manifest digest, repeated adversarial review, and the subsequent
owner approval recorded here. PR #135 adds the authoritative namespaced Rust
`ProfiledSignatureVerifier`: profile selection is typed, explicit, and
fail-closed; external-key admissibility is bound at construction; and the exact
uncofactored ADR-0010 relation is implemented without changing the legacy
unprofiled verifier. PR #136 froze the next implementation boundary, and PR
#137 landed the authoritative crate-internal Rust raw-input frontend: exact
profile/key preflight, exact-byte framing and UTF-8, the locked `serde_json`
syntax seam, duplicate-preserving closed schema/lexical decoding, and a private
`PendingRecord` that owns the exclusive continuation. Its production-path
corpus evidence proves all 324 frontend-tranche-final outcomes and accounts for
the remaining 108 cases without preempting their later semantic result.

**Remaining.** The merged cryptographic primitive and frontend are not a
complete portable conformer. Rust still lacks the six-stage portable-history
consumer and complete end-of-history result. Python still lacks a complete
profile-aware conformer, no independent Go conformer exists, and no three-way
differential run exists. The corpus is expected output, not cross-language
conformance evidence.

**Excluded.** No implementation may silently tighten legacy v1, infer a
profile by default, fall back, negotiate `latest`, substitute a library's
ordinary/strict API for Magpie doctrine, or turn structural key admissibility
into key trust.

**Next gate.** Implement the complete crate-internal Rust portable-history
conformer over the merged frontend under the
[frozen Rust conformer contract](../design/portable-verifier-rust-history-conformer-contract-v0.md).
It must consume one pending record at a time, apply `Sequence -> PreviousLink
-> ContentHash -> Signature -> PayloadValidation -> Genesis`, update count/tip
only after complete record success, and be the sole production authority that
releases the continuation. Independent Python and Go conformers, differential
execution, and hostile review remain later gates.

**Current reconciled state (2026-09-01, after owner-merged PR #146).** C2 is
now **Achieved** for the narrow portable-verification claim. The selected
`magpie-ed25519-canonical-prime-subgroup-v1` profile, frozen exact
`magpie-portable-verifier-corpus-v1` (432 cases; 19 ACCEPT, 413 REJECT), and
complete Rust semantics are exercised through the real non-test public
`FileStore::verify_portable_history` file path. That path performs complete
profile/external-key preflight before acquisition, retains exact bytes and
blank/terminator distinctions, and computes the complete governed result from
the same production call without changing legacy `LogStore::read_records`
compatibility semantics. Python independently exercises
`tools/verify_chain.py` itself, and the standalone Go conformer remains
independent. The selected-root differential compares `verdict`, `class`,
`line`, `record_index`, `event_count`, `tip`, and
`ordered_recomputed_hashes` across all 432 cases, is stable under `--repeat 2`,
and has zero unexplained divergences. Hostile review remediated the path
defects, all review threads were resolved, hosted CI was green, and the owner
merged PR #146 (`cb7d2d89f3f02293e4552db3944e9940b9be15eb`).

This closure is bounded to the selected portable language/profile and supplied
finite histories. It does not establish key trust or ownership, real-world
identity, delegation, authority, permission, freshness, latest-head or
canonical-branch selection, anti-rollback, non-equivocation, global
completeness, production durability, truth, cryptographic proof, universal
verifier equivalence, or release readiness.

## C3 — producing coordinates and derived outputs (historical 2026-08-24 assessment)

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

**Current reconciled state (2026-09-01).** C3 remains a boundary decision and
not an automatic implementation requirement. PR #146 intentionally keeps the
complete portable semantic result crate-internal and adds no detachable
governed result or receipt. If the advertised pre-alpha excludes detachable
output, preserve that exclusion; if it requires one, handle the smallest
coordinate-complete result or receipt in a separate C3 tranche.

## C4 — public assurance (historical 2026-08-24 assessment)

**Achieved.** The repository now has the exact corpus/digest binding, hostile
byte and coverage inventories, a clean-checkout CI integrity check, multiple
adversarial corpus-review rounds, typed checkpoint tests, and extensive SQLite
resource, schema, tamper, stale/concurrent-writer, transaction, crash-recovery,
and unknown-commit tests. PR #135 also adds focused hostile cryptographic-
boundary evidence for canonical encodings, subgroup membership, identity
asymmetry, scalar boundaries, challenge binding, and the exact uncofactored
equation.
PR #137 adds production-path hostile evidence for exact profile/key precedence,
framing and UTF-8 boundaries, decoded duplicates, raw integer and hex lexemes,
parser-diagnostic independence, and exclusive continuation ownership across the
exact 324/108 corpus split.

**Remaining.** There are no independent portable conformers or differential CI
runs. GitHub Actions, Rust `stable`, `ubuntu-latest`, and the Python crypto
installation remain moving inputs. Wider platform/release-environment evidence,
compatibility-reader limits, and the stale-boundary compile-fail examples in
A-018 remain open.

**Excluded.** Routine green CI is not release assurance, corpus integrity is
not verifier correctness, and local SQLite evidence is not a network or
distributed-storage guarantee.

**Next gate.** Complete the bounded Rust history conformer, then a separate
non-normative Rust fuzz/metamorphic tranche before treating Rust as
cross-language comparison evidence. After independent Python and Go conformers
and full differential evidence exist, pin the applicable toolchain/action/
Python inputs and collect only the additional platform and release-environment
evidence required by the selected boundary.

**Current reconciled state (2026-09-01).** The exact corpus/digest binding,
hostile inventories, focused profile/frontend evidence, contract-qualified Rust
file/public path, independent `tools/verify_chain.py`, independent Go
conformer, seven-field 432-case differential, repeat-two stability, hosted CI,
and review/merge evidence are now achieved evidence for the bounded portable
verification surface. C4 remains **Partially remediated** overall because
`ubuntu-latest`, Rust `stable`, action major tags, complete artifact/environment
identity, native/libsodium reproducibility, broader platform evidence, and
A-018 exact-boundary/resource/crash assurance remain open. Green CI and corpus
agreement are not release assurance or a claim of universal verifier or
cryptographic correctness.

## C5 — release falsification (historical 2026-08-24 assessment)

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

**Current reconciled state (2026-09-01).** C5 remains **Pending**. The #146
portable-verification correction and this administrative reconciliation add
bounded evidence but do not create a public pre-alpha release contract, freeze
a release candidate, complete whole-candidate falsification, or make an owner
release decision.

## Current critical path (historical through 2026-08-24)

The evidence supports this order:

```text
complete Rust portable-history conformer over the merged frontend
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

## Current critical path (2026-09-01)

The portable-conformer implementation and reconciliation steps above are now
complete on current `main`. The current non-authorizing planning order is:

```text
C3 boundary decision:
  preserve the exclusion if the advertised pre-alpha does not need detachable
  governed output
  OR add one narrow coordinate-complete verification/replay result/receipt if it does
-> C4 boundary-specific public assurance and reproducibility hardening
-> C5 public pre-alpha release contract
-> freeze one exact candidate
-> hostile whole-candidate falsification
-> explicit owner release decision
```

This order does not make C3 mandatory, convert C4 evidence into release
approval, or authorize implementation. The portable closure remains bounded to
the selected profile and supplied finite histories; compatibility debt and the
non-claims above remain in force.

## Administrative consequences (historical through 2026-08-24)

- The corpus lifecycle is now owner-approved, merged, frozen, and
  non-repointable. The manifest's embedded freeze-time candidate string remains
  byte-identical.
- PR #135 merged the authoritative Rust `V_sig` primitive and explicit
  fail-closed profile selection. It did not add raw portable parsing, complete
  history conformance, cross-language evidence, or finding closure.
- PR #136 froze the Rust frontend architecture, and PR #137 merged its
  authoritative crate-internal implementation through `Schema`, with 324 exact
  frontend-final results and explicit non-preemption accounting for the other
  108 cases. It did not add the six remaining history stages, a complete Rust
  result, cross-language evidence, or finding closure.
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

The narrowest next executable tranche at that historical coordinate was
therefore the complete crate-internal Rust portable-history conformer over the
merged frontend, with merged profiled `V_sig` retained as the exact
`Signature` primitive rather than reimplemented.

## Current administrative consequences (2026-09-01)

- PR #146 is owner-merged at `cb7d2d89f3f02293e4552db3944e9940b9be15eb` and
  closes A-004, RQ-006, and A-021 only after the contract-qualified Rust
  production/file path, independent `tools/verify_chain.py`, independent Go,
  exact 432-case seven-field differential, repeat-two stability, hostile
  review, and merge evidence were verified and reconciled.
- RQ-013 remains **Partially remediated**: the portable path gap is removed,
  but A-018 exact-boundary compile-fail assurance, broader platform/resource/
  crash evidence, and broader release/environment reproducibility remain.
  RQ-015 remains open for moving runner/toolchain/action/native inputs. A-011
  remains Partially remediated for its supply-chain/release remainder.
- C3 is a boundary decision, not an automatic implementation requirement. C4
  remains assurance/reproducibility work, and C5 remains the unreleased
  contract/falsification gate. No release, trust, authority, currentness,
  global-completeness, or cryptographic-proof claim follows from this
  reconciliation.
