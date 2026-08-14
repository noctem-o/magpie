# ADR-0009: Verified Supplied History and Explicit Checkpoint Expectations

**Status:** Proposed (explicit repository-owner ratification required)

## Summary / Y-statement

In the context of replay that completely verifies one exact supplied record
snapshot but has no external expected-history input, facing the risk that a
valid old prefix is described as the history a caller expected, we propose a
strict separation:

```text
V = successfully verified result for the exact supplied history H

C = explicit immutable checkpoint expectation
E = explicit normative expectation relation

D = expectation outcome for V, C, and E under one immutable profile G
```

`V` establishes only that the exact history supplied for that evaluation
passed the selected verification semantics. It does not establish that the
history was expected.

`D` establishes only whether that verified supplied history satisfies the
explicit checkpoint relation selected in its complete producing context. It
does not make the checkpoint trusted and does not establish freshness, latest
history, global completeness, non-equivocation, durability, or rollback
resistance.

This is a Proposed constitutional decision. It is not Accepted doctrine until
the repository owner explicitly ratifies it.

## Context and current evidence

Current replay has one strong, narrow property:

```text
read one exact record snapshot
-> parse and verify every supplied record under the reader's supplied key
-> only after complete success, replay those same retained records
-> return the exact supplied event count and chain tip
```

`LogReader::replay_with_summary` implements that ordering. A late parse,
sequence, predecessor-link, content-hash, signature, payload, genesis-profile,
or genesis-key-binding failure produces no projection application and no replay
summary. `VerifiedReplaySummary` then reports the event count and tip of the
exact supplied snapshot that was replayed.

The claims layer derives `VerifiedLogPrefixIdentityV0` from that summary as
roughly:

```text
event_count + tip_sha256
```

That object is useful evidence of one verified supplied prefix. It is not an
expected checkpoint, does not record who expected anything, carries no
expectation relation, and does not compare the supplied history with an
external historical commitment.

The verifier accepts any finite supplied history that satisfies its rules. If
records 0 through 100 once existed, the otherwise valid records 0 through 80
remain a valid supplied history. Verification must not call them invalid merely
because a separate caller expectation names position 100. Conversely, a
checkpoint evaluator must not call them expected merely because verification
succeeded.

## Scope

This ADR governs the semantic distinction between:

1. verification of one exact supplied historical input; and
2. evaluation of that verified input against one explicit immutable historical
   expectation.

It standardizes the two expectation relations required by the present
architectural boundary because they answer different questions:

- **Exact** — does the supplied verified history terminate at the expected
  historical position?
- **ContainsCheckpoint** — does the supplied verified history contain the
  exact expected historical position, possibly with a verified suffix after
  it?

`ContainsCheckpoint` is reflexive: a history whose terminal position is the
checkpoint contains that checkpoint. `Exact` remains distinct because it also
requires that the supplied history contain no supplied suffix after the
checkpoint.

These standardized relations are not a universal ceiling on `E`. A future ADR
or profile may define another explicit immutable expectation predicate without
amending ADR-0009 merely because that predicate differs, provided its complete
producing context obeys ADR-0008 and this ADR's non-ambient and non-overclaim
laws.

This ADR defines propositions, not wire schemas, Rust types, persistence, or
rollback protection.

## Terms

**Supplied history (`H`).** The finite exact historical input supplied to one
verification evaluation. It is not presumed to be the latest, longest,
globally complete, canonical, or caller-expected history.

**Verified supplied history (`V`).** The semantic outcome that the exact
supplied `H` passed one explicitly selected immutable verification
interpretation under its complete producing context. It speaks only about that
input and those verification semantics. Trust in an externally selected key or
root remains a separate input and does not arise from the history itself.

**History checkpoint (`C`).** An explicit immutable historical expectation
value that minimally identifies one expected historical position and one exact
immutable commitment to the history through that position. `C` need not itself
contain every semantic coordinate needed to interpret those values; those
dependencies belong to the complete producing context under ADR-0008. A
checkpoint is expectation material, not automatically trusted material.

