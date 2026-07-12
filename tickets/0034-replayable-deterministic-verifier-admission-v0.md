# Ticket 0034: Replayable deterministic-verifier admission v0

## Goal and starting boundary

Land the documentation contract for one proof-carrying witness, one closed
deterministic checker, one replay-derived verifier receipt, and one future
policy-v2 direct `Supported` rule. Implement none of them.

Starting commit:
`2bbfbd1f451e35b65e8c89aeaf39801621e484df` (PR #44 merge).
The human-controlled `v0.1.0` tag must exist and resolve exactly there before
this work begins. This ticket creates, moves, deletes, or publishes no tag or
release.

## Architectural law

```text
An evidence label is not verifier authority.
A witness supplied by an event is not a receipt.
A receipt exists only when a closed deterministic checker successfully
re-evaluates exact replay-derived material under an explicit policy.
A successful receipt may admit one bounded contribution, not general truth.
```

Provenance is not factual commitment. Producer-supplied witness, identifiers,
expected values, hashes, attribution, actor class, evidence kind, and claimed
success remain untrusted. Unknown and malformed inputs fail closed.

This ticket defines no verifier authority by itself.

No metadata schema becomes trusted merely because it is documented.

Authority begins only when a later reviewed implementation derives a
successful receipt through the closed checker over one verified replay
snapshot.

## Exact scope and schemas

The claim metadata is exactly:

```json
{
  "claim_domain": "ExactMachineCheckable",
  "machine_predicate": {
    "schema": "magpie-machine-predicate-v0",
    "predicate_id": "sha256_bytes_equals_v0",
    "expected_sha256": "<64 lowercase hexadecimal characters>"
  }
}
```

The evidence metadata is exactly:

```json
{
  "verification_witness": {
    "schema": "magpie-verification-witness-v0",
    "predicate_id": "sha256_bytes_equals_v0",
    "subject_claim_id": "<exact claim id>",
    "scope_ref": "<exact scope>",
    "witness_hex": "<strict lowercase even-length hexadecimal>"
  }
}
```

Both are policy-defined interpretations of existing opaque `metadata_json`,
not L0 fields. Each object permits only the shown required string keys;
unknown, duplicate, missing, wrong-type, trailing, or ambiguously encoded input
fails closed. Hashes use exact lowercase hexadecimal. Witness hex is lowercase,
even-length, and bounded by the checker-owned
`MAX_WITNESS_BYTES_V0 = 4096`; empty bytes are allowed. A claimed result field
is rejected as an unknown key.

The exact proposition is only: the raw bytes in this exact evidence node, under
this exact claim, supports edge, and scope binding, have SHA-256 digest H. It
makes no external artifact, acquisition, execution, safety, semantic, or origin
claim.

## Binding and trusted construction

The later checker must require exact supports edge source/target, exact claim,
evidence and edge IDs, `DeterministicVerification`,
`ExactMachineCheckable`, exact four-way scope equality, exact claim-ID binding,
exact schema and predicate binding, and exact digest comparison. It must
re-fetch nodes and the edge rather than trust candidate trace fields.

The sole legitimate construction is:

```text
one successful LogReader verification
→ one private composite replay
→ one StandingReplaySnapshot
→ closed candidate classification
→ replayed-node and edge revalidation
→ closed sha256_bytes_equals_v0 checker
→ derived Matched receipt
```

Public struct construction, independent projections or replays, a second read,
metadata, booleans, callbacks, plugins, processes, shell, network, and ambient
mutable policy confer no authority. A serializable audit value is not proof of
trusted resolver origin.

## Policy-v2 boundary

The future identity is `magpie-claims-standing-v2`. It preserves v1 and adds:

```text
DeterministicVerification
× ExactMachineCheckable
× sha256_bytes_equals_v0
× successful same-snapshot deterministic verifier context
→ Some(Status::Supported)
```

The contribution deliberately remains below its `Settled` ceiling. Existing
Deadbolt matched occurrence remains `Settled` and takes result precedence;
otherwise any successful deterministic direct path yields `Supported`;
otherwise resolution falls back to v0. Multiple receipts add audit traces only
and never amplify.

This is direct support, not aggregation: it combines no weak inputs, counts no
sources, and infers no independence.

## Serialization recommendation

Use explicit `StandingPolicyRuleV2`, `StandingPolicyContextV2`,
`StandingPolicyApplicationV2`, `StandingTraceEntryV2`, and
`StandingResolutionV2`. Do not add a v2 rule to the existing v1 serialized rule
type. V2 explicitly dispatches both inherited Deadbolt v1 semantics and the new
SHA-256 direct-support rule. Exact v0 and v1 types, variants, policy IDs,
behaviour, and literal canonical fixtures remain unchanged.

## Files in scope

- `README.md`
- `docs/design/replayable-deterministic-verifier-admission-v0.md`
- `docs/design/standing-aggregation-independence-groups.md`
- `tickets/0034-replayable-deterministic-verifier-admission-v0.md`

## Frozen surfaces and non-goals

No change to Cargo, dependencies, CI, Rust, tests, fixtures, tools, changelog,
format, portable-base, seams, ADRs, release documents, licenses, tags, or
GitHub releases. No L0/canonical/signature/genesis/golden/verifier/replay/
standing/policy behaviour changes. No implementation of a payload, checker,
receipt, policy v2, standing result, aggregation, independence, refutation,
contradiction debt, invalidation, supersession, gate, writer, artifact access,
file/shell/network/plugin authority, proof system, or model/lens infrastructure.

## Acceptance criteria

- The design fixes the exact predicate, schemas, 4096-byte limit, strict parse
  law, bindings, checker algorithm, outcome vocabulary, trusted origin, proof
  boundary, policy-v2 rule, precedence, non-amplification, explicit v2 types,
  canonical-byte requirements, hostile tests, and stop conditions.
- README records the exact tag target and reviewed source-release meaning while
  preserving non-publication, path-workspace, packageability, format, policy,
  and absence boundaries; it does not imply registry publication.
- README and the aggregation roadmap show verifier admission before genuine
  aggregation and do not imply implementation.
- Only the four scoped files change. Historical tagged-release contents remain
  byte-identical.

## Documentation validation

Run `git diff --check`, full formatting/Clippy/workspace tests, the governed
tour, release metadata checker, both independent chain verifiers, changed and
prohibited path audits, release-boundary audit, overclaiming search, and final
diff/status/PR inspection exactly as specified in the task.

## Follow-up sequence

1. Implement a standing-inert strict metadata parser and closed checker inside
   the private same-snapshot construction, with every hostile failure.
2. Review the derived context and canonical audit fixtures without standing
   changes.
3. Implement explicit policy-v2 types and exactly one direct `Supported` rule,
   retaining literal v0/v1 bytes and Deadbolt precedence.
4. Only then begin genuine aggregation and conservative independence work.

Stop the later implementation if it needs new L0 or artifact infrastructure,
external authority, v0/v1 mutation, or a result broader than the exact bounded
proposition and `Supported` contribution.
