> **⚠️ SUPERSEDED — ALREADY SHIPPED (2026-07-03). DO NOT RE-RUN.**
> This work landed in the **deadbolt** repo as commit `40fdfe8`, merged via
> **PR #316** (`64bf35f`) onto `main`. `crates/cog-anchor` exists with its
> smoke test green.
>
> **Mechanism actually shipped differs from §2 below and is better — leave it:**
> a pinned **HTTPS** git dep
> `magpie-log = { git = "https://github.com/noctem-o/magpie.git", rev = "7c7f043…" }`.
> It resolves locally via the Windows git credential manager and in CI via
> deadbolt `ci.yml`'s scoped `url.…insteadOf` rewrite onto the
> `MAGPIE_DEPLOY_KEY` deploy key — one URL for laptop + CI. The §2 SSH
> host-alias + `~/.ssh/config` pre-flight is therefore **obsolete** (and
> intentionally absent). Do not add ssh-alias config or rewrite the manifest.
>
> Next real work is deadbolt ticket **0007-anchor-emitter**, which 0006
> de-risked. Original ticket text preserved below for the record.

---

# Ticket 0006 — cog-anchor: cross-repo dependency probe + LogWriter smoke test

(Runs in the DEADBOLT repository.)

## 1. Goal

Create a new leaf crate `crates/cog-anchor` that proves deadbolt can consume
`magpie-log` and exercise the anchor path end to end. This ticket's product
is equal parts code and *findings*: it de-risks the repo boundary before the
real emitter (next ticket) is written.

Deliverables:

1. `crates/cog-anchor` added to the workspace, depending on `magpie-log`.
2. A minimal API surface, deliberately thin:
   - `pub struct Anchorer` — owns a `magpie_log::LogWriter`; constructing an
     `Anchorer` is the ONLY place in the deadbolt workspace where a
     `LogWriter` may be created. Document this in the crate root.
   - `Anchorer::open(log_path, signing_key_bytes) -> Result<Self, _>`
   - `Anchorer::anchor(bundle_kind, witness_root, witness_algorithm,
     canonicalization_profile, run_id) -> Result<SignedEvent, _>` — appends
     one `SegmentAnchored` event.
3. One integration test: open an `Anchorer` on a temp-dir log with a
   generated key; confirm the genesis was auto-written (seq 0); anchor one
   fixture root (64 lowercase hex); reopen; verify the chain reports 2 events
   and the anchor round-trips with all five fields intact.

## 2. Dependency mechanism — probe in this order, report the outcome

The credential is a read-only DEPLOY KEY (private half at
`C:\Users\herpe\.ssh\deadbolt_ci_read_magpie`, public half registered on
noctem-o/magpie). Deploy keys authenticate over **SSH only** — the git URL
must be the ssh form, never https.

**Pre-flight (ticket author, on the HOST, before dispatch):**

1. Ensure `~/.ssh/config` has an entry so git can find the key:
   ```
   Host github.com-magpie
       HostName github.com
       User git
       IdentityFile C:\Users\herpe\.ssh\deadbolt_ci_read_magpie
       IdentitiesOnly yes
   ```
2. Confirm host-level auth works:
   `git ls-remote ssh://git@github.com-magpie/noctem-o/magpie.git` — if this
   fails on the host, fix it before dispatch; otherwise the probe measures
   the wrong failure.
3. Obtain the pin: `git -C C:\magpie rev-parse main` → `<REV>` below.

**Probe order (Codex, in the sandbox):**

1. **Preferred:** pinned git dependency using the ssh host alias:
   `magpie-log = { git = "ssh://git@github.com-magpie/noctem-o/magpie.git", rev = "<REV>" }`
   Known cargo footgun, permitted fix: if cargo's internal fetch fails where
   plain `git ls-remote` succeeds, set `net.git-fetch-with-cli = true` in
   `.cargo/config.toml` and note that in the report.
   If the sandbox cannot reach the ssh agent/key at all, do not thrash —
   record the exact failure and fall back.
2. **Fallback:** path dependency to the sibling checkout
   (`path = "C:\magpie\crates\magpie-log"` or relative equivalent). If the
   sandbox cannot read outside the worktree, record that too.

The final summary MUST state which mechanism is in the committed Cargo.toml
and why, plus the verbatim error from any failed attempt. A clean report of
"git dep impossible in sandbox, used path dep" is a success outcome — CI
uses the MAGPIE_DEPLOY_KEY Actions secret and can carry the git dep even if
the local sandbox cannot; reconciling those two worlds is the next ticket's
concern, not this one's.

## 3. Scope

- New files under `crates/cog-anchor/` only, plus the workspace-members line
  and any lockfile change those imply.

## 4. Non-goals

- No integration with seal points, cog-cli, or any existing crate — nothing
  outside the new crate imports `cog-anchor` yet.
- No config system, no key management beyond "bytes in, dev-grade" (real
  custody is the emitter ticket's concern).
- No reconciliation, no pending-anchor markers.
- `magpie-log` must NOT appear in the dependencies of any other crate,
  especially not the kernel/verified core.

## 5. Constraints

- Match deadbolt's workspace conventions (edition, lints, doc style).
- If `magpie-log` fails to compile under this workspace's toolchain, that is
  a FINDING to report with the full error, not something to patch around.
  Do not vendor, fork, or modify magpie-log.

## 6. Tests

(Reviewer runs `cargo test -p cog-anchor`; note any commands the sandbox
could not execute.)

- The integration test in §1.3, green.
- `cargo tree -i magpie-log` (reviewer-run) shows exactly one dependent:
  cog-anchor.

## 7. Acceptance criteria

- Workspace builds; new test green; no existing test disturbed.
- Dependency-mechanism report present and honest.
- `LogWriter` construction confined to `Anchorer::open`, stated in docs.

## 8. Frozen surfaces

Do not modify `scripts/codex-delegate.ps1`, any existing crate's source, or
anything in the magpie repository. Read-only everywhere except
`crates/cog-anchor/` and the workspace manifest.
