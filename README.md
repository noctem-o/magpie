# Magpie

A local-first, provenance-first research mind. **One signed append-only log is the
sole source of truth; one verified capability gate is the only thing that may write
to it; everything else is a derived, regenerable projection.**

The skeleton walks: the bottom of the stack (`magpie-log`, L0, format-frozen and
golden-pinned), two worked projections (`magpie-claims`, `magpie-episodic`), an
independent Python verifier, a ratified live seam to `deadbolt`, and a
fail-closed governed-standing explanation surface whose explicit policies now
include v1 direct occurrence settlement, v2 deterministic direct support, and
one narrow v3 external-report corroboration rule. Magpie retains typed claims,
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
matches. The newer explicit policy v3 corroboration rule (ledger entry 25) is
not part of this tour, and the tour's output is unchanged by it. A structured
mismatch and an interpretation claim remain unpromoted, then every snapshot and
resolution byte regenerates identically after all derived standing state is
dropped. It does not verify foreign bundle contents or implement general
aggregation or writer admission.

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
    design/standing-aggregation-independence-groups.md   aggregation doctrine: first v3 rule implemented, broader constraints future
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
`Supported`, not `Settled`, and is not aggregation. Explicit policy v3
(ratified by Ticket 0053, implemented by Ticket 0054, merged via PR #68)
resolves through the same verified replay context and an immutable closure: a
complete internally derived support audit with at least two distinct admitted
origin groups in one exact `OriginComparisonNamespaceV0` lane contributes
governed `Supported`, never `Settled`, to the exact requested `ExternalReport`
claim. That corroboration rule is the only implemented aggregation rule.
Broader aggregation, refutation application, contradiction debt, invalidation,
supersession/currentness, `EpistemicGate`, and ordinary claim-bearing writer
surfaces remain future work. The low-level `LogWriter` append capability
already exists.

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
   Aggregation beyond the first narrow policy-v3 rule (entry 25),
   contradiction debt, invalidation, and supersession remain separate later
   phases.
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
    `ResolutionContentClosureV0` construction is implemented separately by
    Ticket 0041. The public explicit-slice APIs remain unchanged; Ticket 0045
    adds only crate-private closure-aware reuse for origin-binding composition.
    Origin admission is implemented separately by Ticket 0048 and the
    admitted-contribution audit by Ticket 0050. The first narrow standing
    policy v3 aggregation rule is implemented later by Ticket 0054 (entry
    25); broader aggregation, refutation, contradiction debt, invalidation,
    supersession, `EpistemicGate`, and ordinary claim-bearing writer
    surfaces remain later.
17. **Resolution content closure v0 contract — landed.**
    [`docs/design/resolution-content-closure-v0.md`](docs/design/resolution-content-closure-v0.md)
    and [Ticket 0040](tickets/0040-resolution-content-closure-v0.md) now specify
    the exact finite keyed availability universe, canonical manifest and
    closure identity used as input `M` for deterministic resolution. The
    public artifact-provenance verifier methods still receive explicit optional
    byte slices. Origin admission is implemented separately by Ticket 0048 and
    admitted-contribution audit by Ticket 0050. The first narrow standing
    policy v3 aggregation rule is implemented later by Ticket 0054 (entry
    25); conservative aggregation beyond that rule and ordinary
    claim-bearing writer surfaces remain future work.
18. **Resolution content closure v0 implementation — landed.** Exact bounded
    construction now collapses identical duplicates, rejects deterministic
    conflicts, owns immutable keyed bytes, and exposes exact read-only lookup.
    Its purpose-built canonical manifest and four-field typed identity
    reproduce the pinned vectors. No loader exists. Public artifact-provenance
    verification remains explicit-slice based; Ticket 0045 reuses the same
    reviewed pipeline through a private exact-closure composition seam.
19. **Origin-binding verifier v0 — implemented and standing-inert.**
    [`docs/design/origin-binding-bundle-v0.md`](docs/design/origin-binding-bundle-v0.md)
    and [Ticket 0045](tickets/0045-standing-inert-origin-binding-verifier-v0.md)
    now have strict two-family parsing, exact immutable-closure lookup,
    canonical root and same-replay anchor verification, exact claim/evidence/
    edge contribution revalidation, nested artifact-provenance composition,
    artifact coherence, and private-construction audit receipts. Matched
    receipts verify claimed governance content only. The compiled authority
    classification and standing-inert origin-admission fold are implemented
    separately by Ticket 0048, and the admitted-contribution audit is
    implemented by Ticket 0050. The first narrow standing policy v3
    aggregation rule — one exact lane, same-origin collapse, and a
    distinct-group threshold — is implemented later by Ticket 0054 (entry
    25); conservative aggregation beyond that rule, the ordinary
    claim-bearing writer, `EpistemicGate`, and their standing effects remain
    future work.
20. **Origin-admission audit v0 — implemented by Ticket 0048.**
    [`docs/design/origin-admission-audit-v0.md`](docs/design/origin-admission-audit-v0.md)
    and [Ticket 0046](tickets/0046-origin-admission-audit-v0-contract.md)
    freeze the contract; [Ticket 0048](tickets/0048-origin-admission-audit-v0-implementation.md)
    implements complete same-replay candidate auditing over exact verified-
    prefix and closure identities. The fixed `magpie-origin-admission-v0`
    policy selects one exact trusted authority pair, globally fails admission
    closed when any candidate binding bytes are unavailable, classifies every
    candidate deterministically, collapses duplicate trusted assignments
    without amplification, detects trusted conflicts, scopes eligible
    unresolved blockers to one exact contribution and namespace, and emits
    deterministic standing-inert audit bytes. It establishes origin-group
    assignment only. It creates no support, aggregation, statistical
    independence or standing. The admitted-contribution audit is implemented
    by Ticket 0050. The first narrow standing policy v3 aggregation rule
    is implemented later by Ticket 0054 (entry 25); it establishes
    corroboration separation, never statistical independence. Conservative
    aggregation beyond that rule, `EpistemicGate`, an ordinary claim-bearing
    writer, loader, CAS, and filesystem or network ingestion remain future
    work.
21. **Origin-admission replay substrate v0 — implemented by Ticket 0047.**
    One-read verified
    replay now returns an exact event-count and chain-tip summary while
    preserving the existing replay API. A separately versioned origin-
    admission replay context co-derives that exact prefix identity with the
    unchanged standing and anchor projections, then provides crate-private,
    complete deterministic enumeration of supported acquisition and derivation
    origin-binding anchor candidates. Exact repeated anchor occurrences remain
    attached without candidate or authority amplification. Ticket 0048 consumes
    this exact same-replay carrier for the complete standing-inert
    `OriginAdmissionAuditV0`; the substrate itself still grants no origin
    admission or standing authority.
22. **Admitted-contribution audit v0 — implemented by Ticket 0050.**
    [`docs/design/admitted-contribution-audit-v0.md`](docs/design/admitted-contribution-audit-v0.md)
    and [Ticket 0049](tickets/0049-admitted-contribution-audit-v0-contract.md)
    freeze the contract implemented by
    [Ticket 0050](tickets/0050-admitted-contribution-audit-v0-implementation.md).
    Runtime now audits every replay-derived justification edge in exact edge-ID
    order, preserves exact first-failure reasons, selects one exact
    `ExternalSource × ExternalReport` lane, binds asserted evidence
    `content_hash` to the exact verified artifact identity, derives
    `OriginAdmissionAuditV0` internally from the same context and closure, and
    fails all downstream admission closed when origin admission is globally
    incomplete. Exact conflict, unresolved, absent and admitted dispositions
    serialize as deterministic standing-inert audit bytes, while distinct
    contributions assigned to one origin group remain distinct. The
    support-contribution contract is ratified separately by Ticket 0051.
    The first narrow standing policy v3 aggregation rule is implemented
    later by Ticket 0054 (entry 25); it establishes corroboration
    separation, never statistical independence. Conservative aggregation
    beyond that rule, `EpistemicGate`, an ordinary claim-bearing writer,
    loader, CAS, and filesystem or network ingestion remain future work. No
    support or standing changed.
23. **Support-contribution audit v0 — contract ratified by Ticket 0051,
    runtime implemented by Ticket 0052.**
    [`docs/design/support-contribution-audit-v0.md`](docs/design/support-contribution-audit-v0.md)
    and [Ticket 0051](tickets/0051-support-contribution-audit-v0-contract.md)
    freeze internal same-context admitted-audit composition, exact upstream
    identity validation, complete candidate/top-level alignment, one exact
    `ExternalSource × ExternalReport` positive lane, exact ceiling and
    support-context drift checks, one-to-one support-input projection,
    complete global failure behavior, deterministic ordering and
    serialization, and preservation of distinct same-origin contributions.
    [Ticket 0052](tickets/0052-support-contribution-audit-v0-implementation.md)
    implements `OriginAdmissionReplayContextV0::resolve_support_contribution_audit_v0`
    through one crate-private composition path with the four crate-private
    origin-binding grammar seams, five canonical fixtures and the reserved
    hostile-test suite. The contract introduces no aggregation or standing
    effect. The first narrow standing policy v3 aggregation rule is
    implemented later by Ticket 0054 (entry 25); it establishes
    corroboration separation, never statistical independence. Conservative
    aggregation beyond that rule, `EpistemicGate`, ordinary claim-bearing
    writer, loader, CAS, and filesystem or network ingestion remain future
    work.
24. **Historical review remediation contract.**
    [`docs/design/historical-review-remediation-ledger.md`](docs/design/historical-review-remediation-ledger.md)
    and [Ticket 0043](tickets/0043-historical-review-remediation-contract.md)
    ratify the remaining doctrine corrections from the PR #1-54 review audit
    and classify the bounded follow-up work. No runtime behavior changes
    there. Two of its dedicated follow-ups have since progressed: PR #56
    landed the legacy raw versus inherited governed `Refuted` regression
    coverage without changing production standing behavior, and PR #57
    removed parsed-event retention for verification-only and writer-recovery
    verification, while `LogStore::read_records` raw-record materialisation
    remains future interface work. The ledger's section 5 records the exact
    status.
25. **Standing policy v3 external-report corroboration v0 — contract
    ratified by Ticket 0053, runtime implemented by Ticket 0054.**
    [`docs/design/standing-policy-v3-external-report-corroboration-v0.md`](docs/design/standing-policy-v3-external-report-corroboration-v0.md)
    and [Ticket 0053](tickets/0053-standing-policy-v3-external-report-corroboration-contract.md)
    ratify the first explicit standing policy v3 aggregation rule: one exact
    `ExternalSource × ExternalReport` corroboration rule over one exact
    `OriginComparisonNamespaceV0` lane, requiring at least two distinct
    admitted origin groups for governed `Supported`, never `Settled`.
    With identity-valid inherited v2, governed `Settled`, `Refuted`, and
    `Supported` are preserved; inherited claim or policy identity mismatches
    yield no top-level standing; legacy raw status remains quarantined;
    same-origin contributions remain visible but count once. [Ticket
    0054](tickets/0054-standing-policy-v3-external-report-corroboration-implementation.md)
    implements that one narrow rule through the same verified replay context:
    a complete internally derived support audit with at least two distinct
    admitted origin groups in the exact requested claim's lane may contribute
    governed `Supported`, never `Settled`. Ticket 0054's implementation head
    `f030be8ff329c93fd7138d6f831bb44cc42e9409` merged via PR #68 at merge
    commit `7720a2749f3b85f95e38580d0af4fd26c6f02658` on 2026-07-23.
    Corroboration separation remains distinct from statistical independence.
    Refutation aggregation, contradiction debt, invalidation, supersession
    and currentness policy, general source-standing propagation,
    `EpistemicGate`, ordinary claim-bearing writer, loader, CAS, and
    filesystem or network ingestion remain future work.
26. **Standing policy v4 deterministic direct refutation v0 — contract
    ratified by Ticket 0056.**
    [`docs/design/standing-policy-v4-deterministic-direct-refutation-v0.md`](docs/design/standing-policy-v4-deterministic-direct-refutation-v0.md)
    and [Ticket 0056](tickets/0056-standing-policy-v4-deterministic-direct-refutation-contract.md)
    ratify the first direct-refutation rule: one exact
    `ExactMachineCheckable` claim, one exact typed
    `DeterministicVerification` evidence node, one exact `contradicts`
    edge, exact claim/evidence/edge/scope bindings, and one same-snapshot
    negative evaluation proving `sha256_bytes_equals_v0` false for the
    exact bound witness contribute governed `Refuted`. The
    existing `DigestMismatch` trace remains a positive-lane failure token
    and is never negative authority. Conflicting exact positive and
    negative verifications produce a closed blocker, never a silent
    override. No runtime is implemented. Direct-refutation runtime,
    contradiction debt, invalidation, supersession/currentness,
    `EpistemicGate`, ordinary claim-bearing writer, loader, CAS, and
    filesystem or network ingestion remain future work.

Conserve the log. Derive the rest.
