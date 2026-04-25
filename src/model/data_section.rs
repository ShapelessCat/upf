use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::text::{format_f64_slice, parse_f64_vec};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpfDataType {
    #[serde(rename = "character")]
    Character,
    #[serde(rename = "logical")]
    Logical,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "real")]
    Real,
    #[serde(rename = "complex")]
    Complex,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct NumericSection {
    #[serde(rename = "@type", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<UpfDataType>,
    #[serde(rename = "@size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<usize>,
    #[serde(rename = "@columns", default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<usize>,
    #[serde(
        rename = "$text",
        deserialize_with = "deserialize_numeric_section_values",
        serialize_with = "serialize_numeric_section_values"
    )]
    pub values: Vec<f64>,
}

impl NumericSection {
    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

fn deserialize_numeric_section_values<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let text = String::deserialize(deserializer)?;
    parse_f64_vec(&text).map_err(serde::de::Error::custom)
}

fn serialize_numeric_section_values<S>(values: &[f64], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format_f64_slice(values))
}
