use serde::{Deserialize, Serialize};

use super::{NumericArray, PpWavefunction};

/// `PP_FULL_WFC` section with numbered all-electron and pseudo-wavefunction data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpFullWfc {
    /// Numbered `PP_AEWFC.n` and `PP_PSWFC.n` entries.
    #[serde(rename = "$value", default)]
    pub entries: Vec<PpFullWfcEntry>,
}

/// `PP_PAW` section used by PAW datasets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpPaw {
    /// PAW format version string from `PP_PAW_FORMAT_VERSION`.
    #[serde(rename = "PP_PAW_FORMAT_VERSION")]
    pub format_version: String,
    /// Mandatory augmentation block from `PP_AUGMENTATION`.
    #[serde(rename = "PP_AUGMENTATION")]
    pub augmentation: PpAugmentation,
    /// Optional all-electron atomic charge from `PP_AE_RHO_ATC`.
    #[serde(rename = "PP_AE_RHO_ATC", skip_serializing_if = "Option::is_none")]
    pub ae_rho_atc: Option<NumericArray>,
    /// Remaining numbered PAW wavefunction entries.
    #[serde(rename = "$value", default)]
    pub entries: Vec<PpPawEntry>,
    /// Optional local potential from `PP_AEVLOC`.
    #[serde(rename = "PP_AEVLOC", skip_serializing_if = "Option::is_none")]
    pub aevloc: Option<NumericArray>,
    /// Optional kinetic-energy difference data from `PP_KDIFF`.
    #[serde(rename = "PP_KDIFF", skip_serializing_if = "Option::is_none")]
    pub kdiff: Option<NumericArray>,
    /// Optional occupations from `PP_OCCUP`.
    #[serde(rename = "PP_OCCUP", skip_serializing_if = "Option::is_none")]
    pub occupations: Option<NumericArray>,
    /// Optional reconstruction grid from `PP_GRID_RECON`.
    #[serde(rename = "PP_GRID_RECON", skip_serializing_if = "Option::is_none")]
    pub grid_recon: Option<NumericArray>,
}

/// `PP_AUGMENTATION` block nested inside `PP_PAW`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpAugmentation {
    /// `@q_with_l` flag.
    #[serde(rename = "@q_with_l")]
    pub q_with_l: String,
    /// Augmentation tolerance from `@augmentation_epsilon`.
    #[serde(rename = "@augmentation_epsilon")]
    pub augmentation_epsilon: f64,
    /// Augmentation cutoff radius from `@cutoff_r`.
    #[serde(rename = "@cutoff_r")]
    pub cutoff_r: f64,
    /// Maximum augmentation angular momentum from `@l_max_aug`.
    #[serde(rename = "@l_max_aug")]
    pub l_max_aug: usize,
    /// Numeric child nodes contained in the augmentation block.
    #[serde(rename = "$value", default)]
    pub channels: Vec<PpNumericNode>,
}

/// Supported numbered children inside `PP_FULL_WFC`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PpFullWfcEntry {
    #[serde(rename = "PP_AEWFC.1")]
    Ae1(PpWavefunction),
    #[serde(rename = "PP_PSWFC.1")]
    Ps1(PpWavefunction),
}

/// Supported numbered children inside `PP_PAW`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PpPawEntry {
    #[serde(rename = "PP_AEWFC.1")]
    Ae1(PpWavefunction),
    #[serde(rename = "PP_PSWFC_FULL.1")]
    PsFull1(PpWavefunction),
}

/// Generic numeric child node used in PAW augmentation content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpNumericNode {
    /// Numeric samples stored as whitespace-delimited body text.
    #[serde(rename = "$text")]
    pub values: Vec<f64>,
}
