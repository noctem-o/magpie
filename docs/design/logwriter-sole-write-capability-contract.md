# LogWriter sole-write capability contract

## Status

Narrow implementation-contract candidate for owner review.

Accepted ADR-0001 already ratifies the constitutional doctrine that
`LogWriter` is Magpie's sole L0 write capability. This document does not make a
new architecture decision. It makes that existing decision precise enough for
one later Rust/API remediation and hostile tests.

This document is not runtime evidence. At the baseline below,
`LogStore::append_record` remains public and A-001 / RQ-001 remains
**Confirmed**. A separate implementation PR and independent review are
required before administrative reclassification.

Resolved contract baseline:

```text
repository: noctem-o/magpie
origin/main: 253fc72350ae5232fc428da9ab5657f8ccf5c912
branch: agent/logwriter-sole-write-capability-contract
audit target: A-001 / RQ-001
```

## Purpose

Freeze the smallest supported-public-API law that removes the current raw
backend append bypass while preserving:

- `LogWriter` as the typed, structurally governed L0 writer;
- public read, verification, replay, testing, and embedding paths;
- `FileStore` and `MemStore` as the current writer-supported backends;
- construction of caller-owned raw historical values where already supported;
- the current genesis, chain, canonical-byte, hash, and signature behavior; and
- the separate future admission boundary.

The implementation must make the capability graph smaller. It must not add an
ambient authority, token registry, mode, global switch, or admission concept.

## Current contradiction

Current doctrine and crate documentation say:

```text
possession of LogWriter
= Magpie's raw L0 append capability

LogReader and memory layers
= structurally unable to write
```

Current code instead re-exports this combined abstraction:

```rust
pub trait LogStore {
    fn append_record(&mut self, bytes: &[u8]) -> Result<(), LogError>;
    fn read_records(&self) -> Result<Vec<Vec<u8>>, LogError>;
}
```

`FileStore`, `MemStore`, and `LogStore` are public. An ordinary downstream
caller with a mutable store can therefore call the raw persistence operation
without possessing a `LogWriter`.

The bypass is especially direct for the current stores:

- a retained `MemStore` clone shares the same `Rc<RefCell<...>>` buffer as a
  clone moved into a reader or writer;
- another `FileStore` can be constructed for the same path; and
- the public trait method accepts arbitrary caller-supplied serialized bytes.

`LogWriter::append` is the only current writer method found. It accepts typed
`Provenance` and `Payload`, validates and constructs the event, hashes, signs,
serializes, and then calls the same public `append_record` sink. No other
current `LogWriter` raw-byte method exists.

Verification may later reject malformed or incoherent history. That is not a
capability boundary: rejection after mutation does not remove the caller's
ability to mutate the backend.

## Terminology

### L0 record

One serialized `SignedEvent` storage record in Magpie's append-only source of
truth. Physical JSON-lines bytes are storage representation, not the canonical
event preimage.

### Raw-L0 append operation

Any Magpie-provided callable operation that accepts record bytes, or an
equivalent caller-serialized record value, and appends them to an existing L0
store without going through `LogWriter`'s typed event construction.

### Read abstraction

The public storage capability needed to obtain a record snapshot for
`LogReader`, verification, and replay. The current name is `LogStore`; the
future implementation may retain that name while reducing it to read-only
behavior.

### Persistence seam

The implementation-only operation that persists the record bytes constructed
by `LogWriter`. The seam is necessary, but possession of a public storage value
must not expose it.

### Ordinary downstream caller

Safe downstream Rust code using Magpie's supported public API. This excludes
operating-system mutation, arbitrary unsafe code, debugger intervention, and a
custom storage implementation mutating its own internals through APIs it
defines itself.

### Existing store

A storage instance or backing medium that already represents a Magpie log,
including a shared `MemStore` buffer or a `FileStore` path. Appending to it is
different from constructing a new caller-owned value containing bytes.

## Constitutional capability law

Within Magpie's supported public Rust API:

