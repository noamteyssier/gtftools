use super::{Attribute, GtfRecordRef};
use anyhow::Result;

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
    pub attribute: Attribute,
}
impl GtfRecord {
    pub fn from_bytes(record: &[u8]) -> Result<Self> {
        let ref_record = GtfRecordRef::from_bytes(record)?;
        Ok(ref_record.to_owned())
    }
}
