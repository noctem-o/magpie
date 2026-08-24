use serde::Deserialize;
use serde_json::value::RawValue;

/// An opaque failure from the complete syntax pass.
///
/// The host parser's category, location, and diagnostic string are
/// deliberately discarded. The caller knows this failure occurred in the
/// syntax phase and maps it to the frozen `JsonSyntax` class.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct JsonSyntaxError;

pub(super) fn recognize(candidate: &str) -> Result<&RawValue, JsonSyntaxError> {
    let mut deserializer = serde_json::Deserializer::from_slice(candidate.as_bytes());
    let raw = <&RawValue>::deserialize(&mut deserializer).map_err(|_| JsonSyntaxError)?;
    deserializer.end().map_err(|_| JsonSyntaxError)?;
    Ok(raw)
}

#[cfg(test)]
mod characterization {
    use serde::de::{MapAccess, Visitor};
    use serde::{Deserialize, Deserializer};

    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct OrderedMembers(Vec<(String, String)>);

    impl<'de> Deserialize<'de> for OrderedMembers {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct OrderedMembersVisitor;

            impl<'de> Visitor<'de> for OrderedMembersVisitor {
                type Value = OrderedMembers;

                fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    formatter.write_str("a JSON object")
                }

                fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: MapAccess<'de>,
                {
                    let mut members = Vec::new();
                    while let Some(name) = map.next_key::<String>()? {
                        let value = map.next_value::<&RawValue>()?;
                        members.push((name, value.get().to_owned()));
                    }
                    Ok(OrderedMembers(members))
                }
            }

            deserializer.deserialize_map(OrderedMembersVisitor)
        }
    }

    fn raw_text(input: &str) -> Result<&str, JsonSyntaxError> {
        recognize(input).map(RawValue::get)
    }

    #[test]
    fn serde_json_1_0_150_and_raw_value_feature_are_pinned() {
        let lock = include_str!("../../../../Cargo.lock");
        assert!(lock.contains(concat!(
            "name = \"serde_json\"\n",
            "version = \"1.0.150\"\n",
            "source = \"registry+https://github.com/rust-lang/crates.io-index\"\n",
            "checksum = \"e8014e44b4736ed0538adeecded0fce2a272f22dc9578a7eb6b2d9993c74cfb9\"",
        )));

        let crate_manifest = include_str!("../../Cargo.toml");
        assert!(crate_manifest
            .contains("serde_json = { workspace = true, features = [\"raw_value\"] }"));
    }

    #[test]
    fn duplicate_names_and_values_remain_ordered_and_unmodified() {
        let input = r#"{"a":1,"a":2,"\u0061":3}"#;
        assert_eq!(raw_text(input), Ok(input));

        let mut deserializer = serde_json::Deserializer::from_str(input);
        let observed = OrderedMembers::deserialize(&mut deserializer).unwrap();
        deserializer.end().unwrap();
        assert_eq!(
            observed,
            OrderedMembers(vec![
                ("a".to_owned(), "1".to_owned()),
                ("a".to_owned(), "2".to_owned()),
                ("a".to_owned(), "3".to_owned()),
            ])
        );
    }

    #[test]
    fn raw_number_spelling_is_preserved_across_boundaries() {
        for spelling in [
            "18446744073709551615",
            "18446744073709551616",
            "999999999999999999999999999999999999999999999999999999999999999999",
            "1.0",
            "0.0",
            "1e0",
            "1E0",
            "1e+0",
            "1e-0",
        ] {
            assert_eq!(raw_text(spelling), Ok(spelling));
        }
    }

    #[test]
    fn forbidden_number_spellings_fail_the_complete_syntax_pass() {
        for spelling in ["+1", "+0", "01", "00", "NaN", "Infinity", "-Infinity"] {
            assert!(
                matches!(recognize(spelling), Err(JsonSyntaxError)),
                "{spelling}"
            );
        }
    }

    #[test]
    fn strings_preserve_the_characterized_surrogate_stage_split() {
        let escaped = r#""line\nquote\"slash\\tab\t""#;
        assert_eq!(
            serde_json::from_str::<String>(escaped).unwrap(),
            "line\nquote\"slash\\tab\t"
        );

        let pair = r#""\uD834\uDD1E""#;
        assert_eq!(raw_text(pair), Ok(pair));
        assert_eq!(serde_json::from_str::<String>(pair).unwrap(), "\u{1d11e}");

        for lone in [r#""\uD834""#, r#""\uDD1E""#] {
            assert_eq!(raw_text(lone), Ok(lone));
            assert!(serde_json::from_str::<String>(lone).is_err());
        }
    }

    #[test]
    fn top_level_shape_is_not_owned_by_the_syntax_pass() {
        for input in ["[]", "[null]", "null", "true", "42", r#""text""#] {
            assert_eq!(raw_text(input), Ok(input));
        }
    }

    #[test]
    fn explicit_end_check_rejects_second_values_and_trailing_garbage() {
        let mut without_end = serde_json::Deserializer::from_str("{} []");
        let first = <&RawValue>::deserialize(&mut without_end).unwrap();
        assert_eq!(first.get(), "{}");
        assert!(without_end.end().is_err());

        for input in ["{} []", "{} true", "{} trailing"] {
            assert!(matches!(recognize(input), Err(JsonSyntaxError)));
        }
    }

    #[test]
    fn raw_syntax_scan_is_iterative_at_characterized_depth() {
        let depth = 1_024;
        let input = format!("{}0{}", "[".repeat(depth), "]".repeat(depth));
        assert_eq!(raw_text(&input), Ok(input.as_str()));
    }

    #[test]
    fn parser_diagnostic_text_does_not_select_the_normative_class() {
        let first_host_error = serde_json::from_str::<serde_json::Value>("{")
            .unwrap_err()
            .to_string();
        let second_host_error = serde_json::from_str::<serde_json::Value>("[1,")
            .unwrap_err()
            .to_string();
        assert_ne!(first_host_error, second_host_error);
        assert!(matches!(recognize("{"), Err(JsonSyntaxError)));
        assert!(matches!(recognize("[1,"), Err(JsonSyntaxError)));
    }
}
