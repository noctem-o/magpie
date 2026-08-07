# Ticket 0067: Episodic Statusless Claim Assertion Projection

## Status

Implementation ticket: runtime-semantic remediation of the episodic
projection for `ClaimAssertedV2`.

Required base:

```text
64d3dbeddcb431575de4d1a2b27130d39380e652
origin/main at branch creation
```

Branch:

```text
agent/episodic-statusless-claim-assertion
```

Proposed commit title:

```text
episodic: preserve statusless v2 claim assertions
```

Proposed draft PR title:

```text
episodic: stop synthesizing ClaimAssertedV2 status
```

This branch is independent of the parked/open stacks #108-#111. It is based
directly on current `origin/main` and shares no commit with those branches
beyond `origin/main` itself.

## Audit target

Intended remediation evidence for:

```text
A-006 / RQ-004:
Episodic output assigns Conjectured to ClaimAssertedV2,
which records no status.
```

Disposition of record: **Confirmed; current semantic defect**
(`docs/audits/audit-disposition-2026-08.md`). This ticket does not edit the
disposition ledger and does not claim the finding is Closed;
reclassification is a later administrative step after review and merge
evidence exist.

## Defect

`Payload::ClaimAssertedV2` (`crates/magpie-log/src/event.rs`) contains no
status field; its repository comment states that standing remains a derived
projection. The episodic projection nevertheless mapped it to
`claim_status: Some("Conjectured")`, inventing signed-event content that was
never present.

## Positive law

```text
signed event contains an explicit status
-> episodic timeline may retain that exact status

signed event contains no status
-> episodic timeline records no status
```

The episodic projection is a timeline of what the signed event actually
contained. It must not become a second standing resolver, and it must not
infer a default status from event kind, actor class, claim domain,
metadata, legacy policy, current standing, compatibility conventions, prior
events, or future events.

An assertion act is not a `Conjectured` status. `ClaimAssertedV2`
establishes that an assertion occurred; it does not encode `Open`,
`Conjectured`, `Supported`, `Settled`, or `Refuted`.

## Exact correction

In `crates/magpie-episodic/src/lib.rs`:

```text
ClaimAssertedV2.claim_status:
Some("Conjectured")
->
None
```

Preserved for the same row:

```text
kind        = "claim_asserted_v2"
claim_id    = exact claim_id
from_status = None
to_status   = None
body        = exact statement
```

FTS, provenance, sequence, and timestamp behavior are unchanged.

## Preserved status-bearing projections

- `ClaimAsserted.status` genuinely carries a signed `Status`; its
  projection remains `claim_status = Some(exact status name)` for every
  current status.
- `ClaimStatusChanged` remains `claim_status = None`,
  `from_status = Some(exact from)`, `to_status = Some(exact to)`.

## Projection version

```text
SCHEMA_VERSION:
2
->
3
```

The physical SQLite column layout is unchanged. The bump is required
because the meaning of persisted derived rows changes: under projection
version 2 a `claim_asserted_v2` row stores `"Conjectured"`; under version 3
a fresh replay of the same log stores `NULL`. Without the bump, an
already-built file-backed database could retain the fabricated value while
a fresh replay produces `NULL`, violating the rebuild-equivalence
invariant. This is a projection-semantics version bump, not an L0 format
bump.

## Migration law

No in-place SQL rewrite of stale rows. The derived database is disposable:

```text
old episodic schema/projection version
-> discard derived tables
-> recreate empty current schema
-> caller replays authoritative verified log
-> corrected rows regenerated deterministically
```

`EpisodicView::at_path()` does not possess the authoritative log and must
not replay automatically. Opening a stale-version database detects the
mismatch, recreates the empty current schema, and returns an empty
projection; the existing caller-owned `LogReader::replay(...)` then
repopulates it. No migration chains, per-version transforms, schema
registries, log access from `EpisodicView`, callbacks, hidden replay, or
ambient filesystem/log discovery.

