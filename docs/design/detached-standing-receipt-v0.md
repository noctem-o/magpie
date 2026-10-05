# Detached Standing Receipt v0

## Status and purpose

Proposed implementation contract; owner acceptance freezes the identifiers and
rules below. No receipt runtime exists. Define one additive, read-only carrier
for `D = Derive_G(I_G)`, suitable for later CLI or librarian transport without
requiring either. Legacy outputs and audit findings A-003/A-019 remain unchanged.

## Governing doctrine and authority

[ADR-0003](../adr/0003-canonical-definition-of-standing.md) defines standing;
[ADR-0008](../adr/0008-complete-producing-coordinates.md) governs complete
coordinates and exact outcome binding;
[ADR-0009](../adr/0009-verified-supplied-history-and-explicit-checkpoint-expectations.md)
separates verification from expectation;
[ADR-0010](../adr/0010-ed25519-verification-profile-and-external-trust-root-admissibility.md)
governs the selected signature subprofile and external root.
[FORMAT](../FORMAT.md) governs L0 unchanged.

> Magpie runs alone; integrations add evidence, not authority.

Serialization adds no assertion, evidence entitlement, historical truth,
permission, execution authorization, promotion, currentness, admission, or write
capability. Standing describes only the named computation. The
[provenance response](provenance-response-contract.md),
[MCP boundary](mcp-boundary-contract.md),
[librarian](mcp-librarian-contract.md), and
[query](verified-librarian-query-contract.md) contracts govern carriage unchanged;
a query transports a resolver result rather than making a standing decision.

## Closed profile and coordinate classification

`G = magpie-standing-receipt-history-v2-v0` names the immutable composition below:
the complete portable input/verification law, explicit expectation gate, selector
gate, same-vector projection, native v2 derivation, and receipt encoding. It fixes
`magpie-claims-standing-v2`, `magpie-ed25519-canonical-prime-subgroup-v1`, and
`magpie-log-history-expectation-v0`. Callers explicitly select G and the full v2
policy identity; any other selection is refused. These fixed semantic dependencies
are committed through G, not duplicated as variable inputs.

The frozen dependency revision is
`b590a34cede0656b9ee69aeded36d9ed5b68da07`. References below import their exact
normative rules and native type/ordering definitions at that revision, not future
contents of the linked paths. G identifies this complete composition, not that
Git SHA, a mutable URL, crate/build version, or display label. A semantic or
encoding change requires a new profile; code drift under the same G is a defect.

Apply ADR-0008's mutation test: a value that can change or retarget D with the
other inputs fixed belongs in I_G directly or through a checked immutable identity.

| Coordinate | Class and contract |
| --- | --- |
| Exact supplied history | I_G: `{sha256,byte_count}` identifies retained JSONL bytes; SHA-256 and u64 byte count are both checked against those bytes. No normalization. |
| External verifying key | I_G: exact 32 bytes represented by 64 lowercase hex characters. The caller selects the root independently of receipt/genesis; structural admissibility does not establish trust. |
| Verification/replay interpretation | G: the complete portable law below, including first-failure order, canonical core, signature selection, genesis, and same-vector projection. V_sig alone is insufficient. |
| Full standing policy / resolver | G fixes v2 and its internally derived v0/v1 and verifier rules. Selection is explicit; no separate executable identity or inherited detached outcome is an input. |
| History expectation | I_G: mandatory closed `none`, `exact`, or `contains_checkpoint` value; the latter two include the exact expected count/commitment. The fixed expectation semantics are in G. |
| Claim selector | I_G: exact nonempty `claim_id`; no case folding or Unicode normalization. Unique typed assertion gate below makes scope/record selection unambiguous. |
| Verified count/tip; expectation verdict | Derived output from that history and expectation, not additional requested snapshot inputs. Expected checkpoint and actual verified prefix have different roles. |
| Claim event seq/hash/scope; evidence/contribution identities | Derived from bound history; no additional caller selector or duplicate input. Native trace retains the identities it uses; event correspondence can be reconstructed from history. |
| Native outcome, applications, traces, blockers | Derived D, retained without summary replacement or status upgrade. |
| Build/revision, host, package, implementation version | Assurance metadata only; outside the governed receipt. Never selects G. |
| File paths, caches, clocks, environment, process state, resource limits | Operational state; never silent semantics. Inability to complete evaluation produces no D. |

I_G is exactly history identity, external key, expectation, and claim selector.
The supplied bytes are the material behind the identity, not a second independent
input. No optional closure, authority, clock, query, or currentness fields exist.

## Verification, selection, and failure gates

Apply the frozen
[portable input language](portable-verifier-input-language-contract.md),
[frontend composition](portable-verifier-rust-frontend-contract-v0.md), and
[complete history law](portable-verifier-rust-history-conformer-contract-v0.md).
Preflight profile and external key before inspecting history; retain one exact
byte acquisition. The portable verification gate owns parsing, payload validation,
ordered chain/hash/signature/genesis checks, and the verified event vector.
After profile/key preflight, check the supplied history identity before its
portable parsing/verification; then expectation, selector, and resolution, in
that order. Invalid checkpoint syntax is a request refusal, not a history verdict.
Project only that vector, in order, through the existing verified replay boundary.
An unprofiled `LogReader` alone does not satisfy G; an additional unprofiled
acceptance test must not change G's accepted language. A future private
continuation into projection must preserve verified-event capability, not bless
arbitrary deserialized events.

