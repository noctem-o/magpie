//! # magpie-claims — the epistemic claim store, as a projection
//!
//! [`ClaimsView`] folds log events into per-claim state: a statement, its
//! current position on the epistemic spectrum, accumulated evidence, and a
//! transition count. It is *derived*: drop it and rebuild it from the log at
//! any time. It reads through a `LogReader` and cannot write. That asymmetry
//! is enforced by the type system.

pub mod policy;

mod standing;

use std::collections::BTreeMap;

use magpie_log::{Payload, Projection, SignedEvent, Status};
use serde::Serialize;

pub use standing::{
    StandingClaim, StandingView, TypedClaimNode, TypedEvidenceNode, TypedJustificationEdge,
};

/// A single tracked claim and its current epistemic state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Claim {
    pub statement: String,
    pub status: Status,
    pub evidence: Vec<String>,
    pub transitions: u64,
}

/// The derived claim store. `BTreeMap` (not `HashMap`) so serialization is
/// deterministic — which is what lets us assert byte-identical regeneration.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct ClaimsView {
    pub claims: BTreeMap<String, Claim>,
}

impl ClaimsView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, claim_id: &str) -> Option<&Claim> {
        self.claims.get(claim_id)
    }

    pub fn len(&self) -> usize {
        self.claims.len()
    }

    pub fn is_empty(&self) -> bool {
        self.claims.is_empty()
    }

    /// Deterministic bytes for the "is this regenerable?" check.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("ClaimsView is always serializable")
    }
}

impl Projection for ClaimsView {
    fn apply(&mut self, event: &SignedEvent) {
        match &event.core.payload {
            Payload::ClaimAsserted {
                claim_id,
                statement,
                status,
            } => {
                self.claims
                    .entry(claim_id.clone())
                    .or_insert_with(|| Claim {
                        statement: statement.clone(),
                        status: *status,
                        evidence: Vec::new(),
                        transitions: 0,
                    });
            }
            Payload::EvidenceRecorded { claim_id, summary } => {
                if let Some(claim) = self.claims.get_mut(claim_id) {
                    claim.evidence.push(summary.clone());
                }
            }
            Payload::ClaimStatusChanged { claim_id, to, .. } => {
                if let Some(claim) = self.claims.get_mut(claim_id) {
                    claim.status = *to;
                    claim.transitions += 1;
                }
            }
            Payload::Note { .. } => {}
            Payload::Genesis { .. } => {}
            // Anchors commit execution evidence, not epistemic claims; the
            // claim store deliberately ignores them. (The episodic store is
            // where they surface as timeline rows.)
            Payload::SegmentAnchored { .. } => {}
            // ADR-0002 typed events are interpreted by StandingView, not the
            // legacy ClaimsView. Keep this compatibility projection explicit.
            Payload::ClaimAssertedV2 { .. } => {}
            Payload::EvidenceRegistered { .. } => {}
            Payload::JustificationEdgeRecorded { .. } => {}
        }
    }
}
