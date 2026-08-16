use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use magpie_log::{
    ContentHash, FileStore, HistoryCheckpointV0, HistoryExpectationOutcomeV0,
    HistoryExpectationRelationV0, L0ResourceLimitsV0, LogError, LogReader, LogWriter, Payload,
    Provenance, Sig, SignedEvent, SigningKey, SqliteCheckpointOpenError, SqliteL0Store,
};
use rusqlite::{params, Connection, OpenFlags};

static NEXT_PATH: AtomicU64 = AtomicU64::new(0);

struct TestPath(PathBuf);

impl TestPath {
    fn new(label: &str) -> Self {
        let serial = NEXT_PATH.fetch_add(1, Ordering::Relaxed);
        Self(std::env::temp_dir().join(format!(
            "magpie-sqlite-l0-{label}-{}-{serial}.db",
            std::process::id()
        )))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TestPath {
    fn drop(&mut self) {
        for suffix in ["", "-journal", "-wal", "-shm"] {
            let path = PathBuf::from(format!("{}{}", self.0.display(), suffix));
            let _ = fs::remove_file(path);
        }
    }
}

fn key() -> SigningKey {
    SigningKey::from_bytes(&[42u8; 32])
}

fn other_key() -> SigningKey {
    SigningKey::from_bytes(&[43u8; 32])
}

fn broad_limits() -> L0ResourceLimitsV0 {
    L0ResourceLimitsV0::new(32 * 1024 * 1024, 10_000, 2 * 1024 * 1024, 24 * 1024 * 1024).unwrap()
}

fn fixed_clock() -> magpie_log::Clock {
    let mut timestamp = 1000u64;
    Box::new(move || {
        timestamp += 1;
        timestamp
    })
}

fn note(text: impl Into<String>) -> Payload {
    Payload::Note { text: text.into() }
}

fn provenance(label: &str) -> Provenance {
    Provenance::new("sqlite-l0-test", label)
}

fn raw_connection(path: &Path) -> Connection {
    Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_NO_MUTEX
            | OpenFlags::SQLITE_OPEN_PRIVATE_CACHE,
    )
    .unwrap()
}

fn logical_bytes(path: &Path) -> u64 {
    let connection = raw_connection(path);
    let page_count: i64 = connection
        .query_row("PRAGMA page_count", [], |row| row.get(0))
        .unwrap();
    let page_size: i64 = connection
        .query_row("PRAGMA page_size", [], |row| row.get(0))
        .unwrap();
    u64::try_from(page_count).unwrap() * u64::try_from(page_size).unwrap()
}

fn record_lengths(path: &Path) -> Vec<u64> {
    let connection = raw_connection(path);
    let mut statement = connection
        .prepare("SELECT length(record_bytes) FROM magpie_l0_records ORDER BY position_be")
        .unwrap();
    statement
        .query_map([], |row| row.get::<_, i64>(0))
        .unwrap()
        .map(|length| u64::try_from(length.unwrap()).unwrap())
        .collect()
}

fn records(path: &Path) -> Vec<Vec<u8>> {
    let connection = raw_connection(path);
    let mut statement = connection
        .prepare("SELECT record_bytes FROM magpie_l0_records ORDER BY position_be")
        .unwrap();
    statement
        .query_map([], |row| row.get::<_, Vec<u8>>(0))
        .unwrap()
        .map(Result::unwrap)
        .collect()
}

fn create(path: &Path) -> LogWriter<SqliteL0Store> {
    LogWriter::<SqliteL0Store>::create_new_with_clock(path, key(), broad_limits(), fixed_clock())
        .unwrap()
}

#[test]
fn p1_creation_is_atomic_bounded_and_initializes_exact_writer_coordinates() {
    for values in [(0, 1, 1, 1), (1, 0, 1, 1), (1, 1, 0, 1), (1, 1, 1, 0)] {
        assert!(L0ResourceLimitsV0::new(values.0, values.1, values.2, values.3).is_err());
    }

    let reference = TestPath::new("p1-reference");
    let writer = create(reference.path());
    let genesis_length = writer.total_record_bytes();
    let initial_logical_bytes = logical_bytes(reference.path());
    assert_eq!(writer.len(), 1);
    assert_ne!(writer.tip(), ContentHash::ZERO);
    assert_eq!(record_lengths(reference.path()), vec![genesis_length]);
    assert_eq!(writer.resource_limits(), broad_limits());
    drop(writer);

    let count_path = TestPath::new("p1-count");
    let count_limits = L0ResourceLimitsV0::new(
        broad_limits().max_database_bytes(),
        1,
        broad_limits().max_record_bytes(),
        broad_limits().max_total_record_bytes(),
    )
    .unwrap();
    let exact = LogWriter::<SqliteL0Store>::create_new_with_clock(
        count_path.path(),
        key(),
        count_limits,
        fixed_clock(),
    )
    .unwrap();
    assert_eq!(exact.len(), 1);
    drop(exact);

    let unowned = TestPath::new("p1-empty-unowned-sqlite");
    {
        let connection = Connection::open(unowned.path()).unwrap();
        connection.execute_batch("VACUUM").unwrap();
    }
    assert!(fs::metadata(unowned.path()).unwrap().len() >= 100);
    let initialized = LogWriter::<SqliteL0Store>::create_new_with_clock(
        unowned.path(),
        key(),
        broad_limits(),
        fixed_clock(),
    )
    .unwrap();
    assert_eq!(initialized.len(), 1);
    drop(initialized);

    let cases = [
        (
            "record-count",
            L0ResourceLimitsV0::new(
                broad_limits().max_database_bytes(),
                1,
                broad_limits().max_record_bytes(),
                broad_limits().max_total_record_bytes(),
            )
            .unwrap(),
            false,
        ),
        (
            "record-bytes",
            L0ResourceLimitsV0::new(
                broad_limits().max_database_bytes(),
                1,
                genesis_length - 1,
                broad_limits().max_total_record_bytes(),
            )
            .unwrap(),
            true,
        ),
        (
            "total-bytes",
            L0ResourceLimitsV0::new(
                broad_limits().max_database_bytes(),
                1,
                broad_limits().max_record_bytes(),
                genesis_length - 1,
            )
            .unwrap(),
            true,
        ),
        (
            "database-bytes",
            L0ResourceLimitsV0::new(
                initial_logical_bytes - 1,
                1,
                broad_limits().max_record_bytes(),
                broad_limits().max_total_record_bytes(),
            )
            .unwrap(),
            true,
        ),
    ];
    for (label, limits, must_fail) in cases {
        if !must_fail {
            continue;
        }
        let path = TestPath::new(label);
        let result = LogWriter::<SqliteL0Store>::create_new_with_clock(
            path.path(),
            key(),
            limits,
            fixed_clock(),
        );
        assert!(matches!(result, Err(LogError::ResourceLimit { .. })));
        if path.path().exists() && fs::metadata(path.path()).unwrap().len() >= 100 {
            let connection = raw_connection(path.path());
            let application_id: i64 = connection
                .query_row("PRAGMA application_id", [], |row| row.get(0))
                .unwrap();
            let user_version: i64 = connection
                .query_row("PRAGMA user_version", [], |row| row.get(0))
                .unwrap();
            let tables: i64 = connection
                .query_row(
                    "SELECT count(*) FROM sqlite_schema WHERE name = 'magpie_l0_records'",
                    [],
                    |row| row.get(0),
                )
                .unwrap();
            assert_eq!((application_id, user_version, tables), (0, 0, 0));
        }
    }
}

#[test]
fn p19_creation_rejects_oversized_journal_before_any_sqlite_open() {
    let limits = L0ResourceLimitsV0::new(16 * 1024, 10, 2 * 1024 * 1024, 4 * 1024 * 1024).unwrap();
    for (label, initialize) in [
        ("absent", false),
        ("zero-byte", true),
        ("empty-unowned", true),
    ] {
        let path = TestPath::new(label);
        if initialize {
            if label == "empty-unowned" {
                let connection = Connection::open(path.path()).unwrap();
                connection.execute_batch("VACUUM").unwrap();
            } else {
                fs::write(path.path(), []).unwrap();
            }
        }
        let before = if path.path().exists() {
            Some(fs::read(path.path()).unwrap())
        } else {
            None
        };
        let journal_path = PathBuf::from(format!("{}-journal", path.path().display()));
        fs::write(
            &journal_path,
            vec![0u8; usize::try_from(limits.max_database_bytes() + 1).unwrap()],
        )
        .unwrap();

        assert!(matches!(
            LogWriter::<SqliteL0Store>::create_new_with_clock(
                path.path(),
                key(),
                limits,
                fixed_clock(),
            ),
            Err(LogError::ResourceLimit {
                resource: "physical rollback journal bytes",
                ..
            })
        ));
        match before {
            None => assert!(!path.path().exists()),
            Some(before) => assert_eq!(fs::read(path.path()).unwrap(), before),
        }
    }
}

#[test]
fn p2_p3_p17_ownership_and_schema_refusals_do_not_adopt_or_repair() {
    let owned = TestPath::new("already-owned");
    drop(create(owned.path()));
    let before = fs::read(owned.path()).unwrap();
    assert!(matches!(
        LogWriter::<SqliteL0Store>::create_new(owned.path(), key(), broad_limits()),
        Err(LogError::ForeignDatabase { .. })
    ));
    assert_eq!(fs::read(owned.path()).unwrap(), before);

    let foreign = TestPath::new("foreign");
    {
        let connection = Connection::open(foreign.path()).unwrap();
        connection
            .execute_batch(
                "CREATE TABLE precious(value TEXT); INSERT INTO precious VALUES ('keep');",
            )
            .unwrap();
    }
    let before = fs::read(foreign.path()).unwrap();
    assert!(matches!(
        LogWriter::<SqliteL0Store>::open_verified_prefix(foreign.path(), key(), broad_limits()),
        Err(LogError::ForeignDatabase { .. })
    ));
    assert!(matches!(
        LogWriter::<SqliteL0Store>::create_new(foreign.path(), key(), broad_limits()),
        Err(LogError::ForeignDatabase { .. })
    ));
    assert_eq!(fs::read(foreign.path()).unwrap(), before);

    let nonsqlite = TestPath::new("not-sqlite");
    fs::write(nonsqlite.path(), b"not a sqlite database but important").unwrap();
    let before = fs::read(nonsqlite.path()).unwrap();
    assert!(matches!(
        LogWriter::<SqliteL0Store>::open_verified_prefix(nonsqlite.path(), key(), broad_limits()),
        Err(LogError::ForeignDatabase { .. })
    ));
    assert_eq!(fs::read(nonsqlite.path()).unwrap(), before);

    let future = TestPath::new("future-version");
    drop(create(future.path()));
    {
        let connection = raw_connection(future.path());
        connection.execute_batch("PRAGMA user_version = 2").unwrap();
    }
    let before = fs::read(future.path()).unwrap();
    assert!(matches!(
        LogWriter::<SqliteL0Store>::open_verified_prefix(future.path(), key(), broad_limits()),
        Err(LogError::UnsupportedDatabase { .. })
    ));
    assert_eq!(fs::read(future.path()).unwrap(), before);

    let extra = TestPath::new("extra-schema");
    drop(create(extra.path()));
    {
        let connection = raw_connection(extra.path());
        connection
            .execute_batch("CREATE TABLE projection_cache(x)")
            .unwrap();
    }
    assert!(matches!(
        LogWriter::<SqliteL0Store>::open_verified_prefix(extra.path(), key(), broad_limits()),
        Err(LogError::UnsupportedDatabase { .. })
    ));
}

#[test]
fn p3_exact_schema_refuses_extra_index_trigger_and_view() {
    for (label, sql) in [
        (
            "index",
            "CREATE INDEX extra_i ON magpie_l0_records(record_bytes)",
        ),
        (
            "trigger",
            "CREATE TRIGGER extra_t AFTER INSERT ON magpie_l0_records BEGIN SELECT 1; END",
        ),
        (
            "view",
            "CREATE VIEW extra_v AS SELECT position_be FROM magpie_l0_records",
        ),
    ] {
        let path = TestPath::new(label);
        drop(create(path.path()));
        raw_connection(path.path()).execute_batch(sql).unwrap();
        assert!(matches!(
            LogWriter::<SqliteL0Store>::open_verified_prefix(path.path(), key(), broad_limits()),
            Err(LogError::UnsupportedDatabase { .. })
        ));
    }
}

#[test]
fn p4_p5_p6_p7_checkpoint_open_preserves_verification_and_mismatch_distinctions() {
    let path = TestPath::new("checkpoint-relations");
    let mut writer = create(path.path());
    let genesis = HistoryCheckpointV0::new(writer.len(), writer.tip());
    let first = writer.append(provenance("first"), note("first")).unwrap();
    let first_checkpoint = HistoryCheckpointV0::new(writer.len(), first.hash);
    writer.append(provenance("suffix"), note("suffix")).unwrap();
    let terminal = HistoryCheckpointV0::new(writer.len(), writer.tip());
    drop(writer);

    let weak = LogWriter::<SqliteL0Store>::open_verified_prefix(path.path(), key(), broad_limits())
        .unwrap();
    assert_eq!(weak.len(), 3);
    drop(weak);

    for checkpoint in [genesis, first_checkpoint, terminal] {
        let qualified = LogWriter::<SqliteL0Store>::open_containing_checkpoint_v0(
            path.path(),
            key(),
            broad_limits(),
            checkpoint,
        )
        .unwrap();
        assert_eq!(
            qualified.evaluation().outcome(),
            HistoryExpectationOutcomeV0::Satisfied
        );
        assert_eq!(qualified.evaluation().checkpoint(), checkpoint);
        assert_eq!(
            qualified.evaluation().relation(),
            HistoryExpectationRelationV0::ContainsCheckpoint
        );
        assert_eq!(qualified.writer().len(), 3);
    }

    let beyond = HistoryCheckpointV0::new(4, ContentHash::ZERO);
    let error = LogWriter::<SqliteL0Store>::open_containing_checkpoint_v0(
        path.path(),
        key(),
        broad_limits(),
        beyond,
    )
    .err()
    .unwrap();
    let evaluation = error.not_satisfied_evaluation().unwrap();
    assert_eq!(
        evaluation.outcome(),
        HistoryExpectationOutcomeV0::NotSatisfied
    );
    assert_eq!(evaluation.checkpoint(), beyond);
    assert_eq!(evaluation.verified_history().event_count(), 3);
}

#[test]
fn checkpoint_qualified_opens_retain_distinct_genuine_producing_contexts() {
    let path = TestPath::new("checkpoint-carriage");
    let mut writer = create(path.path());
    writer.append(provenance("C1"), note("C1")).unwrap();
    let checkpoint_one = HistoryCheckpointV0::new(writer.len(), writer.tip());
    writer.append(provenance("C2"), note("C2")).unwrap();
    let checkpoint_two = HistoryCheckpointV0::new(writer.len(), writer.tip());
    writer
        .append(provenance("suffix"), note("shared terminal suffix"))
        .unwrap();
    let terminal = (writer.len(), writer.tip());
    drop(writer);

    let open_one = LogWriter::<SqliteL0Store>::open_containing_checkpoint_v0(
        path.path(),
        key(),
        broad_limits(),
        checkpoint_one,
    )
    .unwrap();
    let open_two = LogWriter::<SqliteL0Store>::open_containing_checkpoint_v0(
        path.path(),
        key(),
        broad_limits(),
        checkpoint_two,
    )
    .unwrap();
    assert_eq!((open_one.writer().len(), open_one.writer().tip()), terminal);
    assert_eq!((open_two.writer().len(), open_two.writer().tip()), terminal);
    assert_eq!(open_one.evaluation().checkpoint(), checkpoint_one);
    assert_eq!(open_two.evaluation().checkpoint(), checkpoint_two);
    assert_ne!(open_one.evaluation(), open_two.evaluation());
}

#[test]
fn checkpoint_qualified_append_preserves_binding_and_writer_state_machine() {
    let path = TestPath::new("checkpoint-wrapper-append");
    let writer = create(path.path());
    let checkpoint = HistoryCheckpointV0::new(writer.len(), writer.tip());
    drop(writer);

    let mut qualified = LogWriter::<SqliteL0Store>::open_containing_checkpoint_v0(
        path.path(),
        key(),
        broad_limits(),
        checkpoint,
    )
    .unwrap();
    let qualified_history = qualified.evaluation().verified_history();
    let appended = qualified
        .append(provenance("qualified"), note("qualified append"))
        .unwrap();
    assert_eq!(qualified.writer().len(), 2);
    assert_eq!(qualified.writer().tip(), appended.hash);
    assert_eq!(qualified.evaluation().checkpoint(), checkpoint);
    assert_eq!(qualified.evaluation().verified_history(), qualified_history);

    let mut competing =
        LogWriter::<SqliteL0Store>::open_verified_prefix(path.path(), key(), broad_limits())
            .unwrap();
    competing
        .append(provenance("competing"), note("advance persisted history"))
        .unwrap();
    assert!(matches!(
        qualified.append(provenance("stale"), note("must not rebase")),
        Err(LogError::WriterStale)
    ));
    assert!(qualified.writer().is_poisoned());
    assert!(matches!(
        qualified.append(provenance("poisoned"), note("must not touch SQLite")),
        Err(LogError::WriterPoisoned)
    ));
    assert_eq!(qualified.evaluation().checkpoint(), checkpoint);
}

#[test]
fn p8_wrong_branch_length_never_substitutes_for_checkpoint_ancestry() {
    let left = TestPath::new("branch-left");
    let right = TestPath::new("branch-right");
    let mut left_writer = create(left.path());
    let mut right_writer = create(right.path());
    left_writer
        .append(provenance("fork"), note("left"))
        .unwrap();
    right_writer
        .append(provenance("fork"), note("right"))
        .unwrap();
    right_writer
        .append(provenance("longer"), note("right suffix"))
        .unwrap();
    let left_checkpoint = HistoryCheckpointV0::new(left_writer.len(), left_writer.tip());
    drop(left_writer);
    drop(right_writer);

    let error = LogWriter::<SqliteL0Store>::open_containing_checkpoint_v0(
        right.path(),
        key(),
        broad_limits(),
        left_checkpoint,
    )
    .err()
    .unwrap();
    assert!(matches!(error, SqliteCheckpointOpenError::NotSatisfied(_)));
}

#[test]
fn p1_missing_persisted_terminal_is_stale_and_cannot_advance_writer() {
    let path = TestPath::new("missing-terminal");
    drop(create(path.path()));
    let mut writer =
        LogWriter::<SqliteL0Store>::open_verified_prefix(path.path(), key(), broad_limits())
            .unwrap();
    let before = (writer.len(), writer.tip(), writer.total_record_bytes());

    raw_connection(path.path())
        .execute("DELETE FROM magpie_l0_records", [])
        .unwrap();

    assert!(matches!(
        writer.append(provenance("missing-terminal"), note("must not acknowledge")),
        Err(LogError::WriterStale)
    ));
    assert!(writer.is_poisoned());
    assert_eq!(
        (writer.len(), writer.tip(), writer.total_record_bytes()),
        before
    );
    assert!(records(path.path()).is_empty());
    assert!(matches!(
        writer.append(provenance("poisoned"), note("must not retry")),
        Err(LogError::WriterPoisoned)
    ));
}

#[test]
fn p19_append_terminal_blob_is_bounded_before_parsing() {
    let path = TestPath::new("oversized-terminal");
    let limits = L0ResourceLimitsV0::new(32 * 1024 * 1024, 10, 1024, 24 * 1024 * 1024).unwrap();
    let writer = LogWriter::<SqliteL0Store>::create_new_with_clock(
        path.path(),
        key(),
        limits,
        fixed_clock(),
    )
    .unwrap();
    assert!(writer.total_record_bytes() < limits.max_record_bytes());
    drop(writer);

    let mut writer =
        LogWriter::<SqliteL0Store>::open_verified_prefix(path.path(), key(), limits).unwrap();
    let before = (writer.len(), writer.tip(), writer.total_record_bytes());
    let oversized = limits.max_record_bytes() + 1;
    raw_connection(path.path())
        .execute(
            "UPDATE magpie_l0_records SET record_bytes = zeroblob(?1) WHERE position_be = ?2",
            params![
                i64::try_from(oversized).unwrap(),
                0u64.to_be_bytes().as_slice()
            ],
        )
        .unwrap();

    assert!(matches!(
        writer.append(provenance("oversized-terminal"), note("must bound first")),
        Err(LogError::ResourceLimit {
            resource: "individual record bytes",
            ..
        })
    ));
    assert!(writer.is_poisoned());
    assert_eq!(
        (writer.len(), writer.tip(), writer.total_record_bytes()),
        before
    );
    assert_eq!(records(path.path()).len(), 1);
    assert_eq!(record_lengths(path.path()), vec![oversized]);
}

#[test]
fn p9_stale_writers_cannot_commit_sibling_successors() {
    let path = TestPath::new("stale-writers");
    drop(create(path.path()));
    let mut writer_a =
        LogWriter::<SqliteL0Store>::open_verified_prefix(path.path(), key(), broad_limits())
            .unwrap();
    let mut writer_b =
        LogWriter::<SqliteL0Store>::open_verified_prefix(path.path(), key(), broad_limits())
            .unwrap();
    let committed = writer_a.append(provenance("A"), note("A wins")).unwrap();
    let stale = writer_b.append(provenance("B"), note("B sibling"));
    assert!(matches!(stale, Err(LogError::WriterStale)));
    assert!(writer_b.is_poisoned());
    assert!(matches!(
        writer_b.append(provenance("B-again"), note("must not run")),
        Err(LogError::WriterPoisoned)
    ));
    drop(writer_a);
    drop(writer_b);
    let persisted = records(path.path());
    assert_eq!(persisted.len(), 2);
    let last: SignedEvent = serde_json::from_slice(&persisted[1]).unwrap();
    assert_eq!(last.hash, committed.hash);
}

#[test]
fn p10_stable_reader_forces_commit_busy_cleanup_without_hidden_retry() {
    let path = TestPath::new("commit-busy");
    drop(create(path.path()));
    let mut writer =
        LogWriter::<SqliteL0Store>::open_verified_prefix(path.path(), key(), broad_limits())
            .unwrap();

    let reader = raw_connection(path.path());
    reader.execute_batch("BEGIN DEFERRED").unwrap();
    let anchored_page_count: i64 = reader
        .query_row("PRAGMA main.page_count", [], |row| row.get(0))
        .unwrap();
    assert!(anchored_page_count > 0);

    let started = Instant::now();
    let result = writer.append(provenance("busy"), note("must roll back"));
    assert!(started.elapsed() < Duration::from_secs(2));
    assert!(matches!(
        result,
        Err(LogError::StorageBusy {
            operation: "commit append transaction"
        })
    ));
    assert!(writer.is_poisoned());
    assert_eq!(
        reader
            .query_row("SELECT count(*) FROM magpie_l0_records", [], |row| row
                .get::<_, i64>(0))
            .unwrap(),
        1
    );
    reader.execute_batch("COMMIT").unwrap();

    let lock_probe = raw_connection(path.path());
    lock_probe
        .execute_batch("BEGIN IMMEDIATE; ROLLBACK")
        .unwrap();
    drop(lock_probe);
    assert!(matches!(
        writer.append(provenance("poisoned"), note("no retry")),
        Err(LogError::WriterPoisoned)
    ));
    drop(writer);

    let mut reopened =
        LogWriter::<SqliteL0Store>::open_verified_prefix(path.path(), key(), broad_limits())
            .unwrap();
    assert_eq!(reopened.len(), 1);
    reopened
        .append(provenance("deliberate-reopen"), note("now commit"))
        .unwrap();
}

#[test]
fn begin_immediate_busy_is_not_retried_and_poisons_without_committing() {
    let path = TestPath::new("begin-busy");
    drop(create(path.path()));
    let mut writer =
        LogWriter::<SqliteL0Store>::open_verified_prefix(path.path(), key(), broad_limits())
            .unwrap();
    let blocker = raw_connection(path.path());
    blocker.execute_batch("BEGIN IMMEDIATE").unwrap();

    let started = Instant::now();
    let result = writer.append(provenance("begin-busy"), note("must not insert"));
    assert!(started.elapsed() < Duration::from_secs(2));
    assert!(matches!(
        result,
        Err(LogError::StorageBusy {
            operation: "begin append transaction"
        })
    ));
    assert!(writer.is_poisoned());
    blocker.execute_batch("ROLLBACK").unwrap();
    assert_eq!(records(path.path()).len(), 1);
    assert!(matches!(
        writer.append(provenance("poisoned"), note("must not run SQL")),
        Err(LogError::WriterPoisoned)
    ));
}

#[test]
fn p11_post_insert_resource_failure_rolls_back_and_poisons() {
    let reference = TestPath::new("post-insert-reference");
    drop(create(reference.path()));
    let initial_logical_bytes = logical_bytes(reference.path());

    let path = TestPath::new("post-insert-rollback");
    let limits =
        L0ResourceLimitsV0::new(initial_logical_bytes, 100, 2 * 1024 * 1024, 8 * 1024 * 1024)
            .unwrap();
    let mut writer = LogWriter::<SqliteL0Store>::create_new_with_clock(
        path.path(),
        key(),
        limits,
        fixed_clock(),
    )
    .unwrap();
    let result = writer.append(provenance("page-growth"), note("x".repeat(20_000)));
    assert!(matches!(result, Err(LogError::ResourceLimit { .. })));
    assert!(writer.is_poisoned());
    assert_eq!(records(path.path()).len(), 1);
    assert!(matches!(
        writer.append(provenance("after-poison"), note("must not run")),
        Err(LogError::WriterPoisoned)
    ));
}

#[test]
fn p12_lost_process_acknowledgement_requires_reopen_instead_of_blind_retry() {
    const CHILD_ENV: &str = "MAGPIE_SQLITE_L0_P12_CHILD";
    const PATH_ENV: &str = "MAGPIE_SQLITE_L0_P12_PATH";
    if std::env::var_os(CHILD_ENV).is_some() {
        let path = PathBuf::from(std::env::var_os(PATH_ENV).unwrap());
        let mut writer =
            LogWriter::<SqliteL0Store>::open_verified_prefix(&path, key(), broad_limits()).unwrap();
        writer
            .append(
                provenance("child"),
                note("durable before parent acknowledgement"),
            )
            .unwrap();
        std::process::exit(86);
    }

    let path = TestPath::new("lost-ack");
    drop(create(path.path()));
    let status = Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("p12_lost_process_acknowledgement_requires_reopen_instead_of_blind_retry")
        .arg("--nocapture")
        .env(CHILD_ENV, "1")
        .env(PATH_ENV, path.path())
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(86));

