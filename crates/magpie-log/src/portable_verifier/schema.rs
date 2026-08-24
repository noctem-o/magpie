use std::cell::Cell;
use std::fmt;

use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;
use serde_json::value::RawValue;

use crate::{ContentHash, EventCore, Payload, Provenance, Status};

use super::lexical::{decode_lower_hex, decode_u64_token};

const SCHEMA_REJECTION_MARKER: &str = "portable frontend schema rejection";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SchemaDecodeError {
    Rejected,
    InternalParserInconsistency,
}

pub(super) struct DecodedRecord {
    pub(super) core: EventCore,
    pub(super) stored_hash: [u8; 32],
    pub(super) signature: [u8; 64],
}

struct DecodeContext {
    rejected: Cell<bool>,
}

impl DecodeContext {
    fn new() -> Self {
        Self {
            rejected: Cell::new(false),
        }
    }

    fn mark_schema_rejection(&self) {
        self.rejected.set(true);
    }

    fn failure(&self) -> SchemaDecodeError {
        if self.rejected.get() {
            SchemaDecodeError::Rejected
        } else {
            SchemaDecodeError::InternalParserInconsistency
        }
    }
}

struct Member<'de> {
    name: String,
    value: &'de RawValue,
}

struct ObjectSeed<'context> {
    context: &'context DecodeContext,
}

struct ObjectVisitor<'context> {
    context: &'context DecodeContext,
}

impl<'de> DeserializeSeed<'de> for ObjectSeed<'_> {
    type Value = Vec<Member<'de>>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(ObjectVisitor {
            context: self.context,
        })
    }
}

impl<'de> Visitor<'de> for ObjectVisitor<'_> {
    type Value = Vec<Member<'de>>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a governed Magpie JSON object")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut members: Vec<Member<'de>> = Vec::new();
        loop {
            let name = match map.next_key::<String>() {
                Ok(Some(name)) => name,
                Ok(None) => break,
                Err(error) => {
                    // Pass 1 established JSON string syntax. Failure to turn a
                    // member name into a Unicode scalar string is Schema.
                    self.context.mark_schema_rejection();
                    return Err(error);
                }
            };

            if members.iter().any(|member| member.name == name) {
                self.context.mark_schema_rejection();
                return Err(de::Error::custom(SCHEMA_REJECTION_MARKER));
            }

            let value = map.next_value::<&'de RawValue>()?;
            members.push(Member { name, value });
        }
        Ok(members)
    }

    fn visit_bool<E>(self, _value: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.reject_wrong_shape()
    }

    fn visit_i64<E>(self, _value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.reject_wrong_shape()
    }

    fn visit_i128<E>(self, _value: i128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.reject_wrong_shape()
    }

    fn visit_u64<E>(self, _value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.reject_wrong_shape()
    }

    fn visit_u128<E>(self, _value: u128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.reject_wrong_shape()
    }

    fn visit_f64<E>(self, _value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.reject_wrong_shape()
    }

    fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.reject_wrong_shape()
    }

    fn visit_borrowed_str<E>(self, _value: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.reject_wrong_shape()
    }

    fn visit_string<E>(self, _value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.reject_wrong_shape()
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.reject_wrong_shape()
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.reject_wrong_shape()
    }

    fn visit_seq<A>(self, _sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        self.reject_wrong_shape()
    }
}

impl ObjectVisitor<'_> {
    fn reject_wrong_shape<T, E>(self) -> Result<T, E>
    where
        E: de::Error,
    {
        self.context.mark_schema_rejection();
        Err(E::custom(SCHEMA_REJECTION_MARKER))
    }
}

pub(super) fn decode_record(candidate: &str) -> Result<DecodedRecord, SchemaDecodeError> {
    let context = DecodeContext::new();
    let members = collect_object(candidate, &context)?;
    ensure_members(&members, &["core", "hash", "signature"], &context)?;

    let core = decode_event_core(required_member(&members, "core", &context)?, &context)?;
    let stored_hash = decode_hex_member::<32>(&members, "hash", &context)?;
    let signature = decode_hex_member::<64>(&members, "signature", &context)?;

    Ok(DecodedRecord {
        core,
        stored_hash,
        signature,
    })
}

