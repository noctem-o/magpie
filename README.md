# Magpie

A local-first, provenance-first research mind. **One signed append-only log is the
sole source of truth; one verified capability gate is the only thing that may write
to it; everything else is a derived, regenerable projection.**

The skeleton walks: the bottom of the stack (`magpie-log`, L0, format-frozen and
golden-pinned), two worked projections (`magpie-claims`, `magpie-episodic`), an
independent Python verifier, a ratified live seam to `deadbolt`, and a
fail-closed governed-standing explanation surface. Magpie retains typed claims,
evidence, and justification edges; applies closed support, refutation, and
privileged-context policy; and still exposes no ordinary writer, ambient
promotion mechanism, or implicit latest-policy selector.

## Run it

```sh
cargo test                                   # whole workspace, incl. the thesis test
cargo run --example tour -p magpie-claims    # governed-standing end-to-end tour
```

The tour demonstrates the current vertical thesis: one signed history verifies
completely and co-replays standing with exact anchor context; policy v0 exposes
candidate-only standing, explicit policy v1 settles only the exact matched
Deadbolt occurrence proposition, and explicit policy v2 directly supports one
exact canonical inline digest proposition after the same-snapshot verifier
matches. A structured mismatch and an interpretation claim remain unpromoted,
then every snapshot and resolution byte regenerates identically after all
derived standing state is dropped. It does not verify foreign bundle contents
or implement general aggregation or writer admission.

The thesis, as a test (`crates/magpie-claims/tests/regenerable.rs`):
> build a projection → **drop all derived state** → replay from the log alone →
> assert the result is **byte-for-byte identical**.

### Release boundary

The human-created `v0.1.0` tag exists and resolves to
`2bbfbd1f451e35b65e8c89aeaf39801621e484df`. It names the reviewed v0.1.0
source release, whose format, policy, replay, and architectural milestone is
defined by
[`docs/releases/v0.1.0-contract.md`](docs/releases/v0.1.0-contract.md). The
workspace now has shared, explicitly non-publishable release metadata, and CI
checks all three package inventories. Only `magpie-log` is verified as a
standalone archive; `magpie-claims` and `magpie-episodic` remain path-workspace
members rather than independently registry-resolvable packages. The exact
source release is versioned `0.1.0` and recorded in
[`CHANGELOG.md`](CHANGELOG.md). A GitHub prerelease exists for the tag; it is
not registry publication. No crates.io or other registry publication is
implied or enabled, and all path-workspace and packageability boundaries remain.

## Layout

```
magpie/
  crates/
    magpie-log/        L0 — the signed, hash-chained, append-only event log (the hoard)
      src/event.rs         Event model: Provenance, Status spectrum, Payload, EventCore, SignedEvent
      src/canonical.rs     magpie-core-v1 — the canonical byte encoding (FROZEN; see docs/FORMAT.md)
      src/hashing.rs       SHA-256 ContentHash (the chain link)
      src/store.rs         LogStore trait + FileStore (durable) + MemStore (tests/embedding)
      src/logimpl.rs       LogWriter (the write capability) / LogReader (read-only) / Projection / replay
      tests/chain.rs       chain integrity, tip recovery, tamper detection, genesis rules
      tests/golden.rs      golden vectors: pinned hashes, signatures, canonical preimages
      testdata/            golden-v1.jsonl — the committed fixture chain (nine events, incl. ADR-0002 tags)
      examples/regen_golden.rs   regenerates the fixture for reviewed format-surface changes
    magpie-claims/     projection #1: the epistemic claim store, folded from the log
      src/lib.rs           ClaimsView compatibility projection + public read surfaces
      src/policy.rs        closed evidence/domain ceilings and support-context policy
      src/standing.rs      StandingView + fail-closed StandingResolution v0
      src/replay_snapshot.rs   co-replayed standing + anchor context, standing-inert
      tests/regenerable.rs the thesis test
      tests/standing_resolution.rs   adversarial governed-resolution coverage
      examples/tour.rs     governed v0/v1 standing → hostile controls → byte-identical replay
    magpie-episodic/   projection #2: SQLite + FTS5 full-text search over events
      src/lib.rs           EpisodicView : Projection (the only crate that may depend on SQLite)
      tests/episodic.rs    incl. rebuilt-from-zero == incrementally-built
  docs/
    FORMAT.md          the normative format spec — a reimplementation from this page
                       alone must reproduce the golden vectors
    portable-base.md   the machine-agnostic base boundary: Magpie runs without Deadbolt
    seams/deadbolt-anchor-contract.md   the optional Deadbolt anchor protocol contract
    adr/0001-deadbolt-seam.md   the anchored-hierarchy decision (see seam section below)
    adr/0002-governed-claim-memory.md   how memory events earn standing
    design/standing-view-evidence-ceilings.md   evidence/domain ceiling doctrine
    design/standing-aggregation-independence-groups.md   future aggregation constraints
  tools/
    verify_chain.py    independent verifier, written from FORMAT.md alone; CI runs it
                       against the golden fixture with a pinned trust root
```

