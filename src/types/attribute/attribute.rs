use crate::parse::parse_optional_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Attribute {
    #[serde(serialize_with = "parse_optional_bytes")]
    pub ccds_id: Option<Vec<u8>>,
    #[serde(serialize_with = "parse_optional_bytes")]
    pub exon_id: Option<Vec<u8>>,
    pub exon_number: Option<usize>,
    pub exon_version: Option<usize>,
    #[serde(serialize_with = "parse_optional_bytes")]
    pub gene_biotype: Option<Vec<u8>>,
    #[serde(serialize_with = "parse_optional_bytes")]
    pub gene_id: Option<Vec<u8>>,
    #[serde(serialize_with = "parse_optional_bytes")]
    pub gene_name: Option<Vec<u8>>,
    #[serde(serialize_with = "parse_optional_bytes")]
    pub gene_source: Option<Vec<u8>>,
    pub gene_version: Option<usize>,
    #[serde(serialize_with = "parse_optional_bytes")]
    pub protein_id: Option<Vec<u8>>,
    pub protein_version: Option<usize>,
    #[serde(serialize_with = "parse_optional_bytes")]
    pub tag: Option<Vec<u8>>,
    #[serde(serialize_with = "parse_optional_bytes")]
    pub transcript_biotype: Option<Vec<u8>>,
    #[serde(serialize_with = "parse_optional_bytes")]
    pub transcript_id: Option<Vec<u8>>,
    #[serde(serialize_with = "parse_optional_bytes")]
    pub transcript_name: Option<Vec<u8>>,
    #[serde(serialize_with = "parse_optional_bytes")]
    pub transcript_source: Option<Vec<u8>>,
    #[serde(serialize_with = "parse_optional_bytes")]
    pub transcript_support_level: Option<Vec<u8>>,
    pub transcript_version: Option<usize>,
}