    let reopened =
        LogWriter::<SqliteL0Store>::open_verified_prefix(path.path(), key(), broad_limits())
            .unwrap();
    assert_eq!(reopened.len(), 2);
}

#[test]
fn p14_corruption_never_truncates_to_an_earlier_valid_prefix() {
    let malformed = TestPath::new("malformed-record");
    drop(create(malformed.path()));
    raw_connection(malformed.path())
        .execute(
            "UPDATE magpie_l0_records SET record_bytes = ?1 WHERE position_be = ?2",
            params![b"not-json".as_slice(), 0u64.to_be_bytes().as_slice()],
        )
        .unwrap();
    assert!(matches!(
        LogWriter::<SqliteL0Store>::open_verified_prefix(malformed.path(), key(), broad_limits()),
        Err(LogError::Serde(_))
    ));

    let late = TestPath::new("late-signature");
    let mut writer = create(late.path());
    writer
        .append(provenance("valid"), note("valid prefix"))
        .unwrap();
    writer
        .append(provenance("late"), note("corrupt me"))
        .unwrap();
    drop(writer);
    let mut all_records = records(late.path());
    let mut last: SignedEvent = serde_json::from_slice(all_records.last().unwrap()).unwrap();
    last.signature = Sig::new([0u8; 64]);
    let tampered = serde_json::to_vec(&last).unwrap();
    raw_connection(late.path())
        .execute(
            "UPDATE magpie_l0_records SET record_bytes = ?1 WHERE position_be = ?2",
            params![tampered, 2u64.to_be_bytes().as_slice()],
        )
        .unwrap();
    all_records.clear();
    assert!(matches!(
        LogWriter::<SqliteL0Store>::open_verified_prefix(late.path(), key(), broad_limits()),
        Err(LogError::BadSignature { seq: 2 })
    ));

    let wrong_position = TestPath::new("wrong-position");
    drop(create(wrong_position.path()));
    raw_connection(wrong_position.path())
        .execute(
            "UPDATE magpie_l0_records SET position_be = ?1 WHERE position_be = ?2",
            params![1u64.to_be_bytes().as_slice(), 0u64.to_be_bytes().as_slice()],
        )
        .unwrap();
    assert!(matches!(
        LogWriter::<SqliteL0Store>::open_verified_prefix(
            wrong_position.path(),
            key(),
            broad_limits()
        ),
        Err(LogError::ChainBroken { .. })
    ));
}

