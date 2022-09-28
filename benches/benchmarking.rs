use criterion::{criterion_group, criterion_main, Criterion};
use gtftools::{Gtf, GtfRecord, GtfRecordRef};

pub fn benchmark_gtf_record_ref(c: &mut Criterion) {
    let line: &[u8] = br#"1	ensembl_havana	gene	1471765	1497848	.	+	.	gene_id "ENSG00000160072"; gene_version "20"; gene_name "ATAD3B"; gene_source "ensembl_havana"; gene_biotype "protein_coding";"#;
    c.bench_function("gtf_record_ref", |b| {
        b.iter(|| GtfRecordRef::from_bytes(&line).unwrap())
    });
}

pub fn benchmark_gtf_record(c: &mut Criterion) {
    let line: &[u8] = br#"1	ensembl_havana	gene	1471765	1497848	.	+	.	gene_id "ENSG00000160072"; gene_version "20"; gene_name "ATAD3B"; gene_source "ensembl_havana"; gene_biotype "protein_coding";"#;
    c.bench_function("gtf_record_owned", |b| {
        b.iter(|| GtfRecord::from_bytes(&line).unwrap())
    });
}

pub fn benchmark_introns(c: &mut Criterion) {
    let gtf = Gtf::from_file("benches/data/dual.gtf").unwrap();
    c.bench_function("gtf_introns", |b| {
        b.iter(|| gtf.introns())
    });
}

criterion_group!(benches, benchmark_gtf_record, benchmark_gtf_record_ref, benchmark_introns);
criterion_main!(benches);
