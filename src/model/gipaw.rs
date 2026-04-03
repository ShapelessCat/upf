use serde::{Deserialize, Serialize};

use super::NumericArray;

/// `PP_GIPAW` section for datasets that include GIPAW reconstruction data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpGipaw {
    /// GIPAW format version string from `PP_GIPAW_FORMAT_VERSION`.
    #[serde(rename = "PP_GIPAW_FORMAT_VERSION")]
    pub format_version: String,
    /// Optional `GIPAW_CORE_ORBITALS` block.
    #[serde(
        rename = "GIPAW_CORE_ORBITALS",
        skip_serializing_if = "Option::is_none"
    )]
    pub core_orbitals: Option<GipawCoreOrbitals>,
    /// Optional `GIPAW_LOCAL_DATA` block.
    #[serde(rename = "GIPAW_LOCAL_DATA", skip_serializing_if = "Option::is_none")]
    pub local_data: Option<GipawLocalData>,
    /// Optional `GIPAW_ORBITALS` block.
    #[serde(rename = "GIPAW_ORBITALS", skip_serializing_if = "Option::is_none")]
    pub orbitals: Option<GipawOrbitals>,
}

/// `GIPAW_CORE_ORBITALS` container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GipawCoreOrbitals {
    /// Core-orbital entries.
    #[serde(rename = "$value", default)]
    pub orbitals: Vec<GipawOrbital>,
}

/// `GIPAW_LOCAL_DATA` section with local all-electron and pseudized potentials.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GipawLocalData {
    /// `GIPAW_VLOCAL_AE` samples.
    #[serde(rename = "GIPAW_VLOCAL_AE")]
    pub vlocal_ae: NumericArray,
    /// `GIPAW_VLOCAL_PS` samples.
    #[serde(rename = "GIPAW_VLOCAL_PS")]
    pub vlocal_ps: NumericArray,
}

/// `GIPAW_ORBITALS` container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GipawOrbitals {
    /// Orbital entries.
    #[serde(rename = "$value", default)]
    pub orbitals: Vec<GipawOrbital>,
}

/// One orbital-like numeric entry inside a GIPAW block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GipawOrbital {
    /// Optional label from `@label`.
    #[serde(rename = "@label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Optional angular momentum from `@l`.
    #[serde(rename = "@l", skip_serializing_if = "Option::is_none")]
    pub l: Option<usize>,
    /// Orbital samples stored as body text.
    #[serde(rename = "$text")]
    pub values: Vec<f64>,
}
