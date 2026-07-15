# Ticket 0043: Historical review remediation contract

## 1. Exact resolved base and worker authority

This documentation-only ticket starts exactly from merged PR #54 at:

```text
a72d994b951e5dcf18980462dc590c48a7829f0e
```

That merge contains the reviewed PR #54 head:

```text
7c592492883d1f46c0c489c93007b92c7829ad67
```

The branch is exactly:

```text
agent/historical-review-remediation-contract
```

The suggested human commit and PR title are both:

```text
docs: ratify historical review remediations
```

`AGENTS.md` is binding. Codex may edit and validate the exact documentation
scope but may not commit, push, open or edit a PR, mark readiness, merge,
resolve review threads, or enable auto-merge.

## 2. Purpose and non-goals

This ticket records the architectural disposition of retained Codex review
findings across Magpie PRs #1-54. It ratifies four doctrine corrections,
records one scalability debt and the merged PR #54 remediation, and classifies
bounded future work.

The governing distinction is:

```text
GitHub thread state
!= architectural disposition

architectural disposition
= concrete remediation
or later ratified replacement
or explicit documentation correction
or bounded implementation follow-up
or rejection/narrowing with rationale
```

This ticket implements no runtime capability. It does not change Rust, tests,
fixtures, formats, vectors, public APIs, policies, standing behavior, writer
behavior, Deadbolt behavior, origin admission, or aggregation.

## 3. Exact changed paths

Exactly these six paths may change:

```text
README.md
docs/design/adr-0002-tags-6-8-implementation-plan.md
docs/design/historical-review-remediation-ledger.md
docs/design/replayable-deterministic-verifier-admission-v0.md
docs/design/standing-aggregation-independence-groups.md
tickets/0043-historical-review-remediation-contract.md
```

The ledger and this ticket are new. Tickets 0001-0042 are historical and must
not change. No other path is permitted.

## 4. Complete ledger classification rules

Every retained inline review finding receives exactly one primary
classification in
[`historical-review-remediation-ledger.md`](../docs/design/historical-review-remediation-ledger.md):

### Remediated

A concrete current implementation, regression test, or ratified contract
directly addresses the concern. The ledger must name it.

### Superseded by later ratified architecture

A later explicit architecture replaced the original premise or remedy. The
ledger must name the replacement and may not rely on thread state.

### Live documentation correction in this PR

Runtime behavior is already coherent but current prose is inaccurate,
authority-confusing, or dependency-ordered impossibly. This PR changes only
the scoped prose.

### Live dedicated implementation follow-up

The concern remains valid and unimplemented. The ledger freezes a narrow
future contract and states the preserved trust boundaries.

### Rejected or narrowed with rationale

The proposed remedy is too broad, crosses a frozen boundary, or confuses raw,
candidate, ceiling, advisory, or claimed material with governed authority. The
ledger states the narrower law and why it is sufficient.

“Resolved,” “outdated,” “dismissed,” or “open” on GitHub is never an
architectural classification.

## 5. Ratified doctrine correction 1: legacy raw versus governed standing

Ratify exactly:

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

Policy v2 may derive `Supported` from successful deterministic verification
even when the separately exposed legacy raw value is `Refuted`. This is not
revival of a governed refutation. The raw value was never governed standing.

An inherited governed `Refuted` result is different. Current v2 combination
preserves it, and no support path may override it without a separately reviewed
versioned policy change.

### Required dedicated regression-test follow-up

One focused later PR must prove both:

```text
legacy raw Refuted
+ successful deterministic verification
→ governed Supported
+ raw Refuted remains audit-visible
```

and:

```text
inherited governed Refuted
+ successful deterministic verification
→ governed Refuted
```

Ticket 0043 adds no tests and changes no standing behavior.

## 6. Ratified doctrine correction 2: low-level writer capability

Correct every implication that Magpie literally lacks a writer-facing surface:

```text
possession of LogWriter plus its signing key
= raw L0 append capability

absence of an ordinary CLI, MCP, or application writer
!= absence of a low-level Rust write surface
```

The layer boundary is:

```text
magpie-log validates L0 structural integrity

magpie-log must not absorb higher-level epistemic admission policy

future EpistemicGate
→ governs construction and admission of claim/evidence/edge writes
→ then invokes the raw append capability
```

Do not restrict `LogWriter::append` here. Do not add a gate, capability token,
writer facade, payload visibility change, or Rust API change.