**Expectation relation (`E`).** The exact normative predicate being evaluated.
This ADR defines `Exact` and `ContainsCheckpoint`. Relation selection may not
be inferred from names, input shape, availability, length, defaults, or
ambient state.

**Expectation outcome (`D`).** The profile-defined semantic statement that the
selected relation is satisfied or not satisfied, together with any
profile-defined typed semantic failures. It states no property beyond the
selected relation. Operational inability to complete evaluation produces no
semantic conclusion, as required by ADR-0008.

**Historical position.** A position under the selected history semantics. This
ADR does not freeze it as an event count, sequence number, byte offset, or
database row.

**Historical commitment.** The immutable commitment that lets the selected
semantics identify the exact history through one position, including the
ancestry required by those history semantics. A commitment to only an isolated
terminal record is insufficient when it does not commit to that ancestry. This
ADR does not freeze the commitment as a SHA-256 tip, Merkle root, full-prefix
digest, or any other mechanism.

## Decision

The following three invariants govern this proposal.

### 1. Verification describes only the exact supplied history

Successful verification states:

```text
the exact finite history H supplied to this evaluation
passed the selected verification semantics
under the complete producing context of that verification
```

It does not state:

```text
H is the history the caller expected
H reaches a remembered or published checkpoint
H is fresh, latest, globally complete, or uniquely canonical
```

A valid old prefix therefore remains cryptographically and structurally valid.
Failure to meet an external expectation must not be recast as invalid history.
Malformed or unverifiable supplied history produces its verification failure;
it does not produce a false checkpoint comparison.

### 2. Expected-history agreement requires explicit immutable expectation semantics

Every expectation evaluation requires:

- an exact verified supplied-history input, internally re-derived or carried
  according to ADR-0008;
- one explicit immutable checkpoint `C`; and
- one exact relation `E` selected immutably in the producing context.

`C` is an evaluation-specific semantic input and therefore belongs in the
expectation profile's complete `I_G`. The checkpoint value itself needs to
carry only the position-and-commitment floor below. Every semantic dependency
needed to interpret it must be identified by the complete producing context in
the ADR-0008 coordinate appropriate to its role; it need not be embedded in
`C`. If checkpoint provenance, authentication, witness identity,
authorization, or immutable secure-retention evidence can affect the result,
the complete producing context must likewise identify that exact material.
The word *checkpoint* supplies none of it.

The relation may be represented in either of two conforming ways:

1. `G` commits a closed relation vocabulary and `E` is an explicit member of
   `I_G`; or
2. one immutable `G` fixes exactly one relation, so selecting `G` selects `E`.

This ADR does not force an enum or separate profiles because either packaging
provides the same semantic invariant. In both cases `(G, I_G)` immutably and
unambiguously identifies the exact relation. Different relations are different
producing contexts. No default, fallback, try-both, or success-dependent
relation choice is permitted.

For `ContainsCheckpoint`, position comparison alone is insufficient:

```text
supplied terminal position >= checkpoint position
!= supplied history contains checkpoint
```

The exact commitment named by `C` must occur at the exact expected position in
the verified supplied chain under the selected semantics.

### 3. Expectation satisfaction grants only the selected relation

A successful expectation result states only:

```text
this exact verified supplied history
satisfies this exact immutable checkpoint
under this exact normative relation and producing context
```

It does not authenticate the source of `C`, elect a canonical branch, rule out
another branch or unseen suffix, establish currentness, or make the result
authority-bearing. Any stronger proposition requires its own explicit inputs
and normative profile. It may not be inferred from checkpoint vocabulary or
from a successful relation result.

## Exact relation

`Exact(H, C)` is satisfied only when:

1. `H` is successfully verified under the selected complete verification
   context;
2. `H` has the exact historical position named by `C` as its terminal
   position; and
3. the historical commitment at that position exactly matches the commitment
   named by `C` under the selected semantics.

Conceptually:

```text
terminal_position(H) == position(C)
commitment_at(H, position(C)) == commitment(C)
```

