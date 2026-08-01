# Epistemic Invariant Ledger

## Status

Orientation and reference artifact. Docs-only. This ledger introduces no
doctrine, no mechanisms, and no schemas, APIs, or events. Governing ADRs
and contracts override this document: if the ledger conflicts with any
governing document, the ledger is wrong and should be corrected to match.

## Purpose

The ledger answers one question: when reviewing a proposed change, which
existing invariant should I check?

It provides invariant lookup, review guidance, and architectural
orientation. It does not provide governance, implementation rules, policy
selection, or design decisions. It does not answer how anything should be
implemented, what mechanism should enforce an invariant, or what policy
should exist.

Preserved without qualification:

> Magpie runs alone; integrations add evidence, not authority.

> Information may move downstream. Authority does not.

## 1. Invariant ledger

Every entry below is established by existing doctrine; the ledger creates
none of them.

| Invariant | Source | Protects against |
| --- | --- | --- |
| The signed append-only log is the sole historical authority | ADR-0001 | shadow history; side stores becoming canonical |
| `LogWriter` is the sole append authority | ADR-0001 | hidden write paths |
| Vocabulary presence grants no authority | v0.1.0 contract; ADR-0002 | labels and self-assertion becoming standing |
| Policy selection is explicit; there is no ambient latest | README; resolution boundary contract | implementation defaults becoming policy |
| Admission ≠ endorsement | admission record boundary contract | historical inclusion becoming approval |
| History ≠ evidence | evidence admission boundary contract | automatic evidence creation |
| Evidence ≠ standing | epistemic resolution boundary contract | skipping governed evaluation stages |
| Standing ≠ truth | epistemic resolution boundary contract | derived status becoming truth authority |
| Provenance ≠ trust | provenance response contract | origin becoming reliability |
| Identity ≠ authority | capability and AgentProposer boundary contracts | external identity becoming permission |
| Availability ≠ permission | capability boundary contract | interface exposure becoming authority |
| Proposal ≠ evidence | AgentProposer boundary contract | generated content becoming evidence automatically |
| Projection ≠ history | pipeline separation contract; README | derived views replacing source records |
| Failure ≠ falsity | boundary contracts; README | operational failures becoming epistemic conclusions |

## 2. Collapse detection

Implementation proposals should be checked for accidental collapses —
paths that silently connect things the ledger holds apart:

```text
external material → authority
history → endorsement
provenance → trust
proposal → evidence
evidence → standing
standing → truth
```

This section is a recognition aid, not a new prohibition list. Every
collapse named here is already prohibited by the source documents indexed
above.

## 3. Usage guidance

Before proposing a new boundary, mechanism, or write surface:

- identify which invariant it touches;
- read the owning ADR or contract;
- do not rely on this ledger as the governing source.

## Non-goals

This document adds none of the following:

- an `EpistemicGate` design;
- admission implementation;
- evidence registration;
- resolver implementation;
- APIs, schemas, or event formats;
- identity or trust systems;
- scoring or ranking;
- policy mechanisms.
