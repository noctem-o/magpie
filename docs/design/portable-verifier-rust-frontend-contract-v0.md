# Rust portable-verifier frontend contract v0

## Status and authority

Status: implementation contract for the next bounded Rust tranche.

This document maps the already frozen portable verifier input language into a
small Rust frontend architecture. It is not an ADR, FORMAT, a new input
language, a corpus revision, a complete-history verifier contract, an
implementation, a release contract, or finding closure.

Authority remains, in order:

1. [FORMAT](../FORMAT.md) and Accepted ADRs, especially
   [ADR-0010](../adr/0010-ed25519-verification-profile-and-external-trust-root-admissibility.md);
2. the frozen
   [portable input-language contract](portable-verifier-input-language-contract.md)
   and [A-021 verification-semantics contract](a021-ed25519-verification-semantics-contract.md);
3. the exact frozen
   [portable corpus](portable-verifier-corpus-v1.md) and its
   [manifest](../../fixtures/verifier-language-v1/manifest.json);
4. merged implementation and tests; and
5. living administrative documentation and open reference evidence.

If this document conflicts with a higher layer, the higher layer wins and the
runtime tranche stops. This document does not silently amend any accepted or
frozen source.

## Baseline and landed dependency

This contract is reconciled against `main`
`a0910cc1c4cb7cc76204b26d5eaaeee6581570b9`, the merge of PR #135. Its reviewed
implementation head is `397bb17db81aef7cfef98cc3ca3d33189f65e82a`.

PR #135 supplies the namespaced
`magpie_log::signature_profile::ProfiledSignatureVerifier`. It binds an opaque,
explicitly selected profile and exact `[u8; 32]` external-key bytes at
construction, performs external-key admissibility once, retains the original
bytes and decoded point, and verifies exact `[u8; 64]` signatures over a
`[u8; 32]` content hash. The legacy unprofiled verifier remains unchanged.

That primitive is authoritative for the Rust implementation of Accepted
ADR-0010's `V_sig` relation. It is not a raw-input parser, history conformer,
trust decision, receipt, or portable-conformance result.

The primitive is not altered in this tranche. Its direct
`curve25519-dalek = "=4.1.3"` boundary remains pinned. Any future dalek upgrade
is a separate semantic-review event, not routine parser or dependency churn.

## Contract thesis

The frontend must not deserialize an entire history into ordinary Rust values.
It owns only the distinctions that must be decided before host normalization
would make the frozen language impossible to enforce:

```text
exact input bytes
    |
    v
Magpie-owned external profile/key preflight
    |
    v
Magpie-owned sequential framing and UTF-8 boundary
    |
    v
one framed record
    |
    v
constrained JSON syntax recognition
    |
    v
duplicate-preserving closed schema and lexical decoding
    |
    v
one accepted typed frontend record + producing coordinates
    |
    v
return to caller / later complete-history conformer
```

The later history conformer processes that one record in the remaining
normative order:

```text
Sequence -> PreviousLink -> ContentHash -> Signature
-> PayloadValidation -> Genesis
```

Only after those stages succeed may the frontend advance to the next physical
record. A whole-file parse-then-validate design is non-conforming because a
malformed later record could outrank an earlier history or signature failure.

The entire immutable input may be supplied as one byte slice. Processing that
slice must still be incremental and sequential.

## Required production seam

Exact Rust type names are implementation details, but the following ownership
and typestate properties are required.

### Session preflight

A session begins with three separate inputs:

- the exact profile identity selected by the caller or configuration;
- the exact external-key transport text; and
- the immutable history bytes.

The session must perform, in order:

1. call `SignatureVerificationProfile::from_identity` with the exact identity;
2. reject every identity except
   `magpie-ed25519-canonical-prime-subgroup-v1`, with no default, alias, case
   fold, `latest`, fallback, or ambient negotiation;
3. enforce exactly 64 lowercase ASCII hex characters for the external key;
4. decode exactly `[u8; 32]` without normalization;
5. construct `ProfiledSignatureVerifier::new(profile, A_bytes)`; and
6. only after construction succeeds, inspect the first history byte.

