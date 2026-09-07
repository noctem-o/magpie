# Public Pre-Alpha Assurance Profile v1

## 1. Status and authority

This document defines the C4.1 assurance-profile contract for the exact profile
identity `magpie-public-prealpha-assurance-v1`. It freezes evidence requirements
for a later assurance record; it does not report that those requirements are
satisfied.

This profile does not:

- release Magpie, approve a release, or freeze a C5 candidate;
- close findings, prove correctness, security, truth, or production readiness;
- create trust, authority, provenance, freshness, currentness, or publication;
- amend `docs/FORMAT.md`, an Accepted ADR, the frozen corpus, or the audit ledger;
- authorize implementation merely because a requirement is written here.

Owner merge may freeze this profile's shape. Evidence must later be produced for
the exact candidate, reviewed independently, and submitted to the owner for a
separate decision. The owner remains the sole authority for acceptance, finding
disposition, candidate selection, and release.

The C3 direction used here is owner-directed and narrow: the advertised
pre-alpha excludes a detachable governed verification/replay receipt. The
existing public verification path may compute its complete semantic result
internally over caller-supplied finite history, but that result is not advertised
as a detachable trust, freshness, currentness, authority, or publication object.
This profile therefore records no receipt and does not redesign coordinate-poor
compatibility surfaces.

## 2. Selected public boundary

The smallest defensible boundary at base `313287c8ac9a0b266ab3a477b143d79072b88650`
is:

> Given an explicitly supplied verifying key, exact finite history bytes, the
> selected immutable portable semantics, and where requested an explicit caller
> checkpoint, Magpie can verify those supplied bytes, distinguish verification
> from checkpoint expectation, replay verified material deterministically, and
> expose only results whose producing inputs remain inside the active call.

Inside the assurance boundary:

- the real public `FileStore::verify_portable_history` exact-byte path;
- `magpie-ed25519-canonical-prime-subgroup-v1` and the frozen
  `magpie-portable-verifier-corpus-v1` (432 cases, manifest SHA-256
  `7d758d3f2dac1161fe15dd064b130ccbfcdaf0437ea8ab8d7772492194801a81`);
- the Rust, independent Python, and independent Go conformer lanes and their
  seven-field differential result;
- deterministic governed-standing tour and replay/regeneration demonstrations;
- the supported local `SqliteL0Store` L0 profile, including finite limits,
  stable snapshots, stale-writer handling, and checkpoint-relative open;
- the selected hosted validation lane described in section 3.

Outside this boundary, and not silently promoted by this profile:

- `FileStore` durability, locking, concurrent-writer, crash, or bounded-read
  guarantees; it remains compatibility/development/inspection infrastructure;
- `MemStore` persistence or cross-process guarantees;
- detachable coordinate-complete results, receipts, librarian/MCP/Vesper paths,
  ambient authority, ordinary admission, or authority-bound corroboration;
- key trust, ownership, identity, delegation, permission, latest/canonical
  history, freshness, anti-rollback, non-equivocation, global completeness,
  distributed/network storage, production KMS, or general truth;
- arbitrary future platforms, release packages, registry publication, or a
  hermetic/bit-for-bit reproducible build.

## 3. Platform support boundary

The public assurance claim is limited to one hosted lane:

| Coordinate | Profile requirement |
| --- | --- |
| Selected runner label | `ubuntu-24.04` |
| OS family | Linux |
| Architecture | The observed `RUNNER_ARCH`, retained in the evidence record; the current lane is expected to report `X64` and must not be inferred if absent. |
| Hosted image | The actual `ImageOS` and `ImageVersion` emitted by the run, not a presumed immutable image. |
| Rust host/toolchain | The observed `rustc -Vv` and `cargo --version` for selected Rust `1.98.1`. |
| Python | Observed interpreter identity and version selected as `3.12.14`. |
| Go | Observed `go version`, `GOOS`, and `GOARCH` selected as `1.27.0`. |

