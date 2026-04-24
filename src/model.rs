//! Typed Rust models for UPF sections.
//!
//! The module layout follows the major top-level section families from the UPF
//! 2.0.1 reference, with one module per non-builtin `UpfData` field type and
//! shared helpers extracted when reused across sections.

mod upf_data;
mod info;
mod header;
mod mesh;
mod semilocal;
mod nonlocal;
mod pseudo_wavefunctions;
mod full_wfc;
mod paw;
mod gipaw;
mod wavefunction;
mod numeric_text;
mod data_section;
mod numbered;

pub use upf_data::*;
pub use info::*;
pub use header::*;
pub use mesh::*;
pub use semilocal::*;
pub use nonlocal::*;
pub use pseudo_wavefunctions::*;
pub use full_wfc::*;
pub use paw::*;
pub use gipaw::*;
pub use wavefunction::*;
pub use data_section::*;
pub use numbered::*;
