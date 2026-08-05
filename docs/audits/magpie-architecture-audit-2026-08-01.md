# Magpie Principal Architecture Audit — 2026-08-01

## Audit ledger

This document is the durable, evidence-backed ledger for the audit point below.
It records confirmed, rejected, deferred, and conditional conclusions without
ratifying doctrine or authorizing implementation.

## Audit record

| Field | Value |
| --- | --- |
| Audit point | `main` at `3b46fdb81857d77b827271777b86745bca5f7b12` |
| Repository | `noctem-o/magpie` |
| Worktree | `C:\magpie`, clean of pre-existing changes; this audit artifact is the sole untracked file |
| Mode | Read-only architecture audit; no runtime, format, policy, or Git-history changes |
| Active PR | PR #103, documentation-only ADR-0006 candidate; non-draft, no formal approval, one unresolved doctrine comment |
| Overall result | Coherent narrow provenance-first kernel; not constitutionally closed |

This ledger is durable audit evidence, not ratified doctrine. It does not
authorize a merge, release, policy change, new vocabulary, or implementation.

## Executive judgment

The implemented L0, verified replay, closed vocabularies, exact policy v0–v4
lanes, provenance/closure audits, and rebuildable projections form a coherent
architecture for the narrow capabilities they claim.

The complete architecture is not yet closed. The principal unresolved
boundaries are:

1. public raw storage mutation alongside the declared `LogWriter` append boundary;
2. incomplete file durability, snapshot semantics, concurrency, and L0 resource bounds;
3. contradictory document status and precedence;
4. incomplete coordinates on detached v0–v2 standing outputs;
5. Rust/Python verifier-language drift;
6. wildcard status promotion in v2/v3;
7. synthetic status in the episodic projection;
8. lifecycle/currentness doctrine and replay-substrate gaps; and
9. public v0–v2 resolution values that can fabricate currentness-looking
   output without a verified snapshot;
10. actor-local withdrawal semantics without an identity coordinate;
11. legacy public projection fields that expose raw status with
     authority-looking names;
12. compile-fail boundary tests that do not always fail for the forbidden
     surface itself; and
13. detached episodic/query outputs that omit the record, snapshot, and query
     identity required by the provenance-response doctrine; and
14. an ignored delegation queue containing pending tickets whose preconditions
     describe obsolete repository states; and
15. Ed25519 weak-key acceptance and strictness are not pinned at the trust-root
    boundary; and
16. admission representation, rejection recording, and retry-amplification
    semantics remain unanswered future-write gates; and
17. direct artifact/origin matched traces omit the verified-prefix and closure
    identities required to detach them safely as provenance responses; and
18. the claimant-controlled origin authority pair can create corroboration
    separation without an independently verified authority binding.

No confirmed Critical finding was established: ordinary replay fails closed for
malformed records under a normal supplied root, while A-021 is conditional on
accepting a weak external root. Constitutional findings remain open, so the
audit has not reached the requested provisional-closure condition.

## Governing-corpus disposition

- ADR-0001 is explicitly Accepted.
- ADR-0002 through ADR-0005 remain marked Proposed while later contracts and
  runtime code treat substantial portions as governing.
- `docs/FORMAT.md` is the normative canonical-format source.
- Policy-specific documents and tickets govern their narrow landed slices.
- `docs/design/epistemic-boundary-map.md` and
  `docs/design/epistemic-invariant-ledger.md` are reference maps, not authorities.
- `magpie-bootstrap.md`, `CLAUDE.md`, and `docs/magpie-architecture-ledger.html`
  are bootstrap/worker/visual artifacts with no explicit governing status; their
  stronger gate, bi-temporal, and build-now language is included in A-002 and
  must not authorize implementation.
- `CLAUDE.local.md` is a local personal-preference file, not an architecture
  or governance source; its delegation/model preferences cannot amend the
  worker or doctrine boundaries.
- `docs/design/deadbolt-occurrence-context-v0.md` has a follow-up status, but
  `deadbolt-occurrence-standing-v1.md` and
  `standing-contribution-lanes-v1.md` have no top-level status block even though
  they describe landed policy/control-flow behavior; `governed-standing-tour-v1.md`
  is an executable release contract rather than doctrine. Their runtime claims
  match the narrow implementation, but the corpus still needs explicit status
  and precedence labels before a reader treats any of them as ratified authority.
- Admission, AgentProposer, MCP, librarian, and generic resolver documents remain
  proposed or exploratory and have no runtime implementation.

The status vocabulary and precedence require an owner-approved registry or
equivalent corpus correction. A README or map must not silently resolve a
conflict between ADRs and ratified policy contracts.

## System map

The system map below separates information flow from authority flow. A value
may be carried downstream without granting the receiving component authority
to reinterpret it or to write history.

## Authority-flow map

```text
external material
    -> future governed admission boundary
    -> LogWriter
    -> signed append-only L0 history
    -> complete verified replay snapshot
    -> policy-specific interpretation
    -> typed standing / audit / projection output
```

The main bypass is:

```text
public LogStore::append_record
    -> raw stored bytes without LogWriter
```

The main provenance gap is:

```text
v0-v2 standing output
    -> policy result without explicit verified-prefix identity
```

The narrow direct-trace gap is separate:

```text
matched artifact/origin trace
    -> receipt without the producing verified-prefix or closure identity
```

The new authority-flow gap is separate from both receipt carriage and exact
label selection:

```text
claimant-controlled authority pair and origin group
    -> exact-label origin-admission fold
    -> distinct origin groups
    -> v3 corroboration count
    -> `Supported` without an independently verified authority binding
```

## Confirmed findings

The following findings are confirmed against the audited repository state. They
are ordered by architectural severity, with conditional findings marked as
such; the detailed records below preserve the concrete failure scenarios and
correction loci.

## Findings ledger

| ID | Finding | Category | Severity | Primary source references | Minimal remediation | Status / correction locus |
| --- | --- | --- | --- | --- | --- | --- |
| A-001 | Public `LogStore::append_record` bypasses the declared append boundary | Authority/API | Constitutional | `docs/adr/0001-deadbolt-seam.md:9-10,56-62`; `crates/magpie-log/src/store.rs:12-14,31-38,80-86`; `crates/magpie-log/src/lib.rs:7-14`; `crates/magpie-log/src/logimpl.rs:223-231` | Seal or hide the mutating backend capability; expose only a writer-owned append path and a read interface | Confirmed; separate code/API PR |
| A-002 | ADR, contract, and living-status precedence is not closed | Governance/compatibility | Constitutional | `docs/adr/0002-governed-claim-memory.md:3,203-211`; `docs/design/mcp-boundary-contract.md:3-8`; `docs/design/mcp-librarian-contract.md:3-16`; `docs/design/origin-admission-audit-v0.md:3-7`; `README.md:291-313`; `magpie-bootstrap.md:23-35,47-64`; `docs/magpie-architecture-ledger.html:286-317,340-354,384-396` | Owner-approved status/precedence registry; mark historical/aspirational text explicitly | Confirmed; separate docs/governance PR |
| A-005 | v2/v3 wildcard status matches can silently promote future enum variants | Policy compatibility | Constitutional | `crates/magpie-claims/src/standing_v2.rs:300-305`; `crates/magpie-claims/src/standing_v3.rs:885-891`; exhaustive pattern in `crates/magpie-claims/src/standing_v4.rs:1168-1195` | Replace promotion wildcards with explicit arms before any status extension | Confirmed; code PR |
| A-010 | Currentness has material doctrine and substrate divergence | Lifecycle/resolver | Constitutional | `docs/adr/0003-canonical-definition-of-standing.md:73-74`; `docs/adr/0004-claim-lifecycle-facets.md:68-73`; `docs/adr/0005-claim-withdrawal-acts.md:102-137`; `crates/magpie-claims/src/standing.rs:27-30`; `crates/magpie-claims/src/replay_snapshot.rs:22-28` | Future ADR must settle target, order, acts, scope, actors, cycles, output, and coordinates | Confirmed blocker; owner decision/future ADR |
| A-022 | Admission rejection, historical representation, and retry semantics are not governed | Admission/replay/governance | Constitutional (future readiness) | `docs/design/admission-design-readiness-checklist.md:159-165` (§5); `docs/design/admission-boundary-questions.md` §5; `docs/design/admission-boundary-minimal-semantics.md` §9-10; `docs/design/admission-record-boundary-contract.md` §7-8 | Future owner ADR/contract must answer the historical object, rejection-recording, retry/idempotence, provenance, and failure semantics before any write mechanism | Explicitly deferred; blocks future admission/EpistemicGate implementation |
| A-024 | Claimant-controlled origin authority labels can manufacture corroboration separation | Provenance/evidence/authority | Constitutional | `docs/design/origin-admission-audit-v0.md:29-52,119-151,301-356,459-470,927-945`; `docs/design/origin-binding-bundle-v0.md:463-489`; `docs/design/standing-aggregation-independence-groups.md:385-413,472-476,702`; `crates/magpie-claims/src/origin_binding_verifier.rs:1819-1842`; `crates/magpie-claims/src/origin_admission_audit.rs:232-236,559-601,643-694`; `crates/magpie-claims/src/standing_v3.rs:801-819,835-853`; `crates/magpie-claims/tests/origin_admission_audit.rs:37-38,568-625,787-798`; `crates/magpie-claims/tests/standing_v3.rs:20-25,359-365,412-445` | Before corroboration is authority-bearing, require an independently verifiable authority binding or explicitly keep claimant-controlled group assignments non-corrobating; pin the authority/root/profile coordinate | Confirmed; future authority/provenance ADR and code/test gate |
| A-003 | Detached v0–v2 standing output lacks complete verified-prefix coordinates | Replay/provenance | High | `docs/adr/0003-canonical-definition-of-standing.md:41-44`; `crates/magpie-claims/src/replay_snapshot.rs:22-63`; `docs/design/origin-admission-audit-v0.md:519-521` | Add an additive coordinate-bearing response wrapper; retain old output as compatibility-only | Confirmed; future API contract |
| A-004 | Rust and Python verifiers accept different JSONL/storage languages | Canonical/tooling | High | `crates/magpie-log/src/hashing.rs:48-58`; `crates/magpie-log/src/event.rs:315-324`; `tools/verify_chain.py:50-64,214-230`; `docs/FORMAT.md:188-193` | Choose one grammar and add differential malformed-input fixtures | Confirmed; code/tooling PR |
| A-006 | Episodic projection synthesizes `Conjectured` for a typed event that did not record status | Projection/provenance | High | `crates/magpie-log/src/event.rs:83-92`; `crates/magpie-episodic/src/lib.rs:65-85,328-338` | Preserve `None` or use an explicitly derived field; add a typed-assertion regression | Confirmed; code PR |
| A-021 | Ed25519 weak-key acceptance and strictness are not pinned at the trust-root boundary | Historical integrity/cryptography | High conditional | `docs/FORMAT.md:160-170`; `crates/magpie-log/src/logimpl.rs:147-150,337-343`; `tools/verify_chain.py:188-193,207-221`; `Cargo.lock:125-136` | Ratify v1 weak-key/strict-verification behavior, reject or explicitly gate weak external roots, and add cross-tool hostile vectors; preserve compatibility or use a new format/profile if accepted-language changes | Confirmed conditional; separate crypto/format PR |
| A-023 | Direct artifact/origin matched traces omit producing prefix and closure identities | API/provenance/replay | Medium | `crates/magpie-claims/src/artifact_provenance_verifier.rs:144-209,219-299,368-387`; `crates/magpie-claims/src/origin_binding_verifier.rs:239-259,360-389,541-548`; `docs/design/provenance-response-contract.md:35-49,97-114` | Add a coordinate-bearing wrapper for detached trace responses, or keep these traces explicitly non-detachable/internal; preserve receipt shapes as audit-only compatibility values | Confirmed, contained; future provenance/API contract |
| A-007 | Replay freshness and projection failure semantics are caller conventions | Replay/availability | Medium | `crates/magpie-log/src/logimpl.rs:417-421`; `crates/magpie-episodic/src/lib.rs:213-223,390-398` | Define fresh/rebuild preconditions and typed boundary failures; avoid hidden reset | Confirmed; future code PR |
| A-008 | File durability, snapshot semantics, concurrency, and L0 resource bounds are incomplete | Availability/durability | Medium | `crates/magpie-log/src/store.rs:9-14,17,30-50`; `crates/magpie-log/src/event.rs:83-115`; `crates/magpie-log/src/logimpl.rs:31-34,108-196`; `crates/magpie-claims/src/deterministic_verifier_context.rs:247-264,328-373,394-414`; `crates/magpie-claims/src/deadbolt_context.rs:198-245,298-367`; `docs/FORMAT.md:139-143`; `README.md:117-121`; `docs/releases/v0.1.0-contract.md:75-76`; `docs/design/historical-review-remediation-ledger.md:265-271` | Govern single-writer, sync, framing, append-order/complete/atomic snapshot semantics, intentional-prefix selection, streaming, bounded metadata, recursion, and pre-materialization limits | Confirmed; deferred storage/resource contract |
| A-009 | Public unverified projection APIs can be mistaken for trusted replay inputs | API/provenance | Medium | `crates/magpie-log/src/logimpl.rs:345-355,417-421`; `crates/magpie-claims/src/deadbolt_context.rs:41-56,119-245` | Opaque verified input or unmistakably untrusted API; preserve compatibility deliberately | Confirmed, contained; future API review |
| A-011 | Dependency and review gates do not fully protect governed semantics | Workflow/supply chain | Medium | `Cargo.toml:11-24`; `.github/workflows/ci.yml:33-62`; `docs/releases/v0.1.0-contract.md:447-450` | Require semantic dependency review, security audit, differential vector checks, and the release-contract Deadbolt fixture gate | Confirmed process debt |
| A-012 | Arbitrary SQLite path can trigger destructive derived-index migration | Security/availability | Medium conditional | `crates/magpie-episodic/src/lib.rs:96-117` | Require an owned database marker or non-destructive mismatch handling; do not add hidden database authority | Confirmed conditional; separate security/API contract |
| A-014 | Anchor payload validates the root but not the remaining identity fields | Deadbolt seam/validation | Medium conditional | `crates/magpie-log/src/event.rs:89-94,118-133`; `crates/magpie-claims/src/deadbolt_context.rs:249-279` | Ratify non-empty/size rules at the seam or require the anchorer to enforce them before append; retain exact-match fail-closed behavior | Confirmed contained seam gap; separate contract/code PR |
| A-015 | Public v0–v2 resolution/application values can fabricate authority-looking output | API/provenance/compatibility | Medium | `crates/magpie-claims/src/deadbolt_context.rs:20-39`; `crates/magpie-claims/src/standing.rs:63-84`; `crates/magpie-claims/src/standing_v1.rs:32-132`; `crates/magpie-claims/src/standing_v2.rs:44-145`; `crates/magpie-claims/src/lib.rs:117-132` | Quarantine or make derived resolution/application construction opaque; add compile-fail construction tests and preserve old shapes as explicitly untrusted compatibility values | Confirmed, contained; separate API/compatibility PR |
| A-016 | Withdrawal self-ownership has no enforceable identity semantics | Lifecycle/identity/authority | Medium conditional | `docs/adr/0005-claim-withdrawal-acts.md:67-91,126-138`; `crates/magpie-log/src/event.rs:202-235` | Defer self-ownership or pin a non-ambient identity/authority coordinate before any withdrawal runtime; preserve asserted attribution as non-authenticating | Confirmed future-governance gap; future ADR/identity contract |
| A-017 | Public legacy projections expose raw status with authority-looking semantics | API/compatibility/epistemic | Medium | `crates/magpie-claims/src/lib.rs:154-232`; `crates/magpie-claims/src/standing.rs:181-295,661-778`; `docs/releases/v0.1.0-contract.md:91-98,150-157,374-379` | Keep legacy shapes frozen but quarantine names/accessors and require governed resolver outputs for authority-bearing consumers | Confirmed, contained; separate compatibility/API PR or audit ledger |
| A-018 | Several compile-fail doctests fail for stale API shapes, not the intended authority boundary | Test/enforcement | Medium | `crates/magpie-claims/src/origin_binding_verifier.rs:43-70`; `crates/magpie-claims/src/origin_admission_audit.rs:80-160`; `crates/magpie-claims/src/support_contribution_audit.rs:74-169`; current resolver methods `crates/magpie-claims/src/origin_admission_replay.rs:151-169` | Replace stale-arity examples with snippets that directly attempt forbidden construction, deserialization, or authority substitution; retain compile-fail enforcement | Confirmed test gap; separate test/docs PR |
| A-019 | Public episodic rows and search results omit detached record, prefix, and query provenance | API/provenance/query | Medium | `crates/magpie-episodic/src/lib.rs:39-52,135-187,153-175,359-370,386-388`; `Cargo.toml:17-24`; `Cargo.lock:321-332`; `docs/design/provenance-response-contract.md:35-49,97-106`; `docs/design/verified-librarian-query-contract.md:38-40,88-99,127-138`; `crates/magpie-episodic/tests/episodic.rs:291-317` | Add an additive authority-bearing query/response wrapper carrying record/prefix identity, explicit query/projection semantics, and projection/query coordinates; retain `EpisodicEvent` and bare search as derived compatibility data | Confirmed, contained; future query/API contract |
| A-020 | Ignored delegation queue has stale tickets with obsolete base-state assumptions | Workflow/governance | Medium conditional | `.agent-runs/pending/0001-readme-dev-workflow.md:1-41`; `.agent-runs/pending/0002-compare-v1-freeze-snapshot.md:1-57`; `.agent-runs/pending/0003-migrate-to-v1-format-freeze-layout.md:1-79`; `.agent-runs/pending/0006-episodic-schema-version-guard.md:1-107`; `scripts/codex-delegate.ps1:1-55`; `.gitignore:1-5` | Require ticket retirement/status, exact-base and current-state preflight, and explicit ownership before invoking a pending ticket; never treat ignored queue state as doctrine | Confirmed workspace-only workflow debt; separate process/docs PR or audit ledger |
| A-013 | Accepted ADR-0001 names an absent implementation artifact | Governance/traceability | Low | `docs/adr/0001-deadbolt-seam.md:112-120`; repository file inventory contains no `seam-slice.patch` | Replace the absent filename with landed-file/commit references or add an explicitly archival artifact | Confirmed documentation debt; separate docs PR |

### A-001 details

Any caller with a `FileStore` or `MemStore` can invoke the public trait method
with arbitrary bytes. This can poison a durable log or bypass a future gate.
Verification rejects the resulting invalid chain, so this is not currently a
way to create accepted history without the signing key; it is nevertheless a
second raw append capability.

ADR-0001 describes Deadbolt as holding `LogWriter` as the sole L0 write
capability. The repository intentionally keeps `LogWriter` as a low-level
capability for the current Magpie-side slice, but the public backend mutation
seam is still independently reachable. The finding is that second seam, not a
demand that L0 absorb epistemic admission or that the Deadbolt seam become a
generic writer.

### A-002 details

The status conflict is broader than the four ADR headers. ADR-0002 still
labels tags 6–8 as `Provisional future event tags`, says they were “not added
by this change,” and says the format update is future
(`docs/adr/0002-governed-claim-memory.md:203-211,246,273`), while the normative
format already records tags 6–8 and the runtime validates and projects them.
The same family appears in living implementation handoffs:

| Corpus text | Current evidence | Concrete risk |
| --- | --- | --- |
| `docs/design/origin-admission-audit-v0.md:5-7` says `OriginAdmissionAuditV0` and exact-prefix/candidate enumeration remain future | `crates/magpie-claims/src/origin_admission_audit.rs` and `origin_admission_replay.rs:151-169` expose the landed audit path | An implementer can treat a landed replay/audit surface as unavailable or redesign it |
| `docs/design/standing-policy-v3-external-report-corroboration-v0.md:5-10,817-824` says runtime is not implemented and no runtime claim should appear | `crates/magpie-claims/src/standing_v3.rs` and `origin_admission_replay.rs:173-193` provide the runtime path | A completed policy is mistaken for a future slice, while its remaining broader aggregation deferral is obscured |
| `docs/design/standing-view-evidence-ceilings.md:248-253` says the Ticket 0064 v4 runtime remains future | `crates/magpie-claims/src/standing_v4.rs` and its vectors implement the subject-bound v4 rule | The living ceiling note contradicts the current policy boundary |
| `docs/design/claim-inline-subject-binding-v0.md:593` and `inline-predicate-attestation-binding-v0.md:9,1007` call Ticket 0063 a current candidate patch | Ticket 0063/PR #78 is recorded as landed, and `claim-inline-predicate-edge-lanes-v0.md:5-9` says so | Reviewers can repeat a completed implementation or misclassify its standing-inert boundary |
| `tickets/0061-inline-predicate-attestation-binding-implementation.md:3-15` and `tickets/0063-claim-inline-predicate-edge-lanes-implementation.md:3-13` call the implementation an uncommitted candidate patch while also recording merged PRs #76/#77 | The corresponding runtime is present on `main`, and the same ticket records describe the merged bases and landed contract/runtime sequence | A reader can treat shipped code as an uncommitted candidate, rerun an already-landed slice, or infer that current checkout state is still part of the ticket's historical implementation branch |
| `docs/design/deadbolt-occurrence-standing-v1.md:1-11,34-46`, `standing-contribution-lanes-v1.md:1-17,37-69`, and `governed-standing-tour-v1.md:1-22,127-173` describe an exact achieved-standing policy, a landed control-flow refactor, and an executable release contract without a top-level status block | The corresponding v1 snapshot methods, private lane dispatch, and release example exist and are covered by the current tests; `deadbolt-occurrence-context-v0.md:208-213` supplies only the context note's follow-up status | A reader can treat a landed policy note as an unratified design, or treat an implementation/release artifact as a new doctrine source; the status/precedence boundary is not inspectable from the files themselves |
| `magpie-bootstrap.md:23-35,47-64,111-120` and `CLAUDE.md:87-120,142-163` present a bootstrap thesis, “locked” gate/write decisions, and a queued/landed history without a governing-status relationship to the ADR corpus | Current `FORMAT`, runtime, README, and later contracts have materially narrowed the gate, status, admission, and lifecycle claims; the public store seam also contradicts the asserted sole-write enforcement | A fresh implementer can treat bootstrap-era aspirations or worker guidance as current doctrine, or mistake historical “landed” statements for a ratified authority boundary |
| `docs/magpie-architecture-ledger.html:286-317,340-354,384-396,509-525` is titled an architecture ledger but has no status block and describes a verified Deadbolt gate as the only writer, bi-temporal validity intervals, and `supported → refuted` invalidation as build-now work | No such gate or currentness runtime exists; ADR-0004 forbids ambient time semantics and current doctrine keeps invalidation/supersession/currentness policy-deferred | The visual artifact can authorize a materially different implementation if read as the architecture source rather than as an unclassified historical/aspirational note |
| `README.md:291-313` lists “origin admission” under Implemented while ordinary governed admission and `EpistemicGate` remain under Future | Runtime exposes narrow origin/provenance audit and replay substrates, not a generic admission writer or gate | A reader can conflate a landed origin-admission audit with the deferred authority-bearing admission boundary; the README needs the same narrow qualifier used by the corpus map |
| `docs/design/adr-0002-governed-claim-memory-dossier.html:202-230,331-363` explicitly says `Status: Proposed` and `Docs-only companion`, but its future tag-6–8 rollout text remains a second handoff surface | `FORMAT` and runtime now contain tags 6–8 | This artifact is not itself a new conflict because its status is explicit, but it must be included in the corpus map so its historical future language cannot be mistaken for current absence |
| The admission exploration set refers to “ratified MCP contracts” and “six ratified boundary contracts,” while the MCP, capability, librarian, query, and provenance contracts themselves declare Proposed/docs-only status | Current status markers classify those interface contracts as unaccepted future boundaries; no MCP service exists | A future implementer can treat interface proposals as governing admission constraints, or treat the admission set as authorizing a write boundary that the proposed contracts have not ratified |

These are not all independent constitutional findings: they are one status,
precedence, and historical-label failure family. Frozen future-tense contract
body is not itself stale when the opening status or explicit “current” handoff
correctly identifies it as historical. The failure occurs where a living
reader is told both that a surface is implemented and that the same surface is
still future without a dated historical label.

The bootstrap and visual-ledger artifacts widen the same family: they are
architecturally assertive but do not identify whether they are governing,
historical, exploratory, or merely worker context. Their stronger gate,
bi-temporal, and invalidation language therefore cannot safely be treated as
implementation permission. The smallest correction remains a status/
precedence registry and explicit non-governing labels; it must not rewrite
historical bootstrap material, silently ratify a generic gate, or introduce
time-aware lifecycle semantics.

The concrete divergence is implementer-visible: implementation A follows
`FORMAT`/the ADRs and leaves generic admission, currentness, and Deadbolt
execution authority deferred; implementation B follows the bootstrap/visual
ledger and builds Deadbolt as the sole memory gate while wiring time-bounded
invalidation into the status spectrum. Both readings are available in the
repository, but they produce different authority and lifecycle systems under
the same checkout. No new gate, identity model, time semantics, or invalidation
runtime belongs in this audit's remediation.

Two plausible implementers therefore select different governing nodes: one
follows the document's explicit future language, while another follows the
runtime and README. The resulting policy or review scope diverges even before
code changes. The smallest correction is one owner-approved status/precedence
registry plus narrow status blocks and historical labels; it must not silently
ratify Proposed ADRs or rewrite frozen historical contract prose.

### A-003 details

`StandingReplaySnapshot::canonical_bytes()` is correctly documented as derived
comparison bytes rather than prefix identity. The problem is detached
authority-bearing output: v0–v2 resolutions do not independently identify the
verified tip, and no distinct resolver identity is exposed consistently. Two
logs can have equal derived projections and event counts but different tips.

### A-023 details

