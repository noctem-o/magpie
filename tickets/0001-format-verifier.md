# Ticket 0001 — Independent verifier for magpie-core-v1 (Python)

## 1. Goal

Implement `tools/verify_chain.py`: a standalone verifier for Magpie log chains,
written **from the format spec alone**, as an independent check that
`docs/FORMAT.md` is complete and unambiguous.

Given a JSON-lines chain file, it must:

1. Parse each line as a SignedEvent (structure per FORMAT.md §6).
2. Recompute each event's canonical bytes per FORMAT.md §1–§3.
3. Verify, per FORMAT.md §4–§5 and in this order per event: sequence,
   prev-link, recomputed SHA-256 content hash, Ed25519 signature over
   `"magpie-sig-v1" || hash`, genesis rules.
4. Take the trust root (verifying key, hex) as a CLI argument; additionally
   check the genesis self-description matches it.
5. Print one line per event: `seq`, recomputed hash hex, `OK`/failure reason.
   Exit 0 only if the entire chain verifies.

Usage: `python tools/verify_chain.py <chain.jsonl> <verifying_key_hex>`

## 2. Scope

- May read: `docs/FORMAT.md`, `crates/magpie-log/testdata/golden-v1.jsonl`.
- May create: `tools/verify_chain.py` only.

## 3. Non-goals

- Do NOT read any `.rs` file, anywhere in the repo. This is the point of the
  ticket: the spec must stand on its own. Consulting the Rust implementation
  invalidates the exercise even if the code works.
- No writing/appending of chains, no key generation, no projection logic.
- No CLI framework, no config files, no packaging.

## 4. Constraints

- Python 3.10+, standard library plus exactly one crypto dependency
  (`pynacl` or `cryptography`) for Ed25519 verification. SHA-256 from
  `hashlib`.
- Single file, under ~150 lines, no classes required.
- Failure output must state which check failed at which seq.

## 5. Tests

(Reviewer-run; do not assume shell access in the sandbox.)

- `python tools/verify_chain.py crates/magpie-log/testdata/golden-v1.jsonl <vk>`
  where `<vk>` is the `verifying_key` from the fixture's genesis line.
- Negative check: edit one character of a payload in a COPY of the fixture;
  the verifier must fail at that seq with a hash mismatch, not crash.

## 6. Acceptance criteria

- Exit 0 on the untouched golden fixture.
- The five printed hashes match, byte for byte, the `EXPECTED_HASHES` pinned
  in `crates/magpie-log/tests/golden.rs` — verified by the REVIEWER, who holds
  that file; the implementer never sees it.
- Tampered copy fails at the tampered seq with a correct reason.
- If any of this is impossible because FORMAT.md is ambiguous or incomplete,
  the correct output is a report saying exactly where the spec fails — that is
  a SUCCESS outcome for this ticket, not a failure. Do not guess past a spec
  hole by peeking at the implementation.

## 7. Frozen surfaces

Do not touch `crates/magpie-log/src/canonical.rs`, `docs/FORMAT.md`,
`crates/magpie-log/testdata/`, or any `tests/*.rs`. Read-only where permitted
by §2; no edits under any circumstances.
