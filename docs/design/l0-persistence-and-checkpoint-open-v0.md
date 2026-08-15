# Supported L0 persistence and checkpoint-aware open contract v0

## Status and scope

Narrow documentation-only implementation contract for the supported local L0
persistence path.

Resolved baseline:

```text
repository: noctem-o/magpie
origin/main: 2c38ffe32367bbb38d96525b938f148d40eb29d4
branch: docs/l0-persistence-open-contract-v0
runtime baseline: PR #127, verified history checkpoint evaluation v0
administrative targets: A-008 / RQ-007 / RQ-008 / NEW-LOG-01
```

This document selects the contract that a later Rust implementation must
satisfy. It does not implement `SqliteL0Store`, change current `FileStore` or
`MemStore` behavior, add a dependency, change FORMAT, or close any audit or
readiness item. The word **supported** below names the target status after that
implementation and its hostile evidence land; it is not a claim about current
runtime at this documentation-only baseline.

Accepted ADR-0001, ADR-0008, and ADR-0009 remain governing. In particular:

```text
verified supplied history
!= expected history
!= fresh or latest history
!= authority

checkpoint satisfaction
!= secure checkpoint retention
!= universal rollback resistance
```

## Purpose

Freeze the smallest physical-history contract that lets the next runtime
tranche implement, without inventing storage semantics:

- one supported local persistent L0 backend;
- explicit creation distinct from reopening;
- complete verification over one stable physical snapshot;
- explicit weak prefix-relative reopen;
- explicit checkpoint-relative reopen using PR #127 semantics;
- atomic stale-writer detection at append commit;
- stated acknowledgement, crash, and uncertain-commit behavior;
- bounded record iteration rather than unbounded whole-history duplication;
- fail-closed database ownership and schema handling; and
- continued separation of L0 from rebuildable SQLite projections.

The contract preserves `LogWriter` as the sole public Magpie append capability
and `VerifiedReplayEvent` as the only public projection input.

## Existing evidence

At the resolved baseline, current code establishes all of the following:

- `FileStore::read_records` calls `std::fs::read`, splits on LF, discards empty
  slices, copies each retained record, and returns `Vec<Vec<u8>>`;
- `FileStore::append_record` opens with append/create, writes serialized event
  bytes and LF in two separate `write_all` calls, and returns without a lock or
  synchronization call;
- `LogWriter::open` verifies the store once and creates genesis when the
  returned record vector is empty;
- `LogWriter::append` advances its cached count and tip only after the current
  backend reports success;
- the crate-private `WriterStore::append_record` receives only record bytes; it
  receives no expected count or predecessor, and its `Result<(), LogError>`
  cannot distinguish definitely-not-committed from commit-state-unknown;
- `replay_with_summary` reads one record vector, completely verifies that exact
  vector, retains parsed events, and applies those same events only after full
  verification;
- PR #127 evaluates `HistoryCheckpointV0` under an explicit `Exact` or
  `ContainsCheckpoint` relation after complete verification of the same one-read
  snapshot; and
- workspace metadata already pins `rusqlite 0.37` with bundled SQLite, although
  only `magpie-episodic` currently consumes it.

Those are exact present facts, not a durability or concurrency contract.
Current `FileStore` is not described here as broken. It is a small JSONL store
whose stronger physical guarantees have not been selected or implemented.

The current episodic SQLite projection is cautionary evidence, not an L0
template. It treats schema mismatch as permission to drop derived tables and
has no application-ownership marker. Destructive reset is acceptable only for
an already-owned disposable projection; it is forbidden for conserved L0.

## Failure and threat model

The v0 supported profile reasons about:

- multiple processes on one host opening the same local database;
- a stable read transaction concurrent with a later writer;
- lock contention and `SQLITE_BUSY`;
- process interruption before, during, or after commit;
- ordinary OS or filesystem I/O errors;
- disk-full and SQLite operational errors;
- rollback-journal recovery after process or ordinary OS interruption;
- power loss subject to SQLite's selected sync mode and a conforming local
  filesystem/storage stack;
- malformed or cryptographically invalid stored event bytes;
- a foreign, unowned, projection, or unknown-version SQLite database; and
- explicit finite resource-budget exhaustion.

The supported profile assumes a local filesystem whose locking and sync
operations behave as documented. It excludes:

- a malicious or compromised kernel;
- a storage controller or device that acknowledges synchronization falsely;
- arbitrary hardware corruption outside SQLite's recoverable model;
- out-of-band SQL or filesystem mutation while a writer is in use;
- deletion, relocation, aliasing, or cleanup of a database or its hot rollback
  journal during recovery;
- network, distributed, cluster, or cloud-synchronized filesystem semantics;
- a compromised signing key;
- physical rollback of both L0 and the externally retained checkpoint;
- remote split view, witness, gossip, consensus, and non-equivocation; and
- availability under arbitrary resource exhaustion.

An excluded failure can still be detected later by SQLite or Magpie
verification. Detection does not turn it into a supported guarantee.

## Falsification record and selected architecture

The SQLite recommendation survives current-repository falsification:

