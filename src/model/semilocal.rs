use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};

use super::{Numbered, numeric_text::deserialize_f64_values};

/// `PP_SEMILOCAL` section containing semilocal potential channels.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PpSemilocal {
    /// Numbered `PP_VNL.n` channels.
    #[serde(rename = "$value", default)]
    pub channels: Vec<Numbered<PpSemilocalValues>>,
}

impl Serialize for PpSemilocal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.channels.len()))?;
        for channel in &self.channels {
            map.serialize_entry(&channel.tag.to_string(), &channel.value)?;
        }
        map.end()
    }
}

/// One semilocal channel body.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpSemilocalValues {
    /// Angular momentum in attribute `l`.
    #[serde(rename = "@l", alias = "@L")]
    pub l: usize,
    /// Total angular momentum in attribute `j`.
    #[serde(rename = "@j", alias = "@J", skip_serializing_if = "Option::is_none")]
    pub j: Option<f64>,
    /// Channel samples stored as whitespace-delimited body text.
    #[serde(rename = "$text", deserialize_with = "deserialize_f64_values")]
    pub values: Vec<f64>,
}
