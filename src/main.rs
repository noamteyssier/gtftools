use std::fs::File;
use anyhow::Result;
use csv::ReaderBuilder;
use nom_gtf::GtfRecord;

fn main() -> Result<()> {
    let handle = File::open("data/example.gtf")?;

    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .comment(Some(b'#'))
        .from_reader(handle);

    for row in reader.byte_records() {
        let record = GtfRecord::from_byte_record(&row?)?;
        println!("{:#?}", record);
    }

    Ok(())
}
