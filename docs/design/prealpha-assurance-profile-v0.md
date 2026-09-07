# Public pre-alpha assurance profile v0

## Status and authority

Status: proposed C4-A qualification-profile contract.

Profile identity: `magpie-public-prealpha-assurance-profile-v0`.

This document is reconciled against `main`
`313287c8ac9a0b266ab3a477b143d79072b88650`. It defines a candidate
qualification contract for owner acceptance. It is not qualification evidence,
an implementation, an ADR, a release contract, an audit-disposition change, or
a release decision. CI success, review approval, and merge do not substitute
for an explicit owner decision to accept the profile.

Authority remains, in order:

1. [FORMAT](../FORMAT.md) and Accepted ADRs, especially
   [ADR-0008](../adr/0008-complete-producing-coordinates.md),
   [ADR-0009](../adr/0009-verified-supplied-history-and-explicit-checkpoint-expectations.md),
   and [ADR-0010](../adr/0010-ed25519-verification-profile-and-external-trust-root-admissibility.md);
2. the frozen portable-verification contracts and
   [corpus](portable-verifier-corpus-v1.md);
3. the supported [SQLite L0 contract](l0-persistence-and-checkpoint-open-v0.md);
4. current implementation and tests; and
5. this profile and living administrative documentation.

A conflict with a higher layer stops qualification. After owner acceptance, a
normative change to this profile requires a new profile identity; this identity
may not be repointed.

## Purpose and selected public boundary

This profile answers one question: under what smallest explicit environment
can Magpie's currently advertised public pre-alpha portable verification and
supported local L0 boundary be tested and described honestly?

The selected boundary is:

- the exact supplied-history portable JSONL verification path under
  `magpie-ed25519-canonical-prime-subgroup-v1`, including the public Rust file
  path and independent Python and Go conformers;
- deterministic replay and the ordinary workspace behavior exercised by the
  current test graph and governed-standing tour;
- `SqliteL0Store` as the supported bounded local persistent L0 path; and
- the current package/source metadata posture.

The profile assumes and preserves the current C3 exclusion. It does not add or
qualify a detachable coordinate-complete governed verification/replay result,
receipt, standing result, search row, or query response. `FileStore` remains a
compatibility, development, inspection, and portable-input path, not the
supported durability or concurrent-writer boundary.

"Machine-agnostic" architecture and cross-language portable semantics do not
mean that every machine or operating system is qualified. For this profile,
"supported local L0" means tested only in the selected environment below.

## Qualification claim

> For one exact Magpie source revision and one exact v0 governed-input set, a
> clean qualification execution can establish repeatable qualification
> evidence for the selected public pre-alpha portable-verification and
> supported-local-L0 boundary on the GitHub-hosted Ubuntu 24.04 x64 environment
> family, using the exact selected toolchains and controlled dependency inputs,
> only when the record identifies the actual checkout, workflow, runner image,
> toolchains, governed inputs, and every required assurance result. The pass
> applies only to those recorded coordinates; it is not qualification of other
> platforms and grants no release authority, which remains a C5 owner decision.

## Selected environment and toolchains

The sole v0 hosted qualification environment is:

| Coordinate | Required value or treatment |
| --- | --- |
| Runner provider | GitHub-hosted standard runner |
| Selected family | `ubuntu-24.04` |
| OS and architecture | Linux, x86-64 (`X64` / Go `amd64`) |
| Actual hosted image | `ImageOS` and `ImageVersion` must be non-empty and recorded for every run |
| Rust | `rustc 1.98.1` and Cargo `1.98.1`; verbose executed identity recorded |
| Python | CPython `3.12.14`; executed interpreter identity recorded |
| Python installer | pip `26.2.1`; selected and executed identity recorded |
| Go | Go `1.27.0`, `linux/amd64`; executed identity recorded |
| Actions | every third-party action selected by full commit SHA |

