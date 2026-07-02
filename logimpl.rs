use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

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

/// Walk the whole stored chain, validating it, and return `(count, tip_hash)`.
/// Shared by [`LogReader::verify_chain`] and [`LogWriter`]'s recovery on open.
fn verify_chain_on<S: LogStore>(
    store: &S,
    vk: &VerifyingKey,
) -> Result<(u64, ContentHash), LogError> {
    let records = store.read_records()?;
    let mut prev = ContentHash::ZERO;
    let mut expected_seq = 0u64;

    for record in &records {
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
        vk.verify(event.hash.as_bytes(), &sig)
            .map_err(|_| LogError::BadSignature { seq: event.core.seq })?;

        prev = event.hash;
        expected_seq += 1;
    }

    Ok((expected_seq, prev))
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
    /// Open for appending, recovering and verifying any existing chain. Uses the
    /// system clock.
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
        let (count, tip) = verify_chain_on(&store, &vk)?;
        Ok(Self { store, signing_key, last_hash: tip, next_seq: count, clock })
    }

    /// Append an event: build the core, hash it (chaining onto the tip), sign the
    /// hash, persist, and advance the tip. This is the only write path in Magpie.
    pub fn append(
        &mut self,
        provenance: Provenance,
        payload: Payload,
    ) -> Result<SignedEvent, LogError> {
        let core = EventCore {
            seq: self.next_seq,
            timestamp_nanos: (self.clock)(),
            prev_hash: self.last_hash,
            provenance,
            payload,
        };
        let hash = core.hash();
        let signature = self.signing_key.sign(hash.as_bytes());
        let signed = SignedEvent { core, hash, signature: Sig::new(signature.to_bytes()) };

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
        Self { store, verifying_key }
    }

    /// All stored events, parsed but not verified. Prefer [`Self::verify_chain`]
    /// or [`Self::replay`] for anything you intend to trust.
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
        verify_chain_on(&self.store, &self.verifying_key).map(|(count, _)| count)
    }

    /// Fold the (verified) log into a projection. This is how every derived view
    /// is (re)built: drop the view, `replay`, and you are back exactly where you
    /// were — the log is the source of truth.
    pub fn replay<P: Projection>(&self, projection: &mut P) -> Result<u64, LogError> {
        self.verify_chain()?; // never fold an unverified log
        let events = self.events()?;
        for event in &events {
            projection.apply(event);
        }
        Ok(events.len() as u64)
    }
}

/// A derived view over the log. Implementors fold events into state; that state
/// is, by construction, regenerable from the log alone.
pub trait Projection {
    fn apply(&mut self, event: &SignedEvent);
}
