# Ticket 0030: Governed-standing end-to-end tour v1

## Goal

Replace the public compatibility-status example with one executable
governed-standing release contract, without changing policy or authority.

Architectural law:

```text
One deterministic signed history demonstrates that writing a claim does
not make it believed: v0 exposes bounded candidate material, v1 settles
only one exact replay-proven occurrence proposition, hostile and
interpretive paths remain unpromoted, and complete replay reproduces
byte-identical derived results.
```

## Implementation

Rewrite `crates/magpie-claims/examples/tour.rs` in place using only existing
public APIs. Preserve:

```text
cargo run --example tour -p magpie-claims
```

The deterministic 11-event signed history contains genesis, three typed claims,
three typed evidence nodes, three exact-scope support edges, and one five-field
Deadbolt anchor.

The paths are:

- exact `Occurrence/Inclusion`: v0 `Conjectured`, v1 `Matched` through
  `DeadboltOccurrenceInclusionV1`, achieved and governed `Settled`;
- one-field `run_id` mismatch: valid v0 candidate, v1
  `PredicateReferenceMismatch`, no contribution, `Conjectured`;
- `Interpretation` over the same anchor: actual v0 `Supported` ceiling,
  governed `Conjectured`, no v1 occurrence-lane application.

The tour must not use `ClaimStatusChanged` or legacy evidence to manufacture the
demonstrated result.

## Replay contract

Verify the complete chain, then build one composite snapshot only through
`replay_standing_context`. Capture canonical bytes for the snapshot and all six
v0/v1 resolutions plus typed semantic and audit facts. Let the snapshot and
full resolutions leave scope, replay again from signed history alone, and assert:

- typed facts are structurally equal;
- matched identity, sequence, event hash, and provenance are identical;
- snapshot canonical bytes are identical;
- every v0 and v1 resolution's canonical bytes are identical.

Do not substitute one hand-built transcript serialization for the existing
component byte surfaces.

## Public contract

Update README wording only around the command and example description. Add a
separate CI step after Rust tests:

```text
cargo run --locked --example tour -p magpie-claims
```

The executable assertions, rather than a pinned stdout transcript, are the
release contract.

## Trusted-construction claim

### What this construction proves

The example's `StandingView` and `DeadboltAnchorIndex` were co-derived from one
completely verified event vector, and the explicit policy-v1 resolver settled
only the exact occurrence proposition whose typed claim predicate, typed
evidence reference, and replayed anchor identity matched.

### What it does not prove

- that the example signing key is production-secure;
- that the caller selected a socially correct real-world trust root;
- that foreign Deadbolt bundle contents were verified by this example;
- that the anchored action was safe, wise, desirable, or successful;
- that an interpretation of the occurrence is true;
- that Magpie is a general truth oracle;
- that support aggregation, admission, refutation, contradiction debt,
  invalidation, or supersession exists;
- that ordinary writer or MCP surfaces exist;
- that the deterministic example key should be reused outside tests and
  demonstrations.

### Sole legitimate origin path

One successful `LogReader::replay` through `replay_standing_context`, followed
by the explicit snapshot-only v0 and v1 resolvers.

### Manual or adversarial bypasses

The claim excludes manually constructing public trace or resolution structs;
independently constructing standing and anchor projections; using two separate
replay calls as though they prove shared provenance; reading unverified events;
trusting actor-class or evidence-kind labels; trusting prose, content hashes, or
`verified` booleans; treating the anchor as foreign-bundle verification; and
treating the positive occurrence result as interpretation truth.

### Hostile regression

The example and CI must fail if v0 promotes the positive fixture beyond
`Conjectured`; v1 settles the mismatched path; v1 settles the interpretation
path through the occurrence lane; the positive path no longer settles through
exact matched context; replay ceases to regenerate byte-identical snapshots or
resolutions; the example returns to manual legacy status mutation; or the
primary README command stops executing the governed-standing thesis.

## Files in scope

- `crates/magpie-claims/examples/tour.rs`
- `.github/workflows/ci.yml`
- `README.md`
- `docs/design/governed-standing-tour-v1.md`
- `tickets/0030-governed-standing-tour-v1.md`

## Non-goals

No public API, policy, standing result, canonical byte, serialized type,
dependency, fixture, verifier, L0 format, CLI, aggregation, admission,
refutation, writer surface, or foreign Deadbolt verification change.

## Validation

Run the complete Rust suite, strict workspace Clippy, the tour twice with a
deterministic-output comparison, both independent Python verifiers, frozen-path
and policy-source audits, and final diff review.
