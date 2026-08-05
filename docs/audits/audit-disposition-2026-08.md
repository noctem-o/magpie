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
| Repository | `noctem-o/magpie`, checkout `C:\magpie` |
| Disposition date | 2026-08-05 |
| Branch and worktree mode | `main`; ordinary checkout; existing uncommitted documentation draft preserved in place; ignored local `.agent-runs\pending` state was inspected but not edited |
| Current `main` | `7889150b100795409bfe0d6f676c5780cc7b2830` |
| Local `origin/main` | `7889150b100795409bfe0d6f676c5780cc7b2830` |
| Live `origin/main` check | `git ls-remote origin refs/heads/main` returned the same SHA |
| Audit commit | `3b46fdb81857d77b827271777b86745bca5f7b12` |
| Local HEAD / `origin/main` merge base | `7889150b100795409bfe0d6f676c5780cc7b2830` |
| Audit commit / current-main merge base | `3b46fdb81857d77b827271777b86745bca5f7b12` |
| Expected post-PR-104 SHA | Equal to the verified current `main`; difference from `7889150b100795409bfe0d6f676c5780cc7b2830` is none |
| Relevant merged change | PR #103, merge `21067dcfe00175430fe751d669f136a182f6989b`; documentation-only ADR/status reconciliation |
| Evidence-preservation change | PR #104, merge `7889150b100795409bfe0d6f676c5780cc7b2830`; documentation-only audit preservation |
| Open remediation PRs | None at disposition time |
| Open issues | None at disposition time |
| Preflight changed paths | `docs/audits/README.md` modified and `docs/audits/audit-disposition-2026-08.md` untracked; no staged paths |
| Runtime change since audit | None. The complete audit-to-baseline diff changes documentation paths only; no Rust, Python, test, fixture, dependency, CI, or generated path changed |

The runtime audit was performed on local `main` at the audit commit while the
then-current remote was nine documentation-only commits ahead. The intervening
PR #103 changes corrected and clarified doctrine/status text, not runtime
paths. PR #104 preserved the audit evidence and added its historical index; it
does not close a substantive finding. Current runtime dispositions therefore
retain the audit conclusions unless the evidence below says that a doctrine
boundary was narrowed.

The post-audit ancestry was inspected as:

`d449418`, `5cf4207`, `ae26ebb`, `d453906`, `b6cc4dc`, `5b1682c`, `459d729`,
`4c51dc7`, `21067dc`, `6aef94d`, and `7889150`.

PR #103 changed `README.md`, ADR-0002 through ADR-0006, and two design notes.
It made ADR-0002 through ADR-0006 Accepted doctrine effective 2026-08-02,
separated standing from currentness, and retired a former relationship-edge
invalidation convention. Its body explicitly records that no Rust, FORMAT,
canonical encoding, schema, vocabulary, fixture, policy, or runtime behavior
changed. PR #104 changed only the audit index and the two preserved audit
records.

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
| Confirmed | 34 |
| Partially remediated | 3 |
| Superseded | 0 |
| Accepted debt | 0 |
| Future implementation gate | 2 |
| Scheduled | 0 |
| Closed | 0 |
| **Total** | **39** |

The principal convergence families are: write capability and verified replay;
status/precedence and currentness doctrine; cross-language grammar; wildcard
status evolution; synthetic projection status; projection failure and storage
resource law; compatibility/raw standing; detached provenance coordinates;
authority/corroboration; admission and identity gates; and test/release
assurance. Similar names were not enough to merge distinct surfaces; the
family boundaries and correction loci are retained in section 6.

