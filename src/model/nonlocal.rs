use serde::{Deserialize, Deserializer, Serialize};

use super::NumericArray;

/// Human-oriented `PP_INFO` section.
///
/// The UPF reference treats this section as informational rather than required
/// machine-readable input. This crate preserves both free-form body text and the
/// optional nested `PP_INPUTFILE`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpInfo {
    /// Free-form text directly inside `PP_INFO`.
    #[serde(rename = "$text", default)]
    pub body_text: String,
    /// Optional nested `PP_INPUTFILE` content.
    #[serde(rename = "PP_INPUTFILE", skip_serializing_if = "Option::is_none")]
    pub input_file: Option<String>,
}

/// A numbered `PP_BETA.n` projector entry inside `PP_NONLOCAL`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpBeta {
    /// Projector index from `@index`.
    #[serde(rename = "@index")]
    pub index: usize,
    /// Angular momentum from `@angular_momentum`.
    #[serde(rename = "@angular_momentum")]
    pub angular_momentum: usize,
    /// Cutoff radius from `@cutoff_radius`.
    #[serde(rename = "@cutoff_radius")]
    pub cutoff_radius: f64,
    /// Optional ultrasoft cutoff radius from `@ultrasoft_cutoff_radius`.
    #[serde(
        rename = "@ultrasoft_cutoff_radius",
        skip_serializing_if = "Option::is_none"
    )]
    pub ultrasoft_cutoff_radius: Option<f64>,
    /// Projector samples stored as whitespace-delimited body text.
    #[serde(rename = "$text", deserialize_with = "deserialize_f64_values")]
    pub values: Vec<f64>,
}

/// `PP_DIJ` matrix data stored as a flat numeric list.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PpDij {
    #[serde(rename = "$text", deserialize_with = "deserialize_f64_values")]
    pub values: Vec<f64>,
}

impl PpDij {
    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// `PP_NONLOCAL` section, containing projector nodes and optional `PP_DIJ`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PpNonlocal {
    /// Numbered `PP_BETA.n` projector entries.
    #[serde(rename = "$value", default)]
    pub betas: Vec<PpBetaNode>,
    /// Coupling matrix from `PP_DIJ`.
    #[serde(rename = "PP_DIJ", default, skip_serializing_if = "PpDij::is_empty")]
    pub dij: PpDij,
}

/// Supported numbered `PP_BETA.n` tags inside `PP_NONLOCAL`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PpBetaNode {
    #[serde(rename = "PP_BETA.1")]
    Beta1(PpBeta),
    #[serde(rename = "PP_BETA.2")]
    Beta2(PpBeta),
    #[serde(rename = "PP_BETA.3")]
    Beta3(PpBeta),
}

/// `PP_PSWFC` section containing numbered pseudo-wavefunctions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpPseudoWavefunctions {
    /// Numbered `PP_CHI.n` nodes.
    #[serde(rename = "$value", default)]
    pub orbitals: Vec<PpChiNode>,
}

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

/// Supported numbered `PP_CHI.n` tags inside `PP_PSWFC`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PpChiNode {
    #[serde(rename = "PP_CHI.1")]
    Chi1(PpWavefunction),
    #[serde(rename = "PP_CHI.2")]
    Chi2(PpWavefunction),
    #[serde(rename = "PP_CHI.3")]
    Chi3(PpWavefunction),
}

/// A wavefunction-like numeric section used by several UPF containers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpWavefunction {
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

/// Alias for the `PP_NLCC` nonlinear core correction array.
pub type PpNlcc = NumericArray;

fn deserialize_f64_values<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let text = String::deserialize(deserializer)?;
    text.split_whitespace()
        .map(|token| token.parse::<f64>().map_err(serde::de::Error::custom))
        .collect()
}
