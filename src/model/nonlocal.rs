use std::fmt;
use std::marker::PhantomData;

use serde::de::{EnumAccess, VariantAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::internal::{
    Numbered,
    NumericSectionTextValueRef,
    bool_flag,
    deserialize_f64_values,
    deserialize_opt_f64,
    format_bool_flag,
    numeric_section_vec,
    optional_numeric_section_vec,
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
    ///
    /// UPF v2 files in practice may omit this attribute, and QE tolerates that.
    #[serde(rename = "@index", default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
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
    ///
    /// QE tolerates this attribute being absent in UPF v2 files.
    #[serde(
        rename = "@cutoff_radius",
        default,
        deserialize_with = "deserialize_opt_f64",
        skip_serializing_if = "Option::is_none"
    )]
    pub cutoff_radius: Option<f64>,
    /// Ultrasoft cutoff radius in attribute `ultrasoft_cutoff_radius`.
    #[serde(
        rename = "@ultrasoft_cutoff_radius",
        alias = "@norm_conserving_radius",
        default,
        deserialize_with = "deserialize_opt_f64",
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
    #[serde(rename = "@q_with_l", with = "bool_flag")]
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
    #[serde(
        rename = "@cutoff_r",
        default,
        deserialize_with = "deserialize_opt_f64",
        skip_serializing_if = "Option::is_none"
    )]
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
        deserialize_with = "deserialize_opt_f64",
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
    #[serde(
        rename = "@raug",
        default,
        deserialize_with = "deserialize_opt_f64",
        skip_serializing_if = "Option::is_none"
    )]
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
    /// Open-ended augmentation channels such as `PP_QIJ.i.j` or `PP_QIJL.i.j.l`.
    #[serde(rename = "$value", default)]
    pub channels: Vec<AugmentationChannel>,
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
        map.serialize_entry("@q_with_l", format_bool_flag(self.q_with_l))?;
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
            match channel {
                AugmentationChannel::Qij { tag, value } => {
                    map.serialize_entry(&tag.to_string(), value)?;
                }
                AugmentationChannel::Qijl { tag, value } => {
                    map.serialize_entry(
                        &tag.to_string(),
                        &QijlChannelWireRef {
                            composite_index: &value.composite_index,
                            angular_momentum: tag.angular_momentum,
                            values: &value.values,
                        },
                    )?;
                }
            }
        }
        map.end()
    }
}

/// Projector-pair tag for `PP_QIJ.i.j`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QijTag {
    pub first_projector: usize,
    pub second_projector: usize,
}

impl QijTag {
    pub fn parse(input: &str) -> Result<Self, String> {
        let mut parts = input.split('.');
        let prefix = parts
            .next()
            .ok_or_else(|| format!("expected PP_QIJ.i.j, got `{input}`"))?;
        let first = parts
            .next()
            .ok_or_else(|| format!("expected PP_QIJ.i.j, got `{input}`"))?;
        let second = parts
            .next()
            .ok_or_else(|| format!("expected PP_QIJ.i.j, got `{input}`"))?;
        if parts.next().is_some() || prefix != "PP_QIJ" {
            return Err(format!("expected PP_QIJ.i.j, got `{input}`"));
        }
        let first_projector = first
            .parse::<usize>()
            .map_err(|_| format!("expected numeric projector index in `{input}`"))?;
        let second_projector = second
            .parse::<usize>()
            .map_err(|_| format!("expected numeric projector index in `{input}`"))?;
        if first_projector == 0 || second_projector == 0 {
            return Err(format!(
                "expected projector indices greater than zero in `{input}`"
            ));
        }
        Ok(Self {
            first_projector,
            second_projector,
        })
    }
}

impl fmt::Display for QijTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PP_QIJ.{}.{}", self.first_projector, self.second_projector)
    }
}

/// Projector-pair-plus-angular-momentum tag for `PP_QIJL.i.j.l`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QijlTag {
    pub first_projector: usize,
    pub second_projector: usize,
    pub angular_momentum: usize,
}

