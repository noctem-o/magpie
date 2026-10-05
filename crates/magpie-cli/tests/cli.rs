//! End-to-end tests that run the real `magpie` binary.
//!
//! Most tests are hostile: each refusal must leave the log byte-for-byte
//! unchanged, because a refused write that half-happened would be worse than
//! no refusal at all.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use magpie_claims::{
    produce_standing_receipt_v0, StandingReceiptCheckpointV0, StandingReceiptContextV0,
    StandingReceiptExpectationV0, StandingReceiptHistoryIdentityV0, StandingReceiptRequestV0,
    MAGPIE_CLAIMS_POLICY_V2_ID, STANDING_RECEIPT_HISTORY_PROFILE_V0,
};
use serde_json::Value;
use sha2::{Digest, Sha256};

static NEXT_DIR: AtomicU32 = AtomicU32::new(0);

/// A scratch area with one store and its own checkpoint directory.
struct Scratch {
    root: PathBuf,
}

impl Scratch {
    fn new() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let root = std::env::temp_dir().join(format!(
            "magpie-cli-test-{}-{}-{nanos}",
            std::process::id(),
            NEXT_DIR.fetch_add(1, Ordering::SeqCst)
        ));
        fs::create_dir_all(&root).unwrap();
        Self { root }
    }

    fn store(&self) -> PathBuf {
        self.root.join("store")
    }

    fn state(&self) -> PathBuf {
        self.root.join("state")
    }

    fn log(&self) -> PathBuf {
        self.store().join("log.jsonl")
    }

    fn command(&self, store: &Path, args: &[&str]) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_magpie"));
        command
            .args(args)
            .env("MAGPIE_STORE", store)
            .env("MAGPIE_STATE_DIR", self.state())
            .env_remove("XDG_STATE_HOME");
        command
    }

    fn run_in(&self, store: &Path, args: &[&str]) -> Output {
        self.command(store, args)
            .output()
            .expect("magpie binary runs")
    }

    fn run(&self, args: &[&str]) -> Output {
        self.run_in(&self.store(), args)
    }

    /// Run a command that must succeed and return its stdout.
    fn ok(&self, args: &[&str]) -> String {
        let output = self.run(args);
        assert!(
            output.status.success(),
            "`magpie {}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap()
    }

    fn json(&self, args: &[&str]) -> Value {
        serde_json::from_str(&self.ok(args)).expect("valid JSON output")
    }

    fn init(&self) {
        self.ok(&["init", self.store().to_str().unwrap(), "--agent", "tester"]);
    }

    fn log_bytes(&self) -> Vec<u8> {
        fs::read(self.log()).unwrap()
    }

    /// The verifying key recorded in the store's config.
    fn verifying_key(&self) -> String {
        let config: Value =
            serde_json::from_slice(&fs::read(self.store().join("config.json")).unwrap()).unwrap();
        config["verifying_key"].as_str().unwrap().to_owned()
    }

    /// Where the store's rollback checkpoint is saved.
    fn checkpoint_file(&self) -> PathBuf {
        self.state()
            .join("checkpoints")
            .join(format!("{}.json", self.verifying_key()))
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn code(output: &Output) -> i32 {
    output.status.code().expect("exited normally")
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// Copy a store directory, key and all, as a backup tool would.
fn copy_store(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        fs::copy(entry.path(), to.join(entry.file_name())).unwrap();
    }
}

/// A store with one claim, one evidence node with a hashed file, one link, and a note.
fn small_ledger(scratch: &Scratch) -> Vec<u8> {
    scratch.init();
    scratch.ok(&[
        "claim",
        "Theorem 2 holds",
        "--domain",
        "OperationalObservation",
        "--source",
        "manuscript",
    ]);
    let artifact = b"exponent agrees to 1e-15 for N=64..192\n".to_vec();
    let artifact_path = scratch.root.join("numerics.txt");
    fs::write(&artifact_path, &artifact).unwrap();
    scratch.ok(&[
        "evidence",
        "numerical check of Theorem 2",
        "--kind",
        "ExecutionEvidence",
        "--file",
        artifact_path.to_str().unwrap(),
    ]);
    scratch.ok(&[
        "link",
        "supports",
        "ev-2",
        "claim-1",
        "--rationale",
        "numerics",
    ]);
    scratch.ok(&["note", "F5 spectral lower bound is still open"]);
    artifact
}

#[test]
fn records_reads_and_exports_a_small_ledger() {
    let scratch = Scratch::new();
    let artifact = small_ledger(&scratch);

    let claim = scratch.json(&["show", "claim-1", "--json"]);
    assert_eq!(claim["type"], "claim");
    assert_eq!(claim["domain"], "OperationalObservation");
    assert_eq!(claim["actor_class"], "HumanRoot");
    assert_eq!(claim["recorded"]["agent"], "tester");
    assert_eq!(claim["recorded"]["source"], "manuscript");
    assert_eq!(claim["links_in"][0]["id"], "edge-3");

    let hits = scratch.json(&["search", "Theorem", "--json"]);
    assert_eq!(hits[0]["seq"], 1);

    let standing = scratch.json(&["standing", "claim-1", "--policy", "v0", "--json"]);
    assert_eq!(standing["policy"], "magpie-claims-standing-v0");
    assert_eq!(standing["governed_standing"], "Conjectured");

    let why = scratch.json(&["why", "claim-1", "--policy", "v2", "--json"]);
    assert_eq!(why["policy_id"], "magpie-claims-standing-v2");
    assert!(why.to_string().contains("edge-3"));

    let export = scratch.json(&["export", "--format", "desk-v0", "--policy", "v2"]);
    let verified = scratch.json(&["verify", "--json"]);
    assert_eq!(export["format"], "magpie-desk-export-v0");
    assert_eq!(export["policy"]["id"], "magpie-claims-standing-v2");
    assert_eq!(export["source"]["event_count"], 5);
    assert_eq!(export["source"]["log_tip"], verified["tip"]);
    let exported_claim = &export["claims"][0];
    assert_eq!(exported_claim["id"], "claim-1");
    assert_eq!(exported_claim["domain"], "OperationalObservation");
    assert_eq!(exported_claim["standing"]["status"], "Conjectured");
    assert_eq!(exported_claim["currentness"]["state"], "unknown");
    assert_eq!(exported_claim["labels"], serde_json::json!({}));
    assert_eq!(
        export["evidence"][0]["content_hash"],
        hex::encode(Sha256::digest(&artifact))
    );
    assert_eq!(export["edges"][0]["kind"], "supports");
    assert_eq!(
        export["notes"][0]["text"],
        "F5 spectral lower bound is still open"
    );
}

#[test]
fn verify_accepts_its_own_log_on_both_verifiers() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let report = scratch.json(&["verify", "--json"]);
    assert_eq!(report["ok"], true);
    assert_eq!(report["signatures"], "ok");
    assert_eq!(report["portable"], "accept");
    assert_eq!(report["event_count"], 5);
    assert!(report["checkpoint"]
        .as_str()
        .unwrap()
        .starts_with("contained"));
}

#[test]
fn tampered_log_fails_verification_and_blocks_reads() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let log = String::from_utf8(scratch.log_bytes()).unwrap();
    fs::write(
        scratch.log(),
        log.replace("Theorem 2 holds", "Theorem 3 holds"),
    )
    .unwrap();

    let output = scratch.run(&["verify", "--json"]);
    assert_eq!(code(&output), 1);
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["ok"], false);
    assert_eq!(report["portable"], "reject");
    assert_ne!(report["signatures"], "ok");

    assert_eq!(code(&scratch.run(&["log"])), 1);
    assert_eq!(code(&scratch.run(&["note", "after tamper"])), 1);
}

