# Portable Verifier Input-Language Contract

## Status

Narrow documentation-only implementation contract for A-004 / RQ-006.

Resolved baseline:

```text
repository: noctem-o/magpie
worktree: C:\Users\herpe\.codex\worktrees\magpie-portable-verifier-language-contract
branch: agent/portable-verifier-language-contract
HEAD: f6a4759ef3cdc2a3974aad4e2be57df58d04c1c3
origin/main: f6a4759ef3cdc2a3974aad4e2be57df58d04c1c3
live refs/heads/main: f6a4759ef3cdc2a3974aad4e2be57df58d04c1c3
merge-base(HEAD, origin/main): f6a4759ef3cdc2a3974aad4e2be57df58d04c1c3
```

This contract adds no parser, verifier, Go module, fixture, test, CI step,
canonicalization profile, public Rust type or runtime behavior. It freezes the
decision surface for a later implementation PR.

A-004 and RQ-006 remain **Confirmed** until all of the following have happened:

1. this contract is owner-merged;
2. a dedicated A-021 contract defines the exact Ed25519
   signature-verification relation and is owner-ratified;
3. Rust, independently implemented Go and independently maintained Python
   implement that relation alongside this portable input-language contract;
4. the exact-byte conformance corpus covers every portable-language case and
   the A-021 signature-semantic vectors;
5. all three conformers demonstrate equivalent governed verdicts over that
   complete normative corpus;
6. the exact frozen histories remain correctly handled under the ratified
   compatibility law;
7. an independent hostile review completes;
8. the human owner merges the implementation tranche or tranches; and
9. the living audit ledger is reconciled separately.

Grammar alignment, green ordinary-key fixtures, canonical point decoding and
the N1-N30 matrix are not sufficient by themselves for full A-004/RQ-006
closure. A-021 is a hard prerequisite because it owns the exact Ed25519
signature-verification relation, not merely weak-root admissibility.

The future corpus will also improve the evidence owned by RQ-013, but it will
not close RQ-013's separate resource, crash and exact-boundary assurance gaps.

## Purpose

Define one portable language for Magpie's reference JSONL verification
interface and one cross-language verdict law over its governed conformance
domain:

```text
                         reviewed Magpie contract
                                   |
                      portable verifier language
                                   |
                 +-----------------+-----------------+
                 |                 |                 |
              Rust               Go              Python
           production        independent         readable
           conformer       oracle candidate      reference
                 |                 |                 |
                 +-----------------+-----------------+
                                   |
                         one exact-byte corpus
                                   |
                    equivalent governed verdicts
```

No implementation defines this language. Rust's `serde_json`, Python's
`json`, and Go's future `encoding/json` are implementation tools whose defaults
must be constrained where they differ from this contract.

The portable invariant is deliberately conditional.

For the current governed conformance domain:

```text
same exact input bytes
+ same canonical representation-valid external verifying key
+ a normative corpus case whose expected result does not depend on the
  unresolved A-021 signature-verification relation
+ this reviewed contract and corpus version
= same ACCEPT/REJECT verdict
+ same first-failure class
+ same failure coordinate where one is defined
+ same successful count, tip and ordered hashes
```

That domain is defined case-by-case by the normative corpus, not by a runtime
key or signature classifier. Representation-invalid external keys are governed
by `ExternalKey`. The exact committed golden and Deadbolt histories remain
normative positive compatibility cases: their exact signatures MUST continue
to be accepted. Acceptance of one exact signature under a key does not settle
the verification relation for other signatures under that key. In particular,
the ordinary golden key does **not** make every signature that names it
A-021-neutral.

For any semantically well-shaped 64-byte signature whose ACCEPT/REJECT outcome
depends on the unresolved Ed25519 verification semantics, this contract does
**not** yet define a portable result. It does not silently inherit Rust,
Python or Go behavior and does not label either ordinary or strict verification
non-conforming. This is a limitation of the reviewed contract's proof domain,
not a third runtime result: conformer result vocabulary for governed cases
remains only `ACCEPT` or `REJECT`.

This contract does not claim to classify every possible signature as
A-021-neutral or A-021-sensitive. A21-D1 and A21-D2 are dependency witnesses,
not an exhaustive definition of the disputed domain. A complete classifier
would itself require the owner-ratified A-021 verification relation.

Language-specific diagnostic prose, stack traces and exception names are not
part of the invariant.

## Audited scope

### A-004

The preserved architecture audit found that Rust and Python accepted different
serialized record languages and could report different first failures. It
identified mixed-case hex, blank records, empty input, numeric status,
non-finite JSON, duplicate members and payload-versus-cryptographic ordering as
concrete examples. Its broader requirement was one explicit input grammar and
one differential suite rather than case-by-case parser patches.

### RQ-006

The preserved runtime-quality audit independently found the same operational
interoperability defect: the Python reference verifier and Rust production path
did not agree on all accepted inputs or failure ordering. RQ-006 requires
negative differential vectors with stable expected verdicts, classes and
coordinates.

### RQ-013 relationship

An exact malformed and multi-defect corpus materially improves RQ-013's
cross-language assurance. RQ-013 also covers bounded-resource behavior,
crashes and other proof surfaces that this contract deliberately does not
specify. It remains separate and open.

## Non-canonical transport distinction

Magpie canonical identity remains exactly:

```text
typed EventCore semantics
        |
        v
magpie-core-v1 canonical byte encoder
        |
        +--> SHA-256 ContentHash
        |
        `--> Ed25519 over "magpie-sig-v1" || ContentHash
```

The reference transport path is different:

```text
JSONL file bytes
        |
        v
portable input parser
        |
        v
typed SignedEvent
        |
        v