## Compatibility

Unchanged:

```text
L0 record format and event canonicalization
log hashes and signatures
Status enum and standing policy identity
claim identity and claim semantics
public EpisodicEvent row shape
legacy status-bearing event projection
FTS behavior
FORMAT.md and ADR identities
```

Intentionally changed:

```text
episodic ClaimAssertedV2.claim_status: "Conjectured" -> null
canonical episodic projection bytes for histories containing ClaimAssertedV2
episodic schema/projection version 2 -> 3
```

No previously signed log event becomes invalid. The same authoritative log
regenerates a corrected derived projection. This PR must not claim "no
observable behavior changes"; the observable correction is the point.

## Standing remains separate

Forbidden: asking `magpie-claims` for standing and writing it into episodic
status fields. `claim_status` is historical signed-event content, not
current claim state.

## Scope allowlist

Exactly:

```text
tickets/0067-episodic-statusless-claim-assertion.md
crates/magpie-episodic/src/lib.rs
crates/magpie-episodic/tests/episodic.rs
```

No change to `magpie-log`, `magpie-claims`, the audit disposition ledger,
FORMAT, ADRs, Cargo manifests or lockfile, or README. This ticket remediates
A-006 / RQ-004 only; the neighboring episodic findings A-007 / A-008 /
A-012 (projection failure, storage/durability, schema ownership) are
explicitly out of scope, and `EpisodicView::at_path()` is not redesigned.

## Tests

In `crates/magpie-episodic/tests/episodic.rs`:

1. replayed `ClaimAssertedV2` row: exact kind, claim_id, `claim_status ==
   None`, `from_status == None`, `to_status == None`, exact body;
2. the exact `ClaimAssertedV2` canonical row serializes
   `"claim_status":null` (row-level JSON inspection, not whole-buffer
   search);
3. legacy `ClaimAsserted(Open|Conjectured)` rows keep their exact signed
   status, and their canonical bytes differ;
4. `ClaimStatusChanged` from/to projection unchanged (existing test);
5. existing schema-v0 stale-file rebuild test retained;
6. new schema-v2 file with current physical columns,
   `PRAGMA user_version = 2`, and a stale sentinel
   `claim_asserted_v2`/`"Conjectured"` row: opening with v3 code discards
   the stale row, stamps version 3, and does not fabricate replay;
7. after caller replay into the rebuilt file, `ClaimAssertedV2` rows carry
   `claim_status == None` and canonical bytes equal a fresh in-memory
   projection of the same log;
8. reopening the migrated v3 file preserves the NULL row without replay;
9. existing current-schema reopen-preservation test retained (now
   exercising version 3);
10. drop/rebuild determinism unchanged.

The `ClaimAssertedV2` fixture event is signed/appended normally through
`LogWriter` with valid repository-native field values.

## Validation

```text
git diff --check
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked --no-fail-fast
cargo run --locked --example tour -p magpie-claims
python tools/verify_chain.py <golden and deadbolt-anchor chains>
python tools/check_release_metadata.py
cargo package -p magpie-log --locked
```

Record genuine exit codes. This host's default MSVC detection selects an
incomplete VS 18 BuildTools installation (no `lib`/`include`); cargo
commands that link or compile C must run under the complete VS 2022
BuildTools `vcvars64.bat` environment. Do not report anything as passed
that did not run.

## Publication authority

The owner instruction for this ticket authorizes the worker to commit on
the task branch, push it, and open one **draft** PR against `main`. The
human owner retains sole merge authority. The worker must not merge, must
not enable auto-merge, must not mark the PR ready for review, and must not
touch the #108-#111 branches.

## Stop conditions

Stop and report rather than proceeding if:

- any currently passing behavior outside the single `ClaimAssertedV2`
  status correction would change;
- any path outside the allowlist appears objectively required;
- a mandatory repository check fails in a way this ticket cannot fix
  within scope.