#[test]
fn wrong_verifying_key_fails_verification() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let other_store = scratch.root.join("other");
    let created = scratch.run_in(
        &other_store,
        &["init", other_store.to_str().unwrap(), "--json"],
    );
    assert!(created.status.success());
    let other_key = serde_json::from_slice::<Value>(&created.stdout).unwrap()["verifying_key"]
        .as_str()
        .unwrap()
        .to_owned();

    let output = scratch.run(&["verify", "--verifying-key", &other_key, "--json"]);
    assert_eq!(code(&output), 1);
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["ok"], false);
    assert_eq!(report["portable"], "reject");
}

#[test]
fn rolled_back_log_is_refused_until_accepted() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let older = scratch.log_bytes();
    scratch.ok(&["note", "written after the backup"]);
    fs::write(scratch.log(), &older).unwrap();

    let refused = scratch.run(&["note", "written to the restored copy"]);
    assert_eq!(code(&refused), 1);
    assert!(stderr(&refused).contains("does not contain the checkpoint"));
    assert_eq!(
        scratch.log_bytes(),
        older,
        "a refused write must not append"
    );
    assert_eq!(code(&scratch.run(&["show", "claim-1"])), 1);
    assert_eq!(code(&scratch.run(&["verify"])), 1);

    scratch.ok(&["checkpoint", "--accept-current"]);
    scratch.ok(&["note", "written after accepting the restored copy"]);
    scratch.ok(&["verify"]);
}

#[test]
fn torn_final_record_blocks_appends() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let mut torn = scratch.log_bytes();
    torn.pop();
    fs::write(scratch.log(), &torn).unwrap();

    let refused = scratch.run(&["note", "must not be glued onto the torn record"]);
    assert_eq!(code(&refused), 1);
    assert!(stderr(&refused).contains("part-way through a record"));
    assert_eq!(scratch.log_bytes(), torn);
}

#[test]
fn blank_physical_record_blocks_reads_and_writes() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    // A blank line between two events: the record reader skips it, but the
    // portable language rejects the history.
    let log = scratch.log_bytes();
    let first_end = log.iter().position(|byte| *byte == b'\n').unwrap() + 1;
    let mut blank = log[..first_end].to_vec();
    blank.push(b'\n');
    blank.extend_from_slice(&log[first_end..]);
    fs::write(scratch.log(), &blank).unwrap();
    let checkpoint = fs::read(scratch.checkpoint_file()).unwrap();

    for args in [
        vec!["note", "must not extend a portable-invalid log"],
        vec!["checkpoint", "--accept-current"],
        vec!["log"],
        vec!["show", "claim-1"],
    ] {
        let output = scratch.run(&args);
        assert_eq!(code(&output), 1, "{args:?}: {}", stderr(&output));
        assert!(
            stderr(&output).contains("portable verifier rejects"),
            "{args:?}: {}",
            stderr(&output)
        );
    }
    assert_eq!(
        scratch.log_bytes(),
        blank,
        "a refused command must not write"
    );
    assert_eq!(fs::read(scratch.checkpoint_file()).unwrap(), checkpoint);

    let output = scratch.run(&["verify", "--json"]);
    assert_eq!(code(&output), 1);
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        report["signatures"], "ok",
        "the record reader alone accepts it"
    );
    assert_eq!(report["portable"], "reject");
}

#[test]
fn external_key_verifies_a_store_whose_config_is_damaged() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let key = scratch.verifying_key();
    let config = scratch.store().join("config.json");

    fs::write(&config, "{ not json").unwrap();
    assert_eq!(code(&scratch.run(&["verify"])), 1);
    let report = scratch.json(&["verify", "--verifying-key", &key, "--json"]);
    assert_eq!(report["ok"], true);
    assert_eq!(report["signatures"], "ok");
    assert_eq!(report["portable"], "accept");
    assert!(report["checkpoint"]
        .as_str()
        .unwrap()
        .starts_with("contained"));

    fs::remove_file(&config).unwrap();
    let report = scratch.json(&["verify", "--verifying-key", &key, "--json"]);
    assert_eq!(report["ok"], true);

    let elsewhere = scratch.root.join("not-a-store");
    fs::create_dir_all(&elsewhere).unwrap();
    let output = scratch.run_in(&elsewhere, &["verify", "--verifying-key", &key]);
    assert_eq!(code(&output), 2, "{}", stderr(&output));
    assert!(
        fs::read_dir(&elsewhere).unwrap().next().is_none(),
        "nothing may be created outside a store"
    );
}

#[test]
fn copies_of_a_store_cannot_fork_its_history() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let copy = scratch.root.join("copy");
    copy_store(&scratch.store(), &copy);
    let copy_log = copy.join("log.jsonl");
    let before = fs::read(&copy_log).unwrap();

    // A command in the original holds the key's checkpoint lock. The copy has
    // its own store lock, so only the key-scoped lock can stop it.
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(scratch.checkpoint_file().with_extension("lock"))
        .unwrap();
    lock.try_lock().unwrap();
    let busy = scratch.run_in(&copy, &["note", "from the copy"]);
    assert_eq!(code(&busy), 1, "{}", stderr(&busy));
    assert!(stderr(&busy).contains("this key's checkpoint"));
    assert_eq!(fs::read(&copy_log).unwrap(), before);
    drop(lock);

    // Once one copy extends the checkpoint, the other is a fork.
    let written = scratch.run_in(&copy, &["note", "from the copy"]);
    assert!(written.status.success(), "{}", stderr(&written));
    let original = scratch.log_bytes();
    let forked = scratch.run(&["note", "from the original"]);
    assert_eq!(code(&forked), 1);
    assert!(stderr(&forked).contains("does not contain the checkpoint"));
    assert_eq!(scratch.log_bytes(), original);
}

#[test]
fn failure_after_an_append_says_the_event_was_recorded() {
    let scratch = Scratch::new();
    scratch.init();
    scratch.ok(&["note", "first"]);
    // A directory where the checkpoint's temporary file goes makes the save
    // fail after the note is already in the log.
    let blocker = scratch.checkpoint_file().with_extension("json.tmp");
    fs::create_dir_all(&blocker).unwrap();

    let output = scratch.run(&["note", "second"]);
    assert_eq!(code(&output), 3, "{}", stderr(&output));
    assert!(
        stderr(&output).contains("recorded seq 2"),
        "{}",
        stderr(&output)
    );
    assert!(stderr(&output).contains("Don't retry"));
    let events = scratch.json(&["log", "--json"]);
    assert_eq!(events[2]["subject"], "second");

    fs::remove_dir(&blocker).unwrap();
    scratch.ok(&["checkpoint", "--accept-current"]);
    scratch.ok(&["note", "third"]);
    scratch.ok(&["verify"]);
}

