//! Hostile proofs for verified history checkpoint evaluation v0.

use std::cell::Cell;
use std::rc::Rc;

use magpie_log::{
    ContentHash, HistoryCheckpointV0, HistoryExpectationEvaluationV0, HistoryExpectationOutcomeV0,
    HistoryExpectationRelationV0, LogError, LogReader, LogStore, LogWriter, MemStore, Payload,
    Provenance, Sig, SignedEvent, SigningKey, HISTORY_EXPECTATION_PROFILE_V0_ID,
};

const SEED: [u8; 32] = [91u8; 32];

fn key() -> SigningKey {
    SigningKey::from_bytes(&SEED)
}

fn test_clock() -> magpie_log::Clock {
    let mut timestamp = 0u64;
    Box::new(move || {
        timestamp += 1;
        timestamp
    })
}

fn build_history(notes: &[&str]) -> MemStore {
    let store = MemStore::new();
    {
        let mut writer =
            LogWriter::<MemStore>::open_with_clock(store.clone(), key(), test_clock()).unwrap();
        for text in notes {
            writer
                .append(
                    Provenance::new("history-expectation-test", "explicit-fixture"),
                    Payload::Note {
                        text: (*text).to_owned(),
                    },
                )
                .unwrap();
        }
    }
    store
}

fn checkpoint_at(store: &MemStore, event_count: u64) -> HistoryCheckpointV0 {
    let commitment = if event_count == 0 {
        ContentHash::ZERO
    } else {
        let record = &store.records()[(event_count - 1) as usize];
        serde_json::from_slice::<SignedEvent>(record).unwrap().hash
    };
    HistoryCheckpointV0::new(event_count, commitment)
}

fn evaluate<S: LogStore>(
    store: S,
    checkpoint: HistoryCheckpointV0,
    relation: HistoryExpectationRelationV0,
) -> Result<HistoryExpectationEvaluationV0, LogError> {
    LogReader::open(store, key().verifying_key())
        .evaluate_history_expectation_v0(checkpoint, relation)
}

fn assert_outcome(
    evaluation: &HistoryExpectationEvaluationV0,
    expected: HistoryExpectationOutcomeV0,
) {
    assert_eq!(evaluation.outcome(), expected);
}

#[test]
fn exact_terminal_match_is_satisfied_and_carries_complete_v0_coordinates() {
    let store = build_history(&["terminal"]);
    let checkpoint = checkpoint_at(&store, 2);

    let evaluation = evaluate(store, checkpoint, HistoryExpectationRelationV0::Exact).unwrap();

    assert_outcome(&evaluation, HistoryExpectationOutcomeV0::Satisfied);
    assert_eq!(evaluation.profile().id(), HISTORY_EXPECTATION_PROFILE_V0_ID);
    assert_eq!(
        evaluation.verification_key_bytes(),
        key().verifying_key().as_bytes()
    );
    assert_eq!(evaluation.verified_history().event_count(), 2);
    assert_eq!(evaluation.verified_history().tip(), checkpoint.commitment());
    assert_eq!(evaluation.checkpoint(), checkpoint);
    assert_eq!(evaluation.relation(), HistoryExpectationRelationV0::Exact);
}

#[test]
fn terminal_checkpoint_reflexively_satisfies_contains_checkpoint() {
    let store = build_history(&["terminal"]);
    let checkpoint = checkpoint_at(&store, 2);

    let evaluation = evaluate(
        store,
        checkpoint,
        HistoryExpectationRelationV0::ContainsCheckpoint,
    )
    .unwrap();

    assert_outcome(&evaluation, HistoryExpectationOutcomeV0::Satisfied);
}

#[test]
fn valid_suffix_makes_exact_false_but_containment_true() {
    let store = build_history(&["checkpoint", "valid suffix"]);
    let checkpoint = checkpoint_at(&store, 2);

    let exact = evaluate(
        store.clone(),
        checkpoint,
        HistoryExpectationRelationV0::Exact,
    )
    .unwrap();
    let contains = evaluate(
        store,
        checkpoint,
        HistoryExpectationRelationV0::ContainsCheckpoint,
    )
    .unwrap();

    assert_outcome(&exact, HistoryExpectationOutcomeV0::NotSatisfied);
    assert_outcome(&contains, HistoryExpectationOutcomeV0::Satisfied);
}

#[test]
fn valid_old_prefix_verifies_but_satisfies_neither_relation() {
    let longer = build_history(&["shared prefix", "later checkpoint"]);
    let old_prefix = build_history(&["shared prefix"]);
    let checkpoint = checkpoint_at(&longer, 3);

    for relation in [
        HistoryExpectationRelationV0::Exact,
        HistoryExpectationRelationV0::ContainsCheckpoint,
    ] {
        let evaluation = evaluate(old_prefix.clone(), checkpoint, relation).unwrap();
        assert_eq!(evaluation.verified_history().event_count(), 2);
        assert_outcome(&evaluation, HistoryExpectationOutcomeV0::NotSatisfied);
    }
}