#[test]
fn p15_p16_checkpoint_comparison_detects_only_the_explicit_retained_coordinate() {
    let source = TestPath::new("rollback-source");
    let old = TestPath::new("rollback-old");
    let mut writer = create(source.path());
    writer
        .append(provenance("old"), note("old retained point"))
        .unwrap();
    let old_checkpoint = HistoryCheckpointV0::new(writer.len(), writer.tip());
    drop(writer);
    fs::copy(source.path(), old.path()).unwrap();

    let mut writer =
        LogWriter::<SqliteL0Store>::open_verified_prefix(source.path(), key(), broad_limits())
            .unwrap();
    writer
        .append(provenance("new"), note("newer retained point"))
        .unwrap();
    let new_checkpoint = HistoryCheckpointV0::new(writer.len(), writer.tip());
    drop(writer);

    let rolled_back_together = LogWriter::<SqliteL0Store>::open_containing_checkpoint_v0(
        old.path(),
        key(),
        broad_limits(),
        old_checkpoint,
    )
    .unwrap();
    assert_eq!(
        rolled_back_together.evaluation().outcome(),
        HistoryExpectationOutcomeV0::Satisfied
    );
    drop(rolled_back_together);

    let retained_newer = LogWriter::<SqliteL0Store>::open_containing_checkpoint_v0(
        old.path(),
        key(),
        broad_limits(),
        new_checkpoint,
    )
    .err()
    .unwrap();
    assert_eq!(
        retained_newer.not_satisfied_evaluation().unwrap().outcome(),
        HistoryExpectationOutcomeV0::NotSatisfied
    );
}

