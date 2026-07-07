# Metadata Conventions for Governed Claim Memory (tags 6-8)

## Status

Proposed design note.

## Purpose

Tags 6-8 (`ClaimAssertedV2`, `EvidenceRegistered`, `JustificationEdgeRecorded`)
each carry a `metadata_json: String`. This note defines the JSON structure that
string is expected to hold, so the future `EpistemicGate` and `StandingView`
fold have a stable, closed convention to read — **without changing L0 canonical
encoding**.

It is the companion to `docs/design/standing-view-evidence-ceilings.md`: the
ceiling law is indexed by `claim_domain`, and this note pins where `claim_domain`
and the citation locators live.

## Relationship to canonical encoding

Under `magpie-core-v1`, `metadata_json` is a single opaque, length-prefixed
UTF-8 string. These conventions describe its *interior* JSON structure, which the
codec never parses, hashes as structure, or validates. Consequences:

- Canonical bytes and golden hashes do not move because of anything in this note.
- The codec stays oblivious to `metadata_json` contents.
- A malformed, absent, or unexpected key is a **policy failure** at the gate or
  fold, **never a chain-integrity failure**.

Nothing in this note is admissible as a format change. If a convention here ever
needs to become a real typed field, that is a separate ADR and the full freeze
ritual (new profile name, golden vectors, verifier), not a metadata edit.

## Boundary / Non-goals

- Docs-only. No `Payload`, tag, or canonical-encoding change.
- No new fields on tags 6-8.
- No `docs/FORMAT.md`, golden vector, Python verifier, or Rust change.
- No parsing, validation, or gate implementation — conventions only.
- No source text or quoted spans stored in L0; only locators and hashes.
- No numeric confidence or scores as conventions.

## `claim_domain`

- **v1 location:** a top-level key `"claim_domain"` inside `metadata_json` on
  `ClaimAssertedV2`, and on `EvidenceRegistered` where an evidence node asserts a
  domain of its own.
- **Value:** MUST be one of the closed claim-domain vocabulary defined in the
  evidence-ceilings note — `Occurrence/Inclusion`, `ExactMachineCheckable`,
  `OperationalObservation`, `Interpretation`, `ExternalReport`,
  `ModelIntrospection`, `HumanJudgment`. This note does not extend that set;
  adding a domain is an amendment there first.
- **Provisional, with a promotion path:** reading a domain out of an opaque JSON
  string is a known smell. If `claim_domain` proves load-bearing for the fold, a
  future ADR may promote it to a dedicated typed field or fold it into `scope_ref`
  semantics — a format change requiring the full freeze ritual. It lives in
  `metadata_json` for v1 only.
- **Absent or ambiguous domain:** the fold takes the weaker applicable ceiling or
  refuses promotion, per "Domain assignment is admitted policy material" in the
  ceilings note. A missing `claim_domain` is never read as a stronger domain.

## Source locators (`ExternalSource` evidence)

On an `EvidenceRegistered` whose `evidence_kind` is `ExternalSource`, the
citation is only real evidence if it identifies the source and the exact span it
relies on, tamper-evidently. Convention keys inside `metadata_json`:

- `source_uri`: canonical identifier or URI of the cited source.
- `source_locator`: human-scoped position within the source (e.g. page, section,
  line span).
- `quote_hash`: hash of the canonical byte form of the quoted span (the preimage
  is fixed by "Quote canonicalization" below — not the raw source bytes).
- `quote_hash_algorithm`: the hash name, default `"sha256"`, mirroring how
  `SegmentAnchored` names `witness_algorithm`.
- `quote_canonicalization_profile`: the named profile that fixes the preimage
  bytes, default `"magpie-quote-v1"`. A `quote_hash` is only comparable within a
  single stated profile.

The quoted span itself is **not** stored in L0. `quote_hash` makes it
tamper-evident without embedding source text in the log: a later reader who holds
the source can recompute the hash of the cited span and confirm the citation was
not silently re-pointed. A citation with no locator and no `quote_hash` is a bare
reference and cannot rise above its `ExternalSource` ceiling on locator strength
alone.

### Quote canonicalization

