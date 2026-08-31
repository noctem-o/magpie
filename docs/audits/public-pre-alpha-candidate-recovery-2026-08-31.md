# Public pre-alpha candidate recovery assessment — 2026-08-31

> This is a dated assessment of the local candidate checkout, not a rewrite of
> historical audits, a release contract, an owner release decision, or a claim
> about unmerged `main`. It supplements the 2026-08-22 convergence
> reassessment with evidence produced after PRs #139–#141.
>
> The recovery, original candidate, original executed-evidence, and original
> local-review sections below preserve the chronology of the local candidate
> before the first Codex review of PR #143. The post-review addendum records
> the later findings, remediation, re-review, and final-stack validation. A
> later result does not make an earlier claim retroactively true.

## Recovery boundary

The unusable predecessor task was treated only as optional reference material.
No execution state was resumed, replayed, repaired, forked, or trusted. The
repository, accepted decisions, frozen artifacts, implementation, tests, Git
history, and current worktree were reconstructed directly.

| Coordinate | Recovered value |
| --- | --- |
| Authoritative starting branch | `main` |
| Starting `HEAD`, local `origin/main`, and remote `main` | `8ae50f2b093ce33de4dee3cb9421e3f5ee5eb0f4` |
| Candidate branch | `codex/sol-max-prealpha-20260830` |
| Candidate worktree | `C:\Users\herpe\.codex\worktrees\magpie-sol-max-prealpha-20260830` |
| Frozen corpus identity | `magpie-portable-verifier-corpus-v1` |
| Frozen manifest SHA-256 | `7d758d3f2dac1161fe15dd064b130ccbfcdaf0437ea8ab8d7772492194801a81` |
| Corpus result inventory | 432 cases: 19 `ACCEPT`, 413 `REJECT` |