## The two invariants, enforced by types (not by convention)

1. **Append-only.** There is no `update` or `delete`. A correction is a new event that
   supersedes — which is exactly how the epistemic spectrum records a claim moving
   `Supported → Refuted`.
2. **The gate is the only writer.** Only `LogWriter` has `.append()`. Holding a
   `LogWriter` *is* the write capability. Memory layers are handed a `LogReader`, which
   exposes no way to write. Projections are pure folds and stay structurally unable
   to write.

## How this meets `deadbolt` (the seam — ratified and live)

Magpie core is portable and can run without Deadbolt. Deadbolt integration is
optional but protocol-defined by `docs/seams/deadbolt-anchor-contract.md`; the
portable base boundary is in `docs/portable-base.md`. The always-on
librarian/navigator is deliberately future work and is not part of the base
trust architecture.

ADR-0001 (**anchored hierarchy**, ratified 2026-07-02): deadbolt keeps its internal
witness/evidence machinery unchanged; after each successful **seal-and-verify**, the
kernel appends one `SegmentAnchored` event (payload tag 5) to this chain, carrying
the bundle kind, the witness root (verbatim), the hash algorithm, the **foreign**
canonicalization profile that produced the root, and the run id.

- **A segment that is not anchored is not part of the record.** The Magpie chain is
  the record of what execution history exists, and in what order.
- **Two-step verification, one root of trust.** The chain verifies order, signature,
  and inclusion; a bundle's contents verify locally against its anchored root using
  deadbolt's own verifier for that profile.
- **Kind-agnostic by construction.** New deadbolt bundle kinds are new `bundle_kind`
  strings — never new Magpie payload variants.

Status of the deadbolt side (in the deadbolt repo): `cog-anchor` is the only crate
there that may depend on `magpie-log`, and its `Anchorer::open` is the sole
`LogWriter` construction site in that estate. Identity creation is explicit and
fail-closed (`cog anchor init`, refuses to overwrite); emission is fail-open —
anchoring failure writes a `pending-anchor.json` marker beside the bundle and never
fails a seal. All four production seal points emit anchors today — kernel witness,
observation, codelet, and desktop-transaction dry-run: every sealed bundle either
has an anchor in the log or a `pending-anchor.json` beside it. `cog anchor
reconcile` (deadbolt ticket 0009, merged) pays the recorded debt idempotently,
re-verifying every bundle with its own kind's verifier before anchoring — reconcile
is deliberately the first consumer of anchors, folding an `AnchorSet` projection
over a chain-verified replay of this log.

## Governed claim memory (ADR-0002 — additive vocabulary, no new authority)

ADR-0002 answers the question the anchor seam intentionally does not answer:
Deadbolt can prove or refuse occurrence, but what may Magpie believe about that
evidence?