| Challenge | Evidence and result |
| --- | --- |
| Does SQLite become canonical event identity? | No. FORMAT §6 explicitly makes storage non-normative. Event hash/signature identity remains derived from typed canonical bytes. |
| Does SQLite introduce ambient semantic input? | No. The store supplies physical historical input; the exact verification key, checkpoint, and relation remain explicit. SQLite page layout and schema are operational representation, not `G` or semantic `C`. |
| Can current `WriterStore` enforce stale-writer atomicity? | No. It lacks expected terminal coordinates and commit-state vocabulary. The supported backend requires a new private transactional seam, not a weakening of `LogWriter`. |
| Can current `read_records` satisfy the resource law? | No. It duplicates the whole history. The supported backend requires a stable cursor snapshot with bounded passes and may not delegate open/replay through that method. |
| Can SQLite serialize competing appends? | Yes. One `BEGIN IMMEDIATE` write transaction can compare the physical terminal coordinate and insert one successor atomically; another writer then observes contention or a different terminal coordinate. |
| Does the repository need a new third-party dependency? | No. Bundled `rusqlite 0.37` is already workspace-pinned. The runtime tranche will deliberately add the existing workspace dependency to `magpie-log`. |
| Does current doctrine forbid SQLite in `magpie-log`? | No Accepted ADR does. The current CLAUDE crate-layout sentence that SQLite is only in `magpie-episodic` describes the pre-contract layout and must be reconciled mechanically when the implementation lands. |
| Is WAL required? | No. The required local invariants are satisfied by rollback-journal transactions; WAL is not part of Magpie's event protocol. |
| Is a Merkle structure required? | No. The existing linear hash chain transitively commits each tip to its ancestry, and PR #127 checks the exact commitment at the exact checkpoint position. |
| Can unbounded whole-history allocation remain supported? | No. The selected resource law requires explicit finite caller budgets and record-at-a-time passes over one stable snapshot. |

The selected architecture is therefore:

```text
SqliteL0Store
    = supported local persistent L0 boundary

FileStore
    = retained JSONL compatibility, development, fixture, export,
      and portable-inspection surface

MemStore
    = in-memory test and embedding substrate
```

SQLite is the persistence mechanism for the conserved L0 history. It is not an
epistemic authority and does not replace Magpie event verification.

## Supported SQLite profile

The v0 implementation must configure and verify one exact local connection
profile after L0 ownership has been established:

| Coordinate | Required v0 value and behavior |
| --- | --- |
| Filesystem | local filesystem only; remote/network-mounted database access is outside the supported guarantee |
| Database attachment | one `main` database; no `ATTACH` and no extension loading |
| Shared cache | disabled |
| Uncommitted reads | `read_uncommitted = OFF` |
| Journal | rollback journal, `journal_mode = DELETE`; the returned mode must exact-match |
| Synchronization | `synchronous = EXTRA` on every connection; do not rely on a build or connection default |
| Locking | normal SQLite locking, not a process-lifetime exclusive lease |
| Stable reads | explicit read transaction; the first ordered record query establishes the snapshot, and the transaction remains open through every required pass |
| Appends and creation | `BEGIN IMMEDIATE` before inspecting or changing the persisted terminal coordinate |
| Busy handling | zero automatic busy timeout in v0; lock contention returns a typed operational failure and performs no hidden retry |
| Application identity | `PRAGMA application_id = 0x4D504C30` (ASCII `MPL0`) |
| Schema version | `PRAGMA user_version = 1` |
| Database integrity | full `PRAGMA integrity_check`; exactly one result row equal to `ok` is required before event verification |

`synchronous=EXTRA`, rather than merely `FULL`, is selected because SQLite's
rollback-journal documentation makes the extra directory synchronization
relevant to durability of a just-committed transaction in `DELETE` mode. This
selection is an operational persistence profile, not FORMAT or event semantics.

Every pragma whose returned value can differ from the requested value must be
queried and exact-matched. An unsupported or refused setting is an open/create
failure, not permission to continue under an implementation default.

The implementation must not silently convert an existing recognized database
from WAL or another journal profile during ordinary open. A profile mismatch
fails closed. Creation establishes the selected profile. Migration to another
profile requires a later reviewed contract.

## Database ownership and schema identity

The application ID is a file-type/ownership marker, not authentication. It
does not make bytes trusted. Its purpose is to prevent Magpie from adopting or
destroying an arbitrary SQLite file.

The ownership law is:

```text
absent or exactly empty/unowned destination
    + explicit create_new
    -> may initialize

recognized MPL0 application_id
    + exact schema version 1
    + exact v1 schema fingerprint
    -> may proceed to verification

foreign/non-SQLite/non-empty unowned database
    -> refuse without mutation

MPL0 application_id + unsupported user_version
    -> refuse without migration

MPL0 application_id + unexpected user schema objects
    -> refuse without DROP, replacement, or repair
```

Before creating any write-capable SQLite connection for an existing non-empty
path, the implementation must inspect the SQLite header read-only and exact-
match the SQLite magic, `MPL0` application ID, and v1 user version. A foreign
application ID, unsupported version, or non-SQLite file is rejected before
SQLite can perform recovery or another write. An explicit create operation may
inspect an existing empty SQLite file through a non-mutating preflight, but
must reject any user object before initialization.

Once the application ID claims `MPL0`, ordinary SQLite hot-journal recovery may
run. The implementation then exact-matches `user_version`, the table/index/
trigger/view allowlist, column declarations, constraints, and the selected
connection profile before event verification. A deliberately forged `MPL0`
marker is not authority; at worst it moves the file into Magpie's claimed-file
validation path, where schema and signed-history checks still fail closed.

There is no v0 automatic migration, reset, truncation, salvage, or table drop.
Unknown future schema is refused. Repair and export are separate tools and may
not occur as an ordinary open side effect.

An episodic/projection database has no `MPL0` identity and is never accepted as
L0. L0 and derived stores remain separate files and separate connections.

## Exact records and physical schema

The normative v1 physical schema for the runtime tranche is the following
minimal shape:

