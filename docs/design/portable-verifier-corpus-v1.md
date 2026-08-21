# Portable Verifier Corpus v1

## Status and identity

This is the **candidate normative portable-verifier corpus/profile manifest**
required by Accepted ADR-0010. It is pending owner review and merge. It is not
an Accepted ADR, a conformer implementation, cross-language conformance
evidence, or finding closure.

Its immutable candidate identity is:

```text
magpie-portable-verifier-corpus-v1
```

The machine-readable artifact is
[`fixtures/verifier-language-v1/manifest.json`](../../fixtures/verifier-language-v1/manifest.json).
Once owner-approved and merged, this identity names the exact governing-source
commitments, input bytes, external-key text, and expected results in that
manifest. It is never a mutable `latest` alias. Any semantic change to a case
input, expected result, source commitment, failure vocabulary, or coverage
inventory requires a distinct manifest identity. Corrections made while this
artifact remains an unmerged candidate are review amendments, not silent
repointing of an accepted identity.

## Governing composition

The manifest binds every case to these layers:

- `docs/FORMAT.md` and `magpie-core-v1` own canonical `EventCore` bytes, hashes,
  `magpie-sig-v1`, chain order, payload validation, and Genesis rules;
- `docs/design/portable-verifier-input-language-contract.md` owns the reference
  JSONL/framing/schema language, external-key text decoding, stable portable
  failures, coordinates, and first-failure order; and
- Accepted ADR-0010 owns
  `V_sig = "magpie-ed25519-canonical-prime-subgroup-v1"`, including exact
  `A_bytes`, `R`, `S`, message, challenge, subgroup, and equation semantics.

Each governing file is committed by repository-root-relative path and SHA-256
at the ratified semantic baseline. The manifest fixes `V_sig` directly for all
cases. There is no ambient, default, negotiated, `latest`, substituted, or
fallback signature profile.

This corpus does **not** mint the complete `G_history` required by ADR-0008.
It freezes conformance cases over the cited semantics while the eventual
complete history-verification derivation identity remains separate work. A
portable case result is identified by the pair:

```text
(magpie-portable-verifier-corpus-v1, case_id)
```

Detaching a result from that manifest context loses required producing
semantics. Conversely, carrying only `V_sig` remains insufficient because it
does not identify transport interpretation, history checks, failure semantics,
the supplied key, or the exact supplied input.

## Manifest result law

Every one of the 432 cases has exactly one expected semantic result:

```text
ACCEPT {
    event_count,
    tip,
    ordered_recomputed_hashes
}

REJECT {
    class,
    line,
    record_index
}
```

There is no dependency sentinel and no `UNKNOWN`, `INDETERMINATE`, partial,
gate-only, or operational result in the normative case set. Operational
inability to execute a conformer is outside the semantic verdict vocabulary.

The stable rejection classes are:

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

`ExternalKey` precedes all input processing. Per-record precedence remains:

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

The manifest records 19 ACCEPT cases and 413 REJECT cases:

| First class | Cases |
| --- | ---: |
| `ExternalKey` | 21 |
| `Framing` | 13 |
| `JsonSyntax` | 18 |
| `Schema` | 272 |
| `Sequence` | 4 |
| `PreviousLink` | 5 |
| `ContentHash` | 3 |
| `Signature` | 16 |
| `PayloadValidation` | 48 |
| `Genesis` | 13 |

## Coverage inventories

The manifest makes the required coverage mechanical rather than inferential:

- all N1–N30 hostile families;
- K1–K7 and K9–K15, composed with ADR-0010's stricter `A_bytes`
  admissibility;
- P1–P12 corpus and future-conformer proof obligations;
- U64-P1–U64-P5 exact numeric boundaries;
- every required status, actor class, evidence kind, edge kind, and payload
  discriminator in an ACCEPT history;
- exactly one literal duplicate, omission, and null vector for each of 57
  required member coordinates;
- one isolated unknown-member vector for `SignedEvent`, `EventCore`,
  `Provenance`, and all nine payload variants; and