#[test]
fn p19_physical_logical_and_blob_limits_fail_before_semantic_publication() {
    let padded = TestPath::new("padded-main");
    drop(create(padded.path()));
    let original_length = fs::metadata(padded.path()).unwrap().len();
    let limits = L0ResourceLimitsV0::new(
        original_length,
        broad_limits().max_record_count(),
        broad_limits().max_record_bytes(),
        broad_limits().max_total_record_bytes(),
    )
    .unwrap();
    let exact = LogReader::<SqliteL0Store>::verify_persisted_prefix(
        padded.path(),
        key().verifying_key(),
        limits,
    )
    .unwrap();
    assert_eq!(exact.event_count(), 1);
    OpenOptions::new()
        .append(true)
        .open(padded.path())
        .unwrap()
        .write_all(&[0u8; 4096])
        .unwrap();
    assert!(matches!(
        LogReader::<SqliteL0Store>::verify_persisted_prefix(
            padded.path(),
            key().verifying_key(),
            limits
        ),
        Err(LogError::ResourceLimit {
            resource: "physical main database bytes",
            ..
        })
    ));

    let journal = TestPath::new("oversized-journal");
    drop(create(journal.path()));
    let main_length = fs::metadata(journal.path()).unwrap().len();
    let journal_path = PathBuf::from(format!("{}-journal", journal.path().display()));
    fs::write(
        &journal_path,
        vec![0u8; usize::try_from(main_length + 1).unwrap()],
    )
    .unwrap();
    let journal_limits = L0ResourceLimitsV0::new(
        main_length,
        broad_limits().max_record_count(),
        broad_limits().max_record_bytes(),
        broad_limits().max_total_record_bytes(),
    )
    .unwrap();
    assert!(matches!(
        LogWriter::<SqliteL0Store>::open_verified_prefix(journal.path(), key(), journal_limits),
        Err(LogError::ResourceLimit {
            resource: "physical rollback journal bytes",
            ..
        })
    ));
    fs::remove_file(journal_path).unwrap();

    let blob = TestPath::new("blob-length-first");
    drop(create(blob.path()));
    raw_connection(blob.path())
        .execute(
            "UPDATE magpie_l0_records SET record_bytes = zeroblob(4096) WHERE position_be = ?1",
            params![0u64.to_be_bytes().as_slice()],
        )
        .unwrap();
    let blob_limits = L0ResourceLimitsV0::new(32 * 1024 * 1024, 10, 1024, 1024 * 1024).unwrap();
    assert!(matches!(
        LogReader::<SqliteL0Store>::verify_persisted_prefix(
            blob.path(),
            key().verifying_key(),
            blob_limits
        ),
        Err(LogError::ResourceLimit {
            resource: "individual record bytes",
            ..
        })
    ));
}