The public snapshot methods for artifact acquisition and direct derivation
return `ArtifactProvenanceContextTraceV0`, and the public origin-binding method
returns `OriginBindingContextTraceV0`. Their matched receipt variants retain
the exact selector, canonical bundle/profile data, artifact or contribution
identity, recomputed digest/root, and matching anchor occurrences. They do not
retain the `VerifiedLogPrefixIdentityV0` of the snapshot or the
`ResolutionContentClosureIdentityV0` from which the supplied bytes were
looked up. The stronger origin-admission, admitted-contribution, support, and
v3 standing audits do carry both identities; this finding is limited to the
direct trace/receipt surfaces.

That omission creates a concrete replay/provenance ambiguity. Let `H1` be a
verified prefix containing the matching anchor and let `H2` extend `H1` with
unrelated valid events. Let `M1` contain the required bundle/artifact bytes and
let `M2` contain the same required bytes plus an unrelated bounded object.
The direct matched receipt can serialize identically for `H1+M1` and
`H2+M2`, even though the declared deterministic inputs and the provenance
context differ. Implementation A retains the caller's live snapshot and
closure and interprets the trace as caller-scoped; implementation B forwards
only `canonical_bytes()` and treats the same receipt as a self-contained
verified provenance answer. Both behaviors are plausible because the output
itself does not carry the coordinates required by the provenance-response
contract.

The smallest safe remediation is additive: either return a coordinate-bearing
wrapper for detached trace responses, containing the exact verified-prefix and
closure identities plus the existing trace, or explicitly keep these direct
trace methods internal/non-detachable and document their caller-scoped audit
meaning. The existing receipt shapes should remain serialization-only audit
values unless a versioned response contract changes them. The remediation
must not make a receipt trusted authority, infer identity or source quality,
change artifact/origin policy, add a cache or loader, or make the trace feed
standing without the already-governed composition path. Correction belongs in
a future provenance/API contract, not the active PR and not a resolver
algorithm change.

### A-024 details

The origin-binding bundle makes `origin_group`, `authority.kind`, and
`authority.reference` claimant-controlled canonical content. The parser's
semantic checks require only the protocol-identifier grammar for those fields
(`crates/magpie-claims/src/origin_binding_verifier.rs:1819-1842`); they do not
bind the claimed authority to a signer, identity, credential, or independently
verified Deadbolt result. The bundle contract itself says that the authority
object merely asserts an authority path and contains no signer, certificate,
or trust root (`docs/design/origin-binding-bundle-v0.md:463-489`).

The origin-admission implementation then treats exact string equality with
`human-review` and `authority:origin-review-v0` as the trusted path and copies
the claimant's `origin_group` into the trusted fold
(`crates/magpie-claims/src/origin_admission_audit.rs:232-236,559-601`). The fold
partitions matched contributions by those strings and emits an admitted group
when the assignment is otherwise complete (`:643-694`). Standing v3 consumes
the resulting groups and produces `Supported` once the distinct-group threshold
is met (`crates/magpie-claims/src/standing_v3.rs:801-819,835-853`). The current
fixtures make this behavior concrete: they define the same exact authority pair
as test input and assert two claimant-selected groups reach `Supported`
(`crates/magpie-claims/tests/origin_admission_audit.rs:37-38,568-625,787-798`;
`crates/magpie-claims/tests/standing_v3.rs:20-25,359-365,412-445`).

A concrete hostile scenario is two canonical, correctly anchored origin-binding
bundles for two distinct external-report contributions in one exact claim and
scope. A low-authority producer chooses two fresh group strings and copies the
exact human-review authority pair into both bundles. With valid artifact
provenance and no independently verified reviewer/authority binding, the
current path can classify both as trusted matched assignments, count two
groups, and derive `Supported`. The actor has manufactured the very
corroboration separation that the standing-aggregation contract says a
self-declared group must not create. Foreign bundle byte verification and
same-replay anchor occurrence do not authenticate the authority claim; they
prove bytes and occurrence only.

Two plausible implementations remain compliant-looking under the current
wording. Implementation A is the current exact-label implementation: the
compiled policy is treated as sufficient authority and claimant group strings
are admitted. Implementation B requires a separate immutable authority
binding/verification coordinate and leaves an unbound claimant group
unresolved or non-corrobating. The missing boundary is not merely an identity
cosmetic: it changes whether the same `(H, P, M)` input can produce a
corroborated `Supported` result.

The smallest safe correction is an owner-ratified authority/provenance
contract that either binds the selected authority path to an independently
verifiable credential/root/profile or explicitly prevents claimant-controlled
groups from entering corroboration until that binding exists. The correction
must not add a caller-provided `trusted` boolean, a reputation score, source
ranking, generic trust registry, new event/tag/actor vocabulary, or infer
identity from an anchor occurrence. This belongs in a future authority/
provenance ADR and code/test gate, not PR #103 and not a generic admission
implementation.

### A-004 details

The committed golden chain is accepted by both implementations, but their
accepted storage languages and first-failure behavior differ. Rust's `Sig` and
`ContentHash` deserializers use `hex::decode` and therefore accept mixed-case
hex, while `tools/verify_chain.py` requires lowercase. Rust `FileStore` skips
blank lines, while the Python verifier parses a blank line as an error; an
inserted empty line is therefore omitted by Rust before sequence accounting but
is an explicit failure in Python. Rust can verify an empty `MemStore` summary,
while the Python CLI rejects an empty
file because it has no genesis. The Python verifier also accepts numeric
status tags in JSON while Rust Serde accepts the named `Status` variants.
Python's default `json.loads` also accepts non-standard constants such as
`NaN`, while Rust Serde rejects them. This widens the accepted-language
divergence beyond spelling and blank-line policy.

There is a second divergence for a deliberately signed but semantically
malformed typed payload: Rust checks sequence, links, canonical hash, and
signature before `Payload::validate`; Python validates the payload while
constructing canonical bytes, so the first reported failure differs. `FORMAT`
specifies the cryptographic order but does not pin this semantic-validation
phase. A caller can therefore implement either the Rust language or the Python
language and appear compliant with the current prose. The smallest correction
is a grammar/first-failure decision plus differential vectors; it must not
change the frozen binary canonical profile or make JSON storage authoritative.

A concrete duplicate-member check strengthens the same finding: inserting a
second `status` member into the signed golden event leaves Python's
`json.loads` with the last (`Conjectured`) value, and the current Python
`verify_event` accepts the event and its unchanged hash/signature. The Rust
Serde-derived `SignedEvent` boundary rejects duplicate known struct fields.
This is a storage-language divergence, not a claim that JSONL bytes are
canonical; it needs the same explicit grammar decision and differential
fixture coverage.

### A-021 details

The v1 format names Ed25519 and an external verifying key, but does not pin
whether weak/low-order public keys are rejected. The Magpie replay path accepts
the caller's `VerifyingKey` and invokes ordinary `verify` at
`crates/magpie-log/src/logimpl.rs:147-150`; it does not reject a weak root at
`LogReader::open`. The Python verifier constructs an Ed25519 key directly and
has no corresponding weak-key check. The locked `ed25519-dalek` dependency
distinguishes ordinary verification from its stricter method, so another
implementation can reject the same root while appearing to implement the
documented Ed25519 rule.

As a targeted adversarial check, a low-order root
`ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f` was used
to construct a two-event `Genesis`/`Note` chain. The signatures were generated
from the low-order public-key equation without a private signing key; the
current Python verifier returned `OK` for both events. The corresponding Rust
call site uses the same non-strict verification mode. A consumer that accepts
that external root can therefore treat attacker-created records as valid
history, while a strict verifier rejects them. This is conditional because
trust in the verifying key is explicitly external and a normal trusted root is
not forgeable by this construction.

Two plausible implementations are therefore divergent under the current
coordinates: ordinary v1 verification accepts weak roots and strict v1
verification rejects them. The smallest correction is to ratify the v1
acceptance rule, reject weak external roots or use strict verification at the
trust boundary, and add a cross-tool hostile vector. If changing the accepted
language would invalidate existing v1 chains, the compatibility consequence
must be explicit or a new format/profile must be used. The correction must not
make the genesis key self-authenticating, introduce a trust registry, or infer
identity from key presence.

### A-005 details

With the current five `Status` variants, tests pass. The defect is latent
compatibility behavior: adding a future lifecycle status can compile while
changing standing promotion without an explicit policy decision. The v4
exhaustive match is the required pattern.

### A-006 details

`ClaimAssertedV2` records no status, but `magpie-episodic` writes
`claim_status = Some("Conjectured")` for that payload. A reader of the
episodic timeline can therefore attribute a signed status to an event that
only registered a typed claim. This is not a hash-chain alteration, but it is
a provenance/authority misrepresentation in a public projection. Two
plausible readers can treat the column as signed event content or as a derived
initial standing. The smallest fix is `None` or an explicitly derived field,
plus a typed-assertion regression; the frozen payload must not change.

### A-007 details

`Projection::apply` is infallible and replay has no freshness/reset precondition.
Applying one verified log twice to a reused projection can duplicate claim
evidence and anchor occurrences; SQLite-backed episodic replay can collide on
sequence primary keys. Separately, a valid `u64` timestamp above `i64::MAX`
can panic in the episodic projection after earlier rows have been applied.
Existing tests prove drop-and-rebuild, not reuse rejection or typed failure.
The smallest correction is a fresh-projection/rebuild contract and a fallible
or preflighted projection boundary; it must not make partial projection state
authoritative or mutate L0.

### A-008 details

`FileStore` is documented as durable but appends with `write_all` and newline
without lock, atomic framing, or synchronization. Concurrent writers can race
on the same tip; a crash can leave a partial record or lose an acknowledged
write. At the trait boundary, `LogStore::read_records` promises only a vector
of byte records; it does not say that the vector is an append-order,
complete, atomic snapshot, nor how an intentionally selected prefix or stale
view is identified. The current `FileStore` and `MemStore` happen to preserve
append order, and no current backend was found to return a stale cache. The
gap matters for the explicitly swappable backend seam: implementation A can
return one atomic complete snapshot, while implementation B can return a
valid but stale prefix. `parse_and_verify_records` will correctly accept that
prefix as a complete chain relative to the supplied bytes, so the same
caller-level request can produce different verified summaries without a
typed stale/incomplete signal. This is a storage-contract ambiguity, not a
claim that the verifier should reject every valid historical prefix.

`read_records` also materializes the entire file, and opaque metadata and
record fields lack a general L0 budget. The downstream bounded-looking
parsers do not fully repair that omission: the deterministic verifier
deserializes `witness_hex` into a `String` before checking its 4 KiB decoded
limit, while the Deadbolt metadata resolvers deserialize unbounded identity
strings and walk arbitrary ignored JSON members before their semantic checks.
A hostile or merely large local log can therefore exhaust resources during
replay or interpretation before a local signature, size, or policy rejection.
The smallest correction is a storage/resource contract covering single-writer
behavior, sync/durability, append-order and complete/atomic snapshot
semantics, explicit selected-prefix coordinates, bounded streaming, metadata
and recursion limits, and byte-count-before-materialization rules. It must
not introduce a cache as a second history, silently treat a stale prefix as
the current log, or bypass verification.

### A-009 details

`LogReader::events()` returns parsed-but-unverified `SignedEvent` values and
the public `Projection` trait accepts ordinary public events. A caller can
construct a forged `SegmentAnchored` event or index, apply it to
`DeadboltAnchorIndex`, and obtain `Matched` from the free occurrence resolver
without chain/signature verification. Current v1-v4 standing paths use a
private same-replay context, so this is a contained public-boundary hazard,
not a demonstrated standing bypass. The smallest correction is an opaque
verified-event/replay input or an unmistakably untrusted API; it must preserve
the existing raw projection compatibility surface deliberately.

The helper has a second caller-controlled eligibility input: its public
`evidence_kind` and `claim_domain` arguments are checked for the required enum
values, but the function does not derive them from the target claim and
evidence metadata. Its own trusted-use documentation says the caller must
perform that derivation. Two plausible consumers can therefore use the same
index and metadata while one selects `OccurrenceInclusion` and another selects
a different domain; only the former reaches exact anchor matching. The private
standing paths avoid this by deriving the relevant typed structures from one
verified replay, but the public helper still exposes a policy-input seam where
a low-authority caller can choose eligibility by convention. The same narrow
remediation should either co-derive the domain/kind input or make this helper
explicitly mechanics-only and untrusted; it must not add a generic trust store
or make caller presence an authorization signal.

### A-015 details

`StandingResolution`, `StandingResolutionV1`, and `StandingResolutionV2` are
reexported public structs with public fields and public `canonical_bytes()`
methods. Their public nested v1/v2 application and trace values, together
with public `DeadboltAnchorIdentity`/`DeadboltAnchorOccurrence` fields, also
permit a caller to construct a `Matched` occurrence and `Settled` application
without a verified replay. A downstream caller can therefore construct a
value with `currentness: StandingCurrentness::Current`, an arbitrary
`policy_id`, and any standing/trace/application fields, then serialize it in
the same JSON shape as an actual v0–v2 resolver result. The current replay
paths construct `Unknown` for v0 and inherit/quarantine values in later
policies; the fabricated value does not alter L0 or enter the private v3/v4
composer, so this is contained at the public output/compatibility boundary
rather than a standing bypass.

The concrete failure is a confused consumer: one producer returns a genuine
coordinate-poor v2 result while another returns a manually constructed value;
both have the same public type and no verified-prefix identity, and a query,
cache, or report layer can mistake the latter's `Current`/`Settled`/`Matched`
fields for log-derived standing. The smallest correction is to make
construction opaque or mark these values explicitly untrusted/compatibility-
only, with compile-fail tests for direct literals and application values. It
must not add currentness semantics, mutate the frozen event format, or make
serialized output authoritative merely by validating its JSON shape.

### A-016 details

ADR-0005 says a withdrawal targets the withdrawing actor's own assertion and
that a withdrawal directed at another actor's act is invalid by construction,
while the same document says attribution is asserted data and never verified
identity, with delegation deferred to identity/governance doctrine. There is no
withdrawal event or runtime on `main`, so this is a future readiness gap rather
than a present append or standing bypass.

Two plausible implementations appear compliant today: implementation A treats
equality of asserted actor labels as self-ownership; implementation B requires
an out-of-band signer-to-identity/delegation record or a future gate-owned
authority binding. A forged signer can make A accept a withdrawal of another
actor's assertion, while B rejects it. The missing identity/authority
coordinate means the implementations can disagree about a future currency or
standing consequence from the same recorded material.

The smallest safe correction is to state explicitly that self-ownership is
unimplemented until identity/governance semantics are ratified, or to freeze
asserted-label equality as a historical, non-authenticating relation and keep
any stronger authority behind a separately coordinated policy. It must not
invent an identity registry, delegation system, trust score, or withdrawal
runtime in this audit.

### A-017 details

`Claim`, `ClaimsView`, `StandingClaim`, and the public maps in `StandingView`
are mutable, serializable projection values. A legacy
`ClaimStatusChanged { to: Settled }` can leave `Claim.status` or
`StandingClaim.standing` displaying `Settled`, while the governed v0 resolver
quarantines legacy raw status for typed claims and newer policy outputs carry
their own controlled result. The release contract explicitly classifies these
as legacy/manual compatibility surfaces, but the public names and JSON shape
do not make that distinction structural.

Two consumers can therefore both use the current public API: one treats the
field as historical/raw compatibility data, while another treats
`ClaimsView::get(...).status` or serialized `StandingView` as current governed
standing. Direct construction or mutation makes the same ambiguity possible
without any log at all. This does not bypass the private v1–v4 composition
path, so the finding is contained at the compatibility/query boundary.

The smallest correction is an additive quarantine: preserve old fields and
bytes, but provide explicitly named legacy/raw access and require
coordinate-bearing governed resolution for authority-bearing consumers. It
must not rewrite legacy events, repurpose `Status`, or make a projection a
second history.

### A-018 details

Several public-module compile-fail examples still call resolver methods with
caller-selected selectors, audits, lists, or policy values even though the
current replay-context methods accept only their closure/context inputs. The
examples consequently fail on stale method arity or removed method shape,
not necessarily because an authority-bearing value was rejected. The intended
boundary is visible in the current methods at
`origin_admission_replay.rs:151-169`.

This creates false enforcement confidence: a future edit can change the
method signature and make an example fail or pass for an unrelated reason,
while the test suite still provides no focused proof that a caller cannot
construct, deserialize, or substitute an audit/context value. The two
plausible implementations are a correctly opaque context API and a method
that accepts caller-selected audit material but happens to retain a different
arity; the current compile-fail result does not distinguish them.

The smallest correction is to rewrite each example around the forbidden
surface itself—private-field literal, `Deserialize`, or substitution into the
actual current method—and, where necessary, add a diagnostic-oriented
compile-fail harness. It must not require brittle compiler-message snapshots
or add a new authority mechanism.

### A-019 details

`EpisodicEvent` exposes `seq`, timestamp, agent, source, kind, claim/status
strings, and body, but no event hash, signature, predecessor, verifying-key or
log identity, or verified-prefix identity. `EpisodicView::get` returns that row
by local sequence and `search` returns only sequence numbers; `canonical_bytes`
serializes the same row projection. The projection documentation correctly
calls SQLite derived, but the public row type does not enforce the provenance
package required when derived information leaves the core.

Two plausible consumers are therefore allowed by the current surface: one
retains the verified replay context beside the row and treats it as a derived
view, while another serializes the row or sequence as a standalone query
answer and treats `agent`, `source`, `claim_status`, and `body` as a signed
historical fact. Two different verified logs can contain equivalent projected
rows at the same sequence while differing in genesis key, predecessor, or
event hash; a detached row cannot distinguish them. The public unverified
projection seam makes the same ambiguity possible even without a verified
replay.

The search method has a second, narrower reproducibility gap. It hardcodes an
FTS5 phrase query (`fts_phrase_query`) but returns no snapshot identity,
projection/schema identity, query-language definition, tokenizer/engine
semantic version, or derivation envelope. The proposed librarian contract
explicitly defers query-language/API design while requiring reproducible query
derivations. Two plausible future consumers can therefore use the same
verified snapshot and query with different allowed implementations—one with
the current bundled SQLite/FTS5 phrase semantics and one with a separately
versioned tokenizer or linear query implementation—and return different
sequence sets. A `rusqlite`/bundled SQLite upgrade can create the same drift
without changing the log. This is not a current standing or write bypass
because no librarian service exists, but it is a concrete compatibility and
future query-coordinate gate.

This is distinct from A-003's standing-output coordinate omission, A-006's
invented status, and A-009's unverified input boundary: even a correctly
verified replay can lose its source binding when the row is detached. It
conflicts with the provenance-response and librarian
requirements that a derived answer identify its verified snapshot/prefix and
derivation class. The smallest correction is an additive response/context
wrapper for authority-bearing query surfaces containing the verified prefix or
log identity, exact event identity where a row is returned, an explicit
query/projection semantic identity where search is returned, and projection,
query, and schema coordinates. Existing `EpisodicEvent` bytes and bare search
results may remain raw derived compatibility surfaces.

The correction must not make SQLite canonical, treat a row as a new event,
add generic query permission, or introduce an unverified index authority. With
no live librarian or MCP runtime on `main`, this is a future query/API contract
and readiness gate rather than a current write bypass.

### A-020 details

`.gitignore` excludes `.agent-runs/`, and the local pending queue still contains
old tickets whose hard assumptions contradict the audited checkout. In
particular, pending Ticket 0003 says `HEAD` has no `crates/` directory and does
not build, while the audited `HEAD` contains the three-crate workspace and has
passed the recorded validation; pending Ticket 0006 describes the pre-schema-
guard episodic failure even though the current source already contains the
schema-version guard. Ticket 0001 also describes the workflow itself rather
than a current implementation task.

The delegation wrapper accepts any caller-supplied ticket path and, in write
mode, creates a worktree from the current `HEAD`; it does not check that a
pending ticket's assumptions, base, status, or changed-path allowlist still
match the checkout. A future coordinator can therefore follow the worker rule
faithfully and launch an obsolete migration or duplicate an already-landed
change. The ignored files are not repository doctrine and no automatic queue
runner was found, so this is conditional workspace workflow debt rather than a
current runtime authority break.

Two plausible workflows are both available: workflow A retires or revalidates
the ticket against the current base before invocation; workflow B treats the
presence of a file under `pending` as actionable and relies on the ticket's
stale precondition. They can produce materially different edits, including
destructive layout replacement, without any change in the delegation wrapper.

The smallest correction is a process/status gate requiring an owner, base SHA,
current-state preflight, explicit `pending`/`active`/`retired` state, and a
changed-path audit before write mode. It must not add automatic execution,
broaden Codex authority, or treat ignored run artifacts as canonical history or
architecture. Correction belongs in the workflow documentation or audit
ledger; no current ticket should be executed as part of this audit.

### A-011 details

Workspace dependency requirements are broad semver ranges and CI has no
`cargo audit`, `cargo deny`, declared MSRV/toolchain, or dependency semantic
review gate. A future serialization, crypto, parser, or ordering dependency
upgrade could alter governed bytes or acceptance while ordinary tests remain
green. This is process/supply-chain debt rather than evidence of a malicious
dependency. The smallest correction is a release/review gate requiring
dependency diff inspection, golden/differential vectors, and security review;
it must not assume a lockfile alone is a format or API compatibility policy.
The CI workflow also upgrades `pip` and installs an unpinned latest
`cryptography` package before running the independent verifier, while its
GitHub actions are referenced by mutable `@v4` tags. A verifier or action
change can therefore alter the validation environment without a Magpie source
change. The same gate should pin or otherwise review those toolchain inputs;
it must not treat a green run from a mutable environment as a frozen semantic
vector.

The release contract also requires independent Python verification of the
portable Deadbolt anchor fixture (`docs/releases/v0.1.0-contract.md:447-450`),
but CI invokes `tools/verify_chain.py` only for `golden-v1.jsonl`
(`.github/workflows/ci.yml:54-62`). The required Deadbolt command passes on the
audited checkout and verifies two records, so this is a missing CI gate rather
than a failed fixture. A verifier or tag-5 serialization regression could
therefore leave CI green while failing the stated release acceptance checklist.
The smallest correction is to add the exact fixture command to CI or explicitly
make it a separately enforced release-owner gate; it must not treat the Python
chain check as foreign Deadbolt-bundle verification.

### A-010 details

Independent implementer simulation produced divergent plausible designs for:

- claim versus assertion identity;
- log sequence versus timestamp ordering;
- Current versus Unknown absence semantics;
- supersession-only versus invalidation/withdrawal eligibility;
- exact target scope and actor ownership;
- cycles and multiple successors; and
- output vocabulary and coordinate carriage.

Current typed projections do not retain enough event identity to resolve these
questions safely by implementation convention.

The latest ADR-0006 candidate narrows the doctrine to three coordinates, but
it explicitly defers resolver algorithm details and output encoding
(`PR #103`, head `b6cc4dc5ff75672b3b9e5a879ee4f2e749faa5b7`,
`docs/adr/0006-claim-currency-and-currentness-semantics.md:128-135,400-414`).
A future implementer could bind resolver identity to an immutable,
versioned algorithm descriptor, while another could treat it as a stable
module name whose implementation may be upgraded. Both readings appear
possible until the identity-to-algorithm binding is governed; the latter
would permit different outputs under the same declared coordinates. This
is retained under A-010 as a pre-runtime coordinate/compatibility gate, not
as a new current runtime finding: no currentness resolver exists on `main`.

Before implementation, resolver identity must have a stable, explicitly
serializable binding to its algorithm and any immutable configuration; an
ambient implementation upgrade must not preserve the same identity while
changing output.

### A-022 details

The admission readiness checklist explicitly requires reviewed answers for
whether rejected attempts are recorded, how a recorded rejection cannot become
negative evidence, and how an unrecorded rejection avoids retry amplification.
The admission questions, minimal-semantics exploration, and admission-record
boundary all leave those questions open. They also leave the exact historical
representation — reference, copied content, summary, and provenance carriage —
unresolved. The final readiness gate says an unanswered item is a blocker, so
this is a future implementation gate rather than a defect in a nonexistent
admission runtime.

Two plausible implementations remain materially divergent while appearing
compatible with the current explorations. Implementation A records each
rejected attempt, with a future governed non-epistemic attempt identity and
reason, in replayable history; implementation B keeps rejection state outside
L0 and uses an external deterministic deduplication or retry ledger. A changes
the historical snapshot, provenance, replay cost, and the query-visible audit
surface; B permits a different retry and amplification model and leaves no
historical rejection record. A third independent choice — recording a
reference, copied bytes, or only a digest — changes what later provenance and
availability claims can mean. The current corpus intentionally chooses none
of these.

The smallest safe remediation is an owner-ratified admission ADR or contract
that answers, before implementation: the exact admitted historical object;
whether rejected attempts are recorded; the typed, non-epistemic semantics of
any rejection record; retry/idempotence/amplification handling; provenance and
source coordinates; and the structural/policy failure boundary. It must then
be covered by a readiness matrix and hostile replay tests. This remediation
must not add a new event tag, generic write path, trust score, truth or
standing judgment, identity/delegation system, cache-backed shadow history,
ambient policy, or generic Deadbolt semantics. Correction belongs in a future
owner-governed ADR/contract, not the active PR and not an implementation PR.

### A-013 details

ADR-0001's implementation section claims that the Magpie-side slice ships with
`seam-slice.patch`, then identifies the Deadbolt-side emitter and reconciliation
set as the next ticket. The current repository contains the landed tag-5
sources, fixtures, and tests but no file named `seam-slice.patch`. Two
implementers can therefore disagree about whether the patch is an archival
artifact to apply, a historical label for the landed files, or a missing part
of the accepted ADR. The smallest correction is a precise landed commit/file
reference or an explicitly archived artifact; no new runtime seam or authority
should be inferred from the filename.

### A-014 details