An unsupported profile is a configuration/selection failure outside the ten
portable history result classes. It must not be invented as a portable class or
collapsed into `ExternalKey` unless the frozen contract is separately amended.
The exact external-key transport or constructor failure is
`REJECT(ExternalKey)` with no line or record coordinate.

Mapping is by typed phase, never by `Display` or `Debug` text. Successful
representation and subgroup checks do not establish key trust, custody,
identity, authority, freshness, or permission.

### Cursor, pending record, and continuation

The production API must be cursor/session-shaped, not
`Iterator<Item = Record>`, `Vec<Record>`, or a public `parse_history`
convenience function.

A ready session may inspect and decode at most the next physical record. It
returns exactly one of these internal outcomes:

- end of input;
- a governed frontend rejection; or
- a pending accepted frontend record that exclusively owns the continuation.

While a record is pending, no operation may inspect the following record. The
complete-history conformer must consume the pending record through the current
record's semantic stages. Only an explicit success transition returns a ready
continuation. Dropping or rejecting the pending record terminates evaluation.

The concrete implementation may use private states, consuming methods, or an
equivalent linear API. It must make advance-before-semantic-success unavailable
through the intended production path and test that property. No detachable
accepted-record constructor or deserializer may bypass frontend checks.

### Frontend-specific accepted product

The accepted product is internal and frontend-specific. It carries only what
the following history stages require:

- zero-based `record_index` and one-based physical `line`;
- the fully closed decoded `EventCore`, provenance, and payload shapes;
- full-width `u64` values;
- decoded fixed-size `prev_hash`, stored hash, and signature arrays;
- decoded string values required by canonical encoding and later payload or
  genesis validation; and
- a borrow or private ownership arrangement sufficient to keep the current
  input/session state alive until the caller completes the record.

Raw lexical forms are discarded after their governed lexical checks unless a
later normative stage genuinely needs the exact source bytes. The frozen law
does not define a byte-column coordinate, so this contract does not add one.
The product is not a governed receipt, `G_history`, a verified replay result, or
an authority-bearing object. It must not be publicly constructible or
deserializable merely to run the corpus.

## Magpie-owned framing and UTF-8 boundary

Framing is decided from exact bytes before JSON. Text-mode input, universal
newline translation, UTF-8 replacement, and split helpers that discard empty
slices are prohibited.

After the complete external-key gate succeeds, the frontend must implement the
frozen law exactly:

- zero bytes is a valid empty snapshot; the complete history conformer later
  returns count zero and the all-zero tip;
- LF and CRLF are terminators, may be mixed, and the final terminator is
  optional;
- a second final terminator, an interior zero-length slice, or LF-only input is
  a zero-length record and `Framing` failure;
- a non-empty candidate containing only space and/or tab is also `Framing`;
- lone CR is never a terminator or JSON whitespace and is `Framing` within the
  already identified non-empty candidate;
- exact UTF-8 BOM bytes at candidate offset zero are `Framing` for the first or
  any later candidate;
- the same U+FEFF scalar away from candidate offset zero follows ordinary JSON
  grammar, and U+FEFF inside a string is ordinary string content;
- invalid UTF-8 in the current candidate is `Framing`; and
- trailing bytes are part of the current candidate and are not silently
  discarded.

Coordinates are assigned while framing:

- `line` is one-based and counted from exact LF/CRLF boundaries;
- each non-zero-length candidate receives the next zero-based `record_index`
  before UTF-8 or JSON parsing;
- a zero-length framed slice has a line but no `record_index`;
- candidate-offset-zero BOM and lone CR have both coordinates; and
- `ExternalKey` has neither coordinate.

The cursor must not search, validate, or count later candidates while a current
record is pending semantic validation. Bookkeeping needed to identify the
current candidate is permitted; speculative later-record processing is not.

## JSON syntax adapter

JSON syntax recognition and Magpie schema/lexical decoding are distinct passes.
A convenience deserializer must not own both.

The selected Rust seam for the later tranche is the locked `serde_json 1.0.150`
syntax engine with its documented `raw_value` feature, behind the Magpie-owned
framing/UTF-8 boundary. The later runtime may enable that feature without adding
a new package. It must not enable `preserve_order`, `arbitrary_precision`, or
`unbounded_depth` as a substitute for the architecture specified here.

For each already framed, valid-UTF-8 candidate:

1. borrow the entire candidate as one `serde_json::value::RawValue` through a
   `serde_json::Deserializer`;
2. require the deserializer to reach end-of-input after permitted space/tab;
3. on failure of this complete syntax pass, return `JsonSyntax`; and
4. only after syntax success, run a second custom seed/visitor over the same
   candidate for closed schema and lexical decoding.

The first pass establishes exactly one syntactically valid JSON value, including
rejection of leading-plus/leading-zero pseudo-numerals, non-finite tokens,
trailing values, and trailing garbage. It does not decide top-level object
shape, duplicates, names, string scalar validity, u64 suitability, or schema.

Do not deserialize through `serde_json::Value`, directly into `SignedEvent`, or
into an ordinary map/struct as the authoritative portable-language path.

## Locked serde_json characterization gate

The architecture above was characterized against the exact locked source and
API for `serde_json 1.0.150` with default/`std` plus an isolated scratch probe
using `raw_value`. `Cargo.lock` binds that source to checksum
`e8014e44b4736ed0538adeecded0fce2a272f22dc9578a7eb6b2d9993c74cfb9`.
No probe artifact belongs in the repository.

### Documented/public properties used

- Borrowed `RawValue` references the exact source range occupied by one valid
  JSON value and preserves its original formatting.
- The streaming deserializer supports custom `Visitor`/`MapAccess` decoding and
  an explicit end-of-input check.
- `raw_value` is a feature of the same crate, not a new dependency package.
- Parser diagnostics expose line and column information, but those diagnostics
  are not the portable result law.

### Locked-source and probe observations

These observations are characterization evidence, not a new language:

| Input/property | `1.0.150` observation | Contract use |
| --- | --- | --- |
| literal and escape-equivalent duplicate names | `RawValue` retains both; custom `MapAccess` visits both in source order after name decoding; `Value` is last-wins | require the custom ordered visitor; forbid `Value`/map-first decoding |
| member order | custom `MapAccess` exposes encounter order; default `Value` representation may reorder | order is observed only to detect occurrences; member order remains semantically insignificant |
| numeric tokens | `RawValue` retains exact `u64::MAX`, overflow, huge, fraction, and exponent spellings; `Value` may convert large integers or normalize exponents | validate raw token text before host numeric conversion |
| leading-zero/leading-plus/non-finite tokens | complete syntax pass rejects them | map only the syntax-pass failure to `JsonSyntax` |
| standard escapes and a valid surrogate pair | ordinary string decoding yields the expected UTF-8 scalar sequence | use decoded strings in the schema pass |
| lone surrogate escape | `RawValue` syntax walk accepts the token; ordinary string decoding rejects it | syntax pass succeeds, then schema decoding maps the failure to `Schema` |
| top-level array/null | syntax succeeds; object visitor rejects shape | map to `Schema` |
| invalid UTF-8/BOM | the host parser rejects, but with host-owned diagnostics | never delegate these decisions; Magpie rejects them at `Framing` first |
| trailing value/garbage | complete syntax plus end check rejects | map to `JsonSyntax` |
| nesting depth | ordinary `Value` decoding reaches a default recursion limit; `RawValue` syntax scanning is iterative in locked source | do not turn either behavior into language law |
| diagnostics | messages, categories, and reported locations vary by parsing route | never match or expose them as normative vocabulary |

The lone-surrogate split and iterative raw syntax walk are locked-source
behaviors rather than sufficient standalone public guarantees. They are safe at
this selected seam only because Magpie owns the two phases and must pin them
with direct characterization tests. No governed result may depend on an error
string or parser category.

Conclusion: the locked version is safe at this constrained seam. The conclusion
does not bless ordinary `Value` or derived-struct deserialization, parser error
locations, or an unreviewed upgrade. Every relied-on observation is isolated
behind Magpie-owned phase boundaries and mandatory tests.

Any `serde_json` version or relevant feature-set change is semantic-review
input. Before such a change, rerun the characterization matrix and the
applicable frozen corpus. If complete syntax recognition, decoded ordered
member observation, raw number visibility, or the lone-surrogate stage split no
longer holds, stop rather than remap errors, normalize input, or silently write
a new JSON parser.

## Duplicate-preserving closed decoder