```sql
CREATE TABLE magpie_l0_records (
    position_be BLOB NOT NULL PRIMARY KEY CHECK(length(position_be) = 8),
    record_bytes BLOB NOT NULL
) STRICT, WITHOUT ROWID;
```

Together with the exact application ID and user version above, this is the
complete allowed user schema. No head table, projection table, mutable status
row, remembered checkpoint, trigger, view, or auxiliary semantic column is
part of v1.

`position_be` is the event's `u64` sequence encoded as exactly eight big-endian
bytes. This preserves the FORMAT domain and makes lexicographic ordering equal
numeric ordering without narrowing positions to SQLite's signed integer range.

`record_bytes` is the exact serialized `SignedEvent` record byte slice produced
by `LogWriter`—the same record content currently passed to `WriterStore` before
FileStore adds LF. It is stored as a BLOB with no JSONL delimiter.

The signed record is the semantic source. The physical position is only an
ordering/index duplicate and must exact-match `SignedEvent.core.seq` during
verification. Any disagreement is corruption and open fails. Event hash,
predecessor, count, genesis key/profile, and signature are always recomputed or
verified from the exact stored record. No auxiliary cached count or hash may be
trusted as an independent source of L0 truth.

The supported public API exposes no SQL connection and no update/delete/raw
insert operation. The only permitted row mutation is one insert performed by
`LogWriter`'s private persistence path. Direct external SQL or file mutation is
outside the supported writer model and is detected where later verification or
terminal comparison can detect it.

This schema is a Magpie storage representation, not `magpie-core-v2`, not a
portable wire format, and not an amendment to FORMAT §6.

## Creation semantics

The supported persistent path must expose an explicit operation conceptually
named:

```text
LogWriter::<SqliteL0Store>::create_new(...)
```

Exact Rust parameter grouping may follow crate style, but the operation and its
separation from open are fixed.

Creation must:

1. accept only an absent path, zero-byte file, or exact empty/unowned SQLite
   destination with no user objects;
2. reject a non-empty foreign, projection, or already-owned database without
   mutation;
3. establish and exact-match the connection-local profile and `DELETE` journal
   mode while the explicitly selected destination is still empty/unowned;
4. start one `BEGIN IMMEDIATE` initialization transaction;
5. establish the `MPL0` identity, schema version, and exact v1 table in that
   transaction;
6. create one genesis event under the supplied `SigningKey`, existing
   `magpie-core-v1` rules, and the supplied/injected clock;
7. insert that exact genesis record at position zero in the same transaction;
8. commit ownership, schema, and genesis atomically; and
9. return a writer only after `COMMIT` reports success.

No recognized supported L0 database is valid with zero records: initialization
and genesis are one transaction. A failed or interrupted create returns no
writer. If commit acknowledgement is uncertain, the caller must inspect the
path through an explicit reopen; it must not blindly call `create_new` again.

Ordinary reopen may never create schema or genesis. This intentionally differs
from current `LogWriter<FileStore>::open` and `LogWriter<MemStore>::open`, whose
compatibility behavior remains unchanged by this contract.

The supported backend exposes no ambiguous bare `SqliteL0Store::new(path)` that
silently creates, initializes, repairs, or opens for writing. Public entry must
select creation, verified-prefix reopen, checkpoint-relative reopen, or an
explicit read-only operation.

## Prefix-relative reopen

The deliberately weaker supported reopen mode is conceptually:

```text
LogWriter::<SqliteL0Store>::open_verified_prefix(...)
```

It must:

1. require an existing recognized exact-v1 `MPL0` database;
2. take the explicit signing key, derive its exact verifying key, and use that
   key as the external verification coordinate;
3. begin one stable read transaction;
4. check database integrity/schema/profile without repair;
5. iterate the ordered records under the explicit finite resource budget;
6. completely verify sequence, predecessor, hash, signature, payload, genesis
   profile, and genesis key binding;
7. derive the exact terminal `{event_count, tip}` and checked total record-byte
   count for that snapshot;
8. end the read transaction only after successful verification; and
9. return a writer cached against exactly that terminal coordinate and total
   record-byte count.

This mode establishes only:

```text
the exact physical history observed during open
was one completely valid supplied prefix under the supplied key
```

It does not establish that the prefix is expected, fresh, latest,
rollback-resistant, globally complete, canonical, or unique. Its explicit name
must retain `verified_prefix`; an unqualified `open`, `open_current`, or
`open_latest` is forbidden for this supported backend.

An empty, malformed, corrupt, or wrong-key recognized database fails open. No
ordinary open truncates a bad tail, manufactures genesis, or picks a shorter
valid prefix.

## Checkpoint-relative reopen

The stronger v0 mode is conceptually and semantically named:

```text
LogWriter::<SqliteL0Store>::open_containing_checkpoint_v0(
    ...,
    checkpoint: HistoryCheckpointV0,
)
```

The method identity deliberately fixes
`HistoryExpectationRelationV0::ContainsCheckpoint`. There is no relation
default: selecting this method selects one named relation-specific open mode,
and its implementation still constructs the explicit enum value consumed by
`magpie-log-history-expectation-v0`. An unqualified `open_with_checkpoint` is
not sufficient. V0 adds no speculative exact-terminal writer-open variant.

The method must perform all prefix-relative verification steps and, within the
same stable read transaction and over the same verified records, evaluate the
explicit caller-supplied checkpoint using PR #127's exact profile semantics.
It may return a writer only when the genuine evaluation outcome is
`Satisfied`.