```text
possession of a writable storage/backend handle alone
!= authority to append L0 record bytes

possession of LogReader
!= authority to append L0 record bytes

possession of FileStore or MemStore
!= authority to append L0 record bytes

possession of SigningKey without LogWriter
!= a Magpie-provided append operation

possession of LogWriter
= the only ordinary public Magpie capability that can cause
  a new L0 record to be appended
```

The positive law is:

```text
public caller without LogWriter
-> cannot invoke a Magpie-provided operation that appends record bytes
   to an existing L0 store

public caller with LogWriter
-> supplies typed Provenance and Payload
-> LogWriter validates the requested event
-> LogWriter constructs EventCore
-> LogWriter hashes and signs
-> LogWriter serializes SignedEvent storage bytes
-> LogWriter invokes its non-public persistence seam
-> LogWriter advances its local chain state only after persistence succeeds
```

No public trait method, inherent method, extension point, returned adapter,
callback, or public associated function may provide an equivalent raw append
path around this law.

## Supported scope and threat model

The guarantee is deliberately limited to Magpie's supported public Rust API
capability graph.

Magpie guarantees:

```text
Magpie does not hand an ordinary downstream caller
a public raw-L0 append operation.

The only supported public Magpie path that requests a new valid L0 append
is LogWriter.
```

This is not a filesystem or process-isolation claim. It does not claim to
prevent:

- an operating-system user from opening and modifying a `FileStore` path;
- another process from modifying the file;
- root or administrator access;
- a debugger from modifying memory;
- arbitrary unsafe Rust from fabricating or mutating values;
- disk failure or out-of-process corruption; or
- a custom storage implementation from mutating its own internals through its
  own code or non-Magpie APIs.

Possession of a `FileStore` path plus ordinary filesystem APIs remains outside
the invariant. Rust visibility cannot make that path tamper-proof. Existing
verification obligations remain: a mutation that violates the current
sequence, predecessor, hash, signature, genesis, or payload rules is rejected
according to those rules. The A-001 implementation must neither weaken nor
overstate that verification.

Filesystem ACLs, file locking, synchronization, durability, journaling,
atomicity, crash recovery, concurrency, framing, snapshot completeness, and
resource limits are not part of this contract. They remain A-008 / RQ-007 /
RQ-008 work.

## Exact future capability graph

The future supported graph is:

```text
public read-capable storage abstraction
    -> LogReader
    -> verify_chain
    -> replay / replay_with_summary
    -> existing reader-oriented projections

FileStore / MemStore
    -> public construction and read participation
    -> LogWriter construction as crate-supported backends

LogWriter
    -> typed Provenance + Payload
    -> writer-owned construction/hash/signature/serialization
    -> non-public persistence seam
    -> FileStore / MemStore persistence

ordinary public caller without LogWriter
    -X-> raw record append
```

The persistence seam may be a crate-private write trait, private adapter,
private inherent method, or an equally small visibility arrangement. It must
not be a public callable method merely hidden behind documentation or naming.

The preferred minimal Rust shape is conceptually:

```text
public LogStore (or equivalently named read abstraction)
    read_records(...)

crate-private WriterStore / persistence adapter
    append_record(...)

LogReader<S: public read abstraction>
LogWriter over crate-supported writer backends only
```

The exact private spelling is not constitutional. The visibility and ownership
properties are. If Rust privacy or public-bound lints make one spelling
awkward, the implementation may choose a private adapter or closed internal
backend representation without reopening the compatibility decision below.

## Storage abstraction is not write authority

Storage remains useful for:

- construction;
- read access;
- complete verification and replay;
- testing changing or hostile snapshots;
- embedding;
- reader-side backend substitution; and
- snapshot inspection already exposed by `MemStore`.

None of those uses requires a publicly callable raw append.

The governing law is:

```text
swappable storage
does not imply
publicly callable raw append
```

Trait symmetry is not an authority requirement. A public read method and a
private write method may operate on the same concrete store without making the
write method a peer public capability.

## Repository compatibility survey

The resolved baseline provides the following evidence:

1. `LogStore` was introduced with the v1 workspace scaffold at commit
   `c62e0860f4c1d0c38f3d8d61d8856ddf34ed82c2`. Its read and write methods,
   root re-export, and generic `LogWriter`/`LogReader` bounds arrived together
   and have not acquired a later compatibility contract.
