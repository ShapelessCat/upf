use serde::{Deserialize, Serialize};

/// Human-oriented `PP_INFO` section.
///
/// The UPF reference treats this section as informational rather than required
/// machine-readable input. This crate preserves both free-form body text and
/// nested `PP_INPUTFILE`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpInfo {
    /// Free-form text directly inside `PP_INFO`.
    #[serde(rename = "$text", default)]
    pub body_text: String,
    /// Nested `PP_INPUTFILE` content.
    #[serde(rename = "PP_INPUTFILE", skip_serializing_if = "Option::is_none")]
    pub input_file: Option<String>,
}
