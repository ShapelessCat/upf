use serde::{Deserialize, Serialize};

use super::optional_numeric_section_vec;

/// Standalone metagga sections that are enabled by `PP_HEADER/@with_metagga_info`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpMetagga {
    /// Modified kinetic-energy density in tag `PP_TAUMOD`.
    /// Expected size: `header.mesh_size`.
    #[serde(
        rename = "PP_TAUMOD",
        default,
        skip_serializing_if = "Option::is_none",
        with = "optional_numeric_section_vec"
    )]
    pub taumod: Option<Vec<f64>>,
    /// Atomic kinetic-energy density in tag `PP_TAUATOM`.
    /// Expected size: `header.mesh_size`.
    #[serde(
        rename = "PP_TAUATOM",
        default,
        skip_serializing_if = "Option::is_none",
        with = "optional_numeric_section_vec"
    )]
    pub tauatom: Option<Vec<f64>>,
}
