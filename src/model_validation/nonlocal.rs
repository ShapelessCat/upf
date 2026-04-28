use crate::UpfError;
use crate::model::{AugmentationChannel, PpAugmentation, PpNonlocal};
use crate::model_validation::common::{
    ValidationContext, ValidationErrors, validate_len, validate_optional_len,
};

impl PpNonlocal {
    pub(crate) fn validate_with(&self, cx: &ValidationContext<'_>) -> Result<(), UpfError> {
        let mut errors = ValidationErrors::new();

        errors.extend_result(self.validate_projectors_with(cx))?;
        errors.extend_result(self.validate_augmentation_with(cx))?;
        errors.extend_result(self.validate_dij_with(cx))?;
        errors.into_result()
    }

    pub(crate) fn validate_projectors_with(
        &self,
        cx: &ValidationContext<'_>,
    ) -> Result<(), UpfError> {
        let mut errors = ValidationErrors::new();

        if cx.number_of_proj() != self.betas.len() {
            errors.push(format!(
                "PP_HEADER declares {} projectors but PP_NONLOCAL contains {} PP_BETA entries",
                cx.number_of_proj(),
                self.betas.len()
            ));
        }

        for beta in &self.betas {
            errors.extend_result(validate_len(
                &beta.tag.to_string(),
                beta.value.values.len(),
                cx.mesh_size(),
            ))?;
        }

        errors.into_result()
    }

    pub(crate) fn validate_augmentation_with(
        &self,
        cx: &ValidationContext<'_>,
    ) -> Result<(), UpfError> {
        let mut errors = ValidationErrors::new();

        if let Some(augmentation) = &self.augmentation {
            errors.extend_result(augmentation.validate_with(cx))?;
        } else if cx.is_ultrasoft() || cx.is_paw() {
            errors.push("ultrasoft or PAW datasets require PP_AUGMENTATION inside PP_NONLOCAL");
        }

        errors.into_result()
    }

    pub(crate) fn validate_dij_with(&self, cx: &ValidationContext<'_>) -> Result<(), UpfError> {
        validate_len(
            "PP_DIJ",
            self.dij.len(),
            cx.number_of_proj() * cx.number_of_proj(),
        )
    }
}

impl PpAugmentation {
    pub(crate) fn validate(&self) -> Result<(), UpfError> {
        let mut errors = ValidationErrors::new();

        for channel in &self.channels {
            match (self.q_with_l, channel) {
                (true, AugmentationChannel::Qijl { .. }) => {}
                (true, _) => {
                    errors.push(format!(
                        "q_with_l is true but {} does not use PP_QIJL.i.j.l naming",
                        channel.tag_name()
                    ));
                }
                (false, AugmentationChannel::Qij { .. }) => {}
                (false, _) => {
                    errors.push(format!(
                        "q_with_l is false but {} does not use PP_QIJ.i.j naming",
                        channel.tag_name()
                    ));
                }
            }
        }

        errors.into_result()
    }

    pub(crate) fn validate_with(&self, cx: &ValidationContext<'_>) -> Result<(), UpfError> {
        let proj = cx.number_of_proj();
        let effective_nqlc = cx.effective_nqlc(self);
        let mut errors = ValidationErrors::new();

        errors.extend_result(self.validate())?;

        errors.extend_result(validate_optional_len(
            "PP_AUGMENTATION/PP_Q",
            self.q.as_deref(),
            proj * proj,
        ))?;
        errors.extend_result(validate_optional_len(
            "PP_AUGMENTATION/PP_MULTIPOLES",
            self.multipoles.as_deref(),
            proj * proj * (2 * cx.l_max() + 1),
        ))?;
        errors.extend_result(validate_optional_len(
            "PP_AUGMENTATION/PP_RINNER",
            self.rinner.as_deref(),
            effective_nqlc,
        ))?;
        errors.extend_result(validate_optional_len(
            "PP_AUGMENTATION/PP_QFCOEF",
            self.qfcoef.as_deref(),
            self.nqf * effective_nqlc * proj * proj,
        ))?;

        for channel in &self.channels {
            errors.extend_result(validate_len(
                &channel.tag_name(),
                channel.values().len(),
                cx.mesh_size(),
            ))?;
        }

        errors.into_result()
    }
}
