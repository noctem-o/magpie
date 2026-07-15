use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

use crate::canonical::{CANONICALIZATION_PROFILE, SIG_DOMAIN};
use crate::error::LogError;
use crate::event::{EventCore, Payload, Provenance, Sig, SignedEvent};
use crate::hashing::ContentHash;
use crate::store::LogStore;

/// A source of timestamps (nanoseconds). Injectable so tests are deterministic.
pub type Clock = Box<dyn FnMut() -> u64>;

fn system_clock() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}

/// The message actually signed: `magpie-sig-v1 || content-hash`. Domain
/// separation — this key's signatures over log events can never be confused
/// with its signatures over anything else it may sign.
fn signed_message(hash: &ContentHash) -> Vec<u8> {
    let mut msg = Vec::with_capacity(SIG_DOMAIN.len() + 32);
    msg.extend_from_slice(SIG_DOMAIN);
    msg.extend_from_slice(hash.as_bytes());
    msg
}

/// Walk the whole stored chain without retaining parsed events. Shared by
/// [`LogReader::verify_chain`] and [`LogWriter`]'s recovery on open.
fn verify_chain_on<S: LogStore>(store: &S, vk: &VerifyingKey) -> Result<VerifiedSummary, LogError> {
    let records = store.read_records()?;
    match parse_and_verify_records(&records, vk, VerificationRetention::SummaryOnly)? {
        VerifiedRecords::Summary(summary) => Ok(summary),
        VerifiedRecords::Snapshot(_) => {
            unreachable!("summary-only verification returned retained events")
        }
    }
}

#[derive(Clone, Copy)]
enum VerificationRetention {
    SummaryOnly,
    RetainEvents,
}

struct VerifiedSummary {
    count: u64,
    tip: ContentHash,
}

struct VerifiedSnapshot {
    events: Vec<SignedEvent>,
    tip: ContentHash,
}

enum VerifiedRecords {
    Summary(VerifiedSummary),
    Snapshot(VerifiedSnapshot),
}

/// Parse and verify one already-read record snapshot without consulting its
/// store again. Summary-only verification never allocates an event vector;
/// retained events are returned only after the complete snapshot verifies.
fn parse_and_verify_records(
    records: &[Vec<u8>],
    vk: &VerifyingKey,
    retention: VerificationRetention,
) -> Result<VerifiedRecords, LogError> {
    let mut prev = ContentHash::ZERO;
    let mut count = 0u64;
    let mut events = match retention {
        VerificationRetention::SummaryOnly => None,
        VerificationRetention::RetainEvents => Some(Vec::with_capacity(records.len())),
    };

    for (expected_seq, record) in records.iter().enumerate() {
        let expected_seq = expected_seq as u64;
        let event: SignedEvent = serde_json::from_slice(record)?;

        if event.core.seq != expected_seq {
            return Err(LogError::ChainBroken {
                seq: event.core.seq,
                detail: format!("out-of-order: expected seq {expected_seq}"),
            });
        }
        if event.core.prev_hash != prev {
            return Err(LogError::ChainBroken {
                seq: event.core.seq,
                detail: "prev_hash does not match the previous event".into(),
            });
        }
        if event.core.hash() != event.hash {
            return Err(LogError::ChainBroken {
                seq: event.core.seq,
                detail: "content hash mismatch (event was altered)".into(),
            });
        }
        let sig = Signature::from_bytes(event.signature.as_bytes());
        vk.verify(&signed_message(&event.hash), &sig)
            .map_err(|_| LogError::BadSignature {
                seq: event.core.seq,
            })?;
        event
            .core
            .payload
            .validate()
            .map_err(|detail| LogError::ChainBroken {
                seq: event.core.seq,
                detail: detail.into(),
            })?;

        // Genesis rules: exactly one, exactly at seq 0, self-describing and
        // consistent with the externally provided key. Trust in the key comes
        // from outside; the genesis makes the chain *self-describing*.
        match (&event.core.payload, event.core.seq) {
            (
                Payload::Genesis {
                    canonicalization_profile,
                    verifying_key,
                },
                0,
            ) => {
                if canonicalization_profile != CANONICALIZATION_PROFILE {
                    return Err(LogError::ChainBroken {
                        seq: 0,
                        detail: format!(
                            "genesis declares profile {canonicalization_profile:?}; \
this implementation verifies {CANONICALIZATION_PROFILE:?}"
                        ),
                    });
                }
                if verifying_key != &hex::encode(vk.as_bytes()) {
                    return Err(LogError::ChainBroken {
                        seq: 0,
                        detail: "genesis-declared verifying key does not match the provided key"
                            .into(),
                    });
                }
            }
            (Payload::Genesis { .. }, seq) => {
                return Err(LogError::ChainBroken {
                    seq,
                    detail: "genesis event after seq 0".into(),
                });
            }
            (_, 0) => {
                return Err(LogError::ChainBroken {
                    seq: 0,
                    detail: "chain does not begin with a genesis event".into(),
                });
            }
            _ => {}
        }

        prev = event.hash;
        count += 1;
        if let Some(events) = &mut events {
            events.push(event);
        }
    }

    match events {
        None => Ok(VerifiedRecords::Summary(VerifiedSummary {
            count,
            tip: prev,
        })),
        Some(events) => Ok(VerifiedRecords::Snapshot(VerifiedSnapshot {
            events,
            tip: prev,
        })),
    }
}

