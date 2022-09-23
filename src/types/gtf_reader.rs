use anyhow::{Result, anyhow};
use std::io::BufRead;
use bstr::io::{BufReadExt, ByteLines};
use super::GtfRecord;


pub struct GtfReader <R: BufRead> {
    inner: ByteLines<R>
}
impl <R: BufRead> GtfReader <R> {
    pub fn from_bufread(reader: R) -> Self {
        let inner = reader.byte_lines();
        Self { inner }
    }
}
impl <R: BufRead> Iterator for GtfReader <R> {
    type Item = Result<GtfRecord>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(line) => match line {
                Ok(bytes) => match GtfRecord::from_bytes(&bytes) {
                    Ok(record) => Some(Ok(record)),
                    Err(e) => Some(Err(e))
                },
                Err(e) => Some(Err(anyhow!("{}", e)))
            },
            None => None
        }
    }
}