#[cfg(target_os = "linux")]
#[test]
fn output_failure_after_a_write_says_the_event_was_recorded() {
    let scratch = Scratch::new();
    scratch.init();
    // Every write to /dev/full fails, as a closed pipe or a full disk would,
    // but only after the note and its checkpoint are committed.
    let full = fs::OpenOptions::new()
        .write(true)
        .open("/dev/full")
        .unwrap();
    let output = scratch
        .command(&scratch.store(), &["note", "printed nowhere"])
        .stdout(full)
        .output()
        .expect("magpie binary runs");
    assert_eq!(code(&output), 3, "{}", stderr(&output));
    assert!(
        stderr(&output).contains("recorded seq 1"),
        "{}",
        stderr(&output)
    );
    assert!(stderr(&output).contains("Don't retry"));
    let events = scratch.json(&["log", "--json"]);
    assert_eq!(events[1]["subject"], "printed nowhere");
    scratch.ok(&["verify"]);
}

#[test]
fn empty_log_is_refused_and_fails_verification() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let key = scratch.verifying_key();
    // Truncated, and the checkpoint lost too. Both verifiers accept an empty
    // history, so only the genesis requirement catches this.
    fs::write(scratch.log(), b"").unwrap();
    fs::remove_dir_all(scratch.state()).unwrap();

    for args in [
        vec!["verify", "--json"],
        vec!["verify", "--verifying-key", key.as_str(), "--json"],
    ] {
        let output = scratch.run(&args);
        assert_eq!(code(&output), 1, "{args:?}: {}", stderr(&output));
        let report: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["ok"], false, "{args:?}");
        assert!(
            report["signatures"].as_str().unwrap().contains("empty"),
            "{args:?}: {report}"
        );
    }
    for args in [
        vec!["log"],
        vec!["note", "must not become the first event"],
        vec!["checkpoint", "--accept-current"],
    ] {
        let output = scratch.run(&args);
        assert_eq!(code(&output), 1, "{args:?}: {}", stderr(&output));
        assert!(
            stderr(&output).contains("is empty"),
            "{args:?}: {}",
            stderr(&output)
        );
    }
    assert!(scratch.log_bytes().is_empty());
}

#[test]
fn writes_refuse_an_empty_agent_name() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let config_path = scratch.store().join("config.json");
    let mut config: Value = serde_json::from_slice(&fs::read(&config_path).unwrap()).unwrap();
    config["agent"] = Value::String("  ".into());
    fs::write(&config_path, serde_json::to_vec_pretty(&config).unwrap()).unwrap();
    let before = scratch.log_bytes();

    let refused = scratch.run(&["note", "written by nobody"]);
    assert_eq!(code(&refused), 1, "{}", stderr(&refused));
    assert!(stderr(&refused).contains("empty agent name"));
    assert_eq!(scratch.log_bytes(), before);
    scratch.ok(&["log"]);
}

#[test]
fn reads_never_write_to_the_store_directory() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let key = scratch.verifying_key();
    // With its lock file gone, a read that wrote here would recreate it.
    fs::remove_file(scratch.store().join("lock")).unwrap();
    let listing = |dir: &Path| {
        let mut names: Vec<String> = fs::read_dir(dir)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        names
    };
    let before = listing(&scratch.store());
    for args in [
        vec!["log"],
        vec!["show", "claim-1"],
        vec!["search", "Theorem"],
        vec!["standing", "claim-1", "--policy", "v2"],
        vec!["checkpoint"],
        vec!["verify"],
        vec!["verify", "--verifying-key", key.as_str()],
    ] {
        scratch.ok(&args);
        assert_eq!(listing(&scratch.store()), before, "{args:?}");
    }
    assert!(
        listing(&scratch.state().join("scratch")).is_empty(),
        "scratch copies must be removed"
    );
}

/// Root ignores directory permissions, so this bites only when the tests run
/// as an ordinary user, as they do in CI.
#[cfg(unix)]
#[test]
fn a_read_only_store_can_still_be_read_and_verified() {
    use std::os::unix::fs::PermissionsExt;
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let key = scratch.verifying_key();
    let store = scratch.store();
    fs::set_permissions(&store, fs::Permissions::from_mode(0o555)).unwrap();
    let outputs: Vec<Output> = [
        vec!["log"],
        vec!["verify"],
        vec!["verify", "--verifying-key", key.as_str()],
    ]
    .iter()
    .map(|args| scratch.run(args))
    .collect();
    // Restore before asserting, so a failure still lets the scratch area go.
    fs::set_permissions(&store, fs::Permissions::from_mode(0o755)).unwrap();
    for output in &outputs {
        assert!(output.status.success(), "{}", stderr(output));
    }
}

#[cfg(unix)]
#[test]
fn init_refuses_a_dangling_link_where_a_store_file_goes() {
    let scratch = Scratch::new();
    let store = scratch.store();
    fs::create_dir_all(&store).unwrap();
    let elsewhere = scratch.root.join("elsewhere");
    std::os::unix::fs::symlink(elsewhere.join("log.jsonl"), store.join("log.jsonl")).unwrap();

    let output = scratch.run(&["init", store.to_str().unwrap()]);
    assert_eq!(code(&output), 2, "{}", stderr(&output));
    assert!(
        !elsewhere.exists(),
        "nothing may be written through the link"
    );
    assert!(!store.join("signing.key").exists());
}

#[test]
fn unknown_vocabulary_is_rejected_before_anything_is_appended() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let before = scratch.log_bytes();
    for args in [
        vec!["claim", "x", "--domain", "Mathematics"],
        vec!["evidence", "y", "--kind", "Vibes"],
        vec!["link", "endorses", "ev-2", "claim-1", "--rationale", "r"],
        vec![
            "evidence",
            "z",
            "--kind",
            "ExecutionEvidence",
            "--source-uri",
            "https://example.org",
        ],
    ] {
        let output = scratch.run(&args);
        assert_eq!(code(&output), 2, "{args:?}: {}", stderr(&output));
    }
    assert_eq!(scratch.log_bytes(), before);
}

#[test]
fn duplicate_and_dangling_ids_are_rejected() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let before = scratch.log_bytes();
    let duplicate = scratch.run(&[
        "claim",
        "again",
        "--domain",
        "HumanJudgment",
        "--id",
        "claim-1",
    ]);
    assert_eq!(code(&duplicate), 2);
    let dangling = scratch.run(&["link", "supports", "ev-404", "claim-1", "--rationale", "r"]);
    assert_eq!(code(&dangling), 2);
    let odd_id = scratch.run(&["note", "x", "--id", "has space"]);
    assert_eq!(code(&odd_id), 2, "--id does not apply to notes");
    assert_eq!(scratch.log_bytes(), before);
}