A future authority-boundary implementation is mandatory before Magpie exposes
ordinary CLI, MCP, or application claim-bearing write surfaces. That slice must
keep structural L0 validation separate from higher-level epistemic admission.

## 7. Ratified doctrine correction 3: canonical metadata wording

Replace the inaccurate implication that origin-group metadata is not L0
canonical material.

```text
metadata_json bytes
= signed L0 canonical event material

an origin-group or independence label inside metadata_json
= asserted opaque content
!= a native typed origin-assignment field
!= admitted origin authority
!= corroboration separation
```

Changing `metadata_json` changes the exact event canonical bytes and hash.
L0's canonical encoder commits the length-prefixed string bytes; it does not
parse or canonicalize the string *as JSON*.

The authority distinction is:

```text
committed as bytes
!= interpreted as authority
```

Such metadata cannot change the canonicalisation profile or encoding rules,
cannot admit itself, and cannot create an authoritative origin group.

## 8. Ratified doctrine correction 4: verifier validation order

Align the conceptual checker with the implemented dependency order.

Before witness parsing:

- re-fetch claim, evidence, and edge;
- validate supports/source/target structure;
- validate evidence kind and claim domain; and
- validate three-way replay scope equality:

```text
claim.scope_ref == evidence.scope_ref == edge.scope_ref
```

After strict witness parsing:

- validate witness schema and subject binding;
- validate `witness.scope_ref` against the already matched replay scope; and
- thereby complete four-way scope equality.

The contract must not claim that `witness.scope_ref` is checked before the
witness is parsed. Successful verification still requires four-way equality.

This is a documentation-order correction only. Failure enums, parser/checker
implementation, tests, and verifier behavior remain unchanged.

## 9. Verification-memory scalability debt

Record the live PR #37 debt exactly:

```text
verification-only callers currently reuse a path that retains
the complete parsed SignedEvent vector

the LogStore interface also materialises complete raw records
```

Classify it as:

```text
correctness: unaffected
authority boundaries: unaffected
availability and long-run scalability: potentially affected
```

The dedicated future contract is:

```text
verification-only mode
→ count and tip without retaining parsed events

replay mode
→ retain exact events from one completely verified snapshot

both modes
→ one snapshot read
→ complete verification before projection mutation
```

The optimization must not restore a verify-then-reread flow, apply a valid
prefix before later failure, mix snapshots, weaken payload validation, or let
projection mutation occur before the complete snapshot verifies.

Avoiding complete raw-record materialization at the `LogStore` boundary may
require its own explicit interface design. Ticket 0043 does not choose that
API or implement the optimization.

## 10. PR #54 remediation record

Record that the final Codex finding was remediated before merge:

```text
scope_ref: 1-1,024 UTF-8 bytes

empty contribution.scope_ref
→ invalid replay reference length
→ no replay lookup
→ no matched origin-binding receipt
```

Ticket 0043 must not edit either origin-binding document, either schema, any
field or field order, canonicalization rules, vector bytes or lengths, roots,
selectors, identifiers, namespaces, or verifier precedence.

## 11. Required historical ledger coverage

The ledger must account for all retained inline Codex findings in the audit,
including at minimum:

- CI Cargo `--locked`;
- literal FTS search escaping;
- retained signed episodic status;
- `SegmentAnchored.witness_root` validation;
- episodic schema migration;
- edge endpoint namespace ambiguity;
- cross-scope support;
- scoped claim identity;
- structured Deadbolt context;
- duplicate edge semantics;
- HumanRoot settlement wording;
- ExternalSource settlement wording;
- Deadbolt verifier-context requirement;
- LensReadout domain restriction;
- quote-hash byte definition;
- metadata canonical-material wording;
- deterministic witness validation order;
- low-level writer capability;
- verification memory retention;
- legacy raw `Refuted`; and
- origin-binding empty `scope_ref`.

For every row, state the concrete later mechanism, doctrine, follow-up, or
narrowing rationale. Never use GitHub resolution state as the mechanism.

## 12. Hostile reviewer questions

### Does legacy raw `Refuted` veto policy v2 support?

No. It is quarantined compatibility/audit material and never governed
authority. The follow-up test must prove the raw value remains visible.

### Can deterministic support override inherited governed `Refuted`?

No. Current v2 composition preserves governed `Refuted`. Changing that law
requires an explicit versioned invalidation, supersession, recovery, or
refutation policy.

### Does this PR change current standing behavior to resolve PR #47?

