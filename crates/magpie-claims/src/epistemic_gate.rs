//! A-022 EpistemicGate capability-skeleton spike.
//!
//! Experimental. NOT ratified doctrine. NOT A-022 closure. NOT an admission
//! implementation. NOT an ordinary write surface. Safe to discard: the
//! explored topology and the decisions deliberately left open are recorded in
//! `docs/design/epistemic-gate-skeleton-spike.md`.
//!
//! This module is the smallest honest capability shell for the future
//! EpistemicGate boundary: a crate-private type that custodies a
//! [`magpie_log::LogWriter`] by value. It demonstrates the custody shape —
//! the gate holds the chain's sole append capability, and no code outside
//! this crate can name the type, construct it, reach the writer, or select a
//! write — without choosing what admission itself will mean.
//!
//! The type is unreachable from outside this crate:
//!
//! ```compile_fail
//! use magpie_claims::epistemic_gate::EpistemicGate;
//! ```
//!
//! No public constructor, accessor, conversion, `Deref`, `Clone`, or `serde`
//! implementation exists on the type. Construction consumes the supplied
//! writer and appends nothing, and the type exposes no `admit`, `submit`,
//! `write`, or `append` operation.

use magpie_log::LogWriter;

/// Inert capability shell for the future EpistemicGate boundary.
///
/// Custodies a [`LogWriter`] by value. The field is private and has no
/// accessor, conversion, `Deref`, or writer-returning method: only
/// crate-private code inside this module can reach the writer, and only once
/// the future admission boundary is ratified. The generic parameter is the
/// store the writer already addresses; the gate adds no backend of its own.
pub(crate) struct EpistemicGate<S> {
    writer: LogWriter<S>,
}

impl<S> EpistemicGate<S> {
    /// Take custody of an existing [`LogWriter`].
    ///
    /// Crate-private on purpose: there is no public construction path.
    /// Construction appends nothing and changes no chain state.
    pub(crate) fn from_writer(writer: LogWriter<S>) -> Self {
        Self { writer }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey};

    const SEED: [u8; 32] = [7u8; 32];

    fn test_key() -> SigningKey {
        SigningKey::from_bytes(&SEED)
    }

    fn writer(store: MemStore) -> LogWriter<MemStore> {
        let mut tick = 0u64;
        LogWriter::<MemStore>::open_with_clock(
            store,
            test_key(),
            Box::new(move || {
                tick += 1;
                tick
            }),
        )
        .unwrap()
    }

    #[test]
    fn construction_takes_custody_without_appending() {
        let w = writer(MemStore::new());
        let len_before = w.len();
        let tip_before = w.tip();

        let gate = EpistemicGate::from_writer(w);

        assert_eq!(gate.writer.len(), len_before);
        assert_eq!(gate.writer.tip(), tip_before);
    }

    #[test]
    fn custody_preserves_the_chain_and_dropping_leaves_it_verifiable() {
        let store = MemStore::new();
        let control = store.clone();
        let mut w = writer(store);
        w.append(
            Provenance::new("test", "epistemic-gate-spike"),
            Payload::Note {
                text: "pre-custody".into(),
            },
        )
        .unwrap();
        let len_before = w.len();
        let tip_before = w.tip();

        let gate = EpistemicGate::from_writer(w);
        assert_eq!(gate.writer.len(), len_before);
        assert_eq!(gate.writer.tip(), tip_before);
        drop(gate);

        let reader = LogReader::<MemStore>::open(control, test_key().verifying_key());
        assert_eq!(reader.verify_chain().unwrap(), len_before);
    }
}