#[test]
fn standing_and_export_require_an_explicit_policy() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    for args in [
        vec!["standing", "claim-1"],
        vec!["why", "claim-1"],
        vec!["export", "--format", "desk-v0"],
        vec!["standing", "claim-1", "--policy", "latest"],
    ] {
        let output = scratch.run(&args);
        assert_eq!(code(&output), 2, "{args:?}");
        assert!(stderr(&output).contains("--policy"), "{args:?}");
    }
}

#[test]
fn missing_checkpoint_blocks_writes_but_not_reads() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    fs::remove_dir_all(scratch.state()).unwrap();
    let before = scratch.log_bytes();

    let refused = scratch.run(&["note", "no checkpoint yet"]);
    assert_eq!(code(&refused), 1);
    assert_eq!(scratch.log_bytes(), before);

    let read = scratch.run(&["log"]);
    assert!(read.status.success());
    assert!(stderr(&read).contains("no checkpoint is saved"));

    scratch.ok(&["checkpoint", "--accept-current"]);
    scratch.ok(&["note", "checkpoint accepted"]);
}

#[test]
fn init_refuses_an_existing_store() {
    let scratch = Scratch::new();
    scratch.init();
    let again = scratch.run(&["init", scratch.store().to_str().unwrap()]);
    assert_eq!(code(&again), 2);
}

#[test]
fn init_without_a_state_directory_leaves_nothing_behind() {
    let scratch = Scratch::new();
    let store = scratch.store();
    let output = Command::new(env!("CARGO_BIN_EXE_magpie"))
        .args(["init", store.to_str().unwrap()])
        .env_remove("MAGPIE_STATE_DIR")
        .env_remove("XDG_STATE_HOME")
        .env_remove("HOME")
        .env_remove("LOCALAPPDATA")
        .output()
        .expect("magpie binary runs");
    assert_eq!(code(&output), 2, "{}", stderr(&output));
    for name in ["log.jsonl", "signing.key", "config.json"] {
        assert!(!store.join(name).exists(), "{name} would block a retry");
    }
    scratch.init();
}

#[test]
fn external_source_evidence_records_its_locators() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    scratch.ok(&[
        "evidence",
        "reported in the published erratum",
        "--kind",
        "ExternalSource",
        "--source-uri",
        "https://example.org/erratum",
        "--locator",
        "p. 2",
        "--id",
        "erratum",
    ]);
    let shown = scratch.json(&["show", "erratum", "--json"]);
    assert_eq!(
        shown["metadata"]["source_uri"],
        "https://example.org/erratum"
    );
    assert_eq!(shown["metadata"]["source_locator"], "p. 2");
}

#[cfg(unix)]
#[test]
fn signing_key_is_readable_only_by_its_owner() {
    use std::os::unix::fs::PermissionsExt;
    let scratch = Scratch::new();
    scratch.init();
    let mode = fs::metadata(scratch.store().join("signing.key"))
        .unwrap()
        .permissions()
        .mode();
    assert_eq!(mode & 0o777, 0o600);
}

#[test]
fn search_refuses_a_timestamp_its_index_cannot_store() {
    use magpie_log::{FileStore, LogWriter, Payload, Provenance, SigningKey};
    let scratch = Scratch::new();
    scratch.init();
    // A validly signed event from a writer whose clock reads u64::MAX, past
    // the signed 64-bit integers the search index stores.
    let key_text = fs::read_to_string(scratch.store().join("signing.key")).unwrap();
    let key: [u8; 32] = hex::decode(key_text.trim()).unwrap().try_into().unwrap();
    let mut writer = LogWriter::<FileStore>::open_with_clock(
        FileStore::new(scratch.log()),
        SigningKey::from_bytes(&key),
        Box::new(|| u64::MAX),
    )
    .unwrap();
    writer
        .append(
            Provenance::new("elsewhere", "test"),
            Payload::Note {
                text: "from the far future".into(),
            },
        )
        .unwrap();
    drop(writer);

    scratch.ok(&["verify"]);
    scratch.ok(&["log"]);
    let output = scratch.run(&["search", "future"]);
    assert_eq!(code(&output), 1, "{}", stderr(&output));
    assert!(
        stderr(&output).contains("search index"),
        "{}",
        stderr(&output)
    );
}

#[cfg(target_os = "linux")]
#[test]
fn reads_fall_back_when_the_scratch_directory_refuses_files() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    // /proc is a directory in which no one, not even root, can create a file:
    // the state directory's scratch area exists but can't take the copy.
    let scratch_dir = scratch.state().join("scratch");
    let _ = fs::remove_dir_all(&scratch_dir);
    std::os::unix::fs::symlink("/proc", &scratch_dir).unwrap();
    scratch.ok(&["log"]);
    scratch.ok(&["verify"]);
}

#[cfg(unix)]
#[test]
fn a_link_planted_at_the_checkpoint_temporary_path_is_not_followed() {
    let scratch = Scratch::new();
    scratch.init();
    // Someone who can write the state directory plants a link where the next
    // checkpoint save writes its temporary file, aiming at a file of ours.
    let victim = scratch.root.join("victim.txt");
    fs::write(&victim, "precious\n").unwrap();
    let temporary = scratch.checkpoint_file().with_extension("json.tmp");
    std::os::unix::fs::symlink(&victim, &temporary).unwrap();

    scratch.ok(&["note", "saved past the planted link"]);
    assert_eq!(fs::read_to_string(&victim).unwrap(), "precious\n");
    let report = scratch.json(&["verify", "--json"]);
    assert_eq!(report["event_count"], 2);
    assert!(report["checkpoint"]
        .as_str()
        .unwrap()
        .starts_with("contained (saved at event 2)"));
}

#[test]
fn relative_store_and_state_paths_work() {
    let scratch = Scratch::new();
    // Bare relative names, whose directory entries live in the working
    // directory rather than under any named parent.
    let run = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_magpie"))
            .args(args)
            .current_dir(&scratch.root)
            .env("MAGPIE_STORE", "ledger")
            .env("MAGPIE_STATE_DIR", "state")
            .env_remove("XDG_STATE_HOME")
            .output()
            .expect("magpie binary runs")
    };
    for args in [
        vec!["init", "ledger", "--agent", "tester"],
        vec!["note", "written through relative paths"],
        vec!["checkpoint", "--accept-current"],
        vec!["verify"],
    ] {
        let output = run(&args);
        assert!(output.status.success(), "{args:?}: {}", stderr(&output));
    }
    assert!(scratch.root.join("ledger").join("log.jsonl").is_file());
    assert!(scratch.root.join("state").join("checkpoints").is_dir());
}

