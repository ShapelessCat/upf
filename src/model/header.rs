use serde::{Deserialize, Serialize};

use super::internal::deserialize_f64;
use super::internal::bool_flag;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpHeader {
    /// Generation code in attribute `generated`.
    #[serde(rename = "@generated")]
    pub generated: String,
    /// Author in attribute `author`.
    #[serde(rename = "@author")]
    pub author: String,
    /// Generation date in attribute `date`.
    #[serde(rename = "@date")]
    pub date: String,
    /// Free-form summary in attribute `comment`.
    ///
    /// QE reads missing string attributes as empty text. `comment` is metadata
    /// only and does not affect the physical content or structural section
    /// contracts of the UPF, so absent values are normalized to `""`.
    #[serde(rename = "@comment", default)]
    pub comment: String,
    /// Chemical symbol in attribute `element`.
    #[serde(rename = "@element")]
    pub element: String,
    /// Pseudopotential kind in attribute `pseudo_type`.
    #[serde(rename = "@pseudo_type")]
    pub pseudo_type: PseudopotentialType,
    /// Relativistic mode in attribute `relativistic`.
    #[serde(rename = "@relativistic")]
    pub relativistic: AtomicRelativisticFormalism,
    /// Flag in attribute `is_ultrasoft`.
    #[serde(rename = "@is_ultrasoft", with = "bool_flag")]
    pub is_ultrasoft: bool,
    /// Flag in attribute `is_paw`.
    #[serde(rename = "@is_paw", with = "bool_flag")]
    pub is_paw: bool,
    /// Flag in attribute `is_coulomb`.
    #[serde(rename = "@is_coulomb", with = "bool_flag")]
    pub is_coulomb: bool,
    /// Flag in attribute `has_so`.
    #[serde(rename = "@has_so", with = "bool_flag")]
    pub has_so: bool,
    /// Flag in attribute `has_wfc`.
    ///
    /// The bundled UPF prose describes this as a boolean flag. The sibling
    /// `upf-schema` project currently types it as `xs:NCName`, so this crate
    /// intentionally enforces the stronger boolean interpretation.
    #[serde(rename = "@has_wfc", with = "bool_flag")]
    pub has_wfc: bool,
    /// Flag in attribute `has_gipaw`.
    #[serde(rename = "@has_gipaw", with = "bool_flag")]
    pub has_gipaw: bool,
    /// Flag in attribute `paw_as_gipaw`.
    ///
    /// As with `has_wfc`, the prose reference describes this as boolean while
    /// the current `upf-schema` XSD uses `xs:NCName`. The crate keeps the
    /// boolean semantics and writes canonical `T`/`F` UPF flags.
    #[serde(rename = "@paw_as_gipaw", default, with = "bool_flag")]
    pub paw_as_gipaw: bool,
    /// Flag in attribute `core_correction`.
    #[serde(rename = "@core_correction", with = "bool_flag")]
    pub core_correction: bool,
    /// Flag in attribute `with_metagga_info`.
    #[serde(rename = "@with_metagga_info", default, with = "bool_flag")]
    pub with_metagga_info: bool,
    /// Exchange-correlation functional label in attribute `functional`.
    #[serde(rename = "@functional")]
    pub functional: String,
    /// Valence charge in attribute `z_valence`.
    #[serde(rename = "@z_valence", deserialize_with = "deserialize_f64")]
    pub z_valence: f64,
    /// Total pseudopotential energy in attribute `total_psenergy`.
    #[serde(
        rename = "@total_psenergy",
        default,
        deserialize_with = "deserialize_f64",
        skip_serializing_if = "is_zero_f64"
    )]
    pub total_psenergy: f64,
    /// Suggested wavefunction cutoff in attribute `wfc_cutoff`.
    #[serde(
        rename = "@wfc_cutoff",
        default,
        deserialize_with = "deserialize_f64",
        skip_serializing_if = "is_zero_f64"
    )]
    pub wfc_cutoff: f64,
    /// Suggested charge-density cutoff in attribute `rho_cutoff`.
    #[serde(
        rename = "@rho_cutoff",
        default,
        deserialize_with = "deserialize_f64",
        skip_serializing_if = "is_zero_f64"
    )]
    pub rho_cutoff: f64,
    /// Maximum angular momentum in attribute `l_max`.
    #[serde(rename = "@l_max")]
    pub l_max: usize,
    /// Maximum rho angular momentum in attribute `l_max_rho`.
    #[serde(rename = "@l_max_rho", default, skip_serializing_if = "is_zero_usize")]
    pub l_max_rho: usize,
    /// Local channel angular momentum in attribute `l_local`.
    #[serde(rename = "@l_local", default, skip_serializing_if = "is_zero_isize")]
    pub l_local: isize,
    /// Declared radial grid length used by several other sections.
    #[serde(rename = "@mesh_size")]
    pub mesh_size: usize,
    /// Number of pseudo-wavefunctions in attribute `number_of_wfc`.
    #[serde(rename = "@number_of_wfc")]
    pub number_of_wfc: usize,
    /// Number of projectors in attribute `number_of_proj`.
    #[serde(rename = "@number_of_proj")]
    pub number_of_proj: usize,
}

/// `PP_HEADER/@pseudo_type` values used by UPF datasets.
///
/// The Rust variant names expand the compact UPF wire values while serde keeps
/// the serialized attribute values aligned with the format (`NC`, `SL`, `1/r`,
/// `US`, `PAW`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PseudopotentialType {
    /// Norm-conserving pseudopotential in fully nonlocal form.
    #[serde(rename = "NC")]
    NormConserving,
    /// Norm-conserving pseudopotential with semilocal and nonlocal forms.
    #[serde(rename = "SL")]
    Semilocal,
    /// Coulomb `1/r` potential.
    #[serde(rename = "1/r")]
    Coulomb,
    /// Ultrasoft pseudopotential.
    #[serde(rename = "US", alias = "USPP")]
    Ultrasoft,
    /// Projector-augmented-wave dataset.
    #[serde(rename = "PAW")]
    ProjectorAugmentedWave,
}

/// `PP_HEADER/@relativistic` values used by the atomic calculation metadata.
///
/// QE writes `no`, `scalar`, and `full`. The older prose reference used
/// `nonrelativistic` for the first variant. The crate serializes the canonical
/// QE spelling `no` and accepts `nonrelativistic` as an alias on input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AtomicRelativisticFormalism {
    /// Non-relativistic atomic calculation.
    #[serde(rename = "no", alias = "nonrelativistic")]
    NonRelativistic,
    /// Scalar-relativistic atomic calculation.
    #[serde(rename = "scalar")]
    ScalarRelativistic,
    /// Fully relativistic atomic calculation.
    #[serde(rename = "full")]
    FullyRelativistic,
}
fn is_zero_f64(value: &f64) -> bool {
    *value == 0.0
}

fn is_zero_usize(value: &usize) -> bool {
    *value == 0
}

fn is_zero_isize(value: &isize) -> bool {
    *value == 0
}
