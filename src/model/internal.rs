//! Helper types and serde glue shared across multiple UPF section models.

mod scalar_text;
mod numeric_values;
mod numeric_section;
mod tagged_children;

pub(crate) use scalar_text::{
    bool_flag,
    deserialize_f64,
    deserialize_opt_f64,
    format_bool_flag,
};
pub(crate) use numeric_values::{deserialize_f64_values, format_f64_slice};
pub(crate) use numeric_section::{
    NumericSectionTextValueRef,
    numeric_section_vec,
    optional_numeric_section_vec,
};
pub use tagged_children::{Numbered, NumberedTag};
pub(crate) use tagged_children::Tagged;