#[test]
fn same_position_with_wrong_commitment_is_not_satisfied() {
    let supplied = build_history(&["supplied branch"]);
    let other = build_history(&["other branch"]);
    let wrong_checkpoint = checkpoint_at(&other, 2);

    for relation in [
        HistoryExpectationRelationV0::Exact,
        HistoryExpectationRelationV0::ContainsCheckpoint,
    ] {
        let evaluation = evaluate(supplied.clone(), wrong_checkpoint, relation).unwrap();
        assert_outcome(&evaluation, HistoryExpectationOutcomeV0::NotSatisfied);
    }
}

#[test]
fn checkpoint_beyond_supplied_history_is_not_satisfied() {
    let supplied = build_history(&["short"]);
    let longer = build_history(&["short", "beyond"]);
    let checkpoint = checkpoint_at(&longer, 3);

    for relation in [
        HistoryExpectationRelationV0::Exact,
        HistoryExpectationRelationV0::ContainsCheckpoint,
    ] {
        let evaluation = evaluate(supplied.clone(), checkpoint, relation).unwrap();
        assert_outcome(&evaluation, HistoryExpectationOutcomeV0::NotSatisfied);
    }
}

#[test]
fn commitment_from_another_position_cannot_be_substituted() {
    let supplied = build_history(&["first", "second"]);
    let commitment_at_one = checkpoint_at(&supplied, 1).commitment();
    let wrong_position = HistoryCheckpointV0::new(2, commitment_at_one);

    for relation in [
        HistoryExpectationRelationV0::Exact,
        HistoryExpectationRelationV0::ContainsCheckpoint,
    ] {
        let evaluation = evaluate(supplied.clone(), wrong_position, relation).unwrap();
        assert_outcome(&evaluation, HistoryExpectationOutcomeV0::NotSatisfied);
    }
}

#[test]
fn equal_or_greater_fork_length_cannot_masquerade_as_containment() {
    let expected_branch = build_history(&["left fork", "left continuation"]);
    let supplied_fork = build_history(&["right fork", "right continuation"]);
    let checkpoint = checkpoint_at(&expected_branch, 2);

    let evaluation = evaluate(
        supplied_fork,
        checkpoint,
        HistoryExpectationRelationV0::ContainsCheckpoint,
    )
    .unwrap();

    assert_eq!(evaluation.verified_history().event_count(), 3);
    assert_outcome(&evaluation, HistoryExpectationOutcomeV0::NotSatisfied);
}

#[test]
fn sibling_forks_after_checkpoint_may_both_contain_it() {
    let left = build_history(&["shared checkpoint", "left suffix"]);
    let right = build_history(&["shared checkpoint", "right suffix"]);
    let checkpoint = checkpoint_at(&left, 2);
    assert_eq!(checkpoint_at(&right, 2), checkpoint);

    for branch in [left, right] {
        let evaluation = evaluate(
            branch,
            checkpoint,
            HistoryExpectationRelationV0::ContainsCheckpoint,
        )
        .unwrap();
        assert_outcome(&evaluation, HistoryExpectationOutcomeV0::Satisfied);
    }
}

#[test]
fn corrupt_suffix_after_apparent_checkpoint_produces_no_semantic_outcome() {
    let store = build_history(&["checkpoint", "corrupt suffix"]);
    let checkpoint = checkpoint_at(&store, 2);
    let mut records = store.records();
    let mut corrupt: SignedEvent = serde_json::from_slice(records.last().unwrap()).unwrap();
    corrupt.signature = Sig::new([0u8; 64]);
    *records.last_mut().unwrap() = serde_json::to_vec(&corrupt).unwrap();

    let result = evaluate(
        MemStore::from_records(records),
        checkpoint,
        HistoryExpectationRelationV0::ContainsCheckpoint,
    );

    assert!(matches!(result, Err(LogError::BadSignature { seq: 2 })));
}

#[test]
fn arbitrary_caller_checkpoint_is_expectation_material_not_a_credential() {
    let store = build_history(&["caller selected"]);
    let actual = checkpoint_at(&store, 2);
    let caller_checkpoint = HistoryCheckpointV0::new(2, actual.commitment());

    assert_eq!(caller_checkpoint.event_count(), 2);
    assert_eq!(caller_checkpoint.commitment(), actual.commitment());
    let evaluation = evaluate(
        store,
        caller_checkpoint,
        HistoryExpectationRelationV0::Exact,
    )
    .unwrap();
    assert_outcome(&evaluation, HistoryExpectationOutcomeV0::Satisfied);
}

