use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

use crate::canonical::{CANONICALIZATION_PROFILE, SIG_DOMAIN};
use crate::error::LogError;
use crate::event::{EventCore, Payload, Provenance, Sig, SignedEvent};
use crate::hashing::ContentHash;
use crate::history_expectation::{
    evaluate_history_expectation_v0, HistoryCheckpointV0, HistoryExpectationEvaluationV0,
    HistoryExpectationRelationV0,
};
use crate::store::{FileStore, LogStore, MemStore, WriterStore};

/// A source of timestamps (nanoseconds). Injectable so tests are deterministic.
pub type Clock = Box<dyn FnMut() -> u64>;

pub(crate) fn system_clock() -> u64 {
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

pub(crate) struct VerifiedSummary {
    pub(crate) count: u64,
    pub(crate) tip: ContentHash,
    pub(crate) total_record_bytes: u64,
}

struct VerifiedSnapshot {
    events: Vec<SignedEvent>,
    tip: ContentHash,
}

enum VerifiedRecords {
    Summary(VerifiedSummary),
    Snapshot(VerifiedSnapshot),
}

/// An ephemeral projection input borrowed from one completely verified replay snapshot.
///
/// A value of this type means only that [`SignedEvent`] is currently being
/// presented by Magpie's replay path from a retained record snapshot whose
/// complete replay verification has already succeeded under this
/// [`LogReader`]'s supplied verifying key. Trust in that key remains external.
/// It does not establish trust, authorization, admission, identity, truth,
/// currentness, standing, confidence, permission, or scientific correctness.
///
/// Downstream code can inspect the raw event but cannot construct, clone,
/// deserialize, convert, or retain this replay input as a portable witness.
///
/// The private field prevents downstream struct-literal construction:
///
/// ```compile_fail,E0451
/// use magpie_log::{SignedEvent, VerifiedReplayEvent};
///
/// fn cannot_wrap(raw: &SignedEvent) -> VerifiedReplayEvent<'_> {
///     VerifiedReplayEvent { event: raw }
/// }
/// ```
///
/// Raw events have no conversion route into replay inputs:
///
/// ```compile_fail,E0277
/// use magpie_log::{SignedEvent, VerifiedReplayEvent};
///
/// fn cannot_convert(raw: &SignedEvent) -> VerifiedReplayEvent<'_> {
///     raw.into()
/// }
/// ```
///
/// Serialized material cannot mint a replay input:
///
/// ```compile_fail,E0277
/// use magpie_log::VerifiedReplayEvent;
///
/// fn cannot_deserialize<'event>(json: &str) -> VerifiedReplayEvent<'event> {
///     serde_json::from_str(json).unwrap()
/// }
/// ```
///
/// A projection cannot retain the borrowed wrapper beyond its callback:
///
/// ```compile_fail
/// use magpie_log::{Projection, VerifiedReplayEvent};
///
/// struct RetainingProjection<'event> {
///     retained: Option<&'event VerifiedReplayEvent<'event>>,
/// }
///
/// impl Projection for RetainingProjection<'_> {
///     fn apply(&mut self, event: &VerifiedReplayEvent<'_>) {
///         self.retained = Some(event);
///     }
/// }
/// ```
pub struct VerifiedReplayEvent<'event> {
    event: &'event SignedEvent,
}

impl<'event> VerifiedReplayEvent<'event> {
    fn from_verified_snapshot(event: &'event SignedEvent) -> Self {
        Self { event }
    }

    /// Observe the raw historical event presented by this replay callback.
    ///
    /// Cloning the returned value produces only a raw [`SignedEvent`]; it does
    /// not produce another verified replay input.
    pub fn event(&self) -> &'event SignedEvent {
        self.event
    }
}

