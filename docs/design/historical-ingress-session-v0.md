# Governed Historical Ingress Session v0

## Status

**EXPERIMENTAL / NOT RATIFIED.** This implementation candidate does not close
A-022, change audit disposition, authorize a merge or release, or establish
doctrine. It is a composition guarantee for one delegated capability.

`EpistemicGate` was the prior exploratory name. The candidate was renamed
because the surviving abstraction is a custody and historical-ingress
boundary, not a generic epistemic decision engine.

Predecessor: `spike/epistemic-gate-skeleton-v0` (historical exploratory
evidence; the remote branch is unchanged).

## Meaning and capability graph

Admission means controlled historical inclusion: a typed submission may become
part of this Magpie history. It does not imply truth, support, settlement,
trust, authentication, relevance, or standing.

```text
trusted application bootstrap
        |
        | transfers an already-open writer by value
        v
HistoricalIngressSession
        |
        `--> AgentProposalIngress
                 |
                 `--> mechanical LogWriter<SqliteL0Store>
                              |
                              `--> SQLite L0
```

The session owns the writer by value. The writer field is private. The public
category surface contains exactly one operation: `propose_claim`, accepting an
`AgentClaimProposal` and caller-supplied asserted `Provenance`.

The proposal maps to the existing `Payload::ClaimAssertedV2` event. The ingress
supplies `actor_class = "AgentProposer"`; callers supply neither actor class
nor `Status`, and no arbitrary `Payload` method exists. `ClaimStatusChanged` is
therefore impossible through this capability.

## Threat model and enforcement

This uses AUTH-0 trusted-process semantics. Provenance is asserted attribution,
not authenticated identity. Agent names, proposer IDs, and source strings are
not identity credentials. No identity PKI or signed external envelope is
implemented.

Safe downstream code holding only `AgentProposalIngress` cannot recover the
writer or perform another supported category operation. Rust privacy and the
closed method surface enforce this at compile time. This is delegated
capability enforcement, not globally exclusive write enforcement: standalone
public `LogWriter` construction remains available, so this candidate does not
claim that only a session can create Magpie history.

The session is SQLite-only. There is no `FileStore`, `MemStore`, or generic
`S` session constructor. Construction, borrowing, and dropping the session do
not append.

## Failure semantics

The existing SQLite writer remains responsible for opening, validation,
transactions, stale-writer checks, and persistence. The session adds no retry,
deduplication, side table, or hidden state. `StorageBusy`, `WriterStale`, and
`CommitStateUnknown` remain operational storage outcomes. Unknown commit state
remains unresolved at this boundary and is never reinterpreted as rejection,
falsity, or safe-to-retry. These append failures poison the current writer
according to the existing SQLite law; recovery requires dropping the session
and reopening the history.

There is no rejection event and no second rejection ledger. Failures are
returned to the caller; failure is not falsity. Durable idempotency is not
provided, and lost-ack/retry behavior remains a documented limitation.

## Format and scope limits

This candidate changes no `EventCore` fields, payload variants or tags,
canonical ordering, hash/signature inputs, genesis, `magpie-core-v1`, golden
vectors, conformers, or corpus identity. No command identity is smuggled into
provenance, metadata, or SQLite details. A future need for command identity is
**OWNER DECISION REQUIRED**.

Only agent claim proposals are implemented. Evidence registration and
justification edges are intentionally absent because this experiment does not
make the additional category-policy decisions required to expose them.

This does **not** close A-022. The standalone `LogWriter` limitation and the
system-wide question of session-mediated coverage remain owner decisions.
