# Independent Go portable conformer

This directory contains a standalone Go implementation of the frozen Magpie
portable JSONL verifier. Its semantics come from `docs/FORMAT.md`, Accepted
ADR-0010, `docs/design/portable-verifier-input-language-contract.md`, and the
exact `fixtures/verifier-language-v1/manifest.json` cases. It does not call or
generate code from the Rust or Python verifiers and does not use another
implementation as an oracle.

## Dependency rationale

The Go standard library does not expose the Edwards25519 point arithmetic
needed to implement ADR-0010's canonical point, full prime-subgroup, and
uncofactored equation gates. The module therefore pins the focused
`filippo.io/edwards25519` dependency at **v1.2.0**. The adapter does not treat
the package's defaults as Magpie semantics: `Point.SetBytes` is documented to
accept non-canonical encodings, so the conformer independently checks
`y < p`, RFC 8032 square-root/sign rules, and byte-identical `Point.Bytes()`.
It checks `[L]P = I` with unreduced integer point multiplication, uses
`Scalar.SetCanonicalBytes` for `S`, and `Scalar.SetUniformBytes` for the
SHA-512 challenge reduction.

## CLI

The isolated Go SDK requested for this worktree is:

```text
.scratch/toolchains/go1.27.0/go/bin/go.exe
```

The single-history command takes exactly the profile identity, lowercase
external-key text, and history path:

```powershell
& .scratch/toolchains/go1.27.0/go/bin/go.exe -C tools/go-verify-chain run . magpie-ed25519-canonical-prime-subgroup-v1 <64-lowercase-key-hex> <history-path>
```

It emits one deterministic JSON result with the manifest result fields:
`verdict`, `class`, `line`, `record_index`, `event_count`, `tip`, and
`ordered_recomputed_hashes`. Exit status is 0 for `ACCEPT`, 1 for a governed
`REJECT`, and 2 for usage, profile, I/O, dependency, or internal failure.

The manifest export mode validates the frozen manifest identity/digest,
validates every case input SHA-256, runs each case in manifest order, and
emits the actual result without consulting any expected-result field:

```powershell
& .scratch/toolchains/go1.27.0/go/bin/go.exe -C tools/go-verify-chain run . --manifest ../../fixtures/verifier-language-v1/manifest.json
```

Each export line is `{"id":...,"result":{...}}`. Export exits 0 after all
cases complete; operational failures exit 2. The separate check mode compares
the independently computed result fields with the frozen expected fields and
returns 0 on complete agreement, 1 on mismatch, and 2 on operational failure:

```powershell
& .scratch/toolchains/go1.27.0/go/bin/go.exe -C tools/go-verify-chain run . --check-manifest ../../fixtures/verifier-language-v1/manifest.json
```

This Go command pins the complete manifest bytes and each governed input, but
it is a semantic-conformer check rather than the repository's full corpus
source/coverage audit. Run `python tools/check_verifier_corpus_manifest.py`
from the repository root as its companion; CI runs both checks.

Focused tests include the complete 432-case check:

```powershell
& .scratch/toolchains/go1.27.0/go/bin/go.exe -C tools/go-verify-chain test ./...
```

An `ACCEPT` proves only the exact supplied bytes under the selected profile
and external key. It does not establish trust, identity, authority,
freshness, completeness, durability, or permission to act.