#[test]
fn a_state_directory_inside_the_store_is_refused() {
    let scratch = Scratch::new();
    let store = scratch.store();
    let inside = store.join("state");
    let run = |state: &Path, args: &[&str]| -> Output {
        Command::new(env!("CARGO_BIN_EXE_magpie"))
            .args(args)
            .env("MAGPIE_STORE", &store)
            .env("MAGPIE_STATE_DIR", state)
            .env_remove("XDG_STATE_HOME")
            .output()
            .expect("magpie binary runs")
    };
    // A checkpoint kept inside the store would be restored along with it.
    let refused = run(&inside, &["init", store.to_str().unwrap()]);
    assert_eq!(code(&refused), 2, "{}", stderr(&refused));
    assert!(stderr(&refused).contains("inside the store"));
    assert!(!store.join("log.jsonl").exists());

    // A store whose state was kept apart, later pointed at a state directory
    // inside it: writes and reads refuse, and verify fails its checkpoint row.
    small_ledger(&scratch);
    let before = scratch.log_bytes();
    for args in [vec!["note", "must not be written"], vec!["log"]] {
        let output = run(&inside, &args);
        assert_eq!(code(&output), 2, "{args:?}: {}", stderr(&output));
    }
    let verify = run(&inside, &["verify", "--json"]);
    assert_eq!(code(&verify), 1, "{}", stderr(&verify));
    let report: Value = serde_json::from_slice(&verify.stdout).unwrap();
    assert!(report["checkpoint"]
        .as_str()
        .unwrap()
        .contains("inside the store"));
    #[cfg(unix)]
    {
        let link = scratch.root.join("link-to-store");
        std::os::unix::fs::symlink(&store, &link).unwrap();
        let through_link = run(&link.join("state"), &["note", "must not be written"]);
        assert_eq!(code(&through_link), 2, "{}", stderr(&through_link));
    }
    assert!(!inside.exists(), "nothing may be created inside the store");
    assert_eq!(scratch.log_bytes(), before);
}

#[test]
fn show_reports_metadata_that_is_not_a_json_object_as_recorded() {
    use magpie_log::{FileStore, LogWriter, Payload, Provenance, SigningKey};
    let scratch = Scratch::new();
    scratch.init();
    // Another conforming writer may record any text as metadata; this one
    // repeats a key, which the standing projection refuses to read.
    let raw = r#"{"source_uri":"https://a.example","source_uri":"https://b.example"}"#;
    let key_text = fs::read_to_string(scratch.store().join("signing.key")).unwrap();
    let key: [u8; 32] = hex::decode(key_text.trim()).unwrap().try_into().unwrap();
    let mut writer =
        LogWriter::<FileStore>::open(FileStore::new(scratch.log()), SigningKey::from_bytes(&key))
            .unwrap();
    writer
        .append(
            Provenance::new("elsewhere", "test"),
            Payload::EvidenceRegistered {
                evidence_id: "ev-odd".into(),
                evidence_kind: "ExternalSource".into(),
                summary: "recorded by another writer".into(),
                scope_ref: "scope:default".into(),
                actor_class: "HumanRoot".into(),
                content_hash: String::new(),
                metadata_json: raw.into(),
            },
        )
        .unwrap();
    drop(writer);

    let shown = scratch.json(&["show", "ev-odd", "--json"]);
    assert_eq!(shown["metadata"], Value::Null);
    assert_eq!(shown["metadata_json"], raw);
    let human = scratch.ok(&["show", "ev-odd"]);
    assert!(
        human.contains("not a JSON object with unique keys"),
        "{human}"
    );
    assert!(human.contains("https://b.example"), "{human}");
}

#[cfg(unix)]
#[test]
fn a_state_directory_holding_the_real_log_is_refused() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let before = scratch.log_bytes();
    // Move the signed log into the state directory and link it back: a
    // restore of the state directory would roll back log and checkpoint alike.
    let moved = scratch.state().join("moved").join("log.jsonl");
    fs::create_dir_all(moved.parent().unwrap()).unwrap();
    fs::rename(scratch.log(), &moved).unwrap();
    std::os::unix::fs::symlink(&moved, scratch.log()).unwrap();
    for args in [vec!["note", "must not be written"], vec!["log"]] {
        let output = scratch.run(&args);
        assert_eq!(code(&output), 2, "{args:?}: {}", stderr(&output));
        assert!(
            stderr(&output).contains("holds the log"),
            "{args:?}: {}",
            stderr(&output)
        );
    }
    assert_eq!(fs::read(&moved).unwrap(), before);

    // The same log in a sibling directory, with the state directory beside it.
    let data = scratch.root.join("data");
    fs::create_dir_all(&data).unwrap();
    fs::remove_file(scratch.log()).unwrap();
    fs::rename(&moved, data.join("log.jsonl")).unwrap();
    std::os::unix::fs::symlink(data.join("log.jsonl"), scratch.log()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_magpie"))
        .args(["note", "must not be written"])
        .env("MAGPIE_STORE", scratch.store())
        .env("MAGPIE_STATE_DIR", data.join("state"))
        .env_remove("XDG_STATE_HOME")
        .output()
        .expect("magpie binary runs");
    assert_eq!(code(&output), 2, "{}", stderr(&output));
    assert!(!data.join("state").exists());
    assert_eq!(fs::read(data.join("log.jsonl")).unwrap(), before);
}

#[test]
fn a_value_option_does_not_swallow_the_next_option() {
    let scratch = Scratch::new();
    let refused = scratch.run(&[
        "init",
        scratch.store().to_str().unwrap(),
        "--agent",
        "--json",
    ]);
    assert_eq!(code(&refused), 2, "{}", stderr(&refused));
    assert!(stderr(&refused).contains("--agent needs a value"));
    assert!(!scratch.log().exists());

    scratch.init();
    let before = scratch.log_bytes();
    let refused = scratch.run(&["note", "sourced", "--source", "--json"]);
    assert_eq!(code(&refused), 2, "{}", stderr(&refused));
    assert_eq!(scratch.log_bytes(), before);

    // A value that really starts with -- is still possible, written inline.
    scratch.ok(&["note", "sourced", "--source=--json"]);
    let events = scratch.json(&["log", "--json"]);
    assert_eq!(events[1]["source"], "--json");
}

#[test]
fn show_exposes_link_metadata() {
    use magpie_log::{FileStore, LogWriter, Payload, Provenance, SigningKey};
    let scratch = Scratch::new();
    small_ledger(&scratch);
    // Another conforming writer may attach metadata to a link; this CLI
    // writes `{}`, but must not drop what others recorded.
    let key_text = fs::read_to_string(scratch.store().join("signing.key")).unwrap();
    let key: [u8; 32] = hex::decode(key_text.trim()).unwrap().try_into().unwrap();
    let mut writer =
        LogWriter::<FileStore>::open(FileStore::new(scratch.log()), SigningKey::from_bytes(&key))
            .unwrap();
    writer
        .append(
            Provenance::new("elsewhere", "test"),
            Payload::JustificationEdgeRecorded {
                edge_id: "edge-meta".into(),
                edge_kind: "supports".into(),
                source_id: "ev-2".into(),
                target_id: "claim-1".into(),
                scope_ref: "scope:default".into(),
                actor_class: "HumanRoot".into(),
                rationale: "recorded elsewhere".into(),
                metadata_json: r#"{"reviewed_by":"a second reader"}"#.into(),
            },
        )
        .unwrap();
    drop(writer);

    let shown = scratch.json(&["show", "edge-meta", "--json"]);
    assert_eq!(shown["metadata"]["reviewed_by"], "a second reader");
    assert_eq!(
        shown["metadata_json"],
        r#"{"reviewed_by":"a second reader"}"#
    );
    let human = scratch.ok(&["show", "edge-meta"]);
    assert!(human.contains("reviewed_by: a second reader"), "{human}");
}