The implementation must not copy or subtly fork PR #127's relation logic. It
may refactor the current private observer/evaluation kernel so both the current
one-read API and the SQLite stable-snapshot path construct the same
`HistoryExpectationEvaluationV0`. It must not obtain a second store snapshot or
expose retained raw events publicly.

`NotSatisfied` refuses writer construction but does not recast the physical
history as invalid. The typed open error must distinguish checkpoint mismatch
from verification and operational failure. If the semantic mismatch result is
returned across the open boundary, it must carry the genuine context-bearing
`HistoryExpectationEvaluationV0`, never a bare boolean or caller-fabricated
enum.

The checkpoint is explicit caller input. No checkpoint row, sidecar file,
environment variable, database alias, callback, “latest” lookup, or L0 head is
consulted to select it.

## Checkpoint reopen is not universal rollback resistance

Checkpoint-aware reopen establishes only:

> This exact completely verified persisted history contains the exact explicit
> checkpoint commitment at the exact checkpoint position under the fixed v0
> relation and verification context.

It usefully rejects `H80` when a separately retained `C100` is supplied, and it
rejects an equal-or-longer sibling branch that lacks `C100`'s exact commitment.

It does not detect:

```text
yesterday: H100 + external C100
restore both from an older backup: H80 + external C80
today: H80 verifies and contains C80
```

Secure checkpoint retention, anti-rollback storage, witnessing, and checkpoint
source trust remain separate later work.

## Stable-snapshot law

Every semantic observation made by one supported open/evaluation operation
must refer to one SQLite read transaction:

```text
begin stable read transaction
pass A: ordered bounded verification of the entire snapshot
        + record at most the requested checkpoint commitment
only after pass A succeeds: produce verification/checkpoint conclusion
optional pass B: replay the same ordered snapshot into private staging
end read transaction
publish result / construct writer
```

The read transaction must remain open across every required pass. A concurrent
commit after the snapshot begins remains outside that snapshot. The returned
writer is cached against the snapshot terminal coordinate and must compare it
again atomically when appending.

Under the selected `DELETE` rollback-journal profile, a reader can delay or
cause a prompt busy failure at another connection's commit until the reader
releases its transaction. V0 accepts that lower write concurrency. It does not
switch to WAL or weaken the snapshot law merely to allow a writer to commit
while pass A or pass B is active.

The contract forbids:

```text
read A -> verify
read B -> compare checkpoint
read C -> replay
```

It also forbids applying even an early valid record to a public projection
before complete pass-A verification. If replay is requested, pass B applies
only `VerifiedReplayEvent` wrappers from the same stable snapshot and publishes
no partial projection on failure.

The current `LogStore::read_records` and current retained-vector replay APIs may
remain for FileStore, MemStore, and compatibility callers. The supported SQLite
path must use a private stable-snapshot cursor seam and must not implement
bounded open by materializing through `read_records`.

`SqliteL0Store` must not implement the current public `LogStore` trait merely
to gain access to generic `LogReader` methods, because that would make whole-
history `Vec<Vec<u8>>` materialization the apparent supported SQLite read
contract. The runtime may move the `S: LogStore` bound from the `LogReader`
struct to its existing compatibility impl and add a specialized
`LogReader<SqliteL0Store>` impl, or use an equivalently small private layout.
The semantic requirements are fixed: public custom read stores remain valid;
the SQLite cursor/transaction seam remains crate-private; and no public raw SQL,
unverified cursor, or stable-snapshot minting surface is added.

## Resource and bounded-read law

No universal numeric limit can be derived honestly from current repository
evidence. V0 therefore freezes explicit finite caller-selected operational
budgets rather than an arbitrary hidden constant.

Every supported create/open/replay operation must receive an immutable value
conceptually named `L0ResourceLimitsV0` with four positive finite `u64`
members:

```text
max_database_bytes
max_record_count
max_record_bytes
max_total_record_bytes
```

There is no ambient/default resource profile. The writer retains the same
limits and the verified snapshot's checked total record-byte count for append;
a deliberate reopen may select a different finite budget.

The implementation must:

- reject an existing main database file or associated rollback-journal file
  whose logical file length exceeds `max_database_bytes` before a write-capable
  SQLite open or recovery attempt;
- inside the stable snapshot, reject checked `page_count * page_size` above
  `max_database_bytes` before full integrity checking or record iteration;
- count ordered rows with checked arithmetic;
- query/check each BLOB length before materializing that BLOB;
- reject a record whose length exceeds `max_record_bytes` before allocation;
- reject before fetching a record that would make cumulative bytes exceed
  `max_total_record_bytes`;
- reject before processing a row that would exceed `max_record_count`;
- retain at most the current raw record, current parsed event, verification
  state, exact terminal state, and explicitly requested checkpoint observation
  during pass A;
- use checked conversions between SQLite lengths, Rust allocation sizes, and
  Magpie `u64` coordinates; and
- return a typed operational resource failure with no verification summary,
  checkpoint `D`, writer, or projection publication.

Before starting an append transaction, the writer must reject a proposed
record when checked `event_count + 1` or `total_record_bytes + record_length`
would exceed its retained limits. After inserting but before commit, it must
also check the transaction's resulting `page_count * page_size` and roll back
if `max_database_bytes` would be exceeded. The later transactional terminal-
coordinate comparison prevents a concurrently advanced database from making
cached totals authoritative for another history.

The schema preserves the full eight-byte sequence domain. V0's event-count
coordinate can represent at most `u64::MAX` events, so appending when the count
is already `u64::MAX` is a definite pre-transaction `SequenceExhausted`
failure. No wrap or narrowing to `i64` is allowed.

