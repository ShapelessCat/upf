use serde::{Deserialize, Serialize};

use super::NumericSection;

/// `PP_MESH` radial grid definition and its two required numeric arrays.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpMesh {
    /// Logarithmic mesh spacing parameter `@dx`.
    #[serde(rename = "@dx", default, skip_serializing_if = "Option::is_none")]
    pub dx: Option<f64>,
    /// Mesh point count declared in `PP_MESH/@mesh`.
    #[serde(rename = "@mesh", default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<usize>,
    /// Minimum mesh coordinate from `@xmin`.
    #[serde(rename = "@xmin", default, skip_serializing_if = "Option::is_none")]
    pub xmin: Option<f64>,
    /// Maximum radius from `@rmax`.
    #[serde(rename = "@rmax", default, skip_serializing_if = "Option::is_none")]
    pub rmax: Option<f64>,
    /// Nuclear charge mesh parameter from `@zmesh`.
    #[serde(rename = "@zmesh", default, skip_serializing_if = "Option::is_none")]
    pub zmesh: Option<f64>,
    /// Radial coordinate samples from `PP_R`.
    #[serde(rename = "PP_R")]
    pub r: NumericSection,
    /// Radial step samples from `PP_RAB`.
    #[serde(rename = "PP_RAB")]
    pub rab: NumericSection,
}
