# Ticket 0072: Portable verifier input-language contract

## Status

Documentation-only implementation contract candidate for A-004 / RQ-006.

This ticket does not implement a parser, verifier, Go tool, fixture, test or CI
change and does not close either finding. Human owner merge authority remains
exclusive.

Required sequence:

```text
contract merge
-> dedicated owner-ratified A-021 Ed25519 verification-semantics contract
-> complete corpus plus Rust/Go/Python implementation of #120 and A-021
-> independent hostile review
-> owner merge
-> separate living-ledger reconciliation
```

The A-021 contract is the preferred next architectural step and a hard
prerequisite before portable verifier implementation, any general
signature-verification equivalence claim or full A-004/RQ-006 closure.

## Baseline

```text
repository: noctem-o/magpie
worktree: C:\Users\herpe\.codex\worktrees\magpie-portable-verifier-language-contract
branch: agent/portable-verifier-language-contract
starting HEAD: f6a4759ef3cdc2a3974aad4e2be57df58d04c1c3
origin/main: f6a4759ef3cdc2a3974aad4e2be57df58d04c1c3
merge-base(HEAD, origin/main): f6a4759ef3cdc2a3974aad4e2be57df58d04c1c3
live refs/heads/main: f6a4759ef3cdc2a3974aad4e2be57df58d04c1c3
```

The isolated worktree was clean and exactly based on live main. No tracked
Ticket 0072 and no open PR owning A-004/RQ-006 portable-verifier contract work
was found before editing.

## Finding IDs

- A-004 — Rust and Python independent verifiers accept different serialized
  record languages and can disagree on first failure.
- RQ-006 — cross-language parser and validation mismatch lacks one executable
  portability contract and differential corpus.

Both remain **Confirmed**. The future corpus materially improves RQ-013
evidence but does not close RQ-013's wider resource, crash or exact-boundary
assurance scope. A-021 also remains **Confirmed** and now records a required
upstream decision defining the exact Ed25519 signature-verification relation.

## Governing contract

[`docs/design/portable-verifier-input-language-contract.md`](../docs/design/portable-verifier-input-language-contract.md)
is normative for the future implementation slice.

No implementation is normative. The reviewed contract plus exact-byte corpus
define the language and verdict law.

## Problem statement

The signed identity of an event is its typed `EventCore` encoded by the frozen
`magpie-core-v1` binary encoder. JSONL is a replaceable reference transport,
not the hash/signature preimage.

Current Rust and Python nevertheless expose different languages at that
transport boundary. Discovery confirmed disagreements in empty and blank
input, lone CR, invalid UTF-8, duplicate names, non-finite JSON, mixed-case
hash/signature/link hex, numeric status, u64 token handling and payload versus
cryptographic first-failure ordering. Shared permissiveness also leaves
unknown members and duplicate unknown names accepted without a contract.

Two hostile witnesses limit the equivalence claim. Ordinary verification
accepts attacker-constructed signatures under the canonical low-order root
`ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f`, while
strict verification rejects them. It can also accept a keyholder-constructed
signature under the ordinary golden key when `R` is the identity point and
canonical `S` satisfies the ordinary equation, while strict verification
rejects the small-order `R`. The disputed boundary is therefore the complete
verification relation, not a class of unusual external keys. This ticket does
not select either behavior.

The correction is one strict, explicit portable JSONL grammar and one stable
verdict model over its governed conformance domain, without changing canonical
bytes.

## Selected contract summary

- Zero bytes is an accepted empty snapshot with count 0 and zero tip; it makes
  no Genesis/key-binding claim. LF-only input is a zero-length framed record
  and rejects with a physical line but no `record_index`.
- LF and CRLF separators may be mixed; the final terminator is optional;
  zero-length lines, whitespace-only candidates, extra final terminators and
  lone CR reject. A physically non-empty whitespace-only candidate receives
  the next `record_index` before failing `Framing`.
- Input is valid UTF-8 without BOM and contains exactly one JSON object per
  non-empty record.