A valid suffix after `C` makes `Exact` not satisfied. Successful `Exact` means
only that no suffix was supplied in this exact `H`. It does not prove that no
later or sibling history exists elsewhere.

## ContainsCheckpoint relation

`ContainsCheckpoint(H, C)` is satisfied only when:

1. `H` is successfully verified under the selected complete verification
   context;
2. the exact position named by `C` exists in `H`; and
3. the historical commitment at that exact position matches the commitment
   named by `C` under the selected semantics.

Conceptually:

```text
position(C) exists in H
commitment_at(H, position(C)) == commitment(C)
```

The full supplied `H` must still pass verification, so any suffix presented
after `C` is verified as part of that same supplied history. Equality is
allowed: a checkpoint is contained in the history that terminates at it.

Two sibling histories can both contain the same checkpoint. Therefore
`ContainsCheckpoint` is an ancestry-floor proposition, not branch selection,
non-equivocation, consensus, or global uniqueness.

## Minimum checkpoint semantic floor

A conforming checkpoint value must minimally identify:

```text
one exact expected historical position
+ one exact immutable commitment to the history through that position
```

That is the semantic floor for `C` itself, not for its complete producing
context. Position without commitment admits a length-substitution attack.
Commitment without its required position does not answer the wrong-position
hostile case.

The complete producing context must separately identify every semantic input
needed to interpret `C`. Depending on the selected profile, those dependencies
may include namespace or log identity, verification profile, trust root,
commitment or canonicalization interpretation, externally selected trust
material, or another profile-specific semantic input. Each belongs in `G`,
`I_G`, or another ADR-0008-conforming coordinate according to its role and need
not be embedded or serialized in `C`.

This semantic floor deliberately does not choose:

- `{event_count, tip_sha256}`;
- sequence plus event hash;
- full-prefix digest;
- Merkle root or proof;
- checkpoint object encoding;
- canonicalization or hash profile; or
- equality implementation.

Today's `VerifiedLogPrefixIdentityV0` is evidence that count and tip can name
one current verified prefix. This ADR neither reinterprets that type as a
checkpoint nor makes its shape the constitutional representation.

## Complete producing context under ADR-0008

This proposal is an application of, not an alternative to, ADR-0008:

```text
D = Derive_G(I_G)
```

For a governed detached expectation result:

- `G` immutably commits to the exact frozen semantics for internally performing
  or consuming verification, the exact checkpoint and relation semantics,
  typed semantic failures, ordering, and output meaning;
- `I_G` includes the exact supplied historical input or an ADR-0008-conforming
  verified-history input, the explicit checkpoint `C`, the relation `E` when
  it is not fixed by `G`, and every other semantic input;
- every semantic dependency needed to interpret `C` is identified by the
  complete producing context in `G`, `I_G`, or another ADR-0008-conforming
  coordinate according to its role; no such dependency must be serialized
  into `C`;
- no checkpoint, relation, verification root, checkpoint authority, or
  historical material is ambient;
- operational I/O, storage, resource, interruption, or host failure produces
  no `D`; and
- detached `D` obeys ADR-0008's immutable producing-context and exact-result
  carriage law.

If the evaluator internally re-verifies `H`, its enclosing producing context
must close over the complete verification context. If it accepts a detached
verified-history result, a context identity alone is insufficient: the exact
verification outcome must be re-derived and exact-matched or carried through
an ADR-0008-conforming immutable result binding. `(context_id, arbitrary_V)` is
not a verified-history input.

## No ambient checkpoint or relation

None of the following can select semantic `C` or `E`:

- “latest checkpoint” or “current head”;
- `checkpoint.txt` by convention;
- whichever checkpoint a database row currently names;
- process-global remembered state;
- a network response, callback, plugin, registry, or environment default;
- the greatest available position;
- a mutable alias, handle, lookup table, or re-pointable name; or
- an ambient content-addressed lookup.

An immutable identity is not permission for an ambient fetch. Material needed
for evaluation must be supplied explicitly and checked against its immutable
identity. Failure to read or fetch it is an operational failure, not
`NotSatisfied`.