The second pass must use a purpose-specific seed/visitor/event stream that sees
each object member occurrence in encounter order. At every governed object
level it must:

1. decode the member name as a JSON string;
2. compare the decoded name exactly, without case folding or Unicode
   normalization;
3. reject a second occurrence of the same decoded name as `Schema`, including
   literal/escaped-equivalent names and duplicate unknown names;
4. reject every unknown name as `Schema`;
5. decode each known value through its field-specific path;
6. require the exact closed member set exactly once and non-null; and
7. only then construct the internal accepted shape.

These rules apply independently and recursively to `SignedEvent`, `EventCore`,
`Provenance`, and all nine payload variants. `metadata_json` is a decoded JSON
string whose contents are opaque L0 material; it is not recursively parsed.

`HashMap`, `BTreeMap`, `serde_json::Value`, and direct derived deserialization
are forbidden as the first object representation because they can erase member
occurrence evidence or assign the wrong stage.

## Lexical ownership and stage boundary

The decoder must classify the frozen fields without shifting later semantic
checks into the frontend.

| Field/rule | Required frontend behavior | Stage |
| --- | --- | --- |
| `seq`, `timestamp_nanos` | inspect the raw JSON value token; require exactly `0` or `[1-9][0-9]*`, then prove mathematical range `0..=u64::MAX` without float coercion | `Schema` after full JSON syntax succeeds |
| invalid JSON numeral spelling such as `+1`, `+0`, `01`, `00` | leave to the complete syntax pass | `JsonSyntax` |
| negative, `-0`, fraction, exponent, overflow, Boolean, null, quoted digits in a u64 field | reject before sequence comparison | `Schema` |
| `core.prev_hash`, stored `hash`, transported `signature` | after JSON string decoding, require exact lowercase ASCII hex and exact 64/64/128-character widths, then decode fixed arrays | `Schema` |
| status and `payload.kind` | require the exact frozen case-sensitive closed vocabulary and type | `Schema` |
| required members, null, JSON shape, decoded string scalar validity | enforce exactly as frozen | `Schema` |
| tag 6-8 non-empty strings, actor/evidence/edge vocabularies, witness root, optional content hash | retain decoded values without validating these rules | later `PayloadValidation` |
| Genesis position/kind, profile equality, declared-key lexical shape/equality | retain decoded values without validating these rules | later `Genesis` |

Hex comparison occurs after ordinary JSON escape decoding but before byte
decoding. Uppercase, prefixes, whitespace, odd lengths, and lookalikes are not
accepted then normalized. Raw number text is checked before conversion can
erase sign, fraction, exponent, range, or spelling distinctions.

Once a record passes `Schema`, the existing frozen canonical encoder must be
total for that accepted typed input. An encoding failure is an implementation
or operational defect, not a new portable rejection class.

## Exact first-failure composition

The production composition is:

```text
exact profile selection (operational/configuration result)
-> complete ExternalKey gate (no coordinates)
-> current record Framing
-> current record JsonSyntax
-> current record Schema
-> RETURN TO HISTORY CONFORMER
-> current record Sequence
-> current record PreviousLink
-> current record ContentHash
-> current record Signature via ProfiledSignatureVerifier
-> current record PayloadValidation
-> current record Genesis
-> unlock and advance the frontend cursor
-> repeat
```

The history conformer calls `ProfiledSignatureVerifier::verify` only at
`Signature`, with the accepted `[u8; 32]` stored content hash and `[u8; 64]`
signature. It must not reconstruct `A` through another key type, invoke legacy
`VerifyingKey::verify`, infer a profile, clear a cofactor, or alter `V_sig`.

An early record failure terminates the session without inspecting a later
record. In particular, an early `Signature` failure beats malformed later
bytes, while a current-record `Schema` failure beats any current-record chain
or cryptographic defect. The frozen within-`Genesis` order remains
position/kind, profile equality, then declared-key lexical validity/equality.

## Parser diagnostics and portable rejections

Host errors remain private diagnostics:

```text
host/parser diagnostic
    -> Magpie-owned phase classification
    -> exact frozen rejection class + frozen coordinate
```

