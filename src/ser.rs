use std::{fs::File, io::Write, path::Path};

use crate::{UpfData, UpfError};

/// Serialize a validated UPF document into an XML string.
///
/// The document is checked with [`UpfData::validate`] before serialization.
pub fn to_string(doc: &UpfData) -> Result<String, UpfError> {
    doc.validate()?;
    quick_xml::se::to_string(doc).map_err(UpfError::from)
}

/// Serialize a validated UPF document into an arbitrary writer.
pub fn to_writer<W: Write>(mut writer: W, doc: &UpfData) -> Result<(), UpfError> {
    let xml = to_string(doc)?;
    writer.write_all(xml.as_bytes())?;
    Ok(())
}

/// Serialize a validated UPF document to a filesystem path.
pub fn to_file<P: AsRef<Path>>(path: P, doc: &UpfData) -> Result<(), UpfError> {
    let file = File::create(path)?;
    to_writer(file, doc)
}
