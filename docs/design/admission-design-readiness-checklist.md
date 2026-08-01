# Admission Design Readiness Checklist

## Status

Design exploration. Docs-only. Nothing in this document is decided
doctrine: it is a prerequisite checklist, recording the questions that must
be answered before any governed admission mechanism is designed or
implemented. It introduces no doctrine and defines no mechanism. ADR-0002,
the ratified MCP contracts, and the admission exploration documents
(`admission-boundary-questions`, `admission-boundary-threat-model`) remain
authoritative; where anything here appears to conflict with them, they
govern.

## Purpose

The questions exploration named what is unresolved; the threat model named
what could go wrong. This checklist names what "ready" means: the explicit
answers a future admission design must have in hand, through reviewed
doctrine artifacts, before implementation begins.

Checklist questions over answers; constraints over designs; readiness
criteria over mechanisms.

The base invariant constrains every answer:

> Magpie runs alone; integrations add evidence, not authority.

## 1. Authority readiness

A ready design can clearly identify:

- the single historical authority (the signed append-only log);
- the single append authority (`LogWriter`);
- the exact transition where external material becomes historical;
- every path that must remain structurally unable to write history.

The settled shape it must preserve:

```text
external material
        |
        v
future governed admission boundary
        |
        v
LogWriter
        |
        v
historical record
```

## 2. Admission semantics readiness

A ready design can clearly separate:

```text
admission ≠ truth creation
          ≠ standing assignment
          ≠ policy decision
```

and can explain:

- what admission decides;
- what admission explicitly does not decide;
- what belongs to structural validation;
- what belongs to standing resolution.

These are recorded as readiness criteria; this document does not answer
them.

## 3. Input category readiness

For each candidate input category — AgentProposer material, external
evidence references, Deadbolt execution anchors, human observations,
imported records — a ready design can answer:

- why does this category require admission?
- what provenance does it require?
- does it need category-specific rules, and if so, where do those rules
  live?
- can category handling avoid creating hidden authority?

This document defines no categories and no schemas.

## 4. Provenance readiness

A ready design can preserve:

- recorded attribution;
- provenance coordinates (the verified record snapshot or log prefix);
- source history linkage;
- the distinction between attribution and identity.

It must prevent the slide:

```text
attribution ≠ verified identity ≠ trust
```

## 5. Rejection and failure readiness

A ready design can answer:

- are rejected admission attempts recorded?
- if recorded, how are they prevented from becoming negative evidence?
- if not recorded, how are retry and amplification concerns handled?

Preserved without qualification: failure is not falsity.

## 6. Policy boundary readiness

A ready design can show that admission cannot silently become:

- trust scoring;
- evidence weighting;
- standing calculation;
- source ranking;
- policy selection.

Policy selection is explicit. There is no ambient latest.

## 7. AgentProposer readiness

A ready design can preserve:

```text
AgentProposer
    |
    v
proposal material
    |
    ?
    |
    v
historical record
```

without introducing:

- agent authority;
- model confidence as epistemic status;
- repetition as evidence;
- autonomous admission.

## 8. Projection and retrieval readiness

A ready design can prevent:

- caches becoming history;
- embeddings becoming authority;
- retrieval ranking becoming standing;
- summaries replacing source records.

Derived views remain subordinate to the signed append-only log.

## 9. Final readiness gate

A future admission mechanism is not ready for implementation unless every
checklist item above has an explicit answer through an ADR, a ratified
contract, or an equivalent reviewed doctrine artifact.

An unanswered item is not a detail to discover during implementation; it is
a blocker. This is the same rule ADR-0002 sets for its own rollout: the
boundary and its tests come before the write surface they govern.

## Non-goals

This document adds none of the following:

- an `EpistemicGate` design or admission mechanism;
- implementation details or code structure;
- MCP write paths;
- authentication, permissions, identity systems, or capability tokens;
- trust scores or confidence scores;
- ranking systems or retrieval architecture;
- agent autonomy mechanisms.
