# Ticket 0046: Origin-admission audit v0 contract

## 1. Publication identity and exact base

```text
base: 0947169bdb0b6d7744bdcd4c9c12ee677cef5496
branch: agent/origin-admission-audit-v0-contract
commit: docs: ratify origin-admission audit v0
PR title: docs: ratify origin-admission audit v0
PR state: draft
```

The base is the exact post-PR-#58 `main` commit. PR #58 merged with reviewed
head `95c58e7bb38441828426ad753a57e27ade4ea960`, and that reviewed head is an
ancestor of the recorded base.

## 2. Goal and documentation-only authority

Ratify the exact contract for deciding when complete replay-derived,
strictly verified origin-binding statements may produce one standing-inert
admitted-origin assignment.

This ticket creates documentation only. It does not implement verified-prefix
identity, candidate enumeration, origin admission, the audit resolver or any
other runtime capability. It changes no Rust, test, fixture, dependency, L0
surface or existing serialized value.

## 3. Exact changed-path allowlist

Only:

```text
README.md
docs/design/origin-admission-audit-v0.md
tickets/0046-origin-admission-audit-v0-contract.md
```

The design document and this ticket are new. Every historical contract remains
unchanged evidence, including Tickets 0037-0045 and the existing provenance,
closure, origin-binding, aggregation and deterministic-verifier design notes.

## 4. Central architectural laws

```text
verified origin binding
!= trusted authority

trusted origin assignment
!= support contribution

origin admission
!= aggregation

distinct admitted origin groups
!= proof of statistical independence

caller-selected candidate subset
!= complete conflict audit
```

The deterministic law is:

```text
H = exact completely verified Magpie log prefix
P = exact compiled origin-admission policy identity
M = exact immutable ResolutionContentClosureV0 identity

same H + same P + same M
-> byte-identical origin-admission audit
```

No ambient state participates.

## 5. Exact selected policy

```text
policy_id:
magpie-origin-admission-v0
```

Reviewed versioned code selects the policy. The bundle's
`origin_admission_policy_id` is claimant-controlled canonical content and does
not select code.

```text
receipt.origin_admission_policy_id
== magpie-origin-admission-v0
```

Any different value is `policy mismatch`. It remains audit-visible but admits
no origin, enters neither the selected-policy trusted group fold nor its
conflict fold, cannot veto a selected-policy assignment and is not an
eligible-unresolved blocker.

There is no runtime policy parameter, registry, alias, implicit latest policy,
environment override, bundle-selected implementation or caller-selected
policy.

## 6. Exact trusted grouping-authority path

```text
authority.kind:
human-review

authority.reference:
authority:origin-review-v0
```

The compiled selected policy trusts exactly this pair. The pair authorizes only
one exact contribution-scoped origin-group assignment. It authorizes no claim
truth, evidence truth, publisher identity, source quality, support, standing,
settlement, statistical independence, aggregation or writer access.

Any other pair is `origin-binding authority untrusted`. An untrusted matched
binding remains visible but admits no origin, does not enter the trusted fold,
cannot conflict with or veto a trusted assignment and cannot manufacture a
denial-of-service conflict.

No actor class, event provenance, URL, domain, filename, signature count,
model score, metadata flag or bundle-carried Boolean grants authority.

## 7. Complete replay-derived candidate universe

The future API accepts no caller-supplied selector list or receipts.
`OriginBindingCandidateUniverseV0(H)` is derived only from the complete
`DeadboltAnchorIndex` produced by the same replay.

It contains every distinct five-field anchor identity satisfying:

```text
bundle_kind
in {
  magpie-origin-binding-acquisition-v0,
  magpie-origin-binding-derivation-v0
}

witness_algorithm == sha256

canonicalization_profile == magpie-origin-binding-json-v0
```

Selectors use exact lexicographic field order:

```text
bundle_kind
witness_root
witness_algorithm
canonicalization_profile
run_id
```

All repeated exact occurrences remain attached to their one selector in event
sequence order. Repetition does not create more candidate statements or
authority. Unsupported anchor kinds, algorithms and profiles remain outside
v0 and negotiate nothing. Extra unanchored closure bundles are ignored.

The future implementation may add only the smallest crate-private enumeration
seam over the actual same-replay index. It may not expose a mutable map, accept
a caller-created index, change anchor-index or snapshot canonical bytes, change
occurrence lookup or parse serialized index bytes.

## 8. Global unavailable-binding completeness gate

Every candidate selector requires exact bytes from:

```text
closure.foreign_bundle(selector)
```

