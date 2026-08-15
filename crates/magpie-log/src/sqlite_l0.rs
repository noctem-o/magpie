use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Duration;

use ed25519_dalek::{SigningKey, VerifyingKey};
use rusqlite::ffi::ErrorCode;
use rusqlite::types::ValueRef;
use rusqlite::{params, Connection, OpenFlags};

use crate::canonical::CANONICALIZATION_PROFILE;
use crate::error::LogError;
use crate::event::{Payload, Provenance, SignedEvent};
use crate::hashing::ContentHash;
use crate::history_expectation::{
    evaluate_verified_history_expectation_v0, HistoryCheckpointV0, HistoryExpectationEvaluationV0,
    HistoryExpectationOutcomeV0, HistoryExpectationRelationV0,
};
use crate::logimpl::{
    build_signed_record, system_clock, verify_event_integrity, Clock, IncrementalVerifier,
    LogReader, LogWriter, VerifiedReplaySummary, VerifiedSummary,
};

const SQLITE_HEADER_LEN: usize = 100;
const SQLITE_MAGIC: &[u8; 16] = b"SQLite format 3\0";
const USER_VERSION_OFFSET: usize = 60;
const APPLICATION_ID_OFFSET: usize = 68;
const MPL0_APPLICATION_ID: u32 = 0x4D50_4C30;
const MPL0_USER_VERSION: u32 = 1;
const RECORDS_TABLE: &str = "magpie_l0_records";
const CREATE_SCHEMA_SQL: &str = "CREATE TABLE magpie_l0_records (\
position_be BLOB NOT NULL PRIMARY KEY CHECK(length(position_be) = 8), \
record_bytes BLOB NOT NULL\
) STRICT, WITHOUT ROWID";

/// Explicit finite limits for one supported SQLite L0 operation.
///
/// Every value is positive. There is no default or ambient unlimited mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct L0ResourceLimitsV0 {
    max_database_bytes: u64,
    max_record_count: u64,
    max_record_bytes: u64,
    max_total_record_bytes: u64,
}

impl L0ResourceLimitsV0 {
    pub fn new(
        max_database_bytes: u64,
        max_record_count: u64,
        max_record_bytes: u64,
        max_total_record_bytes: u64,
    ) -> Result<Self, LogError> {
        let limits = Self {
            max_database_bytes,
            max_record_count,
            max_record_bytes,
            max_total_record_bytes,
        };
        if [
            max_database_bytes,
            max_record_count,
            max_record_bytes,
            max_total_record_bytes,
        ]
        .contains(&0)
        {
            return Err(LogError::InvalidResourceLimits {
                detail: "all four limits must be positive".into(),
            });
        }
        Ok(limits)
    }

    pub fn max_database_bytes(&self) -> u64 {
        self.max_database_bytes
    }

    pub fn max_record_count(&self) -> u64 {
        self.max_record_count
    }

    pub fn max_record_bytes(&self) -> u64 {
        self.max_record_bytes
    }

    pub fn max_total_record_bytes(&self) -> u64 {
        self.max_total_record_bytes
    }
}

/// Supported local SQLite L0 storage capability.
///
/// This type exposes neither its SQLite connection nor raw SQL and deliberately
/// does not implement [`crate::LogStore`]. Creation and reopening are available
/// only through the semantically named `LogWriter<SqliteL0Store>` operations.
/// It exposes no general second cursor pass into arbitrary [`crate::Projection`]
/// implementations because that trait has no replay-wide atomic staging law.
/// The supported durability and locking profile assumes a local filesystem;
/// no network-filesystem guarantee is made.
///
/// ```compile_fail,E0599
/// use magpie_log::SqliteL0Store;
/// let _ = SqliteL0Store::new("history.db");
/// ```
///
/// ```compile_fail,E0277
/// use magpie_log::{LogStore, SqliteL0Store};
/// fn needs_whole_vector<S: LogStore>() {}
/// needs_whole_vector::<SqliteL0Store>();
/// ```
///
/// ```compile_fail,E0599
/// use magpie_log::SqliteL0Store;
/// fn cannot_escape_connection(store: &SqliteL0Store) {
///     let _ = store.connection();
/// }
/// ```
///
/// ```compile_fail,E0599
/// use magpie_log::SqliteL0Store;
/// fn cannot_raw_append(store: &mut SqliteL0Store) {
///     store.append_record(b"caller bytes").unwrap();
/// }
/// ```
pub struct SqliteL0Store {
    path: PathBuf,
    connection: Option<Connection>,
    limits: L0ResourceLimitsV0,
    total_record_bytes: u64,
    poisoned: bool,
    #[cfg(test)]
    fault_after_commit_unknown: bool,
}

/// A successful checkpoint-qualified writer open.
///
/// The genuine evaluation is retained alongside the writer so two opens of the
/// same terminal history under different checkpoints remain distinguishable.
/// `Satisfied` means only that the exact verified history contained the exact
/// explicit checkpoint under `magpie-log-history-expectation-v0`. It does not
/// establish checkpoint authenticity, trust, authority, freshness, currentness,
/// canonicality, non-equivocation, secure retention, or rollback resistance.
///
/// Callers cannot fabricate this value by pairing arbitrary parts:
///
/// ```compile_fail,E0451
/// use magpie_log::{CheckpointQualifiedWriterOpenV0, SqliteL0Store, LogWriter,
///     HistoryExpectationEvaluationV0};
/// fn fabricate(writer: LogWriter<SqliteL0Store>, evaluation: HistoryExpectationEvaluationV0) {
///     let _ = CheckpointQualifiedWriterOpenV0 { writer, evaluation };
/// }
/// ```
///
/// The retained writer also cannot be replaced through a mutable accessor:
///
/// ```compile_fail,E0599
/// use magpie_log::{CheckpointQualifiedWriterOpenV0, LogWriter, SqliteL0Store};
/// fn substitute(
///     qualified: &mut CheckpointQualifiedWriterOpenV0,
///     other: LogWriter<SqliteL0Store>,
/// ) {
///     let _ = std::mem::replace(qualified.writer_mut(), other);
/// }
/// ```
pub struct CheckpointQualifiedWriterOpenV0 {
    writer: LogWriter<SqliteL0Store>,
    evaluation: HistoryExpectationEvaluationV0,
}