impl QijlTag {
    pub fn parse(input: &str) -> Result<Self, String> {
        let mut parts = input.split('.');
        let prefix = parts
            .next()
            .ok_or_else(|| format!("expected PP_QIJL.i.j.l, got `{input}`"))?;
        let first = parts
            .next()
            .ok_or_else(|| format!("expected PP_QIJL.i.j.l, got `{input}`"))?;
        let second = parts
            .next()
            .ok_or_else(|| format!("expected PP_QIJL.i.j.l, got `{input}`"))?;
        let angular_momentum = parts
            .next()
            .ok_or_else(|| format!("expected PP_QIJL.i.j.l, got `{input}`"))?;
        if parts.next().is_some() || prefix != "PP_QIJL" {
            return Err(format!("expected PP_QIJL.i.j.l, got `{input}`"));
        }
        let first_projector = first
            .parse::<usize>()
            .map_err(|_| format!("expected numeric projector index in `{input}`"))?;
        let second_projector = second
            .parse::<usize>()
            .map_err(|_| format!("expected numeric projector index in `{input}`"))?;
        let angular_momentum = angular_momentum
            .parse::<usize>()
            .map_err(|_| format!("expected numeric angular momentum in `{input}`"))?;
        if first_projector == 0 || second_projector == 0 {
            return Err(format!(
                "expected projector indices greater than zero in `{input}`"
            ));
        }
        Ok(Self {
            first_projector,
            second_projector,
            angular_momentum,
        })
    }
}

impl fmt::Display for QijlTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PP_QIJL.{}.{}.{}",
            self.first_projector, self.second_projector, self.angular_momentum
        )
    }
}

/// `PP_QIJ.i.j` payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpQijChannel {
    /// Composite index in attribute `composite_index`.
    #[serde(rename = "@composite_index", default, skip_serializing_if = "Option::is_none")]
    pub composite_index: Option<usize>,
    /// Radial samples stored as whitespace-delimited body text.
    #[serde(rename = "$text", deserialize_with = "deserialize_f64_values")]
    pub values: Vec<f64>,
}

/// `PP_QIJL.i.j.l` payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpQijlChannel {
    /// Composite index in attribute `composite_index`.
    #[serde(rename = "@composite_index", default, skip_serializing_if = "Option::is_none")]
    pub composite_index: Option<usize>,
    /// Radial samples stored as whitespace-delimited body text.
    #[serde(rename = "$text", deserialize_with = "deserialize_f64_values")]
    pub values: Vec<f64>,
}

/// One augmentation radial channel such as `PP_QIJ.1.2` or `PP_QIJL.1.2.0`.
#[derive(Debug, Clone, PartialEq)]
pub enum AugmentationChannel {
    Qij { tag: QijTag, value: PpQijChannel },
    Qijl { tag: QijlTag, value: PpQijlChannel },
}

impl AugmentationChannel {
    pub fn tag_name(&self) -> String {
        match self {
            Self::Qij { tag, .. } => tag.to_string(),
            Self::Qijl { tag, .. } => tag.to_string(),
        }
    }

    pub fn values(&self) -> &[f64] {
        match self {
            Self::Qij { value, .. } => &value.values,
            Self::Qijl { value, .. } => &value.values,
        }
    }
}

impl Serialize for AugmentationChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        match self {
            Self::Qij { tag, value } => map.serialize_entry(&tag.to_string(), value)?,
            Self::Qijl { tag, value } => map.serialize_entry(
                &tag.to_string(),
                &QijlChannelWireRef {
                    composite_index: &value.composite_index,
                    angular_momentum: tag.angular_momentum,
                    values: &value.values,
                },
            )?,
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for AugmentationChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AugmentationChannelVisitor(PhantomData<()>);

