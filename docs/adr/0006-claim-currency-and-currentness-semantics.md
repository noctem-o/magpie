# ADR-0006: Claim Currency and Currentness Semantics

**Status:** Proposed

## Summary / Y-statement

In the context of a record that accumulates currency-relevant acts —
supersession edges, invalidation edges, and withdrawal acts — alongside
the assertions they describe, facing the risk that "current" hardens into
stored lifecycle state or into ambient-latest authority that no policy
chose and no snapshot can audit, we decide that currentness is a derived
facet: recorded acts supply the history, and a resolver at explicit
snapshot and policy coordinates computes which recorded material is
applicable — never writing currentness back into the record — to achieve
currency answers that are replayable, policy-auditable, and able to
disagree legitimately, accepting that "current" has no global answer,
that multiple policies may coexist over the same record, and that every
representation question waits for the vocabulary freeze ritual.

## Context and problem statement

ADR-0004 drew the lifecycle facet boundary: a claim has no stored
lifecycle state; the record holds attributed acts; lifecycle is a derived
facet view over verified history. It named currentness as a future
currency facet and seeded its doctrine: a later event may mark an earlier
one superseded, but both remain in the record, and "current" is always a
resolver-relative, snapshot-relative answer.

ADR-0005 settled withdrawal: an attributed act targeting the actor's own
assertion act, asserting no falsity, determining neither standing nor
currentness by itself.

What remains unsettled is the currency facet itself. The frozen edge
vocabulary of FORMAT §3 already contains `supersedes` and `invalidates`.
ADR-0002 settled one invalidation rule — invalidated evidence contributes
no support — and the contradiction rule: contradiction creates debt and
blocks settlement. The rest has no ratified semantics: what `supersedes`
asserts, whether `invalidates` reaches beyond evidence, and what
"current" means when a resolver answers a question. Working sketches
exist in the tags 6–8 implementation plan ("supersedes preserves lineage
and makes old material non-current under the same exact scope";
"invalidates removes support contribution on replay"), but sketches are
not doctrine.

Without a boundary, "current" is where hidden authority enters. A reader
who treats the latest recorded assertion as current has installed a
policy nobody wrote down: recency, applied universally, immune to
snapshot and inspection. A record that stored currency on the claim
would reintroduce the mutable lifecycle state ADR-0004 prohibited. Both
failure modes are quiet. This ADR makes the boundary explicit before
either becomes load-bearing.

## Decision questions

1. What does "current" mean in Magpie?
2. At what granularity does the currency facet answer — per claim, or per
   assertion/material?
3. What does `supersedes` assert, and what does it not assert?
4. What does `invalidates` assert, and over which targets?
5. May a currency policy inspect recorded time metadata?
6. What is the boundary of a currentness resolver?

## Decision

**1. "Current" means resolver-relative applicability.** A currency answer
is the output of a named resolver, running a named policy version, over a
named snapshot. Applicability is always applicability to a question under
a policy, never a property of the record itself. Rejected readings:

- *latest recorded assertion* — latest is a property of log ordering, not
  of applicability; adopting it makes recency a policy nobody chose;
- *latest event wins* / *newest means correct* — the same error with a
  truth claim attached;
- *ambient-latest semantics* — a currency answer without coordinates is
  not a Magpie currency answer;
- *latest surviving assertion* — "surviving" hides policy inside an
  undefined word and collapses currency into standing, smuggling the
  fold's support accounting into a facet that must stay policy-explicit.

**2. The currency facet is per-assertion/material.** Each relevant
assertion or material receives a currency interpretation under the
resolver. The facet does not elect one current item per claim: a
claim-level single answer would become a canonical truth carrier, which
Magpie's multiplicity, provenance, and actor-separation doctrine forbids.
A policy may derive a claim-level summary from per-assertion annotations
as a convenience view; that summary is identified by its policy version,
is never stored, and is never canonical.

**3. `supersedes` records lineage. Nothing more.** The edge records that
an attributed actor registered one item as the successor of another. Its
currency consequence belongs to resolver policy, not to the edge.

**4. `invalidates` remains scoped to evidence.** The settled ADR-0002
rule stands — invalidated evidence contributes no support on replay — and
this ADR does not extend invalidation to claims, assertion acts, or
relationship edges.

**5. Recorded time creates no currency semantics by itself.** The log is
ordered, not clocked. A future explicitly named policy may inspect
recorded metadata such as timestamps; "latest timestamp determines
currentness" is forbidden.

**6. The currentness resolver is a pure function of coordinates.**
`resolver(record snapshot, policy version) -> currentness facet`, with
the boundary stated below.

**7. Withdrawal, supersession, invalidation, and contradiction are
distinct recorded acts and relationships.** Withdrawal changes neither
currentness nor standing automatically; both remain resolver policy
decisions, exactly as ADR-0005 settled.

## Currency facet semantics

**Acts versus views.** The record holds attributed acts and recorded
relationships: assertions, withdrawals, and edges such as `supersedes`
and `invalidates`. Views are computed. Currentness is a view. No event
writes a currency outcome; no reader trusts a stored one.

**Coordinates.** Every currency answer carries its snapshot and policy
version. The facet is computed over the verified record prefix ending at
the snapshot, under the named policy. Later events can never change an
earlier snapshot's answer.

