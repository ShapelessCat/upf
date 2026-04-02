use thiserror::Error;

#[derive(Debug, Error)]
pub enum UpfError {
    #[error("xml decode error: {0}")]
    XmlDecode(#[from] quick_xml::DeError),

    #[error("xml encode error: {0}")]
    XmlEncode(#[from] quick_xml::se::SeError),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid floating-point value `{token}`")]
    InvalidFloat { token: String },

    #[error("invalid boolean flag `{value}`")]
    InvalidBoolFlag { value: String },

    #[error("{0}")]
    Validation(String),
}
