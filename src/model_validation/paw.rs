use crate::UpfError;
use crate::model::PpPaw;
use crate::model_validation::common::{ValidationContext, ValidationErrors, validate_len};

impl PpPaw {
    pub(crate) fn validate_with(&self, cx: &ValidationContext<'_>) -> Result<(), UpfError> {
        let mut errors = ValidationErrors::new();

        errors.extend_result(validate_len(
            "PP_PAW/PP_OCCUPATIONS",
            self.occupations.len(),
            cx.number_of_proj(),
        ))?;
        errors.extend_result(validate_len(
            "PP_PAW/PP_AE_NLCC",
            self.ae_nlcc.len(),
            cx.mesh_size(),
        ))?;
        errors.extend_result(validate_len(
            "PP_PAW/PP_AE_VLOC",
            self.ae_vloc.len(),
            cx.mesh_size(),
        ))?;
        errors.into_result()
    }
}