2. The store comment says storage is swappable and names possible future NAS,
   mmap, or SQLite-WAL implementations. It does not state that third-party
   writer-backend implementation is a stable public promise.
3. README and release material advertise `FileStore`, `MemStore`,
   `LogWriter`, and `LogReader`; they contain no custom writer-backend example
   or downstream implementation promise.
4. Repository production code has exactly the built-in `FileStore` and
   `MemStore` implementations. No downstream workspace production crate
   implements a custom store.
5. Five test-only custom stores exist. Four are intentionally read-only
   changing/one-read fixtures whose required `append_record` implementation
   only panics. One counting wrapper delegates append so writer-recovery tests
   can count reads. These prove the utility of reader substitution and internal
   writer test instrumentation, not a supported external writer ecosystem.
6. Public `magpie-claims` replay entry points are generic over `LogStore`
   because `LogReader` needs a readable backend. They do not write.
7. The release contract states that public Rust APIs are pre-1.0 unstable,
   source compatibility is not promised, the crates remain explicitly
   non-publishable, and the current release is a source candidate rather than
   a stable-API or registry-publication promise.

Answers to the compatibility questions:

```text
Is downstream implementation of a custom writable LogStore an intentional
supported public contract?
No. The current surface permits it, but repository evidence does not establish
it as a promised compatibility boundary.

Is a public custom read store required for LogReader?
Yes. Reader/replay APIs and hostile one-snapshot tests benefit directly from
an externally implementable read abstraction.

Is a public custom writer store required for LogWriter?
No. Current supported writer behavior is exercised through FileStore and
MemStore. Test-only writer instrumentation can live behind the private seam.

Can read and write responsibilities be separated without losing an intentional
public capability?
Yes. Preserve public read substitution and close external writer substitution.
```

## Custom-backend compatibility decision: Option B

Public custom stores are read-only; writer backends are closed.

The future implementation must enforce all of the following:

- the public storage trait is read-capable only;
- downstream implementations of that read trait remain accepted by
  `LogReader`, verification, and replay;
- a downstream custom read store cannot be passed to `LogWriter` as a writer
  backend;
- the writer persistence interface is non-public and implementable only inside
  `magpie-log`;
- `FileStore` and `MemStore` remain supported inputs to `LogWriter`;
- a future writer backend requires a reviewed `magpie-log` code change rather
  than an external implementation of a public raw sink; and
- no public token or permit is added to simulate the removed writer extension
  point.

This is an intentional source-compatibility change:

- a downstream `impl LogStore` that currently defines `append_record` will
  need to become a read-only implementation; and
- `LogWriter<CustomStore>` will no longer be supported.

The change is acceptable at this project stage because the public Rust API is
explicitly unstable and unpublished, no repository evidence demonstrates a
third-party writer backend, and the constitutional sole-writer law is stronger
than accidental generic shape. The useful part of extensibility—custom
reading, verification, replay, and test snapshots—is preserved.

This compatibility decision is complete. The runtime PR does not need another
architecture decision and must not substitute Option A or introduce an
unforgeable public append token without a new owner-authorized contract.

## Construction versus mutation

`MemStore::new`, `MemStore::from_records`, `MemStore::records`, and
`FileStore::new` are not themselves raw append operations.

The future implementation should preserve these current distinctions unless a
separate finding requires a later change:

```text
construct a new caller-owned MemStore from caller-owned record bytes
!= append bytes to an existing Magpie store through a Magpie raw append API

inspect a cloned record snapshot
!= mutate the shared store

construct FileStore for a path
!= Magpie preventing direct filesystem access to that path
```

`MemStore::from_records` is used for fixtures, hostile/tamper input, raw import
construction, and verification tests. It may remain public. Its contents are
not verified merely because the value exists. `LogReader` verification and
replay retain their existing trust rules; A-009 / RQ-002 governs verified
versus unverified replay typing, and A-004 / RQ-006 governs accepted record
grammar.

A `MemStore` clone retained beside a `LogWriter` may observe the shared buffer
and may be passed to a reader. It must not expose a Magpie append operation.
Only the `LogWriter` clone owner may request persistence through the private
seam.

