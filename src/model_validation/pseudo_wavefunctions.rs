use crate::UpfError;
use crate::model::PpPseudoWavefunctions;
use crate::model_validation::common::{ValidationContext, ValidationErrors, validate_len};

impl PpPseudoWavefunctions {
    pub(crate) fn validate_with(&self, cx: &ValidationContext<'_>) -> Result<(), UpfError> {
        let mut errors = ValidationErrors::new();

        if cx.number_of_wfc() != 0 && self.orbitals.len() != cx.number_of_wfc() {
            errors.push(format!(
                "PP_HEADER declares {} wavefunctions but PP_PSWFC contains {} PP_CHI entries",
                cx.number_of_wfc(),
                self.orbitals.len()
            ));
        }

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