impl CheckpointQualifiedWriterOpenV0 {
    pub fn evaluation(&self) -> &HistoryExpectationEvaluationV0 {
        &self.evaluation
    }

    pub fn writer(&self) -> &LogWriter<SqliteL0Store> {
        &self.writer
    }

    pub fn append(
        &mut self,
        provenance: Provenance,
        payload: Payload,
    ) -> Result<SignedEvent, LogError> {
        self.writer.append(provenance, payload)
    }
}

/// Failure from checkpoint-qualified reopening.
#[derive(Debug, thiserror::Error)]
pub enum SqliteCheckpointOpenError {
    /// The history verified completely, but did not contain the checkpoint.
    #[error("verified SQLite L0 history did not contain the explicit checkpoint")]
    NotSatisfied(HistoryExpectationEvaluationV0),
    /// Verification or an operational step could not complete, so no semantic
    /// checkpoint conclusion exists.
    #[error(transparent)]
    Operational(#[from] LogError),
}

impl SqliteCheckpointOpenError {
    pub fn not_satisfied_evaluation(&self) -> Option<&HistoryExpectationEvaluationV0> {
        match self {
            Self::NotSatisfied(evaluation) => Some(evaluation),
            Self::Operational(_) => None,
        }
    }
}

struct RawHeader {
    application_id: u32,
    user_version: u32,
}

struct OpenVerification {
    summary: VerifiedSummary,
    observed_checkpoint_commitment: Option<ContentHash>,
}

fn sqlite_error(operation: &'static str, error: rusqlite::Error) -> LogError {
    LogError::SqliteOperational {
        operation,
        detail: error.to_string(),
    }
}

fn is_busy(error: &rusqlite::Error) -> bool {
    matches!(
        error.sqlite_error_code(),
        Some(ErrorCode::DatabaseBusy | ErrorCode::DatabaseLocked)
    )
}

fn check_limit(resource: &'static str, actual: u64, limit: u64) -> Result<(), LogError> {
    if actual > limit {
        Err(LogError::ResourceLimit {
            resource,
            actual,
            limit,
        })
    } else {
        Ok(())
    }
}

fn checked_file_len(path: &Path) -> Result<Option<u64>, LogError> {
    match std::fs::metadata(path) {
        Ok(metadata) => Ok(Some(metadata.len())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error.into()),
    }
}

fn check_main_file_limit(path: &Path, limits: L0ResourceLimitsV0) -> Result<Option<u64>, LogError> {
    let length = checked_file_len(path)?;
    if let Some(length) = length {
        check_limit(
            "physical main database bytes",
            length,
            limits.max_database_bytes,
        )?;
    }
    Ok(length)
}

fn check_journal_file_limit(path: &Path, limits: L0ResourceLimitsV0) -> Result<(), LogError> {
    let mut journal_name = path.as_os_str().to_os_string();
    journal_name.push("-journal");
    let journal = PathBuf::from(journal_name);
    if let Some(length) = checked_file_len(&journal)? {
        check_limit(
            "physical rollback journal bytes",
            length,
            limits.max_database_bytes,
        )?;
    }
    Ok(())
}

fn read_header(path: &Path) -> Result<RawHeader, LogError> {
    let mut bytes = [0u8; SQLITE_HEADER_LEN];
    let mut file = File::open(path)?;
    file.read_exact(&mut bytes).map_err(|error| {
        if error.kind() == std::io::ErrorKind::UnexpectedEof {
            LogError::ForeignDatabase {
                detail: "non-empty file is shorter than the SQLite header".into(),
            }
        } else {
            error.into()
        }
    })?;
    if &bytes[..SQLITE_MAGIC.len()] != SQLITE_MAGIC {
        return Err(LogError::ForeignDatabase {
            detail: "non-empty file does not have SQLite format-3 magic".into(),
        });
    }
    Ok(RawHeader {
        user_version: u32::from_be_bytes(
            bytes[USER_VERSION_OFFSET..USER_VERSION_OFFSET + 4]
                .try_into()
                .expect("fixed SQLite header slice"),
        ),
        application_id: u32::from_be_bytes(
            bytes[APPLICATION_ID_OFFSET..APPLICATION_ID_OFFSET + 4]
                .try_into()
                .expect("fixed SQLite header slice"),
        ),
    })
}

fn require_owned_header(path: &Path) -> Result<(), LogError> {
    let header = read_header(path)?;
    if header.application_id != MPL0_APPLICATION_ID {
        return Err(LogError::ForeignDatabase {
            detail: format!(
                "application_id is 0x{:08X}, expected MPL0 0x{MPL0_APPLICATION_ID:08X}",
                header.application_id
            ),
        });
    }
    if header.user_version != MPL0_USER_VERSION {
        return Err(LogError::UnsupportedDatabase {
            detail: format!(
                "MPL0 user_version is {}, supported version is {MPL0_USER_VERSION}",
                header.user_version
            ),
        });
    }
    Ok(())
}

fn open_flags(read_only: bool, create: bool) -> OpenFlags {
    let access = if read_only {
        OpenFlags::SQLITE_OPEN_READ_ONLY
    } else {
        OpenFlags::SQLITE_OPEN_READ_WRITE
    };
    access
        | if create {
            OpenFlags::SQLITE_OPEN_CREATE
        } else {
            OpenFlags::empty()
        }
        | OpenFlags::SQLITE_OPEN_NO_MUTEX
        | OpenFlags::SQLITE_OPEN_PRIVATE_CACHE
        | OpenFlags::SQLITE_OPEN_NOFOLLOW
        | OpenFlags::SQLITE_OPEN_EXRESCODE
}

fn open_connection(path: &Path, read_only: bool, create: bool) -> Result<Connection, LogError> {
    Connection::open_with_flags(path, open_flags(read_only, create))
        .map_err(|error| sqlite_error("open", error))
}

fn query_text(
    connection: &Connection,
    sql: &str,
    operation: &'static str,
) -> Result<String, LogError> {
    connection
        .query_row(sql, [], |row| row.get(0))
        .map_err(|error| sqlite_error(operation, error))
}

fn configure_connection(connection: &Connection) -> Result<(), LogError> {
    connection
        .busy_timeout(Duration::ZERO)
        .map_err(|error| sqlite_error("configure busy timeout", error))?;

    let journal_mode = query_text(connection, "PRAGMA main.journal_mode", "query journal mode")?;
    if journal_mode.eq_ignore_ascii_case("wal") {
        return Err(LogError::UnsupportedDatabase {
            detail: "persistent WAL journal mode is outside the supported v0 profile".into(),
        });
    }
    let selected = query_text(
        connection,
        "PRAGMA main.journal_mode = DELETE",
        "select DELETE journal mode",
    )?;
    if !selected.eq_ignore_ascii_case("delete") {
        return Err(LogError::UnsupportedDatabase {
            detail: format!("SQLite selected journal mode {selected:?}, expected DELETE"),
        });
    }

    connection
        .execute_batch(
            "PRAGMA main.synchronous = EXTRA;\
             PRAGMA main.locking_mode = NORMAL;\
             PRAGMA main.read_uncommitted = OFF;",
        )
        .map_err(|error| sqlite_error("configure connection profile", error))?;

    let synchronous: i64 = connection
        .query_row("PRAGMA main.synchronous", [], |row| row.get(0))
        .map_err(|error| sqlite_error("verify synchronous", error))?;
    let locking_mode = query_text(
        connection,
        "PRAGMA main.locking_mode",
        "verify locking mode",
    )?;
    let read_uncommitted: i64 = connection
        .query_row("PRAGMA main.read_uncommitted", [], |row| row.get(0))
        .map_err(|error| sqlite_error("verify read_uncommitted", error))?;
    if synchronous != 3 || !locking_mode.eq_ignore_ascii_case("normal") || read_uncommitted != 0 {
        return Err(LogError::UnsupportedDatabase {
            detail: format!(
                "connection profile mismatch: synchronous={synchronous}, locking_mode={locking_mode:?}, read_uncommitted={read_uncommitted}"
            ),
        });
    }

    let databases: Vec<(i64, String)> = {
        let mut statement = connection
            .prepare("PRAGMA database_list")
            .map_err(|error| sqlite_error("prepare database-list check", error))?;
        let rows = statement
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .map_err(|error| sqlite_error("query database-list check", error))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|error| sqlite_error("read database-list check", error))?
    };
    if databases != vec![(0, "main".into())] {
        return Err(LogError::UnsupportedDatabase {
            detail: format!("unexpected attached databases: {databases:?}"),
        });
    }
    Ok(())
}

