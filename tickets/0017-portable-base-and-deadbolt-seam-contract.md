# Ticket 0017: Portable base and Deadbolt seam contract

## Goal

Document the first phase of the portable Magpie base roadmap.

This is a docs-only phase. It clarifies that Magpie core is machine-agnostic
and can run without Deadbolt, while preserving the Deadbolt anchor seam as a
portable, optional, protocol-defined integration.

## Why

Magpie's base architecture is the signed append-only log plus deterministic
replay and derived projections. Deadbolt can add sealed evidence through
`SegmentAnchored`, but Magpie core must remain useful on ordinary machines and
must not inherit assumptions from the maintainer's personal topology.

The contract needs to be explicit before later PRs update Deadbolt's
`magpie-log` pin or add portable seam fixtures.

## Scope

- Add `docs/portable-base.md`.
- Add `docs/seams/deadbolt-anchor-contract.md`.
- Add minimal README links and boundary wording.
- Do not add runtime behavior.

## Non-goals / boundaries

- No Rust source changes.
- No `Cargo.toml` or `Cargo.lock` changes.
- No `docs/FORMAT.md` changes.
- No golden fixture changes.
- No `tools/verify_chain.py` changes.
- No CLI, daemon, background task, model integration, network sync, or Deadbolt
  runtime dependency.
- No payload variants.
- No canonical encoding changes.
- No `StandingView` changes.
- No `EpistemicGate`.
- No production trust, certification, attestation, or live-readiness claims.
- No implication that Deadbolt anchors settle truth.

## Contract points

- Magpie core requires a Rust toolchain, filesystem access, a Magpie log,
  deterministic replay, and optional SQLite projection support.
- Magpie core must not require Deadbolt, Hyprland, Tailscale, a local LLM
  daemon, an always-on server, special GPU/NPU hardware, maintainer-specific
  paths, or private machine topology.
- Portable base components are `magpie-log`, `magpie-claims`,
  `magpie-episodic`, `tools/verify_chain.py`, and deterministic examples,
  tests, and fixtures.
- Future optional layers include librarian/navigator read/propose tooling,
  model-backed summarization, always-on indexing, remote sync, UI/wiki, and a
  knowledge graph.
- Base invariant: "Magpie runs alone; integrations add evidence, not
  authority."
- `SegmentAnchored` preserves `bundle_kind`, `witness_root`,
  `witness_algorithm`, the foreign `canonicalization_profile`, and `run_id`.
- The foreign canonicalization profile is preserved verbatim and is not
  relabeled as `magpie-core-v1`.
- Anchors record occurrence/inclusion only and do not settle interpretation
  truth, create typed claims, or promote standing without later admitted
  claim/evidence/edge policy.

## Validation performed

- `git diff --check` passed.
- Changed-file scope checked with `git status --short`: README plus docs/ticket
  files only; no Rust, Cargo, format, fixture, or verifier files changed.
- Grep/sanity-check of new docs passed for:
  - `machine-agnostic`
  - `Deadbolt is optional`
  - `SegmentAnchored`
  - `occurrence/inclusion`
  - `does not settle interpretation truth`
  - `librarian`
  - `future work`
- `cargo test --workspace --locked` passed.

## Follow-up phases

- Deadbolt bumps its `magpie-log` pin and keeps `AnchorSet` exhaustive.
- Magpie adds portable Deadbolt anchor fixtures.
- Standing doctrine correction: humans settle nothing epistemically.
- Policy enums and exhaustive support ceilings.
- Refutation and aggregation.
- `EpistemicGate`.
- Optional librarian later.

## Suggested reviewer checks

- Confirm the diff is docs/tickets only.
- Confirm Magpie core remains described as portable and independent of
  Deadbolt.
- Confirm the Deadbolt seam remains optional but protocol-defined.
- Confirm `SegmentAnchored` remains occurrence/inclusion evidence and not
  interpretation truth.
- Confirm the README was not rewritten wholesale.
