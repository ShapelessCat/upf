use serde::{Deserialize, Serialize};

use super::internal::optional_numeric_section_vec;

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
    /// Expected size: `header.number_of_proj`.
    #[serde(
        rename = "PP_OCCUPATIONS",
        default,
        skip_serializing_if = "Option::is_none",
        with = "optional_numeric_section_vec"
    )]
    pub occupations: Option<Vec<f64>>,
    /// All-electron NLCC in tag `PP_AE_NLCC`.
    /// Expected size: `header.mesh_size`.
    #[serde(
        rename = "PP_AE_NLCC",
        default,
        skip_serializing_if = "Option::is_none",
        with = "optional_numeric_section_vec"
    )]
    pub ae_nlcc: Option<Vec<f64>>,
    /// All-electron local potential in tag `PP_AE_VLOC`.
    /// Expected size: `header.mesh_size`.
    #[serde(
        rename = "PP_AE_VLOC",
        default,
        skip_serializing_if = "Option::is_none",
        with = "optional_numeric_section_vec"
    )]
    pub ae_vloc: Option<Vec<f64>>,
}