fn check_empty_unowned_sqlite(path: &Path, limits: L0ResourceLimitsV0) -> Result<(), LogError> {
    check_main_file_limit(path, limits)?;
    let header = read_header(path)?;
    if header.application_id != 0 || header.user_version != 0 {
        return Err(LogError::ForeignDatabase {
            detail: "existing SQLite destination is not empty and unowned".into(),
        });
    }
    let connection = open_connection(path, true, false)?;
    configure_connection(&connection)?;
    let object_count: i64 = connection
        .query_row(
            "SELECT count(*) FROM main.sqlite_schema WHERE name NOT LIKE 'sqlite_%'",
            [],
            |row| row.get(0),
        )
        .map_err(|error| sqlite_error("inspect unowned schema", error))?;
    if object_count != 0 {
        return Err(LogError::ForeignDatabase {
            detail: "existing unowned SQLite destination contains user objects".into(),
        });
    }
    Ok(())
}

fn logical_database_bytes(connection: &Connection) -> Result<u64, LogError> {
    let page_count: i64 = connection
        .query_row("PRAGMA main.page_count", [], |row| row.get(0))
        .map_err(|error| sqlite_error("query page count", error))?;
    let page_size: i64 = connection
        .query_row("PRAGMA main.page_size", [], |row| row.get(0))
        .map_err(|error| sqlite_error("query page size", error))?;
    let page_count = u64::try_from(page_count).map_err(|_| LogError::SqliteOperational {
        operation: "query page count",
        detail: "SQLite returned a negative page count".into(),
    })?;
    let page_size = u64::try_from(page_size).map_err(|_| LogError::SqliteOperational {
        operation: "query page size",
        detail: "SQLite returned a negative page size".into(),
    })?;
    page_count
        .checked_mul(page_size)
        .ok_or(LogError::ResourceLimit {
            resource: "logical database bytes",
            actual: u64::MAX,
            limit: u64::MAX,
        })
}

