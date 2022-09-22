use anyhow::Result;
use csv::ByteRecord;
use super::{Attribute, GtfRecordRef};

#[derive(Debug, Default)]
pub struct GtfRecord {
    pub seqname: Vec<u8>,
    pub source: Vec<u8>,
    pub feature: Vec<u8>,
    pub start: Vec<u8>,
    pub end: Vec<u8>,
    pub score: Vec<u8>,
    pub strand: Vec<u8>,
    pub frame: Vec<u8>,
    pub attribute: Attribute
}
impl GtfRecord {
    pub fn from_byte_record(record: &ByteRecord) -> Result<Self> {
        let ref_record = GtfRecordRef::from_byte_record(record)?;
        Ok(ref_record.to_owned())
    }
}



