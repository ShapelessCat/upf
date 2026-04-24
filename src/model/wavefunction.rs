use serde::{Deserialize, Serialize};

use super::UpfDataType;
use super::numeric_text::deserialize_f64_values;

/// A wavefunction-like numeric section used by several UPF containers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpWavefunction {
    /// Optional UPF numeric type from `@type`.
    #[serde(rename = "@type", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<UpfDataType>,
    /// Optional declared element count from `@size`.
    #[serde(rename = "@size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<usize>,
    /// Optional display column hint from `@columns`.
    #[serde(rename = "@columns", default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<usize>,
    /// Orbital label from `@label`.
    #[serde(rename = "@label")]
    pub label: String,
    /// Angular momentum from `@l`.
    #[serde(rename = "@l")]
    pub l: usize,
    /// Optional occupation from `@occupation`.
    #[serde(rename = "@occupation", skip_serializing_if = "Option::is_none")]
    pub occupation: Option<f64>,
    /// Orbital samples stored as whitespace-delimited body text.
    #[serde(rename = "$text", deserialize_with = "deserialize_f64_values")]
    pub values: Vec<f64>,
}
