# Magpie runtime quality audit — 2026-08-04

## Audit point and operating contract

This is a read-only principal-engineer runtime quality and assurance audit of
the exact local checkout at:

    C:\magpie

The audited commit is:

    3b46fdb81857d77b827271777b86745bca5f7b12

The audit was performed on 2026-08-04 against local branch `main`. The
requested deliverable is this file. No Rust source, tests, fixtures,
Cargo.toml, Cargo.lock, CI configuration, generated artifact, Git reference,
commit, PR, issue, or other implementation file was changed.

The existing untracked file
`docs/audits/magpie-architecture-audit-2026-08-01.md` was treated as user
material and preserved. It was used as prior evidence, then the current
implementation and current external state were independently checked.

### Preflight record

| Check | Observed result |
|---|---|
| Repository / remote | `noctem-o/magpie`; `origin` is `https://github.com/noctem-o/magpie.git` |
| Repository-local instructions | `AGENTS.md`, `CLAUDE.md`, `CLAUDE.local.md`, `.claude/agents/codex-worker.md`, and `docs/FORMAT.md` were read before analysis |
| Active branch | `main` |
| Local HEAD | `3b46fdb81857d77b827271777b86745bca5f7b12` |
| Local `refs/heads/main` | Same SHA as HEAD |
| `origin/main` | `21067dcfe00175430fe751d669f136a182f6989b` |
| Divergence | `git rev-list --left-right --count HEAD...origin/main` = `0 9`; local main is behind by nine commits |
| Merge base | `3b46fdb81857d77b827271777b86745bca5f7b12` |
| Open PRs at audit time | `gh api repos/noctem-o/magpie/pulls?state=open&per_page=100` returned `[]` |
| Worktree | `C:\magpie` is on `main`; other listed worktrees were not touched |
| Tracked worktree | No tracked modifications |
| Pre-existing untracked material | `docs/audits/magpie-architecture-audit-2026-08-01.md` |
| Toolchain | `rustc 1.94.1`, `cargo 1.94.1`, stable MSVC default, Python `3.11.15` |
| Lockfile | `Cargo.lock` is tracked; 59 package stanzas |

No fetch, reset, rebase, branch switch, merge, push, PR mutation, or worktree
repair was performed. The findings below therefore apply to the exact local
commit, not to the nine commits currently ahead on `origin/main`.

## Executive judgment

**Verdict: CONDITIONAL for local experimentation; NOT READY for
authority-bearing production or release approval.**

The current commit is test-green and has several good structural qualities:

- the frozen `magpie-core-v1` canonical format has Rust golden coverage and an
  independent Python verifier;
- the normal replay path verifies one stored snapshot before applying that
  same retained event set;
- serialized derived maps generally use `BTreeMap`;
- the versioned policy code is explicit, with private composition seams in the
  newer audit paths;
- no unsafe Rust code was found;
- the full workspace test, format, clippy, tour, metadata, package, and golden
  verifier checks passed locally.

Those results do not establish the larger authority contract. The most
important risks are API and boundary risks that passing tests do not exercise:

1. a public `LogStore::append_record` seam remains alongside the declared sole
   `LogWriter` write capability;
2. parsed-but-unverified and verified events share `SignedEvent`, while public
   `Projection::apply` accepts that same type;
3. legacy status is still exposed through public, authority-looking fields;
4. Rust and Python accept different serialized languages;
5. the file backend materializes the whole log and has no explicit crash,
   durability, locking, or resource contract;
6. the infallible projection trait permits operational SQLite failures to
   become panics;
7. current documentation has several status statements that describe landed
   runtime as future or “not implemented.”

The primary release decision is therefore not “the tests are failing.” It is
“the test suite does not yet close the declared replay, authority, resource,
and cross-language boundaries.” The P0 items are RQ-001, RQ-002, and RQ-006.
RQ-003 through RQ-010 are P1 quality and boundary work. The remaining items
are P2 assurance and maintenance work.

## Governing corpus and status interpretation

The audit used the following evidence hierarchy:

1. the actual Rust/Python implementation and tests at the audited commit;
2. normative format and accepted ADR text;
3. ratified versioned design contracts, with historical future-tense text
   treated as historical where the document explicitly says so;
4. the prior architecture audit as a lead list and historical record, not as a
   substitute for current source inspection.

Relevant current corpus:

| Corpus | Current interpretation |
|---|---|
| `CLAUDE.md` | Repository implementation constraints; it says the log is the source of truth, `LogWriter` is the only write path, projections are pure folds, and serialized maps use `BTreeMap`. |
| `docs/FORMAT.md` | Normative frozen `magpie-core-v1` bytes, event tags, JSON-lines storage convention, and external trust-root rule. |
| `docs/adr/0001-deadbolt-seam.md` | Accepted; explicitly describes `LogWriter` as the sole L0 write capability and the log as the source of truth. |
| `docs/adr/0002-governed-claim-memory.md` through `docs/adr/0005-claim-withdrawal-acts.md` | Current ADR corpus; the first ADR is accepted and the later ADR status must not be inferred from implementation alone. |
| `README.md` | Current product description; it describes the experimental v0-v4 kernel, replay, and non-authority of audit output. |
| `docs/design/standing-policy-v3-external-report-corroboration-v0.md` | Ratified contract whose header still says runtime is not implemented; current code contains `standing_v3` runtime. |
| `docs/design/origin-admission-audit-v0.md` | Ratified contract whose header still says origin admission runtime remains future; current code contains the audit path. |
| `docs/design/inline-predicate-attestation-binding-v0.md` and related edge-lane/v4 notes | Mixed historical handoff language; some sections correctly preserve historical contract wording while others call the current candidate patch future or incomplete. |
| `docs/audits/magpie-architecture-audit-2026-08-01.md` | Existing untracked prior evidence. It was not edited or treated as current live PR state. |

The status drift is recorded as RQ-014 rather than used to rewrite doctrine in
this audit.

## Codebase and dependency map

### Runtime components

| Component | Responsibility | Quality boundary |
|---|---|---|
| `crates/magpie-log` | Event vocabulary, canonical bytes, hash/signature verification, stores, writer, reader, replay | L0 source-of-truth and verification seam |
| `crates/magpie-claims` | Legacy and typed claim projections, standing v0-v4, deterministic predicates, provenance/origin contexts, private policy composition | Policy and provenance authority-looking outputs |
| `crates/magpie-episodic` | SQLite-derived event timeline and FTS search projection | Infallible projection API, schema lifecycle, query output provenance |
| `tools/verify_chain.py` | Independent Python verifier for frozen chain fixtures | Cross-language grammar and acceptance parity |
| `tools/check_release_metadata.py` | Package metadata and inventory checks | Release metadata assurance |
| `.github/workflows/ci.yml` | Formatting, clippy, tests, tour, metadata, package, Python golden verification | Toolchain and dependency reproducibility |

### Direct dependency map

The workspace has seven direct workspace dependency declarations:

- `serde` with derive;
- `serde_json`;
- `sha2`;
- `ed25519-dalek`;
- `thiserror`;
- `hex`;
- `rusqlite` with bundled SQLite.

The lockfile contains 59 package stanzas. The dependency set is small and
appropriate to the implementation: serialization, hashing, signatures,
errors, hex, and embedded SQLite. No unused or speculative network dependency
was added. The main reproducibility concern is outside the lockfile: CI
installs the latest stable Rust, latest `cryptography`, and mutable GitHub
Action major tags.

### Production module disposition

This table covers every Rust source file under the three crate src trees
(28 files, including the nested edge-lane module). Line counts are physical
lines from Get-Content; the disposition records the owned invariant and the
quality outcome of the audit.

| Module | Lines | Owned responsibility and disposition |
|---|---:|---|
| magpie-log/src/canonical.rs | 183 | Frozen event byte encoder; singular and explicit; retain as the canonical owner |
| magpie-log/src/error.rs | 11 | Typed L0 errors; appropriately small and clear |
| magpie-log/src/event.rs | 335 | Event vocabulary, serde, payload validation; raw public type seam and grammar review required (RQ-002, RQ-006) |
| magpie-log/src/hashing.rs | 59 | Content-hash wrapper and serde; small, with hex-language drift (RQ-006) |
| magpie-log/src/lib.rs | 31 | Root export boundary; small but exposes storage/projection seams (RQ-001, RQ-002) |
| magpie-log/src/logimpl.rs | 495 | Writer, verifier, reader, and replay; largest L0 core module but proportionate; verified-type/resource follow-up (RQ-002, RQ-007) |
| magpie-log/src/store.rs | 88 | File/memory backends; clear ownership but public append and missing storage contract (RQ-001, RQ-007, RQ-008) |
| magpie-claims/src/admitted_contribution_audit.rs | 910 | Derived admission audit composition; private construction and explicit ordering are good; policy review surface is large |
| magpie-claims/src/artifact_provenance_verifier.rs | 1,439 | Artifact parser/verifier and receipts; singular boundary, mechanical duplication contributes to RQ-012 |
| magpie-claims/src/claim_inline_sha256_predicate.rs | 1,889 | Bounded inline subject/predicate resolver; strong hostile bounds, high parser review cost |
| magpie-claims/src/claim_inline_sha256_predicate/edge_lane.rs | 1,743 | Closed edge/relation matrix; explicit standing-inert lane, high but justified policy surface |
| magpie-claims/src/deadbolt_context.rs | 375 | Anchor index and occurrence context; derived/read-only seam is clear |
| magpie-claims/src/deterministic_verifier_context.rs | 784 | Deterministic predicate context and bounded witness handling; private receipt boundary is strong |
| magpie-claims/src/inline_predicate_attestation.rs | 1,215 | Strict attestation parser and routing output; singular, with repeated parser machinery noted in RQ-012 |
| magpie-claims/src/lib.rs | 234 | Root re-exports and legacy ClaimsView; high fan-in API surface, not a policy owner |
| magpie-claims/src/origin_admission_audit.rs | 701 | Candidate enumeration and admission audit composition; private resolver path is good, status prose is stale (RQ-014) |
| magpie-claims/src/origin_admission_replay.rs | 516 | One verified origin replay context; strong snapshot/provenance seam |
| magpie-claims/src/origin_binding_verifier.rs | 2,473 | Origin binding parser/evaluator and private receipts; largest verifier hotspot, otherwise singular (RQ-012) |
| magpie-claims/src/policy.rs | 1,553 | Closed support/refutation/context matrices; normative ownership is clear, mechanical size is review debt |
| magpie-claims/src/replay_snapshot.rs | 106 | Private verified standing snapshot; appropriately small and structurally useful |
| magpie-claims/src/resolution_content_closure.rs | 865 | Bounded closure construction and identity; explicit resource law is strong |
| magpie-claims/src/standing.rs | 2,345 | Legacy and typed projection plus v0 resolver; mixed responsibilities and public raw standing are the main god-module risk (RQ-003) |
| magpie-claims/src/standing_v1.rs | 437 | Versioned v1 lane application; explicit policy, one duplicated inherited-standing branch |
| magpie-claims/src/standing_v2.rs | 816 | Deterministic direct-support policy; explicit failure tests, wildcard extension risk (RQ-005) |
| magpie-claims/src/standing_v3.rs | 2,260 | Corroboration policy and composition; separate policy ownership is correct, status/docs drift and size remain (RQ-005, RQ-014) |
| magpie-claims/src/standing_v4.rs | 2,711 | Direct-refutation policy and candidate derivation; explicit private composition, largest policy hotspot |
| magpie-claims/src/support_contribution_audit.rs | 2,943 | Support audit composition and invariant ordering; intentionally explicit but largest review surface (RQ-012) |
| magpie-episodic/src/lib.rs | 399 | SQLite/FTS derived projection and search; clear ownership, but synthetic status, panic, reset, and provenance gaps (RQ-004, RQ-009, RQ-010, RQ-011) |

