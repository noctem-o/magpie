use std::cell::Cell;
use std::rc::Rc;

use magpie_claims::{
    replay_origin_admission_context_v0, replay_standing_context,
    ArtifactProvenanceAnchorSelectorV0, DeadboltAnchorIdentity, OriginAdmissionReplayContextV0,
    StandingReplaySnapshot, ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
    ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0, ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0,
    ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
};
use magpie_log::{
    ContentHash, LogError, LogReader, LogStore, LogWriter, MemStore, Payload, Provenance, Sig,
    SignedEvent, SigningKey, Status,
};

const SEED: [u8; 32] = [48; 32];
const CLAIM_ID: &str = "claim-origin-replay";
const EVIDENCE_ID: &str = "evidence-origin-replay";
const EDGE_ID: &str = "edge-origin-replay";
const SCOPE: &str = "scope:origin-replay";

fn signing_key() -> SigningKey {
    SigningKey::from_bytes(&SEED)
}

fn writer(store: MemStore) -> LogWriter<MemStore> {
    let mut timestamp = 0u64;
    LogWriter::open_with_clock(
        store,
        signing_key(),
        Box::new(move || {
            timestamp += 1;
            timestamp
        }),
    )
    .unwrap()
}

fn provenance(source: &str) -> Provenance {
    Provenance::new("origin-admission-replay-test", source)
}

fn selector(kind: &str, root: char, run_id: &str) -> ArtifactProvenanceAnchorSelectorV0 {
    ArtifactProvenanceAnchorSelectorV0::new(
        kind,
        root.to_string().repeat(64),
        ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
        ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
        run_id,
    )
}

fn acquisition_selector() -> ArtifactProvenanceAnchorSelectorV0 {
    selector(
        ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
        'a',
        "run-origin-replay-acquisition",
    )
}

fn derivation_selector() -> ArtifactProvenanceAnchorSelectorV0 {
    selector(
        ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0,
        'b',
        "run-origin-replay-derivation",
    )
}

fn identity(selector: &ArtifactProvenanceAnchorSelectorV0) -> DeadboltAnchorIdentity {
    DeadboltAnchorIdentity {
        bundle_kind: selector.bundle_kind().to_owned(),
        witness_root: selector.witness_root().to_owned(),
        witness_algorithm: selector.witness_algorithm().to_owned(),
        canonicalization_profile: selector.canonicalization_profile().to_owned(),
        run_id: selector.run_id().to_owned(),
    }
}

fn anchor_payload(selector: &ArtifactProvenanceAnchorSelectorV0) -> Payload {
    Payload::SegmentAnchored {
        bundle_kind: selector.bundle_kind().to_owned(),
        witness_root: selector.witness_root().to_owned(),
        witness_algorithm: selector.witness_algorithm().to_owned(),
        canonicalization_profile: selector.canonicalization_profile().to_owned(),
        run_id: selector.run_id().to_owned(),
    }
}

fn append_standing_material(writer: &mut LogWriter<MemStore>) {
    writer
        .append(
            provenance("legacy-control"),
            Payload::ClaimAsserted {
                claim_id: CLAIM_ID.into(),
                statement: "one replay-bound origin claim".into(),
                status: Status::Conjectured,
            },
        )
        .unwrap();
    writer
        .append(
            provenance("typed-claim"),
            Payload::ClaimAssertedV2 {
                claim_id: CLAIM_ID.into(),
                statement: "one replay-bound origin claim".into(),
                scope_ref: SCOPE.into(),
                actor_class: "AgentProposer".into(),
                content_hash: String::new(),
                metadata_json: r#"{"claim_domain":"ExternalReport"}"#.into(),
            },
        )
        .unwrap();
    writer
        .append(
            provenance("typed-evidence"),
            Payload::EvidenceRegistered {
                evidence_id: EVIDENCE_ID.into(),
                evidence_kind: "ExternalSource".into(),
                summary: "one replay-bound source observation".into(),
                scope_ref: SCOPE.into(),
                actor_class: "SourceImporter".into(),
                content_hash: String::new(),
                metadata_json: "{}".into(),
            },
        )
        .unwrap();
    writer
        .append(
            provenance("typed-edge"),
            Payload::JustificationEdgeRecorded {
                edge_id: EDGE_ID.into(),
                edge_kind: "supports".into(),
                source_id: EVIDENCE_ID.into(),
                target_id: CLAIM_ID.into(),
                scope_ref: SCOPE.into(),
                actor_class: "HumanRoot".into(),
                rationale: "candidate relationship only".into(),
                metadata_json: "{}".into(),
            },
        )
        .unwrap();
}

