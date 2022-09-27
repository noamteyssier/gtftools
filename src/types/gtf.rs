use anyhow::{Result, bail};
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
}

#[cfg(test)]
mod testing {
    use super::Gtf;

    #[test]
    fn test_gtf_from_file() {
        let path = "examples/data/example.gtf";
        let gtf = Gtf::from_file(path).unwrap();
        assert_eq!(gtf.len(), 5);
    }

}
