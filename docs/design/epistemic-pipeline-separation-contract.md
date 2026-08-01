# Epistemic Pipeline Separation Contract

## Status

Boundary contract / design exploration. Docs-only. This document introduces
no mechanism and defines no APIs, schemas, event formats, or write paths.
It becomes doctrine only through explicit repository-owner acceptance, and
adoption of any future semantics still requires explicit governance — an
ADR or contract amendment.

## Purpose

Previous contracts defined individual membranes: the MCP boundary, the
librarian and query surfaces, provenance carriage, capability discovery,
AgentProposer, admission, the admission record, and evidence admission.
This document connects them into one separation model.

The central question:

> What transformations may occur between external material and epistemic
> status, and which authority belongs to each stage?

It consolidates; it does not extend. Where this document touches any
governing document — ADR-0001, ADR-0002, the ratified boundary contracts,
or the admission exploration set — those govern. This contract narrows; it
never widens.

The base invariant applies unchanged:

> Magpie runs alone; integrations add evidence, not authority.

## 1. Pipeline overview

The conceptual flow:

```text
external material
        |
        v
admission boundary
        |
        v
historical record
        |
        v
evidence interpretation boundary
        |
        v
standing resolver
        |
        v
epistemic status
```

Each transition changes representation or produces a derived view. No
transition increases authority.

## 2. Authority ownership

Conceptual ownership per stage — what each owns, and what it never owns:

| Stage | Owns | Does not own |
| --- | --- | --- |
| Admission | controlled historical inclusion | truth; evidence status; standing; policy decisions |
| Historical record | durable ordered record; replayable history | endorsement; interpretation; trust |
| Evidence interpretation | governed interpretation of recorded material | truth; standing; policy authority |
| Standing resolver | deterministic status computation over verified history | history mutation; source authority |

Ownership here means responsibility, never authority over another stage's
domain.

## 3. Forbidden collapses

Each of the following collapses creates a shadow authority layer — an
evaluation happening somewhere other than the one place it is defined:

```text
admission ≠ evidence assignment
historical record ≠ endorsement
provenance ≠ trust
identity ≠ authority
source ≠ evidential weight
evidence ≠ standing
standing ≠ truth creation
projection ≠ history
availability ≠ authority
```

Every one of these is already prohibited by a governing document; this
section names them together so the pattern is visible: any implementation
that performs one of these collapses has built an unreviewed epistemic
system alongside the governed one.

## 4. Authority flow

The core doctrine of the pipeline:

> Information may move downstream. Authority does not.

The pipeline must not allow:

- interfaces to become permissions;
- capabilities to become grants;
- agents to become authorities;
- provenance to become trust;
- evidence interpretation to become standing.

## 5. AgentProposer relationship

```text
AgentProposer
    |
    v
proposal material
    |
    v
historical record
```

does not imply:

```text
proposal = evidence
proposal = settlement
proposal = authority
```

Generated material remains attributed asserted data unless a future
governed process determines otherwise.

## 6. Deadbolt relationship

Preserved:

- specialised seams remain independently governed by their own ratified
  contracts;
- specialised semantics do not become generic semantics;
- wrapping a seam grants no additional authority.

## 7. Failure semantics

Failure is not falsity. A failure at any stage describes the operation or
the attempt, not the proposition. This document defines no failure
workflows.

## 8. Remaining open questions

This document does not answer, and explicitly reserves to future ADR or
contract work:

- `EpistemicGate` design;
- admission mechanics;
- evidence registration;
- resolver algorithms;
- policy selection mechanics;
- schemas of any kind.

The admission exploration set holds the finer-grained open questions; this
document adds none and closes none.

## Non-goals

This document adds none of the following:

- an `EpistemicGate`, evidence-registration, or resolver implementation;
- APIs, schemas, or event formats;
- write paths;
- identity or trust systems;
- scoring or ranking.

## Reviewer checklist

- Does this document preserve the signed append-only log as sole
  historical authority?
- Does it preserve `LogWriter` as sole append authority?
- Does it prevent admission becoming truth creation?
- Does it prevent history becoming evidence automatically?
- Does it prevent evidence becoming standing automatically?
- Does it preserve resolver separation?
- Does it preserve vocabulary closure?
- Does it preserve AgentProposer boundaries?
- Does it preserve Deadbolt boundaries?
- Does it avoid implementation assumptions?
