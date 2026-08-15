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
   bounded verification and checkpoint observations;
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
       first read of main establishes the snapshot
write/create = BEGIN IMMEDIATE
```

Ordinary open first queries the current journal mode. Observable persistent
`WAL` is refused without issuing a conversion. A recognized non-WAL database
then explicitly requests `DELETE` and exact-matches the returned mode on every
supported connection. V0 makes no claim that a fresh connection can detect a
discarded historical `PERSIST`, `TRUNCATE`, `MEMORY`, or `OFF` setting, and adds
no durable profile metadata.

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
    consumes explicit finite L0ResourceLimitsV0
    bounds genesis and staged logical database before commit
    initializes ownership + schema + genesis in one transaction

open_verified_prefix
    verifies one explicit stable persisted prefix
    establishes no checkpoint or freshness proposition

open_containing_checkpoint_v0(checkpoint)
    fixes ContainsCheckpoint by the named method
    reuses magpie-log-history-expectation-v0
    accepts no ambient/default relation
    returns a dedicated checkpoint-qualified value carrying both
        the writer
        the genuine Satisfied HistoryExpectationEvaluationV0
```

The exact Rust wrapper name is left to implementation, but its private
construction and exact evaluation carriage are mandatory. The evaluation is
not reconstructed from caller input. Ordinary `open_verified_prefix` carries
no checkpoint baggage. V0 does not add an exact-terminal writer-open method or
a generic result-identity framework.

### Stable snapshot vocabulary

An explicit read transaction begins before semantic database observations. Its
first read of `main` establishes the SQLite snapshot; page/schema/integrity
reads may therefore anchor it before ordered record traversal. Every later
semantic `main` read for that open/evaluation remains in the same transaction,
so a concurrent commit cannot substitute another history partway through.
Connection-profile setup before the transaction does not itself establish this
historical snapshot.

### Resource vocabulary

Every supported operation receives one explicit finite
`L0ResourceLimitsV0`-equivalent value:

```text
max_database_bytes
max_record_count
max_record_bytes
max_total_record_bytes
```

The physical main database file is length-checked before every supported open
of an existing L0, including read-only replay. An associated rollback journal
is length-checked before an operation that may recover or open write-capably.
Inside the stable snapshot, checked `page_count * page_size` remains a separate
logical-page bound before integrity checking. Each record's BLOB length is
checked before BLOB materialization. Resource failure is operational and
produces no checkpoint outcome, verification/open result, or writer.

Creation consumes the same exact immutable limits. Before initialization can
commit, the checked serialized genesis length must satisfy `max_record_bytes`
and `max_total_record_bytes`, one record must satisfy `max_record_count`, and
the staged transaction's checked logical `page_count * page_size` must satisfy
`max_database_bytes`. Failure rolls back, returns no writer, and publishes no
valid partially initialized L0. Successful commit initializes the writer with
count one, the genesis tip, genesis record bytes as its checked cumulative
total, and the exact supplied limits.

A successfully opened writer retains the verified snapshot's checked total
record bytes and the selected limits. Before append it computes checked
`next_total_record_bytes = total_record_bytes + record_length` and rejects
overflow or limit excess before the transaction. Only observed commit success
advances cached count, tip, and total record bytes together; every failed append
advances none. The transaction still rejects a stale physical terminal rather
than trusting cached totals against another writer's history.

### Stale and uncertain writer vocabulary

| Class | Commit meaning | Writer state |
| --- | --- | --- |
| pre-transaction rejection | definitely no transaction and no commit | may remain usable only where the contract permits and the cached predecessor is unchanged |
| `BEGIN IMMEDIATE -> SQLITE_BUSY` | write transaction did not begin; definitely no commit | poisoned under v0; reopen required |
| `WriterStale` | definitely no successor committed by this attempt | poisoned; no refresh/retry; reopen required |
| transaction-crossing failure before `COMMIT` | definitely not committed only if rollback or prior automatic rollback is mechanically established; otherwise state is unknown | poison; force/verify rollback or abandon connection; reopen required |
| `COMMIT -> SQLITE_BUSY` | commit did not complete and the transaction remains active; successful explicit rollback establishes definitely not committed | no commit retry/wait; terminate transaction or abandon connection; poison and reopen |
| `CommitStateUnknown` | transaction may or may not have committed, including uncertain cleanup | abandon connection; poison; reopen and completely verify |
| observed successful `COMMIT` | exact row committed under the fixed local SQLite profile | cached count/tip/total-record-bytes advance together exactly once |

