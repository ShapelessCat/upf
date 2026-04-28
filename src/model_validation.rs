//! Private validation layer for the public UPF model types.
//!
//! Validation code lives outside `model` so serde mappings and structural
//! invariants stay physically separate while `UpfData::validate()` remains the
//! public entry point.

#[rustfmt::skip] pub(crate) mod common;
#[rustfmt::skip] mod upf_data;
#[rustfmt::skip] mod mesh;
#[rustfmt::skip] mod semilocal;
#[rustfmt::skip] mod nonlocal;
#[rustfmt::skip] mod pseudo_wavefunctions;
#[rustfmt::skip] mod full_wfc;
#[rustfmt::skip] mod paw;
#[rustfmt::skip] mod gipaw;
#[rustfmt::skip] mod spin_orb;
