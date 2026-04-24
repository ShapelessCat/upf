use serde::{Deserialize, Serialize};

use super::NumericSection;

/// `PP_PAW` section used by PAW datasets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpPaw {
    /// PAW format version string from `@paw_data_format`.
    #[serde(rename = "@paw_data_format")]
    pub data_format: String,
    /// Optional core energy from `@core_energy`.
    #[serde(
        rename = "@core_energy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub core_energy: Option<f64>,
    /// Occupations from `PP_OCCUPATIONS`.
    #[serde(
        rename = "PP_OCCUPATIONS",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub occupations: Option<NumericSection>,
    /// Optional all-electron NLCC from `PP_AE_NLCC`.
    #[serde(
        rename = "PP_AE_NLCC",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ae_nlcc: Option<NumericSection>,
    /// Optional all-electron local potential from `PP_AE_VLOC`.
    #[serde(
        rename = "PP_AE_VLOC",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ae_vloc: Option<NumericSection>,
}
