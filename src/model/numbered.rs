use std::cmp::Ordering;
use std::fmt;
use std::marker::PhantomData;

use serde::de::{EnumAccess, VariantAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Tag name that ends in a positive numeric suffix, such as `PP_BETA.4`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NumberedTag {
    /// Stable prefix before the numeric suffix, including the trailing dot.
    pub prefix: String,
    /// Positive 1-based numeric suffix.
    pub index: usize,
}

impl NumberedTag {
    /// Parse a numbered XML tag name into prefix and numeric suffix.
    pub fn parse(input: &str) -> Result<Self, String> {
        let (prefix, index) = input
            .rsplit_once('.')
            .ok_or_else(|| format!("expected numbered tag like PREFIX.n, got `{input}`"))?;
        let index = index
            .parse::<usize>()
            .map_err(|_| format!("expected numeric suffix in numbered tag `{input}`"))?;

        if index == 0 {
            return Err(format!(
                "expected numbered tag suffix greater than zero, got `{input}`"
            ));
        }

        Ok(Self {
            prefix: format!("{prefix}."),
            index,
        })
    }

    /// Build a numbered tag from a prefix and index.
    pub fn new(prefix: impl Into<String>, index: usize) -> Self {
        Self {
            prefix: prefix.into(),
            index,
        }
    }

    /// Return the full XML tag name.
    pub fn as_str(&self) -> String {
        format!("{}{}", self.prefix, self.index)
    }

    /// Return whether the tag belongs to the requested family prefix.
    pub fn has_prefix(&self, prefix: &str) -> bool {
        self.prefix == prefix
    }
}

impl Ord for NumberedTag {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.prefix.cmp(&other.prefix) {
            Ordering::Equal => self.index.cmp(&other.index),
            order => order,
        }
    }
}

impl PartialOrd for NumberedTag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for NumberedTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_str())
    }
}

impl Serialize for NumberedTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.as_str())
    }
}

impl<'de> Deserialize<'de> for NumberedTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input = String::deserialize(deserializer)?;
        Self::parse(&input).map_err(serde::de::Error::custom)
    }
}

/// One `$value` child whose XML tag encodes a numbered family suffix.
#[derive(Debug, Clone, PartialEq)]
pub struct Numbered<T> {
    /// Parsed XML tag name.
    pub tag: NumberedTag,
    /// Section payload stored under that tag.
    pub value: T,
}

impl<T> Serialize for Numbered<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.tag.as_str(), &self.value)?;
        map.end()
    }
}

impl<'de, T> Deserialize<'de> for Numbered<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NumberedVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for NumberedVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = Numbered<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a numbered XML tag carrying a section payload")
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: EnumAccess<'de>,
            {
                let (tag_name, variant) = data.variant::<String>()?;
                let tag = NumberedTag::parse(&tag_name).map_err(serde::de::Error::custom)?;
                let value = variant.newtype_variant::<T>()?;

                Ok(Numbered { tag, value })
            }
        }

        deserializer.deserialize_enum("Numbered", &[], NumberedVisitor(PhantomData))
    }
}

/// One `$value` child whose XML tag should be preserved verbatim.
#[derive(Debug, Clone, PartialEq)]
pub struct Tagged<T> {
    /// Raw XML tag name.
    pub tag: String,
    /// Section payload stored under that tag.
    pub value: T,
}

impl<T> Serialize for Tagged<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.tag, &self.value)?;
        map.end()
    }
}

impl<'de, T> Deserialize<'de> for Tagged<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TaggedVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for TaggedVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = Tagged<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an XML tag carrying a section payload")
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: EnumAccess<'de>,
            {
                let (tag, variant) = data.variant::<String>()?;
                let value = variant.newtype_variant::<T>()?;
                Ok(Tagged { tag, value })
            }
        }

        deserializer.deserialize_enum("Tagged", &[], TaggedVisitor(PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::{Numbered, NumberedTag};

    #[test]
    fn parses_numbered_tags() {
        let tag = NumberedTag::parse("PP_BETA.4").unwrap();
        assert_eq!(tag.prefix, "PP_BETA.");
        assert_eq!(tag.index, 4);
        assert_eq!(tag.as_str(), "PP_BETA.4");
    }

    #[test]
    fn rejects_missing_or_invalid_suffixes() {
        assert!(NumberedTag::parse("PP_BETA").is_err());
        assert!(NumberedTag::parse("PP_BETA.zero").is_err());
        assert!(NumberedTag::parse("PP_BETA.0").is_err());
    }

    #[test]
    fn deserializes_numbered_values_from_xml_tags() {
        #[derive(Debug, Clone, PartialEq, Deserialize)]
        struct Container {
            #[serde(rename = "$value", default)]
            entries: Vec<Numbered<Payload>>,
        }

        #[derive(Debug, Clone, PartialEq, Deserialize)]
        struct Payload {
            #[serde(rename = "@index")]
            index: usize,
            #[serde(rename = "$text")]
            text: String,
        }

        let xml = "<Container><PP_BETA.4 index=\"4\">hello</PP_BETA.4></Container>";
        let parsed: Container = quick_xml::de::from_str(xml).unwrap();

        assert_eq!(parsed.entries.len(), 1);
        assert_eq!(parsed.entries[0].tag.as_str(), "PP_BETA.4");
        assert_eq!(parsed.entries[0].value.index, 4);
    }
}
