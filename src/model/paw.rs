use serde::{Deserialize, Serialize};

use super::{NumericArray, PpWavefunction};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpFullWfc {
    #[serde(rename = "$value", default)]
    pub entries: Vec<PpFullWfcEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpPaw {
    #[serde(rename = "PP_PAW_FORMAT_VERSION")]
    pub format_version: String,
    #[serde(rename = "PP_AUGMENTATION")]
    pub augmentation: PpAugmentation,
    #[serde(rename = "PP_AE_RHO_ATC", skip_serializing_if = "Option::is_none")]
    pub ae_rho_atc: Option<NumericArray>,
    #[serde(rename = "$value", default)]
    pub entries: Vec<PpPawEntry>,
    #[serde(rename = "PP_AEVLOC", skip_serializing_if = "Option::is_none")]
    pub aevloc: Option<NumericArray>,
    #[serde(rename = "PP_KDIFF", skip_serializing_if = "Option::is_none")]
    pub kdiff: Option<NumericArray>,
    #[serde(rename = "PP_OCCUP", skip_serializing_if = "Option::is_none")]
    pub occupations: Option<NumericArray>,
    #[serde(rename = "PP_GRID_RECON", skip_serializing_if = "Option::is_none")]
    pub grid_recon: Option<NumericArray>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpAugmentation {
    #[serde(rename = "@q_with_l")]
    pub q_with_l: String,
    #[serde(rename = "@augmentation_epsilon")]
    pub augmentation_epsilon: f64,
    #[serde(rename = "@cutoff_r")]
    pub cutoff_r: f64,
    #[serde(rename = "@l_max_aug")]
    pub l_max_aug: usize,
    #[serde(rename = "$value", default)]
    pub channels: Vec<PpNumericNode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PpFullWfcEntry {
    #[serde(rename = "PP_AEWFC.1")]
    Ae1(PpWavefunction),
    #[serde(rename = "PP_PSWFC.1")]
    Ps1(PpWavefunction),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PpPawEntry {
    #[serde(rename = "PP_AEWFC.1")]
    Ae1(PpWavefunction),
    #[serde(rename = "PP_PSWFC_FULL.1")]
    PsFull1(PpWavefunction),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpNumericNode {
    #[serde(rename = "$text")]
    pub values: Vec<f64>,
}
