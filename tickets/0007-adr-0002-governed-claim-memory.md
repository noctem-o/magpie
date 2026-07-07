# Ticket 0007: ADR-0002 governed claim memory package

## Goal

Create the first docs-only ADR-0002 package for governed claim memory:

- `docs/adr/0002-governed-claim-memory.md`
- `docs/design/adr-0002-governed-claim-memory-dossier.html`
- `tickets/0007-adr-0002-governed-claim-memory.md`

ADR-0002 answers: Deadbolt can prove or refuse occurrence. What is Magpie
allowed to believe about that evidence?

Core thesis: a memory event earns standing only through a governed write path
and a deterministic projection over the append-only log.

## Scope

- Documentation only.
- Define the proposed semantics for governed claim memory.
- Define semantic actor classes, evidence ceilings, justification edge
  vocabulary, v0 compatibility, future tags, and future enforcement tests.
- Create a self-contained static HTML explanatory dossier.
- Create this ticket for review and follow-up planning.

## Non-goals

- No Rust changes.
- No canonical encoding changes.
- No golden vector regeneration.
- No new event tags.
- No verifier changes.
- No EpistemicGate implementation.
- No MCP write surface.
- No lens evidence implementation.
- No Deadbolt changes.
- No knowledge graph.
- No changes to `docs/FORMAT.md`.
- No changes to Python verifier files.

## Constraints

- Do not modify Magpie canonical encoding, event tags, golden vectors, or
  verifier semantics.
- Do not modify Deadbolt.
- Do not modify Rust source files.
- Do not implement `StandingView`; define only the semantic fold and future
  enforcement tests.
- `scope_ref` is opaque exact-match string material in v1; no inheritance,
  containment, lattice, ontology, wildcard, or prefix semantics.
- Treat formal systems as explanatory ancestry, not imported machinery.
- If implementing this requires editing Rust, canonical encoding, golden
  vectors, verifier code, or Deadbolt, stop and report why rather than
  proceeding.

## Files changed

Expected files:

- `docs/adr/0002-governed-claim-memory.md`
- `docs/design/adr-0002-governed-claim-memory-dossier.html`
- `tickets/0007-adr-0002-governed-claim-memory.md`

## Acceptance criteria

- ADR markdown exists and is internally consistent.
- HTML dossier exists and is static/mobile-clean.
- Ticket exists.
- No Rust source files changed.
- `docs/FORMAT.md` unchanged.
- Golden vectors unchanged.
- Existing event tags 0-5 unchanged.
- ADR states formal theories are ancestry, not machinery.
- ADR lists future enforcement tests.
- ADR states scope refs are opaque strings in v1.
- ADR states `SegmentAnchored` is occurrence evidence, not interpretation
  truth.
- ADR states future tags 6-8 are proposed but not added by this change.
- ADR states adding tags later requires `docs/FORMAT.md`, canonical encoding,
  golden vectors, verifier support, and explicit projection match arms.

## Tests to run

Docs-only minimum:

- `git diff --stat`
- `git diff --name-only`
- `git diff --check`

Optional if available in the repo:

- markdown lint
- link check

Do not run or change cargo tests for this docs-only ticket unless the reviewer
explicitly asks. If tests are run and fail for unrelated reasons, report that
clearly and do not fix unrelated failures.

## Reviewer checklist

- Confirm only the three expected documentation/ticket files changed.
- Confirm no Rust files changed.
- Confirm `docs/FORMAT.md` is unchanged.
- Confirm `crates/magpie-log/testdata/golden-v1.jsonl` is unchanged.
- Confirm `tools/verify_chain.py` is unchanged.
- Confirm Deadbolt was not modified.
- Confirm actor classes are exactly:
  `HumanRoot`, `AgentProposer`, `AutomatedVerifier`, `DeadboltAnchorer`,
  `LensWitness`, `SourceImporter`.
- Confirm evidence ceilings contain no numeric confidence.
- Confirm edge vocabulary is limited to:
  `supports`, `derived_from`, `contradicts`, `supersedes`, `invalidates`,
  `ratifies`.
- Confirm rejected vague edges are listed.
- Confirm v0 event compatibility is explicit.
- Confirm `SegmentAnchored` is occurrence evidence, not interpretation truth.

## Follow-up tickets

1. Implement `StandingView` as a derived projection over existing v0 events only.
2. Add the ADR-0002 enforcement tests against the v0-compatible `StandingView`.
3. Implement additive tags 6-8 with `docs/FORMAT.md`, canonical encoding,
   golden vectors, independent verifier support, and exhaustive projection
   match arms.
4. Design `EpistemicGate` / governed write admission for typed claim, evidence,
   and justification events.
5. Define any future lens evidence integration under the `LensReadout` ceiling.
6. Revisit ADR-0002 before adding scope inheritance, probabilistic fusion,
   credulous acceptance, or multiple competing accepted standings.

Reviewer note: tags 6-8 are intentionally behind `StandingView` and enforcement
tests so new claim-writing vocabulary cannot enter L0 before the projection law
exists.
