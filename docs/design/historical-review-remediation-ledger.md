# Historical Review Remediation Ledger

## 1. Status and audit boundary

Ratified documentation contract under Ticket 0043.

Resolved base:

```text
a72d994b951e5dcf18980462dc590c48a7829f0e
```

This ledger accounts for the 32 retained inline Codex review findings returned
by the GitHub review-comment audit across Magpie PRs #1-54. It records the
architectural disposition of each finding. It does not change Rust, tests,
fixtures, canonical bytes, policy behavior, writer behavior, or GitHub thread
state.

GitHub thread resolution is workflow metadata. A resolved, outdated, or open
thread is not itself proof that the architectural concern was remediated. Each
entry below names the concrete later mechanism, doctrine, bounded follow-up, or
rationale that disposes of the concern.

## 2. Closed classification rules

Every retained finding receives one of these classifications:

### Remediated

The current repository contains a concrete implementation, test, or ratified
contract that directly addresses the finding. The ledger names that mechanism.

### Superseded by later ratified architecture

A later explicit architecture replaced the premise or proposed solution. The
later boundary must be named; “the thread was resolved” is not a disposition.

### Live documentation correction in this PR

Current behavior is already coherent, but a scoped document describes it
incorrectly or in an impossible order. Ticket 0043 changes prose only.

### Live dedicated implementation follow-up

The concern is valid and remains unimplemented. This ledger freezes a narrow
future implementation contract; Ticket 0043 does not claim it has landed.

### Rejected or narrowed with rationale

The original proposed remedy would cross an authority, compatibility, or scope
boundary, or it treated a ceiling/advisory value as achieved authority. The
ledger records the narrower law that remains valid.

No category means “ignored,” and no category is inferred from GitHub thread
state.

## 3. Ratified doctrine checkpoints

### 3.1 Legacy raw standing and governed standing

```text
legacy_raw_standing
= quarantined compatibility and audit material
= never governed authority

legacy raw Settled
does not establish governed Settled

legacy raw Refuted
does not establish a governed veto

inherited governed Refuted
remains Refuted until an explicit versioned invalidation,
supersession, recovery, or refutation policy changes that law
```

Policy v2 may derive governed `Supported` from successful deterministic
verification while the separately exposed raw value remains `Refuted`. This is
not revival of a governed refutation because the raw value was never governed
standing. An inherited governed `Refuted` result remains `Refuted`.

### 3.2 Low-level writer capability

```text
possession of LogWriter plus its signing key
= raw L0 append capability

absence of an ordinary CLI, MCP, or application writer
!= absence of a low-level Rust write surface
```

```text
magpie-log validates L0 structural integrity

magpie-log must not absorb higher-level epistemic admission policy

future EpistemicGate
→ governs construction and admission of claim/evidence/edge writes
→ then invokes the raw append capability
```

Ticket 0043 neither restricts `LogWriter::append` nor adds a gate, capability
token, writer facade, payload visibility change, or Rust API change.

### 3.3 Canonical metadata bytes versus authority

```text
metadata_json bytes
= signed L0 canonical event material

an origin-group or independence label inside metadata_json
= asserted opaque content
!= a native typed origin-assignment field
!= admitted origin authority
!= corroboration separation

committed as bytes
!= interpreted as authority
```

Changing `metadata_json` changes the event's canonical bytes and hash. The
string is not canonically parsed as JSON by L0, cannot change the
canonicalisation profile or rules, cannot admit itself, and cannot create an
authoritative origin group.

### 3.4 Deterministic-verifier dependency order

Before witness parsing, the checker re-fetches the claim, evidence, and edge;
validates supports/source/target structure, evidence kind, and claim domain;
and validates:

```text
claim.scope_ref == evidence.scope_ref == edge.scope_ref
```

After strict witness parsing, it validates the witness schema and subject
binding, then validates `witness.scope_ref` against the already matched replay
scope. Successful verification therefore still requires four-way equality,
but the design no longer claims to inspect a witness field before parsing the
witness.

## 4. Complete retained-finding ledger

