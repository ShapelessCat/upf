use serde::{Deserialize, Serialize};

use super::NumericSection;

/// `PP_PAW` section used by PAW datasets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpPaw {
    /// PAW format version string in attribute `paw_data_format`.
    #[serde(rename = "@paw_data_format")]
    pub data_format: String,
    /// Core energy in attribute `core_energy`.
    #[serde(
        rename = "@core_energy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub core_energy: Option<f64>,
    /// Occupations in tag `PP_OCCUPATIONS`.
    #[serde(
        rename = "PP_OCCUPATIONS",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub occupations: Option<NumericSection>,
    /// All-electron NLCC in tag `PP_AE_NLCC`.
    #[serde(
        rename = "PP_AE_NLCC",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ae_nlcc: Option<NumericSection>,
    /// All-electron local potential in tag `PP_AE_VLOC`.
    #[serde(
        rename = "PP_AE_VLOC",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ae_vloc: Option<NumericSection>,
}
