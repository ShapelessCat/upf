use std::fmt::Write;

use serde::{Deserialize, Deserializer};

use crate::error::UpfError;

pub(super) fn parse_f64_token(token: &str) -> Result<f64, UpfError> {
    let token = token.trim();
    let normalized = if token.contains(['d', 'D']) {
        token
            .chars()
            .map(|ch| match ch {
                'd' => 'e',
                'D' => 'E',
                _ => ch,
            })
            .collect::<String>()
    } else {
        token.to_string()
    };

    normalized
        .parse::<f64>()
        .map_err(|_| UpfError::InvalidFloat {
            token: token.into(),
        })
}

/// Parse a whitespace-delimited UPF numeric field into `f64` values.
pub(crate) fn parse_f64_vec(input: &str) -> Result<Vec<f64>, UpfError> {
    input.split_whitespace().map(parse_f64_token).collect()
}

/// Format numeric values the same compact way used by this crate's serializer.
pub(crate) fn format_f64_slice(values: &[f64]) -> String {
    let mut out = String::new();
    for (i, value) in values.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        let _ = write!(out, "{value}");
        if out.ends_with(".0") {
            out.truncate(out.len() - 2);
        }
    }
    out
}

pub(crate) fn deserialize_f64_values<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let text = String::deserialize(deserializer)?;
    parse_f64_vec(&text).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use crate::error::UpfError;

    use super::{deserialize_f64_values, format_f64_slice, parse_f64_vec};

    #[test]
    fn parses_free_format_float_vectors() {
        let values = parse_f64_vec("0.0  1.5\n2.25 3.75").unwrap();
        assert_eq!(values, vec![0.0, 1.5, 2.25, 3.75]);
    }

    #[test]
    fn parses_fortran_double_precision_float_vectors() {
        let values = parse_f64_vec("1.d-12 0.0125D0 -7.d0").unwrap();
        assert_eq!(values, vec![1.0e-12, 0.0125, -7.0]);
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
    fn formats_values_for_round_trip_output() {
        assert_eq!(format_f64_slice(&[1.0, 2.5, 3.0]), "1 2.5 3");
    }

    #[test]
    fn formats_empty_slice_as_empty_string() {
        assert_eq!(format_f64_slice(&[]), "");
    }

    #[test]
    fn numeric_text_deserializer_reports_invalid_token() {
        #[derive(Debug, serde::Deserialize)]
        struct Wrapper {
            #[serde(rename = "$text", deserialize_with = "deserialize_f64_values")]
            _values: Vec<f64>,
        }
        let xml = "<Wrapper>1.0 nope 3.0</Wrapper>";
        let err = quick_xml::de::from_str::<Wrapper>(xml).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("nope"),
            "error should name the bad token, got: {msg}"
        );
    }
}
