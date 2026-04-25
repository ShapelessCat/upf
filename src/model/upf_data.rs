use serde::{Deserialize, Serialize};

use super::{
    PpAugmentation, PpFullWfc, PpGipaw, PpHeader, PpInfo, PpMesh, PpNonlocal, PpPaw,
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
        skip_serializing_if = "PpNonlocal::is_empty"
    )]
    pub nonlocal: PpNonlocal,
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

impl UpfData {
    fn validate_len(name: &str, actual: usize, expected: usize) -> Result<(), crate::UpfError> {
        if actual != expected {
            return Err(crate::UpfError::Validation(format!(
                "{name} length {actual} does not match expected size {expected}"
            )));
        }

        Ok(())
    }

    fn validate_optional_len(
        name: &str,
        section: Option<&Vec<f64>>,
        expected: usize,
    ) -> Result<(), crate::UpfError> {
        if let Some(section) = section {
            Self::validate_len(name, section.len(), expected)?;
        }

        Ok(())
    }

    fn effective_nqlc(augmentation: &PpAugmentation, header_l_max: usize) -> usize {
        if augmentation.nqlc == 0 {
            2 * header_l_max + 1
        } else {
            augmentation.nqlc
        }
    }

    /// Check structural invariants that are not enforced by XML shape alone.
    ///
    /// The current validation rules ensure that mesh-sized numeric sections
    /// match `PP_HEADER/@mesh_size`, and that header flags requiring PAW or
    /// GIPAW data are consistent with the presence of `PP_PAW` and `PP_GIPAW`.
    pub fn validate(&self) -> Result<(), crate::UpfError> {
        let mesh = self.header.mesh_size;
        if let Some(declared_mesh) = self.mesh.mesh
            && declared_mesh != mesh
        {
            return Err(crate::UpfError::Validation(format!(
                "PP_MESH declares mesh {} but PP_HEADER mesh_size is {}",
                declared_mesh, mesh
            )));
        }

        Self::validate_len("PP_R", self.mesh.r.len(), mesh)?;
        Self::validate_len("PP_RAB", self.mesh.rab.len(), mesh)?;
        if !self.header.is_coulomb && self.local.is_empty() {
            return Err(crate::UpfError::Validation(
                "PP_HEADER marks the dataset as non-Coulomb but PP_LOCAL is missing".into(),
            ));
        }
        if !self.local.is_empty() {
            Self::validate_len("PP_LOCAL", self.local.len(), mesh)?;
        }
        Self::validate_len("PP_RHOATOM", self.rhoatom.len(), mesh)?;
        Self::validate_optional_len("PP_NLCC", self.nlcc.as_ref(), mesh)?;
        Self::validate_optional_len("PP_TAUMOD", self.taumod.as_ref(), mesh)?;
        Self::validate_optional_len("PP_TAUATOM", self.tauatom.as_ref(), mesh)?;
        if self.header.core_correction && self.nlcc.is_none() {
            return Err(crate::UpfError::Validation(
                "PP_HEADER enables core correction but PP_NLCC is missing".into(),
            ));
        }
        if self.header.with_metagga_info && (self.taumod.is_none() || self.tauatom.is_none()) {
            return Err(crate::UpfError::Validation(
                "PP_HEADER enables metagga info but PP_TAUMOD or PP_TAUATOM is missing".into(),
            ));
        }
        if self.header.number_of_proj != self.nonlocal.betas.len() {
            return Err(crate::UpfError::Validation(format!(
                "PP_HEADER declares {} projectors but PP_NONLOCAL contains {} PP_BETA entries",
                self.header.number_of_proj,
                self.nonlocal.betas.len()
            )));
        }
        for beta in &self.nonlocal.betas {
            Self::validate_len(&beta.tag.to_string(), beta.value.values.len(), mesh)?;
        }
        if !self.nonlocal.dij.is_empty() {
            Self::validate_len(
                "PP_DIJ",
                self.nonlocal.dij.len(),
                self.header.number_of_proj * self.header.number_of_proj,
            )?;
        }
        if let Some(semilocal) = &self.semilocal {
            for channel in &semilocal.channels {
                Self::validate_len(&channel.tag.to_string(), channel.value.values.len(), mesh)?;
            }
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
        if let Some(pswfc) = &self.pswfc {
            for orbital in &pswfc.orbitals {
                Self::validate_len(&orbital.tag.to_string(), orbital.value.values.len(), mesh)?;
            }
        }
        if self.header.has_wfc && self.full_wfc.is_none() {
            return Err(crate::UpfError::Validation(
                "PP_HEADER marks the dataset as having full wavefunctions but PP_FULL_WFC is missing"
                    .into(),
            ));
        }
        if let Some(full_wfc) = &self.full_wfc {
            if let Some(number_of_wfc) = full_wfc.number_of_wfc {
                for prefix in ["PP_AEWFC.", "PP_PSWFC.", "PP_AEWFC_REL."] {
                    let count = full_wfc.entry_count(prefix);
                    if count != 0 && count != number_of_wfc {
                        let family = prefix.trim_end_matches('.');
                        return Err(crate::UpfError::Validation(format!(
                            "PP_FULL_WFC declares number_of_wfc {number_of_wfc} but contains {count} {family} entries"
                        )));
                    }
                }
            }
            for entry in &full_wfc.entries {
                Self::validate_len(&entry.tag.to_string(), entry.value.values.len(), mesh)?;
            }
        }
        if self.header.is_paw && self.paw.is_none() {
            return Err(crate::UpfError::Validation(
                "PP_HEADER marks the dataset as PAW but PP_PAW is missing".into(),
            ));
        }
        if self.header.has_so && self.spin_orb.is_none() {
            return Err(crate::UpfError::Validation(
                "PP_HEADER marks the dataset as spin-orbit-enabled but PP_SPIN_ORB is missing"
                    .into(),
            ));
        }
        // Validate PP_SPIN_ORB entry counts against header.
        if let Some(spin_orb) = &self.spin_orb {
            if self.header.number_of_wfc > 0
                && spin_orb.relwfcs.len() != self.header.number_of_wfc
            {
                return Err(crate::UpfError::Validation(format!(
                    "PP_HEADER declares {} wavefunctions but PP_SPIN_ORB contains {} PP_RELWFC entries",
                    self.header.number_of_wfc,
                    spin_orb.relwfcs.len()
                )));
            }
            if self.header.number_of_proj > 0
                && spin_orb.relbetas.len() != self.header.number_of_proj
            {
                return Err(crate::UpfError::Validation(format!(
                    "PP_HEADER declares {} projectors but PP_SPIN_ORB contains {} PP_RELBETA entries",
                    self.header.number_of_proj,
                    spin_orb.relbetas.len()
                )));
            }
        }
        // Validate PP_AEWFC_REL presence: only allowed when has_so && is_paw.
        if let Some(full_wfc) = &self.full_wfc {
            let rel_count = full_wfc.entry_count("PP_AEWFC_REL.");
            if rel_count > 0 && !(self.header.has_so && self.header.is_paw) {
                return Err(crate::UpfError::Validation(
                    "PP_FULL_WFC contains PP_AEWFC_REL entries but has_so and is_paw are not both true"
                        .into(),
                ));
            }
            if self.header.has_so && self.header.is_paw && self.header.has_wfc && rel_count == 0 {
                return Err(crate::UpfError::Validation(
                    "PP_HEADER has has_so, is_paw, and has_wfc all true but PP_FULL_WFC contains no PP_AEWFC_REL entries"
                        .into(),
                ));
            }
        }
        if let Some(paw) = &self.paw {
            if paw.occupations.is_none() {
                return Err(crate::UpfError::Validation(
                    "PP_PAW is present but PP_OCCUPATIONS is missing".into(),
                ));
            }
            Self::validate_optional_len(
                "PP_PAW/PP_OCCUPATIONS",
                paw.occupations.as_ref(),
                self.header.number_of_proj,
            )?;
            Self::validate_optional_len("PP_PAW/PP_AE_NLCC", paw.ae_nlcc.as_ref(), mesh)?;
            Self::validate_optional_len("PP_PAW/PP_AE_VLOC", paw.ae_vloc.as_ref(), mesh)?;
        }
        if self.header.has_gipaw && self.gipaw.is_none() {
            return Err(crate::UpfError::Validation(
                "PP_HEADER marks the dataset as GIPAW-enabled but PP_GIPAW is missing".into(),
            ));
        }
        // When paw_as_gipaw is true, only core orbitals are present.
        if self.header.paw_as_gipaw
            && let Some(gipaw) = &self.gipaw
        {
            if gipaw.orbitals.is_some() {
                return Err(crate::UpfError::Validation(
                    "paw_as_gipaw is true but PP_GIPAW_ORBITALS is present".into(),
                ));
            }
            if gipaw.vlocal.is_some() {
                return Err(crate::UpfError::Validation(
                    "paw_as_gipaw is true but PP_GIPAW_VLOCAL is present".into(),
                ));
            }
        }
        if (self.header.is_ultrasoft || self.header.is_paw) && self.nonlocal.augmentation.is_none()
        {
            return Err(crate::UpfError::Validation(
                "ultrasoft or PAW datasets require PP_AUGMENTATION inside PP_NONLOCAL".into(),
            ));
        }
        if let Some(augmentation) = &self.nonlocal.augmentation {
            let proj = self.header.number_of_proj;
            let effective_nqlc = Self::effective_nqlc(augmentation, self.header.l_max);
            Self::validate_optional_len("PP_AUGMENTATION/PP_Q", augmentation.q.as_ref(), proj * proj)?;
            Self::validate_optional_len(
                "PP_AUGMENTATION/PP_MULTIPOLES",
                augmentation.multipoles.as_ref(),
                proj * proj * (2 * self.header.l_max + 1),
            )?;
            Self::validate_optional_len(
                "PP_AUGMENTATION/PP_RINNER",
                augmentation.rinner.as_ref(),
                effective_nqlc,
            )?;
            Self::validate_optional_len(
                "PP_AUGMENTATION/PP_QFCOEF",
                augmentation.qfcoef.as_ref(),
                augmentation.nqf * effective_nqlc * proj * proj,
            )?;
            for channel in &augmentation.channels {
                Self::validate_len(&channel.tag, channel.value.values.len(), mesh)?;
            }
            // Validate q_with_l consistency with channel naming and angular_momentum.
            for channel in &augmentation.channels {
                let dot_count = channel.tag.chars().filter(|&c| c == '.').count();
                if augmentation.q_with_l {
                    if channel.value.angular_momentum.is_none() {
                        return Err(crate::UpfError::Validation(format!(
                            "q_with_l is true but {} has no angular_momentum attribute",
                            channel.tag
                        )));
                    }
                    if dot_count != 3 {
                        return Err(crate::UpfError::Validation(format!(
                            "q_with_l is true but {} does not use PP_QIJL.i.j.l naming",
                            channel.tag
                        )));
                    }
                } else {
                    if channel.value.angular_momentum.is_some() {
                        return Err(crate::UpfError::Validation(format!(
                            "q_with_l is false but {} has angular_momentum attribute",
                            channel.tag
                        )));
                    }
                    if dot_count != 2 {
                        return Err(crate::UpfError::Validation(format!(
                            "q_with_l is false but {} does not use PP_QIJ.i.j naming",
                            channel.tag
                        )));
                    }
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
                Self::validate_len(&orbital.tag.to_string(), orbital.value.values.len(), mesh)?;
            }
        }
        if let Some(gipaw) = &self.gipaw {
            if let Some(orbitals) = &gipaw.orbitals {
                if orbitals.number_of_valence_orbitals != orbitals.orbitals.len() {
                    return Err(crate::UpfError::Validation(format!(
                        "PP_GIPAW_ORBITALS declares {} entries but contains {} orbitals",
                        orbitals.number_of_valence_orbitals,
                        orbitals.orbitals.len()
                    )));
                }
                for orbital in &orbitals.orbitals {
                    Self::validate_len(
                        "PP_GIPAW_ORBITAL/PP_GIPAW_WFS_AE",
                        orbital.value.ae.len(),
                        mesh,
                    )?;
                    Self::validate_len(
                        "PP_GIPAW_ORBITAL/PP_GIPAW_WFS_PS",
                        orbital.value.ps.len(),
                        mesh,
                    )?;
                }
            }
            if let Some(vlocal) = &gipaw.vlocal {
                Self::validate_len("PP_GIPAW_VLOCAL/PP_GIPAW_VLOCAL_AE", vlocal.ae.len(), mesh)?;
                Self::validate_len("PP_GIPAW_VLOCAL/PP_GIPAW_VLOCAL_PS", vlocal.ps.len(), mesh)?;
            }
        }
        Ok(())
    }
}