        impl<'de> Visitor<'de> for AugmentationChannelVisitor {
            type Value = AugmentationChannel;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an augmentation channel tag carrying a section payload")
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: EnumAccess<'de>,
            {
                let (tag_name, variant) = data.variant::<String>()?;
                let raw = variant.newtype_variant::<RawAugmentationChannel>()?;

                if tag_name.starts_with("PP_QIJL.") {
                    let tag = QijlTag::parse(&tag_name).map_err(serde::de::Error::custom)?;
                    let Some(angular_momentum) = raw.angular_momentum else {
                        return Err(serde::de::Error::custom(format!(
                            "{tag_name} is missing @angular_momentum"
                        )));
                    };
                    if angular_momentum != tag.angular_momentum {
                        return Err(serde::de::Error::custom(format!(
                            "{tag_name} has @angular_momentum {angular_momentum} but tag suffix {}",
                            tag.angular_momentum
                        )));
                    }
                    return Ok(AugmentationChannel::Qijl {
                        tag,
                        value: PpQijlChannel {
                            composite_index: raw.composite_index,
                            values: raw.values,
                        },
                    });
                }

                if tag_name.starts_with("PP_QIJ.") {
                    let tag = QijTag::parse(&tag_name).map_err(serde::de::Error::custom)?;
                    if raw.angular_momentum.is_some() {
                        return Err(serde::de::Error::custom(format!(
                            "{tag_name} must not have @angular_momentum"
                        )));
                    }
                    return Ok(AugmentationChannel::Qij {
                        tag,
                        value: PpQijChannel {
                            composite_index: raw.composite_index,
                            values: raw.values,
                        },
                    });
                }

                Err(serde::de::Error::custom(format!(
                    "unexpected augmentation channel tag `{tag_name}`"
                )))
            }
        }

        deserializer.deserialize_enum(
            "AugmentationChannel",
            &[],
            AugmentationChannelVisitor(PhantomData),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct RawAugmentationChannel {
    /// Composite index in attribute `composite_index`.
    #[serde(rename = "@composite_index", default, skip_serializing_if = "Option::is_none")]
    composite_index: Option<usize>,
    /// Angular momentum in attribute `angular_momentum`.
    #[serde(
        rename = "@angular_momentum",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    angular_momentum: Option<usize>,
    /// Radial samples stored as whitespace-delimited body text.
    #[serde(rename = "$text", deserialize_with = "deserialize_f64_values")]
    values: Vec<f64>,
}

#[derive(Serialize)]
struct QijlChannelWireRef<'a> {
    #[serde(rename = "@composite_index", skip_serializing_if = "Option::is_none")]
    composite_index: &'a Option<usize>,
    #[serde(rename = "@angular_momentum")]
    angular_momentum: usize,
    #[serde(rename = "$text", serialize_with = "serialize_f64_values")]
    values: &'a [f64],
}

fn serialize_f64_values<S>(values: &[f64], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&super::internal::format_f64_slice(values))
}

#[cfg(test)]
mod tests {
    use super::{AugmentationChannel, QijTag, QijlTag};

    #[test]
    fn parses_qij_tag() {
        let tag = QijTag::parse("PP_QIJ.1.2").unwrap();
        assert_eq!(
            tag,
            QijTag {
                first_projector: 1,
                second_projector: 2,
            }
        );
        assert_eq!(tag.to_string(), "PP_QIJ.1.2");
    }

    #[test]
    fn parses_qijl_tag() {
        let tag = QijlTag::parse("PP_QIJL.1.2.0").unwrap();
        assert_eq!(
            tag,
            QijlTag {
                first_projector: 1,
                second_projector: 2,
                angular_momentum: 0,
            }
        );
        assert_eq!(tag.to_string(), "PP_QIJL.1.2.0");
    }

    #[test]
    fn rejects_bad_augmentation_tag_shapes() {
        assert!(QijTag::parse("PP_QIJ").is_err());
        assert!(QijTag::parse("PP_QIJ.0.2").is_err());
        assert!(QijlTag::parse("PP_QIJL.1.2").is_err());
        assert!(QijlTag::parse("PP_QIJL.1.2.x").is_err());
    }

    #[test]
    fn augmentation_channel_tag_name_is_canonical() {
        let qij = AugmentationChannel::Qij {
            tag: QijTag {
                first_projector: 1,
                second_projector: 1,
            },
            value: super::PpQijChannel {
                composite_index: Some(1),
                values: vec![0.2],
            },
        };
        let qijl = AugmentationChannel::Qijl {
            tag: QijlTag {
                first_projector: 1,
                second_projector: 1,
                angular_momentum: 0,
            },
            value: super::PpQijlChannel {
                composite_index: Some(1),
                values: vec![0.2],
            },
        };

        assert_eq!(qij.tag_name(), "PP_QIJ.1.1");
        assert_eq!(qijl.tag_name(), "PP_QIJL.1.1.0");
    }
}