| PR | Retained finding | Classification | Concrete disposition |
| ---: | --- | --- | --- |
| #1 | Add Cargo `--locked` to dependency-resolving CI checks. | Remediated | CI runs Clippy, workspace tests, the tour, and package verification with `--locked`; lockfile drift cannot be silently refreshed by those gates. |
| #5 | Escape literal prose before SQLite FTS `MATCH`. | Remediated | `fts_phrase_query` wraps user text as one FTS5 phrase and doubles embedded quotes; tests cover `1e-15` and `N=64..192`. |
| #5 | Preserve signed claim status in episodic rows. | Remediated | Episodic schema v2 retains `claim_status`, `from_status`, and `to_status`; projection and canonical projection-byte tests preserve the signed status distinctions. |
| #8 | Validate `SegmentAnchored.witness_root` before signing and during verification. | Remediated | `Payload::validate` requires exactly 64 lowercase hexadecimal characters; writer, replay verification, Rust tests, and the independent verifier share the contract. |
| #10 | Migrate existing episodic databases. | Remediated | `SCHEMA_VERSION = 2`; a stale derived schema is transactionally dropped and rebuilt, and tests prove stale rebuild plus current-schema reopen behavior. |
| #18 | Disambiguate edge endpoint namespaces. | Rejected or narrowed with rationale | L0 keeps `source_id` and `target_id` as opaque strings rather than adding endpoint-kind fields. Policy-specific consumers must interpret the closed edge kind and re-fetch the exact expected typed tables; implemented `supports` lanes require evidence source and claim target. General claim/evidence/edge targeting remains a future explicit convention, not ambient inference. |
| #18 | Require negative cross-scope support coverage. | Remediated | Standing candidate resolution requires exact claim/evidence/edge scope equality, reports `ScopeMismatch`, and `scope_mismatch_blocks_support` pins the fail-closed path. |
| #18 | Treat public payload variants as a writer surface. | Live dedicated implementation follow-up | The low-level `LogWriter` capability is now stated honestly. `magpie-log` remains structural; a future `EpistemicGate` must govern claim/evidence/edge construction and admission before any ordinary claim-bearing CLI, MCP, or application writer invokes raw append. |
| #18 | Define scoped claim identity. | Remediated | `StandingView` keys typed claims by exact `claim_id` with deterministic first-write retention; `scope_ref` is retained as bound claim content and every standing-bearing lane revalidates exact scope. Same-ID later assertions do not create a second silently merged claim. |
| #18 | Add a structured Deadbolt anchor reference. | Superseded by later ratified architecture | Tickets 0025-0028 introduced duplicate-aware structured Deadbolt occurrence identity and one-snapshot context derived from exact five-field `SegmentAnchored` matches. No new L0 endpoint field was needed; metadata remains asserted until that strict parser and replay context match. |
| #18 | Define duplicate edge-ID replay semantics. | Remediated | Typed edge replay is deterministic first-write-wins; `duplicate_justification_edge_recorded_does_not_overwrite` proves a later conflicting definition neither replaces nor amplifies the first. |
| #21 | Require verifier context before Deadbolt settlement. | Remediated | `SupportContextRequirement::DeadboltVerifierContext` gates all positive Deadbolt cells, and policy v1 settles only a same-snapshot exact occurrence match. A bare `DeadboltAnchor` label remains inert. |
| #21 | Gate HumanRoot ratification before settlement. | Superseded by later ratified architecture | Ticket 0019 ratified that humans settle nothing epistemically. Human ratification may later support bounded governance/judgment claims only through `HumanAdmission`; no HumanRoot path reaches epistemic `Settled`. |
| #21 | Preserve source standing for support edges. | Superseded by later ratified architecture | Later doctrine separates Candidate, PolicyEligible, Admitted, and Support Contribution. No generic support fold landed; any future aggregation must cap claim-source contribution by governed source standing, while current v1/v2 direct lanes consume exact verifier receipts instead. |
| #23 | Remove zero-width characters from domain headers. | Rejected or narrowed with rationale | Display wrapping is not an authority-bearing identifier surface. Closed Rust enums and exact ASCII parser strings define claim-domain identity; rendered table typography cannot select a domain or policy. |
| #23 | Keep metadata placement in the metadata convention. | Remediated | `docs/design/metadata-conventions.md` owns the advisory `claim_domain`, source-locator, quote-hash, and edge-target conventions while L0 retains one opaque `metadata_json` string. |
| #23 | Do not broaden `DeadboltAnchor` beyond anchor domains. | Rejected or narrowed with rationale | A ceiling is not achieved standing. All positive Deadbolt cells require exact verifier context; the only implemented Deadbolt standing rule is occurrence/inclusion v1. Other documented ceilings create no support without a separately reviewed rule. |
| #23 | Restrict `LensReadout` to model-introspection claims. | Rejected or narrowed with rationale | The ratified matrix permits at most `Conjectured` hypothesis seeding for `Interpretation` and `ModelIntrospection`, and no contribution elsewhere. A lens label cannot produce `Supported`, `Settled`, or world-truth authority. |
| #24 | Define quote-hash input bytes. | Remediated | `magpie-quote-v1` fixes Unicode NFC, line-ending normalization, whitespace preservation, UTF-8 encoding, algorithm naming, and lowercase digest rendering; it explicitly distinguishes text-to-hash determinism from extraction determinism. |
| #29 | Remove remaining HumanRatification settlement guidance. | Remediated | The tag plan now says `ratifies` may support bounded governance/judgment claims only after admission and must not settle truth. Ticket 0019 doctrine is explicit. |
| #29 | Correct the dossier's HumanRoot card. | Remediated | The dossier states that HumanRoot may ratify governance, consent, preferences, scoped judgments, or operational decisions, but not epistemic settlement. |
| #29 | Keep `ExternalSource` below settlement. | Remediated | Support ceilings cap it at `Supported`, refutation ceilings never grant it `Refuted`, and the aggregation note states that corroboration cannot make an external report `Settled`. |
| #29 | Retain HumanRoot and exact-scope ratification tests. | Superseded by later ratified architecture | Settlement is now forbidden, while any future bounded human support remains blocked by `HumanAdmission` and exact scope. Its actor/scope tests belong to the future `EpistemicGate` slice, not a current achieved-standing rule. |
| #32 | Clarify that `metadata_json` remains canonical input. | Live documentation correction in this PR | The standing aggregation note now distinguishes signed canonical string bytes from policy interpretation and removes the inaccurate “not L0 canonical material” implication. |
| #32 | Mark `StandingResolution` as future rather than current substrate. | Superseded by later ratified architecture | At review time the surface was pending; Ticket 0023 later landed `StandingResolution` v0. It is now correctly current, policy-identified, fail-closed, and separately exposes quarantined raw status. |
| #32 | Do not treat current support edges as achieved candidates. | Remediated | The closed vocabulary now defines Candidate as inspectable material only and distinguishes it from PolicyEligible, Admitted, Support Contribution, aggregation, and achieved standing. |
| #37 | Avoid retaining parsed events for verification-only callers. | Live dedicated implementation follow-up | Correctness and authority remain intact, but `verify_chain_on` currently builds `VerifiedSnapshot.events`, and `LogStore::read_records` materializes all raw records. Section 5 freezes a count/tip-only verification mode without weakening one-read verify-before-apply replay. |
| #45 | Keep release wording conditional until the tag exists. | Superseded by later ratified architecture | The human-created `v0.1.0` tag and GitHub prerelease now exist; the release contract pins the exact source commit and preserves the no-registry-publication boundary. |
| #45 | Keep verifier implementation before aggregation. | Remediated | The standing-inert deterministic verifier and explicit non-aggregating policy-v2 direct-support rule landed before any aggregation; the next sequence still places origin verification and audits before policy v3. |
| #45 | Move witness scope validation after witness parsing. | Live documentation correction in this PR | The checker sequence now performs three-way replay scope validation before witness parsing and completes four-way equality only after strict witness parsing. Runtime order and failure vocabulary are unchanged. |
| #47 | Preserve raw refutations before deterministic support. | Rejected or narrowed with rationale | The proposal treated quarantined compatibility state as governed authority. Ticket 0043 ratifies that legacy raw `Refuted` is audit-only, so policy v2 may derive governed `Supported`; an inherited governed `Refuted` still wins. Dedicated regression tests remain required. |
| #54 | Reject empty origin-binding `scope_ref`. | Remediated | Before merge, both origin-binding documents changed the replay-reference bound to 1-1,024 UTF-8 bytes and pinned `invalid replay reference length` before replay lookup and before any matched receipt. Normative vectors and roots remained unchanged. |

