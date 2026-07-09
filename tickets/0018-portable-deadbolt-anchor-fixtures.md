# Ticket 0018: Portable Deadbolt anchor fixtures

## Purpose

Add Magpie-side fixture coverage for the portable Deadbolt anchor seam.

The fixture proves that an ordinary Magpie checkout can verify and replay a log
containing `SegmentAnchored` without requiring a live Deadbolt installation,
Hyprland, special hardware, a local model, Tailscale, maintainer-specific
paths, or private topology.

## Scope

- Add `fixtures/deadbolt-anchor-v1/README.md`.
- Add `fixtures/deadbolt-anchor-v1/anchor-log.jsonl`.
- Add fixture-specific `magpie-log` tests that verify and replay the checked-in
  log, observe the anchor payload, and assert the foreign anchor fields are
  preserved verbatim.
- Add fixture-specific `magpie-claims` coverage proving the same anchor-only log
  does not create claims, typed evidence, justification edges, or standing.
- Add a narrow pointer from the Deadbolt anchor contract to the fixture.

## Non-goals

- No Deadbolt runtime dependency.
- No Deadbolt repo changes.
- No new payload variants.
- No canonical encoding changes.
- No `docs/FORMAT.md` changes.
- No existing golden vector changes.
- No `tools/verify_chain.py` changes.
- No writer-facing surfaces.
- No MCP write paths.
- No `EpistemicGate`.
- No librarian or navigator functionality.
- No model integration.
- No network sync.
- No standing policy, evidence ceilings, refutation, aggregation,
  contradiction debt, invalidation, supersession, or ratification.
- No production trust, external attestation, certification, or live-readiness
  claim.
- No implication that Deadbolt anchors settle interpretation truth.

## Changed files

- `fixtures/deadbolt-anchor-v1/README.md`
- `fixtures/deadbolt-anchor-v1/anchor-log.jsonl`
- `crates/magpie-log/tests/deadbolt_anchor_fixture.rs`
- `crates/magpie-claims/tests/deadbolt_anchor_fixture.rs`
- `docs/seams/deadbolt-anchor-contract.md`
- `tickets/0018-portable-deadbolt-anchor-fixtures.md`

## Validation performed

- `git diff --check` passed.
- `cargo test -p magpie-log --locked` passed.
- `cargo test -p magpie-claims --locked` passed.
- `cargo test --workspace --locked` passed.
- `cargo fmt --all -- --check` passed after formatting the new Rust tests.
- `python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737`
  passed, printing `OK` for seq 0 and seq 1.
- `git diff --name-only` was run. Because the new fixture/test/ticket files are
  untracked until the reviewer stages them, `git status --short` was also used
  to confirm the full changed-file set.
- The machine-assumption scan was run:
  `rg -n "C:\\\\|/home/|/mnt/|Tailscale|Hyprland|Qwen|Ryzen|4090|lastgarlean|noctem-o" fixtures docs tickets crates`.
  Hits were limited to existing docs/tickets plus this ticket's boundary wording
  and recorded command; the fixture directory itself had no hits.

## Follow-up phases

- Standing doctrine correction: humans settle nothing epistemically.
- Policy enums and exhaustive support ceilings.
- Refutation and aggregation.
- `EpistemicGate`.
- Optional librarian/navigator later.

## Suggested reviewer checks

- Confirm the fixture is deterministic test material and not a live Deadbolt
  artifact.
- Confirm the fixture preserves the foreign canonicalization profile verbatim
  and does not relabel it as `magpie-core-v1`.
- Confirm the tests prove chain verification, replay, and standing inertness
  without adding runtime behavior.
- Confirm no format surface, canonical encoding, payload variant, writer
  surface, or authority boundary changed.