## LogWriter-owned event and persistence law

`LogWriter::append` remains typed. The caller supplies `Provenance` and
`Payload`, not serialized `SignedEvent` bytes.

For each append request, the implementation must preserve this ordering and
state law:

1. check the genesis-position rule and structurally validate the payload;
2. use the writer's exact current `next_seq` and `last_hash`;
3. obtain the recorded timestamp through the writer's configured clock;
4. construct the exact `EventCore`;
5. compute the content hash from the existing canonical core bytes;
6. sign the existing domain-separated signature message;
7. construct `SignedEvent`;
8. serialize the existing JSON storage representation;
9. invoke the non-public persistence seam exactly once for that attempt; and
10. only after successful persistence, update `last_hash` and `next_seq` and
    return the event.

If serialization or backend append fails:

```text
writer.last_hash remains the prior tip
writer.next_seq remains the prior sequence
no successful append result is returned
```

This contract does not require rewinding an injected clock or guaranteeing
identical timestamps across caller retries. It requires that writer chain
state not advance as though failed persistence succeeded.

No public `LogWriter` method may accept caller-serialized record bytes or a
caller-constructed `SignedEvent` for persistence.

## Genesis and open law

Current genesis behavior is preserved:

```text
empty supported writer backend
+ LogWriter::open(...)
-> verification reports the empty identity
-> LogWriter creates typed Genesis
-> Genesis uses the same writer-owned persistence seam
-> exactly one sequence-zero genesis record is persisted before open succeeds
```

There is no separate public genesis-byte writer.

For a non-empty backend:

```text
LogWriter::open(...)
-> reads and verifies the existing chain under the supplied key
-> recovers exact count and tip
-> appends no second genesis
-> returns a writer positioned at the exact next sequence
```

No genesis format, canonicalization profile, verifying-key binding, trust-root
rule, or recovery behavior changes.

## Reader law

The public read path must remain sufficient for:

- `LogReader::open`;
- `verify_chain`;
- `replay`;
- `replay_with_summary`;
- `replay_standing_context`;
- `replay_origin_admission_context_v0`; and
- existing reader-oriented projections.

Possession of the public read abstraction or `LogReader` must not expose the
private persistence seam, a raw append method, a mutable backend escape hatch,
or a way to acquire a `LogWriter` without explicitly constructing one through
its supported API.

This contract does not solve `LogReader::events()` returning parsed but
unverified `SignedEvent`. That remains A-009 / RQ-002. Separating storage read
from write must not be presented as verified-event typing closure.

## Canonical and behavior compatibility

The runtime remediation is an API/capability correction only. It requires:

```text
no L0 payload change
no payload tag change
no canonical field-order or encoding change
no hash change
no signature domain or signature-message change
no SignedEvent JSON storage change
no golden-chain byte change
no verifying-key semantics change
no genesis format change
no standing or projection policy change
no FORMAT evolution
```

Existing golden vectors and fixed-clock writer outputs must remain
byte-for-byte identical.

## Forbidden bypass shapes

None of the following satisfies this contract:

- keeping `append_record` public and documenting it as internal;
- relying on later verification to catch unauthorized mutation;
- claiming an append-only backend makes every append authorized;
- saying only Deadbolt should call a method that ordinary downstream Rust can
  also call;
- renaming the public method to an internal-looking or dangerous-looking name;
- moving the same public sink to an extension trait or public adapter;
- exposing a public inherent raw append on `FileStore` or `MemStore`;
- exposing a public callback that receives the private persistence operation;
- returning the persistence seam or a publicly callable mutable raw backend
  handle from `LogWriter`;
- adding a runtime Boolean, environment variable, global singleton, registry,
  feature mode, or process convention as the authority boundary;
- retaining a public append API with a convention that callers ask a gate
  first; or
- adding an `unsafe` public append API and calling visibility solved.

Verification, naming, documentation, and convention are not capability
sealing.

## L0 structure is not epistemic admission

A-001 owns only the raw L0 append capability graph.

`LogWriter` remains responsible for:

- genesis position;
- payload structural validation;
- sequence and predecessor binding;
- recorded timestamp;
- content hashing;
- signature construction;
- canonical-event and storage serialization; and
- persistence ordering relative to its local chain state.

`LogWriter` does not decide:

- claim truth;
- evidence truth or eligibility;
- standing;
- currentness;
- origin authority;
- human authorization;
- agent permission;
- whether a proposal deserves admission;
- rejection-history representation; or
- generic retry or deduplication semantics.

Therefore:

```text
A-001 remediation
!= EpistemicGate implementation

sole raw L0 writer
!= generic epistemic admission policy
```

No `EpistemicGate`, admission record, rejected-attempt record, retry authority,
ordinary claim/evidence writer, Deadbolt emitter, or new capability is part of
this contract. A-022 remains separate.

## Relationship to adjacent findings

This contract does not close, narrow, or implement:

- A-004 / RQ-006 — cross-language record grammar;
- A-008 / RQ-007 / RQ-008 — durability, locking, atomicity, complete-snapshot,
  framing, and resource law;
- A-009 / RQ-002 — verified versus unverified event typing;
- A-014 — anchor identity-field validation;
- A-021 — external Ed25519 root strictness;
- A-022 — generic admission / `EpistemicGate`; or
- A-024 — authority-bound origin corroboration.

It introduces no Deadbolt or EpistemicGate implementation and grants neither
component a new capability.

## Deterministic hostile scenarios

### Scenario A — MemStore plus valid serialized bytes, no LogWriter

Can the caller append those bytes through Magpie's supported public API?

**No.** `MemStore` exposes no public raw append operation. Construction through
`from_records` creates a caller-owned initial value; it is not mutation of an
existing store. A retained shared clone does not change the answer.

### Scenario B — FileStore, no LogWriter

Can the caller invoke a Magpie public raw append operation?

**No.** `FileStore` construction and read participation do not expose the
private writer persistence seam.

### Scenario C — OS write permission to the FileStore path

Can Rust visibility prevent the caller from opening the file directly?

**No.** This is outside the invariant. Violations of the current chain rules
remain detectable by existing verification; this contract does not provide
filesystem access control or attribution.

### Scenario D — LogReader possession

Can the caller obtain append authority from the reader?

**No.** The reader owns only a read abstraction and verifying key. It exposes
no persistence seam, backend mutation escape, or writer conversion.

### Scenario E — LogWriter possession

Can the caller append arbitrary caller-serialized record bytes?

**No.** The public operation remains typed `Provenance` plus `Payload`.
`LogWriter` owns core construction, hashing, signing, serialization, and
persistence. The repository survey found no other current writer raw-byte
method to preserve.

### Scenario F — SigningKey possession, no LogWriter

Does the key alone invoke a Magpie public append API?

**No.** The caller may sign values in its own program, but Magpie exposes no
separate raw persistence operation that turns them into an L0 append. Direct
filesystem mutation remains outside this invariant.

### Scenario G — custom store implementation

Can it preserve intended substitution without a second Magpie-issued write
authority?

**Yes, for reading only.** A downstream custom store may implement the public
read abstraction and participate in `LogReader`, verification, and replay. It
cannot implement the private writer seam and cannot be used as a supported
`LogWriter` backend. A custom type may mutate itself through its own APIs, but
that is not a Magpie-issued raw append capability.

### Scenario H — backend append fails

Does `LogWriter` advance its chain tip or sequence?

**No.** The persistence result is checked before `last_hash` or `next_seq`
changes. A failed open-time genesis append likewise returns no successfully
opened writer.

## Future implementation proof obligations

The implementation PR must include direct compile/API tests that reach the
intended boundary, with all relevant public traits imported and otherwise
valid syntax.

Required negative proofs:

1. ordinary downstream caller + `FileStore` - `LogWriter` cannot invoke any
   Magpie raw append operation;
2. ordinary downstream caller + `MemStore` - `LogWriter` cannot invoke any
   Magpie raw append operation;
3. ordinary downstream caller generic over the public read abstraction cannot
   append;
4. ordinary downstream caller + `LogReader` cannot append or extract the
   persistence seam;