The surviving present constitutional defects are A-001, the residual A-002
status/precedence boundary, A-005, and A-024. A-010 and A-022 retain
constitutional future-gate significance: each prohibits premature
implementation, but neither presently describes an implemented runtime that
contradicts a completed deferred mechanism. High or P0 boundaries also remain
in A-003, A-004, A-015, A-021, RQ-001, RQ-002, RQ-005, and RQ-006, with
RQ-003 retaining its authority-looking compatibility leak. None is
objectively closed.

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
| A-001 / RQ-001 | Public backend append remains a second write capability beside `LogWriter`. | Confirmed | constitutional; contained misuse hazard; P0 | `crates/magpie-log/src/store.rs:12-13,31-37,80`; `crates/magpie-log/src/logimpl.rs:307`; ADR-0001 still names the writer as sole capability. Verification can reject bad bytes, but does not remove the callable sink. | No runtime change. PR #103 and #104 were documentation-only. | `magpie-log` API/storage |
| A-002 / RQ-014 | Current-status and precedence drift remains outside the portion reconciled by PR #103. | Partially remediated | constitutional remainder; narrowed; documentation-only remainder | ADR-0002 through ADR-0006 are Accepted and ADR-0006 separates standing/currentness (`docs/adr/0002-governed-claim-memory.md:3,190-204`; `docs/adr/0006-claim-currency-and-currentness-semantics.md:3,111-137`). V3, origin-admission, and attestation notes still contain future/candidate handoff language (`docs/design/standing-policy-v3-external-report-corroboration-v0.md:3-14`; `docs/design/origin-admission-audit-v0.md:3-16`; `docs/design/inline-predicate-attestation-binding-v0.md:3-10`). | PR #103 (`21067dc`) corrected the ADR stack and several living status claims. PR #104 (`7889150`) preserved historical audit records and did not establish doctrine. | owner doctrine decision |
| A-003 | Detached v0-v2 standing outputs still omit complete verified-prefix coordinates. | Confirmed | high; compatibility-only | `StandingResolution`, `StandingResolutionV1`, and `StandingResolutionV2` remain public coordinate-poor shapes in `crates/magpie-claims/src/standing.rs:76-97`, `standing_v1.rs:124`, and `standing_v2.rs:135`; newer v3/v4 paths carry private prefix/closure identities. | No runtime/API change. | provenance-response / compatibility API |
| A-004 / RQ-006 | Rust and Python still accept different JSONL record languages and first-failure behavior. | Confirmed | high; P0; interoperability | Rust uses `hex::decode` (`crates/magpie-log/src/hashing.rs:48-54`, `event.rs:315-325`) and skips blank lines (`store.rs:41-54`); Python has its own hex/status/line grammar (`tools/verify_chain.py:49-60,206-237`). | No verifier or format change. Existing golden vectors cover the common intersection only. | verifier/tooling contract |
| A-005 / RQ-005 | v2/v3 wildcard inherited-status arms remain latent promotion paths for a future status. | Confirmed | constitutional; latent; high | `standing_v2.rs:304` uses `Some(_) if deterministic_supported`; `standing_v3.rs:890` uses `_ if corroborated`; v4 remains more explicit. | No policy-code change and no new status was added. | versioned standing policy code and tests |
| A-006 / RQ-004 | Episodic output still assigns `Conjectured` to `ClaimAssertedV2`, which records no status. | Confirmed | current semantic defect | `Payload::ClaimAssertedV2` has no status (`crates/magpie-log/src/event.rs:85-92`); `crates/magpie-episodic/src/lib.rs:328-338` emits `Some("Conjectured")`. | No projection or fixture change. | `magpie-episodic` projection/schema |
| A-007 / RQ-009 | Projection reuse and fallible episodic operations still lack a typed lifecycle/error boundary. | Confirmed | current availability defect; contained to derived state | `Projection::apply` remains infallible (`crates/magpie-log/src/logimpl.rs:419-425`); episodic transaction, insert, commit, and signed-range paths still use `expect`/panic (`crates/magpie-episodic/src/lib.rs:214-250,390-398`). | No replay/projection runtime change. | storage/resource contract |
| A-008 / RQ-007 / RQ-008 | The storage seam still lacks a complete snapshot, durability, concurrency, framing, and outer resource law. | Confirmed | operational/resource debt; conditional under local use | `FileStore` writes two pieces without sync or lock (`crates/magpie-log/src/store.rs:17-47`); `read_records` materializes the file and splits every line (`store.rs:41-54`); replay retains raw and parsed data (`logimpl.rs:375-386`). | No storage/runtime change. | storage/resource contract |
| A-009 / RQ-002 | Unverified parsed events and verified replay inputs remain the same public projection-compatible type. | Confirmed | contained misuse hazard; P0 | `LogReader::events` returns `Vec<SignedEvent>` while `Projection::apply` consumes `SignedEvent` (`crates/magpie-log/src/logimpl.rs:350-360,371-425`); `deadbolt_context.rs:41-56` documents the unverified path. | No type or capability change. | `magpie-log` replay/projection API |
| A-010 | Accepted currentness doctrine now exists, but resolver substrate, representation, eligibility, and algorithm identity remain future. | Partially remediated | constitutional; narrowed; future implementation gate | ADR-0006 defines coordinate-bound derived currentness and separates it from standing (`docs/adr/0006-claim-currency-and-currentness-semantics.md:3,111-137,356-385`); no currentness resolver implementation exists, and existing `StandingCurrentness` compatibility fields must not be reinterpreted. | PR #103 (`21067dc`) materially resolved the doctrine/status conflict and explicitly deferred runtime, encoding, eligibility, and identity details. | owner doctrine decision |
| A-011 / RQ-015 | Dependency, CI, and release validation inputs remain insufficiently pinned/reviewed for semantic reproducibility. | Confirmed | process/release debt | `.github/workflows/ci.yml:13-18,23,52-60` uses moving toolchain/action/Python inputs; `Cargo.lock` pins Rust dependency resolution but not the external verifier environment; the release contract expects both verifier fixtures. | No CI or dependency change. | CI/release assurance |
| A-012 / RQ-010 | A public episodic path can still drop same-named tables on schema mismatch without an ownership marker. | Confirmed | conditional; contained to derived storage | `EpisodicView::at_path` and its `user_version` mismatch branch remain at `crates/magpie-episodic/src/lib.rs:100-120`. | No storage-safety change. | storage/resource contract |
| A-013 | ADR-0001 still names a `seam-slice.patch` that is not present as a current repository artifact. | Confirmed | documentation-only remainder | `docs/adr/0001-deadbolt-seam.md:116` names the patch; `Test-Path seam-slice.patch` is false. Landed tag-5 sources and fixtures are present, but the reference is not traceable to a file. | No traceability correction. | ADR traceability / documentation maintenance |
| A-014 | Segment-anchor validation checks the witness-root lexical form but not all remaining identity-field bounds at the Magpie seam. | Confirmed | conditional; contained seam gap | `Payload::validate` and `deadbolt_context` remain at `crates/magpie-log/src/event.rs:118-133` and `crates/magpie-claims/src/deadbolt_context.rs:249-279`; later exact metadata matching contains malformed markers. | No format or seam-runtime change. | Deadbolt–Magpie seam contract |
| A-015 / A-017 / RQ-003 | Public legacy/raw standing and mutable compatibility projections can still look like governed living status. | Confirmed | compatibility-only; narrowed by doctrine clarification; high | Public `standing` and `legacy_raw_standing` remain in `crates/magpie-claims/src/standing.rs:76-108,273-349,688`; the quarantine test still demonstrates raw `Settled` beside governed `Conjectured` (`crates/magpie-claims/src/standing.rs:1840-1875`). | PR #103 clarified that standing is not currentness and that legacy status is compatibility-only in accepted ADR text, but changed no public fields or construction paths. | compatibility API |
| A-016 | Withdrawal target doctrine is more exact, but identity/delegation and withdrawal runtime remain unimplemented. | Future implementation gate | narrowed; identity/delegation gate | ADR-0005 now requires exactly one prior attributed assertion act and immutable/replayable targeting (`docs/adr/0005-claim-withdrawal-acts.md:38-91`), while attribution is still asserted data and delegation is deferred; no withdrawal event/runtime exists. | PR #103 (`21067dc`) narrowed the target law and accepted ADR-0005 without adding representation or runtime. | identity/withdrawal doctrine |
| A-018 / RQ-013 | Passing tests do not close the missing negative, differential, resource, crash, and exact compile-fail assurance families. | Confirmed | assurance gap; not a test-green closure | Current compile-fail examples include stale-arity patterns (`crates/magpie-claims/src/origin_binding_verifier.rs:40-69`, `crates/magpie-claims/src/origin_admission_audit.rs:92-207`, `crates/magpie-claims/src/support_contribution_audit.rs:86-192`). Current tests pass, but no cross-language malformed corpus, base-log resource, crash, concurrency, or SQLite-failure family was found. | No test/code change. | CI/release assurance |
| A-019 / RQ-011 | Current public legacy resolution, episodic row, and search-result APIs remain detachable and coordinate-poor. | Confirmed | contained current provenance/API gap; compatibility-only; future authority-bearing integration gate | `StandingResolution` lacks prefix/closure identity (`crates/magpie-claims/src/standing.rs:76-97`); public `EpisodicEvent` rows omit complete record identity, verified-prefix, projection, and query coordinates, and `search` returns bare sequence numbers (`crates/magpie-episodic/src/lib.rs:41-56,154-177`). No librarian or MCP runtime exists on current `main`, but the implemented public API deficiency is current. | No query/API change. | provenance-response contract |
| A-020 | Ignored pending delegation tickets can still describe obsolete bases and no wrapper preflight proves current status/allowlist. | Confirmed | mixed-scope: local-only stale-ticket evidence; repository-level wrapper gap; confirmed in the recorded `C:\magpie` checkout | `.agent-runs/pending/0003-migrate-to-v1-format-freeze-layout.md` and `.agent-runs/pending/0006-episodic-schema-version-guard.md` are ignored local files and are absent from a fresh clone of GitHub `main`. Tracked repository code `scripts/codex-delegate.ps1` accepts a caller-supplied existing ticket path without validating ticket root, current status, expected base, or an allowed change surface. | No local ticket-content or tracked wrapper change; ignored ticket files are not repository authority and were not edited. | local delegation/worktree workflow |
| A-021 | Weak external Ed25519 roots and strict verification remain an unpinned profile decision. | Confirmed | conditional; high; trust-root/profile | `docs/FORMAT.md:160-170`, `crates/magpie-log/src/logimpl.rs:147-150,337-343`, and `tools/verify_chain.py:188-221` still leave ordinary versus strict weak-root handling open. The trust root remains externally supplied. | No cryptographic or format change; PR #103/104 did not address it. | verifier/tooling contract |
| A-022 | Admission rejection representation, historical recording, provenance, and retry semantics remain unanswered before a write gate. | Future implementation gate | constitutional; future admission gate | `docs/design/admission-design-readiness-checklist.md:159-165` and the admission question/record contracts retain unanswered rejection and retry choices; no generic admission or `EpistemicGate` runtime exists. | No admission or event-format change. | future admission ADR |
| A-023 | Direct artifact/origin matched traces still omit producing verified-prefix and closure identities. | Confirmed | contained; compatibility/API remainder | Direct trace/receipt surfaces remain in `crates/magpie-claims/src/artifact_provenance_verifier.rs:144-209,219-299,368-387` and `crates/magpie-claims/src/origin_binding_verifier.rs:239-259,360-389,541-548`; stronger v3/v4/audit paths carry more coordinates. | No provenance-response API change. | provenance-response contract |
| A-024 | Claimant-controlled authority labels can still manufacture corroboration separation. | Confirmed | constitutional; high; distinct authority finding | `crates/magpie-claims/src/origin_binding_verifier.rs:1819-1842` validates protocol spelling, not signer/credential/root binding; `crates/magpie-claims/src/origin_admission_audit.rs:232-236,559-601,643-694` copies claimant groups into the fold; `crates/magpie-claims/src/standing_v3.rs:801-819,835-853` can count them. ADR-0006's evidence-only invalidation boundary does not bind origin authority. | PR #103 changed currentness/invalidation doctrine only; no authority-binding code, tests, or contract landed. | authority/provenance doctrine, then origin-admission and standing-v3 runtime |
| RQ-012 | Explicit versioned policy/parser duplication continues to create review and drift pressure. | Confirmed | review-cost debt; no authority closure claimed | Separate policy/parser owners remain in `crates/magpie-claims/src/standing_v2.rs::combine_v2_standing`, `standing_v3.rs::combine_standing`, `origin_binding_verifier.rs::parse_binding_envelope`, and `support_contribution_audit.rs::resolve_support_contribution_audit_v0`; current tests cover present surfaces but no cross-version inventory check was found. | No refactor or inventory change. | claims maintainability / private mechanical helpers |

