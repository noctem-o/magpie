# Ticket 0019: Standing doctrine correction — humans settle nothing epistemically

## Purpose

Correct ADR-0002 standing doctrine so human ratification is governance/judgment
evidence, not epistemic settlement authority.

The corrected law is:

```text
Humans settle nothing epistemically.
```

Human authority is real, but it lives on the governance/consent axis rather
than the truth/standing axis. A human can approve a risky action; they cannot
make a hash match by decree.

## Scope

- Update `docs/design/standing-view-evidence-ceilings.md`.
- Update `docs/adr/0002-governed-claim-memory.md`.
- Supersede stale HumanRatification settlement wording in the ADR-0002 tags
  implementation plan and dossier.
- Supersede stale HumanRatification settlement wording in:
  - `tickets/0012-standing-view-evidence-ceilings.md`
  - `tickets/0013-ceilings-evidence-kind-x-claim-domain-matrix.md`
- Add this ticket.

## Doctrine correction

- `HumanRatification` may contribute at most `Supported`.
- `HumanRatification` must not yield `Settled` for `HumanJudgment`,
  `Interpretation`, or any other claim domain.
- `Settled` is reserved for deterministic verification, cryptographic
  verification, exact replay equivalence, or exact occurrence/inclusion evidence
  admitted by explicit policy.
- Human ratification may support the fact "a human ratified X" while not
  settling "X is true."
- Later policy may settle the occurrence of ratification if it is logged,
  signed, and verified, but not the truth of the ratified statement.
- `ceiling != achieved standing`: a ceiling is the maximum contribution a source
  may make after all required checks, not automatic promotion.

## Non-goals

- No Rust changes.
- No L0 payload definition changes.
- No payload variants.
- No canonical encoding changes.
- No `docs/FORMAT.md` changes.
- No golden fixture changes.
- No `tools/verify_chain.py` changes.
- No writer-facing surfaces.
- No MCP write paths.
- No `EpistemicGate`.
- No standing policy implementation.
- No `support_ceiling`, `refutation_ceiling`, aggregation, contradiction debt,
  invalidation, supersession, or ratification semantics in code.
- No Deadbolt changes.
- No model integration.
- No librarian/navigator functionality.
- No network sync, remote services, or always-on behavior.
- No production trust, certification, attestation, or live-readiness claim.
- No social/political deployment layer.

## Changed files

- `docs/adr/0002-governed-claim-memory.md`
- `docs/design/adr-0002-governed-claim-memory-dossier.html`
- `docs/design/adr-0002-tags-6-8-implementation-plan.md`
- `docs/design/standing-view-evidence-ceilings.md`
- `tickets/0012-standing-view-evidence-ceilings.md`
- `tickets/0013-ceilings-evidence-kind-x-claim-domain-matrix.md`
- `tickets/0019-standing-doctrine-humans-settle-nothing.md`

## Validation performed

- `git diff --check` passed.
- `cargo test --workspace --locked` passed. No Rust changed; this was run only
  as a regression check for docs-only work.
- `git diff --name-only` was run. Because this new ticket file is untracked
  until staging, `git status --short` was also used to confirm the full changed
  file set.
- `rg -n "HumanRatification|Settled|HumanJudgment|humans settle|settle nothing|ceiling != achieved" docs README.md tickets crates`
  was run as the broad sanity scan.
- A targeted stale-doctrine scan found no remaining old wording that says
  `HumanRatification` may reach `Settled` or settle `HumanJudgment` /
  `Interpretation`.

## Follow-up phases

- Policy enums and exhaustive support ceilings.
- Refutation ceiling.
- Aggregation / independence groups.
- Contradiction debt.
- Invalidation / supersession.
- `EpistemicGate`.
- Optional librarian/navigator later.

## Suggested reviewer checks

- Confirm no docs say HumanRatification may yield `Settled`.
- Confirm no docs imply HumanRoot can settle interpretation or human-judgment
  truth by decree.
- Confirm `Settled` remains available only for deterministic, cryptographic,
  exact replay, or exact occurrence/inclusion facts admitted by policy.
- Confirm this phase does not implement fold behavior or writer surfaces.
