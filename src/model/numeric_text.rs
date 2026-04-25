use serde::{Deserialize, Deserializer};

use crate::text::parse_f64_vec;

pub(crate) fn deserialize_f64_values<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let text = String::deserialize(deserializer)?;
    parse_f64_vec(&text).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::deserialize_f64_values;

    #[test]
    fn numeric_text_deserializer_reports_invalid_token() {
        // Exercises the serde path used by PP_BETA, PP_CHI, etc.
        // After deduplication this should produce the same UpfError::InvalidFloat
        // message as parse_f64_vec.
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
