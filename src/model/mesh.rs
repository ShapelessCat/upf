use serde::{Deserialize, Serialize};

use super::internal::numeric_section_vec;
use super::internal::deserialize_opt_f64;

/// `PP_MESH` radial grid definition and its two required numeric arrays.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpMesh {
    /// Logarithmic mesh spacing parameter in attribute `dx`.
    #[serde(
        rename = "@dx",
        default,
        deserialize_with = "deserialize_opt_f64",
        skip_serializing_if = "Option::is_none"
    )]
    pub dx: Option<f64>,
    /// Mesh point count in attribute `mesh`.
    #[serde(rename = "@mesh", default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<usize>,
    /// Minimum mesh coordinate in attribute `xmin`.
    #[serde(
        rename = "@xmin",
        default,
        deserialize_with = "deserialize_opt_f64",
        skip_serializing_if = "Option::is_none"
    )]
    pub xmin: Option<f64>,
    /// Maximum radius in attribute `rmax`.
    #[serde(
        rename = "@rmax",
        default,
        deserialize_with = "deserialize_opt_f64",
        skip_serializing_if = "Option::is_none"
    )]
    pub rmax: Option<f64>,
    /// Nuclear charge mesh parameter in attribute `zmesh`.
    #[serde(
        rename = "@zmesh",
        default,
        deserialize_with = "deserialize_opt_f64",
        skip_serializing_if = "Option::is_none"
    )]
    pub zmesh: Option<f64>,
    /// Radial coordinate samples in tag `PP_R`.
    /// Expected size: `header.mesh_size`.
    #[serde(rename = "PP_R", with = "numeric_section_vec")]
    pub r: Vec<f64>,
    /// Radial step samples in tag `PP_RAB`.
    /// Expected size: `header.mesh_size`.
    #[serde(rename = "PP_RAB", with = "numeric_section_vec")]
    pub rab: Vec<f64>,
}
