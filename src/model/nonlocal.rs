use serde::{Deserialize, Serialize};

use super::NumericArray;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpInfo {
    #[serde(rename = "$text", default)]
    pub body_text: String,
    #[serde(rename = "PP_INPUTFILE")]
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
    #[serde(rename = "@ultrasoft_cutoff_radius")]
    pub ultrasoft_cutoff_radius: Option<f64>,
    #[serde(rename = "$text")]
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PpDij {
    #[serde(rename = "$text")]
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PpNonlocal {
    #[serde(rename = "$value", default)]
    pub betas: Vec<PpBetaNode>,
    #[serde(rename = "PP_DIJ", default)]
    pub dij: PpDij,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PpBetaNode {
    Beta1(#[serde(rename = "PP_BETA.1")] PpBeta),
    Beta2(#[serde(rename = "PP_BETA.2")] PpBeta),
    Beta3(#[serde(rename = "PP_BETA.3")] PpBeta),
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
#[serde(untagged)]
pub enum PpSemilocalChannel {
    Vnl1(#[serde(rename = "PP_VNL1")] PpSemilocalValues),
    Vnl2(#[serde(rename = "PP_VNL2")] PpSemilocalValues),
    Vnl3(#[serde(rename = "PP_VNL3")] PpSemilocalValues),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpSemilocalValues {
    #[serde(rename = "@L")]
    pub l: usize,
    #[serde(rename = "@J")]
    pub j: Option<f64>,
    #[serde(rename = "$text")]
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PpChiNode {
    Chi1(#[serde(rename = "PP_CHI.1")] PpWavefunction),
    Chi2(#[serde(rename = "PP_CHI.2")] PpWavefunction),
    Chi3(#[serde(rename = "PP_CHI.3")] PpWavefunction),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpWavefunction {
    #[serde(rename = "@label")]
    pub label: String,
    #[serde(rename = "@l")]
    pub l: usize,
    #[serde(rename = "@occupation")]
    pub occupation: Option<f64>,
    #[serde(rename = "$text")]
    pub values: Vec<f64>,
}

pub type PpNlcc = NumericArray;