`ubuntu-24.04` is a workflow selection, not an immutable machine image. A
successful run on another OS is supplemental evidence and does not widen this
claim. Widening the public platform boundary requires owner direction and a
separate evidence tranche covering each newly claimed platform; it cannot be
inferred from local development or a second incidental run.

## 4. Governed inputs and observed environment

The future assurance record must distinguish selected inputs from what actually
ran. A selected version or SHA says what the workflow requested. An observed
coordinate says what the execution reported. Neither alone proves hermeticity.

The following identities are governed because changing them can change the
candidate, semantics, acquisition, or comparison population:

| Identity | Required treatment | Reason |
| --- | --- | --- |
| Candidate SHA and actual checkout SHA | Retain both; require the actual checkout to be the candidate being claimed. | PR merge SHAs, candidate heads, and checked-out heads can differ. |
| Workflow ref and workflow commit SHA | Retain `GITHUB_WORKFLOW_REF` and `MAGPIE_WORKFLOW_SHA`; bind the executed workflow bytes. | CI instructions are part of the assurance method. |
| Action commit SHAs | Retain exact revisions for checkout, Python, Go, and cache actions. | Action implementation is selected execution input; tags alone are insufficient. |
| `Cargo.lock` | Hash and retain the exact file. | Rust dependency resolution affects the build and tests. |
| Python requirements | Hash and retain `tools/requirements-portable-verifier.txt`; record installed distributions. | The Python conformer and its parser/crypto boundary are exercised. |
| Go module files and vendor tree | Hash `go.mod`, `go.sum`, and the complete vendor tree; require `-mod=vendor`. | The independent conformer must be source-identifiable and offline-available. |
| Frozen corpus manifest and case inputs | Verify manifest sidecar, every case digest, governing-source digests, and exact case order. | Expected results are a fixed comparison oracle, not an implementation. |
| Release/package inventories | Record the package-check result and package identity. | Inventory and package checks are evidence of the selected source boundary. |
| Relevant workflow/configuration and fixtures | Bind any file used by a required lane; do not invent hashes for irrelevant files. | Hashes are useful only when the file materially controls the claim. |

The committed `magpie-validation-environment-v1` emitter records the governed
input hashes, candidate/checkout/workflow coordinates, run and attempt, selected
runner label, actual hosted image coordinates, toolchain outputs, Python package
versions, Go OS/architecture, and the explicit unavailable libsodium version.
The canonical JSON and its SHA-256 are evidence records, not signed provenance
or authority. The emitter does not make the run hermetic and cannot turn an
unknown native library identity into a known one.

## 5. Required semantic and front-door evidence

Each lane must be reported with its property, boundary, and result. Passing
multiple tests of the same invariant is not multiple independent arguments.