`Payload::validate` checks only that `SegmentAnchored.witness_root` is exactly
64 lowercase hexadecimal characters. It does not reject empty or oversized
`bundle_kind`, `witness_algorithm`, `canonicalization_profile`, or `run_id`.
The exact Deadbolt metadata parser later requires those fields to be non-empty
and the root to be canonical, so a malformed signed anchor is currently
retained in L0 and in a manually applied index but cannot match a well-formed
claim/evidence predicate through the current exact resolver. That containment
prevents this audit from calling it a false-match or history-corruption bug.

The remaining ambiguity is whether such a marker is a valid opaque historical
anchor or a malformed anchor that the seam must reject before append. The
smallest correction is a seam contract decision with bounded field rules (or
an explicit anchorer precondition), followed by hostile vectors. It must not
make a marker prove foreign-bundle interpretation, identity, trust, or generic
write permission.

## Finding-standard coverage

The findings ledger supplies each finding's severity, category, exact sources,
runtime surface, failure scenario, minimal remediation, and status. The table
below makes the remaining requested fields explicit: governing invariant,
plausible divergent implementations where ambiguity is material, concepts
that the remediation must not introduce, and correction locus.

| ID | Governing invariant | Divergent implementations / concrete counterexample | Remediation must not introduce | Correction locus |
| --- | --- | --- | --- | --- |
| A-001 | `LogWriter` is the sole append authority. | Writer-owned append only versus any caller with `LogStore::append_record`; the latter can append malformed raw bytes and corrupt or strand the backend. | Generic admission, a second history, or a generic Deadbolt writer. | Separate Rust API/storage code PR. |
| A-002 | Governing status and precedence must be explicit; historical text cannot silently authorize runtime. | Implementer A follows `FORMAT`/ADR future language; implementer B follows bootstrap/README/visual build-now language and creates a different gate/lifecycle system. | Silent ADR ratification, time-aware currentness, or a new admission mechanism. | Owner-governed documentation/status PR. |
| A-003 | Every detached authoritative output carries the verified snapshot/prefix and resolver coordinates. | Consumer A carries the verified context; consumer B serializes a coordinate-poor v0-v2 result and treats equal projection bytes/count as the same history. | Redefining comparison bytes as history identity or changing frozen v0-v2 meaning. | Future additive response/API contract. |
| A-023 | A detached provenance response must carry the exact producing prefix and immutable closure coordinates. | `H1+M1` and an extended `H2+M2` can produce identical direct matched receipt bytes; one consumer keeps caller-scoped context while another treats the receipt as self-contained. | Trust in receipt serialization, a loader/cache authority, or a new provenance policy. | Future provenance/API contract or explicit internal/non-detachable boundary. |
| A-004 | Canonical verification language and first-failure order must be cross-implementation reproducible. | Rust skips blank lines and accepts mixed-case hex while Python rejects them; Python accepts numeric statuses/`NaN` that Rust rejects. | Making JSONL storage canonical or changing frozen binary preimages. | Separate verifier/tooling contract and differential-vector code PR. |
| A-021 | Historical integrity must pin the accepted Ed25519 trust-root and verification language. | Ordinary verification accepts a low-order external root; strict verification rejects it, yielding different accepted histories under the same named v1 rule. | Genesis self-authentication, a trust registry, or identity inference from key presence. | Separate crypto/format compatibility PR. |
| A-005 | Closed vocabulary membership grants no authority; status extension requires an explicit policy arm. | v2/v3 wildcard arms promote a future `Status` variant; v4's exhaustive arms force an explicit decision. | Adding a status, implicit fallback semantics, or silent policy promotion. | Code PR before any status extension. |
| A-006 | Recorded material must not be presented as a status that the event did not record. | One reader treats episodic `Conjectured` as signed event content; another treats it as derived initial standing. | Changing the frozen typed payload or making the episodic projection authoritative. | Projection code/test PR. |
| A-007 | Projections are rebuildable and failed/partial state is not authoritative. | A caller reuses a projection and duplicates state; another starts fresh; episodic overflow can panic after partial application. | Hidden reset, partial-state authority, or L0 mutation. | Future projection/replay contract and code PR. |
| A-008 | Storage and replay must remain bounded, durable, and deterministic under hostile local input. | A minimal `LogStore` can return an append-ordered complete atomic snapshot, while another swappable backend returns a valid stale prefix because the trait does not define completeness or selected-prefix semantics; independently, whole-file/read-all and pre-materialization can diverge from bounded streaming/framing, and concurrent writers can race or crash between bytes. | A cache as shadow history, silently treating a stale prefix as current, bypassing verification, or premature optimization unrelated to safety. | Separate storage/resource contract and code PR. |
| A-009 | Verified replay must precede trusted projection interpretation. | A private same-replay path is verified; a public caller-created index plus `Projection::apply` can return `Matched` without verification. | A trust database, caller-presence authorization, or silent removal of compatibility APIs. | Future public API/compatibility PR. |
| A-010 | Currentness is a pure, coordinate-complete resolver facet, not stored state or ambient latest. | One implementer orders assertions by sequence and defers acts; another uses timestamps/claim identity and treats asserted labels as actor authority. | A currentness runtime, identity registry, time semantics, or facet feedback loop before doctrine. | Owner ADR/contract before implementation. |
| A-022 | Admission must not silently acquire historical, epistemic, retry, or provenance authority from an unanswered failure boundary. | Implementation A records typed rejected attempts and exact candidate references in L0; implementation B keeps rejection/deduplication outside L0 and retries until acceptance. The two produce different snapshots, provenance, replay, and amplification behavior while both fit the current open questions. | A new event tag, shadow history, trust/standing judgment, identity system, cache authority, or generic write path. | Future owner-governed admission ADR/contract and readiness tests. |
| A-024 | Provenance and attribution cannot become trusted authority merely through claimant-controlled labels; self-declared groups must not amplify standing. | Implementation A treats exact `human-review` plus `authority:origin-review-v0` strings as sufficient and v3 counts fresh claimant groups; implementation B requires an independently bound authority coordinate and leaves the same bundles unresolved/non-corrobating. | A caller-provided trusted flag, reputation score, generic trust registry, source ranking, new actor/event vocabulary, or identity inferred from anchor occurrence. | Future authority/provenance ADR and code/test gate. |
| A-011 | Governed bytes, verifiers, and release claims require reviewed dependency/toolchain changes. | A broad semver/toolchain/action update can alter semantics while lockfile/tests remain green; a release gate can run the required Deadbolt fixture or omit it. | Treating green CI or a lockfile as semantic supply-chain approval. | Release/CI/dependency review PR. |
| A-012 | Derived-state interfaces must not gain unrelated destructive authority. | `at_path` can migrate and drop tables in any caller-selected database; an owned-marker/refuse path would contain the operation. | A generic permission system, hidden database authority, or SQLite-as-history. | Separate security/API contract and code PR. |
| A-013 | Governing documents must point to actual implementation evidence. | `seam-slice.patch` can be read as an unapplied required artifact or as an archival label for landed files. | A new runtime seam or authority inferred from a filename. | Documentation traceability PR. |
| A-014 | Deadbolt anchor validation must fail closed at the specialized seam. | A permissive opaque marker can be retained as history; a stricter seam rejects empty/oversized identity fields before anchoring. | Foreign interpretation, trust, identity, or generic write permission from an anchor. | Deadbolt seam contract/code PR. |
| A-015 | Public serialized compatibility values are not verified resolver authority. | A caller can construct `Current`/`Settled`/`Matched` JSON-shaped v0-v2 values; private replay construction cannot be substituted that way. | JSON validation as authority, new currentness semantics, or format changes. | API/compatibility code and compile-fail test PR. |
| A-016 | Attribution is not verified identity or withdrawal authority. | Label equality can treat a forged actor label as self-ownership; signer/delegation binding would reject it. | An identity registry, delegation system, trust score, or withdrawal runtime. | Future identity/governance ADR. |
| A-017 | Legacy/raw projection fields cannot become canonical standing by naming or presence. | Consumer A treats `Claim.status` as historical raw data; consumer B treats the same field or JSON as governed standing. | Repurposing `Status`, rewriting legacy events, or making a projection a second history. | Compatibility/API PR or explicit ledger quarantine. |
| A-018 | A compile-fail test must fail for the forbidden authority surface itself. | A stale-arity example and a truly opaque-construction example can both fail today while proving different boundaries. | Brittle compiler-diagnostic snapshots or a new authority mechanism. | Test/docs PR. |
| A-019 | Derived responses must retain record/prefix and derivation/query provenance. | One consumer carries replay context; another detaches an episodic row or bare sequence list and treats it as signed history. | SQLite as canonical, a new query permission system, or an index authority. | Future query/provenance API contract. |
| A-020 | Write workflows require fresh base, status, ownership, and allowlist preflight. | Workflow A revalidates or retires a pending ticket; workflow B treats an ignored stale file as actionable and may repeat an obsolete migration. | Automatic execution, expanded agent authority, or ignored queue as doctrine. | Process/documentation PR or audit ledger. |

The table is a reporting aid, not a new governing contract. It does not alter
the repository's doctrine or authorize any remediation.

## Deadbolt seam field and readiness record

| Crossing field | Producer/meaning | Magpie-side handling | Authority explicitly not transferred | Enforcement/current state |
| --- | --- | --- | --- | --- |
| `bundle_kind` | Deadbolt's sealed-bundle kind identifier | Stored opaquely in `SegmentAnchored` and exact `DeadboltAnchorIdentity` | No bundle-kind interpretation, source ranking, or new Magpie tag | Root-only L0 validation; non-empty rule is A-014 |
| `witness_root` | Deadbolt's root over a sealed bundle | Stored as exact lowercase 64-hex string; included in chain hash/signature and exact identity | No proof that the foreign bundle is available or semantically correct | `Payload::validate` and metadata parser enforce shape; foreign verification remains external |
| `witness_algorithm` | Algorithm used by Deadbolt to calculate the root | Stored opaquely and exact-matched | No algorithm execution or downgrade decision inside Magpie | No L0 non-empty/allow-list rule; future seam contract required |
| `canonicalization_profile` | Foreign profile that produced the root | Stored opaquely; explicitly distinct from `magpie-core-v1` | No foreign canonicalization or interpretation | Exact metadata matching; no foreign verifier runtime in Magpie |
| `run_id` | Deadbolt execution/seal-run reference | Stored opaquely and exact-matched | No authenticated identity, delegation, or run authorization | Metadata parser requires non-empty; L0 accepts empty today (A-014) |
| `seq`, `prev_hash`, event hash, signature | Magpie's historical chain coordinates | Verified by Magpie before trusted replay/indexing | No inference that chain inclusion verifies foreign bundle contents | Hash/signature/genesis tests and golden vectors; raw append seam remains A-001 |
| `provenance.agent/source` | Signed attribution supplied to the event | Retained as audit material | No verified person/service identity or trust root | Signed and chained, but identity remains external |
| Claim/evidence metadata anchor identity | Magpie typed claim/evidence relation | Exact parser checks schema, root, fields, and equality before lookup | No independent foreign-bundle verification or standing from metadata presence | Closed metadata matching and hostile malformed cases; caller-created index remains untrusted |

Readiness disposition:

- **Verification before anchoring:** not implemented as a generic Magpie
  runtime. The current Magpie verifier verifies the Magpie chain; a future
  Deadbolt emitter must complete foreign-bundle verification before invoking
  `LogWriter`.
- **Initialization/key material:** no live Deadbolt emitter or key lifecycle
  implementation is present in this repository; therefore initialization
  stranding is a future operational gate, not a passing runtime claim.
- **Reconciliation:** ADR-0001 explicitly defers the sealed-but-unanchored
  reconciliation set/retry story. No unverified history may be appended to
  compensate for a reconciliation failure.
- **Specialized seam scope:** tag 5 and its exact occurrence resolver are not
  precedent for generic write authority, admission, or interpretation.
- **MCP/agent wrapping:** no MCP/agent runtime exists to expand the seam; any
  future wrapper must preserve the same field ownership and read/proposal
  boundary.
- **Execution evidence:** `ExecutionEvidence` and `DeadboltAnchor` remain
  closed evidence vocabulary, but presence or anchor occurrence alone does
  not exceed the named standing policy ceiling.

## Rejected findings

| Candidate | Disposition |
| --- | --- |
| `LogWriter::append` itself is an unauthorized admission policy | Narrowed: raw L0 capability is intentional; higher-level admission is deferred. |
| Provenance labels authenticate identity or trust | Rejected: the doctrine explicitly separates attribution, identity, and trust. |
| Deadbolt occurrence proves interpretation truth | Rejected: the seam is occurrence/inclusion only and foreign verification remains external. |
| `docs/deadbolt-witness-survey.md` silently delegates a second Deadbolt root or write authority | Rejected: the survey labels itself a static, non-design, non-authoritative report (`docs/deadbolt-witness-survey.md:3,134-136`); it describes diagnostic/root surfaces and explicitly withholds Magpie integration authority. Its absent `cog-*` citations are therefore not a Magpie runtime contract; any future export remains subject to ADR-0001 and the narrow seam contract. |
| Missing MCP, CLI, or librarian runtime is a defect | Rejected: these are explicitly future and no implementation exists. |
| Wall-clock creation time breaks replay determinism | Rejected as a replay defect: timestamps are recorded historical inputs; deterministic clocks exist for fixtures. |
| `StandingCurrentness::Unknown` is incorrect | Rejected: currentness is intentionally deferred. |
| Snapshot comparison bytes must become prefix identity | Rejected: the governing origin-admission contract explicitly forbids that substitution. |
| Verified-prefix identity must include signature bytes or a second verifying-key field | Rejected for the current core-history identity: the genesis payload includes the declared verifying-key bytes and is included in the canonical chain hash, while signatures and JSONL storage bytes are not the canonical event preimage (`docs/FORMAT.md:160-193`). `VerifiedLogPrefixIdentityV0` intentionally identifies the completely verified core prefix by count plus tip; caller trust in the supplied key remains an explicit embedding coordinate (`crates/magpie-claims/src/origin_admission_replay.rs:90-110`; `docs/design/origin-admission-audit-v0.md:476-521`). A future detached response may carry trust-root metadata if its contract needs that context, but it must not redefine this prefix identity or infer trust from it. |
| Extra JSON storage members let non-canonical data become signed history | Rejected as a separate canonical-integrity finding: `docs/FORMAT.md:188-193` makes JSONL storage non-normative, both current verifiers reconstruct the typed signed core while ignoring unknown object members, and no authority-bearing replay path consumes those members. Any future cross-tool storage grammar decision belongs under A-004; storage must not become canonical. |
| Earlier rejection of the exact `human-review` authority pair concern | Reclassified as A-024. Fixed string selection prevents caller policy substitution, but it does not independently bind a claimant-controlled authority claim; current origin admission transfers the claimed group into the fold and v3 consumes distinct groups for `Supported`. The prior rejection therefore did not answer the self-declared-group law. |
| Public Deadbolt matching already bypasses v1–v4 standing | Narrowed: the free helper is dangerous if misused, but policy paths consume private same-replay context. |
| Missing tests for deliberately deferred doctrine are current defects | Rejected: these are rollout gates, not unimplemented runtime failures. |
| v3/v4 outputs have a new current resolver-identity defect because they do not carry a separately named resolver field | Rejected as a duplicate current finding: the implemented v3/v4 surfaces are private, fixed one-policy methods, carry verified-prefix and closure identities, and the release contract requires a new policy identity for semantic changes; stable resolver-to-algorithm binding remains a future A-010 gate. |
| Resolver identity must already be a new public type on `main` | Narrowed: the three-coordinate doctrine is present, but stable identity-to-algorithm binding and encoding are explicitly future gates under A-010; adding a type before the resolver contract would be premature. |
| All future-tense wording in design documents is stale doctrine | Narrowed: frozen historical body text may remain future-tense when its status is explicitly historical; A-002 covers contradictory living status and handoff text. |
| Public raw status is already governed standing | Rejected: the release contract classifies these fields as legacy/manual compatibility surfaces; A-017 concerns the remaining ambiguity at the public consumer boundary. |
| Existing compile-fail doctests prove the intended authority boundary because they fail | Rejected: A-018 records that several fail on stale arity or removed API shape, so the boundary is not demonstrated. |
| The public artifact selector constructor is independently an unbounded provenance/resource authority flaw | Rejected as a separate finding: the selector is explicitly untrusted and infallible; direct resolution compares fixed protocol fields and rejects mismatches before constructing a matched receipt, while `ResolutionContentClosureV0::construct` bounds selector key fields. The broader storage/input resource contract remains A-008, but this pass found no independent authority or replay divergence from the selector constructor itself. |

## Compatibility inventory

The following surfaces must not be silently repurposed:

- `magpie-core-v1`, canonical field order, hash/signature domains, tags 0–8,
  golden vectors, and the JSONL distinction between storage bytes and canonical
  preimages;
- legacy payloads, raw status fields, `ClaimsView`, and `StandingView` maps;
- public legacy `Claim.status`, `StandingClaim.standing`, and mutable
  projection maps must remain compatibility/raw surfaces (A-017);
- `LogStore`, `FileStore`, `MemStore`, and `Projection` public behavior;
- v0–v2 resolution serialization shapes and policy IDs;
- `StandingReplaySnapshot::canonical_bytes()` and
  `DeadboltAnchorIndex::canonical_bytes()` as comparison-only bytes;
- the five-field `SegmentAnchored` identity and foreign profile semantics;
- serialization-only provenance, origin, admission, and contribution audits;
- `EpisodicEvent` row/search meaning and detached record/prefix/query provenance (A-019);
- compile-fail doctest intent and the current resolver method signatures
  (A-018);
- public Ed25519 key reexports and v1 verification acceptance, plus public
  `rusqlite` errors (A-021);
- proposed, ratified, historical, and reference document status meanings.

## Confidence ledger

| Area | Confidence |
| --- | --- |
| Historical integrity | Medium-high for a normal supplied key and complete verified replay; medium-low if weak-root acceptance is in scope (A-021) |
| Canonical encoding | Medium-high in Rust/golden path; medium cross-tool |
| Provenance | High for top-level landed audits; medium for direct matched traces and medium-low for detached projection/query rows and claimant-controlled authority/corroboration binding (A-019, A-023, A-024) |
| Evidence eligibility | High for exact v0–v4 lanes; medium for independently verified authority eligibility behind external corroboration (A-024) |
| Standing | High for landed policies; medium-low where external corroboration relies on claimant-controlled authority labels; low for general standing (A-024) |
| Currentness | Low |
| Lifecycle acts | Medium doctrine-only; low runtime |
| Resolver coordinates | Medium-low |
| Admission | Low / unaudited |
| Deadbolt seam | Medium Magpie-side; low end-to-end |
| Query/librarian and MCP | Low / unaudited |
| Runtime compatibility | Medium |
| Testing | Medium-high for landed slices; medium overall, with compile-fail boundary evidence reduced by A-018 |
| Security | Medium |
| Operational readiness | Low |

## Security assessment

Security properties are intentionally separated rather than collapsed into a
single trust judgment:

| Property | Current assessment | Principal residual risk |
| --- | --- | --- |
| Historical integrity | Medium-high for complete replay under a normal supplied verifying key | Public raw storage append, incomplete durability/concurrency bounds, and unpinned weak-root acceptance (A-001, A-008, A-021) |
| Epistemic integrity | Medium for the landed narrow policy lanes; low for deferred facets | Document precedence, wildcard promotion, synthetic projection status, fabricated compatibility resolutions, raw legacy projection status, detached episodic rows, coordinate-poor direct provenance traces, claimant-controlled corroboration separation, and currentness gaps (A-002, A-005, A-006, A-010, A-015, A-017, A-019, A-023, A-024) |
| Execution authority | High for the current repository because no generic CLI/MCP writer or gate is implemented | Future admission, agent, and MCP work could widen authority unless readiness contracts remain separate |
| Identity and attribution | Low for authority-bearing origin corroboration; otherwise medium-low | Signed actor/source labels are not identity or trust; the origin-admission path also treats claimant-controlled exact authority labels as sufficient to transfer an origin group into corroboration without an independently verified authority binding (A-024); withdrawal self-ownership has no current identity coordinate (A-016), no actor/delegation runtime exists, and foreign-bundle verification remains Deadbolt-owned |
| Availability | Medium-low | Unbounded/whole-file materialization, projection panic paths, arbitrary SQLite migration, and absent fuzz/crash/concurrency coverage (A-007, A-008, A-012) |

No signature, authentication result, capability description, or verified
occurrence was treated as proof of truth, authorization, identity, or correct
interpretation.

### Threat coverage matrix

| Threat | Current boundary/evidence | Residual disposition |
| --- | --- | --- |
| Unauthorized append | `LogWriter` signs and chains accepted payloads, but public `LogStore::append_record` can mutate the backend without that path | A-001; raw bytes are not accepted history until verification, but the second mutation seam can corrupt or strand the store |
| Forged attribution / identity spoofing | `provenance.agent`, `source`, origin group, and authority references are signed labels; no identity, delegation, or independently verified authority-binding runtime exists | Attribution is not identity or trust; A-016 keeps actor-local withdrawal semantics deferred, and A-024 prevents claimant-controlled authority labels from being treated as independent corroboration |
| Confused deputy / capability escalation | No live CLI, MCP, librarian, AgentProposer, or generic gate runtime; future contracts explicitly deny permission from capability presence | Future-only readiness gate; no current execution authority was found |
| Replay or snapshot substitution | Verified prefix identity exists for newer contexts and top-level audits; detached v0-v2 outputs and direct matched provenance traces omit one or more producing coordinates, while future currentness coordinates remain open | A-003/A-010/A-023; policy, resolver, prefix, closure, and output identities must be bound before authoritative detached answers |
| Policy or resolver substitution | Landed v0-v4 methods use fixed policy IDs and no ambient latest selector; stable resolver-to-algorithm binding is not yet a general contract | Narrow slices are contained; A-010 future coordinate gate remains |
| Provenance stripping / stale-cache authority | Projection stores are rebuildable and no query service is live; public legacy, v0-v2, direct matched traces, and episodic outputs can omit or expose weak coordinates | A-003/A-007/A-017/A-019/A-023; no cache may become a shadow history or authority |
| Path traversal / destructive derived-state access | No path-taking CLI exists; `EpisodicView::at_path` accepts an arbitrary caller path and may migrate a mismatched SQLite file | A-012 conditional; future API/storage contract must establish ownership without adding hidden authority |
| Malformed canonical data / parser confusion | Rust chain validation and golden vectors fail closed, but Rust/Python acceptance and first-failure order differ | A-004; freeze one grammar and differential malformed fixtures |
| Denial of service, unbounded cycles, or oversized metadata | Whole-file record materialization, unbounded L0 fields, projection panic paths, and future graph-cycle/resource questions remain | A-007/A-008/A-010; add bounds and cycle/resource gates before untrusted ingestion or graph resolution |
| Algorithm downgrade | Current narrow roots and signature domains use pinned profile/algorithm identifiers and no negotiation path was found | Weak-key/strictness acceptance is not pinned (A-021); every future verifier/provider algorithm change needs an explicit versioned contract and vector review |
| Dependency or supply-chain compromise | Lockfile and CI checks exist, but no `cargo audit`/`cargo deny`, MSRV, fuzz, or semantic dependency policy is enforced | A-011 process debt; separate historical integrity from dependency trust |

## Validation evidence

Passed on the audited clean `main`:

```text
cargo fmt --all --check
cargo test --workspace --locked
cargo test --doc --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked --no-fail-fast  # fresh rerun; 281 passed
cargo fmt --all --check  # fresh rerun
cargo clippy --workspace --all-targets --locked -- -D warnings  # fresh rerun
cargo run --locked --example tour -p magpie-claims
cargo test -p magpie-claims --locked --no-fail-fast  # fresh rerun; 281 passed
python tools/check_release_metadata.py
cargo package -p magpie-log --locked
python tools/verify_chain.py crates/magpie-log/testdata/golden-v1.jsonl ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c  # fresh rerun; 9 records
python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737  # fresh targeted pass; 2 records
inline weak-root Ed25519 adversarial vector  # current Python verifier accepted both forged events; A-021
git diff --check
```

No `cargo audit`/`cargo deny`, Rust-side weak-root fixture, fuzzing, cross-platform, concurrent-writer,
crash-recovery, differential malformed-record, MCP, CLI, or end-to-end foreign
Deadbolt verification tests were available or run. The portable Magpie-side
Deadbolt chain fixture was independently verified above, but that command is
not currently present in `.github/workflows/ci.yml` (A-011).

The durable report itself was checked after the current continuation edits: all
required deliverable headings, the source-level API inventory, the interface
operation matrix, and the threat coverage matrix are present; there are no
trailing-whitespace lines; every finding ID A-001 through A-024 remains
referenced; and Git reports no tracked code changes. The sole worktree addition
is this untracked audit artifact.

## Closure status and next slice

Provisional architecture closure is **not reached**. Constitutional findings
A-001, A-002, A-005, A-010, A-022, and A-024 remain open, and independent
implementer simulation still produces material currentness, admission, and
origin-authority divergence.

The smallest safe next slice is documentation governance: establish document
precedence/status, correct stale living claims, and resolve PR #103’s currentness
references. It must not change runtime, canonical bytes, vocabulary, policy
algorithms, admission, identity, or MCP.

Only after that closure should a separately reviewed L0/API boundary and
differential-verification slice be considered. Currentness, generic admission,
EpistemicGate, MCP writes, and production release claims remain blocked or
premature.

## Scope and completion matrix

