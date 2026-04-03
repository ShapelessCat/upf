//! Rust types and helpers for the Unified Pseudopotential Format (UPF).
//!
//! This crate models UPF `2.0.1` documents as typed Rust data structures built
//! around the top-level [`UpfData`] type. Parsing and serialization are backed
//! by `quick-xml`, while a small validation layer enforces invariants that are
//! required by the UPF structure used in this repository:
//!
//! - `PP_HEADER/@mesh_size` must match the lengths of `PP_R`, `PP_RAB`,
//!   `PP_LOCAL`, and `PP_RHOATOM`
//! - `PP_HEADER/@is_paw="T"` requires a `PP_PAW` section
//! - `PP_HEADER/@has_gipaw="T"` requires a `PP_GIPAW` section
//!
//! The public API intentionally stays small:
//!
//! - [`from_str`], [`from_reader`], and [`from_file`] deserialize UPF text
//! - [`to_string`], [`to_writer`], and [`to_file`] serialize validated models
//! - [`model`] exposes Rust representations of UPF sections
//!
//! # Example
//!
//! ```rust
//! use upf::from_str;
//!
//! let xml = r#"
//! <UPF version="2.0.1">
//!   <PP_HEADER generated="unit" author="tester" date="2026-04-03" comment="demo"
//!              element="He" pseudo_type="NC" relativistic="scalar"
//!              is_ultrasoft="F" is_paw="F" is_coulomb="F"
//!              has_so="F" has_wfc="F" has_gipaw="F" core_correction="F"
//!              z_valence="2.0" total_psenergy="-1.25"
//!              wfc_cutoff="20.0" rho_cutoff="80.0"
//!              l_max="0" l_max_rho="0" l_local="0"
//!              mesh_size="3" number_of_wfc="0" number_of_proj="0" />
//!   <PP_MESH dx="0.1" mesh="3" xmin="0.0" rmax="0.2" zmesh="1.0">
//!     <PP_R>0.0 0.1 0.2</PP_R>
//!     <PP_RAB>0.1 0.1 0.1</PP_RAB>
//!   </PP_MESH>
//!   <PP_LOCAL>1.0 2.0 3.0</PP_LOCAL>
//!   <PP_NONLOCAL />
//!   <PP_RHOATOM>0.2 0.3 0.4</PP_RHOATOM>
//! </UPF>
//! "#;
//!
//! let upf_data = from_str(xml).unwrap();
//! assert_eq!(upf_data.header.element, "He");
//! assert_eq!(upf_data.mesh.r.values.len(), upf_data.header.mesh_size);
//! ```

mod de;
mod error;
/// Rust representations of UPF sections and sub-sections.
pub mod model;
mod ser;
mod text;

pub use de::{from_file, from_reader, from_str};
pub use error::UpfError;
pub use model::UpfData;
pub use ser::{to_file, to_string, to_writer};
pub use text::{format_bool_flag, format_f64_slice, parse_bool_flag, parse_f64_vec};