/// Exact count and chain tip returned by one completely verified replay.
///
/// The fields are private and the type has no public constructor:
///
/// ```compile_fail
/// use magpie_log::{ContentHash, VerifiedReplaySummary};
/// let _summary = VerifiedReplaySummary {
///     event_count: 0,
///     tip: ContentHash::ZERO,
/// };
/// ```
///
/// Serialized material cannot be deserialized into a replay summary:
///
/// ```compile_fail
/// use magpie_log::VerifiedReplaySummary;
/// let _: VerifiedReplaySummary = serde_json::from_str("{}").unwrap();
/// ```
///
/// A copied summary cannot authorize applying another record set:
///
/// ```compile_fail
/// use magpie_log::{LogReader, LogStore, Projection, VerifiedReplaySummary};
/// fn substitute<S: LogStore, P: Projection>(
///     reader: &LogReader<S>,
///     projection: &mut P,
///     summary: VerifiedReplaySummary,
/// ) {
///     let _ = reader.replay_with_summary(projection, summary);
/// }
/// ```
///
/// Nor can a genuine summary wrap a caller-selected raw event:
///
/// ```compile_fail,E0599
/// use magpie_log::{SignedEvent, VerifiedReplayEvent, VerifiedReplaySummary};
///
/// fn cannot_bless<'event>(
///     summary: &VerifiedReplaySummary,
///     raw: &'event SignedEvent,
/// ) -> VerifiedReplayEvent<'event> {
///     summary.wrap(raw)
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VerifiedReplaySummary {
    event_count: u64,
    tip: ContentHash,
}

impl VerifiedReplaySummary {
    pub(crate) fn from_verified_parts(event_count: u64, tip: ContentHash) -> Self {
        Self { event_count, tip }
    }

    pub fn event_count(&self) -> u64 {
        self.event_count
    }

    pub fn tip(&self) -> ContentHash {
        self.tip
    }
}

/// Parse and verify one already-read record snapshot without consulting its
/// store again. Summary-only verification never allocates an event vector;
/// retained events are returned only after the complete snapshot verifies.
fn parse_and_verify_records(
    records: &[Vec<u8>],
    vk: &VerifyingKey,
    retention: VerificationRetention,
) -> Result<VerifiedRecords, LogError> {
    let mut verifier = IncrementalVerifier::new(*vk);
    let mut total_record_bytes = 0u64;
    let mut events = match retention {
        VerificationRetention::SummaryOnly => None,
        VerificationRetention::RetainEvents => Some(Vec::with_capacity(records.len())),
    };

    for record in records {
        total_record_bytes = total_record_bytes
            .checked_add(u64::try_from(record.len()).map_err(|_| LogError::SequenceExhausted)?)
            .ok_or(LogError::SequenceExhausted)?;
        let event = verifier.verify_record(verifier.count(), record)?;
        if let Some(events) = &mut events {
            events.push(event);
        }
    }

    let summary = verifier.finish(total_record_bytes);

    match events {
        None => Ok(VerifiedRecords::Summary(summary)),
        Some(events) => Ok(VerifiedRecords::Snapshot(VerifiedSnapshot {
            events,
            tip: summary.tip,
        })),
    }
}

/// Shared incremental `magpie-core-v1` verifier. SQLite feeds one bounded row
/// at a time; compatibility stores feed their already-materialized snapshot.
pub(crate) struct IncrementalVerifier {
    verifying_key: VerifyingKey,
    previous_hash: ContentHash,
    count: u64,
}

impl IncrementalVerifier {
    pub(crate) fn new(verifying_key: VerifyingKey) -> Self {
        Self {
            verifying_key,
            previous_hash: ContentHash::ZERO,
            count: 0,
        }
    }

    pub(crate) fn count(&self) -> u64 {
        self.count
    }

    pub(crate) fn verify_record(
        &mut self,
        physical_position: u64,
        record: &[u8],
    ) -> Result<SignedEvent, LogError> {
        if physical_position != self.count {
            return Err(LogError::ChainBroken {
                seq: physical_position,
                detail: format!("physical position mismatch: expected {}", self.count),
            });
        }

        let event: SignedEvent = serde_json::from_slice(record)?;
        if event.core.seq != physical_position {
            return Err(LogError::ChainBroken {
                seq: event.core.seq,
                detail: format!("out-of-order: expected seq {physical_position}"),
            });
        }
        if event.core.prev_hash != self.previous_hash {
            return Err(LogError::ChainBroken {
                seq: event.core.seq,
                detail: "prev_hash does not match the previous event".into(),
            });
        }
        verify_event_integrity(&self.verifying_key, &event)?;

        self.previous_hash = event.hash;
        self.count = self
            .count
            .checked_add(1)
            .ok_or(LogError::SequenceExhausted)?;
        Ok(event)
    }

