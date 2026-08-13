# Public pre-alpha convergence baseline — 2026-08

## Status and administrative boundary

This is a living administrative programme baseline for one pinned `main`
revision. It is non-doctrinal and non-authorizing. It is not an ADR, an
implementation contract, a release contract, a historical audit, or evidence
that any unresolved finding is closed. It neither ratifies nor amends any
existing ADR, format, policy, audit disposition, compatibility semantics, or
release boundary.

The programme labels and readiness classifications below are planning tools.
They do not authorize a work item, choose an owner, decide a constitutional
question, or make a release declaration. A planned remediation is not a
landed remediation, and excluding a capability from a possible public surface
does not close its audit finding.

## Pinned baseline and evidence

| Field | Recorded value |
| --- | --- |
| Repository | `noctem-o/magpie` |
| Baseline date | 2026-08-13 |
| Live `origin/main` | `fbd02a9616da3c1aa64ff0f5a44e566e7ebd5bec` |
| Baseline relation | the live remote matched the expected post-PR #121 commit; there were no later mainline changes to assess |
| Relevant merged PRs | #108–#121; see the table below |
| Last formal source release | `v0.1.0` |
| Open remediation PRs at inspection | none |
| Open issues at inspection | none |
| Routine CI observation | `.github/workflows/ci.yml` runs formatting, Clippy, locked tests, the tour, release metadata, package verification, and the Python golden-chain verifier; `ubuntu-latest`, Rust `stable`, action major tags, and Python crypto installation remain unpinned |

The evidence hierarchy for this baseline is, in order:

1. current code, tests, public API, CI configuration, and Git history at the
   pinned revision;
2. accepted normative format and ADRs;
3. ratified narrow contracts and the v0.1.0 release contract;
4. the current audit-disposition ledger as administrative comparison;
5. preserved historical audit records as findings and evidence at their own
   recorded revisions; and
6. the owner-supplied external-audit synopsis described below.

This document is not an additional governing source in that hierarchy.

| PR | Merge commit | Recorded result |
| --- | --- | --- |
| #108 | `71d096ff3c7ca8107d51484f486c30031d337ee8` | ADR-0007 accepted |
| #109 | `b79a96ab0ddc5c8b9008231ef53e325efdd067d3` | origin-authority compatibility wording reconciled |
| #110 | `a1a59200c5a1af2c81afe17b76b89609134d5787` | claimant-origin compatibility wording quarantined |
| #111 | `dc07034d4af7d2b41130ba04a6a9d4c5af000995` | v2/v3 status promotion made exhaustive |
| #112 | `91ebb5d6a4f5ba99ad377c77eb96dc70bc829245` | statusless `ClaimAssertedV2` projection correction |
| #113 | `253fc72350ae5232fc428da9ab5657f8ccf5c912` | audit-disposition reconciliation |
| #114 | `142793a47c0f5a7e26ec29fc100d2d6cde9ddb3a` | sole-write boundary contract |
| #115 | `66198af8dc06b086e77036fafd5e18adcdd967b2` | public raw append sealed behind `LogWriter` |
| #116 | `c2fe991d8023d237f7d589bd3f022233c1f332cc` | sole-write disposition reconciliation |
| #117 | `99512b5eddd39ecc98e31e59173252da717f2fa2` | verified-replay projection boundary contract |
| #118 | `c70c0fb28157212f0f73cef88bae116658c787f7` | raw event/projection-input separation |
| #119 | `f6a4759ef3cdc2a3974aad4e2be57df58d04c1c3` | A-009 disposition reconciliation |
| #120 | `0bc8f5c8ab38e407ef87274fbb0e2f50d5fe65b6` | portable verifier input-language contract |
| #121 | `fbd02a9616da3c1aa64ff0f5a44e566e7ebd5bec` | A-021 Ed25519 verification-semantics contract |

The recent owner-run hostile review is available here only as an
owner-supplied synopsis. The repository contains no complete preserved artifact
for that external review. Its synopsis is treated as a lead: Magpie is a
strong experimental kernel whose supplied-byte verification is stronger than
its proof of semantic authority, expected history, or cross-implementation
agreement. This baseline does not reconstruct or preserve a substitute audit.

## Purpose and candidate advertised boundary

The programme goal is to produce the smallest public-facing pre-alpha boundary
whose advertised guarantees survive hostile review. It is not a programme to
eliminate every historical finding or to construct every future architecture.

Until the constitutional reassessment below, the only candidate boundary this
baseline can describe safely is an experimental, local, supplied-snapshot
kernel: it can parse and verify a supplied record snapshot under an externally
selected key, replay that verified snapshot into derived views, and make
current policy results available only within the explicit input and policy
limits documented for each current path; it does not state a complete
producing-coordinate law. It must not be advertised as proving an externally
expected head, freshness, durable physical storage, authority-bearing
corroboration, portable cross-language agreement, or a detached result's full
producing context.

