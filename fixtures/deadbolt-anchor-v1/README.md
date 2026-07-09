# Deadbolt Anchor Fixture v1

This directory contains a portable Magpie-side fixture for the Deadbolt anchor
seam. It is a protocol fixture, not a live Deadbolt run.

`anchor-log.jsonl` is a two-event Magpie log:

- seq 0: `Genesis`
- seq 1: `SegmentAnchored`

The fixture uses deterministic test-only signing material and contains no
private machine paths, host output, screenshots, live Deadbolt artifacts, or
production trust material. The trust root for verification is the public
verifying key recorded in the genesis event:

```text
d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737
```

The signing seed used to create this checked-in fixture is not required to run
the tests and is not part of the trust boundary; the committed log plus public
verifying key are the fixture.

The anchor payload records occurrence, inclusion, and order material only:

```text
bundle_kind: kernel-decision-witness
witness_root: 79791d454eaaf2f86c1b5a3c944d021c6721905a7a70790c7089bb5a6638af0b
witness_algorithm: sha256
canonicalization_profile: kernel-decision-witness-v1
run_id: portable-run-0001
```

The `canonicalization_profile` is the foreign profile named by the anchor and
must remain verbatim. It is not `magpie-core-v1`.

Magpie verifies the log chain, order, signatures, and inclusion of this anchor.
It does not verify the foreign bundle contents, does not create typed claims,
does not promote standing, and does not settle interpretation truth.

Independent verifier command:

```sh
python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737
```