These laws bound Magpie-owned whole-history materialization. They do not claim
an absolute process-memory ceiling for SQLite, serde, the allocator, or an
arbitrary downstream projection. Projection-specific resource growth remains a
separate consumer obligation.

## Concurrent-writer and stale-writer law

The central invariant is:

> For one persisted terminal coordinate, no two conflicting successor appends
> may both successfully commit.

Two processes may both hold writers opened at `{count=N, tip=T}`. A permanent
exclusive-open lease is not required. Each append must execute one transaction:

```text
construct and serialize proposed successor for {N, T}
BEGIN IMMEDIATE
derive actual terminal coordinate from exact persisted records

if actual != {N, T}:
    ROLLBACK
    return WriterStale
    commit nothing

insert exactly one row at sequence N
COMMIT

only after observed commit success:
    advance writer cached count/tip
    return Ok(SignedEvent)
```

The compare and insert occur in the same write transaction. Position uniqueness
is defense in depth; comparing only row absence or count is insufficient. The
actual terminal commitment is derived from the exact stored terminal record,
not a mutable head alias.

If writer A commits first, writer B's later attempt from the same predecessor
must return typed `WriterStale` and insert nothing. The B event is not rewritten
onto A's tip. The writer does not refresh, retry, or select another predecessor.
The caller must deliberately reopen and create a new append request.

Lock contention before `BEGIN IMMEDIATE` succeeds is a typed operational
failure. In particular, `BEGIN IMMEDIATE -> SQLITE_BUSY` starts no transaction
and commits nothing. V0 nevertheless poisons the affected writer and performs
no hidden wait or retry because another writer may commit during that interval.
Reopen is required before a later append attempt.

## Writer state and failure classes

The supported append surface must distinguish at least these operational
classes, even if exact Rust variant names differ:

| Class | Commit meaning | Writer state |
| --- | --- | --- |
| pre-transaction validation/serialization/resource rejection | definitely no transaction and no commit | may remain usable when its cached predecessor is unchanged |
| `BEGIN IMMEDIATE -> SQLITE_BUSY` | the write transaction did not begin; definitely no commit | unusable under v0; reopen required |
| `WriterStale` | definitely no new row from this attempt | unusable; reopen required |
| transaction-crossing failure before `COMMIT` | definitely not committed only when rollback or prior automatic rollback is mechanically established; otherwise state is unknown | unusable; force/verify rollback or abandon the connection; reopen required |
| `COMMIT -> SQLITE_BUSY` | commit did not complete and SQLite leaves the transaction active; successful explicit rollback establishes definitely not committed, while uncertain cleanup becomes `CommitStateUnknown` | unusable; never retry `COMMIT`; terminate the transaction or abandon the connection; reopen required |
| `CommitStateUnknown` | transaction may or may not have committed | unusable; connection abandoned; reopen and complete verification required |
| observed successful commit | the exact row committed under the selected profile | cached count/tip advance exactly once |
| `WriterPoisoned` on a later call | no append is attempted | reopen required |

After a write-transaction attempt begins, any returned failure poisons the
writer in v0. An implementation may report “definitely not committed” only
when it can mechanically prove that fact without inference—for example, a
pre-insert rejection or an observed successful rollback before any commit
attempt.

Before returning from any transaction-crossing failure, the implementation
must finalize or drop transaction-owned statements and use SQLite transaction
state, autocommit state, or an equivalently direct supported mechanism to
determine whether the transaction already rolled back. If a write transaction
remains active, Magpie must attempt an explicit `ROLLBACK`. An observed
successful rollback establishes that this append did not commit. If a terminal
non-write-transaction state cannot be established, Magpie must abandon/close
the connection and report `CommitStateUnknown`. No poisoned writer returned to
a caller may retain a live SQLite write transaction.

`COMMIT -> SQLITE_BUSY` is a special, stronger-information case under the
selected rollback-journal profile: SQLite did not complete the commit, leaves
the transaction active, and permits a later commit retry. Magpie v0 deliberately
does not take that retry path. It finalizes/drops transaction-owned statements,
attempts explicit `ROLLBACK`, poisons the writer, and requires reopen. If
rollback is observed successful, the append is definitely not committed. If
rollback or connection cleanup cannot be established, Magpie abandons/closes
the connection and conservatively reports `CommitStateUnknown`.

Other transaction-crossing failures are not assigned that same factual state.
SQLite may automatically roll back some `SQLITE_FULL`, `SQLITE_IOERR`,
`SQLITE_NOMEM`, `SQLITE_BUSY`, or `SQLITE_INTERRUPT` failures, or may leave the
transaction active depending on the statement and failure point. Magpie must
inspect/force a terminal state where possible and otherwise abandon the
connection as unknown; it must never infer either commit or rollback merely
from a broad error class.

This supersedes the current generic test assumption that every persistence
`Err` means bytes were not stored and the same writer may retry. That assumption
remains valid only for current deterministic MemStore/FileStore instrumentation
whose injected failure is known to occur before mutation; it is not sufficient
for a transactional backend.

No public caller may clear the poison bit, replace the cached predecessor, or
extract a mutable SQL connection from `LogWriter`.

## Append acknowledgement and durability

For the supported v0 backend:

```text
Ok(event)
= SQLite COMMIT reported success for the exact one-row append transaction
  under journal_mode=DELETE and synchronous=EXTRA
```

Subject to SQLite's documented rollback-journal behavior and a conforming local
filesystem, OS, and storage device, this is the selected local durability
acknowledgement. It does not promise survival against a lying controller,
malicious kernel, arbitrary media corruption, namespace damage, or deletion of
the database/journal files.

