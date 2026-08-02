# ADR-0005: Claim Withdrawal Acts

**Status:** Proposed

## Summary / Y-statement

In the context of a claim memory whose lifecycle is a derived facet view
over attributed acts, facing the need for actors to retract their own
assertions without implying falsity or mutating history, we decide that
withdrawal is a recorded act by an attributed actor targeting that actor's
own prior assertion act, to achieve retraction without truth claims,
accepting that standing interpretation of withdrawn assertions is a future
policy decision and that delegation is deferred to identity doctrine.

## Context and problem statement

Actors change their minds. Doctrine has edge kinds for invalidation,
supersession, and contradiction, but no way for an actor to withdraw
*their own* assertion. Without a withdrawal act, that need will be served
by misuse: `invalidates` aimed at one's own claim — an interpretation
that incorrectly gives withdrawal truth-evaluating semantics — or worse,
a desire to edit history.

ADR-0004 bounded the concept without defining it: withdrawal may record a
retraction by an attributed actor and may never assert the falsity of what
it retracts. This ADR defines the act within that bound.

## Decision

Two canonical invariants:

> **Withdrawal is a recorded act by an attributed actor; it records
> cessation of endorsement of a prior assertion and does not assert
> falsity of what it withdraws.**

> **Withdrawal is an act; standing remains a resolver output.**

And the target rule: withdrawal targets an attributed assertion act, never
a claim object.

This ADR admits no vocabulary. If a new event, tag, or edge representation
is required to express withdrawal, that representation is a decision of
this ADR in principle and an item of the vocabulary freeze ritual in
practice: new payload tags require the full format evolution path —
`docs/FORMAT.md`, canonical encoding, golden vectors, and the independent
verifier in the same reviewed change.

## Withdrawal semantics

A withdrawal act records exactly one thing: the attributed actor's
cessation of endorsement of their own prior assertion.

It does not:

- invalidate a claim;
- contradict a claim;
- delete history;
- correct history;
- settle standing;
- determine currentness.

The withdrawn assertion remains in the record, inspectable forever. The
withdrawal joins it; it does not erase anything.

## Target semantics

The target of a withdrawal is the withdrawing actor's own prior assertion
act.

Claims are shared proposition identities — scoped, first-write-retained
records that multiple actors may assert alike. Assertions are actor
commitments. Withdrawing a claim would imply mutable lifecycle state over
a shared object; withdrawing an assertion preserves multiplicity and
provenance: actor A's withdrawal of A's assertion says nothing about actor
B's assertion of the same proposition.

How a withdrawal references its target (event-level identity versus
claim-level naming) is representation — decided through the freeze ritual
above, not silently chosen.

## Attribution rules

- An actor may withdraw only its own assertion acts. Withdrawal directed
  at another actor's acts is invalid by construction.
- Attribution remains recorded, asserted data — never verified identity.
- Resolvers and systems do not withdraw: writers and actors create
  recorded acts under authority, while resolvers interpret recorded
  history — interpretation creates no withdrawal acts. A verification
  process gains no withdrawal authority merely by observing evidence.
- Delegation and organisational withdrawal (an organisation retracting a
  member's statement) are deferred to identity and governance doctrine;
  this ADR decides nothing about them.

## Relationship to lifecycle facets

Under ADR-0004, a withdrawal act is a historical act; lifecycle facets
derive over it like any other recorded act. The facet non-authority
invariant applies without modification: a facet view over withdrawals
cannot create, replace, or strengthen evidence, and settles nothing by
existing.

## Relationship to standing

Withdrawal's effect on standing is deferred entirely. Whether a withdrawn
assertion remains candidate-eligible, produces a blocker, or changes any
outcome is a future standing-policy decision (a v5+-shaped ADR). This ADR
defines the act's existence and semantics; no fold behaviour is defined
here.

## Permanent non-definition

Withdrawal is not:

- **falsity** — it records cessation of endorsement, never untruth;
- **invalidation** — that is a different edge with different semantics;
- **contradiction** — it names no conflict and no winner;
- **supersession** — it establishes no replacement material and no
  lineage;
- **deletion** — the record is append-only;
- **historical correction** — the prior act remains inspectable and
  unchanged;
- **an automatic standing change** — standing remains a resolver output;
- **an automatic currentness change** — ADR-0006 governs currentness; a
  withdrawal act changes no currency result by itself, and any effect
  remains the exact-target-local output of an explicitly selected currency
  policy;
- **evidence removal** — the withdrawn act remains in history and in
  projections until a policy says otherwise;
- **authority over other actors' acts** — self-targeting only;
- **identity verification** — attribution is asserted data.

## Deferred decisions

- Vocabulary representation of the withdrawal act (freeze ritual: tag or
  edge, target-identity scheme, reason fields, actor-class constraints).
- Standing and fold interpretation of withdrawn assertions (future policy
  ADR).
- Concrete currency-policy treatment, resolver algorithm, representation,
  canonical encoding, runtime output surface, and tests and vectors for
  withdrawal under ADR-0006's exact-target-local boundary.
- Delegation and organisational withdrawal (identity / governance
  doctrine).

## References

- ADR-0002 — fold rules, correction-event practice, closed actor classes;
- ADR-0003 — the canonical definition of standing;
- ADR-0004 — lifecycle facets; the withdrawal bound this ADR defines
  within;
- ADR-0006 — currentness and currency semantics, including the
  exact-target-local interpretation boundary for withdrawal;
- `docs/FORMAT.md` §3 — the frozen payload vocabulary and its evolution
  rule;
- `docs/design/historical-review-remediation-ledger.md` — scoped claim
  identity and first-write retention.
