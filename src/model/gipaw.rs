use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};

use super::{Numbered, UpfDataType, numeric_text::deserialize_f64_values};

/// `PP_GIPAW` section for datasets that include GIPAW reconstruction data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpGipaw {
    /// GIPAW format version string from `@gipaw_data_format`.
    #[serde(rename = "@gipaw_data_format")]
    pub data_format: String,
    /// Optional `PP_GIPAW_CORE_ORBITALS` block.
    #[serde(
        rename = "PP_GIPAW_CORE_ORBITALS",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub core_orbitals: Option<GipawCoreOrbitals>,
}

/// `PP_GIPAW_CORE_ORBITALS` container.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GipawCoreOrbitals {
    /// Declared number of core orbitals from `@number_of_core_orbitals`.
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

/// One `PP_GIPAW_CORE_ORBITAL.n` entry inside a GIPAW block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GipawOrbital {
    /// Optional UPF numeric type from `@type`.
    #[serde(rename = "@type", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<UpfDataType>,
    /// Optional declared element count from `@size`.
    #[serde(rename = "@size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<usize>,
    /// Optional display column hint from `@columns`.
    #[serde(rename = "@columns", default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<usize>,
    /// Orbital index from `@index`.
    #[serde(rename = "@index")]
    pub index: usize,
    /// Orbital label from `@label`.
    #[serde(rename = "@label")]
    pub label: String,
    /// Principal quantum number from `@n`.
    #[serde(rename = "@n")]
    pub n: f64,
    /// Angular momentum from `@l`.
    #[serde(rename = "@l")]
    pub l: f64,
    /// Orbital samples stored as body text.
    #[serde(rename = "$text", deserialize_with = "deserialize_f64_values")]
    pub values: Vec<f64>,
}