| Audit scope | Evidence examined | Disposition |
| --- | --- | --- |
| 1. Constitutional corpus | ADR-0001 through ADR-0005, proposed ADR-0006, FORMAT, design contracts, README, bootstrap/worker/visual artifacts, tickets, maps, and roadmap sections | Partially closed. The dependency graph is mapped, but status, precedence, unclassified bootstrap/visual claims, and several landed-slice handoffs remain contradictory (A-002); PR #103 is not merged and still has one unresolved thread. |
| 2. Historical record and canonical encoding | Event model, canonical tags/bytes, hash/signature verification, chain replay, snapshots, golden vectors, Rust/Python verifier | Narrow L0 path is strong. Cross-tool grammar and failure-order differences remain (A-004); weak-root/strict-signature acceptance is unpinned (A-021); storage framing, sync, bounds, and anchor-field validation remain incomplete (A-008/A-014). |
| 3. Provenance and origin | Provenance fields, origin-binding/admission contexts, closure identities, source/artifact binding, Deadbolt anchor identity, episodic/query outputs | Narrow landed audits are high confidence. Detached v0-v2 standing output lacks prefix coordinates (A-003), and episodic rows/search results lose record/prefix/query identity when detached (A-019); end-to-end foreign-bundle/operational provenance remains unaudited. |
| 4. Evidence architecture | Evidence kinds/domains, support/refutation lanes, exact target/scope matching, ceilings, independence groups, invalidation and contradiction wording | Landed v0-v4 lanes are enforced by fixtures and private replay composition. General invalidation/currentness and broader contradiction policy remain deferred; no general facet-composition contract exists. |
| 5. Standing | Standing v0-v4 policies, coordinates, traces, blockers, legacy status quarantine, canonical vectors | Landed policies are coherent. v0-v2 detached coordinates are incomplete (A-003), and v2/v3 wildcard promotion is a future compatibility hazard (A-005). |
| 6. Currency and lifecycle | ADR-0003/0004/0005, withdrawal acts, lifecycle fields, standing currentness substrate, independent implementer simulation | Not closed. Currentness target, order, scope, actor semantics, cycles, output, and coordinates diverge (A-010); withdrawal self-ownership also lacks an identity coordinate (A-016). Existing `StandingCurrentness` remains compatibility-only/unknown. |
| 7. Resolver ownership and composition | Policy-specific resolver modules, replay context ownership, cross-facet wording, v3/v4 handoffs | Narrow owners are identifiable; general currentness and admission ownership are deferred. No ambient shared eligibility authority was accepted as current doctrine. |
| 8. Admission boundary | Origin-admission contracts, AgentProposer/MCP contracts, LogWriter, storage capability, readiness gates | Correctly not implemented as a generic runtime. Readiness is not complete; raw public storage mutation is a separate append seam (A-001). |
| 9. Deadbolt seam | Deadbolt anchor contract, anchor index/context, fixture verification, replay integration | Magpie-side occurrence/inclusion boundary is coherent and exact-match fail-closed. Anchor-field validation is asymmetric (A-014); end-to-end Deadbolt initialization, key lifecycle, reconciliation failure, and foreign verification were not executable-tested in this audit. |
| 10. Public Rust API | Reexports, event/provenance/standing/replay/projection types, policy/audit/closure/predicate families, compile-fail docs, legacy fields, and the source-level declaration scan below | Source-level inventory is complete at the audited `HEAD`; public raw append, unverified projection, fabricatable v0-v2 resolutions, raw legacy projection status, and detached episodic/query provenance remain (A-001/A-009/A-015/A-017/A-019); compatibility fields must not be repurposed. |
| 11. CLI and MCP | MCP boundary/capability/librarian/AgentProposer contracts, repository binaries and interfaces | No live CLI/MCP writer was found; absence is intentional, not a defect. Future read/write readiness remains unaudited at runtime and must not be inferred from proposed contracts. |
| 12. Security and authority | Append paths, identity/attribution, provenance, Deadbolt, path handling, resource exposure, policy/snapshot substitution | Verified replay is fail-closed relative to the supplied key. Raw append, unverified projections, fabricated compatibility resolutions, raw legacy status, detached episodic/search outputs, weak-root acceptance, unbounded ingestion, and conditional arbitrary SQLite migration remain risks (A-001, A-008, A-009, A-012, A-015, A-017, A-019, A-021). Withdrawal identity is future governance (A-016). |
| 13. Replay and determinism | One-read verified replay, projections, prefix identity, duplicate/cycle/missing-data reasoning, cache and serialization paths | Core verified replay is deterministic for a fixed snapshot. Reused projection state, detached snapshot/row/search outputs, parser drift, query-engine drift, and projection panic behavior remain gaps (A-003/A-004/A-007/A-019). |
| 14. Tests and fixtures | Unit/integration/adversarial tests, compile-fail docs, golden vectors, replay and cross-policy vectors | Landed slices have strong targeted vectors. Cross-tool, weak-root/strict-signature, crash/concurrency, resource, typed episodic-status/provenance, detached-query, and intended compile-fail-boundary tests are rollout gates; A-018 confirms some current compile-fail cases do not prove their stated boundary. |
| 15. Dependencies and supply chain | Cargo manifests/lockfile, CI, release/package checks, unsafe/build/toolchain assumptions | No current semantic dependency change was found; no audit/deny/MSRV/fuzz policy is enforced (A-011). |
| 16. Performance and resource bounds | Full-file storage read, replay allocation, opaque metadata, graph/projection behavior, SQLite migration | Safety-relevant bounds are incomplete (A-008); no premature optimization finding was raised for bounded landed policies. |
| 17. Development and governance | Branch/base state, PR #103, review threads/reviews, CI and owner-ratification posture, ignored local delegation queue | Main is clean and validated; currentness PR is open/non-draft with one unresolved doctrine thread and no formal approval. Review debt and conditional workflow debt remain (A-002/A-011/A-020). |

The matrix shows why this remains a provisional audit rather than a closure
approval: several scope areas are intentionally deferred, while A-001, A-002,
A-005, A-010, and the future identity boundary in A-016 are concrete
unresolved constitutional or authority questions.

## Architecture map

| Component | Inputs | Outputs | Authority owned | Authority explicitly not owned | Coordinates | Governing source | Runtime implementation | Enforcement |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| External material | Bytes, references, claimed source/origin metadata | Proposal candidate | None; presence is not authority | Admission, truth, evidence, standing, identity | Caller-supplied material coordinates only; not yet an authoritative snapshot | Origin/admission explorations and invariant floor | No generic importer | None; future readiness gate |
| Proposed admission | External material plus a governed proposal/provenance envelope | Governed admission decision or typed rejection | Only the declared admission decision | Endorsement, truth, evidence interpretation, standing, trust, identity, execution permission | Future admission policy/rule version, source coordinates, verified snapshot context where required | `docs/design/origin-admission-audit-v0.md`, AgentProposer and MCP contracts | No generic path | None; intentionally deferred |
| `LogWriter` | Validated payload, provenance, signing key, sequence/tip state, clock, store | Signed append to L0 | Append/signing and chain continuation | Admission policy, truth, standing, currentness, identity, trust | Canonical profile, event fields, sequence, predecessor, key/signature domain, recorded timestamp | `docs/FORMAT.md`, ADR-0001, `crates/magpie-log/src/logimpl.rs` | Implemented; low-level public capability | Payload validation, hash/signature checks; admission is not enforced |
| Historical log / L0 | Signed records and storage bytes | Replay input and verified prefix | Historical record only | Projection, evidence, standing, currentness, query authority | Canonical bytes, sequence, predecessor, signature, verifying key/root; complete/atomic selected-prefix semantics are not explicit at `LogStore` | `docs/FORMAT.md`, README/release replay contract | `FileStore`, `MemStore`, `LogReader` | Chain/hash/signature/genesis verification and golden vectors; storage snapshot semantics/resource bounds remain A-008, weak-root/strict-signature acceptance is not pinned (A-021), and raw store bypass remains A-001 |
| Provenance and origin | Verified records, source/artifact identity, closure/anchor data | Attribution, binding, origin/admission/contribution audits | Declared provenance/binding conclusions under their named contract | Trust, identity authentication, standing, truth | Top-level audits carry policy/profile, prefix, closure, and artifact/source coordinates; direct matched traces omit producing prefix/closure identity (A-023); claimant-controlled authority labels can transfer groups into corroboration without an independent authority binding (A-024) | Provenance/origin design contracts | Narrow typed contexts and audit resolvers | Private construction, strict parsers, fixtures; detached direct-trace coordinate binding and independent authority binding are not enforced |
| Evidence interpretation | Verified snapshot, exact typed evidence/edges, policy and resolver inputs | Eligibility/contribution/audit facets | Policy-relative interpretation and ceilings | History mutation, truth, other resolver facets, implicit endorsement | Snapshot/prefix, policy IDs/versions, resolver identity, closure where used, exact target/scope; external corroboration needs an independently verified authority coordinate (A-024) | ADR-0002 and landed v0-v4 contracts | `magpie-claims` policy/resolver modules | Closed enums, exact matching, hostile vectors, private replay composition; claimant-controlled authority separation is not independently checked |
| Standing resolver | Verified replay snapshot, inherited/eligible contributions, named standing policy | Typed standing, blockers, trace, legacy quarantine | Standing under its own policy | Currentness, truth, provenance trust, other facet authority | Snapshot, policy version/ID, resolver identity, prefix/closure where carried; corroboration authority binding is incomplete (A-024) | ADR-0003 and policy v0-v4 contracts | `standing.rs`, `standing_v1` through `standing_v4` | Exact policy tests; v0-v2 detached coordinate gap, v2/v3 wildcards, and claimant-controlled corroboration remain |
| Currency/currentness resolver | Recorded acts/relationships plus a verified snapshot and future policy | Future per-material currency/currentness facet | None currently; future policy would own only its declared facet | Standing, truth, history mutation, time semantics unless explicitly amended | Not yet fully governed; target, scope, order, actor, stable resolver-to-algorithm binding, policy/resolver and output coordinates are open | ADR-0003/0004/0005; candidate ADR-0006 in PR #103 | No runtime resolver | `Unknown` compatibility field only; no currentness authority |
| Librarian/query projection | Verified snapshot, explicit query, projection/index state | Read-only query result with provenance | Query/projection selection only | Append, admission, evidence, standing, truth, permission | Explicit snapshot and query/projection version; future contract | Proposed librarian/MCP contracts | No live librarian; direct `EpisodicView` rows/search results are derived but omit detached record/prefix/query identity (A-019) | No runtime enforcement; must remain a future gate |
| MCP boundary | Caller request, capability description, read/query context | Future typed read result or proposal | Transport and declared read/proposal boundary only | Permission from capability presence, hidden writes, admission bypass, generic execution | Explicit operation, snapshot, policy, and provenance coordinates | Proposed MCP boundary/capability contracts | No live MCP server | None; proposal text only |
| `AgentProposer` | Agent-produced material and provenance | Proposal for governed admission | Proposal formation only | Append, admission decision, identity, trust, standing, execution | Proposal/run/source coordinates; future contract | Proposed AgentProposer contract | No runtime implementation | None; proposal-only |
| Deadbolt seam | Deadbolt-verified bundle/anchor operation and exact occurrence data | Anchor occurrence/inclusion record and replayable anchor identity | Occurrence/inclusion at the seam | Interpretation, truth, foreign-bundle verification ownership, generic writes | Anchor root/profile, bundle identity, occurrence sequence, source coordinates | ADR-0001 and Deadbolt seam/anchor contracts | `deadbolt_context.rs`, anchor fixtures | Exact matching and fixture verification; end-to-end operational seam not fully tested |
| Human ratification | Candidate doctrine, corpus audit, reviews, CI, owner decision | Merge/rate/status decision | Sole doctrine ratification and merge authority | Runtime standing, automatic truth, agent permission | Owner, document/commit/PR, review and CI evidence | AGENTS and governance process | Human/process only | Review/CI/prose; not mechanically enforced |
| Future `EpistemicGate` | Proposal, structural/provenance checks, explicit admission policy | Admission decision handed to `LogWriter` or typed rejection | Admission only | Truth, endorsement, evidence assignment, standing, source ranking, identity, generic execution | Named gate/rule version, source/artifact coordinates, verified snapshot/prefix and immutable config as applicable | Readiness-gate contracts; not yet ratified | Not implemented | Must be a separate future contract; no current path may imply it exists |

The map exposes two current implementation/API authority breaks—the storage
backend can append without `LogWriter`, and public projections can accept
unverified events—and one current provenance authority gap: exact claimant-
controlled labels can be consumed as corroboration separation without an
independently verified authority binding (A-024). It also distinguishes absent
future components from implementation defects.

## Interface operation boundary matrix

Because the repository has no live CLI or MCP service, the rows below classify
the current low-level operations and the proposed interface operations without
turning proposal text into permission.

| Operation | May read | May derive | May write | Authorizer/owner | Provenance returned | Caller must not infer |
| --- | --- | --- | --- | --- | --- | --- |
| `LogReader` verified replay | Store records under the supplied verifying key | Verified events, prefix summary, and projection state | None | `LogReader` verification contract | Prefix count/tip and projection-owned coordinates | A projection is history, or a verified occurrence is a verified interpretation |
| `LogReader::events` / public `Projection::apply` | Parsed records or caller-supplied `SignedEvent` values | Unverified projection state | None to L0, but caller can mutate the projection | No verified-input owner on this public seam | None beyond the caller's supplied values | Parsed or projected data is verified (A-009) |
| Proposed librarian/query read | Explicit verified snapshot, query, and named projection | Query result and provenance response | None | Future query/projection contract | Snapshot/prefix, query, projection/schema, and policy coordinates as applicable | Read capability permits append, standing, trust, or hidden cache writes |
| Proposed MCP read | The operation's explicitly allowed read context | Typed read result only | None | Future MCP boundary plus its underlying read owner | Same explicit snapshot/query/provenance envelope | Capability description is permission or tool output is historical truth |
| Proposed `AgentProposer` | Agent-produced material and asserted provenance | Proposal envelope | None | AgentProposer forms a proposal; future admission owns the decision | Proposal/run/source/artifact coordinates | Proposal is admitted history, evidence, standing, identity, or execution permission |
| Future admission / `EpistemicGate` | Proposal plus structural/provenance inputs named by its contract | Typed admission decision | At most a handoff of an accepted payload to `LogWriter`; no raw bypass | Future named admission rule and `LogWriter` append capability | Admission policy/rule, source/artifact, and verified context coordinates | Admission is truth, endorsement, evidence, standing, trust, identity, or generic execution |
| Deadbolt specialized anchor | Deadbolt-verified bundle operation and exact replay occurrence | Inclusion/occurrence context | Only the specialized anchor event through `LogWriter` after foreign verification | Deadbolt foreign verifier plus Magpie chain verification | Root/profile/bundle/run/sequence/tip identity | Anchor presence proves foreign interpretation, truth, trust, or generic write authority |
| Public `LogStore::append_record` | Caller-supplied raw bytes | None | Raw backend bytes | No admission owner; backend mutation only | None | Backend presence is accepted history (A-001) |

## Fresh independent-pass adjudication

Six independent read-only passes were collected without consensus-building:

- The constitutional pass corroborated A-002 and A-010 and found the same
  stale status/deferral family in the living corpus.
- The runtime pass corroborated canonical-byte/golden-vector strength, A-003,
  A-004, A-006, A-008, and A-009.
- The replay pass corroborated A-003/A-004/A-007 and added the important detail
  that valid `u64` timestamps above `i64::MAX` can panic in the episodic
  projection.
- The security pass corroborated A-001/A-008/A-009 and separated raw
  attribution from identity; it did not treat proposed MCP/AgentProposer text
  as runtime.
- The compatibility pass corroborated A-003/A-005/A-006, identified public
  legacy and pre-1.0 surfaces that must remain quarantined, and isolated the
  detached `EpisodicEvent` provenance gap recorded as A-019.
- The hostile implementer independently reproduced the A-010 divergence
  matrix: claim versus assertion target, sequence versus timestamp order,
  absence semantics, eligible acts, scope, actor authority, cycles, output,
  and coordinates.

The constitutional pass also identified stale landed-slice handoffs beyond the
already-recorded ADR status conflict: ADR-0002's future-tag wording, the
origin-admission and v3 runtime status blocks, the v4 ceiling note, and
Ticket-0063 “current candidate” language. These are adjudicated as the A-002
status/precedence family, not separate architecture IDs. The hostile and
compatibility passes independently confirmed that public legacy status fields
remain easy to mistake for governed standing; this is recorded as A-017. The
identity pass's withdrawal self-ownership concern is recorded as the future
governance finding A-016.

The continuation corpus sweep added the unclassified bootstrap/visual-ledger
family to A-002: `magpie-bootstrap.md`, `CLAUDE.md`, and
`docs/magpie-architecture-ledger.html` contain stronger gate, bi-temporal, or
build-now language than the current ADR/FORMAT/runtime boundary. The explicit
Proposed/docs-only ADR-0002 HTML dossier is classified as historical support,
not as a separate conflict. This is a status/precedence expansion, not a new
runtime finding.

No independent pass established a new Critical finding or overturned a
confirmed finding. The following conditional availability issue is retained:

### A-012 details

- **Severity:** Medium, conditional on an untrusted or attacker-influenced
  path; not a current MCP risk because no MCP runtime exists.
- **Category:** Security/availability/derived-state boundary.
- **References:** `crates/magpie-episodic/src/lib.rs:21-36,96-117`.
- **Runtime surface:** `EpisodicView::at_path` opens any caller-provided path;
  `from_connection` drops `events_fts` and `events` when `user_version` differs,
  but trusts a matching version without an ownership marker or schema
  fingerprint.
- **Invariant:** A rebuildable projection is not history and a convenience
  interface must not acquire unrelated destructive authority.
- **Failure scenario:** A caller supplies a path or symlink to an existing
  SQLite file with a different schema version. Opening it can execute the
  migration branch and delete same-named tables, damaging unrelated derived or
  user data even though the append-only L0 log is untouched. Conversely, an
  unrelated file that happens to report `user_version = 2` bypasses migration;
  `CREATE TABLE IF NOT EXISTS` does not prove that its existing `events` table
  has Magpie's schema, so later reads/inserts can consume or mutate unrelated
  data under a trusted-looking projection API.
- **Smallest remediation:** require an owned Magpie database marker and refuse
  arbitrary existing databases, or explicitly restrict the API to a validated
  application-owned path and make mismatched-schema handling non-destructive.
- **Do not introduce:** a generic permission system, hidden database authority,
  or any change that treats SQLite contents as historical truth.
- **Correction locus:** separate security/API code PR or an explicit storage
  contract; audit ledger status is confirmed-conditional.

The compatibility pass additionally confirmed A-018: several compile-fail
examples in `crates/magpie-claims/src/origin_binding_verifier.rs:43-70`,
`origin_admission_audit.rs:80-160`, and
`support_contribution_audit.rs:74-169` appear to fail on stale argument
counts or removed method shapes rather than the intended forbidden
construction/substitution boundary. The examples still passing does not prove
the architectural boundary they claim to test.

## Replay and determinism assessment

| Output or state | Coordinates actually present | Hidden or missing input | Assessment |
| --- | --- | --- | --- |
| Canonical L0 event/hash/signature | Canonical profile, ordered event fields, sequence, predecessor, payload tag, timestamp, key/signature domain | Default writer clock varies creation; this is recorded history, not a replay input | Replay of fixed bytes is deterministic; creation fixtures should inject a clock |
| Verified prefix identity | Event count and verified tip hash in `VerifiedLogPrefixIdentityV0` | Trust-root/key context is caller-supplied and must remain explicit at embedding boundary | Suitable narrow prefix coordinate; do not substitute projection bytes |
| `StandingReplaySnapshot::canonical_bytes()` | Event count and derived projection comparison bytes | Verified tip and full chain identity omitted by design | Comparison-only; unsafe as detached authority identity (A-003) |
| Standing v0-v2 detached outputs | Policy ID and standing/currentness/trace fields | Verified prefix identity and consistently named resolver identity are absent | Incomplete coordinate carriage (A-003) |
| Standing v3-v4 outputs | Policy-specific outputs, verified prefix identity, closure identity, traces | Resolver identity remains implicit unless policy ID is explicitly governed as that identity | Stronger than v0-v2; wording must pin resolver identity before reuse |
| Provenance/origin/contribution audits | Named schema/profile/policy IDs, prefix and closure identities where applicable | General source availability and trust remain declared inputs, not inferred authority | Narrow outputs are reproducible; source binding must not be detached |
| `LogStore::read_records` snapshot | Backend-provided record vector | The trait does not require append order, completeness, atomic visibility, or an explicit selected-prefix/staleness coordinate; current File/Mem stores preserve order but do not establish the swappable-backend contract | A valid stale prefix must not be silently presented as the current complete log; define the storage snapshot contract before caches or alternate stores become authority-bearing (A-008) |
| Projections/caches | Rebuild from a replayed log/snapshot | Freshness, reset, and cache prefix binding are not a general API contract | Derived-only; never a shadow history; A-007/A-008 gates remain |
| Future currentness | No runtime output | Subject, order, scope, actor authority, stable resolver-to-algorithm binding, policy/resolver, conflict and encoding coordinates | Blocked on A-010; `Unknown` is the only safe compatibility result |
| Rust/Python verification result | Input file and expected tip in CLI invocation | Accepted grammar, blank-line policy, hex case, numeric status, JSON constants, and first-failure phase differ | Cross-tool determinism is incomplete (A-004) |

No current standing result was found to consult environment variables, locale,
timezone, network state, mutable trust databases, or ambient latest policy.
The remaining hidden-input risks are primarily public API conventions, storage
state, parser grammar, and unbound future resolver choices.

## Test and enforcement assessment

### Enforced in the landed narrow slices

- Canonical field order, fixed tags, hash/signature domains, and golden bytes.
- Verify-before-replay and failure before projection, including hostile partial
  replay tests.
- Closed event/evidence/domain/edge vocabularies and strict nested parsers.
- Exact target/scope matching and support/refutation matrix behavior in the
  landed policy v0-v4 fixtures.
- Private same-replay composition for the current standing policy paths.
- Prefix/closure vectors for the newer policy/audit outputs.
- Raw legacy standing quarantine characterization tests.

### Prose-only or API-convention enforcement

- `LogWriter` as the sole intended append authority; A-001 leaves a public
  backend mutation seam.
- Projection freshness/reset and cache binding; A-007.
- Public unverified projection/Deadbolt APIs being treated as untrusted;
  A-009.
- Attribution not being identity, and capability description not being
  permission; current contracts state this but no generic runtime exists.
- Claimant-controlled origin groups and exact authority labels not being
  independently verified corroboration authority (A-024); current code copies
  the claimed group into the trusted fold after exact string matching.
- Human ratification, document precedence, and review independence; A-002 and
  workflow policy are not mechanically enforced.

### Future rollout gates, not missing current tests

- Currentness/currency representation and resolver tests under A-010.
- Generic admission/EpistemicGate readiness and AgentProposer/MCP write
  prohibitions.
- Rust/Python differential malformed-input vectors and grammar decision.
- Crash/concurrency/partial-write and L0 size/metadata/resource tests.
- `LogStore` append-order, complete/atomic snapshot, and explicit
  selected-prefix/staleness semantics (A-008).
- Valid oversized timestamp handling and fallible projection application.
- Typed `ClaimAssertedV2` episodic status regression (A-006).
- Detached episodic/query provenance binding to verified prefix and record
  identity (A-019).
- Independent authority binding and self-declared-group non-corroboration
  tests for external origin corroboration (A-024).
- Compile-fail cases that fail for the forbidden authority surface itself rather
  than stale method arity or removed signatures (A-018).
- End-to-end Deadbolt foreign-bundle verification, key initialization, and
  reconciliation-failure tests.
- Dependency audit/deny, MSRV/toolchain, fuzz, and cross-platform gates.

Passing existing tests is therefore evidence for the landed narrow slices,
not evidence that the full constitutional floor is enforced.

## Review-debt assessment

Live GitHub state at the audit point:

- PR #103 is open, non-draft, mergeable, based on `main` at
  `3b46fdb81857d77b827271777b86745bca5f7b12`, with head
  `b6cc4dc5ff75672b3b9e5a879ee4f2e749faa5b7` and three changed files.
- One inline review thread remains unresolved and current at
  `docs/adr/0006-claim-currency-and-currentness-semantics.md:425-429`; it
  requests retirement of remaining ADR-0003/ADR-0005 currentness deferrals.
- Earlier Codex review threads are resolved/outdated, but their review
  submissions are comments rather than formal approvals. No owner ratification
  or merge is inferred.
- Main has no local code changes; the only audit artifact is this untracked
  ledger. No GitHub mutation was performed.

The review debt is therefore not “no comments”; it is unresolved constitutional
status, no formal approval on a doctrine PR, and the absence of independent
hostile review for future admission/MCP/currentness runtime surfaces.

The ignored local delegation state adds conditional workflow debt: pending
tickets 0003 and 0006 contain obsolete base-state assumptions, while
`scripts/codex-delegate.ps1` does not preflight ticket status, base identity, or
changed-path scope before write-mode invocation (A-020). No automatic queue
runner or repository-canonical authority was found, so this does not expand
the runtime findings; it requires ticket retirement or explicit revalidation
before any future delegated write.

## Active PR #103 delta audit

The current PR head was inspected independently from `main`:

- `README.md` and `docs/adr/0004-claim-lifecycle-facets.md` reclassify
  currentness/currency semantics as governed by the new ADR while deferring
  representation, concrete eligibility, identity/actor authority, algorithms,
  encoding, and runtime.
- `docs/adr/0006-claim-currency-and-currentness-semantics.md` closes several
  doctrine axes: currentness is per recorded material, resolver-relative and
  snapshot-relative; time does not create currency; supersession is lineage;
  invalidation is evidence-scoped and non-self-executing; and standing and
  currency consumers own their own eligibility coordinates.
- The same candidate explicitly leaves payload representation, invalidation
  eligibility, actor authority/identity, output encoding, resolver algorithms,
  and runtime implementation for later governance. The hostile implementer
  divergence therefore remains valid for implementation readiness.
