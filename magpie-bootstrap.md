# Magpie — Project Bootstrap

*Paste or attach this at the start of a fresh thread. It carries everything needed to continue building Magpie, plus the reasoning behind the decisions so nothing has to be re-derived.*

---

## 0. How to use this

I'm bootstrapping a fresh conversation to **build Magpie**. This document is the state file: who I am, what Magpie is, what's already built, what's decided, what's still open, and what to do next. Read it, then help me build — bottom of the stack up, one working slice at a time. Push back honestly; I follow the evidence, not hype.

---

## 1. Me & context

- **George** (GitHub: `noctem-o`). Self-taught Rust + systems developer, UK, working independently.
- **Mathematical physicist.** I have a manuscript on semiclassical exponents in bosonic models (targeting SciPost). Theorem 2 verified numerically to ~1e-15; the C4 reproduction holds over N=64–192; the F5 spectral lower bound is left **genuinely open** (honest about that). The manuscript's proven/conjectured/open/refuted tracking is where Magpie's epistemic-status idea came from — and it's my first real dogfooding content.
- **Hardware.** Primary workstation: **NixOS / Hyprland**, Ryzen 5800X3D / **RTX 4090** / 32 GB DDR4 (dual-boots Windows for gaming only). Planned: a **Ryzen AI Max+ 395** (120 GB unified memory) always-on hub; a possible 192 GB box later.
- **Values (these break every close call):** local-first, provenance, longevity (regenerate from the log rather than hoard), honest assessment over hype, and understanding what I build rather than pasting it.
- **How I like to work:** direct and technical, no flattery, tell me when I'm wrong and why, follow the evidence, humour welcome. Don't over-explain basics. Minimal formatting/bullet-spam in normal prose; structure is fine in reference docs like this one.

---

## 2. What Magpie is (the thesis)

