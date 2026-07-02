# Ticket 0002 — CI format-freeze gate

## 1. Goal

Add a GitHub Actions CI gate that protects Magpie's `magpie-core-v1` format freeze and verifies the workspace on every push and pull request.

The CI should make it hard to accidentally change the signed-log format, golden vectors, or regenerable-projection invariant without noticing immediately.

## 2. Scope

Allowed changes:

- Add `.github/workflows/ci.yml`.
- Optionally add a short note to `README.md` explaining that CI protects the format freeze.
- Do not edit any Rust source files unless the reviewer explicitly asks in a follow-up.
- Do not edit the delegation wrapper.
- Do not edit existing golden fixtures or expected hashes.

## 3. Non-goals

- No changes to `crates/magpie-log/src/canonical.rs`.
- No changes to `docs/FORMAT.md`.
- No changes to `crates/magpie-log/testdata/golden-v1.jsonl`.
- No changes to `crates/magpie-log/tests/golden.rs`.
- No regeneration of golden vectors.
- No dependency changes unless required only for the CI workflow itself.
- No publishing, releases, badges, or deployment.

## 4. CI requirements

Create a workflow at:

`.github/workflows/ci.yml`

The workflow should run on:

- `push`
- `pull_request`

It should run on `ubuntu-latest`.

The workflow should include:

1. Checkout.
2. Install Rust stable.
3. Cache Cargo build artifacts if reasonable.
4. Run:

   ```bash
   cargo fmt --all --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   ```

5. Install Python dependencies needed by `tools/verify_chain.py`.

   The verifier currently requires Python's `cryptography` package.

6. Run the independent verifier against:

   ```text
   crates/magpie-log/testdata/golden-v1.jsonl
   ```

   using the verifying key from the golden fixture/genesis.

The verifier step must fail CI if the verifier returns a non-zero exit code.

## 5. Frozen surfaces

The following files define or protect the frozen v1 format:

- `crates/magpie-log/src/canonical.rs`
- `docs/FORMAT.md`
- `crates/magpie-log/testdata/golden-v1.jsonl`
- `crates/magpie-log/tests/golden.rs`
- `tools/verify_chain.py`

If any golden test fails, stop. Do not update the fixture, expected hashes, canonical encoding, or format spec to make CI green.

A red golden test means either:

1. the implementation accidentally broke the frozen format and must be reverted; or
2. the project is intentionally creating a new format profile, which is out of scope for this ticket.

## 6. Windows execution constraint

On native Windows, Codex shell execution may fail with:

`CreateProcessAsUserW failed: 5`

Do not rely on shell commands to complete this ticket. If shell commands fail, continue only if the requested file edits can be made safely from the repository contents.

Report skipped validation commands in the final summary. The human reviewer will run validation outside Codex.

Do not claim tests passed unless they actually ran and passed.

## 7. Tests / reviewer validation

The reviewer should run locally:

```powershell
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

The reviewer should also run the independent verifier manually before merge.

Use the verifying key from the genesis row of:

```text
crates/magpie-log/testdata/golden-v1.jsonl
```

or from the pinned golden test if already surfaced there.

Example shape:

```powershell
python .\tools\verify_chain.py .\crates\magpie-log\testdata\golden-v1.jsonl <VERIFYING_KEY_HEX>
```

## 8. Acceptance criteria

- `.github/workflows/ci.yml` exists.
- CI runs on push and pull request.
- CI runs `cargo fmt --all --check`.
- CI runs `cargo clippy --workspace --all-targets -- -D warnings`.
- CI runs `cargo test --workspace`.
- CI installs Python `cryptography`.
- CI runs `tools/verify_chain.py` against the committed golden fixture.
- No frozen-surface files are modified.
- No golden vectors are regenerated.
- No changes are made to `scripts/codex-delegate.ps1`.

## 9. Suggested final response format

End with:

- Files changed
- Behaviour added
- CI commands added
- Verifier command added
- Tests run
- Validation skipped
- Frozen surfaces touched? yes/no
- Known risks
- Suggested reviewer checks
