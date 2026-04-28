use crate::UpfError;
use crate::model::PpFullWfc;
use crate::model_validation::common::{ValidationContext, ValidationErrors, validate_len};

impl PpFullWfc {
    pub(crate) fn validate_with(&self, cx: &ValidationContext<'_>) -> Result<(), UpfError> {
        let expected_proj_entries = cx.number_of_proj();
        let mut errors = ValidationErrors::new();

        for (tag_category_name, entries) in [
            ("PP_AEWFC", &self.ae_wfc),
            ("PP_PSWFC", &self.ps_wfc),
        ] {
            if entries.len() != expected_proj_entries {
                errors.push(format!(
                    "PP_FULL_WFC contains {} {tag_category_name} entries but PP_HEADER number_of_proj is {expected_proj_entries}",
                    entries.len()
                ));
            }
        }

        for entry in &self.ae_wfc {
            errors.extend_result(validate_len(
                &entry.tag.to_string(),
                entry.value.values.len(),
                cx.mesh_size(),
            ))?;
        }

        if let Some(ae_wfc_rel) = &self.ae_wfc_rel {
            for entry in ae_wfc_rel {
                errors.extend_result(validate_len(
                    &entry.tag.to_string(),
                    entry.value.values.len(),
                    cx.mesh_size(),
                ))?;
            }
        }

        for entry in &self.ps_wfc {
            errors.extend_result(validate_len(
                &entry.tag.to_string(),
                entry.value.values.len(),
                cx.mesh_size(),
            ))?;
        }

        errors.into_result()
    }

    pub(crate) fn validate_rel_with(&self, cx: &ValidationContext<'_>) -> Result<(), UpfError> {
        let expected_proj_entries = cx.number_of_proj();
        let mut errors = ValidationErrors::new();

        match (&self.ae_wfc_rel, cx.has_so() && cx.is_paw() && cx.has_wfc()) {
            (Some(_), false) => {
                errors.push(
                    "PP_FULL_WFC contains PP_AEWFC_REL entries but has_so and is_paw are not both true",
                );
            }
            (None, true) => {
                errors.push(
                    "PP_FULL_WFC is missing PP_AEWFC_REL entries required when has_so, is_paw, and has_wfc are all true",
                );
            }
            (Some(ae_wfc_rel), true) => {
                if ae_wfc_rel.len() != expected_proj_entries {
                    errors.push(format!(
                        "PP_FULL_WFC contains {} PP_AEWFC_REL entries but PP_HEADER number_of_proj is {expected_proj_entries}",
                        ae_wfc_rel.len()
                    ));
                }
            }
            (None, false) => {}
        }

        errors.into_result()
    }
}
