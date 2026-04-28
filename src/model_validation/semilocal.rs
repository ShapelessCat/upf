use crate::UpfError;
use crate::model::PpSemilocal;
use crate::model_validation::common::{ValidationContext, ValidationErrors, validate_len};

impl PpSemilocal {
    pub(crate) fn validate_with(&self, cx: &ValidationContext<'_>) -> Result<(), UpfError> {
        let mut errors = ValidationErrors::new();

        for channel in &self.channels {
            errors.extend_result(validate_len(
                &channel.tag.to_string(),
                channel.value.values.len(),
                cx.mesh_size(),
            ))?;
        }

        errors.into_result()
    }
}