The runner label selects a maintained image family, not an immutable VM image.
GitHub documents that runner images are normally updated weekly. Standard
hosted runners do not give this profile an immutable image digest to pin, so a
run qualifies on the actual recorded `ImageOS` and `ImageVersion` within the
selected family. A different image version is a different execution coordinate,
not retroactive invalidation of a complete historical record.

Toolchain versions are selected execution inputs. They are not claims that
compiler binaries, operating-system libraries, or produced binaries are
bit-identical across time.

## Controlled acquisition

Dependency-network access is permitted only in explicit toolchain provisioning
and dependency-acquisition steps. Assurance execution begins only after
controlled acquisition succeeds and must run with Cargo, pip, and Go dependency
retrieval and resolution disabled. Infrastructure needed to upload the final
evidence record is outside that dependency boundary. An acquisition or
operational failure produces no qualification pass.

### Python

C4-B must add one qualification-only acquisition manifest satisfying all of
these requirements:

1. It lists `cryptography==50.0.0`, `PyNaCl==1.6.2`, `cffi==2.0.0`, and
   `pycparser==3.0` explicitly; no transitive dependency may be implicit.
2. Every entry uses exact `==` selection and exactly one locally reviewed
   SHA-256 hash for the wheel selected for CPython 3.12 on Linux x86-64.
3. Only binary wheels are allowed. Source distributions, VCS requirements,
   local directories, editable installs, and unhashed alternatives are
   rejected.
4. Acquisition uses pip hash-checking mode, binary-only selection, and
   `--no-deps` into a fresh wheelhouse. Installation uses only that wheelhouse,
   with no index, no dependency resolution, and no system site packages.
5. The record contains the manifest digest and the exact filename, byte length,
   and SHA-256 of every acquired and installed wheel.
6. A change to any allowed distribution byte requires review, a changed
   acquisition-manifest identity, and a new qualification execution.

Wheel-only is claim-specific here: the selected environment has compatible
wheels, while accepting source distributions would introduce compiler,
header, build-backend, and system-library inputs not governed by this profile.
Multiple-platform wheel hashes are not needed because v0 qualifies only one
platform.

PyNaCl's supported wheels include the native libsodium dependency. Therefore,
for v0, the exact selected PyNaCl wheel digest is the identity of the packaged
PyNaCl/libsodium native artifact; a separately exposed libsodium version string
is not required. `SODIUM_INSTALL=system`, source builds, and another PyNaCl
wheel are outside this profile. C4-B must verify that the selected wheel carries
the bundled native extension and does not dynamically select a system
libsodium. The wheel still depends on the selected host runtime, including
libc; that environment is recorded through the runner image coordinates and is
not claimed to be independently byte-pinned.

### Rust

C4-B must implement these properties without claiming a hermetic build:

1. The root and member `Cargo.toml` files and committed `Cargo.lock` are
   governed inputs. The v0 source set is limited to current workspace paths and
   crates.io registry packages carrying lockfile checksums; a new git,
   alternate-registry, or unchecksummed source requires profile review.
2. A fresh `CARGO_HOME` is used. `cargo fetch --locked` is the explicit
   network-permitted acquisition phase; starting a cold acquisition with
   `--offline` is not equivalent.
3. Cargo must exact-match the lockfile and verify registry archive checksums.
   The qualification record lists every consumed registry package's name,
   version, source, lockfile checksum, archive byte length, and observed
   SHA-256.
4. After fetch, every Cargo assurance command uses the same acquired set with
   `--locked` and offline mode. The build target directory is fresh and no
   dependency or build cache is restored.

This relies on `Cargo.lock` for exact resolution and registry archive checksum
identity. It does not claim that the lockfile identifies the Rust toolchain,
host libraries, build-script environment, or final binary bytes, nor that the
resolved dependencies are safe.

### Go

The current Go mechanism already satisfies the dependency-acquisition boundary
and needs no redesign:

1. `go.mod`, `go.sum`, `vendor/modules.txt`, and a content-sensitive identity
   of the complete vendor tree are governed inputs.
