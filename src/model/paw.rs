use serde::{Deserialize, Serialize};

use super::internal::numeric_section_vec;
use super::internal::deserialize_f64;

/// `PP_PAW` section used by PAW datasets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpPaw {
    /// PAW format version string in attribute `paw_data_format`.
    #[serde(rename = "@paw_data_format")]
    pub data_format: String,
    /// Constant offset added to the valence energy to recover the all-electron
    /// energy; it does not affect the calculation result itself.
    #[serde(rename = "@core_energy", default, deserialize_with = "deserialize_f64")]
    pub core_energy: f64,
    /// Occupations in tag `PP_OCCUPATIONS`.
    /// Expected size: `header.number_of_proj`.
    #[serde(rename = "PP_OCCUPATIONS", with = "numeric_section_vec")]
    pub occupations: Vec<f64>,
    /// All-electron NLCC in tag `PP_AE_NLCC`.
    /// Expected size: `header.mesh_size`.
    #[serde(rename = "PP_AE_NLCC", with = "numeric_section_vec")]
    pub ae_nlcc: Vec<f64>,
    /// All-electron local potential in tag `PP_AE_VLOC`.
    /// Expected size: `header.mesh_size`.
    #[serde(rename = "PP_AE_VLOC", with = "numeric_section_vec")]
    pub ae_vloc: Vec<f64>,
}