- Duplicate names reject at every governed object level. N10 independently
  rejects an unknown member in `SignedEvent`, `EventCore`, `Provenance` and
  each of the nine payload variants.
- Key order, space/tab JSON whitespace and equivalent standard JSON string
  escapes are permitted; non-finite values and unpaired surrogates reject.
- Every SignedEvent/core/provenance/payload variant has exact required,
  non-null members with no defaults.
- Core/stored hash and signature spellings are exact lowercase ASCII hex.
- #120 governs signature transport as exactly 128 lowercase hex characters
  decoding to 64 bytes; well-shaped transport is not proof of signature
  validity. A-021 supplies the general mathematical predicate at the existing
  `Signature` stage.
- The external key must pass lowercase syntax, exact 32-byte decoding and the
  contract-owned canonical RFC 8032 Edwards25519 point-representation gate
  before any framing or record processing. That gate requires encoded
  `y < 2^255 - 19`, successful point recovery, rejection of sign bit 1 when
  recovered `x = 0`, and byte-identical canonical re-encoding. Representation
  failure wins even for zero-byte input.
- Canonical external-key representation does not decide key trust, strength or
  signature validity. A21-D1 and A21-D2 preserve key-sensitive and
  ordinary-root signature-sensitive divergence without premature verdicts;
  they are witnesses, not an exhaustive A-021 classifier.
- JSON statuses are exact named strings, never numeric aliases.
- JSON syntax is decided before the u64 schema. Invalid JSON numeral spellings
  such as `+1` and `01` are `JsonSyntax`; syntactically valid negative, `-0`,
  fractional, exponent, overflow, Boolean, null and quoted-digit values that
  violate the selected u64 law are `Schema`. No coercion is permitted.
- Payload kind and ADR-0002 vocabularies are closed and case-sensitive.
- `metadata_json` remains an opaque string.
- Existing canonical bytes, hash/signature messages, Genesis and frozen
  fixtures remain unchanged.

Stable rejection classes are:

```text
ExternalKey
Framing
JsonSyntax
Schema
Sequence
PreviousLink
ContentHash
Signature
PayloadValidation
Genesis
```

Per-record precedence is exactly:

```text
Framing
-> JsonSyntax
-> Schema
-> Sequence
-> PreviousLink
-> ContentHash
-> Signature
-> PayloadValidation
-> Genesis
```

The portable result binds ACCEPT/REJECT, first class, one-based physical line
and zero-based record index where defined. ACCEPT also binds event count, tip
and ordered recomputed hashes in the normative corpus. A dependency sentinel
has no portable verdict yet and does not create a third runtime result.

`record_index` counts non-zero-length framed candidates before UTF-8/JSON
parsing. A zero-length record has a line and no index; a non-empty space/tab-only
candidate has its line and next zero-based index. Both reject as `Framing`.

The `Signature` stage remains in the ordering. For exact normative fixtures it
uses the assigned expected result; for arbitrary well-shaped signatures its
general verification predicate comes from the future owner-ratified A-021
contract.

## Exact future change surface

The separate implementation PR is expected to touch only the narrow surfaces
needed for conformance:

```text
.gitattributes (path-specific no-text override for the new case subtree only)
fixtures/verifier-language-v1/**
crates/magpie-log parsing/verification code
focused magpie-log conformance tests or runner
tools/verify_chain.py
tools/go-verify-chain/**
one differential orchestration surface
the existing CI workflow entries needed to run all three conformers
one later implementation ticket
```

Exact Rust source paths must follow the then-current implementation rather than
being guessed here. The implementation must exercise the real `FileStore` /
production verification path, not a test-only serde substitute.

The `.gitattributes` change is authorized only to override the existing
`*.jsonl text eol=lf` rule for new exact-byte hostile cases, using a validated
path-specific binary/no-text rule. It must not alter global JSONL policy or
move/regenerate the existing frozen fixtures.

Protected surfaces below remain unchanged unless the owner separately expands
scope. A need to edit FORMAT or mint a new profile is a stop condition.