At the starting revision, the complete crate-internal Rust conformer (PR #139),
bounded non-normative Rust assurance (PR #140), and README refresh (PR #141)
were already merged. They were not redone. The highest-value remaining C2 gap
was independent complete Python and Go implementation plus real three-way
differential execution.

## Original candidate change (before PR #143 review)

The candidate adds three deliberately separate evidence paths:

1. `tools/portable_verifier.py` is a readable complete Python conformer. It is
   separate from the compatibility-only `tools/verify_chain.py`, consumes raw
   bytes, preserves the frozen first-failure order, and implements the selected
   ADR-0010 relation using pinned low-level PyNaCl operations plus explicit
   Magpie canonicality, subgroup, scalar, identity, and equation gates.
2. `tools/go-verify-chain` is a standalone Go module. It shares no Rust or
   Python verifier code. At this assessment point it pinned
   `filippo.io/edwards25519` v1.2.0 for focused point/scalar arithmetic while
   retaining Magpie-owned parsing, canonicality, subgroup, challenge, and
   uncofactored-equation checks, but it had not yet recorded the later owner
   authorization or vendored the module for offline use.
3. `tools/check_portable_verifier_differential.py` was intended to preflight the pinned
   manifest, sidecar, and every input hash before invoking any conformer. It
   obtains actual results independently from Python, Go, and the real Rust
   conformer, then requires exact case order/identity, strict JSON value types,
   all seven governed fields, and the complete class inventory. Expected
   manifest results are comparisons only; no conformer receives them while
   constructing an actual result. The later PR #143 review found that the
   non-default `root` path still imported Python from the ambient checkout;
   that defect is recorded and remediated in the addendum below.

The Rust bridge is test-only. It writes an actual-results envelope only when
`MAGPIE_PORTABLE_DIFFERENTIAL_OUTPUT` is explicitly set and otherwise performs
no I/O. No public portable-verifier API, compatibility verifier behavior,
FORMAT rule, fixture byte, manifest byte, canonical event identity, signature,
or release package version changes.

## Original local executed evidence (before PR #143 review)

The following commands actually ran in this candidate worktree. “Passed” below
does not refer to static or inferred test evidence.

| Check | Executed result |
| --- | --- |
| `python tools/check_verifier_corpus_manifest.py` | Passed; 432 cases, all exact-byte hashes, governing commitments, schema/coverage inventories, and named A-021 classifications checked. |
| `python -m unittest tools.test_portable_verifier tools.test_portable_verifier_differential -v` | Passed after the original local review remediation; 16 tests at that point. |
| `python tools/check_portable_verifier_python.py` | Passed all seven governed fields for all 432 cases with the exact expected inventory. |
| Go `test -count=1 ./...` and `vet ./...` in `tools/go-verify-chain` | Passed under Go 1.27.0 Windows/AMD64; this predated vendored/offline execution. |
| Go `run . --check-manifest ../../fixtures/verifier-language-v1/manifest.json` | Passed all 432 cases, but did not establish the compiled program's documented process-status contract. `go run` is not the current governed interface. |
| `python tools/check_portable_verifier_differential.py --go <pinned-go-1.27.0> --repeat 2` | Passed under the explicit VS 2022 build environment; both complete runs were semantically identical and all Rust/Python/Go fields matched for the default checkout. This predated selected-root Python isolation. |
| `python tools/verify_chain.py` over the frozen golden history and portable Deadbolt anchor history | Both compatibility histories passed with their pinned external keys. |
| `cargo fmt --all -- --check` | Passed. |
| `cargo test -p magpie-log --locked` | Passed: 75 default library tests, all integration tests, and 37 doctests; one extended assurance test remained intentionally ignored in this command. |
| `cargo test --workspace --locked --no-fail-fast` | Passed under the explicit VS 2022 environment, including workspace doctests. |
| `cargo test --doc -p magpie-log --locked` | Passed all 37 compile-fail/doc tests. |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Passed. |
| opt-in `assurance_a_extended_totality_containment_and_determinism` | Passed its 12,000-input non-normative campaign in 239.73 seconds. |
| `python tools/check_release_metadata.py` after local candidate commits | Passed; package inventories were consistent for `magpie-log` (36 files), `magpie-claims` (48), and `magpie-episodic` (9). |

The default/uninitialised MSVC environment was not silently treated as valid:
earlier Cargo attempts failed with missing MSVC C runtime/header evidence
(`msvcrt.lib` LNK1104 and `stdarg.h` C1083). One explicit Visual Studio 2022
Build Tools environment was then selected and every reported Rust success above
was executed there.

## Original local hostile review and remediation

Independent reviews found and caused three concrete corrections:

- Python originally let an arbitrarily long valid JSON integer escape as a
  Python digit-limit `ValueError`. The parser now rejects lexically at `Schema`
  without integer conversion, with a 5,000-digit regression.
- An early uncommitted local Go README draft invoked the nested module from the
  repository root and failed before execution. The committed examples use Go's
  `-C` module directory switch and were re-executed.
- The differential harness originally allowed Python equality to alias JSON
  `true` or `1.0` with integer `1`. It now enforces exact verdict-specific JSON
  types/shapes, u64 bounds, hash spelling/count, and tip consistency. The
  hostile checker confirmed the remediation and the real repeat-two run passed
  afterward.

Those pre-PR #143 local reviews reported no remaining semantic correctness
defect in their assigned surfaces. They did not supersede the later Codex
review of PR #143, which found the three additional defects below.

## Post-review remediation addendum — 2026-08-31

The first Codex review of PR #143 at `d57069f8e3758ddb991f07adae221b20a9cd9e15`
found one P1 and two independent P2 defects:

- **P1 — dependency authorization and offline availability.** The pinned
  `filippo.io/edwards25519 v1.2.0` module still required network or cache
  availability and lacked an explicit repository authorization. The owner
  authorized exactly that module and version, only for the Go conformer's
  existing low-level Edwards25519 point/scalar arithmetic. Normal Go vendoring
  now retains the upstream license and generated module metadata, adds no
  transitive module, and supports forced-vendor build, test, vet, and direct
  manifest execution with module retrieval disabled. Pinning and vendoring are
  source-identity and offline-availability evidence, not proof of correctness.
- **P2 — executable process status.** The original documentation used
  `go run` while promising the program's `0` / `1` / `2` status contract. The
  governed interface is now a built executable invoked directly. A focused
  child-process regression proves `ACCEPT = 0`, governed `REJECT = 1`, and a
  usage failure `= 2`.
- **P2 — selected-root coherence.** A non-default differential `root` selected
  the target manifest, Go conformer, and Rust conformer but could reuse the
  ambient checkout's imported Python conformer. Python now runs from the
  selected root in a fresh isolated child process. A discriminating
  alternate-root regression would fail the old ambient-import implementation.

The remediation is commit
`77b6b237b6333927fab99fc223347d42a2760ead`. A fresh Codex re-review of that
exact head reported no major issues. That is review evidence only: it is not
formal verification, proof of correctness, release approval, or authority.

The remediated differential binds the manifest, sidecar digest, every case
input digest, exact case inventory, strict primitive JSON types, exact result
shapes, frozen expected comparison material, and all seven governed fields:
`verdict`, `class`, `line`, `record_index`, `event_count`, `tip`, and
`ordered_recomputed_hashes`. Python, Go, and Rust actual results all come from
the selected repository root; expected results never construct an actual
result and no implementation is treated as a doctrine oracle.

### Final-stack local validation

The following checks ran after rebasing PR #144 onto `77b6b237`. Rust commands
used the explicit Visual Studio 2022 Build Tools environment. Go commands used
Go 1.27.0 with `GOENV=off`, empty override variables, `GOPROXY=off`,
`GOSUMDB=off`, `GOTOOLCHAIN=local`, `GOVCS=*:off`, `GOWORK=off`, forced
`-mod=vendor`, and new empty temporary module/build caches where applicable.
The module cache began empty and still contained zero entries after test, vet,
build, direct corpus execution, and the repeat-two differential.

| Check | Final-stack local result |
| --- | --- |
| `python -m unittest tools.test_portable_verifier tools.test_portable_verifier_differential -v` | Passed 17 tests. |
| Focused `test_selected_root_supplies_python_conformer_source` | Passed independently; the alternate-root fixture selected its own Python implementation. |
| `python tools/check_portable_verifier_python.py` | Passed all seven governed fields for all 432 cases. |
| `python tools/check_verifier_corpus_manifest.py` | Passed the exact 432-case inventory, manifest identity, sidecar, input digests, coverage commitments, and named A-021 classifications. |
| Go formatting | Passed for all five first-party Go files by comparing Go 1.27.0 `gofmt` output with the committed LF Git blobs. The Windows checkout itself uses CRLF conversion; no implementation file was rewritten. |
| Go `test -mod=vendor -count=1 ./...` | Passed from an initially empty module cache with retrieval disabled. |
| Go `vet -mod=vendor ./...` | Passed under the same vendored/offline constraints. |
| Focused `TestBuiltBinaryPreservesExitCodeContract` | Passed direct child-process statuses: `ACCEPT = 0`, governed `REJECT = 1`, usage failure `= 2`. |
| Go `build -mod=vendor` plus direct `--check-manifest` | Passed all 432 cases through the built executable under the same hostile offline constraints. |
| Dependency/vendor integrity | Passed: exactly `filippo.io/edwards25519 v1.2.0`, no transitive module, 17 vendor files / 119,735 checkout bytes, upstream license retained. A second `go mod vendor` in a temporary copy used an already populated pinned module cache with `GOPROXY=off`; all regenerated paths and Git blobs were identical. |
| `python tools/check_portable_verifier_differential.py --go <pinned-go-1.27.0> --repeat 2` | Passed all 432 cases across Rust/Python/Go and all seven governed fields; repetitions were semantically identical and the temporary Go module cache remained empty. |
| `cargo fmt --all -- --check` | Passed. |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Passed. |
| `cargo test -p magpie-log --locked` | Passed 75 tests with the 12,000-input opt-in campaign intentionally ignored, plus all integration tests and 37 doctests. |
| `cargo test --workspace --locked --no-fail-fast` | Passed the complete workspace, integration, and doctest graph. |
| `cargo test --doc -p magpie-log --locked` | Passed all 37 doctests. |
| `cargo run --locked --example tour -p magpie-claims` | Passed the deterministic governed-standing tour and byte-identical replay check. |
| `python tools/check_release_metadata.py` | Passed the committed package inventories: `magpie-log` 36 files, `magpie-claims` 48, and `magpie-episodic` 9. |
| `cargo package -p magpie-log --locked` | Passed package creation and verification for 36 files. |
| `python tools/verify_chain.py` over the frozen golden and portable Deadbolt histories with their pinned external keys | Passed all 9 golden and 2 Deadbolt records. |

An earlier pre-commit release-metadata invocation correctly refused the dirty
root README through Cargo's package-list guard. It was not counted as a pass;
the reported result above is the clean committed-tree rerun.

The opt-in 12,000-input assurance campaign was not rerun in this final-stack
pass. Its original local result remains dated evidence in the earlier original
local executed-evidence table; it is not ordinary CI and is not represented as
fresh execution.

## Claim fence

The candidate supports this narrow statement:

> Given one exact caller-supplied finite JSONL history, the exact selected
> `magpie-ed25519-canonical-prime-subgroup-v1` profile, and one caller-supplied
> structurally admissible external verifying key, the Rust, Python, and Go
> conformers agree on `verdict`, `class`, `line`, `record_index`,
> `event_count`, `tip`, and `ordered_recomputed_hashes` for the frozen 432-case
> conformance corpus. The Rust path also retains bounded non-normative assurance.

That statement does **not** establish:

- trust, ownership, identity, delegation, authority, or permission to act;
- freshness, latest-head selection, canonical branch, anti-rollback,
  non-equivocation, global completeness, or distributed consensus;
- universal equivalence for every possible input or proof of cryptographic or
  verifier correctness;
- production key custody, power-loss durability, or network-filesystem safety;
- authenticated independent corroboration (standing v3 remains a
  claimant-label compatibility count under the unimplemented ADR-0007 runtime);
- coordinate-complete authority for legacy detachable standing/search outputs;
  or
- a public release decision.

For standing, v1 may newly produce `Settled` only through its exact eligible
Deadbolt occurrence rule. v2, v3, and v4 cannot newly settle a claim; their
combination functions may only inherit already governed standing. This remains
separate from portable signature/history conformance.

## C3, C4, C5, and administrative position

### C3 — producing coordinates

The selected candidate claim is an immediately computed result over explicitly
supplied history/profile/key inputs. It does not advertise a detached governed
replay, standing, search, or authority-bearing receipt. Exclusion is therefore
sufficient for this boundary; adding a new detached receipt would broaden the
candidate without serving its claim. A coordinate-complete result remains a
future gate if that advertised surface changes.

### C4 — assurance

At the original assessment point, the candidate added routine CI for exact Go
1.27.0, pinned Python package versions, Python and Go complete-corpus checks,
and repeat-two three-way differential execution. The real differential had run
on one Windows host; GitHub-hosted Linux CI had not yet run for that unmerged
candidate, and Go race testing was blocked because the isolated toolchain had
CGO disabled. The old PR #144 head later received a green remote run, but that
run predated the PR #143 findings and remediation and is not final-stack
evidence.

The rebased final-stack CI explicitly forces vendored/offline Go module use
after toolchain setup, builds the governed executable outside the checkout,
runs that executable directly over all 432 cases, and runs the selected-root
three-way differential twice. This improves assurance but is not release
assurance. `ubuntu-latest`, Rust `stable`, action major tags, Python artifact
hashes, and the exact libsodium backend remain moving or incompletely pinned
inputs. Fresh exact-head GitHub-hosted evidence is recorded only after that run
actually completes; an independent second platform remains absent.

### C5 — release falsification

This file is candidate evidence, not a pre-alpha release contract. At the
original assessment point, a local candidate commit chain existed but no green
remote CI result did. The later green old-#144 run was against pre-remediation
bytes. It does not establish a final exact-head result, owner merge, or owner
release decision.
The latest GitHub source release is v0.2.0, while the tracked release contract
and workspace package version remain v0.1.0; those boundaries must not be
collapsed.

### Audit findings

A-004, RQ-006, and A-021 remain administratively **Confirmed** in the living
August disposition. This candidate supplies the previously absent complete
Python/Go implementations and cross-language evidence, but the governing
contract requires owner merge followed by a separate ledger reconciliation.
This implementation tranche therefore does not edit the living disposition or
claim administrative closure. RQ-013 remains only partially remediated and
RQ-015 remains open because wider platform, resource, crash/race, artifact, and
release-environment assurance is still absent.

## Verdict

**DEFENSIBLE LOCAL PRE-ALPHA CANDIDATE FOR OWNER REVIEW**

This verdict is local and bounded. It authorizes neither a release nor finding
closure. Owner review should require a fresh-checkout corpus check, green CI,
normal dependency provenance/license review, and a separate post-merge
A-021/A-004/RQ-006 reconciliation before any stronger public portability claim.