## Quantitative profile

The following counts were measured at the audit point. `tokei` counts are
physical source lines and are intended as review-surface indicators, not
quality scores.

| Area | Files | Physical lines / bytes | Notes |
|---|---:|---:|---|
| `magpie-log` crate | 12 | 2,853 | 2,636 Rust lines |
| `magpie-claims` crate | 42 | 44,877 | 41,510 Rust lines |
| `magpie-episodic` crate | 3 | 782 | 709 Rust lines |
| Production Rust under `src` | 28 | 24,323 | 929 code lines in log, 21,027 in claims, 315 in episodic |
| Rust test directories | 24 | 19,740 | 0.812 test-to-production physical-line ratio |
| Rust source plus tests | 52 | 44,063 | Tests are 44.8% of this combined physical-line total |
| Tracked documentation | 54 | 25,688 | Markdown plus HTML/CSS; excludes the untracked prior audit |
| Tracked tickets | 65 | 25,616 | Historical/contract review surface |
| Python tools | 2 | 383 | 359 code lines |
| Fixtures | 36 | 112,832 bytes | 29 JSON fixtures, three Markdown README files, three text files |
| Tracked repository files | 235 | — | `git ls-files` count |
| Tracked repository content | 235 | 4,043,233 bytes (3.86 MiB) | Sum of the sizes of every `git ls-files` path; excludes ignored target artifacts and untracked audit files |

Largest production Rust modules by physical lines:

| Module | Lines | Review implication |
|---|---:|---|
| `support_contribution_audit.rs` | 2,943 | Explicit policy composition and failure ordering are concentrated here |
| `standing_v4.rs` | 2,711 | Nested v3/v4 policy staging and candidate derivation |
| `origin_binding_verifier.rs` | 2,473 | Parser, binding, and hostile assurance surface |
| `standing.rs` | 2,345 | Legacy compatibility plus typed standing projection |
| `standing_v3.rs` | 2,260 | Corroboration composition and inherited standing |
| `claim_inline_sha256_predicate.rs` | 1,889 | Bounded parsing and predicate resolution |
| `edge_lane.rs` | 1,743 | Four-cell edge/relation lane matrix |

The source-only syntax inventory found 121 public type-like declarations,
365 `pub fn` declaration lines, 58 public constants/statics, and 26 public
use/module lines: 570 public-surface syntax lines. This is not an ABI count;
it includes public items in private implementation modules and methods. It is
useful here because the repository has a deliberately wide re-exported
contract surface.

The production source scan found 48 `expect` calls, two `panic!` calls, and
nine `unreachable!` calls after excluding comments and `#[cfg(test)]` bodies.
The two `unsafe` word hits are panic-message strings in `standing_v4.rs`; no
unsafe Rust item was found. The `expect` count is not itself a defect: many
calls assert serialization or internal validated-state invariants. The
operational SQLite panics are separately recorded as RQ-009.

| Production assertion category | Count | Interpretation |
|---|---:|---|
| Claims serialization/validated-state expect | 38 | Mostly deterministic serialization or post-validation invariants; not automatically defects |
| Episodic operational expect | 10 | SQLite open/read/transaction/insert/commit and canonical-row operations; availability risk (RQ-009) |
| Episodic range-conversion panic! | 2 | Signed SQLite range and negative-value rejection; availability risk (RQ-009) |
| Internal unreachable! branches | 9 | Seven claims enum/invariant branches and two log verification-retention branches; review for extension pressure |
| Production unwrap | 0 | No production unwrap calls found |

### Public API classification counts

The 570-line syntax inventory above is intentionally broad. To make the
capability classification auditable without pretending that re-exported names
are new symbols, I also counted high-value public surface families. A family
can contain several declarations and a declaration can serve both a
construction and a data role; these are review counts, not an ABI total.

| Classification | Reviewed public surface families | Examples |
|---|---:|---|
| Authority-bearing capability | 3 | LogWriter, verified replay/context entry points, resolver-created authority-bearing outputs |
| Verified input/context | 2 | LogReader verified replay and replay-derived snapshot/context types |
| Unverified/raw input or storage | 4 | LogStore/stores, SignedEvent/Payload, selectors, closure input |
| Derived projection | 4 | ClaimsView, StandingView, EpisodicView, DeadboltAnchorIndex |
| Versioned policy output | 5 | standing v0, v1, v2, v3, and v4 resolution families |
| Audit trace or receipt | 8 | artifact, origin, admitted/support, attestation, edge-lane, and closure families |
| Compatibility-only value | 3 | legacy Claim, StandingClaim, and compatibility projection surfaces |
| Construction/storage helper | 5 | store constructors, episodic constructors, and closure/selector constructors |

The exact declaration-level inventory remains: magpie-log has 16 direct
public types/traits/aliases, 2 direct constants, 0 free functions, 26 public
methods, and 19 root re-exported names; magpie-claims has 101 direct public
type/enum declarations, 56 constants, 6 free functions, 326 public methods,
and 152 names in 17 root pub use blocks; magpie-episodic has 2 public
structs and 7 public methods. These counts use anchored source syntax and
exclude tests/examples from the production inventory.

### Required counted quality signals

The following counts use explicit audit rubrics rather than raw search hits:

| Signal | Count | Counting method |
|---|---:|---|
| Positive cross-language vectors exercised | 2 | golden-v1.jsonl and deadbolt-anchor-v1/anchor-log.jsonl were each run through tools/verify_chain.py with an external key |
| Negative cross-language differential vectors | 0 | No committed corpus currently compares uppercase hex, numeric status, blank lines, malformed JSON, or equivalent boundary cases in both implementations |
| High-value authority invariants reviewed | 12 | One rubric covering receipt opacity, caller substitution, replay integrity, standing quarantine/promotion, versioned policy failure, write/projection seams, and detached provenance |
| Authority invariants with a direct negative test | 8 of 12 | Private receipt construction, receipt deserialization/reinjection, caller-selected inputs, replay tamper/no-partial-apply, v0 quarantine, v2 fail-closed/no-amplification, v3 failure/never-settled, and v4 ineligible/unsafe families have direct negative or compile-fail evidence |
| Authority invariants relying primarily on caller convention | 4 of 12 | Sole writer/backend append, unverified-event projection use, raw standing access, and detached-result provenance |
| Comparable duplicated inherited/achieved-standing policy branches | 4 | One branch site in each v1-v4 path: standing_v1.rs:171, standing_v2.rs:300-309, standing_v3.rs:885-893, and standing_v4.rs:1167-1213 |
| Experimentally demonstrated undetected semantic mutations | 0 | No mutation was applied in this non-mutating audit |

The four caller-convention count deliberately excludes the external trust-root
rule, which is an intentional trust boundary documented in docs/FORMAT.md,
not an accidental missing type seam.

### Largest inspected production functions

Function spans were estimated with a brace-balance scan over production source
before #[cfg(test)]; strings and macro tokens make this an approximate review
metric, not a complexity proof.

| Function | Approx. lines | Reference |
|---|---:|---|
| StandingView::apply | 117 | crates/magpie-claims/src/standing.rs:662-778 |
| compose_standing_resolution_v4 | 95 | crates/magpie-claims/src/standing_v4.rs:1119-1213 |
| StandingView::resolved_standing_with_trace | 82 | crates/magpie-claims/src/standing.rs:273-354 |
| evaluate_support_candidate_structure_v0 | 75 | crates/magpie-claims/src/standing.rs:424-498 |
| derive_candidate_trace_with_v4 | 70 | crates/magpie-claims/src/standing_v4.rs:1305-1374 |

## Public capability and API map

### L0 and storage

`crates/magpie-log/src/store.rs` exposes:

- `LogStore`, with public `append_record(&mut self, &[u8])` and
  `read_records(&self)`;
- `FileStore`, whose public constructor accepts any path;
- `MemStore`, with public construction from arbitrary raw records and a public
  record snapshot method.

`crates/magpie-log/src/logimpl.rs` exposes `LogWriter`, `LogReader`, and
`Projection`. `LogWriter` owns the signing key and emits the normal signed
record. `LogReader::verify_chain` and `replay_with_summary` are the intended
verified paths. `LogReader::events` is explicitly documented as parsed but
unverified. The type-system boundary is nevertheless weakened because
`events()` returns the same public `SignedEvent` type consumed by the public
`Projection::apply` trait.

### Claims and standing

`crates/magpie-claims/src/lib.rs` re-exports the versioned policy, provenance,
replay, closure, and receipt-shaped types. Newer authority-bearing outputs
mostly use private fields, read-only getters, `Serialize` without
`Deserialize`, and crate-private composition. That is a strong direction.

