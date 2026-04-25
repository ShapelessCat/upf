use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};

use super::{Numbered, NumericSection, UpfDataType, numeric_text::deserialize_f64_values};

/// `PP_GIPAW` section for datasets that include GIPAW reconstruction data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpGipaw {
    /// GIPAW format version string in attribute `gipaw_data_format`.
    #[serde(rename = "@gipaw_data_format")]
    pub data_format: String,
    /// Block in tag `PP_GIPAW_CORE_ORBITALS`.
    #[serde(
        rename = "PP_GIPAW_CORE_ORBITALS",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub core_orbitals: Option<GipawCoreOrbitals>,
    /// Explicit GIPAW valence orbitals in tag `PP_GIPAW_ORBITALS`.
    #[serde(
        rename = "PP_GIPAW_ORBITALS",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orbitals: Option<GipawValenceOrbitals>,
    /// Explicit GIPAW local potentials in tag `PP_GIPAW_VLOCAL`.
    #[serde(
        rename = "PP_GIPAW_VLOCAL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vlocal: Option<GipawVlocal>,
}

/// `PP_GIPAW_CORE_ORBITALS` container.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GipawCoreOrbitals {
    /// Declared number of core orbitals in attribute `number_of_core_orbitals`.
    #[serde(rename = "@number_of_core_orbitals")]
    pub number_of_core_orbitals: usize,
    /// Numbered `PP_GIPAW_CORE_ORBITAL.n` entries.
    #[serde(rename = "$value", default)]
    pub orbitals: Vec<Numbered<GipawOrbital>>,
}

impl Serialize for GipawCoreOrbitals {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1 + self.orbitals.len()))?;
        map.serialize_entry("@number_of_core_orbitals", &self.number_of_core_orbitals)?;
        for orbital in &self.orbitals {
            map.serialize_entry(&orbital.tag.as_str(), &orbital.value)?;
        }
        map.end()
    }
}

/// `PP_GIPAW_ORBITALS` container.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GipawValenceOrbitals {
    /// Declared number of valence orbitals.
    #[serde(rename = "@number_of_valence_orbitals")]
    pub number_of_valence_orbitals: usize,
    /// Numbered `PP_GIPAW_ORBITAL.n` entries.
    #[serde(rename = "$value", default)]
    pub orbitals: Vec<Numbered<GipawValenceOrbital>>,
}

impl Serialize for GipawValenceOrbitals {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1 + self.orbitals.len()))?;
        map.serialize_entry(
            "@number_of_valence_orbitals",
            &self.number_of_valence_orbitals,
        )?;
        for orbital in &self.orbitals {
            map.serialize_entry(&orbital.tag.as_str(), &orbital.value)?;
        }
        map.end()
    }
}

/// One `PP_GIPAW_CORE_ORBITAL.n` entry inside a GIPAW block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GipawOrbital {
    /// UPF numeric type in attribute `type`.
    #[serde(rename = "@type", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<UpfDataType>,
    /// Declared element count in attribute `size`.
    #[serde(rename = "@size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<usize>,
    /// Display column hint in attribute `columns`.
    #[serde(rename = "@columns", default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<usize>,
    /// Orbital index in attribute `index`.
    #[serde(rename = "@index")]
    pub index: usize,
    /// Orbital label in attribute `label`.
    #[serde(rename = "@label")]
    pub label: String,
    /// Principal quantum number in attribute `n`.
    #[serde(rename = "@n")]
    pub n: f64,
    /// Angular momentum in attribute `l`.
    #[serde(rename = "@l")]
    pub l: f64,
    /// Orbital samples stored as body text.
    #[serde(rename = "$text", deserialize_with = "deserialize_f64_values")]
    pub values: Vec<f64>,
}

/// One `PP_GIPAW_ORBITAL.n` entry with explicit AE/PS wavefunctions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GipawValenceOrbital {
    /// Orbital index in attribute `index`.
    #[serde(rename = "@index", default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
    /// Orbital label in attribute `label`.
    #[serde(rename = "@label")]
    pub label: String,
    /// Angular momentum in attribute `l`.
    #[serde(rename = "@l")]
    pub l: usize,
    /// Cutoff radius in attribute `cutoff_radius`.
    #[serde(rename = "@cutoff_radius")]
    pub cutoff_radius: f64,
    /// Ultrasoft cutoff radius in attribute `ultrasoft_cutoff_radius`.
    #[serde(
        rename = "@ultrasoft_cutoff_radius",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ultrasoft_cutoff_radius: Option<f64>,
    /// All-electron GIPAW wavefunction.
    #[serde(rename = "PP_GIPAW_WFS_AE")]
    pub ae: NumericSection,
    /// Pseudo GIPAW wavefunction.
    #[serde(rename = "PP_GIPAW_WFS_PS")]
    pub ps: NumericSection,
}

/// `PP_GIPAW_VLOCAL` container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GipawVlocal {
    /// All-electron local potential.
    #[serde(rename = "PP_GIPAW_VLOCAL_AE")]
    pub ae: NumericSection,
    /// Pseudo local potential.
    #[serde(rename = "PP_GIPAW_VLOCAL_PS")]
    pub ps: NumericSection,
}
