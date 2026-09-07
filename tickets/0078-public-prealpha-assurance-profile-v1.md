# Ticket 0078: Public pre-alpha assurance profile v1

## Status

Documentation-only C4.1 contract. It does not implement the follow-up tranches,
change runtime behavior, or make a release decision.

## Goal

Freeze the smallest defensible evidence boundary for the public pre-alpha under
the exact profile identity `magpie-public-prealpha-assurance-v1`.

## Scope

- Define the selected caller-supplied finite-history, portable-verification,
  deterministic-replay, and supported local SQLite L0 boundary.
- Distinguish selected inputs from observed execution coordinates.
- Define semantic, front-door, resource/crash, dependency, reproducibility,
  package, checker, and owner evidence requirements.
- Record explicit claim fences, a completion criterion, a gap matrix, and bounded
  later tranches.

## Non-goals

- No Rust, Python, Go, CI, dependency, lockfile, FORMAT, ADR, corpus, package,
  release metadata, or runtime changes.
- No detachable C3 receipt or coordinate-poor compatibility redesign.
- No finding-status edit, release approval, C5 candidate freeze, merge, or release.

## Authority and assumptions

The selected base is live `origin/main` at
`313287c8ac9a0b266ab3a477b143d79072b88650`. Current implementation and accepted
doctrine outrank historical audit prose. The owner-directed C3 exclusion is
preserved: no detachable governed verification/replay result is advertised.

The owner remains the sole authority for acceptance, finding disposition,
candidate selection, and release. A future checker PASS is not owner approval.

## Acceptance criteria

- Profile identity is spelled exactly `magpie-public-prealpha-assurance-v1`.
- The selected boundary and exclusions are explicit and falsifiable.
- `ubuntu-24.04` is treated as a selected runner label, with actual image
  identity retained separately; no unsupported cross-platform claim is made.
- Governed input identities and observed environment coordinates are distinct.
- Required lanes identify their property, front door, and acceptable evidence.
- SQLite, compatibility, projection, verifier, and crash/resource boundaries are
  separated rather than generalized from one another.
- Dependency pinning, vendoring, observation, security review, and native
  identity are not conflated.
- Repetition, deterministic replay, pinned inputs, measured environment,
  offline execution, hermeticity, and bit-for-bit artifacts use distinct terms.
- The evidence record, maker/checker independence, claim fence, completion gate,
  gap matrix, and bounded follow-ups are present.
- No audit finding status, release version/tag, CI, dependency, runtime, FORMAT,
  ADR, or frozen corpus bytes change.
