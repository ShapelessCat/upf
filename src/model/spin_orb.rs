use serde::de::Error as _;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{Numbered, NumberedTag, Tagged};

/// `PP_SPIN_ORB` section containing spin-orbit metadata for wavefunctions and projectors.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PpSpinOrb {
    /// Numbered `PP_RELWFC.n` entries.
    pub relwfcs: Vec<Numbered<PpRelWfc>>,
    /// Numbered `PP_RELBETA.n` entries.
    pub relbetas: Vec<Numbered<PpRelBeta>>,
}

impl Serialize for PpSpinOrb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.relwfcs.len() + self.relbetas.len()))?;
        for relwfc in &self.relwfcs {
            map.serialize_entry(&relwfc.tag.to_string(), &relwfc.value)?;
        }
        for relbeta in &self.relbetas {
            map.serialize_entry(&relbeta.tag.to_string(), &relbeta.value)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for PpSpinOrb {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct SpinOrbHelper {
            #[serde(rename = "$value", default)]
            entries: Vec<Tagged<RawSpinOrbEntry>>,
        }

        let helper = SpinOrbHelper::deserialize(deserializer)?;
        let mut relwfcs = Vec::new();
        let mut relbetas = Vec::new();

        for entry in helper.entries {
            if entry.tag.starts_with("PP_RELWFC.") {
                relwfcs.push(Numbered {
                    tag: NumberedTag::parse(&entry.tag).map_err(D::Error::custom)?,
                    value: PpRelWfc {
                        index: entry.value.index,
                        els: entry.value.els,
                        nn: entry.value.nn,
                        lchi: entry.value.lchi,
                        jchi: entry.value.jchi.ok_or_else(|| {
                            D::Error::custom(format!("{} is missing @jchi", entry.tag))
                        })?,
                        oc: entry.value.oc,
                    },
                });
            } else if entry.tag.starts_with("PP_RELBETA.") {
                relbetas.push(Numbered {
                    tag: NumberedTag::parse(&entry.tag).map_err(D::Error::custom)?,
                    value: PpRelBeta {
                        index: entry.value.index,
                        lll: entry.value.lll,
                        jjj: entry.value.jjj.ok_or_else(|| {
                            D::Error::custom(format!("{} is missing @jjj", entry.tag))
                        })?,
                    },
                });
            } else {
                return Err(D::Error::custom(format!(
                    "unexpected PP_SPIN_ORB child `{}`",
                    entry.tag
                )));
            }
        }

        Ok(Self { relwfcs, relbetas })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RawSpinOrbEntry {
    #[serde(rename = "@index", default, skip_serializing_if = "Option::is_none")]
    index: Option<usize>,
    #[serde(rename = "@els", default, skip_serializing_if = "Option::is_none")]
    els: Option<String>,
    #[serde(rename = "@nn", default, skip_serializing_if = "Option::is_none")]
    nn: Option<usize>,
    #[serde(rename = "@lchi", default, skip_serializing_if = "Option::is_none")]
    lchi: Option<usize>,
    #[serde(rename = "@jchi", default, skip_serializing_if = "Option::is_none")]
    jchi: Option<f64>,
    #[serde(rename = "@oc", default, skip_serializing_if = "Option::is_none")]
    oc: Option<f64>,
    #[serde(rename = "@lll", default, skip_serializing_if = "Option::is_none")]
    lll: Option<usize>,
    #[serde(rename = "@jjj", default, skip_serializing_if = "Option::is_none")]
    jjj: Option<f64>,
}

/// One `PP_RELWFC.n` metadata entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpRelWfc {
    /// Index in attribute `index`.
    #[serde(rename = "@index", default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
    /// Conventional label such as `2P`.
    #[serde(rename = "@els", default, skip_serializing_if = "Option::is_none")]
    pub els: Option<String>,
    /// Principal quantum number.
    #[serde(rename = "@nn", default, skip_serializing_if = "Option::is_none")]
    pub nn: Option<usize>,
    /// Angular momentum.
    #[serde(rename = "@lchi", default, skip_serializing_if = "Option::is_none")]
    pub lchi: Option<usize>,
    /// Total angular momentum.
    #[serde(rename = "@jchi")]
    pub jchi: f64,
    /// Occupation.
    #[serde(rename = "@oc", default, skip_serializing_if = "Option::is_none")]
    pub oc: Option<f64>,
}

/// One `PP_RELBETA.n` metadata entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpRelBeta {
    /// Index in attribute `index`.
    #[serde(rename = "@index", default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
    /// Angular momentum.
    #[serde(rename = "@lll", default, skip_serializing_if = "Option::is_none")]
    pub lll: Option<usize>,
    /// Total angular momentum.
    #[serde(rename = "@jjj")]
    pub jjj: f64,
}