- The candidate is still `Status: Proposed`, while README/ADR-0004 wording
  calls its boundaries settled. That is a current instance of A-002, not an
  implicit ratification.
- The unresolved current thread at
  `docs/adr/0006-claim-currency-and-currentness-semantics.md:425-429`
  correctly identifies the remaining stale ADR-0003/ADR-0005 deferrals. It
  must be resolved in the candidate or explicitly classified as an intentional
  historical boundary before merge.
- Head `b6cc4dc5ff75672b3b9e5a879ee4f2e749faa5b7` has successful `verify`
  runs #30711636782 and #30711638299, plus a successful CodeRabbit status.
  Green CI proves the
  documentation patch builds through repository checks; it does not prove
  doctrine ratification, implementation readiness, or absence of constitutional
  divergence.

Disposition: safe as a documentation candidate after the named corpus/status
remediation; not an approval, not a runtime authorization, and not evidence
that currentness implementation can begin.

## Roadmap judgment

| Candidate next step | Judgment | Gate |
| --- | --- | --- |
| Corpus status/precedence registry and stale-status correction | Safe after named remediation | Owner decision on Proposed versus Accepted, explicit historical labels, and fresh hostile corpus review |
| PR #103 currentness-doc follow-up | Safe after named remediation | Resolve the live ADR-0003/ADR-0005 references without silently ratifying runtime or identity semantics |
| Rust/Python grammar and differential fixtures | Safe after named remediation | Freeze acceptance grammar and first-failure contract; preserve canonical binary bytes |
| L0 storage/API boundary hardening | Safe after named remediation | Separate API/storage contract, compatibility review, bounds/durability decision, and compile-fail tests |
| Deadbolt operational hardening | Safe after named remediation | End-to-end seam contract, key/reconciliation tests, and proof that the specialized seam does not create generic write authority |
| External origin authority/corroboration binding | Blocked on doctrine | A-024 requires an owner-ratified authority-binding coordinate or an explicit rule that claimant-controlled groups cannot corroborate; add hostile two-group vectors only after that contract exists |
| Currentness/currency resolver implementation | Blocked on doctrine | A-010 target/order/scope/actor/coordinate/encoding closure and owner review |
| Generic admission or `EpistemicGate` | Blocked on doctrine and readiness | Complete reviewed admission questions, including A-022's representation/rejection/retry answers; no truth, evidence, standing, identity, or generic execution authority |
| MCP/librarian read surfaces | Premature as runtime | First ratify query/provenance/error/snapshot contract, bind returned rows to record/prefix identity (A-019), and preserve read-only boundary |
| MCP/CLI writes, agent identity, delegation, trust scoring | Premature | Explicitly prohibited by the current audit scope until separate governance |
| Production release/operational readiness claim | Premature | A-001/A-008/A-011 and review-debt closure |

## Recommended next architectural slice

The smallest safe next slice is a documentation-governance slice:

1. establish one owner-approved status and precedence registry for ADRs,
   contracts, historical tickets, and reference maps;
2. correct only unlabelled living stale-status claims, including the remaining
   currentness references identified by PR #103;
3. preserve frozen historical wording and do not silently rewrite Proposed
   doctrine into Accepted doctrine; and
4. add a corpus-dependency/status check that fails when a landed ticket is
   still presented as future without an explicit historical label.

It is ready because it changes no runtime, canonical bytes, tags, evidence
kinds, edge kinds, resolver algorithm, admission mechanism, identity model, or
MCP behavior. It must not:

- ratify ADR-0002 through ADR-0006 by implication;
- implement currentness, generic admission, `EpistemicGate`, or facet
  composition;
- repurpose legacy fields or alter compatibility vectors; or
- resolve review threads or change PR metadata without owner authorization.

This documentation slice does not make the current external-corroboration path
safe to treat as independently identity-backed. A-024 remains a separate
future authority/provenance gate: until it is ratified and implemented, the
claimed authority pair and origin-group labels must not be read as verified
independence outside their narrow existing audit output.

Required review depth is one fresh hostile corpus review, one independent
runtime/documentation comparison, owner review of status/precedence, and clean
CI plus link/status checks. Containment is simple: the slice is documentation
only, can be reverted as one bounded commit by the owner, and leaves the
current runtime untouched.

## Canonical corpus dependency and status map

This map distinguishes authority from summaries, explorations, historical
records, and candidate contracts. A document that says another document is
“ratified” does not change the referenced document's repository status.

| Corpus node | Status at audited `main` | Depends on / amends | Owns or records | Runtime handoff and audit disposition |
| --- | --- | --- | --- | --- |
| `docs/adr/0001-deadbolt-seam.md` | Accepted | Deadbolt survey and seam evidence | Deadbolt/Magpie anchored hierarchy, tag-5 anchor semantics, two-step verification | Magpie-side tag-5/runtime and fixtures landed; Deadbolt emitter/reconciliation remain future. The named `seam-slice.patch` is absent (A-013). |
| `docs/adr/0002-governed-claim-memory.md` | Proposed | Amends ADR-0001 | Typed claim memory, evidence ceilings, governed write-path intent, derived standing boundary | Tags 6–8 and narrow policy/runtime slices exist, but the ADR status and stale future-tag wording remain unresolved (A-002). |
| `docs/adr/0003-canonical-definition-of-standing.md` | Proposed | Consolidates ADR-0002 and standing policy contracts | Standing definition and snapshot/policy/resolver coordinate law | v0–v4 implement narrow standing; currentness is still deferred and v0–v2 detached output is under-coordinate (A-003/A-010). |
| `docs/adr/0004-claim-lifecycle-facets.md` | Proposed | ADR-0002/0003 | No stored lifecycle state; acts and relationships are interpreted as facets | Runtime retains only compatibility currentness and narrow lifecycle vocabulary; currentness/currency remains future. PR #103 proposes a successor boundary but is not merged. |
| `docs/adr/0005-claim-withdrawal-acts.md` | Proposed | ADR-0004 | Actor-local withdrawal act and non-falsity distinctions | No generic withdrawal runtime or identity/delegation authority; currentness and standing effects are deferred. Its remaining deferral wording is the unresolved PR #103 thread. |
| `docs/seams/deadbolt-anchor-contract.md`, `docs/design/deadbolt-occurrence-context-v0.md`, `docs/design/deadbolt-occurrence-standing-v1.md` | Seam contract plus policy/design notes; status varies by file and the v1 note lacks a top-level status block | ADR-0001 and verified replay precondition | Exact anchor identity, occurrence/inclusion context, and the narrow structured proposition that may be settled | Magpie anchor fixtures and private replay context exist; occurrence is not foreign interpretation or generic write authority. The missing status is part of A-002. |
| `docs/design/artifact-provenance-origin-admission.md`, `artifact-acquisition-derivation-bundle-v0.md`, `origin-binding-bundle-v0.md`, `resolution-content-closure-v0.md` | Ratified narrow architecture/protocol contracts | ADR-0002/0003 and preceding provenance tickets | Artifact/source identity, derivation bundle, closure identity, and standing-inert binding surfaces | Narrow parsers/verifiers/audits landed; no trust, identity, loader, CAS, network, or admission authority is created. |
| `docs/design/replayable-deterministic-verifier-admission-v0.md`, `origin-admission-audit-v0.md`, `admitted-contribution-audit-v0.md`, `support-contribution-audit-v0.md` | Ratified contracts; origin-admission audit runtime explicitly future, contribution runtimes landed | Provenance stack, verified replay substrate, ADR-0002/0003 | Deterministic verifier/admission-audit and contribution-audit boundaries | Verified replay, audit contexts, and narrow policy consumers exist. “Admission” here is not a generic write mechanism; status wording must remain explicit (A-002). |
| `docs/design/standing-view-evidence-ceilings.md`, `standing-aggregation-independence-groups.md`, `standing-contribution-lanes-v1.md`, `standing-policy-v3-external-report-corroboration-v0.md` | Living doctrine/ratified policy notes with mixed historical handoff text; the v1 lane note lacks a top-level status block | ADR-0002/0003, contribution and provenance contracts | Ceiling law, origin-group corroboration, v1 lanes, and v3 narrow positive rule | v3 runtime is landed, but stale “future” passages remain. Broader aggregation, contradiction debt, and currentness remain future (A-002). |
| `docs/design/claim-inline-subject-binding-v0.md`, `claim-inline-sha256-predicate-v0.md`, `inline-predicate-attestation-binding-v0.md`, `claim-inline-predicate-edge-lanes-v0.md` | Ratified contracts; standing-inert substrate landed | Typed tags, exact predicate/attestation/edge contracts, ADR-0002/0003 | Claim-owned subject/predicate/attestation/edge eligibility and neutral outcomes | Tickets 0057–0063 and PRs #76–#78 landed; `SupportEligible`/`RefutationEligible` remain standing-inert until a named policy consumes them. |
| `docs/design/standing-policy-v4-claim-inline-direct-refutation-v0.md` | Ratified narrow policy contract | Claim-inline lane contracts and standing v3 handoff | Exact claim-inline direct-refutation rule and its ceiling | Ticket 0065/PR #79 runtime landed; it does not make all contradiction debt or generic `contradicts` edges authoritative. |
| `docs/design/standing-policy-v4-deterministic-direct-refutation-v0.md` | Withdrawn/blocked candidate; no active policy identity | Earlier v4 candidate and later claim-inline contract | Historical rejected/blocked candidate record | Must not be used as current policy doctrine; preserved to prevent rediscovery. |
| `docs/design/historical-review-remediation-ledger.md` | Ratified documentation/audit contract | Review history and owner decisions | Historical-versus-living wording and remediation evidence | It is a ledger, not a substitute for ratifying ADR status; its frozen historical entries must remain labeled. |
| Admission set: `admission-boundary-questions.md`, `admission-boundary-threat-model.md`, `admission-boundary-scenario-analysis.md`, `admission-boundary-minimal-semantics.md`, `admission-design-readiness-checklist.md`, `admission-record-boundary-contract.md` | Docs-only explorations/proposed boundary; no mechanism | ADR-0002 and declared readiness constraints | Questions, threats, scenarios, and readiness gates; no decision or schema | No generic admission runtime exists. These files cannot authorize an implementation or append path. |
| Agent/MCP/query set: `agent-proposer-boundary-contract.md`, `mcp-boundary-contract.md`, `mcp-capability-boundary-contract.md`, `mcp-librarian-contract.md`, `verified-librarian-query-contract.md`, `provenance-response-contract.md` | Proposed boundary contracts; no live service | ADR-0002/0003 and future read/admission governance | Future proposal/read/query/provenance boundaries | The files repeatedly state no server/librarian/implementation exists; references in older exploration text calling MCP contracts ratified are stale (A-002). |
| Boundary/reference set: `epistemic-boundary-map.md`, `epistemic-invariant-ledger.md`, `epistemic-pipeline-separation-contract.md`, `epistemic-resolution-boundary-contract.md`, `evidence-admission-boundary-contract.md` | Reference artifacts or proposed explorations | All governing ADRs and landed policy contracts | Navigation, invariant indexing, and future boundary questions | Explicitly non-governing or unaccepted; they must reflect, not amend, canonical doctrine. |
| Supporting notes: `metadata-conventions.md`, `adr-0002-tags-6-8-implementation-plan.md`, `governed-standing-tour-v1.md`, `standing-policy-v4-deterministic-direct-refutation-v0.md`, `docs/design/adr-0002-governed-claim-memory-dossier.html` | Proposed/historical/executable-tour/blocked-candidate/docs-only companion as stated in each file | ADR-0002 and policy contracts | Metadata convention, implementation history, executable tour, rejected candidate, or explanatory dossier | The dossier explicitly says Proposed and docs-only; future tag language is historical. None may silently define new event/evidence/edge authority. |
| Bootstrap/visual/worker artifacts: `magpie-bootstrap.md`, `AGENTS.md`, `CLAUDE.md`, `CLAUDE.local.md`, `docs/magpie-architecture-ledger.html` | Bootstrap, worker guidance, local personal preferences, and visual architecture material; not an ADR or format authority | Initial thesis, `docs/FORMAT.md`, and the later ADR/contract corpus | Project context, implementation guidance, local workflow preference, or aspirational architecture narrative | Their gate/currentness/bi-temporal language is not implementation authority; `CLAUDE.local.md` cannot amend doctrine or worker scope; A-002 requires explicit status and precedence labeling before reuse. |
| Supporting boundary/release/survey docs: `docs/portable-base.md`, `docs/deadbolt-witness-survey.md`, `docs/releases/v0.1.0-contract.md`, `docs/releases/v0.1.0-release-metadata.md`, `CHANGELOG.md` | Explicit portable-base, static-survey, release-contract, or milestone-record support documents | ADR-0001, FORMAT, and release policy where named | Optional seam limits, historical survey facts, compatibility/release commitments, and milestone history | These documents do not create generic write, trust, currentness, publication, or Deadbolt execution authority; their explicit boundaries are retained. |
| Ticket corpus (`tickets/*.md`) | Historical implementation records; not a single authority layer | The document/contract named by each ticket | Base/head, implementation sequence, compatibility and review evidence | Ticket wording is historical unless explicitly identified as current doctrine; it constrains neither ADR status nor runtime authority. |

The corpus map is therefore dependency-complete at the governing-node level,
while individual ticket files remain evidence of sequencing rather than
independent constitutional sources. The unclosed status/precedence problem is
not solved by the map itself.

## Derived-consequence ownership records

The following table applies the requested ownership test to each consequence.
“None” is intentional: it means the consequence is not currently authorized,
not that a convenience resolver may fill the gap.

| Input | Eligibility owner | Consequence owner | Coordinates | Output |
| --- | --- | --- | --- | --- |
| Support from typed evidence | Named evidence-kind/claim-domain support matrix and the policy-specific contribution lane | Named standing resolver v0–v4 when that policy consumes the contribution; otherwise the audit resolver only reports contribution | Verified replay snapshot/prefix, named policy/version, resolver identity where governed, exact target/scope, closure identity where required | Support contribution/audit; policy-relative standing such as `Supported` or `Settled` only where the named rule permits |
| Contradiction / direct refutation | Only the exact landed v4 claim-inline refutation lane for its closed relation/digest cell; broad contradiction debt has no current eligibility owner | v4 standing resolver for the exact direct-refutation rule; no general resolver owns contradiction debt | Verified prefix, v4 policy/rule identity, exact claim/evidence/relation binding, closure identity | `Refuted` for the exact rule or a typed blocker/neutral result; no generic `contradicts` promotion |
| Invalidation | None on `main`; future consuming policy must own exact target/scope and actor-authority eligibility | None on `main`; no recorded edge self-executes | Future policy/rule version, exact evidence/support target and scope, actor interpretation, verified snapshot | Recorded relationship only; support/contradiction removal is deferred and cannot be inferred from edge presence |
| Supersession | None on `main`; future currency or standing consumer must independently pin its eligibility rule | None on `main`; a relationship records lineage only | Future consumer policy/version, exact predecessor/successor/scope, verified prefix, actor interpretation if relevant | Recorded lineage only; no automatic currentness, standing discharge, or correctness |
| Withdrawal | ADR-0005 defines an actor-local historical act in doctrine, but no verified identity/delegation or active eligibility owner exists | None on `main`; standing/currentness effects remain future | Future policy, exact targeted assertion act, actor/identity interpretation, verified snapshot | Recorded act when a future representation exists; never falsity, deletion, or cross-actor suppression by presence |
| Ratification | Human owner/review process owns eligibility of a doctrine change, not a resolver | Human owner owns merge/status/rate decision | Owner identity, proposal/commit/PR, review and CI evidence, explicit status decision | Ratified or rejected governance state; no epistemic standing output |
| Standing | The selected standing policy owns the eligibility rules for every contribution it consumes; no ambient shared result is allowed | The named standing resolver/composer owns only its declared standing output | Verified snapshot/prefix, standing policy/version, resolver identity, closure and trace coordinates as applicable | Typed standing, blockers, trace, and quarantined legacy fields |
| Origin authority binding / corroboration | No current independently verified authority or identity owner; the exact-label origin-admission audit currently treats the fixed authority pair as trusted and copies the claimed group into the fold | v3 owns only the distinct-group count after receiving the contribution; no current resolver owns the missing authority binding | Authority/root/profile binding, verification method, origin-group assignment, policy/version, verified snapshot/prefix, and output coordinates | Current claimant-controlled groups must not be interpreted as independent corroboration until A-024 is closed |
| Currentness/currency | No current owner; ADR-0006 candidate proposes a future policy-owned eligibility boundary but is not on `main` | No current resolver | Subject/act target, order, scope, actor, policy/version, stable resolver-to-algorithm binding, snapshot/prefix and output encoding remain incomplete | Existing `StandingCurrentness::Unknown` compatibility output only; no currentness conclusion |
| Admission | Future governed admission mechanism must own structural/provenance eligibility; proposal formation is not admission | Future gate owns admission decision; `LogWriter` owns only append/signing | Admission policy/rule version, proposal/source/artifact coordinates, verified context and immutable configuration as applicable | Typed accepted/rejected/deferred admission decision; never truth, evidence, standing, identity, trust, or execution permission |
| Evidence interpretation | The named evidence policy/interpretation contract owns exact kind/domain/target/scope/origin eligibility | The corresponding evidence/audit resolver owns the interpretation facet; standing may consume only through its own contract | Snapshot/prefix, interpretation policy/version, resolver identity, exact bindings, source/origin/closure coordinates | Eligibility/contribution/trace; computing a facet does not make it evidence or authority elsewhere |
| Provenance response | Proposed response contract would own required provenance attachment and error disclosure | No live response resolver; future query/provenance layer only | Snapshot/prefix, policy/resolver, source/artifact/closure/anchor identity, query operation | Provenance-bearing projection; attribution remains distinct from verified identity or trust |
| Query projection/librarian | Future query contract owns allowed read/filter semantics and projection version | Future librarian/query component owns the read projection only | Explicit snapshot/prefix, query, projection/schema version, policy coordinates where a derived result is returned | Read-only result with provenance; no append, admission, standing authority, or hidden cache write |
| Deadbolt occurrence | Deadbolt owns foreign bundle verification; Magpie replay owns exact accepted-chain anchor occurrence | Magpie seam owns occurrence/inclusion context, not interpretation of foreign evidence | Anchor identity, root/algorithm/profile, bundle/run identity, sequence/tip, verified replay context | Occurrence/inclusion fact; no truth, source trust, or generic write authority |

This table shows that the missing currentness and admission rows are deferred
governance, while the public append and unverified projection seams are
implementation/API findings rather than missing policy algorithms.

## Ambiguity classification

| Ambiguity class | Current examples | Required treatment |
| --- | --- | --- |
| Policy-relative | Which named v0-v4 lane consumes an eligible contribution; exact v4 direct-refutation rule; origin-group corroboration | Keep policy/version and resolver identity explicit; do not promote a result into general standing or truth |
| Implementation-local | Projection reset/freshness, storage sync/locking, SQLite ownership, parser API names, public map mutability | Fix or contract in a narrow code/API/storage PR; preserve canonical bytes and compatibility surfaces |
| Compatibility debt | v0-v2 coordinate-poor outputs, direct matched provenance traces (`A-023`), legacy raw standing/currentness fields and public projection status (`A-017`), public unverified projection APIs, fabricatable v0-v2 resolution values, detached episodic rows/search results (`A-019`), Rust/Python grammar differences | Additive wrapper/quarantine/differential contract; never silently repurpose old shapes or fields |
| Test-enforcement debt | Compile-fail examples whose failure cause is stale API shape rather than the forbidden authority surface (`A-018`) | Rewrite narrowly around the prohibited construction/deserialization/substitution; keep the intended boundary explicit |
| Deferred governance | ADR status/precedence, currentness target/order/actor/scope/resolver binding, admission/EpistemicGate, independent authority binding behind claimant-controlled corroboration (`A-024`), identity/delegation (`A-016`), MCP/CLI writes | Owner decision and explicit ADR/contract before implementation; absence is not permission |

## Public Rust API classification

| Surface | Classification | Authority meaning and risk |
| --- | --- | --- |
| `magpie-log::{EventCore, Payload, Provenance, Sig, SignedEvent, Status, ContentHash}` (`crates/magpie-log/src/lib.rs:23-31`) | Canonical serialized/event surface | Closed payload/status vocabulary and canonical encoding are governed; provenance and actor labels remain signed attribution, not identity or trust. |
| `LogReader`, `VerifiedReplaySummary`, `Clock` | Canonical verification/replay surface | Verified replay is the authority-bearing input path; summary/prefix data is derived and must not be mistaken for a new event. |
| `LogWriter` and reexported `SigningKey`/`VerifyingKey` | Low-level append/key capability | Append/signing is authoritative for L0 bytes, but public construction exposes raw capability before future admission; Deadbolt ownership is a system boundary, not enforced by this crate (A-001). |
| `LogStore`, `FileStore`, `MemStore` | Storage implementation plus public compatibility surface | `LogStore::append_record` is a second raw append seam; stores must never become shadow history or admission policy (A-001). |
| `Projection` and `LogReader::events()` | Provisional/unverified projection surface | `events()` returns parsed but unverified records and `Projection::apply` accepts public `SignedEvent`; callers can construct a forged authority-looking projection (A-009). |
| `Claim`, `ClaimsView` (`crates/magpie-claims/src/lib.rs:154-232`) | Compatibility/derived projection | Public mutable maps, raw `Status`, evidence IDs, and transition counts are rebuildable state, not canonical standing. Names remain easy to misread and must not be repurposed (A-017). |
| `StandingView`, `StandingClaim`, `StandingCurrentness`, `StandingResolution`, typed nodes/edges | Governed narrow projection plus compatibility fields | v0 candidate and legacy fields are not general currentness/evidence authority; `StandingCurrentness::Unknown` is compatibility-only. Typed node/edge projections lack complete event identity for future lifecycle resolution (A-010). Public v0 resolution literals can fabricate the same authority-looking shape (A-015), while raw legacy status fields remain compatibility-only (A-017). |
| `StandingResolutionV1` through `StandingResolutionV4` and policy constants | Versioned governed projections | Each policy owns only its declared output. v3/v4 carry stronger prefix/closure context; v0-v2 shapes remain compatibility outputs with coordinate gaps and public fabrication risk (A-003/A-015). |
| Provenance, origin-binding, closure, admitted/support-audit types reexported by `magpie-claims` | Governed projections / typed receipts | Private construction and strict parsers constrain narrow audit conclusions; top-level audits carry prefix/closure coordinates, but direct matched traces omit them and must not be detached as complete provenance responses (A-023). Receipts do not authenticate identity, grant trust, or become standing without a separate consumer policy; claimant-controlled authority labels are not independently verified corroboration authority (A-024). |
| `policy::{EvidenceKind, ClaimDomain, PolicyParseError, SupportContextRequirement}` and ceiling/requirement functions | Closed policy vocabulary and eligibility tables | Membership, parse errors, `ALL` arrays, and ceiling lookup are policy inputs, not evidence, standing, trust, or admission authority; string parsers must remain exact and version-bound. |
| `ArtifactProvenance*`, `OriginBinding*`, `ResolutionContentClosure*`, and `*ContributionAuditV0` constructors/accessors | Coordinate-bearing provenance/closure receipts and audit projections | IDs, source locators, actor labels, authority references, and supplied object bytes remain asserted inputs or typed audit results; they do not authenticate identity or grant trust, and public constructors must not be read as admission. |
| Claim-inline predicate/attestation/edge outcome types | Governed standing-inert projections | Closed matrix outcomes and failure reasons are typed, but `SupportEligible`/`RefutationEligible` do not themselves contribute to standing. |
| `DeadboltAnchorIndex`, occurrence context, `resolve_deadbolt_occurrence_context` | Provisional untrusted helper plus private policy input path | Exact matching in a caller-created index can return `Matched`; only the verified same-replay policy context may be treated as chain occurrence. Manual construction is not verification, and public occurrence/application values can be fabricated (A-009/A-015). |
| `StandingReplaySnapshot`, `OriginAdmissionReplayContextV0`, `VerifiedLogPrefixIdentityV0` | Opaque governed replay context / coordinate surface | Construction is constrained and deserialization is absent; snapshot comparison bytes are not prefix identity. v0-v2 detached answers still lose coordinates when serialized (A-003). |
| `EpisodicEvent`, `EpisodicView` (`crates/magpie-episodic/src/lib.rs:39-117`) | Governed rebuildable query projection | SQLite is derived, not history. Rows and bare search results omit event/prefix/query identity when detached (A-019); `claim_status` can synthesize `Conjectured` for typed assertions (A-006); `at_path` has conditional destructive migration risk (A-012); public errors/panics are availability concerns. |
| CLI/MCP/librarian/AgentProposer interfaces | Proposed/future only | No `main.rs`, `bin`, `cli`, or MCP runtime surface exists under `crates`; contracts grant no current permission and cannot be treated as implementation. |

The pre-1.0 release document explicitly limits API stability, but that does
not remove the need to classify current public semantics: downstream users can
still mistake a public type for authority before a future compatibility break.

### Source-level public declaration inventory

To make the API audit reproducible, I performed a cold source scan of every
Rust file under the three published crates at the audited `HEAD`
(`3b46fdb81857d77b827271777b86745bca5f7b12`). The counts below are syntax
inventory counts, not ABI promises: direct declarations include items in
private implementation modules, while the root-exposure count is taken from
the crate's `pub use`/`pub mod` surface. Public methods are included because
their semantics are part of the callable API even when their receiver type is
opaque.