The compatibility projections remain more permissive:

- `Claim` has public mutable `statement`, `status`, `evidence`, and
  `transitions` fields;
- `ClaimsView` has a public mutable `BTreeMap`;
- `StandingClaim`, `StandingResolution`, `StandingTraceEntry`, and the
  `StandingView` data maps expose public fields;
- `StandingView` implements `Projection::apply(&SignedEvent)` directly.

This creates two simultaneous APIs: a carefully bounded replay-derived policy
path and a low-level manually mutable projection path. The documentation warns
callers not to treat unverified/manual application as authority, but the
public types do not make that distinction hard to cross.

### Episodic projection

`EpisodicEvent` is a public serializable row. `EpisodicView` is a derived
SQLite/FTS index with public `in_memory`, `at_path`, `get`, `search`,
`canonical_bytes`, and size methods. Search returns sequence numbers only; it
does not carry the verified prefix identity, query identity, or snapshot
identity that would make a detached result reproducible.

### Public boundary disposition

| Public surface | Classification | Disposition |
|---|---|---|
| LogWriter and signing-key append methods | Authority-bearing capability | Correct normal writer owner; must be the only reachable mutating backend path (RQ-001) |
| LogReader::verify_chain and replay/replay_with_summary | Verified replay capability | Logical one-snapshot path is strong; verifier-created event type/token is missing (RQ-002) |
| LogReader::events | Unverified/raw input | Correctly documented as unverified, but ordinary SignedEvent output is projection-compatible (RQ-002) |
| LogStore, FileStore, MemStore | Storage backend | Appropriate low-level abstraction for current local use, but public append and file semantics are too permissive (RQ-001, RQ-008) |
| Payload, EventCore, SignedEvent, Provenance, ContentHash, Sig | Raw/canonical historical values | Explicit canonical owner; public construction and cross-language serde grammar require care (RQ-002, RQ-006) |
| ClaimsView and StandingView | Derived compatibility projections | Regenerable, but mutable fields/direct apply and raw standing remain caller-sensitive (RQ-002, RQ-003) |
| StandingResolution v0-v4 and policy application types | Versioned policy outputs | Version separation and newer private seams are good; old outputs lack some coordinates and v2/v3 have extension risk (RQ-005, RQ-011) |
| Artifact/origin/admitted/support/attestation receipts and traces | Audit material | Newer values are private-construction and Serialize-only; compile-fail examples need exact failure contracts (RQ-013) |
| StandingReplaySnapshot and origin replay context | Verified replay context | Strong private snapshot/provenance seam; no caller-supplied receipt route observed |
| EpisodicEvent, EpisodicView, and search results | Derived timeline/query surface | Useful regenerable projection; synthetic status, panic/reset behavior, and detached query identity remain open (RQ-004, RQ-009, RQ-010, RQ-011) |

## Determinism and verifier map

### Strong controls observed

- `docs/FORMAT.md` and `crates/magpie-log/src/canonical.rs` define the frozen
  profile and explicit payload-tag match.
- `BTreeMap` is used for serialized derived maps.
- `LogReader::replay_with_summary` calls `read_records` once, verifies the
  exact retained snapshot, then applies those retained events.
- `StandingReplaySnapshot` and the origin-admission replay context retain
  verified prefix information privately rather than accepting caller-supplied
  receipts.
- The versioned policy outputs use explicit field order and deterministic
  collection order.
- The tour checks identical regenerated snapshot and resolution bytes.

### Acceptance-language mismatches

These are cross-language acceptance differences, not failures of the existing
golden vector:

| Input case | Rust implementation | Python verifier | Consequence |
|---|---|---|---|
| Uppercase content hash or signature hex | `hex::decode` in `ContentHash` and `Sig` deserializers accepts uppercase | `hx()` requires lowercase characters | The same JSON record can parse differently across verifiers |
| Numeric status | Rust serde `Status` accepts the named enum representation | Python `status()` accepts either names or integers 0 through 4 | Python may accept a record Rust rejects |
| Blank JSONL line | `FileStore::read_records` filters empty split lines | Python loops over every line and reports JSON decode failure | Storage and independent verifier disagree on blank-line policy |
| Trust-root spelling | Python lowercases the command-line trust argument; event-field acceptance remains strict lowercase | Rust compares the decoded genesis key after hex decoding | Trust-root and event-field normalization are not one declared grammar |

The relevant evidence is `crates/magpie-log/src/hashing.rs:48-54`,
`crates/magpie-log/src/event.rs:315-325`, `crates/magpie-log/src/store.rs:41-54`,
and `tools/verify_chain.py:49-60, 214-237`. Golden vectors exercise the
intersection, not these boundary differences.

## Test-assurance matrix

The workspace test command reported 281 passing Rust unit/integration tests
with zero failures; the compile-fail doctests also passed in their separate
doctest groups. The coverage profile is:

| Invariant / surface | Current assurance | Evidence | Residual gap |
|---|---|---|---|
| Frozen canonical bytes and tags 0-8 | Strong | Golden vectors, canonical unit tests, Python golden verifier | Cross-language negative grammar is not fully differential |
| Sequence, previous hash, event hash, signature, genesis binding | Strong | Log chain tests and Python golden/anchor fixtures | File crash/torn-write behavior is untested |
| Single-snapshot replay | Strong | `replay_with_summary`, custom store/read-count tests, regeneration tests | Resource limits are absent |
| Bounded inline/foreign parser inputs | Strong locally | Hostile multibyte, escape, malformed, and boundary tests | Does not bound the base log reader |
| Standing v0-v4 policy precedence | Strong for current vocabulary | Focused policy and fixture tests | Future `Status` extension is not fail-closed in v2/v3 |
| Receipt construction and caller-supplied substitution | Strong for private fields | Compile-fail and hostile tests | Some compile-fail snippets fail at stale arity, not the precise forbidden surface |
| Legacy status quarantine | Partial | `legacy_status_change_is_visible_but_quarantined` and related tests | Public raw `standing` remains easy to read and mutate |
| Episodic regeneration and FTS search | Partial | Episodic tests and canonical bytes | Error paths panic; output has no query/prefix provenance |
| Rust/Python acceptance parity | Partial | Golden chain and anchor chain both pass | No uppercase, numeric-status, blank-line, or malformed corpus parity suite |
| File durability and concurrent append | Gap | No crash or concurrency test family found | No fsync, lock, atomic frame, or snapshot contract |
| Resource exhaustion | Partial | Several foreign-object and subject limits | No max log bytes, record bytes, record count, or streaming reader |
| Mutation/differential assurance | Gap in this audit | No mutation experiment was run because the audit remained non-mutating | Undetected semantic mutations are therefore not experimentally measured |

### Compile-fail test quality

The compile-fail suite is valuable for private construction and
deserialization boundaries. It is not uniformly a precise contract test. For
example, `crates/magpie-claims/src/origin_binding_verifier.rs:58-70` calls
`resolve_origin_binding_context_v0` with an extra receipt argument, while the
current method at roughly lines 541-547 accepts only selector and closure.
That proves the shown call does not compile, but the failure is primarily
wrong arity. It does not independently pin the semantic rule that a
serialized/cloned receipt cannot be supplied to an otherwise matching
resolver. Similar stale-arity patterns occur in the origin-admission and
support-contribution doctests.

### Highest-value mutation catch map

| Single weakening mutation | Exact catching evidence | Audit result |
|---|---|---|
| Change canonical field/tag order or hash input | magpie-log/tests/golden.rs golden bytes plus the Python golden fixture | Caught for the existing frozen corpus |
| Apply a valid prefix before a later verification failure | magpie-log/tests/chain.rs:320, 349, 433 | Caught; no partial authoritative projection escapes |
| Add a second store read or mix snapshots | magpie-log/tests/chain.rs:171, 202, 245, 281 and claims/tests/standing_replay_snapshot.rs:135, 251 | Caught for current replay paths |
| Allow v2/v3 policy evidence to amplify or override inherited standing | standing_v2 tests combination_is_closed_and_non_amplifying; standing_v3 tests standing_precedence_and_never_settled_law_is_exact | Caught for current five-status vocabulary; future wildcard extension remains open |
| Remove a private receipt/selector boundary | compile-fail families in origin_binding_verifier.rs, origin_admission_audit.rs, support_contribution_audit.rs, and resolution_content_closure.rs | Usually caught, but some examples fail first for stale arity |
| Manufacture tag-6 episodic status | no direct test asserts ClaimAssertedV2 claim_status is source-present | Not caught; RQ-004 |
| Use raw append or unverified events as authority | no compile-fail or runtime negative test forbids LogStore append or Projection::apply on LogReader::events output | Not caught; RQ-001/RQ-002 |
| Promote a future Status variant through v2/v3 wildcard arms | no current future-variant test can compile; standing_v2.rs:300-309 and standing_v3.rs:885-893 are the unguarded sites | Not caught; RQ-005 |

This table is why the passing suite is strong evidence for current replay and
policy examples but not a complete proof of the outer capability contract.

## Complexity and abstraction-cost assessment

The high line counts are not automatically over-engineering. The newer policy
modules deliberately duplicate explicit versioned vocabulary and precedence so
that a policy change is not hidden behind a dynamic registry or a generic
“latest” selector. That is a defensible authority boundary.

The cost is real:

- `standing.rs`, `standing_v2.rs`, `standing_v3.rs`, `standing_v4.rs`, and the
  audit modules repeat identity, trace, failure, and canonical-output
  patterns;
- parser visitors and typed canonical encoders are hand-written in several
  modules;
- policy evolution requires coordinated updates to source, fixtures, tests,
  docs, and the independent Python language;
- wildcard matches in v2/v3 are especially dangerous because they hide a
  future vocabulary extension in otherwise explicit policy code.

The lowest-risk maintenance strategy is to share only private mechanical
helpers for identical validation, byte framing, and coordinate access. Keep
policy decisions, failure precedence, and versioned public output types
explicit. A generic policy engine, dynamic policy registry, or code generator
would reduce local repetition at the cost of making authority review harder;
that is not recommended by this audit.

## Performance, resource, and failure-path assessment

### Current asymptotic shape

