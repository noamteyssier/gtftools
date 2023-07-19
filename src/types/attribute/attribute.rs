use crate::parse::parse_optional_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Attribute {
    #[serde(
        serialize_with = "parse_optional_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub ccds_id: Option<Vec<u8>>,
    #[serde(
        serialize_with = "parse_optional_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub exon_id: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exon_number: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exon_version: Option<usize>,
    #[serde(
        serialize_with = "parse_optional_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub gene_biotype: Option<Vec<u8>>,
    #[serde(
        serialize_with = "parse_optional_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub gene_id: Option<Vec<u8>>,
    #[serde(
        serialize_with = "parse_optional_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub gene_name: Option<Vec<u8>>,
    #[serde(
        serialize_with = "parse_optional_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub gene_source: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gene_version: Option<usize>,
    #[serde(
        serialize_with = "parse_optional_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub protein_id: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protein_version: Option<usize>,
    #[serde(
        serialize_with = "parse_optional_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub tag: Option<Vec<u8>>,
    #[serde(
        serialize_with = "parse_optional_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub transcript_biotype: Option<Vec<u8>>,
    #[serde(
        serialize_with = "parse_optional_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub transcript_id: Option<Vec<u8>>,
    #[serde(
        serialize_with = "parse_optional_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub transcript_name: Option<Vec<u8>>,
    #[serde(
        serialize_with = "parse_optional_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub transcript_source: Option<Vec<u8>>,
    #[serde(
        serialize_with = "parse_optional_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub transcript_support_level: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_version: Option<usize>,
}
