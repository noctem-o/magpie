# Ticket 0014: Metadata conventions for claim_domain, source locators, quote_hash

## Goal

Create `docs/design/metadata-conventions.md` defining the JSON structure that
tags 6-8 carry inside their opaque `metadata_json: String`, so that the future
`EpistemicGate` and `StandingView` fold have a stable, closed convention to read
— without changing L0 canonical encoding.

This is a docs-only ticket. It defines conventions over an existing field; it
adds no new fields, tags, or encodings.

## Why

Tags 6-8 (`ClaimAssertedV2`, `EvidenceRegistered`, `JustificationEdgeRecorded`)
each carry `content_hash` and a free `metadata_json` string. Two things the
ceiling law (ticket 0013) and a real citation story need are currently
unspecified:

1. **Where `claim_domain` lives.** The ceiling matrix is indexed by claim
   domain, but no event field carries it. It must be pinned to a location and a
   closed vocabulary before the fold can read it.
2. **How `ExternalSource` evidence points at what it cites.** A citation is only
   real evidence if it identifies the source and the exact span, tamper-evidently
   (the PaperQA "passage relevance" lesson). That needs a locator + a quote hash.

Both fit entirely inside `metadata_json` as agreed JSON keys. The L0 codec still
length-prefixes one opaque string; canonical bytes and golden hashes do not move.

## Design decisions to record (architect-set; transcribe, do not re-decide)

- **`claim_domain` location, v1:** a top-level key `"claim_domain"` inside
  `metadata_json` on `ClaimAssertedV2` (and, where an evidence node asserts a
  domain, on `EvidenceRegistered`). Its value MUST be one of the closed
  claim-domain vocabulary defined in ticket 0013.
  - State the known smell explicitly: the fold reading a domain out of an opaque
    JSON string is provisional. Record a **promotion path**: if `claim_domain`
    proves load-bearing, a future ADR may add a dedicated typed field or fold it
    into `scope_ref` semantics — which would be a format change requiring the
    full freeze ritual. It stays in metadata for v1 only.
- **Source locators for `ExternalSource` (`EvidenceRegistered`):** define keys
  - `source_uri`: canonical identifier/URI of the cited source;
  - `source_locator`: human-scoped position (e.g. page, section, line span);
  - `quote_hash`: hash of the canonical byte form of the quoted span;
  - `quote_hash_algorithm`: the hash name (default `sha256`), mirroring how
    `SegmentAnchored` names `witness_algorithm`;
  - `quote_canonicalization_profile`: the named profile that pins the preimage
    bytes (default `magpie-quote-v1`), so two writers hash the same span
    identically. The profile must fix Unicode normalization (NFC), line-ending
    normalization (to LF), whitespace handling (no trim/collapse), and
    text-vs-source-byte choice (text). Include an honest caveat that it cannot
    reconcile lossy re-extraction (PDF hyphenation/ligatures/column order).
  The quoted span itself is NOT stored in L0; `quote_hash` makes it
  tamper-evident without embedding source text in the log.
- **`content_hash` guidance (`ClaimAssertedV2` and `EvidenceRegistered` only):**
  clarify what it should hash per kind (the canonical claim statement bytes; the
  evidence artifact). `JustificationEdgeRecorded` has no `content_hash` field — it
  carries a free-text `rationale` — so it is out of scope for this key. Reaffirm
  the existing write-path rule that `content_hash` is empty-or-lowercase-hex-64;
  the doc explains intent, it does not change the rule.
- **Enforcement status:** these are conventions, advisory until `EpistemicGate`
  enforces them at admission. Because `metadata_json` is not canonically parsed,
  a malformed or absent key is a policy failure, never a chain-integrity failure.
  Say this plainly.

## Scope

- Create `docs/design/metadata-conventions.md` with the sections above.
- The ceilings doc already forward-references `docs/design/metadata-conventions.md`
  (added in ticket 0013 / PR #23), so no edit outside the new file is needed. The
  PR therefore touches only the new doc plus this ticket.

## Non-goals / Constraints

- No Rust changes. No `Payload` changes. No new fields on tags 6-8.
- No canonical encoding changes. No `docs/FORMAT.md` changes. No golden vector
  changes. No Python verifier changes.
- Do NOT implement parsing, validation, or a gate. Conventions only.
- Do NOT store source text or quoted spans in L0. Only locators and hashes.
- Do NOT add numeric confidence or scores as conventions.
- Keep the claim-domain vocabulary identical to ticket 0013's closed set; if they
  disagree, stop and flag — do not silently diverge.

## Tests

- None (docs-only). No code compiles or runs for this ticket.

## Acceptance criteria

- `docs/design/metadata-conventions.md` exists and specifies: the
  `claim_domain` key + its closed vocabulary + the provisional/promotion note;
  the `ExternalSource` locator keys incl. `quote_hash` and its algorithm; and
  per-kind `content_hash` intent.
- The doc states explicitly that these conventions do not alter canonical bytes
  and that violations are policy failures, not integrity failures.
- The ceilings doc's forward-reference already points to the new file (from
  PR #23); no edit to it is required.
- Diff touches exactly two files: the new doc plus this ticket.

## Suggested reviewer checks

- Confirm no example JSON in the doc implies a schema the codec would have to
  parse — the codec must remain oblivious to `metadata_json` contents.
- Sanity-check the `claim_domain` values against ticket 0013's closed set.
- Confirm the promotion-path note is present so a future dedicated field is a
  conscious format decision, not a silent metadata dependency.