struct ChangingSnapshotStore {
    first_snapshot: Vec<Vec<u8>>,
    second_snapshot: Vec<Vec<u8>>,
    read_count: Rc<Cell<usize>>,
}

impl LogStore for ChangingSnapshotStore {
    fn read_records(&self) -> Result<Vec<Vec<u8>>, LogError> {
        let read_count = self.read_count.get();
        self.read_count.set(read_count + 1);
        if read_count == 0 {
            Ok(self.first_snapshot.clone())
        } else {
            Ok(self.second_snapshot.clone())
        }
    }
}

#[test]
fn expectation_evaluation_reads_the_store_exactly_once() {
    let first = build_history(&["checkpoint", "first suffix"]);
    let second = build_history(&["substituted fork", "second suffix"]);
    let checkpoint = checkpoint_at(&first, 2);
    let read_count = Rc::new(Cell::new(0));
    let store = ChangingSnapshotStore {
        first_snapshot: first.records(),
        second_snapshot: second.records(),
        read_count: Rc::clone(&read_count),
    };

    let evaluation = evaluate(
        store,
        checkpoint,
        HistoryExpectationRelationV0::ContainsCheckpoint,
    )
    .unwrap();

    assert_outcome(&evaluation, HistoryExpectationOutcomeV0::Satisfied);
    assert_eq!(read_count.get(), 1, "evaluation must observe one snapshot");
}

#[test]
fn v0_empty_position_has_explicit_zero_commitment_semantics() {
    let empty_checkpoint = HistoryCheckpointV0::new(0, ContentHash::ZERO);

    for relation in [
        HistoryExpectationRelationV0::Exact,
        HistoryExpectationRelationV0::ContainsCheckpoint,
    ] {
        let evaluation = evaluate(MemStore::new(), empty_checkpoint, relation).unwrap();
        assert_eq!(evaluation.verified_history().event_count(), 0);
        assert_eq!(evaluation.verified_history().tip(), ContentHash::ZERO);
        assert_outcome(&evaluation, HistoryExpectationOutcomeV0::Satisfied);
    }

    let non_empty = build_history(&["suffix after empty position"]);
    let exact = evaluate(
        non_empty.clone(),
        empty_checkpoint,
        HistoryExpectationRelationV0::Exact,
    )
    .unwrap();
    let contains = evaluate(
        non_empty,
        empty_checkpoint,
        HistoryExpectationRelationV0::ContainsCheckpoint,
    )
    .unwrap();
    assert_outcome(&exact, HistoryExpectationOutcomeV0::NotSatisfied);
    assert_outcome(&contains, HistoryExpectationOutcomeV0::Satisfied);
}

struct FailingStore;

impl LogStore for FailingStore {
    fn read_records(&self) -> Result<Vec<Vec<u8>>, LogError> {
        Err(std::io::Error::other("injected store failure").into())
    }
}

#[test]
fn operational_store_failure_produces_no_semantic_outcome() {
    let result = evaluate(
        FailingStore,
        HistoryCheckpointV0::new(0, ContentHash::ZERO),
        HistoryExpectationRelationV0::Exact,
    );

    assert!(matches!(result, Err(LogError::Io(_))));
}

#[test]
fn malformed_signature_and_chain_failures_are_not_not_satisfied() {
    let valid = build_history(&["checkpoint", "suffix"]);
    let checkpoint = checkpoint_at(&valid, 2);

    let mut malformed = valid.records();
    *malformed.last_mut().unwrap() = b"{".to_vec();
    assert!(matches!(
        evaluate(
            MemStore::from_records(malformed),
            checkpoint,
            HistoryExpectationRelationV0::ContainsCheckpoint,
        ),
        Err(LogError::Serde(_))
    ));

    let mut bad_signature = valid.records();
    let mut event: SignedEvent = serde_json::from_slice(bad_signature.last().unwrap()).unwrap();
    event.signature = Sig::new([0u8; 64]);
    *bad_signature.last_mut().unwrap() = serde_json::to_vec(&event).unwrap();
    assert!(matches!(
        evaluate(
            MemStore::from_records(bad_signature),
            checkpoint,
            HistoryExpectationRelationV0::ContainsCheckpoint,
        ),
        Err(LogError::BadSignature { seq: 2 })
    ));

    let mut broken_chain = valid.records();
    let mut event: SignedEvent = serde_json::from_slice(broken_chain.last().unwrap()).unwrap();
    event.core.prev_hash = ContentHash::ZERO;
    *broken_chain.last_mut().unwrap() = serde_json::to_vec(&event).unwrap();
    assert!(matches!(
        evaluate(
            MemStore::from_records(broken_chain),
            checkpoint,
            HistoryExpectationRelationV0::ContainsCheckpoint,
        ),
        Err(LogError::ChainBroken { seq: 2, detail })
            if detail == "prev_hash does not match the previous event"
    ));
}
