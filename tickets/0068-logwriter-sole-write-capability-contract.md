# Ticket 0068: LogWriter sole-write capability contract

## Status

Documentation-only implementation-contract slice for A-001 / RQ-001.

This ticket changes no Rust, tests, fixtures, FORMAT, audit ledger, ADR,
dependency, CI, release metadata, schema, policy, writer runtime, Deadbolt
runtime, or `EpistemicGate` runtime. Human owner review is required. A separate
runtime PR must satisfy the linked design before A-001 / RQ-001 can be
reclassified.

Resolved base:

```text
253fc72350ae5232fc428da9ab5657f8ccf5c912
Merge pull request #113 from noctem-o/docs/reconcile-audit-disposition-post-112
```

Branch:

```text
agent/logwriter-sole-write-capability-contract
```

Proposed commit title:

```text
docs: define sole-writer capability contract
```

Proposed draft PR title:

```text
docs: define LogWriter sole-write capability boundary
```

Normative design:

[`docs/design/logwriter-sole-write-capability-contract.md`](../docs/design/logwriter-sole-write-capability-contract.md)

## Audit target

```text
A-001 / RQ-001
Public backend append remains a second write capability beside LogWriter.
```

The living disposition ledger records the finding as **Confirmed** and P0.
This ticket must not edit that ledger or claim closure. A contract is not
runtime evidence.

## Documentation authority

Accepted ADR-0001 already states that Deadbolt will hold `LogWriter` as the
sole L0 write capability. This ticket does not create ADR-0008 and does not
choose new doctrine. It specifies the smallest API/capability correction that
implements the existing accepted doctrine.

The ticket is authorized to:

- record the exact current contradiction;
- decide the custom-backend compatibility rule;
- freeze positive and negative public capability laws;
- freeze the future sequencing, genesis, reader, and compatibility laws; and
- define hostile compile-time and runtime proof obligations.

It does not authorize the Rust remediation.

## Current defect

Current public API:

```text
public LogStore
    append_record(&mut self, arbitrary bytes)
    read_records(&self)

public FileStore implements LogStore
public MemStore implements LogStore
public LogWriter delegates to LogStore::append_record
```

Result:

```text
mutable FileStore or MemStore
- LogWriter
-> Magpie-provided raw record append remains callable
```

That contradicts the current crate, README, CLAUDE, release, and ADR doctrine
that `LogWriter` is the sole write capability and readers/projections are
structurally unable to write. Verification after mutation does not remove the
mutation capability.

The current `LogWriter` API itself is already typed: it accepts `Provenance`
and `Payload`, constructs `EventCore`, hashes, signs, serializes `SignedEvent`,
calls the backend, then advances chain state. The target is the peer public
backend sink, not a redesign of typed append.

## Repository compatibility survey

The complete repository and history survey found:

- `LogStore`, both methods, both stores, and generic writer/reader bounds were
  introduced together in scaffold migration commit `c62e0860f4c1d0c38f3d8d61d8856ddf34ed82c2`;
- no later commit establishes third-party writable-store compatibility;
- the store comment says storage is swappable, but no README, release contract,
  example, or API guide promises downstream writer-backend implementation;
- production `magpie-log` has exactly two store implementations:
  `FileStore` and `MemStore`;
- no downstream workspace production crate implements `LogStore`;
- five custom implementations are test-only: four read-only changing/one-read
  stores whose append method panics, and one counting wrapper that delegates
  append for writer-recovery instrumentation;
- public `magpie-claims` replay entry points require only readable custom
  storage through `LogReader`;
- `FileStore` appears in one round-trip/recovery integration test and product
  documentation, not as a custom-backend protocol;
- `MemStore` is the repository's fixture/embedding backend and
  `from_records`/`records` are used for hostile, golden, replay, and projection
  tests;
- `LogWriter` use outside its owner module is in tests, examples, and doctrine;
  no second current writer raw-byte method was found; and
- the release contract explicitly says the public Rust API is unstable before
  1.0, source compatibility is not promised, crates remain non-publishable,
  and the current release is only a source candidate;

Compatibility answers:

```text
intentional downstream custom writable LogStore contract: no
public custom read-store utility: yes
required external custom writer backend: no
separable read/write responsibilities: yes
```

## Contract decision

Choose Option B from the design question:

```text
public custom stores are read-only
writer backends are closed to magpie-log
```

Exact future graph:

```text
public read abstraction
-> LogReader / verify / replay / downstream replay contexts

crate-private persistence seam
-> FileStore / MemStore
-> callable by LogWriter only

external custom read store
-> allowed for LogReader
-X-> unsupported for LogWriter
```