For `COMMIT -> SQLITE_BUSY`, Magpie finalizes/drops transaction-owned statements
and explicitly attempts `ROLLBACK`; it does not use SQLite's permitted later
commit retry. Successful rollback means this append definitely did not commit.
If rollback or terminal transaction cleanup cannot be established, Magpie
abandons/closes the connection and reports `CommitStateUnknown`.

Other transaction-crossing SQLite errors may automatically roll back or may
leave a transaction active. The implementation must inspect or force a terminal
transaction state where possible and otherwise abandon the connection as
unknown. No poisoned writer returned to a caller may retain a live SQLite write
transaction. The writer remains poisoned even after definite rollback under the
selected v0 policy.

`Err` does not universally mean “nothing committed,” and blind retry does not
mean exactly once. `COMMIT -> SQLITE_BUSY` is the narrower case where SQLite
establishes that commit did not complete while leaving the transaction active.

## Hostile proof matrix for the runtime ticket

The implementation must prove all twenty design scenarios:

| ID | Proof target |
| --- | --- |
| P1 | limits admit genesis and staged logical database; atomic creation returns exactly one genesis and coherent writer count/tip/total/limits only after commit |
| P2 | foreign SQLite refusal without mutation |
| P3 | unknown future schema refusal |
| P4 | explicit weak old-prefix reopen |
| P5 | exact-terminal checkpoint containment reopen carries the genuine satisfied evaluation with its writer |
| P6 | valid suffix beyond checkpoint succeeds under containment and preserves the exact genuine evaluation |
| P7 | valid below-checkpoint history verifies but open refuses |
| P8 | equal/longer wrong branch refuses |
| P9 | two writers from one predecessor cannot commit siblings |
| P10 | stable reader forces writer `COMMIT -> SQLITE_BUSY`; no wait/retry, explicit rollback/termination, poisoned writer, no row from that attempt, unchanged reader snapshot, and only deliberate reopen/new writer may later append |
| P11 | pre-COMMIT failure forces/verifies rollback or abandons the connection; transaction-crossing failure poisons writer |
| P12 | commit-before-observed-ack process death |
| P13 | unknown commit poisons writer |
| P14 | corrupt DB/event open failure without repair |
| P15 | history and external checkpoint rollback together remains undetected |
| P16 | older history with newer retained checkpoint refuses |
| P17 | projection database refused as L0 |
| P18 | network filesystem outside supported guarantee |
| P19 | create-time genesis/logical-page, physical main-file (including trailing bytes), applicable journal, logical-page, count/record/cumulative-byte, and sequential-append resource failures before publication |
| P20 | no ambient checkpoint selection |

Tests must reach the intended stage. A stale-writer test must show one exact
committed successor; a wrong-branch test must use two otherwise valid chains;
a corrupt-suffix test must verify the checkpoint prefix before the late error;
and a foreign-database test must prove the unrelated data stayed unchanged.

P1/P19 creation evidence must cover `max_record_count < 1`, genesis exceeding
`max_record_bytes`, genesis exceeding `max_total_record_bytes`, initialized
logical pages exceeding `max_database_bytes`, exact-at-limit cases where
practical, successful writer initialization from genesis bytes, and failed
creation leaving no writer or valid partially initialized L0.

Snapshot evidence must show that a required pre-record `main` read can establish
the transaction snapshot and that a concurrent writer cannot make later
verification/checkpoint reads in that same operation observe another history.

Checkpoint-carriage evidence must use two different checkpoints contained by
the same terminal history: writer terminal coordinates may match, but the two
dedicated successful open values must retain distinct genuine evaluations for
the exact checkpoints. Callers must not fabricate such a success; ordinary
prefix open carries no checkpoint result; operational failure produces none.

Journal-profile evidence must show persistent WAL refusal without conversion,
explicit `DELETE` establishment/exact-match on a supported non-WAL database,
and failure when the current profile cannot be established; it must not claim
to detect discarded historical non-WAL settings. P19 evidence must separately
exercise physical main-file size (including a logically valid database padded
beyond the limit and rejected by read-only replay), applicable journal size,
logical pages, record/count/cumulative bytes, and two sequential appends where
the second fails before transaction only when the first successful append's
updated cached total is used. Failed append must leave that cached total
unchanged, while successful commit aligns count/tip/total with persisted
history.

#129 does not expose a general fallible second SQLite cursor pass into arbitrary
current `Projection` values. The trait has no replay-wide atomic staging law,
and existing consumers may commit per-event effects. A future bounded replay
integration may claim no partial publication only with a directly tested
all-or-nothing staging/commit-or-discard boundary covering the entire fallible
pass. No trait or projection redesign is selected here.

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
- [ ] add explicit creation with genesis/count/cumulative-byte and staged
      logical-page limits before commit, then coherent genesis writer state;
