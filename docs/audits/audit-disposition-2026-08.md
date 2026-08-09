# August 2026 audit finding disposition

## 1. Status

This document is a living administrative index of the findings recorded in the
two preserved audit ledgers. It does not amend either historical audit, and it
must not be read as an ADR, a governing contract, a doctrine ratification, an
implementation authorization, or a release decision. It authorizes no feature
work.

Its statements apply only to the current-main baseline recorded below. The
historical ledgers remain immutable evidence interpreted at their own audit
commit:

- [architecture audit](magpie-architecture-audit-2026-08-01.md);
- [runtime-quality audit](magpie-runtime-quality-audit-2026-08-04.md).

## 2. Baseline

| Field | Recorded value |
| --- | --- |
| Repository | `noctem-o/magpie`, isolated checkout `C:\Users\herpe\.codex\worktrees\magpie-reconcile-a009-post-118` |
| Disposition date | 2026-08-08 |
| Branch and worktree mode | `docs/reconcile-audit-disposition-post-118`; fresh isolated worktree `C:\Users\herpe\.codex\worktrees\magpie-reconcile-a009-post-118`; clean before editing |
| Current `main` | `c70c0fb28157212f0f73cef88bae116658c787f7` |
| Local `origin/main` | `c70c0fb28157212f0f73cef88bae116658c787f7` |
| Live `origin/main` check | `git ls-remote origin refs/heads/main` returned the same SHA |
| Audit commit | `3b46fdb81857d77b827271777b86745bca5f7b12` |
| Local HEAD / `origin/main` merge base | `c70c0fb28157212f0f73cef88bae116658c787f7` before this ledger edit |
| Audit commit / current-main merge base | `3b46fdb81857d77b827271777b86745bca5f7b12` |
| Expected post-PR-118 SHA | Equal to the verified current `main`; difference from `c70c0fb28157212f0f73cef88bae116658c787f7` is none |
| Relevant merged changes | PR #108, merge `71d096ff3c7ca8107d51484f486c30031d337ee8`, Accepted ADR-0007; PR #109, merge `b79a96ab0ddc5c8b9008231ef53e325efdd067d3`, origin-contract status and compatibility reconciliation; PR #110, merge `a1a59200c5a1af2c81afe17b76b89609134d5787`, downstream claimant-label compatibility quarantine; PR #111, merge `dc07034d4af7d2b41130ba04a6a9d4c5af000995`, standing runtime/test correction; PR #112, merge `91ebb5d6a4f5ba99ad377c77eb96dc70bc829245`, episodic projection/schema/test correction; PR #113, merge `253fc72350ae5232fc428da9ab5657f8ccf5c912`, administrative disposition reconciliation; PR #114, merge `142793a47c0f5a7e26ec29fc100d2d6cde9ddb3a`, documentation-only sole-writer implementation contract; PR #115, merge `66198af8dc06b086e77036fafd5e18adcdd967b2`, A-001/RQ-001 runtime/API remediation; PR #116, merge `c2fe991d8023d237f7d589bd3f022233c1f332cc`, documentation-only A-001/RQ-001 administrative closure; PR #117, merge `99512b5eddd39ecc98e31e59173252da717f2fa2`, documentation-only A-009/RQ-002 implementation contract; PR #118, merge `c70c0fb28157212f0f73cef88bae116658c787f7`, A-009/RQ-002 runtime/API/test remediation |
| Evidence-preservation change | PR #104, merge `7889150b100795409bfe0d6f676c5780cc7b2830`; documentation-only audit preservation |
| Open remediation PRs | None at disposition time |
| Open issues | None at disposition time |
| Preflight changed paths | None in the isolated worktree; the ordinary `C:\magpie` checkout's unrelated untracked `.codex/config.toml` was left untouched |
| Runtime change since audit | PR #111 changed standing-v2/v3 policy combination code and tests without changing currently representable results. PR #112 intentionally changed derived episodic state and canonical projection bytes for `ClaimAssertedV2`, bumped episodic schema/projection version 2 to 3, and added rebuild tests. PR #115 made public `LogStore` read-only, moved raw persistence behind crate-private `WriterStore`, closed writer construction to `FileStore` and `MemStore`, and added direct negative and preservation tests without changing FORMAT or golden bytes. PR #118 separated raw `SignedEvent` from projection-compatible `VerifiedReplayEvent`, restricted wrapper minting to completed one-snapshot replay, renamed raw inspection to `unverified_events`, and added hostile type/lifetime and replay-preservation tests without changing L0 bytes. PRs #108-#110, #113, #114, #116, and #117 were documentation-only. |

The runtime audit was performed on local `main` at the audit commit while the
then-current remote was nine documentation-only commits ahead. PR #103 later
corrected and clarified doctrine/status text, and PR #104 preserved the audit
evidence and added its historical index. PRs #108-#110 then accepted the
authority-bound successor doctrine and reconciled current claimant-label
compatibility semantics without changing runtime. PR #111 removed the latent
v2/v3 wildcard-promotion defect without changing any currently representable
policy result. PR #112 intentionally corrected the episodic projection of
statusless `ClaimAssertedV2` and invalidated stale schema-v2 derived state. PR
#113 reconciled those merged outcomes administratively. PR #114 then froze the
sole-writer implementation contract without changing runtime, and PR #115
sealed raw backend persistence behind `LogWriter` without changing L0 bytes.
PR #116 reconciled that closure administratively. PR #117 then froze the
verified-replay projection-input contract without changing runtime, and PR
#118 implemented the wrapper-only public projection boundary and hostile proof
suite without changing L0 bytes. The dispositions below distinguish those
runtime closures from documentation narrowing and from doctrine that remains
unimplemented.

