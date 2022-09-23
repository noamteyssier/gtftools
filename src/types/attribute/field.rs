#[derive(Debug, Copy, Clone)]
pub enum Field {
    GeneId,
    GeneVersion,
    GeneName,
    TranscriptId,
    TranscriptVersion,
    TranscriptName,
    TranscriptBiotype,
    ProteinId,
    ExonNumber,
}
impl Field {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match bytes {
            b"gene_id" => Some(Self::GeneId),
            b"gene_version" => Some(Self::GeneVersion),
            b"gene_name" => Some(Self::GeneName),
            b"transcript_id" => Some(Self::TranscriptId),
            b"transcript_name" => Some(Self::TranscriptName),
            b"transcript_version" => Some(Self::TranscriptVersion),
            b"transcript_biotype" => Some(Self::TranscriptBiotype),
            b"protein_id" => Some(Self::ProteinId),
            b"exon_number" => Some(Self::ExonNumber),
            _ => None,
        }
    }
}