If a profile deliberately gives one finite supplied-absence value semantic
meaning, that exact immutable value is an explicit member of `I_G` under
ADR-0008. It is not inferred from an I/O, storage, resource, process, or host
failure.

## Checkpoint trust and authority boundary

A checkpoint is an expectation, not a credential. A caller may supply an
arbitrary checkpoint. The evaluator may truthfully report whether the selected
relation holds against that value, but success does not establish that the
caller was authorized, the checkpoint was authentic, or reliance on it is
wise.

Potential future checkpoint sources include prior local observation, a release
manifest, an external witness, another Magpie instance, a signed service, or
hardware-protected state. This ADR selects none. If a future profile's semantic
result depends on source, authentication, authorization, witness agreement, or
trust, ADR-0008 requires those exact immutable inputs and their normative
interpretation to be part of the complete producing context.

Conflicting checkpoints remain separate explicit expectations. No event order,
timestamp, lexical order, signature count, witness count, “newer” label, or
ambient policy selects a winner under this ADR.

## Persistence boundary

Checkpoint expectation semantics do not provide secure checkpoint persistence.
For example:

```text
yesterday: H100 and C100
today after whole-system rollback: H80 and C80
```

`H80` may verify and satisfy `C80` perfectly. Nothing in the relation reveals
that both the log and checkpoint were rolled back together.

A securely retained checkpoint can be an input to a later anti-rollback
system. This ADR does not retain it securely, publish it, rotate it, lock its
store, synchronize it, witness it, or make it durable. Those are later L0 and
persistence decisions.

## Explicit non-implications

Even after successful `Exact` or `ContainsCheckpoint`, none of the following
follows merely from checkpoint satisfaction:

| Property | Why it does not follow |
| --- | --- |
| Fresh | The comparison has no knowledge of a newer history elsewhere. |
| Latest | No global or ambient head is consulted. |
| Globally complete | Only one finite supplied history was evaluated. |
| Globally canonical | The relation does not elect a branch. |
| No unseen suffix exists | `Exact` excludes a suffix only from the supplied `H`; another suffix may exist elsewhere. |
| No sibling fork exists | Both sibling branches after one checkpoint can satisfy `ContainsCheckpoint`. |
| Non-equivocated | No issuer-wide uniqueness or witness comparison is performed. |
| Consensus-backed | No consensus system is an input. |
| Witness-backed | A plain checkpoint carries no witness semantics. |
| Durable | No persistence acknowledgement or crash law is defined. |
| Rollback-resistant across process or storage compromise | The history and checkpoint can be rolled back together. |
| Trusted because of checkpoint provenance | Checkpoint provenance and trust are not automatic. |
| Authority-bearing | Matching an expectation grants no write, admission, standing, or policy authority. |
| Current in the ADR-0006 sense | Currentness is a separate resolver-derived applicability facet. |

A future profile may establish an additional proposition only by naming its
complete immutable inputs and normative semantics under ADR-0008. Such a
future result would not enlarge the meaning of this checkpoint relation.

## Hostile scenario matrix

