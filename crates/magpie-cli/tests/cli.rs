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
