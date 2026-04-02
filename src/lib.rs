mod error;
mod text;

pub use error::UpfError;
pub use text::{format_bool_flag, format_f64_slice, parse_bool_flag, parse_f64_vec};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct UpfDocument;
