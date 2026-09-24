//! What each command does.
//!
//! Every command reads the log once into a [`Snapshot`] and runs all of its
//! checks and projections against that one image. Writes hold the store lock
//! and the checkpoint lock, refuse a torn or portable-invalid log, check the
//! saved checkpoint, validate the new event against the replay, confirm the
//! file still matches the snapshot, append through `LogWriter`, fsync, and move
//! the checkpoint forward. Reads take the same locks in shared mode.

use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use magpie_claims::policy::{ClaimDomain, EvidenceKind};
use magpie_claims::{replay_origin_admission_context_v0, OriginAdmissionReplayContextV0};
use magpie_episodic::EpisodicView;
use magpie_log::{
    FileStore, LogError, LogReader, LogWriter, MemStore, Payload, Provenance, SignedEvent,
    VerifiedReplaySummary,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::args::{Command, Invocation};
use crate::display::{preview, utc};
use crate::export::desk_v0;
use crate::index::{claim_domain, metadata_object, LedgerIndex};
use crate::policy::{PolicyChoice, Resolver};
use crate::snapshot::Snapshot;
use crate::store::{log_failure, parse_verifying_key, CheckpointState, LockMode, Store};
use crate::trace::trace_lines;
use crate::CliError;

/// Writes through this tool are the owner's own; see the crate docs.
const ACTOR_CLASS: &str = "HumanRoot";
const DEFAULT_SCOPE: &str = "scope:default";
const DEFAULT_SOURCE: &str = "magpie-cli";
const EDGE_KINDS: [&str; 6] = [
    "supports",
    "derived_from",
    "contradicts",
    "supersedes",
    "invalidates",
    "ratifies",
];

fn usage(message: impl Into<String>) -> CliError {
    CliError::Usage(message.into())
}

pub(crate) fn execute(
    invocation: Invocation,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Result<(), CliError> {
    let json = invocation.json;
    match invocation.command {
        Command::Help => {
            out.write_all(help().as_bytes())?;
            Ok(())
        }
        Command::Version => {
            writeln!(out, "magpie {}", env!("CARGO_PKG_VERSION"))?;
            Ok(())
        }
        Command::Init { dir, agent } => init(&dir, agent, json, out, err),
        Command::Verify { verifying_key } => {
            let dir = store_dir(invocation.store)?;
            let store = match &verifying_key {
                // An off-machine key must still be able to check the log when
                // the store's own config is damaged, so it needs none.
                Some(text) => Store::for_verification(&dir, parse_verifying_key(text)?)?,
                None => Store::open(&dir)?,
            };
            verify(&store, json, out)
        }
        command => {
            let store = Store::open(&store_dir(invocation.store)?)?;
            match command {
                Command::Note { text, source } => note(&store, text, source, json, out),
                Command::Claim {
                    statement,
                    domain,
                    scope,
                    id,
                    source,
                } => claim(&store, statement, &domain, scope, id, source, json, out),
                Command::Evidence {
                    summary,
                    kind,
                    scope,
                    id,
                    file,
                    source_uri,
                    locator,
                    source,
                } => evidence(
                    &store,
                    EvidenceInput {
                        summary,
                        kind,
                        scope,
                        id,
                        file,
                        source_uri,
                        locator,
                        source,
                    },
                    json,
                    out,
                ),
                Command::Link {
                    kind,
                    from,
                    to,
                    rationale,
                    scope,
                    id,
                    source,
                } => link(
                    &store,
                    LinkInput {
                        kind,
                        from,
                        to,
                        rationale,
                        scope,
                        id,
                        source,
                    },
                    json,
                    out,
                    err,
                ),
                Command::Show { id } => show(&store, &id, json, out, err),
                Command::Search { text } => search(&store, &text, json, out, err),
                Command::Standing { claim, policy } => {
                    standing(&store, &claim, &policy, false, json, out, err)
                }
                Command::Why { claim, policy } => {
                    standing(&store, &claim, &policy, true, json, out, err)
                }
                Command::Log => log(&store, json, out, err),
                Command::Export { format, policy } => export(&store, &format, &policy, out, err),
                Command::Checkpoint { accept_current } => {
                    checkpoint(&store, accept_current, json, out)
                }
                Command::Help
                | Command::Version
                | Command::Init { .. }
                | Command::Verify { .. } => {
                    unreachable!("handled before the store is opened")
                }
            }
        }
    }
}

fn store_dir(flag: Option<PathBuf>) -> Result<PathBuf, CliError> {
    flag.or_else(|| {
        std::env::var_os("MAGPIE_STORE")
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
    })
    .ok_or_else(|| usage("no store selected: pass --store <dir> or set MAGPIE_STORE"))
}

fn help() -> String {
    let domains: Vec<&str> = ClaimDomain::ALL.iter().map(|d| d.as_str()).collect();
    let kinds: Vec<&str> = EvidenceKind::ALL.iter().map(|k| k.as_str()).collect();
    format!(
        "magpie: keep a signed claim ledger

Usage: magpie [--store <dir>] <command> [options] [--json]

Create a store:
  init <dir> [--agent <name>]      create a store and print its verifying key

Record (each write is signed and appended; nothing is ever edited or deleted):
  note <text>
  claim <statement> --domain <domain> [--scope <scope>] [--id <id>]
  evidence <summary> --kind <kind> [--file <path>] [--scope <scope>] [--id <id>]
           [--source-uri <uri>] [--locator <where>]   (ExternalSource only)
  link <kind> <from-id> <to-id> --rationale <text> [--scope <scope>] [--id <id>]
  Every write also takes --source <label>, recorded as provenance.

Read (each read verifies the whole log first):
  show <id>
  search <text>
  standing <claim-id> --policy <v0..v4>
  why <claim-id> --policy <v0..v4>
  log
  verify [--verifying-key <hex>]
  export --format desk-v0 --policy <v0..v4>
  checkpoint [--accept-current]

The store is --store <dir>, or $MAGPIE_STORE. Checkpoints live in
$MAGPIE_STATE_DIR, else $XDG_STATE_HOME/magpie, else ~/.local/state/magpie.

Domains:        {}
Evidence kinds: {}
Link kinds:     {}
",
        domains.join(", "),
        kinds.join(", "),
        EDGE_KINDS.join(", ")
    )
}

// ---------------------------------------------------------------- init

fn init(
    dir: &Path,
    agent: Option<String>,
    json: bool,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Result<(), CliError> {
    let agent = agent
        .or_else(|| std::env::var("USER").ok())
        .or_else(|| std::env::var("USERNAME").ok())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or_else(|| "owner".to_owned());
    let (store, tip) = Store::init(dir, agent)?;
    if cfg!(windows) {
        // The standard library can't set an owner-only ACL, so the key file
        // takes whatever the folder grants. Advisory: the store exists.
        let _ = writeln!(
            err,
            "warning: on Windows, signing.key gets the permissions of {}, so anyone who can \
             read that folder can sign as you; keep the store in a folder only you can read, \
             such as one under your user profile",
            dir.display()
        );
    }
    let verifying_key = hex::encode(store.verifying_key().as_bytes());
    // The store exists now, and `init` refuses to run twice, so a failed print
    // must say where to find the key instead of inviting a retry.
    print_created(dir, &verifying_key, store.agent(), &tip.to_hex(), json, out).map_err(|error| {
        CliError::Io(format!(
            "created the store in {}, but could not print the result: {error}. Don't run `magpie \
             init` again; the verifying key is recorded in {}.",
            dir.display(),
            store.config_path().display()
        ))
    })
}

fn print_created(
    dir: &Path,
    verifying_key: &str,
    agent: &str,
    tip: &str,
    json: bool,
    out: &mut dyn Write,
) -> std::io::Result<()> {
    if json {
        writeln!(
            out,
            "{}",
            json!({
                "store": dir.display().to_string(),
                "verifying_key": verifying_key,
                "agent": agent,
                "tip": tip,
            })
        )?;
    } else {
        writeln!(out, "Created a Magpie store in {}", dir.display())?;
        writeln!(out, "Verifying key: {verifying_key}")?;
        writeln!(out)?;
        writeln!(
            out,
            "Record the verifying key somewhere off this machine; `magpie verify \
             --verifying-key <key>` checks the log against it."
        )?;
        writeln!(
            out,
            "To use this store by default: export MAGPIE_STORE={}",
            dir.display()
        )?;
    }
    out.flush()
}

// ---------------------------------------------------------------- writes

/// Run the shared write sequence. `build` sees the verified index and the next
/// sequence number and returns the events to append; it is the last point at
/// which a command can refuse, and nothing has been written when it does.
fn write_events(
    store: &Store,
    source: Option<String>,
    build: impl FnOnce(&LedgerIndex, u64) -> Result<Vec<Payload>, CliError>,
) -> Result<Vec<SignedEvent>, CliError> {
    let source = source.unwrap_or_else(|| DEFAULT_SOURCE.to_owned());
    if source.trim().is_empty() {
        return Err(usage("--source must not be empty"));
    }
    let agent = store.writing_agent()?;
    let _store_lock = store.lock(LockMode::Exclusive)?;
    let _checkpoint_lock = store.lock_checkpoint(LockMode::Exclusive)?;
    let log_path = store.log_path();
    let snapshot = Snapshot::read_for_write(store)?;
    let reader = snapshot.reader(store.verifying_key());
    if let CheckpointState::Missing = store.check_checkpoint(&reader)? {
        return Err(CliError::Refused(
            "no checkpoint is saved for this log, so a rolled-back copy could not be told apart \
             from the real one. If this log is the one you expect, run \
             `magpie checkpoint --accept-current`, then retry."
                .into(),
        ));
    }
    let (index, summary) = LedgerIndex::replay(&reader).map_err(log_failure)?;
    let payloads = build(&index, summary.event_count())?;

    let mut writer = LogWriter::<FileStore>::open(FileStore::new(&log_path), store.signing_key()?)
        .map_err(log_failure)?;
    // The last check before appending: the file must still be exactly the
    // snapshot that every check above ran against.
    if writer.len() != summary.event_count()
        || writer.tip() != summary.tip()
        || std::fs::read(&log_path)? != snapshot.bytes()
    {
        return Err(CliError::Refused(
            "the log changed while this command was running; nothing was written".into(),
        ));
    }
    let provenance = Provenance::new(agent, source);
    let mut written = Vec::new();
    let mut failure = None;
    for payload in payloads {
        match writer.append(provenance.clone(), payload) {
            Ok(event) => written.push(event),
            Err(error) => {
                failure = Some(error);
                break;
            }
        }
    }
    let recorded = recorded_seqs(&written);
    if !written.is_empty() {
        // The events are in the log now. A failure from here on must say so,
        // or a retry would record them twice.
        if let Err(error) = store
            .sync_log()
            .and_then(|()| store.save_checkpoint(writer.len(), writer.tip()))
        {
            return Err(CliError::Io(format!(
                "recorded {recorded} in the log, but could not finish afterwards: {error}. \
                 Don't retry the command, or it will be recorded twice; fix the problem, then \
                 run `magpie checkpoint --accept-current`."
            )));
        }
    }
    let before = if written.is_empty() {
        String::new()
    } else {
        format!("recorded {recorded}, but then ")
    };
    match failure {
        None => Ok(written),
        // Only a failed write can leave part of a record behind; anything else
        // is refused before the store is touched.
        Some(LogError::Io(error)) => Err(CliError::Io(format!(
            "{before}an append failed: {error}. Part of the event may have reached the log, \
             so run `magpie verify` before retrying."
        ))),
        Some(other) => Err(CliError::Refused(format!(
            "{before}the log refused an event, so it was not written: {other}"
        ))),
    }
}

fn require_text(text: &str, what: &str) -> Result<(), CliError> {
    if text.trim().is_empty() {
        Err(usage(format!("{what} must not be empty")))
    } else {
        Ok(())
    }
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | ':'))
}

/// Use the caller's id or `<prefix>-<seq>`, refusing any id already in use.
fn choose_id(
    index: &LedgerIndex,
    requested: Option<String>,
    prefix: &str,
    next_seq: u64,
) -> Result<String, CliError> {
    let id = match requested {
        Some(id) if valid_id(&id) => id,
        Some(id) => {
            return Err(usage(format!(
                "id `{id}` may use only letters, digits, and - _ . :"
            )))
        }
        None => format!("{prefix}-{next_seq}"),
    };
    if index.id_in_use(&id) {
        return Err(usage(format!(
            "id `{id}` is already used by a claim, evidence, or link; choose another with --id"
        )));
    }
    Ok(id)
}

fn scope_or_default(scope: Option<String>) -> Result<String, CliError> {
    let scope = scope.unwrap_or_else(|| DEFAULT_SCOPE.to_owned());
    require_text(&scope, "--scope")?;
    Ok(scope)
}

/// `seq 4, seq 5`: which events a write recorded.
fn recorded_seqs(events: &[SignedEvent]) -> String {
    let seqs: Vec<String> = events
        .iter()
        .map(|event| format!("seq {}", event.core.seq))
        .collect();
    seqs.join(", ")
}

/// Print what a write recorded. The events are already in the log and the
/// checkpoint has moved, so a failed print must say so, or a retry would
/// record them twice.
fn report_written(
    events: &[SignedEvent],
    id: Option<&str>,
    json: bool,
    out: &mut dyn Write,
) -> Result<(), CliError> {
    print_written(events, id, json, out).map_err(|error| {
        CliError::Io(format!(
            "recorded {}, but could not print the result: {error}. Don't retry the command, or \
             it will be recorded twice.",
            recorded_seqs(events)
        ))
    })
}

fn print_written(
    events: &[SignedEvent],
    id: Option<&str>,
    json: bool,
    out: &mut dyn Write,
) -> std::io::Result<()> {
    for event in events {
        let kind = event_kind(&event.core.payload);
        if json {
            writeln!(
                out,
                "{}",
                json!({
                    "seq": event.core.seq,
                    "kind": kind,
                    "id": id,
                    "hash": event.hash.to_hex(),
                })
            )?;
        } else {
            match id {
                Some(id) => writeln!(out, "recorded {kind} {id} at seq {}", event.core.seq)?,
                None => writeln!(out, "recorded {kind} at seq {}", event.core.seq)?,
            }
        }
    }
    out.flush()
}

fn event_kind(payload: &Payload) -> &'static str {
    match payload {
        Payload::Genesis { .. } => "genesis",
        Payload::ClaimAsserted { .. } => "claim_asserted",
        Payload::EvidenceRecorded { .. } => "evidence_recorded",
        Payload::ClaimStatusChanged { .. } => "claim_status_changed",
        Payload::Note { .. } => "note",
        Payload::SegmentAnchored { .. } => "segment_anchored",
        Payload::ClaimAssertedV2 { .. } => "claim",
        Payload::EvidenceRegistered { .. } => "evidence",
        Payload::JustificationEdgeRecorded { .. } => "link",
    }
}

