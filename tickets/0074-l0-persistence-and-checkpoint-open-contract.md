# Ticket 0074: Supported L0 persistence and checkpoint-aware open contract v0

## Status

Documentation-only implementation-contract tranche for A-008 / RQ-007 /
RQ-008 / NEW-LOG-01.

This ticket changes no Rust, tests, fixtures, FORMAT, schema, dependency, CI,
historical audit, living audit disposition, readiness classification, release
metadata, persistence runtime, checkpoint storage, currentness, standing, or
authority runtime. A separate implementation PR must satisfy the linked design
and hostile evidence before any administrative reclassification.

Resolved base:

```text
2c38ffe32367bbb38d96525b938f148d40eb29d4
Merge pull request #127 from noctem-o/agent/history-checkpoint-evaluation-v0
```

Branch:

```text
docs/l0-persistence-open-contract-v0
```

Proposed commit:

```text
docs: define L0 persistence and checkpoint-open contract
```

Proposed draft PR title:

```text
design: define L0 persistence and checkpoint-aware reopen v0
```

Normative implementation contract:

[l0-persistence-and-checkpoint-open-v0.md](../docs/design/l0-persistence-and-checkpoint-open-v0.md)

## Owner-selected objective

Define the smallest supported local physical-history path before any
checkpoint-qualified history is integrated into standing or another downstream
derivation.

The selected result is:

```text
SQLite-backed supported local L0
+ explicit create/open split
+ weak verified-prefix reopen
+ explicit ContainsCheckpoint-relative reopen
+ stable bounded read transaction
+ atomic expected-predecessor compare-and-commit
+ conservative uncertain-commit recovery

FileStore remains JSONL compatibility/development/inspection
MemStore remains in-memory test/embedding
```

This is a physical persistence decision. It does not change canonical event
identity, expected-history semantics, standing, trust, authority, or FORMAT.

## Governing authority

- ADR-0001 keeps one conserved signed log as L0 and `LogWriter` as the sole
  public append capability.
- ADR-0008 requires complete producing coordinates for governed derived
  results and says operational I/O/resource/process failure produces no
  semantic conclusion.
- ADR-0009 separates verified supplied history from explicit checkpoint
  expectation and forbids ambient checkpoint/relation selection.
- PR #127 implements `HistoryCheckpointV0`, explicit `Exact` and
  `ContainsCheckpoint`, context-bearing evaluation results, one-read behavior,
  and complete-verification-before-outcome.
- FORMAT §6 leaves physical storage non-normative; SQLite rows cannot redefine
  `magpie-core-v1`.

The ticket specifies implementation of those existing boundaries. It does not
amend an ADR.

## Current physical facts

At the resolved base:

```text
FileStore read
    std::fs::read whole file
    split LF
    drop empty slices
    clone all records

FileStore append
    open create+append
    write record bytes
    write LF separately
    no lock or sync

LogWriter open
    verify one whole record vector
    create genesis when empty

WriterStore append
    receives bytes only
    has no expected predecessor
    returns Result<(), LogError>

replay/checkpoint evaluation
    one read
    retain whole snapshot
    verify completely
    publish only after verification
```

The current code therefore supplies strong logical verification and weakly
specified physical persistence. This ticket does not call current FileStore
corrupt merely because stronger guarantees are absent.

## Falsification conclusions

SQLite remained selected because:

1. workspace metadata already pins bundled `rusqlite 0.37`, so no new
   third-party dependency is needed;
2. Accepted FORMAT separates canonical event identity from physical storage;
3. a SQLite `BEGIN IMMEDIATE` transaction can serialize competing writers and
   atomically compare the actual terminal coordinate before insert;
4. one read transaction can preserve an unchanging ordered snapshot across
   multiple bounded passes;
5. rollback-journal recovery supplies a mature physical crash boundary without
   turning FileStore into a custom database; and
6. explicit application identity and schema refusal can avoid the episodic
   projection's destructive foreign-database behavior.

The recommendation required four corrections before it was safe:

- current `WriterStore` cannot be reused unchanged because it lacks predecessor
  and commit-state coordinates;
- current public `LogStore::read_records` cannot be the supported SQLite reader
  seam because it requires whole-history `Vec<Vec<u8>>` materialization;