Only a failure from the complete first syntax pass maps to `JsonSyntax`.
Failures from the second, already-syntax-valid closed decoder map to `Schema`
when the frozen schema law says so. Framing and UTF-8 failures never enter
`serde_json`. Library error types and `Display`/`Debug` strings must not appear
in the normative result vocabulary or public matching surface.

If the second pass reports a syntax inconsistency after the first pass
succeeded and no frozen `Schema` rule explains it, that is an internal
conformance defect or operational failure. It must not be relabelled `Schema`.

Private diagnostics may retain parser details for developers. Magpie must not
promise stable parser byte columns or exact messages. File-not-found,
permission, allocation, dependency, and internal failures remain operational
results outside `ACCEPT`/`REJECT`.

## Resource safety

The frozen input-language contract defines no file-size, line-size, nesting-
depth, string-length, record-count, memory, time, or recursion ceiling. This
contract does not add one.

The runtime implementation must avoid obvious unbounded materialization: it
must process one framed record at a time, avoid a whole-history value tree, and
count any future bounded quantity before allocating where practical. A needed
pre-alpha safety ceiling not already frozen is implementation policy and an
operational result, not an invisible language restriction. If a finite
normative corpus case cannot run safely, that remains a conformance defect.

Any proposed normative resource limit requires a separate governing decision.
Neither `serde_json`'s default recursion limit nor `raw_value`'s current
iterative scan defines Magpie's language.

## Corpus accounting for the frontend tranche

The frozen manifest contains 432 cases:

| Final result/stage | Cases |
| --- | ---: |
| `ACCEPT` | 19 |
| `ExternalKey` | 21 |
| `Framing` | 13 |
| `JsonSyntax` | 18 |
| `Schema` | 272 |
| later `Sequence` through `Genesis` rejection | 89 |
| **Total** | **432** |

The frontend tranche can produce the complete governed result for the 324
cases ending at `ExternalKey`, `Framing`, `JsonSyntax`, or `Schema`. The other
108 cases comprise 19 eventual accepts and 89 later-stage rejections. For those
cases the frontend obligation is to yield the applicable current record without
preempting its later result; the complete history conformer owns the final
verdict.

A test-only driver may model semantic success solely to exercise a later
frontend record, but it must not claim complete corpus conformance. The runtime
PR must report exact accounting rather than saying that a frontend-only tranche
"passes all 432". Corpus bytes, expectations, manifest, and sidecar remain
unchanged.

## Required tests for the later runtime PR

The later implementation must use focused production-path tests plus the
applicable exact corpus cases.

### Framing

- zero input under valid and invalid external keys;
- first/later candidate BOM and U+FEFF inside/outside strings;
- LF, CRLF, mixed terminators, optional final terminator, and extra final
  terminator;
- first/later/terminal lone CR;
- zero-length and space/tab-only records;
- invalid UTF-8 without replacement or crash; and
- trailing framing anomalies with exact frozen coordinates.

### JSON and schema

- malformed syntax, non-finite tokens, two values, and trailing garbage;
- top-level non-object shapes;
- every known-member duplicate and unknown member at every governed object;
- duplicate names equal only after escape decoding;
- missing/null members and exact closed shapes;
- allowed member-order permutations;
- standard escapes, literal/escaped U+FEFF, valid surrogate pairs, and lone
  surrogates; and
- direct `serde_json 1.0.150` characterization tests for the two-pass seam.

### Numbers

- `0`, `2^53 + 1`, `2^63`, and `u64::MAX` where frozen;
- `+1`, `+0`, `01`, and `00` as `JsonSyntax`;
- negative integers, `-0`, fractions, exponents, Boolean, null, and quoted
  digits as `Schema`;
- `u64::MAX + 1` and an excessively large integer token without float
  normalization; and
- upper `seq` cases reaching `Sequence`, not `Schema`.

### Hex and vocabulary

- correct lowercase and escaped-equivalent lowercase strings;
- uppercase/mixed case, wrong or odd length, non-hex, empty where forbidden,
  whitespace, prefix, and Unicode lookalikes;
- exact fixed-array decoding for previous hash, stored hash, and signature;
- frontend schema vocabularies versus later `PayloadValidation` and `Genesis`
  vocabularies, proving the stage boundary; and
- no accept-then-normalize path.

### Precedence and interface

