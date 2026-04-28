//! Typed Rust models for UPF sections.
//!
//! The module layout keeps direct `UpfData` section families at the top level,
//! with `internal` reserved for serde/helper glue and `common` reserved for
//! reusable section shapes shared across top-level sections.

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
#[rustfmt::skip] mod internal;
#[rustfmt::skip] pub mod common;

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
#[rustfmt::skip] pub use internal::{Numbered, NumberedTag};
