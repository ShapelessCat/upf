use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};

use crate::text::deserialize_bool_flag;

use super::{
    Numbered, NumericSectionTextValueRef, Tagged, numeric_section_vec,
    numeric_text::deserialize_f64_values, optional_numeric_section_vec,
};

/// `PP_NONLOCAL` section, containing projector nodes and optional `PP_DIJ`.
#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct PpNonlocal {
    /// Numbered `PP_BETA.n` projector entries.
    #[serde(rename = "$value", default)]
    pub betas: Vec<Numbered<PpBeta>>,
    /// Coupling matrix in tag `PP_DIJ`.
    #[serde(
        rename = "PP_DIJ",
        default,
        skip_serializing_if = "Vec::is_empty",
        with = "numeric_section_vec"
    )]
    pub dij: Vec<f64>,
    /// Augmentation block in tag `PP_AUGMENTATION`.
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
            map.serialize_entry(&beta.tag.to_string(), &beta.value)?;
        }
        if !self.dij.is_empty() {
            map.serialize_entry("PP_DIJ", &NumericSectionTextValueRef(&self.dij))?;
        }
        if let Some(augmentation) = &self.augmentation {
            map.serialize_entry("PP_AUGMENTATION", augmentation)?;
        }
        map.end()
    }
}

impl PpNonlocal {
    pub fn is_empty(&self) -> bool {
        self.betas.is_empty() && self.dij.is_empty() && self.augmentation.is_none()
    }
}