## Future Go role

Go is the independent verifier and release-oracle candidate. It is not a
universally equivalent release oracle for arbitrary well-shaped signatures
until A-021's complete verification relation is ratified and implemented. The
intended isolated home is
`tools/go-verify-chain/`.

It must:

- implement parsing, canonical encoding, hashing, Ed25519, chain checks,
  payload validation and Genesis independently;
- never link to or invoke Rust or Python;
- use no generated/shared Magpie verification logic;
- prefer only Go standard-library cryptography and JSON primitives;
- independently enforce the contract's canonical RFC 8032 point gate,
  including `y` range, recovery, zero-`x` sign and re-encoding checks, even
  though `crypto/ed25519` can store arbitrary 32-byte slices; if the standard
  library cannot support that check, stop for owner review before adding a
  dependency or weakening the rule;
- explicitly constrain permissive `encoding/json` behavior;
- use no cgo without a separately reviewed need;
- use no network access during verification; and
- build as one small deterministic local command.

Go must not make its own ordinary-versus-strict behavior normative for
A21-D1, A21-D2 or any other well-shaped signature. Standard-library-only
remains preferred; this ticket authorizes no new dependency to implement either
the representation law or future A-021 policy.

A minimal `go.mod` is permitted in the future implementation. No Go code or
module is added by this ticket.

## Python role

`tools/verify_chain.py` remains the tiny readable independent/reference
conformer. The implementation must tighten byte framing, duplicates, unknown
members, non-finite numbers, integer tokens, status, CLI-key spelling and
first-failure behavior while retaining an independently written canonical
encoder. Python may continue to use `cryptography`; RQ-015 remains separate.

## Differential corpus requirements

Future location:

```text
fixtures/verifier-language-v1/
  manifest.json
  cases/*.jsonl
```

Every case file is exact bytes and is bound by `input_sha256`. Corpus metadata
must distinguish normative conformance cases from dependency sentinels
mechanically. Normative entries bind case ID, repository-root-relative path,
supplied external key, expected verdict, class, line, record index and purpose;
accepted cases additionally bind event count, tip and ordered event hashes.
Dependency sentinels have no expected portable verdict and cannot contribute
to conformance pass totals. P1/P2 reference the existing golden and Deadbolt
fixtures in place; new inputs live under `cases/`.

The implementation must add and prove an effective no-text Git attribute for
`cases/`. In a fresh checkout/worktree, every checked-out case SHA-256 must
equal the manifest, including literal CRLF, mixed-EOL, unterminated and
invalid-UTF-8 cases, with no inserted BOM/newline and clean Git status after
the runner reads them.

Malformed, duplicate-key, invalid-UTF-8 and framing cases must never be
represented only as parsed/reserialized JSON. The corpus must contain positive,
negative and multi-defect precedence inputs.

Invalid JSON numerals such as `+1` and `01`, and the zero-length versus
space/tab-only N6 records, must be committed as exact literal bytes. They must
not be normalized or produced independently by each conformer's serializer.

## Normative behavior matrix

The complete discovered current-behavior table, selected future law, exact
schema and compatibility analysis are in the governing contract. The
implementation must not replace that table with “match Rust,” “match Python”
or “match Go's standard library.”

## Negative and hostile proof obligations

Implement the full contract N1-N30 matrix, including:

- case and width variants for every hex role;
- numeric/unknown status;
- N6 zero-length and non-empty whitespace-only records with their distinct
  line/index expectations, plus empty input, BOM, UTF-8 and line-ending cases;
- duplicate known/unknown names and N10 unknown-member cases for
  `SignedEvent`, `EventCore`, `Provenance` and each of the nine payload variants;
- non-finite JSON, malformed JSON and wrong shapes;
- invalid JSON u64 spellings (`+1`, `+0`, `01`, `00`) as `JsonSyntax`, and
  syntactically valid fraction, exponent, overflow, negative, `-0`, Boolean,
  null and quoted-digit cases as `Schema`, covering both u64 fields;
