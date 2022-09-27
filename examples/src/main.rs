use std::{fs::File, io::BufReader};

use anyhow::Result;
use flate2::read::MultiGzDecoder;
use gtftools::GtfReader;

fn from_gzip_file(path: &str) -> Result<()> {
    let count = File::open(path)
        .map(MultiGzDecoder::new)
        .map(BufReader::new)
        .map(GtfReader::from_bufread)?
        .into_iter()
        .count();
    println!("Num records: {}", count);
    Ok(())
}

fn from_file(path: &str) -> Result<()> {
    let count = File::open(path)
        .map(BufReader::new)
        .map(GtfReader::from_bufread)?
        .into_iter()
        .count();
    println!("Num records: {}", count);
    Ok(())
}

fn main() -> Result<()> {
    from_file("./data/example.gtf")?;
    from_gzip_file("./data/example.gtf.gz")?;
    Ok(())
}
