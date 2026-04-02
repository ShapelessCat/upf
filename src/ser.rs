use std::{
    fs::File,
    io::Write,
    path::Path,
};

use crate::{UpfData, UpfError};

pub fn to_string(doc: &UpfData) -> Result<String, UpfError> {
    doc.validate()?;
    quick_xml::se::to_string(doc).map_err(UpfError::from)
}

pub fn to_writer<W: Write>(mut writer: W, doc: &UpfData) -> Result<(), UpfError> {
    let xml = to_string(doc)?;
    writer.write_all(xml.as_bytes())?;
    Ok(())
}

pub fn to_file<P: AsRef<Path>>(path: P, doc: &UpfData) -> Result<(), UpfError> {
    let file = File::create(path)?;
    to_writer(file, doc)
}
