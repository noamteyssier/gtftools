use super::{AttributeRef, GtfRecord};
use crate::parse::parse_attributes;
use anyhow::Result;
use bstr::ByteSlice;

#[derive(Debug, Default)]
pub struct GtfRecordRef<'a> {
    pub seqname: &'a [u8],
    pub source: &'a [u8],
    pub feature: &'a [u8],
    pub start: &'a [u8],
    pub end: &'a [u8],
    pub score: &'a [u8],
    pub strand: &'a [u8],
    pub frame: &'a [u8],
    pub attribute: AttributeRef<'a>,
}
impl<'a> GtfRecordRef<'a> {
    pub fn to_owned(&self) -> GtfRecord {
        let seqname = self.seqname.to_owned();
        let source = self.source.to_owned();
        let feature = self.feature.to_owned();
        let start = self.start.to_owned();
        let end = self.end.to_owned();
        let score = self.score.to_owned();
        let strand = self.strand.to_owned();
        let frame = self.frame.to_owned();
        let attribute = self.attribute.to_owned();
        GtfRecord {
            seqname,
            source,
            feature,
            start,
            end,
            score,
            strand,
            frame,
            attribute,
        }
    }

    pub fn from_bytes(record: &'a [u8]) -> Result<Self> {
        let mut it = record.split_str("\t");
        let seqname = it.next().unwrap_or_default();
        let source = it.next().unwrap_or_default();
        let feature = it.next().unwrap_or_default();
        let start = it.next().unwrap_or_default();
        let end = it.next().unwrap_or_default();
        let score = it.next().unwrap_or_default();
        let strand = it.next().unwrap_or_default();
        let frame = it.next().unwrap_or_default();
        let attribute_bytes = it.next().unwrap_or_default();
        let (_, attribute) = parse_attributes(attribute_bytes).expect(" ");
        let gtf_record = Self {
            seqname,
            source,
            feature,
            start,
            end,
            score,
            strand,
            frame,
            attribute,
        };
        Ok(gtf_record)
    }
}