**Magpie** is a local-first, provenance-first memory-and-reasoning substrate for research — "a research mind." (Just **Magpie** — not "Magpie RAG", not "CRAG"; RAG is one sub-layer. It's a *provenance-first memory & reasoning substrate*.)

**The spine, in one sentence:** *One signed append-only log is the sole source of truth; one small verified capability gate is the only thing that may write to it or act from it; everything else is a derived, regenerable projection.*

Physicist framing: **the log is the conserved quantity; the gate is the conservation law; everything else is dynamics.** Models, harnesses, and any future robot are **pluggable edges** — swappable without touching the spine.

### The layers

- **L0 — the log (Magpie core).** Ed25519-signed, SHA-256 hash-chained, append-only. Every observation / decision / action / consolidation / sensor-summary is an event. Doubles as audit + observability for free. Durable home on the NAS. **Boring and bulletproof first** — it's the one layer that can't be refactored later without pain. It's the *only* irreplaceable asset; everything else is rebuildable by replay.

- **L1 — the gate (`deadbolt`).** A ~200k-line Rust userspace microkernel (in progress). Object-capability model: authority is an unforgeable reference, never ambient. The governed loop is **suggest → compile → decide → witness**: the agent proposes an action as code, the harness normalizes it, deadbolt checks capabilities + policy and executes, a witness event is written to the log. Verify the *small* core with **Verus**; make it replayable with deterministic simulation testing. Govern the harness as an **untrusted subprocess from the outside**. deadbolt is also the **memory firewall** (FragFuse defense: admission control at write, capability checks at retrieval keyed to provenance, re-check of the assembled context packet at execution).

- **L2 — memory (a projection of the log).** One **SQLite** file: FTS5 (keyword) + **sqlite-vec** (vector) + entities/edges (graph) + provenance + temporal columns. Kept current by **DBSP / differential-dataflow** (incremental view maintenance, *not* nightly re-index). Five **typed stores**: episodic, semantic claim-graph, procedural runbooks, hypotheses & beliefs, prospective. A **context-packet compiler** assembles bounded, provenance-filtered context for the next action. The differentiator is the **epistemic status spectrum** — `open → conjectured → supported → settled/proven → refuted` — on every item; evidence *moves* items along it and every transition is a logged event. A **consolidation "dream cycle"** (decay → contradiction/supersession → merge → synthesis) writes *new* events, never mutates. A Karpathy-style **markdown wiki** (provenance + claim-status frontmatter) plus Obsidian/git is a **downstream projection**, never an intermediate truth the graph is built from.

- **Fabric.** Fast lane = **RTX 4090** (NixOS/CUDA, 24 GB, ~1000 GB/s) — latency-sensitive turns, agentic loops, small models. **LIVE**: Qwen 3.6 27B via llama.cpp. Hub = **Ryzen AI Max+ 395** (Linux, 120 GB, ~215 GB/s) — always-on big-MoE reasoner + persistent memory services (**PLANNED**). Bandwidth is the binding constraint → MoE mandatory, one big brain resident + swap (`llama-swap`). **LiteLLM** gateway (routing + observability + budget brakes) + **Tailscale** mesh. Free-tier APIs (Gemini, Groq, Cerebras, etc.) are overflow edges only.

- **Loops.** Six pieces: state, automations, worktrees, skills, connectors, sub-agents. The highest-value move is **splitting the maker from the checker** (different models). **Brakes before horsepower:** step cap, budget ceiling, blast radius, circuit breaker, dead-man heartbeat, human-read gate. The **four deaths:** runaway recursion, silent death, random walk, comprehension debt. Local inference flips the economics — marginal cost is electricity (~$2/mo), so the "$47k runaway" can't happen; but the correctness/comprehension brakes still apply.

- **Oversight ("judge before you improve").** **BINEVAL** binary-question evaluation wired to the epistemic spectrum (yes/no verdicts = evidence atoms that move claims along the axis). A **PR review panel** (event-driven, per-diff, *different model families*) + a **heartbeat** (time/phase-driven, over the trajectory). **RQGM epoch discipline** + an **adversarial cross-family judge** gate **Champion evolution** — because a static judge over-accepts AI work at ~1.91× the human rate, so evolving against a weak judge just breeds judge-foolers. The council = panelist / judge / **adversary**. DSPy/GEPA prompt optimization runs **offline** (Python seam), emitting signed prompt artifacts.

---

## 3. What's already built ✅

A working Rust scaffold exists (compiles, **all tests pass** on cargo 1.75). Cargo workspace `magpie/` with two crates:

- **`magpie-log`** (L0):
  - `event.rs` — `Provenance`, `Status` enum (`Open/Conjectured/Supported/Settled/Refuted`), `Payload` enum (`ClaimAsserted / EvidenceRecorded / ClaimStatusChanged / Note`), `EventCore` (with `canonical_bytes()` + `hash()`), `SignedEvent`, `Sig` + `ContentHash` (hex-serialized).
  - `hashing.rs` — SHA-256 `ContentHash` (the chain link).
  - `store.rs` — `LogStore` trait + `FileStore` (durable JSON-lines) + `MemStore` (Rc/RefCell, cloneable, for tests).
  - `logimpl.rs` — **`LogWriter`** (the *write capability* — the only type with `.append()`), **`LogReader`** (read-only: `events()`, `verify_chain()`, `replay()`), the `Projection` trait, and chain recovery/verification on open.
  - `tests/chain.rs` — chain integrity, tip recovery on reopen, tamper detection.
- **`magpie-claims`** (a projection): `ClaimsView : Projection` folds events into claim statuses. `tests/regenerable.rs` holds **the thesis test** — `projection_is_byte_identical_after_dropping_all_derived_state`: write the manuscript's claims, drop *all* derived state, replay from the log alone, assert byte-for-byte identical. Plus `examples/tour.rs`.

**The two invariants are enforced by the type system, not convention:** (1) append-only (no update/delete; a correction is a new superseding event); (2) the gate is the only writer (only `LogWriter` has `.append()`; memory gets a `LogReader`). In the full system, `deadbolt` holds the `LogWriter`.

**Honest caveats baked into the scaffold:**
- `EventCore::canonical_bytes()` uses `serde_json` today. JSON isn't a canonical-bytes format by spec — fine for a single codebase, **not** fine once the chain must verify across machines/versions. **This is the first thing to harden** — swap it for RFC 8785 JCS or a fixed binary codec. It's isolated in one function on purpose.
- Three version pins (`ed25519-dalek = "=2.1.1"`, `base64ct = "=1.6.0"`, `zeroize = "=1.8.1"`) exist **only** because it was built on Ubuntu's cargo 1.75; **drop all three on a current rustc.**
- `MemStore` is single-threaded (swap Rc/RefCell → Arc/Mutex for threads). `FileStore` is the durable one. Keys: the caller provides the `SigningKey`; generate with `OsRng` in production.

*(I have the scaffold as a tarball; I can attach it to the thread if we want to edit the actual files rather than regenerate.)*

---

## 4. Decisions locked

- **Two repos, held firmly** — because it's a *trust boundary*, not an org preference. `deadbolt` stays its own repo: the verified, slow-cadence, small-surface core that acts as the **outer governor** over a *runtime* boundary (it supervises the harness as an untrusted subprocess). `magpie` is a **monorepo Cargo workspace** (log, memory, gateway, loops, council as crates) that co-evolves. The **log crate is shared by both, asymmetrically**: the kernel holds the `LogWriter` (sole writer); memory only ever gets a `LogReader` — type-system enforced. Don't merge them (dilutes the trusted computing base). Only reconsider if the kernel/memory interface keeps *moving* — that'd signal it isn't stable yet.
- **Walking skeleton, not a finished floor.** Freeze the log's invariants early (append-only, signed, chained, replayable); grow the schema. Drive one trivial event through every layer before widening anything.
- **Do NOT build a full knowledge graph.** GraphRAG's gains are confined to global/aggregation queries (the "graph win" is really query-time aggregation). Use SQLite CTEs / `petgraph` over the existing edges; if I ever need "themes across everything," add LazyGraphRAG-style community detection *at query time*, not a maintained graph.
- **Wiki is a projection, never intermediate truth.** The graph is never built *from* the markdown.
- **Dogfood on the physics manuscript's claims** as the first real content.
- **Naming:** system = `Magpie`; gate = `deadbolt`. (Sub-names `cairn`/`signet` for log/gate were floated but not adopted.)
- **Resist the cathedral.** My documented failure mode is over-building the meta-layer. Every feature above the walking skeleton earns its place through *use*.

---

## 5. Research-validated priorities (what to add, ranked)

**Tier 1 — build now, fully Rust:**
1. **Cross-encoder reranker** as the final stage over RRF-fused hybrid candidates. Highest evidence-to-effort change (~+17pp MRR@3 measured). Rust: `fastembed-rs` `TextRerank`. Threshold: drop if it adds <3pp Recall@5 on my own eval.
2. **BINEVAL evaluator → epistemic transitions.** Atomic yes/no questions; N agreeing verdicts to reach "supported", adversary-passed for "settled". Native Rust orchestration.
3. **Formalize bi-temporal supersession.** `supported → refuted` emits an invalidation event with a validity interval — never a deletion (~18.5% gains on long-memory benchmarks).

**Tier 2 — next:**
- Harden the judge (RQGM epoch discipline: freeze within-epoch, evolve cross-family at boundaries, log every utility change) **before** any Champion evolution.
- FragFuse three-layer defense in `deadbolt`.
- Operationalize the dream cycle as **sleep-time compute** (+13–18% precomputing during idle; static embeddings via `model2vec-rs` for high-volume passes).

**Tier 3 — selective / defer:** DSPy/GEPA offline (Python-only, ≥50 logged eval instances per prompt, abort if a generation doesn't beat the incumbent on held-out data). Late-interaction (ColBERT) deferred.

**Rust build map:** embeddings = `fastembed-rs`/Candle/`model2vec-rs`; reranking = `fastembed-rs TextRerank`; vector = `sqlite-vec` (default)/LanceDB/Qdrant; FTS = FTS5 (default)/Tantivy; graph = SQLite CTEs/`petgraph`; incremental views = DBSP/Feldera/differential-dataflow (Rust-native, verified); gateway = LiteLLM (Python sidecar) *or* Helicone (Rust); prompt-opt = DSPy/GEPA (Python only). **Already SOTA and worth keeping as-is:** the signed log, DBSP, the epistemic spectrum (it generalizes bi-temporal validity), wiki-as-projection, and the capability gate as external governor.

---

## 6. Hard-won lessons & open problems (the "follow the evidence" file)

- **Benchmarks won't show Magpie's value.** Standard suites are memoryless and capability-isolating by design, so `Magpie + frontier model ≈ 0` benchmark lift — and the *stronger* the base model, the smaller the harness-visible lift (external memory partly substitutes for capability the model already has). Magpie's value lives on *my* workload — a compounding corpus, ~2–4× effective over a year, dominated by corpus discipline — which is nearly unbenchmarkable. **So build a personal longitudinal eval:** a fixed set of my recurring research tasks, bare-model vs model+Magpie, tracked as the corpus grows. Someday, build my own benchmarks (cross-domain synthesis; longitudinal corpus-rot; provenance/epistemic-correctness) — my signed replayable log *is* a contamination-proof dataset.
- **Two non-negotiables from the agent-memory benchmark (2606.24775):** (a) retrieval must filter by **current validity/epistemic-status**, never naive similarity over all history (else "hallucinations of the past"); (b) keep a **chronological retrieval path** alongside the semantic one (for time-dependent queries, raw long-context can beat lossy memory — and my cheap-long-context hardware makes routing to a chronological log-slice viable). Their four metrics to adopt: representation fidelity, retrieval precision, update correctness, long-horizon stability. Their code: `OpenDataBox/MemoryData`.
- **Verification is the hard part** (Qwen "Verification Horizon"): every verifier is a proxy for underspecified intent, and no fixed reward survives rising capability. My domain lacks a cheap success oracle (unlike ASPIRE's sim), so validation must anchor on **external resolution** (claims resolving over time) + the **adversary**, and the verifier must *evolve*.
- **Rot still happens — a specific kind.** My design kills *destructive* rot (append-only + immutable log) and *silent staleness* (supersession + the spectrum). What remains: **projection rot** (derived views drift; recoverable by replay), **calibration rot** (miscalibrated evidence-weighing — the deep, genuinely open one), and **salience rot** (correct facts, wrong things loud). **Heartbeats help salience/projection rot but NOT calibration rot** — a miscalibrated LLM auditing a miscalibrated LLM ratifies the drift. The only real fixes for calibration are **external ground-truth anchoring** and **self-calibration from my own logged transition history** — both of which I uniquely have (provenance + replay + a domain where claims resolve). The integrity guarantees can *mask* calibration failures, so treat a frontier checkup as a smoke detector, not proof.
- **Adjacent ideas worth revisiting:** activation steering (NPM, 2606.29824) — procedural memory as steering vectors distilled from contrastive experience, *complementary* to text runbooks; `llama.cpp` supports `--control-vector` natively, so it's a cheap weekend prototype. ASPIRE (NVIDIA GEAR) — richer per-step **witness traces for failure attribution**, and a skill schema of *failure-signature + when-to-apply guard + repair + code sketch* (adopt for procedural runbooks); its admission gate ≈ deadbolt; its own open limitation (skill-library staleness) is exactly what my supersession/spectrum targets.
- **On models & benchmarks:** don't trust vendor tables (the Mem0↔Zep dispute; the Agents-A1 "trillion-parameter with 35B" claim is soft benchmaxxing via distillation). Test any candidate on *my* tasks vs its base model, not on its benchmark table.

---

## 7. Build order & immediate next steps

Bottom-up; each phase earns the next:

1. **L0 log** — ✅ skeleton done (Ed25519, chained, append-only, replayable, tested).
2. **Harden the log:** swap `canonical_bytes()` for a canonical codec; re-run the chain tests. Drop the three toolchain pins on my modern rustc.
3. **First L2 projection:** add a **SQLite + FTS5 episodic** view alongside `ClaimsView` — same `Projection` trait, built purely by replay, read-only. Then wire in `sqlite-vec` + the reranker (Tier 1).
4. **Feed the manuscript claims in** and query my own provenance — the "it's actually useful" milestone.
5. **deadbolt:** wire its capture/provenance into `append`; hand it the `LogWriter`.
6. **One governed maker/checker loop** — one harness, brakes-first, doing real research. Not a comparison lab.
7. **Then** panel + heartbeat; **Champion evolution last**, only once the evaluator can't be gamed.

**Start smaller than feels satisfying:** one local maker, one local checker, one boring task I actually do by hand, a hard `--max-turns`, and a kill switch I can reach — before wiring in any SOTA channel.

---

## 8. The Fable 5 window (7 days)

Fable 5 (Mythos-tier) is briefly returning and I want to make the most of it. Guidance from this project's own logic:
- **Use it to *build* Magpie, not to wire *into* Magpie.** Access is temporary — don't build a dependency on a model I'll lose. Spend it as a high-capability collaborator on the **judgment-heavy** work: hardening the canonical codec, the deadbolt gate design, the trickiest L2/retrieval decisions, and reviewing my Rust.
- **If I do put it in a loop:** the Devin-Fusion lesson is that a frontier model should **own the judgment** (plan, ambiguity, review) and **delegate the mechanical** to local models — and Fable 5 is unusually strong at that delegation pattern. Delegating judgment backfires; delegating grunt work wins.
- Net: treat the 7 days as concentrated architecture + hard-problem time, capturing every decision into the log/design notes so the value persists after access ends.

---

*Conserve the log. Derive the rest. Judge before you improve. Stay the engineer. Build the log first — everything else follows from there.* 🐦‍⬛⛓
