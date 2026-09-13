//! Experimental governed historical ingress for SQLite L0.
//!
//! This module is a composition guarantee: a caller holding only an
//! [`AgentProposalIngress`] can perform the one reviewed category operation,
//! but cannot recover the underlying writer or select another payload kind.
//! It is not a global guarantee that every Magpie writer is session-mediated;
//! standalone public [`magpie_log::LogWriter`] construction remains available.
//!
//! The session is intentionally not generic:
//!
//! ```compile_fail,E0308
//! use magpie_claims::historical_ingress::HistoricalIngressSession;
//! use magpie_log::{LogWriter, MemStore};
//! fn no_mem_session(writer: LogWriter<MemStore>) {
//!     HistoricalIngressSession::from_writer(writer);
//! }
//! ```
//!
//! The category capability has no raw writer or arbitrary-payload escape:
//!
//! ```compile_fail,E0599
//! use magpie_claims::historical_ingress::AgentProposalIngress;
//! use magpie_log::Payload;
//! fn no_raw_append(ingress: &mut AgentProposalIngress<'_>, payload: Payload) {
//!     ingress.append(payload);
//! }
//! ```
//!
//! ```compile_fail,E0616
//! use magpie_claims::historical_ingress::HistoricalIngressSession;
//! fn no_writer_escape(session: &HistoricalIngressSession) {
//!     let _ = session.writer;
//! }
//! ```

use magpie_log::{LogError, LogWriter, Payload, Provenance, SignedEvent, SqliteL0Store};

/// A typed agent claim proposal.
///
/// The request contains only fields required by the existing
/// `Payload::ClaimAssertedV2` representation. Actor class is fixed by the
/// ingress and status is not part of this request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentClaimProposal {
    pub claim_id: String,
    pub statement: String,
    pub scope_ref: String,
    pub content_hash: String,
    pub metadata_json: String,
}

impl AgentClaimProposal {
    pub fn new(
        claim_id: impl Into<String>,
        statement: impl Into<String>,
        scope_ref: impl Into<String>,
        content_hash: impl Into<String>,
        metadata_json: impl Into<String>,
    ) -> Self {
        Self {
            claim_id: claim_id.into(),
            statement: statement.into(),
            scope_ref: scope_ref.into(),
            content_hash: content_hash.into(),
            metadata_json: metadata_json.into(),
        }
    }
}

/// SQLite-backed custody session for reviewed historical ingress categories.
///
/// The session takes an already-open writer by value. Its private field has no
/// accessor, conversion, `Deref`, `AsRef`, `Clone`, or serialization surface.
/// Creating or dropping the session does not append anything.
pub struct HistoricalIngressSession {
    writer: LogWriter<SqliteL0Store>,
}

impl HistoricalIngressSession {
    /// Transfer custody of an already-open SQLite writer into the session.
    pub fn from_writer(writer: LogWriter<SqliteL0Store>) -> Self {
        Self { writer }
    }

    /// Borrow the reviewed agent-proposal capability.
    pub fn agent_proposals(&mut self) -> AgentProposalIngress<'_> {
        AgentProposalIngress {
            writer: &mut self.writer,
        }
    }
}

/// Category-specific capability for agent claim proposals.
///
/// This capability has no arbitrary payload, status, actor-class, or writer
/// access. The only supported operation constructs `ClaimAssertedV2` with
/// `actor_class = "AgentProposer"` and delegates persistence to the existing
/// SQLite writer unchanged.
pub struct AgentProposalIngress<'session> {
    writer: &'session mut LogWriter<SqliteL0Store>,
}

impl AgentProposalIngress<'_> {
    pub fn propose_claim(
        &mut self,
        proposal: AgentClaimProposal,
        provenance: Provenance,
    ) -> Result<SignedEvent, LogError> {
        self.writer.append(
            provenance,
            Payload::ClaimAssertedV2 {
                claim_id: proposal.claim_id,
                statement: proposal.statement,
                scope_ref: proposal.scope_ref,
                actor_class: "AgentProposer".to_owned(),
                content_hash: proposal.content_hash,
                metadata_json: proposal.metadata_json,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use magpie_log::{L0ResourceLimitsV0, SigningKey};
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_PATH: AtomicU64 = AtomicU64::new(0);

    fn path() -> PathBuf {
        let serial = NEXT_PATH.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "magpie-historical-ingress-{}-{serial}.db",
            std::process::id()
        ))
    }

    fn limits() -> L0ResourceLimitsV0 {
        L0ResourceLimitsV0::new(32 * 1024 * 1024, 100, 2 * 1024 * 1024, 24 * 1024 * 1024).unwrap()
    }

    fn key() -> SigningKey {
        SigningKey::from_bytes(&[7u8; 32])
    }

    #[test]
    fn proposal_forces_agent_actor_and_appends_one_typed_event() {
        let path = path();
        let writer = LogWriter::<SqliteL0Store>::create_new(&path, key(), limits()).unwrap();
        let mut session = HistoricalIngressSession::from_writer(writer);
        let before = session.writer.len();

        let event = session
            .agent_proposals()
            .propose_claim(
                AgentClaimProposal::new("c-1", "statement", "scope-1", "", "{}"),
                Provenance::new("caller", "asserted-source"),
            )
            .unwrap();

        assert_eq!(session.writer.len(), before + 1);
        assert_eq!(event.core.provenance.agent, "caller");
        assert_eq!(event.core.provenance.source, "asserted-source");
        assert!(matches!(
            event.core.payload,
            Payload::ClaimAssertedV2 {
                actor_class,
                ..
            } if actor_class == "AgentProposer"
        ));
        assert_eq!(
            magpie_log::LogReader::<SqliteL0Store>::verify_persisted_prefix(
                &path,
                key().verifying_key(),
                limits(),
            )
            .unwrap()
            .event_count(),
            before + 1
        );
        cleanup(&path);
    }

    #[test]
    fn session_construction_and_drop_do_not_append() {
        let path = path();
        let writer = LogWriter::<SqliteL0Store>::create_new(&path, key(), limits()).unwrap();
        let count = writer.len();
        let session = HistoricalIngressSession::from_writer(writer);
        drop(session);

        assert_eq!(
            magpie_log::LogReader::<SqliteL0Store>::verify_persisted_prefix(
                &path,
                key().verifying_key(),
                limits(),
            )
            .unwrap()
            .event_count(),
            count
        );
        cleanup(&path);
    }

    fn cleanup(path: &std::path::Path) {
        for suffix in ["", "-journal", "-wal", "-shm"] {
            let path = PathBuf::from(format!("{}{}", path.display(), suffix));
            let _ = std::fs::remove_file(path);
        }
    }
}