5. ordinary downstream custom read store cannot satisfy `LogWriter`'s closed
   writer-backend requirement;
6. `SigningKey` without `LogWriter` does not gain a Magpie persistence call;
   and
7. no public trait or inherent method provides an equivalent raw record sink.

The compile-fail cases must not fail because of a missing trait import, stale
constructor arity, undefined variable, malformed syntax, unrelated ownership
error, or obsolete method signature. Each case must compile through setup and
fail at the exact write-capability boundary.

Required positive proofs:

1. a downstream custom read store implements the public read abstraction and
   works with `LogReader` verification/replay;
2. `LogWriter` + `FileStore` creates and continues a valid chain;
3. `LogWriter` + `MemStore` creates and continues a valid chain;
4. `LogWriter::open` on an empty store creates exactly one genesis through the
   same private persistence seam;
5. `LogWriter::open` on an existing store verifies once, recovers exact count
   and tip, and appends no second genesis;
6. `LogWriter::append` retains exact current canonical bytes, hashes,
   signatures, JSON storage bytes, and fixed-clock golden outputs;
7. an internally instrumented failing backend proves failed persistence leaves
   `len()` and `tip()` unchanged and the next successful append uses the exact
   prior sequence and predecessor; and
8. existing `LogReader`, `verify_chain`, `replay`, `replay_with_summary`, and
   downstream replay-context tests remain green.

The custom-read positive test and custom-writer negative test freeze Option B
without requiring another architecture decision.

## Minimality audit

The selected design needs exactly:

1. one public read-capable storage abstraction; and
2. one non-public writer persistence seam owned by `magpie-log`.

No public token, permit, wrapper hierarchy, backend registry, capability enum,
mode, callback, or global state is required. `FileStore` and `MemStore` already
provide the concrete storage concepts. `LogWriter` already provides the public
write capability.

If a proposed implementation adds another type or concept, reviewers must ask:

```text
Can this type be removed while preserving:
- public read substitution,
- closed writer backends,
- FileStore/MemStore writer support, and
- no public raw append?
```

If yes, remove it.

## Implementation readiness conditions

The implementation slice is ready only when its patch demonstrates all of the
following without a new architecture choice:

- Option B is implemented exactly;
- public read substitution remains usable;
- only crate-supported backends can be persisted by `LogWriter`;
- no public raw append operation remains;
- typed append, sequencing, genesis, and failure-state laws are preserved;
- golden and FORMAT compatibility remain exact;
- compile-fail tests reach the intended boundary;
- runtime tests prove both supported backends and failed persistence;
- no adjacent audit finding is claimed closed;
- no admission, Deadbolt, filesystem-policy, or resource-law scope is added;
  and
- the audit disposition ledger remains unchanged until implementation,
  independent review, and merge evidence exist.

## Reviewer checklist

- [ ] The public storage abstraction contains no raw append member.
- [ ] `FileStore` and `MemStore` expose no inherent or extension raw append.
- [ ] The persistence seam is inaccessible to ordinary downstream Rust.
- [ ] A retained `MemStore` clone cannot append without `LogWriter`.
- [ ] A second `FileStore` handle cannot call a Magpie raw append operation.
- [ ] Public custom read stores still work with `LogReader`.
- [ ] Public custom writer backends are intentionally unsupported and tested.
- [ ] `LogWriter::append` remains typed and owns record construction.
- [ ] Failed persistence does not advance writer chain state.
- [ ] Empty open creates genesis through the same sole-writer path.
- [ ] Existing canonical bytes, hashes, signatures, records, and fixtures are
      unchanged.
- [ ] Direct OS/file mutation is not claimed to be prevented.
- [ ] A-008, A-009, A-022, and the other adjacent findings remain separate.
- [ ] A-001 / RQ-001 remains Confirmed until runtime evidence is merged and
      independently reviewed.

## Administrative disposition

Acceptance of this documentation contract identifies the implementation
target. It does not remediate the current public method.

```text
A-001 / RQ-001 after this contract PR
= Confirmed

possible next evidence
= one separately reviewed runtime/API remediation satisfying this contract

reclassification
= later audit-disposition work after merge and independent evidence review
```