`quote_hash` is meaningless unless its preimage is pinned. Without a defined
canonical form, two compliant writers can hash the same span to different values
— line endings, Unicode normalization, extraction whitespace, and
source-byte-vs-text all diverge — so the future gate cannot verify the
tamper-evident convention this section exists to stabilize. The preimage is
therefore fixed by a named profile, exactly as `SegmentAnchored` names a
`canonicalization_profile`.

Profile `magpie-quote-v1` defines the preimage as follows. The input is the
quoted **text** the writer commits to (a sequence of Unicode scalar values), not
the raw container bytes of a PDF, HTML page, or archive:

1. Apply Unicode Normalization Form C (NFC).
2. Normalize every line terminator to a single `U+000A` (LF): `CRLF`, a lone
   `CR`, `U+2028` (LINE SEPARATOR), and `U+2029` (PARAGRAPH SEPARATOR) all become
   LF.
3. Do not trim leading or trailing whitespace, and do not collapse internal
   whitespace runs. The span is preserved exactly after steps 1-2, so the hash
   stays lossless and the cited spacing is part of what is attested.
4. Encode the result as UTF-8.
5. `quote_hash` is `quote_hash_algorithm` (default SHA-256) over those UTF-8
   bytes, rendered lowercase hex.

Honest limit: `magpie-quote-v1` removes encoding, normalization, and line-ending
drift, but it cannot reconcile two *different extractions* of a lossy source —
PDF hyphenation, ligature expansion, or column-order differences still produce
different text and therefore different hashes. The profile guarantees
text-to-hash determinism, not extraction determinism. The writer attests the
exact quoted text it actually relied on; a verifier that re-extracts differently
sees a mismatch, which is the conservative outcome. Future profiles
(`magpie-quote-v2`, …) may add source-specific extraction rules; a `quote_hash`
is never compared across profiles.

## `content_hash`

`content_hash` is an existing field on **`ClaimAssertedV2` and
`EvidenceRegistered` only** (`JustificationEdgeRecorded` has no `content_hash`;
it carries a free-text `rationale`). The write path and the Python verifier
already require it to be empty or lowercase 64-hex. This note explains intent; it
does not change that rule.

- `ClaimAssertedV2.content_hash`: hash of the canonical claim statement bytes the
  assertion commits to.
- `EvidenceRegistered.content_hash`: hash of the evidence artifact the node
  refers to (e.g. the bundle, report, or verifier output), distinct from
  `quote_hash`, which hashes a cited *span* rather than a whole artifact.
- Empty is permitted and means "no artifact hash committed", not "hash of empty".

## Edge metadata

The namespaced targeting convention for `source_id` / `target_id`
(`claim:<claim_id>`, `evidence:<evidence_id>`, `edge:<edge_id>`) and objections as
`invalidates` / `contradicts` edges are defined in the ceilings note's
"edge targeting and objections" section, not here. `metadata_json` on an edge MAY
carry advisory annotation keys, but must not smuggle in a schema the codec is
expected to parse.

## Enforcement status

- These conventions are **advisory until `EpistemicGate` enforces them** at
  admission. Nothing reads them today.
- Because `metadata_json` is not canonically parsed, a missing or malformed key is
  a policy failure, never a chain-integrity failure. Replay of a chain whose
  metadata violates these conventions still verifies; only the derived standing is
  affected once a gate/fold exists.

## Illustrative example

The following is a **gate/fold-facing** illustration only. The codec does not
parse it, no schema is imposed on the string, and unknown keys are ignored rather
than rejected at the L0 layer.

```json
{
  "claim_domain": "ExternalReport",
  "source_uri": "https://example.org/paper",
  "source_locator": "p. 7, §3.2, lines 4-9",
  "quote_hash": "…64-hex…",
  "quote_hash_algorithm": "sha256",
  "quote_canonicalization_profile": "magpie-quote-v1"
}
```

## Reviewer checklist

- Confirm no example implies a schema the codec must parse; the codec stays
  oblivious to `metadata_json`.
- Confirm the `claim_domain` values match the closed set in the ceilings note.
- Confirm the promotion-path note is present, so a future dedicated field is a
  conscious format decision, not a silent metadata dependency.
- Confirm `content_hash` guidance covers only the two tags that have the field.