| ID | Scenario | Exact truthful result | What may not be inferred |
| --- | --- | --- | --- |
| C1 | Valid old prefix ends below the expected checkpoint | Verification succeeds; `Exact` and `ContainsCheckpoint` are not satisfied | The old prefix is not cryptographically invalid merely because expectation fails |
| C2 | Supplied history terminates at the exact expected position and commitment | `Exact` is satisfied; reflexive `ContainsCheckpoint` is also satisfied when selected | Freshness, latest history, global completeness, or checkpoint trust |
| C3 | Supplied verified history continues validly beyond the checkpoint | `Exact` is not satisfied; `ContainsCheckpoint` is satisfied | The supplied terminal position is latest or globally canonical |
| C4 | Two sibling forks both begin after the same checkpoint | Each fully verified branch satisfies `ContainsCheckpoint` for that checkpoint | Non-equivocation, fork absence, consensus, or branch preference |
| C5 | Checkpoint comes from another genesis or log history | A different committed ancestry or a history namespace or log identity required by the selected producing context does not satisfy. If both inputs commit the exact same history through that position under the same semantics, this relation does not distinguish a copied prefix by external label alone | Similar length or shape is not log identity; the checkpoint is not thereby declared malicious |
| C6 | Supplied history reaches the right position but has the wrong commitment | The relation is not satisfied | Position equality is not ancestry or content equality |
| C7 | The checkpoint commitment is claimed at the wrong historical position | The relation is not satisfied; position and commitment must match together | Commitment resemblance cannot repair position substitution |
| C8 | Checkpoint position is beyond the supplied history | `Exact` and `ContainsCheckpoint` are not satisfied | Verification of the shorter supplied history remains truthful |
| C9 | A mutable “latest checkpoint” alias is offered | No conforming producing context exists and no `D` is produced | Whatever value the alias currently resolves to cannot become semantic input implicitly |
| C10 | Local history and local checkpoint are rolled back together | They may verify and satisfy the selected relation | Rollback detection, secure persistence, or continuity from a prior observation |
| C11 | An untrusted caller supplies an arbitrary checkpoint | The relation is evaluated against that exact value and may be satisfied or not | Success does not authenticate or authorize the caller or checkpoint |
| C12 | Two conflicting checkpoints are supplied separately | Each is a distinct evaluation. Genuinely incompatible checkpoints cannot both match one linear history; distinct compatible ancestral checkpoints can both satisfy `ContainsCheckpoint` | No implicit winner, merge, vote, recency rule, or conflict resolution |
| C13 | The same extended history and checkpoint are evaluated under `Exact` and `ContainsCheckpoint` | `Exact` is not satisfied; `ContainsCheckpoint` is satisfied | The results are not interchangeable; they have different producing contexts |
| C14 | The same checkpoint bytes are interpreted under different profile-specific semantic contexts | They are different `G`/producing contexts; a result cannot be transferred, and incompatible input may produce a typed semantic failure | Byte equality is not semantic-profile equality |
| C15 | Supplied history is malformed or fails verification | Verification returns its typed semantic failure; no expectation-satisfaction conclusion is produced | Unverifiable history is not a false checkpoint relation and cannot feed comparison as verified input |
| C16 | Checkpoint material cannot be read because of I/O, storage, resource, interruption, or host failure | No semantic `D` is produced | Operational failure is neither `NotSatisfied` nor evidence of rollback |
| C17 | A checkpoint matches, but an unseen later suffix exists elsewhere | The selected relation may be satisfied for the supplied `H` | No-unseen-suffix, latest, or freshness |
| C18 | Supplied history forks before the checkpoint but reaches an equal or greater position | `ContainsCheckpoint` is not satisfied because the exact commitment is absent at the required position | Length is not ancestry |
| C19 | Empty history or genesis boundary | Current verification may establish an empty supplied input; it matches only an explicit profile-defined empty checkpoint. A genesis checkpoint is matched by its exact position and commitment; no universal empty checkpoint is implied | Empty acceptance does not establish genesis, key binding, log identity, or trust |
| C20 | A content-addressed checkpoint is discovered through an ambient lookup | No conforming evaluation occurs until the exact immutable `C` is supplied explicitly and checked under the selected context | Content addressing does not authorize ambient retrieval or mutable name resolution |

## Alternatives considered