This is a candidate boundary, not a release promise. C0 determines whether it
is sufficient and what, if anything, may be advertised beyond it.

## Pre-alpha readiness vocabulary

This vocabulary is separate from the audit ledger's controlled **primary
disposition** vocabulary. A ledger finding can remain `Confirmed` while the
candidate release boundary excludes the capability in question.

| Readiness classification | Meaning |
| --- | --- |
| **PRE-ALPHA BLOCKER** | Must be corrected before the named advertised public boundary may ship. |
| **BOUNDARY EXCLUSION** | Existing behaviour may remain, but it must not inhabit or be presented as part of the trusted advertised surface. |
| **COMPATIBILITY-ONLY** | Historically reproducible behaviour retained without an upgrade in semantic authority. |
| **FUTURE IMPLEMENTATION GATE** | No present blocker unless the gated capability is added to the boundary. |
| **ACCEPTABLE PRE-ALPHA LIMITATION** | A known limitation that may remain when stated explicitly. |
| **CLOSED FOR PRE-ALPHA SURFACE** | Objective evidence shows the relevant advertised boundary is already satisfied; this says nothing about a broader audit finding. |

Classifications are conditional when the advertised surface is still an owner
decision. They never replace, aggregate, or rewrite the audit ledger's primary
dispositions.

## Convergence programme

| Programme | Objective | Major finding families | Release relevance and dependency | Explicitly out of scope | Reassessment point |
| --- | --- | --- | --- | --- | --- |
| C0 — Constitutional Convergence | Make the minimum coordinate and history laws coherent enough to describe a public boundary. | ADR-0003 producing-coordinate contradiction; verified-prefix versus expected-history semantics; claimant-v3 public treatment; public crypto/write profile. | Precedes the programme's public claim. It supplies constraints, not runtime authority. | Drafting the successor ADR decisions in this baseline. | After the owner decides the coordinate law and history vocabulary. |
| C1 — History & Persistence | Define the physical-history, open, acknowledgement, and resource laws needed by any advertised persistence claim. | A-008, RQ-007, RQ-008, NEW-LOG-01. | Depends on C0's history vocabulary; blocks any durable, concurrent, checkpoint-matched, or externally expected-history claim. | FileStore redesign in this baseline. | After L0 persistence and checkpoint/open contracts are assessed against the chosen boundary. |
| C2 — Portable Verification | Turn the contracted input language and signature relation into independently conforming portable verification. | A-004, RQ-006, A-021. | Depends on exact C0 profile decisions where necessary; blocks a portable-verification claim. | Treating a contract, one Rust path, or a future Go oracle as present conformance evidence. | After the corpus and independently exercised conformers agree. |
| C3 — Producing Coordinates & Derived State | Prevent trusted or detachable outputs from losing the identity of the computation that produced them. | A-003, A-007, A-012, A-019, A-023, RQ-009, RQ-010, RQ-011. | Depends on C0 coordinates; blocks detached authority-bearing outputs and derived-state publication claims. | Publishing a general query, librarian, or projection-store authority surface. | After coordinate carriage, fallible construction, foreign-store refusal, and publication semantics are re-evaluated together. |
| C4 — Public Assurance | Make the public repository's assurance story match the boundary actually advertised. | A-011, A-018, RQ-013, RQ-015. | Depends on the chosen public boundary and the preceding programme outcomes. | Declaring routine green CI equivalent to release assurance. | After public-repository security, pinned toolchain, platform, executable-tour, and front-door evidence are reviewed. |
| C5 — Release Falsification | Freeze a candidate and seek independent evidence against the release contract. | Cross-programme release evidence and hostile counterexamples. | Depends on a bounded candidate, C0–C4 decisions applicable to it, and a stated release contract. | Broad architecture expansion during final falsification. | Independent hostile audit; remediate only demonstrated release-contract violations, then reassess publication. |

The order is provisional. Each reassessment may delete, split, defer, or move
programme work; no C0–C5 item is an irrevocable implementation commitment.

## Initial readiness map