This intentionally drops source compatibility for external custom writable
stores and preserves reader-side substitution. It requires no public token,
permit, registry, Boolean, callback, global, or mode. Future writer backends
must be added inside `magpie-log` through reviewed code.

The implementation may retain `LogStore` as the public read-trait name and add
a crate-private persistence trait/adapter. An equivalently small private
spelling is acceptable if Rust visibility rules require it. A public callable
raw append is not.

## Positive law

```text
LogWriter + typed Provenance + typed Payload
-> validates the event request
-> binds exact current sequence and previous hash
-> records timestamp
-> constructs EventCore
-> hashes
-> signs
-> serializes SignedEvent
-> invokes the non-public persistence seam
-> advances last_hash and next_seq only after persistence succeeds
```

Supported writer behavior remains available for both `FileStore` and
`MemStore`.

## Negative law

```text
FileStore without LogWriter
-> no Magpie raw append operation

MemStore without LogWriter
-> no Magpie raw append operation

public read abstraction without LogWriter
-> no Magpie raw append operation

LogReader
-> no Magpie raw append operation or writer-seam extraction

SigningKey without LogWriter
-> no Magpie persistence operation

external custom store
-> read substitution only; no Magpie writer-backend implementation
```

The law covers Magpie's supported public Rust API. It does not claim to stop
direct filesystem access, another process, admin/root, debugger mutation,
unsafe code, disk failure, or a custom type's own mutation APIs.

## Construction and reader preservation

The implementation contract preserves the distinction between constructing a
caller-owned value and mutating an existing Magpie log through Magpie:

- `MemStore::from_records` may remain public for fixtures, raw construction,
  import-shaped input, and hostile verification tests;
- `MemStore::records` may remain public for inspection;
- `MemStore` clones may continue sharing one buffer for writer/reader tests;
- `FileStore::new` may remain public;
- a public read abstraction remains sufficient for `LogReader`,
  `verify_chain`, `replay`, `replay_with_summary`, and downstream replay
  contexts; and
- none of these values is verified merely by construction.

A-009 / RQ-002 remains the owner of verified/unverified event typing. A-004 /
RQ-006 remains the owner of record grammar.

## Writer state and genesis law

The runtime follow-up must preserve:

```text
backend append succeeds
-> writer advances exact tip and sequence

backend append fails
-> writer tip and sequence do not advance
```

Opening an empty supported backend must create exactly one typed genesis through
the same `LogWriter`-owned persistence seam. There is no public genesis-byte
append. Opening an existing backend must verify it, recover exact count/tip,
and avoid a second genesis.

No retry timestamp identity is added; only chain-state non-advancement is
required after failure.

## Future implementation proof obligations

Negative compile/API proofs:

1. `FileStore` without `LogWriter` cannot call a Magpie raw append;
2. `MemStore` without `LogWriter` cannot call a Magpie raw append;
3. a generic public read store cannot append;
4. `LogReader` cannot append or expose the private seam;
5. an external custom read store cannot be used as a writer backend;
6. `SigningKey` alone cannot call Magpie persistence; and
7. no public trait, inherent method, extension adapter, or equivalent raw sink
   remains.

Each negative case must import the relevant public API, use valid constructor
arity and syntax, and fail at the intended capability boundary rather than an
unrelated compiler error.

Positive runtime/API proofs:

1. an external custom read store still works with `LogReader`;
2. `LogWriter<FileStore>` creates and continues a valid chain;
3. `LogWriter<MemStore>` creates and continues a valid chain;
4. empty open creates exactly one genesis through the sole-writer path;
5. existing open verifies once and recovers exact count/tip;
6. fixed-clock append produces the exact current canonical bytes, hashes,
   signatures, JSON records, and golden chains;
7. an internal failing backend proves writer state does not advance on failed
   persistence and the next success uses the prior sequence/tip; and
8. existing reader, replay, and replay-context tests remain green.

## Forbidden mechanisms

Do not satisfy this ticket with:

- public `append_record` plus an “internal use” comment;
- detection by later verification;
- “append-only means authorized”;
- “only Deadbolt should call it” while other Rust callers can;
- a scary/private-looking public name;
- a public extension trait, adapter, inherent method, or callback with the same
  sink;
- a runtime flag, environment variable, singleton, registry, or mode;
- a public append method plus a convention to ask a gate first;
- a new public append token or permit; or
- a new admission or writer facade.

## Exact non-goals

This ticket and its future implementation do not:

- implement `EpistemicGate`, admission records, rejection history, retry
  authority, ordinary claim/evidence writing, or generic policy;
- implement Deadbolt or change the Deadbolt seam;
- decide claim truth, evidence truth, standing, currentness, origin authority,
  human authorization, or agent permission;
