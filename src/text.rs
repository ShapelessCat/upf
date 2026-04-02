use crate::error::UpfError;

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

pub fn parse_bool_flag(input: &str) -> Result<bool, UpfError> {
    match input.trim().to_ascii_uppercase().as_str() {
        "T" | ".T." | "TRUE" | ".TRUE." => Ok(true),
        "F" | ".F." | "FALSE" | ".FALSE." => Ok(false),
        other => Err(UpfError::InvalidBoolFlag {
            value: other.to_string(),
        }),
    }
}

pub fn format_bool_flag(value: bool) -> &'static str {
    if value { "T" } else { "F" }
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
        assert!(parse_bool_flag(".TRUE.").unwrap());
        assert!(!parse_bool_flag("F").unwrap());
        assert!(!parse_bool_flag(".FALSE.").unwrap());
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