- for every required member coordinate in `SignedEvent`, `EventCore`,
  `Provenance` and all nine payload variants, one individual omission vector
  (N17) and one individual null vector (N18), each `REJECT(Schema)`;
- sequence, link, hash, signature and Genesis failures;
- all specified multi-defect first-failure pairs; and
- valid prefixes with defective later records.

The case ID prefix `N` denotes a hostile obligation, not necessarily REJECT:
zero-byte N7 is ACCEPT under the selected law only after the complete external
key gate passes, and N30 contains accepted and rejected framing variants.

Also implement K1-K7 and K9-K15 for the external-key gate: malformed length,
case and non-hex; a lowercase 32-byte representation with no Edwards point;
that representation plus zero-byte input; ordinary fixture keys; encoded
`y = p`; encoded `y = p + 1`; no-square-root recovery; zero-`x` sign-bit 1
rejection; zero-`x` sign-bit 0 acceptance; golden-key canonical re-encoding;
and a valid nonzero-`x`, sign-bit-1 round trip. Representation failures are
`REJECT(ExternalKey)` with no coordinates. Exact key bytes and selected results
are frozen in the governing contract.

K8 is replaced by two mandatory A-021 dependency witnesses:

- A21-D1 binds the canonical low-order root
  `ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f` and
  committed exact forged-chain bytes reproducing the preserved divergence.
- A21-D2 binds the ordinary golden root and committed exact bytes for a
  keyholder-constructed strictness-sensitive signature whose `R` is the
  identity point and whose canonical `S` satisfies the ordinary equation.

Neither has an expected portable ACCEPT/REJECT before A-021. They prove that
the dependency spans key and signature behavior but do not enumerate the
complete Ed25519 boundary. The runner must reject any attempt to count either
as a normative pass or assign it a premature verdict. After A-021 is
implemented by all conformers, both exact cases and any additional A-021
vectors become normative under the selected relation.

Schema coverage must be machine-checkable through one shared inventory. N10
must map each exact object shape—`SignedEvent`, `EventCore`, `Provenance` and
each of the nine payload variants—to exactly one unknown-member case. N17/N18
must separately map every required member coordinate (top/core/provenance plus
`kind` and every additional payload member) to exactly one omission and one
null case. The runner fails on missing or ambiguous mappings, or when a new
payload variant lacks N10 coverage. This remains conformance metadata, not a
runtime schema registry; committed SHA-bound case bytes are authoritative even
if authoring is generated.

Couple the expected coordinate set mechanically to the current Rust types with
test-only exhaustive field destructuring/variant matching (or an equally
direct non-runtime mechanism), so a new governed field or payload variant
cannot compile or pass conformance until the inventory, both member mutations
and that variant's unknown-member case are updated.

## Positive proof obligations

Implement P1-P12:

- unchanged golden and Deadbolt acceptance;
- equal count, tip and per-record hashes across Rust/Go/Python;
- equal exact signed message verification;
- allowed key-order, whitespace/escape and line-ending equivalences;
- equal first class/coordinate for every precedence vector;
- an independence audit proving Go shares no verifier implementation; and
- byte identity of the frozen histories.

Additionally prove that the golden fixture key passes the complete
canonical representation gate and re-encodes byte-identically, K13-K15 cover
the positive RFC point boundaries, every new case survives Git checkout
byte-identically, shared N10/N17/N18 coverage is mechanically exhaustive, and
both A-021 witnesses are excluded from normative conformance totals pending
ratification.
P6 binds exact normative signature fixtures and their exact signed messages;
acceptance of a frozen signature does not define every signature under that
key. P1-P12 must not be presented as a general Ed25519 verification law.

## Protected surfaces

The implementation must not change merely to satisfy this contract:

- `docs/FORMAT.md` or `magpie-core-v1` canonical encoding;
- `EventCore`, `Payload` or `SignedEvent` persisted representation;
- SHA-256, `magpie-sig-v1`, Ed25519 verification mode or Genesis semantics;
  any later ordinary/strict change requires the separately owner-ratified
  A-021 decision rather than inference from this ticket;