| Alternative | Decision | Smallest defeating counterexample or reason |
| --- | --- | --- |
| A. Treat a valid supplied prefix as expected history | Rejected | C1: valid 0…80 does not satisfy explicit C100 |
| B. Rename verified prefix to complete history | Rejected | C17: an unseen suffix can exist while the supplied prefix verifies |
| C. Fold checkpoint disagreement into history verification | Rejected | C1 requires verification success and expectation failure simultaneously |
| D. Separate verification from expectation evaluation | Selected | It preserves both truthful propositions without weakening either |
| E. Support only exact equality | Rejected as the only relation | C3: a legitimate continuation beyond C100 would fail despite containing the required floor |
| F. Support only extension/containment | Rejected as the only relation | It cannot express the narrower proposition that the supplied history terminates exactly at C100 |
| G. Support explicit relation semantics | Selected | C13 proves relation choice changes the proposition and must be explicit |
| H. Put relation only in `G` rather than permit it in `I_G` | Not required constitutionally | ADR-0008 secures either an immutable relation-specific `G` or an explicit closed `E` input; forcing packaging adds no safety invariant |
| I. Freeze checkpoint as count plus tip | Rejected | Current shape is implementation evidence, while the constitutional floor is position plus commitment under profile-specific semantics |
| J. Keep checkpoint representation abstract | Selected | It fixes the substitution-resistant proposition without prescribing mechanism |
| K. Treat checkpoint source as authority automatically | Rejected | C11: a caller can choose any checkpoint, including the supplied tip itself |
| L. Define persistence or remembered-head storage here | Rejected | C10 shows matching semantics and secure retention are separate problems |

## Relationship to existing doctrine and contracts

- **ADR-0008 governs producing coordinates.** This ADR instantiates
  `D = Derive_G(I_G)` for checkpoint expectations and does not reopen or amend
  its complete-input, no-ambient, detached-result, or operational-failure law.
- **ADR-0003 remains unchanged.** Standing remains a separate profile-relative
  derived conclusion, not history freshness, checkpoint satisfaction, or
  authority.
- **ADR-0006 remains unchanged.** Checkpoint satisfaction is not currentness or
  resolver-relative applicability.
- **ADR-0007 remains unchanged.** A checkpoint grants no origin-group,
  candidate-designation, trust, or corroboration authority.
- **Verified replay remains unchanged.** The wrapper-only projection boundary
  and one-read, complete-before-apply ordering continue to describe current
  runtime.
- **The portable verifier contract remains unchanged.** Its successful count,
  tip, and ordered hashes describe supplied-input verification, not external
  expectation agreement.
- **The sole-write contract remains unchanged.** This ADR supplies no storage,
  locking, durability, acknowledgement, or recovery law.
- **NEW-LOG-01 is not closed.** This proposal supplies a constitutional answer
  for owner review; runtime and persistence evidence remain absent, and audit
  and readiness records are not changed.

## Consequences

Positive consequences:

- Magpie can describe a valid old history without falsely calling it expected;
- an explicit checkpoint can detect a below-checkpoint or wrong-branch input
  without redefining verification failure;
- exact terminal agreement and checkpoint containment remain distinct;
- a future representation can use today's count-and-tip evidence or a stronger
  mechanism without another constitutional rewrite; and
- successful matching cannot silently acquire trust, currentness, authority,
  durability, or fork-uniqueness semantics.

Costs and constraints:

- callers that need expected-history agreement must supply an explicit
  immutable checkpoint and relation;
- a future `ContainsCheckpoint` implementation must establish exact commitment
  at the required position rather than compare lengths;
- detached verification and expectation results need ADR-0008-conforming
  producing-context/result carriage; and
- rollback resistance still requires a separate secure-retention and
  persistence architecture.

## Deferred decisions

- runtime type and API names;
- checkpoint encoding, hashing, canonicalization, and equality mechanism;
- whether the first implementation uses count plus tip or another commitment;
- whether relation selection is an input enum, separate immutable profiles, or
  another representation satisfying the invariant;
- typed result and semantic-failure vocabulary;
- how an evaluator obtains intermediate historical commitments efficiently;
- detached verified-history and checkpoint-result identities under C3;
- checkpoint source authentication, authorization, witnessing, and trust;
- checkpoint publication, rotation, persistence, locking, durability, and
  recovery; and
- any rollback-protection system.

## Non-goals

This documentation-only proposal does not:

- change Rust runtime, tests, FORMAT, schemas, fixtures, Cargo metadata, CI, or
  release material;
- change `FileStore`, SQLite, portable verifier implementation, or A-021
  signature implementation;
