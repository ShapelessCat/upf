use crate::UpfError;
use crate::model::{PpAugmentation, PpHeader};

#[derive(Debug, Default)]
pub(crate) struct ValidationErrors {
    messages: Vec<String>,
}

impl ValidationErrors {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn push(&mut self, message: impl Into<String>) {
        self.messages.push(message.into());
    }

    pub(crate) fn extend(&mut self, other: Self) {
        self.messages.extend(other.messages);
    }

    pub(crate) fn extend_result<T>(&mut self, result: Result<T, UpfError>) -> Result<(), UpfError> {
        match result {
            Ok(_) => Ok(()),
            Err(UpfError::Validation(messages)) => {
                self.extend(Self { messages });
                Ok(())
            }
            Err(other) => Err(other),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub(crate) fn into_result(self) -> Result<(), UpfError> {
        if self.is_empty() {
            Ok(())
        } else {
            Err(UpfError::Validation(self.messages))
        }
    }
}

pub(crate) struct ValidationContext<'a> {
    header: &'a PpHeader,
}

impl<'a> ValidationContext<'a> {
    pub(crate) fn new(header: &'a PpHeader) -> Self {
        Self { header }
    }

    pub(crate) fn header(&self) -> &PpHeader {
        self.header
    }

    pub(crate) fn mesh_size(&self) -> usize {
        self.header.mesh_size
    }

    pub(crate) fn number_of_proj(&self) -> usize {
        self.header.number_of_proj
    }

    pub(crate) fn number_of_wfc(&self) -> usize {
        self.header.number_of_wfc
    }

    pub(crate) fn l_max(&self) -> usize {
        self.header.l_max
    }

    pub(crate) fn is_paw(&self) -> bool {
        self.header.is_paw
    }

    pub(crate) fn is_ultrasoft(&self) -> bool {
        self.header.is_ultrasoft
    }

    pub(crate) fn has_so(&self) -> bool {
        self.header.has_so
    }

    pub(crate) fn has_wfc(&self) -> bool {
        self.header.has_wfc
    }

    pub(crate) fn paw_as_gipaw(&self) -> bool {
        self.header.paw_as_gipaw
    }

    pub(crate) fn effective_nqlc(&self, augmentation: &PpAugmentation) -> usize {
        if augmentation.nqlc == 0 {
            2 * self.l_max() + 1
        } else {
            augmentation.nqlc
        }
    }
}

pub(crate) fn validate_len(
    name: &str,
    actual: usize,
    expected: usize,
) -> Result<(), UpfError> {
    if actual != expected {
        return Err(UpfError::Validation(vec![format!(
            "{name} length {actual} does not match expected size {expected}"
        )]));
    }

    Ok(())
}

pub(crate) fn validate_optional_len(
    name: &str,
    section: Option<&[f64]>,
    expected: usize,
) -> Result<(), UpfError> {
    if let Some(section) = section {
        validate_len(name, section.len(), expected)?;
    }

    Ok(())
}
