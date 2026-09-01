use std::path::{Path, PathBuf};

use magpie_log::FileStore;

const GOLDEN_KEY: &str = "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c";
const INADMISSIBLE_KEY: &str = "0100000000000000000000000000000000000000000000000000000000000000";

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn public_file_path_accepts_the_unchanged_golden_history() {
    let root = repository_root();
    let store = FileStore::new(root.join("crates/magpie-log/testdata/golden-v1.jsonl"));

    assert!(store.verify_portable_history(GOLDEN_KEY).unwrap());
}

#[test]
fn public_file_path_preserves_portable_input_distinctions() {
    let root = repository_root();
    for relative_path in [
        "fixtures/verifier-language-v1/cases/n7-lf-only-empty-record.jsonl",
        "fixtures/verifier-language-v1/cases/n30-extra-final-terminator.jsonl",
        "fixtures/verifier-language-v1/cases/n30-terminal-lone-cr.jsonl",
        "fixtures/verifier-language-v1/cases/n8-duplicate-signedevent-core.jsonl",
        "fixtures/verifier-language-v1/cases/n10-unknown-signedevent.jsonl",
        "fixtures/verifier-language-v1/cases/n16-seq-negative-zero.jsonl",
    ] {
        let store = FileStore::new(root.join(relative_path));
        assert!(
            !store.verify_portable_history(GOLDEN_KEY).unwrap(),
            "{relative_path} must be rejected by the public file path"
        );
    }
}

#[test]
fn public_file_path_accepts_an_existing_zero_byte_snapshot() {
    let root = repository_root();
    let store = FileStore::new(
        root.join("fixtures/verifier-language-v1/cases/n7-k6-k14-empty-golden-key.jsonl"),
    );

    assert!(store.verify_portable_history(GOLDEN_KEY).unwrap());
}

#[test]
fn public_file_path_gates_the_key_before_file_acquisition() {
    let missing = repository_root()
        .join("fixtures/verifier-language-v1/cases/__pr146_missing_history__.jsonl");
    assert!(!missing.exists(), "test requires an absent history path");
    let store = FileStore::new(missing);

    assert!(matches!(
        store.verify_portable_history(INADMISSIBLE_KEY),
        Ok(false)
    ));
    assert!(matches!(
        store.verify_portable_history(GOLDEN_KEY),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound
    ));
}