fn verify_schema(connection: &Connection) -> Result<(), LogError> {
    let application_id: i64 = connection
        .query_row("PRAGMA main.application_id", [], |row| row.get(0))
        .map_err(|error| sqlite_error("verify application_id", error))?;
    let user_version: i64 = connection
        .query_row("PRAGMA main.user_version", [], |row| row.get(0))
        .map_err(|error| sqlite_error("verify user_version", error))?;
    if application_id != i64::from(MPL0_APPLICATION_ID)
        || user_version != i64::from(MPL0_USER_VERSION)
    {
        return Err(LogError::UnsupportedDatabase {
            detail: format!(
                "in-database ownership mismatch: application_id={application_id}, user_version={user_version}"
            ),
        });
    }

    let object_count: i64 = connection
        .query_row(
            "SELECT count(*) FROM main.sqlite_schema WHERE name NOT LIKE 'sqlite_%'",
            [],
            |row| row.get(0),
        )
        .map_err(|error| sqlite_error("count schema objects", error))?;
    if object_count != 1 {
        return Err(LogError::UnsupportedDatabase {
            detail: format!("schema contains {object_count} user objects, expected exactly one"),
        });
    }
    let object: (String, String, String, Option<String>) = connection
        .query_row(
            "SELECT type, name, tbl_name, sql FROM main.sqlite_schema \
             WHERE name NOT LIKE 'sqlite_%'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .map_err(|error| sqlite_error("read schema fingerprint", error))?;
    let expected = (
        "table".into(),
        RECORDS_TABLE.into(),
        RECORDS_TABLE.into(),
        Some(CREATE_SCHEMA_SQL.into()),
    );
    if object != expected {
        return Err(LogError::UnsupportedDatabase {
            detail: format!("schema fingerprint mismatch: {object:?}"),
        });
    }

    let table_shape: Vec<(String, String, i64, i64, i64)> = {
        let mut statement = connection
            .prepare("PRAGMA main.table_xinfo('magpie_l0_records')")
            .map_err(|error| sqlite_error("prepare table shape", error))?;
        let rows = statement
            .query_map([], |row| {
                Ok((
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(5)?,
                    row.get(6)?,
                ))
            })
            .map_err(|error| sqlite_error("query table shape", error))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|error| sqlite_error("read table shape", error))?
    };
    if table_shape
        != vec![
            ("position_be".into(), "BLOB".into(), 1, 1, 0),
            ("record_bytes".into(), "BLOB".into(), 1, 0, 0),
        ]
    {
        return Err(LogError::UnsupportedDatabase {
            detail: format!("record table shape mismatch: {table_shape:?}"),
        });
    }

    let table_flags: (i64, i64, i64) = connection
        .query_row(
            "SELECT ncol, wr, strict FROM pragma_table_list('magpie_l0_records') \
             WHERE schema = 'main' AND name = 'magpie_l0_records'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .map_err(|error| sqlite_error("verify table flags", error))?;
    if table_flags != (2, 1, 1) {
        return Err(LogError::UnsupportedDatabase {
            detail: format!("record table flags mismatch: {table_flags:?}"),
        });
    }
    Ok(())
}

fn verify_integrity(connection: &Connection) -> Result<(), LogError> {
    let mut statement = connection
        .prepare("PRAGMA main.integrity_check")
        .map_err(|error| sqlite_error("prepare integrity check", error))?;
    let mut rows = statement
        .query([])
        .map_err(|error| sqlite_error("run integrity check", error))?;
    let first = rows
        .next()
        .map_err(|error| sqlite_error("read integrity check", error))?
        .ok_or_else(|| LogError::UnsupportedDatabase {
            detail: "SQLite integrity_check returned no result".into(),
        })?;
    let first: String = first
        .get(0)
        .map_err(|error| sqlite_error("read integrity-check result", error))?;
    let has_more = rows
        .next()
        .map_err(|error| sqlite_error("advance integrity check", error))?
        .is_some();
    if first != "ok" || has_more {
        return Err(LogError::UnsupportedDatabase {
            detail: format!(
                "SQLite integrity_check failed; first result was {first:?}, additional results={has_more}"
            ),
        });
    }
    Ok(())
}

fn decode_position(value: ValueRef<'_>) -> Result<u64, LogError> {
    let ValueRef::Blob(bytes) = value else {
        return Err(LogError::UnsupportedDatabase {
            detail: "record position is not a BLOB".into(),
        });
    };
    let bytes: [u8; 8] = bytes
        .try_into()
        .map_err(|_| LogError::UnsupportedDatabase {
            detail: "record position is not exactly eight bytes".into(),
        })?;
    Ok(u64::from_be_bytes(bytes))
}

fn verify_rows(
    connection: &Connection,
    verifying_key: VerifyingKey,
    limits: L0ResourceLimitsV0,
    checkpoint_event_count: Option<u64>,
) -> Result<OpenVerification, LogError> {
    let mut verifier = IncrementalVerifier::new(verifying_key);
    let mut total_record_bytes = 0u64;
    let mut observed = checkpoint_event_count
        .filter(|count| *count == 0)
        .map(|_| ContentHash::ZERO);
    let mut statement = connection
        .prepare(
            "SELECT position_be, length(record_bytes), record_bytes \
             FROM main.magpie_l0_records ORDER BY position_be",
        )
        .map_err(|error| sqlite_error("prepare bounded record traversal", error))?;
    let mut rows = statement
        .query([])
        .map_err(|error| sqlite_error("query bounded record traversal", error))?;

    while let Some(row) = rows
        .next()
        .map_err(|error| sqlite_error("advance bounded record traversal", error))?
    {
        let next_count = verifier
            .count()
            .checked_add(1)
            .ok_or(LogError::SequenceExhausted)?;
        check_limit("record count", next_count, limits.max_record_count)?;

        let reported_length: i64 = row
            .get(1)
            .map_err(|error| sqlite_error("read record length", error))?;
        let reported_length =
            u64::try_from(reported_length).map_err(|_| LogError::UnsupportedDatabase {
                detail: "SQLite reported a negative record BLOB length".into(),
            })?;
        check_limit(
            "individual record bytes",
            reported_length,
            limits.max_record_bytes,
        )?;
        let next_total =
            total_record_bytes
                .checked_add(reported_length)
                .ok_or(LogError::ResourceLimit {
                    resource: "cumulative record bytes",
                    actual: u64::MAX,
                    limit: limits.max_total_record_bytes,
                })?;
        check_limit(
            "cumulative record bytes",
            next_total,
            limits.max_total_record_bytes,
        )?;

        let physical_position = decode_position(
            row.get_ref(0)
                .map_err(|error| sqlite_error("borrow physical position", error))?,
        )?;
        // The limits above precede any Magpie-owned copy or allocation of the
        // record. `get_ref` borrows SQLite's row-owned BLOB storage.
        let record = match row
            .get_ref(2)
            .map_err(|error| sqlite_error("borrow bounded record", error))?
        {
            ValueRef::Blob(bytes) => bytes,
            _ => {
                return Err(LogError::UnsupportedDatabase {
                    detail: "record_bytes is not a BLOB".into(),
                });
            }
        };
        let actual_length = u64::try_from(record.len()).map_err(|_| LogError::ResourceLimit {
            resource: "individual record bytes",
            actual: u64::MAX,
            limit: limits.max_record_bytes,
        })?;
        if actual_length != reported_length {
            return Err(LogError::UnsupportedDatabase {
                detail: "record BLOB length changed within one SQLite row observation".into(),
            });
        }

        let event = verifier.verify_record(physical_position, record)?;
        if event.core.seq.checked_add(1) == checkpoint_event_count {
            observed = Some(event.hash);
        }
        total_record_bytes = next_total;
    }

    Ok(OpenVerification {
        summary: verifier.finish(total_record_bytes),
        observed_checkpoint_commitment: observed,
    })
}

fn verify_owned_snapshot(
    connection: &Connection,
    verifying_key: VerifyingKey,
    limits: L0ResourceLimitsV0,
    checkpoint_event_count: Option<u64>,
) -> Result<OpenVerification, LogError> {
    connection
        .execute_batch("BEGIN DEFERRED")
        .map_err(|error| sqlite_error("begin stable read transaction", error))?;

    let result = (|| {
        // This is deliberately the first read of main inside the transaction;
        // it anchors the snapshot before every later semantic database read.
        let logical_bytes = logical_database_bytes(connection)?;
        check_limit(
            "logical database bytes",
            logical_bytes,
            limits.max_database_bytes,
        )?;
        verify_schema(connection)?;
        verify_integrity(connection)?;
        let verified = verify_rows(connection, verifying_key, limits, checkpoint_event_count)?;
        if verified.summary.count == 0 {
            return Err(LogError::ChainBroken {
                seq: 0,
                detail: "recognized MPL0 database contains no genesis record".into(),
            });
        }
        Ok(verified)
    })();

    match result {
        Ok(verified) => {
            connection
                .execute_batch("COMMIT")
                .map_err(|error| sqlite_error("end stable read transaction", error))?;
            Ok(verified)
        }
        Err(error) => {
            let _ = connection.execute_batch("ROLLBACK");
            Err(error)
        }
    }
}

fn open_owned_connection(
    path: &Path,
    limits: L0ResourceLimitsV0,
    read_only: bool,
) -> Result<Connection, LogError> {
    check_main_file_limit(path, limits)?;
    require_owned_header(path)?;
    if !read_only {
        check_journal_file_limit(path, limits)?;
    }
    let connection = open_connection(path, read_only, false)?;
    configure_connection(&connection)?;
    Ok(connection)
}

fn make_writer(
    path: PathBuf,
    connection: Connection,
    signing_key: SigningKey,
    limits: L0ResourceLimitsV0,
    verified: VerifiedSummary,
    clock: Clock,
) -> LogWriter<SqliteL0Store> {
    LogWriter {
        store: SqliteL0Store {
            path,
            connection: Some(connection),
            limits,
            total_record_bytes: verified.total_record_bytes,
            poisoned: false,
            #[cfg(test)]
            fault_after_commit_unknown: false,
        },
        signing_key,
        last_hash: verified.tip,
        next_seq: verified.count,
        clock,
    }
}

impl LogReader<SqliteL0Store> {
    /// Completely verify one existing supported SQLite L0 prefix with bounded
    /// memory and one stable read transaction.
    ///
    /// This returns an inspectable count-and-tip compatibility summary of the
    /// exact persisted prefix observed by this call. It is not a complete
    /// producing-coordinate envelope or a coordinate-complete governed input;
    /// trust in the caller-supplied key remains external. It does not establish
    /// expected history, freshness, latest state, currentness, canonicality, or
    /// rollback resistance.
    pub fn verify_persisted_prefix(
        path: impl AsRef<Path>,
        verifying_key: VerifyingKey,
        limits: L0ResourceLimitsV0,
    ) -> Result<VerifiedReplaySummary, LogError> {
        let path = path.as_ref();
        let connection = open_owned_connection(path, limits, true)?;
        let verified = verify_owned_snapshot(&connection, verifying_key, limits, None)?;
        Ok(VerifiedReplaySummary::from_verified_parts(
            verified.summary.count,
            verified.summary.tip,
        ))
    }
}

impl LogWriter<SqliteL0Store> {
    /// Create exactly one new MPL0 v1 history and its genesis event.
    ///
    /// Creation accepts only an absent, zero-byte, or exact empty/unowned SQLite
    /// destination. It applies the explicit limits before initialization commit
    /// and never acts as an implicit reopen, migration, adoption, or repair.
    pub fn create_new(
        path: impl AsRef<Path>,
        signing_key: SigningKey,
        limits: L0ResourceLimitsV0,
    ) -> Result<Self, LogError> {
        Self::create_new_with_clock(path, signing_key, limits, Box::new(system_clock))
    }

    /// Deterministic-clock variant of [`Self::create_new`] for testing.
    pub fn create_new_with_clock(
        path: impl AsRef<Path>,
        signing_key: SigningKey,
        limits: L0ResourceLimitsV0,
        mut clock: Clock,
    ) -> Result<Self, LogError> {
        let path = path.as_ref().to_path_buf();
        let existing_length = check_main_file_limit(&path, limits)?;
        match existing_length {
            Some(0) | None => {}
            Some(_) => check_empty_unowned_sqlite(&path, limits)?,
        }

        let connection = open_connection(&path, false, true)?;
        configure_connection(&connection)?;
        if let Err(error) = connection.execute_batch("BEGIN IMMEDIATE") {
            return if is_busy(&error) {
                Err(LogError::StorageBusy {
                    operation: "begin create transaction",
                })
            } else {
                Err(sqlite_error("begin create transaction", error))
            };
        }

        let result = (|| {
            connection
                .execute_batch(&format!(
                    "PRAGMA main.application_id = {MPL0_APPLICATION_ID};\
                     PRAGMA main.user_version = {MPL0_USER_VERSION};\
                     {CREATE_SCHEMA_SQL};"
                ))
                .map_err(|error| sqlite_error("initialize MPL0 schema", error))?;

            let (genesis, record) = build_signed_record(
                &signing_key,
                0,
                ContentHash::ZERO,
                clock(),
                Provenance::new("magpie-log", "genesis"),
                Payload::Genesis {
                    canonicalization_profile: CANONICALIZATION_PROFILE.to_string(),
                    verifying_key: hex::encode(signing_key.verifying_key().as_bytes()),
                },
            )?;
            let record_length =
                u64::try_from(record.len()).map_err(|_| LogError::ResourceLimit {
                    resource: "genesis record bytes",
                    actual: u64::MAX,
                    limit: limits.max_record_bytes,
                })?;
            check_limit("initial record count", 1, limits.max_record_count)?;
            check_limit(
                "individual record bytes",
                record_length,
                limits.max_record_bytes,
            )?;
            check_limit(
                "cumulative record bytes",
                record_length,
                limits.max_total_record_bytes,
            )?;
            connection
                .execute(
                    "INSERT INTO main.magpie_l0_records(position_be, record_bytes) VALUES (?1, ?2)",
                    params![0u64.to_be_bytes().as_slice(), record.as_slice()],
                )
                .map_err(|error| sqlite_error("insert genesis", error))?;
            let logical_bytes = logical_database_bytes(&connection)?;
            check_limit(
                "logical database bytes",
                logical_bytes,
                limits.max_database_bytes,
            )?;
            check_journal_file_limit(&path, limits)?;
            Ok((genesis, record_length))
        })();

        let (genesis, record_length) = match result {
            Ok(values) => values,
            Err(error) => {
                let rollback = connection.execute_batch("ROLLBACK");
                if rollback.is_err() || !connection.is_autocommit() {
                    drop(connection);
                    return Err(LogError::CommitStateUnknown);
                }
                return Err(error);
            }
        };

        if let Err(error) = connection.execute_batch("COMMIT") {
            if is_busy(&error)
                && !connection.is_autocommit()
                && connection.execute_batch("ROLLBACK").is_ok()
                && connection.is_autocommit()
            {
                return Err(LogError::StorageBusy {
                    operation: "commit creation",
                });
            }
            drop(connection);
            return Err(LogError::CommitStateUnknown);
        }

        Ok(make_writer(
            path,
            connection,
            signing_key,
            limits,
            VerifiedSummary {
                count: 1,
                tip: genesis.hash,
                total_record_bytes: record_length,
            },
            clock,
        ))
    }

    /// Reopen one existing completely verified persisted prefix for writing.
    ///
    /// This deliberately weak mode establishes only validity of the exact
    /// observed prefix under the supplied key. It establishes no expected
    /// history, freshness, latest state, currentness, canonicality,
    /// non-equivocation, checkpoint protection, or rollback resistance.
    ///
    /// ```compile_fail,E0599
    /// use magpie_log::{LogWriter, SqliteL0Store};
    /// fn ordinary_open_has_no_checkpoint_result(writer: &LogWriter<SqliteL0Store>) {
    ///     let _ = writer.evaluation();
    /// }
    /// ```
    pub fn open_verified_prefix(
        path: impl AsRef<Path>,
        signing_key: SigningKey,
        limits: L0ResourceLimitsV0,
    ) -> Result<Self, LogError> {
        Self::open_verified_prefix_with_clock(path, signing_key, limits, Box::new(system_clock))
    }

    /// Deterministic-clock variant of [`Self::open_verified_prefix`] for testing.
    pub fn open_verified_prefix_with_clock(
        path: impl AsRef<Path>,
        signing_key: SigningKey,
        limits: L0ResourceLimitsV0,
        clock: Clock,
    ) -> Result<Self, LogError> {
        let path = path.as_ref().to_path_buf();
        let connection = open_owned_connection(&path, limits, false)?;
        let verified =
            verify_owned_snapshot(&connection, signing_key.verifying_key(), limits, None)?;
        Ok(make_writer(
            path,
            connection,
            signing_key,
            limits,
            verified.summary,
            clock,
        ))
    }

    /// Reopen only when the verified history contains the explicit checkpoint.
    ///
    /// Method identity fixes `ContainsCheckpoint`; no relation is inferred or
    /// selected from ambient state. Success retains the genuine context-bearing
    /// evaluation alongside the writer. It does not authenticate the checkpoint
    /// or establish freshness, authority, currentness, or rollback resistance.
    ///
    /// ```compile_fail,E0061
    /// use magpie_log::{L0ResourceLimitsV0, LogWriter, SigningKey, SqliteL0Store};
    /// fn checkpoint_is_mandatory(path: &std::path::Path) {
    ///     let key = SigningKey::from_bytes(&[7u8; 32]);
    ///     let limits = L0ResourceLimitsV0::new(4096, 1, 1024, 1024).unwrap();
    ///     let _ = LogWriter::<SqliteL0Store>::open_containing_checkpoint_v0(
    ///         path, key, limits,
    ///     );
    /// }
    /// ```
    pub fn open_containing_checkpoint_v0(
        path: impl AsRef<Path>,
        signing_key: SigningKey,
        limits: L0ResourceLimitsV0,
        checkpoint: HistoryCheckpointV0,
    ) -> Result<CheckpointQualifiedWriterOpenV0, SqliteCheckpointOpenError> {
        Self::open_containing_checkpoint_v0_with_clock(
            path,
            signing_key,
            limits,
            checkpoint,
            Box::new(system_clock),
        )
    }

    /// Deterministic-clock variant of [`Self::open_containing_checkpoint_v0`].
    pub fn open_containing_checkpoint_v0_with_clock(
        path: impl AsRef<Path>,
        signing_key: SigningKey,
        limits: L0ResourceLimitsV0,
        checkpoint: HistoryCheckpointV0,
        clock: Clock,
    ) -> Result<CheckpointQualifiedWriterOpenV0, SqliteCheckpointOpenError> {
        let path = path.as_ref().to_path_buf();
        let connection = open_owned_connection(&path, limits, false)?;
        let verified = verify_owned_snapshot(
            &connection,
            signing_key.verifying_key(),
            limits,
            Some(checkpoint.event_count()),
        )?;
        let summary = VerifiedReplaySummary::from_verified_parts(
            verified.summary.count,
            verified.summary.tip,
        );
        let evaluation = evaluate_verified_history_expectation_v0(
            signing_key.verifying_key().to_bytes(),
            summary,
            checkpoint,
            HistoryExpectationRelationV0::ContainsCheckpoint,
            verified.observed_checkpoint_commitment,
        );
        if evaluation.outcome() == HistoryExpectationOutcomeV0::NotSatisfied {
            return Err(SqliteCheckpointOpenError::NotSatisfied(evaluation));
        }
        Ok(CheckpointQualifiedWriterOpenV0 {
            writer: make_writer(
                path,
                connection,
                signing_key,
                limits,
                verified.summary,
                clock,
            ),
            evaluation,
        })
    }

    /// Exact serialized record-byte total at the last acknowledged coordinate.
    pub fn total_record_bytes(&self) -> u64 {
        self.store.total_record_bytes
    }

    /// Exact immutable limits supplied when this writer was created or reopened.
    pub fn resource_limits(&self) -> L0ResourceLimitsV0 {
        self.store.limits
    }

    /// Whether a transaction-crossing failure has made deliberate reopen mandatory.
    pub fn is_poisoned(&self) -> bool {
        self.store.poisoned
    }

    /// Append one successor through an atomic expected-predecessor transaction.
    ///
    /// `Ok` means SQLite reported successful `COMMIT` under the selected local
    /// profile. An error does not universally mean nothing committed; an unknown
    /// outcome poisons and abandons the connection, requiring complete reopen.
    pub fn append(
        &mut self,
        provenance: Provenance,
        payload: Payload,
    ) -> Result<SignedEvent, LogError> {
        if self.store.poisoned {
            return Err(LogError::WriterPoisoned);
        }

        let next_count = self
            .next_seq
            .checked_add(1)
            .ok_or(LogError::SequenceExhausted)?;
        check_limit(
            "record count",
            next_count,
            self.store.limits.max_record_count,
        )?;
        let (event, record) = build_signed_record(
            &self.signing_key,
            self.next_seq,
            self.last_hash,
            (self.clock)(),
            provenance,
            payload,
        )?;
        let record_length = u64::try_from(record.len()).map_err(|_| LogError::ResourceLimit {
            resource: "individual record bytes",
            actual: u64::MAX,
            limit: self.store.limits.max_record_bytes,
        })?;
        check_limit(
            "individual record bytes",
            record_length,
            self.store.limits.max_record_bytes,
        )?;
        let next_total_record_bytes = self
            .store
            .total_record_bytes
            .checked_add(record_length)
            .ok_or(LogError::ResourceLimit {
                resource: "cumulative record bytes",
                actual: u64::MAX,
                limit: self.store.limits.max_total_record_bytes,
            })?;
        check_limit(
            "cumulative record bytes",
            next_total_record_bytes,
            self.store.limits.max_total_record_bytes,
        )?;

        let connection = self
            .store
            .connection
            .as_mut()
            .ok_or(LogError::WriterPoisoned)?;
        if let Err(error) = connection.execute_batch("BEGIN IMMEDIATE") {
            self.store.poisoned = true;
            let mapped = if is_busy(&error) {
                LogError::StorageBusy {
                    operation: "begin append transaction",
                }
            } else {
                sqlite_error("begin append transaction", error)
            };
            if connection.is_autocommit() {
                return Err(mapped);
            }
            if connection.execute_batch("ROLLBACK").is_ok() && connection.is_autocommit() {
                return Err(mapped);
            }
            self.store.connection.take();
            return Err(LogError::CommitStateUnknown);
        }

        let operation = (|| {
            let terminal: Option<(u64, ContentHash)> = {
                let mut statement = connection
                    .prepare(
                        "SELECT position_be, record_bytes FROM main.magpie_l0_records \
                         ORDER BY position_be DESC LIMIT 1",
                    )
                    .map_err(|error| sqlite_error("prepare persisted terminal", error))?;
                let mut rows = statement
                    .query([])
                    .map_err(|error| sqlite_error("query persisted terminal", error))?;
                let Some(row) = rows
                    .next()
                    .map_err(|error| sqlite_error("read persisted terminal", error))?
                else {
                    return Ok::<_, LogError>(None);
                };
                let position = decode_position(
                    row.get_ref(0)
                        .map_err(|error| sqlite_error("read terminal position", error))?,
                )?;
                let bytes = match row
                    .get_ref(1)
                    .map_err(|error| sqlite_error("read terminal record", error))?
                {
                    ValueRef::Blob(bytes) => bytes,
                    _ => {
                        return Err(LogError::UnsupportedDatabase {
                            detail: "terminal record is not a BLOB".into(),
                        });
                    }
                };
                let terminal_event: SignedEvent = serde_json::from_slice(bytes)?;
                if terminal_event.core.seq != position {
                    return Err(LogError::ChainBroken {
                        seq: terminal_event.core.seq,
                        detail: "persisted terminal coordinate is inconsistent".into(),
                    });
                }
                verify_event_integrity(&self.signing_key.verifying_key(), &terminal_event)?;
                Some((
                    position.checked_add(1).ok_or(LogError::SequenceExhausted)?,
                    terminal_event.hash,
                ))
            };
            if terminal != Some((self.next_seq, self.last_hash)) {
                return Err(LogError::WriterStale);
            }

            connection
                .execute(
                    "INSERT INTO main.magpie_l0_records(position_be, record_bytes) VALUES (?1, ?2)",
                    params![self.next_seq.to_be_bytes().as_slice(), record.as_slice()],
                )
                .map_err(|error| sqlite_error("insert successor", error))?;
            let logical_bytes = logical_database_bytes(connection)?;
            check_limit(
                "logical database bytes",
                logical_bytes,
                self.store.limits.max_database_bytes,
            )?;
            check_journal_file_limit(&self.store.path, self.store.limits)?;
            Ok(Some(()))
        })();

        if let Err(error) = operation {
            self.store.poisoned = true;
            let rollback_ok =
                connection.execute_batch("ROLLBACK").is_ok() && connection.is_autocommit();
            if !rollback_ok {
                self.store.connection.take();
                return Err(LogError::CommitStateUnknown);
            }
            return Err(error);
        }

        if let Err(error) = connection.execute_batch("COMMIT") {
            self.store.poisoned = true;
            if is_busy(&error)
                && !connection.is_autocommit()
                && connection.execute_batch("ROLLBACK").is_ok()
                && connection.is_autocommit()
            {
                return Err(LogError::StorageBusy {
                    operation: "commit append transaction",
                });
            }
            self.store.connection.take();
            return Err(LogError::CommitStateUnknown);
        }

        #[cfg(test)]
        if self.store.fault_after_commit_unknown {
            self.store.poisoned = true;
            self.store.connection.take();
            return Err(LogError::CommitStateUnknown);
        }

        self.next_seq = next_count;
        self.last_hash = event.hash;
        self.store.total_record_bytes = next_total_record_bytes;
        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> L0ResourceLimitsV0 {
        L0ResourceLimitsV0::new(16 * 1024 * 1024, 100, 1024 * 1024, 8 * 1024 * 1024).unwrap()
    }

    #[test]
    fn unknown_commit_fault_abandons_connection_and_poison_writer() {
        let path = std::env::temp_dir().join(format!(
            "magpie-sqlite-unknown-{}-{}.db",
            std::process::id(),
            system_clock()
        ));
        let key = SigningKey::from_bytes(&[91u8; 32]);
        let mut writer =
            LogWriter::<SqliteL0Store>::create_new(&path, key.clone(), limits()).unwrap();
        writer.store.fault_after_commit_unknown = true;
        let result = writer.append(
            Provenance::new("test", "unknown-commit"),
            Payload::Note {
                text: "committed but acknowledgement lost".into(),
            },
        );
        assert!(matches!(result, Err(LogError::CommitStateUnknown)));
        assert!(writer.is_poisoned());
        assert!(writer.store.connection.is_none());
        assert!(matches!(
            writer.append(
                Provenance::new("test", "poisoned"),
                Payload::Note {
                    text: "must not run SQL".into()
                },
            ),
            Err(LogError::WriterPoisoned)
        ));
        drop(writer);
        let reopened =
            LogWriter::<SqliteL0Store>::open_verified_prefix(&path, key, limits()).unwrap();
        assert_eq!(reopened.len(), 2);
        let _ = std::fs::remove_file(path);
    }
}