fn note(
    store: &Store,
    text: String,
    source: Option<String>,
    json: bool,
    out: &mut dyn Write,
) -> Result<(), CliError> {
    require_text(&text, "the note")?;
    let written = write_events(store, source, |_, _| Ok(vec![Payload::Note { text }]))?;
    report_written(&written, None, json, out)
}

#[allow(clippy::too_many_arguments)]
fn claim(
    store: &Store,
    statement: String,
    domain: &str,
    scope: Option<String>,
    id: Option<String>,
    source: Option<String>,
    json: bool,
    out: &mut dyn Write,
) -> Result<(), CliError> {
    let domain = ClaimDomain::try_from(domain).map_err(|_| {
        let known: Vec<&str> = ClaimDomain::ALL.iter().map(|d| d.as_str()).collect();
        usage(format!(
            "unknown claim domain `{domain}`; choose one of: {}",
            known.join(", ")
        ))
    })?;
    require_text(&statement, "the claim statement")?;
    let scope = scope_or_default(scope)?;
    let mut chosen = String::new();
    let written = write_events(store, source, |index, next_seq| {
        chosen = choose_id(index, id, "claim", next_seq)?;
        Ok(vec![Payload::ClaimAssertedV2 {
            claim_id: chosen.clone(),
            statement,
            scope_ref: scope,
            actor_class: ACTOR_CLASS.to_owned(),
            content_hash: String::new(),
            metadata_json: json!({ "claim_domain": domain.as_str() }).to_string(),
        }])
    })?;
    report_written(&written, Some(chosen.as_str()), json, out)
}

