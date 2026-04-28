use crate::UpfError;
use crate::model::PpMesh;
use crate::model_validation::common::{ValidationContext, ValidationErrors, validate_len};

impl PpMesh {
    pub(crate) fn validate_with(&self, cx: &ValidationContext<'_>) -> Result<(), UpfError> {
        let mesh_size = cx.mesh_size();
        let mut errors = ValidationErrors::new();

        if let Some(declared_mesh) = self.mesh
            && declared_mesh != mesh_size
        {
            errors.push(format!(
                "PP_MESH declares mesh {} but PP_HEADER mesh_size is {}",
                declared_mesh, mesh_size
            ));
        }

        errors.extend_result(validate_len("PP_R", self.r.len(), mesh_size))?;
        errors.extend_result(validate_len("PP_RAB", self.rab.len(), mesh_size))?;
        errors.into_result()
    }
}
