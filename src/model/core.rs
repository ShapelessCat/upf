use serde::{Deserialize, Serialize};

use super::{
    gipaw::PpGipaw,
    nonlocal::{PpInfo, PpNlcc, PpNonlocal, PpPseudoWavefunctions, PpSemilocal},
    paw::{PpFullWfc, PpPaw},
};

/// Root UPF document corresponding to the top-level `<UPF ...> ... </UPF>` tag.
///
/// Each field maps directly onto one first-level section from the UPF 2.0.1
/// specification. Optional sections are represented as `Option<_>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "UPF")]
pub struct UpfData {
    /// Value of the top-level `UPF/@version` attribute.
    #[serde(rename = "@version")]
    pub version: String,
    /// Human-oriented `PP_INFO` content, including optional `PP_INPUTFILE`.
    #[serde(rename = "PP_INFO", skip_serializing_if = "Option::is_none")]
    pub info: Option<PpInfo>,
    /// Mandatory `PP_HEADER` section describing the dataset metadata.
    #[serde(rename = "PP_HEADER")]
    pub header: PpHeader,
    /// Mandatory `PP_MESH` section containing the radial mesh description.
    #[serde(rename = "PP_MESH")]
    pub mesh: PpMesh,
    /// Optional nonlinear core correction values from `PP_NLCC`.
    #[serde(rename = "PP_NLCC", skip_serializing_if = "Option::is_none")]
    pub nlcc: Option<PpNlcc>,
    /// Mandatory local potential samples from `PP_LOCAL`.
    #[serde(rename = "PP_LOCAL")]
    pub local: PpLocal,
    /// Optional semilocal channels from `PP_SEMILOCAL`.
    #[serde(rename = "PP_SEMILOCAL", skip_serializing_if = "Option::is_none")]
    pub semilocal: Option<PpSemilocal>,
    /// Mandatory nonlocal projector section from `PP_NONLOCAL`.
    #[serde(rename = "PP_NONLOCAL")]
    pub nonlocal: PpNonlocal,
    /// Optional pseudo-wavefunction section from `PP_PSWFC`.
    #[serde(rename = "PP_PSWFC", skip_serializing_if = "Option::is_none")]
    pub pswfc: Option<PpPseudoWavefunctions>,
    /// Optional all-electron and pseudo-wavefunction section from `PP_FULL_WFC`.
    #[serde(rename = "PP_FULL_WFC", skip_serializing_if = "Option::is_none")]
    pub full_wfc: Option<PpFullWfc>,
    /// Mandatory atomic charge density samples from `PP_RHOATOM`.
    #[serde(rename = "PP_RHOATOM")]
    pub rhoatom: PpRhoAtom,
    /// Optional PAW-only section from `PP_PAW`.
    #[serde(rename = "PP_PAW", skip_serializing_if = "Option::is_none")]
    pub paw: Option<PpPaw>,
    /// Optional GIPAW-only section from `PP_GIPAW`.
    #[serde(rename = "PP_GIPAW", skip_serializing_if = "Option::is_none")]
    pub gipaw: Option<PpGipaw>,
}

impl UpfData {
    /// Check structural invariants that are not enforced by XML shape alone.
    ///
    /// The current validation rules ensure that mesh-sized numeric sections
    /// match `PP_HEADER/@mesh_size`, and that header flags requiring PAW or
    /// GIPAW data are consistent with the presence of `PP_PAW` and `PP_GIPAW`.
    pub fn validate(&self) -> Result<(), crate::UpfError> {
        let mesh = self.header.mesh_size;
        if self.mesh.r.values.len() != mesh {
            return Err(crate::UpfError::Validation(format!(
                "mesh_size {} does not match PP_R length {}",
                mesh,
                self.mesh.r.values.len()
            )));
        }
        if self.mesh.rab.values.len() != mesh {
            return Err(crate::UpfError::Validation(format!(
                "mesh_size {} does not match PP_RAB length {}",
                mesh,
                self.mesh.rab.values.len()
            )));
        }
        if self.local.values.len() != mesh {
            return Err(crate::UpfError::Validation(format!(
                "mesh_size {} does not match PP_LOCAL length {}",
                mesh,
                self.local.values.len()
            )));
        }
        if self.rhoatom.values.len() != mesh {
            return Err(crate::UpfError::Validation(format!(
                "mesh_size {} does not match PP_RHOATOM length {}",
                mesh,
                self.rhoatom.values.len()
            )));
        }
        if self.header.is_paw == "T" && self.paw.is_none() {
            return Err(crate::UpfError::Validation(
                "PP_HEADER marks the dataset as PAW but PP_PAW is missing".into(),
            ));
        }
        if self.header.has_gipaw == "T" && self.gipaw.is_none() {
            return Err(crate::UpfError::Validation(
                "PP_HEADER marks the dataset as GIPAW-enabled but PP_GIPAW is missing".into(),
            ));
        }
        Ok(())
    }
}

