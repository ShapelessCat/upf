use serde::{Deserialize, Deserializer};

use crate::error::UpfError;

use super::numeric_values::parse_f64_token;

/// Parse a UPF logical flag such as `T` or `F`.
fn parse_bool_flag(input: &str) -> Result<bool, UpfError> {
    match input.trim().to_ascii_uppercase().as_str() {
        "T" | "TRUE" | ".TRUE." => Ok(true),
        "F" | "FALSE" | ".FALSE." => Ok(false),
        other => Err(UpfError::InvalidBoolFlag {
            value: other.to_string(),
        }),
    }
}

/// Format a Rust boolean as the compact UPF flag used in this crate, `T` or `F`.
pub(crate) fn format_bool_flag(value: bool) -> &'static str {
    if value { "T" } else { "F" }
}

/// Serde adapter for UPF logical flags stored as `T`/`F`-style text.
pub(crate) mod bool_flag {
    use serde::{Deserialize, Deserializer, Serializer};

    use super::{format_bool_flag, parse_bool_flag};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;
        parse_bool_flag(&text).map_err(serde::de::Error::custom)
    }

    pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(format_bool_flag(*value))
    }
}

/// Deserialize a UPF real number, accepting both Rust `e` and Fortran `d`
/// exponent markers.
pub(crate) fn deserialize_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let text = String::deserialize(deserializer)?;
    parse_f64_token(&text).map_err(serde::de::Error::custom)
}

/// Deserialize an optional UPF real number when the field is present,
/// accepting both Rust `e` and Fortran `d` exponent markers.
pub(crate) fn deserialize_opt_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let text = String::deserialize(deserializer)?;
    parse_f64_token(&text)
        .map(Some)
        .map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use crate::error::UpfError;

    use super::{format_bool_flag, parse_bool_flag};

    #[test]
    fn parses_upf_boolean_flags() {
        assert!(parse_bool_flag("T").unwrap());
        assert!(parse_bool_flag("true").unwrap());
        assert!(parse_bool_flag(".true.").unwrap());
        assert!(!parse_bool_flag("F").unwrap());
        assert!(!parse_bool_flag("FALSE").unwrap());
        assert!(!parse_bool_flag(".false.").unwrap());
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
    fn formats_boolean_flags() {
        assert_eq!(format_bool_flag(true), "T");
        assert_eq!(format_bool_flag(false), "F");
    }
}
