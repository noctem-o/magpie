# Admission Boundary Threat Model

## Status

Design exploration. Docs-only. Nothing in this document is decided
doctrine: it identifies failure modes before any admission mechanism exists.
It defines no mechanism, no mitigation, and no security architecture, and it
is not part of the ratified contract stack. ADR-0002 and the ratified MCP
contracts remain authoritative; where anything here appears to conflict with
them, they govern.

## Purpose

The admission boundary exploration
(`docs/design/admission-boundary-questions.md`) intentionally leaves one
boundary unresolved:

```text
outside material
        |
        ?
        |
        v
historical record
```

This document maps how a future governed admission mechanism could
*accidentally* violate Magpie doctrine — the failure modes an eventual
design must make impossible, not the design that makes them impossible.
Failure modes over solutions; questions over decisions; invariants over
mechanisms.

The base invariant constrains every answer:

> Magpie runs alone; integrations add evidence, not authority.

## 1. Authority escalation threats

Ways external layers could accidentally gain authority:

- MCP interface exposure interpreted as permission.
- Capability descriptions treated as grants.
- AgentProposer output treated as evidence.
- External identity treated as trust.
- Interface availability treated as endorsement.

Each converts interface metadata into authority without any governed
decision. The ratified contracts already prohibit every one of these
conversions; the threat is an implementation quietly reintroducing them.

## 2. Admission boundary confusion

Risks where admission becomes more than historical inclusion:

- admission accidentally becoming truth creation;
- admission accidentally becoming standing assignment;
- admission criteria becoming hidden policy;
- rejected submissions interpreted as falsity.

The separation to preserve:

```text
admission ≠ truth creation
          ≠ standing assignment
          ≠ policy decision
```

Admission answers "may this become part of Magpie history?". Anything more
is a doctrine violation, however convenient it appears.

## 3. Log authority threats

Ways an implementation could bypass `LogWriter` as the sole low-level
append authority:

- hidden writes through caches;
- side databases becoming shadow history;
- annotations becoming unofficial records;
- memory layers becoming authoritative;
- projections mistaken for source history.

The signed append-only log is the sole historical authority. Any state that
behaves as history without passing through `LogWriter` is the
duplicated-truth failure the spine sentence exists to forbid (ADR-0001).

## 4. Provenance threats

- recorded attribution mistaken for verified identity;
- source labels becoming trust signals;
- missing provenance silently repaired instead of reported;
- summaries detaching conclusions from source history.

Provenance identifies origin; it is a reproducibility coordinate and audit
aid, never authority. Vocabulary presence grants no authority.

## 5. AgentProposer threats

```text
AgentProposer
    |
    v
proposal material
    |
    ?
    |
    v
history
```

Potential failure modes around the unresolved boundary:

- generated content treated as evidence;
- repeated proposals gaining implicit authority through multiplicity;
- model confidence becoming epistemic status;
- agent labels becoming trust markers;
- autonomous loops bypassing the admission boundary entirely.

Repetition does not amplify without policy, and agents may propose but
cannot settle claims. The threat is an implementation that lets volume,
confidence, or autonomy substitute for governed admission.

## 6. Failure semantics threats

- failed admission attempts becoming negative evidence;
- retries creating accidental amplification;
- unavailable systems interpreted as falsity;
- operational errors becoming epistemic outcomes.

Failure is not falsity. A failure at the boundary is a typed failure about
the attempt, never a fact about the proposition.

## 7. Projection and retrieval threats

- caches becoming history;
- embeddings becoming authority;
- retrieval ranking becoming standing;
- summaries replacing source records;
- stale projections mistaken for current truth.

Derived projections are rebuildable views over recorded history,
subordinate to the log in every case; storage representations are not
canonical, and no retrieval artifact outranks the history it was derived
from.

## Non-goals

This document adds none of the following:

- an `EpistemicGate` design or admission mechanism;
- mitigations or countermeasures;
- implementation details or code structure;
- a security architecture;
- identity systems, permissions, or authentication;
- trust scoring or ranking systems;
- agent autonomy design.

## Relationship to existing doctrine

Whatever eventually answers the "?", it must preserve:

- the signed append-only log as sole historical authority;
- `LogWriter` as the sole append authority;
- vocabulary closure (`actor_class`, `evidence_kind`, `edge_kind`, and
  claim-domain values extend only through their existing governance paths);
- provenance carriage per the provenance response contract;
- projection subordination;
- explicit policy selection — no ambient latest;
- failure is not falsity.

A threat in this document is any path by which an implementation weakens
one of these without a reviewed contract change. That is the whole model.
