use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{UpfError, model::UpfData};

pub fn from_str(input: &str) -> Result<UpfData, UpfError> {
    let doc: UpfData = quick_xml::de::from_str(input)?;
    doc.validate()?;
    Ok(doc)
}

pub fn from_reader<R: BufRead>(mut reader: R) -> Result<UpfData, UpfError> {
    let mut xml = String::new();
    reader.read_to_string(&mut xml)?;
    from_str(&xml)
}

pub fn from_file<P: AsRef<Path>>(path: P) -> Result<UpfData, UpfError> {
    let reader = BufReader::new(File::open(path)?);
    from_reader(reader)
}
