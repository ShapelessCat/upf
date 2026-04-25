//! Typed Rust models for UPF sections.
//!
//! The module layout follows the major top-level section families from the UPF
//! 2.0.1 reference, with one module per non-builtin `UpfData` field type and
//! shared helpers extracted when reused across sections.

#[rustfmt::skip] mod upf_data;
#[rustfmt::skip] mod info;
#[rustfmt::skip] mod header;
#[rustfmt::skip] mod mesh;
#[rustfmt::skip] mod semilocal;
#[rustfmt::skip] mod nonlocal;
#[rustfmt::skip] mod pseudo_wavefunctions;
#[rustfmt::skip] mod full_wfc;
#[rustfmt::skip] mod paw;
#[rustfmt::skip] mod gipaw;
#[rustfmt::skip] mod spin_orb;
#[rustfmt::skip] mod wavefunction;
#[rustfmt::skip] mod numeric_text;
#[rustfmt::skip] mod data_section;
#[rustfmt::skip] mod numbered;

#[rustfmt::skip] pub use upf_data::*;
#[rustfmt::skip] pub use info::*;
#[rustfmt::skip] pub use header::*;
#[rustfmt::skip] pub use mesh::*;
#[rustfmt::skip] pub use semilocal::*;
#[rustfmt::skip] pub use nonlocal::*;
#[rustfmt::skip] pub use pseudo_wavefunctions::*;
#[rustfmt::skip] pub use full_wfc::*;
#[rustfmt::skip] pub use paw::*;
#[rustfmt::skip] pub use gipaw::*;
#[rustfmt::skip] pub use spin_orb::*;
#[rustfmt::skip] pub use wavefunction::*;
#[rustfmt::skip] pub(crate) use data_section::{
    NumericSectionTextValueRef,
    numeric_section_vec,
    optional_numeric_section_vec,
};
#[rustfmt::skip] pub use numbered::*;