| Lane | Property and claim supported | Front door | Acceptable evidence |
| --- | --- | --- | --- |
| A-018 diagnostic gate | The reviewed forbidden authority boundaries fail for the intended diagnostic classes and source-bound inventory. | External-consumer snippets against the governed public modules. | All governed witnesses fail in the expected classes/spans; source or inventory drift fails closed. |
| Rust format, lint, tests, doctests | Current Rust implementation remains internally buildable and its committed hostile tests pass. | Workspace commands and `magpie-log` doctests. | Exact selected commands pass on the recorded candidate; no ignored campaign is silently counted. |
| Governed standing tour | The advertised deterministic tour verifies, resolves selected policies, drops derived state, and regenerates it identically. | `cargo run --locked --example tour -p magpie-claims`. | Assertions pass; stdout alone is not the contract. |
| Metadata/package inventory | Declared package versions, path dependencies, licenses, and inventories match the repository boundary. | `tools/check_release_metadata.py` and `cargo package -p magpie-log --locked`. | Both checks pass for the candidate; this is not publication evidence. |
| Corpus integrity | Frozen identity, sidecar, case bytes, source hashes, and coverage inventory remain exact. | `tools/check_verifier_corpus_manifest.py`. | Exact frozen manifest and all input identities pass. |
| Python conformer | Independent Python implementation computes the selected portable semantics for every frozen case. | `tools/verify_chain.py` through the selected-root checks. | All 432 governed fields match the frozen comparison material, with operational failure distinguished from REJECT. |
| Go conformer | Independent vendored implementation computes the same selected semantics and process status. | Built executable, not `go run`. | Vendored test/vet/build pass; direct executable reports `0` ACCEPT, `1` governed REJECT, `2` operational failure; corpus check passes. |
| Differential | Three implementations agree on `verdict`, `class`, `line`, `record_index`, `event_count`, `tip`, and `ordered_recomputed_hashes`. | The selected-root differential runner. | Exact 432-case agreement and repeat-two semantic stability; expected corpus output never constructs an actual result. |
| Golden histories | Frozen golden and portable Deadbolt histories verify under explicit external keys. | `tools/verify_chain.py` with pinned fixture keys. | Required histories pass; this does not verify foreign Deadbolt contents. |
| Public/file path | The advertised Rust front door reaches the authoritative complete semantics without legacy record splitting. | `FileStore::verify_portable_history`. | Exact-byte acquisition, preflight ordering, and complete result behavior are exercised by committed tests. |
| Environment provenance | The validation record identifies the actual candidate, checkout, environment, and governed inputs. | `tools/emit_validation_environment.py`. | Required GitHub coordinates exist; canonical JSON and digest are retained with the run. |

## 6. Resource, crash, and recovery evidence

Evidence must be mapped to the selected capability, not claimed generically.
For supported SQLite L0, the required evidence covers finite database, record,
count, and cumulative-byte limits; stable snapshots; stale/concurrent writers;
transaction failures; unknown commit-state poisoning; hot-journal recovery;
wrong-key, schema, ownership, corruption, and checkpoint handling. The tests
must show no writer/result publication after the relevant failure and no blind
retry after an uncertain commit.

The profile also requires an explicit disposition for these adjacent classes:

- compatibility `FileStore` whole-file materialization, framing, partial-write,
  crash, and concurrent-writer behavior;
- episodic projection transaction, schema-mismatch, range, panic/abort, and
  resource behavior;
- verifier pathological input, oversized metadata, malformed framing, and
  exception/panic containment;
- projection/resource behavior only where a projection is part of the claim.

Existing SQLite hostile tests are current evidence for the supported L0 subset,
not a universal storage guarantee. Existing compatibility and projection gaps
must remain visible in the record. Network/distributed filesystems and arbitrary
resource exhaustion are not required unless the public claim is widened.

## 7. Dependency and supply-chain assurance

The minimum review boundary is the dependencies that execute or materially
control the selected lanes: locked Rust dependencies, the pinned Python
requirements and their installed distributions, Go's authorized vendored
`filippo.io/edwards25519 v1.2.0`, bundled/native SQLite as used by the supported
L0 path, and the exact GitHub Actions revisions.

The evidence must distinguish:

- identity pinning from source availability;
- vendored source/license retention from semantic correctness;
- offline execution from secure acquisition;
- package/version records from downloaded artifact hashes;
- dependency semantic review from vulnerability/security review;
- native backend identity from the absence of a reported identity.

The current Go vendor tree establishes reviewable source identity, retained
license material, and offline availability for its authorized module. It does
not prove that module secure or correct. The current Python environment records
distribution versions, while wheel/sdist bytes are not artifact-hash pinned and
the PyNaCl libsodium version is explicitly unavailable. C4 evidence must not
claim security review, complete provenance, or reproducible acquisition without
new evidence.

## 8. Reproducibility vocabulary

Later records must use these levels distinctly:

| Term | Meaning here |
| --- | --- |
| Deterministic semantic replay | Same exact supplied history and policy produce the same semantic result. |
| Repeat-two validation stability | One execution repeated twice produces equal reported semantic results. |
| Pinned validation inputs | Workflow, lockfiles, corpus, vendor tree, and selected versions are identity-bound. |
| Measured hosted environment | The actual runner/image/toolchain coordinates were emitted and retained. |
| Clean-cache/offline dependency execution | A specified lane, notably vendored Go, ran without module retrieval under stated constraints. |
| Source/package reproducibility | The selected source/package inventory can be recreated or checked under a stated process. |
| Hermetic build | All inputs and execution effects are closed against an explicitly defined environment boundary. Not established by this profile's current evidence. |
| Bit-for-bit artifact reproducibility | Independently produced artifacts have identical bytes. Not established by this profile. |

Repeat-two differential evidence is not a reproducible build. A pinned input is
not an observed environment, and an observed environment is not hermetic.

## 9. Assurance record

A future record must carry, at minimum:

- profile identity and exact candidate SHA;
- actual checked-out SHA, workflow ref/SHA, run ID, and attempt;
- selected runner label plus actual image OS/version, OS family, architecture,
  runner environment, and relevant host triples;
- observed Rust, Python, Go, package, and native-coordinate outputs;
- hashes/identities for all governed inputs in section 4;
- each required lane, its command or evidence reference, result, and whether it
  exercised a public/front-door or internal path;
- artifact/package identities and known unavailable identities;
- checker identity/evidence reference and unresolved findings;
- an explicit claim fence stating what the record does and does not support.

The record is evidence about an execution. It is not self-authenticating,
signed provenance, owner approval, a receipt, or authority.

## 10. Maker, checker, and owner

The candidate maker may assemble the record but may not be the sole checker. A
checker must inspect the exact candidate and evidence independently, attempt
falsification, and test mutation questions such as whether changing one bound
identity could leave the record claiming the same candidate. A checker PASS is
review evidence only. It is not owner approval, finding closure, or release
authority. The owner separately decides acceptance, disposition, C5 entry, and
release.

## 11. Claim fence

C4 evidence must not be used to infer:

- green CI equals release assurance;
- corpus agreement equals universal verifier correctness;
- dependency pinning equals security review;
- a provenance record equals signed provenance;
- a selected runner label equals an immutable image;
- repeatability equals hermetic reproducibility;
- local SQLite crash evidence equals distributed-storage safety;
- successful supplied-history verification equals key trust or ownership;
- supplied history equals latest, canonical, complete, or non-equivocating history;
- checker PASS equals owner approval;
- C4 completion equals release, correctness, security, production readiness, or
  finding closure.

## 12. C4 completion criterion

The only permitted completion statement is:

> `magpie-public-prealpha-assurance-v1: SATISFIED FOR C5 CANDIDATE SELECTION`

It may be stated only when every required evidence lane and coordinate in this
profile has been produced for one exact candidate, all known gaps are either
outside the selected boundary or explicitly resolved, and an independent
checker has reviewed the record sufficiently to permit entering C5 candidate
selection. This statement does not mean released, approved, correct, secure,
production-ready, or finding-closed.

## 13. Gap matrix at selected base