Then apply ADR-0009 over the entire verified vector:

| Expectation | Required relationship |
| --- | --- |
| `none` | No checkpoint comparison; only supplied-history verification is claimed. |
| `exact` | Actual event count and tip equal the supplied checkpoint count and commitment. |
| `contains_checkpoint` | The supplied checkpoint commitment occurs at its exact requested count; the entire supplied history, including any suffix, verifies. |

Count includes genesis seq 0. Count N names the commitment at seq N−1; count 0
names the zero commitment. No remembered/current checkpoint, fallback between
relations, or latest selection participates.

Require exactly one `ClaimAssertedV2` for the requested ID. Repeated typed
assertions are an ambiguous selector refusal; a missing typed assertion is a
selector refusal. A legacy `ClaimAsserted` plus that typed assertion is allowed:
the frozen native fold determines legacy compatibility material. Scope comes
from the unique typed assertion. Other recorded contributions retain the native
fold/eligibility rules; the receipt invents no new evidence adjudication.

Unsupported profile/policy, history identity mismatch, malformed history, wrong
key, hash/link/signature/genesis failure, unsatisfied expectation, or selector
refusal produces **no standing receipt**. Retain the upstream typed reason/stage
in a separate refusal report; do not invent portable failure classes or call the
claim unassessed/refuted. Packet I/O, missing files, interruption, timeout,
allocation failure, and inability to finish are operational failures with no D.
Resource bounds must refuse operationally rather than evaluate a partial history.

## Exact native outcome and typed wire

From the same verified vector, co-derive standing and anchor context as in
[`StandingReplaySnapshot`](../../crates/magpie-claims/src/replay_snapshot.rs),
then internally rederive v0/v1 and the native
[`StandingResolutionV2`](../../crates/magpie-claims/src/standing_v2.rs), including
the same-snapshot
[deterministic verifier](../../crates/magpie-claims/src/deterministic_verifier_context.rs).
The imported v2 semantics include its transitive policy, standing, v1, and
occurrence-context definitions. No caller-supplied inherited result is accepted.

D is the complete native v2 resolution: `claim_id`, nullable governed and legacy
statuses, compatibility `currentness`, full `policy_id`, optional typed
`resolution_failure`, full candidate/application/context traces, and blockers.
Typed resolver failures after established gates belong to D. Preserve them even
if the native failure also retains a status. Failed verifier attempts remain
typed trace outcomes; absence/mismatch is not falsity. `Supported`, inherited
`Settled`, inherited `Refuted`/`Supported`, `Open`/`Conjectured`, and null remain
distinct. Preservation arms do not assert that each inherited status is reachable
through today's resolver. Legacy status is audit-only; currentness stays `unknown`.

Freeze `schema = magpie-standing-receipt-v0` and
`encoding_profile = magpie-standing-receipt-json-v0`. The purpose-built typed
objects have these fields **in declaration order**:

| Object | Fields in order |
| --- | --- |
| Receipt | `schema`, `encoding_profile`, `context`, `context_sha256`, `verified_prefix`, `outcome`, `result_sha256` |
| Context | `profile`, `history`, `verifying_key`, `expectation`, `claim_id` |
| History identity | `sha256`, `byte_count` |
| Expectation | `kind`; for `exact`/`contains_checkpoint`, then `checkpoint` |
| Checkpoint | `event_count`, `commitment_sha256` |
| Verified prefix | `event_count`, `tip_sha256` |
| Outcome | Native v2 fields in the order above; omit `resolution_failure` only when absent, as native v2 does. |
| Result-binding object | `context_sha256`, `verified_prefix`, `outcome` |

Use direct compact `serde_json::to_vec(&typed_value)`-style serialization:
UTF-8, no BOM, whitespace, or trailing newline; u64 decimal counts; lowercase
64-character hex hashes/keys; native enum spellings/field order/omissions at the
frozen revision. Strings use compact JSON escapes for quotes, backslash, and
controls (`\b`, `\t`, `\n`, `\f`, `\r`; other controls as lowercase `\u00xx`);
other Unicode scalars remain UTF-8, without normalization. No generic canonical
JSON, JCS, recursive key sorting, post-serialization sorting, or production
`serde_json::Value` representation is permitted.

Native trace order is the frozen resolver's order: claim diagnostics precede
edge candidates; edge maps iterate by ID; candidate/application arrays keep
native order; blockers retain native enum/BTreeSet order, not alphabetical
display-name order. Any collection added to the typed wire must have a frozen
deterministic order; unordered maps/sets cannot govern bytes.

Let C be the exact typed Context bytes and B the exact Result-binding object
bytes. Freeze:

