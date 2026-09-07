# Epistemic Gate Skeleton Spike

## Status

Experimental spike note. Docs-only companion to the crate-private
`magpie_claims` module `epistemic_gate` (A-022 capability-skeleton spike,
branch `spike/epistemic-gate-skeleton-v0`). NOT ratified doctrine. NOT A-022
closure. NOT an admission implementation. Safe to discard: it records what
the skeleton demonstrates and what it deliberately leaves open. It introduces
no mechanism, no API, no schema, no event format, and no write path.

## What the spike demonstrates

The smallest honest capability shell for the future EpistemicGate boundary:

- a crate-private `EpistemicGate<S>` in `magpie-claims` that custodies a
  `magpie_log::LogWriter<S>` by value;
- construction consumes the writer and appends nothing;
- no public surface: no public module, type, or constructor; no accessor,
  conversion, `Deref`, `Clone`, `serde` exposure, or signing-key accessor;
- no `ClaimStatusChanged` and no caller-selected `Status`, standing, or
  policy;
- no `admit`, `submit`, `write`, or `append` operation of any kind;
- no raw `SignedEvent`, raw-bytes, or arbitrary `Payload` passthrough.

The point is custody, not behavior: the type that will one day stand between
external material and the chain's sole append capability already exists as a
shape, and the shape has no ordinary write surface.

## What the spike deliberately does not decide

- **Admission semantics.** What an admitted contribution is, what evidence it
  requires, and what it may or may not change. Unresolved; governed by the
  admission-boundary contract set, not by this spike.
- **Persistence outcome model.** `Err` from a write is not the same as
  "rejected", "not historical", or "safe to retry". The supported SQLite L0
  path distinguishes a successful COMMIT from an error whose persistence
  outcome is unknown. This spike invents no public state for those
  distinctions and implements no retry or idempotency; any future gate
  operation must preserve that distinction.
- **How future gate operations drive `append`.** `magpie_log` exposes
  `append` per concrete backend (`FileStore`, `MemStore`), not generically
  over `S`. The skeleton is generic over `S` and therefore cannot call
  `append` today. The future boundary is either written per backend or
  requires a governed change to `LogWriter`. That is a later architectural
  decision, not a gap in this spike.

## Topology

```text
external material
        |
        v
EpistemicGate<S>            (crate-private; custodies LogWriter<S>)
        |
        v
LogWriter<S>::append        (sole append capability; backend-specific today)
        |
        v
LogStore                    (FileStore / MemStore / supported SQLite L0)
```

## Protected surfaces

This spike changes none of: FORMAT/canonical bytes, payload tags 0-8,
golden/verifier corpus, Deadbolt, standing semantics, origin-admission seams,
the release contract, or audit dispositions.

## Related

- `docs/design/logwriter-sole-write-capability-contract.md`
- `docs/design/epistemic-pipeline-separation-contract.md`
- `docs/design/admission-boundary-minimal-semantics.md`
