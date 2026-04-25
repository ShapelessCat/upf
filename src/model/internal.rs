//! Helper types and serde glue shared across multiple UPF section models.

mod data_section;
mod numeric_text;
mod numbered;

pub(crate) use data_section::{
    NumericSectionTextValueRef,
    numeric_section_vec,
    optional_numeric_section_vec,
};
pub(crate) use numeric_text::deserialize_f64_values;
pub use numbered::{Numbered, NumberedTag, Tagged};
