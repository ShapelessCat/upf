use serde::de::Error as _;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::common::PpWavefunction;
use super::internal::{Numbered, NumberedTag, Tagged};

/// `PP_FULL_WFC` section with numbered all-electron and pseudo-wavefunction data.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PpFullWfc {
    /// Declared orbital count in attribute `number_of_wfc`.
    pub number_of_wfc: Option<usize>,
    /// Numbered `PP_AEWFC.n` entries.
    pub ae_wfc: Vec<Numbered<PpWavefunction>>,
    /// Numbered `PP_AEWFC_REL.n` entries.
    pub ae_wfc_rel: Option<Vec<Numbered<PpWavefunction>>>,
    /// Numbered `PP_PSWFC.n` entries.
    pub ps_wfc: Vec<Numbered<PpWavefunction>>,
}

impl Serialize for PpFullWfc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let field_count = self.ae_wfc.len()
            + self.ps_wfc.len()
            + self.ae_wfc_rel.as_ref().map_or(0, Vec::len)
            + usize::from(self.number_of_wfc.is_some());
        let mut map = serializer.serialize_map(Some(field_count))?;

        if let Some(number_of_wfc) = self.number_of_wfc {
            map.serialize_entry("@number_of_wfc", &number_of_wfc)?;
        }
        for entry in &self.ae_wfc {
            map.serialize_entry(&entry.tag.to_string(), &entry.value)?;
        }
        if let Some(ae_wfc_rel) = &self.ae_wfc_rel {
            for entry in ae_wfc_rel {
                map.serialize_entry(&entry.tag.to_string(), &entry.value)?;
            }
        }
        for entry in &self.ps_wfc {
            map.serialize_entry(&entry.tag.to_string(), &entry.value)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for PpFullWfc {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct FullWfcHelper {
            #[serde(
                rename = "@number_of_wfc",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            number_of_wfc: Option<usize>,
            #[serde(rename = "$value", default)]
            entries: Vec<Tagged<PpWavefunction>>,
        }

        let helper = FullWfcHelper::deserialize(deserializer)?;
        let mut ae_wfc = Vec::new();
        let mut ae_wfc_rel = Vec::new();
        let mut ps_wfc = Vec::new();

        for entry in helper.entries {
            if entry.tag.starts_with("PP_AEWFC_REL.") {
                ae_wfc_rel.push(Numbered {
                    tag: NumberedTag::parse(&entry.tag).map_err(D::Error::custom)?,
                    value: entry.value,
                });
            } else if entry.tag.starts_with("PP_AEWFC.") {
                ae_wfc.push(Numbered {
                    tag: NumberedTag::parse(&entry.tag).map_err(D::Error::custom)?,
                    value: entry.value,
                });
            } else if entry.tag.starts_with("PP_PSWFC.") {
                ps_wfc.push(Numbered {
                    tag: NumberedTag::parse(&entry.tag).map_err(D::Error::custom)?,
                    value: entry.value,
                });
            } else {
                return Err(D::Error::custom(format!(
                    "unexpected PP_FULL_WFC child `{}`",
                    entry.tag
                )));
            }
        }

        Ok(Self {
            number_of_wfc: helper.number_of_wfc,
            ae_wfc,
            ae_wfc_rel: (!ae_wfc_rel.is_empty()).then_some(ae_wfc_rel),
            ps_wfc,
        })
    }
}
