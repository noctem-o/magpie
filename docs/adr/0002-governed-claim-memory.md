# ADR-0002: A Memory Event Earns Standing via a Governed Write Path and Deterministic Projection

**Status:** Accepted (ratified by George, 2026-08-02)

ADR-0002 amends ADR-0001. It does not supersede ADR-0001's anchored
hierarchy: Deadbolt still seals evidence and anchors occurrence into Magpie;
this ADR decides what Magpie may derive from claim-relevant memory events after
they enter the log.

## Summary / Y-statement

In the context of an append-only provenance memory for AI-assisted research,
facing the risk that stored claims become treated as truth merely because they
were written, we decide that claim standing is derived by a deterministic
projection over governed, typed memory events, to achieve replayable epistemic
discipline, accepting that the first implementation uses coarse evidence
ceilings and opaque scope references rather than a full belief-revision engine.

Deadbolt acts. Magpie remembers. ADR-0002 decides how memory earns standing.

## Context and problem statement

Magpie's L0 remains a signed, hash-chained, append-only log. Everything else is
a derived projection. ADR-0001 gave Deadbolt a narrow, live seam into that log:
after Deadbolt seals and verifies a bundle, it may append a `SegmentAnchored`
event that records occurrence, order, signature, and inclusion.

A recent Deadbolt field test exercised the seam with hostile material: a real
sealed observation bundle had `witness_root.txt` changed to a different
syntactically valid 64-character lowercase hex value. `cog anchor reconcile`
re-verified the bundle, refused it with `verification_failed`, exited nonzero,
and did not append the tampered root. `cog anchor status` reported the
remaining debt without mutating the anchor log.

That proves the occurrence anchor can be protected. It does not answer the next
question: what is Magpie allowed to believe about that evidence?

## Decision drivers

- L0 is the sole source of truth.
- Standing must not be a mutable field that silently becomes authority.
- Corrections must be replayable events, never edits or deletes.
- Ordinary agents may propose claims or evidence but must not directly settle
  claims.
- Evidence has kind-specific limits; not every signal can establish truth.
- Projection behavior must be deterministic and test-enforced.
- Formal ancestry may guide the design, but Magpie must not smuggle in a large
  theory engine before it has earned one.

## Considered options

1. Store current truth directly in event payloads.
2. Build a full argumentation or belief-revision engine now.
3. Keep L0 simple and add a governed write path plus a deterministic
   `StandingView` projection.

Option 3 is chosen.

## Decision

A memory event earns standing only through a governed write path and a
deterministic projection over the append-only log.

The governed write path admits claim-relevant memory events. A deterministic
projection named `StandingView` derives current standing by replaying L0 in
stable order. Standing is not stored as an authoritative mutable field.
Corrections, invalidations, supersessions, and ratifications are new events.

Tests are enforcement, not decorative documentation. Future implementation of
ADR-0002 must pin the fold with adversarial tests before any new write surface
is treated as real.

## Normative semantics

- L0 remains the sole source of truth.
- Standing is not stored as an authoritative mutable field.
- Standing is derived from replaying the append-only log.
- Corrections are new events, never edits or deletes.
- The governed write path admits claim-relevant memory events.
- Ordinary agents may propose claims or evidence but may not directly settle
  claims.
- `StandingView` derives current standing deterministically.
- Formal systems cited here are explanatory ancestry, not imported semantics.
  If a future implementation requires multiple competing accepted standings,
  credulous acceptance, scope inheritance, or probabilistic evidence fusion,
  this ADR must be revisited before the fold is expanded.

## Actor classes

These are semantic actor classes for the governed claim write-path. This
docs-only ADR does not require a Rust enum implementation.

- `HumanRoot`: a human authority able to ratify governance, consent,
  preference, scoped judgment, or operational decisions. Human authority is not
  epistemic settlement authority.
- `AgentProposer`: an ordinary agent able to propose claims, evidence, and
  review debt, but not settle claims directly.
- `AutomatedVerifier`: deterministic machinery that can verify exact
  predicates and report machine-checkable outcomes.
- `DeadboltAnchorer`: the governed execution boundary that anchors sealed
  occurrence evidence into Magpie.
- `LensWitness`: an interpretability or readout tool that can provide
  admissible but ceilinged signal.
- `SourceImporter`: a controlled importer for external sources and citations.

