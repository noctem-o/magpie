//! Readable lines from a resolver's serialized explanation.
//!
//! Every standing policy serializes its audit trail with the same few field
//! names: `reasons` on a candidate path, `rule` with `achieved_standing` on an
//! application, and `blockers` at claim level. This walker prints exactly
//! those, in the order the resolver produced them, and invents nothing: a line
//! is a compact restatement of one recorded field. The full structure is
//! always available through `--json`.

use std::collections::BTreeSet;

use serde_json::Value;

pub(crate) fn trace_lines(explanation: &Value) -> Vec<String> {
    let mut lines = Vec::new();
    let mut seen = BTreeSet::new();
    walk(explanation, &mut lines, &mut seen);
    lines
}

fn walk(value: &Value, lines: &mut Vec<String>, seen: &mut BTreeSet<String>) {
    match value {
        Value::Object(map) => {
            if let Some(Value::Array(reasons)) = map.get("reasons") {
                let label = match map.get("edge_id") {
                    Some(Value::String(edge)) => format!("edge {edge}"),
                    _ => "claim".to_owned(),
                };
                push(lines, seen, format!("{label}: {}", join(reasons)));
            }
            if let (Some(rule), Some(achieved)) = (map.get("rule"), map.get("achieved_standing")) {
                let achieved = match achieved {
                    Value::Null => "no standing".to_owned(),
                    other => compact(other),
                };
                push(lines, seen, format!("rule {}: {achieved}", compact(rule)));
            }
            // Candidates before the applications made on them, whatever order
            // the map happens to iterate in.
            let mut children: Vec<(&String, &Value)> = map
                .iter()
                .filter(|(key, _)| *key != "reasons" && *key != "blockers")
                .collect();
            children.sort_by(|(a, _), (b, _)| {
                (child_rank(a), a.as_str()).cmp(&(child_rank(b), b.as_str()))
            });
            for (_, child) in children {
                walk(child, lines, seen);
            }
            if let Some(Value::Array(blockers)) = map.get("blockers") {
                for blocker in blockers {
                    push(lines, seen, format!("blocker: {}", compact(blocker)));
                }
            }
        }
        Value::Array(items) => {
            for item in items {
                walk(item, lines, seen);
            }
        }
        _ => {}
    }
}

fn child_rank(key: &str) -> u8 {
    match key {
        "candidate" => 0,
        "application" | "applications" => 2,
        _ => 1,
    }
}

fn push(lines: &mut Vec<String>, seen: &mut BTreeSet<String>, line: String) {
    if seen.insert(line.clone()) {
        lines.push(line);
    }
}

fn compact(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        other => other.to_string(),
    }
}

fn join(items: &[Value]) -> String {
    items.iter().map(compact).collect::<Vec<_>>().join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn restates_reasons_rules_and_blockers_without_duplicates() {
        let explanation = json!({
            "claim_id": "claim-1",
            "trace": [
                {
                    "candidate": {"edge_id": "edge-3", "reasons": ["accepted_candidate"]},
                    "application": {"rule": "deadbolt_occurrence_inclusion_v1", "achieved_standing": null}
                },
                {"edge_id": null, "reasons": ["missing_claim_domain"]}
            ],
            "blockers": ["ceiling_is_candidate_only", "ceiling_is_candidate_only"]
        });
        assert_eq!(
            trace_lines(&explanation),
            vec![
                "edge edge-3: accepted_candidate",
                "rule deadbolt_occurrence_inclusion_v1: no standing",
                "claim: missing_claim_domain",
                "blocker: ceiling_is_candidate_only",
            ]
        );
    }
}