2. Qualification uses `-mod=vendor`, Go `1.27.0`, fresh module/build caches,
   and the existing `GOENV=off`, empty override variables, `GOPROXY=off`,
   `GOSUMDB=off`, `GOTOOLCHAIN=local`, `GOVCS=*:off`, and `GOWORK=off`
   restrictions.
3. No module or VCS retrieval is allowed during qualification. A vendor/
   `go.mod` mismatch or any attempted retrieval fails the run.

`go.sum` records upstream module content identities in the module graph; the
committed vendor tree is the source actually compiled. Their agreement plus
the recorded tree digest establishes exact source selection, not dependency
review or correctness.

## Governed inputs

The canonical qualification record must content-identify at least:

- the exact source candidate and actual checkout;
- the qualification workflow and every full-SHA action selection;
- the root and member Cargo manifests, `Cargo.lock`, and consumed registry
  archives;
- the Python qualification acquisition manifest and selected wheel files;
- `go.mod`, `go.sum`, `vendor/modules.txt`, and the complete Go vendor tree;
- the frozen portable corpus manifest and its already-bound case bytes;
- `tools/a018_diagnostics_inventory.json`, its checker, and the three governed
  source modules named by that inventory;
- the portable Rust, Python, Go, and differential tooling through the exact
  source revision;
- `tools/check_release_metadata.py`, the root/member package manifests,
  `README.md`, `CHANGELOG.md`, and the root/member Apache license files; and
- this qualification-profile identity and the exact implementation of every
  required qualification step.

The source revision transitively identifies repository files. The explicit
digests above make the inputs that determine qualification visible and
independently comparable; they do not turn those inputs into authority.

## Required assurance families

A v0 qualification pass requires every family below. A filtered, skipped,
cancelled, tolerated, or unavailable required step is not a pass.

| Family | Minimum v0 evidence |
| --- | --- |
| A-018 negative boundary | The persistent source-bound 43-witness exact-diagnostic checker under Rust 1.98.1, including inventory/source identity checks |
| Frozen portable corpus | Manifest/sidecar/case-byte integrity and the exact 432-case, 19-ACCEPT/413-REJECT inventory |
| Independent conformers | Python complete-corpus check; vendored Go format, test, vet, direct exit-code, build, and manifest checks |
| Three-way differential | Rust/Python/Go exact agreement on all seven governed fields for 432 cases, repeated twice with stable results |
| Rust bounded assurance | The ordinary portable smoke tests and the opt-in 12,000-input non-normative assurance campaign |
| Supported SQLite L0 | The complete workspace graph, including SQLite resource limits, schema/ownership refusal, stable snapshot, tamper, checkpoint, stale-writer, busy/transaction, unknown-commit, and recovery tests |
| Workspace behavior | Rust formatting, warnings-denied Clippy, workspace tests, `magpie-log` doctests, and the governed-standing tour |
| Package/source posture | Release-metadata and deterministic package-inventory checks plus standalone `magpie-log` package assembly/verification |
| Qualification provenance | A complete canonical record satisfying the next section and content-identifying all results above |

Corpus agreement is bounded conformance evidence. The extended campaign is
bounded non-normative assurance. Neither is a proof of universal semantic or
cryptographic correctness.

Before final C4 handoff, C4-D must also attach a candidate-scoped dependency
review that enumerates the exact Rust, Python, Go, and native components;
compares them with the previously reviewed state; checks current relevant
advisories; and classifies relevance to this selected boundary. That review is
manual/reviewed evidence, not a scanner-as-oracle gate. Exact acquisition and
dependency security remain separate propositions.

## Qualification record

`magpie-validation-environment-v1` is useful existing provenance, but it is not
the v0 qualification record because it does not bind controlled acquisition,
cache state, this profile, or the complete assurance result set.

C4-B/C must produce a canonical JSON record and its SHA-256. The schema must be
versioned and fail closed. It must include:

- profile identity and record-schema identity;
- repository, exact candidate head, GitHub event SHA, actual checked-out HEAD,
  clean-worktree assertion, and ref/event coordinates without collapsing them;