| Requirement | Current evidence on main | Evidence strength | Remaining gap | Proposed later tranche | Blocks C5? |
| --- | --- | --- | --- | --- | --- |
| Selected public portable boundary | README, convergence reassessment, #146-era implementation and tests; C3 receipt exclusion is explicit in `CLAUDE.md` and current docs. | Strong, bounded doctrine plus implementation | No C4 assurance record for one final candidate. | C4.5 profile-conformance record gate | Yes |
| Rust public/file front door | `FileStore::verify_portable_history`; exact-byte and hostile tests; current audit disposition narrows closure to selected profile. | Strong for selected path | Candidate-specific hosted evidence and final record still required. | C4.5 | Yes |
| Frozen corpus and three-way semantics | 432-case manifest, independent Python/Go paths, seven-field differential, repeat-two, corpus checker. | Strong finite conformance evidence | Not universal correctness; must rerun for claimed candidate and retain coordinates. | C4.5 | Yes |
| A-018 exact diagnostic boundary | Current main includes source-bound persistent checker and 43-witness evidence in the audit record. | Strong bounded evidence | Must be included in the future candidate record; no broad parser equivalence. | C4.5 | Yes |
| Supported SQLite L0 resources/recovery | `sqlite_l0.rs` tests cover finite limits, stable snapshots, stale writers, transaction/unknown commit, recovery, schema and checkpoint cases. | Strong for supported local L0 | No equivalent assurance for compatibility FileStore or all projections. | C4.2 | Yes for any claim including those surfaces |
| Compatibility FileStore/resource/crash boundary | README and audit disposition explicitly demote FileStore; whole-file reads and unsynchronized two-write append remain. | Negative/boundary evidence | Do not advertise stronger guarantees; add tests only if boundary widens. | C4.2, only if widened | No for selected exclusion |
| Projection failure/resource behavior | Audit records `expect`/panic and schema-mismatch concerns in episodic paths. | Confirmed gap evidence | No public projection assurance claim is justified. | C4.2, only if widened | No for selected exclusion |
| Hosted platform | Workflow selects `ubuntu-24.04`; emitter records actual image and architecture; historical hosted Linux runs exist. | Strong single-lane evidence | No second platform; image is not immutable. | C4.2 | Yes for broader platform claim, no for selected narrow lane |
| Selected vs observed environment | `magpie-validation-environment-v1` emitter and tests bind both. | Strong mechanism evidence | A retained record for the exact future candidate is missing. | C4.5 | Yes |
| Rust/Python/Go selected inputs | Workflow selects Rust 1.98.1, Python 3.12.14, Go 1.27.0; lockfiles/vendor/requirements are hashed. | Strong selected-input evidence | Toolchain/artifact acquisition is not fully reproducible. | C4.4 | Yes for stronger reproducibility claims |
| Native/libsodium identity | Emitter records unavailable native version; audit explicitly refuses inference. | Honest gap evidence | Native identity and acquisition/reproduction policy remain open. | C4.3 | Yes for native-dependent stronger claim |
| Dependency semantic/security review | Go authorization/vendor/license boundary is documented; Rust/Python lock/version checks exist. | Partial | No complete semantic or vulnerability review; Python artifacts lack hash-required acquisition. | C4.3 | Yes for security/supply-chain claim |
| Package/artifact evidence | Release metadata and `magpie-log` package checks exist. | Strong source/inventory evidence | No release artifact set or bit-for-bit artifact reproduction; C5 contract is separate. | C4.4/C4.5 | Yes for artifact claim |
| Reproducibility | Deterministic replay, repeat-two, pinned inputs, measured environment, and Go offline lane exist. | Strong at distinct lower levels | No hermetic or bit-for-bit build evidence. | C4.4 | Yes for stronger claim |
| Independent/adversarial review | Historical hostile reviews and current maker/checker doctrine exist; current profile requires a fresh checker. | Partial | Exact profile diff and final evidence need independent checker verdict. | C4.5 | Yes |
| Owner authority | Repository doctrine reserves merge, disposition, candidate, and release decisions to owner. | Governing authority boundary | No owner acceptance of future evidence yet, by design. | C5 governance | Yes |

No row changes an audit finding status. The matrix reports current evidence and
gaps only; the living disposition remains untouched by this tranche.

## 14. Bounded follow-ups

The smallest sensible implementation/evidence decomposition is:

1. **C4.2 platform/resource/crash gap closure:** only the selected boundary's
   remaining negative evidence, and only broaden platform/storage/projection
   claims after an owner decision.
2. **C4.3 dependency/native-input assurance:** review the bounded dependency
   set, native identity, licenses, acquisition, and security limitations.
3. **C4.4 clean-environment/reproducibility evidence:** strengthen clean-cache,
   acquisition, package, and reproducibility claims without calling them
   hermetic unless demonstrated.
4. **C4.5 profile conformance and assurance-record gate:** produce the exact
   candidate record, run all required lanes, obtain independent adversarial
   review, and mechanically determine whether the profile permits C5 selection.

These are proposed tranches, not authorization to implement them.
