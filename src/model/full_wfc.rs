use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};

use super::{Numbered, PpWavefunction};

/// `PP_FULL_WFC` section with numbered all-electron and pseudo-wavefunction data.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PpFullWfc {
    /// Declared orbital count in attribute `number_of_wfc`.
    #[serde(
        rename = "@number_of_wfc",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_wfc: Option<usize>,
    /// Numbered `PP_AEWFC.n` and `PP_PSWFC.n` entries.
    #[serde(rename = "$value", default)]
    pub entries: Vec<Numbered<PpWavefunction>>,
}

impl Serialize for PpFullWfc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map =
            serializer.serialize_map(Some(self.entries.len() + usize::from(self.number_of_wfc.is_some())))?;
        if let Some(number_of_wfc) = self.number_of_wfc {
            map.serialize_entry("@number_of_wfc", &number_of_wfc)?;
        }
        for entry in &self.entries {
            map.serialize_entry(&entry.tag.to_string(), &entry.value)?;
        }
        map.end()
    }
}

impl PpFullWfc {
    pub fn entry_count(&self, prefix: &str) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.tag.has_prefix(prefix))
            .count()
    }
}