## 7. Individual-ID coverage matrix

This table is the mechanical coverage record. It has exactly one row for each
finding ID; family labels do not create additional findings.

| ID | Audit title or concise finding | Convergence family | Disposition | Current evidence note |
| --- | --- | --- | --- | --- |
| A-001 | Public backend append bypasses declared sole writer | F01 | Confirmed | `LogStore::append_record` remains public. |
| A-002 | ADR, contract, and living-status precedence is not closed | F02 | Partially remediated | ADR stack status was corrected; wider stale-status corpus remains. |
| A-003 | Detached v0-v2 standing output lacks verified-prefix coordinates | F03 | Confirmed | Legacy resolution shapes remain coordinate-poor. |
| A-004 | Rust and Python accept different record languages | F04 | Confirmed | Case, status, blank-line, and first-failure grammar remains divergent. |
| A-005 | v2/v3 wildcard status matches can promote future variants | F05 | Confirmed | Wildcard arms remain in v2/v3. |
| A-006 | Episodic projection synthesizes `Conjectured` for statusless tag 6 | F06 | Confirmed | Tag-6 mapping still emits a status. |
| A-007 | Replay freshness and projection failure semantics are caller conventions | F07 | Confirmed | Reuse and infallible projection paths remain. |
| A-008 | File durability, snapshot semantics, concurrency, and L0 resources are incomplete | F08 | Confirmed | Storage and outer resource law remain unspecified. |
| A-009 | Public unverified projection APIs can look trusted | F09 | Confirmed | Same public event type feeds raw and verified paths. |
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
| A-024 | Claimant-controlled authority labels manufacture corroboration separation | F23 | Confirmed | No independent authority binding is present. |
| RQ-001 | Public backend append bypasses declared sole writer | F01 | Confirmed | Same public `LogStore` seam as A-001. |
| RQ-002 | Unverified and verified events share projection-compatible type | F09 | Confirmed | `SignedEvent` remains the shared input type. |
| RQ-003 | Legacy raw standing remains public and authority-looking | F15 | Confirmed | Raw field remains public and serializable. |
| RQ-004 | Episodic projection invents `Conjectured` | F06 | Confirmed | Typed assertion row still carries synthetic status. |
| RQ-005 | v2/v3 wildcard matches can promote future variants | F05 | Confirmed | Latent extension defect remains. |
| RQ-006 | Rust and Python accept different record languages | F04 | Confirmed | Current golden intersection does not prove grammar parity. |
| RQ-007 | Base log verification has no file/record resource bound | F08 | Confirmed | Whole-file and whole-record materialization remains. |
| RQ-008 | FileStore lacks durability/atomicity/locking/snapshot contract | F08 | Confirmed | Two-write append has no declared crash/concurrency semantics. |
| RQ-009 | Infallible projection permits SQLite/range panics | F07 | Confirmed | `expect` and range panic paths remain. |
| RQ-010 | Episodic schema reset can drop tables arbitrarily | F12 | Confirmed | Public schema mismatch reset remains conditional hazard. |
| RQ-011 | Detached legacy resolutions/search results lack coordinates | F18 | Confirmed | Current public detachable outputs are coordinate-poor; future query/response use remains gated. |
| RQ-012 | Versioned policy/parser duplication creates drift pressure | F24 | Confirmed | Review-cost debt remains without an inventory gate. |
| RQ-013 | Test suite lacks cross-language/resource/crash/exact-boundary assurance | F17 | Confirmed | Green current tests do not cover listed failure families. |
| RQ-014 | Design-document runtime status drifted from landed code | F02 | Partially remediated | PR #103 corrected part of the status corpus; stale handoffs remain. |
| RQ-015 | CI toolchain/actions/Python crypto dependency are not pinned | F11 | Confirmed | Moving validation inputs remain. |

