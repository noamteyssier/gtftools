use crate::parse::parse_to_usize;

use super::{Attribute, Field};

#[derive(Debug, Default, Clone)]
pub struct AttributeRef<'a> {
    pub ccds_id: Option<&'a [u8]>,
    pub exon_id: Option<&'a [u8]>,
    pub exon_number: Option<usize>,
    pub exon_version: Option<usize>,
    pub gene_biotype: Option<&'a [u8]>,
    pub gene_id: Option<&'a [u8]>,
    pub gene_name: Option<&'a [u8]>,
    pub gene_source: Option<&'a [u8]>,
    pub gene_version: Option<usize>,
    pub protein_id: Option<&'a [u8]>,
    pub protein_version: Option<usize>,
    pub tag: Option<&'a [u8]>,
    pub transcript_biotype: Option<&'a [u8]>,
    pub transcript_id: Option<&'a [u8]>,
    pub transcript_name: Option<&'a [u8]>,
    pub transcript_source: Option<&'a [u8]>,
    pub transcript_support_level: Option<&'a [u8]>,
    pub transcript_version: Option<usize>,
}
impl<'a> AttributeRef<'a> {
    pub fn to_owned(&self) -> Attribute {
        let ccds_id = self.ccds_id.map(|x| x.to_owned());
        let exon_id = self.exon_id.map(|x| x.to_owned());
        let exon_number = self.exon_number.map(|x| x.to_owned());
        let exon_version = self.exon_version.map(|x| x.to_owned());
        let gene_biotype = self.gene_biotype.map(|x| x.to_owned());
        let gene_id = self.gene_id.map(|x| x.to_owned());
        let gene_name = self.gene_name.map(|x| x.to_owned());
        let gene_source = self.gene_source.map(|x| x.to_owned());
        let gene_version = self.gene_version.map(|x| x.to_owned());
        let protein_id = self.protein_id.map(|x| x.to_owned());
        let protein_version = self.protein_version.map(|x| x.to_owned());
        let tag = self.tag.map(|x| x.to_owned());
        let transcript_biotype = self.transcript_biotype.map(|x| x.to_owned());
        let transcript_id = self.transcript_id.map(|x| x.to_owned());
        let transcript_name = self.transcript_name.map(|x| x.to_owned());
        let transcript_source = self.transcript_source.map(|x| x.to_owned());
        let transcript_support_level = self.transcript_support_level.map(|x| x.to_owned());
        let transcript_version = self.transcript_version.map(|x| x.to_owned());
        Attribute {
            ccds_id,
            exon_id,
            exon_number,
            exon_version,
            gene_biotype,
            gene_id,
            gene_name,
            gene_source,
            gene_version,
            protein_id,
            protein_version,
            tag,
            transcript_biotype,
            transcript_id,
            transcript_name,
            transcript_source,
            transcript_support_level,
            transcript_version,
        }
    }
    pub fn update_field(&mut self, field: Field, value: &'a [u8]) {
        match field {
            Field::CcdsId => {
                self.ccds_id = Some(value);
            }
            Field::ExonId => {
                self.exon_id = Some(value);
            }
            Field::ExonNumber => {
                let value = parse_to_usize(value).expect("Could not parse exon number to integer");
                self.exon_number = Some(value);
            }
            Field::ExonVersion => {
                let value = parse_to_usize(value).expect("Could not parse exon version to integer");
                self.exon_version = Some(value);
            }
            Field::GeneBiotype => {
                self.gene_biotype = Some(value);
            }
            Field::GeneName => {
                self.gene_name = Some(value);
            }
            Field::GeneSource => {
                self.gene_source = Some(value);
            }
            Field::GeneVersion => {
                let value = parse_to_usize(value).expect("Could not parse gene version to integer");
                self.gene_version = Some(value);
            }
            Field::ProteinId => {
                self.protein_id = Some(value);
            }
            Field::ProteinVersion => {
                let value =
                    parse_to_usize(value).expect("Could not parse protein version to integer");
                self.protein_version = Some(value);
            }
            Field::Tag => {
                self.tag = Some(value);
            }
            Field::TranscriptBiotype => {
                self.transcript_biotype = Some(value);
            }
            Field::TranscriptId => {
                self.transcript_id = Some(value);
            }
            Field::TranscriptName => {
                self.transcript_name = Some(value);
            }
            Field::TranscriptSource => {
                self.transcript_source = Some(value);
            }
            Field::TranscriptSupportLevel => {
                self.transcript_support_level = Some(value);
            }
            Field::TranscriptVersion => {
                let value =
                    parse_to_usize(value).expect("Could not parse transcript version to integer");
                self.transcript_version = Some(value);
            }
            _ => {}
        }
    }
}
