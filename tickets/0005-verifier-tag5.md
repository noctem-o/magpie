# Ticket 0005 — Extend independent verifier to FORMAT.md tag 5

## 1. Goal

`tools/verify_chain.py` was written against FORMAT.md as of the v1 freeze
(payload tags 0–4). ADR-0001 added tag 5 (`SegmentAnchored`) to the
vocabulary, and the golden fixture now contains six events. The verifier —
being correctly strict — cannot recompute canonical bytes for an unknown tag
and fails on seq 5. Extend it to the amended spec.

This remains a spec-fidelity exercise: the extension must be written from
`docs/FORMAT.md` §3 alone (which now documents tag 5's fields and encoding),
not from the Rust.

## 2. Scope

- May read: `docs/FORMAT.md`, `crates/magpie-log/testdata/golden-v1.jsonl`.
- May modify: `tools/verify_chain.py` only.

## 3. Non-goals

- Do NOT read any `.rs` file. Same rule as ticket 0001, same reason.
- No verification of anchored bundle *contents* — the verifier checks the
  chain; per FORMAT.md, segment contents verify separately against
  `witness_root` by the sealing subsystem's own tooling. Out of scope here.
- No loosening: unknown tags must still fail loudly. Strictness is the
  feature.

## 4. Constraints

- Same as ticket 0001: single file, stdlib + the one existing crypto dep,
  keep it small. The diff should be roughly one new payload-encoding branch.

## 5. Tests

(Reviewer-run.)

- Untouched extended fixture: exit 0, six hashes printed.
- Truncated copy (first five lines only — the pre-seam fixture content):
  still exits 0 with five hashes; backwards compatibility over old chains is
  free and must stay that way.
- A copy with the seq-5 `bundle_kind` altered: fails at seq 5, hash mismatch.

## 6. Acceptance criteria

- All six printed hashes match the reviewer-held `EXPECTED_HASHES` in
  `crates/magpie-log/tests/golden.rs` byte for byte (reviewer checks;
  implementer never sees that file).
- CI's "Verify frozen golden chain" step passes again with no workflow
  change.
- An event with an unrecognized payload kind still causes a loud failure at
  that seq — include no fallback, no tolerance, no guessing.

## 7. Frozen surfaces

Do not touch `crates/magpie-log/src/canonical.rs`, `docs/FORMAT.md`,
`crates/magpie-log/testdata/`, or any `tests/*.rs`.