/// The **write capability**. Possessing a `LogWriter` is the authority to append.
/// In the full system the gate (`deadbolt`) is its only holder. There is no
/// `update` or `delete` — corrections are new, superseding events.
pub struct LogWriter<S: LogStore> {
    store: S,
    signing_key: SigningKey,
    last_hash: ContentHash,
    next_seq: u64,
    clock: Clock,
}

impl<S: LogStore> LogWriter<S> {
    /// Open for appending, recovering and verifying any existing chain. Opening
    /// an **empty** store writes the genesis event (profile + verifying key)
    /// as seq 0 before returning. Uses the system clock.
    pub fn open(store: S, signing_key: SigningKey) -> Result<Self, LogError> {
        Self::open_with_clock(store, signing_key, Box::new(system_clock))
    }

    /// Open with an injected clock (deterministic tests).
    pub fn open_with_clock(
        store: S,
        signing_key: SigningKey,
        clock: Clock,
    ) -> Result<Self, LogError> {
        let vk = signing_key.verifying_key();
        let verified = verify_chain_on(&store, &vk)?;
        let mut writer = Self {
            store,
            signing_key,
            last_hash: verified.tip,
            next_seq: verified.count,
            clock,
        };
        if writer.next_seq == 0 {
            writer.append(
                Provenance::new("magpie-log", "genesis"),
                Payload::Genesis {
                    canonicalization_profile: CANONICALIZATION_PROFILE.to_string(),
                    verifying_key: hex::encode(vk.as_bytes()),
                },
            )?;
        }
        Ok(writer)
    }

    /// Append an event: build the core, hash it (chaining onto the tip), sign the
    /// hash, persist, and advance the tip. This is the only write path in Magpie.
    pub fn append(
        &mut self,
        provenance: Provenance,
        payload: Payload,
    ) -> Result<SignedEvent, LogError> {
        let is_genesis = matches!(payload, Payload::Genesis { .. });
        if is_genesis != (self.next_seq == 0) {
            return Err(LogError::ChainBroken {
                seq: self.next_seq,
                detail: if is_genesis {
                    "genesis may only be written at seq 0".into()
                } else {
                    "seq 0 must be the genesis event".into()
                },
            });
        }
        payload.validate().map_err(|detail| LogError::ChainBroken {
            seq: self.next_seq,
            detail: detail.into(),
        })?;
        let core = EventCore {
            seq: self.next_seq,
            timestamp_nanos: (self.clock)(),
            prev_hash: self.last_hash,
            provenance,
            payload,
        };
        let hash = core.hash();
        let signature = self.signing_key.sign(&signed_message(&hash));
        let signed = SignedEvent {
            core,
            hash,
            signature: Sig::new(signature.to_bytes()),
        };

        let record = serde_json::to_vec(&signed)?;
        self.store.append_record(&record)?;

        self.last_hash = hash;
        self.next_seq += 1;
        Ok(signed)
    }

    /// The current chain tip.
    pub fn tip(&self) -> ContentHash {
        self.last_hash
    }

    /// Number of events written.
    pub fn len(&self) -> u64 {
        self.next_seq
    }

    pub fn is_empty(&self) -> bool {
        self.next_seq == 0
    }
}

/// Read-only access to the log. Holds no signing key and exposes no `append`:
/// memory layers get one of these, so "the gate is the only writer" is a
/// compile-time fact, not a guideline.
pub struct LogReader<S: LogStore> {
    store: S,
    verifying_key: VerifyingKey,
}

