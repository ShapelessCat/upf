use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};

use crate::text::deserialize_bool_flag;

use super::{Numbered, NumericSection, Tagged, UpfDataType, numeric_text::deserialize_f64_values};

/// `PP_NONLOCAL` section, containing projector nodes and optional `PP_DIJ`.
#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct PpNonlocal {
    /// Numbered `PP_BETA.n` projector entries.
    #[serde(rename = "$value", default)]
    pub betas: Vec<Numbered<PpBeta>>,
    /// Coupling matrix from `PP_DIJ`.
    #[serde(rename = "PP_DIJ", default, skip_serializing_if = "PpDij::is_empty")]
    pub dij: PpDij,
    /// Optional augmentation block used by ultrasoft and PAW datasets.
    #[serde(
        rename = "PP_AUGMENTATION",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub augmentation: Option<PpAugmentation>,
}

impl Serialize for PpNonlocal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut field_count = self.betas.len();
        if !self.dij.is_empty() {
            field_count += 1;
        }
        if self.augmentation.is_some() {
            field_count += 1;
        }

        let mut map = serializer.serialize_map(Some(field_count))?;
        for beta in &self.betas {
            map.serialize_entry(&beta.tag.as_str(), &beta.value)?;
        }
        if !self.dij.is_empty() {
            map.serialize_entry("PP_DIJ", &self.dij)?;
        }
        if let Some(augmentation) = &self.augmentation {
            map.serialize_entry("PP_AUGMENTATION", augmentation)?;
        }
        map.end()
    }
}

/// `PP_DIJ` matrix data stored as a flat numeric list.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PpDij {
    #[serde(rename = "$text", deserialize_with = "deserialize_f64_values")]
    pub values: Vec<f64>,
}

impl PpDij {
    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// A numbered `PP_BETA.n` projector entry inside `PP_NONLOCAL`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpBeta {
    /// Projector index from `@index`.
    #[serde(rename = "@index")]
    pub index: usize,
    /// Angular momentum from `@angular_momentum`.
    #[serde(rename = "@angular_momentum")]
    pub angular_momentum: usize,
    /// Cutoff radius from `@cutoff_radius`.
    #[serde(rename = "@cutoff_radius")]
    pub cutoff_radius: f64,
    /// Optional ultrasoft cutoff radius from `@ultrasoft_cutoff_radius`.
    #[serde(
        rename = "@ultrasoft_cutoff_radius",
        skip_serializing_if = "Option::is_none"
    )]
    pub ultrasoft_cutoff_radius: Option<f64>,
    /// Projector samples stored as whitespace-delimited body text.
    #[serde(rename = "$text", deserialize_with = "deserialize_f64_values")]
    pub values: Vec<f64>,
}

/// `PP_AUGMENTATION` block nested inside `PP_NONLOCAL`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PpAugmentation {
    /// `@q_with_l` flag.
    #[serde(
        rename = "@q_with_l",
        deserialize_with = "deserialize_bool_flag",
        serialize_with = "serialize_bool_flag"
    )]
    pub q_with_l: bool,
    /// Number of q functions from `@nqf`.
    #[serde(rename = "@nqf")]
    pub nqf: usize,
    /// Number of composite indices from `@nqlc`.
    #[serde(rename = "@nqlc")]
    pub nqlc: usize,
    /// Optional augmentation shape from `@shape`.
    #[serde(rename = "@shape", default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    /// Optional augmentation cutoff radius from `@cutoff_r`.
    #[serde(rename = "@cutoff_r", default, skip_serializing_if = "Option::is_none")]
    pub cutoff_r: Option<f64>,
    /// Optional cutoff radius index from `@cutoff_r_index`.
    #[serde(
        rename = "@cutoff_r_index",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cutoff_r_index: Option<usize>,
    /// Optional augmentation tolerance from `@augmentation_epsilon`.
    #[serde(
        rename = "@augmentation_epsilon",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub augmentation_epsilon: Option<f64>,
    /// Optional maximum augmentation angular momentum from `@l_max_aug`.
    #[serde(
        rename = "@l_max_aug",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub l_max_aug: Option<usize>,
    /// Integrated q matrix from `PP_Q`.
    #[serde(rename = "PP_Q", default, skip_serializing_if = "Option::is_none")]
    pub q: Option<NumericSection>,
    /// Optional PAW multipoles from `PP_MULTIPOLES`.
    #[serde(
        rename = "PP_MULTIPOLES",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub multipoles: Option<NumericSection>,
    /// Open-ended augmentation channels such as `PP_QIJL.i.j.l`.
    #[serde(rename = "$value", default)]
    pub channels: Vec<Tagged<PpAugmentationQijl>>,
}

impl Serialize for PpAugmentation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut field_count = 3 + self.channels.len();
        if self.shape.is_some() {
            field_count += 1;
        }
        if self.cutoff_r.is_some() {
            field_count += 1;
        }
        if self.cutoff_r_index.is_some() {
            field_count += 1;
        }
        if self.augmentation_epsilon.is_some() {
            field_count += 1;
        }
        if self.l_max_aug.is_some() {
            field_count += 1;
        }
        if self.q.is_some() {
            field_count += 1;
        }
        if self.multipoles.is_some() {
            field_count += 1;
        }

        let mut map = serializer.serialize_map(Some(field_count))?;
        map.serialize_entry("@q_with_l", crate::text::format_bool_flag(self.q_with_l))?;
        map.serialize_entry("@nqf", &self.nqf)?;
        map.serialize_entry("@nqlc", &self.nqlc)?;
        if let Some(shape) = &self.shape {
            map.serialize_entry("@shape", shape)?;
        }
        if let Some(cutoff_r) = &self.cutoff_r {
            map.serialize_entry("@cutoff_r", cutoff_r)?;
        }
        if let Some(cutoff_r_index) = &self.cutoff_r_index {
            map.serialize_entry("@cutoff_r_index", cutoff_r_index)?;
        }
        if let Some(augmentation_epsilon) = &self.augmentation_epsilon {
            map.serialize_entry("@augmentation_epsilon", augmentation_epsilon)?;
        }
        if let Some(l_max_aug) = &self.l_max_aug {
            map.serialize_entry("@l_max_aug", l_max_aug)?;
        }
        if let Some(q) = &self.q {
            map.serialize_entry("PP_Q", q)?;
        }
        if let Some(multipoles) = &self.multipoles {
            map.serialize_entry("PP_MULTIPOLES", multipoles)?;
        }
        for channel in &self.channels {
            map.serialize_entry(&channel.tag, &channel.value)?;
        }
        map.end()
    }
}

/// One augmentation radial channel such as `PP_QIJL.1.2.0`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpAugmentationQijl {
    /// Optional UPF numeric type from `@type`.
    #[serde(rename = "@type", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<UpfDataType>,
    /// Optional declared element count from `@size`.
    #[serde(rename = "@size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<usize>,
    /// Optional display column hint from `@columns`.
    #[serde(rename = "@columns", default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<usize>,
    /// First projector index from `@first_index`.
    #[serde(rename = "@first_index")]
    pub first_index: usize,
    /// Second projector index from `@second_index`.
    #[serde(rename = "@second_index")]
    pub second_index: usize,
    /// Composite index from `@composite_index`.
    #[serde(rename = "@composite_index")]
    pub composite_index: usize,
    /// Angular momentum from `@angular_momentum`.
    #[serde(rename = "@angular_momentum")]
    pub angular_momentum: usize,
    /// Radial samples stored as whitespace-delimited body text.
    #[serde(rename = "$text", deserialize_with = "deserialize_f64_values")]
    pub values: Vec<f64>,
}
