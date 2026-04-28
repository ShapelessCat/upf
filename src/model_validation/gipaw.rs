use crate::UpfError;
use crate::model::{
    GipawCoreOrbitals, GipawValenceOrbitals, GipawVlocal, PpGipaw,
};
use crate::model_validation::common::{ValidationContext, ValidationErrors, validate_len};

impl PpGipaw {
    pub(crate) fn validate_with(&self, cx: &ValidationContext<'_>) -> Result<(), UpfError> {
        let mut errors = ValidationErrors::new();

        if cx.paw_as_gipaw() {
            if self.orbitals.is_some() {
                errors.push("paw_as_gipaw is true but PP_GIPAW_ORBITALS is present");
            }
            if self.vlocal.is_some() {
                errors.push("paw_as_gipaw is true but PP_GIPAW_VLOCAL is present");
            }
        } else {
            if self.orbitals.is_none() {
                errors.push(
                    "PP_HEADER has_gipaw is true and paw_as_gipaw is false but PP_GIPAW_ORBITALS is missing",
                );
            }
            if self.vlocal.is_none() {
                errors.push(
                    "PP_HEADER has_gipaw is true and paw_as_gipaw is false but PP_GIPAW_VLOCAL is missing",
                );
            }
        }

        errors.into_result()
    }

    pub(crate) fn validate_details_with(&self, cx: &ValidationContext<'_>) -> Result<(), UpfError> {
        let mut errors = ValidationErrors::new();

        errors.extend_result(self.core_orbitals.validate_with(cx))?;

        if let Some(orbitals) = &self.orbitals {
            errors.extend_result(orbitals.validate_with(cx))?;
        }

        if let Some(vlocal) = &self.vlocal {
            errors.extend_result(vlocal.validate_with(cx))?;
        }

        errors.into_result()
    }
}

impl GipawCoreOrbitals {
    pub(crate) fn validate(&self) -> Result<(), UpfError> {
        let mut errors = ValidationErrors::new();

        if self.number_of_core_orbitals != self.orbitals.len() {
            errors.push(format!(
                "PP_GIPAW_CORE_ORBITALS declares {} entries but contains {} orbitals",
                self.number_of_core_orbitals,
                self.orbitals.len()
            ));
        }

        for orbital in &self.orbitals {
            if orbital.tag.index != orbital.value.index {
                errors.push(format!(
                    "{} tag suffix {} does not match @index {}",
                    orbital.tag, orbital.tag.index, orbital.value.index
                ));
            }
        }

        errors.into_result()
    }

    pub(crate) fn validate_with(&self, cx: &ValidationContext<'_>) -> Result<(), UpfError> {
        let mut errors = ValidationErrors::new();

        errors.extend_result(self.validate())?;

        for orbital in &self.orbitals {
            errors.extend_result(validate_len(
                &orbital.tag.to_string(),
                orbital.value.values.len(),
                cx.mesh_size(),
            ))?;
        }

        errors.into_result()
    }
}

impl GipawValenceOrbitals {
    pub(crate) fn validate(&self) -> Result<(), UpfError> {
        let mut errors = ValidationErrors::new();

        if self.number_of_valence_orbitals != self.orbitals.len() {
            errors.push(format!(
                "PP_GIPAW_ORBITALS declares {} entries but contains {} orbitals",
                self.number_of_valence_orbitals,
                self.orbitals.len()
            ));
        }

        for orbital in &self.orbitals {
            let Some(index) = orbital.value.index else {
                errors.push(format!(
                    "{} is missing @index",
                    orbital.tag
                ));
                continue;
            };
            if orbital.tag.index != index {
                errors.push(format!(
                    "{} tag suffix {} does not match @index {}",
                    orbital.tag, orbital.tag.index, index
                ));
            }
        }

        errors.into_result()
    }

    pub(crate) fn validate_with(&self, cx: &ValidationContext<'_>) -> Result<(), UpfError> {
        let mut errors = ValidationErrors::new();

        errors.extend_result(self.validate())?;

        for orbital in &self.orbitals {
            errors.extend_result(validate_len(
                "PP_GIPAW_ORBITAL/PP_GIPAW_WFS_AE",
                orbital.value.ae.len(),
                cx.mesh_size(),
            ))?;
            errors.extend_result(validate_len(
                "PP_GIPAW_ORBITAL/PP_GIPAW_WFS_PS",
                orbital.value.ps.len(),
                cx.mesh_size(),
            ))?;
        }

        errors.into_result()
    }
}

impl GipawVlocal {
    pub(crate) fn validate_with(&self, cx: &ValidationContext<'_>) -> Result<(), UpfError> {
        let mut errors = ValidationErrors::new();
        errors.extend_result(validate_len(
            "PP_GIPAW_VLOCAL/PP_GIPAW_VLOCAL_AE",
            self.ae.len(),
            cx.mesh_size(),
        ))?;
        errors.extend_result(validate_len(
            "PP_GIPAW_VLOCAL/PP_GIPAW_VLOCAL_PS",
            self.ps.len(),
            cx.mesh_size(),
        ))?;
        errors.into_result()
    }
}