- rollback-journal durability uses `synchronous=EXTRA`, not an overbroad claim
  from `FULL`;
- database ownership and supported user version must be checked from the file
  header before a write-capable SQLite open of an existing path; and
- the resource law must require explicit finite budgets and length-before-BLOB
  materialization, not merely replace one unbounded whole-file read with an
  unbounded SQL cursor.

No Accepted ADR conflict was found. The current root instruction that only the
episodic crate uses SQLite records the pre-contract crate layout. The runtime
tranche must reconcile that line when it deliberately adds the already-pinned
dependency to `magpie-log`.

## Frozen decisions

### Supported stores

| Store | Target classification |
| --- | --- |
| `SqliteL0Store` | supported local persistent L0 |
| `FileStore` | retained JSONL compatibility/development/inspection; no upgraded guarantee |
| `MemStore` | in-memory tests/embedding; no persistence guarantee |

### SQLite profile

```text
local filesystem
application_id = 0x4D504C30 (MPL0)
user_version = 1
journal_mode = DELETE
synchronous = EXTRA
locking_mode = NORMAL
read_uncommitted = OFF
shared cache disabled
busy timeout = 0
full PRAGMA integrity_check before event verification
read = explicit stable transaction
write/create = BEGIN IMMEDIATE
```

### Physical records

One row stores:

```text
eight-byte big-endian u64 sequence
+ exact serialized SignedEvent record BLOB
```

The parsed signed event is semantically authoritative. Sequence duplication is
exact-checked. No cached head/count/hash is an independent truth source.

### Create/open vocabulary

```text
create_new
    initializes ownership + schema + genesis in one transaction

open_verified_prefix
    verifies one explicit stable persisted prefix
    establishes no checkpoint or freshness proposition

open_containing_checkpoint_v0(checkpoint)
    fixes ContainsCheckpoint by the named method
    reuses magpie-log-history-expectation-v0
    accepts no ambient/default relation
```

V0 does not add an exact-terminal writer-open method.

### Resource vocabulary

Every supported operation receives one explicit finite
`L0ResourceLimitsV0`-equivalent value:

```text
max_database_bytes
max_record_count
max_record_bytes
max_total_record_bytes
```

The main database and rollback journal are length-checked before write-capable
open/recovery, and checked SQLite page size is bounded before integrity
checking. Each record's BLOB length is checked before BLOB materialization.
Resource failure is operational and produces no checkpoint outcome, projection
publication, or writer. A successfully opened writer retains the verified
snapshot's checked total record bytes and the selected limits so it can reject
an over-budget append before entering the write transaction and can roll back
a transaction whose resulting database pages exceed its budget.

### Stale and uncertain writer vocabulary

```text
actual terminal != writer expected terminal
    -> WriterStale
    -> no commit
    -> no silent refresh/retry

commit called but success not observed
    -> CommitStateUnknown
    -> writer poisoned
    -> reopen + complete verification required

Ok
    -> COMMIT reported success under the fixed local SQLite profile
```

`Err` does not universally mean “nothing committed,” and blind retry does not
mean exactly once.

## Hostile proof matrix for the runtime ticket

The implementation must prove all twenty design scenarios:

| ID | Proof target |
| --- | --- |
| P1 | atomic creation and exactly one genesis |
| P2 | foreign SQLite refusal without mutation |
| P3 | unknown future schema refusal |
| P4 | explicit weak old-prefix reopen |
| P5 | exact-terminal checkpoint containment reopen |
| P6 | valid suffix beyond checkpoint succeeds under containment |
| P7 | valid below-checkpoint history verifies but open refuses |
| P8 | equal/longer wrong branch refuses |
| P9 | two writers from one predecessor cannot commit siblings |
| P10 | concurrent reader remains on one stable snapshot |
| P11 | definite pre-commit failure behavior |
| P12 | commit-before-observed-ack process death |
| P13 | unknown commit poisons writer |
| P14 | corrupt DB/event open failure without repair |
| P15 | history and external checkpoint rollback together remains undetected |
| P16 | older history with newer retained checkpoint refuses |
| P17 | projection database refused as L0 |
| P18 | network filesystem outside supported guarantee |
| P19 | database/journal/count/record/total-byte resource failures before publication |
| P20 | no ambient checkpoint selection |