- `FileStore::read_records` reads the whole file with `std::fs::read`, splits
  it, and clones each line into `Vec<Vec<u8>>`: O(F) memory for file bytes plus
  line copies.
- `verify_chain_on` and replay parse JSON into owned `SignedEvent` values.
  Replay retains the raw record vector and parsed event vector during the
  operation, so peak memory is materially greater than the serialized log.
- `BTreeMap` projections provide deterministic order at O(log n) insertion
  cost, which is a deliberate determinism tradeoff.
- `EpisodicView::apply` opens and commits one SQLite transaction per event and
  writes both the base row and FTS row. That is simple and consistent but can
  be expensive for large rebuilds.
- Several foreign closure, subject, bundle, object, and entry limits exist in
  `magpie-claims`. Those bounds do not protect the base L0 reader from an
  oversized file, line, event payload, or record count.

### Failure and availability shape

`Projection::apply` is infallible. `EpisodicView::apply` calls `expect` for
transaction begin, row insertion, FTS insertion, and commit. It also panics
for values outside SQLite's signed 64-bit range and for invalid negative
stored sequence/timestamp values. A disk-full, SQLite corruption, or
resource-range event can therefore panic during a replay rather than return
a typed projection failure.

`FileStore` appends two writes to a newline-delimited file. It does not call
`sync_all`, take a file lock, define atomic record framing, or expose a
snapshot/version boundary. A crash or concurrent writer can leave a partial
line or an ordering that later verification rejects. The current tests check
logical tampering, not these process and filesystem conditions.

Conversion and fallback review found no production unwrap or silent
truncation, but several unchecked u64 casts remain at
crates/magpie-log/src/canonical.rs:48-50 and
crates/magpie-log/src/logimpl.rs:16,126,386; string length, sequence/count,
and nanosecond timestamp values are cast rather than bounded at the outer log
boundary. The current types make ordinary inputs safe; RQ-007 is why this is
not treated as a complete extreme-input contract. The policy parser's lack of
a generic JSON recursion cap is intentional for its current contract and is
not promoted as a defect without a stated depth requirement.

No synthetic performance benchmark was run. Consequently, no numeric
production-size threshold is claimed. The first operationally uncomfortable
input is the point at which raw file bytes plus line copies plus parsed event
values exceed the host's memory/latency budget; the current design does not
make that threshold configurable or observable.

## Documentation-to-code consistency

The README and `CLAUDE.md` are broadly aligned with the implementation's
replay-first direction. The following narrower inconsistencies affect
maintenance and review:

| Document | Statement | Current implementation | Effect |
|---|---|---|---|
| `docs/design/standing-policy-v3-external-report-corroboration-v0.md:5-12` | Runtime not implemented; documentation only | `standing_v3.rs` contains `combine_standing`, composition, and tests | A maintainer may treat an existing policy path as future or reimplement it |
| `docs/design/origin-admission-audit-v0.md:5-16` | Runtime origin admission and `OriginAdmissionAuditV0` remain future | `origin_admission_audit.rs` and `origin_admission_replay.rs` contain the runtime path | The contract's future handoff no longer describes the local code |
| `docs/design/inline-predicate-attestation-binding-v0.md:6-9, 980-1010` | Mixed “current candidate patch” and historical future sequence | Local main contains merged runtime and later policy code | Reviewers must reconstruct which sentences are historical |
| `docs/design/standing-policy-v4-claim-inline-direct-refutation-v0.md:5-15` | Ticket 0065 runtime is implemented | Matches the local v4 runtime more closely | This document is a useful example of the desired status handoff |
| `README.md:197-243` | `LogWriter` is the low-level append capability and `LogReader` is read-only | Public `LogStore::append_record` and public mutable projections remain | Product-level claim is stronger than the actual capability surface |

The fix should be a narrow status ledger and API-boundary clarification, not a
rewrite of historical contract documents.

## Maintainability simulations

### Add one new `Status` variant

The change would force many exhaustive Rust matches to be reviewed, which is
good. However, `standing_v2.rs:300-309` has `Some(_) if
deterministic_supported`, and `standing_v3.rs:885-893` has `_ if corroborated`.
A new variant could therefore be promoted to `Supported` by old policy without
an explicit v2/v3 policy decision. `v4` is more explicit and would provide
better compiler pressure. The simulated change is a latent compatibility
failure, recorded as RQ-005.

### Add one new `Payload` tag

The explicit canonical payload match, Rust projection matches, and Python
payload encoder would all need updates. Rust exhaustiveness catches several
sites, but the independent verifier and every projection are separate
maintenance surfaces. A differential fixture and a compile-time checklist
would reduce omission risk without introducing a generic payload registry.

### Add a provenance coordinate to resolution output

The newer v4 and replay-context paths already retain verified prefix and
closure identities privately. The legacy v0-v2 resolution types, episodic rows,
and search result still have different shapes. The change would touch output
types, canonical bytes, fixture vectors, replay wrappers, and documentation.
An additive coordinate wrapper is safer than mutating old serialized shapes.

### Add a maximum log-record size

The limit cannot be implemented only inside a policy parser. The file reader,
memory store, JSON parser, Python verifier, and replay retention behavior must
agree on whether the limit applies to raw bytes, decoded bytes, event count, or
the entire file. This is why RQ-007 calls for a declared resource law before a
local patch.

### Add policy v5

The explicit versioned modules make the policy decision visible, but
cross-version inherited standing, failure vocabularies, public re-exports,
fixtures, and docs must be updated together. Private shared mechanical
helpers can lower copy/paste cost; a dynamic “latest” abstraction would make
the resulting authority path less auditable.

## Findings ledger

| ID | Finding | Area | Severity | Confidence | State | Priority |
|---|---|---|---|---|---|---|
| RQ-001 | Public backend append bypasses the declared sole writer | API / authority | High | High | Current misuse hazard | P0 |
| RQ-002 | Unverified and verified events share a projection-compatible public type | API / replay | High | High | Current misuse hazard | P0 |
| RQ-003 | Legacy raw standing remains public and can look authoritative | Epistemic boundary | High | High | Current compatibility leak | P1 |
| RQ-004 | Episodic projection invents `Conjectured` for a tag with no status | Projection / provenance | Medium | High | Current semantic defect | P1 |
| RQ-005 | v2/v3 wildcard status matches can promote future variants | Policy compatibility | High | High | Latent extension defect | P1 |
| RQ-006 | Rust and Python accept different record languages | Determinism / tooling | High | High | Current interoperability defect | P0 |
| RQ-007 | Base log verification has no record/file resource bound | Resource / performance | Medium | High | Current availability debt | P1 |
| RQ-008 | FileStore has no durability, atomicity, locking, or snapshot contract | Storage / availability | Medium | High | Current operational debt | P1 |
| RQ-009 | Infallible Projection permits SQLite and range failures to panic | Error handling | Medium | High | Current availability defect | P1 |
| RQ-010 | Episodic schema reset can drop tables at an arbitrary public path | Storage safety | Medium | High | Current conditional data-loss hazard | P1 |
| RQ-011 | Detached legacy resolutions and search results lack full coordinates | Provenance / API | Medium | Medium | Current future-integration gap | P1 |
| RQ-012 | Versioned policy and parser duplication creates drift pressure | Maintainability | Medium | High | Current review-cost debt | P2 |
| RQ-013 | Test suite does not close cross-language, resource, crash, and exact-boundary gaps | Test assurance | Medium | High | Current assurance gap | P1 |
| RQ-014 | Design-document runtime status has drifted from landed code | Documentation | Medium | High | Current maintenance defect | P2 |
| RQ-015 | CI toolchain, actions, and Python crypto dependency are not pinned | Reproducibility / supply chain | Low-Medium | High | Current reproducibility debt | P2 |

## Detailed finding records

### RQ-001 — Public backend append bypasses the declared sole writer

- **Category:** API and authority boundary.
- **Affected surface:** LogStore::append_record, FileStore, MemStore, and the LogWriter backend seam.
- **Governing invariant:** LogWriter is the sole capability that mutates the signed append-only log.
- **Severity:** High; constitutional boundary risk, although normal verification
  still rejects most malformed append attempts.
- **Confidence:** High.
- **State:** Current misuse hazard.
- **Correction locus:** magpie-log/src/store.rs and the magpie-log root export boundary.
- **Evidence:** `CLAUDE.md:114`; `docs/adr/0001-deadbolt-seam.md:9-17`;
  `crates/magpie-log/src/store.rs:12-15,18-47,79-88`; the `LogStore` and
  stores are re-exported from `crates/magpie-log/src/lib.rs`.
- **Concrete scenario:** A caller obtains a mutable `FileStore` or `MemStore`
  and invokes the public trait method `append_record` with arbitrary bytes.
  This creates a second mutation capability that does not require constructing
  the event through `LogWriter`.
- **Impact:** A caller can poison availability, bypass writer-side event
  construction, and make the product-level “only write path” statement false.
  A caller with access to a trusted signing key can also create valid-looking
  records outside the intended writer lifecycle.
- **Current detection:** Chain verification catches broken sequence, links,
  hashes, signatures, and payload validity. It does not reject the existence
  of the backend capability or identify who performed the append.
- **Root cause:** The storage abstraction combines a public read interface with
  a public mutation method, and the trait method is necessarily callable by
  any holder of a mutable store.
- **Boundary interpretation:** Replay remains fail-closed for invalid bytes;
  this finding is about capability topology, not a claim that an arbitrary
  byte append becomes trusted automatically.
- **Minimal remediation:** Split read access from an internal writer-owned sink,
  seal/private the append capability, or make stores constructible only through
  a writer-owned backend adapter. Preserve the canonical format and normal
  `LogWriter` API.
- **Assurance gain:** The declared sole writer becomes mechanically inspectable;
  reviewers no longer need to trust every storage caller.
- **Effort band:** Medium; API migration and focused compile-fail tests.
- **Non-goals:** No generic authorization framework, network service, or new
  persistence engine.
- **Closure evidence:** A compile-fail or visibility test showing ordinary
  callers can read a store but cannot invoke its append sink, plus normal
  writer/replay tests.

### RQ-002 — Unverified and verified events share a projection-compatible public type

