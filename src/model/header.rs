use serde::{Deserialize, Serialize};

use crate::text::{deserialize_bool_flag, serialize_bool_flag};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpHeader {
    /// Generation code recorded in `@generated`.
    #[serde(rename = "@generated")]
    pub generated: String,
    /// Author recorded in `@author`.
    #[serde(rename = "@author")]
    pub author: String,
    /// Generation date recorded in `@date`.
    #[serde(rename = "@date")]
    pub date: String,
    /// Free-form summary from `@comment`.
    #[serde(rename = "@comment")]
    pub comment: String,
    /// Chemical symbol from `@element`.
    #[serde(rename = "@element")]
    pub element: String,
    /// Pseudopotential kind from `@pseudo_type`.
    #[serde(rename = "@pseudo_type")]
    pub pseudo_type: PseudopotentialType,
    /// Relativistic mode from `@relativistic`.
    #[serde(rename = "@relativistic")]
    pub relativistic: AtomicRelativisticFormalism,
    /// Flag from `@is_ultrasoft`.
    #[serde(
        rename = "@is_ultrasoft",
        deserialize_with = "deserialize_bool_flag",
        serialize_with = "serialize_bool_flag"
    )]
    pub is_ultrasoft: bool,
    /// Flag from `@is_paw`.
    #[serde(
        rename = "@is_paw",
        deserialize_with = "deserialize_bool_flag",
        serialize_with = "serialize_bool_flag"
    )]
    pub is_paw: bool,
    /// Flag from `@is_coulomb`.
    #[serde(
        rename = "@is_coulomb",
        deserialize_with = "deserialize_bool_flag",
        serialize_with = "serialize_bool_flag"
    )]
    pub is_coulomb: bool,
    /// Flag from `@has_so`.
    #[serde(
        rename = "@has_so",
        deserialize_with = "deserialize_bool_flag",
        serialize_with = "serialize_bool_flag"
    )]
    pub has_so: bool,
    /// Flag from `@has_wfc`.
    ///
    /// The bundled UPF prose describes this as a boolean flag. The sibling
    /// `upf-schema` project currently types it as `xs:NCName`, so this crate
    /// intentionally enforces the stronger boolean interpretation.
    #[serde(
        rename = "@has_wfc",
        deserialize_with = "deserialize_bool_flag",
        serialize_with = "serialize_bool_flag"
    )]
    pub has_wfc: bool,
    /// Flag from `@has_gipaw`.
    #[serde(
        rename = "@has_gipaw",
        deserialize_with = "deserialize_bool_flag",
        serialize_with = "serialize_bool_flag"
    )]
    pub has_gipaw: bool,
    /// Flag from `@paw_as_gipaw`, defaulting to `false` when absent.
    ///
    /// As with `has_wfc`, the prose reference describes this as boolean while
    /// the current `upf-schema` XSD uses `xs:NCName`. The crate keeps the
    /// boolean semantics and writes canonical `T`/`F` UPF flags.
    #[serde(
        rename = "@paw_as_gipaw",
        default,
        deserialize_with = "deserialize_bool_flag",
        serialize_with = "serialize_bool_flag"
    )]
    pub paw_as_gipaw: bool,
    /// Flag from `@core_correction`.
    #[serde(
        rename = "@core_correction",
        deserialize_with = "deserialize_bool_flag",
        serialize_with = "serialize_bool_flag"
    )]
    pub core_correction: bool,
    /// Exchange-correlation functional label from `@functional`.
    #[serde(rename = "@functional")]
    pub functional: String,
    /// Valence charge from `@z_valence`.
    #[serde(rename = "@z_valence")]
    pub z_valence: f64,
    /// Total pseudopotential energy from `@total_psenergy`, when provided.
    #[serde(
        rename = "@total_psenergy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_psenergy: Option<f64>,
    /// Suggested wavefunction cutoff from `@wfc_cutoff`, when provided.
    #[serde(
        rename = "@wfc_cutoff",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub wfc_cutoff: Option<f64>,
    /// Suggested charge-density cutoff from `@rho_cutoff`, when provided.
    #[serde(
        rename = "@rho_cutoff",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rho_cutoff: Option<f64>,
    /// Maximum angular momentum from `@l_max`.
    #[serde(rename = "@l_max")]
    pub l_max: usize,
    /// Maximum rho angular momentum from `@l_max_rho`, when provided.
    #[serde(
        rename = "@l_max_rho",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub l_max_rho: Option<usize>,
    /// Local channel angular momentum from `@l_local`, when provided.
    #[serde(rename = "@l_local", default, skip_serializing_if = "Option::is_none")]
    pub l_local: Option<isize>,
    /// Declared radial grid length used by several other sections.
    #[serde(rename = "@mesh_size")]
    pub mesh_size: usize,
    /// Number of pseudo-wavefunctions from `@number_of_wfc`.
    #[serde(rename = "@number_of_wfc")]
    pub number_of_wfc: usize,
    /// Number of projectors from `@number_of_proj`.
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
/// The bundled prose reference uses `nonrelativistic`, `scalar`, and `full`.
/// The `upf-schema` XSD uses `no`, `scalar`, and `full`. The crate accepts
/// both non-relativistic spellings on input and serializes the canonical prose
/// spelling `nonrelativistic`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AtomicRelativisticFormalism {
    /// Non-relativistic atomic calculation.
    #[serde(rename = "nonrelativistic", alias = "no")]
    NonRelativistic,
    /// Scalar-relativistic atomic calculation.
    #[serde(rename = "scalar")]
    ScalarRelativistic,
    /// Fully relativistic atomic calculation.
    #[serde(rename = "full")]
    FullyRelativistic,
}