- qualification workflow digest, workflow SHA/ref, job, run ID, and attempt;
- selected runner label plus actual `ImageOS`, `ImageVersion`, runner OS,
  architecture, environment, and name;
- selected and executed Rust/Cargo, CPython/pip, and Go identities;
- every governed-input and acquired-artifact identity required above;
- fresh cache/build-directory identities and the acquisition-versus-offline-
  execution boundary used;
- one closed inventory of required assurance families, with exact command or
  step identity, outcome, and stable result summary/digest for each.

C4-D's dependency review occurs after inspecting the exact run. Its final
evidence index must bind the immutable qualification-record digest and the
separate review digest; it must not rewrite the earlier canonical run record.

The record is content-addressed evidence of what was selected and observed. It
is not an ADR-0008 governed result, portable verification receipt, trust root,
artifact attestation, signature, release artifact, or authority credential.

## Clean execution and caches

Routine CI may continue to restore Cargo dependency and build caches. v0
qualification deliberately restores no dependency, wheel, module, or
build-output cache. It uses fresh acquisition and build directories, may
populate them during the same run, and then consumes only the controlled
acquisition result with network resolution disabled. Provider tool caches used
while provisioning the selected language toolchains remain part of the
recorded runner/provisioning boundary; their bytes are not called clean or
independently pinned by this profile.

A restored cache whose contents were independently identity-verified could be
designed later, but it is not part of v0. "Clean" means no restored caches and
fresh build outputs; it does not mean air-gapped, hermetic, or free of the
recorded runner image and toolchain provisioning.

## Failure, invalidation, and change control

Qualification fails closed when a required coordinate is absent or blank, a
selected version differs, acquisition or an assurance family fails, a required
step is skipped, the checkout is dirty or differs from the candidate, a
governed digest differs, dependency/network restrictions are violated, or the
canonical record cannot be completed.

Evidence for candidate A does not qualify candidate B. Any source revision,
workflow, action revision, toolchain, acquisition manifest, governed input,
runner family, required assurance inventory, qualification-step semantics, or
profile version change requires a new run at the new coordinates. A new actual
runner `ImageVersion` similarly requires a new execution record, while leaving
the earlier record historically valid for its own coordinates.

A change to the selected public boundary, acquisition law, required assurance
families, record semantics, or non-claims requires owner review and a distinct
profile identity. Expanding into C3 detached outputs, another platform, or C5
release mechanics cannot be done as an in-place v0 edit.

## Evidence matrix