#[test]
fn search_reports_the_record_each_hit_belongs_to() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let hits = scratch.json(&["search", "Theorem", "--json"]);
    let ids: Vec<&Value> = hits
        .as_array()
        .unwrap()
        .iter()
        .map(|hit| &hit["id"])
        .collect();
    assert!(ids.contains(&&Value::from("claim-1")), "{hits}");
    assert!(ids.contains(&&Value::from("ev-2")), "{hits}");
    let note = scratch.json(&["search", "spectral", "--json"]);
    assert_eq!(note[0]["kind"], "note");
    assert_eq!(note[0]["id"], Value::Null);
    let human = scratch.ok(&["search", "numerical"]);
    assert!(
        human.contains("ev-2: numerical check of Theorem 2"),
        "{human}"
    );
}

/// Independent explicit request, never selected from the detached receipt.
fn receipt_request(
    scratch: &Scratch,
    expectation: StandingReceiptExpectationV0,
) -> StandingReceiptRequestV0 {
    StandingReceiptRequestV0 {
        context: StandingReceiptContextV0 {
            profile: STANDING_RECEIPT_HISTORY_PROFILE_V0.into(),
            history: StandingReceiptHistoryIdentityV0::of(&scratch.log_bytes()).unwrap(),
            verifying_key: scratch.verifying_key(),
            expectation,
            claim_id: "claim-1".into(),
        },
        policy_id: MAGPIE_CLAIMS_POLICY_V2_ID.into(),
    }
}

fn receipt_args(
    scratch: &Scratch,
    request: &StandingReceiptRequestV0,
    detached: Option<&Path>,
) -> Vec<String> {
    let context = &request.context;
    let mut args: Vec<String> = [
        if detached.is_some() {
            "check-standing-receipt"
        } else {
            "standing-receipt"
        },
        &context.claim_id,
        "--history",
        scratch.log().to_str().unwrap(),
        "--verifying-key",
        &context.verifying_key,
        "--profile",
        &context.profile,
        "--policy",
        &request.policy_id,
        "--expectation",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    let (relation, checkpoint) = match &context.expectation {
        StandingReceiptExpectationV0::None => ("none", None),
        StandingReceiptExpectationV0::Exact { checkpoint } => ("exact", Some(checkpoint)),
        StandingReceiptExpectationV0::ContainsCheckpoint { checkpoint } => {
            ("contains-checkpoint", Some(checkpoint))
        }
    };
    args.push(relation.into());
    if let Some(checkpoint) = checkpoint {
        args.extend([
            "--checkpoint-event-count".into(),
            checkpoint.event_count.to_string(),
            "--checkpoint-sha256".into(),
            checkpoint.commitment_sha256.to_hex(),
        ]);
    }
    if let Some(path) = detached {
        args.extend(["--receipt".into(), path.to_str().unwrap().into()]);
    }
    args
}

/// No ambient store/state by default, and no process-global environment mutation.
fn receipt_command(args: &[String]) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_magpie"));
    command
        .args(args)
        .env_remove("MAGPIE_STORE")
        .env_remove("MAGPIE_STATE_DIR")
        .env_remove("XDG_STATE_HOME");
    command
}

fn receipt_run(args: &[String]) -> Output {
    receipt_command(args).output().unwrap()
}

fn assert_receipt_failure(output: &Output, expected_code: i32) {
    assert_eq!(code(output), expected_code, "{}", stderr(output));
    assert!(output.stdout.is_empty(), "failure must emit no receipt");
    assert!(stderr(output).starts_with("magpie: "));
}

fn set_option(args: &mut [String], name: &str, value: &str) {
    let index = args.iter().position(|arg| arg == name).unwrap();
    args[index + 1] = value.into();
}

#[test]
fn receipt_cli_is_byte_identical_to_library_and_check_has_no_newline() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let request = receipt_request(&scratch, StandingReceiptExpectationV0::None);
    let bytes = scratch.log_bytes();
    let library = produce_standing_receipt_v0(&request, &bytes).unwrap();
    let mut args = receipt_args(&scratch, &request, None);
    let output = receipt_run(&args);
    assert_eq!(code(&output), 0, "{}", stderr(&output));
    assert!(output.stderr.is_empty());
    assert_eq!(output.stdout, library.canonical_bytes());
    assert_eq!(output.stdout.last(), Some(&b'}'));
    args.push("--json".into());
    let json = receipt_run(&args);
    assert_eq!(code(&json), 0);
    assert!(json.stderr.is_empty());
    assert_eq!(json.stdout, output.stdout);
    let detached = scratch.root.join("receipt.json");
    fs::write(&detached, &output.stdout).unwrap();
    let mut check_args = receipt_args(&scratch, &request, Some(&detached));
    for json in [false, true] {
        if json {
            check_args.push("--json".into());
        }
        let checked = receipt_run(&check_args);
        assert_eq!(code(&checked), 0, "{}", stderr(&checked));
        assert!(checked.stderr.is_empty());
        assert_eq!(checked.stdout, output.stdout);
    }
    // Both an added newline and another changed byte must fail raw matching.
    let mut altered = output.stdout.clone();
    altered.push(b'\n');
    fs::write(&detached, &altered).unwrap();
    assert_receipt_failure(&receipt_run(&check_args), 1);
    altered = output.stdout.clone();
    altered[0] = b'[';
    fs::write(&detached, &altered).unwrap();
    assert_receipt_failure(&receipt_run(&check_args), 1);
    // Checking coordinates come from arguments, not the otherwise valid receipt.
    fs::write(&detached, &output.stdout).unwrap();
    set_option(&mut check_args, "--expectation", "exact");
    let checkpoint = &library.wire().verified_prefix;
    check_args.extend([
        "--checkpoint-event-count".into(),
        checkpoint.event_count.to_string(),
        "--checkpoint-sha256".into(),
        checkpoint.tip_sha256.to_hex(),
    ]);
    assert_receipt_failure(&receipt_run(&check_args), 1);
}