| Crate | Source inventory | Root exposure and serialized/field-bearing surfaces |
| --- | --- | --- |
| `magpie-log` | 7 Rust files; 16 direct public types/traits/aliases; 2 direct constants; 0 free functions; 26 public methods | 19 root-reexported names from `crates/magpie-log/src/lib.rs:23-31`. Field-bearing/canonical structures are `Provenance`, `EventCore`, `SignedEvent`; `Payload`, `Status`, `Sig`, `ContentHash`, `LogError`, `LogStore`, `LogReader`, `LogWriter`, `Projection`, and `VerifiedReplaySummary` are separately classified above. |
| `magpie-claims` | 20 Rust files, including the nested claim-inline lane; 101 direct public types/enums; 56 direct constants; 6 free functions; 326 public methods | 152 names in 17 root `pub use` blocks (`crates/magpie-claims/src/lib.rs:34-151`), root `Claim`/`ClaimsView` (`:154-232`), and public module `policy`. The six free functions are `resolve_deadbolt_occurrence_context`, `replay_origin_admission_context_v0`, `replay_standing_context`, `support_ceiling`, `support_context_requirement`, and `refutation_ceiling`. Field-bearing compatibility surfaces are `Claim`, `ClaimsView`, `DeadboltAnchorIdentity`, `DeadboltAnchorOccurrence`, `StandingTraceEntry`, `StandingResolution`, `StandingClaim`, `TypedClaimNode`, `TypedEvidenceNode`, `TypedJustificationEdge`, `StandingView`, the public v1/v2 application/trace/resolution structs, and their public policy/context fields; v3/v4 and receipt families expose accessors rather than public fields. |
| `magpie-episodic` | 1 Rust file; 2 public structs; 0 direct constants; 0 free functions; 7 public methods | `EpisodicEvent` is the serialized row surface and `EpisodicView` is the derived query/rebuild surface (`crates/magpie-episodic/src/lib.rs:39-183`). |

This scan found no unclassified public crate root, free function, public
field-bearing type, or serialized structure outside the classification table.
It did not treat private construction, accessor methods, or a `pub` item in a
private module as an independent authority; those remain classified through
the root exposure and their owning resolver/projection family. The inventory
does not close A-001, A-009, A-015, A-017, A-019, A-021, or the deferred A-010
API/coordinate questions, or the independent authority-binding question in
A-024.

## Test inventory and enforcement matrix

| Test/fixture family | What it actually proves | Boundary it does not prove |
| --- | --- | --- |
| `crates/magpie-log/tests/golden.rs`, `testdata/golden-v1.jsonl` | Canonical profile, fixed tags, exact preimages/hashes/signatures, payload validation, and historical tag stability for the committed vectors | Complete storage grammar, cross-language acceptance, weak-root/strict-signature behavior, resource bounds, or arbitrary malformed JSON behavior (A-004/A-021) |
| `crates/magpie-log/tests/chain.rs` | Genesis binding, sequence/prev/hash/signature failures, no projection before complete verification, one-read replay behavior, and append-time closed typed validation | Reused-projection preconditions, concurrent/crash durability, first-failure parity with Python, or public API compile isolation |
| `crates/magpie-log/tests/deadbolt_anchor_fixture.rs`, `crates/magpie-claims/tests/deadbolt_anchor_fixture.rs` | Portable tag-5 anchor vectors, exact roots/profiles, and replayed occurrence shape | Foreign Deadbolt verifier, key initialization, sealed-but-unanchored reconciliation, or malformed non-root anchor fields |
| Provenance/origin families: `artifact_provenance_verifier.rs`, `origin_binding_verifier.rs`, `origin_admission_replay_substrate.rs`, `origin_admission_audit.rs`, `resolution_content_closure.rs` | Strict bounded parsing, private replay contexts, exact prefix/closure identity, first-failure audit outputs, and standing-inert provenance conclusions | Trust-root ownership, independent authority binding for claimant-controlled origin groups, external source availability, generic admission, identity/delegation, or network/CAS behavior (A-024) |
| Contribution/policy families: `deterministic_support_standing_v2.rs`, `standing_resolution.rs`, `standing_v3.rs`, `standing_v4.rs`, `support_contribution_audit.rs`, `admitted_contribution_audit.rs` | Closed policy lanes, exact scope/target matching, origin-group limits, v3 corroboration, v4 exact direct refutation, blocker/trace vectors, and canonical derived outputs | General contradiction debt, invalidation/supersession/withdrawal effects, currentness, cross-facet composition, or independent authority binding behind external corroboration (A-024) |
| Claim-inline families: `claim_inline_sha256_predicate.rs`, `inline_predicate_attestation.rs`, `claim_inline_predicate_edge_lane.rs` | Four-cell digest/relation matrix, edge/lane precedence, bounded subject decoding, neutral/ineligible outcomes, and standing-inert behavior | Generic evidence authority, new edge semantics, or policy use outside the ratified v4 lane |
| Projection families: `regenerable.rs`, `standing_replay_snapshot.rs`, `episodic.rs` | Deterministic rebuild/drop-and-replay, snapshot co-replay, episodic search/rows/status fields, and partial replay failure behavior | Replaying into an already populated projection, detached row/prefix/query provenance binding, query-engine semantic drift, timestamp overflow typed failure, external DB path ownership, or cache freshness (A-019) |
| Compile-fail doctests in public context modules | Some private-field, private-constructor, and deserialization boundaries | Every example fails for the intended forbidden authority surface; v0-v2 direct resolution literals and fabricated `Current`/`Settled`/`Matched` application values are not prohibited (A-015), and several existing examples are vulnerable to stale signature/arity failures rather than proving the intended boundary (A-018) |
| CI/release checks | Format, test, clippy, tour, package/release metadata, and Python golden verification on the configured toolchain | CI does not run the release-contract portable Deadbolt Python fixture; fuzzing, cross-platform behavior, cargo audit/deny, MSRV, concurrent/crash tests, differential malformed vectors, or live MCP/Deadbolt integration remain uncovered (A-011) |

| Constitutional invariant | Current enforcement | Completion assessment |
| --- | --- | --- |
| L0 is sole historical source of truth | Replay and regenerable projection tests; no projection writes L0 | Strong doctrine/runtime alignment, but raw storage mutation remains a second append seam (A-001). |
| `LogWriter` is sole append authority | `LogWriter` is the intended writer and public docs say the gate holds it | Not structurally enforced because `LogStore::append_record` is public (A-001). |
| Corrections are new events, never edits/deletion | No update/delete API; append-only comments/tests | Narrow API evidence; storage corruption/partial-write behavior remains A-008. |
| Closed vocabulary grants no authority | Closed validators and policy tests | Mostly enforced; v2/v3 wildcard promotion creates latent authority from future vocabulary (A-005). |
| Verify before projection | `chain.rs` hostile late-failure tests and one-read replay | Enforced for `replay`; public `events`/`Projection::apply` seam remains provisional (A-009). |
| Attribution is not identity/trust | Explicit docs and no identity subsystem | Prose-only until a future identity/governance contract; no current runtime claim is made. |
| Facets do not become evidence or feed other facets | Private v3/v4 composition and standing-inert lane tests | Narrow paths enforced; no generic type-level prohibition across all future resolvers. |
| Exact target/scope binding | Claim-inline and policy vectors | Enforced in landed lanes; invalidation/supersession/currentness rules remain deferred. |
| Every authoritative output has complete coordinates | Prefix/closure vectors for newer audits and v3/v4; private contexts | Not achieved for detached v0-v2 outputs and future currentness (A-003/A-010). |
| Compatibility currentness is not canonical standing | v0 replay construction returns `Unknown`; later policies test inherited-value quarantine | Not enforced against public v0-v2 struct literals that can serialize fabricated `Current` values (A-015), and legacy raw status fields remain easy to consume as governed standing (A-017). |
| Admission remains separate from endorsement/truth/standing | No generic admission runtime; readiness documents | Absence plus prose gate; raw `LogWriter` remains a low-level capability, not a governed admission result. |
| Capability exposure is not permission | No MCP/CLI runtime; proposed boundary docs | Future-only, not executable-enforced. |
| Human owner retains merge/ratification authority | Branch/PR workflow and AGENTS instructions | Process-only; no machine-enforced ratification registry (A-002). |

The test architecture is strong for the deliberately landed slices but cannot
support a full-system PASS while the rows marked “not achieved” or
“prose-only” remain open.

## Continuation recheck — 2026-08-01

The remote and local state was rechecked after the initial provisional
handoff. The audit point remains `main` at
`3b46fdb81857d77b827271777b86745bca5f7b12`; the local worktree still has no
tracked code changes and only this untracked audit artifact.

- PR #103 remains open, non-draft, and mergeable, with base `main` at the
  audited SHA and head `b6cc4dc5ff75672b3b9e5a879ee4f2e749faa5b7`.
- A repository-scoped open-PR search returned only PR #103; no second active
  implementation or governance PR changes the review-debt inventory.
- The current unresolved review thread is still at
  `docs/adr/0006-claim-currency-and-currentness-semantics.md:425-429`. Its
  concrete references remain `ADR-0003:73-74` and `ADR-0005:123,135-136`,
  which still say currentness/currency is deferred on `main`.
- All earlier inline threads are resolved or outdated. Review submissions are
  `COMMENTED`; no formal approving review or owner ratification is inferred.
- GitHub Actions CI run #479 for the current head is completed successfully;
  the connector's current combined status also reports CodeRabbit success.
- The PR body says the fifth remediation pass resolved all prior fresh threads,
  but that statement predates the current unresolved thread created by the
  latest review. This is review-state debt, not evidence that the doctrine
  conflict is closed.
- Fresh local validation on the unchanged base passed: workspace tests
  `281 passed, 0 failed`, three compile-fail doc tests passed,
  `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked
  -- -D warnings`, and the Python golden verifier for all nine records.
- A focused source check found no CLI/MCP/currentness runtime binary, confirmed
  the public `LogStore::append_record` seam, and confirmed that non-test v0
  replay construction uses `Unknown` while public v0–v2 resolution fields can
  still be filled manually with `Current` (A-015).
- The fresh Deadbolt helper recheck narrowed A-009 further: the public exact
  matcher also accepts caller-selected `EvidenceKind`/`ClaimDomain` values and
  relies on documentation for their derivation from target metadata. The
  private v1-v4 paths remain same-replay and unaffected; the public helper is
  still a contained mechanics/eligibility seam, not a demonstrated standing
  bypass.

Disposition is unchanged: A-002 and A-010 remain confirmed open, PR #103 is
not approval evidence, and the audit remains provisional. No GitHub mutation,
thread resolution, merge, or PR metadata change was performed.

## Fresh-pass closure check — 2026-08-01

The six independent read-only passes were adjudicated against the complete
ledger after the continuation recheck. Five additional findings are retained:

- A-016 is a conditional future-governance gap: ADR-0005 requires actor-local
  withdrawal without supplying an enforceable identity/delegation coordinate.
  It does not authorize an identity subsystem or withdrawal runtime.
- A-017 is a contained compatibility/API ambiguity: public legacy `Claim` and
  `StandingView` status fields remain raw projection data, but their names and
  serializable shapes can be consumed as governed standing.
- A-018 is a contained test-enforcement gap: several compile-fail doctests fail
  because examples use stale arity or removed method shapes, so they do not
  establish the intended forbidden authority boundary.
- A-019 is a contained query/provenance gap: `EpisodicEvent` rows and bare
  search results can be detached from the verified prefix, exact event identity,
  and query semantics even though the underlying projection was rebuilt from
  verified history.
- A-020 is conditional workspace workflow debt: ignored pending delegation
  tickets retain obsolete preconditions, and the wrapper lacks ticket
  freshness/base/status preflight before write-mode invocation. It is not a
  runtime authority finding and does not make the ignored queue canonical.

The same passes expanded A-002 into a stale landed-slice status family and
expanded A-004 with Python's acceptance of non-standard JSON constants such as
`NaN`. The continuation corpus sweep also folded the unclassified bootstrap and
visual-ledger claims into A-002 rather than splitting a duplicate governance
finding. The targeted standing/Deadbolt sweep additionally folded the
unlabelled landed v1 policy/lane/tour notes into A-002; their runtime behavior
matched the documented narrow paths, so no separate semantic finding was
created. The compatibility pass's detached episodic-row concern is separate
from those findings because it remains a provenance omission even after a
correct verified replay. No pass established a new Critical finding or
overturned an existing finding. The wall-clock timestamp concern remains
rejected because timestamps are recorded historical inputs, not ambient replay
inputs.

The final targeted origin-authority check also rejected a separate finding:
`human-review` plus `authority:origin-review-v0` is selected by the fixed
origin-admission policy and remains confined to the declared origin-group
assignment; it is not treated as an identity, truth, standing, or writer
credential. The rejection is conditional on retaining that narrow scope. A
future generic admission consumer would need a separately ratified authority
or identity contract rather than inheriting this pair by label.

The report now records 21 findings: four Constitutional findings (A-001,
A-002, A-005, A-010), with A-016 conditional future governance, A-021
conditional cryptographic integrity, and the
remaining findings High, Medium, Medium conditional, or Low as tabulated. The
audit is still provisional: no unresolved Constitutional issue has been
closed, no independent formal approval was inferred, and the recommended next
slice remains documentation/status-governance closure before new runtime
authority.

## Targeted storage/canonical closure check — 2026-08-01

A focused read of the storage, typed-event, Rust verifier, Python verifier,
bounded-context, and format sources sharpened A-008 without adding a separate
canonical-integrity finding.

- `FileStore::append_record` opens and appends without a single-writer,
  synchronization, or atomic-frame contract; `read_records` reads the entire
  file and clones every non-empty line. Typed payload metadata remains opaque,
  and the current resource-sensitive contexts materialize or traverse input
  before all relevant limits are known. These are now recorded in A-008 with
  the requirement for bounded streaming, recursion/metadata limits, and
  decoded-byte checks before materialization.
- Rust Serde and Python's JSON-object access both ignore unknown object members
  in the current storage readers. That does not make an extra storage member
  part of the canonical hash or signature: `FORMAT.md` explicitly declares
  JSONL storage non-normative and both verifiers recompute the typed canonical
  core. The candidate that this alone grants canonical status is therefore
  rejected. If raw JSONL exchange becomes a governed interoperability surface,
  its unknown-member/duplicate-member grammar belongs in A-004's differential
  decision; the correction must not make storage bytes authoritative.

No runtime files were changed during this storage pass; the later targeted
release/query checks are recorded below. The report remains an untracked audit
artifact only.

## Fresh dependency/query/release pass — 2026-08-01

A separate source and workflow pass covered dependency resolution, build
surface, query semantics, and the release fixture gate.

- `cargo metadata --locked --no-deps` confirms the three-package workspace, but
  workspace requirements remain broad semver ranges (`serde`/`serde_json`/`thiserror`
  `^1`, `ed25519-dalek` `^2`, and `rusqlite` `^0.37`). `cargo tree --workspace
  --locked -e features --depth 2` confirms the only bundled database feature is
  `rusqlite`'s bundled SQLite. No Rust `unsafe` blocks, build scripts, explicit
  toolchain/MSRV files, `cargo-deny`, or `cargo-audit` configuration were found.
  This confirms and does not broaden A-011; the lockfile and green tests are
  not a semantic dependency-review policy.
- The release-contract portable Deadbolt chain command passed locally with two
  records. `gh pr checks 103` and `gh run list` show PR #103's `verify` checks
  and CodeRabbit status passing, while `.github/workflows/ci.yml` still omits
  that release-contract fixture command. The omission is folded into A-011;
  it is a missing independent gate, not evidence that foreign Deadbolt bundle
  verification belongs in Magpie.
- `EpisodicView::search` returns only `Vec<u64>` sequence numbers after a
  hardcoded FTS5 phrase query. The proposed librarian contract leaves query
  language and API design future while requiring reproducible derivations.
  Because no snapshot, projection/query identity, or tokenizer/engine semantic
  coordinate is returned, this query determinism/compatibility concern is
  folded into A-019. The smallest future correction is a coordinate-bearing
  query response contract; SQLite remains derived and non-canonical.
- The locked dependency's upstream `ed25519-dalek` repudiation test was not
  runnable offline because its registry resolution lacked `bincode`; no Rust
  weak-root fixture was added or claimed as passed. The source-level ordinary
  versus strict verification distinction and the inline Python adversarial
  vector are the current evidence for A-021, which still requires a Rust-side
  cross-tool fixture before implementation remediation.

One additional conditional High finding, A-021, was retained for weak-root
acceptance; the ledger now contains 21 findings, with four Constitutional
findings and no confirmed Critical finding. No runtime, PR, or Git history
mutation was performed.

## Live PR state refresh — 2026-08-01

A fresh connector-first read-only refresh found no external-state change:

- PR #103 remains open, non-draft, and mergeable, with base
  `3b46fdb81857d77b827271777b86745bca5f7b12` and head
  `b6cc4dc5ff75672b3b9e5a879ee4f2e749faa5b7`.
- The non-outdated unresolved thread remains at
  `docs/adr/0006-claim-currency-and-currentness-semantics.md:425-429`; it
  points to the still-deferred currentness statements in ADR-0003 and
  ADR-0005. This is direct evidence that the PR's documentation closure is
  not yet complete, not a reason to infer approval or resolve the thread.
- All submitted reviews remain `COMMENTED`; no formal approving review or
  owner ratification is inferred.
- The connector reports CodeRabbit success, and PR-triggered CI run #479 for
  the current head is completed successfully. Passing checks do not close the
  unresolved doctrine thread or the Constitutional findings on `main`.

No GitHub mutation, thread resolution, merge, PR metadata change, or runtime
file change was performed during this refresh.

## Constitutional re-attack — 2026-08-01

A fresh source-level attack against the four Constitutional findings produced
no closure:

- **A-001 remains executable authority divergence.** `LogStore::append_record`
  is still a public mutating trait method, while `LogWriter::append` is the
  only path that performs the intended payload validation, signing, and chain
  state advance. A caller can therefore mutate a backend with raw bytes
  without possessing a `LogWriter`; later verification may reject those bytes,
  but rejection does not restore the declared sole-append boundary.
  The candidate that this is harmless because raw bytes are not accepted
  history was rejected: it addresses historical acceptance, not unauthorized
  mutation/corruption of the storage substrate.
- **A-002 remains a governance divergence.** The current corpus still mixes
  ADRs marked Proposed, ratified narrow policy contracts, historical future
  wording, and bootstrap/visual claims without one owner-approved precedence
  registry. The live PR's passing checks and documentation intent cannot
  resolve that state while the PR is unmerged and its final status thread is
  open.
- **A-005 remains a future-vocabulary authority leak.** The v2 arm
  `Some(_) if deterministic_supported` and the v3 arm `_ if corroborated`
  would give a future `Status` variant a positive-standing consequence without
  an explicit policy arm. The exhaustive v4 match demonstrates the smallest
  safe pattern; it does not repair v2/v3.
- **A-010 remains a doctrine/substrate gap.** On `main`, ADR-0003 through
  ADR-0005 still defer currentness semantics, the runtime exposes only the
  compatibility `StandingCurrentness` field, and no currentness resolver or
  per-material coordinate-bearing output exists. PR #103 narrows a proposed
  future doctrine but explicitly defers representation, eligibility,
  identity, algorithms, encoding, and runtime; it therefore cannot be treated
  as currentness implementation or closure evidence.

The four findings remain confirmed open; no Critical finding was promoted and
no new Constitutional finding was needed.

## Final validation refresh — 2026-08-01

After the audit-only edits, the unchanged runtime tree was revalidated:

```text
cargo test --workspace --locked --no-fail-fast  # 281 passed; 0 failed
cargo fmt --all --check                         # passed
cargo clippy --workspace --all-targets --locked -- -D warnings  # passed
python tools/verify_chain.py crates/magpie-log/testdata/golden-v1.jsonl ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c  # 9 records passed
python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737  # 2 records passed
```

The report integrity scan now finds 21 ledger IDs, 21 detail headings, 21
finding-standard coverage rows, no missing or orphaned details, no duplicate
IDs, no missing required deliverable sections, and no trailing whitespace.
The worktree still contains only this untracked audit report; no tracked code,
test, Cargo, or GitHub state changed.

## Canonical interoperability continuation — 2026-08-01

The cold canonical pass added independent evidence without changing the
finding set:

- A from-spec encoder written independently of `tools/verify_chain.py`
  reproduced all nine committed golden content hashes, including tags 0
  through 8 and the UTF-8-bearing fields. This supports the narrow binary
  profile and does not prove the JSONL language is frozen.
- A duplicate-known-member case was exercised in memory against the current
  Python verifier: the event contained two `status` members, Python retained
  the last value and returned `OK`, while the Rust Serde-derived boundary's
  duplicate-field behavior rejects the same storage record before replay.
- The divergence is retained under A-004. It must not be “fixed” by making
  JSONL the canonical preimage, by changing existing binary tags, or by
  silently choosing one implementation's failure order without a format/tool
  contract.

A-021 remains conditional on accepting a weak external Ed25519 root; the
normal-root golden and Deadbolt vectors continue to pass. No new Critical or
Constitutional finding was established in this pass.

## Completion-criteria audit — 2026-08-01

The user-mandated completion criteria were checked explicitly against the
current corpus, runtime, tests, and live review state. The audit is not at
provisional closure:

| Criterion | Status | Evidence and limiting condition |
| --------- | ------ | ------------------------------- |
| 1. Every canonical document is mapped to its dependencies | PARTIAL | The corpus dependency/status map exists, but A-002 shows that precedence and living-status authority are not closed across the mapped documents. |
| 2. Every major runtime surface is classified | PARTIAL | Current crates, public APIs, CLI/MCP seams, Deadbolt, and future surfaces are inventoried; A-009, A-015, A-017, A-019, A-023, and A-024 show classifications that still expose compatibility, provenance, or authority-binding risk. |
| 3. Every derived consequence has a declared owner | PARTIAL | Current ownership records are explicit, but currentness, generic admission, provenance response, future query semantics, and independent authority binding behind corroboration remain deferred owner/representation boundaries under A-002, A-010, A-022, and A-024. |
| 4. Every authoritative output has complete reproducibility coordinates | PARTIAL | v3/v4 standing and top-level origin audits are bounded, but detached v0-v2 standing, direct matched provenance traces, claimant-controlled corroboration authority, currentness, and query outputs remain incomplete under A-003, A-010, A-019, A-023, and A-024. |
| 5. Every public compatibility field is classified | PARTIAL | The compatibility inventory classifies the known legacy fields, but A-015 and A-017 remain dangerous authority-looking public surfaces until quarantined or replaced. |
| 6. No known path grants authority from vocabulary, attribution, capability, or mere presence | PARTIAL | The governing doctrine rejects those sources, but public raw append, unverified projection/application, wildcard standing matches, manually constructible authority-looking values, and claimant-controlled authority labels feeding corroboration remain under A-001, A-005, A-009, A-015, and A-024. |
| 7. Exact target and scope boundaries are enforced or explicitly deferred | PARTIAL | Evidence/origin lanes enforce several exact boundaries and ADR-0006 narrows future currency doctrine, while lifecycle eligibility, identity, and some Deadbolt/resource boundaries remain deferred under A-010, A-014, and A-016. |
| 8. Replay has no unexplained ambient inputs | PARTIAL | Verified replay is substantially deterministic, but cross-tool grammar, storage/resource behavior, detached trace coordinates, query coordinates, and weak-root strictness retain unexplained or unpinned variation under A-004, A-008, A-019, A-021, and A-023. |
| 9. Canonical encoding and verification boundaries are tested or gated | PARTIAL | Golden vectors and workspace validation pass, but Rust/Python acceptance order and strictness are not frozen together, and the weak-root boundary lacks a Rust cross-tool fixture under A-004 and A-021. |
| 10. Admission remains unimplemented until readiness questions are governed | PASS as containment | No generic admission or EpistemicGate was implemented; the readiness record keeps write authority deferred, including the unresolved rejection/representation/retry gate A-022. This is a safety condition, not evidence that admission is ready. |
| 11. Two independent hostile passes find no unresolved Critical or Constitutional issue | FAIL | The fresh constitutional re-attack confirms six unresolved Constitutional findings: A-001, A-002, A-005, A-010, future-readiness blocker A-022, and A-024. No Critical finding was confirmed. |
| 12. Independent implementer simulations no longer diverge on settled questions | FAIL | A-010 records materially divergent compliant-looking currentness implementations because representation, eligibility, identity, algorithm, encoding, and runtime decisions remain deferred. |
| 13. All remaining ambiguity is explicitly classified | PARTIAL | The ambiguity ledger classifies most residuals as policy-relative, implementation-local, compatibility debt, or deferred governance; A-002 precedence, A-010 currentness, A-022 admission readiness, A-023 detached trace carriage, and A-024 authority binding still require owner-governed or contract-level closure. |

Accordingly, the safe next slice is limited to the named documentation/status
governance closure and any separately authorized compatibility or verifier
contract work. No new resolver, admission path, currentness runtime, identity
system, canonical-format change, or generic write surface is authorized by
this audit.

## Persistent continuation refresh — 2026-08-01

A further current-state pass found no closure transition or new independent
finding:

- `main` remains at `3b46fdb81857d77b827271777b86745bca5f7b12`, identical to
  `origin/main`; the only worktree artifact is this untracked report, with no
  tracked code, test, Cargo, or Git history mutation.
- PR #103 remains open, non-draft, and mergeable at head
  `b6cc4dc5ff75672b3b9e5a879ee4f2e749faa5b7`. Its changed-file set remains
  exactly `README.md`, `docs/adr/0004-claim-lifecycle-facets.md`, and
  `docs/adr/0006-claim-currency-and-currentness-semantics.md`.
- The non-outdated unresolved thread remains the currentness-deferral
  contradiction at ADR-0006 lines 425-429. Submitted reviews remain
  `COMMENTED`; the current head's PR-triggered CI run #479 and CodeRabbit
  status are successful, but neither is formal owner approval.
- The cold public-declaration scan reconfirmed that the three published
  crates' root-exposed public types, functions, methods, and field-bearing
  structures are represented in the API classification table. It did not
  weaken A-001, A-009, A-015, A-017, A-019, A-021, or the deferred A-010
  coordinate questions.