- **Category:** Replay, API, and projection authority.
- **Affected surface:** LogReader::events, SignedEvent, Projection::apply, ClaimsView, and StandingView.
- **Governing invariant:** Only a completely verified replay snapshot may feed an authority-bearing derived projection.
- **Severity:** High.
- **Confidence:** High.
- **State:** Current misuse hazard.
- **Correction locus:** magpie-log/src/logimpl.rs Projection boundary and claims compatibility projections.
- **Evidence:** `crates/magpie-log/src/logimpl.rs:350-360` documents
  `LogReader::events` as unverified but returns `Vec<SignedEvent>`;
  `:371-425` verifies and applies the same `SignedEvent` type through public
  `Projection::apply`; `crates/magpie-claims/src/lib.rs:156-232` and
  `standing.rs:187-692` expose manually applicable derived projections.
- **Concrete scenario:** A caller calls `reader.events()`, then feeds those
  values to `ClaimsView::apply` or `StandingView::apply`, or constructs a
  `SignedEvent` directly and applies it. The code compiles and produces a
  plausible derived state without the verified replay precondition.
- **Impact:** A non-authoritative or tampered view can be confused with a
  replay-derived view. This is especially risky because the same concrete event
  type is used by both trusted and untrusted paths.
- **Current detection:** Documentation warns against using `events()` as
  authority; verified replay tests prove the intended path. There is no type
  distinction or compile-fail test preventing the misuse.
- **Root cause:** The public trait and event type were optimized for a small
  generic projection API rather than a capability-typed verified event.
- **Boundary interpretation:** The current `replay_with_summary` implementation
  itself is sound at the logical verification step; the exposed API lets
  callers bypass it.
- **Minimal remediation:** Introduce a private/untrusted parsed event type and a
  verifier-created event token, or make direct projection application internal
  and expose only replay methods. Keep `SignedEvent` as a storage boundary if
  necessary.
- **Assurance gain:** Authority-bearing projections become reachable only from
  a completed verification path.
- **Effort band:** Medium to large because it touches the projection trait and
  compatibility views.
- **Non-goals:** Do not make projections dynamic or add a global authority
  registry.
- **Closure evidence:** A compile-fail misuse test and a positive one-snapshot
  replay test that consumes the verifier-created type.

### RQ-003 — Legacy raw standing remains public and can look authoritative

- **Category:** Epistemic boundary and compatibility API.
- **Affected surface:** StandingClaim.standing, StandingView public claim fields, and legacy status-change replay.
- **Governing invariant:** Historical raw status must not become current governed standing.
- **Severity:** High.
- **Confidence:** High.
- **State:** Current compatibility leak.
- **Correction locus:** magpie-claims/src/standing.rs public compatibility shape and accessors.
- **Evidence:** `crates/magpie-claims/src/standing.rs:99-108` has public
  `StandingClaim.standing`; `:273-349` returns
  `legacy_raw_standing` separately from `governed_standing`; `:661-692`
  assigns `claim.standing = *to` for legacy status changes; tests at
  `standing.rs:1837-1875` explicitly show a legacy status change can leave a
  typed v2 claim's public `standing` as `Settled` while governed resolution is
  `Conjectured`.
- **Concrete scenario:** A consumer reads `view.get(id).unwrap().standing`
  rather than calling the governed resolver and observes `Settled`. The
  resolver correctly returns `Conjectured`, but the public field is the easier
  path to use and serialize.
- **Impact:** A compatibility projection can be mistaken for current governed
  standing, defeating the intended `Supported` versus `Settled` and
  historical-record versus living-status distinction.
- **Current detection:** The quarantine test asserts the resolver result and
  the presence of a trace blocker. It does not prevent a caller from reading
  the raw field.
- **Root cause:** Legacy and typed claims share `StandingClaim`, with a public
  compatibility field retained for old consumers.
- **Boundary interpretation:** The resolver is fail-closed; the defect is the
  public authority-looking access path.
- **Minimal remediation:** Make the legacy field private or rename it into an
  explicit compatibility/raw type and expose governed status only through
  resolver methods. Keep old event replay semantics intact.
- **Assurance gain:** Callers must choose a clearly named historical/raw API
  instead of accidentally consuming it as living standing.
- **Effort band:** Medium; additive compatibility accessors and fixture updates.
- **Non-goals:** Do not convert legacy status events into governed authority.
- **Closure evidence:** A compile-fail test for direct raw-field access from the
  public API and a regression proving legacy events remain quarantined.

### RQ-004 — Episodic projection invents `Conjectured` for a tag with no status

- **Category:** Derived projection semantics and provenance.
- **Affected surface:** EpisodicView::payload_parts and EpisodicEvent.claim_status for ClaimAssertedV2.
- **Governing invariant:** A derived status field must identify a source-recorded status or an explicitly named derived value.
- **Severity:** Medium.
- **Confidence:** High.
- **State:** Current semantic defect.
- **Correction locus:** magpie-episodic/src/lib.rs payload_parts/schema output and episodic fixtures.
- **Evidence:** `crates/magpie-log/src/event.rs:25-45` defines the status
  vocabulary; `Payload::ClaimAssertedV2` is a typed registration event with no
  status field; `crates/magpie-episodic/src/lib.rs:263-338` maps that variant to
  `claim_status: Some("Conjectured")`.
- **Concrete scenario:** A timeline consumer reads `EpisodicEvent.claim_status`
  for a `claim_asserted_v2` row and treats `Conjectured` as a recorded status,
  even though the source event did not carry one.
- **Impact:** A derived search/timeline row can manufacture epistemic history
  and blur the distinction between assertion baseline and a later status
  transition.
- **Current detection:** Episodic tests cover row shape and regeneration but
  do not assert that every status-bearing output is source-present versus
  projection-derived.
- **Root cause:** The episodic schema uses one `claim_status` field for both
  legacy asserted status and a typed assertion baseline.
- **Boundary interpretation:** This is a derived view, not a new L0 authority,
  but downstream consumers commonly treat timeline fields as evidence.
- **Minimal remediation:** Keep `claim_status` as `None` for tag 6 or add a
  separately named derived field such as `asserted_baseline` with explicit
  documentation and fixtures.
- **Assurance gain:** Every status field has an inspectable source or an
  explicit derived label.
- **Effort band:** Small to medium; schema/output compatibility decision and
  focused tests.
- **Non-goals:** Do not make episodic output a standing authority.
- **Closure evidence:** A fixture showing the tag-6 row with no recorded status,
  plus a test distinguishing source status from derived baseline.

### RQ-005 — v2/v3 wildcard status matches can promote future variants

- **Category:** Versioned policy compatibility.
- **Affected surface:** standing_v2::combine_v2_standing and standing_v3::combine_standing.
- **Governing invariant:** Closed-vocabulary policy evolution must fail closed until a new status has an explicit policy meaning.
- **Severity:** High.
- **Confidence:** High.
- **State:** Latent extension defect; no fifth status variant exists today.
- **Correction locus:** magpie-claims/src/standing_v2.rs and standing_v3.rs inherited-standing matches.
- **Evidence:** `crates/magpie-claims/src/standing_v2.rs:300-309` uses
  `Some(_) if deterministic_supported`; `standing_v3.rs:885-893` uses
  `_ if corroborated`; v4 uses more explicit inherited-status arms in its
  candidate derivation.
- **Concrete scenario:** A future additive status such as `Withdrawn` or
  `Superseded` is added to the shared `Status` enum. A v2 deterministic
  candidate or v3 corroboration can then promote that unrecognized inherited
  status to `Supported` without a new v2/v3 policy decision.
- **Impact:** A vocabulary extension silently changes old policy semantics and
  can violate fail-closed standing evolution.
- **Current detection:** Current tests exercise the five existing variants but
  cannot exercise a nonexistent future variant. The wildcard also prevents
  compiler pressure at these two policy sites.
- **Root cause:** Wildcards combine “all other statuses” with a positive
  evidence condition.
- **Boundary interpretation:** This is not a current runtime result; it is a
  maintenance-time compatibility hazard that should be fixed before extending
  the enum.
- **Minimal remediation:** Match every current status explicitly and add a
  deliberate extension test or policy rejection arm. Preserve settled,
  refuted, and supported inheritance exactly as currently specified.
- **Assurance gain:** A new status requires an explicit policy review rather
  than inheriting positive semantics accidentally.
- **Effort band:** Small.
- **Non-goals:** Do not create a generic status promotion mechanism.
- **Closure evidence:** A source-level exhaustive match plus a test fixture for
  the chosen future-status behavior when the vocabulary is extended.

### RQ-006 — Rust and Python accept different record languages

- **Category:** Determinism, cross-language verification, and release tooling.
- **Affected surface:** Rust ContentHash/Sig/Status serde, FileStore JSONL handling, and tools/verify_chain.py.
- **Governing invariant:** Every verifier for the frozen profile must accept and reject the same record language.
- **Severity:** High.
- **Confidence:** High.
- **State:** Current interoperability defect.
- **Correction locus:** magpie-log serde/storage grammar and tools/verify_chain.py differential vectors.
- **Evidence:** Rust content-hash and signature deserializers call
  `hex::decode` at `crates/magpie-log/src/hashing.rs:48-54` and
  `event.rs:315-325`; Python `hx()` at `tools/verify_chain.py:57-60`
  requires lowercase; Python `status()` at `:49-55` accepts integers; Rust
  serde accepts named `Status`; `FileStore` filters empty lines at
  `store.rs:41-54`, while Python processes each line at roughly
  `verify_chain.py:214-237`.
- **Concrete scenario:** An event JSON with an uppercase signature/hash, a
  numeric status, or a blank line can be accepted by one verifier and rejected
  by the other.
- **Impact:** Golden fixtures can pass while a portable verifier disagrees on
  real records. A release or incident investigation can produce different
  chain verdicts depending on language.
- **Current detection:** The frozen golden and Deadbolt anchor fixtures pass
  both available paths. No adversarial acceptance-difference corpus is
  present.
- **Root cause:** The Rust serde/storage grammar and Python independent parser
  evolved separately without a complete negative differential corpus.
- **Boundary interpretation:** The canonical byte algorithm may still agree for
  the common language; the accepted JSON-language boundary is not singular.
