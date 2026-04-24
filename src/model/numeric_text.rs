use serde::{Deserialize, Deserializer};

pub(crate) fn deserialize_f64_values<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let text = String::deserialize(deserializer)?;
    text.split_whitespace()
        .map(|token| token.parse::<f64>().map_err(serde::de::Error::custom))
        .collect()
}