This refresh changes no finding status. The four Constitutional findings and
the two failed completion criteria remain open, so the audit must not be
represented as complete or as authorization for a new runtime slice.

## Continued validation refresh — 2026-08-01

The runtime tree is unchanged from the audited `HEAD`, and a fresh validation
ladder passed:

```text
cargo test --workspace --locked --no-fail-fast  # 281 passed; 0 failed, including 3 compile-fail doctests
cargo fmt --all --check                         # passed
cargo clippy --workspace --all-targets --locked -- -D warnings  # passed
python tools/verify_chain.py crates/magpie-log/testdata/golden-v1.jsonl ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c  # 9 records OK
python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737  # 2 records OK
```

The findings ledger remains 21 unique IDs with 21 detail records, all 13
completion rows are present, required report sections are present, and the
report has no trailing whitespace. These passing checks strengthen the
implementation evidence but do not resolve the four Constitutional findings,
the live PR doctrine thread, or the failed currentness implementer-divergence
criterion.

## Corpus inventory continuation — 2026-08-01

A complete read-only filename and status-marker pass covered 5 ADR files, 41
`docs/design` files, 2 release files, the format/seam/portable-base/supporting
documents, and all 65 historical ticket files. Every ADR, design, and release
filename appears in the canonical corpus map above. Tickets are intentionally
grouped as historical implementation records rather than treated as a second
constitutional authority layer.

The pass added one concrete A-002 status-family example: Tickets 0061 and
0063 describe their implementations as uncommitted candidate patches even
though their own records cite merged PRs #76 and #77 and the corresponding
runtime is present on `main`. This is retained as status/precedence debt, not
as a new finding or a reason to rewrite the frozen ticket history. A fresh
implementer must be able to tell that those lines describe historical ticket
execution state, not the current checkout or permission to repeat the slice.

The inventory did not find an omitted ADR, design contract, or release artifact
that would alter the architecture map. No finding status changed, and no
runtime or Git state was modified.

## Fresh constitutional closure pass — 2026-08-01

The current checkout and live PR were rechecked before this continuation. `main`
and `origin/main` remain at `3b46fdb81857d77b827271777b86745bca5f7b12`. PR #103
remains open, non-draft, and mergeable at head
`b6cc4dc5ff75672b3b9e5a879ee4f2e749faa5b7`; its only non-outdated unresolved
thread remains `docs/adr/0006-claim-currency-and-currentness-semantics.md:425-429`.

The direct source attack reconfirmed the four open Constitutional findings:

- A-001: `LogStore::append_record` is public at
  `crates/magpie-log/src/store.rs:12-13`, with public `FileStore` and
  `MemStore` implementations at lines 29-35 and 78-83. `LogWriter` delegates
  to that caller-reachable store seam at `logimpl.rs:226-307`; the source still
  exposes a raw append capability in addition to the documented writer path.
- A-005: v2 retains `Some(_) if deterministic_supported` at
  `crates/magpie-claims/src/standing_v2.rs:300-305`, and v3 retains
  `_ if corroborated` at `standing_v3.rs:885-891`. The v4 policy instead uses
  explicit exhaustive status arms at `standing_v4.rs:1167-1194`, confirming
  that the wildcard behaviour is a real compatibility hazard rather than a
  source-inventory misunderstanding.
- A-002: the live PR adds README/ADR-0004 wording that calls ADR-0006's
  boundaries settled while the new ADR remains `**Status:** Proposed`. The
  PR's own patch therefore supplies a current, concrete status/precedence
  conflict; it does not ratify the ADR.
- A-010: the PR defines a future per-material, coordinate-bound facet but
  explicitly defers representation, eligibility, actor authority/identity,
  resolver algorithm, encoding, and runtime. The current standing substrate
  still exposes only the compatibility `StandingCurrentness::Unknown` path;
  no currentness resolver exists on `main`.

The corpus status scan covered 48 ADR/design/release files. Forty-four have a
front status marker (including numbered `Status` sections); four do not:
`deadbolt-occurrence-context-v0.md`, `deadbolt-occurrence-standing-v1.md`,
`governed-standing-tour-v1.md`, and `standing-contribution-lanes-v1.md`.
The context note has a later follow-up status, but the other three remain
status/precedence debt already contained by A-002. This pass adds no finding
ID and changes no finding status. No runtime, test, Git, or GitHub state was
modified.

## Roadmap, release, and workflow hostile continuation — 2026-08-01

This continuation pass compared the current roadmap and release boundary with
the executable workflow and live pull-request state. It found no new runtime
finding, but it strengthens the existing governance and review-debt records.

The release documents deliberately keep the `v0.1.0` tag and GitHub release
absent: the contract says the version and changelog prepare a candidate rather
than create a release (`docs/releases/v0.1.0-contract.md:8-28`), and the
metadata record repeats that the tag and release are absent
(`docs/releases/v0.1.0-release-metadata.md:3-9,114-121`). The primary README
nevertheless says that the “latest formal owner-created source release remains
v0.1.0” (`README.md:17-20`). That wording can make a source candidate sound
like an existing tagged release. It is retained as another A-002 status/
precedence example, not a new finding, because the release contract itself
contains the controlling non-publication boundary.

The repository roadmap is otherwise correctly contained: `README.md:300-313`
keeps ordinary writers, `EpistemicGate`, ingestion, contradiction/currentness
runtime, librarian infrastructure, and production key custody future. There
are no CLI, MCP, librarian, shell, network, or generic writer binaries in the
current workspace. The bootstrap state file still describes a sole “gate”
writer, bi-temporal validity, evidence-driven status movement, and a canonical
codec hardening step as build guidance (`magpie-bootstrap.md:23-37,47-64,82-96,
111-120`); `CLAUDE.md` likewise retains an older queued-work order and landed
history (`CLAUDE.md:87-120,142-163`). These documents have no explicit
governing status and remain contained by A-002. They must not authorize a
currentness, admission, identity, or generic execution implementation.

The release workflow is intentionally narrower than publication: CI runs
formatting, Clippy, tests, the governed tour, metadata/inventory checks,
standalone `magpie-log` packaging, and the golden verifier
(`.github/workflows/ci.yml:33-62`). The contract's portable Deadbolt fixture
command remains a human/release-checklist item rather than a separate CI step
(`docs/releases/v0.1.0-contract.md:324-329,447-450`), while dependency and
toolchain selection remains broad or floating as recorded in A-011. The
release metadata script validates the selected workspace/package inventory but
does not itself establish a clean exact commit or publication authority
(`tools/check_release_metadata.py:13-15,56-68,117-145`); the contract correctly
leaves those decisions to the later human checklist.

Live PR state remains: PR #103 is open, non-draft, mergeable, and based on
`main` at `3b46fdb81857d77b827271777b86745bca5f7b12`, with head
`b6cc4dc5ff75672b3b9e5a879ee4f2e749faa5b7`; both current CI checks and
CodeRabbit are successful, but `reviewDecision` is empty. GraphQL review-thread
state still has one current unresolved thread at
`docs/adr/0006-claim-currency-and-currentness-semantics.md:429`, requesting
retirement of the remaining ADR-0003/ADR-0005 currentness deferrals. The
submitted reviews remain `COMMENTED`; green checks and automated comments do
not constitute owner ratification. This leaves A-002/A-010 and the existing
review-debt assessment unchanged.

No finding ID, severity, roadmap judgment, or completion criterion changed.
No runtime, test, Git, or GitHub state was modified.

## Policy and compatibility hostile continuation — 2026-08-01

This continuation pass attacked the policy matrix, evidence eligibility, and
public compatibility fields as an independent implementer would: by changing
one alleged authority input at a time and checking whether a derived standing
result changes without a declared coordinate change.

The policy substrate is closed and exhaustive. `EvidenceKind` has eight exact
variants and `ClaimDomain` has seven exact variants with strict string parsing
at `crates/magpie-claims/src/policy.rs:18-137`. The support ceiling,
privileged-context classification, and refutation ceiling each enumerate the
full 56-cell product (`:180-452`, `:455-583`), and the test module freezes
coverage and doctrine-sensitive cells at `:1110-1545`. No unknown vocabulary
can enter a policy cell through `TryFrom`, and a missing support ceiling cannot
be revived by a context classification.

The v0 structural evaluator applies the relevant exact bindings before
consulting a ceiling: target claim, typed target, `supports` edge kind, strict
claim-domain metadata, source evidence, and equality of evidence/edge/target
scope (`crates/magpie-claims/src/standing.rs:430-489`). Duplicate or trailing
claim-domain JSON is a typed blocker. v1 then selects only the exact Deadbolt
occurrence lane and rechecks target, source, edge, scope, evidence kind,
compiled ceiling/context, and the structured predicate-to-anchor identity
(`crates/magpie-claims/src/standing_v1.rs:196-235` and
`crates/magpie-claims/src/deadbolt_context.rs:160-244`). v2 adds the exact
predicate, statement/content-hash, witness scope, and witness-digest checks
before a deterministic receipt (`crates/magpie-claims/src/deterministic_verifier_context.rs:160-270`).

The v3 path does not consume a caller-selected audit, edge list, origin group,
policy ID, or actor label. `resolve_admitted_contribution_audit_v0` derives
the candidate universe from the replay, limits it to
`ExternalSource × ExternalReport`, requires a non-empty artifact content hash,
and derives the exact graph contribution and namespace before consulting the
internally derived origin decision (`crates/magpie-claims/src/admitted_contribution_audit.rs:463-581`).
The v3 composer then consumes only the internally derived support audit and
requires its fixed policy, prefix, closure, target, scope, identity, and
completion fields before the two-origin-group rule can yield `Supported`.
The v4 path remains a separate exact claim-inline relation/digest lane; it does
not turn a bare `contradicts` edge into generic contradiction debt or refuted
standing.

Compatibility attacks produced the following results:

- changing only `actor_class`, rationale, or inert metadata does not enter the
  current standing lanes. The fields remain signed/replayed data, not identity
  or authority; the claim-inline contract explicitly makes these fields
  non-authority inputs, and the evidence-ceiling contract says a bare actor
  label is not an `EpistemicGate` decision;
- changing a `supports` relation to `contradicts`, `invalidates`, or
  `supersedes` is rejected by the current v0-v3 support lane as an unsupported
  edge, without inventing a refutation, invalidation, or currentness effect;
  the exact v4 lane is the only separately consumed negative relation;
- changing unknown evidence/domain vocabulary, duplicate metadata, or scope
  yields a blocker before ceiling consultation;
- repeating an origin contribution from one group does not create a second
  v3 group, while exact contribution identity duplication is rejected;
- serialized or cloned audit values cannot be supplied to the live v3/v4
  resolvers, and the targeted claims suite passed 281 tests including the
  policy matrix, provenance, composition, hostile-boundary, and compile-fail
  cases.

The public mutable legacy maps, manually constructible v0-v2 resolution
values, raw status fields, unverified `SignedEvent`/`Projection` paths, and
detached episodic rows remain the already-recorded A-009, A-015, A-017, and
A-019 compatibility/provenance findings. Their existence does not make the
current private v1-v4 composition paths caller-substitutable. No new distinct
policy or compatibility finding is warranted, and no finding status changed.
The PR #103 status conflict and its unresolved currentness thread remain
covered by A-002/A-010. No runtime, test, Git, or GitHub state was modified
apart from this audit report.

## Interface, Deadbolt, and release-boundary hostile pass — 2026-08-01

The second continuation pass inspected the current public interfaces and the
future boundary documents independently of the earlier architecture map.
There are no `main.rs`, `bin`, `build.rs`, MCP, librarian, CLI, shell, network,
or subprocess runtime surfaces under `crates` or the workspace manifests.
The MCP, librarian, query, provenance-response, capability, and AgentProposer
documents all declare themselves proposed/docs-only contracts. Their absence
is therefore not a current implementation defect, and no generic execution
authority was found.

The following current surfaces reconfirm existing findings:

- `LogReader::events` is explicitly public parsed-but-unverified input at
  `crates/magpie-log/src/logimpl.rs:345-355`, while the public
  `Projection::apply` contract at lines 417-420 accepts a caller-supplied
  `SignedEvent`. The Deadbolt helper at
  `crates/magpie-claims/src/deadbolt_context.rs:198-224` likewise accepts a
  caller-created index and caller-selected policy vocabulary, while its own
  documentation says that accepted-chain provenance requires private
  same-replay construction. This is A-009's contained unverified API seam,
  not a new bypass of the private v1-v4 standing paths.
- `EpisodicView::at_path` opens an arbitrary caller-selected path at
  `crates/magpie-episodic/src/lib.rs:100-101`; a schema-version mismatch then
  executes `DROP TABLE IF EXISTS events_fts` and `DROP TABLE IF EXISTS events`
  at lines 105-115. This is a derived-index migration side effect, not a
  second history or generic write authority, but it reconfirms the conditional
  destructive-path finding A-012 and the detached-index risks A-007/A-019.
- The release/dependency boundary remains semantically under-pinned:
  workspace dependency requirements are broad semver ranges in `Cargo.toml`
  (serde/serde_json/ed25519-dalek/sha2/hex and rusqlite), CI installs floating
  stable Rust at `.github/workflows/ci.yml:15-21`, and installs the unpinned
  `cryptography` package at line 52. `tools/check_release_metadata.py:57`
  also allows an ambient `CARGO` executable override. The lockfile constrains
  the audited checkout, but no policy requires semantic review of those
  changes. This strengthens A-011; it is not evidence of a current dependency
  compromise.
- The specialized Deadbolt boundary remains narrow: Magpie stores opaque
  anchor identity and exact occurrence data, while foreign verification,
  key lifecycle, and reconciliation remain unimplemented external gates.
  Anchor occurrence is not being treated as foreign-bundle interpretation or
  generic write permission. The absence of an executable foreign verifier
  prevents a positive end-to-end readiness claim but does not create a new
  current false-interpretation finding beyond A-001/A-014 and the Deadbolt
  readiness record.

No new finding ID or severity change is warranted. The fresh pass changes no
runtime, test, Git, or GitHub state.

## Canonical and replay hostile pass — 2026-08-01

The live canonical/replay pass compared `docs/FORMAT.md`,
`crates/magpie-log/src/canonical.rs`, `event.rs`, `hashing.rs`, and
`logimpl.rs` against the frozen vectors and hostile paths. The binary profile
still agrees on field order, tags 0–8, UTF-8 length framing, SHA-256 hashing,
and the `magpie-sig-v1 || hash` signature domain. Unknown payload tags fail at
the typed deserialization boundary; malformed known payloads fail before a
verified projection is applied. No new canonical-format divergence was found.

The known limitations remain concrete rather than theoretical: Rust checks
payload validity before genesis rules even though FORMAT's listed verification
order omits that phase (`logimpl.rs:125-202`), Rust accepts mixed-case hash and
signature hex while the Python tool requires lowercase, Rust skips blank JSONL
records while Python treats them as failures, and Python accepts numeric status
values and non-standard JSON constants that Rust rejects. These remain A-004's
cross-tool grammar/first-failure contract. Ordinary Ed25519 verification still
does not pin weak-root rejection, so A-021 remains conditional on the external
trust-root boundary.

Fresh executable evidence on the unchanged runtime tree:

```text
cargo test -p magpie-log --locked --no-fail-fast      # passed, including replay, chain, golden, Deadbolt, and 3 compile-fail doctests
cargo test -p magpie-claims --locked --no-fail-fast   # 281 passed; 0 failed
python tools/verify_chain.py crates/magpie-log/testdata/golden-v1.jsonl ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c  # 9 records OK
python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737  # 2 records OK
```

The focused tests strengthen the narrow profile and fail-closed replay
claims; they do not establish cross-tool grammar parity, weak-root strictness,
storage durability, or complete detached-output coordinates. No new finding
ID or severity change is warranted.

## Evidence and resolver-composition hostile pass — 2026-08-01

The independent composition pass attacked the boundary where externally
supplied resolution material enters evidence and standing derivation. The
availability closure is constructed only from explicit borrowed key/byte
inputs by `ResolutionContentClosureV0::construct` at
`crates/magpie-claims/src/resolution_content_closure.rs:295-421`. It applies
entry, key-field, object-size, total-byte, and conflicting-key checks before
copying bytes; its canonical manifest is ordered by `BTreeMap` keys and its
identity is the SHA-256 digest of that manifest. The constructed object has no
public insertion method and cannot be deserialized.

The closure's artifact key constructor is intentionally untrusted and does
not assert that its bytes match its key. That is not an authority leak in the
current path: the provenance resolver derives lookup keys from the parsed
binding identity at `crates/magpie-claims/src/artifact_provenance_verifier.rs:394-429`,
then recomputes the supplied artifact, parent, and derived-byte SHA-256 values
before returning a matched receipt (`:488-510` and `:577-613`). A forged key,
wrong bytes, malformed bundle, or unavailable object therefore produces an
availability or digest failure rather than a positive provenance result.
The closure remains availability input; it does not itself admit material,
interpret evidence, or grant standing.

`OriginAdmissionReplayContextV0` has private snapshot and prefix fields and is
created by `replay_origin_admission_context_v0` only after the shared verified
replay (`crates/magpie-claims/src/origin_admission_replay.rs:132-140` and
`:233-244`). Its public audit and standing methods accept only the explicit
closure, not caller-created candidate lists, audit objects, policy IDs, or
anchor indexes. The v3 path derives inherited v2 and the support-contribution
audit from that context, then binds both the verified-prefix and closure
identities into the crate-private composition input at `:183-203`.
The support resolver similarly derives the upstream audit internally and
composes it against expected policy, prefix, closure, ceiling, and context
cells at `crates/magpie-claims/src/support_contribution_audit.rs:1059-1097`.

Hostile cases were adjudicated as follows:

- replacing a closure's bytes while retaining a claimed artifact key cannot
  produce a matched receipt because the digest is recomputed;
- replacing a serialized or cloned audit cannot influence v3 because the
  resolver does not accept an audit parameter and the returned audit types are
  not deserializable or constructible through the public struct surface;
- supplying a different closure on the same replay is an explicit coordinate
  change, represented by the closure identity in the output, rather than an
  ambient cache or policy substitution;
- supplying duplicate/conflicting closure entries is rejected before
  materialization, while equivalent input ordering produces the same ordered
  manifest and identity;
- caller-created anchor indexes, candidate selector lists, and policy IDs are
  excluded from the public resolver signatures and covered by compile-fail
  boundary tests.

The pass found no new distinct evidence or resolver-composition finding. The
result depends on the existing boundary classifications: A-009 still covers
the separately exposed unverified projection/helper APIs, A-015 covers
publicly fabricable legacy resolution values, A-019 covers detached query
coordinates, and A-021 covers the conditional external trust-root strictness
question. This pass does not claim that the future admission or foreign
verification gates are ready, nor does closure availability become evidence or
standing by presence alone. No runtime, test, Git, or GitHub state was
modified.

## Residual parser, projection, and readiness continuation — 2026-08-01

This pass performed a residual source scan over production deserialization,
projection application, duplicate identity handling, and the future admission
readiness boundary.

The accepted replay path still has one snapshot boundary: `LogReader::replay`
reads records once, retains parsed events, verifies the complete snapshot, and
only then calls `Projection::apply`
(`crates/magpie-log/src/logimpl.rs:110-209,364-420`). The remaining public
`events()` and `Projection::apply` paths are explicitly parsed/unverified or
caller-applied and are already bounded by A-009; they do not substitute for
the private verified replay path. The Deadbolt context resolver likewise
states that a caller-created index and caller-selected vocabulary do not prove
accepted-chain provenance (`crates/magpie-claims/src/deadbolt_context.rs:41-56,
184-204`).

Duplicate typed claim, evidence, and edge identifiers are not an unrecorded
implementation accident. The projection uses documented first-write retention,
the historical remediation ledger calls the behavior remediated, and hostile
tests pin duplicate claim/evidence/edge behavior
(`crates/magpie-claims/src/standing.rs:661-778,1467-1556`,
`docs/design/historical-review-remediation-ledger.md:154-157`). The narrow
claim-inline and support-contribution contracts additionally specify duplicate
edge handling and reject duplicate candidate IDs where composition requires
global uniqueness. This pass therefore rejects a new “duplicate-ID replay”
finding; the behavior remains policy/compatibility evidence already covered by
the existing records.

The residual panic and whole-materialization sites remain the already recorded
A-007/A-008 availability findings: SQLite projection application is infallible
and can panic on reuse, signed event strings and the log file are unbounded by
the current general L0 contract, and metadata/closure paths have different
resource controls (`crates/magpie-episodic/src/lib.rs:104-120,213-251,390-399`;
`crates/magpie-log/src/store.rs:30-50`). No second history or hidden authority
store was found.

Finally, the admission readiness checklist still states that every item must
be answered by reviewed doctrine before implementation and that an unanswered
item is a blocker (`docs/design/admission-design-readiness-checklist.md:159-165`).
No generic admission, MCP, CLI, or EpistemicGate runtime exists in the current
workspace, so no readiness gate was silently bypassed.

No new finding ID, severity, or status changed. No runtime, test, Git, or
GitHub state was modified.

## Canonical invariant-floor coverage — 2026-08-01

The following table makes the requested invariant floor explicit at the audited
`main` boundary. “Contained” means the current absence or narrow implementation
prevents an active authority path but does not prove that a future mechanism is
ready. “Prose/deferred” means the invariant is stated and no contrary runtime
was found, but its future owner or representation is not executable yet.

