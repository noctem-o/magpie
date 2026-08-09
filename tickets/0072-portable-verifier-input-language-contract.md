# Ticket 0072: Portable verifier input-language contract

## Status

Documentation-only implementation contract candidate for A-004 / RQ-006.

This ticket does not implement a parser, verifier, Go tool, fixture, test or CI
change and does not close either finding. Human owner merge authority remains
exclusive.

Required sequence:

```text
contract merge
-> separate corpus/verifier implementation
-> independent hostile review
-> owner merge
-> separate living-ledger reconciliation
```

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
assurance scope.

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

The correction is one strict, explicit portable JSONL grammar and one stable
verdict model without changing canonical bytes.

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
  contract-owned Ed25519 compressed-point representation/decompression gate
  before any framing or record processing. Representation failure wins even
  for zero-byte input; representation-valid weak-key trust policy remains
  A-021.
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
and ordered recomputed hashes in the corpus.

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

Go is the independent, release-grade verifier and release-oracle candidate.
The intended isolated home is `tools/go-verify-chain/`.

It must:

- implement parsing, canonical encoding, hashing, Ed25519, chain checks,
  payload validation and Genesis independently;
- never link to or invoke Rust or Python;
- use no generated/shared Magpie verification logic;
- prefer only Go standard-library cryptography and JSON primitives;
- independently enforce the contract's compressed-point representation gate
  even though `crypto/ed25519` can store arbitrary 32-byte slices; if the
  standard library cannot support that check, stop for owner review before
  adding a dependency or weakening the rule;
- explicitly constrain permissive `encoding/json` behavior;
- use no cgo without a separately reviewed need;
- use no network access during verification; and
- build as one small deterministic local command.

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

Every case file is exact bytes and is bound by `input_sha256`. The manifest
binds case ID, repository-root-relative path, supplied external key, expected
verdict, class, line, record index and purpose. Accepted cases additionally
bind event count, tip and ordered event hashes. P1/P2 reference the existing
golden and Deadbolt fixtures in place; new inputs live under `cases/`.

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

Also implement K1-K8 for the external-key gate: malformed length, case and
non-hex; a lowercase 32-byte representation that fails point decompression;
that representation plus zero-byte input; ordinary representation-valid and
golden-fixture keys; and an explicit A-021 non-decision for any
representation-valid weak/low-order key. K4/K5 are `REJECT(ExternalKey)` with
no coordinates.

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
representation gate, every new case survives Git checkout byte-identically,
and N17/N18 coverage is mechanically exhaustive.

## Protected surfaces

The implementation must not change merely to satisfy this contract:

- `docs/FORMAT.md` or `magpie-core-v1` canonical encoding;
- `EventCore`, `Payload` or `SignedEvent` persisted representation;
- SHA-256, `magpie-sig-v1`, Ed25519 or Genesis semantics;
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
all three conformers over every exact-byte manifest case
Git attribute inspection plus fresh-checkout SHA-256 verification for every case
machine-checkable omission/null coverage over every required schema coordinate
python tools/check_release_metadata.py
git diff --check
```

It must report exact case totals, no crashes, count/tip/hash equivalence,
first-failure equivalence, dependency/import audits and protected-file hashes.

## Explicit non-goals

- No runtime, verifier, fixture, test, Go, module or CI implementation here.
- No `.gitattributes` change in this contract PR; its scoped corpus override
  belongs to the later implementation.
- No new canonicalization, event or transport profile.
- No JSON-byte canonicalization.
- No weak-key/trust-root policy (A-021).
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
drift and Go implementation sharing. General tests are not a substitute for
the exact corpus.

## Owner merge gate

Only the human owner may decide that the contract or later implementation is
ready, merge it, or authorize any release use. Draft publication and passing
tests confer no merge or release authority.

## Administrative closure gate

A-004 and RQ-006 remain Confirmed through this contract. Only after the corpus
and all three conformers land, pass hostile review and are owner-merged may a
separate administrative PR reconsider their living dispositions. RQ-013 must
still be evaluated against its larger scope.
