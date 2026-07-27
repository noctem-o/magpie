# Claim-Inline Subject Binding v0

## Status

Contract ratified by Ticket 0057. Documentation only.

```text
subject-binding law:
claim-invariant byte-subject identity — ratified

mechanism:
claim-owned inline immutable subject bytes — ratified as the one
concrete subject representation

predicates, policies, standing rules, refutation:
none ratified

runtime:
not implemented
```

This note ratifies the subject-binding prerequisite identified by Ticket
0056 and defines one concrete subject representation and its versioning
boundary. It ratifies no predicate, no standing policy, no evaluation
checker, no refutation rule, and no runtime. It adds no Rust, tests,
fixtures, Cargo metadata, dependency, CI, or L0 surface.

## Purpose

Ticket 0056 proved that predicate inequality cannot carry claim-level
negative authority while the subject of a digest proposition is selected
by each evidence candidate. This contract establishes the missing
prerequisite:

```text
claim-invariant byte-subject identity

an ExactMachineCheckable claim independently commits to exactly one
immutable byte subject, and every positive or negative deterministic
evaluation can establish — inside the same verified replay and without
caller-substitutable authority — that it checked those exact
claim-fixed bytes
```

The mechanism is deliberately the smallest sound one: the claim carries
its own subject bytes. Because the checked bytes come only from the
replayed claim, no evidence node can ever supply or substitute them.

## The Ticket 0056 counterexample this closes

```text
claim C:
  statement = sha256_bytes_equals_v0:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
  (SHA-256 of the exact bytes "abc")

contradicts evidence E-:
  witness_hex = 756e72656c61746564206d6174657269616c
  ("unrelated material"), same subject_claim_id = C, same predicate ID,
  same scope, valid contradicts edge
  → digest d42273bca408c671996c4f6eea5efefd010450655a44d742b0a2ba055b0542dc
  ≠ H
```

The v0 predicate is evidence-relative: the checked proposition is "the
raw byte witness carried by this exact evidence node has digest H".
Nothing established that `"unrelated material"` was the claim's subject.
This contract removes that entire class of substitution by moving the
subject into the claim itself: under the mechanism below, the bytes
being hashed are the claim's own committed bytes, and an evidence node
supplies no subject bytes to that evaluation.

## The subject-binding law

```text
1. Byte equality is the identity relation.
   "Object identity" counts only when a fixed resolver maps it to
   exactly one immutable byte string. "Canonical-form identity" counts
   only when the proposition explicitly names its canonicalization
   profile and is honestly about the resulting canonical bytes.

2. The claim owns the subject descriptor.
   Every digest proposition must carry, inside the claim's own
   canonically committed representation, a complete subject descriptor:
   a closed subject kind, the exact encoding/profile version, the
   subject bytes (inline) or a complete immutable selector (future
   kinds), the exact resource bound, and the expected digest.

3. The canonical statement carries the descriptor.
   The claim's canonical statement must itself encode the complete
   subject descriptor and expected digest. Binding is the exact direct
   three-way comparison `StandingClaim.statement ==
   TypedClaimNode.statement == the descriptor-derived canonical
   statement`; `ClaimAssertedV2.content_hash` is separately recomputed
   as the SHA-256 of the exact UTF-8 statement bytes, with collision
   resistance assumed and hash equality never used as identity. The
   content hash is never repurposed as a subject digest.

4. One resolution path for both polarities.
   Positive and negative evaluation must resolve the subject through
   one identical claim-owned path and may differ only at the terminal
   digest comparison. There are never separate positive and negative
   subject suppliers.

5. Resolution failure is never falsity.
   Missing, malformed, oversized, ambiguous, conflicting, or
   unparseable subject material is a closed audit failure with no
   standing effect. It is never `PredicateFalse`, never a refutation,
   and never evidence of anything.

6. No routing value is content identity.
   A claim ID, evidence ID, edge ID, scope string, actor class,
   provenance string, rationale, signature, or append-only retention
   never establishes byte-subject identity.

7. Evidence supplies no candidate-selectable subject bytes.
   Existing v0 evidence formats — including
   `magpie-verification-witness-v0` and `witness_hex` — remain
   unchanged and valid under their own historical path. For the new
   inline-subject family, the subject comes only from the replayed
   claim; evidence and edges may bind or attest; no evidence or edge
   field may provide an alternate subject byte source consumed by
   inline-subject resolution or later evaluation over that resolved
   subject; any such alternate source is rejected by that future path.
   This is not a global ban on byte-bearing evidence.
```

