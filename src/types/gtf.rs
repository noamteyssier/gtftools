use anyhow::{Result, bail};
use bstr::ByteSlice;
use std::{fs::File, io::{BufReader, BufRead, Read}};
use super::{GtfRecord, GtfReader};

pub struct Gtf {
    records: Vec<GtfRecord>
}
impl Gtf {

    pub fn from_file(path: &str) -> Result<Self> {
        let file = File::open(path)?;
        Self::from_reader(file)
    }

    pub fn from_reader<R: Read>(reader: R) -> Result<Self> {
        let buffer = BufReader::new(reader);
        Self::from_bufread(buffer)
    }

    pub fn from_bufread<B: BufRead>(buffer: B) -> Result<Self> {
        let reader = GtfReader::from_bufread(buffer);
        Self::parse_records(reader)
    }

    fn parse_records<B: BufRead>(reader: GtfReader<B>) -> Result<Self> {
        let records = reader
            .into_iter()
            .filter_map(|x| x.ok())
            .collect::<Vec<GtfRecord>>();

        if records.len() == 0 {
            bail!("No records found in reader")
        } else {
            Ok( Self { records } )
        }
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn feature_subset(&self, feature_name: &str) -> Self {
        let records = self.records
            .iter()
            .filter(|x| x.feature.contains_str(feature_name))
            .map(|x| x.to_owned())
            .collect();
        Self { records }
    }

    pub fn genes(&self) -> Self {
        self.feature_subset("gene")
    }

    pub fn transcripts(&self) -> Self {
        self.feature_subset("transcript")
    }

    pub fn exons(&self) -> Self {
        self.feature_subset("exon")
    }
}

#[cfg(test)]
mod testing {
    use std::io::Cursor;

    use super::Gtf;

    fn example_gtf() -> String {
        let gtf_raw = r##"#!genome-build GRCh38.p13
                #!genome-version GRCh38
                #!genome-date 2013-12
                #!genome-build-accession GCA_000001405.28
                #!genebuild-last-updated 2022-04
                1	ensembl_havana	gene	1471765	1497848	.	+	.	gene_id "ENSG00000160072"; gene_version "20"; gene_name "ATAD3B"; gene_source "ensembl_havana"; gene_biotype "protein_coding";
                1	ensembl_havana	transcript	1471765	1497848	.	+	.	gene_id "ENSG00000160072"; gene_version "20"; transcript_id "ENST00000673477"; transcript_version "1"; gene_name "ATAD3B"; gene_source "ensembl_havana"; gene_biotype "protein_coding"; transcript_name "ATAD3B-206"; transcript_source "ensembl_havana"; transcript_biotype "protein_coding"; tag "CCDS"; ccds_id "CCDS30"; tag "basic";
                1	ensembl_havana	exon	1471765	1472089	.	+	.	gene_id "ENSG00000160072"; gene_version "20"; transcript_id "ENST00000673477"; transcript_version "1"; exon_number "1"; gene_name "ATAD3B"; gene_source "ensembl_havana"; gene_biotype "protein_coding"; transcript_name "ATAD3B-206"; transcript_source "ensembl_havana"; transcript_biotype "protein_coding"; tag "CCDS"; ccds_id "CCDS30"; exon_id "ENSE00003889014"; exon_version "1"; tag "basic";
                1	ensembl_havana	exon	1471765	1472089	.	+	.	gene_id "ENSG00000160072"; gene_version "20"; transcript_id "ENST00000673477"; transcript_version "1"; exon_number "1"; gene_name "ATAD3B"; gene_source "ensembl_havana"; gene_biotype "protein_coding"; transcript_name "ATAD3B-206"; transcript_source "ensembl_havana"; transcript_biotype "protein_coding"; tag "CCDS"; ccds_id "CCDS30"; exon_id "ENSE00003889014"; exon_version "1"; tag "basic";
                1	ensembl_havana	exon	1471765	1472089	.	+	.	gene_id "ENSG00000160072"; gene_version "20"; transcript_id "ENST00000673477"; transcript_version "1"; exon_number "1"; gene_name "ATAD3B"; gene_source "ensembl_havana"; gene_biotype "protein_coding"; transcript_name "ATAD3B-206"; transcript_source "ensembl_havana"; transcript_biotype "protein_coding"; tag "CCDS"; ccds_id "CCDS30"; exon_id "ENSE00003889014"; exon_version "1"; tag "basic";
                1	ensembl_havana	CDS	1471885	1472089	.	+	0	gene_id "ENSG00000160072"; gene_version "20"; transcript_id "ENST00000673477"; transcript_version "1"; exon_number "1"; gene_name "ATAD3B"; gene_source "ensembl_havana"; gene_biotype "protein_coding"; transcript_name "ATAD3B-206"; transcript_source "ensembl_havana"; transcript_biotype "protein_coding"; tag "CCDS"; ccds_id "CCDS30"; protein_id "ENSP00000500094"; protein_version "1"; tag "basic";
                1	ensembl_havana	start_codon	1471885	1471887	.	+	0	gene_id "ENSG00000160072"; gene_version "20"; transcript_id "ENST00000673477"; transcript_version "1"; exon_number "1"; gene_name "ATAD3B"; gene_source "ensembl_havana"; gene_biotype "protein_coding"; transcript_name "ATAD3B-206"; transcript_source "ensembl_havana"; transcript_biotype "protein_coding"; tag "CCDS"; ccds_id "CCDS30"; tag "basic";"##;
        gtf_raw.to_string()
    }

    #[test]
    fn test_gtf_from_file() {
        let path = "examples/data/example.gtf";
        let gtf = Gtf::from_file(path).unwrap();
        assert_eq!(gtf.len(), 5);
    }

    #[test]
    fn test_gtf_from_reader() {
        let gtf_str = example_gtf();
        let cursor = Cursor::new(gtf_str);
        let gtf = Gtf::from_reader(cursor).unwrap();
        assert_eq!(gtf.len(), 7);
    }

    #[test]
    fn test_subset_genes() {
        let path = "examples/data/example.gtf";
        let gtf = Gtf::from_file(path).unwrap();
        let subset = gtf.genes();
        assert_eq!(subset.len(), 1);
    }

    #[test]
    fn test_subset_transcripts() {
        let path = "examples/data/example.gtf";
        let gtf = Gtf::from_file(path).unwrap();
        let subset = gtf.transcripts();
        assert_eq!(subset.len(), 1);
    }

    #[test]
    fn test_subset_exons() {
        let gtf_str = example_gtf();
        let cursor = Cursor::new(gtf_str);
        let gtf = Gtf::from_reader(cursor).unwrap();
        let subset = gtf.exons();
        assert_eq!(subset.len(), 3);
    }
}
