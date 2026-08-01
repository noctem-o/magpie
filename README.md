# Magpie

**A local-first, replayable epistemic memory kernel for AI systems.**

Magpie records what happened, what was claimed, what evidence exists, and what
an explicit policy is allowed to conclude.

It does **not** treat retrieval, repetition, model confidence, or serialized
audit output as authority.

> **Conserve the log. Derive the rest.**

Here, *epistemic* means keeping the basis and limits of a conclusion visible.
*Kernel* means a small set of typed mechanisms, not a finished end-user
application.

Magpie is a functioning experimental Rust workspace. Current development
includes explicit standing policies v0 through v4. The latest formal
owner-created source release remains v0.1.0; this README does not announce a
v0.2.0 release.

## Why Magpie exists

Memory for an AI system is not just a retrieval problem. A stored item may be
available without being verified, verified without having useful provenance,
or well-provenanced without being admitted as an independent contribution.
Even admitted evidence may be only a candidate for a policy rule, not achieved
standing.

Magpie keeps these concepts separate:

- **Observation** records what an event said.
- **Availability** says that exact bytes were supplied to a resolution.
- **Verification** checks those bytes or a signed history against a named
  procedure and trust root.
- **Provenance** connects an artifact to recorded acquisition or derivation.
- **Origin admission** decides whether a contribution has an admitted origin
  in an exact comparison namespace.
- **Candidate eligibility** decides whether evidence may enter a policy rule.
- **Governed standing** is the conclusion an explicitly selected policy
  actually achieves.

The signed append-only log is historical authority relative to an externally
supplied verifying key. Search indexes, claim graphs, standing views, and audit
traces are derived. They can explain history and policy, but they cannot write
authority back into the log.

## Current milestone

Current main implements:

- a frozen signed event format with canonical encoding and golden vectors;
- an append-only, hash-chained log with an independent Python verifier;
- complete verification before deterministic replay;
- byte-identical regeneration of derived state;
- typed claims, evidence, and justification edges;
- an optional live protocol seam for anchoring sealed Deadbolt execution
  evidence;
- immutable resolution-content closure: an exact set of supplied artifact and
  foreign-bundle bytes;
- artifact-provenance verification and origin-admission audit;
- admitted-contribution and support-contribution audits;
- governed standing policies v0 through v4; and
- deterministic, one-way audit traces that cannot be reinjected as authority.

This is a working experimental kernel, not a complete memory product.

## Quick start

From the repository root, run the full locked workspace tests:

```sh
cargo test --workspace --locked
```

Run the deterministic governed-standing tour:

```sh
cargo run --locked --example tour -p magpie-claims
```

The frozen tour writes and verifies one signed history, resolves policies
v0-v2, drops the derived state, replays the history, and checks byte-identical
regeneration. Policies v3 and v4 are implemented and tested separately; they
are not part of the frozen tour output.

Verify the golden chain with the independent Python implementation:

```sh
python tools/verify_chain.py \
  crates/magpie-log/testdata/golden-v1.jsonl \
  ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c
```

The verifier independently checks canonical encoding, sequence and previous
hash links, stored hashes, signatures, genesis binding, and payload validity
against the supplied verifying key.

## How it works

```text
signed append-only history
        |
        v
verified deterministic replay
        |
        +-- episodic/search projection
        +-- typed claims, evidence, and edges
        +-- anchors and immutable closure context
        +-- provenance and origin-admission audits
        `-- explicit standing policies
                    |
                    v
             governed standing