    pub(crate) fn finish(&self, total_record_bytes: u64) -> VerifiedSummary {
        VerifiedSummary {
            count: self.count,
            tip: self.previous_hash,
            total_record_bytes,
        }
    }
}

pub(crate) fn verify_event_integrity(
    expected_verifying_key: &VerifyingKey,
    event: &SignedEvent,
) -> Result<(), LogError> {
    if event.core.hash() != event.hash {
        return Err(LogError::ChainBroken {
            seq: event.core.seq,
            detail: "content hash mismatch (event was altered)".into(),
        });
    }
    let signature = Signature::from_bytes(event.signature.as_bytes());
    expected_verifying_key
        .verify(&signed_message(&event.hash), &signature)
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

    match (&event.core.payload, event.core.seq) {
        (
            Payload::Genesis {
                canonicalization_profile,
                verifying_key: declared_verifying_key,
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
            if declared_verifying_key != &hex::encode(expected_verifying_key.as_bytes()) {
                return Err(LogError::ChainBroken {
                    seq: 0,
                    detail: "genesis-declared verifying key does not match the provided key".into(),
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
    Ok(())
}

/// The **write capability**. Possessing a `LogWriter` is the authority to append.
/// In the full system the gate (`deadbolt`) is its only holder. There is no
/// `update` or `delete` — corrections are new, superseding events. Public
/// construction and append methods exist only for crate-supported backends.
/// [`crate::SqliteL0Store`] adds explicit create, verified-prefix reopen, and
/// checkpoint-qualified reopen operations rather than an ambiguous `open`.
pub struct LogWriter<S> {
    pub(crate) store: S,
    pub(crate) signing_key: SigningKey,
    pub(crate) last_hash: ContentHash,
    pub(crate) next_seq: u64,
    pub(crate) clock: Clock,
}

/// Construct, hash, and sign one event record.
///
/// The clock is lazy on purpose: sequence/genesis and payload validation run
/// before `clock` is invoked, so an event that fails validation never consumes
/// a value from a stateful injected clock. A timestamp is observed only for an
/// event that passed validation and will actually be constructed and offered
/// to the persistence path.
pub(crate) fn build_signed_record(
    signing_key: &SigningKey,
    sequence: u64,
    previous_hash: ContentHash,
    clock: &mut dyn FnMut() -> u64,
    provenance: Provenance,
    payload: Payload,
) -> Result<(SignedEvent, Vec<u8>), LogError> {
    let is_genesis = matches!(payload, Payload::Genesis { .. });
    if is_genesis != (sequence == 0) {
        return Err(LogError::ChainBroken {
            seq: sequence,
            detail: if is_genesis {
                "genesis may only be written at seq 0".into()
            } else {
                "seq 0 must be the genesis event".into()
            },
        });
    }
    payload.validate().map_err(|detail| LogError::ChainBroken {
        seq: sequence,
        detail: detail.into(),
    })?;
    let timestamp_nanos = clock();
    let core = EventCore {
        seq: sequence,
        timestamp_nanos,
        prev_hash: previous_hash,
        provenance,
        payload,
    };
    let hash = core.hash();
    let signature = signing_key.sign(&signed_message(&hash));
    let signed = SignedEvent {
        core,
        hash,
        signature: Sig::new(signature.to_bytes()),
    };
    let record = serde_json::to_vec(&signed)?;
    Ok((signed, record))
}

fn open_writer_with_clock<S: WriterStore>(
    store: S,
    signing_key: SigningKey,
    clock: Clock,
) -> Result<LogWriter<S>, LogError> {
    let vk = signing_key.verifying_key();
    let verified = verify_chain_on(&store, &vk)?;
    let mut writer = LogWriter {
        store,
        signing_key,
        last_hash: verified.tip,
        next_seq: verified.count,
        clock,
    };
    if writer.next_seq == 0 {
        append_to_writer(
            &mut writer,
            Provenance::new("magpie-log", "genesis"),
            Payload::Genesis {
                canonicalization_profile: CANONICALIZATION_PROFILE.to_string(),
                verifying_key: hex::encode(vk.as_bytes()),
            },
        )?;
    }
    Ok(writer)
}

fn append_to_writer<S: WriterStore>(
    writer: &mut LogWriter<S>,
    provenance: Provenance,
    payload: Payload,
) -> Result<SignedEvent, LogError> {
    let next_seq = writer
        .next_seq
        .checked_add(1)
        .ok_or(LogError::SequenceExhausted)?;
    let (signed, record) = build_signed_record(
        &writer.signing_key,
        writer.next_seq,
        writer.last_hash,
        &mut writer.clock,
        provenance,
        payload,
    )?;
    writer.store.append_record(&record)?;

    writer.last_hash = signed.hash;
    writer.next_seq = next_seq;
    Ok(signed)
}

impl LogWriter<FileStore> {
    /// Open for appending, recovering and verifying any existing chain. Opening
    /// an **empty** store writes the genesis event (profile + verifying key)
    /// as seq 0 before returning. Uses the system clock.
    pub fn open(store: FileStore, signing_key: SigningKey) -> Result<Self, LogError> {
        open_writer_with_clock(store, signing_key, Box::new(system_clock))
    }

    /// Open with an injected clock (deterministic tests).
    pub fn open_with_clock(
        store: FileStore,
        signing_key: SigningKey,
        clock: Clock,
    ) -> Result<Self, LogError> {
        open_writer_with_clock(store, signing_key, clock)
    }

    /// Append an event: build the core, hash it (chaining onto the tip), sign the
    /// hash, persist, and advance the tip. This is the only write path in Magpie.
    pub fn append(
        &mut self,
        provenance: Provenance,
        payload: Payload,
    ) -> Result<SignedEvent, LogError> {
        append_to_writer(self, provenance, payload)
    }
}

impl LogWriter<MemStore> {
    /// Open for appending, recovering and verifying any existing chain. Opening
    /// an **empty** store writes the genesis event (profile + verifying key)
    /// as seq 0 before returning. Uses the system clock.
    pub fn open(store: MemStore, signing_key: SigningKey) -> Result<Self, LogError> {
        open_writer_with_clock(store, signing_key, Box::new(system_clock))
    }

    /// Open with an injected clock (deterministic tests).
    pub fn open_with_clock(
        store: MemStore,
        signing_key: SigningKey,
        clock: Clock,
    ) -> Result<Self, LogError> {
        open_writer_with_clock(store, signing_key, clock)
    }

    /// Append an event: build the core, hash it (chaining onto the tip), sign the
    /// hash, persist, and advance the tip. This is the only write path in Magpie.
    pub fn append(
        &mut self,
        provenance: Provenance,
        payload: Payload,
    ) -> Result<SignedEvent, LogError> {
        append_to_writer(self, provenance, payload)
    }
}

impl<S> LogWriter<S> {
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
pub struct LogReader<S> {
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

    /// All stored events, parsed but unverified.
    ///
    /// This parses the snapshot returned by its own [`LogStore::read_records`]
    /// call, but does not verify chain linkage, content hashes, signatures,
    /// genesis key binding, canonicalization-profile binding, or payload
    /// validity. The returned raw [`SignedEvent`] values cannot be used as
    /// [`Projection`] inputs; use [`Self::replay`] for projection construction.
    ///
    /// ```compile_fail,E0308
    /// use magpie_log::{
    ///     LogReader, MemStore, Projection, SigningKey, VerifiedReplayEvent,
    /// };
    ///
    /// struct Counter;
    /// impl Projection for Counter {
    ///     fn apply(&mut self, _event: &VerifiedReplayEvent<'_>) {}
    /// }
    ///
    /// let key = SigningKey::from_bytes(&[7u8; 32]);
    /// let reader = LogReader::open(MemStore::new(), key.verifying_key());
    /// let raw = reader.unverified_events().unwrap();
    /// let mut projection = Counter;
    /// projection.apply(&raw[0]);
    /// ```
    pub fn unverified_events(&self) -> Result<Vec<SignedEvent>, LogError> {
        self.store
            .read_records()?
            .iter()
            .map(|r| serde_json::from_slice::<SignedEvent>(r).map_err(LogError::from))
            .collect()
    }

    /// Validate the entire chain: sequence, prev-links, recomputed hashes, and
    /// every signature. Returns only the number of valid events; any tampering
    /// errors. The count cannot wrap caller-selected events for projection:
    ///
    /// ```compile_fail,E0599
    /// use magpie_log::{
    ///     LogReader, MemStore, SigningKey, VerifiedReplayEvent,
    /// };
    ///
    /// let key = SigningKey::from_bytes(&[7u8; 32]);
    /// let reader = LogReader::open(MemStore::new(), key.verifying_key());
    /// let verified_count = reader.verify_chain().unwrap();
    /// let raw = reader.unverified_events().unwrap();
    /// let _: VerifiedReplayEvent<'_> = verified_count.wrap(&raw[0]);
    /// ```
    pub fn verify_chain(&self) -> Result<u64, LogError> {
        verify_chain_on(&self.store, &self.verifying_key).map(|verified| verified.count)
    }

    /// Completely verify one exact supplied snapshot and evaluate one explicit
    /// checkpoint under one explicit v0 relation.
    ///
    /// This operation reads the store once through [`Self::replay_with_summary`],
    /// verifies that retained snapshot completely, and only then observes the
    /// checkpoint commitment from those same verified replay inputs. A semantic
    /// mismatch returns `NotSatisfied`; parsing, chain, signature, and storage
    /// failures reported by the evaluator return [`LogError`]. Any inability to
    /// complete evaluation produces no returned semantic result.
    ///
    /// The returned result carries the exact v0 profile, externally supplied
    /// verification key bytes, verified snapshot count and terminal commitment,
    /// checkpoint, relation, and outcome. Trust in the key remains external.
    /// Even `Satisfied` establishes no freshness, latest state, currentness,
    /// global completeness, canonicality, non-equivocation, authority, trust,
    /// durability, secure checkpoint persistence, or rollback resistance.
    ///
    /// Relation selection is mandatory; no default or inferred relation exists:
    ///
    /// ```compile_fail,E0061
    /// use magpie_log::{ContentHash, HistoryCheckpointV0, LogReader, MemStore, SigningKey};
    ///
    /// let key = SigningKey::from_bytes(&[7u8; 32]);
    /// let reader = LogReader::open(MemStore::new(), key.verifying_key());
    /// let checkpoint = HistoryCheckpointV0::new(0, ContentHash::ZERO);
    /// let _ = reader.evaluate_history_expectation_v0(checkpoint);
    /// ```
    pub fn evaluate_history_expectation_v0(
        &self,
        checkpoint: HistoryCheckpointV0,
        relation: HistoryExpectationRelationV0,
    ) -> Result<HistoryExpectationEvaluationV0, LogError> {
        evaluate_history_expectation_v0(self, self.verifying_key.to_bytes(), checkpoint, relation)
    }

    /// Fold one verified record snapshot into a projection and return its exact
    /// verified event count and chain tip.
    ///
    /// Replay reads one snapshot, verifies that exact snapshot completely, then
    /// folds those same retained parsed events. If snapshot parsing or
    /// verification fails, no event is applied and no summary is returned.
    /// Trust in the supplied verifying key remains external.
    pub fn replay_with_summary<P: Projection>(
        &self,
        projection: &mut P,
    ) -> Result<VerifiedReplaySummary, LogError> {
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
        let event_count = verified.events.len() as u64;
        debug_assert_eq!(
            verified
                .events
                .last()
                .map(|event| event.hash)
                .unwrap_or(ContentHash::ZERO),
            verified.tip,
        );
        for event in &verified.events {
            let replay_event = VerifiedReplayEvent::from_verified_snapshot(event);
            projection.apply(&replay_event);
        }
        Ok(VerifiedReplaySummary {
            event_count,
            tip: verified.tip,
        })
    }

    /// Fold one verified record snapshot into a projection. Replay reads one
    /// snapshot, verifies that exact snapshot completely, then folds those same
    /// parsed events. If snapshot parsing or verification fails, no event is
    /// applied. Trust in the supplied verifying key remains external.
    ///
    /// This is how every derived view is (re)built: drop the view, `replay`, and
    /// you are back exactly where you were — the log is the source of truth.
    pub fn replay<P: Projection>(&self, projection: &mut P) -> Result<u64, LogError> {
        self.replay_with_summary(projection)
            .map(|summary| summary.event_count())
    }
}

/// A derived view over the log. Implementors fold completely replay-verified
/// inputs into state; that state is, by construction, regenerable from the log
/// alone.
///
/// A raw [`SignedEvent`] cannot cross this public boundary:
///
/// ```compile_fail,E0308
/// use magpie_log::{Projection, SignedEvent};
///
/// fn cannot_apply_raw<P: Projection>(projection: &mut P, raw: &SignedEvent) {
///     projection.apply(raw);
/// }
/// ```
///
/// A correctly signed event returned by [`LogWriter::append`] remains raw and
/// likewise cannot be applied directly:
///
/// ```compile_fail,E0308
/// use magpie_log::{
///     LogWriter, MemStore, Payload, Projection, Provenance, SigningKey,
///     VerifiedReplayEvent,
/// };
///
/// struct Counter;
/// impl Projection for Counter {
///     fn apply(&mut self, _event: &VerifiedReplayEvent<'_>) {}
/// }
///
/// let key = SigningKey::from_bytes(&[7u8; 32]);
/// let mut writer = LogWriter::<MemStore>::open(MemStore::new(), key).unwrap();
/// let raw = writer
///     .append(Provenance::new("test", "raw"), Payload::Note { text: "raw".into() })
///     .unwrap();
/// let mut projection = Counter;
/// projection.apply(&raw);
/// ```
pub trait Projection {
    fn apply(&mut self, event: &VerifiedReplayEvent<'_>);
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;

    use super::*;
    use crate::store::{LogStore, MemStore, WriterStore};

    #[derive(Clone, Default)]
    struct InstrumentedWriterStore {
        inner: MemStore,
        read_count: Rc<Cell<usize>>,
        fail_next_append: Rc<Cell<bool>>,
    }

    impl LogStore for InstrumentedWriterStore {
        fn read_records(&self) -> Result<Vec<Vec<u8>>, LogError> {
            self.read_count.set(self.read_count.get() + 1);
            self.inner.read_records()
        }
    }

    impl WriterStore for InstrumentedWriterStore {
        fn append_record(&mut self, bytes: &[u8]) -> Result<(), LogError> {
            if self.fail_next_append.replace(false) {
                return Err(std::io::Error::other("injected append failure").into());
            }
            <MemStore as WriterStore>::append_record(&mut self.inner, bytes)
        }
    }

    fn test_clock() -> Clock {
        let mut timestamp = 0u64;
        Box::new(move || {
            timestamp += 1;
            timestamp
        })
    }

    fn test_key() -> SigningKey {
        SigningKey::from_bytes(&[73u8; 32])
    }

    fn test_note(text: &str) -> Payload {
        Payload::Note { text: text.into() }
    }

    fn test_provenance() -> Provenance {
        Provenance::new("test", "logimpl.rs")
    }

    #[test]
    fn summary_only_verification_returns_count_and_tip_without_events() {
        let signing_key = test_key();
        let verifying_key = signing_key.verifying_key();
        let store = MemStore::new();
        let expected_tip = {
            let mut timestamp = 0u64;
            let mut writer = LogWriter::<MemStore>::open_with_clock(
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

    #[test]
    fn writer_recovery_verifies_once_and_recovers_exact_count_and_tip() {
        let source = MemStore::new();
        let expected_tip = {
            let mut writer =
                LogWriter::<MemStore>::open_with_clock(source.clone(), test_key(), test_clock())
                    .unwrap();
            writer
                .append(test_provenance(), test_note("first recovery event"))
                .unwrap();
            writer
                .append(test_provenance(), test_note("second recovery event"))
                .unwrap()
                .hash
        };
        let initial_record_count = source.records().len();
        let read_count = Rc::new(Cell::new(0));
        let counting_store = InstrumentedWriterStore {
            inner: source.clone(),
            read_count: Rc::clone(&read_count),
            fail_next_append: Rc::new(Cell::new(false)),
        };

        let mut recovered =
            open_writer_with_clock(counting_store, test_key(), test_clock()).unwrap();

        assert_eq!(read_count.get(), 1, "writer recovery must verify once");
        assert_eq!(recovered.len(), 3);
        assert_eq!(recovered.tip(), expected_tip);
        assert_eq!(
            source.records().len(),
            initial_record_count,
            "reopening a non-empty store must not append another genesis"
        );

        let next = append_to_writer(
            &mut recovered,
            test_provenance(),
            test_note("after recovery"),
        )
        .unwrap();
        assert_eq!(next.core.seq, 3);
        assert_eq!(next.core.prev_hash, expected_tip);
        assert_eq!(read_count.get(), 1, "append must not trigger a second read");
    }

    #[test]
    fn failed_persistence_does_not_advance_writer_state() {
        let store = InstrumentedWriterStore::default();
        let control = store.clone();
        let mut writer = open_writer_with_clock(store, test_key(), test_clock()).unwrap();
        let original_tip = writer.tip();
        let original_len = writer.len();
        let original_records = control.inner.records();

        control.fail_next_append.set(true);
        let error = append_to_writer(
            &mut writer,
            test_provenance(),
            test_note("injected failure"),
        )
        .unwrap_err();

        assert!(matches!(error, LogError::Io(_)));
        assert_eq!(writer.tip(), original_tip);
        assert_eq!(writer.len(), original_len);
        assert_eq!(control.inner.records(), original_records);

        let next = append_to_writer(
            &mut writer,
            test_provenance(),
            test_note("successful retry"),
        )
        .unwrap();
        assert_eq!(next.core.seq, original_len);
        assert_eq!(next.core.prev_hash, original_tip);
        assert_eq!(writer.len(), original_len + 1);
        assert_eq!(writer.tip(), next.hash);
        assert_eq!(control.inner.records().len(), original_records.len() + 1);
    }

    fn invalid_payload() -> Payload {
        Payload::ClaimAssertedV2 {
            claim_id: String::new(),
            statement: "invalid".into(),
            scope_ref: "scope".into(),
            actor_class: "AgentProposer".into(),
            content_hash: String::new(),
            metadata_json: String::new(),
        }
    }

    #[test]
    fn rejected_append_does_not_consume_injected_clock_memstore() {
        let mut baseline =
            open_writer_with_clock(MemStore::new(), test_key(), test_clock()).unwrap();
        let expected =
            append_to_writer(&mut baseline, test_provenance(), test_note("clocked")).unwrap();

        let store = MemStore::new();
        let mut writer = open_writer_with_clock(store.clone(), test_key(), test_clock()).unwrap();
        let rejected =
            append_to_writer(&mut writer, test_provenance(), invalid_payload()).unwrap_err();
        assert!(
            matches!(rejected, LogError::ChainBroken { .. }),
            "invalid payload must be rejected: {rejected:?}"
        );
        assert_eq!(store.records().len(), 1, "rejection persisted nothing");
        assert_eq!(writer.len(), 1);
        assert_eq!(writer.tip(), expected.core.prev_hash);

        let accepted =
            append_to_writer(&mut writer, test_provenance(), test_note("clocked")).unwrap();
        assert_eq!(accepted.core.timestamp_nanos, expected.core.timestamp_nanos);
        assert_eq!(accepted.hash, expected.hash);
    }

    #[test]
    fn rejected_append_does_not_consume_injected_clock_filestore() {
        let unique = format!(
            "magpie-clock-rejection-{}-{}.jsonl",
            std::process::id(),
            system_clock()
        );
        let baseline_path = std::env::temp_dir().join(format!("baseline-{unique}"));
        let rejected_path = std::env::temp_dir().join(format!("rejected-{unique}"));

        let mut baseline =
            open_writer_with_clock(FileStore::new(&baseline_path), test_key(), test_clock())
                .unwrap();
        let expected =
            append_to_writer(&mut baseline, test_provenance(), test_note("clocked")).unwrap();

        let mut writer =
            open_writer_with_clock(FileStore::new(&rejected_path), test_key(), test_clock())
                .unwrap();
        let rejected =
            append_to_writer(&mut writer, test_provenance(), invalid_payload()).unwrap_err();
        assert!(matches!(rejected, LogError::ChainBroken { .. }));
        let persisted = std::fs::read(&rejected_path).unwrap();
        assert_eq!(
            persisted
                .split(|byte| *byte == b'\n')
                .filter(|line| !line.is_empty())
                .count(),
            1,
            "rejection persisted nothing beyond genesis"
        );

        let accepted =
            append_to_writer(&mut writer, test_provenance(), test_note("clocked")).unwrap();
        assert_eq!(accepted.core.timestamp_nanos, expected.core.timestamp_nanos);
        assert_eq!(accepted.hash, expected.hash);

        let _ = std::fs::remove_file(&baseline_path);
        let _ = std::fs::remove_file(&rejected_path);
    }
}