## The chosen mechanism: claim-owned inline immutable subject bytes

### Subject descriptor

The outer claim metadata object keeps the repository's established
nested machine-predicate shape. It carries exactly two keys:

```text
claim_domain:                 exactly ExactMachineCheckable, parsed and
                              revalidated by the existing strict
                              claim-domain parser
machine_predicate_inline_bytes: the closed inline-subject descriptor
```

The nested descriptor carries, and carries only:

```text
schema:          exactly magpie-machine-predicate-inline-bytes-v0
predicate_id:    a closed lowercase [a-z0-9_]+ identifier, at most
                 MAX_PREDICATE_ID_BYTES_V0 bytes long, introduced only
                 by a later predicate contract (this contract ratifies
                 none)
subject_hex:     the complete inline subject bytes, strict lowercase
                 even-length hex, empty string permitted
expected_sha256: the expected SHA-256 digest of the exact subject
                 bytes, in one exact representation: exactly 64
                 characters of lowercase hexadecimal only — no prefix,
                 no uppercase, no alternate encoding, no whitespace —
                 validated before canonical statement construction
```

No descriptor field may appear at the outer level. There are no other
outer keys, no duplicate keys at either level, and no unknown keys in
the nested descriptor.

The predicate identifier is bounded by an exact compiled constant:

```text
MAX_PREDICATE_ID_BYTES_V0 = 64 bytes
```

The bound is checked after the missing, wrong-type, and empty checks,
and before character validation, canonical statement construction, or
any allocation or copy. The longest current policy and predicate
identifiers are about 35 characters (`sha256_bytes_equals_v0` is 21;
`magpie-claims-standing-v3` is 25); 64 bytes gives comfortable headroom
while keeping the value compiled and checked, never caller-unbounded.

Each schema version fixes its own field set, subject encoding, and
resource bound; the schema constant names that complete profile, so no
profile or bound value ever travels as caller data.

The decoded subject is bounded by an exact compiled constant:

```text
MAX_INLINE_SUBJECT_BYTES_V0 = 4096 decoded bytes
```