- [ ] add explicit weak verified-prefix reopen;
- [ ] add explicit containment-checkpoint reopen whose dedicated success value
      carries both the writer and genuine `Satisfied`
      `HistoryExpectationEvaluationV0`;
- [ ] reuse PR #127 checkpoint semantics on the same stable snapshot;
- [ ] make the first read of `main` inside the explicit read transaction the
      snapshot anchor and keep every later semantic read in that transaction;
- [ ] replace unbounded supported-path history duplication with bounded
      verification/open;
- [ ] keep SQLite off the whole-vector public `LogStore::read_records` seam
      while preserving existing custom read-store compatibility;
- [ ] atomically compare expected terminal count/tip at append;
- [ ] return stale-writer error with no sibling commit;
- [ ] query journal mode before setting, refuse persistent WAL without
      conversion, then explicitly request/exact-match DELETE; do not test for
      discarded historical non-WAL connection modes;
- [ ] enforce physical main-file size before read-only and write-capable opens,
      applicable journal size before recovery, and logical page size separately
      before integrity checking;
- [ ] compute checked next total record bytes before append and advance cached
      count/tip/total together only after successful commit, never after failure;
- [ ] handle `COMMIT -> SQLITE_BUSY` without retry: finalize/drop transaction-
      owned statements, explicitly roll back/terminate, poison writer, and
      abandon the connection if cleanup cannot be established;
- [ ] prove after every public failed append that no retained poisoned writer
      has an active write transaction, using direct transaction/autocommit state
      where the supported SQLite/rusqlite mechanism permits;
- [ ] poison writer after every transaction-crossing or unknown persistence
      outcome;
- [ ] preserve `LogWriter` sole-write and verified-replay boundaries;
- [ ] expose no general arbitrary-`Projection` second-cursor replay in #129;
      any later no-partial-publication claim requires an explicit replay-wide
      atomic staging sink;
- [ ] preserve FileStore/MemStore and all golden/FORMAT bytes;
- [ ] add P1-P20 hostile evidence; and
- [ ] leave audit/readiness administration unchanged.

The implementation must stop rather than broaden if exact database ownership,
stable-snapshot open/evaluation, or uncertain-commit behavior cannot be represented
without reopening an Accepted invariant.

## Explicit non-goals

This ticket does not implement or authorize checkpoint storage, remembered
heads, anti-rollback hardware, WAL, Merkle trees, exact-once retry, automatic
repair/migration, projection co-location, claims/standing integration,
currentness, authority, portable-verifier changes, release changes, or audit
closure. It also does not authorize generic result identity, checkpoint baggage
on ordinary writer opens, general bounded projection replay, or a generic
transactional `Projection` redesign.

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
- [ ] Creation applies all caller-selected limits before genesis commit and a
      returned writer's count/tip/total/limits describe the same acknowledged
      genesis history.
- [ ] Prefix reopen does not imply expected/fresh history.
- [ ] Checkpoint reopen fixes `ContainsCheckpoint`, reuses #127, and carries the
      genuine context-bearing satisfied evaluation in its dedicated success
      value.
- [ ] The first read of `main` inside one explicit read transaction establishes
      the snapshot and every later semantic read stays in that transaction.
- [ ] Resource limits are explicit, finite, and checked before BLOB allocation.
- [ ] Observable persistent WAL is refused without conversion; discarded prior
      non-WAL modes are not claimed observable; each connection establishes
      and exact-matches DELETE.
- [ ] Physical main-file size is checked before read-only and write-capable
      opens independently of logical database pages.
- [ ] Successful append advances cached count/tip/total together; sequential
      budget checks use the updated total and failed appends advance none.
- [ ] #129 claims no atomic bounded replay through arbitrary current
      `Projection`; any later claim is gated on replay-wide commit-or-discard
      staging.
- [ ] Competing writers cannot both commit siblings.
- [ ] Stale writers never refresh or retry implicitly.
- [ ] A busy `COMMIT` is explicitly rolled back/terminated without retry, and no
      poisoned writer retains an active write transaction.
- [ ] Commit uncertainty poisons the writer and requires reopen.
- [ ] `Ok` is no stronger than the selected conditional local durability model.
- [ ] Whole-system rollback of history and checkpoint remains possible.
- [ ] FileStore, MemStore, FORMAT, claims, audits, readiness, and release state
      remain unchanged.
