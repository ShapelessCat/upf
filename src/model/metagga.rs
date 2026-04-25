use serde::{Deserialize, Serialize};

use super::NumericSection;

/// Standalone metagga sections that are enabled by `PP_HEADER/@with_metagga_info`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpMetagga {
    /// Modified kinetic-energy density in tag `PP_TAUMOD`.
    #[serde(rename = "PP_TAUMOD", default, skip_serializing_if = "Option::is_none")]
    pub taumod: Option<NumericSection>,
    /// Atomic kinetic-energy density in tag `PP_TAUATOM`.
    #[serde(
        rename = "PP_TAUATOM",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tauatom: Option<NumericSection>,
}