- change A-004 / RQ-006 grammar;
- change A-008 / RQ-007 / RQ-008 durability, locking, atomicity, snapshot, or
  resource law;
- change A-009 / RQ-002 verified/unverified event typing;
- change A-014 anchor validation, A-021 root strictness, A-022 admission, or
  A-024 authority-bound corroboration;
- add filesystem ACL, locking, sync, journaling, concurrency, or crash policy;
- change L0 payloads, tags, FORMAT, canonical encoding, hashes, signatures,
  golden fixtures, verifying-key semantics, or genesis format; or
- edit historical audits or the audit disposition ledger.

## Changed-path allowlist

This documentation PR changes exactly:

```text
docs/design/logwriter-sole-write-capability-contract.md
tickets/0068-logwriter-sole-write-capability-contract.md
```

No third documentation path is required. In particular, do not change:

```text
crates/**
tools/**
docs/FORMAT.md
docs/adr/**
docs/audits/**
README.md
Cargo.toml
Cargo.lock
.github/**
```

## Validation contract

For this documentation-only slice:

```text
git diff --check
git status --short --untracked-files=all
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
```

The final changed-path union must be the exact two-file allowlist. Targeted
case-insensitive scans must review every occurrence of:

```text
sole writer
only writer
append_record
LogStore
LogWriter
LogReader
FileStore
MemStore
raw append
write capability
storage capability
Deadbolt
EpistemicGate
A-001
RQ-001
```

The scans must confirm that this contract:

- does not promise filesystem tamper prevention;
- does not claim runtime remediation already exists;
- keeps A-001 / RQ-001 Confirmed;
- adds no generic admission semantics;
- does not close A-008 or A-009;
- requires no new L0 bytes; and
- records rather than hides the custom-writer compatibility break.

Rust tests are not required for this documentation-only patch. The future
runtime ticket owns the Rust validation ladder.

## Hostile review checklist

- [ ] A. No ordinary downstream caller has a contractually permitted Magpie
      raw-L0 append path without `LogWriter`.
- [ ] B. The contract does not claim direct OS/file mutation is preventable.
- [ ] C. `LogWriter::append` remains typed, not a serialized-record sink.
- [ ] D. Structural L0 validation remains separate from future epistemic
      admission.
- [ ] E. A-001 / RQ-001 remains administratively Confirmed.
- [ ] F. A-008 and A-009 remain open and separate.
- [ ] G. Option B is the one explicit custom-backend decision.
- [ ] H. Exact negative and positive tests are derivable without another
      architecture decision.
- [ ] I. Both historical audit files remain byte-for-byte unchanged.
- [ ] J. The PR is documentation-only and exactly two files.

## Review checklist

- [ ] Exact base and branch are recorded.
- [ ] Current defect is described as capability topology, not accepted-history
      forgery.
- [ ] Public read substitution is preserved.
- [ ] External custom writer substitution is explicitly removed.
- [ ] `FileStore` and `MemStore` remain writer-supported through `LogWriter`.
- [ ] `MemStore::from_records` is classified as construction, not mutation.
- [ ] File path possession is outside the Rust API guarantee.
- [ ] Failed persistence cannot advance writer state.
- [ ] Genesis uses the same sole-writer path.
- [ ] Golden and FORMAT compatibility is exact.
- [ ] No adjacent architecture work is absorbed.

## Implementation handoff

The next slice is one separately authorized Rust/API remediation that:

1. reduces the public storage abstraction to reading;
2. adds the smallest non-public writer persistence seam;
3. closes writer backends to the crate-owned `FileStore` and `MemStore`;
4. updates internal bounds and test scaffolding without changing record bytes;
5. adds exact compile-fail/API tests for the removed bypass;
6. adds the positive custom-read and supported-writer tests;
7. adds failed-persistence writer-state coverage; and
8. runs the complete repository validation ladder.

It must not edit the audit disposition ledger in the implementation PR.
Reclassification is a later administrative slice after the runtime change is
merged and independently reviewed.

## Publication authority

The owner instruction for Ticket 0068 authorizes one commit on the task
branch, one push, and one **draft** PR against `main`. Human owner retains sole
merge authority. Do not merge, enable auto-merge, or mark the PR ready for
review.

## Stop conditions

Stop and report rather than broadening scope if:

- live main already contains Ticket 0068 or either required path;
- A-001 / RQ-001 has already been materially changed;
- repository evidence establishes a supported external custom writer backend
  that this compatibility decision would strand;
- a third path is objectively required;
- the two-file allowlist cannot be preserved; or
- documentation validation fails in a way that requires runtime, FORMAT, ADR,
  audit, dependency, CI, or release changes.
