use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};

use super::internal::Numbered;
use super::common::PpWavefunction;

/// `PP_PSWFC` section containing numbered pseudo-wavefunctions.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PpPseudoWavefunctions {
    /// Numbered `PP_CHI.n` nodes.
    #[serde(rename = "$value", default)]
    pub orbitals: Vec<Numbered<PpWavefunction>>,
}

impl Serialize for PpPseudoWavefunctions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.orbitals.len()))?;
        for orbital in &self.orbitals {
            map.serialize_entry(&orbital.tag.to_string(), &orbital.value)?;
        }
        map.end()
    }
}
