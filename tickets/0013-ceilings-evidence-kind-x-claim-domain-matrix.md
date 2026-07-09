# Ticket 0013: Evidence ceilings as an evidence_kind × claim_domain matrix

Superseded doctrine note: ticket 0019 corrects the HumanRatification row.
Humans settle nothing epistemically; HumanRatification is capped at `Supported`
and cannot settle `HumanJudgment` or scoped `Interpretation`.

## Goal

Amend `docs/design/standing-view-evidence-ceilings.md` so the ceiling law is
explicitly two-dimensional — `ceiling = f(evidence_kind, claim_domain)` — and
so the set of claim domains is a **closed vocabulary**, not open prose.

Today the doc has a one-dimensional evidence ceiling table (per evidence kind)
plus a separate "Claim-domain distinction" section whose "Domain constraints"
bullets already encode a 2-D relationship informally. This ticket promotes that
informal relationship into a single normative matrix and closes the domain set.
It also records the standing model's thesis line.

This is a docs-only rule-freezing ticket. No promotion, settlement, gate, or
writer semantics are implemented.

## Why

The same evidence kind carries different authority in different domains:
`DeterministicVerification` may settle `ExactMachineCheckable` but must not
settle `Interpretation`; `DeadboltAnchor` may settle `Occurrence/Inclusion` but
only support `Interpretation`. A single per-kind ceiling cannot express that; a
matrix can. And an open-ended domain vocabulary reintroduces exactly the
relation-slop the ADR rejected for edge kinds — so the domain set must be closed
the way `actor_class`, `evidence_kind`, and `edge_kind` already are.

## Scope

- Edit `docs/design/standing-view-evidence-ceilings.md` only.
- Add a normative **evidence_kind × claim_domain ceiling matrix**:
  - Rows: the 8 evidence kinds already in the ceiling table.
  - Columns: the claim domains already listed under "Claim-domain distinction":
    `Occurrence/Inclusion`, `ExactMachineCheckable`, `OperationalObservation`,
    `Interpretation`, `ExternalReport`, `ModelIntrospection`, `HumanJudgment`.
  - Cell values are drawn from a fixed set only: `Settled`, `Supported`,
    `Conjectured`, or `—` (no admissible contribution).
  - **Every cell must be justified by an existing bullet** in the current
    "Domain constraints" or per-kind sections. Where the existing prose does not
    already decide a cell, fill it with `TBD-arch` and leave a one-line note —
    do NOT invent a new ceiling. The architect resolves `TBD-arch` cells on
    review; that judgement is not delegated.
- Declare the 7-domain set a **closed v1 vocabulary**: add an explicit statement
  that these are the only admissible claim domains in v1, that adding one is an
  amendment to this doc (and later to the metadata convention and gate), and
  that it mirrors the closed `actor_class`/`evidence_kind`/`edge_kind` sets.
- Keep the existing per-kind sections and the 1-D table; the matrix is the
  authoritative cross-reference, and the prose sections stay as rationale. Add
  one sentence pointing the reader from the 1-D table to the matrix.
- Add the standing model's thesis as a normative framing line near the top
  (Core rule / Purpose): **"A belief is a replay result, not a string in
  memory."** State plainly that standing is never stored as an authoritative
  mutable field; it is always the deterministic result of replaying L0.
- Add a short forward-reference sentence: where `claim_domain` is carried on an
  event is defined by the metadata-conventions doc (ticket 0014), not here.

## Non-goals / Constraints

- No Rust changes. No `Payload` changes. No canonical encoding changes.
- No `docs/FORMAT.md` changes. No golden vector changes. No Python verifier
  changes. No `StandingView` implementation changes.
- Do NOT mutate tags 6-8 or add a `claim_domain` L0 field. Where the field
  lives is 0014's decision (metadata for v1), and even that is docs-only.
- No promotion, settlement, contradiction debt, invalidation, supersession,
  ratification, `EpistemicGate`, or writer/CLI/MCP surfaces.
- No probabilistic or numeric confidence. Cell values are the ordinal juridical
  set only. Scores are never standing.
- No new claim domains beyond the 7 already listed. If one seems needed, stop
  and flag it rather than adding it.
- Preserve the `SegmentAnchored` occurrence/inclusion boundary unchanged.

## Tests

- None (docs-only). No code compiles or runs for this ticket.
- Report (do not run as a hard precondition on Windows): a human should confirm
  `cargo test -p magpie-log` is unaffected, since no code is touched.

## Acceptance criteria

- The doc contains one matrix with 8 evidence-kind rows and 7 claim-domain
  columns; every cell is one of `Settled` / `Supported` / `Conjectured` / `—`
  / `TBD-arch`.
- No cell contradicts the existing per-kind ceiling (e.g. `ModelSelfReport` is
  never above `Conjectured` in any column; `ExternalSource` never `Settled`).
- The 7 claim domains are stated as a closed v1 vocabulary with an amendment
  rule.
- The belief-is-a-replay-result line appears once as a normative framing
  statement.
- The 1-D table and per-kind sections remain; nothing is deleted, only
  cross-referenced.
- Diff touches exactly one file: `docs/design/standing-view-evidence-ceilings.md`
  (plus this ticket).

## Suggested reviewer checks

- Read each `TBD-arch` cell and decide it by hand; these are the real design
  calls (e.g. may `BehavioralEvaluation` reach `Supported` in `Interpretation`,
  or only `OperationalObservation`?).
- Confirm the matrix and the surviving 1-D table cannot drift: the 1-D table's
  max per row must equal the max across that row in the matrix.

## Review adjustments incorporated (2026-07-07)

Pre-PR review clarifications added to the doc (still docs-only, no new tags, no
`claim_domain` L0 field):

1. `claim_domain` is closed by future `StandingView` / `EpistemicGate` policy and
   is not currently enforced by L0 payload validation; this PR does not mutate
   tags 6-8 or add a `claim_domain` field to `ClaimAssertedV2`.
2. New section "Domain assignment is admitted policy material": domain is an
   assertion-time/gate-time classification, never inferred from confident prose;
   ambiguous/absent/conflicting classification takes the weaker applicable ceiling
   or refuses promotion; a broad `Interpretation` claim is not reclassified as
   `ExactMachineCheckable` because it contains a checkable subclaim.
3. `HumanRatification` may support `HumanJudgment`, scoped `Interpretation`,
   and bounded `OperationalObservation` only; it does not settle any domain by
   override.
4. `ExecutionEvidence` × `Interpretation` = no direct contribution; interpretive
   bearing travels through claims and edges.
5. `DeterministicVerification` × `OperationalObservation` = `Supported`, with a
   classification rule separating it from `ExactMachineCheckable`.
6. `ModelSelfReport` / `LensReadout` are hypothesis seeds, not support.
7. Future edge-targeting convention: namespaced `claim:` / `evidence:` / `edge:`
   refs over opaque strings (not a format change); objections are `invalidates`
   or `contradicts` edges targeting a support edge.
8. `BeliefChangeLedger` recorded as a derived replay trace, not a second store.
9. Thesis line preserved: a belief is a replay result, not a string in memory.