| Concern | Current evidence | Remaining gap | In v0? | Required later mechanism | Explicit non-claim |
| --- | --- | --- | --- | --- | --- |
| Source/candidate identity | Emitter separates candidate, event, and checkout SHAs | Not bound to full qualification results | Yes | Canonical v0 record | A PR merge SHA is not the candidate head |
| Workflow identity | Workflow SHA/ref and file digest recorded | No dedicated qualification workflow/result binding | Yes | C4-C workflow and record | Workflow provenance is not release authority |
| Hosted runner identity | `ubuntu-24.04`, `ImageOS`, `ImageVersion`, runner fields | Family moves over time | Yes | Record actual image each run | No immutable runner |
| Action identity | Current third-party actions use full SHAs | Must remain in qualification workflow | Yes | Preserve and record selections | Pinning is not action correctness |
| Rust toolchain | Rust/Cargo 1.98.1 selected and observed | No clean acquisition lane | Yes | Fresh locked fetch, offline execution | No deterministic binary claim |
| Python toolchain | CPython 3.12.14 and pip observed | pip not currently asserted as qualification input | Yes | Select/assert CPython 3.12.14 and pip 26.2.1 | Version labels are not interpreter byte identity |
| Go toolchain | Go 1.27.0 `linux/amd64` selected and observed | Must be carried into qualification | Yes | Preserve/assert/record | No other Go platform qualified |
| Cargo acquisition | Lockfile versions, sources, checksums; broad `--locked` use | Routine CI restores cache; no explicit cold fetch/offline split | Yes | C4-B fresh `cargo fetch --locked`, archive identity, offline run | Lockfile is not hermeticity or safety |
| Python acquisition | Exact versions only | No local hashes, complete artifact allowlist, or binary-only lane | Yes | C4-B hash-bound wheel manifest/wheelhouse | Exact versions are not exact bytes |
| Go acquisition | Vendored v1.2.0 dependency, tree digest, retrieval disabled | No redesign required | Yes | Preserve current boundary in C4-C | Vendoring is not correctness |
| PyNaCl/libsodium/native | Distribution version recorded; missing public runtime version explicit | Distribution bytes not bound; host libc remains environmental | Yes | Bind exact PyNaCl wheel; record image | Wheel identity is not all host native code |
| Frozen corpus | Frozen manifest digest and all case hashes checked | None for selected corpus; must be retained | Yes | Run integrity gate | Corpus is not a verifier |
| Three-way differential | 432 cases, seven fields, repeat two | Routine-CI execution is not qualification | Yes | Run in clean lane and bind result | No universal equivalence |
| A-018 | Owner-merged source-bound 43-witness exact-diagnostic gate | Must be retained under controlled Rust inputs | Yes | Run in clean lane | Closure is not complete API safety |
| SQLite supported L0 | Extensive resource, hostile, transaction, and recovery tests | Qualified only on one selected environment | Yes | Execute/map named families | No FileStore, network, or distributed durability claim |
| Package/release metadata | Metadata, inventory, and `magpie-log` package checks | Results absent from canonical qualification record | Yes | Bind inputs and outcomes | Package assembly is not release/publication |
| Platform scope | One Ubuntu hosted lane; README disclaims release-environment portability | Scope must be explicit | Yes | Linux/x64 family only | No Windows, macOS, or universal Linux qualification |
| Caches | Routine Cargo cache is toolchain/lock-keyed | Restored state defeats the simplest clean claim | Yes | No restored caches in C4-C | Clean is not air-gapped |
| Environment record | Canonical environment JSON and digest | Missing acquisition, profile, cache, and result semantics | Yes | Extend or supersede in C4-B | Provenance is not hermeticity |
| Dependency/security review | Prior bounded reviews and exact selections | No current candidate-wide semantic/advisory review | Yes, separate evidence | C4-D reviewed dependency delta/advisory classification | Zero alerts would not prove safety |
| Attestations | None required | Release artifact does not yet exist | No | Consider and verify for frozen C5 artifact only | Attestation is not security proof |
| C5 release boundary | Existing v0.1 contract is historical; no current pre-alpha contract/candidate | No frozen candidate, whole-candidate falsification, or owner release decision | No | C5 contract, freeze, hostile review, owner decision | Qualification pass is not release approval |

## Explicit non-claims

This profile establishes none of the following:

- Windows, macOS, non-x64, self-hosted, or universal Linux qualification;
- an immutable hosted runner, byte-pinned operating system, perpetual rerun
  ability, hermetic build, air-gapped build, reproducible build, or
  bit-identical artifact;
- universal Rust/Python/Go equivalence beyond the frozen governed corpus and
  selected profile, or cryptographic/software correctness proof;
- safe, vulnerability-free, trusted, or secure dependencies merely because
  their acquisition bytes are exact;
- complete public API safety merely because A-018's bounded defect is closed;
- `FileStore` durability, production SQLite durability, network/distributed
  storage, global completeness, non-equivocation, or universal anti-rollback;
- trust in or ownership of a caller-supplied external key, identity,
  delegation, authority, permission, admission, authority-bound ADR-0007
  corroboration, or production KMS;
- freshness, latest history, canonical branch, or absence of an unseen suffix
  or fork;
- a detachable coordinate-complete governed output, result, receipt, standing,
  search, librarian, or query surface under C3/ADR-0008;
- authority in the qualification record or security from an artifact
  attestation; or
- candidate freeze, release-contract satisfaction, release approval,
  publication, merge authority, or any other C5 owner decision.

## Routine CI, qualification, and C5

