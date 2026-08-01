# ADR-0003: Canonical Definition of Standing

**Status:** Proposed

## Summary / Y-statement

In the context of a replayable epistemic memory kernel whose governed
standing policies v0–v4 already compute standing over verified history,
facing the risk that the word *standing* drifts as later ADRs and documents
reference it, we decide to freeze one canonical definition that
consolidates existing doctrine, to achieve a single citable meaning for
every future document, accepting that the definition constrains wording
without constraining any particular resolver mechanism.

This ADR invents nothing. Standing is already computed by the implemented
policies v0–v4 and already described, in fragments, across the doctrine
corpus. This ADR freezes that existing meaning.

## Context and problem statement

Standing is referenced by the README, ADR-0002, the v0.1.0 release
contract, the boundary contracts, and the admission exploration set. Each
document restates its own fragment of the meaning; none is canonical, and
later ADRs will need to cite the concept constantly. Without a single
definition, each new document either duplicates a fragment or drifts from
it.

## Decision

Standing is defined canonically by one sentence:

> **Standing is the conclusion a named policy reaches from verified
> history: the deterministic output of a named resolver over a verified
> record snapshot, reproducible from exactly those coordinates — snapshot,
> policy version, resolver — and authoritative about nothing beyond them.**

Future documents cite this ADR; they do not restate the definition.

## Normative semantics

- Standing has exactly three coordinates: the verified record snapshot, the
  explicitly selected policy version, and the named resolver identity.
- Standing is deterministic: the same coordinates produce the same result.
- Standing is reproducible from exactly those coordinates and nothing else.
- Standing is derived, never stored: it is computed by replaying recorded
  history, not written into it.
- Standing is policy-relative: it reports what the explicitly selected
  policy version concluded, nothing more universal.
- There is no ambient latest: no latest policy, latest resolver, or latest
  snapshot is ever implied.
- Policy selection is explicit and remains a caller act.

## Permanent non-definition

Standing is not:

- **truth** — a resolver output is a derived view, never a truth
  assertion;
- **confidence, belief, probability, or ranking** — ADR-0002 rejected
  numeric confidence as standing; ceilings are not numeric;
- **authority** — standing is derived state and carries none;
- **evidence** — evidence ≠ standing; standing is evaluated output, never
  input;
- **stored** — standing is not an authoritative mutable field; it is
  derived by replay, never written into history;
- **ambient** — no latest anything is implied;
- **a policy decision** — the resolver applies a chosen policy version; it
  does not decide which policy should exist;
- **a human or agent decree** — humans settle nothing epistemically, and
  agents propose but cannot settle;
- **endorsement** — a standing result reports a policy conclusion; it
  approves nothing;
- **currentness** — "current" is not a standing property; currentness is
  separate, deferred doctrine.

## Relationship to existing documents

- ADR-0001 and ADR-0002 govern their domains unchanged; this ADR depends
  on them and amends neither. ADR-0002 decides how memory earns standing;
  this ADR defines what standing is.
- The README gains a pointer to this ADR in its governed-standing section;
  its existing wording remains true and stays.
- Ratified contracts and exploration documents are not retrofitted. Their
  restatements are consistent with this definition; future edits to them
  should reference this ADR rather than restate the definition.
- Every future document that uses the word *standing* cites this ADR.

## Rejected alternatives

- **Standing as a stored status field** — rejected by ADR-0002 already;
  stored status silently becomes authority.
- **Standing as truth or correctness** — rejected; Magpie is not a truth
  oracle, and no policy output becomes one.
- **Standing as numeric confidence or probability** — rejected by
  ADR-0002; numbers invite ranking, and ranking invites authority.
- **A latest-standing selector** — rejected; policy selection is explicit,
  and there is no ambient latest.
- **Per-policy definitions of standing** — rejected; the definition is
  policy-relative by construction and must not fork per policy version.

## Confirmation

This ADR changes no code and no tests. The meaning it freezes is already
enforced by existing enforcement surfaces: the governed-standing tour's
drop-and-replay byte-identical regeneration, the legacy raw-status
quarantine tests, the policies v0–v4 test suites, and the frozen golden
vectors. If a future change makes any of those contradict this definition,
the change is wrong, not this ADR.

## References

- README, "Governed standing policies" — the front-door formulation this
  ADR elevates;
- ADR-0002 — governed claim memory doctrine;
- `docs/releases/v0.1.0-contract.md` — ceiling ≠ achieved standing;
  quarantine of legacy raw standing; pinned policy identifiers;
- `docs/design/epistemic-resolution-boundary-contract.md` — the resolver /
  result / status distinction and standing ≠ truth;
- `docs/design/admission-boundary-minimal-semantics.md` — standing as
  deterministic resolver output over verified history under an explicitly
  selected policy version.