#[test]
fn sqlite_record_bytes_match_filestore_bytes_without_jsonl_framing() {
    let sqlite = TestPath::new("byte-equivalence-sqlite");
    let file = TestPath::new("byte-equivalence-file");
    let mut sqlite_writer = LogWriter::<SqliteL0Store>::create_new_with_clock(
        sqlite.path(),
        key(),
        broad_limits(),
        fixed_clock(),
    )
    .unwrap();
    let mut file_writer =
        LogWriter::<FileStore>::open_with_clock(FileStore::new(file.path()), key(), fixed_clock())
            .unwrap();
    sqlite_writer
        .append(provenance("same"), note("same payload"))
        .unwrap();
    file_writer
        .append(provenance("same"), note("same payload"))
        .unwrap();
    drop(sqlite_writer);
    drop(file_writer);

    let sqlite_records = records(sqlite.path());
    let file_bytes = fs::read(file.path()).unwrap();
    let file_records: Vec<Vec<u8>> = file_bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(<[u8]>::to_vec)
        .collect();
    assert_eq!(sqlite_records, file_records);
}

#[test]
fn p19_sequential_append_uses_updated_total_and_failed_append_advances_nothing() {
    let reference = TestPath::new("sequential-reference");
    let mut reference_writer = create(reference.path());
    let genesis_total = reference_writer.total_record_bytes();
    let first = reference_writer
        .append(provenance("A"), note("A".repeat(40)))
        .unwrap();
    let after_first = reference_writer.total_record_bytes();
    let first_length = after_first - genesis_total;
    let second = reference_writer
        .append(provenance("B"), note("B".repeat(80)))
        .unwrap();
    let second_length = reference_writer.total_record_bytes() - after_first;
    assert!(second_length > first_length);
    drop(reference_writer);

    let path = TestPath::new("sequential-budget");
    let total_limit = after_first + second_length - 1;
    let limits = L0ResourceLimitsV0::new(
        broad_limits().max_database_bytes(),
        10,
        broad_limits().max_record_bytes(),
        total_limit,
    )
    .unwrap();
    let mut writer = LogWriter::<SqliteL0Store>::create_new_with_clock(
        path.path(),
        key(),
        limits,
        fixed_clock(),
    )
    .unwrap();
    let committed = writer
        .append(provenance("A"), note("A".repeat(40)))
        .unwrap();
    assert_eq!(committed.hash, first.hash);
    let before = (writer.len(), writer.tip(), writer.total_record_bytes());
    let rejected = writer.append(provenance("B"), note("B".repeat(80)));
    assert!(matches!(
        rejected,
        Err(LogError::ResourceLimit {
            resource: "cumulative record bytes",
            ..
        })
    ));
    assert_eq!(
        (writer.len(), writer.tip(), writer.total_record_bytes()),
        before
    );
    assert!(!writer.is_poisoned());
    assert_eq!(records(path.path()).len(), 2);
    assert_ne!(second.hash, writer.tip());
}