Caller observation is later than physical commit. A process can die after the
transaction commits but before `Ok` reaches the caller. Therefore:

```text
Err or lost response
!= nothing committed

blind retry
!= exactly once
```

After any unknown outcome, the safe sequence is:

```text
drop/abandon writer
reopen recognized database
allow SQLite recovery
completely verify exact persisted history
inspect actual terminal coordinate and event content
decide a new operation
```

Magpie makes no automatic idempotency or exactly-once claim for retries.

## Crash and recovery semantics

On supported local storage, SQLite rollback-journal recovery is the physical
transaction recovery mechanism. Magpie's reopen then supplies the semantic
recovery check.

Ordinary reopen must:

1. preserve the database and any hot `-journal` file at their original paths;
2. allow SQLite to complete applicable hot-journal recovery only after the raw
   `MPL0` ownership gate;
3. exact-match schema and connection profile;
4. run full `PRAGMA integrity_check` and require the single exact result `ok`;
5. completely verify all exact ordered `record_bytes` under the supplied key;
6. apply any explicit checkpoint relation; and
7. construct a writer only after every required step succeeds.

A crash before transaction commit may recover to the old history. A crash after
commit may reveal the new history. Either is compatible with atomic
transaction semantics; the caller's missing acknowledgement is the ambiguity.

Corrupt database pages, malformed records, bad signatures, chain gaps, wrong
genesis, or schema mismatch fail open. Ordinary open performs no silent
truncation, tail salvage, repair, or branch selection.

## L0 and derived SQLite remain structurally separate

The supported topology is:

```text
magpie-l0.db
    conserved signed L0 records
    MPL0 application identity
    no projection tables

projection-*.db
    disposable/rebuildable derived state
    never an L0 terminal/head source
```

An L0 create/open operation must not share a connection, file, transaction,
application ID, schema, or migration path with `magpie-episodic`. A projection
database is refused as foreign L0 and is never dropped or repurposed.

SQLite page layout, rowid behavior, query planning, and projection schema do
not affect event hash/signature identity or checkpoint semantics.

## FileStore and MemStore boundaries

`FileStore` remains the simple JSONL compatibility/development/inspection
surface. It remains useful for human inspection, exports, frozen fixtures, and
portable verifier input. This contract does not alter its current create-on-
empty behavior or claim new locking, framing, crash recovery, concurrent-writer
safety, stable multi-pass snapshot, durable acknowledgement, or bounded-read
semantics for it.

`MemStore` remains the deterministic in-memory test/embedding substrate. Its
shared `Rc<RefCell<...>>` behavior is not persistence or cross-process
concurrency.

Neither store is deleted or redesigned by this contract. The runtime tranche
must preserve their existing tests and public behavior unless a separately
reviewed compatibility change is explicitly authorized.

## Explicit non-implications

Successful create, append, prefix reopen, or checkpoint-relative reopen does
not establish:

- freshness or latest history;
- global completeness or canonical branch selection;
- absence of a sibling or unseen suffix;
- non-equivocation, consensus, witness agreement, or federation;
- checkpoint authenticity, trusted source, authority, or secure retention;
- claim truth, standing, currentness, admission, or origin authority;
- universal rollback resistance;
- remote/network filesystem safety;
- survival against a malicious kernel or dishonest storage hardware;
- portable SQLite file bytes or a new Magpie canonical format; or
- availability under arbitrary resource exhaustion.

An explicitly supplied retained checkpoint can detect a substituted history
that lacks that checkpoint. That is the complete stronger proposition.

## Hostile scenario matrix

| ID | Scenario | Required result |
| --- | --- | --- |
| P1 | Clean creation at a new target | Ownership, exact schema, and one valid genesis commit atomically; writer returns only after observed commit success. |
| P2 | Existing unrelated SQLite database | Raw ownership preflight refuses before a write-capable SQLite open; no table is dropped, replaced, or adopted. |
| P3 | `MPL0` identity with unknown future schema version | Refuse without migration or mutation. |
| P4 | Valid old persisted prefix, explicit `open_verified_prefix` | Open may succeed relative to that exact verified prefix; no freshness or expectation claim. |
| P5 | Persisted history terminates exactly at explicit checkpoint | `open_containing_checkpoint_v0` verifies and succeeds under reflexive `ContainsCheckpoint`. |
| P6 | Persisted history contains checkpoint plus valid suffix | Containment-based open succeeds; suffix does not require terminal equality. |
| P7 | Valid persisted history ends below checkpoint | Verification succeeds, checkpoint is `NotSatisfied`, writer construction is refused; history is not called invalid. |
| P8 | Equal/greater-length sibling fork before checkpoint | Refuse because exact commitment is absent at exact position; length is not ancestry. |
| P9 | Writers A and B open at the same `{N,T}`; A commits, B proposes sibling | B returns `WriterStale`, inserts nothing, and cannot silently refresh or retry. |
| P10 | Reader R holds one stable read snapshot; writer W reaches `COMMIT`, which immediately returns `SQLITE_BUSY` because R holds the required read lock | W does not wait or retry `COMMIT`; it finalizes/drops transaction-owned statements, rolls back or terminates the transaction under the v0 cleanup law, and is poisoned. No row from W's attempt commits, no live transaction leaks, and R continues observing its original snapshot. Only a deliberately reopened/new writer may append after R releases the transaction. |
| P11 | Failure before `COMMIT`, including one after the write transaction began | No row commits when rollback is mechanically established. The writer is poisoned after transaction crossing; uncertain cleanup abandons the connection and returns `CommitStateUnknown`. Reuse is allowed only for an entirely pre-transaction rejection with unchanged predecessor. |
| P12 | Process dies after durable commit before caller observes `Ok` | Reopen may reveal the event. No blind retry or exactly-once claim is made. |
| P13 | Commit result unknown | Writer is poisoned; no further append; reopen and complete verification are mandatory. |
| P14 | Corrupt SQLite page or corrupt/malformed event bytes | Open fails operationally or at verification. No repair, truncation, checkpoint `D`, replay publication, or writer. |
| P15 | Whole system restores both H100/C100 to H80/C80 | H80 may verify and satisfy C80. Universal rollback resistance is explicitly not established. |
| P16 | History restored to H80 while separately retained C100 survives | Checkpoint-aware open refuses because H80 does not contain C100. |
| P17 | Episodic/projection database supplied as L0 | Refuse by ownership/schema gate without mutation. |
| P18 | Database path is on a network filesystem | Outside the supported v0 guarantee; reject when mechanically identifiable and otherwise document the violated precondition. No network-FS durability/concurrency claim. |
| P19 | Database/journal/record/count/total-byte budget would be exceeded | Typed operational resource failure before recovery, oversized BLOB materialization, or commit as applicable; no semantic result, writer, or partial projection. |
| P20 | Caller omits checkpoint and expects a database/environment “latest” value | No checkpoint-relative open exists. Checkpoint input must be explicit; no lookup or fallback occurs. |

