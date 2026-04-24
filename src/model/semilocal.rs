use serde::{Deserialize, Serialize};

use super::numeric_text::deserialize_f64_values;

/// `PP_SEMILOCAL` section containing semilocal potential channels.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpSemilocal {
    /// Numbered `PP_VNLn` channels.
    #[serde(rename = "$value", default)]
    pub channels: Vec<PpSemilocalChannel>,
}

/// Supported numbered `PP_VNLn` tags inside `PP_SEMILOCAL`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PpSemilocalChannel {
    #[serde(rename = "PP_VNL1")]
    Vnl1(PpSemilocalValues),
    #[serde(rename = "PP_VNL2")]
    Vnl2(PpSemilocalValues),
    #[serde(rename = "PP_VNL3")]
    Vnl3(PpSemilocalValues),
}

/// One semilocal channel body from `PP_VNLn`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpSemilocalValues {
    /// Angular momentum from `@L`.
    #[serde(rename = "@L")]
    pub l: usize,
    /// Optional total angular momentum from `@J`.
    #[serde(rename = "@J", skip_serializing_if = "Option::is_none")]
    pub j: Option<f64>,
    /// Channel samples stored as whitespace-delimited body text.
    #[serde(rename = "$text", deserialize_with = "deserialize_f64_values")]
    pub values: Vec<f64>,
}
