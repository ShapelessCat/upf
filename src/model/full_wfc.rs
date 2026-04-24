use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};

use super::{Numbered, PpWavefunction};

/// `PP_FULL_WFC` section with numbered all-electron and pseudo-wavefunction data.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PpFullWfc {
    /// Numbered `PP_AEWFC.n` and `PP_PSWFC.n` entries.
    #[serde(rename = "$value", default)]
    pub entries: Vec<Numbered<PpWavefunction>>,
}

impl Serialize for PpFullWfc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.entries.len()))?;
        for entry in &self.entries {
            map.serialize_entry(&entry.tag.as_str(), &entry.value)?;
        }
        map.end()
    }
}