struct EvidenceInput {
    summary: String,
    kind: String,
    scope: Option<String>,
    id: Option<String>,
    file: Option<PathBuf>,
    source_uri: Option<String>,
    locator: Option<String>,
    source: Option<String>,
}

fn evidence(
    store: &Store,
    input: EvidenceInput,
    json: bool,
    out: &mut dyn Write,
) -> Result<(), CliError> {
    let kind = EvidenceKind::try_from(input.kind.as_str()).map_err(|_| {
        let known: Vec<&str> = EvidenceKind::ALL.iter().map(|k| k.as_str()).collect();
        usage(format!(
            "unknown evidence kind `{}`; choose one of: {}",
            input.kind,
            known.join(", ")
        ))
    })?;
    require_text(&input.summary, "the evidence summary")?;
    let scope = scope_or_default(input.scope)?;
    if (input.source_uri.is_some() || input.locator.is_some())
        && kind != EvidenceKind::ExternalSource
    {
        return Err(usage(
            "--source-uri and --locator describe a cited source, so they apply only to --kind ExternalSource",
        ));
    }
    let mut metadata = serde_json::Map::new();
    if let Some(uri) = input.source_uri {
        metadata.insert("source_uri".into(), Value::String(uri));
    }
    if let Some(locator) = input.locator {
        metadata.insert("source_locator".into(), Value::String(locator));
    }
    // The artifact itself stays off the log; only its SHA-256 is committed.
    let content_hash = match &input.file {
        Some(path) => sha256_file(path)?,
        None => String::new(),
    };
    let summary = input.summary;
    let id = input.id;
    let mut chosen = String::new();
    let written = write_events(store, input.source, |index, next_seq| {
        chosen = choose_id(index, id, "ev", next_seq)?;
        Ok(vec![Payload::EvidenceRegistered {
            evidence_id: chosen.clone(),
            evidence_kind: kind.as_str().to_owned(),
            summary,
            scope_ref: scope,
            actor_class: ACTOR_CLASS.to_owned(),
            content_hash,
            metadata_json: Value::Object(metadata).to_string(),
        }])
    })?;
    report_written(&written, Some(chosen.as_str()), json, out)
}

