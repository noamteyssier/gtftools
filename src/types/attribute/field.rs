#[derive(Debug, Copy, Clone)]
pub enum Field {
    CcdsId,
    ExonId,
    ExonNumber,
    ExonVersion,
    GeneBiotype,
    GeneId,
    GeneName,
    GeneSource,
    GeneVersion,
    ProteinId,
    ProteinVersion,
    Tag,
    TranscriptBiotype,
    TranscriptId,
    TranscriptName,
    TranscriptSource,
    TranscriptSupportLevel,
    TranscriptVersion,
}
impl Field {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match bytes {
            b"ccds_id" => Some(Self::CcdsId),
            b"exon_id" => Some(Self::ExonId),
            b"exon_number" => Some(Self::ExonNumber),
            b"exon_version" => Some(Self::ExonVersion),
            b"gene_biotype" => Some(Self::GeneBiotype),
            b"gene_id" => Some(Self::GeneId),
            b"gene_name" => Some(Self::GeneName),
            b"gene_source" => Some(Self::GeneSource),
            b"gene_version" => Some(Self::GeneVersion),
            b"protein_id" => Some(Self::ProteinId),
            b"protein_version" => Some(Self::ProteinVersion),
            b"tag" => Some(Self::Tag),
            b"transcript_biotype" => Some(Self::TranscriptBiotype),
            b"transcript_id" => Some(Self::TranscriptId),
            b"transcript_name" => Some(Self::TranscriptName),
            b"transcript_source" => Some(Self::TranscriptSource),
            b"transcript_support_level" => Some(Self::TranscriptSupportLevel),
            b"transcript_version" => Some(Self::TranscriptVersion),
            _ => None,
        }
    }
}
