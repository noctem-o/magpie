# Ticket 0066: Standing Policy Status Exhaustiveness (v2/v3)

## Status

Implementation ticket: behavior-preserving exhaustiveness hardening of the
standing-policy v2/v3 status-combination seams.

Required base:

```text
64d3dbeddcb431575de4d1a2b27130d39380e652
origin/main at branch creation
```

Branch:

```text
agent/standing-status-exhaustiveness
```

Proposed commit title:

```text
claims: make standing status evolution exhaustive
```

Proposed draft PR title:

```text
claims: make v2/v3 status promotion exhaustive
```

This branch is independent of the parked ADR-0007/A-024 stack (PRs
#108-#110). It is based directly on current `origin/main`, shares no commit
with those branches beyond `origin/main` itself, and does not modify their
files.

## Audit target

Intended remediation target:

```text
A-005 / RQ-005:
v2/v3 wildcard inherited-status arms remain latent promotion paths for a
future Status variant.
```

Evidence of record:

- `docs/audits/magpie-runtime-quality-audit-2026-08-04.md` RQ-005:
  `standing_v2.rs` used `Some(_) if deterministic_supported`;
  `standing_v3.rs` used `_ if corroborated`.
- `docs/audits/audit-disposition-2026-08.md` records A-005 / RQ-005 as
  **Confirmed** (constitutional; latent; high).

This ticket does **not** edit the disposition ledger and does **not** claim
the finding is Closed. Reclassification is a later administrative
evidence-reconciliation step after implementation, review, and merge
evidence exist.

## Defect

No currently representable input produces a wrong result. The defect is
latent: a future `Status` variant added to
`crates/magpie-log/src/event.rs::Status` would silently enter the wildcard
arms of both combination seams and could acquire `Supported` semantics
without an explicit policy decision.

ADR-0002 already requires explicit projection match arms rather than
wildcards for format evolution; the same fail-closed rule is applied here to
the standing-policy status dimension. The v4 policy seam
(`standing_v4.rs`) already follows the required explicit-enumeration
pattern and is read-only context for this ticket.

## Laws

### Law 1 — current behavior is frozen

For every currently representable `Option<Status>` value and both values of
the relevant Boolean (`deterministic_supported` for v2, `corroborated` for
v3), the combination result before and after this ticket is identical. No
current policy outcome may change.

### Law 2 — future statuses receive no implicit semantics

Both combination seams explicitly enumerate every current `Status` variant.
No arm equivalent to `Some(_)`, `_`, or `other => other` may consume an
unenumerated future `Status`. Wildcards over the independent Boolean
dimension are permitted only where the `Status` variant has already been
explicitly selected, e.g. `(Some(Status::Settled), _)`, because such a
wildcard cannot capture a future `Status`.

Required maintenance behavior after this ticket:

```text
new Status variant
-> non-exhaustive compiler error at both policy seams
-> explicit policy decision required
-> tests/docs/versioning reviewed deliberately
```

### Law 3 — no policy-version reinterpretation

Because every currently representable input/output mapping is identical:

```text
no policy ID change
no schema change
no canonical-byte change
no runtime API change
```

If implementation discovers that a currently representable case must
change, stop and report the discrepancy. Do not silently bump or alter
policy identity.

## Exact v2 truth table (compatibility law)

`combine_v2_standing(inherited, deterministic_supported)`:

| inherited | false | true |
| --- | --- | --- |
| `None` | `None` | `None` |
| `Some(Open)` | `Some(Open)` | `Some(Supported)` |
| `Some(Conjectured)` | `Some(Conjectured)` | `Some(Supported)` |
| `Some(Supported)` | `Some(Supported)` | `Some(Supported)` |
| `Some(Settled)` | `Some(Settled)` | `Some(Settled)` |
| `Some(Refuted)` | `Some(Refuted)` | `Some(Refuted)` |

V2 never synthesizes standing from `None` (Ticket 0036: support applies
only "for an existing inherited claim"). `Settled` and governed `Refuted`
remain dominant. 6 inherited states x 2 Boolean states = 12 cases; all 12
are tested.

## Exact v3 truth table (compatibility law)

`combine_standing(inherited, corroborated)`:

| inherited | false | true |
| --- | --- | --- |
| `None` | `None` | `Some(Supported)` |
| `Some(Open)` | `Some(Open)` | `Some(Supported)` |
| `Some(Conjectured)` | `Some(Conjectured)` | `Some(Supported)` |
| `Some(Supported)` | `Some(Supported)` | `Some(Supported)` |
| `Some(Settled)` | `Some(Settled)` | `Some(Settled)` |
| `Some(Refuted)` | `Some(Refuted)` | `Some(Refuted)` |

`None + corroboration -> Supported` is intentional policy-v3 behavior
(Ticket 0053 section 15 precedence step 4: otherwise, successful v3
corroboration yields `Supported`). It is compatibility law for this ticket
and must not be "normalized" toward v2. 12 cases; all 12 are tested.

## Implementation shape

Modify only:

```text
crates/magpie-claims/src/standing_v2.rs  (combine_v2_standing)
crates/magpie-claims/src/standing_v3.rs  (combine_standing)
```

The resulting matches must be exhaustive over the current `Status`
vocabulary by naming every variant (or-patterns such as
`Status::Open | Status::Conjectured` are permitted where semantics are
identical). Rust exhaustiveness checking is the compile-time guard; no
fake sixth variant is added for tests.

Do not change `Status` itself: no new variant, no reordering, no serde
change, no `#[non_exhaustive]`, no canonical-encoding or FORMAT change.
The task is to make the consumers exhaustive, not to change the
vocabulary.

## Tests

Add focused table-driven unit tests to the existing `#[cfg(test)]` modules
in both files:

- v2: the full 12-case `Option<Status> x deterministic_supported` matrix.
- v3: the full 12-case `Option<Status> x corroborated` matrix.

The matrices must make particularly obvious that:

- `Settled` + support/corroboration stays `Settled`;
- `Refuted` + support/corroboration stays `Refuted`;
- v2 `None + deterministic support` stays `None`;
- v3 `None + corroboration` becomes `Supported`;
- `Supported` stays `Supported` under both Boolean values.

The two policies keep separate combination functions; their intentional
`None`-semantics difference must not be hidden inside a shared helper.

## Scope allowlist

This PR changes exactly:

```text
tickets/0066-standing-policy-status-exhaustiveness.md
crates/magpie-claims/src/standing_v2.rs
crates/magpie-claims/src/standing_v3.rs
```

No change to `README.md`, `docs/FORMAT.md`, Cargo manifests or lockfile,
`standing_v1.rs`, `standing_v4.rs`, `event.rs`, the audit disposition
ledger, ADR files, or any file belonging to PRs #108-#110. If a mandatory
repository check reveals an objective requirement beyond this allowlist,
stop and report before expanding scope.

## Non-goals

- unifying v2/v3/v4 or creating a generic standing-combination engine;
- deduplicating standing policy code (RQ-012 is separate debt);
- redesigning precedence, renaming statuses, or changing policy identities;
- rewriting traces, currentness, legacy/raw standing, or ceilings;
- ADR-0007 / A-024 origin-authority work;
- new public APIs;
- closing A-005 / RQ-005 in the disposition ledger (later administrative
  step).

## Validation

Run and record exit codes for:

```text
git diff --check
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --no-fail-fast
cargo run --locked --example tour -p magpie-claims
python tools/verify_chain.py crates/magpie-log/testdata/golden-v1.jsonl <pinned key>
python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl <pinned key>
python tools/check_release_metadata.py
```

Do not mask failing exit codes. If `magpie-episodic` cannot build on this
host because of the known local MSVC-header limitation, report that
precisely and rely on CI for that component; never report it as locally
passed.

## Publication authority

The owner instruction for this ticket authorizes the worker to commit on
the task branch, push it, and open one **draft** PR against `main`. The
human owner retains sole merge authority. The worker must not merge, must
not enable auto-merge, must not mark the PR ready for review, and must not
touch the parked #108-#110 branches.

## Stop conditions

Stop and report rather than proceeding if:

- any currently representable v2/v3 case would change result;
- the same wildcard defect is found in `standing_v4.rs` (out of scope
  unless discovered; report evidence first);
- any path outside the allowlist appears objectively required;
- validation fails in a way this ticket cannot fix within scope.
