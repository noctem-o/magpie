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
2. the exact-byte conformance corpus and the Rust, Go and Python changes land;
3. the three implementations demonstrate equivalent governed verdicts over
   the complete corpus;
4. an independent hostile review completes;
5. the human owner merges the implementation; and
6. the living audit ledger is reconciled separately.

The future corpus will also improve the evidence owned by RQ-013, but it will
not close RQ-013's separate resource, crash and exact-boundary assurance gaps.

## Purpose

Define one portable language for Magpie's reference JSONL verification
interface and one cross-language verdict law:

```text
                         reviewed Magpie contract
                                   |
                      portable verifier language
                                   |
                 +-----------------+-----------------+
                 |                 |                 |
              Rust               Go              Python
           production        independent         readable
           conformer       release conformer     reference
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

The portable invariant is:

```text
same exact input bytes
+ same supplied external verifying key
+ this reviewed contract and corpus version
= same ACCEPT/REJECT verdict
+ same first-failure class
+ same failure coordinate where one is defined
```

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

## Current implementations and future roles

### Rust

Rust remains the production and in-process Magpie verifier. It is one future
conformer. Its current serde and file-splitting behavior is evidence, not law.

### Go

Go is selected for the future independent, release-grade verifier. It will be
a small standalone local command, preferably standard-library-only, with no
network access and no dependency on either existing implementation. Its role
is frozen below; no Go code or module exists in this PR.

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
| framing | interior zero-length line | ACCEPT because it is dropped | REJECT parse | never emitted | FORMAT describes one event per record, not ignorable holes | REJECT `Framing` | Rust narrows | N6 |
| framing | whitespace-only line | REJECT serde | REJECT parse | never emitted | not a record | REJECT `Framing` | class becomes portable | N6 |
| framing | CRLF throughout | ACCEPT | ACCEPT | LF | cross-platform reference transport may preserve ordinary line endings | ACCEPT | preserve | N30/P9 |
| framing | mixed LF and CRLF record terminators | ACCEPT | ACCEPT | LF | no identity consequence | ACCEPT per record | preserve | N30/P9 |
| framing | lone CR between JSON objects | REJECT trailing data | ACCEPT because Python text mode translates CR to a newline | never emitted | lone CR is not selected as a record terminator | REJECT `Framing` | Python narrows | N30 |
| framing | leading/trailing space or tab around one JSON object | ACCEPT | ACCEPT | none | insignificant JSON whitespace does not enter the canonical preimage | ACCEPT | preserve | P8 |
| framing | UTF-8 BOM | REJECT parse | REJECT parse | none | no BOM is emitted or promised | REJECT `Framing` | preserve verdict, stabilize class | N13 |
| framing | invalid UTF-8 byte | REJECT serde | uncaught `UnicodeDecodeError` traceback | valid UTF-8 | canonical strings are UTF-8 | REJECT `Framing`; no crash | Python gains governed failure | N12 |
| framing | NUL byte inside a framed record | REJECT | REJECT | none | NUL is not valid unescaped JSON content | REJECT `JsonSyntax` | preserve verdict, stabilize class | framing negative |
| framing | two JSON values on one line or trailing garbage | REJECT trailing characters | REJECT extra data | one object per line | reference storage is one event per record | REJECT `JsonSyntax` | preserve | N11/N30 |
| object grammar | duplicate known `seq`, `hash`, or `kind` with the same value | REJECT duplicate field | ACCEPT last-wins | exactly one member | no duplicate spelling was promised | REJECT `Schema` for every duplicate name | Python narrows | N8 |
| object grammar | duplicate unknown member | ACCEPT, ignored | ACCEPT, last-wins/ignored | no unknown member | closed record grammar is safer than coincidentally shared permissiveness | REJECT `Schema` | both narrow | N9 |
| object grammar | one unknown top/core/provenance/payload member | ACCEPT, ignored | ACCEPT, ignored | exact known members only | payload variants and format fields are explicitly enumerated | REJECT `Schema` at every governed object | both narrow | N10 |
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
| external key | uppercase/mixed-case CLI key | not applicable: Rust production receives typed key bytes, not CLI text | Python lowercases then accepts | release commands use lowercase | external trust selection is separate from lexical transport | portable CLI requires exactly 64 lowercase ASCII hex; invalid input is `ExternalKey` | Python CLI narrows; no A-021 decision | external-key vectors |
| status | exact `Open`, `Conjectured`, `Supported`, `Settled`, `Refuted` strings | accepted as enum; changed signed value later fails hash | accepted | named string | serde writer and frozen fixture use names; FORMAT numeric tags describe canonical bytes | exact case-sensitive strings only | preserve writer language | status positives |
| status | integers 0-4, especially `1` replacing `"Conjectured"` | REJECT schema | accepts; `1` verifies unchanged canonical bytes | named string | no release promise of numeric JSON status | REJECT `Schema` | Python narrows | N4 |
| status | unknown/lowercase/mixed string, out-of-range/negative integer, float, Boolean or null | REJECT | REJECT, with varying parse/canonicalization paths | exact named string | closed status vocabulary | REJECT `Schema` | stabilize class | N5 |
| payload kind | unknown/case-changed/numeric/missing/null `payload.kind` | REJECT serde | REJECT canonicalization | exact case-sensitive name | FORMAT closes tags 0-8; Ticket 0005 forbids fallback | REJECT `Schema` | preserve | N19 |
| closed strings | unknown/case-changed/spaced actor class, evidence kind or edge kind | type is string; unsigned mutation reaches hash mismatch before Rust validation | Python canonicalization rejects before hash | exact closed value | FORMAT and Ticket 0010 enumerate the vocabularies | exact decoded value, no trimming; reject as `PayloadValidation` after crypto | Python ordering changes | N20/N28 |
| integers | `seq` or `timestamp_nanos` token `0` through `18446744073709551615` | accepted as u64, then chain/hash rules apply | accepted as Python int, then chain/hash rules apply | unsigned decimal | FORMAT uses u64 | accept lexical value; semantic checks follow | preserve | integer positives |
| integers | `u64::MAX + 1` or negative integer | REJECT serde | Python accepts the JSON integer; range/sequence rejection occurs later | never emitted | canonical encoder requires u64 | REJECT `Schema` | stabilize Python ordering | N16 |
| integers | `-0`, fraction, exponent, Boolean, null or string | REJECT serde; `-0` is treated as float | Python may turn `-0` into integer zero and may inspect null/string during sequence | unsigned integer token | writer emits plain unsigned decimal; canonical type is u64 | only unsigned decimal JSON tokens in u64 range | Python narrows and classes align | N15/N16 |
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

1. Input is an exact byte sequence. A conformer MUST inspect the original
   bytes; text-mode newline translation is not permitted before framing.
2. The zero-length byte sequence is an accepted empty snapshot. Its successful
   result is `event_count = 0` and the all-zero 32-byte tip. It is not a claim
   that genesis or supplied-key binding occurred.
3. Any non-empty input contains one or more non-empty record texts.
4. A record terminator is LF (`0a`) or CRLF (`0d 0a`). Either may be used for
   any record; mixed LF/CRLF input is permitted.
5. The last record terminator is optional. A second terminator after the last
   record creates an empty record and MUST be rejected. Thus zero bytes is the
   only empty-snapshot spelling; LF alone is an empty record, not an alias.
6. Lone CR is never a terminator or insignificant whitespace and MUST be
   rejected as `Framing`.
7. Empty and whitespace-only record texts MUST be rejected as `Framing`.
8. Space (`20`) and horizontal tab (`09`) MAY appear as insignificant JSON
   whitespace before, after or within the one JSON value. CR is permitted only
   as the first byte of a CRLF terminator; LF is always framing.
9. Each record text MUST contain exactly one complete JSON value and no
   non-whitespace trailing bytes.
10. A UTF-8 BOM is forbidden. NUL is neither whitespace nor a terminator.
11. Every record reached in physical order MUST be valid UTF-8. Invalid UTF-8
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
| portable CLI external key | 64 lowercase hex characters | `ExternalKey`, before record processing |

This table governs lexical shape only. In particular, accepting a syntactically
valid Ed25519 point or supplied public key does not establish that it is a
trusted root. A-021 remains separate.

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

`seq` and `timestamp_nanos` MUST be JSON number tokens in this lexical form:

```text
0 | [1-9][0-9]*
```

The mathematical value MUST be in `0..=18446744073709551615`.

Negative zero, a leading plus or minus sign, fractional syntax, exponent
syntax, quoted digits, Boolean and null are forbidden even when a host JSON
library would coerce them to an integral value. No implicit float conversion
is permitted. These checks are `Schema`, before sequence comparison.

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
- Ed25519 MUST verify over exactly `"magpie-sig-v1" || hash` using the supplied
  external verifying key;
- payload validity MUST pass;
- Genesis MUST appear exactly once at record 0;
- record-0 `canonicalization_profile` MUST equal `magpie-core-v1`; and
- record-0 `verifying_key` MUST have its exact lexical shape and equal the
  lowercase encoding of the externally supplied verifying key.

The empty-snapshot acceptance rule is deliberately separate: there is no
record, Genesis, signature or key-binding claim to make.

## First-failure law

### Stable rejection classes

The portable verdict vocabulary is exactly:

| Class | Meaning |
| --- | --- |
| `ExternalKey` | supplied key text is not exactly 64 lowercase ASCII hex characters decoding to 32 bytes |
| `Framing` | byte encoding, BOM, record terminator, empty record or other file/record framing violation |
| `JsonSyntax` | one framed record is not exactly one syntactically valid JSON value |
| `Schema` | wrong JSON shape/type, duplicate/unknown/missing member, invalid u64 token/range, invalid status/payload kind, invalid core/stored hex, null, or invalid decoded string |
| `Sequence` | decoded `seq` differs from the zero-based record index |
| `PreviousLink` | decoded `prev_hash` differs from the prior verified hash, including the all-zero genesis predecessor |
| `ContentHash` | recomputed canonical `EventCore` hash differs from the stored hash |
| `Signature` | well-shaped signature does not verify over the exact domain-separated message |
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

The verifier first validates portable external-key syntax. It then processes
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
- `record_index`: zero-based index of the non-empty record that was being
  processed.

For a record-local rejection, both coordinates are present. Because empty
records are forbidden, a valid prefix ordinarily has
`line = record_index + 1`. A blank-record framing error has a physical line and
no record index. An external-key rejection has neither. A byte-encoding error
reports the physical line containing the first invalid byte where that line can
be determined by raw terminators; otherwise its coordinates may be absent.

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

### Manifest model

`manifest.json` is test metadata, not a signed Magpie history. Each case entry
MUST contain:

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
| N6 | interior empty and whitespace-only record: `REJECT(Framing)` |
| N7 | zero-byte input: `ACCEPT`, count 0, zero tip; LF-only input: `REJECT(Framing)` |
| N8 | duplicate known key at top, core, provenance and payload/discriminator levels: `REJECT(Schema)` |
| N9 | duplicate unknown key: `REJECT(Schema)` |
| N10 | one unknown member at every governed object level: `REJECT(Schema)` |
| N11 | malformed JSON, two values on one record and trailing garbage: `REJECT(JsonSyntax)` |
| N12 | invalid UTF-8: `REJECT(Framing)`, not replacement or crash |
| N13 | UTF-8 BOM: `REJECT(Framing)` |
| N14 | `NaN`, `Infinity` and `-Infinity`, including in an unknown member: `REJECT(JsonSyntax)` |
| N15 | fractional and exponent syntax for each u64 field: `REJECT(Schema)` |
| N16 | u64 overflow, negative and `-0` for each u64 field: `REJECT(Schema)` |
| N17 | every required top/core/provenance member and at least one field per payload variant omitted: `REJECT(Schema)` |
| N18 | required object/string/integer/status members replaced by null: `REJECT(Schema)` |
| N19 | unknown, case-changed, numeric, missing and null payload kind: `REJECT(Schema)` |
| N20 | unknown/case-changed/spaced/numeric closed actor, evidence and edge values: selected schema or payload-validation class and exact precedence |
| N21 | wrong sequence: `REJECT(Sequence)` |
| N22 | wrong genesis or later predecessor: `REJECT(PreviousLink)` |
| N23 | well-shaped wrong stored hash: `REJECT(ContentHash)` |
| N24 | well-shaped invalid signature: `REJECT(Signature)` |
| N25 | non-Genesis at zero and Genesis after zero: `REJECT(Genesis)` |
| N26 | correctly hashed/signed wrong canonicalization profile: `REJECT(Genesis)` |
| N27 | correctly hashed/signed genesis key mismatch: `REJECT(Genesis)` |
| N28 | all precedence pairs listed in the first-failure table, plus invalid core hex + sequence, payload + genesis and later multi-defect cases |
| N29 | valid prefix followed by malformed, bad-link, bad-hash and bad-signature final records: reject at that final coordinate; no ACCEPT summary |
| N30 | no final terminator, LF, CRLF and mixed LF/CRLF: ACCEPT when records are valid; extra final terminator, blank line and lone CR: `REJECT(Framing)` |

Additional required negative families include duplicate member names expressed
through escapes, case-changed member names, unpaired surrogates, wrong JSON
container types, NUL, external-key lexical variants, all payload-specific
non-empty rules, witness/content-hash rules and trailing bytes.

### Required positive matrix

| ID | Required proof |
| --- | --- |
| P1 | existing golden chain is accepted identically and unchanged |
| P2 | existing Deadbolt anchor chain is accepted identically and unchanged |
| P3 | Rust, Go and Python return the same event count |
| P4 | Rust, Go and Python return the same final tip |
| P5 | every valid record yields the same ordered canonical hash in all implementations |
| P6 | every signature verifies against the same exact `magpie-sig-v1` message |
| P7 | permitted object-member reordering preserves the result |
| P8 | permitted space/tab and standard string-escape variants preserve the result |
| P9 | permitted final-terminator, LF, CRLF and mixed-LF/CRLF variants behave identically |
| P10 | every multi-defect case produces the same first class and coordinate |
| P11 | Go imports/invokes no Rust or Python verification implementation and uses no shared canonicalization library |
| P12 | frozen golden and Deadbolt fixture bytes remain byte-identical |

The corpus MUST contain positive, negative and multi-defect precedence cases.
A comparison of final exit status alone is insufficient.

## Rust conformance obligation

The future Rust implementation MUST make the production/file verification path
conform, not merely add a serde toy parser used only by corpus tests. In
particular it must address:

- `FileStore` blank-line elision and exact terminator handling;
- lowercase-only stored hash, previous hash and signature transport spelling;
- duplicate and unknown members at all governed levels;
- raw JSON integer-token constraints;
- stable class/coordinate mapping; and
- the selected empty-snapshot result.

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
release-oracle candidate. The smallest repository-consistent handoff is a
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
- enforce the selected lowercase CLI-key spelling;
- separate canonical byte construction from semantic payload validation so
  hash/signature failures retain their precedence; and
- expose the same first class and coordinate as Rust and Go.

Python may continue to depend on `cryptography`. Version pinning and toolchain
reproducibility remain RQ-015, outside this contract.

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

Python will broaden one behavior: zero bytes becomes an accepted empty
snapshot with count 0 and zero tip. This follows FORMAT's explicit
“non-empty chain” genesis wording and the stable Rust empty-store identity. It
does not certify a genesis/key binding.

These are pre-1.0 reference-input-language compatibility changes. They are not
canonical format changes and MUST NOT be described as a new
`magpie-core-v1` profile.

## Trust-root boundary

The external verifying key remains selected outside the history. Successful
syntax, Ed25519 verification and genesis key equality establish integrity
relative to that supplied key. They do not establish that the key belongs to a
trusted actor, is current, is authorized or is cryptographically strong under
any additional policy.

This contract governs only the portable textual key spelling and equality
check. It deliberately does not select whether a syntactically valid 32-byte
weak or low-order Ed25519 encoding is an admissible trust root, and the v1
corpus must not smuggle that policy choice into a lexical case. A-021 owns
weak-root/strict-Ed25519 trust-root behavior and remains unchanged.

## Resource and storage boundary

This contract defines no file-size, line-size, nesting-depth, string-length,
record-count, memory, time or recursion ceiling. It does not select streaming,
atomicity, locking, durability, snapshot or crash-recovery semantics. Those
are A-008 / RQ-007 / RQ-008 and part of the wider RQ-013 assurance boundary.

Conformers must agree on all finite corpus inputs. A later resource contract
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

1. The complete N1-N30 and additional hostile matrix has exact-byte cases.
2. P1-P12 pass for Rust, Go and Python.
3. Every conformer consumes the same unmodified case bytes and manifest key.
4. Every accepted case agrees on count, tip and ordered record hashes.
5. Every rejected case agrees on first class and coordinate.
6. Rust exercises its real file/public verification path.
7. Python exercises `tools/verify_chain.py`, not a substitute parser.
8. Go independently encodes and verifies; a dependency/import/source audit
   proves P11.
9. Invalid UTF-8, duplicate keys and malformed inputs do not crash any tool.
10. Existing golden and Deadbolt fixtures and their published verifier results
    remain unchanged.
11. `docs/FORMAT.md`, canonical Rust source and historical release evidence
    remain unchanged unless a later owner-approved scope explicitly says
    otherwise.
12. A-004/RQ-006 remain administratively open until independent review, owner
    merge and separate ledger reconciliation.

## Hostile scenarios

An independent reviewer should attempt at least these attacks:

- hide a signed value behind a duplicate member whose first and last values
  differ;
- express member names, statuses or hex through JSON escapes;
- place `NaN` or invalid UTF-8 in an unknown member that a decoder might skip;
- use Python's lone-CR/universal-newline behavior or checkout line conversion;
- exploit Go's case-insensitive struct matching, duplicate-name handling or
  UTF-8 replacement;
- use `-0`, exponent syntax, huge integers, Booleans or quoted digits;
- make malformed signature hex compete with sequence/link failures;
- make invalid payload semantics compete with hash/signature/genesis failures;
- append a malformed record after a fully valid prefix;
- cause one implementation to report a later failure as the governed first
  result;
- replace a corpus file while leaving a parsed manifest representation
  unchanged;
- make Go or Python call into Rust, or one verifier relay another's result;
- drift canonical hashes while retaining the same ACCEPT Boolean;
- treat empty-snapshot acceptance as a trusted genesis/key-binding claim; or
- inflate syntactic key acceptance into A-021 trust-root authority.

## Non-goals and adjacent findings

This contract does not:

- change FORMAT, canonical bytes, payload tags, signatures or genesis;
- implement Go or modify Rust/Python;
- define resource, streaming, storage, locking, durability or atomicity law
  (A-008 / RQ-007 / RQ-008);
- change projection fallibility (A-007 / RQ-009);
- reopen the completed verified-replay event-type correction or remediate the
  residual Deadbolt eligibility portion of A-009;
- pin Python, Rust or Go dependencies/toolchains (A-011 / RQ-015);
- close the wider assurance finding (A-018 / RQ-013);
- decide weak-root or authority policy (A-021);
- add admission or `EpistemicGate` (A-022);
- add provenance-complete detached responses;
- add authority-bound corroboration (A-024);
- add a generic parser framework, dynamic verifier registry or “latest”
  profile; or
- authorize any implementation PR.

## Implementation handoff

The intended future implementation surface is narrowly:

```text
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
2. implement corpus + Rust/Python alignment + independent Go verifier
3. perform hostile independent review over the complete matrix
4. after owner merge, reconcile A-004/RQ-006 administratively in a separate PR
```

## Reviewer checklist

- [ ] No implementation language is described as normative.
- [ ] JSONL remains non-canonical transport around unchanged
      `magpie-core-v1` typed identity.
- [ ] Empty zero-byte input and LF-only input are distinguished explicitly.
- [ ] Duplicate names and unknown fields are rejected at every governed level.
- [ ] Invalid UTF-8, BOM, non-finite JSON and lone CR are decided explicitly.
- [ ] Status is exact named-string JSON, not the canonical numeric byte tag.
- [ ] Core/stored hex and payload hex stages are explicit.
- [ ] Integer token syntax is stricter than host-language numeric coercion.
- [ ] Every field and payload variant has an exact required-member set.
- [ ] Payload semantic validation remains L0-only and has explicit precedence.
- [ ] Stable classes, coordinates and multi-defect ordering are finite and
      deterministic.
- [ ] Positive results bind count, tip and ordered hashes.
- [ ] Corpus cases are exact bytes with SHA-256 bindings.
- [ ] Go is standalone, preferably standard-library-only, with no FFI/shared
      verifier logic.
- [ ] Python is retained and tightened rather than deleted.
- [ ] Frozen valid fixtures remain accepted without regeneration.
- [ ] A-021, resource/storage law, RQ-013 closure and RQ-015 remain outside.
- [ ] A-004 and RQ-006 remain Confirmed.

## Target law

> The reviewed Magpie contract and exact-byte conformance corpus define one
> portable JSONL verifier input language. Rust, independently implemented Go,
> and readable Python must return the same governed verdict, first-failure
> class and coordinate for the same bytes and supplied key, while
> `magpie-core-v1` canonical event identity and external trust-root selection
> remain unchanged.
