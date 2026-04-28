use crate::UpfError;
use crate::model::PpSpinOrb;
use crate::model_validation::common::{ValidationContext, ValidationErrors};

impl PpSpinOrb {
    pub(crate) fn validate_with(&self, cx: &ValidationContext<'_>) -> Result<(), UpfError> {
        let mut errors = ValidationErrors::new();

        if cx.number_of_wfc() > 0 && self.relwfcs.len() != cx.number_of_wfc() {
            errors.push(format!(
                "PP_HEADER declares {} wavefunctions but PP_SPIN_ORB contains {} PP_RELWFC entries",
                cx.number_of_wfc(),
                self.relwfcs.len()
            ));
        }

        if cx.number_of_proj() > 0 && self.relbetas.len() != cx.number_of_proj() {
            errors.push(format!(
                "PP_HEADER declares {} projectors but PP_SPIN_ORB contains {} PP_RELBETA entries",
                cx.number_of_proj(),
                self.relbetas.len()
            ));
        }

        errors.into_result()
    }
}