If any binding bundle is unavailable, completion is
`IncompleteCandidateUniverse`, every unavailable selector is retained in exact
order, and zero contribution decisions or admitted origins are produced
globally.

This v0 rule is global because an unavailable binding's exact subject is
unknown and it could conflict with any contribution. There is no ambient
lookup, best-effort admission, favourable-subset result, caller irrelevance
assertion or partial trusted fold. Available candidates may remain visible as
standing-inert verifier traces but cannot be folded while completion is
incomplete.

A later separately ratified contribution-to-selector index may narrow this
rule. V0 has no such index.

## 9. Shared origin-binding evaluation requirement

Every available candidate must reuse the exact PR #58 parser and resolver
pipeline. The implementation must not duplicate origin-binding parsing,
semantic validation, canonical encoding, selector/root checks, contribution
revalidation, nested provenance composition, artifact-coherence checks or
receipt construction.

A crate-private evaluation result may expose validated contribution,
namespace, policy, authority and claimed group material needed for late-stage
availability-unresolved audit. It must expose no unvalidated parser material as
authority and must preserve unchanged:

```text
OriginBindingContextTraceV0
OriginBindingReceiptV0
StandingReplaySnapshot::resolve_origin_binding_context_v0
```

## 10. Definitive rejection versus scoped unresolved eligibility

Definitive parser, schema, canonical, selector, root, replay-structure, anchor,
digest and artifact-coherence failures remain audit-visible but enter neither
the trusted group partition nor the unresolved blocker set.

A binding becomes scoped unresolved eligible only after establishing:

```text
valid exact origin-binding subject
selected policy ID
trusted authority pair
exact ContributionIdentityV0
exact OriginComparisonNamespaceV0
```

and only because one required closure object is unavailable:

```text
artifact-provenance bundle unavailable
acquisition artifact unavailable
derivation parent artifact unavailable
derivation output artifact unavailable
```

Such a candidate blocks admission only for its exact contribution plus
namespace. Once the subject is known, unrelated contributions are not blocked.
The unresolved claimed group remains audit material and does not enter the
matched group partition.

Disposition order and vocabulary are:

```text
BindingBytesUnavailable
DefinitivelyRejected
PolicyMismatch
AuthorityUntrusted
TrustedEligibleUnresolved
TrustedMatched
```

Only validated material may populate policy, authority, subject, namespace or
group details.

## 11. Trusted duplicate and conflict fold

Only matched receipts satisfying the exact selected policy and exact trusted
authority pair enter the fold. Group by:

```text
ContributionAdmissionKeyV0 {
    contribution: ContributionIdentityV0,
    namespace: OriginComparisonNamespaceV0,
}
```

The namespace remains exactly policy ID, target claim ID and `scope_ref`. It
must not gain evidence ID, edge ID, artifact identity, selector, binding ID,
run ID or complete contribution identity.

Within one key, partition matched receipts by exact opaque `origin_group`.

Duplicate law:

```text
N trusted matched receipts
+ same contribution
+ same namespace
+ same group
-> one assignment
```

Every selector, binding ID, binding run ID, receipt and exact anchor occurrence
remains visible. There is no count, weight, confidence, quorum, majority or
amplification.

Conflict law:

```text
same contribution
+ same namespace
+ selected policy
+ trusted authority
+ two or more distinct matched group strings
-> ConflictingBindings
-> zero admitted groups
```

No write order, sequence, lexical preference, majority, anchor count, binding
ID or run ID selects a winner. Supersession, revocation, expiry and temporal
ownership remain future policies.

## 12. Untrusted and policy-mismatch non-veto law

Wrong-policy and untrusted matched bindings remain candidate audit output but
do not admit, conflict, veto or become unresolved blockers. They cannot
manufacture a conflict against a trusted selected-policy assignment.

## 13. Terminal precedence

For one exact contribution-admission key:

```text
1. two or more distinct matched trusted origin groups
   -> ConflictingBindings

2. otherwise one or more scoped unresolved eligible bindings
   -> EligibleBindingUnresolved

3. otherwise exactly one matched trusted origin group
   -> OriginAdmitted

4. otherwise
   -> no admitted-origin decision
```

A known conflict stays terminal when unresolved candidates also exist. Those
candidates remain referenced in the conflict audit.

`OriginAdmitted` means only that policy `magpie-origin-admission-v0` assigns one
exact contribution to one exact opaque group inside one exact comparison
namespace. It means no truth, publisher identity, independence, reputation,
support, standing, settlement or aggregation.

## 14. Exact verified-prefix identity

```text
VerifiedLogPrefixIdentityV0 {
    event_count
    tip_sha256
}
```