```

Only signed events are conserved as historical authority. Replay first reads
one record snapshot, verifies that complete snapshot, and then folds those same
events into projections. Standing paths that need artifacts or foreign bundles
also receive an immutable closure of exact bytes; resolution performs no
ambient filesystem, content-addressed store, or network lookup.

The policy path is deliberately staged:

1. Exact bytes must be available in the supplied closure.
2. Named verifiers establish only their exact predicates.
3. Provenance binds content to recorded acquisition or derivation.
4. Origin admission evaluates attributed origin under explicit policy.
5. Contribution audits retain complete, typed eligibility decisions.
6. An explicitly selected standing policy may produce governed standing.

Verification is not provenance. Provenance is not origin separation. Origin
separation is not statistical independence. Eligibility is not achieved
standing.

## Governed standing policies

Standing is a policy result such as `Conjectured`, `Supported`, `Settled`, or
`Refuted`. Raw serialized status is retained for historical audit, but governed
standing comes only from a named resolver.

| Policy | Authority added | Result and boundary |
| --- | --- | --- |
| v0 | Candidate explanation, ceilings, and blockers | Explains eligible-looking evidence without promoting it. |
| v1 | Exact same-replay Deadbolt occurrence/inclusion rule | A matching five-field anchor occurrence may achieve `Settled` for that exact proposition. |
| v2 | One exact deterministic direct-support rule | A matched `sha256_bytes_equals_v0` candidate may achieve `Supported`, never `Settled`. |
| v3 | One exact `ExternalSource` x `ExternalReport` corroboration rule | At least two distinct admitted origin groups in one exact comparison namespace may achieve `Supported`, never `Settled`; this is not a statistical-independence claim. |
| v4 | One exact subject-bound deterministic direct-refutation rule | The eligible negative lane may achieve governed `Refuted` under conservative inherited-standing precedence. |

These are explicit versioned surfaces, not an ambient sequence in which every
new version silently replaces the last. There is no implicit
`resolved_standing_latest` selector.

### Why v4 is subject-bound

An earlier proposed v4 path, recorded in Ticket 0056, exposed an
evidence-relative substitution failure. Evidence could supply unrelated
`witness_hex` bytes; inequality against the expected digest would then show
only that those evidence-selected bytes differed, not that the claim's subject
was false. That path was not ratified.

The implemented v4 path closes the substitution:

- the claim owns both the exact subject bytes and expected digest;
- evidence supplies routing and binding, never alternate subject bytes;
- one claim-owned digest relation is evaluated per policy call;
- each candidate evidence and `contradicts` edge binds separately; and
- parse or binding failure is a resolution failure, never digest inequality or
  falsity.

Only the exact `DigestUnequal` + `contradicts` + `RefutationEligible` lane with
the compiled `Refuted` ceiling can produce governed `Refuted`. Inherited
`None`, `Open`, or `Conjectured` may promote. Inherited `Supported` and
`Settled` are preserved behind an explicit contradiction-policy blocker.
Inherited governed `Refuted` is preserved without another application.
Legacy raw `Refuted` remains audit-only and cannot authorize the result.

`SupportEligible` remains standing-inert in v4. The rule is direct and
non-amplifying: several eligible candidates remain visible in the trace but
produce at most one policy application.

## Core guarantees

### Append-only historical authority

There is no update or delete operation on the log. A correction must be a
later signed event, so the prior record remains inspectable. The log itself
does not decide that a later event invalidates, supersedes, or becomes the
current interpretation of an earlier one.

The genesis event makes the chain self-describing, but trust in the supplied
verifying key remains external.

### Capability-based low-level writing

`LogWriter` holds the signing key and is the low-level append capability.
`LogReader` and projection types expose no append method. The optional Deadbolt
anchor path has a reviewed writer boundary, but Magpie does not yet provide an
ordinary governed claim/evidence writer or `EpistemicGate`.

### Regenerable, same-replay projections

Authoritative resolution uses one completely verified record snapshot.
Derived claim, anchor, and episodic state can be dropped and rebuilt from that
history. Same-replay composition prevents a result from mixing observations
from different log prefixes.

### Audit is not authority

Standing and contribution traces are deterministic, serializable explanations.
Authority-bearing audit types have no public deserialization or caller-supplied
resolver path. Serialized audit output is one-way: it can be inspected, stored
elsewhere, or compared, but not fed back in as authority.

### Failure is not falsity

Missing bytes, malformed metadata, an unknown kind, a failed signature, a
binding mismatch, or incomplete audit state cannot manufacture a negative
fact. Failures remain typed failures.

### Repetition does not amplify without policy

Repeated anchors, evidence, or audit outcomes do not become stronger merely
through multiplicity. Policy v3 counts distinct admitted origin groups and
counts same-origin multiplicity once. Policy v4 emits at most one direct
application regardless of how many eligible candidates are retained.

### Policy selection is explicit

Each standing policy has a stable identifier and resolver. Callers choose the
version they intend to apply; Magpie does not silently select a latest policy.

## Workspace

### `magpie-log`

The signed append-only history:

- frozen canonical encoding and event vocabulary;
- hash chaining and Ed25519 signatures;
- `LogWriter` and read-only `LogReader`;
- durable `FileStore` and in-memory `MemStore`; and
- golden vectors, complete verification, and deterministic replay.

### `magpie-claims`

The epistemic policy projection:

- typed claim, evidence, and justification-edge graph;
- immutable content closure plus provenance and origin-audit substrate;
- governed standing policies v0-v4; and
- deterministic standing traces and canonical one-way audit output.

### `magpie-episodic`

A rebuildable SQLite event projection with FTS5 full-text search. It provides
deterministic log-order search over replayed events. It is useful as a search
surface, but it is not a second source of authority and has no write path back
to the log.

## Deadbolt integration

Magpie remains portable without Deadbolt. The optional integration is defined
by protocol rather than a Rust crate dependency.

`SegmentAnchored` records the exact identity of a sealed foreign bundle:
`bundle_kind`, `witness_root`, `witness_algorithm`,
`canonicalization_profile`, and `run_id`. Magpie verifies the signed event's
chain order, signature, and inclusion. Verification of the foreign bundle's
contents remains the responsibility of the appropriate foreign verifier.

New foreign bundle kinds remain values of the `bundle_kind` string. They do
not require new Magpie payload variants.

## What Magpie is not

Magpie is not:

- a chatbot or autonomous research agent;
- a vector database or mutable knowledge graph;
- a general-purpose truth engine;
- a statistical-independence oracle;
- an automatic contradiction resolver;
- a crawler or public ingestion service;
- a production key-management system; or
- a system with ambient latest-policy selection.

## Current boundaries

### Implemented

- signed historical authority and deterministic replay;
- rebuildable search and typed evidence projections;
- immutable closure, provenance verification, and origin admission;
- direct positive standing and narrow external-report corroboration;
- subject-bound deterministic direct refutation; and
- deterministic explanations with one-way audit output.

### Future

- an ordinary governed claim/evidence writer and `EpistemicGate`;
- an acquisition loader and content-addressed store;
- filesystem or network ingestion;
- contradiction debt and broader conflict policy;
- invalidation, supersession, and currentness semantics;
- broader source-standing propagation;
- a read-only librarian or research navigator; and
- production key custody and rotation.

The contradiction lifecycle is a natural next architectural arc, not an
implemented behavior or an approved implementation ticket. Future work may
change these boundaries only through explicit contracts and review.

## Documentation map

- [Signed event format](docs/FORMAT.md) - canonical bytes, payload tags, and
  frozen vectors.
- [Portable base](docs/portable-base.md) - the dependency-minimal core and
  optional seams.
- [Deadbolt anchor contract](docs/seams/deadbolt-anchor-contract.md) - the
  protocol and writer boundary.
- [MCP boundary contract](docs/design/mcp-boundary-contract.md) - the agent
  and tool interface boundary: reads, proposals, and no authority.
- [MCP librarian contract](docs/design/mcp-librarian-contract.md) - the
  read-only query surface: verified state, traces, and provenance.
- [Verified librarian query contract](docs/design/verified-librarian-query-contract.md) -
  what queries and query results may and may not be.
- [Provenance response contract](docs/design/provenance-response-contract.md) -
  what must accompany a Magpie-derived response.
- [MCP capability boundary contract](docs/design/mcp-capability-boundary-contract.md) -
  what capability discovery may and may not mean.
- [AgentProposer boundary contract](docs/design/agent-proposer-boundary-contract.md) -
  the semantic weight of agent proposals before governed admission.
- [Admission boundary questions](docs/design/admission-boundary-questions.md) -
  unresolved questions and invariants for a future governed admission
  boundary (exploration, not doctrine).
- [ADR 0001](docs/adr/0001-deadbolt-seam.md) - why foreign execution evidence
  is anchored through a minimal payload.
- [ADR 0002](docs/adr/0002-governed-claim-memory.md) - typed governed claim
  memory and raw-status quarantine.
- [Standing ceilings](docs/design/standing-view-evidence-ceilings.md) -
  evidence domains, candidate ceilings, and future boundaries.
- [Provenance and origin admission](docs/design/artifact-provenance-origin-admission.md) -
  the staged authority model.
- [Policy v3](docs/design/standing-policy-v3-external-report-corroboration-v0.md) -
  the exact external-report corroboration rule.
- [Policy v4](docs/design/standing-policy-v4-claim-inline-direct-refutation-v0.md) -
  subject-bound direct refutation and conservative precedence.
- [v0.1.0 release contract](docs/releases/v0.1.0-contract.md) - the frozen
  formal source-release boundary.
- [Tickets](tickets/) - detailed implementation records and review history.

## Development

The principal local validation surface is:

```sh
cargo fmt --all --check
cargo test --workspace --locked
cargo test --doc --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
python tools/check_release_metadata.py
```

The workspace also carries focused hostile and integration tests for
provenance, origin admission, policies v3 and v4, raw-status quarantine,
same-replay binding, failure ordering, non-amplification, and canonical audit
vectors.

Changes should remain small, typed, replayable, and explicit about authority.
See [AGENTS.md](AGENTS.md) for repository working rules.

## Release status

v0.1.0 is the latest formal owner-created source release. Current main has
advanced beyond that milestone into a prospective v0.2-shaped development
state. This README does not imply a v0.2.0 tag or release, registry
publication, or package availability.
