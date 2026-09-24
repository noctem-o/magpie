//! Command-line parsing with the standard library only.
//!
//! Options may appear anywhere after the program name. Free text (a note, a
//! claim statement, a search) is every remaining positional word joined by
//! spaces, so quoting is optional. Arguments after `--` are always positional,
//! for text that itself starts with `--`.

use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::path::PathBuf;

use crate::policy::POLICY_HINT;
use crate::CliError;

const VALUE_OPTIONS: &[&str] = &[
    "agent",
    "domain",
    "file",
    "format",
    "id",
    "kind",
    "locator",
    "policy",
    "rationale",
    "scope",
    "source",
    "source-uri",
    "store",
    "verifying-key",
];
const SWITCHES: &[&str] = &["accept-current", "help", "json", "version"];

/// One parsed invocation.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Invocation {
    pub store: Option<PathBuf>,
    pub json: bool,
    pub command: Command,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Command {
    Help,
    Version,
    Init {
        dir: PathBuf,
        agent: Option<String>,
    },
    Note {
        text: String,
        source: Option<String>,
    },
    Claim {
        statement: String,
        domain: String,
        scope: Option<String>,
        id: Option<String>,
        source: Option<String>,
    },
    Evidence {
        summary: String,
        kind: String,
        scope: Option<String>,
        id: Option<String>,
        file: Option<PathBuf>,
        source_uri: Option<String>,
        locator: Option<String>,
        source: Option<String>,
    },
    Link {
        kind: String,
        from: String,
        to: String,
        rationale: String,
        scope: Option<String>,
        id: Option<String>,
        source: Option<String>,
    },
    Show {
        id: String,
    },
    Search {
        text: String,
    },
    Standing {
        claim: String,
        policy: String,
    },
    Why {
        claim: String,
        policy: String,
    },
    Log,
    Verify {
        verifying_key: Option<String>,
    },
    Export {
        format: String,
        policy: String,
    },
    Checkpoint {
        accept_current: bool,
    },
}

#[derive(Default)]
struct Raw {
    positionals: Vec<String>,
    values: BTreeMap<String, String>,
    switches: BTreeSet<String>,
}

fn usage(message: impl Into<String>) -> CliError {
    CliError::Usage(message.into())
}

fn tokenize(args: &[OsString]) -> Result<Raw, CliError> {
    let mut raw = Raw::default();
    let mut iter = args.iter();
    let mut literal = false;
    while let Some(arg) = iter.next() {
        let arg = arg
            .to_str()
            .ok_or_else(|| usage("arguments must be valid UTF-8"))?;
        if literal || !arg.starts_with("--") {
            raw.positionals.push(arg.to_owned());
            continue;
        }
        if arg == "--" {
            literal = true;
            continue;
        }
        let body = &arg[2..];
        let (name, inline) = match body.split_once('=') {
            Some((name, value)) => (name, Some(value)),
            None => (body, None),
        };
        if VALUE_OPTIONS.contains(&name) {
            let value = match inline {
                Some(value) => value.to_owned(),
                None => {
                    let value = iter
                        .next()
                        .and_then(|value| value.to_str())
                        .ok_or_else(|| usage(format!("--{name} needs a value")))?;
                    // A following option is almost always a forgotten value,
                    // and taking it as one would record it in the log for good.
                    if value.starts_with("--") {
                        return Err(usage(format!(
                            "--{name} needs a value, but the next argument is the option \
                             {value}; to give a value that starts with --, write \
                             --{name}={value}"
                        )));
                    }
                    value.to_owned()
                }
            };
            if raw.values.insert(name.to_owned(), value).is_some() {
                return Err(usage(format!("--{name} was given twice")));
            }
        } else if SWITCHES.contains(&name) {
            if inline.is_some() {
                return Err(usage(format!("--{name} takes no value")));
            }
            raw.switches.insert(name.to_owned());
        } else {
            return Err(usage(format!("unknown option --{name}; run `magpie help`")));
        }
    }
    Ok(raw)
}

/// Options not yet claimed by the command being built.
struct Options {
    values: BTreeMap<String, String>,
    switches: BTreeSet<String>,
}

impl Options {
    fn take(&mut self, name: &str) -> Option<String> {
        self.values.remove(name)
    }

    fn require(&mut self, name: &str, hint: &str) -> Result<String, CliError> {
        self.take(name).ok_or_else(|| usage(hint.to_owned()))
    }

    fn switch(&mut self, name: &str) -> bool {
        self.switches.remove(name)
    }

    /// Refuse options the command doesn't use, rather than silently ignoring them.
    fn finish(self, command: &str) -> Result<(), CliError> {
        let leftover: Vec<String> = self
            .values
            .keys()
            .chain(self.switches.iter())
            .map(|name| format!("--{name}"))
            .collect();
        if leftover.is_empty() {
            Ok(())
        } else {
            Err(usage(format!(
                "{} does not apply to `{command}`",
                leftover.join(", ")
            )))
        }
    }
}

fn text(words: &[String], what: &str) -> Result<String, CliError> {
    let joined = words.join(" ");
    if joined.trim().is_empty() {
        Err(usage(format!("{what} is missing")))
    } else {
        Ok(joined)
    }
}

fn one(words: &[String], shape: &str) -> Result<String, CliError> {
    match words {
        [only] => Ok(only.clone()),
        _ => Err(usage(format!("expected `magpie {shape}`"))),
    }
}

fn none(words: &[String], command: &str) -> Result<(), CliError> {
    if words.is_empty() {
        Ok(())
    } else {
        Err(usage(format!(
            "`{command}` takes no arguments, but got `{}`",
            words.join(" ")
        )))
    }
}

