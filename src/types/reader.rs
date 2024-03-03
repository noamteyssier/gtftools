use crate::GtfRecordRef;

use super::GtfRecord;
use anyhow::{anyhow, bail, Result};
use bstr::io::{BufReadExt, ByteLines};
use bstr::ByteSlice;
use buf_redux::BufReader;
use std::io::{BufRead, Read};

pub struct GtfReader<R: BufRead> {
    inner: ByteLines<R>,
}
impl<R: BufRead> GtfReader<R> {
    pub fn from_bufread(reader: R) -> Self {
        let inner = reader.byte_lines();
        Self { inner }
    }
}
impl<R: BufRead> Iterator for GtfReader<R> {
    type Item = Result<GtfRecord>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(line) => match line {
                Ok(bytes) => match GtfRecord::from_bytes(&bytes) {
                    Ok(record) => Some(Ok(record)),
                    Err(e) => Some(Err(e)),
                },
                Err(e) => Some(Err(anyhow!("{}", e))),
            },
            None => None,
        }
    }
}

pub struct GtfRefReader<R: Read> {
    inner: BufReader<R>,
    newline_pos: usize,
    num_records: usize,
}
impl<R: Read> GtfRefReader<R> {
    pub fn from_read(reader: R) -> Self {
        let inner = BufReader::new(reader);
        Self {
            inner,
            newline_pos: 0,
            num_records: 0,
        }
    }
    fn raw_buffer(&self) -> &[u8] {
        self.inner.buffer()
    }
    fn buffer(&self) -> &[u8] {
        &self.raw_buffer()[..self.newline_pos]
    }
    fn fill_buf(&mut self) -> Result<()> {
        let initial_size = self.raw_buffer().len();
        let mut num_read = 0;
        while initial_size + num_read < self.inner.capacity() {
            match self.inner.read_into_buf() {
                Ok(0) => break,
                Ok(n) => num_read += n,
                Err(e) => bail!("Error reading into buffer: {}", e),
            }
        }
        Ok(())
    }
    fn find_newline(&self) -> Option<usize> {
        self.raw_buffer().find_byte(b'\n')
    }
    /// Consumes the buffer up to the newline
    fn consume_line(&mut self) -> Result<()> {
        if self.newline_pos > self.raw_buffer().len() {
            bail!("Newline position is out of bounds");
        }
        self.inner.consume(self.newline_pos + 1);
        self.inner.make_room();
        Ok(())
    }
    fn update_new_line(&mut self) -> Result<()> {
        if self.raw_buffer().is_empty() {
            self.fill_buf()?;
            if self.raw_buffer().is_empty() {
                return Ok(());
            }
            self.update_new_line()?;
        }
        self.newline_pos = match self.find_newline() {
            Some(pos) => pos,
            None => {
                // fill buffer again / retry
                self.fill_buf()?;
                match self.find_newline() {
                    Some(pos) => pos,
                    None => bail!("No newline found in buffer"),
                }
            }
        };
        Ok(())
    }

    fn clear_headers(&mut self) -> Result<()> {
        loop {
            self.update_new_line()?;
            if self.buffer().starts_with(b"#") {
                self.consume_line()?;
            } else {
                break;
            }
        }
        Ok(())
    }

    /// Prepare the buffer for reading the next record
    /// and clears the buffer of headers
    ///
    /// 1. Clears the buffer of headers
    /// 2. Consumes the buffer if there are any records in memory (i.e. for all steps except the
    ///    first)
    /// 3. Updates the newline position and error if no newline is found
    /// 4. Increments the number of records
    fn prepare_buffer(&mut self) -> Result<()> {
        if self.num_records == 0 {
            self.clear_headers()?;
        }
        if self.num_records > 0 {
            self.consume_line()?;
        }
        self.update_new_line()?;
        self.num_records += 1;
        Ok(())
    }
    pub fn next(&mut self) -> Option<Result<GtfRecordRef>> {
        if let Err(why) = self.prepare_buffer() {
            return Some(Err(why));
        }
        if self.raw_buffer().is_empty() {
            return None;
        }
        Some(GtfRecordRef::from_bytes(self.buffer()))
    }
}
