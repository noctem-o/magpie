# Independent Go portable conformer

This directory contains a standalone Go implementation of the frozen Magpie
portable JSONL verifier. Its semantics come from `docs/FORMAT.md`, Accepted
ADR-0010, `docs/design/portable-verifier-input-language-contract.md`, and the
exact `fixtures/verifier-language-v1/manifest.json` cases. It does not call or
generate code from the Rust or Python verifiers and does not use another
implementation as an oracle.

## Authorized vendored dependency

The owner authorizes exactly `filippo.io/edwards25519 v1.2.0` for this module,
and only for the existing low-level Edwards25519 point/scalar arithmetic. The
source is committed under `vendor/filippo.io/edwards25519`, Go's generated
module inventory is `vendor/modules.txt`, and the upstream BSD-style license is
retained at `vendor/filippo.io/edwards25519/LICENSE`. Builds and tests use
`-mod=vendor` so a clean checkout does not need to retrieve the module.

The Go standard library does not expose the point arithmetic needed to
implement ADR-0010's canonical point, full prime-subgroup, and uncofactored
equation gates. The adapter does not treat the dependency's defaults as Magpie
semantics: `Point.SetBytes` accepts encodings beyond Magpie's selected language,
so the conformer independently checks `y < p`, the RFC 8032 square-root/sign
rules, and byte-identical `Point.Bytes()`. It checks `[L]P = I` with unreduced
integer point multiplication, uses `Scalar.SetCanonicalBytes` for `S`, and
`Scalar.SetUniformBytes` for the SHA-512 challenge reduction.

Pinning and vendoring establish a reviewable source identity and offline source
availability; they do not prove the dependency or the conformer correct.

## Build the governed executable

Use Go 1.27.0. Build the vendored source once, then invoke the compiled
executable directly. Do not use `go run` as the governed interface because the
Go driver does not transparently preserve the program's exit status.

PowerShell, from the repository root:

```powershell
go version
go -C tools/go-verify-chain build -mod=vendor -o go-verify-chain.exe .
& .\tools\go-verify-chain\go-verify-chain.exe magpie-ed25519-canonical-prime-subgroup-v1 <64-lowercase-key-hex> <history-path>
$LASTEXITCODE
```

POSIX shell, from the repository root:

```sh
go version
go -C tools/go-verify-chain build -mod=vendor -o go-verify-chain .
./tools/go-verify-chain/go-verify-chain magpie-ed25519-canonical-prime-subgroup-v1 <64-lowercase-key-hex> <history-path>
status=$?
```

The single-history command emits one deterministic JSON result with the
manifest result fields: `verdict`, `class`, `line`, `record_index`,
`event_count`, `tip`, and `ordered_recomputed_hashes`.

Its direct executable status is:

- 0: `ACCEPT`;
- 1: governed `REJECT`; and
- 2: usage, profile, I/O, dependency, or internal operational failure.

`binary_exit_test.go` builds the vendored executable and invokes it as a child
process to pin all three statuses.

## Manifest modes

Run these direct executable commands from the repository root. Export validates
the frozen manifest identity/digest and every input SHA-256, then emits each
actual result in manifest order without consulting expected-result fields:

```powershell
& .\tools\go-verify-chain\go-verify-chain.exe --manifest fixtures/verifier-language-v1/manifest.json
```

Each line is `{"id":...,"result":{...}}`. Export exits 0 after all cases
complete; an operational failure exits 2. Governed case rejections are emitted
results, not export-process failures.

Check mode compares independently computed results with the frozen expected
fields:

```powershell
& .\tools\go-verify-chain\go-verify-chain.exe --check-manifest fixtures/verifier-language-v1/manifest.json
$LASTEXITCODE
```

Check mode returns 0 on complete agreement, 1 on a governed-field mismatch, and
2 on an operational failure. The command pins the complete manifest bytes and
each governed input, but remains a semantic-conformer check rather than the
repository's full source/coverage audit. Run
`python tools/check_verifier_corpus_manifest.py` as its companion.

## Offline development checks

The following PowerShell commands explicitly disable module retrieval and force
the committed vendor tree:

```powershell
$env:GOENV = "off"
$env:GOFLAGS = ""
$env:GONOPROXY = ""
$env:GONOSUMDB = ""
$env:GOPRIVATE = ""
$env:GOPROXY = "off"
$env:GOSUMDB = "off"
$env:GOTOOLCHAIN = "local"
$env:GOVCS = "*:off"
$env:GOWORK = "off"
go -C tools/go-verify-chain test -mod=vendor -count=1 ./...
go -C tools/go-verify-chain vet -mod=vendor ./...
go -C tools/go-verify-chain build -mod=vendor -o go-verify-chain.exe .
```

Only an intentional dependency refresh should run `go mod vendor`. A refresh
must retain the exact approved module/version, its upstream license, and a
drift-free second vendoring pass.

An `ACCEPT` proves only the exact supplied bytes under the selected profile and
external key. It does not establish trust, identity, authority, freshness,
completeness, durability, or permission to act.
