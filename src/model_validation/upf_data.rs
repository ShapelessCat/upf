use crate::UpfError;
use crate::model::UpfData;
use crate::model_validation::common::{
    ValidationContext, ValidationErrors, validate_len, validate_optional_len,
};

impl UpfData {
    /// Check structural invariants that are not enforced by XML shape alone.
    ///
    /// The current validation rules ensure that mesh-sized numeric sections
    /// match `PP_HEADER/@mesh_size`, and that header flags requiring optional
    /// section families are consistent with the presence and contents of those
    /// sections.
    pub fn validate(&self) -> Result<(), UpfError> {
        let cx = ValidationContext::new(&self.header);
        let mut errors = ValidationErrors::new();
        errors.extend_result(self.validate_top_level(&cx))?;
        errors.extend_result(self.validate_sections(&cx))?;
        errors.into_result()
    }

    pub(crate) fn validate_top_level(&self, cx: &ValidationContext<'_>) -> Result<(), UpfError> {
        let mut errors = ValidationErrors::new();

        errors.extend_result(self.mesh.validate_with(cx))?;

        if !cx.header().is_coulomb && self.local.is_empty() {
            errors.push("PP_HEADER marks the dataset as non-Coulomb but PP_LOCAL is missing");
        }
        if !self.local.is_empty() {
            errors.extend_result(validate_len("PP_LOCAL", self.local.len(), cx.mesh_size()))?;
        }
        errors.extend_result(validate_len("PP_RHOATOM", self.rhoatom.len(), cx.mesh_size()))?;
        errors.extend_result(validate_optional_len(
            "PP_NLCC",
            self.nlcc.as_deref(),
            cx.mesh_size(),
        ))?;
        errors.extend_result(validate_optional_len(
            "PP_TAUMOD",
            self.taumod.as_deref(),
            cx.mesh_size(),
        ))?;
        errors.extend_result(validate_optional_len(
            "PP_TAUATOM",
            self.tauatom.as_deref(),
            cx.mesh_size(),
        ))?;
        errors.into_result()
    }

    pub(crate) fn validate_sections(&self, cx: &ValidationContext<'_>) -> Result<(), UpfError> {
        let mut errors = ValidationErrors::new();

        if cx.header().core_correction && self.nlcc.is_none() {
            errors.push("PP_HEADER enables core correction but PP_NLCC is missing");
        }
        if cx.header().with_metagga_info && (self.taumod.is_none() || self.tauatom.is_none()) {
            errors.push("PP_HEADER enables metagga info but PP_TAUMOD or PP_TAUATOM is missing");
        }

        if let Some(nonlocal) = &self.nonlocal {
            errors.extend_result(nonlocal.validate_with(cx))?;
        } else {
            if cx.number_of_proj() != 0 {
                errors.push(format!(
                    "PP_HEADER declares {} projectors but PP_NONLOCAL contains 0 PP_BETA entries",
                    cx.number_of_proj()
                ));
            }
            if cx.is_ultrasoft() || cx.is_paw() {
                errors.push("ultrasoft or PAW datasets require PP_AUGMENTATION inside PP_NONLOCAL");
            }
        }

        if let Some(semilocal) = &self.semilocal {
            errors.extend_result(semilocal.validate_with(cx))?;
        }

        if cx.number_of_wfc() != 0 && self.pswfc.is_none() {
            errors.push("PP_HEADER declares wavefunctions but PP_PSWFC is missing");
        }
        if let Some(pswfc) = &self.pswfc {
            errors.extend_result(pswfc.validate_with(cx))?;
        }

        if cx.has_wfc() && self.full_wfc.is_none() {
            errors.push(
                "PP_HEADER marks the dataset as having full wavefunctions but PP_FULL_WFC is missing",
            );
        }
        if let Some(full_wfc) = &self.full_wfc {
            errors.extend_result(full_wfc.validate_with(cx))?;
            errors.extend_result(full_wfc.validate_rel_with(cx))?;
        }

        if let Some(spin_orb) = &self.spin_orb {
            errors.extend_result(spin_orb.validate_with(cx))?;
        } else if cx.has_so() {
            errors.push(
                "PP_HEADER marks the dataset as spin-orbit-enabled but PP_SPIN_ORB is missing",
            );
        }

        if let Some(paw) = &self.paw {
            errors.extend_result(paw.validate_with(cx))?;
        } else if cx.is_paw() {
            errors.push("PP_HEADER marks the dataset as PAW but PP_PAW is missing");
        }

        if let Some(gipaw) = &self.gipaw {
            errors.extend_result(gipaw.validate_with(cx))?;
            errors.extend_result(gipaw.validate_details_with(cx))?;
        } else if cx.header().has_gipaw {
            errors.push("PP_HEADER marks the dataset as GIPAW-enabled but PP_GIPAW is missing");
        }

        errors.into_result()
    }
}