fn populated_log() -> (MemStore, ContentHash) {
    let acquisition = acquisition_selector();
    let derivation = derivation_selector();
    let store = MemStore::new();
    let final_hash = {
        let mut writer = writer(store.clone());
        append_standing_material(&mut writer);
        writer
            .append(
                provenance("acquisition-anchor"),
                anchor_payload(&acquisition),
            )
            .unwrap();
        writer
            .append(provenance("derivation-anchor"), anchor_payload(&derivation))
            .unwrap();
        writer
            .append(
                provenance("unrelated-note"),
                Payload::Note {
                    text: "unrelated replay material".into(),
                },
            )
            .unwrap()
            .hash
    };
    (store, final_hash)
}

#[test]
fn exact_same_replay_constructs_standing_anchors_count_and_tip() {
    let (store, final_hash) = populated_log();
    let reader = LogReader::open(store, signing_key().verifying_key());

    let context = replay_origin_admission_context_v0(&reader).unwrap();

    assert_eq!(context.snapshot().event_count(), 8);
    assert_eq!(
        context.snapshot().event_count(),
        context.verified_prefix_identity().event_count()
    );
    assert_eq!(
        context.verified_prefix_identity().tip_sha256(),
        final_hash.to_hex()
    );
    assert_eq!(context.verified_prefix_identity().tip_sha256().len(), 64);
    assert!(context
        .verified_prefix_identity()
        .tip_sha256()
        .bytes()
        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)));

    assert_eq!(context.snapshot().standing().typed_claim_len(), 1);
    assert_eq!(context.snapshot().standing().typed_evidence_len(), 1);
    assert_eq!(context.snapshot().standing().justification_edge_len(), 1);
    assert!(context
        .snapshot()
        .standing()
        .typed_claim(CLAIM_ID)
        .is_some());
    assert!(context
        .snapshot()
        .standing()
        .typed_evidence(EVIDENCE_ID)
        .is_some());
    assert!(context
        .snapshot()
        .standing()
        .justification_edge(EDGE_ID)
        .is_some());
    assert_eq!(context.snapshot().anchors().len(), 2);
    assert_eq!(
        context
            .snapshot()
            .anchors()
            .occurrences(&identity(&acquisition_selector()))
            .len(),
        1
    );
    assert_eq!(
        context
            .snapshot()
            .anchors()
            .occurrences(&identity(&derivation_selector()))
            .len(),
        1
    );
}

#[test]
fn existing_snapshot_path_is_equal_and_byte_identical() {
    let (store, _final_hash) = populated_log();
    let existing_reader = LogReader::open(store.clone(), signing_key().verifying_key());
    let origin_reader = LogReader::open(store, signing_key().verifying_key());

    let existing = replay_standing_context(&existing_reader).unwrap();
    let origin = replay_origin_admission_context_v0(&origin_reader).unwrap();

    assert_eq!(&existing, origin.snapshot());
    assert_eq!(
        existing.canonical_bytes(),
        origin.snapshot().canonical_bytes()
    );
}

fn note_only_log(text: &str) -> MemStore {
    let store = MemStore::new();
    {
        let mut writer = writer(store.clone());
        writer
            .append(
                provenance("ignored-note"),
                Payload::Note { text: text.into() },
            )
            .unwrap();
    }
    store
}