impl<S: LogStore> LogReader<S> {
    pub fn open(store: S, verifying_key: VerifyingKey) -> Self {
        Self {
            store,
            verifying_key,
        }
    }

    /// All stored events, parsed but not verified. This parses the snapshot
    /// returned by its own [`LogStore::read_records`] call, but does not verify
    /// chain linkage, hashes, signatures, genesis binding, or payload validity.
    /// It must not be used as the source of authority-bearing projections.
    /// Prefer [`Self::verify_chain`] or [`Self::replay`] for trusted use.
    pub fn events(&self) -> Result<Vec<SignedEvent>, LogError> {
        self.store
            .read_records()?
            .iter()
            .map(|r| serde_json::from_slice::<SignedEvent>(r).map_err(LogError::from))
            .collect()
    }

    /// Validate the entire chain: sequence, prev-links, recomputed hashes, and
    /// every signature. Returns the number of valid events; any tampering errors.
    pub fn verify_chain(&self) -> Result<u64, LogError> {
        verify_chain_on(&self.store, &self.verifying_key).map(|verified| verified.count)
    }

    /// Fold one verified record snapshot into a projection. Replay reads one
    /// snapshot, verifies that exact snapshot completely, then folds those same
    /// parsed events. If snapshot parsing or verification fails, no event is
    /// applied. Trust in the supplied verifying key remains external.
    ///
    /// This is how every derived view is (re)built: drop the view, `replay`, and
    /// you are back exactly where you were — the log is the source of truth.
    pub fn replay<P: Projection>(&self, projection: &mut P) -> Result<u64, LogError> {
        let records = self.store.read_records()?;
        let verified = match parse_and_verify_records(
            &records,
            &self.verifying_key,
            VerificationRetention::RetainEvents,
        )? {
            VerifiedRecords::Snapshot(snapshot) => snapshot,
            VerifiedRecords::Summary(_) => {
                unreachable!("retained verification returned a summary")
            }
        };
        let count = verified.events.len() as u64;
        debug_assert_eq!(
            verified
                .events
                .last()
                .map(|event| event.hash)
                .unwrap_or(ContentHash::ZERO),
            verified.tip,
        );
        for event in &verified.events {
            projection.apply(event);
        }
        Ok(count)
    }
}

/// A derived view over the log. Implementors fold events into state; that state
/// is, by construction, regenerable from the log alone.
pub trait Projection {
    fn apply(&mut self, event: &SignedEvent);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::MemStore;

    #[test]
    fn summary_only_verification_returns_count_and_tip_without_events() {
        let signing_key = SigningKey::from_bytes(&[73u8; 32]);
        let verifying_key = signing_key.verifying_key();
        let store = MemStore::new();
        let expected_tip = {
            let mut timestamp = 0u64;
            let mut writer = LogWriter::open_with_clock(
                store.clone(),
                signing_key,
                Box::new(move || {
                    timestamp += 1;
                    timestamp
                }),
            )
            .unwrap();
            writer
                .append(
                    Provenance::new("test", "summary-retention"),
                    Payload::Note {
                        text: "first retained comparison event".into(),
                    },
                )
                .unwrap();
            writer
                .append(
                    Provenance::new("test", "summary-retention"),
                    Payload::Note {
                        text: "second retained comparison event".into(),
                    },
                )
                .unwrap()
                .hash
        };
        let records = store.records();
        let expected_events: Vec<SignedEvent> = records
            .iter()
            .map(|record| serde_json::from_slice(record).unwrap())
            .collect();

        let summary = match parse_and_verify_records(
            &records,
            &verifying_key,
            VerificationRetention::SummaryOnly,
        )
        .unwrap()
        {
            VerifiedRecords::Summary(summary) => summary,
            VerifiedRecords::Snapshot(_) => panic!("summary-only mode retained events"),
        };
        assert_eq!(summary.count, 3);
        assert_eq!(summary.tip, expected_tip);

        let snapshot = match parse_and_verify_records(
            &records,
            &verifying_key,
            VerificationRetention::RetainEvents,
        )
        .unwrap()
        {
            VerifiedRecords::Snapshot(snapshot) => snapshot,
            VerifiedRecords::Summary(_) => panic!("retained mode returned only a summary"),
        };
        assert_eq!(snapshot.events.len() as u64, summary.count);
        assert_eq!(snapshot.tip, summary.tip);
        assert_eq!(snapshot.events, expected_events);
    }
}