The complete post-audit ancestry was inspected. The relevant post-ledger
tranche is ordered as merge `71d096f` (#108), `b79a96a` (#109), `a1a5920`
(#110), `dc07034` (#111), `91ebb5d` (#112), `253fc72` (#113), `142793a`
(#114), `66198af` (#115), `c2fe991` (#116), `99512b5` (#117), and `c70c0fb`
(#118); every merge is an ancestor of the next and of current `origin/main`.

PR #103 changed `README.md`, ADR-0002 through ADR-0006, and two design notes.
It made ADR-0002 through ADR-0006 Accepted doctrine effective 2026-08-02,
separated standing from currentness, and retired a former relationship-edge
invalidation convention. PR #104 changed only the audit index and the two
preserved audit records. PRs #108-#110 are the documentation/doctrine/status
portion of the new tranche; #111 is the standing runtime/test correction; and
#112 is the episodic projection/schema/test correction. PR #113 changed only
this living administrative ledger. PR #114 changed only the sole-writer design
contract and Ticket 0068; it did not remediate runtime. PR #115 is the
runtime/API capability correction: public `LogStore` is read-only, raw append
is crate-private, custom writer backends are intentionally unsupported, and
the existing canonical format and fixtures are unchanged. PR #116 changed only
this living administrative ledger. PR #117 changed only the verified-replay
design contract and Ticket 0070; it did not remediate runtime. PR #118 is the
runtime/API/test correction: projection input is an ephemeral replay-created
wrapper, raw inspection is explicitly named `unverified_events`, and FORMAT,
event representation, canonical encoding, and fixtures remain unchanged.

## 3. Evidence hierarchy

Current dispositions were made in this order:

1. current implementation and tests;
2. accepted normative format and ADRs;
3. current ratified narrow contracts;
4. merged change history since the audit;
5. the historical audits as finding records and leads.

This ledger is not added to that hierarchy as a governing source. It records
an administrative comparison against the baseline; it cannot ratify doctrine,
reclassify an accepted contract, or authorize implementation.

## 4. Disposition vocabulary

The primary disposition is controlled and has exactly one of these meanings:

| Primary disposition | Meaning |
| --- | --- |
| **Confirmed** | The finding remains materially present on current `main`. |
| **Partially remediated** | A current change corrected a material part, but the finding is not closed. |
| **Superseded** | Later doctrine or architecture replaced the premise, and no equivalent defect remains under the new governing model. |
| **Accepted debt** | The finding remains, but current governing material explicitly accepts it for the present scope. |
| **Future implementation gate** | No current runtime defect is claimed, but the issue must be resolved before a named future capability may be implemented. |
| **Scheduled** | A current issue or PR owns the remediation; the issue or PR must be identified. |
| **Closed** | Current code, doctrine, tests, and interfaces provide objective closure evidence for the entire finding. |

Qualifiers such as `contained`, `conditional`, `latent`, `narrowed`,
`compatibility-only`, and `documentation-only remainder` describe scope. They
never replace the primary disposition.

## 5. Executive disposition

There are 24 architecture findings, 15 runtime-quality findings, and 39
unique finding IDs. The individual IDs are covered mechanically in section 7.

| Primary disposition | Finding IDs |
| --- | ---: |
| Confirmed | 26 |
| Partially remediated | 4 |
| Superseded | 0 |
| Accepted debt | 0 |
| Future implementation gate | 2 |
| Scheduled | 0 |
| Closed | 7 |
| **Total** | **39** |

The principal convergence families are: sealed write capability; the closed
raw-versus-verified replay projection boundary and residual caller-controlled
Deadbolt occurrence eligibility; status/precedence and currentness doctrine;
cross-language grammar;
closed wildcard-status evolution; closed synthetic projection status;
projection failure and storage resource law; compatibility/raw standing;
detached provenance coordinates;
authority/corroboration; admission and identity gates; and test/release
assurance. Similar names were not enough to merge distinct surfaces; the
family boundaries and correction loci are retained in section 6.

The surviving present constitutional defects are the residual A-002
status/precedence boundary and A-024. A-010 and A-022 retain
constitutional future-gate significance: each prohibits premature
implementation, but neither presently describes an implemented runtime that
contradicts a completed deferred mechanism. High or P0 boundaries also remain
in A-003, A-004, A-009, A-015, A-021, and RQ-006, with RQ-003
retaining its authority-looking compatibility leak. A-005/RQ-005 and
A-006/RQ-004 remain objectively Closed by their merged runtime and test
evidence. A-001/RQ-001 is now also objectively Closed by the merged contract,
runtime/API change, negative capability proofs, and preservation evidence
recorded below. RQ-002 is objectively Closed by the merged wrapper-only
runtime/API boundary, hostile type/lifetime proofs, one-snapshot
failure-ordering tests, independent verification, and owner merge. A-009 is
Partially remediated: the same change closes its raw-event projection bypass,
but its separate caller-controlled Deadbolt occurrence eligibility input
remains.

The primary **Future implementation gate** disposition applies only to A-016
and A-022. A-010 additionally carries a future currentness-implementation
gate while remaining **Partially remediated**. Confirmed findings
A-019/RQ-011 and A-024 also prohibit specific future authority-bearing uses
until their present boundaries are corrected. No finding is Scheduled because
no current issue or PR owns a remediation.

## 6. Converged findings ledger

Each row is a current convergence family, not a new finding. Where related IDs
remain distinct in severity, conditions, or layer, those differences are stated
in the row.

| Audit IDs | Current finding family | Primary disposition | Qualifier | Current-main evidence | Change since audit | Next correction locus |
| --- | --- | --- | --- | --- | --- | --- |
| A-001 / RQ-001 | Public raw backend append is sealed behind `LogWriter`. | Closed | runtime/API remediation; intentional pre-1.0 custom-writer source break | Public `LogStore` contains only `read_records`; crate-private `WriterStore` owns `append_record`, is not re-exported, and is implemented by `FileStore` and `MemStore` (`crates/magpie-log/src/store.rs:12-20,36-60,87-98`; `crates/magpie-log/src/lib.rs:91-105`). Public typed construction and append exist only for `LogWriter<FileStore>` and `LogWriter<MemStore>` (`crates/magpie-log/src/logimpl.rs:311-464`); `LogReader` retains a private backend and exposes no persistence escape (`:466-585`). Seven compile-fail cases deny built-in/generic raw append, reader append/extraction, custom writer construction, and bare-key persistence (`crates/magpie-log/src/lib.rs:16-89`). Runtime tests preserve genesis, recovery, failure-state ordering, and golden bytes (`crates/magpie-log/tests/chain.rs:126-151,559-606,790-819`; `crates/magpie-log/src/logimpl.rs:747-825`; `crates/magpie-log/tests/golden.rs:219-318`). | PR #114, merge `142793a47c0f5a7e26ec29fc100d2d6cde9ddb3a`, established the reviewed Option B contract. PR #115, merge `66198af8dc06b086e77036fafd5e18adcdd967b2`, implemented it without a FORMAT or golden-byte change. | Closed; reopen only if a public non-`LogWriter` persistence path is introduced. |
| A-002 / RQ-014 | Current-status and precedence drift remains outside the portions reconciled by PRs #103, #109, and #110. | Partially remediated | constitutional remainder; further narrowed; documentation-only remainder | The standing-v3 and origin-admission headers now identify their landed runtime accurately (`docs/design/standing-policy-v3-external-report-corroboration-v0.md:3-11`; `docs/design/origin-admission-audit-v0.md:3-12`). Current stale handoffs remain: the attestation and subject-binding notes still call landed runtime a `current candidate patch` (`docs/design/inline-predicate-attestation-binding-v0.md:3-10,980-1009`; `docs/design/claim-inline-subject-binding-v0.md:593`), Tickets 0061/0063 retain uncommitted-candidate language, and the wider bootstrap/visual-ledger/status-registry ambiguity recorded by A-002 is unchanged. | PR #103 (`21067dc`) corrected the ADR stack and several living status claims. PR #109 (`b79a96a`) reconciled current origin-verification/admission handoffs; PR #110 (`a1a5920`) reconciled standing-v3 runtime/status wording. Neither changed the remaining status corpus. | status/precedence registry and narrow living-status handoffs |
| A-003 | Detached v0-v2 standing outputs still omit complete verified-prefix coordinates. | Confirmed | high; compatibility-only | `StandingResolution`, `StandingResolutionV1`, and `StandingResolutionV2` remain public coordinate-poor shapes in `crates/magpie-claims/src/standing.rs:76-97`, `standing_v1.rs:124`, and `standing_v2.rs:135`; newer v3/v4 paths carry private prefix/closure identities. | No runtime/API change. | provenance-response / compatibility API |
| A-004 / RQ-006 | Rust and Python still accept different JSONL record languages and first-failure behavior. | Confirmed | high; P0; interoperability | Rust uses `hex::decode` (`crates/magpie-log/src/hashing.rs:48-54`, `event.rs:315-325`) and skips blank lines (`store.rs:41-46`); Python has its own hex/status/line grammar (`tools/verify_chain.py:49-60,206-237`). | No verifier or format change. Existing golden vectors cover the common intersection only. | verifier/tooling contract |
| A-005 / RQ-005 | v2/v3 status combination now fails closed when the shared status vocabulary evolves. | Closed | behavior-preserving runtime remediation | `Status` has five current variants (`crates/magpie-log/src/event.rs:25-31`). `combine_v2_standing` and `combine_standing` explicitly select every one (`crates/magpie-claims/src/standing_v2.rs:300-314`; `standing_v3.rs:885-899`), so a future variant makes both matches non-exhaustive. Complete 12-case truth tables preserve every current result (`standing_v2.rs:803-830`; `standing_v3.rs:2268-2295`). | PR #111, merge `dc07034d4af7d2b41130ba04a6a9d4c5af000995`, removed both status wildcards and added permanent complete truth-table tests without changing current policy identity or semantics. | Closed; re-evaluate explicitly if `Status` evolves. |
| A-006 / RQ-004 | Episodic output now preserves `ClaimAssertedV2` as statusless. | Closed | intentional derived-projection correction; schema-versioned | `ClaimAssertedV2` maps to `claim_status: None`, while legacy `ClaimAsserted` and `ClaimStatusChanged` preserve exact signed status fields (`crates/magpie-episodic/src/lib.rs:271-366`). `SCHEMA_VERSION` is 3 (`:19`), and tests prove NULL canonical rows, exact legacy status retention, schema-v2 stale-row discard, caller-owned verified replay, fresh/rebuilt byte equality, and v3 reopen preservation (`crates/magpie-episodic/tests/episodic.rs:442-559`). | PR #112, merge `91ebb5d6a4f5ba99ad377c77eb96dc70bc829245`, changed derived episodic state and canonical bytes for tag 6, bumped schema/projection version 2 to 3, and added migration/replay regressions. | Closed; neighboring A-007/A-008/A-012 remain separate. |
| A-007 / RQ-009 | Projection reuse and fallible episodic operations still lack a typed lifecycle/error boundary. | Confirmed | current availability defect; contained to derived state | `Projection::apply` remains infallible (`crates/magpie-log/src/logimpl.rs:623-624`); episodic transaction, insert, commit, and signed-range paths still use `expect`/panic (`crates/magpie-episodic/src/lib.rs:220-258,400-408`). | PR #118 changed projection input provenance only; projection fallibility and episodic failure semantics remain unchanged. | storage/resource contract |
| A-008 / RQ-007 / RQ-008 | The storage seam still lacks a complete snapshot, durability, concurrency, framing, and outer resource law. | Confirmed | operational/resource debt; conditional under local use | `FileStore` writes record bytes and newline separately without sync or lock (`crates/magpie-log/src/store.rs:50-59`); `read_records` materializes the file and splits every line (`store.rs:36-47`); retained replay keeps the already-read record vector and parsed event vector in memory (`crates/magpie-log/src/logimpl.rs:198-210,294-307,540-565`). | PR #118 preserved one-snapshot replay and changed only the projection-input type-state; storage semantics and outer resource bounds remain unchanged. | storage/resource contract |
| A-009 | Verified replay provenance is structurally enforced, but public Deadbolt occurrence eligibility remains caller-selected. | Partially remediated | raw-event bypass closed; caller-controlled policy-input remainder; contained public mechanics seam | The closed portion is structural: raw `SignedEvent` cannot satisfy `Projection::apply`, and only replay-created `VerifiedReplayEvent` crosses that boundary (`crates/magpie-log/src/logimpl.rs:63-135,533-571,587-624`). The open portion is independent: public `resolve_deadbolt_occurrence_context` still accepts `EvidenceKind` and `ClaimDomain`, checks the eligible values, but does not prove their provenance; its trusted-use documentation requires caller co-derivation (`crates/magpie-claims/src/deadbolt_context.rs:181-206`). The private composite standing projection derives standing and anchors from one verified replay (`crates/magpie-claims/src/replay_snapshot.rs:68-108`), so no governed-standing bypass is claimed. | PR #117 contracted the verified-replay portion. PR #118 structurally removed that raw projection bypass; the independent eligibility seam was unchanged. | Co-derive Deadbolt occurrence eligibility from the same typed claim/evidence metadata, or explicitly constrain the public helper to mechanics-only/untrusted use; do not infer authority from caller selection. |
| RQ-002 | Raw `SignedEvent` and verified replay projection inputs are structurally distinct. | Closed | runtime/API remediation; intentional pre-1.0 source break | `SignedEvent` remains a public Clone/serde raw record (`crates/magpie-log/src/event.rs:330-336`) returned by writer append and raw inspection (`crates/magpie-log/src/logimpl.rs:411-445,482-511`). Public `VerifiedReplayEvent` has a private field, one private constructor, and only the public `event()` observer (`logimpl.rs:63-135`; re-export `crates/magpie-log/src/lib.rs:98-105`). `Projection::apply` accepts only `&VerifiedReplayEvent<'_>` (`logimpl.rs:587-624`), and no `events` alias exists. The sole constructor call follows complete retained-snapshot verification (`:533-571`). Compile-fail proofs cover private construction, conversion, deserialization, lifetime retention, raw/inspection/writer-returned apply, and summary/`verify_chain` blessing (`:75-119,138-181,490-528,591-622`); late signature/hash/link tests apply nothing (`crates/magpie-log/tests/chain.rs:354-452`). | PR #117, merge `99512b5eddd39ecc98e31e59173252da717f2fa2`, froze the documentation-only contract. PR #118, merge `c70c0fb28157212f0f73cef88bae116658c787f7`, implemented the runtime/API/test remediation and owner-merged the reviewed head `99e5b9f783ad78cce6fd4af419f6de6e8719fb25`. | Closed; reopen only if raw/caller-selected events can again enter the public projection boundary without complete replay verification. |
| A-010 | Accepted currentness doctrine now exists, but resolver substrate, representation, eligibility, and algorithm identity remain future. | Partially remediated | constitutional; narrowed; future implementation gate | ADR-0006 defines coordinate-bound derived currentness and separates it from standing (`docs/adr/0006-claim-currency-and-currentness-semantics.md:3,111-137,356-385`); no currentness resolver implementation exists, and existing `StandingCurrentness` compatibility fields must not be reinterpreted. | PR #103 (`21067dc`) materially resolved the doctrine/status conflict and explicitly deferred runtime, encoding, eligibility, and identity details. | owner doctrine decision |
| A-011 / RQ-015 | Dependency, CI, and release validation inputs remain insufficiently pinned/reviewed for semantic reproducibility. | Confirmed | process/release debt | `.github/workflows/ci.yml:13-18,23,52-60` uses moving toolchain/action/Python inputs; `Cargo.lock` pins Rust dependency resolution but not the external verifier environment; the release contract expects both verifier fixtures. | No CI or dependency change. | CI/release assurance |
| A-012 / RQ-010 | A public episodic path can still drop same-named tables on schema mismatch without an ownership marker. | Confirmed | conditional; contained to derived storage | `EpisodicView::at_path` and its `user_version` mismatch branch remain at `crates/magpie-episodic/src/lib.rs:103-127`. | No storage-safety change. | storage/resource contract |
| A-013 | ADR-0001 still names a `seam-slice.patch` that is not present as a current repository artifact. | Confirmed | documentation-only remainder | `docs/adr/0001-deadbolt-seam.md:116` names the patch; `Test-Path seam-slice.patch` is false. Landed tag-5 sources and fixtures are present, but the reference is not traceable to a file. | No traceability correction. | ADR traceability / documentation maintenance |
| A-014 | Segment-anchor validation checks the witness-root lexical form but not all remaining identity-field bounds at the Magpie seam. | Confirmed | conditional; contained seam gap | `Payload::validate` and `deadbolt_context` remain at `crates/magpie-log/src/event.rs:118-133` and `crates/magpie-claims/src/deadbolt_context.rs:249-279`; later exact metadata matching contains malformed markers. | No format or seam-runtime change. | Deadbolt–Magpie seam contract |
| A-015 / A-017 / RQ-003 | Public legacy/raw standing and mutable compatibility projections can still look like governed living status. | Confirmed | compatibility-only; narrowed by doctrine clarification; high | Public `standing` and `legacy_raw_standing` remain in `crates/magpie-claims/src/standing.rs:76-108,273-349,688`; the quarantine test still demonstrates raw `Settled` beside governed `Conjectured` (`crates/magpie-claims/src/standing.rs:1840-1875`). | PR #103 clarified that standing is not currentness and that legacy status is compatibility-only in accepted ADR text, but changed no public fields or construction paths. | compatibility API |
| A-016 | Withdrawal target doctrine is more exact, but identity/delegation and withdrawal runtime remain unimplemented. | Future implementation gate | narrowed; identity/delegation gate | ADR-0005 now requires exactly one prior attributed assertion act and immutable/replayable targeting (`docs/adr/0005-claim-withdrawal-acts.md:38-91`), while attribution is still asserted data and delegation is deferred; no withdrawal event/runtime exists. | PR #103 (`21067dc`) narrowed the target law and accepted ADR-0005 without adding representation or runtime. | identity/withdrawal doctrine |
| A-018 / RQ-013 | Passing tests do not close the missing negative, differential, resource, crash, and exact compile-fail assurance families. | Confirmed | assurance gap; not a test-green closure | Current compile-fail examples include stale-arity patterns (`crates/magpie-claims/src/origin_binding_verifier.rs:40-69`, `crates/magpie-claims/src/origin_admission_audit.rs:92-207`, `crates/magpie-claims/src/support_contribution_audit.rs:86-192`). Current tests pass, but no cross-language malformed corpus, base-log resource, crash, concurrency, or SQLite-failure family was found. | No test/code change. | CI/release assurance |
| A-019 / RQ-011 | Current public legacy resolution, episodic row, and search-result APIs remain detachable and coordinate-poor. | Confirmed | contained current provenance/API gap; compatibility-only; future authority-bearing integration gate | `StandingResolution` lacks prefix/closure identity (`crates/magpie-claims/src/standing.rs:76-97`); public `EpisodicEvent` rows omit complete record identity, verified-prefix, projection, and query coordinates, and `search` returns bare sequence numbers (`crates/magpie-episodic/src/lib.rs:39-52,160-182`). No librarian or MCP runtime exists on current `main`, but the implemented public API deficiency is current. | No query/API change. | provenance-response contract |
| A-020 | Ignored pending delegation tickets can still describe obsolete bases and no wrapper preflight proves current status/allowlist. | Confirmed | mixed-scope: local-only stale-ticket evidence; repository-level wrapper gap; confirmed in the recorded `C:\magpie` checkout | `.agent-runs/pending/0003-migrate-to-v1-format-freeze-layout.md` and `.agent-runs/pending/0006-episodic-schema-version-guard.md` are ignored local files and are absent from a fresh clone of GitHub `main`. Tracked repository code `scripts/codex-delegate.ps1` accepts a caller-supplied existing ticket path without validating ticket root, current status, expected base, or an allowed change surface. | No local ticket-content or tracked wrapper change; ignored ticket files are not repository authority and were not edited. | local delegation/worktree workflow |
| A-021 | Weak external Ed25519 roots and strict verification remain an unpinned profile decision. | Confirmed | conditional; high; trust-root/profile | `docs/FORMAT.md:160-170`, `crates/magpie-log/src/logimpl.rs:235-275,474-479`, and `tools/verify_chain.py:188-221` still leave ordinary versus strict weak-root handling open. The trust root remains externally supplied. | PR #118 explicitly preserves external trust in the supplied key and makes no cryptographic or format change. | verifier/tooling contract |
| A-022 | Admission rejection representation, historical recording, provenance, and retry semantics remain unanswered before a write gate. | Future implementation gate | constitutional; future admission gate | `docs/design/admission-design-readiness-checklist.md:159-165` and the admission question/record contracts retain unanswered rejection and retry choices; no generic admission or `EpistemicGate` runtime exists. | No admission or event-format change. | future admission ADR |
| A-023 | Direct artifact/origin matched traces still omit producing verified-prefix and closure identities. | Confirmed | contained; compatibility/API remainder | Direct trace/receipt surfaces remain in `crates/magpie-claims/src/artifact_provenance_verifier.rs:144-209,219-299,368-387` and `crates/magpie-claims/src/origin_binding_verifier.rs:239-259,360-389,541-548`; stronger v3/v4/audit paths carry more coordinates. | No provenance-response API change. | provenance-response contract |
| A-024 | Claimant-controlled authority labels can still manufacture corroboration separation. | Confirmed | constitutional; high; compatibility semantics explicitly quarantined; accepted successor doctrine unimplemented | The v0 runtime still matches the compiled `human-review` / `authority:origin-review-v0` labels, copies claimant `origin_group` values into trusted fold/admission output (`crates/magpie-claims/src/origin_admission_audit.rs:234-236,559-603,612-700`), and standing v3 counts distinct exact groups at threshold two (`crates/magpie-claims/src/standing_v3.rs:434-435,787-817`). The current contracts now accurately call this claimant-label compatibility admission/corroboration and deny any authenticated independence property (`docs/design/origin-admission-audit-v0.md:5-12,145-152,508-513`; `standing-policy-v3-external-report-corroboration-v0.md:68-81`). ADR-0007 is Accepted but explicitly says acceptance does not implement or remediate A-024 (`docs/adr/0007-authority-bound-origin-corroboration.md:1427-1434,1533-1537`). | PR #108 (`71d096f`) accepted successor doctrine; #109 (`b79a96a`) reconciled current v0 compatibility semantics; #110 (`a1a5920`) quarantined downstream v3 meaning. No authority-bound runtime, hostile tests, or additive successor types landed. | ADR-0007 implementation contract and additive authority-bound successor runtime |
| RQ-012 | Explicit versioned policy/parser duplication continues to create review and drift pressure. | Confirmed | review-cost debt; no authority closure claimed | Separate policy/parser owners remain in `crates/magpie-claims/src/standing_v2.rs::combine_v2_standing`, `standing_v3.rs::combine_standing`, `origin_binding_verifier.rs::parse_binding_envelope`, and `support_contribution_audit.rs::resolve_support_contribution_audit_v0`; current tests cover present surfaces but no cross-version inventory check was found. | No refactor or inventory change. | claims maintainability / private mechanical helpers |

## 7. Individual-ID coverage matrix

This table is the mechanical coverage record. It has exactly one row for each
finding ID; family labels do not create additional findings.

| ID | Audit title or concise finding | Convergence family | Disposition | Current evidence note |
| --- | --- | --- | --- | --- |
| A-001 | Public backend append bypasses declared sole writer | F01 | Closed | Public raw backend append is removed; `LogWriter` is the sole supported Magpie append capability. |
| A-002 | ADR, contract, and living-status precedence is not closed | F02 | Partially remediated | ADR, origin-admission, and v3 status were corrected; attestation/candidate and wider stale-status surfaces remain. |
| A-003 | Detached v0-v2 standing output lacks verified-prefix coordinates | F03 | Confirmed | Legacy resolution shapes remain coordinate-poor. |
| A-004 | Rust and Python accept different record languages | F04 | Confirmed | Case, status, blank-line, and first-failure grammar remains divergent. |
| A-005 | v2/v3 wildcard status matches can promote future variants | F05 | Closed | Both policy seams explicitly enumerate every current status; a future variant is a compile error. |
| A-006 | Episodic projection synthesizes `Conjectured` for statusless tag 6 | F06 | Closed | Tag 6 now projects NULL status and stale schema-v2 derived rows are rebuilt under schema v3. |
| A-007 | Replay freshness and projection failure semantics are caller conventions | F07 | Confirmed | Reuse and infallible projection paths remain. |
| A-008 | File durability, snapshot semantics, concurrency, and L0 resources are incomplete | F08 | Confirmed | Storage and outer resource law remain unspecified. |
| A-009 | Public unverified projection APIs can look trusted | F09 | Partially remediated | PR #118 closes the raw-event projection bypass, but the public Deadbolt occurrence resolver still accepts caller-selected eligibility kind/domain rather than co-deriving their provenance. |
| A-010 | Currentness has doctrine/substrate divergence | F10 | Partially remediated | Accepted doctrine exists; resolver substrate remains future. |
| A-011 | Dependency and review gates do not protect governed semantics fully | F11 | Confirmed | CI/release inputs and fixture gate remain insufficiently fixed. |
| A-012 | Arbitrary SQLite path can trigger destructive derived migration | F12 | Confirmed | Public schema mismatch branch can drop tables. |
| A-013 | Anchor seam references an absent `seam-slice.patch` | F13 | Confirmed | Current ADR reference remains untraceable. |
| A-014 | Anchor payload does not validate all identity fields | F14 | Confirmed | Root is checked; remaining fields rely on later seam matching. |
| A-015 | Public v0-v2 values can fabricate authority-looking output | F15 | Confirmed | Public construction/serialization ambiguity remains. |
| A-016 | Withdrawal attribution lacks verified identity/delegation semantics | F16 | Future implementation gate | Accepted target law still defers identity and runtime. |
| A-017 | Legacy/raw projection fields can become canonical standing | F15 | Confirmed | Public raw standing remains easier to consume than governed resolution. |
| A-018 | Compile-fail tests do not always fail for the forbidden surface | F17 | Confirmed | Several examples fail first on stale arity. |
| A-019 | Derived responses omit record/query provenance coordinates | F18 | Confirmed | Public detachable resolution/episodic/search outputs lack complete coordinates; future authority-bearing use is also gated. |
| A-020 | Pending delegation queue has obsolete state assumptions | F19 | Confirmed | Ignored stale tickets are local-only; the tracked delegation wrapper still lacks ticket-root, status, base, and scope preflight. |
| A-021 | Weak-root and strict Ed25519 verification are not pinned | F20 | Confirmed | Trust-root/profile decision remains conditional. |
| A-022 | Admission rejection and retry semantics are not governed | F21 | Future implementation gate | Generic admission/EpistemicGate remains prohibited pending answers. |
| A-023 | Direct matched traces omit prefix and closure identities | F22 | Confirmed | Direct trace surfaces remain less coordinate-complete than newer audits. |
| A-024 | Claimant-controlled authority labels manufacture corroboration separation | F23 | Confirmed | Current compatibility semantics are quarantined and Accepted successor doctrine exists, but no independent authority-binding runtime is present. |
| RQ-001 | Public backend append bypasses declared sole writer | F01 | Closed | The same capability seam is objectively removed with direct API and compile-fail evidence. |
| RQ-002 | Unverified and verified events share projection-compatible type | F09 | Closed | Raw and verified replay inputs now have distinct public types, and the verified wrapper is non-constructible downstream. |
| RQ-003 | Legacy raw standing remains public and authority-looking | F15 | Confirmed | Raw field remains public and serializable. |
| RQ-004 | Episodic projection invents `Conjectured` | F06 | Closed | Statusless typed assertions now project NULL with schema-v2-to-v3 rebuild coverage. |
| RQ-005 | v2/v3 wildcard matches can promote future variants | F05 | Closed | Exhaustive policy matches force explicit review for every future status variant. |
| RQ-006 | Rust and Python accept different record languages | F04 | Confirmed | Current golden intersection does not prove grammar parity. |
| RQ-007 | Base log verification has no file/record resource bound | F08 | Confirmed | Whole-file and whole-record materialization remains. |
| RQ-008 | FileStore lacks durability/atomicity/locking/snapshot contract | F08 | Confirmed | Two-write append has no declared crash/concurrency semantics. |
| RQ-009 | Infallible projection permits SQLite/range panics | F07 | Confirmed | `expect` and range panic paths remain. |
| RQ-010 | Episodic schema reset can drop tables arbitrarily | F12 | Confirmed | Public schema mismatch reset remains conditional hazard. |
| RQ-011 | Detached legacy resolutions/search results lack coordinates | F18 | Confirmed | Current public detachable outputs are coordinate-poor; future query/response use remains gated. |
| RQ-012 | Versioned policy/parser duplication creates drift pressure | F24 | Confirmed | Review-cost debt remains without an inventory gate. |
| RQ-013 | Test suite lacks cross-language/resource/crash/exact-boundary assurance | F17 | Confirmed | Green current tests do not cover listed failure families. |
| RQ-014 | Design-document runtime status drifted from landed code | F02 | Partially remediated | PRs #103, #109, and #110 corrected more of the corpus; attestation/candidate and wider stale handoffs remain. |
| RQ-015 | CI toolchain/actions/Python crypto dependency are not pinned | F11 | Confirmed | Moving validation inputs remain. |

## 8. Materially changed findings

The following records capture every primary disposition marked Partially
remediated, the three convergence families fully Closed by PRs #111/#112/#115,
the RQ-002 closure and A-009 material partial remediation from PR #118, and
findings materially narrowed by documentation in PRs #103/#108-#110. PR #117
supplied the contract for the verified-replay slice but did not itself
remediate runtime. Closure and partial remediation are based on current
implementation and tests, not on ticket or PR prose alone.

### A-001 / RQ-001 — sole public L0 write capability

- Original audit conclusion: public `LogStore::append_record` gave every
  mutable `FileStore`, `MemStore`, or custom writable store holder a raw
  persistence capability beside `LogWriter`. Later verification could detect
  invalid history, but could not remove that peer capability.
- Contract step: PR #114 merged as
  `142793a47c0f5a7e26ec29fc100d2d6cde9ddb3a`. Its reviewed Option B retains
  public custom read substitution while intentionally removing external custom
  writer backends; storage abstraction no longer implies public raw append.
  This documentation-only step did not itself remediate runtime.
- Runtime change: PR #115 merged as
  `66198af8dc06b086e77036fafd5e18adcdd967b2`. Public `LogStore` now has only
  `read_records`. Crate-private `WriterStore` owns `append_record`; only
  crate-owned `FileStore`, `MemStore`, and crate-internal test instrumentation
  implement it. Public writer construction and typed `append(Provenance,
  Payload)` are specialized to `LogWriter<FileStore>` and
  `LogWriter<MemStore>`.
- Negative evidence: permanent compile-fail cases deny raw append from
  `FileStore`, `MemStore`, and generic `S: LogStore`; deny append and backend
  extraction from `LogReader`; deny `LogWriter<MyReadStore>::open`; and show a
  bare `SigningKey` has no persistence operation. The cases use current imports
  and constructors and fail at the intended missing-method boundary.
- Positive preservation: external-style `ChangingSnapshotStore`,
  `CountingStore`, `ChangingStandingStore`, `OneReadStore`, and `ChangingStore`
  remain read-only `LogStore` implementations for reader/replay tests.
  `LogWriter<MemStore>` and `LogWriter<FileStore>` still create, reopen, and
  continue valid chains; empty open creates exactly one genesis; existing
  history recovers its exact count and tip; an injected private persistence
  failure leaves the writer tip and sequence unchanged; and golden/hash/
  signature/Deadbolt-anchor bytes remain unchanged.
- Scope distinction: direct operating-system mutation of a `FileStore` path is
  outside this safe public Rust API invariant. `MemStore::from_records` creates
  caller-owned historical input rather than exposing mutation of an existing
  store. Storage durability/resource semantics (A-008/RQ-007/RQ-008), exact
  compile-fail assurance elsewhere (A-018/RQ-013), generic admission (A-022),
  and authority-bound corroboration (A-024) remain separate. The later
  verified-replay projection-input remediation is recorded independently below.
- Disposition basis: **Closed**. Current code structurally removes the entire
  audited public backend-append bypass while preserving typed supported writing
  and public read substitution. Reopen only if Magpie introduces another public
  non-`LogWriter` persistence path.

### A-009 / RQ-002 — verified replay remediation and residual eligibility seam

- Original audit conclusion: A-009 contained two public-boundary hazards.
  First, public `SignedEvent` was simultaneously raw, caller-controlled
  historical material and the input to `Projection::apply`, so a caller could
  drive authority-looking derived state manually without complete chain replay
  verification. Second, the public Deadbolt occurrence resolver accepted
  caller-selected `EvidenceKind` and `ClaimDomain` eligibility inputs rather
  than deriving their provenance from the target claim/evidence metadata.
  RQ-002 covered the first, raw-versus-verified projection-compatible type
  defect specifically.
- Contract step: PR #117 merged as
  `99512b5eddd39ecc98e31e59173252da717f2fa2`. It selected and froze the
  verified-replay type-state slice: `SignedEvent` remains raw, while
  `VerifiedReplayEvent<'_>` is an ephemeral replay-only projection input. This
  documentation-only step did not itself remediate runtime or erase the second
  historical A-009 scenario.
- Runtime change: PR #118 merged as
  `c70c0fb28157212f0f73cef88bae116658c787f7`. It added public
  `VerifiedReplayEvent<'_>` with a private field and private constructor,
  changed `Projection::apply` to accept `&VerifiedReplayEvent<'_>`, renamed raw
  inspection to `unverified_events()`, and removed `events()` without an alias.
  The sole constructor call is in the apply loop after the exact retained
  snapshot has completely verified.
- Negative evidence for the resolved slice: permanent N1-N10 compile-fail and
  structural proofs reject raw `SignedEvent` application, raw
  `unverified_events` output, downstream struct literals, deserialization, raw
  conversion, `verify_chain` blessing, `VerifiedReplaySummary` blessing, and
  direct application of a correctly signed writer-returned event. The common
  trait signature covers built-in and custom projections; a repository-wide
  public-surface scan finds no raw apply alias or adapter. The lifetime proof
  rejects retaining the borrowed wrapper beyond its callback, while cloning
  `event.event()` yields only raw `SignedEvent`.
- Positive preservation: a downstream-style custom `Projection` remains
  replay-driven; ClaimsView, StandingView, Deadbolt occurrence context, and
  episodic canonical bytes retain their valid-replay behavior; the private
  composite forwards the same witness to both children; replay reads one
  snapshot; late signature, content-hash, and previous-link failures apply
  nothing and return no summary; successful summary count/tip identify the
  exact applied snapshot; and raw diagnostics remain available through
  `unverified_events()`.
- Resolved portion: raw `SignedEvent` no longer crosses `Projection::apply`.
  RQ-002 is fully closed, and the first A-009 scenario is closed.
- Remaining A-009 portion: public `resolve_deadbolt_occurrence_context` still
  receives `evidence_kind` and `claim_domain`. It checks the required enum
  values but does not co-derive or prove their provenance, and its documentation
  requires trusted callers to derive them consistently from the same target
  metadata. The private composite used by governed standing replay derives the
  relevant structures from one verified replay, so this contained mechanics
  seam is not evidence of a current governed-standing bypass.
- Canonical and trust boundary: FORMAT, `Payload`, `EventCore`, `SignedEvent`
  persisted shape, canonical encoding, hashes/signatures, genesis, and both
  frozen fixture histories are unchanged. Successful replay still depends on
  an externally supplied verifying key and does not establish trust in that
  key.
- Scope distinction: projection fallibility (A-007/RQ-009), detached response
  coordinates (A-019/RQ-011), trust-root policy (A-021), admission semantics
  (A-022), and authority-bound corroboration (A-024) remain distinct and
  unchanged.
- Disposition basis: **RQ-002 is Closed** because its entire audited
  raw-versus-verified projection-compatible type seam is structurally removed
  from the supported public Rust API. Reopen RQ-002 only if caller-selected raw
  events can again enter the public projection boundary without complete replay
  verification. **A-009 is Partially remediated** because that same bypass is
  removed while its separate eligibility-input seam remains. Close A-009 only
  after that seam is objectively removed or its public mechanics-only/untrusted
  semantics structurally eliminate the audited hazard.

### A-002 / RQ-014 — status and runtime handoff reconciliation

- Original audit conclusion: ADR/status precedence was not closed; several
  landed narrow runtimes were still described as future or candidate work.
- Exact subsequent change: PR #103 merged as
  `21067dcfe00175430fe751d669f136a182f6989b`. It changed ADR-0002 through
  ADR-0006 and related README/design status text, setting ADR-0002 through
  ADR-0006 to Accepted and recording the 2026-08-02 ratification. PR #109
  (`b79a96ab0ddc5c8b9008231ef53e325efdd067d3`) reconciled the origin-binding
  and origin-admission handoffs; PR #110
  (`a1a59200c5a1af2c81afe17b76b89609134d5787`) reconciled standing-v3 and
  contribution/aggregation status and compatibility wording.
- Resolved portion: the former ADR header conflict was corrected; tags 6-8
  were described as frozen representation; standing was explicitly separated
  from currentness; supersession, invalidation, withdrawal, and ratification
  were assigned consequences by their selected facets rather than by edge
  presence. The origin-admission and standing-v3 notes now accurately identify
  their landed runtimes and distinguish compatibility behavior from the future
  authority-bound successor.
- Remaining portion: `inline-predicate-attestation-binding-v0.md` and
  `claim-inline-subject-binding-v0.md` still call landed runtime a `current
  candidate patch`; Tickets 0061/0063 retain uncommitted-candidate handoffs;
  and the bootstrap, visual-ledger, and other unclassified/mixed-status
  materials identified by A-002 still lack one inspectable current-status and
  precedence reading.
- Disposition basis: Partially remediated is correct because a material
  doctrine/status portion changed, while the broader living-status defect and
  its implementation confusion remain. The correction locus is now a narrow
  status/precedence registry and current handoffs, not another runtime change.

### A-005 / RQ-005 — fail-closed standing status evolution

- Original audit conclusion: wildcard inherited-status arms in standing v2 and
  v3 allowed a future `Status` variant to acquire positive semantics without
  an explicit policy decision.
- Exact subsequent change: PR #111 merged as
  `dc07034d4af7d2b41130ba04a6a9d4c5af000995`. Both policy seams now match the
  complete current five-variant `Status` vocabulary explicitly; wildcards are
  limited to the independent Boolean dimension after status selection.
- Current behavior preservation: the permanent v2 and v3 truth tables cover
  all 6 `Option<Status>` states by both Boolean states, 12 cases per policy,
  and retain every previously representable result, including their
  intentional difference for `None + positive evidence`.
- Future evolution property: adding a new `Status` variant makes both Rust
  matches non-exhaustive, forcing an explicit policy arm and review before the
  vocabulary extension can compile.
- Disposition basis: **Closed**. The whole audited wildcard-promotion seam is
  removed; no equivalent wildcard remains in the v2/v3 combination scope.

### A-006 / RQ-004 — statusless episodic assertion projection

- Original audit conclusion: `ClaimAssertedV2` records no status, but episodic
  rows and canonical projection bytes synthesized `Conjectured`.
- Exact subsequent change: PR #112 merged as
  `91ebb5d6a4f5ba99ad377c77eb96dc70bc829245`. The tag-6 projection now stores
  `claim_status = None`, and episodic schema/projection version is 3.
- Preserved behavior: legacy `ClaimAsserted` retains its exact signed status;
  `ClaimStatusChanged` retains exact `from`/`to` values; row shape, L0 format,
  hashes, signatures, and standing policy are unchanged.
- Semantic migration evidence: a real schema-v2-shaped database containing a
  stale `claim_asserted_v2` / `Conjectured` row is discarded on v3 open; the
  caller-owned verified replay regenerates NULL status; rebuilt canonical bytes
  equal a fresh replay; and current schema-v3 state survives reopen.
- Disposition basis: **Closed**. The correction is intentionally observable in
  derived episodic state and canonical projection bytes. It does not close or
  narrow A-007, A-008, or A-012.

### A-010 — standing/currentness doctrine and future substrate

- Original audit conclusion: currentness had unresolved target, ordering,
  scope, actor, cycle, output, and coordinate choices, with no currentness
  resolver.
- Exact subsequent change: accepted ADR-0006 now defines currentness as a
  derived, resolver-relative facet at explicit snapshot, policy-version, and
  resolver-identity coordinates; it rejects time/ambient latest semantics,
  separates currentness from standing, and gives relationship consequences to
  selected policies.
- Resolved portion: the standing/currentness conceptual collision and the
  former ADR status conflict were materially narrowed. The former
  supersession wording in ADR-0002 was expressly superseded only where it
  conflated standing with currentness.
- Remaining portion: ADR-0006 explicitly defers concrete eligibility,
  identity/authority, representation, encoding, resolver algorithm, and
  runtime. Existing compatibility currentness values are not a currentness
  implementation.
- Disposition basis: Partially remediated, with a future implementation-gate
  qualifier. The new doctrine replaced part of the old premise, but the
  missing substrate and unresolved future choices are an equivalent
  implementation boundary, not closure.

### A-016 — withdrawal identity and target

- Original audit conclusion: withdrawal was absent at runtime and attribution
  was asserted data rather than verified identity; label equality and
  signer/delegation binding could diverge.
- Exact subsequent change: accepted ADR-0005 now requires a valid withdrawal
  to identify exactly one prior assertion act through an immutable,
  replayable, injective target reference.
- Resolved portion: ambiguous “latest,” timestamp, all-match, or claim-only
  target selection is no longer the accepted doctrine.
- Remaining portion: concrete representation, actor identity, delegation,
  organisational authority, and withdrawal runtime remain future. The current
  record still cannot verify that the asserted actor owns the target.
- Disposition basis: Future implementation gate, narrowed. The named future
  withdrawal capability must not be implemented on label equality or by
  inferring identity from the log.

### A-015 / A-017 / RQ-003 — raw standing and currentness separation

- Original audit conclusion: public mutable/serializable compatibility fields
  could be read as governed standing or currentness, even though private
  resolver paths quarantined raw status.
- Exact subsequent change: PR #103 accepted ADR-0003 and ADR-0006 language
  stating that currentness is not standing and that legacy status is
  compatibility material rather than a governed truth decision.
- Resolved portion: the governing prose now gives a reader a direct
  standing/currentness distinction.
- Remaining portion: `StandingClaim.standing`, public legacy fields, public
  constructors/serialization shapes, and the test-demonstrated raw `Settled`
  versus governed `Conjectured` split remain in current code. No structural
  capability boundary changed.
- Disposition basis: Confirmed with a narrowed/compatibility-only qualifier.
  Prose reduces one interpretation ambiguity but does not remove the
  authority-looking API surface; documentation acknowledgement is not
  closure.

### A-024 — claimant-controlled corroboration separation

- Original audit conclusion: claimant-supplied authority labels and origin
  groups can pass canonical/anchored verification, compiled selection,
  admission, and standing-v3 distinct-group counting without independently
  authenticated grouping authority.
- Exact subsequent change: PR #108 merged Accepted ADR-0007 as
  `71d096ff3c7ca8107d51484f486c30031d337ee8`; PR #109 merged current-v0
  contract reconciliation as `b79a96ab0ddc5c8b9008231ef53e325efdd067d3`;
  PR #110 merged downstream standing/contribution quarantine as
  `a1a59200c5a1af2c81afe17b76b89609134d5787`.
- Resolved portion: accepted successor doctrine now requires a separately owned
  authority-binding object and explicit `H + P + M + A` coordinates. Current
  v0/v3 documentation now consistently classifies existing output as
  claimant-label compatibility admission/corroboration and denies any
  authenticated organisational, causal, statistical, editorial, or
  legal-entity independence property.
- Remaining portion: the current runtime still selects the claimant's compiled
  authority-label pair, admits claimant `origin_group` values, and counts two
  distinct groups for v3 corroboration. No ADR-0007 implementation contract,
  additive authority-bound types/runtime, hostile suite, or compatibility
  review has landed.
- Disposition basis: **Confirmed**. Accepted doctrine and semantic quarantine
  are real progress, but they do not authenticate current v0 authority or make
  current v3 authority-bound. The future-use prohibition remains in force.

## 9. Confirmed constitutional and P0 boundaries

These are the highest-priority surviving boundaries. This section records why
they remain open and the owning layer; it does not design a fix.

- **Caller-controlled Deadbolt occurrence eligibility — A-009 remainder.**
  Verified replay provenance is now structurally enforced, and raw
  `SignedEvent` cannot drive `Projection::apply`. The public occurrence
  resolver nevertheless still accepts caller-selected `EvidenceKind` and
  `ClaimDomain`; it checks those values but does not co-derive them from the
  same claim/evidence metadata. Trusted use therefore still relies on caller
  convention at this contained mechanics boundary. Private governed standing
  paths derive the relevant structures from one verified replay and are not
  claimed to be bypassed.

- **Claimant-controlled corroboration separation — A-024.** The binding
  parser checks protocol-shaped labels, while the origin-admission fold copies
  claimant-selected groups and the v3 policy counts distinct groups. No
  independent signer, credential, root, or authority coordinate binds the
  labels. ADR-0007 is Accepted and current compatibility semantics are
  explicitly quarantined, but the required additive authority-bound successor
  is unimplemented. The next correction locus is its separately reviewed
  implementation contract and runtime, not another doctrine decision.

- **Rust/Python accepted-language divergence — A-004/RQ-006.** Rust and
  Python still disagree on hex case, status representation, blank lines, and
  JSON/storage behavior around the shared record boundary. The next decision
  belongs to the verifier/tooling contract and cross-language vector owner.

- **Weak-root/strict-verification ambiguity — A-021.** The external trust root
  remains caller-supplied, but v1 does not select one weak-key acceptance rule
  shared by Rust and Python. The next decision belongs to the crypto/format
  compatibility owner.

## 10. Future implementation gates

Only A-016 and A-022 carry **Future implementation gate** as their primary
disposition. The table also records secondary future-use prohibitions for
A-010, A-019/RQ-011, and A-024 without changing those findings' primary
dispositions.

| Future capability | Gate findings and disposition role | Prohibited premature capability |
| --- | --- | --- |
| Generic admission / `EpistemicGate` | A-022 — primary future gate | Do not add a generic writer, admission mechanism, rejected-attempt history, retry authority, or status-bearing gate while rejection representation and retry semantics remain unanswered. |
| Currentness resolver | A-010 — secondary gate; primary Partially remediated | Do not implement a resolver, currentness encoding, or ambient “latest” behavior before the accepted coordinates are bound to concrete eligibility, identity, algorithm, and output decisions. |
| Withdrawal runtime and identity/delegation | A-016 — primary future gate | Do not add withdrawal effects, actor ownership, delegation, or organisational authority based on asserted labels alone. |
| Authority-bearing librarian/MCP/query surfaces | A-019 / RQ-011 — secondary gate; primary Confirmed | Do not expose detached rows, search sequences, or legacy resolutions as reproducible authority-bearing responses before the record, snapshot, policy, query, and derivation coordinates are governed. |
| Authority-bearing origin corroboration | A-024 — secondary gate; primary Confirmed | Do not treat claimant-controlled authority labels or origin groups as independently verified corroboration merely because the bytes are valid and anchored. |

## 11. Provisional programme order

This is non-authorizing planning guidance only. It is not a roadmap
ratification, an implementation ticket, or permission to start work.

1. cross-language verifier contract and normative vectors — A-004 / RQ-006;
2. authority-bound corroboration implementation contract and additive
   successor — A-024;
3. provenance-complete response surfaces;
4. storage/resource/fallible-projection contracts;
5. remaining crypto/release assurance;
6. deferred admission, currentness, withdrawal, identity, and
   authority-bearing query capabilities.

The prior authority-decision-first wording became stale when ADR-0007 was
Accepted. The authority-bound successor remains unimplemented, but its doctrine
decision is no longer pending. The revised order records dependency pressure
visible in the evidence: the append-capability and verified-replay/projection
substrates are now sealed, so the next named programme item is the
cross-language verifier grammar boundary. The already-decided authority-bound
successor still requires a separate authorized slice. This order does not
choose mechanisms or owners and does not authorize A-004 or ADR-0007 work.

## 12. Coverage and validation

The complete audit files were read, and their finding headings were extracted
with anchored PowerShell regexes so occurrences such as the numeric fragment
inside a hash notation could not become a false finding ID:

```powershell
$a = Get-Content -Raw docs/audits/magpie-architecture-audit-2026-08-01.md
$r = Get-Content -Raw docs/audits/magpie-runtime-quality-audit-2026-08-04.md
$archIds = @([regex]::Matches($a, '(?m)^###\s+(A-\d{3})\b') |
  ForEach-Object { $_.Groups[1].Value } | Sort-Object -Unique)
$runtimeIds = @([regex]::Matches($r, '(?m)^###\s+(RQ-\d{3})\b') |
  ForEach-Object { $_.Groups[1].Value } | Sort-Object -Unique)
```

That extraction returned 24 architecture IDs, A-001 through A-024, and
15 runtime IDs, RQ-001 through RQ-015. The broad unanchored scan also
demonstrated why hash prose must not be used as a finding inventory.

The following read-only commands were used for the baseline, change, evidence,
and validation review:

```text
git fetch --all --prune
git status --short --untracked-files=all
git branch --show-current
git rev-parse HEAD
git rev-parse origin/main
git merge-base HEAD origin/main
git merge-base 3b46fdb81857d77b827271777b86745bca5f7b12 HEAD
git merge-base --is-ancestor 99512b5eddd39ecc98e31e59173252da717f2fa2 HEAD
git merge-base --is-ancestor c70c0fb28157212f0f73cef88bae116658c787f7 HEAD
git log --oneline --decorate -25 origin/main
git ls-remote origin refs/heads/main
git show --stat --summary 99512b5eddd39ecc98e31e59173252da717f2fa2
git show --stat --summary c70c0fb28157212f0f73cef88bae116658c787f7
git diff --name-status 99512b5eddd39ecc98e31e59173252da717f2fa2..c70c0fb28157212f0f73cef88bae116658c787f7
rg -n "^### (A|RQ)-[0-9]{3}\b" docs/audits/magpie-architecture-audit-2026-08-01.md docs/audits/magpie-runtime-quality-audit-2026-08-04.md
rg -n "VerifiedReplayEvent|from_verified_snapshot|unverified_events|impl Projection for|\.apply\(" crates
cargo fmt --all --check
cargo test -p magpie-log --locked
cargo test --doc -p magpie-log --locked
cargo test -p magpie-claims --locked --test standing_replay_snapshot
cargo test -p magpie-episodic --locked --test episodic
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked --no-fail-fast
cargo doc -p magpie-log --locked --no-deps
python tools/verify_chain.py crates/magpie-log/testdata/golden-v1.jsonl ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c
python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737
python tools/check_release_metadata.py
git diff --check
```

The focused `magpie-log` command passed 47 unit/integration tests (3 library,
27 chain, 4 Deadbolt-fixture, and 13 golden) plus all 19 documentation tests.
Nine verified-replay boundary compile-fail cases cover the wrapper boundary
and lifetime. Direct metadata probes against the built public crate reproduced
E0308 for raw apply, E0451 for struct-literal construction, E0277 for raw
conversion and
deserialization, E0599 for `verify_chain`/summary blessing, and the intended
callback-lifetime failure. The focused standing snapshot suite passed all 5
tests, including one-read changing-store behavior and same-witness composite
fan-out. The focused episodic integration suite passed all 11 tests, including
independent verified-replay byte identity and zero partial state on tampering.

The full locked workspace suite passed with zero failures across every unit,
integration, documentation, and compile-fail target; the `magpie-claims`
documentation-test target reported 281 passed and the `magpie-log` target
reported 19 passed. Passing tests support RQ-002 closure and the resolved
A-009 portion only together with the exact type/capability evidence above;
unrelated eligibility/hostile/resource/crash/differential gaps remain recorded.

Formatting and workspace clippy with warnings denied passed. Rustdoc generation
and generated-API inspection found public `VerifiedReplayEvent`, public
`event()`, a wrapper-only `Projection::apply`, and public
`unverified_events()`, while showing private fields and no public constructor,
`events()` alias, Clone/Copy/Default/serde/conversion minting route, or raw
projection adapter. Both Python chain verifiers accepted 9/9 golden and 2/2
Deadbolt-anchor records. Release metadata inventories matched: `magpie-log` 19,
`magpie-claims` 48, and `magpie-episodic` 9.

The final check also used `git diff --check`, a changed-path allowlist audit,
post-edit SHA256 hashes for every protected evidence file, and this one-off
PowerShell table check. The table check parses only the first cell of the
convergence rows and the one-ID first cell of the individual matrix:

```powershell
$ledger = Get-Content -Raw docs/audits/audit-disposition-2026-08.md
$arch = @('A-001','A-002','A-003','A-004','A-005','A-006','A-007','A-008',
  'A-009','A-010','A-011','A-012','A-013','A-014','A-015','A-016','A-017',
  'A-018','A-019','A-020','A-021','A-022','A-023','A-024')
$runtime = @('RQ-001','RQ-002','RQ-003','RQ-004','RQ-005','RQ-006','RQ-007',
  'RQ-008','RQ-009','RQ-010','RQ-011','RQ-012','RQ-013','RQ-014','RQ-015')
$expected = @($arch + $runtime)
$familyBlock = [regex]::Match($ledger,
  '(?ms)^## 6\. Converged findings ledger.*?(?=^## 7\.)').Value
$matrixBlock = [regex]::Match($ledger,
  '(?ms)^## 7\. Individual-ID coverage matrix.*?(?=^## 8\.)').Value
$familyIds = @($familyBlock -split "`r?`n" |
  Where-Object { $_ -match '^\|\s*[^-].*\|' } |
  ForEach-Object {
    $cell = ($_ -split '\|')[1]
    [regex]::Matches($cell, '(?:A|RQ)-\d{3}') |
      ForEach-Object { $_.Value }
  })
$matrixIds = @($matrixBlock -split "`r?`n" |
  Where-Object { $_ -match '^\|\s*(?:A|RQ)-\d{3}\s*\|' } |
  ForEach-Object { [regex]::Match($_,
    '^\|\s*((?:A|RQ)-\d{3})\s*\|').Groups[1].Value })
$unknown = @($familyIds + $matrixIds | Where-Object { $_ -notin $expected } |
  Sort-Object -Unique)
$missing = @($expected | Where-Object { $_ -notin $familyIds -or
  $_ -notin $matrixIds })
$familyDuplicates = @($familyIds | Group-Object |
  Where-Object Count -ne 1 | Select-Object -ExpandProperty Name)
$matrixDuplicates = @($matrixIds | Group-Object |
  Where-Object Count -ne 1 | Select-Object -ExpandProperty Name)
if ($unknown.Count -or $missing.Count -or $familyDuplicates.Count -or
    $matrixDuplicates.Count -or $familyIds.Count -ne 39 -or
    $matrixIds.Count -ne 39) { throw 'finding coverage check failed' }
'coverage: PASS; 39 family memberships and 39 individual rows'

$familyPrimary = @($familyBlock -split "`r?`n" |
  Where-Object { $_ -match '^\|\s*(?:A|RQ)-\d{3}' } |
  ForEach-Object {
    $cells = $_ -split '\|'
    $disposition = $cells[3].Trim()
    [regex]::Matches($cells[1], '(?:A|RQ)-\d{3}') |
      ForEach-Object {
        [pscustomobject]@{ Id = $_.Value; Disposition = $disposition }
      }
  })
$matrixPrimary = @($matrixBlock -split "`r?`n" |
  Where-Object { $_ -match '^\|\s*(?:A|RQ)-\d{3}\s*\|' } |
  ForEach-Object {
    $cells = $_ -split '\|'
    [pscustomobject]@{
      Id = $cells[1].Trim()
      Disposition = $cells[4].Trim()
    }
  })
$familyDisposition = @{}
$matrixDisposition = @{}
$familyPrimary | ForEach-Object {
  $familyDisposition[$_.Id] = $_.Disposition
}
$matrixPrimary | ForEach-Object {
  $matrixDisposition[$_.Id] = $_.Disposition
}
$confirmed = @($familyPrimary |
  Where-Object Disposition -eq 'Confirmed').Count
$partial = @($familyPrimary |
  Where-Object Disposition -eq 'Partially remediated').Count
$future = @($familyPrimary |
  Where-Object Disposition -eq 'Future implementation gate').Count
$closed = @($familyPrimary |
  Where-Object Disposition -eq 'Closed').Count
$other = @($familyPrimary | Where-Object {
  $_.Disposition -notin
    @('Confirmed', 'Partially remediated', 'Future implementation gate',
      'Closed')
}).Count
$futureIds = @($familyPrimary |
  Where-Object Disposition -eq 'Future implementation gate' |
  Select-Object -ExpandProperty Id | Sort-Object)
$statusMismatch = @($expected |
  Where-Object { $familyDisposition[$_] -ne $matrixDisposition[$_] })
if ($confirmed -ne 26 -or $partial -ne 4 -or $future -ne 2 -or
    $closed -ne 7 -or
    $other -ne 0 -or ($futureIds -join ',') -ne 'A-016,A-022' -or
    $familyDisposition['A-009'] -ne 'Partially remediated' -or
    $familyDisposition['RQ-002'] -ne 'Closed' -or
    $familyDisposition['A-019'] -ne 'Confirmed' -or
    $familyDisposition['RQ-011'] -ne 'Confirmed' -or
    $familyDisposition['A-010'] -ne 'Partially remediated' -or
    $familyDisposition['A-024'] -ne 'Confirmed' -or
    $familyDisposition['A-005'] -ne 'Closed' -or
    $familyDisposition['RQ-005'] -ne 'Closed' -or
    $familyDisposition['A-006'] -ne 'Closed' -or
    $familyDisposition['RQ-004'] -ne 'Closed' -or
    $familyDisposition['A-001'] -ne 'Closed' -or
    $familyDisposition['RQ-001'] -ne 'Closed' -or
    $statusMismatch.Count) { throw 'primary disposition check failed' }
'dispositions: PASS; Confirmed=26, Partially remediated=4, Future implementation gate=2, Closed=7, Total=39'
```

The disposition counts above are weighted by individual finding ID, not by
the number of convergence rows. A-016 and A-022 are the only primary future
gates; A-010, A-019/RQ-011, and A-024 retain the secondary prohibitions stated
in sections 5 and 10 without changing their primary dispositions.

Recorded preservation and scope checks:

- architecture audit SHA256 before and after:
  `7FA0A453E6A6EFBC2650F57D2C2D87FFC24EF86F7E41312DEF3F20184506F31F`;
- runtime audit SHA256 before and after:
  `105EA5CCEE563F4E23330E790B5FB0281A93E11398F73E6E1627B1DF86248FB9`;
- verified-replay design contract SHA256 before and after:
  `58383B2C6592D71C8B72BA09AA02C0AA9195D81448EC6BDF60789431A7294FE4`;
- Ticket 0070 SHA256 before and after:
  `EC05089F892A37482AD51BDC74D6C88F5ED7F9D9F2DBD32EB31D94656E79D338`;
- Ticket 0071 SHA256 before and after:
  `449F71623606E62B007BB21379DA16AD4020EDD3506B8917E79BAFA0548C6F4A`;
- historical v0.1.0 release contract SHA256 before and after:
  `7B810A6E10D0D8C165DD466F8CEC4F05D49D73BB0FF7ACD1F006B7647F9A2F95`;
  tag commit `2bbfbd1f451e35b65e8c89aeaf39801621e484df` and tag/current-main
  blob `dfd8e9312cec0c4b3f4827e70f142e7e5e4ac176` are identical; its historical
  `LogReader::events()` wording is intentionally preserved;
- FORMAT SHA256 before and after:
  `A777875CC745DDCEFC3891701CE930DDDA06C5FA8B4EB046198D41116A3184EC`;
- golden fixture SHA256 before and after:
  `767EF86B7E2AD1D65E8315AE28BAC682BCEEE29223B5F1A0612EC7EF062CFFB4`;
- Deadbolt-anchor fixture SHA256 before and after:
  `2C715006E3E5BEF571B3D31B95F47D3C4BBB3EA68FA9750365B4A62B66CD41E5`;
- final allowed changed path: `docs/audits/audit-disposition-2026-08.md` only;
- `git diff --check`: pass;
- runtime source diff: none;
- final worktree and publication state: recorded in the completion report and
  draft PR; historical audits, contracts, tickets, release evidence, FORMAT,
  fixtures, and runtime were not edited.
