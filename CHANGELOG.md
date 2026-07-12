# Changelog

This file records repository release milestones. Crate semantic versions,
canonical format identities, and governed policy identities are separate
commitments; changing one does not silently rename or mutate the others.

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
  duplicate-anchor, and multiple-path controls prove fail-closed behavior and
  no epistemic amplification.
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