the existing magpie-core-v1 canonical byte encoder
```

The JSONL bytes are not the hash or signature preimage. Object-member order,
permitted JSON whitespace and permitted string escapes may therefore differ
without changing a record's typed value or canonical identity. This contract
does not redefine `magpie-core-v1` as canonical JSON, create a second
canonicalization profile or change any persisted event representation.

`docs/FORMAT.md` correctly makes the reference JSON-lines storage replaceable
relative to signed canonical identity. Replaceability does not permit each
shipped verifier to invent a different language for the named reference JSONL
interface. The contract governs that interface only.

## Evidence and discovery method

The current behavior inventory was reconstructed from:

- the complete historical A-004, RQ-006 and RQ-013 findings;
- `docs/FORMAT.md`, the tagged v0.1.0 release contract, release tickets and
  frozen-fixture commitments;
- the Rust `FileStore`, serde boundary, payload validator and complete
  `parse_and_verify_records` path;
- the complete Python `tools/verify_chain.py` path;
- all current `magpie-log` golden, chain and portable-anchor tests; and
- exact-byte probes against both real verifier paths.

The discovery run used the nine-record golden fixture as the base, changed one
transport dimension at a time, and sent the resulting bytes directly to both
verifiers. Duplicate-key, malformed-JSON, invalid-UTF-8 and framing cases were
not parsed and reserialized before verification. The run covered 157 ordinary
mutations, 13 correctly hashed/signed genesis and semantic-precedence cases,
and external-key lexical probes. All probe material lived under ignored
`target/` storage and is not part of this contract PR.

Current source control flow corroborates the probes:

- Rust `FileStore` splits at LF and drops every zero-length slice;
- Rust deserializes an entire `SignedEvent` before checking sequence, link,
  content hash, signature, payload validity and genesis, in that order;
- Python opens in text mode, so universal-newline translation occurs before
  `json.loads`;
- Python accepts numeric status values and lowercases the CLI trust key;
- Python performs payload validation while constructing canonical bytes; and
- Python continues after failures, while Rust returns its first error.

The first review-remediation probe against locked `ed25519-dalek` 2.2.0
established that
`0200000000000000000000000000000000000000000000000000000000000000`
fails `VerifyingKey::from_bytes`, while the all-zero encoding is representable
and reported weak and the golden fixture key is representable and non-weak.
The current Python `cryptography` constructor accepts the representation-invalid
`02...00` bytes. This proves that representation decoding and weak-key policy
are separate decisions and that length/hex checks alone cannot govern portable
entry into verification.

The second review pass exposed a finer representation difference. The locked
`ed25519-dalek` source says `VerifyingKey::from_bytes` uses ZIP-215 point
validation rather than RFC 8032/NIST criteria. A throwaway probe confirmed that
it accepts the non-canonical encodings `y = p`
(`edff...ff7f`), `y = p + 1` (`eeff...ff7f`), and the identity point encoded
with sign bit 1 (`01` followed by zero bytes and a final `80`). RFC 8032
section 5.1.3 rejects those encodings. The portable representation law below
therefore cannot be defined as “whatever `VerifyingKey::from_bytes` accepts.”
The temporary probes were not retained.

The third review pass exposed that A-021 cannot be scoped by external-key
properties. Under the ordinary golden key, a legitimate keyholder can construct
a signature with identity-point `R` (`01` followed by 31 zero bytes) and
canonical `S = H(R || A || M) * a mod l`. The ordinary equation holds, while
locked-dalek `verify_strict` rejects the small-order `R`. Locked-dalek also
documents canonical `S < l` parsing for its normal build, so this witness is not
merely malformed signature transport or an out-of-range scalar. It demonstrates
that the unresolved boundary is the entire verification relation. No probe or
fixture is added here.

## Current implementations and future roles

### Rust

Rust remains the production and in-process Magpie verifier. It is one future
conformer. Its current serde and file-splitting behavior is evidence, not law.

### Go

Go is selected for the future independent verifier and release-oracle
candidate. It will be a small standalone local command, preferably
standard-library-only, with no network access and no dependency on either
existing implementation. It cannot be described as a universally equivalent
release oracle for arbitrary well-shaped signatures until A-021's complete
verification relation is owner-ratified and implemented. Its role is frozen
below; no Go code or module exists in this PR.

### Python

`tools/verify_chain.py` remains the small, readable independent reference
implementation. It is valuable for inspection, debugging and implementation
diversity. Python parser defaults are not law, and RQ-015's dependency-pinning
question remains separate.

## Current divergence inventory

“Writer form” means the current `LogWriter`/serde spelling. “Preserve” means no
currently documented valid frozen history is narrowed. “Narrow” or “broaden”
describes only the future reference-input language; it is not a
`magpie-core-v1` byte change.

| Family | Exact input mutation | Rust now | Python now | Writer-emitted form | Historical / FORMAT evidence | Selected future law | Compatibility consequence | Future vector |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| framing | untouched nine-record golden chain | ACCEPT, count 9 | ACCEPT, 9 OK | compact JSON plus LF | frozen golden and release contract require acceptance | ACCEPT, count 9 and frozen tip | preserve | P1, P3-P6 |
| framing | valid genesis record only | ACCEPT, count 1 | ACCEPT | compact JSON plus LF | a one-record genesis chain satisfies FORMAT | ACCEPT | preserve | positive framing |
| framing | zero-byte file | ACCEPT, count 0, zero tip | REJECT “no genesis” | an uninitialized store may be empty; a writer creates genesis on open | FORMAT says record 0 of any **non-empty** chain is Genesis; Rust pins empty verified identity | ACCEPT as an empty snapshot, count 0, zero tip; no genesis/key-binding claim | Python broadens; distinction becomes explicit | N7, empty positive |
| framing | omit final LF | ACCEPT | ACCEPT | writer emits LF | no historical promise requires a final terminator | ACCEPT | preserve | N30/P9 |
| framing | exactly one final LF or CRLF | ACCEPT | ACCEPT | LF | ordinary JSONL transport; frozen fixtures use LF | ACCEPT | preserve | N30/P9 |
| framing | second or later final LF | ACCEPT because all empty slices are dropped | REJECT blank record | never emitted | no compatibility promise | REJECT `Framing` | Rust narrows | N30 |
| framing | interior zero-length line | ACCEPT because it is dropped | REJECT parse | never emitted | FORMAT describes one event per record, not ignorable holes | REJECT `Framing`; physical line present, `record_index` absent | Rust narrows | N6 |
| framing | non-empty whitespace-only line | REJECT serde | REJECT parse | never emitted | not a JSON record, but physically non-empty | REJECT `Framing`; physical line and next candidate `record_index` present | class and coordinates become portable | N6 |
| framing | CRLF throughout | ACCEPT | ACCEPT | LF | cross-platform reference transport may preserve ordinary line endings | ACCEPT | preserve | N30/P9 |
| framing | mixed LF and CRLF record terminators | ACCEPT | ACCEPT | LF | no identity consequence | ACCEPT per record | preserve | N30/P9 |
| framing | lone CR within a first or later non-zero-length candidate, including terminal CR before EOF | REJECT trailing data | Python text mode may translate CR to a newline | never emitted | lone CR is not selected as a record terminator | REJECT `Framing` with that candidate's physical line and already-assigned `record_index`; the no-index ambiguity allowance never applies | Python narrows; coordinates become portable | N30 |
| framing | leading/trailing space or tab around one JSON object | ACCEPT | ACCEPT | none | insignificant JSON whitespace does not enter the canonical preimage | ACCEPT | preserve | P8 |
| framing | exact `ef bb bf` at byte offset zero of the first or any later non-zero-length candidate | REJECT parse | REJECT parse | none | no per-record BOM is emitted or promised | REJECT `Framing` with that candidate's line and `record_index` | preserve verdict, stabilize position/class/coordinates | N13/BOM-1/BOM-2 |
| JSON lexical | permitted space/tab followed by literal U+FEFF outside a string | REJECT parse | REJECT parse | none | U+FEFF is not RFC 8259 whitespace | because the bytes are not at candidate offset zero, normal JSON grammar applies: `REJECT(JsonSyntax)` with candidate coordinates | stabilize stage boundary | N13/BOM-3 |
| JSON string | literal UTF-8 U+FEFF or escaped `\uFEFF` inside string content | ACCEPT when the resulting event is otherwise valid | ACCEPT when otherwise valid | writer may emit the scalar literally or escaped through an equivalent JSON spelling | canonical strings preserve decoded UTF-8 scalar content without normalization | the BOM rule does not reject; the complete normative case receives its ordinary result | preserve semantic string language | BOM-4/BOM-5 |
| framing | invalid UTF-8 byte | REJECT serde | uncaught `UnicodeDecodeError` traceback | valid UTF-8 | canonical strings are UTF-8 | REJECT `Framing`; no crash | Python gains governed failure | N12 |
| framing | NUL byte inside a framed record | REJECT | REJECT | none | NUL is not valid unescaped JSON content | REJECT `JsonSyntax` | preserve verdict, stabilize class | framing negative |
| framing | two JSON values on one line or trailing garbage | REJECT trailing characters | REJECT extra data | one object per line | reference storage is one event per record | REJECT `JsonSyntax` | preserve | N11/N30 |
| object grammar | duplicate any required known member, including every member of every payload variant | REJECT duplicate field where serde reaches that shape | ACCEPT last-wins | exactly one member | no duplicate spelling was promised | duplicate each required member coordinate individually: `REJECT(Schema)`; escaped-name equivalence is proved separately | Python narrows; every schema branch is covered | N8 |
| object grammar | duplicate unknown member | ACCEPT, ignored | ACCEPT, last-wins/ignored | no unknown member | closed record grammar is safer than coincidentally shared permissiveness | REJECT `Schema` | both narrow | N9 |
| object grammar | one unknown member in `SignedEvent`, `EventCore`, `Provenance`, or each individual payload variant | ACCEPT, ignored | ACCEPT, ignored | exact known members only | payload variants and format fields are explicitly enumerated | REJECT `Schema`; N10 covers all three non-payload shapes and each of the nine payload variants independently | both narrow | N10 |
| object grammar | arbitrary object-member ordering | ACCEPT | ACCEPT | deterministic serde order | JSONL bytes are not canonical identity | ACCEPT | preserve | P7 |
| JSON lexical | `NaN`, `Infinity`, or `-Infinity` in an ignored unknown member | REJECT JSON token | ACCEPT then ignore | never emitted | these are not RFC 8259 JSON numbers | REJECT `JsonSyntax` everywhere | Python narrows | N14 |
| JSON lexical | equivalent JSON Unicode escape, for example `g\u0065orge` | ACCEPT and verifies | ACCEPT and verifies | printable ASCII is emitted literally | typed decoded UTF-8, not JSON token bytes, enters canonical encoding | ACCEPT equivalent decoded scalar value | preserve | Unicode positive |
| JSON lexical | unpaired high or low surrogate escape | REJECT | REJECT during UTF-8 canonicalization | never emitted | canonical strings require valid UTF-8 scalar values | REJECT `Schema` | preserve verdict, stabilize class | Unicode negative |
| JSON shape | array, empty object, null or wrong primitive where an object is required | REJECT schema | REJECT, sometimes after sequence inspection | objects | explicit SignedEvent/EventCore/Provenance/Payload shapes | REJECT `Schema` before chain checks | stabilize Python ordering | N17/N18 |
| stored hex | uppercase or mixed-case `SignedEvent.hash` | ACCEPT; decoded bytes compare equal | REJECT string comparison | 64 lowercase hex | FORMAT reference spelling and fixtures are lowercase | REJECT `Schema` | Rust narrows | N1 |
| stored hex | uppercase or mixed-case `signature` | ACCEPT | REJECT strict lowercase decoder | 128 lowercase hex | FORMAT and fixtures are lowercase | REJECT `Schema` | Rust narrows | N2 |
| stored hex | uppercase or mixed-case `core.prev_hash` | ACCEPT | REJECT strict lowercase decoder | 64 lowercase hex | FORMAT and fixtures are lowercase | REJECT `Schema` | Rust narrows | N1 family |
| stored hex | wrong/odd length, non-hex, empty, whitespace, `0x`, Unicode lookalike | REJECT serde | REJECT at the relevant check | exact lowercase, no decoration | fixed byte widths in FORMAT | REJECT `Schema` | preserve verdict, stabilize class | N3 |
| payload hex | uppercase/wrong-length witness root or non-empty typed `content_hash` | Rust can reach hash mismatch before payload validation | Python payload canonicalization rejects first | lowercase exact width; typed content hash may be empty | FORMAT tags 5-8 and Ticket 0010 pin these rules | string is decoded as a field; governed lexical validity is `PayloadValidation` after signature | aligns precedence with FORMAT/Rust chain order | N1/N3/N28 |
| genesis key | changed uppercase or malformed genesis-declared key | typed string; hash/signature can fail first, then key binding | payload canonicalization accepts string; hash/signature can fail first, then genesis equality | 64 lowercase hex | FORMAT genesis spelling and key binding | after prior checks, require 64 lowercase hex and equality; failures are `Genesis` | explicit law | N3/N27 |
| external key | uppercase/mixed-case CLI key | not applicable: Rust production receives typed key bytes, not CLI text | Python lowercases then accepts | release commands use lowercase | external trust selection is separate from lexical transport | portable CLI requires exactly 64 lowercase ASCII hex; invalid input is `ExternalKey` | Python CLI narrows; no A-021 decision | K1-K3 |
| external key | exact lowercase 64 hex whose 32 bytes do not encode an RFC 8032 Edwards25519 point, for example `0200000000000000000000000000000000000000000000000000000000000000` | `ed25519_dalek::VerifyingKey::from_bytes` rejects this example before `LogReader::open` | the current `cryptography` constructor accepts this 32-byte value; zero-byte input then reaches Python's separate no-genesis rejection | writer/release fixture keys are canonically representable | the portable interface needs one explicit representation law; trust in that key remains external | `REJECT(ExternalKey)` before framing or empty-snapshot handling | makes representation validity portable without deciding weak-key policy | K4-K5, K11 |
| external key | non-canonical compressed-point encodings: `y >= p` or recovered `x = 0` with sign bit 1 | locked `ed25519-dalek` 2.2.0 ZIP-215 decoding accepts the probed boundary forms | host-library acceptance is not a portable promise | never writer-emitted; frozen fixture keys are canonical | RFC 8032 section 5.1.3 defines canonical decoding independently of host reduction behavior | `REJECT(ExternalKey)` before framing; require exact boundary vectors and canonical re-encoding | narrows incidental Rust constructor language; no canonical event-byte change | K9-K15 |
| signature semantics | known canonical low-order root `ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f` plus attacker-constructed signatures | current Rust uses ordinary verification, which accepts the audited construction | current Python experiment accepted the forged chain; a future strict conformer may reject | not an ordinary fixture root | preserved A-021 proves ordinary/strict divergence and owns the unresolved policy | mandatory dependency sentinel with no portable verdict until A-021 is ratified | prevents false universal-equivalence claim; no runtime third result | A21-D1 |
| signature semantics | ordinary golden external key plus a keyholder-constructed signature whose `R` is the identity point and whose canonical `S` satisfies the ordinary equation | current Rust ordinary `verify` accepts the equation; locked `verify_strict` rejects small-order `R` | Go `crypto/ed25519` ordinary verification can accept; no implementation default is normative | exact frozen golden signatures remain valid, but this is a different signature under the same key | fresh hostile review proves strictness sensitivity is not confined to unusual roots | mandatory dependency sentinel with no portable verdict until A-021 defines the complete verification relation | prevents treating an ordinary key as a settled signature domain | A21-D2 |
| status | exact `Open`, `Conjectured`, `Supported`, `Settled`, `Refuted` strings | accepted as enum; changed signed value later fails hash | accepted | named string | serde writer and frozen fixture use names; FORMAT numeric tags describe canonical bytes | exact case-sensitive strings only; every value appears in a normative accepted case | preserve writer language and prove no strict subset | vocabulary positives |
| status | integers 0-4, especially `1` replacing `"Conjectured"` | REJECT schema | accepts; `1` verifies unchanged canonical bytes | named string | no release promise of numeric JSON status | REJECT `Schema` | Python narrows | N4 |
| status | unknown/lowercase/mixed string, out-of-range/negative integer, float, Boolean or null | REJECT | REJECT, with varying parse/canonicalization paths | exact named string | closed status vocabulary | REJECT `Schema` | stabilize class | N5 |
| payload kind | unknown/case-changed/numeric/missing/null `payload.kind` | REJECT serde | REJECT canonicalization | exact case-sensitive name | FORMAT closes tags 0-8; Ticket 0005 forbids fallback | REJECT `Schema` | preserve | N19 |
| closed strings | exact accepted actor class, evidence kind, edge kind and payload discriminator values | accepted on the matching Rust branch | accepted | exact closed value | FORMAT and Ticket 0010 enumerate the complete finite vocabularies | every accepted value appears in at least one normative accepted case, with machine-checked coverage | prove no conformer implements a strict subset | vocabulary positives |
| closed strings | unknown/case-changed/spaced actor class, evidence kind or edge kind | type is string; unsigned mutation reaches hash mismatch before Rust validation | Python canonicalization rejects before hash | exact closed value | FORMAT and Ticket 0010 enumerate the vocabularies | exact decoded value, no trimming; reject as `PayloadValidation` after crypto | Python ordering changes | N20/N28 |
| integers | `seq` or `timestamp_nanos` token `0` through `18446744073709551615` | accepted as u64, then chain/hash rules apply | accepted as Python int, then chain/hash rules apply | unsigned decimal | FORMAT uses u64 | accept the complete lexical/range domain without floating-point coercion; accepted `timestamp_nanos` cases cover `2^53 + 1`, `2^63` and `u64::MAX`, while in-range upper `seq` cases reach `Sequence`, not `Schema` | preserve and prove full-width handling | U64-P1 through U64-P5 |
| integers | leading plus (`+1`, `+0`) or a multi-digit leading-zero spelling (`01`, `00`) in either u64 field | REJECT JSON syntax | REJECT JSON syntax | never emitted | these are not RFC 8259 number tokens | REJECT `JsonSyntax` before schema interpretation | makes lexical boundary explicit | N11 |
| integers | syntactically valid negative integer, `-0`, or `u64::MAX + 1` | REJECT serde; `-0` is not accepted as u64 | Python accepts the JSON number and may reach sequence/range handling | never emitted | canonical encoder requires u64 | REJECT `Schema` | stabilize Python ordering | N16 |
| integers | syntactically valid fraction or exponent (`1.0`, `0.0`, `1e0`, `1E0`, `1e+0`, `1e-0`) | REJECT serde | Python accepts the JSON number and may coerce or inspect it later | never emitted | writer emits plain unsigned decimal; canonical type is u64 | REJECT `Schema` | Python narrows and classes align | N15 |
| integers | Boolean, null or quoted digits | REJECT serde | Python may inspect the decoded value during sequence handling | never emitted | syntactically valid JSON values, but not u64 number tokens | REJECT `Schema` | stabilize type boundary | N16 |
| fields | any required member omitted or replaced by null | REJECT serde | REJECT, sometimes only after sequence or canonicalization | all fields emitted | Rust types contain no defaults or optional structural fields | REJECT `Schema`; all variant fields are required exactly once and non-null | stabilize | N17/N18 |
| field emptiness | empty legacy/provenance/tag-5 non-root string | accepted if chain is correctly signed and other rules pass | accepted | writer permits it | FORMAT does not add non-empty rules for these fields | permit where not explicitly forbidden | preserve | payload positives |
| field emptiness | empty required tag 6-8 ID, statement, summary, scope or rationale | Rust checks after signature | Python checks while canonicalizing | non-empty | FORMAT/Ticket 0010 explicitly require non-empty | REJECT `PayloadValidation` after crypto | Python ordering changes | payload negative/N28 |
| metadata | empty or non-JSON `metadata_json` string | accepted as opaque signed text | accepted as opaque signed text | string, often JSON-looking | FORMAT/Ticket 0010 explicitly say L0 does not parse it | accept any valid JSON string value, including empty/non-JSON contents | preserve | payload positive |
| chain | wrong sequence plus ordinary later defects | `Sequence` first | sequence reason retained | sequential | FORMAT and Ticket 0001 put sequence first | `Sequence` | preserve | N21/N28 |
| chain | wrong previous link plus invalid payload | `PreviousLink` first | previous-link reason retained | linked | FORMAT puts previous link second | `PreviousLink` | preserve | N22/N28 |
| chain | wrong stored hash | content-hash mismatch | hash mismatch | correct hash | FORMAT puts recomputed hash before signature | `ContentHash` | preserve | N23 |
| chain | well-shaped invalid signature | bad signature | bad signature | valid Ed25519 | FORMAT puts signature after content hash | `Signature` | preserve | N24 |
| precedence | correctly hashed invalid payload plus bad stored hash | `ContentHash` | payload canonicalization failure | never emitted | FORMAT crypto order plus Rust's explicit post-signature payload validation | `ContentHash` before `PayloadValidation` | Python changes | N28 |
| precedence | correctly hashed invalid payload plus bad signature | `Signature` | payload canonicalization failure | never emitted | same evidence | `Signature` before `PayloadValidation` | Python changes | N28 |
| genesis | correctly signed non-Genesis at zero, late Genesis, wrong profile or key binding | genesis-specific failure after crypto | same semantic failure after crypto | one valid Genesis at zero | FORMAT §5 and frozen genesis | `Genesis`; position, profile, then key binding | preserve | N25-N27 |
| precedence | bad signature plus wrong genesis profile/key | `Signature` | `Signature` | never emitted | signature precedes genesis in FORMAT/Ticket 0001 | `Signature` | preserve | N28 |
| later record | valid prefix plus malformed final record | REJECT on final serde parse; no physical coordinate in `LogError` | REJECT at physical final line, then may continue | all valid | whole input must verify | first failure at final line/record; no ACCEPT result | add portable coordinate | N29 |
| reporting | multiple bad records | Rust returns first error | Python prints later errors too and exits non-zero | n/a | audits require governed first failure, not identical prose | verdict binds the earliest governed failure only; extra diagnostics may not replace it | both expose stable first result | N28 |

The table records shared permissiveness as well as disagreements. Rejecting
duplicate unknown names and unknown members is intentional even though both
current implementations accept them: neither writer output, FORMAT nor the
v0.1.0 release evidence promised that extension surface.

## Normative input-language law

The words MUST, MUST NOT, SHOULD and MAY in this section govern the future
portable/reference JSONL verification interface.

### File and record framing

1. Before inspecting the input snapshot, a conformer MUST complete the
   portable external-key gate defined below. `ExternalKey` failure wins even
   for a zero-length input and has no line or record coordinate.
2. Input is an exact byte sequence. A conformer MUST inspect the original
   bytes; text-mode newline translation is not permitted before framing.
3. After the supplied key passes that complete gate, the zero-length byte
   sequence is an accepted empty snapshot. Its successful result is
   `event_count = 0` and the all-zero 32-byte tip. It is not a claim that
   genesis or supplied-key binding occurred.
4. Any non-empty input contains one or more non-empty record texts.
5. A record terminator is LF (`0a`) or CRLF (`0d 0a`). Either may be used for
   any record; mixed LF/CRLF input is permitted.
6. The last record terminator is optional. A second terminator after the last
   record creates an empty record and MUST be rejected. Thus zero bytes is the
   only empty-snapshot spelling; LF alone is an empty record, not an alias.
7. Lone CR is never a terminator or insignificant whitespace. A lone CR within
   otherwise non-zero-length bytes belongs to the current framed record
   candidate, which has already received its zero-based `record_index`. It MUST
   be rejected as `Framing` with that candidate's physical `line` and
   `record_index`, whether it occurs inside a first candidate, inside a later
   candidate, or after JSON bytes immediately before EOF. Lone CR never uses
   the no-index ambiguity allowance. The two JSON bytes `5c 72` spelling the
   escape `\\r` inside a string are not a raw CR and are unaffected.
8. A zero-length record text and a non-empty record text containing only space
   and/or horizontal tab are distinct cases, but both MUST be rejected as
   `Framing`. The coordinate law below assigns no `record_index` to the
   zero-length slice and assigns the next candidate index to the physically
   non-empty whitespace-only slice.
9. Space (`20`) and horizontal tab (`09`) MAY appear as insignificant JSON
   whitespace before, after or within the one JSON value. CR is permitted only
   as the first byte of a CRLF terminator; LF is always framing.
10. Each record text MUST contain exactly one complete JSON value and no
   non-whitespace trailing bytes.
11. The exact bytes `ef bb bf` at candidate byte offsets 0 through 2 are a
    forbidden per-record UTF-8 BOM and MUST produce `REJECT(Framing)` with the
    non-zero candidate's normal `line` and `record_index`. This applies to the
    first candidate and to every later candidate after an LF or CRLF
    terminator; it is not merely a file-start rule. The same valid UTF-8 bytes
    occurring inside JSON string content, or the escape `\uFEFF`, encode an
    ordinary U+FEFF scalar and MUST NOT fail merely because of the BOM rule.
    If `ef bb bf` occurs outside a string at any candidate offset other than
    zero—for example after permitted space/tab bytes—it is not a framing BOM;
    U+FEFF is not RFC 8259 whitespace, so ordinary JSON grammar produces
    `REJECT(JsonSyntax)` when no earlier defect exists. NUL is neither
    whitespace nor a terminator.
12. Every record reached in physical order MUST be valid UTF-8. Invalid UTF-8
    MUST produce governed `REJECT(Framing)`, never replacement characters, a
    traceback or a crash. A later invalid byte does not preempt an earlier
    record's governed failure.

This law deliberately accepts both common line endings, optional final
termination and ordinary horizontal JSON whitespace. It deliberately rejects
blank-record elision and Python universal-newline aliases.

### JSON lexical and object grammar

Each record value MUST be a JSON object in the RFC 8259 grammar, with these
additional closed-language rules:

- `NaN`, `Infinity` and `-Infinity` are forbidden in every location, including
  an otherwise unknown member.
- Numeric-looking spellings outside the RFC 8259 JSON number grammar are
  `JsonSyntax`, before typed/schema interpretation. This includes every leading
  plus spelling (`+1`, `+0`) and every multi-digit leading-zero spelling such as
  `01` or `00`.
- Every JSON object MUST contain unique member names after JSON string escape
  decoding. A duplicate known or unknown name is `Schema`.
- Every governed object has the exact member set listed below. Unknown members
  are `Schema`.
- Member-name comparison is case-sensitive. No case folding or Unicode
  normalization occurs.
- Object-member ordering is insignificant.
- Standard JSON string escapes are permitted. Escaped and unescaped spellings
  that decode to the same Unicode scalar sequence have the same typed value.
- Unpaired surrogate escapes and any decoded string that cannot be represented
  as valid UTF-8 are `Schema`.
- No Unicode normalization is performed. Distinct scalar sequences remain
  distinct signed string values even if a renderer makes them look alike.

The duplicate and unknown-member checks apply recursively to `SignedEvent`,
`EventCore`, `Provenance` and every `Payload` variant. The contents of the
`metadata_json` **string** are opaque L0 material and are not recursively
parsed under this rule.

### Exact record schema

Every named member below is required exactly once and MUST be non-null.

| Object | Exact members | JSON types |
| --- | --- | --- |
| `SignedEvent` | `core`, `hash`, `signature` | object, string, string |
| `EventCore` | `seq`, `timestamp_nanos`, `prev_hash`, `provenance`, `payload` | u64 token, u64 token, string, object, object |
| `Provenance` | `agent`, `source` | string, string |

`payload.kind` is a required case-sensitive string discriminator. Each payload
object has exactly the following members:

| `kind` | Exact additional members |
| --- | --- |
| `Genesis` | `canonicalization_profile`, `verifying_key` |
| `ClaimAsserted` | `claim_id`, `statement`, `status` |
| `EvidenceRecorded` | `claim_id`, `summary` |
| `ClaimStatusChanged` | `claim_id`, `from`, `to`, `reason` |
| `Note` | `text` |
| `SegmentAnchored` | `bundle_kind`, `witness_root`, `witness_algorithm`, `canonicalization_profile`, `run_id` |
| `ClaimAssertedV2` | `claim_id`, `statement`, `scope_ref`, `actor_class`, `content_hash`, `metadata_json` |
| `EvidenceRegistered` | `evidence_id`, `evidence_kind`, `summary`, `scope_ref`, `actor_class`, `content_hash`, `metadata_json` |
| `JustificationEdgeRecorded` | `edge_id`, `edge_kind`, `source_id`, `target_id`, `scope_ref`, `actor_class`, `rationale`, `metadata_json` |

All additional payload members are JSON strings except the three status-bearing
members `status`, `from` and `to`, which use the exact status law below.
There are no implicit defaults and no optional structural members.

### Hex strings

Hex string values are compared after ordinary JSON string escape decoding.
They MUST contain only ASCII `0`-`9` and lowercase `a`-`f`, with no prefix,
sign, separator, surrounding whitespace or Unicode lookalike.

| Value | Required decoded string shape | Governed stage |
| --- | --- | --- |
| `core.prev_hash` | 64 lowercase hex characters | `Schema` |
| `SignedEvent.hash` | 64 lowercase hex characters | `Schema` |
| `SignedEvent.signature` | 128 lowercase hex characters | `Schema` |
| `Genesis.verifying_key` | 64 lowercase hex characters | `Genesis` |
| `SegmentAnchored.witness_root` | 64 lowercase hex characters | `PayloadValidation` |
| tag 6/7 `content_hash` | empty string or 64 lowercase hex characters | `PayloadValidation` |
| portable CLI external key | 64 lowercase hex characters decoding to 32 bytes that satisfy the verification-key representation law below | `ExternalKey`, before record processing |

### Portable external-key gate

The portable verifier MUST complete these steps, in order, before framing or
processing any record:

1. require exactly 64 lowercase ASCII hex characters;
2. decode exactly 32 octets;
3. interpret those octets as the standard RFC 8032 compressed Edwards25519
   representation: bits 0 through 254 encode `y` as a little-endian integer,
   and bit 255 is `x_0`, the least-significant bit of the recovered
   `x` coordinate;
4. before field interpretation, require the encoded integer `y` to satisfy
   `0 <= y < p`, where `p = 2^255 - 19`; reduction of `y >= p` modulo `p`
   MUST NOT make an encoding acceptable;
5. recover `x` from the Edwards25519 curve equation
   `x^2 = (y^2 - 1) / (d*y^2 + 1) mod p`, where
   `d = -121665/121666 mod p`, using the RFC 8032 section 5.1.3 decoding law;
6. reject when the required square root does not exist;
7. reject when recovered `x = 0` and `x_0 = 1`; otherwise choose the root
   whose least-significant bit equals `x_0`; and
8. encode the recovered point by RFC 8032 section 5.1.2 and require the
   resulting 32 octets to equal the supplied 32 octets byte-for-byte.

The governing point algorithms are
[RFC 8032 section 5.1.2](https://www.rfc-editor.org/rfc/rfc8032#section-5.1.2)
and
[section 5.1.3](https://www.rfc-editor.org/rfc/rfc8032#section-5.1.3),
not a host library's broader decoding profile.

Failure at any step is `REJECT(ExternalKey)` with no line and no
`record_index`. In particular, a representation-invalid key plus zero input is
`REJECT(ExternalKey)`, not empty-snapshot `ACCEPT`. The empty-snapshot rule is
evaluated only after the complete key gate succeeds.

This representation rule is contract-owned. It is not defined by
`ed25519-dalek`, Go's ability to store an arbitrary 32-byte slice, or Python's
constructor defaults. Locked `ed25519-dalek` 2.2.0 explicitly documents its
`VerifyingKey::from_bytes` rule as ZIP-215 rather than RFC 8032/NIST point
validation, and its reduction-oriented decompression accepts the probed
`y >= p` and zero-`x` sign-bit encodings. The future Rust path therefore needs
an explicit canonical gate in addition to the host constructor. Every
conformer MUST independently reproduce the eight contract steps above.

Representation validity stops at successful canonical RFC 8032 point decoding
and byte-identical re-encoding. It does not reject a canonical key because it
is small-order, low-order, weak, untrusted, unauthorized, stale or owned by an
unexpected actor. A canonical low-order key can therefore pass this gate while
still participating in the unresolved A-021 verification relation. Low-order
key behavior is one witness, not the definition of that relation. A-021
continues to own key admissibility and the complete signature predicate.
“Representation-valid” MUST NOT be shortened to an authority claim such as
“trusted key.”

### Status representation

Every JSON status value MUST be one of these case-sensitive strings:

```text
Open
Conjectured
Supported
Settled
Refuted
```

Integers 0-4, other numbers, Booleans, null, lowercase aliases, mixed-case
aliases and surrounding whitespace inside the string are forbidden.

The `magpie-core-v1` canonical encoder continues to encode the typed statuses
as byte tags 0-4. That binary fact does not authorize numeric JSON status.

### Closed vocabularies

The payload discriminator is exactly one of the nine names in the schema
table. Unknown kinds and numeric substitutions are `Schema`.

The following string-valued vocabularies are checked at
`PayloadValidation`, after the cryptographic checks defined below:

```text
actor_class:
  HumanRoot
  AgentProposer
  AutomatedVerifier
  DeadboltAnchorer
  LensWitness
  SourceImporter