- focused escaped-name duplicates proving comparison after JSON member-name
  decoding.

P3–P6, P10, and P11 retain their future conformer-level assertions in the
manifest. The corpus supplies their exact inputs and expected results but does
not claim that Rust, Python, or Go already agrees.

## A-021 classifications

The formerly unresolved A-021 witnesses are now ordinary normative cases:

| Witness | Ratified portable result | First gate | Reconstruction |
| --- | --- | --- | --- |
| A21-S4 | `REJECT(ExternalKey)` | `A != I` | Exact historical identity-`A`, identity-`R`, `S=0` full Genesis record retained. |
| A21-S5 | `REJECT(ExternalKey)` | `A != I` | Exact historical identity-`A`, `R=-B`, `S=L-1` full Genesis record retained. |
| A21-D1 | `REJECT(ExternalKey)` | `[L]A != I` | A complete two-record order-two-root chain was reconstructed; both ordinary equations were independently checked before the selected key gate rejects. |
| A21-D2 | `ACCEPT` | none | The frozen golden Genesis core uses canonical `R=I` and keyholder-derived canonical `S`; exact V_sig equation and full pipeline were recomputed. |
| A21-T2 | `REJECT(ExternalKey)` | `[L]A != I` | The exact mixed-torsion equation-true historical Genesis construction is retained. |

Additional complete signature cases cover malformed/non-canonical `R`,
identity-`R` equation false, non-identity small-order and mixed-torsion `R`,
`S=0`, `S=L-1`, `S=L`, `S=L+1`, maximal `S`, altered messages, altered
signatures, and cofactor-sensitive residues. A Signature-stage case first
passes external-key, framing, JSON, schema, sequence, link, and content-hash
checks.

## Exact-byte and fixture authority

New case bytes live under `fixtures/verifier-language-v1/cases/` and are
covered by the path-specific Git rule:

```gitattributes
fixtures/verifier-language-v1/cases/** binary
```

This preserves CRLF, mixed EOLs, lone CR, BOM, invalid UTF-8, absent final
terminators, duplicate members, and invalid literal number tokens. Every case
has a manifest SHA-256 and byte-property record.

The existing golden and Deadbolt fixtures remain the sole byte authority for
those histories. P1/P2 reference them in place; the corpus does not duplicate
or regenerate them. Their frozen SHA-256 values remain:

```text
crates/magpie-log/testdata/golden-v1.jsonl
767ef86b7e2ad1d65e8315ae28bac682bceee29223b5f1a0612ec7ef062cffb4

fixtures/deadbolt-anchor-v1/anchor-log.jsonl
2c715006e3e5bef571b3d31b95f47d3c4bbb3ea68fa9750365b4a62b66cd41e5
```

[`tools/check_verifier_corpus_manifest.py`](../../tools/check_verifier_corpus_manifest.py)
checks manifest structure, unique IDs, source/file hashes, result completeness,
all mechanical inventories, named A-021 classifications, and representative
hostile byte properties. It does not parse histories, implement V_sig, or
override a reviewed expected result.

## Security and epistemic boundary

An ACCEPT result establishes only that the exact supplied input satisfies the
selected composed verification semantics under the exact externally supplied
key. It does not establish key ownership, key trust, identity, authority,
admission, truth, freshness, an expected head, canonical history, global
completeness, non-equivocation, durability, or permission to act.

The external key remains external. A structurally admissible key is not thereby
trusted. The profile identity remains outside signed `magpie-core-v1` bytes.

## Review and follow-up

This candidate requires hostile corpus review before implementation. Reviewers
should independently reconstruct A21-D1/D2 and the cofactor-sensitive cases,
verify every supposed late-stage failure reaches its claimed first gate, and
prove exact bytes survive a clean checkout.

After owner review and merge, the smallest next implementation tranche is a
typed Rust `V_sig` and explicit profile-selection path exercised over this
corpus, without changing the legacy unprofiled verifier. Matching Python and
independent Go conformers follow. A-021, A-004, and RQ-006 remain open until
all conformers agree and the separate administrative closure gate is met.
