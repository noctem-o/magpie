# Changelog

This file records repository release milestones. Crate semantic versions,
canonical format identities, and governed policy identities are separate
commitments; changing one does not silently rename or mutate the others.

## Unreleased

### Status history moved from CLAUDE.md

Until 2026-09-24, `CLAUDE.md` carried a running queue and status log that
every agent session loaded. It moved here unchanged, below, so `CLAUDE.md` can
hold rules and a short "Now" list. The C3–C5 queue it records is paused, not
dropped.

1. C3 decision — only if the advertised pre-alpha requires detachable governed
   output, add the smallest coordinate-complete verification/replay result or
   receipt; otherwise preserve the explicit exclusion.
2. C4 — harden the applicable public assurance boundary: pinned validation
   inputs, platform evidence, and release-environment reproducibility.
3. C5 — write the public pre-alpha release contract, freeze a candidate, and
   falsify it before an owner release decision.

Keep general ingestion/CAS, `EpistemicGate`, governed ordinary agent writing,
contradiction/currentness runtime, ADR-0007 authority runtime, librarian,
effectful MCP, production KMS, and ambient/Vesper agent work deferred unless a
separate owner decision moves one into the release boundary.

Landed on current `main`: PR #146's non-test public
`FileStore::verify_portable_history` acquires exact file bytes once after the
complete selected-profile/external-key gate and invokes the authoritative
portable semantics without legacy `LogStore::read_records` splitting. Blank
physical records and newline distinctions are preserved. After successful key
preflight, missing files remain operational I/O failures; an inadmissible key
returns governed rejection before acquisition, and an existing zero-byte file
is the governed empty snapshot. The complete result is computed internally
from the same call;
no public typed receipt/result was added. The independent Python reference is
`tools/verify_chain.py` itself; it remains separate from the comparison-only
`tools/portable_verifier.py`. The standalone Go conformer still uses only its
authorized vendored `filippo.io/edwards25519 v1.2.0` dependency, and the
selected-root differential compares `verdict`, `class`, `line`,
`record_index`, `event_count`, `tip`, and `ordered_recomputed_hashes` for the
exact frozen 432 cases with isolated Python execution and repeat-two stability.
This is bounded selected-profile evidence, not trust, authority, universal
verifier equivalence, release readiness, or a release decision.

Administrative reconciliation after owner-merged PR #146: A-004, RQ-006, and
A-021 are Closed narrowly for the fixed portable language/profile after the
contract-qualified Rust/Python/Go evidence, hostile review, hosted CI, owner
merge, and separate ledger reconciliation. RQ-013 remains Partially
remediated with A-018 exact-boundary, broader platform/resource/crash, and
release/environment assurance scope open. RQ-015 remains Confirmed under its
controlled reproducibility disposition; A-011 remains Partially remediated.
Closure does not establish trust in the caller-supplied external key, key
ownership, identity, delegation, authority, permission, freshness, latest head,
canonical branch, anti-rollback, non-equivocation, global completeness,
production durability, truth, cryptographic proof, or release approval.
Historical audit records and the dated pre-alpha candidate assessment remain
untouched.