No. It ratifies the boundary already expressed by the governed/raw split and
records missing focused regression coverage.

### Does Magpie have no writer?

No. A caller holding `LogWriter` plus its signing key has raw L0 append
capability. Magpie lacks an ordinary governed CLI/MCP/application writer.

### Should `magpie-log` reject epistemically inadmissible claims?

No. L0 validates structural integrity. Higher-level epistemic admission belongs
in a future `EpistemicGate`, which then invokes raw append.

### Does signing `metadata_json` make an origin label trusted?

No. Signing commits the exact bytes and provenance assertion. It does not
admit the label, prove authority, or create corroboration separation.

### Can metadata select another canonicalization profile?

No. It is a length-prefixed string inside the fixed event encoding. Its content
cannot alter the profile or encoding rules that committed it.

### Can the checker compare `witness.scope_ref` before parsing the witness?

No. It first checks three replay scopes, parses the witness strictly, then
completes four-way equality.

### Does the memory follow-up permit a second store read?

No. Both modes retain the one-snapshot boundary. Replay verifies completely
before applying any event.

### Is the memory debt a correctness or authority defect?

No. It is availability and long-run scalability debt. The current one-read
verify-before-apply behavior remains correct and fail-closed.

### Does Ticket 0043 alter the PR #54 vector roots?

No. The origin-binding contract and all vectors are prohibited paths.

### Does resolving a GitHub thread resolve the architecture?

No. Only the concrete disposition recorded in the ledger does.

## 13. Frozen surfaces and explicit non-goals

This ticket changes no:

- Rust source;
- tests or fixtures;
- Cargo manifest or lockfile;
- public API;
- L0 payload or canonical encoding;
- `docs/FORMAT.md`;
- golden vector or trust root;
- Python verification tool;
- policy identity;
- support or refutation matrix;
- governed standing behavior;
- Deadbolt behavior;
- origin-binding schema, canonical bytes, byte length, or SHA-256 root;
- acquisition or derivation protocol;
- `ResolutionContentClosureV0` behavior;
- writer runtime behavior;
- origin admission or contribution admission;
- policy v3 or aggregation; or
- completed historical ticket.

It introduces no generic policy registry, ambient latest policy,
caller-selected policy, trust Boolean, inferred authority, or new runtime
promise.

## 14. Required validation

Run and report actual results only:

```powershell
git diff --check
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo run --locked --example tour -p magpie-claims

git status --short
git diff --name-only
git diff
```

Confirm exactly the six scoped paths changed and no historical ticket changed.

Search the complete diff for accidental runtime or authority claims:

```text
implemented
enforced at runtime
trusted origin
admitted origin
aggregation active
writer gated
legacy Refuted veto
metadata not canonical
```

Review every match in context. Historical descriptions of already-landed
mechanisms are permitted; claims that Ticket 0043 implements runtime behavior
are prohibited.

## 15. Reviewer checklist

- Confirm base `a72d994b951e5dcf18980462dc590c48a7829f0e` is exact.
- Confirm exactly the six scoped paths changed.
- Confirm Tickets 0001-0042 remain untouched.
- Confirm all retained findings receive one closed classification and a
  concrete disposition.
- Confirm GitHub thread state is never used as architectural evidence.
- Confirm raw legacy standing is separated from inherited governed standing.
- Confirm both legacy/inherited `Refuted` test obligations are explicit.
- Confirm low-level raw append capability is acknowledged without gating it.
- Confirm higher-level admission remains outside `magpie-log`.
- Confirm metadata is committed as canonical bytes but not interpreted as
  authority merely by being committed.
- Confirm verifier scope order is three-way before parsing, four-way after.
- Confirm memory optimization preserves one read and verify-before-apply.
- Confirm PR #54's empty-scope remediation is recorded without editing its
  contract or vectors.
- Confirm no runtime, cryptographic, policy, writer, origin, or aggregation
  implementation claim appears.
- Confirm every reported validation actually ran.

## 16. Proposed next sequence

```text
Ticket 0043 historical review remediation contract
→ legacy raw-refutation regression tests
→ verification-only memory-retention optimisation
→ standing-inert origin-binding verifier
→ standing-inert origin-admission audit
→ admitted-contribution audit
→ policy v3
→ conservative aggregation
```

The future `EpistemicGate` authority boundary remains mandatory before exposing
ordinary claim-bearing write surfaces. It is not inserted into this immediate
standing/provenance sequence and is not implemented here.