- unsupported profile remains outside the portable result vocabulary;
- invalid `ExternalKey` plus malformed/empty history yields `ExternalKey`;
- early `Signature` or other history failure plus malformed later record never
  inspects the later record;
- early `Schema` plus later cryptographic failure yields current-record
  `Schema`;
- frozen multiple-defect cases retain their exact class and coordinate;
- no accepted record can be publicly constructed or deserialized without all
  frontend checks;
- no next-record operation is available while the current record is pending;
  and
- parser diagnostic strings are not matchable normative vocabulary.

### Corpus and integrity

- execute all 324 frontend-final cases through the production frontend;
- account for all 108 remaining cases without claiming a frontend-final
  verdict;
- prove representative later-record cases through the staged continuation;
- run `tools/check_verifier_corpus_manifest.py`; and
- do not alter corpus, manifest, sidecar, FORMAT, golden, or Deadbolt bytes.

## Non-normative fuzz/metamorphic handoff

A separate assurance tranche is reserved after the Rust frontend and complete
history conformer. The frozen 432-case corpus remains the normative oracle.
Generated, mutated, differential-parser, property, or fuzz cases are
non-normative falsification evidence. Concrete bugs may become ordinary
permanent regression tests, but fuzzing never mutates or repoints corpus v1.

That later tranche may evaluate cargo-fuzz/libFuzzer, proptest or another
justified generator, parser differential fuzzing, accepted-record mutation, and
precedence-preservation properties. This contract does not select a tool.

## Explicit non-goals and non-closures

This contract does not include or establish:

- complete history verification runtime or the later sequence/link/hash/
  signature/payload/genesis composition;
- a Python conformer, Go conformer, or 432-by-3 differential harness;
- a fuzz corpus or assurance implementation;
- receipts, `G_history`, or ADR-0008 runtime completion;
- legacy verifier migration or a changed `V_sig` relation;
- key trust, custody, identity, authority, admission, or permission;
- latest state, freshness, completeness, canonical history, or
  non-equivocation;
- standing, currentness, release hardening, or release evidence; or
- closure of A-021, A-004, RQ-006, RQ-013, or any other finding.

The eventual Python and Go conformers implement the same governing law
independently. They are not required to copy Rust's host-parser architecture.

## Runtime-tranche stop conditions

Stop for owner review rather than improvising if:

1. a higher governing source or exact corpus case conflicts with this contract;
2. exact first-failure order or coordinates are ambiguous;
3. the locked syntax engine cannot expose complete syntax, ordered decoded
   member occurrences, or raw number tokens at the selected seam;
4. a required classification depends on undocumented parser behavior that is
   not isolated and pinned by Magpie-owned tests;
5. duplicate decoded names would be overwritten before observation;
6. raw u64 or lowercase-hex distinctions would be normalized before checking;
7. implementation appears to require a complete new JSON parser;
8. an accepted frontend product appears to require a new constitutional
   producing-coordinate decision, receipt, or `G_history`;
9. a new ADR, FORMAT/profile/corpus change, or legacy-path switch appears
   necessary; or
10. a resource ceiling would silently change accepted language.

## Acceptance criteria for the next tranche

The runtime tranche is bounded and reviewable only when:

- production code follows the staged session seam rather than a corpus-only
  adapter;
- exact profile/key preflight completes before any history byte is inspected;
- Magpie owns byte framing, UTF-8, coordinates, duplicate evidence, closed
  schema, raw u64 text, and lowercase hex;
- `serde_json` is confined to the characterized two-pass seam;
- one accepted record returns to the history caller before later bytes are
  inspected;
- all 324 frontend-final corpus cases and exact 108-case remainder accounting
  are demonstrated;
- no protected byte, legacy verification behavior, or finding disposition is
  changed; and
- diagnostics, types, and documentation make no trust or complete-conformance
  claim.

## Target architecture law

> The Rust portable frontend validates the exact external profile/key gate,
> frames and decodes only the current exact-byte record, preserves every frozen
> duplicate and lexical distinction, and yields one private typed record to the
> complete-history conformer. It cannot advance until that caller completes the
> current record's normative semantic stages. The merged
> `ProfiledSignatureVerifier` remains the downstream `Signature` primitive;
> neither the frontend nor its accepted product establishes trust, history
> completeness, portable conformance, or authority.