## 5. Live dedicated implementation follow-ups

### 5.1 Legacy raw-refutation regression tests

One focused test-only PR must prove both current laws:

```text
legacy raw Refuted
+ successful deterministic verification
→ governed Supported
+ raw Refuted remains audit-visible
```

```text
inherited governed Refuted
+ successful deterministic verification
→ governed Refuted
```

This follow-up must not change policy identifiers, standing behavior, canonical
resolution bytes except by adding new pinned fixtures, or the meaning of raw
compatibility state.

### 5.2 Verification-only memory retention

The live debt is:

```text
verification-only callers currently reuse a path that retains
the complete parsed SignedEvent vector

the LogStore interface also materialises complete raw records
```

Classification:

```text
correctness: unaffected
authority boundaries: unaffected
availability and long-run scalability: potentially affected
```

The dedicated optimisation contract is:

```text
verification-only mode
→ count and tip without retaining parsed events

replay mode
→ retain exact events from one completely verified snapshot

both modes
→ one snapshot read
→ complete verification before projection mutation
```

Changing `LogStore` to avoid complete raw-record materialization may require a
separate explicit interface design. The optimization must not reintroduce
multiple reads, prefix application, or verification/projection snapshot skew.

### 5.3 Higher-level write authority boundary

Before any ordinary CLI, MCP, or application claim-bearing writer is exposed,
a separately reviewed implementation must make `EpistemicGate` govern event
construction and admission, then invoke the existing raw append capability.
It must keep L0 structural validation separate from epistemic policy. Ticket
0043 neither designs the complete gate policy nor changes `LogWriter`.

## 6. PR #54 remediation record

The merged origin-binding contract now requires:

```text
scope_ref: 1-1,024 UTF-8 bytes

empty contribution.scope_ref
→ invalid replay reference length
→ no replay lookup
→ no matched origin-binding receipt
```

Ticket 0043 does not edit the origin-binding contract, either bundle schema,
canonical vectors, byte lengths, or SHA-256 roots.

## 7. Authority and history boundary

This ledger ratifies doctrine and classifies work. It does not:

- reinterpret GitHub thread state as evidence;
- rewrite completed Tickets 0001-0042;
- create a generic policy registry or ambient latest policy;
- let callers choose policy by supplying an identifier;
- turn a trust Boolean, evidence label, metadata key, or origin-group string
  into authority;
- claim that future tests or optimisations are already implemented; or
- change runtime, cryptographic, replay, standing, writer, provenance, origin,
  admission, or aggregation behavior.
