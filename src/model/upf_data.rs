use serde::{Deserialize, Serialize};

use super::{
    NumericSection, PpFullWfc, PpGipaw, PpHeader, PpInfo, PpMesh, PpNonlocal, PpPaw,
    PpPseudoWavefunctions, PpSemilocal,
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
    #[serde(rename = "PP_NLCC", default, skip_serializing_if = "Option::is_none")]
    pub nlcc: Option<NumericSection>,
    /// Mandatory local potential samples from `PP_LOCAL`.
    #[serde(rename = "PP_LOCAL")]
    pub local: NumericSection,
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
    pub rhoatom: NumericSection,
    /// Optional PAW-only section from `PP_PAW`.
    #[serde(rename = "PP_PAW", skip_serializing_if = "Option::is_none")]
    pub paw: Option<PpPaw>,
    /// Optional GIPAW-only section from `PP_GIPAW`.
    #[serde(rename = "PP_GIPAW", skip_serializing_if = "Option::is_none")]
    pub gipaw: Option<PpGipaw>,
}

impl UpfData {
    fn validate_section_size(name: &str, section: &NumericSection) -> Result<(), crate::UpfError> {
        if let Some(size) = section.size
            && size != section.len()
        {
            return Err(crate::UpfError::Validation(format!(
                "{name} declares size {size} but contains {} values",
                section.len()
            )));
        }

        Ok(())
    }

    /// Check structural invariants that are not enforced by XML shape alone.
    ///
    /// The current validation rules ensure that mesh-sized numeric sections
    /// match `PP_HEADER/@mesh_size`, and that header flags requiring PAW or
    /// GIPAW data are consistent with the presence of `PP_PAW` and `PP_GIPAW`.
    pub fn validate(&self) -> Result<(), crate::UpfError> {
        let mesh = self.header.mesh_size;
        Self::validate_section_size("PP_R", &self.mesh.r)?;
        Self::validate_section_size("PP_RAB", &self.mesh.rab)?;
        Self::validate_section_size("PP_LOCAL", &self.local)?;
        Self::validate_section_size("PP_RHOATOM", &self.rhoatom)?;

        if let Some(nlcc) = &self.nlcc {
            Self::validate_section_size("PP_NLCC", nlcc)?;
        }

        if let Some(declared_mesh) = self.mesh.mesh
            && declared_mesh != mesh
        {
            return Err(crate::UpfError::Validation(format!(
                "PP_MESH declares mesh {} but PP_HEADER mesh_size is {}",
                declared_mesh, mesh
            )));
        }

        if self.mesh.r.len() != mesh {
            return Err(crate::UpfError::Validation(format!(
                "mesh_size {} does not match PP_R length {}",
                mesh,
                self.mesh.r.len()
            )));
        }
        if self.mesh.rab.len() != mesh {
            return Err(crate::UpfError::Validation(format!(
                "mesh_size {} does not match PP_RAB length {}",
                mesh,
                self.mesh.rab.len()
            )));
        }
        if self.local.len() != mesh {
            return Err(crate::UpfError::Validation(format!(
                "mesh_size {} does not match PP_LOCAL length {}",
                mesh,
                self.local.len()
            )));
        }
        if self.rhoatom.len() != mesh {
            return Err(crate::UpfError::Validation(format!(
                "mesh_size {} does not match PP_RHOATOM length {}",
                mesh,
                self.rhoatom.len()
            )));
        }
        if self.header.core_correction && self.nlcc.is_none() {
            return Err(crate::UpfError::Validation(
                "PP_HEADER enables core correction but PP_NLCC is missing".into(),
            ));
        }
        if self.header.number_of_proj != self.nonlocal.betas.len() {
            return Err(crate::UpfError::Validation(format!(
                "PP_HEADER declares {} projectors but PP_NONLOCAL contains {} PP_BETA entries",
                self.header.number_of_proj,
                self.nonlocal.betas.len()
            )));
        }
        if self.header.number_of_wfc != 0 {
            match &self.pswfc {
                Some(pswfc) if pswfc.orbitals.len() == self.header.number_of_wfc => {}
                Some(pswfc) => {
                    return Err(crate::UpfError::Validation(format!(
                        "PP_HEADER declares {} wavefunctions but PP_PSWFC contains {} PP_CHI entries",
                        self.header.number_of_wfc,
                        pswfc.orbitals.len()
                    )));
                }
                None => {
                    return Err(crate::UpfError::Validation(
                        "PP_HEADER declares wavefunctions but PP_PSWFC is missing".into(),
                    ));
                }
            }
        }
        if self.header.is_paw && self.paw.is_none() {
            return Err(crate::UpfError::Validation(
                "PP_HEADER marks the dataset as PAW but PP_PAW is missing".into(),
            ));
        }
        if let Some(paw) = &self.paw {
            if paw.occupations.is_none() {
                return Err(crate::UpfError::Validation(
                    "PP_PAW is present but PP_OCCUPATIONS is missing".into(),
                ));
            }
            if let Some(section) = &paw.occupations {
                Self::validate_section_size("PP_PAW/PP_OCCUPATIONS", section)?;
            }
            if let Some(section) = &paw.ae_nlcc {
                Self::validate_section_size("PP_PAW/PP_AE_NLCC", section)?;
            }
            if let Some(section) = &paw.ae_vloc {
                Self::validate_section_size("PP_PAW/PP_AE_VLOC", section)?;
            }
        }
        if self.header.has_gipaw && self.gipaw.is_none() {
            return Err(crate::UpfError::Validation(
                "PP_HEADER marks the dataset as GIPAW-enabled but PP_GIPAW is missing".into(),
            ));
        }
        if (self.header.is_ultrasoft || self.header.is_paw) && self.nonlocal.augmentation.is_none()
        {
            return Err(crate::UpfError::Validation(
                "ultrasoft or PAW datasets require PP_AUGMENTATION inside PP_NONLOCAL".into(),
            ));
        }
        if let Some(augmentation) = &self.nonlocal.augmentation {
            if let Some(q) = &augmentation.q {
                Self::validate_section_size("PP_AUGMENTATION/PP_Q", q)?;
            }
            if let Some(multipoles) = &augmentation.multipoles {
                Self::validate_section_size("PP_AUGMENTATION/PP_MULTIPOLES", multipoles)?;
            }
            for channel in &augmentation.channels {
                if let Some(size) = channel.value.size
                    && size != channel.value.values.len()
                {
                    return Err(crate::UpfError::Validation(format!(
                        "{} declares size {} but contains {} values",
                        channel.tag,
                        size,
                        channel.value.values.len()
                    )));
                }
            }
        }
        if let Some(gipaw) = &self.gipaw
            && let Some(core_orbitals) = &gipaw.core_orbitals
        {
            if core_orbitals.number_of_core_orbitals != core_orbitals.orbitals.len() {
                return Err(crate::UpfError::Validation(format!(
                    "PP_GIPAW_CORE_ORBITALS declares {} entries but contains {} orbitals",
                    core_orbitals.number_of_core_orbitals,
                    core_orbitals.orbitals.len()
                )));
            }
            for orbital in &core_orbitals.orbitals {
                if let Some(size) = orbital.value.size
                    && size != orbital.value.values.len()
                {
                    return Err(crate::UpfError::Validation(format!(
                        "{} declares size {} but contains {} values",
                        orbital.tag,
                        size,
                        orbital.value.values.len()
                    )));
                }
            }
        }
        Ok(())
    }
}
