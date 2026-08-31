# Public pre-alpha candidate recovery assessment — 2026-08-31

> This is a dated assessment of the local candidate checkout, not a rewrite of
> historical audits, a release contract, an owner release decision, or a claim
> about unmerged `main`. It supplements the 2026-08-22 convergence
> reassessment with evidence produced after PRs #139–#141.

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

## Candidate change

The candidate adds three deliberately separate evidence paths:

1. `tools/portable_verifier.py` is a readable complete Python conformer. It is
   separate from the compatibility-only `tools/verify_chain.py`, consumes raw
   bytes, preserves the frozen first-failure order, and implements the selected
   ADR-0010 relation using pinned low-level PyNaCl operations plus explicit
   Magpie canonicality, subgroup, scalar, identity, and equation gates.
2. `tools/go-verify-chain` is a standalone Go module. It shares no Rust or
   Python verifier code. It pins `filippo.io/edwards25519` v1.2.0 for focused
   point/scalar arithmetic while retaining Magpie-owned parsing, canonicality,
   subgroup, challenge, and uncofactored-equation checks.
3. `tools/check_portable_verifier_differential.py` preflights the pinned
   manifest, sidecar, and every input hash before invoking any conformer. It
   obtains actual results independently from Python, Go, and the real Rust
   conformer, then requires exact case order/identity, strict JSON value types,
   all seven governed fields, and the complete class inventory. Expected
   manifest results are comparisons only; no conformer receives them while
   constructing an actual result.

The Rust bridge is test-only. It writes an actual-results envelope only when
`MAGPIE_PORTABLE_DIFFERENTIAL_OUTPUT` is explicitly set and otherwise performs
no I/O. No public portable-verifier API, compatibility verifier behavior,
FORMAT rule, fixture byte, manifest byte, canonical event identity, signature,
or release package version changes.

## Executed evidence

The following commands actually ran in this candidate worktree. “Passed” below
does not refer to static or inferred test evidence.

| Check | Executed result |
| --- | --- |
| `python tools/check_verifier_corpus_manifest.py` | Passed; 432 cases, all exact-byte hashes, governing commitments, schema/coverage inventories, and named A-021 classifications checked. |
| `python -m unittest tools.test_portable_verifier tools.test_portable_verifier_differential -v` | Passed after review remediation; 16 tests. |
| `python tools/check_portable_verifier_python.py` | Passed all seven governed fields for all 432 cases with the exact expected inventory. |
| Go `test -count=1 ./...` and `vet ./...` in `tools/go-verify-chain` | Passed under Go 1.27.0 Windows/AMD64. |
| Go `run . --check-manifest ../../fixtures/verifier-language-v1/manifest.json` | Passed all 432 cases. |
| `python tools/check_portable_verifier_differential.py --go <pinned-go-1.27.0> --repeat 2` | Passed under the explicit VS 2022 build environment; both complete runs were semantically identical and all Rust/Python/Go fields matched. |
| `python tools/verify_chain.py` over the frozen golden history and portable Deadbolt anchor history | Both compatibility histories passed with their pinned external keys. |
| `cargo fmt --all -- --check` | Passed. |
| `cargo test -p magpie-log --locked` | Passed: 75 default library tests, all integration tests, and 37 doctests; one extended assurance test remained intentionally ignored in this command. |
| `cargo test --workspace --locked --no-fail-fast` | Passed under the explicit VS 2022 environment, including workspace doctests. |
| `cargo test --doc -p magpie-log --locked` | Passed all 37 compile-fail/doc tests. |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Passed. |
| opt-in `assurance_a_extended_totality_containment_and_determinism` | Passed its 12,000-input non-normative campaign in 239.73 seconds. |

The default/uninitialised MSVC environment was not silently treated as valid:
earlier Cargo attempts failed with missing MSVC C runtime/header evidence
(`msvcrt.lib` LNK1104 and `stdarg.h` C1083). One explicit Visual Studio 2022
Build Tools environment was then selected and every reported Rust success above
was executed there.

## Hostile review and remediation

Independent reviews found and caused three concrete corrections:

- Python originally let an arbitrarily long valid JSON integer escape as a
  Python digit-limit `ValueError`. The parser now rejects lexically at `Schema`
  without integer conversion, with a 5,000-digit regression.
- Every initial Go README example invoked the nested module from the repository
  root and failed before execution. All examples now use Go's `-C` module
  directory switch and were re-executed.
- The differential harness originally allowed Python equality to alias JSON
  `true` or `1.0` with integer `1`. It now enforces exact verdict-specific JSON
  types/shapes, u64 bounds, hash spelling/count, and tip consistency. The
  hostile checker confirmed the remediation and the real repeat-two run passed
  afterward.

The final Python, Go, and differential reviews reported no remaining semantic
correctness defect in their assigned surfaces.

## Claim fence

The candidate supports this narrow statement:

> Given one exact caller-supplied finite JSONL history, the exact selected
> `magpie-ed25519-canonical-prime-subgroup-v1` profile, and one caller-supplied
> structurally admissible external verifying key, the Rust, Python, and Go
> conformers agree on every governed result field for the frozen 432-case
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

The candidate adds routine CI for exact Go 1.27.0, pinned Python package
versions, Python and Go complete-corpus checks, and repeat-two three-way
differential execution. This improves assurance but is not release assurance.
`ubuntu-latest`, Rust `stable`, action major tags, Python artifact hashes and
the exact libsodium backend remain moving or incompletely pinned inputs. The
real differential was executed on one Windows host; GitHub-hosted Linux CI and
an independent second platform have not yet run for this unmerged branch. Go
race testing was blocked because the isolated toolchain had CGO disabled.

### C5 — release falsification

This file is candidate evidence, not a pre-alpha release contract. There is no
frozen candidate commit until the local review commits exist, no green remote
CI result for those commits, no owner merge, and no owner release decision.
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