- **Minimal remediation:** Choose and document one record grammar, then add
  cross-language vectors for case, numeric-vs-string status, blank lines,
  malformed JSON, and first-failure behavior. Make the Rust and Python
  implementations agree without changing frozen canonical bytes unless the
  format owner explicitly approves that.
- **Assurance gain:** Portable verification becomes a tested equivalence claim,
  not an assumption from one golden fixture.
- **Effort band:** Medium.
- **Non-goals:** Do not remove the independent verifier or replace the frozen
  format with a new encoding.
- **Closure evidence:** A shared or duplicated negative corpus passing in both
  directions, with exact expected exit/error classification.

### RQ-007 — Base log verification has no record/file resource bound

- **Category:** Resource safety and performance.
- **Affected surface:** FileStore::read_records, parse_and_verify_records, and retained replay event storage.
- **Governing invariant:** Hostile or extreme input must be bounded before whole-file and whole-event materialization.
- **Severity:** Medium.
- **Confidence:** High.
- **State:** Current availability debt.
- **Correction locus:** magpie-log/src/store.rs outer reader and logimpl.rs retention/replay boundary.
- **Evidence:** `crates/magpie-log/src/store.rs:41-54` calls
  `std::fs::read` and collects every split line; `logimpl.rs:33-34,113-122`
  passes a complete `Vec<Vec<u8>>` into parsing and retains parsed events for
  replay at `:375-386`; policy-specific limits appear in claims modules but
  do not bound the base store.
- **Concrete scenario:** A caller points `FileStore` at a very large file or a
  single oversized JSON line. The reader allocates the full file, line copies,
  parsed strings, and replay event vector before policy-level limits can reject
  any nested object.
- **Impact:** Peak memory is proportional to the entire input and can cause
  excessive latency, allocation failure, or process termination. A hostile
  malformed record is not rejected before all of its raw bytes are
  materialized.
- **Current detection:** Foreign bundle, subject, closure-entry, and object
  limits have focused hostile tests. There is no base-log file-size,
  record-size, record-count, or streaming test family.
- **Root cause:** `LogStore::read_records` is a whole-snapshot convenience API
  and the resource law is not defined at the L0 boundary.
- **Boundary interpretation:** Existing parser bounds are valuable but do not
  compose into a bound on the outer log reader.
- **Minimal remediation:** Define explicit maximum file/record/count semantics
  and enforce them before materializing unbounded values, or add a bounded
  streaming/frame reader with a clear snapshot law. Keep rejection fail-closed.
- **Assurance gain:** Memory and failure behavior become predictable for
  untrusted or merely large local logs.
- **Effort band:** Medium to large; storage API and fixture work.
- **Non-goals:** Do not silently truncate records or perform partial replay.
- **Closure evidence:** Boundary tests for zero, exact-limit, over-limit,
  multibyte/escaped values, truncated frames, and a bounded allocation/read
  behavior test.

### RQ-008 — FileStore has no durability, atomicity, locking, or snapshot contract

- **Category:** Filesystem storage and operational availability.
- **Affected surface:** FileStore append/read behavior and concurrent writer/reader interaction.
- **Governing invariant:** A successful append must have a declared durability, framing, writer-ownership, and snapshot meaning.
- **Severity:** Medium.
- **Confidence:** High.
- **State:** Current operational debt.
- **Correction locus:** magpie-log/src/store.rs FileStore contract and backend implementation.
- **Evidence:** `crates/magpie-log/src/store.rs:17-47` describes a durable
  append-only file but only opens with append and performs `write_all` twice;
  there is no `sync_all`, lock, atomic frame, or writer identity. Reads use
  the whole current file with no captured file version.
- **Concrete scenario:** A process crashes between the two writes or during a
  filesystem flush, leaving a partial JSON line. Two processes append
  concurrently, or a reader observes the file while a writer is appending.
  Reopen/replay can then fail or observe an undocumented snapshot.
- **Impact:** Availability and recoverability are undefined at the storage
  boundary. The log can be logically immutable after a successful append yet
  operationally torn.
- **Current detection:** Tamper and chain-integrity tests catch many logical
  corruptions. There are no process-crash, concurrent-writer, fsync, or
  partial-frame tests.
- **Root cause:** JSON-lines storage was implemented as a minimal local backend
  without a stated filesystem contract.
- **Boundary interpretation:** This is separate from canonical correctness:
  a correct hash cannot repair a torn append.
- **Minimal remediation:** Either document a strict single-process durability
  contract and reject unsupported use, or add explicit single-writer locking,
  atomic framing/recovery, sync semantics, and a reader snapshot boundary.
- **Assurance gain:** Operators know whether a successful append means durable,
  recoverable state and what concurrent access does.
- **Effort band:** Medium.
- **Non-goals:** Do not introduce a general distributed storage layer.
- **Closure evidence:** Crash/torn-write fixtures where feasible, a concurrent
  access policy test, and an explicit durability statement verified by the
  backend implementation.

### RQ-009 — Infallible Projection permits SQLite and range failures to panic

- **Category:** Error handling and replay availability.
- **Affected surface:** Projection::apply and EpisodicView SQLite transactions/conversions.
- **Governing invariant:** Derived-index failure must be typed and distinguishable from successful verified log replay.
- **Severity:** Medium.
- **Confidence:** High.
- **State:** Current availability defect.
- **Correction locus:** magpie-log/src/logimpl.rs projection contract and magpie-episodic/src/lib.rs failure adapter.
- **Evidence:** `Projection::apply` is infallible at
  `crates/magpie-log/src/logimpl.rs:419-425`. `EpisodicView::apply` calls
  `expect` for transaction, insert, FTS insert, and commit at
  `crates/magpie-episodic/src/lib.rs:213-250`; range conversion panics at
  `:390-398`.
- **Concrete scenario:** A replay hits disk-full, a damaged SQLite file, a
  constraint/FTS error, or an event sequence/timestamp above signed i64.
  `replay` calls `projection.apply` and the process panics instead of returning
  a typed projection failure.
- **Impact:** A derived-index failure can take down a host and can obscure the
  distinction between source-log verification success and projection rebuild
  failure.
- **Current detection:** Normal in-memory SQLite tests and regeneration tests
  pass. No injected SQLite failure or overflow event is exercised.
- **Root cause:** The generic projection trait was designed as an infallible
  fold, while the episodic backend performs fallible I/O and conversion.
- **Boundary interpretation:** The derived index is not authority, but a
  non-authoritative failure should still be reported without process panic.
- **Minimal remediation:** Add a fallible projection/replay adapter or an
  explicit error-reporting wrapper for operational projections. Keep the log
  verification result separate from index rebuild status.
- **Assurance gain:** Rebuild failures become diagnosable and recoverable.
- **Effort band:** Medium.
- **Non-goals:** Do not blanket-catch panics or make a failed projection look
  successfully rebuilt.
- **Closure evidence:** Injected SQLite failure and signed-range tests with
  typed errors and no partial-success claim.

### RQ-010 — Episodic schema reset can drop tables at an arbitrary public path

- **Category:** Storage safety and public constructor behavior.
- **Affected surface:** EpisodicView::at_path and from_connection schema-version reset.
- **Governing invariant:** Opening caller-supplied derived storage must not silently destroy existing data.
- **Severity:** Medium.
- **Confidence:** High.
- **State:** Current conditional data-loss hazard.
- **Correction locus:** magpie-episodic/src/lib.rs at_path/from_connection schema ownership or migration boundary.
- **Evidence:** `EpisodicView::at_path` is public at
  `crates/magpie-episodic/src/lib.rs:96-102`; `from_connection` checks
  `PRAGMA user_version` and, when it differs, executes
  `DROP TABLE IF EXISTS events_fts` and `DROP TABLE IF EXISTS events` at
  `:104-120`.
- **Concrete scenario:** A caller points `at_path` at an existing SQLite
  database with a different user version or an old Magpie-derived schema.
  Construction drops the named tables before rebuilding the current schema.
- **Impact:** Existing derived data is destroyed without an explicit migration
  or ownership marker. The source log may preserve recoverability, but the
  public constructor still performs a destructive operation on caller-supplied
  storage.
- **Current detection:** Schema tests cover expected initialization, not
  arbitrary existing database contents and preservation behavior.
- **Root cause:** Rebuild-from-log semantics were implemented as unconditional
  schema replacement for version mismatch.
- **Boundary interpretation:** “Derived” limits epistemic authority; it does
  not make filesystem/database deletion harmless to an embedding application.
- **Minimal remediation:** Require an owned/dedicated database, use a
  namespaced schema/marker, or provide an explicit migration/reset operation
  instead of silently dropping on public construction.
- **Assurance gain:** Callers can reason about data preservation before opening
  a path.
- **Effort band:** Small to medium.
- **Non-goals:** Do not persist authority outside the append-only log.
- **Closure evidence:** A pre-existing database fixture with unrelated and old
  tables, plus an assertion that ordinary open preserves or explicitly refuses
  it.

### RQ-011 — Detached legacy resolutions and search results lack full coordinates

- **Category:** Provenance and output API.
- **Affected surface:** StandingResolution compatibility outputs, EpisodicEvent, and EpisodicView::search results.
- **Governing invariant:** Detached derived results need verified prefix/policy/closure/query coordinates or an explicit non-authority label.
- **Severity:** Medium.
- **Confidence:** Medium.
- **State:** Current future-integration gap.
- **Correction locus:** additive claims output wrappers and episodic query-result coordinate types.
- **Evidence:** `StandingResolution` in
  `crates/magpie-claims/src/standing.rs:76-97` has claim, policy, status, and
  trace fields but no verified prefix identity or closure identity. The
  newer replay snapshot keeps those coordinates privately. `EpisodicEvent` at
  `crates/magpie-episodic/src/lib.rs:41-56` carries row data only, and
  `search` at `:154-177` returns sequence numbers without query or snapshot
  identity.
- **Concrete scenario:** A resolution or search result is serialized, sent to
  another process, or stored in a report. A later consumer cannot tell which
  verified prefix, policy input, closure, or exact query produced it.
- **Impact:** Detached outputs are difficult to reproduce, compare, or audit.
  They can be mistaken for current truth even when they are only historical
  derived views.
- **Current detection:** v4 and replay-context tests cover private prefix and
  closure identities; compatibility output tests do not require the same
  coordinates.
