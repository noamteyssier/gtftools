use serde::{Deserialize, Serialize};
use crate::parse::parse_optional_bytes;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Attribute {
    #[serde(serialize_with = "parse_optional_bytes")]
    pub gene_id: Option<Vec<u8>>,
    #[serde(serialize_with = "parse_optional_bytes")]
    pub gene_version: Option<Vec<u8>>,
    #[serde(serialize_with = "parse_optional_bytes")]
    pub gene_name: Option<Vec<u8>>,
    #[serde(serialize_with = "parse_optional_bytes")]
    pub transcript_id: Option<Vec<u8>>,
    #[serde(serialize_with = "parse_optional_bytes")]
    pub transcript_version: Option<Vec<u8>>,
    #[serde(serialize_with = "parse_optional_bytes")]
    pub transcript_name: Option<Vec<u8>>,
    #[serde(serialize_with = "parse_optional_bytes")]
    pub transcript_biotype: Option<Vec<u8>>,
    #[serde(serialize_with = "parse_optional_bytes")]
    pub protein_id: Option<Vec<u8>>,
    #[serde(serialize_with = "parse_optional_bytes")]
    pub exon_number: Option<Vec<u8>>,
}
