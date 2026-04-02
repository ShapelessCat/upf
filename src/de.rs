use std::io::BufRead;

use crate::{UpfError, model::UpfData};

pub fn from_str(input: &str) -> Result<UpfData, UpfError> {
    quick_xml::de::from_str(input).map_err(UpfError::from)
}

pub fn from_reader<R: BufRead>(mut reader: R) -> Result<UpfData, UpfError> {
    let mut xml = String::new();
    reader.read_to_string(&mut xml)?;
    from_str(&xml)
}
