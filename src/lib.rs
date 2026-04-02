mod de;
mod error;
pub mod model;
mod text;

pub use de::{from_file, from_reader, from_str};
pub use error::UpfError;
pub use model::UpfData;
pub use text::{format_bool_flag, format_f64_slice, parse_bool_flag, parse_f64_vec};