fn decode_event_core(
    raw: &RawValue,
    context: &DecodeContext,
) -> Result<EventCore, SchemaDecodeError> {
    let members = collect_object(raw.get(), context)?;
    ensure_members(
        &members,
        &[
            "seq",
            "timestamp_nanos",
            "prev_hash",
            "provenance",
            "payload",
        ],
        context,
    )?;

    let seq = decode_u64_member(&members, "seq", context)?;
    let timestamp_nanos = decode_u64_member(&members, "timestamp_nanos", context)?;
    let prev_hash =
        ContentHash::from_bytes(decode_hex_member::<32>(&members, "prev_hash", context)?);
    let provenance = decode_provenance(required_member(&members, "provenance", context)?, context)?;
    let payload = decode_payload(required_member(&members, "payload", context)?, context)?;

    Ok(EventCore {
        seq,
        timestamp_nanos,
        prev_hash,
        provenance,
        payload,
    })
}

fn decode_provenance(
    raw: &RawValue,
    context: &DecodeContext,
) -> Result<Provenance, SchemaDecodeError> {
    let members = collect_object(raw.get(), context)?;
    ensure_members(&members, &["agent", "source"], context)?;
    Ok(Provenance {
        agent: decode_string_member(&members, "agent", context)?,
        source: decode_string_member(&members, "source", context)?,
    })
}

fn decode_payload(raw: &RawValue, context: &DecodeContext) -> Result<Payload, SchemaDecodeError> {
    let members = collect_object(raw.get(), context)?;
    let kind = decode_string(required_member(&members, "kind", context)?, context)?;

    match kind.as_str() {
        "Genesis" => {
            ensure_members(
                &members,
                &["kind", "canonicalization_profile", "verifying_key"],
                context,
            )?;
            Ok(Payload::Genesis {
                canonicalization_profile: decode_string_member(
                    &members,
                    "canonicalization_profile",
                    context,
                )?,
                verifying_key: decode_string_member(&members, "verifying_key", context)?,
            })
        }
        "ClaimAsserted" => {
            ensure_members(
                &members,
                &["kind", "claim_id", "statement", "status"],
                context,
            )?;
            Ok(Payload::ClaimAsserted {
                claim_id: decode_string_member(&members, "claim_id", context)?,
                statement: decode_string_member(&members, "statement", context)?,
                status: decode_status_member(&members, "status", context)?,
            })
        }
        "EvidenceRecorded" => {
            ensure_members(&members, &["kind", "claim_id", "summary"], context)?;
            Ok(Payload::EvidenceRecorded {
                claim_id: decode_string_member(&members, "claim_id", context)?,
                summary: decode_string_member(&members, "summary", context)?,
            })
        }
        "ClaimStatusChanged" => {
            ensure_members(
                &members,
                &["kind", "claim_id", "from", "to", "reason"],
                context,
            )?;
            Ok(Payload::ClaimStatusChanged {
                claim_id: decode_string_member(&members, "claim_id", context)?,
                from: decode_status_member(&members, "from", context)?,
                to: decode_status_member(&members, "to", context)?,
                reason: decode_string_member(&members, "reason", context)?,
            })
        }
        "Note" => {
            ensure_members(&members, &["kind", "text"], context)?;
            Ok(Payload::Note {
                text: decode_string_member(&members, "text", context)?,
            })
        }
        "SegmentAnchored" => {
            ensure_members(
                &members,
                &[
                    "kind",
                    "bundle_kind",
                    "witness_root",
                    "witness_algorithm",
                    "canonicalization_profile",
                    "run_id",
                ],
                context,
            )?;
            Ok(Payload::SegmentAnchored {
                bundle_kind: decode_string_member(&members, "bundle_kind", context)?,
                witness_root: decode_string_member(&members, "witness_root", context)?,
                witness_algorithm: decode_string_member(&members, "witness_algorithm", context)?,
                canonicalization_profile: decode_string_member(
                    &members,
                    "canonicalization_profile",
                    context,
                )?,
                run_id: decode_string_member(&members, "run_id", context)?,
            })
        }
        "ClaimAssertedV2" => {
            ensure_members(
                &members,
                &[
                    "kind",
                    "claim_id",
                    "statement",
                    "scope_ref",
                    "actor_class",
                    "content_hash",
                    "metadata_json",
                ],
                context,
            )?;
            Ok(Payload::ClaimAssertedV2 {
                claim_id: decode_string_member(&members, "claim_id", context)?,
                statement: decode_string_member(&members, "statement", context)?,
                scope_ref: decode_string_member(&members, "scope_ref", context)?,
                actor_class: decode_string_member(&members, "actor_class", context)?,
                content_hash: decode_string_member(&members, "content_hash", context)?,
                metadata_json: decode_string_member(&members, "metadata_json", context)?,
            })
        }
        "EvidenceRegistered" => {
            ensure_members(
                &members,
                &[
                    "kind",
                    "evidence_id",
                    "evidence_kind",
                    "summary",
                    "scope_ref",
                    "actor_class",
                    "content_hash",
                    "metadata_json",
                ],
                context,
            )?;
            Ok(Payload::EvidenceRegistered {
                evidence_id: decode_string_member(&members, "evidence_id", context)?,
                evidence_kind: decode_string_member(&members, "evidence_kind", context)?,
                summary: decode_string_member(&members, "summary", context)?,
                scope_ref: decode_string_member(&members, "scope_ref", context)?,
                actor_class: decode_string_member(&members, "actor_class", context)?,
                content_hash: decode_string_member(&members, "content_hash", context)?,
                metadata_json: decode_string_member(&members, "metadata_json", context)?,
            })
        }
        "JustificationEdgeRecorded" => {
            ensure_members(
                &members,
                &[
                    "kind",
                    "edge_id",
                    "edge_kind",
                    "source_id",
                    "target_id",
                    "scope_ref",
                    "actor_class",
                    "rationale",
                    "metadata_json",
                ],
                context,
            )?;
            Ok(Payload::JustificationEdgeRecorded {
                edge_id: decode_string_member(&members, "edge_id", context)?,
                edge_kind: decode_string_member(&members, "edge_kind", context)?,
                source_id: decode_string_member(&members, "source_id", context)?,
                target_id: decode_string_member(&members, "target_id", context)?,
                scope_ref: decode_string_member(&members, "scope_ref", context)?,
                actor_class: decode_string_member(&members, "actor_class", context)?,
                rationale: decode_string_member(&members, "rationale", context)?,
                metadata_json: decode_string_member(&members, "metadata_json", context)?,
            })
        }
        _ => reject(context),
    }
}