## Implementation obligations for the runtime tranche

- [ ] Add a supported `SqliteL0Store` in `magpie-log` using the already-pinned
      workspace `rusqlite` dependency.
- [ ] Preserve `LogWriter` as the sole public append capability; expose no raw
      SQL connection or record insert.
- [ ] Add a private transactional persistence seam carrying the writer's exact
      expected count and predecessor commitment.
- [ ] Implement the exact `MPL0` raw-header ownership gate, schema version, and
      v1 schema fingerprint.
- [ ] Require full `PRAGMA integrity_check` success before event verification.
- [ ] Keep L0 physically separate from `magpie-episodic` databases.
- [ ] Implement explicit `create_new`; do not create through ordinary reopen.
- [ ] Implement explicit weak `open_verified_prefix`.
- [ ] Implement relation-specific `open_containing_checkpoint_v0` with an
      explicit `HistoryCheckpointV0` and fixed `ContainsCheckpoint` relation.
- [ ] Reuse/refactor PR #127's private evaluator semantics; do not duplicate
      relation law or perform a second physical read.
- [ ] Implement one stable read transaction with bounded record-at-a-time
      passes and explicit `L0ResourceLimitsV0`.
- [ ] Keep `SqliteL0Store` off the whole-vector `LogStore::read_records` seam;
      preserve that public trait for existing compatibility readers through a
      specialized reader/private snapshot layout.
- [ ] Check main/journal file size before recovery, database pages before
      integrity checking, and record/cumulative lengths before BLOB allocation.
- [ ] Verify complete history before checkpoint outcome, replay application,
      projection publication, or writer construction.
- [ ] Configure and exact-match `DELETE`, `EXTRA`, no shared cache,
      `read_uncommitted=OFF`, and zero automatic busy timeout.
- [ ] Atomically compare the persisted terminal coordinate and insert one
      successor under `BEGIN IMMEDIATE`.
- [ ] Return typed stale-writer failure and prohibit silent refresh/retry.
- [ ] On `COMMIT -> SQLITE_BUSY`, perform no commit retry or wait; finalize/drop
      transaction-owned statements, explicitly roll back, poison the writer,
      and abandon the connection as `CommitStateUnknown` if terminal cleanup
      cannot be established.
- [ ] After every public failed append, directly establish that no active write
      transaction remains before retaining a connection; otherwise close or
      abandon that connection and require reopen.
- [ ] Poison writer after every transaction-crossing or ambiguous persistence
      failure; expose `CommitStateUnknown` where commit cannot be proved absent.
- [ ] Advance cached count/tip only after observed successful commit.
- [ ] Preserve raw-versus-verified replay type state and no-partial-apply law.
- [ ] Preserve FileStore/MemStore behavior and existing golden/FORMAT bytes.
- [ ] Add process/concurrency/crash/resource/foreign-database hostile tests.
- [ ] Reconcile the current crate-layout note that says SQLite is episodic-only
      when the runtime dependency actually moves; do not pretend the old layout
      remains current.
- [ ] Do not edit audit/readiness dispositions in the runtime PR.

## Required runtime proof families

The implementation PR must include direct evidence for every P1-P20 row and at
least:

- two independent database connections/processes attempting sibling appends;
- stale writer exact error plus unchanged row set;
- stable reader causing `COMMIT -> SQLITE_BUSY`, with no wait or commit retry,
  explicit rollback/transaction termination, a poisoned writer, no committed
  row from that attempt, an unchanged reader snapshot, and append allowed only
  through a deliberately reopened/new writer after the reader releases;
- wrong-branch and below-checkpoint open refusal with successful history
  verification kept distinct;
- a matching checkpoint followed by a late corrupt record producing no `D`;
- zero second physical snapshot reads for checkpoint open;
- foreign SQLite and projection database byte/hash preservation after refusal;
- unknown-schema refusal without migration;
- zero/exact/over-bound database, recovery-journal, record, count, and total-
  byte budgets, with a test proving oversized BLOB bytes are not fetched or
  materialized;
