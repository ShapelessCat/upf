use serde::{Deserialize, Serialize};

use super::NumericArray;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpGipaw {
    #[serde(rename = "PP_GIPAW_FORMAT_VERSION")]
    pub format_version: String,
    #[serde(
        rename = "GIPAW_CORE_ORBITALS",
        skip_serializing_if = "Option::is_none"
    )]
    pub core_orbitals: Option<GipawCoreOrbitals>,
    #[serde(rename = "GIPAW_LOCAL_DATA", skip_serializing_if = "Option::is_none")]
    pub local_data: Option<GipawLocalData>,
    #[serde(rename = "GIPAW_ORBITALS", skip_serializing_if = "Option::is_none")]
    pub orbitals: Option<GipawOrbitals>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GipawCoreOrbitals {
    #[serde(rename = "$value", default)]
    pub orbitals: Vec<GipawOrbital>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GipawLocalData {
    #[serde(rename = "GIPAW_VLOCAL_AE")]
    pub vlocal_ae: NumericArray,
    #[serde(rename = "GIPAW_VLOCAL_PS")]
    pub vlocal_ps: NumericArray,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GipawOrbitals {
    #[serde(rename = "$value", default)]
    pub orbitals: Vec<GipawOrbital>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GipawOrbital {
    #[serde(rename = "@label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "@l", skip_serializing_if = "Option::is_none")]
    pub l: Option<usize>,
    #[serde(rename = "$text")]
    pub values: Vec<f64>,
}
