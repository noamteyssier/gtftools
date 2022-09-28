use crate::types::{Gtf, GtfRecord};

#[derive(Debug, Clone)]
pub struct Intron {
    pub start: usize,
    pub end: usize,
}
impl Intron {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl Gtf {
    pub fn introns(&self) {
        let genes = self.genes();
        let transcripts = self.transcripts();
        let exons = self.exons();

        for t in transcripts.iter() {
            let records = filter_transcripts(t, &exons, true);
            let (start_sites, end_sites) = collect_endpoints(&records);
            build_introns(t, start_sites, end_sites);
            break;
        }
    }
}

fn filter_transcripts(needle: &GtfRecord, haystack: &Gtf, sort: bool) -> Gtf {
    let records = haystack
        .iter()
        .filter(|x| x.attribute.transcript_id == needle.attribute.transcript_id)
        .map(|x| x.to_owned())
        .collect();
    let mut gtf = Gtf::new(records);
    if sort {
        gtf.sort();
    }
    gtf
}

fn collect_endpoints(exons: &Gtf) -> (Vec<usize>, Vec<usize>) {
    exons
        .iter()
        .map(|x| (x.start, x.end))
        .unzip()
}

fn build_introns(transcript: &GtfRecord, start_sites: Vec<usize>, end_sites: Vec<usize>) {
    let num_sites = start_sites.len();
    
    // println!("\n==========================");
    // println!("TID: {}", String::from_utf8(transcript.attribute.transcript_id.to_owned().unwrap()).unwrap());
    // println!("transcript: {} {}", transcript.start, transcript.end);
    // println!("starts: {:?}", start_sites);
    // println!("ends:   {:?}", end_sites);
    // println!("--");
    //
    for idx in 0..num_sites + 1 {
        let start = if idx == 0 {
            transcript.start
        } else {
            end_sites[idx - 1]
        };

        let end = if idx == num_sites {
            transcript.end
        } else {
            start_sites[idx]
        };

        if start == end {
            continue;
        }

        if start > end {
            panic!("Start site above end site");
        }

        let intron = Intron::new(start, end);

        // println!("{:?}", intron);
    }
    // println!("\n==========================");
}