Landed: the complete crate-internal Rust portable-history conformer applies the
six ordered post-schema stages through one crate-internal implementation path
and produces the complete count/tip/hash trace used by all 432 frozen cases
(PR #139, 2026-08-24). PR #146 now reaches that authoritative semantics from
the real public file path; the crate-internal result boundary remains intact.
This remains a caller-supplied finite-history check, not trust, authority,
currentness, or global-completeness evidence.

Landed: bounded non-normative Rust fuzz/metamorphic assurance over the complete
crate-internal conformer, including focused hostile repairs (PR #140,
2026-08-25).
It is assurance evidence, not a normative oracle or proof of correctness.

Landed: the authoritative crate-internal Rust portable raw-input frontend through
`Schema`, including exact profile/external-key preflight, exact-byte framing and
UTF-8, the locked `serde_json` syntax seam, duplicate-preserving closed decoding,
and a private pending-record/exclusive-continuation boundary (PR #137,
2026-08-24). It proves 324 frontend-tranche-final outcomes and 108 later-stage
non-preemption cases, not complete history conformance or finding closure.

Landed: typed, explicit, fail-closed Rust `V_sig` profile selection and the exact
ADR-0010 relation as the namespaced `ProfiledSignatureVerifier` primitive; the
legacy unprofiled verifier remains unchanged (PR #135, 2026-08-24). This is a
cryptographic primitive landing, not complete portable conformance or finding
closure.

Landed: SQLite + FTS5 episodic projection (`magpie-episodic`, PR #5, 2026-07-02).
Landed: Deadbolt seam ratified as anchored hierarchy — ADR-0001, `SegmentAnchored`
tag 5, FORMAT.md §3 addition, extended golden vectors (six events, seqs 0-4
unchanged), tag-5 Python verifier (PR #8, 2026-07-03).
Landed: Deadbolt-side seam complete (deadbolt PRs #316–#320, 2026-07-03..05):
`cog-anchor` sole-writer crate; kernel, observation, codelet, and transaction
seal points emit fail-open with `pending-anchor.json` debt markers; `cog anchor
reconcile` re-verifies bundles then anchors idempotently via the `AnchorSet`
projection — the first consumer of anchors.

## 0.1.0 — 2026-07-12

Magpie v0.1.0 establishes a local-first, provenance-first memory core whose
signed append-only history is verified before deterministic replay. Typed
evidence remains candidate-only under policy v0. One exact Deadbolt
occurrence/inclusion proposition can earn `Settled` under explicit
snapshot-only policy v1. All other general achieved-standing, admission,
aggregation, writer, and production capabilities remain outside this release.

### Added

- `magpie-log`: a signed, hash-chained append-only L0 with automatic genesis,
  an externally supplied trust root, frozen `magpie-core-v1` canonical bytes,
  domain-separated `magpie-sig-v1` signatures, golden vectors, an independent
  Python verifier, and complete verified-before-apply replay.
- Deterministic derived projections: compatibility `ClaimsView`, typed
  `StandingView`, same-replay `StandingReplaySnapshot`, exact
  `DeadboltAnchorIndex`, and SQLite/FTS5 `EpisodicView`.
- A portable, protocol-defined Deadbolt anchor seam that records exact
  occurrence/inclusion evidence without making Deadbolt a core dependency or
  interpretation authority.
- Typed claims, evidence, and justification edges with closed support and
  refutation ceilings, closed support-context requirements, and exact scope
  matching.
- Candidate-only `magpie-claims-standing-v0` and one explicit snapshot-only
  `magpie-claims-standing-v1` rule for exact five-field Deadbolt occurrence
  membership.
- Deterministic closed contribution lanes with no counting or amplification,
  plus the governed-standing executable tour and byte-identical drop-and-replay
  regeneration.
- The exhaustive v0.1.0 release contract, coordinated explicitly
  non-publishable metadata, deterministic package inventories for all three
  crates, and standalone package verification for `magpie-log` only.

### Trust and verification

- Complete event snapshots are parsed and verified before any projection is
  mutated; chain, signature, genesis, sequence, link, hash, and malformed
  payload failures are tamper-loud.
- Frozen golden bytes and signatures are checked by Rust tests and an
  independent Python implementation; the portable Deadbolt fixture is also
  independently verified.
- Standing and anchor context are co-derived from one verified replay. Policy
  v1 requires exact agreement across all five Deadbolt identity fields.
- Hostile mismatch, authority-confusion, interpretation, changing-store,
  duplicate-anchor, and multiple-path controls exercise fail-closed behavior
  and reject epistemic amplification.
- CI executes formatting, strict Clippy, workspace tests, the governed tour,
  metadata and inventory checks, standalone `magpie-log` packaging, and
  independent frozen-chain verification.

### Explicit boundaries

Version 0.1.0 does not provide a general truth oracle; general achieved
`Supported`, `Settled`, or `Refuted`; aggregation or independence
amplification; contradiction debt; invalidation, supersession, or currentness
semantics; `EpistemicGate`; ordinary writer or MCP write surfaces; foreign
bundle-content verification; production key custody or operational security;
model, lens, retrieval, graph, reranking, evaluator, librarian, or navigator
infrastructure; stable Rust APIs; crates.io publication; or independently
registry-resolvable dependent packages.

The canonical format remains `magpie-core-v1`. The governed policies remain
`magpie-claims-standing-v0` and `magpie-claims-standing-v1`. The crate version
does not rename or mutate either identity.

The exhaustive source, trust, policy, packaging, and absence boundaries are in
[`docs/releases/v0.1.0-contract.md`](docs/releases/v0.1.0-contract.md) and
[`docs/releases/v0.1.0-release-metadata.md`](docs/releases/v0.1.0-release-metadata.md).
This entry prepares the source milestone; the `v0.1.0` tag remains a separate
human-controlled action.