## Evidence kinds and ceilings

A ceiling is a deterministic cap on what one evidence kind may establish alone.
It is not a probability and it is not numeric confidence.

Corrected standing doctrine: humans settle nothing epistemically. A human may
approve an action, ratify a judgment, or provide evidence that a judgment
occurred, but `HumanRatification` is governance/judgment evidence rather than a
truth-settlement authority. `Settled` is reserved for deterministic,
cryptographic, exact replay, or exact occurrence/inclusion facts admitted by
policy. A human can approve a risky action; they cannot make a hash match by
decree.

| evidence kind | maximum standing ceiling |
| --- | --- |
| `DeterministicVerification` | May reach `Settled` for exact machine-checkable predicates. |
| `HumanRatification` | May reach `Supported` for scoped human judgment or governance evidence; it must not settle truth. |
| `DeadboltAnchor` plus successful verifier context | May reach `Settled` for occurrence/inclusion, but only `Supported` for interpretation. |
| `ExecutionEvidence` | May reach `Supported`. |
| `BehavioralEvaluation` | May reach `Supported`, discounted because models may detect or game evaluations. |
| `ExternalSource` | May reach `Conjectured` or `Supported` depending on policy and corroboration. |
| `ModelSelfReport` | May reach `Conjectured`. |
| `LensReadout` | May reach `Conjectured`; it may corroborate, contradict, or create review debt, but must never settle a claim alone. |

## Justification edge vocabulary

The v1 justification edge vocabulary is intentionally small:

- `supports`
- `derived_from`
- `contradicts`
- `supersedes`
- `invalidates`
- `ratifies`

Vague relation types are rejected:

- `related_to`
- `similar_to`
- `about`
- `probably_true`
- `associated_with`

The small vocabulary is intentionally restrictive to prevent relation slop.

## Scope references

Tags 6–8 use `scope_ref: String` as a v1 design rule.

- `scope_ref` is an opaque exact-match string in v1.
- There is no inheritance, containment, lattice, ontology, wildcard, or prefix
  semantics in v1.
- Scope inheritance is explicitly deferred.
- If scope later acquires containment or inheritance semantics, ADR-0002 must
  be amended or superseded.

## StandingView fold rules

The intended fold is semantic only in this ADR.

- Replay is deterministic.
- Projection uses stable ordering.
- Standing derives from claims, typed evidence, and justification edges.
- Invalidated evidence contributes no support.
- A recorded `supersedes` edge preserves attributed lineage; predecessor
  and successor remain in append-only history, inspectable and unchanged.
- Edge presence has no self-executing standing or currency consequence.
- Any contradiction-debt or other standing consequence of supersession
  belongs to an explicitly selected standing policy under ADR-0003
  standing coordinates.
- Any applicability or currentness consequence of supersession belongs to
  an explicitly selected currency policy under ADR-0006 coordinates.
- Ratification has only the consequence granted by the explicitly selected
  policy that consumes it; it is not ambient authority.
- Contradiction creates debt and blocks `Settled` until the explicitly
  selected standing policy determines that eligible invalidation,
  supersession, or ratification resolves it.
- Cyclic justification must not self-support.
- Acyclic, well-founded support is required.
- A single low-ceiling evidence kind cannot promote a claim beyond its
  ceiling.
- Multiple independent evidence records may promote standing only under
  explicit fold rules; ADR-0002 does not invent probabilistic fusion.

ADR-0006 supersedes only this ADR's former `StandingView` sentence that
superseded claims "should not remain current unless explicitly ratified under
the new scope." That sentence conflated standing and currentness; the facet
ownership above replaces it. ADR-0006 does not otherwise supersede ADR-0002,
and the v1 exact-match scope rules remain unchanged.

`StandingView` is a small deterministic fold. It is not a Dung, JTMS, ATMS, AGM,
or probability engine.

## Compatibility with v0 events

Existing tags 0-5 remain unchanged.

- `ClaimAsserted`: legacy assertion event. Its status is treated as an
  asserted initial status, not current truth.
- `EvidenceRecorded`: legacy untyped evidence note. It may weakly support the
  target claim, with a ceiling no higher than `Supported` unless re-registered
  by a governed typed `EvidenceRegistered` event.