evidence_kind:
  DeterministicVerification
  HumanRatification
  DeadboltAnchor
  ExecutionEvidence
  BehavioralEvaluation
  ExternalSource
  ModelSelfReport
  LensReadout

edge_kind:
  supports
  derived_from
  contradicts
  supersedes
  invalidates
  ratifies
```

Comparison is exact and case-sensitive. Verifiers MUST NOT trim, normalize,
case-fold or accept numeric substitutes. This is L0 vocabulary validation, not
standing-policy interpretation.

### Integer fields

The complete record MUST first be syntactically valid JSON under the lexical
law above. A leading plus spelling such as `+1` or `+0`, or a multi-digit
leading-zero spelling such as `01` or `00`, is not an RFC 8259 JSON number and
MUST be `REJECT(JsonSyntax)` at that physical record coordinate. A conformer
MUST NOT parse or coerce such bytes into a number before assigning the class.

Only after JSON syntax succeeds, `seq` and `timestamp_nanos` MUST each be a
JSON number token in this exact lexical form:

```text
0 | [1-9][0-9]*
```

The mathematical value MUST be in `0..=18446744073709551615`.

Syntactically valid JSON numbers that fail this u64 law are `Schema`. They
include negative integers, `-0`, fractional forms (`1.0`, `0.0`), exponent
forms (`1e0`, `1E0`, `1e+0`, `1e-0`) and values above u64 maximum. Booleans,
null and quoted digits are syntactically valid JSON values but have the wrong
schema type and are likewise `Schema`. No host-language numeric coercion or
implicit float conversion is permitted. These schema checks occur before
sequence comparison.

### Required fields and emptiness

All fields in the exact schema are required and non-null. String emptiness is
then governed as follows:

- `ClaimAssertedV2`: `claim_id`, `statement` and `scope_ref` are non-empty.
- `EvidenceRegistered`: `evidence_id`, `summary` and `scope_ref` are non-empty.
- `JustificationEdgeRecorded`: `edge_id`, `source_id`, `target_id`, `scope_ref`
  and `rationale` are non-empty.
- `SegmentAnchored.witness_root` has the exact hex rule above.
- tag 6/7 `content_hash` is empty or exact lowercase hex.
- `metadata_json` is any JSON string value. Its contents may be empty or may be
  text that is not itself JSON.
- Other legacy, provenance and tag-5 string fields have no additional
  non-empty rule in the current `magpie-core-v1` contract.

Genesis profile and key requirements are checked by the Genesis stage and will
reject empty values there. This contract introduces no new identifier policy.

### Payload validation boundary

Payload validation is L0 record validity only. It enforces the non-empty,
closed-vocabulary and embedded-hex rules above. It does not parse
`metadata_json`, interpret a claim, validate a foreign Deadbolt bundle, admit
evidence, infer standing, authenticate an actor or decide truth.

### Chain semantics

For a non-empty input:

- record index 0 MUST have `seq = 0`, all-zero `prev_hash`, and payload kind
  `Genesis`;
- each later record's `seq` MUST equal its zero-based record index;
- each later `prev_hash` MUST equal the prior verified record's content hash;
- the recomputed SHA-256 over the existing `magpie-core-v1` canonical
  `EventCore` bytes MUST equal `SignedEvent.hash`;
- the 64 decoded signature bytes MUST be evaluated over exactly
  `"magpie-sig-v1" || hash` using the supplied external verifying key under
  the owner-ratified A-021 Ed25519 verification relation;
- payload validity MUST pass;
- Genesis MUST appear exactly once at record 0;
- record-0 `canonicalization_profile` MUST equal `magpie-core-v1`; and
- record-0 `verifying_key` MUST have its exact lexical shape and equal the
  lowercase encoding of the externally supplied verifying key.

The empty-snapshot acceptance rule is deliberately separate: there is no
record, Genesis, signature or key-binding claim to make.

This contract owns signature **transport**: `SignedEvent.signature` is exactly
128 lowercase ASCII hexadecimal characters decoding to exactly 64 bytes, and a
transport failure is `Schema` before the `Signature` stage. “Well-shaped” does
not mean “valid.” A-021 owns the complete mathematical acceptance relation for
those 64 bytes, including point, scalar, cofactor and verification-equation
semantics. Exact frozen positive signatures retain their compatibility verdicts;
arbitrary well-shaped signatures do not acquire a general verdict from #120.

## First-failure law

### Stable rejection classes

The portable verdict vocabulary is exactly:

| Class | Meaning |
| --- | --- |
| `ExternalKey` | supplied key text fails exact lowercase 64-hex syntax, exact 32-byte decoding, or any canonical RFC 8032 representation step: `y < p`, curve-point recovery, zero-`x` sign-bit validity, or byte-identical canonical re-encoding |
| `Framing` | byte encoding, BOM, record terminator, empty record or other file/record framing violation |
| `JsonSyntax` | one framed record is not exactly one syntactically valid JSON value, including a numeric-looking token forbidden by RFC 8259 such as `+1` or `01` |
| `Schema` | wrong JSON shape/type, duplicate/unknown/missing member, syntactically valid JSON value/number that violates the u64 type/form/range law, invalid status/payload kind, invalid core/stored hex, null, or invalid decoded string |
| `Sequence` | decoded `seq` differs from the zero-based record index |
| `PreviousLink` | decoded `prev_hash` differs from the prior verified hash, including the all-zero genesis predecessor |
| `ContentHash` | recomputed canonical `EventCore` hash differs from the stored hash |
| `Signature` | for a normative case with an assigned expected result, well-shaped signature bytes fail the owner-ratified A-021 verification relation over the exact domain-separated message |
| `PayloadValidation` | typed payload violates a non-empty, closed-vocabulary, witness-root or optional-content-hash rule |
| `Genesis` | Genesis position, profile, declared-key lexical shape or declared/external key binding is wrong |

Exact diagnostic prose beneath a class is informative and non-normative.
Implementations may retain richer native error values as long as their
portable result maps unambiguously to this vocabulary.

There is no portable `Canonicalization` rejection class. Once a record passes
`Schema`, the frozen canonical encoder is total for that typed input. An
implementation that cannot encode it has a conformance defect or operational
failure; it must not relabel arbitrary accepted input as a new record class.

File-not-found, permission, dependency-loading and internal implementation
failures are operational results outside `ACCEPT`/`REJECT`. They MUST NOT be
confused with a governed rejection or silently converted to acceptance.

### Ordering

The verifier first completes the entire portable external-key gate: syntax,
32-byte decoding and verification-key representation. Any failure is
`ExternalKey` before framing, including for zero-byte input. It then processes
records in physical order. For each record, the first applicable stage wins:

```text
1  Framing
2  JsonSyntax
3  Schema
4  Sequence
5  PreviousLink
6  ContentHash
7  Signature
8  PayloadValidation
9  Genesis
```

Within `Genesis`, the order is:

```text
position/kind
-> canonicalization_profile equality
-> declared-key lexical validity and equality to the supplied key
```

A conformer need not expose sub-reasons as stable identifiers. If multiple
defects map to the same class, only the class and earliest coordinate are
portable.

The `Signature` stage remains in this ordering, and no new rejection class is
introduced. Before A-021 is ratified, #120 does not define its general
mathematical predicate for arbitrary well-shaped signatures. Dependency
sentinels carry no runtime verdict; after A-021, the ratified relation supplies
the predicate at this existing stage.

The following precedence examples are normative:

| Defects in the same record | Required first class |
| --- | --- |
| malformed signature hex + wrong sequence | `Schema` |
| wrong sequence + invalid closed payload value | `Sequence` |
| wrong previous link + invalid payload | `PreviousLink` |
| wrong stored hash + invalid payload | `ContentHash` |
| bad signature + invalid payload | `Signature` |
| bad signature + wrong Genesis profile/key binding | `Signature` |
| invalid payload + wrong Genesis profile | `PayloadValidation` |
| valid prefix + malformed later record | `JsonSyntax` at the later record |

This ordering selects the existing FORMAT/Ticket-0001 cryptographic sequence
and Rust's explicit post-signature payload validation. It does not select Rust
as the normative implementation. The selection is supported by the frozen
format ordering, avoids letting unauthenticated semantic defects mask a bad
stored hash or signature, and requires Python to separate canonical byte
construction from semantic payload validation.

### Coordinates

Coordinates are:

- `line`: one-based physical line, counted from raw bytes using LF/CRLF
  framing; and
- `record_index`: zero-based index assigned to each non-zero-length framed
  record candidate in physical order, before UTF-8 or JSON parsing.

A non-zero-length candidate receives its index even if it is whitespace-only,
invalid UTF-8, malformed JSON or a JSON value followed by trailing garbage.
Such a record-local rejection has both coordinates once raw LF/CRLF framing has
identified the candidate. A zero-length framed slice, including LF-only input
or a slice between terminators, is rejected before it becomes a candidate: it
has a physical `line` and no `record_index`.
An external-key rejection has neither coordinate. A framing defect that truly
prevents a candidate boundary from being determined may omit `record_index`.
That narrow allowance never applies to a forbidden candidate-offset-zero BOM
or to lone CR: both occur inside an already identified non-zero-length
candidate and therefore report both coordinates.

For example, with two valid records followed by a third physical line
containing only space/tab bytes, that line fails `Framing` with `line = 3` and
`record_index = 2`. If the third physical line instead contains zero bytes
between its surrounding terminators, it fails `Framing` with `line = 3` and no
`record_index`. No later candidate is considered because first failure wins.

Likewise, a lone CR within the first non-zero candidate reports `line = 1` and
`record_index = 0`. After two LF-terminated valid candidates, a lone CR in the
third reports `line = 3` and `record_index = 2`. A valid JSON object followed
by a terminal lone CR and EOF reports both coordinates for that same candidate;
the CR is not reinterpreted as a terminator or JSON whitespace.

The governed failure is the earliest failure under physical record order and
the per-record stage order above. A tool may print later diagnostics for human
debugging, but its machine-readable/result summary MUST retain this first
failure and MUST NOT use a later result as the portable verdict.

## Verdict and successful result

A portable result is one of:

```text
ACCEPT {
    event_count: u64,
    tip: 32-byte lowercase hex
}