## 8. Materially changed findings

The following records capture every primary disposition marked Partially
remediated and the findings materially narrowed by PR #103. None is Closed:
the merged change was documentation-only, and no runtime, interface, test, or
format closure evidence was added.

### A-002 / RQ-014 — status and runtime handoff reconciliation

- Original audit conclusion: ADR/status precedence was not closed; several
  landed narrow runtimes were still described as future or candidate work.
- Exact subsequent change: PR #103 merged as
  `21067dcfe00175430fe751d669f136a182f6989b`. It changed ADR-0002 through
  ADR-0006 and related README/design status text, setting ADR-0002 through
  ADR-0006 to Accepted and recording the 2026-08-02 ratification.
- Resolved portion: the former ADR header conflict was corrected; tags 6-8
  were described as frozen representation; standing was explicitly separated
  from currentness; supersession, invalidation, withdrawal, and ratification
  were assigned consequences by their selected facets rather than by edge
  presence.
- Remaining portion: `standing-policy-v3`, `origin-admission-audit-v0`, and
  `inline-predicate-attestation-binding-v0` still expose future/candidate
  handoffs even though corresponding narrow runtime paths exist. Bootstrap,
  visual-ledger, ticket, and mixed-status materials still require an explicit
  status/precedence reading. No code status changed.