Hex input exceeding 8192 characters (strict pre-decode maximum,
corresponding to 2 × 4096 decoded bytes) is rejected before decoding;
the decoded length is checked after decode. Oversize in either check is
a closed audit failure, never falsity. The empty byte string is a valid
subject (its SHA-256 is
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`).

Boundary of the compiled caps: `MAX_INLINE_SUBJECT_BYTES_V0` bounds
only decoded inline subject bytes, and the encoded check protects only
the inline descriptor field; `MAX_PREDICATE_ID_BYTES_V0` bounds only
`predicate_id`. These limits do not bound the full `ClaimAssertedV2`
event, `metadata_json`, the full statement, whitespace, unknown values
before strict rejection, or pre-existing allocation performed by
upstream replay or application code. A true pre-allocation cap for raw
claims or events requires a separate future writer/ingestion/L0
boundary contract and is not ratified here. No resolver can undo
memory already allocated upstream.

### Canonical commitment

The canonical statement of an inline-subject claim has one exact
injective form:

```text
magpie-machine-predicate-inline-bytes-v0:<predicate_id>:<expected_sha256>:<subject_hex>
```

Every field is delimiter-free by construction (the fixed schema
constant, closed-alphabet predicate IDs, and lowercase hex), so the
canonical *encoding* is injective without length framing: one byte
string encodes exactly one descriptor. SHA-256 is not injective. The
binding is established by exact direct comparison — descriptor-derived
statement versus replayed legacy and typed statements — never by hash
equality. `content_hash == SHA-256(exact statement bytes)` is the
existing cryptographic commitment and check; collision resistance is
assumed, and hash equality alone never establishes descriptor or
statement equality. The direct statement comparisons are never omitted.

Statement, metadata, and content hash bind three ways, mirroring the
existing v0 binding law:

```text
StandingClaim.statement
== TypedClaimNode.statement
== the canonical statement derived from the strictly parsed descriptor

ClaimAssertedV2.content_hash
== lowercase hex of SHA-256(UTF-8(exact canonical statement))
```

Because the complete subject bytes appear in the statement itself, the
statement hash commits to the subject, not merely to the expected
digest. The replayed metadata is signed and chained like every other
event field, but metadata alone is not the semantic commitment — the
three-way binding is.

### Versioning boundary

`magpie-machine-predicate-v0`, `sha256_bytes_equals_v0`,
`magpie-verification-witness-v0` and its `witness_hex` field,
`DeterministicVerifierContextTraceV0`, and standing policies v0-v3
remain byte- and behavior-identical. The v0
strict parser rejects the new schema as unknown; the new schema has its
own strict parser that rejects duplicate or unknown keys, non-object or
trailing content, and wrong-typed values. No L0 payload, tag, canonical
encoding, `docs/FORMAT.md`, golden vector, or Python verifier change is
required: claim `metadata_json` is opaque to L0.

Any later predicate identity introduced over this mechanism
(`<predicate_id>` above) must be new and must not reinterpret
`sha256_bytes_equals_v0`.

## Replay resolution law

A future subject resolver — not ratified here — must work exactly this
way:

1. re-fetch the typed claim from the same verified replay snapshot;
2. strictly parse the outer metadata envelope and then the nested
   descriptor: `claim_domain` via the existing strict claim-domain
   parser, required to equal `ExactMachineCheckable`; exactly the two
   outer keys (`claim_domain`, `machine_predicate_inline_bytes`) with
   no other outer keys; duplicate keys rejected at both levels; any
   descriptor field rejected at the outer level; then the nested
   descriptor's closed failures (malformed/non-object/trailing content,
   duplicate or unknown nested key, missing or wrong-typed field,
   unknown schema, missing/wrong-typed/empty `predicate_id`,
   `predicate_id` beyond the compiled
   `MAX_PREDICATE_ID_BYTES_V0 = 64` bound — checked before character
   validation, statement construction, or allocation —
   `predicate_id` characters outside closed `[a-z0-9_]+`,
   `expected_sha256` missing, wrong-typed, or not exactly 64 lowercase
   hexadecimal characters — no prefix, no uppercase, no whitespace —
   validated before canonical statement construction,
   missing/wrong-typed/invalid subject hex, oversize);
3. derive the canonical statement and require exact three-way statement
   equality by direct byte comparison —
   `StandingClaim.statement == TypedClaimNode.statement == the
   descriptor-derived canonical statement` — then separately recompute
   and verify `content_hash` as the SHA-256 of the exact UTF-8
   statement bytes, with collision resistance assumed and hash equality
   never used as identity in place of the direct comparisons;
4. decode the subject from the claim — never from evidence, edges,
   caller arguments, or any other source;
5. only then perform any predicate evaluation, through the one shared
   path for both polarities.

Every prerequisite failure is a closed audit failure with no standing
effect and no negative meaning. Subject resolution by itself grants no
standing.

## Substitution resistance

- An evidence node cannot select the subject: the new family reads
  subject bytes only from the replayed claim. Any alternate subject
  byte source offered to the inline path is rejected by that path.
  Existing v0 witness evidence — including
  `magpie-verification-witness-v0` and `witness_hex` — remains valid
  under its own historical path and is untouched.
- A claim author chooses subject and expected digest when asserting the
  claim; that defines the proposition, exactly as today's authors
  choose a matching witness. It is not substitution after the fact.
- For the new inline-subject path, any future evidence attestation
  consumed by that path may carry routing and binding fields only
  (`schema`, `predicate_id`, `subject_claim_id`, `scope_ref`) and may
  not supply an alternate subject byte source — never `witness_hex` or
  any other byte-carrying field. The `predicate_id` in that schema is
  the already validated claim predicate identity: closed `[a-z0-9_]+`,
  bounded by the compiled `MAX_PREDICATE_ID_BYTES_V0 = 64` constant,
  checked after missing/wrong-type/empty and before character
  validation, canonical statement construction, or allocation. This
  contract defines only that boundary; it ratifies no attestation
  schema and no global restriction on byte-bearing evidence.
- No caller-provided claim bytes, metadata, subject, receipt, or
  evaluation result may enter any resolver. Every **new**
  authority-bearing type introduced for the inline-subject family — any
  future resolved-subject result, receipt, or application type — uses
  private construction, private fields, read-only getters,
  `Serialize`-only, no `Deserialize`, no `Default`, no public
  constructor, and no public mutation. Existing v0-v3
  standing-resolution types (`StandingResolution`,
  `StandingResolutionV1`, `StandingResolutionV2`) keep their frozen
  public fields and are unchanged; where existing public audit values
  are caller-constructible, caller constructibility does not make them
  authority, and no resolver accepts caller-created audit values as
  authority input. Public audit material is never resolver authority.

## What remains standing-inert

- Subject parsing, descriptor binding, and subject resolution itself.
- `ResolutionContentClosureV0`, artifact-provenance verification, and
  origin admission — none is consumed by the inline mechanism.
- No ambient lookup, no second store read, no closure requirement, no
  network, filesystem, CAS, callback, or loader is introduced.

## Candidate architectures considered

Two independent fresh `gpt-5.6-sol` (xHigh) audits compared four
candidate subject architectures; the full comparison and K3's
adjudication are recorded in Ticket 0057. Summary:

| Candidate | Verdict | Reason |
| --- | --- | --- |
| 1. Claim-owned inline immutable subject bytes | **chosen** | Smallest sound mechanism; claim-invariant by construction; decidable in both directions; no L0, closure, or acquisition dependency; covers the entire existing ExactMachineCheckable usage |
| 2. Claim-owned object identity + verified acquisition | bounded future extension | Sound with named additions (compound selector, claim-key/receipt-key equality, injective canonical encoding, one closure-aware internal path), but content-addressed subjects make digest equality near-degenerate; the real proposition becomes acquisition-binding — a different family deserving its own contract |
| 3. Predicate over replay-owned canonical event bytes | bounded future extension | Sound but narrow and currently without substrate (requires a new byte-retaining replay projection); by-content-hash references make digest predicates nearly tautological; in-log subjects only |
| 4. Closure-key content-addressed M-relative predicate | rejected for this contract | Proves only resolution-input-relative facts, no external-object semantics; adds nothing over candidates 1-2 |

Do not select any of 2-4 from this document; they are recorded, not
ratified.

## Bounded future extensions — explicitly unratified

### External acquired-object subject (future contract required)

An external subject must be a compound selector —
`(ArtifactProvenanceAnchorSelectorV0, ResolutionArtifactObjectKeyV0)` —
canonically encoded injectively (no free-string concatenation), with
the acquisition receipt's parsed artifact identity required to equal
the claim-owned artifact key before any evaluation, resolved only
through one crate-private closure-aware internal path. The public
slice-based acquisition verifier remains audit-only and must never
become an authority path. Missing bundles, missing artifacts, root or
digest mismatches, and absent anchors are unavailable — never falsity.
This contract ratifies none of it.

### Replay-owned event-core subject (future contract required)

An in-log subject must be pinned as `EventCore::canonical_bytes()`
under `magpie-core-v1`, selected by verified event identity, and
resolved through a private same-replay byte index that does not exist
today. Caller-constructed events, indexes, or receipts are never
accepted. This contract ratifies none of it.

### Stop conditions

Do not proceed to any direct-refutation contract if a future design
cannot fix all of: injective canonical subject encoding; one
internal-only same-call authority path; missing/malformed/unavailable
never false; no alternate subject byte source consumed by the
inline-subject path (existing v0 evidence formats — including
`magpie-verification-witness-v0` and `witness_hex` — preserved under
their own historical path); brand-new schema/predicate identities
leaving `sha256_bytes_equals_v0` and policies v0-v3 untouched.

## Explicitly unratified future designs

The following become possible over this mechanism but are **not**
ratified here:

- a positive standing rule over inline-subject claims (a new
  `DeterministicVerification × ExactMachineCheckable` family whose
  checker hashes claim-owned bytes);
- a direct-refutation contract over inline-subject claims (predicate
  inequality over the claim-fixed bytes as negative authority);
- an attestation-consumption rule for evidence polarity;
- any policy v4 or later identity, any negative receipt, any conflict
  or precedence rule, any resolver signature, and any runtime.

## Hostile cases

| Case | Expected result under this contract |
| --- | --- |
| an evidence node offers `witness_hex` as an alternate subject to the inline path | rejected by that path — no evaluation; the existing `magpie-verification-witness-v0` format and its `witness_hex` field remain unchanged under their own historical path |
| contradicts edge with well-formed routing | attestation only; never changes the checked bytes |
| claim metadata with duplicate keys | closed parse failure; no standing effect |
| claim metadata with unknown keys or unknown schema | closed parse failure |
| descriptor fields placed at the outer metadata level | closed parse failure — the descriptor is nested beside `claim_domain` only |
| `predicate_id` empty, beyond the compiled `MAX_PREDICATE_ID_BYTES_V0 = 64` bound, or outside closed `[a-z0-9_]+` | closed parse failure — the bound is enforced after missing/wrong-type/empty and before character validation, statement construction, or allocation |
| `expected_sha256` missing, wrong-typed, not exactly 64 lowercase hexadecimal characters, or carrying prefix, uppercase, or whitespace | closed parse failure before canonical statement construction |
| subject hex oversized (encoded or decoded) | closed audit failure; never falsity |
| subject hex invalid (odd length, uppercase, non-hex) | closed audit failure |
| `StandingClaim.statement` differs from the descriptor-derived canonical statement | binding failure — exact direct comparison is required; no evaluation |
| `TypedClaimNode.statement` differs from `StandingClaim.statement` | binding failure — the legacy and typed statements must be byte-equal; no evaluation |
| `ClaimAssertedV2.content_hash` differs from the recomputed SHA-256 of the exact statement bytes | binding failure — the hash is a separate commitment check, never an identity shortcut; no evaluation |
| matching `content_hash` used in place of the direct three-way statement comparison | rejected — hash equality is never identity; collision resistance is assumed, not injectivity |
| caller-created subject-resolution receipt | unconstructible; never accepted |
| cloned or deserialized receipt presented as authority | audit material only; never accepted |
| v0-family claim presented to the inline parser (or vice versa) | unknown schema/predicate — fail closed |
| empty subject bytes | valid subject; evaluation is exact |
| claim author sets expected digest unequal to the subject's digest | the proposition is false — but only a later, separately ratified predicate contract may say so; this contract takes no standing position |
| oversized inline descriptor field (encoded or decoded) | closed resolver failure under the field-specific compiled bound |
| arbitrarily inflated raw event, statement, or `metadata_json` | outside this subject-resolver contract — a true pre-allocation bound requires a separately ratified writer/ingestion/L0 boundary contract; the resolver cannot undo memory already allocated upstream |

## Exact future implementation boundary

A future implementation ticket may add only additive, separately
reviewed surfaces: a new strict claim-metadata parser module, a
snapshot-private subject resolver, private-construction result types,
hostile tests, and documentation. It must not change `sha256_bytes_equals_v0`,
its parser, trace, fixtures, policies v0-v3, L0, `docs/FORMAT.md`,
golden vectors, the Python verifier, Cargo manifests, CI, or any
historical ticket. No standing rule may be implemented in the same
slice as the subject resolver.

## Validation commands (for this documentation change)

```powershell
git diff --check
cargo fmt --all --check
cargo test --workspace --locked
cargo test --doc --locked
cargo run --locked --example tour -p magpie-claims
python tools/check_release_metadata.py
git status --short
git diff --name-only
git diff --stat
```

Because this change is documentation-only, all cargo outputs must remain
identical to the exact base; the tour must remain 1917 bytes with
SHA-256 `48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f`.
The release-metadata checker may refuse the intentionally dirty
worktree; the unchanged script is then run in a temporary VCS-free
mirror. No `--allow-dirty`.

## Reviewer checklist

- Confirm only the authorised documentation paths changed.
- Confirm no predicate, policy, standing rule, refutation, resolver, or
  runtime is ratified or claimed.
- Confirm the outer metadata carries exactly `claim_domain` and the
  nested `machine_predicate_inline_bytes` and no other outer keys,
  duplicate keys are rejected at both levels, no descriptor fields
  appear at the outer level, and `claim_domain` is parsed by the
  existing strict claim-domain parser.
- Confirm the new inline family reads subject bytes only from the
  replayed claim, while existing v0 evidence formats — including
  `magpie-verification-witness-v0`, `witness_hex`, and
  `sha256_bytes_equals_v0` behavior — remain unchanged under their own
  historical path; no global ban on byte-bearing evidence is stated.
- Confirm `predicate_id` is closed `[a-z0-9_]+`, bounded by the
  compiled `MAX_PREDICATE_ID_BYTES_V0 = 64` constant, checked after
  missing/wrong-type/empty and before character validation, statement
  construction, or allocation.
- Confirm `expected_sha256` has one exact representation everywhere:
  exactly 64 lowercase hexadecimal characters, no prefix, no uppercase,
  no alternate encoding, no whitespace, validated before canonical
  statement construction.
- Confirm the compiled caps bound only their specific fields
  (`predicate_id`, the inline descriptor field's decoded bytes) and no
  global event, statement, or metadata size limit is claimed or
  ratified.
- Confirm subject resolution re-fetches the typed claim from the same
  verified replay snapshot, parses the outer envelope before the
  nested descriptor in that order, and decodes the subject from the
  claim alone.
- Confirm no SHA-256 injectivity is claimed anywhere: binding is by
  exact direct statement comparison, with collision resistance assumed
  and direct comparisons never omitted.
- Confirm the canonical statement carries the complete subject
  descriptor; binding is the exact direct three-way comparison
  `StandingClaim.statement == TypedClaimNode.statement == the
  descriptor-derived canonical statement`, with
  `ClaimAssertedV2.content_hash` recomputed separately as the SHA-256
  of the exact statement bytes — collision resistance assumed, hash
  equality never identity.
- Confirm `sha256_bytes_equals_v0`, its parser and trace, policies
  v0-v3, L0, and FORMAT are untouched, and the new schema is
  unknown/fail-closed to v0.
- Confirm missing/malformed/oversized subject material is never
  falsity.
- Confirm private-construction requirements apply to new inline-family
  authority-bearing types only, that existing v0-v3 standing-resolution
  APIs keep their frozen public fields, and that caller
  constructibility never confers authority.
- Confirm the two bounded future extensions are recorded as
  unratified, with their prerequisites and stop conditions.
- Confirm the Ticket 0056 counterexample is answered by construction:
  no evidence node can supply subject bytes to the new inline-subject
  path, while existing v0 evidence formats
  (`magpie-verification-witness-v0`, `witness_hex`) remain unchanged.

## Next slice

The next separately reviewed slice is a predicate contract over this
mechanism (positive and/or negative deterministic evaluation of
inline-subject claims), followed only later by any direct-refutation
contract. This contract ratifies none of those.
