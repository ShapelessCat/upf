use serde::{Deserialize, Serialize, Serializer};

use super::numeric_values::{format_f64_slice, parse_f64_vec};

#[derive(Debug, Deserialize)]
struct NumericSectionText {
    #[serde(rename = "@type", default)]
    _data_type: Option<String>,
    #[serde(rename = "@size", default)]
    _size: Option<usize>,
    #[serde(rename = "@columns", default)]
    _columns: Option<usize>,
    #[serde(rename = "$text", deserialize_with = "deserialize_numeric_section_text")]
    values: Vec<f64>,
}

#[derive(Serialize)]
struct NumericSectionTextRef<'a> {
    #[serde(rename = "@size")]
    size: usize,
    #[serde(rename = "$text", serialize_with = "serialize_numeric_section_text")]
    values: &'a [f64],
}

pub(crate) struct NumericSectionTextValueRef<'a>(pub &'a [f64]);

impl Serialize for NumericSectionTextValueRef<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        NumericSectionTextRef {
            size: self.0.len(),
            values: self.0,
        }
        .serialize(serializer)
    }
}

pub(crate) mod numeric_section_vec {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::{NumericSectionText, NumericSectionTextRef};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        NumericSectionText::deserialize(deserializer).map(|section| section.values)
    }

    pub fn serialize<S>(values: &[f64], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        NumericSectionTextRef {
            size: values.len(),
            values,
        }
        .serialize(serializer)
    }
}

pub(crate) mod optional_numeric_section_vec {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::{NumericSectionText, NumericSectionTextRef};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<f64>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<NumericSectionText>::deserialize(deserializer)
            .map(|section| section.map(|section| section.values))
    }

    pub fn serialize<S>(values: &Option<Vec<f64>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match values {
            Some(values) => NumericSectionTextRef {
                size: values.len(),
                values,
            }
            .serialize(serializer),
            None => serializer.serialize_none(),
        }
    }
}

fn deserialize_numeric_section_text<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let text = String::deserialize(deserializer)?;
    parse_f64_vec(&text).map_err(serde::de::Error::custom)
}

fn serialize_numeric_section_text<S>(values: &[f64], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format_f64_slice(values))
}