REJECT {
    class: one of the ten stable classes,
    line: one-based integer or absent,
    record_index: zero-based integer or absent
}
```

These are the only runtime result forms for inputs governed by the contract.
The A-021 dependency sentinels have no normative portable result yet; that
specification gap does not add `UNKNOWN`, `DEFERRED`, `UNSUPPORTED` or any
other third result to a conformer's interface.

For non-empty accepted input, `event_count` is the exact number of verified
records and `tip` is the recomputed hash of the last record. For the accepted
empty snapshot, `event_count = 0` and `tip` is 64 zero characters.

The future command-line conformers MUST use exit code 0 for `ACCEPT`, 1 for a
governed `REJECT`, and a distinct non-1 nonzero code for usage, I/O,
dependency or internal operational failure. Exact human-readable output is not
normative. The conformance runner may consume a small machine-readable mode or
an in-process adapter, but that mechanism must not become a second grammar.

## Differential conformance corpus

### Location and byte identity

The future normative corpus location is:

```text
fixtures/verifier-language-v1/
  manifest.json
  cases/
    <case-id>.jsonl
```

The `v1` in this fixture path versions the corpus contract. It is not a new
event, canonicalization or transport profile identifier and is not a runtime
selector.

Every case file is authoritative as bytes. New cases live under `cases/`.
Positive P1 and P2 point directly at the existing repository-root-relative
golden and Deadbolt fixture paths rather than duplicating them. Malformed JSON,
invalid UTF-8, duplicate members and line-ending cases MUST NOT be represented
only as parsed JSON or regenerated before verification. The manifest MUST bind
each file by SHA-256 so an editor or checkout conversion cannot silently change
the test.

The invalid-JSON integer spellings `+1` and `01` MUST be committed as literal
token bytes rather than produced by a JSON serializer, which cannot emit them
conformantly. N6's zero-length and whitespace-only cases likewise preserve the
exact terminators, spaces and tabs whose coordinate behavior they test.

The repository currently applies `*.jsonl text eol=lf`. The future
implementation PR MUST add a path-specific `.gitattributes` override for every
new exact-byte hostile case, conceptually:

```gitattributes
fixtures/verifier-language-v1/cases/** -text
```

The implementation MUST validate the exact Git syntax and effective
attributes; an equivalent binary/no-text rule is permitted only if it proves
that checkout performs no text or EOL conversion. The override MUST be scoped
to the new corpus subtree and MUST NOT remove or weaken the global JSONL rule
for existing files. P1/P2 continue to reference the existing frozen fixtures
in place; they are not moved or regenerated.

This documentation-only contract PR does not edit `.gitattributes`; that
narrow path override belongs to the later implementation surface.

Repository blob bytes, checked-out fixture bytes and verifier input bytes MUST
be identical for this subtree. The implementation proof MUST inspect effective
attributes (for example with `git check-attr`) for representative paths and,
in a fresh checkout or worktree,
verify every physical SHA-256 against `manifest.input_sha256`. It MUST also
show that the CRLF case retains CRLF, the mixed-EOL case retains both forms,
the no-final-terminator case remains unterminated, invalid UTF-8 remains exact,
no BOM or newline is inserted, and reading the corpus leaves Git status clean.

### Normative cases and dependency sentinels

The corpus metadata MUST place every entry in exactly one of two disjoint,
machine-detectable categories:

1. **Normative conformance case.** The case has an expected `ACCEPT` or
   `REJECT` result and participates in conformance totals.
2. **Dependency sentinel.** The case records a known unresolved dependency and
   has no portable expected verdict. It MUST NOT be counted as a passing
   ordinary conformance case or used to claim complete equivalence.

The exact metadata field spelling is an implementation detail, but the runner
MUST reject ambiguous categorization, a sentinel carrying a premature
portable verdict, or a sentinel included in normative pass totals. The current
mandatory sentinels are A21-D1 and A21-D2 below. After an owner-ratified A-021
decision is implemented by every conformer, every A-021 sentinel MUST migrate
into the normative set with the selected expected result and ordinary
hostile-vector assertions. This is conformance-test metadata, not a runtime
profile, runtime mode or third verifier result.

There is no third manifest category such as `gate-only`, `partial success` or
`representation passed`. Every manifest entry is either a complete normative
case with its portable result or one of the dependency sentinels above. A
substage proof therefore MUST either be embedded in a complete normative
result—zero-byte ACCEPT is used below for representation-positive keys—or be a
non-manifest implementation assertion. A21-D1 and A21-D2 are the only current
entries permitted to lack a portable ACCEPT/REJECT result.

### Manifest model

`manifest.json` is test metadata, not a signed Magpie history. Each normative
case entry MUST contain:

```text
id
repository-root-relative input path
input_sha256
external_verifying_key_hex
expected verdict: ACCEPT or REJECT
expected class for REJECT, otherwise null
expected one-based physical line where defined
expected zero-based record_index where defined
purpose/invariant
```

Every accepted entry MUST also contain:

```text
event_count
tip
ordered recomputed event hashes
```

The ordered hashes let the corpus prove canonical-byte and signed-message
equivalence, not merely a final Boolean. P1 and P2 MUST reference the existing
golden and Deadbolt files in place and assert their existing SHA-256 identities.

A dependency-sentinel entry MUST at least bind its ID, exact input path and
SHA-256, exact external-key text, owning dependency, and purpose/evidence. It
MUST be structurally distinguishable from the expected-verdict entries above
and MUST NOT supply an A-004/RQ-006 portable verdict before its dependency is
ratified.

### Required negative and hostile matrix

The identifiers below name obligations, not a presumption that every case must
reject. N7 deliberately accepts zero bytes under the selected empty-snapshot
law, and N30 contains both accepted and rejected framing variants.

| ID | Required cases and selected result |
| --- | --- |
| N1 | uppercase and mixed-case stored/previous hash: `REJECT(Schema)`; uppercase embedded content hash: `REJECT(PayloadValidation)` once earlier checks pass |
| N2 | uppercase and mixed-case signature: `REJECT(Schema)` |
| N3 | wrong/odd length, non-hex, empty, whitespace, `0x` and Unicode-lookalike hex across each role; reject at its governed stage |
| N4 | numeric status 0-4: `REJECT(Schema)` |
| N5 | unknown/lowercase/mixed status, out-of-range/negative number, float, Boolean and null: `REJECT(Schema)` |
| N6 | two distinct interior cases: a zero-length framed record is `REJECT(Framing)` with physical `line` present and `record_index` absent; a non-empty space/tab-only candidate is `REJECT(Framing)` with physical `line` and the next zero-based `record_index` present |
| N7 | zero-byte input: `ACCEPT`, count 0, zero tip; LF-only input: `REJECT(Framing)` |
| N8 | for every required member coordinate in `SignedEvent`, `EventCore`, `Provenance` and every payload variant, duplicate that known member individually while keeping the record otherwise suitable to isolate schema decoding: `REJECT(Schema)`; focused escaped-name cases separately prove that names compare after JSON escape decoding |
| N9 | duplicate unknown key: `REJECT(Schema)` |
| N10 | one unknown-member vector for each non-payload object shape (`SignedEvent`, `EventCore`, `Provenance`) and independently for each of the nine payload variants: `REJECT(Schema)` |
| N11 | malformed JSON, two values on one record, trailing garbage, and invalid JSON numeral spellings in each u64 field—including `+1`, `+0`, `01` and `00`: `REJECT(JsonSyntax)` |
| N12 | invalid UTF-8: `REJECT(Framing)`, not replacement or crash |
| N13 | BOM-1 first candidate starts `ef bb bf`: `REJECT(Framing)`, line 1/index 0; BOM-2 a later candidate starts `ef bb bf`: `REJECT(Framing)` with its later line/index; BOM-3 space/tab then literal U+FEFF outside a string: `REJECT(JsonSyntax)` with candidate coordinates; BOM-4 literal UTF-8 U+FEFF inside an otherwise valid JSON string and BOM-5 escaped `\uFEFF` inside such a string are not rejected by the BOM rule and, once correctly hashed/signed under the owner-ratified A-021 relation, are complete normative `ACCEPT` cases |
| N14 | `NaN`, `Infinity` and `-Infinity`, including in an unknown member: `REJECT(JsonSyntax)` |
| N15 | syntactically valid JSON fractional and exponent forms for each u64 field—including `1.0`, `0.0`, `1e0`, `1E0`, `1e+0` and `1e-0`: `REJECT(Schema)` |
| N16 | u64 overflow, negative integers, `-0`, Boolean, null and quoted digits for each u64 field: `REJECT(Schema)` |
| N17 | for every required member coordinate in `SignedEvent`, `EventCore`, `Provenance` and each payload variant, omit that member individually while keeping the remainder structurally valid enough to isolate the omission: `REJECT(Schema)` |
| N18 | for every required member coordinate in `SignedEvent`, `EventCore`, `Provenance` and each payload variant, replace that member individually with JSON null: `REJECT(Schema)` |
| N19 | unknown, case-changed, numeric, missing and null payload kind: `REJECT(Schema)` |
| N20 | unknown/case-changed/spaced/numeric closed actor, evidence and edge values: selected schema or payload-validation class and exact precedence |
| N21 | wrong sequence: `REJECT(Sequence)` |
| N22 | wrong genesis or later predecessor: `REJECT(PreviousLink)` |
| N23 | well-shaped wrong stored hash: `REJECT(ContentHash)` |
| N24 | under the owner-ratified A-021 relation, committed well-shaped signature bytes with the selected invalid outcome: `REJECT(Signature)`; any still-unresolved semantic boundary remains a dependency sentinel rather than acquiring a verdict here |
| N25 | non-Genesis at zero and Genesis after zero: `REJECT(Genesis)` |
| N26 | correctly hashed/signed wrong canonicalization profile: `REJECT(Genesis)` |
| N27 | correctly hashed/signed genesis key mismatch: `REJECT(Genesis)` |
| N28 | all precedence pairs listed in the first-failure table, plus invalid core hex + sequence, payload + genesis and later multi-defect cases |
| N29 | valid prefix followed by malformed, bad-link, bad-hash and bad-signature final records: reject at that final coordinate; no ACCEPT summary |
| N30 | no final terminator, LF, CRLF and mixed LF/CRLF: ACCEPT when records are valid; extra final terminator and the two N6 interior cases: `REJECT(Framing)` with N6 coordinates; raw lone CR within a first candidate, a later candidate, and immediately before EOF: `REJECT(Framing)` with both the current candidate's physical line and already-assigned `record_index` |

Additional required negative families include duplicate member names expressed
through escapes, case-changed member names, unpaired surrogates, wrong JSON
container types, NUL, external-key lexical variants, all payload-specific
non-empty rules, witness/content-hash rules and trailing bytes.

### External-key hostile matrix

The future corpus MUST include these key-gate obligations in addition to
N1-N30:

| ID | Required case and result |
| --- | --- |
| K1 | malformed-length external key text: `REJECT(ExternalKey)` |
| K2 | uppercase and mixed-case external key text: `REJECT(ExternalKey)` |
| K3 | non-hex external key text: `REJECT(ExternalKey)` |
| K4 | exact lowercase 64 hex / 32 bytes that fail canonical compressed-point representation decoding: `REJECT(ExternalKey)` with no coordinates |
| K5 | K4 key plus zero-byte input: `REJECT(ExternalKey)` with no coordinates |
| K6 | canonical ordinary golden-fixture key `ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c` plus zero-byte input: `ACCEPT`, `event_count = 0`, zero tip; the key passes the complete gate and canonical re-encoding is byte-identical |
| K7 | canonical representation-valid golden-fixture key plus the normal golden fixture: `ACCEPT` |
| K9 | encoded `y = p`, exact bytes `edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f`: `REJECT(ExternalKey)` |
| K10 | encoded `y = p + 1`, exact bytes `eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f`: `REJECT(ExternalKey)`; field reduction must not accept it |
| K11 | canonical `y` with no recoverable Edwards25519 square root, including exact bytes `0200000000000000000000000000000000000000000000000000000000000000`: `REJECT(ExternalKey)` |
| K12 | identity point `y = 1`, recovered `x = 0`, sign bit 1, exact bytes `0100000000000000000000000000000000000000000000000000000000000080`: `REJECT(ExternalKey)` |
| K13 | identity point with sign bit 0, exact key `0100000000000000000000000000000000000000000000000000000000000000`, plus zero-byte input: `ACCEPT`, `event_count = 0`, zero tip; representation succeeds and canonical re-encoding is byte-identical |
| K14 | ordinary golden-fixture key `ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c` plus zero-byte input: `ACCEPT`, `event_count = 0`, zero tip; representation succeeds and canonical re-encoding is byte-identical |
| K15 | valid `x != 0`, sign-bit-1 key `ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d2ac` plus zero-byte input: `ACCEPT`, `event_count = 0`, zero tip; representation succeeds and canonical re-encoding is byte-identical |

K4 MUST use committed exact text with a documented point-recovery failure,
not merely a test double that forces a constructor error.

K9-K15 MUST likewise use committed exact key bytes and independently check the
RFC 8032 representation algorithm; host-constructor acceptance and
`VerifyingKey::to_bytes()` preserving its stored input are not proof of
canonical re-encoding.

All normative K rows MUST bind exact external-key text, exact input bytes and a
complete portable result. Zero-byte input is the isolating input for pure key
gate proofs: its SHA-256 is
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`, and a
representation-valid key proceeds to the accepted empty snapshot without
invoking A-021 signature semantics; the tip is exactly 64 zero hex characters.
Acceptance says nothing about signatures under that key. K6 and K14 MAY map to
the same exact golden-key/zero-byte case when the manifest records both
obligations. Negative
K rows SHOULD likewise use zero bytes where it isolates `ExternalKey`; K5
specifically preserves the precedence proof that representation failure beats
empty-snapshot acceptance. No normative K row may end at “gate succeeds.”

### Mandatory A-021 dependency witnesses

The former optional K8 idea is replaced by two mandatory, non-conformance
sentinels:

| ID | Required sentinel |
| --- | --- |
| A21-D1 | canonical representation-valid low-order root `ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f` plus committed exact attacker-constructed two-record chain bytes reproducing the preserved A-021 scenario |
| A21-D2 | ordinary golden external key `ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c` plus committed exact chain bytes containing a keyholder-constructed signature whose `R` is the identity point (`01` followed by 31 zero bytes) and whose canonical `S` satisfies the ordinary verification equation |

Their purpose is evidentiary. A21-D1 proves divergence with a low-order external
root. A21-D2 proves that an ordinary external root does not settle the result
for every signature under that key: ordinary verification accepts the stated
equation while locked-dalek strict verification rejects the small-order `R`.
Neither observed result is normative here.

The sentinels are witnesses to one unresolved verification relation, not an
exception list or exhaustive classifier. The future corpus MUST NOT assign
ACCEPT or REJECT, class, coordinate or success summary to either sentinel until
A-021 selects the complete Ed25519 verification law. It MAY record observed
ordinary/strict outcomes as evidence. The implementation need not discover
every strictness-sensitive input before A-021; these two cases are sufficient
to prove that the dependency spans both key and signature behavior.

After A-021 is owner-ratified, both exact sentinels and any additional vectors
required by that contract MUST become normative cases with selected expected
results and must pass across Rust, Go and Python before full A-004/RQ-006
closure is reconsidered.

### Exhaustive schema-coverage proof

N8, N10, N17 and N18 share one mechanically checked schema inventory. N8,
N17 and N18 are Cartesian member-coverage obligations, not representative
sampling. The conformance metadata MUST maintain a machine-checkable expected
set of required-member coordinates covering:

- `SignedEvent.core`, `.hash` and `.signature`;
- `EventCore.seq`, `.timestamp_nanos`, `.prev_hash`, `.provenance` and
  `.payload`;
- `Provenance.agent` and `.source`; and
- `Payload.<Variant>.kind` plus every exact additional member of all nine
  payload variants listed in the normative schema.

For every coordinate, the manifest MUST identify exactly one literal-name
duplicate vector (N8), exactly one omission vector (N17) and exactly one null
vector (N18), each with expected `REJECT(Schema)`. A focused, separately mapped
set of escaped-name duplicate cases proves that decoded member-name equality
governs duplicates; it need not be crossed with every member coordinate.

The same inventory MUST also maintain this exact N10 object-shape set:

```text
SignedEvent
EventCore
Provenance
Payload.Genesis
Payload.ClaimAsserted
Payload.EvidenceRecorded
Payload.ClaimStatusChanged
Payload.Note
Payload.SegmentAnchored
Payload.ClaimAssertedV2
Payload.EvidenceRegistered
Payload.JustificationEdgeRecorded
```

For every shape in that set, the manifest MUST identify exactly one
structurally isolated unknown-member vector with expected `REJECT(Schema)`.
One vector per exact payload variant is sufficient; N10 does not require an
unknown-member cross-product with every declared member. The inserted unknown
name has no normative significance; the mutation must only use an actually
unknown name and leave the remainder valid enough to isolate schema rejection.

The conformance runner MUST fail for a missing N10 shape, a missing N8/N17/N18
mutation, an ambiguous or multiply claimed manifest mapping, or a new governed
payload/member absent from the relevant expected set. This is test metadata
only; it MUST NOT introduce a runtime schema registry or make the Magpie public
API depend on the corpus inventory.

The implementation MUST couple that expected set mechanically to the current
Rust schema, for example through test-only exhaustive struct-field
destructuring and exhaustive payload-variant matching with no wildcard or
`..`. Adding a governed field or variant must therefore fail compilation or
the conformance test until its required-member coordinates, duplicate/omission/
null mutations and payload-shape unknown-member vector are added. An equally direct
test-only mechanism is acceptable; a reviewer-maintained count alone is not.

Corpus authoring MAY generate these fixtures, but the reviewed, committed
post-generation bytes are authoritative. All N8/N10/N17/N18 cases remain
SHA-256-bound exact-byte files and MUST NOT be generated independently at
verifier runtime.

### Required positive matrix

| ID | Required proof |
| --- | --- |
| P1 | existing golden chain is accepted identically and unchanged |
| P2 | existing Deadbolt anchor chain is accepted identically and unchanged |
| P3 | for every normative accepted case, Rust, Go and Python return the same event count |
| P4 | for every normative accepted case, Rust, Go and Python return the same final tip |
| P5 | every valid record in the normative corpus yields the same ordered canonical hash in all implementations |
| P6 | for every exact normative signature fixture with an assigned expected result, all conformers consume the same exact `"magpie-sig-v1" || hash` message and produce that assigned result; this does not define the general Ed25519 verification relation, and A21-D1/A21-D2 remain dependency witnesses until A-021 ratification |
| P7 | permitted object-member reordering preserves the result |
| P8 | permitted space/tab and standard string-escape variants preserve the result |
| P9 | permitted final-terminator, LF, CRLF and mixed-LF/CRLF variants behave identically |
| P10 | every multi-defect case produces the same first class and coordinate |
| P11 | Go imports/invokes no Rust or Python verification implementation and uses no shared canonicalization library |
| P12 | frozen golden and Deadbolt fixture bytes remain byte-identical |

The future normative corpus MUST also prove the complete accepted `u64` width
without host narrowing or floating-point intermediates:

| Coverage ID | Field and exact token | Required complete result |
| --- | --- | --- |
| U64-P1 | `timestamp_nanos = 9007199254740993` (`2^53 + 1`) | correctly canonicalized, hashed and signed normative chain `ACCEPT`; exact value survives all conformers |
| U64-P2 | `timestamp_nanos = 9223372036854775808` (`2^63`) | correctly canonicalized, hashed and signed normative chain `ACCEPT` |
| U64-P3 | `timestamp_nanos = 18446744073709551615` (`u64::MAX`) | correctly canonicalized, hashed and signed normative chain `ACCEPT` |
| U64-P4 | `seq = 9223372036854775808` (`2^63`) | JSON and `Schema` pass; the otherwise isolated record reaches `REJECT(Sequence)`, not `Schema` |
| U64-P5 | `seq = 18446744073709551615` (`u64::MAX`) | JSON and `Schema` pass; the otherwise isolated record reaches `REJECT(Sequence)`, not `Schema` |

The three accepted timestamp cases MUST bind equal canonical hashes, event
count and tip across Rust, Go and Python after signing under the owner-ratified
A-021 relation. The sequence cases need not construct an impractically long
chain: `Sequence` precedes hash/signature, so recomputing later fields is
required only where useful to isolate that intended first failure. A
machine-checkable numeric-boundary inventory MUST map U64-P1 through U64-P5 to
exact case IDs; an overflow-only suite is not sufficient.

Positive conformance MUST exercise every accepted member of every finite
vocabulary in at least one complete normative `ACCEPT` case:

```text
status:
  Open, Conjectured, Supported, Settled, Refuted
actor_class:
  HumanRoot, AgentProposer, AutomatedVerifier, DeadboltAnchorer,
  LensWitness, SourceImporter
evidence_kind:
  DeterministicVerification, HumanRatification, DeadboltAnchor,
  ExecutionEvidence, BehavioralEvaluation, ExternalSource,
  ModelSelfReport, LensReadout
edge_kind:
  supports, derived_from, contradicts, supersedes, invalidates, ratifies
payload.kind:
  Genesis, ClaimAsserted, EvidenceRecorded, ClaimStatusChanged, Note,
  SegmentAnchored, ClaimAssertedV2, EvidenceRegistered,
  JustificationEdgeRecorded
```

Cases MAY cover multiple values. Test-only metadata MUST map each vocabulary
coordinate/value to one or more accepted case IDs and fail if any value is
uncovered, a mapping is ambiguous, or a new governed enum/vocabulary value is
introduced without positive coverage. Where practical, test-only exhaustive
Rust matching SHOULD couple the expected sets to the governed types. This MUST
NOT introduce public/runtime vocabulary metadata.

In addition, positive conformance MUST prove that K6, K13, K14 and K15 have
their complete zero-byte ACCEPT/count/zero-tip results (with K6/K14 permitted
to share one exact case), that the ordinary golden external key passes the
complete canonical representation gate and re-encodes byte-identically, that
every new exact-byte case survives Git checkout byte-identically, and that the
shared N8/N10/N17/N18 schema-coverage assertion is complete. A sentinel
assertion MUST prove that neither A21-D1 nor A21-D2 can be counted as a
normative conformance pass before A-021 ratification.
Acceptance of the exact golden and Deadbolt
signatures MUST NOT be generalized into a verdict law for other signatures
under either fixture key.

The corpus MUST contain positive, negative and multi-defect precedence cases.
A comparison of final exit status alone is insufficient.

## Rust conformance obligation

The future Rust implementation MUST make the production/file verification path
conform, not merely add a serde toy parser used only by corpus tests. In
particular it must address:

- `FileStore` blank-line elision and exact terminator handling;
- lowercase-only stored hash, previous hash and signature transport spelling;
- duplicate members at all governed levels and unknown members independently
  in each governed payload variant as well as the non-payload object shapes;
- raw JSON integer-token constraints;
- stable class/coordinate mapping; and
- the complete external-key representation gate before the selected
  empty-snapshot result.

Because a locked-dalek `VerifyingKey` can retain a ZIP-215 representation that
fails this canonical law, the real Rust path or its portable entry adapter MUST
validate `VerifyingKey::as_bytes()` under the contract before reading the
snapshot. Successful construction of the Rust type is not sufficient.

Rust's current ordinary signature verification is runtime evidence, not an
A-021 decision. The future portable verifier implementation MUST follow the
owner-ratified A-021 relation. Passing exact frozen signatures under the golden
key does not establish a general signature domain for that key.

The existing `magpie-core-v1` canonical encoder and cryptographic operations
remain the production implementation. Their outputs MUST be compared with the
independent implementations. Serde defaults are insufficient evidence of
conformance.

The future change should avoid adding a public Rust concept unless the
implementation proves that a public result surface is necessary. An internal
conformance result or small tool adapter is preferred over broadening
`magpie-log`'s public API merely to run fixtures.

## Go independent-verifier obligation

The future Go verifier is an independently implemented portable verifier and
release-oracle candidate. It is not yet a universally equivalent release
oracle for the complete signature-verification relation; that status depends
on the A-021 gate below.
The smallest repository-consistent handoff is a
self-contained command directory under:

```text
tools/go-verify-chain/
```

The implementation PR may adjust the executable filename inside that isolated
directory if Go tooling requires it, but moving the role into a shared runtime
library requires new owner review.

The Go implementation MUST:

- be standalone and local;
- link to, invoke and generate no code from Rust or Python;
- implement `magpie-core-v1` canonical encoding independently from
  `docs/FORMAT.md` and this contract;
- implement chain, hash, signature, payload and genesis checks independently;
- parse the portable JSONL byte language explicitly;
- use no network access during verification;
- be deterministic;
- use no cgo unless a separately reviewed need is demonstrated; and
- produce one small command suitable for release verification.

Standard-library `crypto/sha256`, `crypto/ed25519` and `encoding/json` are the
preferred foundation. A minimal `go.mod` is acceptable for tooling identity
even if there are no third-party dependencies. Standard-library-only is the
default expectation, not a permission to inherit parser defaults.

Go's ability to hold an arbitrary 32-byte `ed25519.PublicKey` MUST NOT bypass
the portable point-representation gate. If the Go standard library exposes no
equivalent RFC 8032 canonical decoding predicate, the implementation MUST
enforce `y < p`, point recovery, the zero-`x` sign rule and canonical
re-encoding independently or STOP for owner review before adding a dependency
or weakening the rule. This contract authorizes no third-party Go dependency;
standard-library-only remains preferred where feasible.

Go MUST NOT make its own ordinary-versus-strict choice normative for A21-D1,
A21-D2 or any other well-shaped signature. The complete implementation follows
the owner-ratified A-021 relation; until then Go remains a release-oracle
candidate rather than proof of universal signature equivalence.

In particular, the Go implementation must explicitly check UTF-8 before JSON
decoding, duplicate names, unknown members, exact case, trailing JSON values,
number tokens and all closed vocabularies. `encoding/json`'s tolerance for
unknown members, duplicate names, case-insensitive struct matching or invalid
UTF-8 replacement must not define Magpie's language. No experimental JSON
package or experimental Go feature is a constitutional dependency merely for
strictness.

No Go toolchain was available during this documentation-only discovery, and no
Go code, module or dependency is added here.

## Python reference-verifier obligation

`tools/verify_chain.py` remains the readable independent/reference conformer.
The future implementation MUST retain its small review surface while making
parser behavior explicit. At minimum it must:

- read bytes and frame records before text-mode newline conversion;
- reject invalid UTF-8 and BOM with a governed result rather than traceback;
- reject duplicate names, unknown members and non-finite numbers;
- enforce exact integer token syntax rather than Python integer coercion;
- accept only named JSON statuses;
- enforce the complete external-key syntax, exact decode and compressed-point
  representation gate before opening or framing the input;
- separate canonical byte construction from semantic payload validation so
  hash/signature failures retain their precedence; and
- expose the same first class and coordinate as Rust and Go.

Python may continue to depend on `cryptography`. Version pinning and toolchain
reproducibility remain RQ-015, outside this contract.

Python's current ordinary-compatible verification behavior is likewise
evidence, not the A-021 law. Tightening its grammar or point representation
does not authorize it to settle A21-D1, A21-D2 or the general verification
relation.

## Implementation independence law

Permitted sharing among conformers is limited to:

- this reviewed specification and `docs/FORMAT.md`;
- exact fixture bytes and their manifest;
- expected hashes, tips, classes and coordinates; and
- release/conformance commands.

Forbidden sharing includes:

- calling the Rust canonical encoder from Go or Python;
- FFI to the Rust verifier;
- generated bindings containing canonicalization or verification logic;
- invoking another verifier and relaying its answer;
- a common native parser/crypto library that performs Magpie verification; or
- one implementation generating expected answers at conformance-test runtime.

An orchestration script may execute the three commands and compare their
already-independent results. It must not parse or verify Magpie records on
their behalf.

The purpose of three implementations is independent failure modes. Three
wrappers around one implementation do not satisfy P11.

## Compatibility analysis

### Preserved canonical compatibility

This contract changes none of:

- `EventCore`, `Payload`, `SignedEvent`, their field order or serde storage
  representation;
- the `magpie-core-v1` canonical encoder;
- SHA-256 content hashes;
- the `magpie-sig-v1` domain or Ed25519 signed message;
- genesis canonical bytes or semantics;
- any frozen fixture byte; or
- the historical meaning of the v0.1.0 tag.

Both existing frozen valid histories use the selected lowercase hex, named
status, exact fields, valid UTF-8 and LF framing and MUST remain accepted
byte-for-byte.

### Reference-input compatibility changes

The future implementation intentionally narrows incidental transport behavior:

- Rust will stop accepting blank-record elision and mixed/uppercase core and
  stored hex spellings.
- Both current implementations will stop ignoring unknown fields and duplicate
  unknown names.
- Python will stop accepting duplicate known names, numeric status,
  non-finite JSON, lone-CR newline aliases and uppercase/mixed CLI keys.
- Python will turn invalid UTF-8 into governed rejection rather than an
  uncaught exception.
- Python will move payload semantic validation after hash/signature checks.
- The portable Rust entry will reject RFC 8032-noncanonical external-key
  encodings that locked-dalek's ZIP-215 constructor currently represents.

Python will broaden one behavior: zero bytes becomes an accepted empty
snapshot with count 0 and zero tip, but only after the supplied external key
passes syntax, exact decoding and representation validation. This follows
FORMAT's explicit “non-empty chain” genesis wording and the stable Rust
empty-store identity. It does not certify a genesis/key binding.

These are pre-1.0 reference-input-language compatibility changes. They are not
canonical format changes and MUST NOT be described as a new
`magpie-core-v1` profile.

No compatibility result in this contract selects ordinary or strict signature
verification for arbitrary well-shaped signatures. The exact frozen positive
fixtures remain accepted compatibility commitments, but that does not settle
other signatures under the same keys. A21-D1 and A21-D2 preserve this boundary
for the separately reviewed A-021 decision.

## Trust-root boundary

The external verifying key remains selected outside the history. Successful
syntax, Ed25519 verification and genesis key equality establish integrity
relative to that supplied key. They do not establish that the key belongs to a
trusted actor, is current, is authorized or is cryptographically strong under
any additional policy.

This contract governs the portable textual key spelling, exact decoding,
canonical RFC 8032 compressed-point representation and equality check.
`ExternalKey` acceptance means only that verification can structurally proceed
with the supplied representation. It does not establish key strength, trust or
authority and does not define which well-shaped signatures verify.

A-021 owns the exact Ed25519 signature-verification relation, including
ordinary-versus-strict behavior, low-order/torsion public keys, signature `R`
encoding and point properties, scalar `S` range/canonicality, cofactor/equation
semantics and compatibility across the locked libraries. It remains Confirmed
and is an explicit prerequisite to full A-004/RQ-006 equivalence and closure.
This cryptographic ACCEPT/REJECT boundary relative to a caller-supplied root is
separate from key custody, identity, authorization, PKI, reputation and actor
authority.

### A-021 closure dependency

The preferred next architectural step after owner merge of #120 is a dedicated
A-021 contract answering:

> What exact Ed25519 verification relation does Magpie mean when it records
> that a signature verifies?

Without answering it here, that contract must investigate and settle external
public-key representation versus verification semantics; ordinary versus
strict verification; public-key low-order/torsion behavior; signature `R`
canonical encoding and point requirements; small-order/torsion `R`; scalar `S`
range and canonicality; cofactor and equation semantics; RFC 8032 compatibility;
locked `ed25519-dalek`, Python `cryptography` and Go `crypto/ed25519`
divergence; v0.1.0 frozen-history compatibility; whether stricter semantics are
backward-compatible; whether incompatibility requires an additive
profile/version; and exact hostile differential vectors.

This is a handoff only. #120 does not create that contract, mint an ADR/ticket,
or choose ordinary or strict verification. Portable verifier implementation is
sequenced after owner ratification so three verifier stacks are not built
around an unfrozen central predicate.

A-004 and RQ-006 MUST NOT be administratively Closed merely because grammar,
point representation, schema, ordinary fixtures, N1-N30 or K1-K15 pass. Full
closure requires:

1. owner merge of #120;
2. an A-021 contract defining the exact Ed25519 verification relation;
3. owner ratification of that decision;
4. Rust implementation of that relation;
5. independent Go implementation of that relation;
6. independent Python implementation of that relation;
7. all portable-language cases passing;
8. all A-021 signature-semantic vectors passing identically;
9. exact frozen histories handled under the ratified compatibility law;
10. independent hostile review finding no unresolved divergence;
11. owner merge of the implementation tranche or tranches; and
12. a separate living-ledger reconciliation.

## Resource and storage boundary

This contract defines no file-size, line-size, nesting-depth, string-length,
record-count, memory, time or recursion ceiling. It does not select streaming,
atomicity, locking, durability, snapshot or crash-recovery semantics. Those
are A-008 / RQ-007 / RQ-008 and part of the wider RQ-013 assurance boundary.

Conformers must agree on all finite normative corpus inputs in the current
governed domain. Dependency sentinels are excluded until their governing
decision activates them. A later resource contract
may add explicit operational limits without redefining canonical event
identity. An implementation crash on a small corpus vector is still a
conformance failure even though general resource bounds are out of scope.

## Rejected alternatives

### Rust behavior is automatically normative

Rejected. Rust currently accepts uppercase stored hex and drops empty records,
and serde accepts unknown fields. Production status does not turn those
incidental defaults into a portable grammar.

### Python behavior is automatically normative

Rejected. Python currently accepts numeric status, duplicate names,
non-finite values in ignored members and lone-CR newline aliases, and can crash
on invalid UTF-8.

### Go standard-library defaults are automatically normative

Rejected. `encoding/json` is an implementation tool and requires explicit
checks for this contract.

### Let each conformer choose ordinary or strict verification

Rejected. A21-D1 proves divergence with a low-order root, while A21-D2 proves
divergence under the ordinary golden root through a small-order signature `R`.
The disputed boundary is the complete verification relation, not a list of key
exceptions. This contract surfaces that dependency rather than pretending two
incompatible relations are one portable law or selecting crypto policy outside
A-021.

### Treat every low-order key as a malformed representation

Rejected. A low-order point can have a canonical RFC 8032 encoding. Folding
weak-root policy into `ExternalKey` would disguise the unresolved A-021 choice
as syntax and could silently select strictness without compatibility review.

### Canonicalize JSON bytes directly

Rejected. It would replace the frozen typed binary preimage and change every
hash/signature commitment.

### Accept every form accepted by any current implementation

Rejected. It would preserve all parser accidents, numeric status aliases,
duplicate-key ambiguity and non-finite pseudo-JSON.

### Drop the Python verifier

Rejected. Its small readable implementation is valuable independent evidence
and debugging surface.

### Replace Python with Go without differential overlap

Rejected. The overlap and one shared corpus are what demonstrate that the new
implementation did not merely introduce a different divergence.

### Share Rust canonicalization through FFI or generated logic

Rejected. Shared implementation bugs would defeat the independent verifier's
purpose.

### Make JSONL a new `magpie-core` profile

Rejected. The portable language is a tooling/interoperability contract around
the existing typed records, not a new signed identity.

### Fix only the headline audit examples

Rejected. Discovery found additional framing, UTF-8, duplicate-member,
non-finite, integer, field-shape and first-failure differences.

### Compare only final Boolean success

Rejected. A verifier that fails at the wrong record or for the wrong governed
reason can mask security-relevant disagreement.

### Make exact diagnostic prose normative

Rejected. Stable finite classes and coordinates are portable; exact prose is
language-specific and needlessly brittle.

## Implementation proof obligations

The future implementation PR must prove all of the following:

1. The complete N1-N30 and additional hostile matrix has exact-byte normative
   cases, and A21-D1/A21-D2 exist as disjoint dependency sentinels.
2. P1-P12 pass for Rust, Go and Python over the current normative corpus.
3. Every conformer consumes the same unmodified normative case bytes and
   manifest key.
4. Every normative accepted case agrees on count, tip and ordered record
   hashes.
5. Every normative rejected case agrees on first class and coordinate.
6. Rust exercises its real file/public verification path.
7. Python exercises `tools/verify_chain.py`, not a substitute parser.
8. Go independently encodes and verifies; a dependency/import/source audit
   proves P11.
9. Invalid UTF-8, candidate-position BOMs, duplicate keys and malformed inputs
   do not crash any tool.
10. Existing golden and Deadbolt fixtures and their published verifier results
    remain unchanged.
11. `docs/FORMAT.md`, canonical Rust source and historical release evidence
    remain unchanged unless a later owner-approved scope explicitly says
    otherwise.
12. A-004/RQ-006 remain administratively open until A-021 is ratified and
    incorporated, independent review completes, the owner merges, and a
    separate ledger reconciliation occurs.
13. K1-K7 and K9-K15 prove the complete canonical external-key gate, including
    representation failure before zero-byte acceptance; A21-D1 and A21-D2 are
    mandatory dependency witnesses with no portable verdict yet.
14. Effective Git attributes and fresh-checkout SHA-256s prove that every new
    exact-byte case survives checkout without normalization.
15. One machine-checkable schema inventory proves one N8 duplicate, one N17
    omission and one N18 null vector for every required member, plus one N10
    unknown-member vector for `SignedEvent`, `EventCore`, `Provenance` and every
    payload variant, with no missing or ambiguous coverage.
16. The runner prevents either A-021 sentinel from contributing to conformance
    pass totals, and full A-004/RQ-006 closure remains blocked until A-021 is
    ratified, implemented across all conformers and covered by activated
    normative signature-semantic vectors.
17. Exact N6 fixtures prove the zero-length/no-index versus non-empty
    whitespace/candidate-index distinction, and exact integer fixtures prove
    `JsonSyntax` for non-JSON numeral spellings versus `Schema` for valid JSON
    values that violate the u64 law.
18. N13 proves candidate-offset-zero BOM rejection for first and later records,
    normal JSON staging for non-offset-zero U+FEFF, and non-rejection of literal
    and escaped U+FEFF string content.
19. N30 proves first-candidate, later-candidate and terminal-before-EOF lone CR
    failures with both line and `record_index`.
20. U64-P1 through U64-P5 prove accepted full-width timestamp values and
    in-range upper `seq` values reaching `Sequence`, never host narrowing or
    `Schema`.
21. A machine-checked accepted-vocabulary inventory covers every status,
    actor class, evidence kind, edge kind and payload discriminator.
22. Every normative K manifest row has a complete portable result; K6/K13/K14/
    K15 use zero-byte ACCEPT with count 0 and zero tip where appropriate, and
    only A21-D1/A21-D2 lack verdicts.

## Hostile scenarios

An independent reviewer should attempt at least these attacks:

- hide a signed value behind a duplicate member whose first and last values
  differ;
- accept duplicate `EvidenceRegistered.metadata_json` or another untested
  required payload member while rejecting a representative duplicate;
- express member names, statuses or hex through JSON escapes;
- place `NaN` or invalid UTF-8 in an unknown member that a decoder might skip;
- reject an unknown member for `Genesis` while silently accepting one for a
  different payload variant such as `EvidenceRegistered`;
- coerce `+1` or `01` and report `Schema` instead of the earlier
  `JsonSyntax` failure;
- collapse a non-empty space/tab-only candidate into the zero-length record
  coordinate rule;
- reject all U+FEFF bytes as BOMs, or accept a per-record offset-zero BOM after
  a valid prefix;
- omit `record_index` for a lone CR even though its non-zero candidate is
  already framed;
- use Python's lone-CR/universal-newline behavior or checkout line conversion;
- exploit Go's case-insensitive struct matching, duplicate-name handling or
  UTF-8 replacement;
- use `-0`, exponent syntax, huge integers, Booleans or quoted digits;
- narrow in-range JSON integers through signed-64-bit or IEEE-754
  intermediates;
- accept only a strict subset of a finite status/actor/evidence/edge/payload
  vocabulary while passing every negative case;
- make malformed signature hex compete with sequence/link failures;
- make invalid payload semantics compete with hash/signature/genesis failures;
- append a malformed record after a fully valid prefix;
- cause one implementation to report a later failure as the governed first
  result;
- replace a corpus file while leaving a parsed manifest representation
  unchanged;
- let Git normalize CRLF, mixed-EOL, unterminated or invalid-UTF-8 case bytes;
- let a lowercase 32-byte representation-invalid key reach empty-snapshot
  acceptance;
- accept `y >= p` through field reduction or accept `x = 0` with sign bit 1;
- treat host-key byte preservation as canonical point re-encoding;
- confuse point representation with signature verification, weak-key policy or
  trust authority;
- count A21-D1 or A21-D2 as an ordinary conformance pass or assign either a
  verdict before A-021 ratification;
- leave a normative positive key case at “representation gate succeeds” rather
  than binding a complete ACCEPT/REJECT result;
- infer that all signatures under the golden key have settled semantics merely
  because the exact golden fixture signatures pass;
- enumerate low-order keys and small-order `R` as though those witnesses
  exhaust the unresolved verification relation;
- omit one required payload member, such as
  `EvidenceRegistered.metadata_json`, from N17/N18 coverage;
- make Go or Python call into Rust, or one verifier relay another's result;
- drift canonical hashes while retaining the same ACCEPT Boolean;
- treat empty-snapshot acceptance as a trusted genesis/key-binding claim; or
- inflate syntactic key acceptance into A-021 trust-root authority.

## Non-goals and adjacent findings

This contract does not:

- change FORMAT, canonical bytes, payload tags, signatures or genesis;
- implement Go or modify Rust/Python;
- edit `.gitattributes`, fixtures or CI in this contract PR;
- define resource, streaming, storage, locking, durability or atomicity law
  (A-008 / RQ-007 / RQ-008);
- change projection fallibility (A-007 / RQ-009);
- reopen the completed verified-replay event-type correction or remediate the
  residual Deadbolt eligibility portion of A-009;
- pin Python, Rust or Go dependencies/toolchains (A-011 / RQ-015);
- close the wider assurance finding (A-018 / RQ-013);
- define the Ed25519 signature-verification relation owned by A-021, including
  weak/low-order keys or strictness-sensitive signatures, although it records
  A-021 as the next preferred contract and a hard implementation/closure gate;
- add admission or `EpistemicGate` (A-022);
- add provenance-complete detached responses;
- add authority-bound corroboration (A-024);
- add a generic parser framework, dynamic verifier registry or “latest”
  profile; or
- authorize any implementation PR.

## Implementation handoff

The intended future implementation surface is narrowly:

```text
.gitattributes (path-specific no-text rule for the new exact-byte corpus only)
fixtures/verifier-language-v1/**
crates/magpie-log parsing/verification implementation and focused tests
tools/verify_chain.py
tools/go-verify-chain/**
one differential conformance runner/test
the existing CI workflow entries needed to run all conformers
one later implementation ticket
```

The implementation must justify exact Rust file placement from the code at its
then-current base. It must not change `docs/FORMAT.md`, frozen event identity,
existing fixture bytes or historical release evidence merely to satisfy a
transport rule. Any apparent need for a new profile identifier or format
change is a stop condition requiring owner review.

The sequence is:

```text
1. merge this contract
2. create and owner-ratify a dedicated A-021 Ed25519
   verification-semantics contract
3. implement the complete portable verifier tranche: exact-byte corpus,
   Rust conformance, independent Go, Python conformance and the selected A-021
   relation
4. run the complete cross-language differential corpus, including activated
   A21-D1/A21-D2 and any additional A-021 vectors
5. perform hostile independent review over the complete matrix
6. owner-merge the implementation tranche or tranches
7. only then reconsider A-004/RQ-006 administratively in a separate PR
```

This order is preferred because it avoids implementing three verifier stacks
before their cryptographic acceptance relation is frozen. This contract does
not create or authorize the A-021 contract/implementation and does not edit the
living programme ledger.

## Reviewer checklist

- [ ] No implementation language is described as normative.
- [ ] JSONL remains non-canonical transport around unchanged
      `magpie-core-v1` typed identity.
- [ ] Empty zero-byte input and LF-only input are distinguished explicitly.
- [ ] The complete external-key representation gate runs before empty-snapshot
      acceptance and follows canonical RFC 8032 decoding, including `y < p`,
      zero-`x` sign handling and byte-identical re-encoding.
- [ ] A canonical low-order key is not rejected merely to hide A-021, and
      representation validity is not described as trust or authority.
- [ ] The present portable invariant is limited to normative cases that do not
      depend on the unresolved general Ed25519 verification relation.
- [ ] Signature transport is governed here, while well-shaped signature
      validity is supplied by A-021 at the existing `Signature` stage.
- [ ] A21-D1 and A21-D2 are mandatory dependency witnesses, are explicitly
      non-exhaustive, have no premature portable verdict and cannot count
      toward conformance totals until A-021 is ratified.
- [ ] Exact frozen positive signatures remain accepted, without generalizing
      that result to every signature under the same key.
- [ ] N8 duplicates every required member coordinate; N10 independently covers
      unknown members in `SignedEvent`, `EventCore`, `Provenance` and each of
      the nine payload variants through the shared schema inventory.
- [ ] Candidate-offset-zero BOMs fail `Framing` for first and later candidates,
      while literal/escaped U+FEFF string content and non-offset-zero placement
      follow their selected string/JSON laws.
- [ ] Invalid UTF-8, non-finite JSON and lone CR are decided explicitly; every
      lone-CR case has both line and candidate index.
- [ ] Status is exact named-string JSON, not the canonical numeric byte tag.
- [ ] Core/stored hex and payload hex stages are explicit.
- [ ] Invalid JSON numeral spellings such as `+1` and `01` are `JsonSyntax`;
      valid JSON negatives, `-0`, fractions, exponents, overflow, Booleans,
      null and quoted digits that violate the u64 law are `Schema`.
- [ ] Every field and payload variant has an exact required-member set.
- [ ] Payload semantic validation remains L0-only and has explicit precedence.
- [ ] Stable classes, coordinates and multi-defect ordering are finite and
      deterministic.
- [ ] N6 gives a zero-length record a physical line and no index, while a
      non-empty whitespace-only candidate gets both its line and next index.
- [ ] Positive results bind count, tip and ordered hashes.
- [ ] Timestamp positives cover `2^53 + 1`, `2^63` and `u64::MAX`; in-range
      upper `seq` values pass `Schema` and reach `Sequence`.
- [ ] Every accepted status, actor class, evidence kind, edge kind and payload
      discriminator has machine-checked positive normative coverage.
- [ ] Every normative K row has a complete result; K6/K13/K14/K15 use isolated
      zero-byte ACCEPT results where appropriate, with no gate-only category.
- [ ] Corpus cases are exact bytes with SHA-256 bindings.
- [ ] A path-specific Git attribute prevents normalization of new hostile case
      bytes, and a fresh checkout proves every manifest hash.
- [ ] N8/N10/N17/N18 share one machine-checked schema inventory: duplicate,
      omission and null for every required member coordinate, plus per-shape
      unknown members.
- [ ] Go is standalone, preferably standard-library-only, with no FFI/shared
      verifier logic.
- [ ] Python is retained and tightened rather than deleted.
- [ ] Frozen valid fixtures remain accepted without regeneration.
- [ ] A-021 remains undecided but is an explicit hard prerequisite to full
      implementation, A-004/RQ-006 equivalence and closure; it is the preferred
      next contract after #120. Resource/storage law, RQ-013 closure and RQ-015
      remain outside.
- [ ] A-004 and RQ-006 remain Confirmed.

## Target law

> The reviewed Magpie contract and exact-byte conformance corpus define one
> portable JSONL verifier input language. Rust, independently implemented Go,
> and readable Python must return the same governed verdict, first-failure
> class, coordinate, count, tip and hashes for each normative case in the
> current conformance domain, while `magpie-core-v1` canonical event identity
> and external trust-root selection remain unchanged. #120 governs signature
> transport but not the general validity of well-shaped signatures. A-021 must
> select the exact Ed25519 verification relation before the portable verifier
> implementation and full equivalence claim proceed.
