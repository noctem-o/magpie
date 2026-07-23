# Ticket 0055: Post-v3 architecture ledger reconciliation

## 1. Exact base and authority

```text
base:
7720a2749f3b85f95e38580d0af4fd26c6f02658
```

That base is the merged PR #68 merge commit. Its parents are
`757b76b9cc19a6dbdfbbc6af6940127d239c656e` (main at the PR #66 merge) and
`f030be8ff329c93fd7138d6f831bb44cc42e9409` (the reviewed Ticket 0054
implementation head); its tree is identical to the implementation head's
tree.

Authority for this reconciliation:

```text
merged PR #56 (merge commit 351333ac17cb1d7b82d0ee144568d48a7406f80a)
merged PR #57 (merge commit c12ef0756a2984c99329679d8630ddec5bc4bc39)
merged PR #68 (head f030be8ff329c93fd7138d6f831bb44cc42e9409,
               merge commit 7720a2749f3b85f95e38580d0af4fd26c6f02658)
Ticket 0043 (historical review remediation contract)
Ticket 0054 (standing policy v3 corroboration implementation)
the living README and design notes at the exact base
```

The user retains sole commit, push, publication, and merge authority. This
ticket designs, implements, or implies no new epistemic policy.

## 2. Classification

```text
correctness:
current-status and architecture-ledger reconciliation only

runtime:
unchanged

standing:
unchanged

policy:
unchanged

canonical bytes:
unchanged

L0:
unchanged

dependencies:
unchanged

fixtures:
unchanged

tests:
unchanged

authority:
unchanged

positive effect:
current-status documents accurately distinguish the completed narrow v3
corroboration rule, the completed PR #56 regression coverage, the partially
completed PR #57 optimisation, and still-future epistemic work
```

## 3. Exact changed-path allowlist

Only:

```text
README.md
docs/design/historical-review-remediation-ledger.md
docs/design/standing-aggregation-independence-groups.md
tickets/0054-standing-policy-v3-external-report-corroboration-implementation.md
tickets/0055-post-v3-architecture-ledger-reconciliation.md
```

No Rust source, test, fixture, Cargo manifest or lockfile, CI, release
metadata, changelog, `docs/FORMAT.md`, `AGENTS.md`, historical ticket
0001-0053, the Ticket 0053 contract, the v3 normative design contract,
generated file, Python cache, or GitHub workflow path may change.

## 4. Historical-boundary law

```text
historical phase scope
!= current repository status

updating a living current-status ledger
!= retroactively changing what an earlier ticket implemented
```

A sentence such as "this ticket implements no aggregation" can remain correct
inside the historical ticket that ratified only a lower layer. A sentence
such as "standing policy v3 remains future work" is stale when it appears in
a living current-status surface after Ticket 0054 has merged. Reconciliation
time-bounds historical wording, adds current-status follow-ups, or replaces
unbounded present-tense status claims with exact current claims. It does not
erase historical sequencing.

## 5. Reconciliation table

| Surface | Previous stale or ambiguous claim | Exact reconciled status |
| --- | --- | --- |
| Standing policy v3 / conservative aggregation | "standing policy v3 remains future work" (unbounded, living surfaces) | One narrow rule implemented by Ticket 0054 and merged via PR #68: complete internally derived `SupportContributionAuditV0` + exact requested `ExternalReport` claim + one exact `OriginComparisonNamespaceV0` lane + at least two distinct admitted origin groups → governed `Supported`, never `Settled`. Broader aggregation remains future. |
| PR #56 regression coverage | "a dedicated follow-up test PR must pin both cases" (remediation ledger §5.1, aggregation note substrate) | Implemented by PR #56 (merge commit `351333ac17cb1d7b82d0ee144568d48a7406f80a`): `legacy_raw_refuted_remains_audit_visible_and_does_not_veto_matched_v2_support` and `inherited_governed_refuted_overrides_matched_deterministic_support`. No production standing behavior changed. |
| PR #57 verification memory | "verification-only callers currently reuse a path that retains the complete parsed SignedEvent vector" (living ledger §5.2) | Partially resolved by PR #57 (merge commit `c12ef0756a2984c99329679d8630ddec5bc4bc39`): verification-only chain verification and writer recovery retain only count and tip; replay retains one completely verified snapshot; one-read and verify-before-apply laws unchanged. `LogStore::read_records` raw-record materialisation remains future interface work. |
| Ticket 0054 publication state | "the implementation remains uncommitted and unpushed" (Ticket 0054 §16) | Pre-publication record preserved as historical; the reviewed candidate was committed by the user as `f030be8ff329c93fd7138d6f831bb44cc42e9409` and merged via PR #68 at `7720a2749f3b85f95e38580d0af4fd26c6f02658` on 2026-07-23. |
| Ticket 0054 checker closure | "closure requires an entirely fresh Checker B" (Ticket 0054 §14 tail, true when written) | The first Checker B `FAIL` became stale after remediation; an entirely fresh amended Checker B reviewed the final candidate and returned `PASS`; Git history records no further change before publication beyond the user's own commit. |
| Remaining future work | various unbounded "future" lists | Direct refutation, contradiction debt, invalidation, supersession/currentness, `EpistemicGate`, ordinary claim-bearing writers, loaders, CAS, filesystem or network ingestion, and librarian integration remain future. |

## 6. Runtime inertia

This ticket changes no:

- Rust source, test, or fixture;
- canonical byte, golden vector, or `docs/FORMAT.md` surface;
- policy identifier, policy matrix, or standing behavior;
- L0 payload, store, writer, or replay behavior;
- dependency manifest or lockfile;
- loader, storage, network, or model surface; or
- authority boundary of any kind.

## 7. Validation actually run

Observed on the candidate diff; nothing here is pre-claimed.

| Command | Observed result |
| --- | --- |
| `git diff --check` | pass |
| `cargo fmt --all --check` | pass |
| `cargo test --workspace --locked` | pass; all workspace targets green, zero failures. Run with `RUSTC_LINKER`, `LIB`, and `INCLUDE` pointed at the complete VS 2022 BuildTools MSVC toolset: the default VS 18 BuildTools installation lacks `msvcrt.lib` and cannot link (`LNK1104`), an environment limitation, not a code diagnostic |
| `cargo test --doc --locked` | pass; 120 `magpie-claims` plus 3 `magpie-log`, zero failures |
| `cargo run --locked --example tour -p magpie-claims` | pass; 1917 bytes, sha256 `48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f`, byte-identical to the frozen Ticket 0054 capture |
| `python tools/check_release_metadata.py` (direct) | expected refusal: Cargo's package inventory rejects the intentionally dirty `README.md`. Not a repository defect; `--allow-dirty` not used; no commit made to satisfy it |
| `python tools/check_release_metadata.py` (VCS-free mirror of the exact candidate) | pass; package inventories 18/39/8, release metadata consistent; mirror deleted afterwards |
| pre-edit and post-edit stale-phrase scans | every remaining match manually classified as a correct remaining-future claim or a historically scoped record; the post-edit scan found and fixed one additional stale claim (README entry 9's unbounded aggregation-phasing) |
| focused future-work vocabulary audit | every match verified as a correct future boundary or doctrine statement |
| `git diff --name-only` / `git status --short` | exactly the five authorised paths: four modified, one new |

The cargo ladder and the release-metadata checks ran before the section 7/8
status records were appended to this ticket. That amendment is
markdown-only, so Rust, doc-test, manifest, lockfile, and package-inventory
inputs are byte-identical across it. `git diff --check`,
`cargo fmt --all --check`, the phrase scans, and the VCS-free mirror
release-metadata check were rerun after the final amendment and remained
green.

## 8. Checker adjudication

Checker A ran before editing: `gpt-5.6-sol`, xHigh reasoning, entirely fresh
context, read-only, hostile status-reconciliation audit of the exact base.
Verdict: `PROCEED`. K3 independently verified its findings against the
repository and adjudicated as follows:

- accepted every stale living-status finding within the allowlist;
- overrode its recommendation to leave Ticket 0054 §12 unchanged: the
  mandated time-bounding to "final pre-publication candidate" preserves the
  same provenance and satisfies this ticket's authority;
- accepted its caution that no checker verdict may be inferred from Git: the
  amended Checker B `PASS` recorded in Ticket 0054 is evidenced by that
  checker's verbatim fresh-context closure transcript, while Git supplies
  only the publication record (user commit `f030be8`, PR #68 merge
  `7720a274`, identical tree);
- accepted its negative-capability sweep: no direct-refutation,
  contradiction-debt, invalidation, supersession/currentness policy,
  `EpistemicGate`, ordinary writer, loader, CAS, network, or librarian
  implementation exists.

Checker B (first, fresh) ran after the candidate diff and validation:
`gpt-5.6-sol`, xHigh reasoning, entirely fresh context, read-only hostile
final review against the twelve falsification targets. Verdict: `PASS`, no
findings. K3 independently inspected the complete diff against the same
targets before launching it and again after its verdict.

Recording this section and section 7 amends the diff Checker B reviewed. Per
the stale-PASS rule, an entirely fresh closure review of the final amended
diff follows; its verdict is recorded in the K3 final report and the Epic
decision log rather than in this ticket, avoiding an unbounded record-review
regress.

## 9. Residual stale-language findings outside the allowlist

Found during reconciliation and deliberately not edited (outside the
allowlist); candidates for a later reconciliation:

- `docs/design/standing-policy-v3-external-report-corroboration-v0.md`
  status block: "runtime: not implemented" is stale after Ticket 0054. The
  normative contract is a prohibited path under this ticket; any correction
  needs its own authorised scope.
- `docs/design/artifact-provenance-origin-admission.md:1032`: the checklist
  item "Confirm no aggregation threshold or implemented policy v3 appears"
  predates Ticket 0054. As scoped to that architecture layer it remains
  arguably true; whether it needs rewording is left to a later
  reconciliation. Previously flagged out of scope during the Ticket 0053
  phase as well.
- Historical tickets 0043, 0044, and 0053 contain present-tense debt and
  sequencing language that was accurate at authoring time. They are frozen
  phase records, preserved deliberately; living surfaces cross-reference
  later outcomes instead.

## 10. Future work

The exact remaining sequence is unchanged and is not ratified here:

1. direct-refutation contract;
2. direct-refutation runtime;
3. contradiction debt and precedence;
4. invalidation;
5. supersession/currentness;
6. `EpistemicGate`;
7. ordinary claim-bearing writer surfaces;
8. governed acquisition/loading;
9. librarian integration.

This ticket ratifies none of those and designs none of them.

## 11. Stop conditions

Stop rather than proceeding when reconciliation appears to require: Rust,
test, fixture, Cargo, or dependency changes; a new policy identifier or
standing rule; direct-refutation, contradiction-debt, invalidation,
supersession, `EpistemicGate`, writer, loader, or storage design; editing an
old frozen contract or any path outside the allowlist; claiming PR #57
removed raw-record materialisation; claiming v3 proves source independence;
claiming v3 produces `Settled`; rewriting historical tickets to pretend
later work existed earlier; or guessing an unverified PR or commit fact.

## 12. Suggested reviewer checklist

1. README entries 16-25 no longer contradict entry 25.
2. Ticket 0054 preserves its chronology while recording final publication.
3. PR #56 is recorded as implemented without runtime overclaim.
4. PR #57 is recorded as partial, not complete.
5. The aggregation design note distinguishes the implemented first rule
   from broader future aggregation.
6. No source-independence claim appears.
7. `ExternalReport` corroboration remains never-`Settled`.
8. Direct refutation and later phases remain future.
9. Only the five authorised paths changed.
10. No historical ticket was rewritten.