#[test]
fn receipt_cli_explicit_expectations_delegate_and_bind_entire_suffix() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let base = receipt_request(&scratch, StandingReceiptExpectationV0::None);
    let library = produce_standing_receipt_v0(&base, &scratch.log_bytes()).unwrap();
    let checkpoint = StandingReceiptCheckpointV0 {
        event_count: library.wire().verified_prefix.event_count,
        commitment_sha256: library.wire().verified_prefix.tip_sha256,
    };
    for expectation in [
        StandingReceiptExpectationV0::Exact {
            checkpoint: checkpoint.clone(),
        },
        StandingReceiptExpectationV0::ContainsCheckpoint {
            checkpoint: checkpoint.clone(),
        },
    ] {
        let request = receipt_request(&scratch, expectation);
        let expected = produce_standing_receipt_v0(&request, &scratch.log_bytes()).unwrap();
        let output = receipt_run(&receipt_args(&scratch, &request, None));
        assert_eq!(code(&output), 0, "{}", stderr(&output));
        assert!(output.stderr.is_empty());
        assert_eq!(output.stdout, expected.canonical_bytes());
    }
    let original = scratch.log_bytes();
    scratch.ok(&["note", "verified suffix"]);
    let request = receipt_request(
        &scratch,
        StandingReceiptExpectationV0::ContainsCheckpoint {
            checkpoint: checkpoint.clone(),
        },
    );
    let expected = produce_standing_receipt_v0(&request, &scratch.log_bytes()).unwrap();
    let output = receipt_run(&receipt_args(&scratch, &request, None));
    assert_eq!(code(&output), 0);
    assert!(output.stderr.is_empty());
    assert_eq!(output.stdout, expected.canonical_bytes());
    assert_eq!(
        expected.wire().verified_prefix.event_count,
        checkpoint.event_count + 1
    );
    let exact = receipt_request(
        &scratch,
        StandingReceiptExpectationV0::Exact {
            checkpoint: checkpoint.clone(),
        },
    );
    assert_receipt_failure(&receipt_run(&receipt_args(&scratch, &exact, None)), 1);
    // A valid truncated image is not allowed to meet the later checkpoint.
    fs::write(scratch.log(), original).unwrap();
    let later = StandingReceiptCheckpointV0 {
        event_count: expected.wire().verified_prefix.event_count,
        commitment_sha256: expected.wire().verified_prefix.tip_sha256,
    };
    let request = receipt_request(
        &scratch,
        StandingReceiptExpectationV0::ContainsCheckpoint { checkpoint: later },
    );
    assert_receipt_failure(&receipt_run(&receipt_args(&scratch, &request, None)), 1);
}

#[test]
fn receipt_cli_wrong_key_malformed_history_and_missing_selector_emit_nothing() {
    use magpie_log::SigningKey;
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let request = receipt_request(&scratch, StandingReceiptExpectationV0::None);
    let args = receipt_args(&scratch, &request, None);
    let mut wrong_key = args.clone();
    set_option(
        &mut wrong_key,
        "--verifying-key",
        &hex::encode(SigningKey::from_bytes(&[73; 32]).verifying_key().as_bytes()),
    );
    assert_receipt_failure(&receipt_run(&wrong_key), 1);
    let mut missing = args.clone();
    missing[1] = "claim-missing".into();
    assert_receipt_failure(&receipt_run(&missing), 1);
    let original = scratch.log_bytes();
    for invalid in [
        b"{not json}\n".to_vec(),
        b"\n".to_vec(),
        String::from_utf8(original.clone())
            .unwrap()
            .replace("Theorem 2 holds", "tampered statement")
            .into_bytes(),
    ] {
        fs::write(scratch.log(), &invalid).unwrap();
        assert_receipt_failure(&receipt_run(&args), 1);
        assert_eq!(scratch.log_bytes(), invalid);
    }
}

#[test]
fn receipt_cli_ambiguous_typed_selector_refuses_without_store_uniqueness_rule() {
    use magpie_log::{FileStore, LogWriter, Payload, Provenance, SigningKey};
    let scratch = Scratch::new();
    // Supported writer construction, not a forged history or receipt.
    fs::create_dir_all(scratch.store()).unwrap();
    let key = SigningKey::from_bytes(&[66; 32]);
    let verifying_key = hex::encode(key.verifying_key().as_bytes());
    let mut writer = LogWriter::<FileStore>::open(FileStore::new(scratch.log()), key).unwrap();
    for _ in 0..2 {
        writer
            .append(
                Provenance::new("test", "receipt-cli"),
                Payload::ClaimAssertedV2 {
                    claim_id: "claim-1".into(),
                    statement: "observation".into(),
                    scope_ref: "scope:test".into(),
                    actor_class: "HumanRoot".into(),
                    content_hash: String::new(),
                    metadata_json: r#"{"claim_domain":"OperationalObservation"}"#.into(),
                },
            )
            .unwrap();
    }
    drop(writer);
    let request = StandingReceiptRequestV0 {
        context: StandingReceiptContextV0 {
            profile: STANDING_RECEIPT_HISTORY_PROFILE_V0.into(),
            history: StandingReceiptHistoryIdentityV0::of(&scratch.log_bytes()).unwrap(),
            verifying_key,
            expectation: StandingReceiptExpectationV0::None,
            claim_id: "claim-1".into(),
        },
        policy_id: MAGPIE_CLAIMS_POLICY_V2_ID.into(),
    };
    let output = receipt_run(&receipt_args(&scratch, &request, None));
    assert_receipt_failure(&output, 1);
    assert!(stderr(&output).contains("AmbiguousTypedClaim"));
}

/// Inventory both bytes and directory names to detect scratch/state creation too.
fn file_inventory(root: &Path) -> std::collections::BTreeMap<PathBuf, Option<Vec<u8>>> {
    fn visit(
        root: &Path,
        dir: &Path,
        result: &mut std::collections::BTreeMap<PathBuf, Option<Vec<u8>>>,
    ) {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            let directory = path.is_dir();
            result.insert(
                path.strip_prefix(root).unwrap().to_owned(),
                if directory {
                    None
                } else {
                    Some(fs::read(&path).unwrap())
                },
            );
            if directory {
                visit(root, &path, result);
            }
        }
    }
    let mut result = std::collections::BTreeMap::new();
    visit(root, root, &mut result);
    result
}

#[test]
fn receipt_cli_ignores_ambient_store_config_and_saved_state_and_writes_no_files() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let other = Scratch::new();
    small_ledger(&other);
    let request = receipt_request(&scratch, StandingReceiptExpectationV0::None);
    let args = receipt_args(&scratch, &request, None);
    let expected = receipt_run(&args);
    assert_eq!(code(&expected), 0);
    let detached = scratch.root.join("receipt.json");
    fs::write(&detached, &expected.stdout).unwrap();
    let check_args = receipt_args(&scratch, &request, Some(&detached));
    // A saved checkpoint ahead of explicit history A must have no effect.
    scratch.ok(&["note", "new checkpoint"]);
    let older: Vec<u8> = scratch
        .log_bytes()
        .split_inclusive(|b| *b == b'\n')
        .take(5)
        .flatten()
        .copied()
        .collect();
    fs::write(scratch.log(), older).unwrap();
    for command_args in [&args, &check_args] {
        for (store, state) in [
            (other.store(), other.state()),
            (scratch.store(), scratch.state()),
            (
                other.root.join("absent-store"),
                other.root.join("absent-state"),
            ),
        ] {
            let before_a = file_inventory(&scratch.root);
            let before_b = file_inventory(&other.root);
            let output = receipt_command(command_args)
                .env("MAGPIE_STORE", store)
                .env("MAGPIE_STATE_DIR", state)
                .env("XDG_STATE_HOME", other.root.join("absent-xdg"))
                .output()
                .unwrap();
            assert_eq!(code(&output), 0, "{}", stderr(&output));
            assert!(output.stderr.is_empty());
            assert_eq!(output.stdout, expected.stdout);
            assert_eq!(file_inventory(&scratch.root), before_a);
            assert_eq!(file_inventory(&other.root), before_b);
        }
    }
    // Explicit history needs neither its own config nor its signing key.
    fs::remove_file(scratch.store().join("config.json")).unwrap();
    fs::remove_file(scratch.store().join("signing.key")).unwrap();
    let no_config = receipt_run(&args);
    assert_eq!(code(&no_config), 0, "{}", stderr(&no_config));
    assert!(no_config.stderr.is_empty());
    assert_eq!(no_config.stdout, expected.stdout);
}

