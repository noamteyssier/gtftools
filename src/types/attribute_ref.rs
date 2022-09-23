use super::{Attribute, Field};

#[derive(Debug, Default)]
pub struct AttributeRef<'a> {
    pub gene_id: Option<&'a [u8]>,
    pub gene_version: Option<&'a [u8]>,
    pub gene_name: Option<&'a [u8]>,
    pub transcript_id: Option<&'a [u8]>,
    pub transcript_version: Option<&'a [u8]>,
    pub transcript_name: Option<&'a [u8]>,
    pub transcript_biotype: Option<&'a [u8]>,
    pub protein_id: Option<&'a [u8]>,
    pub exon_number: Option<&'a [u8]>,
}
impl<'a> AttributeRef<'a> {
    pub fn to_owned(&self) -> Attribute {
        let gene_id = self.gene_id.map(|x| x.to_owned());
        let gene_version = self.gene_version.map(|x| x.to_owned());
        let gene_name = self.gene_name.map(|x| x.to_owned());
        let transcript_id = self.transcript_id.map(|x| x.to_owned());
        let transcript_version = self.transcript_version.map(|x| x.to_owned());
        let transcript_name = self.transcript_name.map(|x| x.to_owned());
        let transcript_biotype = self.transcript_biotype.map(|x| x.to_owned());
        let protein_id = self.protein_id.map(|x| x.to_owned());
        let exon_number = self.exon_number.map(|x| x.to_owned());
        Attribute {
            gene_id,
            gene_version,
            gene_name,
            transcript_id,
            transcript_version,
            transcript_name,
            transcript_biotype,
            protein_id,
            exon_number,
        }
    }
    pub fn update_field(&mut self, field: Field, value: &'a [u8]) {
        match field {
            Field::GeneId => {
                self.gene_id = Some(value);
            }
            Field::GeneVersion => {
                self.gene_version = Some(value);
            }
            Field::GeneName => {
                self.gene_name = Some(value);
            }
            Field::TranscriptId => {
                self.transcript_id = Some(value);
            }
            Field::TranscriptVersion => {
                self.transcript_version = Some(value);
            }
            Field::TranscriptName => {
                self.transcript_name = Some(value);
            }
            Field::TranscriptBiotype => {
                self.transcript_biotype = Some(value);
            }
            Field::ProteinId => {
                self.protein_id = Some(value);
            }
            Field::ExonNumber => {
                self.exon_number = Some(value);
            }
        }
    }
}