- Disposition basis: Partially remediated is correct because a material
  doctrine/status portion changed, while the broader living-status defect and
  its implementation confusion remain. PR #104's audit index is historical
  evidence, not closure.

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

## 9. Confirmed constitutional and P0 boundaries

These are the highest-priority surviving boundaries. This section records why
they remain open and the owning layer; it does not design a fix.

- **Claimant-controlled corroboration separation — A-024.** The binding
  parser checks protocol-shaped labels, while the origin-admission fold copies
  claimant-selected groups and the v3 policy counts distinct groups. No
  independent signer, credential, root, or authority coordinate binds the
  labels. The next decision belongs to authority/provenance doctrine, followed
  by the origin-admission and standing-v3 runtime layers.

- **Public backend append capability — A-001/RQ-001.** `LogStore` still
  exposes `append_record` to any mutable store holder even though `LogWriter`
  is the declared sole writer. Verification catches bad history after the
  capability is used; it does not seal the capability. The next decision
  belongs to the `magpie-log` API/storage layer.

- **Unverified versus verified projection inputs — A-009/RQ-002.**
  `LogReader::events()` and verifier-created replay still use
  projection-compatible `SignedEvent` values. Documentation and passing
  replay tests do not prevent a caller from applying raw events. The next
  decision belongs to the `magpie-log` replay/projection API.