#[test]
fn receipt_cli_usage_shapes_and_explicit_full_identities_are_closed() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let request = receipt_request(&scratch, StandingReceiptExpectationV0::None);
    let args = receipt_args(&scratch, &request, None);
    for name in [
        "--history",
        "--verifying-key",
        "--profile",
        "--policy",
        "--expectation",
    ] {
        let mut missing = args.clone();
        let index = missing.iter().position(|arg| arg == name).unwrap();
        missing.drain(index..index + 2);
        assert_receipt_failure(&receipt_run(&missing), 2);
    }
    for (name, value) in [
        ("--expectation", "latest"),
        ("--expectation", "contains_checkpoint"),
        ("--profile", "latest"),
        ("--policy", "v2"),
        ("--policy", "current"),
        ("--verifying-key", "ABC"),
        ("--verifying-key", &"A".repeat(64)),
    ] {
        let mut invalid = args.clone();
        set_option(&mut invalid, name, value);
        assert_receipt_failure(&receipt_run(&invalid), 2);
    }
    for flags in [
        vec!["--store", "unrelated"],
        vec!["--checkpoint-event-count", "0"],
        vec!["--checkpoint-sha256", &"0".repeat(64)],
    ] {
        let mut invalid = args.clone();
        invalid.extend(flags.into_iter().map(str::to_owned));
        assert_receipt_failure(&receipt_run(&invalid), 2);
    }
    for relation in ["exact", "contains-checkpoint"] {
        let mut checkpoint_args = args.clone();
        set_option(&mut checkpoint_args, "--expectation", relation);
        assert_receipt_failure(&receipt_run(&checkpoint_args), 2);
        for single in [
            vec!["--checkpoint-event-count", "0"],
            vec!["--checkpoint-sha256", &"0".repeat(64)],
        ] {
            let mut incomplete = checkpoint_args.clone();
            incomplete.extend(single.into_iter().map(str::to_owned));
            assert_receipt_failure(&receipt_run(&incomplete), 2);
        }
        checkpoint_args.extend([
            "--checkpoint-event-count".into(),
            "0".into(),
            "--checkpoint-sha256".into(),
            "0".repeat(64),
        ]);
        for count in ["-1", "+1", "1.0", "18446744073709551616", ""] {
            let mut invalid = checkpoint_args.clone();
            set_option(&mut invalid, "--checkpoint-event-count", count);
            assert_receipt_failure(&receipt_run(&invalid), 2);
        }
        for hash in ["xyz".into(), "A".repeat(64), "0".repeat(63)] {
            let mut invalid = checkpoint_args.clone();
            set_option(&mut invalid, "--checkpoint-sha256", &hash);
            assert_receipt_failure(&receipt_run(&invalid), 2);
        }
    }
    let mut missing_receipt = args.clone();
    missing_receipt[0] = "check-standing-receipt".into();
    assert_receipt_failure(&receipt_run(&missing_receipt), 2);
    let mut empty_claim = args.clone();
    empty_claim[1].clear();
    assert_receipt_failure(&receipt_run(&empty_claim), 2);
}

#[test]
fn receipt_cli_missing_files_are_operational_and_help_is_explicit() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let request = receipt_request(&scratch, StandingReceiptExpectationV0::None);
    let mut args = receipt_args(&scratch, &request, None);
    set_option(
        &mut args,
        "--history",
        scratch.root.join("missing-history").to_str().unwrap(),
    );
    assert_receipt_failure(&receipt_run(&args), 3);
    let missing = scratch.root.join("missing-receipt");
    assert_receipt_failure(
        &receipt_run(&receipt_args(&scratch, &request, Some(&missing))),
        3,
    );
    let help = scratch.ok(&["standing-receipt", "--help"]);
    for text in [
        "Read-only native policy-v2",
        "check-standing-receipt",
        "Expectation is mandatory",
        "without a newline",
        "No current/latest/saved checkpoint",
    ] {
        assert!(help.contains(text), "{help}");
    }
}

#[cfg(target_os = "linux")]
#[test]
fn receipt_cli_output_device_failure_is_exit_three_without_state_changes() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let request = receipt_request(&scratch, StandingReceiptExpectationV0::None);
    let before = file_inventory(&scratch.root);
    let full = fs::OpenOptions::new()
        .write(true)
        .open("/dev/full")
        .unwrap();
    let output = receipt_command(&receipt_args(&scratch, &request, None))
        .stdout(full)
        .output()
        .unwrap();
    assert_receipt_failure(&output, 3);
    assert_eq!(file_inventory(&scratch.root), before);
}

#[test]
fn receipt_cli_retains_exact_crlf_and_unterminated_history_bytes() {
    let scratch = Scratch::new();
    small_ledger(&scratch);
    let original = scratch.log_bytes();
    let base = receipt_request(&scratch, StandingReceiptExpectationV0::None);
    let first = produce_standing_receipt_v0(&base, &original).unwrap();
    let crlf = String::from_utf8(original.clone())
        .unwrap()
        .replace('\n', "\r\n")
        .into_bytes();
    let mut unterminated = original.clone();
    unterminated.pop();
    for bytes in [crlf, unterminated] {
        fs::write(scratch.log(), &bytes).unwrap();
        let request = receipt_request(&scratch, StandingReceiptExpectationV0::None);
        let expected = produce_standing_receipt_v0(&request, &bytes).unwrap();
        let output = receipt_run(&receipt_args(&scratch, &request, None));
        assert_eq!(code(&output), 0, "{}", stderr(&output));
        assert!(output.stderr.is_empty());
        assert_eq!(output.stdout, expected.canonical_bytes());
        assert_eq!(
            expected.wire().context.history.byte_count,
            bytes.len() as u64
        );
        assert_ne!(expected.context_sha256(), first.context_sha256());
        assert_eq!(expected.wire().outcome, first.wire().outcome);
        assert_eq!(scratch.log_bytes(), bytes);
    }
}
