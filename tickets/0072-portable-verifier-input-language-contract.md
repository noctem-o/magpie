# Ticket 0072: Portable verifier input-language contract

## Status

Documentation-only implementation contract candidate for A-004 / RQ-006.

This ticket does not implement a parser, verifier, Go tool, fixture, test or CI
change and does not close either finding. Human owner merge authority remains
exclusive.

Required sequence:

```text
contract merge
-> corpus/verifier work independent of A-021
-> owner-ratified A-021 signature-verification semantics
-> apply that decision to Rust/Go/Python and activate weak-key vectors
-> independent hostile review
-> owner merge
-> separate living-ledger reconciliation
```

The A-021 steps may occur earlier, but they are a hard prerequisite before any
claim of universal signature-verification equivalence or full A-004/RQ-006
closure.

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
upstream decision for full equivalence over representation-valid weak/low-order
external roots.

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

The preserved A-021 experiment adds a separate limit to the equivalence claim:
ordinary verification accepts attacker-constructed signatures under the
canonical low-order root
`ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f`,
while strict verification rejects them. This ticket does not select either
behavior. It constrains current conformance to normative cases that do not
engage that unresolved choice and makes A-021 a closure prerequisite.

The correction is one strict, explicit portable JSONL grammar and one stable
verdict model over its governed conformance domain, without changing canonical
bytes.

## Selected contract summary

- Zero bytes is an accepted empty snapshot with count 0 and zero tip; it makes
  no Genesis/key-binding claim. LF-only input is a blank record and rejects.
- LF and CRLF separators may be mixed; the final terminator is optional; blank
  lines, extra final terminators and lone CR reject.
- Input is valid UTF-8 without BOM and contains exactly one JSON object per
  non-empty record.
- Duplicate and unknown names reject at every governed object level.
- Key order, space/tab JSON whitespace and equivalent standard JSON string
  escapes are permitted; non-finite values and unpaired surrogates reject.
- Every SignedEvent/core/provenance/payload variant has exact required,
  non-null members with no defaults.
- Core/stored hash and signature spellings are exact lowercase ASCII hex.
- The external key must pass lowercase syntax, exact 32-byte decoding and the
  contract-owned canonical RFC 8032 Edwards25519 point-representation gate
  before any framing or record processing. That gate requires encoded
  `y < 2^255 - 19`, successful point recovery, rejection of sign bit 1 when
  recovered `x = 0`, and byte-identical canonical re-encoding. Representation
  failure wins even for zero-byte input.
- Canonical representation validity does not decide weak-root admissibility or
  ordinary-versus-strict signature verification. The current portable verdict
  law covers the normative ordinary-key corpus; A21-D1 preserves the known
  low-order divergence without a premature ACCEPT/REJECT result.
- JSON statuses are exact named strings, never numeric aliases.
- `seq` and `timestamp_nanos` are unsigned decimal integer tokens in u64 range,
  with no sign, float, exponent, Boolean, null or string coercion.
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
universally equivalent release oracle across all external keys until A-021 is
ratified and implemented. The intended isolated home is
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
A21-D1. Standard-library-only remains preferred; this ticket authorizes no new
dependency to implement either the representation law or future A-021 policy.

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

## Normative behavior matrix

The complete discovered current-behavior table, selected future law, exact
schema and compatibility analysis are in the governing contract. The
implementation must not replace that table with “match Rust,” “match Python”
or “match Go's standard library.”

## Negative and hostile proof obligations

Implement the full contract N1-N30 matrix, including:

- case and width variants for every hex role;
- numeric/unknown status;
- blank, empty, BOM, UTF-8 and line-ending cases;
- duplicate known/unknown and unknown members;
- non-finite JSON, malformed JSON and wrong shapes;
- u64 fractional, exponent, overflow, negative and coercion cases;
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

K8 is replaced by mandatory dependency sentinel A21-D1: canonical low-order
root `ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f`
plus committed exact two-record forged-chain bytes reproducing the preserved
A-021 scenario. It preserves the known
ordinary/strict divergence but has no expected portable ACCEPT/REJECT until
A-021 is owner-ratified. The runner must reject any attempt to count it as a
normative pass or assign it a premature verdict. After A-021 is implemented by
all conformers, the same exact case becomes a normative hostile vector.

Schema coverage must be machine-checkable. Test metadata must enumerate the
complete expected coordinate set (top/core/provenance plus `kind` and every
additional member of every payload variant) and map each coordinate to exactly
one omission and one null case. The runner fails on missing/ambiguous coverage
or a new governed member absent from the inventory. This remains conformance
metadata, not a runtime schema registry; committed SHA-bound case bytes are
authoritative even if authoring is generated.

Couple the expected coordinate set mechanically to the current Rust types with
test-only exhaustive field destructuring/variant matching (or an equally
direct non-runtime mechanism), so a new governed field or payload variant
cannot compile or pass conformance until the inventory and both mutations are
updated.

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
byte-identically, N17/N18 coverage is mechanically exhaustive, and A21-D1 is
excluded from normative conformance totals pending ratification. P1-P12 apply
to the current normative conformance domain and must not be presented as
universal weak-key equivalence.

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
machine-checkable omission/null coverage over every required schema coordinate
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
- No selection of ordinary versus strict weak-key/trust-root policy (A-021);
  this ticket only records A-021 as a hard prerequisite to full equivalence.
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
duplicate handling, unknown fields, invalid UTF-8, host numeric coercion,
parser aliases, multi-defect ordering, result coordinates, canonical hash
drift, non-canonical point encodings, zero-`x` sign handling, accidental
A21-D1 verdict assignment and Go implementation sharing. General tests are not
a substitute for the exact corpus.

## Owner merge gate

Only the human owner may decide that the contract or later implementation is
ready, merge it, or authorize any release use. Draft publication and passing
tests confer no merge or release authority.

## Administrative closure gate

A-004 and RQ-006 remain Confirmed through this contract. They cannot be fully
Closed until A-021's weak/low-order signature-verification semantics are
owner-ratified, implemented in all three conformers, covered by activated
cross-language hostile vectors, and shown equivalent. Only after that complete
corpus and implementation pass hostile review and are owner-merged may a
separate administrative PR reconsider their living dispositions. Green
grammar, canonical point, ordinary fixture, schema and N1-N30 results alone are
insufficient. RQ-013 must still be evaluated against its larger scope.