- **Rust/Python accepted-language divergence — A-004/RQ-006.** Rust and
  Python still disagree on hex case, status representation, blank lines, and
  JSON/storage behavior around the shared record boundary. The next decision
  belongs to the verifier/tooling contract and cross-language vector owner.

- **Weak-root/strict-verification ambiguity — A-021.** The external trust root
  remains caller-supplied, but v1 does not select one weak-key acceptance rule
  shared by Rust and Python. The next decision belongs to the crypto/format
  compatibility owner.

- **Future status wildcard promotion — A-005/RQ-005.** v2/v3 positive
  wildcard arms can assign future enum values positive meaning without an
  explicit policy arm. Current tests pass only because the current vocabulary
  has no such value. The next decision belongs to versioned standing policy
  code and tests before any status extension.

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

1. authority/corroboration decision;
2. append-capability sealing;
3. verified replay/projection boundary;
4. cross-language verifier contract and vectors;
5. small semantic corrections;
6. provenance-complete response surfaces;
7. storage/resource/fallible-projection contracts;
8. future admission, currentness, withdrawal, identity, and authority-bearing
   query capabilities.

The order expresses dependency pressure visible in the evidence: authority
decisions precede authority-bearing use; replay and grammar boundaries precede
portable derived interpretation; resource and failure contracts precede
durable local deployment. It does not choose mechanisms or owners.

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
git remote -v
git branch --show-current
git rev-parse HEAD
git rev-parse origin/main
git merge-base HEAD origin/main
git merge-base 3b46fdb81857d77b827271777b86745bca5f7b12 HEAD
git status --short --untracked-files=all
git diff --name-only
git diff --cached --name-only
git ls-files --others --exclude-standard
git ls-remote origin refs/heads/main
git log --oneline --decorate --reverse 3b46fdb81857d77b827271777b86745bca5f7b12..HEAD
git diff --stat 3b46fdb81857d77b827271777b86745bca5f7b12 HEAD
git diff --name-status 3b46fdb81857d77b827271777b86745bca5f7b12 HEAD
rg -n "^### (A|RQ)-[0-9]{3}\b" docs/audits/magpie-architecture-audit-2026-08-01.md docs/audits/magpie-runtime-quality-audit-2026-08-04.md
cargo test --workspace --locked --no-fail-fast
```

The current test command passed with 281 tests passed and zero failed, and the
`magpie-log` compile-fail doctest group also passed with 3 tests. This is
current test evidence, not closure evidence: the missing hostile/resource/
crash/differential cases remain recorded above.

The final check also used `git diff --check`, a changed-path allowlist audit,
post-edit SHA256 hashes for both historical audit files, and this one-off
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
$other = @($familyPrimary | Where-Object {
  $_.Disposition -notin
    @('Confirmed', 'Partially remediated', 'Future implementation gate')
}).Count
$futureIds = @($familyPrimary |
  Where-Object Disposition -eq 'Future implementation gate' |
  Select-Object -ExpandProperty Id | Sort-Object)
$statusMismatch = @($expected |
  Where-Object { $familyDisposition[$_] -ne $matrixDisposition[$_] })
if ($confirmed -ne 34 -or $partial -ne 3 -or $future -ne 2 -or
    $other -ne 0 -or ($futureIds -join ',') -ne 'A-016,A-022' -or
    $familyDisposition['A-019'] -ne 'Confirmed' -or
    $familyDisposition['RQ-011'] -ne 'Confirmed' -or
    $familyDisposition['A-010'] -ne 'Partially remediated' -or
    $familyDisposition['A-024'] -ne 'Confirmed' -or
    $statusMismatch.Count) { throw 'primary disposition check failed' }
'dispositions: PASS; Confirmed=34, Partially remediated=3, Future implementation gate=2'
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
- final allowed changed paths: `docs/audits/audit-disposition-2026-08.md` and
  `docs/audits/README.md` only;
- `git diff --check`: pass;
- final worktree status: recorded in the completion report; no commit, push,
  branch, PR, issue, or Git-history mutation was performed.
