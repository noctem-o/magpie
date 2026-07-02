# Ticket 0004 — READ-ONLY survey: deadbolt witness/evidence machinery census

## 1. Goal

Produce a factual survey of deadbolt's existing witness, attestation, and
evidence machinery, as input to the Magpie↔deadbolt seam design. This ticket
produces a REPORT, not code. No file in the repo may be modified.

The report must answer:

1. **Event inventory.** Every distinct witness/attestation/evidence record
   type the kernel can emit (from `kernel_witness`, `cog-attestation`,
   `cog-evidence`, and anywhere else). For each: name, fields, approximate
   serialized size, and what triggers it.
2. **Rate profile.** Per record type: emitted how often, roughly? (per
   tool-dispatch / per policy decision / per session / per boot). Where exact
   rates aren't derivable from code, say "unknown" — do not estimate from
   vibes.
3. **Chain structure.** How records are currently chained/rooted (BLAKE3
   hash-chain details, root computation, where roots surface), and which
   canonicalization profile(s) from `cog-canonical` are in use where.
4. **Trust roots and keys.** What signs what, with which keys, and where
   verification currently happens.
5. **Natural seal points.** Boundaries in the existing design where a segment
   of witness history is already summarized, rooted, or closed (end of run?
   policy epoch? manifest emission?) — candidates for anchor cadence.
6. **Coupling map.** Which crates would be touched if (a) witness events were
   re-emitted as magpie-log events wholesale, versus (b) only segment roots
   were exported. List crate names and the specific seams, no code changes.

## 2. Scope

- May read: the entire deadbolt repository.
- May write: exactly one file, the report — `SURVEY.md` at the repo root of
  the working copy (do not commit it).
- Harness note: run this in WRITE mode. Read mode's sandbox rejects all file
  writes including the report itself (learned the hard way — run
  `20260702T151936Z-539440514` completed the census and lost the report).
  Write mode's isolated worktree keeps the no-modification rule reviewer-
  enforceable: `git status` must show nothing but the untracked `SURVEY.md`.
- Belt and braces: ALSO include the full report verbatim in the final
  response, so the summary file preserves it even if the file write fails.

## 3. Non-goals

- No code changes, no refactors, no "while I was in there" fixes.
- No design recommendation. Facts and structure only; the design decision is
  explicitly reserved (AGENTS.md: deadbolt integration authority is not
  delegated). If the surveyor has an opinion, it goes in a clearly marked
  final section titled "Surveyor notes (non-authoritative)".

## 4. Constraints

- Every claim in the report must cite a file path (and ideally a symbol name).
  Uncited claims will be treated as unverified.
- Prefer tables. Keep the whole report under ~300 lines.

## 5. Tests

None — read-only. Reviewer validates by spot-checking citations against the
codebase.

## 6. Acceptance criteria

- All six questions answered or explicitly marked unknown with reasons.
- Zero modifications to tracked files (`git status` shows nothing except the
  untracked `SURVEY.md`).
- Citations spot-check correctly.

## 7. Frozen surfaces

Everything. This is a read-only run.