**No stored current state.** `claim.current = true` and
`claim.status = active` are forbidden shapes. The invariant:

> Currency acts are recorded history; currentness is never recorded.
> Currentness exists only as resolver output at explicit snapshot and
> policy coordinates.

**Determinism.** Same snapshot plus same policy version produces
identical facet output — the replay discipline the standing view already
pins.

**The facet is not evidence.** Facet output never re-enters the record as
an act. A projection is a reading, not a contribution; it carries no
provenance authority of its own.

**Withdrawal interaction.** Withdrawal is an act; currentness is a view.
A withdrawal act does not move any currency answer by itself — a policy
decides what a recorded withdrawal means for applicability, just as it
decides what a recorded supersession means. No act reaches around policy
to set its own interpretation.

## Supersession semantics

`A supersedes B` records that an attributed actor registered A as the
successor of B. It is a lineage fact.

It does **not** mean:

- the earlier item was false;
- the earlier item is deleted;
- the earlier item is invalid;
- the newer item is automatically authoritative;
- the resolver must ignore the earlier item.

Both items remain in the record, replayable and auditable. The currency
consequence of supersession belongs to resolver policy: one policy may
treat superseded material as no longer applicable within a scope; another
may keep it applicable. The edge compels nothing — a self-executing
supersession would be authority smuggled into vocabulary.

**Target scope.** The edge relates recorded material. What it may
reference, and how, is a representation question deferred to the
vocabulary freeze ritual; this ADR defines doctrine, not payloads.

**Scope equality.** Succession may be recorded as scope-bound. Whether
succession across differing scopes carries currency meaning is a policy
decision, not an edge property.

**Cycles.** A supersession cycle is degenerate lineage. The record keeps
it without comment; a resolver policy must treat cycles
deterministically; cyclic or self-referential edge sets do not
self-promote into currency.

## Invalidation semantics

The settled core, from ADR-0002: invalidation of evidence removes the
target's support contribution during replay. It does not delete evidence;
it does not assert falsity; the target remains in the record.

This ADR does not extend invalidation beyond evidence. The self-directed
case for assertions is already covered by withdrawal (ADR-0005). Removing
the contribution of another actor's claim, assertion, or edge is a
stronger power whose authority rules are undeveloped, and conservative
doctrine admits no scope that existing acts cannot yet justify. Any
extension requires a future ADR and the vocabulary freeze ritual.

Invalidation remains distinct from:

- **falsity** — no act asserts falsity; truth-adjacent judgment is
  standing and verity resolver output;
- **contradiction** — contradiction signals conflict and creates debt
  while removing nothing; invalidation removes contribution and creates
  no debt. The bookkeeping runs in opposite directions;
- **withdrawal** — withdrawal is self-directed cessation of endorsement;
  invalidation is other-directed challenge to contribution;
- **deletion** — an append-only record admits no deletion.

## Currentness resolver boundary

- **Deterministic replay.** Same snapshot plus same policy version yields
  the same facet, byte-identical.
- **Snapshot relativity.** The resolver reads only the verified record
  prefix ending at the snapshot. Later events never change an earlier
  snapshot's output.
- **Policy relativity.** Two policy versions over one snapshot may return
  different facets. Both are valid answers, each identified by its policy
  version. Neither is "the" answer.
- **Coexistence of projections.** Any number of resolvers and policies
  may operate over one record. No currentness projection becomes
  universal authority — an authoritative projection is the ambient-latest
  trap in resolver clothing.
- **No act creation.** The resolver interprets history; it does not
  create acts. Interpretation never produces withdrawal, supersession, or
  invalidation acts — ADR-0005's resolver boundary carried into currency.

## Permanent non-definition

Currentness is not:

- truth;
- latest;
- freshness;
- a score;
- wall-clock validity;
- deletion;
- stored state.

Supersession is not:

- falsity;
- deletion.

Invalidation is not:

- contradiction;
- withdrawal.

## Deferred decisions

- Payload representation for `supersedes` and `invalidates` semantics —
  target kinds, any additional fields — through the vocabulary freeze
  ritual.
- Vocabulary freeze items: representation-level ratification of the
  tags 6–8 sketches; any future invalidation target extension; currency
  facet naming.
- Currency facet field naming and output encoding.
- Resolver algorithm details.
- Future policy specifics, including the rules of any policy that
  inspects recorded time metadata.
- Any claim-level summary derivation convention, if one is ever wanted.

## References

- ADR-0002 — fold rules, contradiction debt, invalidated evidence
  contributes no support, corrections are new events;
- ADR-0003 — the canonical definition of standing: resolver output over a
  verified snapshot under an explicit policy;
- ADR-0004 — lifecycle facets; the currency seed sentence; the log is
  ordered, not clocked;
- ADR-0005 — the withdrawal boundary; resolvers interpret history and do
  not create acts;
- `docs/FORMAT.md` §3 — the frozen payload vocabulary and its evolution
  rule;
- `docs/design/historical-review-remediation-ledger.md` — the historical
  remediation ledger;
- `docs/design/adr-0002-tags-6-8-implementation-plan.md` — the tags 6–8
  exploration sketches for `supersedes` and `invalidates`.