fn collect_object<'de>(
    source: &'de str,
    context: &DecodeContext,
) -> Result<Vec<Member<'de>>, SchemaDecodeError> {
    let mut deserializer = serde_json::Deserializer::from_str(source);
    let members = ObjectSeed { context }
        .deserialize(&mut deserializer)
        .map_err(|_| context.failure())?;
    deserializer
        .end()
        .map_err(|_| SchemaDecodeError::InternalParserInconsistency)?;
    Ok(members)
}

fn ensure_members(
    members: &[Member<'_>],
    required: &[&str],
    context: &DecodeContext,
) -> Result<(), SchemaDecodeError> {
    if members.len() != required.len()
        || members
            .iter()
            .any(|member| !required.contains(&member.name.as_str()))
        || required
            .iter()
            .any(|name| !members.iter().any(|member| member.name == *name))
    {
        return reject(context);
    }
    Ok(())
}

fn required_member<'de>(
    members: &[Member<'de>],
    name: &str,
    context: &DecodeContext,
) -> Result<&'de RawValue, SchemaDecodeError> {
    members
        .iter()
        .find(|member| member.name == name)
        .map(|member| member.value)
        .ok_or_else(|| {
            context.mark_schema_rejection();
            SchemaDecodeError::Rejected
        })
}

