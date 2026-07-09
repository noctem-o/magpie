# Portable Base Boundary

## Purpose

Magpie core is a machine-agnostic, local-first research memory base. It records
signed append-only events, replays them deterministically, and derives every
projection from that record.

Base invariant:

> Magpie runs alone; integrations add evidence, not authority.

This page defines the base boundary before later integration work changes
Deadbolt pins, fixtures, or optional read/propose tooling.

## Core Requirements

The portable base requires only:

- a Rust toolchain;
- filesystem access for the append-only log and local artifacts;
- a Magpie log using the active canonical profile;
- deterministic replay from the log;
- optional SQLite projection support for `magpie-episodic`.

SQLite is a projection dependency, not a trust dependency. A projection may be
deleted and rebuilt from the log.

## Core Non-Requirements

Magpie core must not require:

- Deadbolt installed;
- Hyprland;
- Tailscale;
- a local LLM daemon;
- an always-on server;
- special GPU or NPU hardware;
- maintainer-specific paths such as `C:\magpie`;
- private machine topology.

Those systems may exist around one installation. They are not base
requirements and must not be assumed by core tests, examples, the verifier, or
portable projections.

## Portable Base Components

The portable base includes:

- `magpie-log`;
- `magpie-claims`;
- `magpie-episodic`;
- `tools/verify_chain.py`;
- deterministic examples, tests, and fixtures.

These components should remain useful on ordinary machines. They conserve the
log, derive the rest, and keep authority explicit.

## Optional Future Layers

The following are future work, not base trust architecture:

- librarian/navigator read and propose tooling;
- model-backed summarization;
- always-on indexing;
- remote sync;
- UI, wiki, or knowledge graph projections.

These layers may read from Magpie or propose events through later governed
surfaces. They do not become sources of authority merely by existing.

## Deadbolt Boundary

Deadbolt is optional. A Magpie installation can run, test, verify, replay, and
project without Deadbolt.

When present, Deadbolt may add evidence through the `SegmentAnchored` protocol
defined in `docs/seams/deadbolt-anchor-contract.md`. That seam records
occurrence/inclusion material only. It does not settle interpretation truth,
create typed claims by itself, or make Deadbolt a base dependency.