/// A numbered `PP_BETA.n` projector entry inside `PP_NONLOCAL`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpBeta {
    /// Projector index in attribute `index`.
    #[serde(rename = "@index")]
    pub index: usize,
    /// Projector label in attribute `label`.
    #[serde(rename = "@label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Angular momentum in attribute `angular_momentum`.
    #[serde(rename = "@angular_momentum")]
    pub angular_momentum: usize,
    /// Cutoff radius index in attribute `cutoff_radius_index`.
    #[serde(
        rename = "@cutoff_radius_index",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cutoff_radius_index: Option<usize>,
    /// Cutoff radius in attribute `cutoff_radius`.
    #[serde(rename = "@cutoff_radius")]
    pub cutoff_radius: f64,
    /// Ultrasoft cutoff radius in attribute `ultrasoft_cutoff_radius`.
    #[serde(
        rename = "@ultrasoft_cutoff_radius",
        alias = "@norm_conserving_radius",
        default,
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
    /// Flag in attribute `q_with_l`.
    #[serde(
        rename = "@q_with_l",
        deserialize_with = "deserialize_bool_flag",
        serialize_with = "serialize_bool_flag"
    )]
    pub q_with_l: bool,
    /// Number of q functions in attribute `nqf`.
    #[serde(rename = "@nqf")]
    pub nqf: usize,
    /// Number of composite indices in attribute `nqlc`.
    #[serde(rename = "@nqlc")]
    pub nqlc: usize,
    /// Augmentation shape in attribute `shape`.
    #[serde(rename = "@shape", default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    /// Augmentation cutoff radius in attribute `cutoff_r`.
    #[serde(rename = "@cutoff_r", default, skip_serializing_if = "Option::is_none")]
    pub cutoff_r: Option<f64>,
    /// Cutoff radius index in attribute `cutoff_r_index`.
    #[serde(
        rename = "@cutoff_r_index",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cutoff_r_index: Option<usize>,
    /// Augmentation tolerance in attribute `augmentation_epsilon`.
    #[serde(
        rename = "@augmentation_epsilon",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub augmentation_epsilon: Option<f64>,
    /// Maximum augmentation angular momentum in attribute `l_max_aug`.
    #[serde(
        rename = "@l_max_aug",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub l_max_aug: Option<usize>,
    /// Augmentation radius index in attribute `iraug`.
    #[serde(rename = "@iraug", default, skip_serializing_if = "Option::is_none")]
    pub iraug: Option<usize>,
    /// Augmentation radius in attribute `raug`.
    #[serde(rename = "@raug", default, skip_serializing_if = "Option::is_none")]
    pub raug: Option<f64>,
    /// Integrated q matrix in tag `PP_Q`.
    /// Expected size: `header.number_of_proj * header.number_of_proj`.
    #[serde(
        rename = "PP_Q",
        default,
        skip_serializing_if = "Option::is_none",
        with = "optional_numeric_section_vec"
    )]
    pub q: Option<Vec<f64>>,
    /// PAW multipoles in tag `PP_MULTIPOLES`.
    /// Expected size: `header.number_of_proj * header.number_of_proj * (2 * header.l_max + 1)`.
    #[serde(
        rename = "PP_MULTIPOLES",
        default,
        skip_serializing_if = "Option::is_none",
        with = "optional_numeric_section_vec"
    )]
    pub multipoles: Option<Vec<f64>>,
    /// Taylor-expansion coefficients in tag `PP_QFCOEF`.
    /// Expected size: `augmentation.nqf * effective_nqlc * header.number_of_proj * header.number_of_proj`.
    #[serde(
        rename = "PP_QFCOEF",
        alias = "PP_QFCOEFF",
        default,
        skip_serializing_if = "Option::is_none",
        with = "optional_numeric_section_vec"
    )]
    pub qfcoef: Option<Vec<f64>>,
    /// Inner-radius metadata in tag `PP_RINNER`.
    /// Expected size: `effective_nqlc`.
    #[serde(
        rename = "PP_RINNER",
        default,
        skip_serializing_if = "Option::is_none",
        with = "optional_numeric_section_vec"
    )]
    pub rinner: Option<Vec<f64>>,
    /// Open-ended augmentation channels such as `PP_QIJL.i.j.l`.
    #[serde(rename = "$value", default)]
    pub channels: Vec<Tagged<PpAugmentationChannel>>,
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
        if self.iraug.is_some() {
            field_count += 1;
        }
        if self.raug.is_some() {
            field_count += 1;
        }
        if self.q.is_some() {
            field_count += 1;
        }
        if self.multipoles.is_some() {
            field_count += 1;
        }
        if self.qfcoef.is_some() {
            field_count += 1;
        }
        if self.rinner.is_some() {
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
        if let Some(iraug) = &self.iraug {
            map.serialize_entry("@iraug", iraug)?;
        }
        if let Some(raug) = &self.raug {
            map.serialize_entry("@raug", raug)?;
        }
        if let Some(q) = &self.q {
            map.serialize_entry("PP_Q", &NumericSectionTextValueRef(q))?;
        }
        if let Some(multipoles) = &self.multipoles {
            map.serialize_entry("PP_MULTIPOLES", &NumericSectionTextValueRef(multipoles))?;
        }
        if let Some(qfcoef) = &self.qfcoef {
            map.serialize_entry("PP_QFCOEF", &NumericSectionTextValueRef(qfcoef))?;
        }
        if let Some(rinner) = &self.rinner {
            map.serialize_entry("PP_RINNER", &NumericSectionTextValueRef(rinner))?;
        }
        for channel in &self.channels {
            map.serialize_entry(&channel.tag, &channel.value)?;
        }
        map.end()
    }
}

/// One augmentation radial channel such as `PP_QIJL.1.2.0`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpAugmentationChannel {
    /// First projector index in attribute `first_index`.
    #[serde(rename = "@first_index")]
    pub first_index: usize,
    /// Second projector index in attribute `second_index`.
    #[serde(rename = "@second_index")]
    pub second_index: usize,
    /// Composite index in attribute `composite_index`.
    #[serde(rename = "@composite_index", default, skip_serializing_if = "Option::is_none")]
    pub composite_index: Option<usize>,
    /// Angular momentum in attribute `angular_momentum`.
    #[serde(
        rename = "@angular_momentum",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub angular_momentum: Option<usize>,
    /// Radial samples stored as whitespace-delimited body text.
    #[serde(rename = "$text", deserialize_with = "deserialize_f64_values")]
    pub values: Vec<f64>,
}
