#[derive(Debug, Default, Clone)]
pub struct Attribute {
    pub gene_id: Option<Vec<u8>>,
    pub gene_version: Option<Vec<u8>>,
    pub gene_name: Option<Vec<u8>>,
    pub transcript_id: Option<Vec<u8>>,
    pub transcript_version: Option<Vec<u8>>,
    pub transcript_name: Option<Vec<u8>>,
    pub transcript_biotype: Option<Vec<u8>>,
    pub protein_id: Option<Vec<u8>>,
    pub exon_number: Option<Vec<u8>>,
}
