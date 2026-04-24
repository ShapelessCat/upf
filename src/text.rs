use crate::error::UpfError;
use serde::{Deserialize, Deserializer, Serializer};

/// Parse a whitespace-delimited UPF numeric field into `f64` values.
pub fn parse_f64_vec(input: &str) -> Result<Vec<f64>, UpfError> {
    input
        .split_whitespace()
        .map(|token| {
            token.parse::<f64>().map_err(|_| UpfError::InvalidFloat {
                token: token.into(),
            })
        })
        .collect()
}

/// Format numeric values the same compact way used by this crate's serializer.
pub fn format_f64_slice(values: &[f64]) -> String {
    values
        .iter()
        .map(|value| {
            let mut text = value.to_string();
            if text.ends_with(".0") {
                text.truncate(text.len() - 2);
            }
            text
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Parse a UPF logical flag such as `T` or `F`.
pub fn parse_bool_flag(input: &str) -> Result<bool, UpfError> {
    match input.trim().to_ascii_uppercase().as_str() {
        "T" | "TRUE" => Ok(true),
        "F" | "FALSE" => Ok(false),
        other => Err(UpfError::InvalidBoolFlag {
            value: other.to_string(),
        }),
    }
}

/// Format a Rust boolean as the compact UPF flag used in this crate, `T` or `F`.
pub fn format_bool_flag(value: bool) -> &'static str {
    if value { "T" } else { "F" }
}

/// Deserialize a UPF logical flag into a Rust `bool`.
pub(crate) fn deserialize_bool_flag<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let text = String::deserialize(deserializer)?;
    parse_bool_flag(&text).map_err(serde::de::Error::custom)
}

/// Serialize a Rust `bool` as the compact UPF `T`/`F` representation.
pub(crate) fn serialize_bool_flag<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(format_bool_flag(*value))
}

#[cfg(test)]
mod tests {
    use super::{UpfError, format_bool_flag, format_f64_slice, parse_bool_flag, parse_f64_vec};

    #[test]
    fn parses_free_format_float_vectors() {
        let values = parse_f64_vec("0.0  1.5\n2.25 3.75").unwrap();
        assert_eq!(values, vec![0.0, 1.5, 2.25, 3.75]);
    }

    #[test]
    fn rejects_invalid_float_vectors() {
        let err = parse_f64_vec("1.0 nope 3.0").unwrap_err();
        assert!(matches!(
            err,
            UpfError::InvalidFloat { token } if token == "nope"
        ));
    }

    #[test]
    fn parses_upf_boolean_flags() {
        assert!(parse_bool_flag("T").unwrap());
        assert!(parse_bool_flag("true").unwrap());
        assert!(!parse_bool_flag("F").unwrap());
        assert!(!parse_bool_flag("FALSE").unwrap());
    }

    #[test]
    fn rejects_invalid_boolean_flags() {
        let err = parse_bool_flag("maybe").unwrap_err();
        assert!(matches!(
            err,
            UpfError::InvalidBoolFlag { value } if value == "MAYBE"
        ));
    }

    #[test]
    fn formats_values_for_round_trip_output() {
        assert_eq!(format_bool_flag(true), "T");
        assert_eq!(format_bool_flag(false), "F");
        assert_eq!(format_f64_slice(&[1.0, 2.5, 3.0]), "1 2.5 3");
    }
}