`event_count` is the exact verified record count. `tip_sha256` is the exact
final `ContentHash` rendered as 64 lowercase hexadecimal characters.
Empty-chain behavior remains governed by existing log rules.

Both values must come from the one record snapshot used for replay:

```text
one read_records() snapshot
-> complete verification
-> retained verified events and tip
-> projection application
-> exact event_count and tip identity
```

No second `read_records()` call is permitted. The implementation may add an
additive replay summary or separately versioned origin-resolution context but
must preserve `LogReader::replay`, existing public summaries,
`StandingReplaySnapshot` public fields and canonical bytes, and existing replay
behavior. Hashing snapshot canonical bytes is not log-prefix identity.

## 15. Complete closure identity binding

The audit retains the complete existing:

```text
ResolutionContentClosureIdentityV0 {
    schema
    canonicalization_profile
    digest_algorithm
    manifest_sha256
}
```

It never substitutes the bare manifest digest. The typed identity binds input
`M`; it neither verifies nor reconstructs closure objects.

## 16. Future audit types and exact ordering

The future public concepts are:

```text
VerifiedLogPrefixIdentityV0
OriginAdmissionAuditV0
OriginAdmissionAuditCompletionV0
OriginAdmissionCandidateAuditV0
OriginAdmissionCandidateDispositionV0
OriginAdmissionDecisionV0
AdmittedOriginV0
OriginBindingConflictV0
```

The normative field order is defined completely in
[`origin-admission-audit-v0.md`](../docs/design/origin-admission-audit-v0.md).
At minimum the top-level order is:

```text
schema
canonicalization_profile
policy_id
verified_prefix_identity
closure_identity
completion
candidate_audits
contribution_decisions
```

Candidate audits retain, in order, selector, all ordered anchor occurrences,
the complete existing origin-binding trace, deterministic disposition, and
optional validated contribution and namespace. Optional values serialize as
explicit `null` when they cannot safely be derived.

Decisions are only `ConflictingBindings`, `EligibleBindingUnresolved` or
`OriginAdmitted`, declared in terminal-precedence order. They retain complete
supporting and blocking selector references. There is no decision entry for
the fourth terminal case and no decision at all when the candidate universe is
incomplete.

Ordering is exact:

```text
candidate selectors: exact five-field lexicographic order
anchor occurrences: ascending event sequence
contribution keys: contribution, then namespace
origin groups: exact lexical string order
receipts inside one group: binding-selector order
```

All future public types have private fields, getters only, deterministic
equality and deterministic `Serialize`, no `Deserialize` and no public
constructor from serialized material. Enums use adjacent `outcome`/`details`
tagging with snake-case variants. The top-level audit alone has
`canonical_bytes()`.

## 17. Audit serialization

```text
audit schema:
magpie-origin-admission-audit-v0

audit canonicalization profile:
magpie-origin-admission-audit-json-v0
```

This is purpose-built deterministic derived audit JSON, not Magpie L0,
`magpie-core-v1`, RFC 8785 or JCS. Fields follow contract type order; arrays
follow the exact ordering above; encoding has compact exact strings, direct
UTF-8, no normalization, no whitespace, no BOM and no trailing newline.

No raw artifact or foreign-bundle bytes appear. Serialized audits, decisions,
admitted origins and conflicts are audit output only and cannot substitute for
replay and closure.

The future implementation must pin literal byte fixtures for one admission,
duplicate collapse, trusted conflict, scoped unresolved eligibility and global
candidate-universe incompleteness. This docs-only ticket adds no fixture.

## 18. Standing-inert boundary

The future audit changes no:

```text
StandingView
StandingReplaySnapshot canonical bytes
standing v0
standing v1
standing v2
support or refutation ceiling
deterministic-verifier context
origin-binding verifier public trace or receipt
```

Policy v2 does not consume this audit. No standing changes.

## 19. Explicit non-goals

No admitted-contribution semantics, support contribution, standing policy v3,
aggregation, corroboration threshold, statistical independence, probability,
confidence, source reputation, publisher ontology, trust score, authority
hierarchy, wildcard authority, multiple trusted authority classes,
supersession, revocation, expiry, temporal ownership, loader, CAS,
filesystem/network access, writer authority, L0 payload or Deadbolt change.

No existing standing, replay, closure, deterministic-verifier,
artifact-provenance or origin-binding serialized surface changes.

## 20. Future hostile implementation tests

The implementation must prove:

