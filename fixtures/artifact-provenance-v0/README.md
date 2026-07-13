# Artifact Provenance Fixture v0

This directory contains portable protocol fixtures for the exact acquisition
and direct-derivation families defined by Ticket 0038. They are protocol test
material, not a live acquisition or transformation run.

## Acquisition fixture

- `acquisition-artifact.txt`: 15 bytes, exactly `hello artifact` followed by
  one final LF byte (`0a`). SHA-256 is
  `51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076`.
- `acquisition-bundle.json`: 291 canonical JSON bytes with no trailing newline.
  SHA-256 witness root is
  `4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e`.

The exact acquisition selector is:

```text
bundle_kind: magpie-artifact-acquisition-v0
witness_root: 4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e
witness_algorithm: sha256
canonicalization_profile: magpie-artifact-provenance-json-v0
run_id: run-acq-0001
```

## Direct-derivation fixture

- `derivation-parent.txt`: 6 bytes, exactly `alpha` followed by one final LF
  byte (`0a`). SHA-256 is
  `b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060`.
- `derivation-derived.txt`: 6 bytes, exactly `ALPHA` followed by one final LF
  byte (`0a`). SHA-256 is
  `1921b918b15842c7fdb115078e610263fac85f159c1d8e0ecec3d89a0faa4005`.
- `derivation-bundle.json`: 374 canonical JSON bytes with no trailing newline.
  SHA-256 witness root is
  `472e857153fe8c901b6c0d7d1b15bd876450b57b4743222a2fab91eb60bc76dd`.

The exact derivation selector is:

```text
bundle_kind: magpie-artifact-derivation-v0
witness_root: 472e857153fe8c901b6c0d7d1b15bd876450b57b4743222a2fab91eb60bc76dd
witness_algorithm: sha256
canonicalization_profile: magpie-artifact-provenance-json-v0
run_id: run-der-0001
```

These fixtures establish no publisher identity, factual truth, locator
authenticity, transformation correctness, or authority to admit an origin.
They contain no production keys, private machine paths, usernames, secrets,
live URLs, or production signing material.

## Independent regression commands

Run these from the repository root. Each command reads only one committed file
and independently prints its byte length and SHA-256 digest:

```sh
python -c "from pathlib import Path; import hashlib; p=Path('fixtures/artifact-provenance-v0/acquisition-artifact.txt'); b=p.read_bytes(); print(len(b), hashlib.sha256(b).hexdigest())"
python -c "from pathlib import Path; import hashlib; p=Path('fixtures/artifact-provenance-v0/acquisition-bundle.json'); b=p.read_bytes(); print(len(b), hashlib.sha256(b).hexdigest())"
python -c "from pathlib import Path; import hashlib; p=Path('fixtures/artifact-provenance-v0/derivation-parent.txt'); b=p.read_bytes(); print(len(b), hashlib.sha256(b).hexdigest())"
python -c "from pathlib import Path; import hashlib; p=Path('fixtures/artifact-provenance-v0/derivation-derived.txt'); b=p.read_bytes(); print(len(b), hashlib.sha256(b).hexdigest())"
python -c "from pathlib import Path; import hashlib; p=Path('fixtures/artifact-provenance-v0/derivation-bundle.json'); b=p.read_bytes(); print(len(b), hashlib.sha256(b).hexdigest())"
```

The two bundle files must not end in `0a` or `0d`. Each artifact file must end
in exactly its one specified LF byte. No fixture file has a UTF-8 BOM.