/// `PP_HEADER` metadata describing the pseudopotential dataset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpHeader {
    /// Generation code recorded in `@generated`.
    #[serde(rename = "@generated")]
    pub generated: String,
    /// Author recorded in `@author`.
    #[serde(rename = "@author")]
    pub author: String,
    /// Generation date recorded in `@date`.
    #[serde(rename = "@date")]
    pub date: String,
    /// Free-form summary from `@comment`.
    #[serde(rename = "@comment")]
    pub comment: String,
    /// Chemical symbol from `@element`.
    #[serde(rename = "@element")]
    pub element: String,
    /// Pseudopotential kind from `@pseudo_type`, such as `NC` or `PAW`.
    #[serde(rename = "@pseudo_type")]
    pub pseudo_type: String,
    /// Relativistic mode from `@relativistic`.
    #[serde(rename = "@relativistic")]
    pub relativistic: String,
    /// Flag from `@is_ultrasoft`.
    #[serde(rename = "@is_ultrasoft")]
    pub is_ultrasoft: String,
    /// Flag from `@is_paw`.
    #[serde(rename = "@is_paw")]
    pub is_paw: String,
    /// Flag from `@is_coulomb`.
    #[serde(rename = "@is_coulomb")]
    pub is_coulomb: String,
    /// Flag from `@has_so`.
    #[serde(rename = "@has_so")]
    pub has_so: String,
    /// Flag from `@has_wfc`.
    #[serde(rename = "@has_wfc")]
    pub has_wfc: String,
    /// Flag from `@has_gipaw`.
    #[serde(rename = "@has_gipaw")]
    pub has_gipaw: String,
    /// Flag from `@core_correction`.
    #[serde(rename = "@core_correction")]
    pub core_correction: String,
    /// Valence charge from `@z_valence`.
    #[serde(rename = "@z_valence")]
    pub z_valence: f64,
    /// Total pseudopotential energy from `@total_psenergy`.
    #[serde(rename = "@total_psenergy")]
    pub total_psenergy: f64,
    /// Suggested wavefunction cutoff from `@wfc_cutoff`.
    #[serde(rename = "@wfc_cutoff")]
    pub wfc_cutoff: f64,
    /// Suggested charge-density cutoff from `@rho_cutoff`.
    #[serde(rename = "@rho_cutoff")]
    pub rho_cutoff: f64,
    /// Maximum angular momentum from `@l_max`.
    #[serde(rename = "@l_max")]
    pub l_max: usize,
    /// Maximum rho angular momentum from `@l_max_rho`.
    #[serde(rename = "@l_max_rho")]
    pub l_max_rho: usize,
    /// Local channel angular momentum from `@l_local`.
    #[serde(rename = "@l_local")]
    pub l_local: isize,
    /// Declared radial grid length used by several other sections.
    #[serde(rename = "@mesh_size")]
    pub mesh_size: usize,
    /// Number of pseudo-wavefunctions from `@number_of_wfc`.
    #[serde(rename = "@number_of_wfc")]
    pub number_of_wfc: usize,
    /// Number of projectors from `@number_of_proj`.
    #[serde(rename = "@number_of_proj")]
    pub number_of_proj: usize,
}

/// `PP_MESH` radial grid definition and its two required numeric arrays.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpMesh {
    /// Logarithmic mesh spacing parameter `@dx`.
    #[serde(rename = "@dx")]
    pub dx: f64,
    /// Mesh point count declared in `PP_MESH/@mesh`.
    #[serde(rename = "@mesh")]
    pub mesh: usize,
    /// Minimum mesh coordinate from `@xmin`.
    #[serde(rename = "@xmin")]
    pub xmin: f64,
    /// Maximum radius from `@rmax`.
    #[serde(rename = "@rmax")]
    pub rmax: f64,
    /// Nuclear charge mesh parameter from `@zmesh`.
    #[serde(rename = "@zmesh")]
    pub zmesh: f64,
    /// Radial coordinate samples from `PP_R`.
    #[serde(rename = "PP_R")]
    pub r: PpR,
    /// Radial step samples from `PP_RAB`.
    #[serde(rename = "PP_RAB")]
    pub rab: PpRab,
}

/// A whitespace-delimited numeric UPF field represented as `Vec<f64>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumericArray {
    /// Numeric values parsed from the field body text.
    #[serde(rename = "$text")]
    pub values: Vec<f64>,
}

/// Alias for the `PP_R` radial mesh array.
pub type PpR = NumericArray;
/// Alias for the `PP_RAB` radial step array.
pub type PpRab = NumericArray;
/// Alias for the `PP_LOCAL` local potential array.
pub type PpLocal = NumericArray;
/// Alias for the `PP_RHOATOM` atomic charge density array.
pub type PpRhoAtom = NumericArray;