/// Parse the arguments after the program name.
pub(crate) fn parse(args: &[OsString]) -> Result<Invocation, CliError> {
    let raw = tokenize(args)?;
    let mut options = Options {
        values: raw.values,
        switches: raw.switches,
    };
    let json = options.switch("json");
    let store = options.take("store").map(PathBuf::from);
    let mut positionals = raw.positionals.into_iter();
    let name = positionals.next();

    if options.switch("version") {
        return Ok(Invocation {
            store,
            json,
            command: Command::Version,
        });
    }
    let wants_help = options.switch("help");
    let name = match name {
        Some(name) if !wants_help && name != "help" && name != "-h" => name,
        _ => {
            return Ok(Invocation {
                store,
                json,
                command: Command::Help,
            })
        }
    };
    let rest: Vec<String> = positionals.collect();

    let command = match name.as_str() {
        "init" => Command::Init {
            dir: PathBuf::from(one(&rest, "init <dir>")?),
            agent: options.take("agent"),
        },
        "note" => Command::Note {
            text: text(&rest, "the note text")?,
            source: options.take("source"),
        },
        "claim" => Command::Claim {
            statement: text(&rest, "the claim statement")?,
            domain: options.require(
                "domain",
                "a claim needs --domain <domain>; run `magpie help` for the list",
            )?,
            scope: options.take("scope"),
            id: options.take("id"),
            source: options.take("source"),
        },
        "evidence" => Command::Evidence {
            summary: text(&rest, "the evidence summary")?,
            kind: options.require(
                "kind",
                "evidence needs --kind <kind>; run `magpie help` for the list",
            )?,
            scope: options.take("scope"),
            id: options.take("id"),
            file: options.take("file").map(PathBuf::from),
            source_uri: options.take("source-uri"),
            locator: options.take("locator"),
            source: options.take("source"),
        },
        "link" => {
            let [kind, from, to] = <[String; 3]>::try_from(rest.clone()).map_err(|_| {
                usage("expected `magpie link <kind> <from-id> <to-id> --rationale <text>`")
            })?;
            Command::Link {
                kind,
                from,
                to,
                rationale: options.require("rationale", "a link needs --rationale <text>")?,
                scope: options.take("scope"),
                id: options.take("id"),
                source: options.take("source"),
            }
        }
        "show" => Command::Show {
            id: one(&rest, "show <id>")?,
        },
        "search" => Command::Search {
            text: text(&rest, "the search text")?,
        },
        "standing" => Command::Standing {
            claim: one(&rest, "standing <claim-id> --policy <v0..v4>")?,
            policy: options.require("policy", POLICY_HINT)?,
        },
        "why" => Command::Why {
            claim: one(&rest, "why <claim-id> --policy <v0..v4>")?,
            policy: options.require("policy", POLICY_HINT)?,
        },
        "log" => {
            none(&rest, "log")?;
            Command::Log
        }
        "verify" => {
            none(&rest, "verify")?;
            Command::Verify {
                verifying_key: options.take("verifying-key"),
            }
        }
        "export" => {
            none(&rest, "export")?;
            Command::Export {
                format: options.require("format", "export needs --format desk-v0")?,
                policy: options.require("policy", POLICY_HINT)?,
            }
        }
        "checkpoint" => {
            none(&rest, "checkpoint")?;
            Command::Checkpoint {
                accept_current: options.switch("accept-current"),
            }
        }
        other => {
            return Err(usage(format!(
                "unknown command `{other}`; run `magpie help`"
            )))
        }
    };
    options.finish(&name)?;
    Ok(Invocation {
        store,
        json,
        command,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_words(words: &[&str]) -> Result<Invocation, CliError> {
        let args: Vec<OsString> = words.iter().map(OsString::from).collect();
        parse(&args)
    }

    #[test]
    fn free_text_joins_words_and_options_go_anywhere() {
        let invocation = parse_words(&[
            "claim",
            "Theorem",
            "2",
            "--domain",
            "Interpretation",
            "holds",
            "--store=/tmp/s",
            "--json",
        ])
        .unwrap();
        assert!(invocation.json);
        assert_eq!(invocation.store, Some(PathBuf::from("/tmp/s")));
        assert_eq!(
            invocation.command,
            Command::Claim {
                statement: "Theorem 2 holds".into(),
                domain: "Interpretation".into(),
                scope: None,
                id: None,
                source: None,
            }
        );
    }

    #[test]
    fn double_dash_keeps_dashed_text_positional() {
        let invocation = parse_words(&["note", "--", "--not-an-option"]).unwrap();
        assert_eq!(
            invocation.command,
            Command::Note {
                text: "--not-an-option".into(),
                source: None
            }
        );
    }

    #[test]
    fn unknown_repeated_and_misplaced_options_are_usage_errors() {
        for words in [
            vec!["log", "--bogus"],
            vec!["claim", "x", "--domain", "A", "--domain", "B"],
            vec!["log", "--policy", "v0"],
            vec!["note", "x", "--json=yes"],
            vec!["frobnicate"],
        ] {
            assert!(
                matches!(parse_words(&words), Err(CliError::Usage(_))),
                "{words:?} should be a usage error"
            );
        }
    }

    #[test]
    fn standing_requires_an_explicit_policy() {
        let error = parse_words(&["standing", "claim-1"]).unwrap_err();
        assert!(error.to_string().contains("--policy"));
    }

    #[test]
    fn no_arguments_and_help_forms_show_help() {
        for words in [vec![], vec!["help"], vec!["-h"], vec!["log", "--help"]] {
            assert_eq!(parse_words(&words).unwrap().command, Command::Help);
        }
    }
}