#[test]
fn equal_projections_from_different_histories_have_different_prefix_identities() {
    let reader_a = LogReader::open(note_only_log("history a"), signing_key().verifying_key());
    let reader_b = LogReader::open(note_only_log("history b"), signing_key().verifying_key());

    let context_a = replay_origin_admission_context_v0(&reader_a).unwrap();
    let context_b = replay_origin_admission_context_v0(&reader_b).unwrap();

    assert_eq!(context_a.snapshot().event_count(), 2);
    assert_eq!(context_a.snapshot(), context_b.snapshot());
    assert_eq!(
        context_a.snapshot().canonical_bytes(),
        context_b.snapshot().canonical_bytes()
    );
    assert_ne!(
        context_a.verified_prefix_identity(),
        context_b.verified_prefix_identity()
    );
}

struct OneReadStore {
    snapshot: Vec<Vec<u8>>,
    read_count: Rc<Cell<usize>>,
}

impl LogStore for OneReadStore {
    fn append_record(&mut self, _bytes: &[u8]) -> Result<(), LogError> {
        panic!("OneReadStore is read-only")
    }

    fn read_records(&self) -> Result<Vec<Vec<u8>>, LogError> {
        let prior = self.read_count.get();
        self.read_count.set(prior + 1);
        assert_eq!(prior, 0, "origin replay attempted a second store read");
        Ok(self.snapshot.clone())
    }
}

#[test]
fn origin_admission_context_reads_the_store_exactly_once() {
    let (store, final_hash) = populated_log();
    let read_count = Rc::new(Cell::new(0));
    let reader = LogReader::open(
        OneReadStore {
            snapshot: store.records(),
            read_count: Rc::clone(&read_count),
        },
        signing_key().verifying_key(),
    );

    let context = replay_origin_admission_context_v0(&reader).unwrap();

    assert_eq!(read_count.get(), 1);
    assert_eq!(
        context.verified_prefix_identity().tip_sha256(),
        final_hash.to_hex()
    );
}

#[test]
fn late_invalid_record_yields_no_partially_trusted_context() {
    let (store, _final_hash) = populated_log();
    let mut records = store.records();
    let mut invalid: SignedEvent = serde_json::from_slice(records.last().unwrap()).unwrap();
    invalid.signature = Sig::new([0u8; 64]);
    *records.last_mut().unwrap() = serde_json::to_vec(&invalid).unwrap();
    let reader = LogReader::open(
        MemStore::from_records(records),
        signing_key().verifying_key(),
    );

    let result: Result<OriginAdmissionReplayContextV0, LogError> =
        replay_origin_admission_context_v0(&reader);

    assert!(matches!(result, Err(LogError::BadSignature { seq: 7 })));
}

fn standing_bytes(snapshot: &StandingReplaySnapshot) -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
    (
        snapshot.standing().canonical_bytes(),
        snapshot
            .standing()
            .resolved_standing_with_trace(CLAIM_ID)
            .canonical_bytes(),
        snapshot
            .resolved_standing_with_trace_v1(CLAIM_ID)
            .canonical_bytes(),
        snapshot
            .resolved_standing_with_trace_v2(CLAIM_ID)
            .canonical_bytes(),
    )
}

#[test]
fn versioned_prefix_context_is_standing_and_snapshot_byte_inert() {
    let (store, _final_hash) = populated_log();
    let existing_reader = LogReader::open(store.clone(), signing_key().verifying_key());
    let origin_reader = LogReader::open(store, signing_key().verifying_key());

    let before = replay_standing_context(&existing_reader).unwrap();
    let after = replay_origin_admission_context_v0(&origin_reader).unwrap();

    assert_eq!(before.canonical_bytes(), after.snapshot().canonical_bytes());
    assert_eq!(standing_bytes(&before), standing_bytes(after.snapshot()));
}
