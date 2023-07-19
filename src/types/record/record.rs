use crate::parse::parse_bytes;
use crate::types::{attribute::Attribute, record::GtfRecordRef};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GtfRecord {
    #[serde(serialize_with = "parse_bytes")]
    pub seqname: Vec<u8>,
    #[serde(serialize_with = "parse_bytes")]
    pub source: Vec<u8>,
    #[serde(serialize_with = "parse_bytes")]
    pub feature: Vec<u8>,
    pub start: usize,
    pub end: usize,
    #[serde(serialize_with = "parse_bytes")]
    pub score: Vec<u8>,
    #[serde(serialize_with = "parse_bytes")]
    pub strand: Vec<u8>,
    #[serde(serialize_with = "parse_bytes")]
    pub frame: Vec<u8>,
    #[serde(flatten)]
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
        assert_eq!(record.start, 1471765);
        assert_eq!(record.end, 1497848);
        assert_eq!(record.score, b".");
        assert_eq!(record.strand, b"+");
        assert_eq!(record.frame, b".");

        // Testing Attributes
        assert_eq!(record.attribute.gene_name, Some(b"ATAD3B".to_vec()));
        assert_eq!(record.attribute.transcript_id, None);
    }

    #[test]
    fn test_serialize() {
        let line: &[u8] = br#"1	ensembl_havana	gene	1471765	1497848	.	+	.	gene_id "ENSG00000160072"; gene_version "20"; gene_name "ATAD3B"; gene_source "ensembl_havana"; gene_biotype "protein_coding";"#;
        let record = GtfRecord::from_bytes(line).unwrap();

        let serialized = serde_json::to_string(&record).unwrap();
        assert_eq!(
            serialized,
            "{\"seqname\":\"1\",\"source\":\"ensembl_havana\",\"feature\":\"gene\",\"start\":1471765,\"end\":1497848,\"score\":\".\",\"strand\":\"+\",\"frame\":\".\",\"gene_name\":\"ATAD3B\",\"gene_source\":\"ensembl_havana\",\"gene_version\":20}"
        )
    }
}