| Invariant | Current assessment | Evidence / residual boundary |
| --- | --- | --- |
| External material is not admitted history | Contained | No generic importer or admission runtime exists; admission readiness remains a blocker (`docs/design/admission-design-readiness-checklist.md:159-165`). |
| Admission is not endorsement | Prose/deferred | Admission explorations separate inclusion from truth, evidence, and standing; no admission mechanism exists (`docs/design/epistemic-pipeline-separation-contract.md:63-74`). |
| History is not evidence | Enforced in narrow lanes | Typed evidence must be explicitly recorded and related; anchor occurrence and log inclusion do not themselves interpret evidence (`crates/magpie-claims/src/standing.rs:702-775`; Deadbolt readiness record above). |
| Evidence is not standing | Enforced in narrow lanes | Closed ceilings are candidate maxima and only named v0-v4 resolvers may apply their declared contribution (`docs/design/standing-view-evidence-ceilings.md:23-38`; policy tests). |
| Standing is not truth | Prose/contained | No general truth resolver exists; landed policies state bounded policy outcomes and the release contract denies a truth oracle (`docs/releases/v0.1.0-contract.md:32-36,344-355`). |
| Standing is not currentness | Open finding A-010 | The only currentness value on `main` is compatibility `Unknown`; target, order, acts, and future coordinates remain unresolved (`crates/magpie-claims/src/standing.rs:27-30`; A-010). |
| Provenance is not trust | Enforced as a declared boundary | Provenance and origin receipts carry binding/audit conclusions but explicitly do not authenticate identity or trust (`docs/design/provenance-response-contract.md:51-93`; A-016). |
| Attribution is not verified identity | Prose/deferred; A-016 | Signed `agent`/`source` labels remain asserted attribution; withdrawal self-ownership has no identity/delegation coordinate (`crates/magpie-log/src/event.rs:5-19`; `docs/adr/0005-claim-withdrawal-acts.md:67-91`). |
| Capability exposure is not permission | Contained | No MCP/CLI runtime exists; proposed capability text explicitly denies permission from discovery (`docs/design/mcp-capability-boundary-contract.md:10-18,112-125`). |
| Proposal is not authority | Contained | AgentProposer is proposal-only and future admission owns any decision (`docs/design/agent-proposer-boundary-contract.md:81-122`). |
| Recorded relationship is not an automatic consequence | Enforced for current generic edges; narrow policy exceptions explicit | Typed edges are retained inertly; only exact named policy lanes consume selected relations, while broad invalidation/supersession/currentness effects remain deferred (`crates/magpie-claims/src/standing.rs:753-775`; `docs/adr/0005-claim-withdrawal-acts.md:102-137`). |
| Projection is not history | Enforced for verified replay; detached-output risk remains | Replay verifies before applying and projections are rebuildable, but detached rows/outputs and direct matched provenance traces can lose producing coordinates (A-007/A-019/A-023). |
| Policy selection is not ambient latest | Enforced for landed policies | v0-v4 expose explicit policy identities and no latest selector; policy tests freeze the named cells (`README.md:136-153`; `crates/magpie-claims/src/policy.rs:18-137`). |
| Failure is not falsity | Enforced in narrow resolver outputs | Resolution failures and blockers are typed; no failure path is converted into a truth/falsity claim (`docs/design/standing-view-evidence-ceilings.md:233-253`; v0-v4 failure enums). |
| Withdrawal is not falsity | Prose/deferred | ADR-0005 keeps withdrawal as a historical actor-local act and defers standing/currentness effects; no withdrawal runtime exists (`docs/adr/0005-claim-withdrawal-acts.md:102-137`). |
| Invalidation is not falsity | Prose/deferred | No current edge self-executes invalidation; future eligibility and standing consequences require separate policy ownership (A-010 and the PR #103 candidate, not current `main`). |
| Supersession is not correctness | Prose/deferred | Current doctrine treats supersession as lineage and does not grant correctness or automatic standing; no currentness resolver consumes it (`docs/adr/0005-claim-withdrawal-acts.md:102-123`; A-010). |
| Repetition is not independence | Enforced in landed contribution lanes | Duplicate anchors retain occurrences without amplification, and v3 distinguishes admitted origin groups without claiming statistical independence (`docs/design/standing-aggregation-independence-groups.md:10-19,59-75`; hostile duplicate tests). |
| Availability is not endorsement | Enforced in provenance/closure composition | Closure availability and artifact bytes are rehashed and remain inputs; availability alone does not create evidence or standing (`docs/design/resolution-content-closure-v0.md:913-928`; evidence/resolver continuation above). |

This floor is a coverage aid, not a new doctrine source. The “enforced” rows
describe only the named narrow runtime contracts; they do not authorize broader
facet composition, admission, identity, currentness, or generic write work.

## Source-reference integrity continuation — 2026-08-01

The report's source-reference scan found 130 unique `path:line` references.
127 resolve against the audited `main` tree. Three references intentionally
target the active PR rather than `main`:

- `docs/adr/0006-claim-currency-and-currentness-semantics.md:128-135`;
- `docs/adr/0006-claim-currency-and-currentness-semantics.md:425-429`; and
- `docs/adr/0006-claim-currency-and-currentness-semantics.md:429`.

The file is absent from the audited base but exists at PR #103 head
`b6cc4dc5ff75672b3b9e5a879ee4f2e749faa5b7` with blob
`e4ed93c66a0f3e57d47ffd34a63a8a290cefe990`; line 429 is the ADR-0005
reference identified by the unresolved review thread. These are therefore
 explicit PR-head evidence, not broken `main` references. The scan found no
 other missing or out-of-range local reference.

## Public API inventory continuation — 2026-08-01

A fresh `src/`-only declaration scan reproduces the recorded inventory at
the audited `main` commit: `magpie-log` has 7 Rust source files, 16 direct
public types/traits/aliases, 2 public constants, and 26 public methods;
`magpie-claims` has 20 files, 101 direct public types/enums, 56 public
constants, 6 top-level public functions, and 326 public methods;
`magpie-episodic` has 1 file, 2 public structs, and 7 public methods. No
additional crate manifest, CLI binary, MCP runtime, or unclassified public
root surface appeared in the recheck. These counts are inventory evidence,
not ABI or authority approval: A-001, A-009, A-015, A-017, A-019, A-021,
A-023, and the deferred A-010 coordinate questions remain unchanged.

## Fresh runtime validation continuation — 2026-08-01

On the unchanged audited runtime at `main` commit
`3b46fdb81857d77b827271777b86745bca5f7b12`, the workspace validation was
rerun:

```text
cargo test --workspace --locked --no-fail-fast  # 281 passed; 0 failed; all compile-fail doctests passed
cargo fmt --all --check                         # passed
cargo clippy --workspace --all-targets --locked -- -D warnings  # passed
python tools/verify_chain.py crates/magpie-log/testdata/golden-v1.jsonl ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c  # 9 records OK
python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737  # 2 records OK
```

The Python commands verified the existing golden and Deadbolt fixture tips
without modifying files. These gates strengthen the landed narrow slices;
they do not establish Rust/Python malformed-input parity, storage durability,
weak-root strictness, detached-output coordinates, or readiness of generic
admission/currentness runtime.

## Live PR state continuation — 2026-08-01

A read-only refresh of PR #103 confirms: state `open`, `merged=false`,
`draft=false`, `mergeable=true`, base `main` at
`3b46fdb81857d77b827271777b86745bca5f7`, head
`traycer/adr-0006-currency-semantics` at
`b6cc4dc5ff75672b3b9e5a879ee4f2e749faa5b7`, and changed files exactly
`README.md`, `docs/adr/0004-claim-lifecycle-facets.md`, and
`docs/adr/0006-claim-currency-and-currentness-semantics.md`. The current
checks are CodeRabbit success and verify runs `30711636782` and
`30711638299`. Review submissions remain `COMMENTED`; all other inline
threads are resolved or outdated, while the non-outdated unresolved thread
at ADR-0006 line 429 still requests retirement of the remaining ADR-0003
and ADR-0005 currentness deferrals. No owner approval is inferred.

The PR body also says its documentation scope is ADR-0006 plus one README
entry, while the live changed-file list includes the substantive ADR-0004
amendment. The later remediation narrative mentions ADR-0004, but the scope
summary is still incomplete. This is review-debt under A-002: a reviewer
using only the scope bullet could under-audit a canonical ADR change. It is
not a new runtime finding; the smallest correction is an accurate PR scope
summary before owner review, without changing the doctrine by metadata.

## Fresh closure recheck — 2026-08-01

The current-state recheck found no repository drift: `main` and `origin/main`
remain at `3b46fdb81857d77b827271777b86745bca5f7b12`, and the audit report is
the only worktree change. A connector-first refresh of PR #103 confirms the
same base and head, five commits ahead with no base divergence, exactly three
changed files (`README.md`, ADR-0004, and the new ADR-0006), one unresolved
non-outdated thread at line 429, and successful CodeRabbit plus both verify
runs. No approval or ratification is inferred from those checks or from the
PR's `mergeable` state.

The fresh hostile reattack does not close a Constitutional finding:

- A-001 remains an executable append-capability divergence: `LogStore` still
  exposes `append_record` separately from `LogWriter`.
- A-005 remains a latent closed-vocabulary leak: v2/v3 still promote any
  future `Status` variant through wildcard arms.
- A-002 remains a corpus-governance divergence: ADR-0006 is still Proposed,
  while the PR body and README present its semantic boundary as an active
  map entry and ADR-0003/0005 retain currentness deferral language.
- A-010 remains an implementer-divergence blocker: ADR-0006 improves the
  coordinate doctrine, but representation, eligibility, identity, algorithm,
  encoding, and runtime choices remain deferred, so no currentness
  implementation can yet be claimed coordinate-complete.

The hidden-input scan found no new production dependency on ambient policy,
wall-clock interpretation, locale, unordered map iteration, or directory
iteration. Test-only clock injection and vector-emission environment controls
remain outside authority-bearing replay paths. The recommended documentation
slice is therefore the safest candidate, but “ready” means ready for an
owner-reviewed proposal only: it is not authorization to edit canonical
doctrine, resolve PR threads, or begin currentness/admission implementation.

## Fresh trust-root and enforcement pass — 2026-08-01

Inspection of the locked `ed25519-dalek` 2.2.0 source adds direct dependency
evidence for A-021: the ordinary verification method is intentionally distinct
from `verify_strict`, the library exposes `is_weak`, and its own repudiation
test constructs a small-order public key whose signature verifies under the
ordinary method but fails under strict verification. Magpie currently calls
ordinary verification and has no format-level decision selecting or rejecting
that accepted-key language. This remains a conditional external-root finding,
not a claim that a normal trusted root can be forged.

The storage/projection test inventory likewise confirms the existing
boundaries without closing them. `file_store_round_trips_and_recovers` tests
normal persistence and reopening; tamper and late-failure tests establish
fail-closed replay for ordinary records; episodic tests establish drop/rebuild
equality and the intended schema migration behavior. No test injects a partial
write or crash, concurrent writers, an oversized whole-log read, an oversized
opaque metadata field, a same-version foreign SQLite schema, or a timestamp
overflow through a typed non-panicking API. These remain A-007/A-008/A-012
rollout gates rather than unverified claims of safety.

No finding severity, status, or ID changed in this pass. No runtime, test,
Git, or GitHub state was modified.

## Canonical release and compatibility closure — 2026-08-01

The final canonical-format comparison found no new distinct compatibility
finding. The normative `docs/FORMAT.md`, the implementation in
`crates/magpie-log/src/canonical.rs` and `event.rs`, the frozen fixture, and
the hardcoded golden tests agree on the current additive tag rule: tags 0-8
are represented, old prefix vectors are pinned, and a changed existing
encoding requires a new profile. The regeneration test compares the generated
records against the committed fixture, while the independent Python verifier
recomputes the same current fixture under its separately written parser. The
remaining Rust/Python malformed-input and failure-order differences are A-004,
not a newly discovered vector mismatch.

The v0.1.0 release contract also matches the current Deadbolt projection
implementation at the stated level: the five-field anchor identity is
separate from ordered occurrence audit material, and the index's JSON bytes
are explicitly derived comparison bytes rather than L0 canonical encoding.
The actual index orders identities through its ordered map and occurrences by
sequence; manual projection application remains the untrusted helper path
already bounded by A-009. No release prose was found to promote anchor
occurrence into interpretation, standing, or generic write authority.

Read-only release checks passed on the unchanged tree:

```text
python tools/check_release_metadata.py       # all three package inventories checked
cargo package --list -p magpie-log --locked  # expected 19-file inventory listed
git diff --check                             # passed
```

These checks strengthen package and vector evidence but do not close the
semantic dependency, crash-durability, resource-bound, or external trust-root
gates in A-008, A-011, and A-021. The golden regeneration example is
intentionally a write-capable maintenance tool; its documentation restricts
use to a reviewed format change, and no regeneration command was run during
this audit. No finding severity, status, or ID changed. No runtime, test,
Git, or GitHub state was modified.

## Live PR reattack — 2026-08-01

A connector-first refresh of PR #103 at head `b6cc4dc5ff75672b3b9e5a879ee4f2e749faa5b7`
found no repository or PR-head drift. The pull request remains open,
non-draft, unmerged, and mergeable against the audited `main` base. Its
changed-file set remains exactly `README.md`, ADR-0004, and the new ADR-0006.
CodeRabbit and both `verify` workflow runs remain successful; those signals are
validation evidence, not doctrine approval.

The current patch makes the A-002 precedence problem more concrete rather
than resolving it. ADR-0004 now says ADR-0006 establishes the currency and
currentness boundaries, and the README describes the ADR-0006 boundaries as
settled, while ADR-0006 still declares `Status: Proposed`. The PR scope prose
still describes only ADR-0006 and one README entry even though ADR-0004 is a
substantive changed canonical document. The unresolved, non-outdated review
thread remains at ADR-0006 line 429 and asks for the remaining ADR-0003 and
ADR-0005 deferrals to be retired.

This is review/governance debt under A-002, not permission to resolve the
thread or silently ratify ADR-0006. The safe disposition remains an
owner-approved status/precedence decision, an accurate PR scope summary, and a
fresh hostile corpus review before any currentness implementation. No new
finding ID, severity, or status changed; no runtime, test, Git, or GitHub state
was modified.

## Admission readiness hostile pass — 2026-08-01

The admission corpus was cold-read as a future implementer would read it. It
correctly keeps generic admission, `EpistemicGate`, MCP writes, and ordinary
claim/evidence writes unimplemented. The readiness checklist also correctly
states that an unanswered item blocks implementation. That containment is
sound.

The remaining failure is substantive and is recorded as A-022. The corpus
leaves the historical object representation unresolved and separately leaves
rejection recording, negative-evidence prevention, retry amplification, and
operational failure handling unresolved. One future implementation could
record a typed non-epistemic rejected-attempt audit in L0; another could keep
rejection and deduplication outside L0 and retry until acceptance. A third
could preserve only a reference, only copied content, or only a digest. These
choices change replay snapshots, provenance/availability claims, retry
semantics, and query-visible history. The current documents permit no
reviewed choice between them.

This is not a reason to invent an `AdmissionRejected` tag, a proposal inbox,
an identity system, or a cache-backed staging authority. The smallest safe
next action is an owner-governed admission ADR or contract that answers the
readiness questions and pins the failure/retry/representation coordinates,
followed by hostile replay and provenance tests. Until then, A-022 keeps the
future admission and EpistemicGate roadmap rows blocked. No generic admission,
MCP, CLI, or runtime write path exists on `main`; no runtime, test, Git, or
GitHub state was modified by this pass.

## Future-interface to runtime cross-check — 2026-08-01

The current workspace contains no binary, MCP server, librarian service,
network transport, or generic proposal/write runner. The only production
write-capable Rust surface remains the low-level `LogWriter` plus its public
storage capability; the episodic surface creates or migrates only a derived
SQLite projection. The read/query-like surfaces expose parsed events,
caller-applied projections, episodic rows, and sequence-only search results,
not a future contract-compliant provenance response.

The hostile comparison therefore confirms containment rather than readiness:
MCP and librarian doctrine correctly forbids hidden writes, capability-based
permission, cache authority, and detached results, while current public APIs
remain the already-recorded A-001, A-009, A-012, and A-019 compatibility and
provenance seams. No public runtime path was found that silently implements
the proposed MCP/admission semantics, and no new distinct finding was added.

## Historical ledger state after admission pass — 2026-08-01

At that earlier admission pass, the live audit ledger contained 22 unique
findings and 22 detail records. Five were Constitutional or Constitutional
future-readiness blockers:
A-001, A-002, A-005, A-010, and A-022. No Critical finding is confirmed.
A-022 is explicitly deferred on owner-governed admission doctrine and does
not represent a current generic write path. The report's earlier continuation
sections retain their historical counts from the passes in which they were
written; the later dated section at the end of this report is the current
ledger state.

The closure judgment is unchanged in direction and stricter in evidence:
Magpie's narrow landed kernel remains coherent, while currentness and generic
admission remain blocked by incomplete doctrine, coordinates, and future-write
readiness. No implementation or ratification is authorized by this audit.

## Provenance receipt coordinate hostile pass — 2026-08-01

The direct public provenance surfaces were then attacked separately from the
same-snapshot origin/admission audit path. `StandingReplaySnapshot` exposes
artifact acquisition, direct-derivation, and origin-binding trace methods.
Their matched receipt values retain exact selectors, canonical bundle/profile
identities, recomputed artifact or contribution digests, and Deadbolt
occurrences. The receipts themselves do not retain the producing verified-log
prefix identity or the immutable resolution-closure identity.

This is not a fabrication or caller-substitution finding: the current
resolver still takes a snapshot and explicit bytes/closure, and the receipts
are documented as standing-inert audit material. It is a detached-response
finding. Extending a verified log after a matching anchor, or adding unrelated
objects to an otherwise equivalent closure, can change the declared replay or
closure coordinates without changing the serialized direct receipt. A caller
that keeps the live context can preserve the scope; a downstream consumer
that receives only receipt bytes cannot reconstruct it. This is A-023, a
Medium contained API/provenance gap.

The top-level origin-admission, admitted-contribution, support-contribution,
and standing-v3 audit outputs were checked separately and do carry both
`VerifiedLogPrefixIdentityV0` and `ResolutionContentClosureIdentityV0`.
Therefore A-023 does not weaken the same-context composition conclusions or
create a second resolver. The smallest safe correction remains additive:
provide a coordinate-bearing detached wrapper, or make the direct trace
surface explicitly internal/non-detachable. It must not add trust, identity,
source ranking, a loader, a cache authority, a new evidence/edge/tag, or a
standing feedback path.

No runtime, test, Cargo, Git, or GitHub state was modified by this pass.

## Historical ledger state after provenance pass — 2026-08-01

At that earlier provenance pass, the audit ledger contained 23 unique findings
and 23 detail records. Five remained Constitutional or Constitutional
future-readiness blockers:
A-001, A-002, A-005, A-010, and A-022. A-023 is Medium and contained; it does
not add a Critical or Constitutional finding. No Critical finding is confirmed.

All seven newly introduced A-023 source ranges resolve against the audited
`main` tree; no new PR-head-only reference was introduced. The earlier
130-reference count belongs to the prior continuation before A-023 was added
and is intentionally retained as historical validation evidence.

The overall judgment remains: the narrow landed provenance-first kernel is
coherent for its declared scope, but the repository is not at provisional
architecture closure. The safest next slice remains owner-approved
documentation/status precedence closure, followed by separately authorized
compatibility/provenance-response contracts. Generic admission, currentness,
identity, generic writes, canonical-format changes, and resolver redesign
remain blocked or premature. This report neither approves PR #103 nor
ratifies any doctrine.

## Storage snapshot contract hostile pass — 2026-08-01

The storage seam was re-read as an independent implementer would read it. The
current `FileStore` and `MemStore` return records in append order, and
`LogReader` verifies the exact vector it receives before applying any event.
That landed behavior is sound for the existing backends. The trait itself,
however, promises only `Vec<Vec<u8>>`; it does not define a complete atomic
snapshot, append ordering, or how a deliberately selected prefix or stale
view is named.

That omission permits two plausible backend implementations under the same
caller-level operation: implementation A returns one complete atomic snapshot;
implementation B returns a valid stale prefix from a swappable backend or
cache. The verifier accepts B as a valid chain because a prefix is internally
valid, so the resulting summary and replay state differ without a typed
staleness signal. This does not make every valid historical prefix invalid,
and no current backend was found to make this substitution. It is therefore a
Medium extension/contract gap folded into A-008, not a new history-integrity
finding.

The smallest future correction is to govern the storage snapshot contract
before alternate stores or caches become authority-bearing: append order,
complete/atomic visibility, explicit selected-prefix coordinates, and
resource/framing limits. A cache must not become a second history or silently
stand in for the current log. No runtime, test, Cargo, Git, or GitHub state was
modified by this pass.

## Historical ledger state after storage/dependency pass — 2026-08-01

At that earlier storage/dependency pass, the audit ledger remained at 23 unique
findings and 23 detail records.
The storage hostile pass sharpened A-008 to include the missing
append-order/complete/atomic snapshot contract and explicit selected-prefix
semantics; it did not add a new ID or raise its Medium severity. No Critical
finding was confirmed, and the Constitutional blockers at that pass were
A-001, A-002,
A-005, A-010, and A-022.

The independent corpus passes already recorded in this ledger covered
constitutional status, runtime mapping, hostile implementation, replay,
security-boundary, compatibility, provenance, admission, and release/dependency
questions. The current source scan found only ordered production collections,
no local unsafe implementation or production build/runtime command surface;
the release checker's subprocess path remains A-011. There is no registry
or network dependency source outside the locked registry graph, and no new
ambient replay input beyond the clock, storage, SQLite, and release-tooling
seams already recorded. The existing dependency/review debt remains A-011.

The closure judgment is unchanged: the narrow landed kernel is coherent for
its declared scope, but the repository has not reached provisional
constitutional closure. The next safe slice remains owner-approved
documentation/status precedence closure, followed by separately authorized
storage/API or provenance-response contracts. Currentness, generic admission,
identity/delegation, generic writes, canonical-format changes, and resolver
redesign remain blocked or premature. No implementation, ratification, merge,
release, or GitHub metadata change was made.

## Live PR #103 fifth-pass verification — 2026-08-01

The live connector refresh was repeated after the fifth remediation pass. PR
#103 is still open, non-draft, unmerged, and mergeable; it targets `main` at
`3b46fdb81857d77b827271777b86745bca5f7b12` and remains at head
`b6cc4dc5ff75672b3b9e5a879ee4f2e749faa5b7`. Its changed files are exactly
`README.md`, `docs/adr/0004-claim-lifecycle-facets.md`, and
`docs/adr/0006-claim-currency-and-currentness-semantics.md`. The two `verify`
runs and CodeRabbit status are successful; review submissions remain
`COMMENTED`, not formal approval.

The fifth pass closes the previously identified internal seams: standing and
currency consumers now own and pin the eligibility rules for the consequences
they apply; supersession may discharge contradiction debt only through a
separately governed, exact-scope standing policy; withdrawal-derived currency
effects are actor-local and exact-target; and the legacy
`StandingCurrentness` field is explicitly compatibility-only and must remain
`Unknown`. These are substantive improvements to the candidate and do not
create a new finding.

The constitutional status problem remains live. The candidate still declares
ADR-0006 `Status: Proposed`, while its README and ADR-0004 edits describe the
ADR-0006 boundaries as settled/governing. ADR-0003:73-74 and ADR-0005:123,
135-136 still describe currentness/currency as deferred, and the only
non-outdated unresolved thread at ADR-0006:425-429 requests that those
references be reconciled. The PR body also still says that only ADR-0006 and
one README map entry are in scope, omitting the substantive ADR-0004 change.
Those are one precedence/status/scope-governance problem, not three new
runtime findings; they remain A-002.

The correct disposition is therefore unchanged: the patch is a promising
documentation candidate, not ratified doctrine, implementation authorization,
or evidence that currentness runtime work is ready. The smallest correction
is an owner-approved status/precedence decision, accurate scope and historical
labels, retirement or explicit classification of the remaining deferrals, and
a fresh hostile corpus review. This audit made no GitHub mutation, did not
resolve the thread, and did not infer approval from green checks.

## Historical ledger state after live PR fifth-pass verification — 2026-08-01

At that earlier live-PR pass, the durable ledger remained at 23 unique findings
and 23 detail records. The
live PR refresh changes no finding ID, severity, or status: A-001, A-002,
A-005, A-010, and A-022 remain Constitutional blockers; A-008 and A-021
retain their existing Medium/High resource and external-trust conditions; and
A-023 remains a contained Medium provenance-response gap. No Critical finding
is confirmed.

The architecture judgment remains provisional rather than closed. The landed
kernel is coherent for its narrow append/replay/provenance/standing scope, but
the currentness candidate has not closed corpus precedence, and no currentness
resolver, generic admission path, EpistemicGate, identity/delegation system,
or generic write surface should be implemented from it. No runtime, test,
Cargo, Git, or GitHub state was modified by this refresh.

## Historical pre-authority-pass recheck — 2026-08-01

A fresh repository and external-state check found no drift. Local `main`,
`origin/main`, GitHub `main`, and PR #103's base all remain at
`3b46fdb81857d77b827271777b86745bca5f7b12`; PR #103 remains the only open PR,
non-draft, unmerged, and without a formal review decision. Its head remains
`b6cc4dc5ff75672b3b9e5a879ee4f2e749faa5b7`, and the current non-outdated
unresolved thread still points to the ADR-0003/ADR-0005 deferrals at
ADR-0006:425-429.

The report's structural completion check was rerun before the A-024 addition:
all 23 finding rows and detail records are present, all requested deliverable
sections and architecture-map component classes are present, and the stale
validation wording that stopped at A-022 was corrected to cover A-023. This
changes no finding disposition. The only worktree addition remains the
untracked audit report; no runtime, test, Git, or GitHub state was modified.

## Historical canonical and replay focused pass — 2026-08-01

The current `main` tree was exercised at the remaining canonical/replay seam.
`cargo test -p magpie-log --locked --no-fail-fast` passed all 24 log tests,
four Deadbolt fixture tests, 13 golden/canonical tests, and three compile-fail
doc tests. The Python verifier independently accepted the nine-record golden
chain and the two-record portable Deadbolt chain with their pinned roots.

These results strengthen the positive evidence for the normal-root, frozen
binary profile and verify-before-projection path. They do not close A-004's
Rust/Python storage-language and first-failure divergence, A-021's weak-root
strictness coordinate, or A-008's storage durability/resource contract. No
new finding or severity change is warranted: passing the normal vectors does
not decide the hostile or cross-implementation acceptance language. No code,
Git, or GitHub state was modified by this pass.

## Evidence/resolver authority hostile pass — 2026-08-01

A fresh read of the origin-admission, origin-binding, independence-group, and
standing-v3 contracts was compared with the verifier, origin-admission audit,
replay, and standing-v3 implementations. This pass confirmed A-024: the
bundle parser validates canonical structure, exact bytes, anchors, scope, and
provenance coherence, but does not independently verify the claimant's
`human-review` / `authority:origin-review-v0` assertion or bind the claimed
origin group to an immutable authority/root/profile coordinate. The
origin-admission audit then copies the claimed group into its trusted fold
after exact string matching, and v3 counts distinct groups toward `Supported`.

The hostile scenario is two otherwise valid, correctly anchored external-report
bundles for one exact claim/scope, each carrying a fresh claimant-selected
origin group and the exact authority label pair. The current path can count
two groups even though no independent reviewer/authority binding was verified.
That conflicts with the independence-group contract's rule that a self-
declared unique group must not count as corroboration. A fixed label pair
prevents policy substitution, but it does not authenticate the claimant's
authority claim; the earlier rejection of this concern was therefore
reclassified as A-024.

The focused claims-policy validation passed: `cargo test -p magpie-claims
--locked --no-fail-fast` reported 281 passed and 0 failed, including the
origin-admission and standing-v3 suites. Those tests verify the current exact
label and distinct-group behavior; they do not resolve the missing authority
binding contract. The ledger now contains 24 unique findings and 24 detail
records, including six open Constitutional findings (A-001, A-002, A-005,
A-010, A-022, and A-024) and no confirmed Critical finding. No code, Git, or
GitHub state was modified by this pass.

## Persistent audit refresh — 2026-08-02

A fresh local and connector-backed state check found no repository drift:
`main`, `origin/main`, and the audited `HEAD` remain
`3b46fdb81857d77b827271777b86745bca5f7b12`; the only worktree addition is
this untracked audit report. PR #103 remains open, non-draft, unmerged, and
mergeable, based on that same `main` commit and headed by
`b6cc4dc5ff75672b3b9e5a879ee4f2e749faa5b7`. Its three changed files remain
`README.md`, `docs/adr/0004-claim-lifecycle-facets.md`, and
`docs/adr/0006-claim-currency-and-currentness-semantics.md`; the body still
describes only ADR-0006 plus one README entry as its scope. The current
non-outdated unresolved thread remains at ADR-0006:425-429, and review
submissions remain `COMMENTED` with no formal approval. The connector reports
the CodeRabbit status successful; no GitHub mutation was performed.

The fresh corroboration pass found no new distinct finding. A-024 remains
confirmed and separate from A-002: the independence doctrine forbids
self-declared unique groups from counting, while the current exact-label
origin-admission fold accepts claimant-selected groups under the fixed
authority pair and v3 counts them. The relevant integration tests passed:
`cargo test -p magpie-claims --test origin_admission_audit --locked
--no-fail-fast` (18 passed, 0 failed) and
`cargo test -p magpie-claims --test standing_v3 --locked --no-fail-fast`
(13 passed, 0 failed). These tests confirm current behavior and do not close
the missing independent authority-binding contract. The durable ledger remains
at 24 unique findings and 24 detail records, with six open Constitutional
findings and no confirmed Critical finding.

The final physical-log parsing containment check did not produce a new finding.
Rust `FileStore`'s blank-line omission is already an explicit A-004
cross-implementation storage-language divergence, alongside Python's blank-line
failure, and the associated durability/resource concerns remain covered by
A-008. The correct disposition is to keep the issue under those existing
findings rather than inflate the ledger with a duplicate. No code, Git, or
GitHub state was modified by this check.
