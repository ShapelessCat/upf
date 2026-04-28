use serde::{Deserialize, Serialize};

use super::{
    PpFullWfc, PpGipaw, PpHeader, PpInfo, PpMesh, PpNonlocal, PpPaw,
    PpPseudoWavefunctions, PpSemilocal, PpSpinOrb,
};
use super::internal::{numeric_section_vec, optional_numeric_section_vec};

/// Root UPF document corresponding to the top-level `<UPF ...> ... </UPF>` tag.
///
/// Each field maps directly onto one first-level UPF 2.0.1 section.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "UPF")]
pub struct UpfData {
    /// Value of attribute `version` on the top-level `UPF` tag.
    #[serde(rename = "@version")]
    pub version: String,
    /// Human-oriented content in tag `PP_INFO`, including tag `PP_INPUTFILE`.
    #[serde(rename = "PP_INFO", skip_serializing_if = "Option::is_none")]
    pub info: Option<PpInfo>,
    /// Section in tag `PP_HEADER` describing the dataset metadata.
    #[serde(rename = "PP_HEADER")]
    pub header: PpHeader,
    /// Section in tag `PP_MESH` containing the radial mesh description.
    #[serde(rename = "PP_MESH")]
    pub mesh: PpMesh,
    /// Nonlinear core correction values in tag `PP_NLCC`.
    /// Expected size: `header.mesh_size`.
    #[serde(
        rename = "PP_NLCC",
        default,
        skip_serializing_if = "Option::is_none",
        with = "optional_numeric_section_vec"
    )]
    pub nlcc: Option<Vec<f64>>,
    /// Local potential samples in tag `PP_LOCAL`, omitted for Coulomb datasets.
    /// Expected size: `header.mesh_size` whenever present.
    #[serde(
        rename = "PP_LOCAL",
        default,
        skip_serializing_if = "Vec::is_empty",
        with = "numeric_section_vec"
    )]
    pub local: Vec<f64>,
    /// Semilocal channels in tag `PP_SEMILOCAL`.
    #[serde(rename = "PP_SEMILOCAL", skip_serializing_if = "Option::is_none")]
    pub semilocal: Option<PpSemilocal>,
    /// Nonlocal projector section in tag `PP_NONLOCAL`, omitted when no projectors are stored.
    #[serde(
        rename = "PP_NONLOCAL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub nonlocal: Option<PpNonlocal>,
    /// Pseudo-wavefunction section in tag `PP_PSWFC`.
    #[serde(rename = "PP_PSWFC", skip_serializing_if = "Option::is_none")]
    pub pswfc: Option<PpPseudoWavefunctions>,
    /// All-electron and pseudo-wavefunction section in tag `PP_FULL_WFC`.
    #[serde(rename = "PP_FULL_WFC", skip_serializing_if = "Option::is_none")]
    pub full_wfc: Option<PpFullWfc>,
    /// Atomic charge density samples in tag `PP_RHOATOM`.
    /// Expected size: `header.mesh_size`.
    #[serde(rename = "PP_RHOATOM", with = "numeric_section_vec")]
    pub rhoatom: Vec<f64>,
    /// Metagga kinetic-energy density in tag `PP_TAUMOD`.
    /// Expected size: `header.mesh_size`.
    #[serde(
        rename = "PP_TAUMOD",
        default,
        skip_serializing_if = "Option::is_none",
        with = "optional_numeric_section_vec"
    )]
    pub taumod: Option<Vec<f64>>,
    /// Metagga atomic kinetic-energy density in tag `PP_TAUATOM`.
    /// Expected size: `header.mesh_size`.
    #[serde(
        rename = "PP_TAUATOM",
        default,
        skip_serializing_if = "Option::is_none",
        with = "optional_numeric_section_vec"
    )]
    pub tauatom: Option<Vec<f64>>,
    /// Spin-orbit metadata in tag `PP_SPIN_ORB`.
    #[serde(
        rename = "PP_SPIN_ORB",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub spin_orb: Option<PpSpinOrb>,
    /// PAW-only section in tag `PP_PAW`.
    #[serde(rename = "PP_PAW", skip_serializing_if = "Option::is_none")]
    pub paw: Option<PpPaw>,
    /// GIPAW-only section in tag `PP_GIPAW`.
    #[serde(rename = "PP_GIPAW", skip_serializing_if = "Option::is_none")]
    pub gipaw: Option<PpGipaw>,
}
