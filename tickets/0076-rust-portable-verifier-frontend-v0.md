# Ticket 0076: Rust portable-verifier frontend v0

## Status

Documentation-only implementation contract handoff. This ticket and its
documentation PR do not implement or themselves authorize runtime work. Human
owner authority remains exclusive.

Owner merge of the companion
[Rust portable-verifier frontend contract v0](../docs/design/portable-verifier-rust-frontend-contract-v0.md)
freezes the architecture and makes a separate bounded runtime tranche the next
candidate. It does not itself implement the frontend, authorize unrelated work,
or close a finding. Runtime work still requires the repository's normal
separate implementation authority and review.

## Goal

Implement the exact Rust raw-input frontend for the frozen portable verifier
language. The frontend must preserve exact-byte framing, UTF-8, decoded
duplicate-member evidence, closed schema, raw u64 spelling, lowercase hex, and
record-by-record first-failure ordering. It must yield one accepted private
typed record at a time to a later complete-history conformer and retain merged
`ProfiledSignatureVerifier` as the downstream signature primitive.

## Scope

The later runtime PR may:

- add a small private/namespaced frontend module in `magpie-log`;
- add focused production-path tests and a corpus adapter for frontend-owned
  stages;
- perform exact profile/external-key preflight before touching history bytes;
- frame an immutable input slice incrementally using Magpie-owned byte rules;
- use locked `serde_json 1.0.150` behind that boundary as the constrained
  two-pass syntax/schema engine described by the contract;
- enable the existing crate's `raw_value` feature if the implementation proves
  the locked characterization gate and adds no new package;
- construct one internal accepted frontend record with frozen coordinates and
  fixed-size decoded values; and
- expose only the smallest internal staged seam needed by the following
  complete-history tranche.

Exact source placement must follow then-current crate conventions. A production
path, not a test-only toy parser, is required.

## Non-goals

Do not:

- implement complete history verification, sequence/link/hash/signature/
  payload/genesis composition, Python, Go, or differential execution;
- change FORMAT, an Accepted ADR, the frozen language, corpus, manifest,
  sidecar, golden bytes, or Deadbolt fixtures;
- change `ProfiledSignatureVerifier`, Accepted ADR-0010's relation, or the
  legacy unprofiled verifier;
- deserialize directly into `SignedEvent` as the authoritative portable path;
- add a complete handwritten JSON parser or a generic parser framework;
- create a public detachable receipt, `G_history`, or authority-bearing result;
- add trust, custody, identity, admission, latest/freshness/completeness/
  non-equivocation, standing, or release semantics; or
- close or change the controlled disposition of any audit finding.

## Governing sources

Read and apply in this order:

1. [FORMAT](../docs/FORMAT.md) and Accepted
   [ADR-0010](../docs/adr/0010-ed25519-verification-profile-and-external-trust-root-admissibility.md);
2. [portable input-language contract](../docs/design/portable-verifier-input-language-contract.md),
   [A-021 verification-semantics contract](../docs/design/a021-ed25519-verification-semantics-contract.md),
   and this ticket's
   [frontend contract](../docs/design/portable-verifier-rust-frontend-contract-v0.md);
3. [portable corpus contract](../docs/design/portable-verifier-corpus-v1.md),
   exact [manifest](../fixtures/verifier-language-v1/manifest.json), and exact
   corpus bytes;
4. merged
   [`signature_profile` implementation](../crates/magpie-log/src/signature_profile.rs)
   and focused
   [tests](../crates/magpie-log/tests/signature_profile.rs); and
5. current living audit/convergence documents.

Open PRs #133 and #134 are reference evidence only and are not authority.

## Required architecture

The production composition must be:

```text
exact profile selection
-> exact external-key text and [u8; 32] decoding
-> ProfiledSignatureVerifier::new
-> current-record exact framing / UTF-8
-> current-record complete JSON syntax pass
-> current-record ordered duplicate-preserving schema / lexical pass
-> one private accepted record + exclusive continuation
-> return to later history conformer
```

The cursor/session must not inspect a later record until its caller explicitly
completes the current record's `Sequence`, `PreviousLink`, `ContentHash`,
`Signature`, `PayloadValidation`, and `Genesis` stages. Do not return an
iterator or pre-parsed history vector.

The custom decoder must observe decoded member occurrences in order before any
map/struct overwrite; reject duplicates, unknowns, missing/null values, and
wrong shapes; validate raw u64 tokens before conversion; validate lowercase
fixed-width core/stored hex before byte decoding; and leave payload/genesis
rules at their later frozen stages.

The output type is internal, frontend-specific, and not publicly
constructible/deserializable. It carries only the current record's frozen line/
index, closed decoded values, fixed-size arrays, and lifetime/continuation state
needed by the later conformer.

## Host parser characterization gate

The implementation must pin tests against the exact selected `serde_json`
version and relevant feature set for:

- literal and escape-equivalent duplicate names observed in encounter order;
- exact raw number text, including `u64::MAX`, overflow, huge integers,
  fractions, and exponents;