fn sha256_file(path: &Path) -> Result<String, CliError> {
    let mut file = File::open(path)
        .map_err(|error| usage(format!("cannot read --file {}: {error}", path.display())))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(hex::encode(hasher.finalize()))
}

struct LinkInput {
    kind: String,
    from: String,
    to: String,
    rationale: String,
    scope: Option<String>,
    id: Option<String>,
    source: Option<String>,
}

fn link(
    store: &Store,
    input: LinkInput,
    json: bool,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Result<(), CliError> {
    if !EDGE_KINDS.contains(&input.kind.as_str()) {
        return Err(usage(format!(
            "unknown link kind `{}`; choose one of: {}",
            input.kind,
            EDGE_KINDS.join(", ")
        )));
    }
    require_text(&input.rationale, "--rationale")?;
    let LinkInput {
        kind,
        from,
        to,
        rationale,
        scope,
        id,
        source,
    } = input;
    let mut chosen = String::new();
    let mut scope_note = None;
    let written = write_events(store, source, |index, next_seq| {
        for reference in [&from, &to] {
            if !index.id_in_use(reference) {
                return Err(usage(format!(
                    "no claim, evidence, or link has id `{reference}`"
                )));
            }
        }
        let scope = match scope {
            Some(scope) => scope,
            None => index.scope_of(&to).unwrap_or(DEFAULT_SCOPE).to_owned(),
        };
        require_text(&scope, "--scope")?;
        let mismatched: Vec<String> = [&from, &to]
            .into_iter()
            .filter_map(|id| {
                index
                    .scope_of(id)
                    .filter(|other| *other != scope)
                    .map(|other| format!("{id} is in {other}"))
            })
            .collect();
        if !mismatched.is_empty() {
            scope_note = Some(format!(
                "note: this link is in {scope}, but {}; standing reports scope_mismatch for it",
                mismatched.join(" and ")
            ));
        }
        chosen = choose_id(index, id, "edge", next_seq)?;
        Ok(vec![Payload::JustificationEdgeRecorded {
            edge_id: chosen.clone(),
            edge_kind: kind,
            source_id: from,
            target_id: to,
            scope_ref: scope,
            actor_class: ACTOR_CLASS.to_owned(),
            rationale,
            metadata_json: "{}".to_owned(),
        }])
    })?;
    report_written(&written, Some(chosen.as_str()), json, out)?;
    if let Some(note) = scope_note {
        // Advisory only: the link is recorded and reported, so failing to
        // print this must not turn the command into an apparent failure.
        let _ = writeln!(err, "{note}");
    }
    Ok(())
}

// ---------------------------------------------------------------- reads

struct ReadView {
    _store_lock: Option<File>,
    _checkpoint_lock: Option<File>,
    reader: LogReader<MemStore>,
    index: LedgerIndex,
    summary: VerifiedReplaySummary,
    context: Option<OriginAdmissionReplayContextV0>,
}

/// Shared locks, one snapshot, the portable and checkpoint checks, and the
/// projections a read needs, all from that same snapshot.
fn read_view(
    store: &Store,
    with_standing: bool,
    err: &mut dyn Write,
) -> Result<ReadView, CliError> {
    let store_lock = store.lock(LockMode::Shared)?;
    let checkpoint_lock = store.lock_checkpoint(LockMode::Shared)?;
    let snapshot = Snapshot::read_checked(store)?;
    let reader = snapshot.reader(store.verifying_key());
    if let CheckpointState::Missing = store.check_checkpoint(&reader)? {
        writeln!(
            err,
            "warning: no checkpoint is saved for this log, so a rolled-back copy would go \
             unnoticed; run `magpie checkpoint --accept-current` if this log is the one you expect"
        )?;
    }
    let (index, summary) = LedgerIndex::replay(&reader).map_err(log_failure)?;
    let context = if with_standing {
        Some(replay_origin_admission_context_v0(&reader).map_err(log_failure)?)
    } else {
        None
    };
    if !index.repeated_ids.is_empty() {
        let repeated: Vec<&str> = index.repeated_ids.iter().map(String::as_str).collect();
        writeln!(
            err,
            "warning: these ids are recorded more than once, and only the first is shown: {}",
            repeated.join(", ")
        )?;
    }
    Ok(ReadView {
        _store_lock: store_lock,
        _checkpoint_lock: checkpoint_lock,
        reader,
        index,
        summary,
        context,
    })
}

fn recorded_json(recorded: &crate::index::Recorded) -> Value {
    json!({
        "seq": recorded.seq,
        "timestamp_nanos": recorded.timestamp_nanos,
        "agent": recorded.agent,
        "source": recorded.source,
        "hash": recorded.hash,
    })
}

fn recorded_line(recorded: &crate::index::Recorded) -> String {
    format!(
        "seq {}, {}, {} via {}",
        recorded.seq,
        utc(recorded.timestamp_nanos),
        recorded.agent,
        recorded.source
    )
}

/// One link touching an id: the link's own id, its kind, and the id at its other end.
struct LinkRef<'a> {
    edge: &'a str,
    kind: &'a str,
    other: &'a str,
}