- existing golden or Deadbolt fixture bytes;
- historical audits, ADRs or the v0.1.0 release contract;
- A-004/RQ-006 disposition before the separate administrative gate; or
- adjacent authority, storage, resource, admission or standing policy.

This contract PR itself changes only this ticket and the governing design
document.

## A-021 handoff

The preferred next contract must answer what exact Ed25519 verification
relation Magpie means by “signature verifies.” Without answering it here, that
review must cover public-key representation versus semantics; ordinary/strict
verification; key and `R` torsion/small-order behavior; `R` encoding; `S`
range/canonicality; cofactor/equation rules; RFC and locked Rust/Python/Go
compatibility; frozen-history compatibility; additive versioning if required;
and exact hostile vectors. No ADR, ticket number, policy or implementation is
created by Ticket 0072.

## Future validation requirements

The implementation PR must at minimum run:

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked --no-fail-fast
cargo test --doc -p magpie-log --locked
Go format/vet/test/build commands selected by the implementation contract
Python frozen-chain verifier commands
all three conformers over every normative exact-byte manifest case
dependency-sentinel classification that cannot masquerade as conformance
Git attribute inspection plus fresh-checkout SHA-256 verification for every case
machine-checkable N10 unknown-member coverage over every governed object shape
machine-checkable N17/N18 omission/null coverage over every required member coordinate
exact N6 line/index distinction and JsonSyntax-versus-Schema integer staging
python tools/check_release_metadata.py
git diff --check
```

It must report exact normative and sentinel totals separately, no crashes,
count/tip/hash equivalence, first-failure equivalence, dependency/import audits
and protected-file hashes.

## Explicit non-goals

- No runtime, verifier, fixture, test, Go, module or CI implementation here.
- No `.gitattributes` change in this contract PR; its scoped corpus override
  belongs to the later implementation.
- No new canonicalization, event or transport profile.
- No JSON-byte canonicalization.
- No definition of the Ed25519 verification relation owned by A-021, including
  weak/low-order keys, signature `R`, scalar `S`, cofactor or
  ordinary-versus-strict semantics; this ticket records A-021 as the preferred
  next contract and a hard implementation/closure prerequisite.
- No resource/storage/streaming/locking law (A-008/RQ-007/RQ-008).
- No projection fallibility (A-007/RQ-009).
- No A-009 eligibility remediation.
- No dependency/toolchain closure (A-011/RQ-015).
- No claim that RQ-013 is closed.
- No admission, identity, authorization, provenance-response or
  authority-bound corroboration work.
- No generic parser framework, dynamic registry or “latest” selector.

## Independent review gate

After the implementation, a reviewer independent of the author must attack
duplicate handling, per-payload-variant unknown fields, invalid UTF-8, host
numeric coercion and invalid numeral parsing, zero-length versus whitespace-only
coordinates, parser aliases, multi-defect ordering, canonical hash
drift, non-canonical point encodings, zero-`x` sign handling, accidental
A21-D1/A21-D2 verdict assignment, generalization from golden signatures and Go
implementation sharing. General tests are not a substitute for the exact
corpus.

## Owner merge gate

Only the human owner may decide that the contract or later implementation is
ready, merge it, or authorize any release use. Draft publication and passing
tests confer no merge or release authority.

## Administrative closure gate

A-004 and RQ-006 remain Confirmed through this contract. They cannot be fully
Closed until a dedicated A-021 contract defines the complete Ed25519
verification relation; the owner ratifies it; Rust, Go and Python independently
implement it; all portable-language and signature-semantic vectors agree;
frozen histories satisfy the ratified compatibility law; hostile review finds
no unresolved divergence; and the implementation is owner-merged. Only then
may a separate administrative PR reconsider their dispositions. Green grammar,
canonical point, ordinary fixture, schema, N1-N30 or K1-K15 results alone are
insufficient. RQ-013 must still be evaluated against its larger scope.
