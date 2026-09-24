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

    fn run_in(&self, store: &Path, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_magpie"))
            .args(args)
            .env("MAGPIE_STORE", store)
            .env("MAGPIE_STATE_DIR", self.state())
            .env_remove("XDG_STATE_HOME")
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
