use serde::{Deserialize, Deserializer, Serialize};

use super::NumericArray;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpInfo {
    #[serde(rename = "$text", default)]
    pub body_text: String,
    #[serde(rename = "PP_INPUTFILE", skip_serializing_if = "Option::is_none")]
    pub input_file: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpBeta {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "@angular_momentum")]
    pub angular_momentum: usize,
    #[serde(rename = "@cutoff_radius")]
    pub cutoff_radius: f64,
    #[serde(
        rename = "@ultrasoft_cutoff_radius",
        skip_serializing_if = "Option::is_none"
    )]
    pub ultrasoft_cutoff_radius: Option<f64>,
    #[serde(rename = "$text", deserialize_with = "deserialize_f64_values")]
    pub values: Vec<f64>,
}

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PpNonlocal {
    #[serde(rename = "$value", default)]
    pub betas: Vec<PpBetaNode>,
    #[serde(rename = "PP_DIJ", default, skip_serializing_if = "PpDij::is_empty")]
    pub dij: PpDij,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PpBetaNode {
    #[serde(rename = "PP_BETA.1")]
    Beta1(PpBeta),
    #[serde(rename = "PP_BETA.2")]
    Beta2(PpBeta),
    #[serde(rename = "PP_BETA.3")]
    Beta3(PpBeta),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpPseudoWavefunctions {
    #[serde(rename = "$value", default)]
    pub orbitals: Vec<PpChiNode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpSemilocal {
    #[serde(rename = "$value", default)]
    pub channels: Vec<PpSemilocalChannel>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PpSemilocalChannel {
    #[serde(rename = "PP_VNL1")]
    Vnl1(PpSemilocalValues),
    #[serde(rename = "PP_VNL2")]
    Vnl2(PpSemilocalValues),
    #[serde(rename = "PP_VNL3")]
    Vnl3(PpSemilocalValues),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpSemilocalValues {
    #[serde(rename = "@L")]
    pub l: usize,
    #[serde(rename = "@J", skip_serializing_if = "Option::is_none")]
    pub j: Option<f64>,
    #[serde(rename = "$text", deserialize_with = "deserialize_f64_values")]
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PpChiNode {
    #[serde(rename = "PP_CHI.1")]
    Chi1(PpWavefunction),
    #[serde(rename = "PP_CHI.2")]
    Chi2(PpWavefunction),
    #[serde(rename = "PP_CHI.3")]
    Chi3(PpWavefunction),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpWavefunction {
    #[serde(rename = "@label")]
    pub label: String,
    #[serde(rename = "@l")]
    pub l: usize,
    #[serde(rename = "@occupation", skip_serializing_if = "Option::is_none")]
    pub occupation: Option<f64>,
    #[serde(rename = "$text", deserialize_with = "deserialize_f64_values")]
    pub values: Vec<f64>,
}

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