```text
context_sha256 = hex(SHA256(ASCII("magpie-standing-receipt-context-v0") || 0x00 || C))
result_sha256  = hex(SHA256(ASCII("magpie-standing-receipt-result-v0")  || 0x00 || B))
```

Context identifies the requested computation; result binds that context to
exact D plus derived verified-prefix audit. B contains the computed context hash,
not an independently asserted one. Fixed schema/encoding are committed through G;
the expectation verdict follows from a passed gate, so no duplicate success field
is needed. Additional diagnostic metadata belongs outside this closed receipt.

## Detached acceptance and extension

A produced/checked receipt requires private construction after the above gates.
A detached parser yields a separate inert wire type, never that trusted type.
The strong v0 check independently receives retained explicit inputs, checks their
identities, verifies/replays, rederives D, reconstructs both bindings and complete
receipt bytes, then requires **byte-for-byte equality** with the detached bytes.
Any changed coordinate, status, trace, blocker, prefix, digest, field order,
spelling, missing/extra field, duplicate key, whitespace, or unsupported version
refuses acceptance. Parsing/re-serialization must not normalize differences away.
Self-consistent hashes, familiar policy IDs, and reported build versions are
insufficient. Retained material must be supplied explicitly; identities grant no
ambient filesystem/network/CAS fetch. No result-signing protocol is introduced.

An exact/contains comparison establishes only that relationship: no currentness,
freshness, global completeness, non-equivocation, truth, independent origins,
source authenticity beyond the selected verification law, or host authority.
Future closure-consuming v4 receipts require another closed G/I_G variant;
currentness remains a separate
[ADR-0006](../adr/0006-claim-currency-and-currentness-semantics.md) facet unless reviewed composition says
otherwise. This contract changes no L0 encoding, payload, golden vector, legacy
result, runtime, CLI/MCP/API, writer, admission, or acquisition system.

## Normative encoding fixture and reviewer checks

The following ASCII-only fixture is **encoding-only**: dummy history/key/tip
values and an empty native trace are specification inputs, not evidence of an
actual history or reachable standing computation. It cannot pass the production
gates. The single line between the fences has no terminating newline as encoded.

```json
{"schema":"magpie-standing-receipt-v0","encoding_profile":"magpie-standing-receipt-json-v0","context":{"profile":"magpie-standing-receipt-history-v2-v0","history":{"sha256":"0000000000000000000000000000000000000000000000000000000000000000","byte_count":1},"verifying_key":"1111111111111111111111111111111111111111111111111111111111111111","expectation":{"kind":"exact","checkpoint":{"event_count":2,"commitment_sha256":"2222222222222222222222222222222222222222222222222222222222222222"}},"claim_id":"claim-fixture"},"context_sha256":"5026700c2a766d6b1f434552c6ba3c4759180d86475be872ed13c69916b161cb","verified_prefix":{"event_count":2,"tip_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"outcome":{"claim_id":"claim-fixture","governed_standing":"Conjectured","legacy_raw_standing":"Conjectured","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[],"blockers":[]},"result_sha256":"3d12b224a3fffbfa3da2047d2ffe78bb266275cae4731a3f1df2bcc33d96bcff"}
```

`C`: 413 bytes; `B`: 399 bytes; receipt: 997 bytes.
Expected context: `5026700c2a766d6b1f434552c6ba3c4759180d86475be872ed13c69916b161cb`.
Expected result: `3d12b224a3fffbfa3da2047d2ffe78bb266275cae4731a3f1df2bcc33d96bcff`.

| Hostile case | Required check / bounded meaning |
| --- | --- |
| Same claim/Supported, different histories or selector | Different context/result; never cache by status/claim alone. |
| Same history, different external key | Distinct request or verification refusal; never take trust from the receipt. |
| Same history/key, none/exact/contains or different checkpoint | Distinct context; no relation fallback. |
| Valid history truncated before exact checkpoint | Expectation refusal, no receipt. |
| Verified extension beyond contains checkpoint | Accepted only if the full suffix verifies; bind its actual prefix and new history identity. |
| Same context with fabricated stronger D; internally consistent forgery | Trusted rederivation and exact byte match refuse; hashing alone cannot authenticate computation. |
| v1 Settled or inherited Refuted through v2 | Retain native preservation/application material, not a simplified v2 ceiling. |
| Parsed receipt without history; packet failure; unsupported G | Inert data or explicit refusal/operational failure; no normal receipt. |
| Build drift under unchanged G | Different bytes/outcomes are non-conformance, not legitimate new semantics. |
| Supported treated as truth/permission | Consumer misuse; no authority-bearing effect is defined here. |

Later tests must cover exact bytes above, all expectation variants, Unicode/control
escaping, native success/failure/application vectors, field/domain/coordinate
mutations, private construction, detached forgery, and deletion of all derived
state followed by identical rederivation. Owner acceptance of this proposed
profile/encoding is the remaining design gate; implementation and conformance
evidence remain separate. Audit closure is not claimed.