- process termination before commit and after commit-before-ack where feasible;
- explicit unknown-commit writer poisoning and later-call rejection;
- direct transaction/autocommit-state evidence after every public failed append
  proving that no retained poisoned writer has an active SQLite write
  transaction, rather than inferring cleanup from row count alone;
- create/open non-substitutability;
- no public arbitrary result, raw insert, mutable connection, or writer-state
  constructor; and
- complete existing workspace/golden/portable verification preservation.

Failure injection must prove the intended stage. A wrong-checkpoint test must
otherwise hold a fully verified history; a post-checkpoint corrupt-suffix test
must verify every earlier row; a foreign-database test must preserve unrelated
tables byte-for-byte or by an exact logical dump; and a stale-writer test must
show one committed successor, not merely one returned error.

## Non-goals

This documentation tranche does not implement or authorize:

- Rust persistence code, SQL migration files, or Cargo edits;
- changes to `FileStore`, `MemStore`, FORMAT, fixtures, portable verifier, CI,
  release metadata, or current public APIs;
- remembered checkpoint files or database rows;
- automatic checkpoint selection, rotation, publication, or synchronization;
- TPM/HSM, remote witness, quorum, gossip, consensus, replication, or
  federation;
- WAL, Merkle trees, consistency proofs, or partial remote verification;
- `LogWriter` exact-terminal checkpoint open unless separately justified;
- automatic retry, idempotency keys, or exactly-once append;
- repair, truncation, salvage, migration, or branch election;
- secure checkpoint retention or general anti-rollback;
- standing, claims, currentness, authority, admission, MCP, librarian, Vesper,
  or Deadbolt integration;
- co-location of L0 with any projection database;
- closure of A-008, RQ-007, RQ-008, NEW-LOG-01, or any release gate; or
- a public-release declaration.

## Reviewer checklist

- [ ] SQLite remains physical persistence, never canonical event identity or
      epistemic authority.
- [ ] The current generic `WriterStore` is not misrepresented as transaction-
      capable.
- [ ] `create_new` and reopen are distinct; ordinary reopen never creates.
- [ ] Weak prefix reopen implies no expected history.
- [ ] Checkpoint open fixes `ContainsCheckpoint` explicitly by its named method
      and passes that exact relation to PR #127 semantics.
- [ ] No checkpoint source or relation is ambient.
- [ ] Exact commitment at exact position is checked; length is insufficient.
- [ ] One stable read transaction covers verification, checkpoint observation,
      and any replay pass.
- [ ] Complete verification precedes every semantic result and publication.
- [ ] Resource budgets are explicit and finite; main/journal/database-page
      bounds precede recovery/integrity checking and record size precedes BLOB
      materialization.
- [ ] Full `u64` position bytes are preserved without SQLite `i64` narrowing.
- [ ] The exact stored record remains authoritative over physical duplicates.
- [ ] Foreign/projection/unknown-schema databases are refused without DROP or
      adoption.
- [ ] `DELETE` plus `EXTRA` is exact-matched and is not called an event-format
      rule.
- [ ] Two stale writers cannot both commit siblings.
- [ ] Busy/error handling performs no hidden retry or predecessor refresh.
- [ ] `COMMIT -> SQLITE_BUSY` is explicitly rolled back/terminated; the writer
      is poisoned and no live transaction remains attached to it.
- [ ] `Err` is never universally translated as “nothing committed.”
- [ ] Unknown commit poisons the writer and requires reopen/reverification.
- [ ] `Ok` is conditional on the stated local storage failure model.
- [ ] Both L0 and checkpoint may roll back together undetected.
- [ ] FileStore/MemStore, claims, FORMAT, audits, readiness, and release state
      remain unchanged in this PR.

## References

- [ADR-0001](../adr/0001-deadbolt-seam.md), sole L0 source/write seam.
- [ADR-0008](../adr/0008-complete-producing-coordinates.md), complete
  producing coordinates and operational-failure boundary.
- [ADR-0009](../adr/0009-verified-supplied-history-and-explicit-checkpoint-expectations.md),
  verified supplied history and explicit checkpoint expectations.
- [LogWriter sole-write capability contract](logwriter-sole-write-capability-contract.md).
- [Verified replay projection boundary contract](verified-replay-projection-boundary-contract.md).
- [Portable verifier input-language contract](portable-verifier-input-language-contract.md).
- [Public pre-alpha convergence baseline](../audits/public-pre-alpha-convergence-baseline-2026-08.md).
- [Current audit disposition](../audits/audit-disposition-2026-08.md).
- [SQLite transactions](https://www.sqlite.org/lang_transaction.html).
- [SQLite autocommit transaction-state check](https://www.sqlite.org/c3ref/get_autocommit.html).
- [SQLite per-schema transaction state](https://www.sqlite.org/c3ref/txn_state.html).
- [SQLite isolation](https://www.sqlite.org/isolation.html).
- [SQLite PRAGMA reference](https://www.sqlite.org/pragma.html).
- [SQLite atomic commit](https://www.sqlite.org/atomiccommit.html).
- [SQLite network-filesystem caveats](https://www.sqlite.org/useovernet.html).

## Target law

After a conforming implementation and hostile evidence land, Magpie will have
one supported local L0 persistence path that conserves exact signed records
under explicit creation, stable completely verified reopening, explicit
checkpoint-relative reopening, transactional stale-writer rejection, bounded
record traversal, and stated acknowledgement/crash semantics.

That implementation will still not establish freshness, global latest history,
canonicality, non-equivocation, authority, securely retained checkpoints, or
universal rollback resistance.