/// Links into and out of `id`.
fn links_of<'a>(index: &'a LedgerIndex, id: &str) -> (Vec<LinkRef<'a>>, Vec<LinkRef<'a>>) {
    let mut incoming = Vec::new();
    let mut outgoing = Vec::new();
    for (edge_id, edge) in &index.edges {
        if edge.target_id == id {
            incoming.push(LinkRef {
                edge: edge_id,
                kind: &edge.kind,
                other: &edge.source_id,
            });
        }
        if edge.source_id == id {
            outgoing.push(LinkRef {
                edge: edge_id,
                kind: &edge.kind,
                other: &edge.target_id,
            });
        }
    }
    (incoming, outgoing)
}

fn describe(index: &LedgerIndex, id: &str) -> String {
    if let Some(claim) = index.claims.get(id) {
        format!("claim: {}", preview(&claim.statement, 60))
    } else if let Some(evidence) = index.evidence.get(id) {
        format!("{}: {}", evidence.kind, preview(&evidence.summary, 60))
    } else if let Some(edge) = index.edges.get(id) {
        format!(
            "link {} {} -> {}",
            edge.kind, edge.source_id, edge.target_id
        )
    } else {
        "not recorded".to_owned()
    }
}

fn show(
    store: &Store,
    id: &str,
    json: bool,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Result<(), CliError> {
    let view = read_view(store, false, err)?;
    let index = &view.index;
    let (incoming, outgoing) = links_of(index, id);
    let link_json = |links: &[LinkRef<'_>]| -> Value {
        links
            .iter()
            .map(|link| json!({"id": link.edge, "kind": link.kind, "other": link.other}))
            .collect()
    };

    if let Some(claim) = index.claims.get(id) {
        let domain = claim
            .typed
            .as_ref()
            .and_then(|typed| claim_domain(&typed.metadata_json));
        if json {
            let value = json!({
                "type": "claim",
                "id": id,
                "statement": claim.statement,
                "typed": claim.typed.is_some(),
                "domain": domain,
                "metadata_json": claim.typed.as_ref().map(|t| &t.metadata_json),
                "scope": claim.typed.as_ref().map(|t| &t.scope),
                "actor_class": claim.typed.as_ref().map(|t| &t.actor_class),
                "legacy_raw_status": claim.legacy_status,
                "recorded": recorded_json(&claim.recorded),
                "links_in": link_json(&incoming),
                "links_out": link_json(&outgoing),
            });
            writeln!(out, "{}", serde_json::to_string_pretty(&value)?)?;
            return Ok(());
        }
        writeln!(out, "claim {id}  ({})", recorded_line(&claim.recorded))?;
        writeln!(out, "  {}", claim.statement)?;
        match &claim.typed {
            Some(typed) => writeln!(
                out,
                "  domain {} · scope {} · actor {}",
                domain.as_deref().unwrap_or("(missing or malformed)"),
                typed.scope,
                typed.actor_class
            )?,
            None => writeln!(
                out,
                "  legacy claim; its self-asserted status {:?} is quarantined and is not standing",
                claim.legacy_status
            )?,
        }
        for (text, recorded) in &claim.legacy_notes {
            writeln!(out, "  {text}  (seq {})", recorded.seq)?;
        }
        write_links(out, index, &incoming, &outgoing)?;
        writeln!(out, "  standing: magpie standing {id} --policy <v0..v4>")?;
    } else if let Some(evidence) = index.evidence.get(id) {
        // L0 treats metadata as opaque text, and another writer may record
        // text that isn't a JSON object with unique keys. Show it as recorded
        // rather than as if it were empty.
        let metadata = metadata_object(&evidence.metadata_json);
        if json {
            let value = json!({
                "type": "evidence",
                "id": id,
                "kind": evidence.kind,
                "summary": evidence.summary,
                "scope": evidence.scope,
                "actor_class": evidence.actor_class,
                "content_hash": evidence.content_hash,
                "metadata": metadata,
                "metadata_json": evidence.metadata_json,
                "recorded": recorded_json(&evidence.recorded),
                "links_in": link_json(&incoming),
                "links_out": link_json(&outgoing),
            });
            writeln!(out, "{}", serde_json::to_string_pretty(&value)?)?;
            return Ok(());
        }
        writeln!(
            out,
            "evidence {id}  ({})",
            recorded_line(&evidence.recorded)
        )?;
        writeln!(out, "  {}", evidence.summary)?;
        writeln!(
            out,
            "  kind {} · scope {} · actor {}",
            evidence.kind, evidence.scope, evidence.actor_class
        )?;
        if !evidence.content_hash.is_empty() {
            writeln!(out, "  sha256 {}", evidence.content_hash)?;
        }
        write_metadata(out, &evidence.metadata_json)?;
        write_links(out, index, &incoming, &outgoing)?;
    } else if let Some(edge) = index.edges.get(id) {
        if json {
            let value = json!({
                "type": "link",
                "id": id,
                "kind": edge.kind,
                "from": edge.source_id,
                "to": edge.target_id,
                "rationale": edge.rationale,
                "scope": edge.scope,
                "actor_class": edge.actor_class,
                "metadata": metadata_object(&edge.metadata_json),
                "metadata_json": edge.metadata_json,
                "recorded": recorded_json(&edge.recorded),
            });
            writeln!(out, "{}", serde_json::to_string_pretty(&value)?)?;
            return Ok(());
        }
        writeln!(out, "link {id}  ({})", recorded_line(&edge.recorded))?;
        writeln!(
            out,
            "  {} {} -> {}",
            edge.kind, edge.source_id, edge.target_id
        )?;
        writeln!(out, "  from: {}", describe(index, &edge.source_id))?;
        writeln!(out, "  to:   {}", describe(index, &edge.target_id))?;
        writeln!(out, "  rationale: {}", edge.rationale)?;
        writeln!(out, "  scope {} · actor {}", edge.scope, edge.actor_class)?;
        write_metadata(out, &edge.metadata_json)?;
    } else {
        return Err(usage(format!("no claim, evidence, or link has id `{id}`")));
    }
    Ok(())
}

/// Print recorded metadata: each key of a well-formed object, or the text as
/// recorded when it isn't a JSON object with unique keys, so it never passes
/// for empty.
fn write_metadata(out: &mut dyn Write, raw: &str) -> Result<(), CliError> {
    match metadata_object(raw) {
        Some(metadata) => {
            for (key, value) in &metadata {
                writeln!(
                    out,
                    "  {key}: {}",
                    value
                        .as_str()
                        .map(str::to_owned)
                        .unwrap_or_else(|| value.to_string())
                )?;
            }
        }
        None => writeln!(
            out,
            "  metadata, as recorded (not a JSON object with unique keys): {raw:?}"
        )?,
    }
    Ok(())
}

fn write_links(
    out: &mut dyn Write,
    index: &LedgerIndex,
    incoming: &[LinkRef<'_>],
    outgoing: &[LinkRef<'_>],
) -> Result<(), CliError> {
    for (label, links, arrow) in [("links in", incoming, "<-"), ("links out", outgoing, "->")] {
        if links.is_empty() {
            writeln!(out, "  {label}: none")?;
            continue;
        }
        writeln!(out, "  {label}:")?;
        for link in links {
            writeln!(
                out,
                "    {} {} {arrow} {}  ({})",
                link.edge,
                link.kind,
                link.other,
                describe(index, link.other)
            )?;
        }
    }
    Ok(())
}

fn search(
    store: &Store,
    text: &str,
    json: bool,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Result<(), CliError> {
    let view = read_view(store, false, err)?;
    // The search index stores seq and timestamp as SQLite's signed 64-bit
    // integers and panics on a larger value, which a validly signed log can
    // hold. Refuse such a log here instead.
    if let Some(event) = view.index.events.iter().find(|event| {
        i64::try_from(event.recorded.seq).is_err()
            || i64::try_from(event.recorded.timestamp_nanos).is_err()
    }) {
        return Err(CliError::Refused(format!(
            "event seq {} holds a number too large for the search index, which stores seq and \
             timestamp as signed 64-bit integers, so search can't read this log; the other \
             commands still can",
            event.recorded.seq
        )));
    }
    let mut episodic = EpisodicView::in_memory()
        .map_err(|error| CliError::Io(format!("could not open the search index: {error}")))?;
    view.reader.replay(&mut episodic).map_err(log_failure)?;
    let seqs = episodic
        .search(text)
        .map_err(|error| CliError::Io(format!("search failed: {error}")))?;
    let hits: Vec<_> = seqs.iter().filter_map(|seq| episodic.get(*seq)).collect();
    // The search index records a claim id only for claim events, so take the
    // id each hit records or concerns from the ledger index instead.
    let entity_id = |seq: u64| -> Option<&str> {
        let row = view.index.events.get(usize::try_from(seq).ok()?)?;
        if row.recorded.seq == seq {
            row.entity_id()
        } else {
            None
        }
    };
    if json {
        let value: Vec<Value> = hits
            .iter()
            .map(|hit| {
                json!({
                    "seq": hit.seq,
                    "kind": hit.kind,
                    "id": entity_id(hit.seq),
                    "claim_id": hit.claim_id,
                    "body": hit.body,
                    "agent": hit.agent,
                    "source": hit.source,
                })
            })
            .collect();
        writeln!(out, "{}", serde_json::to_string_pretty(&value)?)?;
        return Ok(());
    }
    if hits.is_empty() {
        writeln!(out, "no matches")?;
    }
    for hit in hits {
        let text = match entity_id(hit.seq) {
            Some(id) => format!("{id}: {}", preview(&hit.body, 70)),
            None => preview(&hit.body, 80),
        };
        writeln!(out, "{:>5}  {:<28}  {text}", hit.seq, hit.kind)?;
    }
    Ok(())
}

fn standing(
    store: &Store,
    claim_id: &str,
    policy: &str,
    explain: bool,
    json: bool,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Result<(), CliError> {
    let policy = PolicyChoice::parse(policy)?;
    let view = read_view(store, true, err)?;
    if !view.index.claims.contains_key(claim_id) {
        return Err(usage(format!("no claim has id `{claim_id}`")));
    }
    let context = view
        .context
        .as_ref()
        .expect("standing context was requested");
    let resolution = Resolver::new(context)?.resolve(policy, claim_id)?;
    let status = match resolution.governed {
        Some(status) => format!("{status:?}"),
        None => "no governed standing".to_owned(),
    };
    if json {
        let value = if explain {
            resolution.explanation
        } else {
            json!({
                "claim_id": claim_id,
                "policy": policy.id(),
                "governed_standing": resolution.governed,
                "event_count": view.summary.event_count(),
                "tip": view.summary.tip().to_hex(),
            })
        };
        writeln!(out, "{}", serde_json::to_string_pretty(&value)?)?;
        return Ok(());
    }
    writeln!(out, "{claim_id} under {}: {status}", policy.id())?;
    let lines = trace_lines(&resolution.explanation);
    if explain {
        for line in &lines {
            writeln!(out, "  {line}")?;
        }
        writeln!(out, "  (the full explanation: add --json)")?;
    } else {
        for line in lines.iter().filter(|line| line.starts_with("blocker: ")) {
            writeln!(out, "  {line}")?;
        }
    }
    Ok(())
}

fn log(
    store: &Store,
    json: bool,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Result<(), CliError> {
    let view = read_view(store, false, err)?;
    if json {
        let value: Vec<Value> = view
            .index
            .events
            .iter()
            .map(|event| {
                json!({
                    "seq": event.recorded.seq,
                    "timestamp_nanos": event.recorded.timestamp_nanos,
                    "kind": event.kind,
                    "subject": event.subject,
                    "agent": event.recorded.agent,
                    "source": event.recorded.source,
                    "hash": event.recorded.hash,
                })
            })
            .collect();
        writeln!(out, "{}", serde_json::to_string_pretty(&value)?)?;
        return Ok(());
    }
    for event in &view.index.events {
        writeln!(
            out,
            "{:>5}  {}  {:<28}  {}",
            event.recorded.seq,
            utc(event.recorded.timestamp_nanos),
            event.kind,
            preview(&event.subject, 60)
        )?;
    }
    Ok(())
}

fn verify(store: &Store, json: bool, out: &mut dyn Write) -> Result<(), CliError> {
    let key = store.verifying_key();
    let key_hex = hex::encode(key.as_bytes());
    let _store_lock = store.lock(LockMode::Shared)?;
    // A checkpoint problem is reported in its own row, like the others, so the
    // signature and portable rows need nothing but the log and the key.
    let checkpoint_lock = store.lock_checkpoint(LockMode::Shared);
    let snapshot = Snapshot::read(&store.log_path())?;
    let reader = snapshot.reader(key);

    let (signatures, event_count, tip) = match LedgerIndex::replay(&reader) {
        // Both verifiers accept an empty history, but a store's log must
        // start with the genesis event that binds it to the key.
        Ok(_) if snapshot.is_empty() => (
            "failed: the log is empty, so no genesis event binds it to this key".to_owned(),
            Some(0),
            None,
        ),
        Ok((_, summary)) => (
            "ok".to_owned(),
            Some(summary.event_count()),
            Some(summary.tip().to_hex()),
        ),
        Err(error) => (format!("failed: {}", log_failure(error)), None, None),
    };
    let portable = match snapshot.portable_accepts(&store.scratch_dirs(), key) {
        Ok(true) => "accept".to_owned(),
        Ok(false) => "reject".to_owned(),
        Err(error) => format!("could not run: {error}"),
    };
    // Checkpoints are keyed by verifying key, so an external key finds the
    // store's checkpoint only when it is the store's real key.
    let checkpoint = match checkpoint_lock.and_then(|_lock| store.check_checkpoint(&reader)) {
        Ok(CheckpointState::Contained(saved)) => {
            format!("contained (saved at event {})", saved.event_count)
        }
        Ok(CheckpointState::Missing) => "none saved".to_owned(),
        Err(error) => format!("failed: {error}"),
    };
    let ok = signatures == "ok" && portable == "accept" && !checkpoint.starts_with("failed");

    if json {
        let value = json!({
            "ok": ok,
            "log": store.log_path().display().to_string(),
            "verifying_key": key_hex,
            "event_count": event_count,
            "tip": tip,
            "signatures": signatures,
            "portable": portable,
            "checkpoint": checkpoint,
        });
        writeln!(out, "{}", serde_json::to_string_pretty(&value)?)?;
    } else {
        writeln!(out, "log         {}", store.log_path().display())?;
        writeln!(out, "key         {key_hex}")?;
        if let (Some(count), Some(tip)) = (event_count, &tip) {
            writeln!(out, "events      {count}, tip {tip}")?;
        }
        writeln!(out, "signatures  {signatures}")?;
        writeln!(out, "portable    {portable}")?;
        writeln!(out, "checkpoint  {checkpoint}")?;
    }
    if ok {
        Ok(())
    } else {
        Err(CliError::Refused("verification failed".into()))
    }
}

fn export(
    store: &Store,
    format: &str,
    policy: &str,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Result<(), CliError> {
    if format != "desk-v0" {
        return Err(usage(format!(
            "unknown export format `{format}`; the only format is desk-v0"
        )));
    }
    let policy = PolicyChoice::parse(policy)?;
    let view = read_view(store, true, err)?;
    let context = view
        .context
        .as_ref()
        .expect("standing context was requested");
    let resolver = Resolver::new(context)?;
    let export = desk_v0(
        &view.index,
        view.summary,
        store.verifying_key(),
        policy,
        &resolver,
        err,
    )?;
    writeln!(out, "{}", serde_json::to_string_pretty(&export)?)?;
    Ok(())
}

fn checkpoint(
    store: &Store,
    accept_current: bool,
    json: bool,
    out: &mut dyn Write,
) -> Result<(), CliError> {
    if accept_current {
        let _store_lock = store.lock(LockMode::Exclusive)?;
        let _checkpoint_lock = store.lock_checkpoint(LockMode::Exclusive)?;
        let log_path = store.log_path();
        let snapshot = Snapshot::read_for_write(store)?;
        let (_, summary) =
            LedgerIndex::replay(&snapshot.reader(store.verifying_key())).map_err(log_failure)?;
        // This is also the recovery step after a failed sync, so the log may
        // still be only in the page cache: flush it before the checkpoint that
        // names it, and check that what reached disk is what was verified.
        store.sync_log()?;
        if std::fs::read(&log_path)? != snapshot.bytes() {
            return Err(CliError::Refused(
                "the log changed while this command was running; no checkpoint was saved".into(),
            ));
        }
        store.save_checkpoint(summary.event_count(), summary.tip())?;
        if json {
            writeln!(
                out,
                "{}",
                json!({"event_count": summary.event_count(), "tip": summary.tip().to_hex()})
            )?;
        } else {
            writeln!(
                out,
                "checkpoint saved at event {}, tip {}; later commands will refuse any log that \
                 does not contain it",
                summary.event_count(),
                summary.tip()
            )?;
        }
        return Ok(());
    }
    let _store_lock = store.lock(LockMode::Shared)?;
    let _checkpoint_lock = store.lock_checkpoint(LockMode::Shared)?;
    let snapshot = Snapshot::read_checked(store)?;
    let state = store.check_checkpoint(&snapshot.reader(store.verifying_key()))?;
    match state {
        CheckpointState::Contained(saved) => {
            if json {
                writeln!(
                    out,
                    "{}",
                    json!({
                        "saved": true,
                        "contained": true,
                        "event_count": saved.event_count,
                        "tip": saved.tip.to_hex(),
                        "path": saved.path.display().to_string(),
                    })
                )?;
            } else {
                writeln!(
                    out,
                    "checkpoint at event {} is contained in the log ({})",
                    saved.event_count,
                    saved.path.display()
                )?;
            }
        }
        CheckpointState::Missing => {
            if json {
                writeln!(out, "{}", json!({"saved": false}))?;
            } else {
                writeln!(
                    out,
                    "no checkpoint is saved for this log; run `magpie checkpoint --accept-current` \
                     to save one"
                )?;
            }
        }
    }
    Ok(())
}
