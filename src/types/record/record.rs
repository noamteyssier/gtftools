use crate::types::{
    attribute::Attribute,
    record::GtfRecordRef
};
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

#[cfg(test)]
mod testing {
    use super::GtfRecord;
    
    #[test]
    fn test_gtf() {
        let line: &[u8] = br#"1	ensembl_havana	gene	1471765	1497848	.	+	.	gene_id "ENSG00000160072"; gene_version "20"; gene_name "ATAD3B"; gene_source "ensembl_havana"; gene_biotype "protein_coding";"#;
        let record = GtfRecord::from_bytes(line).unwrap();

        // Testing Record
        assert_eq!(record.seqname, b"1");
        assert_eq!(record.source, b"ensembl_havana");
        assert_eq!(record.feature, b"gene");
        assert_eq!(record.start, b"1471765");
        assert_eq!(record.end, b"1497848");
        assert_eq!(record.score, b".");
        assert_eq!(record.strand, b"+");
        assert_eq!(record.frame, b".");

        // Testing Attributes
        assert_eq!(record.attribute.gene_name, Some(b"ATAD3B".to_vec()));
        assert_eq!(record.attribute.transcript_id, None);
    }
}