- **Root cause:** Older output shapes predate the replay/provenance coordinate
  seam and were preserved for compatibility.
- **Boundary interpretation:** This is not a claim that every legacy type must
  be rewritten; the risk is unlabelled detachment.
- **Minimal remediation:** Add an additive coordinate wrapper for new APIs or
  mark old outputs explicitly non-detachable and require a replay context for
  authority-bearing use.
- **Assurance gain:** Consumers can distinguish a local derived view from a
  reproducible verified result.
- **Effort band:** Medium.
- **Non-goals:** Do not make projection bytes equal to log-prefix identity.
- **Closure evidence:** A serialized result fixture with prefix/policy/query
  coordinates and a test that detached legacy output is rejected or clearly
  labelled.

### RQ-012 — Versioned policy and parser duplication creates drift pressure

- **Category:** Maintainability and review complexity.
- **Affected surface:** standing v0-v4, provenance verifiers, parser visitors, and audit composers.
- **Governing invariant:** Mechanical reuse should reduce drift without merging independent policy decisions.
- **Severity:** Medium.
- **Confidence:** High.
- **State:** Current review-cost debt.
- **Correction locus:** private mechanical helpers and source/fixture inventory checks across claims modules.
- **Evidence:** The largest production modules are
  `support_contribution_audit.rs` (2,943 lines), `standing_v4.rs` (2,711),
  `origin_binding_verifier.rs` (2,473), `standing.rs` (2,345), and
  `standing_v3.rs` (2,260). The source-only public syntax inventory is 570
  lines. Multiple modules hand-code parser validation, canonical output, trace
  order, and inherited standing decisions.
- **Concrete scenario:** A new policy coordinate or failure reason is added to
  one version but omitted from a sibling policy, fixture, Python verifier, or
  documentation checklist. The code compiles because each version is
  intentionally separate.
- **Impact:** Review time and regression risk grow superlinearly with every
  versioned policy surface. A missed duplicate update can cause silent
  cross-version drift.
- **Current detection:** Large focused test families and explicit policy
  functions catch many current omissions. There is no inventory check that
  compares versioned coordinate sets or failure vocabularies.
- **Root cause:** Explicit policy isolation was chosen over a generic policy
  engine, but mechanical repetition is not consistently factored.
- **Boundary interpretation:** The duplication is partly a safety feature; the
  remediation must lower mechanical cost without hiding policy meaning.
- **Minimal remediation:** Factor only private, behavior-identical helpers for
  framing, validated coordinate access, and deterministic collection handling.
  Add source/fixture inventories for versioned public fields and failure
  variants.
- **Assurance gain:** Fewer mechanical omission points while retaining visible
  policy branch sites.
- **Effort band:** Medium, staged opportunistically.
- **Non-goals:** No dynamic registry, “latest” selector, macro-generated policy
  semantics, or broad architecture rewrite.
- **Closure evidence:** A before/after review-surface comparison and a test
  proving the shared helper cannot select policy or authority.

### RQ-013 — Test suite does not close cross-language, resource, crash, and exact-boundary gaps

- **Category:** Test assurance.
- **Affected surface:** Negative tests, compile-fail doctests, cross-language fixtures, storage failure paths, and resource boundaries.
- **Governing invariant:** A weakening of each high-value authority or reliability guarantee must fail at the intended test boundary.
- **Severity:** Medium.
- **Confidence:** High.
- **State:** Current assurance gap.
- **Correction locus:** Rust tests/doctests, Python differential fixtures, and storage/projection failure tests.
- **Evidence:** `cargo test --workspace --locked --no-fail-fast` reports 281
  passes, but inventory found no mutation, fuzz, differential malformed-record,
  file-crash, concurrent-writer, log-size, or SQLite-failure suite. Compile-
  fail examples in `origin_binding_verifier.rs`, `origin_admission_audit.rs`,
  and `support_contribution_audit.rs` include stale-arity substitutions.
- **Concrete scenario:** A future implementation changes uppercase hex
  acceptance, adds a receipt-shaped argument, removes a resource guard, or
  changes projection error handling. The current green suite can remain green
  while the intended negative boundary is gone or the new behavior is only
  caught in production.
- **Impact:** Confidence is high for the tested current examples but low for
  several hostile and operational dimensions.
- **Current detection:** Existing negative tests are strong for many policy
  matrices, malformed metadata, bounds, and private construction. They do not
  cover the listed seams.
- **Root cause:** Test effort is concentrated on current policy contracts and
  deterministic fixtures rather than outer storage, portability, and
  operational failure contracts.
- **Boundary interpretation:** This finding does not discount the 281 passing
  tests; it states what those tests do not establish.
- **Minimal remediation:** Add a narrow matrix for cross-language acceptance,
  raw append/projection misuse, exact compile-fail diagnostics, resource
  boundaries, and injected projection/storage failures. Add mutation testing
  only where it can be bounded and repeatable.
- **Assurance gain:** Green CI more closely implies preservation of the actual
  architecture contract.
- **Effort band:** Medium.
- **Non-goals:** Do not inflate test counts with redundant snapshot tests or
  turn every historical design statement into a runtime test.
- **Closure evidence:** Named tests mapped to each P0/P1 finding and at least
  one negative test that fails for the intended forbidden surface.

### RQ-014 — Design-document runtime status has drifted from landed code

- **Category:** Documentation and maintenance.
- **Affected surface:** Current-status headers and handoff language in the v3, origin-admission, and attestation design notes.
- **Governing invariant:** Documentation must identify the current enforcing owner without rewriting historical contract meaning.
- **Severity:** Medium.
- **Confidence:** High.
- **State:** Current maintenance defect.
- **Correction locus:** narrow current-status handoff blocks in the affected design notes.
- **Evidence:** `docs/design/standing-policy-v3-external-report-corroboration-v0.md:5-12`
  says runtime is not implemented, while `crates/magpie-claims/src/standing_v3.rs`
  has the runtime and tests. `docs/design/origin-admission-audit-v0.md:5-16`
  says origin admission remains future, while
  `origin_admission_audit.rs` and `origin_admission_replay.rs` are present.
  The attestation note at `:6-9` uses “current candidate patch” handoff
  language that is stale at this merge commit.
- **Concrete scenario:** A maintainer reads the contract as future and adds a
  second runtime path, or assumes a code path is unavailable and omits it from
  a security review.
- **Impact:** Review and sequencing errors; possible duplicate policy logic or
  incorrect release claims.
- **Current detection:** The README and source inspection reveal the drift.
  There is no status consistency check between design headers, current module
  presence, and the release/roadmap ledger.
- **Root cause:** Historical contract text was retained across multiple landed
  tickets without one current status index.
- **Boundary interpretation:** Historical wording should remain frozen where it
  records what was true at ratification; current status needs an additive
  handoff rather than silent historical rewriting.
- **Minimal remediation:** Add a narrow current-status ledger or status block
  for landed runtime, future runtime, withdrawn policy, and historical
  contract language. Update only the stale status references.
- **Assurance gain:** Reviewers can identify the governing current path without
  rewriting doctrine.
- **Effort band:** Small.
- **Non-goals:** Do not alter policy semantics or expand the roadmap.
- **Closure evidence:** A docs consistency review showing each landed runtime
  has one current status and historical text remains labelled.

### RQ-015 — CI toolchain, actions, and Python crypto dependency are not pinned

- **Category:** Reproducibility and supply-chain assurance.
- **Affected surface:** CI Rust toolchain, GitHub Action revisions, and Python cryptography installation.
- **Governing invariant:** Validation results must be reproducible from recorded toolchain and dependency inputs.
- **Severity:** Low-Medium.
- **Confidence:** High.
- **State:** Current reproducibility debt.
- **Correction locus:** .github/workflows/ci.yml and a recorded toolchain/Python dependency policy.
- **Evidence:** `.github/workflows/ci.yml:13-18` installs mutable `stable`;
  `:13,23` uses `actions/checkout@v4` and `actions/cache@v4`; `:52` installs
  the latest `cryptography` without a version constraint. Cargo dependencies
  are locked for the Rust build, but the external verifier dependency is not.
- **Concrete scenario:** A future stable compiler, action implementation, or
  Python cryptography release changes diagnostics, parsing, or signature
  behavior. CI passes or fails for reasons not reproduced by the audited local
  toolchain.
- **Impact:** Reduced repeatability and weaker provenance for release
  validation. This is not evidence that the current dependency is unsafe.
- **Current detection:** Local exact commands pass and Cargo.lock constrains
  Rust resolution. CI does not record a pinned toolchain or Python dependency
  set.
- **Root cause:** CI follows moving “latest stable” and action-major
  conventions for convenience.
- **Boundary interpretation:** Mature crypto and bundled SQLite are reasonable
  dependencies; the concern is version provenance, not a recommendation to
  hand-roll either.
- **Minimal remediation:** Pin or centrally record the Rust toolchain, Python
  verifier dependency, and action revisions according to repository policy.
  Add a reproducible CI metadata report.
- **Assurance gain:** A passing release check can be reproduced from recorded
  inputs.
- **Effort band:** Small.
- **Non-goals:** Do not remove cryptography, replace Ed25519, or add a network
  runtime dependency.
- **Closure evidence:** CI log identifies exact toolchain/action/Python
  versions and a clean rerun reproduces the local validation result.

## Rejected or narrowed concerns

The following were considered but are not promoted to confirmed findings in
this ledger:

- **Weak Ed25519 key acceptance and strict signature edge cases:** the prior
  architecture audit raised these as conditional concerns. This audit did not
  perform a mutation or dedicated cryptographic adversarial experiment, so
  they remain review follow-ups rather than current confirmed defects.
- **Every `expect` is a bug:** rejected. Serialization of types with fixed
  internal invariants and impossible enum branches can reasonably use
  assertions. The episodic I/O and numeric conversion panics are specific
  enough to record as RQ-009.
- **No unsafe code:** confirmed as a positive observation, not a release
  certificate. The two source hits were the word “unsafe” in panic messages.
- **The prior architecture audit's complete governance ledger:** not repeated
  wholesale. Its historical findings remain useful context; this document
  focuses on runtime quality, API topology, determinism, resource behavior,
  and test assurance at the current local commit.