```text
routine CI
    = development regression evidence; restored caches are allowed

qualification run
    = clean execution of this complete profile at exact recorded
      source, input, workflow, toolchain, and actual environment coordinates

release decision
    = C5 owner authority over a separately frozen candidate and release contract
```

A routine CI pass is not a qualification pass. A qualification pass is not a
release, candidate freeze, finding closure, or owner decision. C4 produces
content-addressed qualification evidence; C5 may later freeze a release
artifact and, if the C5 contract needs it, generate and independently verify an
artifact attestation for that artifact. Frequent test outputs and this document
are not attestation subjects, and this profile makes no SLSA-level claim.

## Finite C4-A contract exit conditions

The profile contract itself is complete when the owner can accept or reject
all of these finite propositions without selecting an implementation policy:

1. the advertised boundary and C3 detached-output exclusion are identified;
2. the sole selected qualification environment and actual-image recording law
   are explicit;
3. platform qualification and platform non-scope are explicit;
4. exact toolchain and action-selection policies are explicit;
5. Python, Rust, and Go acquisition properties are testable and complete;
6. governed source, dependency, corpus, A-018, workflow, and package-input
   identities are enumerated;
7. every required assurance family is enumerated;
8. the canonical qualification-record semantics are defined;
9. clean/cache/network semantics are defined;
10. routine CI, qualification, and C5 release authority are separated;
11. non-claims prevent platform, reproducibility, security, authority, C3, and
    release inflation;
12. failure, invalidation, and profile change-control rules are finite; and
13. C4-B, C4-C, and C4-D can proceed without another architecture choice.

These conditions complete only C4-A. They do not claim that the profile has
been implemented or executed.

## Follow-up implementation obligations

### C4-B — controlled acquisition and record semantics

- add the exact hash-bound, wheel-only Python qualification manifest and its
  validation tests;
- implement fresh Rust locked acquisition followed by offline execution while
  preserving Cargo's exact lockfile/checksum role;
- preserve the current Go vendor/retrieval boundary without redesign;
- extend or supersede `magpie-validation-environment-v1` with the canonical v0
  qualification schema, governed-input/acquired-artifact inventory, cache/
  network assertions, and closed assurance-result inventory; and
- do not change runtime, portable semantics, the frozen corpus, A-018 inventory,
  or release metadata except where a separately reviewed input identity is
  mechanically required.

### C4-C — clean qualification lane

- add one separate explicit qualification workflow for an exact candidate;
- use no restored dependency or build caches;
- perform controlled acquisition, then disable dependency/network resolution;
- execute every required assurance family, including the extended Rust campaign
  and named supported-SQLite families; and
- publish the canonical qualification record and digest as evidence, not as a
  release artifact or authority object.

### C4-D — final C4 hostile review and reconciliation

- review the exact candidate dependency delta, relevant current advisories, and
  native components against the selected public boundary;
- hostile-review one exact clean qualification record and its claim fences;
- assess the surviving A-011/RQ-013/RQ-015 scope without changing disposition
  merely because evidence exists; and
- produce a finite C5 handoff. Do not freeze, sign, attest, or release a
  candidate in C4-D.

## References

- [Current audit disposition](../audits/audit-disposition-2026-08.md)
- [Current pre-alpha convergence reassessment](../audits/public-pre-alpha-convergence-reassessment-2026-08-22.md)
- [GitHub-hosted runner images](https://github.com/actions/runner-images)
- [GitHub-hosted runner reference](https://docs.github.com/en/actions/reference/runners/github-hosted-runners)
- [pip secure installs](https://pip.pypa.io/en/stable/topics/secure-installs/)
- [pip repeatable installs](https://pip.pypa.io/en/stable/topics/repeatable-installs/)
- [PyNaCl installation](https://pynacl.readthedocs.io/en/latest/install/)
- [Cargo fetch](https://doc.rust-lang.org/cargo/commands/cargo-fetch.html)
- [Go modules reference](https://go.dev/ref/mod)
- [GitHub artifact attestations](https://docs.github.com/en/actions/concepts/security/artifact-attestations)