#[test]
fn p19_record_count_wrong_key_empty_owned_and_wal_are_fail_closed() {
    let count = TestPath::new("record-count-limit");
    let mut writer = create(count.path());
    writer.append(provenance("one"), note("one")).unwrap();
    drop(writer);
    let limits =
        L0ResourceLimitsV0::new(32 * 1024 * 1024, 1, 2 * 1024 * 1024, 24 * 1024 * 1024).unwrap();
    assert!(matches!(
        LogReader::<SqliteL0Store>::verify_persisted_prefix(
            count.path(),
            key().verifying_key(),
            limits
        ),
        Err(LogError::ResourceLimit {
            resource: "record count",
            ..
        })
    ));
    assert!(matches!(
        LogWriter::<SqliteL0Store>::open_verified_prefix(count.path(), other_key(), broad_limits()),
        Err(LogError::ChainBroken { seq: 0, .. }) | Err(LogError::BadSignature { seq: 0 })
    ));

    let empty = TestPath::new("empty-owned");
    drop(create(empty.path()));
    raw_connection(empty.path())
        .execute("DELETE FROM magpie_l0_records", [])
        .unwrap();
    assert!(matches!(
        LogWriter::<SqliteL0Store>::open_verified_prefix(empty.path(), key(), broad_limits()),
        Err(LogError::ChainBroken { seq: 0, .. })
    ));

    let wal = TestPath::new("persistent-wal");
    drop(create(wal.path()));
    {
        let connection = raw_connection(wal.path());
        let selected: String = connection
            .query_row("PRAGMA journal_mode = WAL", [], |row| row.get(0))
            .unwrap();
        assert_eq!(selected.to_ascii_lowercase(), "wal");
    }
    assert!(matches!(
        LogReader::<SqliteL0Store>::verify_persisted_prefix(
            wal.path(),
            key().verifying_key(),
            broad_limits()
        ),
        Err(LogError::UnsupportedDatabase { .. })
    ));
    {
        let connection = raw_connection(wal.path());
        let mode: String = connection
            .query_row("PRAGMA journal_mode", [], |row| row.get(0))
            .unwrap();
        assert_eq!(mode.to_ascii_lowercase(), "wal");
    }
    assert!(matches!(
        LogWriter::<SqliteL0Store>::open_verified_prefix(wal.path(), key(), broad_limits()),
        Err(LogError::UnsupportedDatabase { .. })
    ));
    let connection = raw_connection(wal.path());
    let mode: String = connection
        .query_row("PRAGMA journal_mode", [], |row| row.get(0))
        .unwrap();
    assert_eq!(mode.to_ascii_lowercase(), "wal");
}