- `ClaimStatusChanged`: legacy/manual standing judgment. New ordinary writers
  should not emit it. It remains for compatibility only and is a known
  pre-`EpistemicGate` gap: new governed paths must not use it as a way for
  humans or agents to settle truth by decree.
- `Note`: ignored by `StandingView` unless a future ADR says otherwise.
- `SegmentAnchored`: execution evidence anchor, not itself a claim. It may be
  cited by typed evidence or justification events. It is occurrence
  evidence, not interpretation truth.

## Frozen tags 6–8

ADR-0002 introduced these additive tags:

- tag 6: `ClaimAssertedV2`
- tag 7: `EvidenceRegistered`
- tag 8: `JustificationEdgeRecorded`

They are implemented, and `docs/FORMAT.md` freezes their exact fields and
canonical encodings under `magpie-core-v1`. Existing fields and bytes are not
provisional and must not be reinterpreted. Any future representation change
must be additive under FORMAT's evolution rules, preserve every existing tag
encoding and golden record, and use explicit projection match arms rather than
wildcards.

## Confirmation / enforcement tests

Later code PRs should add these tests before treating ADR-0002 as implemented:

- `standing_replay_is_deterministic`
- `agent_proposer_cannot_settle`
- `human_ratification_supports_judgment_but_does_not_settle_truth`
- `lens_readout_cannot_settle`
- `contradiction_debt_blocks_settled`
- `invalidated_evidence_contributes_no_support`
- `supersession_preserves_lineage`
- `cyclic_justification_does_not_self_support`
- `legacy_v0_chains_still_replay`
- `segment_anchor_is_occurrence_evidence_not_interpretation_truth`

## Consequences

Positive:

- Claim standing becomes replayable, inspectable, and regenerable.
- Agents can contribute memory without becoming hidden authorities.
- Deadbolt anchors can be used as occurrence evidence without overclaiming
  interpretation truth.
- The first implementation can stay small enough to review.

Negative / deferred:

- v1 uses coarse evidence ceilings.
- `scope_ref` is opaque and exact-match only.
- Any additional typed claim, evidence, or edge representation requires
  additive format evolution; tags 6–8 remain frozen.
- No full belief-revision or argumentation machinery exists yet.

## Rejected alternatives

- Status-as-truth stored directly in event payloads.
- Full Dung argumentation engine.
- Full JTMS/ATMS engine.
- AGM-style belief revision as machinery.
- Numeric confidence/probability as v1 standing.
- Knowledge graph in L0.
- Lens readouts as privileged oracle truth.
- Human ratification as epistemic settlement authority.
- Letting agents write settled claims directly.
- Implementing scope inheritance in v1.

## Rollout plan

1. Land this docs-only ADR package.
2. Implement a `StandingView` skeleton over existing v0 events only.
3. Add the ADR-0002 enforcement tests against the v0-compatible `StandingView`.
4. Implement additive tags 6–8 (completed):

   - `ClaimAssertedV2`
   - `EvidenceRegistered`
   - `JustificationEdgeRecorded`

5. Freeze tags 6–8 in `docs/FORMAT.md`, canonical encoding, golden vectors,
   independent verifier support, and explicit projection match arms in the
   same reviewed code PR (completed).
6. Add `EpistemicGate` / governed write admission.
7. Only then expose governed write surfaces.

The fold and its tests preceded tags 6–8 because ADR-0002's own rule is that
standing must be earned before it is trusted.

If implementing this requires editing Rust, canonical encoding, golden vectors,
verifier code, or Deadbolt during this docs-only package, stop and report why
rather than proceeding.

## References / ancestry

These are ancestry, not machinery:

- Grounded argumentation semantics: useful north-star for deterministic,
  skeptical, single-outcome fixed-point reasoning.
- JTMS: useful ancestor for justification-based belief and invalidated support
  relabeling.
- Screened / credibility-limited belief revision: honest lineage for the
  governed write gate, because new evidence does not automatically win.
- Event sourcing and bitemporal correction-event practice: justification for
  demote-don't-delete.
- PROV-O / PROV-DM: provenance half, but not epistemic support.
- SLSA / in-toto: typed evidence and verify-before-trust prior art.
- Dempster-Shafer discounting and legal hearsay/corroboration: ancestry for
  evidence ceilings, not implementation.
- Interpretability / lens work: admissible signal, but model-specific and
  ceilinged.

Magpie does not claim to implement any of these theories in ADR-0002.
