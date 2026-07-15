# Origin-binding v0 portable fixtures

These files are untrusted portable test material for the standing-inert
origin-binding verifier. They freeze canonical statement bytes and selector
roots; they do not establish authority trust, origin admission, support,
corroboration, aggregation, or standing.

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `acquisition-binding.json` | 870 | `4f4bbaab87c1cfdc4dfe3b8c0d8b11a04bdc637de1e8e1481001b09543f0a4d7` |
| `derivation-binding.json` | 1,144 | `24aa77f2f601e70f6d5d6044f0b1274394f164276df07296be16ab59953a5a29` |
| `derivation-parent-acquisition-bundle.json` | 294 | `94593dd609cb44b07c2f3dd5a72226ff4c02ad4b9f4378a84d4432c433a1f7ce` |

All three files start with `0x7b`, end with `0x7d`, contain no UTF-8 BOM,
and have no trailing newline.

## Exact selectors

`acquisition-binding.json`:

```text
bundle_kind: magpie-origin-binding-acquisition-v0
witness_root: 4f4bbaab87c1cfdc4dfe3b8c0d8b11a04bdc637de1e8e1481001b09543f0a4d7
witness_algorithm: sha256
canonicalization_profile: magpie-origin-binding-json-v0
run_id: run-origin-binding-acq-0001
```

`derivation-binding.json`:

```text
bundle_kind: magpie-origin-binding-derivation-v0
witness_root: 24aa77f2f601e70f6d5d6044f0b1274394f164276df07296be16ab59953a5a29
witness_algorithm: sha256
canonicalization_profile: magpie-origin-binding-json-v0
run_id: run-origin-binding-der-0001
```

`derivation-parent-acquisition-bundle.json` is selected as:

```text
bundle_kind: magpie-artifact-acquisition-v0
witness_root: 94593dd609cb44b07c2f3dd5a72226ff4c02ad4b9f4378a84d4432c433a1f7ce
witness_algorithm: sha256
canonicalization_profile: magpie-artifact-provenance-json-v0
run_id: run-acq-alpha-0001
```

## Reused artifact-provenance material

The integration tests reuse, without copying or modifying:

- `fixtures/artifact-provenance-v0/acquisition-bundle.json`
- `fixtures/artifact-provenance-v0/derivation-bundle.json`
- `fixtures/artifact-provenance-v0/acquisition-artifact.txt`
- `fixtures/artifact-provenance-v0/derivation-parent.txt`
- `fixtures/artifact-provenance-v0/derivation-derived.txt`

Their exact nested selectors remain the ratified artifact-provenance v0
selectors. In particular, direct acquisition uses root
`4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e`,
and direct derivation uses root
`472e857153fe8c901b6c0d7d1b15bd876450b57b4743222a2fab91eb60bc76dd`.

No signed anchor log, signing key, trusted key, authority registry, admission
policy, or trust root is included. Matched tests construct deterministic signed
Magpie chains and exact `SegmentAnchored` events at runtime.
