# Ticket 0024: Closed support-context requirement policy

## Goal

Extract the privileged support-context classification embedded in
`StandingResolution` into a pure, closed policy over every `EvidenceKind` and
`ClaimDomain` pair, without changing governed standing or admitting evidence.

## Architectural context

The policy preserves these distinct stages:

```text
candidate
    != policy-eligible
    != admitted
    != aggregated
    != achieved standing
```

It classifies only what additional privileged context a support-policy cell
would require before a future policy could admit it. Evidence-kind and
actor-class labels do not prove authority or verifier provenance.

## Requirement states

- `NoSupportContribution`: `support_ceiling` rejects the cell.
- `NoPrivilegedContext`: this surface requires no HumanRoot or verifier-specific
  privileged context. It does not imply admission, trust, aggregation,
  promotion, or truth.
- `HumanAdmission`: future replayable human admission is required; humans
  settle nothing epistemically.
- `DeterministicVerifierContext`: future deterministic verifier context is
  required and has not been supplied by the evidence label.
- `DeadboltVerifierContext`: future Deadbolt verifier context is required and
  has not been supplied by the evidence label.

## Exhaustive matrix

`support_context_requirement` must explicitly classify all eight evidence kinds
across all seven claim domains. All 56 cells remain visible in the policy
matrix; wildcard arms are forbidden.

The category counts are frozen:

```text
NoSupportContribution:          37
HumanAdmission:                  3
DeterministicVerifierContext:    3
DeadboltVerifierContext:         4
NoPrivilegedContext:             9
Total:                          56
```

## Behaviour-preservation invariant

For existing inputs, policy extraction must preserve `governed_standing`,
`legacy_raw_standing`, `currentness`, `policy_id`, `trace`, `blockers`, and
`canonical_bytes()` exactly. `magpie-claims-standing-v0` and serialized
resolution shapes remain unchanged.

## Acceptance criteria

- The public enum and pure helper expose the five closed requirement states.
- `NoSupportContribution` is equivalent to a missing support ceiling.
- Exact category counts and doctrine-sensitive cells are tested.
- `StandingResolution` delegates context trace reasons to the helper.
- A support candidate is accepted only when its support ceiling exists and its
  context requirement is not `NoSupportContribution`. Either surface fails
  closed in release builds; debug assertions are diagnostic only.
- Candidate ceilings remain candidate-only and never affect achieved standing.
- A representative canonical resolution byte sequence is pinned.

## Non-goals

No admission decisions or tokens, verifier records or payloads, Deadbolt or
cryptographic verification, achieved `Supported`, `Settled`, or `Refuted`,
aggregation, independence groups, corroboration, source-standing propagation,
contradiction debt, invalidation, supersession, currentness, `EpistemicGate`,
writer surfaces, model/lens ingestion, checkpoint publication, network
services, or Verus integration.

## Frozen surfaces

No changes to L0 payloads or tags, `docs/FORMAT.md`, canonical encoding,
signatures, golden fixtures or hashes, `tools/verify_chain.py`, legacy replay,
support/refutation ceiling cells, evidence-kind/domain strings, Deadbolt,
Cogitator, MCP surfaces, or writer surfaces.

## Validation

```text
git diff --check
cargo fmt --all -- --check
cargo test -p magpie-claims --locked
cargo test --workspace --locked
cargo clippy --locked -p magpie-claims --all-targets -- -D warnings
rg -n "SupportContextRequirement|support_context_requirement|RequiresAdmission|RequiresVerifierContext|NoSupportContribution|NoPrivilegedContext|HumanAdmission|DeterministicVerifierContext|DeadboltVerifierContext" crates docs tickets
rg -n "_ =>|, _\\)|\\(_," crates/magpie-claims/src/policy.rs
git diff --name-only
git status --short
```

## Follow-up

The next phase is the narrowly scoped **verified Deadbolt
occurrence/inclusion achieved-standing slice**. This ticket does not admit or
settle Deadbolt evidence.
