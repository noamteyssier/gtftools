use crate::{
    parse::parse_attributes,
    types::{attribute::AttributeRef, record::GtfRecord},
};
use anyhow::{Result, anyhow};
use bstr::ByteSlice;

#[derive(Debug, Default)]
pub struct GtfRecordRef<'a> {
    pub seqname: &'a [u8],
    pub source: &'a [u8],
    pub feature: &'a [u8],
    pub start: usize,
    pub end: usize,
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
        let start = self.start;
        let end = self.end;
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
        let seqname = it.next().ok_or(anyhow!("Missing seqname"))?;
        let source = it.next().ok_or(anyhow!("Missing source"))?;
        let feature = it.next().ok_or(anyhow!("Missing feature"))?;
        let start = parse_to_usize(it.next().ok_or(anyhow!("Missing start"))?)?;
        let end = parse_to_usize(it.next().ok_or(anyhow!("Missing start"))?)?;
        let score = it.next().ok_or(anyhow!("Missing score"))?;
        let strand = it.next().ok_or(anyhow!("Missing strand"))?;
        let frame = it.next().ok_or(anyhow!("Missing frame"))?;
        let attribute_bytes = it.next().ok_or(anyhow!("Missing attributes"))?;
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

fn parse_to_usize(value: &[u8]) -> Result<usize> {
    let usize_value = std::str::from_utf8(value)?.parse()?;
    Ok(usize_value)
}

#[cfg(test)]
mod testing {
    use super::GtfRecordRef;

    #[test]
    fn test_gtf_ref() {
        let line: &[u8] = br#"1	ensembl_havana	gene	1471765	1497848	.	+	.	gene_id "ENSG00000160072"; gene_version "20"; gene_name "ATAD3B"; gene_source "ensembl_havana"; gene_biotype "protein_coding";"#;
        let record = GtfRecordRef::from_bytes(line).unwrap();

        // Testing Record
        assert_eq!(record.seqname, b"1");
        assert_eq!(record.source, b"ensembl_havana");
        assert_eq!(record.feature, b"gene");
        assert_eq!(record.start, 1471765);
        assert_eq!(record.end, 1497848);
        assert_eq!(record.score, b".");
        assert_eq!(record.strand, b"+");
        assert_eq!(record.frame, b".");

        // Testing Attributes
        assert_eq!(record.attribute.gene_name, Some("ATAD3B".as_bytes()));
        assert_eq!(record.attribute.transcript_id, None);
    }
}
