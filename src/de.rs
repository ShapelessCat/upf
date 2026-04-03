use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{UpfError, model::UpfData};

/// Parse a full UPF document from a string slice.
///
/// This accepts XML-like UPF text and returns a validated [`UpfData`] model.
/// Parsing succeeds only if XML deserialization works and the document passes
/// [`UpfData::validate`].
pub fn from_str(input: &str) -> Result<UpfData, UpfError> {
    let doc: UpfData = quick_xml::de::from_str(input)?;
    doc.validate()?;
    Ok(doc)
}

/// Parse a full UPF document from any buffered reader.
///
/// The entire input is read into memory before deserialization.
pub fn from_reader<R: BufRead>(mut reader: R) -> Result<UpfData, UpfError> {
    let mut xml = String::new();
    reader.read_to_string(&mut xml)?;
    from_str(&xml)
}

/// Parse a UPF document from a filesystem path.
pub fn from_file<P: AsRef<Path>>(path: P) -> Result<UpfData, UpfError> {
    let reader = BufReader::new(File::open(path)?);
    from_reader(reader)
}
