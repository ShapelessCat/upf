//! Typed Rust models for UPF sections.
//!
//! The module layout follows the major section families from the UPF 2.0.1
//! reference:
//!
//! - core types cover the document root, header, mesh, and shared numeric arrays
//! - nonlocal types cover `PP_INFO`, `PP_NONLOCAL`, `PP_SEMILOCAL`, and related nodes
//! - PAW types cover PAW-only sections
//! - GIPAW types cover GIPAW-only sections

mod core;
mod gipaw;
mod nonlocal;
mod paw;

pub use core::*;
pub use gipaw::*;
pub use nonlocal::*;
pub use paw::*;