- design or implement checkpoint storage, remembered heads, rollback
  protection, fsync, locking, transactions, TPM/HSM use, remote witnesses,
  consensus, replication, gossip, quorum, or checkpoint rotation;
- make `VerifiedReplaySummary` or `VerifiedLogPrefixIdentityV0` a checkpoint or
  detached authority credential;
- change ADR-0006, ADR-0007, or ADR-0008;
- implement currentness, authority-bound corroboration, checkpoint evaluation,
  or producing-coordinate result envelopes;
- change audit dispositions, audit counts, readiness classifications, or
  historical audit records;
- mark NEW-LOG-01 closed; or
- announce or authorize a release.

## Ratification and implementation boundary

This ADR is Proposed. Explicit repository-owner ratification is required before
it becomes Accepted doctrine.

Merge, green checks, a draft PR, or the presence of this file do not constitute
owner ratification. Ratification would settle only the constitutional
distinction. It would not implement checkpoint evaluation, prove secure
checkpoint persistence, close NEW-LOG-01, or authorize rollback-resistance
claims.

After ratification, a separately reviewed implementation contract may choose
the exact checkpoint representation, relation packaging, typed outcomes,
complete producing coordinates, and hostile runtime proofs. Secure checkpoint
persistence remains a separate later L0/persistence tranche.

## Reviewer checklist

- Confirm verification speaks only about the exact supplied history.
- Confirm a valid old prefix can verify while an explicit expectation fails.
- Confirm `C` is explicit immutable input whose minimum value is position plus
  commitment, while every dependency needed to interpret it is identified by
  the complete producing context and need not be embedded in `C`.
- Confirm no checkpoint source is trusted by terminology alone.
- Confirm relation selection is immutable and explicit in `(G, I_G)` with no
  default or fallback.
- Confirm `Exact` and `ContainsCheckpoint` are standardized for the present
  boundary without becoming a universal ceiling on future explicit immutable
  `E` predicates.
- Confirm `Exact` and reflexive `ContainsCheckpoint` state different
  propositions.
- Confirm `ContainsCheckpoint` checks exact commitment at exact position, not
  only position or length.
- Confirm a fork after the checkpoint can satisfy containment on both branches.
- Confirm a fork before the checkpoint cannot satisfy containment merely by
  reaching a greater position.
- Confirm current count-and-tip types are evidence, not constitutionalized
  checkpoint representation.
- Confirm ADR-0008 governs complete producing inputs, detached result carriage,
  and operational failures.
- Confirm no checkpoint, relation, trust root, alias, filesystem, database,
  network, callback, registry, or content-addressed lookup is ambient.
- Confirm checkpoint satisfaction implies no freshness, latest, global
  completeness, currentness, trust, authority, durability, rollback resistance,
  non-equivocation, or consensus.
- Confirm local history and local checkpoint may be rolled back together and
  still match.
- Confirm runtime, persistence, Accepted ADRs, audits, readiness, and release
  material remain unchanged.
- Confirm Proposed status and explicit owner-ratification gate remain visible.

## References

- ADR-0003, canonical definition of standing.
- ADR-0006, claim currency and currentness semantics.
- ADR-0007, authority-bound origin corroboration.
- ADR-0008, complete producing coordinates.
- `docs/audits/public-pre-alpha-convergence-baseline-2026-08.md`, NEW-LOG-01
  and C0/C1 boundaries.
- `docs/audits/audit-disposition-2026-08.md`, A-008 / RQ-007 / RQ-008.
- `docs/design/portable-verifier-input-language-contract.md`.
- `docs/design/logwriter-sole-write-capability-contract.md`.
- `docs/design/verified-replay-projection-boundary-contract.md`.
- `crates/magpie-log/src/logimpl.rs`.
- `crates/magpie-log/src/store.rs`.
- `crates/magpie-log/src/canonical.rs`.
- `crates/magpie-log/src/event.rs` and `hashing.rs`.
- `crates/magpie-claims/src/origin_admission_replay.rs`.
- `crates/magpie-claims/src/replay_snapshot.rs`.