```text
caller supplies only favourable selector
-> impossible API shape

one replay-anchored binding bundle absent from closure
-> incomplete universe
-> zero admissions globally

extra unanchored closure bundle
-> ignored

three identical anchor occurrences
-> one candidate selector
-> all occurrences visible
-> no amplification

two trusted matched bindings, same group
-> one admitted assignment
-> both receipts visible

two trusted matched bindings, different groups
-> explicit conflict
-> zero admitted groups

trusted match plus untrusted conflicting match
-> trusted group admitted
-> untrusted statement visible
-> no veto

trusted match plus policy-mismatched conflicting match
-> trusted group admitted
-> mismatch visible
-> no veto

trusted match plus same-subject eligible unavailable prerequisite
-> eligible binding unresolved
-> no admission

known trusted conflict plus unresolved candidate
-> conflict remains terminal
-> unresolved candidate retained

same textual group in different namespaces
-> no relationship

same H + P + M
-> byte-identical audit

different M
-> visibly different closure identity and audit

different H with same derived standing projection
-> different verified-prefix identity

serialized audit or admitted-origin value
-> cannot substitute for replay and closure

audit execution
-> no standing v0/v1/v2 byte change
```

## 21. Recommended later implementation sequence

```text
Slice 1:
same-snapshot verified-prefix identity
+ deterministic replay-anchor candidate enumeration

Slice 2:
standing-inert OriginAdmissionAuditV0 implementation
+ candidate completeness
+ trusted duplicate/conflict fold

Then:
admitted-contribution audit
-> policy-v3 contract
-> conservative aggregation
```

Slices 1 and 2 may be combined later only when their authority boundaries and
tests remain independently reviewable.

## 22. Validation commands

Run and report actual results only:

```powershell
git diff --check
cargo fmt --all --check
cargo test --workspace --locked
cargo run --locked --example tour -p magpie-claims

git status --short
git diff --name-only
git diff --stat

git diff -- README.md
git diff -- docs/design/origin-admission-audit-v0.md
git diff -- tickets/0046-origin-admission-audit-v0-contract.md

git diff | Select-String -Pattern `
  'implemented|runtime resolver|now admits|standing changes|policy v3|aggregation complete'
```

Review every search match manually. The exact changed path list must be the
three paths in section 3.

Before commit, stage only those paths and run:

```powershell
git diff --cached --name-only
git diff --cached --stat
git diff --cached --check
git diff --cached
```

After the one authorized commit, confirm one commit above the exact base and a
clean tree, then run:

```powershell
python tools/check_release_metadata.py
cargo package -p magpie-log --locked
```

If either clean-tree command changes generated files or exposes an inventory
mismatch, stop instead of widening the PR.

## 23. Worker and publication authority

`AGENTS.md` remains binding except that George explicitly authorizes, only
after the exact scope and every pre-commit validation pass:

- staging only the three allowlisted paths;
- creating exactly one commit with the title in section 1;
- pushing only the named branch without force; and
- opening exactly one draft PR with the title in section 1.

The worker may not modify `main`, add runtime code or fixtures, mark the PR
ready, merge, enable auto-merge, force-push, resolve or dismiss review threads,
modify another PR, or create another branch or commit.

## 24. Stop conditions

Stop and report rather than drafting around any need to:

- treat a caller-provided selector list as complete;
- trust a bundle-carried Boolean;
- let untrusted bindings veto trusted admission;
- omit exact log-tip identity;
- obtain the tip through a second store read;
- change an existing standing or origin-binding serialized surface;
- change closure construction;
- change Magpie L0;
- add runtime code or fixtures;
- add another policy or authority path; or
- edit outside the three-path allowlist.

## 25. Reviewer questions

1. Does the complete candidate universe come only from the same replay's full
   anchor index, with caller candidate lists impossible?
2. Does any missing replay-anchored binding bundle make the whole v0 audit
   incomplete and suppress all decisions?
3. Are selected policy and trusted authority exact compiled choices rather
   than bundle- or caller-selected values?
4. Can any untrusted or policy-mismatched statement veto or conflict with a
   trusted selected-policy assignment?
5. Is scoped unresolved eligibility limited to availability-only prerequisite
   failures after exact subject, policy and authority validation?
6. Does conflict remain terminal above unresolved and admission, while
   duplicates collapse without losing receipts or occurrences?
7. Are event count and tip derived from the exact one verified record snapshot
   used for projection application, with no second store read?
8. Does the audit bind the complete four-field closure identity and never only
   its manifest digest?
9. Are all future audit values private-construction, deterministic,
   serialization-only material that no resolver accepts as authority?
10. Do standing v0/v1/v2, support/refutation ceilings, deterministic-verifier
    context and existing origin-binding traces remain unchanged?
11. Does the documentation avoid claiming runtime origin admission exists?