- leading-zero/leading-plus/non-finite syntax rejection;
- standard escapes, valid surrogate pairs, and lone-surrogate phase behavior;
- top-level non-object values;
- trailing values/garbage and explicit end-of-input;
- deep nesting without treating host limits as normative; and
- diagnostic text/location independence.

Magpie must own invalid UTF-8 and BOM before JSON. Use a complete `RawValue`
syntax pass followed by a custom seed/visitor over the same record. Do not use
`serde_json::Value`, a map-first representation, or direct `SignedEvent`
deserialization as the authority path.

Any `serde_json` version/feature update is semantic-review input. If the locked
properties fail or require unisolated undocumented behavior, stop. Do not
silently remap diagnostics or write a full parser.

## First-failure requirements

The entire external-key gate wins before framing, including for empty input.
Unsupported profile selection remains an operational/configuration result
outside the ten portable classes. Map failures by typed phase, not error text.

For each current record, preserve exactly:

```text
Framing -> JsonSyntax -> Schema
-> return to caller
-> Sequence -> PreviousLink -> ContentHash -> Signature
-> PayloadValidation -> Genesis
-> advance
```

An early semantic or signature failure must terminate without inspecting
malformed later bytes. A zero-length slice has line/no index; every non-empty
candidate receives line/index before UTF-8/JSON; external-key failure has
neither. Do not add byte-column law.

## Tests required of the runtime PR

### Framing and UTF-8

- valid/invalid key combined with zero, empty-record, and malformed input;
- BOM at first/later candidate offset zero and permitted U+FEFF string content;
- LF, CRLF, mixed, optional/extra final terminators, interior blanks;
- first/later/terminal lone CR, whitespace-only candidates, invalid UTF-8; and
- exact frozen coordinates and no later-record prevalidation.

### JSON and closed schema

- malformed/non-finite/trailing JSON and top-level non-object values;
- all known duplicates, duplicate unknowns, escaped-equivalent duplicate names;
- all unknown/missing/null members at every governed shape;
- permitted member reorderings and Unicode/string escape boundaries; and
- parser characterization failures remaining private and phase-mapped.

### Numbers and hex

- full valid u64 boundaries, `2^53 + 1`, `2^63`, and `u64::MAX`;
- leading plus/zero as `JsonSyntax`;
- negative, `-0`, fraction, exponent, wrong type, `u64::MAX + 1`, and huge
  integers as `Schema`;
- lowercase fixed widths, uppercase, mixed case, wrong/odd length, non-hex,
  prefix, whitespace, empty, and Unicode lookalikes; and
- proof that no numeric or hex normalization precedes rejection.

### Precedence and interface

- invalid `ExternalKey` beats malformed/empty history;
- early `Signature` failure beats malformed later record;
- current-record `Schema` beats later/current cryptographic defects;
- frozen multiple-defect cases retain exact class/coordinate;
- accepted records cannot be constructed without all frontend checks;
- the next record is inaccessible until the pending record is explicitly
  released after semantic success; and
- no normative match depends on parser or private verifier diagnostic strings.

### Corpus and integrity

- execute the exact 324 frontend-final cases: 21 `ExternalKey`, 13 `Framing`,
  18 `JsonSyntax`, and 272 `Schema`;
- account explicitly for the remaining 108 cases: 19 eventual accepts and 89
  later-stage rejections, without claiming frontend-only final conformance;
- consume applicable frozen cases without changing any bytes or expectation;
  and
- run the corpus integrity checker and protected-byte comparisons.

## Stop conditions

Stop and request owner review if:

1. governing sources or corpus cases conflict;
2. exact precedence or coordinates are ambiguous;
3. decoded duplicate names or raw numeric tokens cannot be observed before
   normalization/overwrite;
4. the locked parser cannot support the two-pass seam without unisolated
   undocumented behavior;
5. a full new JSON parser, new dependency package, new ADR, or FORMAT/profile/
   corpus change appears necessary;
6. the frontend output requires new constitutional producing coordinates,
   receipt semantics, or `G_history`;
7. a normative resource limit would be introduced invisibly;
8. the legacy verifier or merged `V_sig` would need to change; or
9. any finding disposition would need to change.

## Acceptance criteria

- Production code implements the staged session/continuation boundary.
- Exact profile and complete external-key admissibility precede history access.
- Exact framing, UTF-8, duplicate, schema, u64, hex, class, and coordinate laws
  are Magpie-owned and tested.
- Locked parser characterization tests pass at the selected seam.
- All 324 frontend-final corpus cases pass with explicit 108-case remainder
  accounting.
- No protected artifact, legacy behavior, or unrelated runtime is changed.
- Public API growth is absent or separately justified as strictly necessary.
- Validation is green and a hostile review finds no normalization or
  first-failure bypass.

## Explicit non-closures

Completion of this ticket would still not establish complete portable
verification, A-021 closure, A-004 closure, RQ-006 closure, RQ-013 closure,
ADR-0008 runtime completion, Python/Go conformance, differential evidence,
trust/authority, freshness/completeness/non-equivocation, receipts, release
evidence, or release readiness.
