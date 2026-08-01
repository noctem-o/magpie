# Admission Boundary Questions

## Status

Design exploration. Docs-only. Nothing in this document is decided
doctrine: it records unresolved questions and the invariants that constrain
their answers. It defines no mechanism, and it is not part of the ratified
contract stack. The governing documents for this area are ADR-0002 and the
ratified MCP contracts; where this note appears to conflict with them, they
govern — and the conflict is a question to resolve, not an exception.

## Purpose

The ratified contracts define what interfaces, queries, capabilities,
provenance carriage, and agent proposals may not do. They deliberately
leave one centre undefined: the future governed admission boundary. This
document collects what must be answered before any such boundary is
implemented.

Central question:

> What is the smallest governed boundary through which something outside
> Magpie history may become part of Magpie history?

The base invariant constrains every answer:

> Magpie runs alone; integrations add evidence, not authority.

## 1. Admission inputs

Settled: nothing enters the signed append-only log except through
`LogWriter`, the sole append authority. No ordinary claim-bearing or
evidence-bearing write surface exists before `EpistemicGate`.

Open questions:

- Which input categories must the boundary accept? Candidates named by
  existing doctrine include agent-generated proposals, external evidence
  references, Deadbolt execution anchors, human observations, and imported
  records. Is that partition right, and is it complete?
- Do different input categories need different admission strictness? If so,
  where is the variation expressed — one boundary with categories, or
  separately reviewed seams per category?
- What recorded attribution does the boundary owe each category?

No schemas, APIs, or category formats are defined here.

## 2. Admission purpose

Settled: admission is not truth creation, not standing assignment, and not
a policy decision. Standing remains a deterministic standing resolver
output over verified history; policy selection is explicit; there is no
ambient latest.

Open questions:

- Is admission only a historical recording boundary — "this material was
  submitted, from this recorded attribution, at this chain position" — or
  does it carry any further semantic weight?
- If admission is recording-only, what exactly does the boundary check
  before permitting an append? Structural validity belongs to L0;
  epistemic policy belongs to the standing resolvers. What remains that is
  admission's own?
- If admission carries any weight beyond recording, what prevents that
  weight from becoming a shadow authority?

## 3. Relationship to LogWriter

Settled:

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
historical authority
```

`LogWriter` remains the sole low-level append authority. No alternative
write path exists, and none is introduced by this exploration.

Open questions:

- Does the admission boundary hold `LogWriter`, invoke it, or return
  constructed material to a caller that holds it? This is a custody
  question, not a mechanism question.
- What does the boundary add beyond what `LogWriter` already enforces
  structurally?

## 4. Relationship to policy and standing

Settled: admission answers "may this become part of Magpie history?";
policy and standing answer "what epistemic status does recorded history
support?". These concepts must not merge.

Open questions:

- May an admission decision ever consult standing resolver output, or must
  admission remain resolver-blind?
- What keeps admission criteria from drifting into de facto policy over
  time?

## 5. Failure semantics

Settled: failure is not falsity. A rejected admission attempt is not
evidence against the underlying proposition; it is a typed failure about
the attempt.

Open questions:

- Is anything recorded about a rejected attempt? If rejections are
  recorded, how are they prevented from reading as negative evidence?
- If rejections are not recorded, what prevents silent retry-amplification
  — the same material resubmitted until it passes?

## 6. AgentProposer implications

Settled (AgentProposer boundary contract): agent-generated output remains
non-authoritative unless a separately governed admission path admits it;
agents may propose but cannot settle claims; the four closed vocabularies
are protected from agent-generated extension.

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

Open questions:

- What, if anything, does the "?" contain that the admission boundary
  itself does not already define?
- Does agent-originated material require anything at admission that
  human-originated material does not — or is the boundary uniform?
- How is recorded attribution preserved through admission without becoming
  verified identity?

This document defines no proposal schemas, review workflows, trust
systems, or agent permissions.

## 7. Future constraints

Any future admission mechanism must preserve, at minimum:

- the signed append-only log as sole historical authority;
- `LogWriter` as the sole append authority;
- vocabulary closure — `actor_class`, `evidence_kind`, `edge_kind`, and
  claim-domain values extend only through their existing governance paths;
- provenance carriage per the provenance response contract;
- projection subordination — derived state remains rebuildable and never
  authority;
- explicit policy selection — no ambient latest;
- no authority from identity, capability, or interface exposure;
- failure is not falsity.

An admission design that cannot preserve every one of these is not a
candidate.

## Non-goals

This document adds none of the following:

- an `EpistemicGate` design or admission mechanism;
- MCP write paths;
- authentication, identity systems, permissions, or capability tokens;
- trust scores or confidence scoring;
- retrieval architecture or embeddings;
- agent autonomy mechanisms;
- implementation details of any kind.

## Using this document

These questions are prerequisites. When the governed write-path design
conversation begins (ADR-0002's rollout), each open question here should
be answered explicitly — by ADR or contract amendment — before
implementation, not discovered during it.