| Finding family | Baseline evidence | Pre-alpha readiness classification | Boundary consequence or owner decision |
| --- | --- | --- | --- |
| A-024 claimant-controlled authority amplification | ADR-0007 requires an additive authority-bound successor; current v3 may reach `Supported` from claimant-selected authority-looking labels and distinct origin groups. | **BOUNDARY EXCLUSION** | Current v3 outputs may remain retained as compatibility/historical output, but must not be advertised as independently authority-bound corroboration. An owner must decide whether any claimant-v3 surface is public. |
| NEW-LOG-01 valid prefix versus expected history | Verification covers the supplied snapshot as a whole; no external expected-head or checkpoint-match mechanism is present. | **BOUNDARY EXCLUSION** | Use `verified supplied snapshot` or `verified prefix`, never freshness, expected-history, or head-completeness language. Becomes a **PRE-ALPHA BLOCKER** if such a claim is advertised. |
| A-008 / RQ-007 / RQ-008 physical persistence | The sole-write capability boundary is landed, while `FileStore` lacks a full durability, locking, framing, acknowledgement, crash, and outer-resource law. | **BOUNDARY EXCLUSION** | Do not advertise FileStore as durable, crash-safe, concurrent-writer-safe, or a checkpoint-matched history source. |
| A-004 / RQ-006 portable input language | PR #120 supplies a contract; current Rust and Python behaviour remains divergent and no Go conformer exists. | **PRE-ALPHA BLOCKER** for a portable-verification claim | C2 must provide the corpus and independent conformers before that claim may be public. A Rust-only experimental boundary must say that it is not portable verification. |
| A-021 signature semantics | PR #121 supplies the v1 relation; it does not itself exercise the relation as a cross-language protocol. | **PRE-ALPHA BLOCKER** for a portable-verification claim | Do not treat the decision contract as cross-language realization. |
| NEW-ADR-01 producing-coordinate contradiction | ADR-0003 says standing has exactly snapshot, policy version, and resolver coordinates; closure-dependent paths consume further immutable material. | **PRE-ALPHA BLOCKER** for any advertised reproducible standing claim beyond the constrained candidate boundary | C0 owner decision: reconcile the complete producing-coordinate law without amending it here. |
| A-003 / A-019 / A-023 / RQ-011 detached results | Older standing and query/result surfaces do not carry complete immutable producing coordinates. | **BOUNDARY EXCLUSION** | No detached result, response, search row, or legacy resolution may be presented as a reproducible authority-bearing public result. |
| A-007 / A-012 / RQ-009 / RQ-010 derived state | Projection failure/publication and foreign SQLite ownership remain insufficiently governed. | **BOUNDARY EXCLUSION** | Derived state may remain rebuildable local material, not an authority-bearing or safely publishable public store. |
| A-015 / A-017 / RQ-003 authority-looking compatibility values | Public legacy fields and constructors retain values that can be mistaken for governed standing or currentness. | **COMPATIBILITY-ONLY** | Preserve historical shapes without presenting raw values as authority, currentness, or an upgraded policy result. |
| A-011 / A-018 / RQ-013 / RQ-015 release assurance | The current workflow is routine CI, not a pinned multi-platform, security, differential, crash, or independent release-assurance programme. | **PRE-ALPHA BLOCKER** for public release declaration | C4 and C5 determine the evidence needed for the eventual claimed boundary; green routine CI alone is insufficient. |
| A-016 / A-022 admission and withdrawal | The ledger records these as future gates; no generic admission gate or withdrawal runtime is present. | **FUTURE IMPLEMENTATION GATE** | Do not add generic admission, broad write paths, or withdrawal/identity semantics without their separate governing work. |
| A-010 currentness | Currentness remains a separate deferred capability with no runtime. | **FUTURE IMPLEMENTATION GATE** | Do not imply a current, fresh, superseding, or ambient-latest result. |
| A-001/RQ-001 and RQ-002; A-005/RQ-005 and A-006/RQ-004 | The ledger records objective evidence for the sealed append capability, verified replay input boundary, exhaustive promotion, and statusless projection correction. | **CLOSED FOR PRE-ALPHA SURFACE** | These specific, narrow boundaries may be relied on only within their documented limits; they do not establish physical persistence, expected history, portable verification, or semantic authority. |

## Explicit pre-alpha non-goals

This programme does not automatically require or authorize:

- a general `EpistemicGate`, generic claim/evidence admission, broad agent
  write paths, network/filesystem ingestion, vector retrieval, source
  reputation, model-scored trust, general truth resolution, general
  contradiction resolution, currentness runtime, or withdrawal runtime;
- a full identity/delegation platform, organisation semantics, production
  KMS/HSM, multi-node consensus, or federation; or
- the full ADR-0007 runtime unless a later owner decision makes it necessary
  for the selected public boundary.

## Reassessment gates and next decision

The programme must be reconsidered:

1. after the C0 constitutional decisions;
2. after L0 history/persistence work and portable verification evidence;
3. after producing-coordinate and derived-state hardening; and
4. before adding any authority-bearing corroboration runtime.

The expected next tranche is constitutional, not runtime: reconcile the
complete producing-coordinate law around ADR-0003, then define verified-prefix
versus checkpoint-matched-history semantics. This baseline neither authorizes
nor decides either tranche.

## Relationship to the audit-disposition ledger

`audit-disposition-2026-08.md` remains the controlled current-main
administrative ledger for historical audit findings. Its counts, primary
dispositions, and evidence statements are unchanged. This baseline adds a
separate readiness lens for a possible public boundary; it must not be used to
claim that an excluded, compatibility-only, or future-gated finding is closed.