Tests must reach the intended stage. A stale-writer test must show one exact
committed successor; a wrong-branch test must use two otherwise valid chains;
a corrupt-suffix test must verify the checkpoint prefix before the late error;
and a foreign-database test must prove the unrelated data stayed unchanged.

## Changed-path allowlist

This documentation tranche is limited to exactly:

```text
docs/design/l0-persistence-and-checkpoint-open-v0.md
tickets/0074-l0-persistence-and-checkpoint-open-contract.md
```

No README/design-map update is required: nearby implementation contracts are
linked by their tickets rather than inventoried individually in README.

## Validation for this tranche

Required documentation-only validation:

```text
git diff --check
git diff --check origin/main...HEAD
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
python tools/check_release_metadata.py
```

Also inspect current CI and confirm the exact two-path allowlist. Rust tests are
not required for this tranche because it changes no runtime, FORMAT, schema,
fixture, dependency, or CI input. The runtime ticket owns the full Rust,
concurrency, crash, and hostile validation ladder.

## Implementation handoff

The next separately authorized tranche should:

- [ ] add the supported SQLite L0 backend;
- [ ] add exact ownership and schema identity;
- [ ] add explicit creation;
- [ ] add explicit weak verified-prefix reopen;
- [ ] add explicit containment-checkpoint reopen;
- [ ] reuse PR #127 checkpoint semantics on the same stable snapshot;
- [ ] replace unbounded supported-path history duplication with bounded passes;
- [ ] keep SQLite off the whole-vector public `LogStore::read_records` seam
      while preserving existing custom read-store compatibility;
- [ ] atomically compare expected terminal count/tip at append;
- [ ] return stale-writer error with no sibling commit;
- [ ] poison writer after unknown persistence outcome;
- [ ] preserve `LogWriter` sole-write and verified-replay boundaries;
- [ ] preserve FileStore/MemStore and all golden/FORMAT bytes;
- [ ] add P1-P20 hostile evidence; and
- [ ] leave audit/readiness administration unchanged.

The implementation must stop rather than broaden if exact database ownership,
stable-snapshot replay, or uncertain-commit behavior cannot be represented
without reopening an Accepted invariant.

## Explicit non-goals

This ticket does not implement or authorize checkpoint storage, remembered
heads, anti-rollback hardware, WAL, Merkle trees, exact-once retry, automatic
repair/migration, projection co-location, claims/standing integration,
currentness, authority, portable-verifier changes, release changes, or audit
closure.

## Publication authority

The owner instruction authorizes one focused documentation commit, one push,
and one **draft** pull request against `main`. Human owner retains sole merge
and ratification authority. Do not merge, enable auto-merge, or mark ready.

## Stop conditions

Stop and report rather than broadening if:

- the #127 merge is absent from current main;
- ADR-0008/0009 contradicts explicit checkpoint-relative reopen;
- available SQLite cannot atomically compare-and-commit one successor;
- exact local durability cannot be stated under an upstream-supported profile;
- foreign-database refusal requires destructive inspection;
- resource behavior cannot avoid unbounded whole-history duplication;
- safe recovery requires ambient checkpoint selection;
- implementation would require a FORMAT break, claims/standing change, or
  audit/readiness update for correctness; or
- the two-file documentation allowlist cannot be preserved.

## Review checklist

- [ ] Exact #127 base and branch are recorded.
- [ ] SQLite is selected only as physical L0 persistence.
- [ ] `MPL0` ownership is a file marker, not trust or authority.
- [ ] Foreign and projection databases are never dropped or adopted.
- [ ] Creation cannot hide inside reopen.
- [ ] Prefix reopen does not imply expected/fresh history.
- [ ] Checkpoint reopen fixes `ContainsCheckpoint` explicitly and reuses #127.
- [ ] One stable read transaction covers every semantic observation.
- [ ] Resource limits are explicit, finite, and checked before BLOB allocation.
- [ ] Competing writers cannot both commit siblings.
- [ ] Stale writers never refresh or retry implicitly.
- [ ] Commit uncertainty poisons the writer and requires reopen.
- [ ] `Ok` is no stronger than the selected conditional local durability model.
- [ ] Whole-system rollback of history and checkpoint remains possible.
- [ ] FileStore, MemStore, FORMAT, claims, audits, readiness, and release state
      remain unchanged.