#[test]
fn read_only_verification_establishes_and_preserves_delete_profile() {
    let path = TestPath::new("read-only-delete-profile");
    drop(create(path.path()));

    let summary = LogReader::<SqliteL0Store>::verify_persisted_prefix(
        path.path(),
        key().verifying_key(),
        broad_limits(),
    )
    .unwrap();
    assert_eq!(summary.event_count(), 1);

    let connection = raw_connection(path.path());
    let mode: String = connection
        .query_row("PRAGMA journal_mode", [], |row| row.get(0))
        .unwrap();
    assert_eq!(mode.to_ascii_lowercase(), "delete");
}

#[test]
fn p20_schema_has_no_ambient_checkpoint_or_mutable_head_state() {
    let path = TestPath::new("no-ambient-checkpoint");
    drop(create(path.path()));
    let connection = raw_connection(path.path());
    let objects: Vec<String> = {
        let mut statement = connection
            .prepare("SELECT name FROM sqlite_schema WHERE name NOT LIKE 'sqlite_%' ORDER BY name")
            .unwrap();
        statement
            .query_map([], |row| row.get(0))
            .unwrap()
            .map(Result::unwrap)
            .collect()
    };
    assert_eq!(objects, vec!["magpie_l0_records"]);

    let summary = LogReader::<SqliteL0Store>::verify_persisted_prefix(
        path.path(),
        key().verifying_key(),
        broad_limits(),
    )
    .unwrap();
    assert_eq!(summary.event_count(), 1);
}