- **Origin authority falsification as a newly reproduced defect:** not claimed
  here because the current audit did not run a new hostile mutation. The
  existing provenance/authority boundary remains part of the recommended
  reviewer checks.
- **Undetected semantic mutation:** none was experimentally demonstrated.
  No source mutation was applied because the audit was required to remain
  non-mutating.

## Prioritised remediation programme

### P0 — close before authority-bearing release work

1. Close the second write capability (RQ-001).
2. Separate unverified parsed data from verified replay application (RQ-002).
3. Choose one Rust/Python record grammar and add negative differential vectors
   (RQ-006).

These three items determine whether a caller can accidentally bypass the
declared source-of-truth and verifier boundaries. They should be reviewed
together, with no format change unless explicitly approved.

### P1 — close before hostile or durable local deployment

1. Quarantine or rename public legacy standing (RQ-003).
2. Remove invented episodic status or label it as derived baseline (RQ-004).
3. Make future status extension fail closed in v2/v3 (RQ-005).
4. Define outer log resource limits and a bounded reader (RQ-007).
5. Define FileStore crash/durability/concurrency semantics (RQ-008).
6. Return typed projection failures rather than panicking (RQ-009).
7. Make public episodic schema reset non-destructive or explicitly opt-in
   (RQ-010).
8. Add provenance coordinates or explicit non-detachable labels to legacy
   outputs and search results (RQ-011).
9. Add focused tests for each boundary (RQ-013).

### P2 — reduce future review and release risk

1. Factor only mechanical private helpers and add versioned-surface inventories
   (RQ-012).
2. Add a current status handoff without rewriting historical contract prose
   (RQ-014).
3. Pin CI toolchain/action/Python inputs (RQ-015).

## Provisional release/readiness decision

| Use | Decision | Conditions |
|---|---|---|
| Local development and policy experimentation | Conditional | Use `LogWriter` plus verified replay contexts; do not use `events()` or manually applied public projections as authority |
| Small trusted single-process demonstrations | Conditional | Treat FileStore as a crash-sensitive local backend and keep logs bounded by operator policy |
| Authority-bearing production | Not ready | P0 boundary and grammar findings remain open; P1 resource/error/storage findings also matter |
| Release-quality claim for this commit | Not approved | The audit point is behind `origin/main`, and the quality contract is not closed by the passing test suite |

This is a quality/readiness decision for the audited local commit, not a claim
about the nine newer remote commits.

## Exact validation commands and observed results

These commands were run against the audited local commit. The requested audit
file was created after the implementation validation; no implementation file
changed between the validation and the ledger creation.

| Command | Result |
|---|---|
| `cargo test --workspace --locked --no-fail-fast` | PASS; 281 Rust unit/integration tests reported passed, zero failed; compile-fail doctest groups also passed |
| `cargo fmt --all --check` | PASS; no output |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | PASS |
| `python tools/verify_chain.py crates/magpie-log/testdata/golden-v1.jsonl ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c` | PASS; records 0 through 8 all `OK` |
| `python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737` | PASS; records 0 and 1 all `OK` |
| `python tools/check_release_metadata.py` | PASS; inventories reported as magpie-log 19, magpie-claims 48, magpie-episodic 9 |
| `cargo run --locked --example tour -p magpie-claims` | PASS; 14 events verified and regenerated snapshot/resolution bytes were identical |
| `cargo package -p magpie-log --locked` | PASS; 19-file package, 142.9 KiB, compilation verification passed |
| `git diff --check` | PASS |

The package command contacted the configured Cargo index as part of normal
Cargo behavior. It produced no tracked repository change.

Not run, and therefore not claimed as passed:

- `cargo audit` or `cargo deny`;
- fuzzing, mutation testing, sanitizer, race, crash-recovery, or concurrent
  writer testing;
- a cross-language malformed-record differential corpus;
- a full clean-clone validation of `origin/main`;
- a dedicated weak-key/strict-signature cryptographic adversarial experiment;
- a Python portability matrix across versions and operating systems.

## Final worktree cleanliness

At the final audit point:

- tracked files remained unchanged;
- the requested root ledger is the only new file created by this audit;
- the pre-existing untracked
  `docs/audits/magpie-architecture-audit-2026-08-01.md` remains untouched;
- generated Cargo build/package outputs remain ignored under `target`;
- no commit, branch, PR, issue, remote ref, or other external state was
  changed.

The worktree is therefore **tracked-clean but overall unclean by design**:
the status contains the pre-existing untracked audit directory and this
requested untracked root ledger.

## Direct answers to the audit questions

1. **What is the actual runtime quality of the current main point?**  
   Strong on current canonical/policy examples and weak at several outer
   authority, portability, resource, and operational boundaries. It is
   conditionally useful locally, not release-ready for authority-bearing use.

2. **Are replay and provenance boundaries mechanically enforced?**  
   The verified one-snapshot replay path is good, and newer receipt paths are
   private-construction. The public backend append, shared event type, mutable
   projections, and legacy raw fields leave material bypass/misuse seams.

3. **Is the implementation deterministic?**  
   The common frozen vector is deterministic. Rust and Python do not yet
   accept exactly the same serialized language, so portability is conditional.

4. **Does the implementation fail closed under malformed or hostile input?**  
   Many bounded policy parsers do. The outer log reader is unbounded, and
   episodic operational failures can panic.

5. **Are errors typed and recoverable?**  
   L0 verification errors are typed. The infallible projection trait forces
   episodic SQLite and range failures into panics.

6. **Is the dependency footprint proportionate?**  
   Yes at runtime: seven direct workspace dependencies and 59 locked package
   stanzas are proportionate. CI version pinning is still weak.

7. **Is the test suite sufficient for a release claim?**  
   It is substantial and all 281 reported tests pass, but it lacks the
   cross-language, resource, filesystem, concurrency, and exact compile-fail
   coverage needed for the declared boundaries.

8. **What is the smallest safe remediation shape?**  
   Close the three P0 seams first, then add outer resource/storage/error
   contracts and quarantine authority-looking compatibility fields. Keep
   versioned policy decisions explicit and additive.

9. **What should a reviewer verify next?**  
   The P0 type/capability changes, differential grammar vectors, future-status
   fail-closed behavior, resource law, FileStore crash semantics, and that
   historical documentation remains clearly historical.

10. **Were changes beyond the requested audit made?**  
    No. Only this requested ledger was added; no implementation or external
    repository state was changed.

## Final judgment — required questions

1. **Is the current implementation appropriately small for its claimed kernel?**  
   The L0 log crate is small and focused, but the full claimed kernel is not
   yet appropriately small for its guarantees: 24,323 production Rust lines
   include several necessary explicit policy contracts and several accidental
   API, compatibility, and duplicated-mechanism costs. The judgment is
   conditionally proportional, not release-ready.

2. **Which modules contribute the most trusted-computing-base risk?**  
   The highest-risk surfaces are magpie-log store/logimpl for write and
   verification capability topology; claims standing.rs and standing_v2-v4
   for status/policy semantics; origin/provenance verifier modules for
   receipt and closure boundaries; and tools/verify_chain.py plus CI for
   independent-language and release acceptance. Episodic is primarily a
   derived availability risk, not an authority source.

3. **Which complexity is essential, and which is accidental?**  
   Essential complexity is the explicit frozen canonical encoder, hostile
   parser precedence, private replay-context construction, and separate
   versioned policy decisions. Accidental complexity is the public raw/write
   seams, shared raw/verified event type, repeated mechanical validation and
   trace scaffolding, unbounded storage convenience API, and stale status
   handoffs.

4. **Can an unfamiliar competent Rust engineer safely modify the system using the code and tests alone?**  
   Not reliably. The code and tests communicate many current invariants, but
   safe modification still requires expert knowledge of caller-only authority
   rules, historical-versus-living standing, Rust/Python grammar differences,
   and which compile-fail examples fail for the intended reason.

5. **Which architectural guarantees are enforced structurally, and which still depend on disciplined callers?**  
   Structural enforcement includes explicit canonical payload matching,
   deterministic ordered maps, private newer receipt fields, and the
   verify-then-replay snapshot sequence. Disciplined callers are still needed
   for the sole-writer rule, avoiding LogReader events for authority, avoiding
   public raw standing, treating detached outputs as non-authoritative, and
   using FileStore as a single-process crash-sensitive backend. The external
   trust root is intentionally caller-supplied.

6. **What are the five highest-value reductions in code, API surface, duplication, or ambiguity?**  
   1. Remove or seal the public backend append sink.
   2. Split raw parsed events from verifier-created projection input.
   3. Quarantine public legacy standing and remove the invented episodic status.
   4. Declare one shared Rust/Python record grammar instead of carrying two
      acceptance languages.
   5. Factor only identical private mechanical helpers and add one current
      runtime-status ledger, reducing duplicated review and documentation
      ambiguity without merging policy decisions.

7. **What are the five highest-value additions to testing or verification?**  
   1. A bidirectional negative Rust/Python grammar corpus.
   2. Compile-fail tests for raw append, unverified projection, and direct raw
      standing access that fail for the intended API reason.
   3. Bounded log-size, record-size, count, escape, and truncation tests.
   4. Injected SQLite failure/range tests plus crash/torn-write and
      single-writer/concurrency contract tests.
   5. Targeted mutation checks for v2/v3 wildcard promotion, episodic
      synthetic status, and legacy standing quarantine.

8. **Is the codebase presently a research prototype, a robust pre-alpha kernel, or something stronger?**  
   It is a robust pre-alpha kernel: substantially beyond a happy-path
   research prototype in canonicalization, replay, hostile policy tests, and
   provenance seams, but below a dependable production epistemic kernel.

9. **What must be true before adding substantial new runtime capability?**  
   P0 write, verified-input, and cross-language grammar boundaries must be
   closed; the outer log resource and FileStore failure contracts must be
   explicit; projection errors must be typed; authority-looking compatibility
   outputs must be quarantined; and each new capability must have a mapped
   negative-test matrix and current documentation status.

10. **Did the audit establish any reason to stop feature work immediately?**  
    It did not establish a need to stop all local research work. It did
    establish a gate against adding substantial authority-bearing runtime
    capability until RQ-001, RQ-002, and RQ-006 are resolved and the relevant
    P1 assurance contracts are tested.
