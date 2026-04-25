use serde::{Deserialize, Serialize};

use crate::model::internal::deserialize_f64_values;

/// A wavefunction-like numeric section used by several UPF containers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpWavefunction {
    /// Orbital index in attribute `index`.
    #[serde(rename = "@index", default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
    /// Orbital label in attribute `label`.
    #[serde(rename = "@label")]
    pub label: String,
    /// Angular momentum in attribute `l`.
    #[serde(rename = "@l")]
    pub l: usize,
    /// Occupation in attribute `occupation`.
    #[serde(rename = "@occupation", skip_serializing_if = "Option::is_none")]
    pub occupation: Option<f64>,
    /// Principal quantum number in attribute `nn` for spin-orbit wavefunctions.
    #[serde(rename = "@nn", default, skip_serializing_if = "Option::is_none")]
    pub nn: Option<usize>,
    /// Total angular momentum in attribute `jchi` for spin-orbit wavefunctions.
    #[serde(rename = "@jchi", default, skip_serializing_if = "Option::is_none")]
    pub jchi: Option<f64>,
    /// Pseudo-principal quantum number in attribute `n`.
    #[serde(rename = "@n", default, skip_serializing_if = "Option::is_none")]
    pub n: Option<usize>,
    /// Pseudo-energy in attribute `pseudo_energy`.
    #[serde(rename = "@pseudo_energy", default, skip_serializing_if = "Option::is_none")]
    pub pseudo_energy: Option<f64>,
    /// Cutoff radius in attribute `cutoff_radius`.
    #[serde(rename = "@cutoff_radius", default, skip_serializing_if = "Option::is_none")]
    pub cutoff_radius: Option<f64>,
    /// Ultrasoft cutoff radius in attribute `ultrasoft_cutoff_radius`.
    #[serde(rename = "@ultrasoft_cutoff_radius", default, skip_serializing_if = "Option::is_none")]
    pub ultrasoft_cutoff_radius: Option<f64>,
    /// Orbital samples stored as whitespace-delimited body text.
    #[serde(rename = "$text", deserialize_with = "deserialize_f64_values")]
    pub values: Vec<f64>,
}