The rule is deliberately conservative: a memory event earns standing through a
governed write path and deterministic replay into `StandingView`. Standing is not
stored as mutable truth, and `SegmentAnchored` remains occurrence/inclusion
evidence, not interpretation truth.

The log now knows three additive payload tags:

- tag 6, `ClaimAssertedV2`: registers a scoped claim node;
- tag 7, `EvidenceRegistered`: registers typed evidence under a deterministic
  ceiling;
- tag 8, `JustificationEdgeRecorded`: records a closed-vocabulary support,
  contradiction, lineage, invalidation, supersession, or ratification edge.

These are format vocabulary, not a permission system. `StandingView` retains
the typed replay tables. `StandingResolution v0` is the canonical governed read
surface: it parses the target claim's closed `claim_domain` fail-closed,
quarantines legacy raw status, and explains candidate support ceilings under the
fixed `magpie-claims-standing-v0` policy.

Candidate is not policy-eligible, admitted, aggregated, or achieved standing.
The closed policy currently provides:

- `support_ceiling(EvidenceKind, ClaimDomain)`;
- `refutation_ceiling(EvidenceKind, ClaimDomain)`;
- `support_context_requirement(EvidenceKind, ClaimDomain)`.

Human ratification still requires future replayable admission. Policy v0
remains candidate-only and still returns `Conjectured`. Explicit snapshot-only
v1 settles only the exact structured proposition that a five-field Deadbolt
occurrence identity appears in the same verified replay. Explicit snapshot-only
v2 directly supports one exact canonical `sha256_bytes_equals_v0` proposition
after same-snapshot deterministic verification. That direct contribution is
`Supported`, not `Settled`, and is not aggregation. Aggregation, refutation
application, `EpistemicGate`, and all writer-facing surfaces remain future work.

## Development workflow

Magpie uses a maker/checker loop for ordinary agent work:

- Claude decides the bounded change and writes a ticket under `.agent-runs\pending\`;
- Codex implements that ticket through `scripts\codex-delegate.ps1`, the delegation boundary;
- Claude reviews the resulting diff and tests;
- a human holds sole merge authority.

The wrapper is not edited during ordinary work, and Codex output is never committed or
merged automatically.

On native Windows, validation commands are normally run by the human reviewer if Codex
shell execution is unavailable.

## Honest notes

- **Canonical encoding is `magpie-core-v1`** — a fixed binary codec (big-endian
  integers, length-prefixed UTF-8, tagged enums, magic-prefixed), spec'd in
  `docs/FORMAT.md` and pinned by golden vectors in CI. The format is **frozen**:
  tags 5-8 were added under §3's additive-evolution rule, with older golden hashes
  and signatures unchanged. The store stays JSON-lines; stored bytes are never the
  hash preimage. Every chain begins with a **genesis event** (seq 0) declaring the
  profile and the verifying key, written automatically when a writer opens an
  empty store. Signatures are made over `magpie-sig-v1 || hash` — domain-separated,
  so this key's log signatures can't be confused with anything else it signs.
- **`MemStore` is single-threaded** (`Rc`/`RefCell`). `FileStore` is the durable one; swap
  to `Arc`/`Mutex` only after a design conversation — single-threaded is currently a choice.
- **Keys are dev-grade everywhere.** `magpie-log` takes a `SigningKey` from the
  caller; deadbolt's anchor identity is a bare hex seed file created by
  `cog anchor init`. Real custody (keyring/HSM, rotation) is deliberately deferred
  and honestly labelled wherever it appears.
- **No knowledge graph.** The wiki, when it exists, is a downstream projection.

## Next steps (from the architecture ledger; kept honest as things land)

1. ~~Canonical codec~~ — **done**: `magpie-core-v1`, genesis, signature domain
   separation, golden vectors, `docs/FORMAT.md`, independent Python verifier in CI.
2. ~~Episodic projection~~ — **done**: `magpie-episodic`, SQLite + FTS5, pure fold.
3. ~~Deadbolt seam~~ — **ratified and live**: ADR-0001, `SegmentAnchored` tag 5,
   deadbolt-side `Anchorer` emitting at the kernel witness seal point.
4. ~~Reconcile pass~~ — **done** (deadbolt PR #320): verify-then-anchor,
   idempotent, first consumer of anchors. The seam is complete end to end.
5. ~~ADR-0002 groundwork~~ — **landed**: governed claim memory ADR, `StandingView`
   v0 skeleton, boundary tests, and additive tags 6-8 with verifier/golden coverage.
6. ~~Standing replay structure~~ — **landed**: typed claim, evidence, and
   justification edge tables make ADR-0002 graph material regenerable.
7. ~~Fail-closed standing policy foundation~~ — **landed**: closed support and
   refutation ceilings, support-context requirements, and deterministic
   `StandingResolution v0` traces without promotion.
8. ~~Exact Deadbolt context and replay prerequisites~~ — **landed**: strict
   structured occurrence matching and single-snapshot verified replay.
9. ~~First achieved-standing slice~~ — **done**: co-replayed standing and anchor
   context now feed one explicit `magpie-claims-standing-v1` direct-proof rule
   for exact Deadbolt occurrence/inclusion. The v0 resolver remains unchanged.
   The governed-standing public tour now executes this boundary in CI.
   Aggregation, contradiction debt, invalidation, and supersession remain
   separate later phases.
10. **v0.1.0 source release tagged** — the semantic contract,
    non-publishable metadata, package inventories, changelog, and coordinated
    workspace version are complete. `magpie-log` alone is standalone-verified;
    dependent crates remain path-workspace members. The human-created `v0.1.0`
    tag names reviewed source commit `2bbfbd1f451e35b65e8c89aeaf39801621e484df`;
    this does not imply registry publication.
11. **Replayable deterministic-verifier admission contract — landed.** The
    contract fixes one exact predicate, same-snapshot trusted construction,
    receipt boundary, and future policy-v2 admission rule.
12. **Standing-inert deterministic verifier context — landed.** Strict schemas,
    canonical statement/content-hash binding, bounded witness checking, and
    snapshot-only receipt traces now derive deterministic audit material with
    no standing effect.
13. **Explicit policy-v2 deterministic direct support and composition
    hardening — landed.**
    `magpie-claims-standing-v2` preserves the exact Deadbolt v1 settlement rule
    and adds one `sha256_bytes_equals_v0` direct contribution that reaches
    `Supported` only after the existing same-snapshot verifier returns
    `Matched`. Repeated successful paths do not amplify or settle. PR #48
    also makes inherited v0/v1 composition mismatches fail closed without
    changing the frozen policy surfaces.
14. **Artifact provenance and origin-admission architecture — landed.** Ticket
    0037 and `docs/design/artifact-provenance-origin-admission.md` define the
    replayable authority substrate and preserve the explicit `(H, P, M)`
    resolution boundary.
15. **Exact acquisition/direct-derivation protocol — landed.** Ticket 0038 and
    `docs/design/artifact-acquisition-derivation-bundle-v0.md` ratify the two
    sibling bundle/schema identities, canonical JSON bytes, SHA-256 roots,
    full anchor matching, and standing-inert verifier outcomes.
16. **Portable artifact-provenance verification — implemented by Ticket
    0039.** Exact acquisition and direct-derivation fixtures now exercise a
    duplicate-aware fixed-schema parser, purpose-built canonical encoder,
    selector-bound same-snapshot replay verification, and closed audit traces.
    Exact matches remain audit context only: they establish neither source
    identity, transformation correctness, truth nor standing. Production
    `ResolutionContentClosureV0` construction is deferred; origin binding,
    origin admission, admitted-contribution audit, aggregation, policy v3,
    refutation, contradiction debt, invalidation, supersession,
    `EpistemicGate`, and writers remain later work.

Conserve the log. Derive the rest.