fn decode_string_member(
    members: &[Member<'_>],
    name: &str,
    context: &DecodeContext,
) -> Result<String, SchemaDecodeError> {
    decode_string(required_member(members, name, context)?, context)
}

fn decode_string(raw: &RawValue, context: &DecodeContext) -> Result<String, SchemaDecodeError> {
    let mut deserializer = serde_json::Deserializer::from_str(raw.get());
    let decoded = String::deserialize(&mut deserializer).map_err(|_| {
        context.mark_schema_rejection();
        SchemaDecodeError::Rejected
    })?;
    deserializer
        .end()
        .map_err(|_| SchemaDecodeError::InternalParserInconsistency)?;
    Ok(decoded)
}

fn decode_u64_member(
    members: &[Member<'_>],
    name: &str,
    context: &DecodeContext,
) -> Result<u64, SchemaDecodeError> {
    let raw = required_member(members, name, context)?;
    decode_u64_token(raw.get()).ok_or_else(|| {
        context.mark_schema_rejection();
        SchemaDecodeError::Rejected
    })
}

fn decode_hex_member<const LENGTH: usize>(
    members: &[Member<'_>],
    name: &str,
    context: &DecodeContext,
) -> Result<[u8; LENGTH], SchemaDecodeError> {
    let decoded_string = decode_string_member(members, name, context)?;
    decode_lower_hex::<LENGTH>(&decoded_string).ok_or_else(|| {
        context.mark_schema_rejection();
        SchemaDecodeError::Rejected
    })
}

fn decode_status_member(
    members: &[Member<'_>],
    name: &str,
    context: &DecodeContext,
) -> Result<Status, SchemaDecodeError> {
    match decode_string_member(members, name, context)?.as_str() {
        "Open" => Ok(Status::Open),
        "Conjectured" => Ok(Status::Conjectured),
        "Supported" => Ok(Status::Supported),
        "Settled" => Ok(Status::Settled),
        "Refuted" => Ok(Status::Refuted),
        _ => reject(context),
    }
}

fn reject<T>(context: &DecodeContext) -> Result<T, SchemaDecodeError> {
    context.mark_schema_rejection();
    Err(SchemaDecodeError::Rejected)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZERO_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";
    const ZERO_SIGNATURE: &str = concat!(
        "0000000000000000000000000000000000000000000000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000",
    );

    fn record_with(seq: &str, prev_hash: &str, payload: &str) -> String {
        format!(
            r#"{{"core":{{"seq":{seq},"timestamp_nanos":0,"prev_hash":"{prev_hash}","provenance":{{"agent":"a","source":"s"}},"payload":{payload}}},"hash":"{ZERO_HASH}","signature":"{ZERO_SIGNATURE}"}}"#
        )
    }

    #[test]
    fn raw_u64_and_core_hex_are_checked_before_conversion() {
        let payload = r#"{"kind":"Note","text":"ok"}"#;
        for value in [
            "0",
            "9007199254740993",
            "9223372036854775808",
            "18446744073709551615",
        ] {
            let decoded = decode_record(&record_with(value, ZERO_HASH, payload)).unwrap();
            assert_eq!(decoded.core.seq, value.parse::<u64>().unwrap());
        }

        for value in [
            "-1",
            "-0",
            "1.0",
            "1e0",
            "18446744073709551616",
            "99999999999999999999999999999999999999999999999999999999999",
            "true",
            "null",
            r#""1""#,
        ] {
            assert_eq!(
                decode_record(&record_with(value, ZERO_HASH, payload)).err(),
                Some(SchemaDecodeError::Rejected),
                "{value}"
            );
        }

        let uppercase = "A000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(
            decode_record(&record_with("0", uppercase, payload)).err(),
            Some(SchemaDecodeError::Rejected)
        );
    }

    #[test]
    fn escaped_equivalent_duplicate_member_is_schema() {
        let duplicate = format!(
            r#"{{"core":null,"c\u006fre":null,"hash":"{ZERO_HASH}","signature":"{ZERO_SIGNATURE}"}}"#
        );
        assert_eq!(
            decode_record(&duplicate).err(),
            Some(SchemaDecodeError::Rejected)
        );
    }

    #[test]
    fn payload_and_genesis_semantics_remain_deferred() {
        let payload = r#"{"kind":"ClaimAssertedV2","claim_id":"","statement":"","scope_ref":"","actor_class":"not-a-vocabulary-value","content_hash":"UPPERCASE","metadata_json":"not json"}"#;
        let decoded = decode_record(&record_with("0", ZERO_HASH, payload)).unwrap();
        match decoded.core.payload {
            Payload::ClaimAssertedV2 {
                claim_id,
                statement,
                scope_ref,
                actor_class,
                content_hash,
                metadata_json,
            } => {
                assert!(claim_id.is_empty());
                assert!(statement.is_empty());
                assert!(scope_ref.is_empty());
                assert_eq!(actor_class, "not-a-vocabulary-value");
                assert_eq!(content_hash, "UPPERCASE");
                assert_eq!(metadata_json, "not json");
            }
            _ => panic!("wrong payload variant"),
        }

        let genesis =
            r#"{"kind":"Genesis","canonicalization_profile":"wrong","verifying_key":"NOT HEX"}"#;
        assert!(decode_record(&record_with("0", ZERO_HASH, genesis)).is_ok());
    }

    #[test]
    fn status_is_schema_but_member_order_is_not() {
        let reordered = format!(
            r#"{{"signature":"{ZERO_SIGNATURE}","hash":"{ZERO_HASH}","core":{{"payload":{{"status":"Open","statement":"s","claim_id":"c","kind":"ClaimAsserted"}},"provenance":{{"source":"s","agent":"a"}},"prev_hash":"{ZERO_HASH}","timestamp_nanos":0,"seq":0}}}}"#
        );
        assert!(decode_record(&reordered).is_ok());

        let numeric_status =
            r#"{"kind":"ClaimAsserted","claim_id":"c","statement":"s","status":1}"#;
        assert_eq!(
            decode_record(&record_with("0", ZERO_HASH, numeric_status)).err(),
            Some(SchemaDecodeError::Rejected)
        );
    }

    #[test]
    fn lone_surrogate_is_schema_after_raw_syntax_success() {
        let payload = r#"{"kind":"Note","text":"\uD834"}"#;
        assert_eq!(
            decode_record(&record_with("0", ZERO_HASH, payload)).err(),
            Some(SchemaDecodeError::Rejected)
        );
    }
}
