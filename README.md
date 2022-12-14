# gtftools

a crate for parsing and querying Ensembl-GTF formatted files.

Parser achieves near `wc -l` throughput.

## Usage

This is meant to be used as an iterator and receives any item implementing `BufRead`.

### From File

```rust
use std::{fs::File, io::BufReader};
use gtftools::GtfReader;

let handle = File::open("data/example.gtf")
  .map(BufReader::new)
  .unwrap();

let num_records = GtfReader::from_bufread(handle)
  .filter_map(|x| x.ok())
  .count();

assert_eq!(num_records, 10);
```

### From Gzip File

```rust
use std::{fs::File, io::BufReader};
use flate2::read::MultiGzDecoder;
use gtftools::GtfReader;

let handle = File::open("data/example.gtf.gz")
  .map(MultiGzDecoder::new)
  .map(BufReader::new)
  .unwrap();

let num_records = GtfReader::from_bufread(handle)
  .filter_map(|x| x.ok())
  .count();

assert_eq!(num_records, 10);
```
