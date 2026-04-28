use thiserror::Error;

/// Errors produced while parsing, validating, or serializing UPF documents.
#[derive(Debug, Error)]
pub enum UpfError {
    /// XML deserialization failed before semantic validation could run.
    #[error("xml decode error: {0}")]
    XmlDecode(#[from] quick_xml::DeError),

    /// XML serialization failed while turning a model back into UPF text.
    #[error("xml encode error: {0}")]
    XmlEncode(#[from] quick_xml::se::SeError),

    /// Reading from or writing to an external stream or file failed.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// A whitespace-delimited numeric field contained a token that is not `f64`.
    #[error("invalid floating-point value `{token}`")]
    InvalidFloat { token: String },

    /// A UPF-style boolean flag could not be interpreted as true or false.
    #[error("invalid boolean flag `{value}`")]
    InvalidBoolFlag { value: String },

    /// The document was syntactically readable but violates crate invariants.
    #[error("{}", format_validation_errors(.0))]
    Validation(Vec<String>),
}

fn format_validation_errors(messages: &[String]) -> String {
    let label = if messages.len() == 1 {
        "validation error"
    } else {
        "validation errors"
    };
    let mut rendered = format!("{} {label}:", messages.len());
    for message in messages {
        rendered.push('\n');
        rendered.push_str("- ");
        rendered.push_str(message);
    }
    rendered
}

#[cfg(test)]
mod tests {
    use super::UpfError;

    #[test]
    fn validation_error_display_lists_count_and_messages() {
        let err = UpfError::Validation(vec!["first problem".into(), "second problem".into()]);

        let message = err.to_string();
        assert!(message.contains("2 validation errors"));
        assert!(message.contains("first problem"));
        assert!(message.contains("second problem"));
    }
}
