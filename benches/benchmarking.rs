use criterion::{criterion_group, criterion_main, Criterion};
use nom_gtf::{GtfRecordRef, GtfRecord};

pub fn benchmark_gtf_record_ref(c: &mut Criterion) {
    let line: &[u8] = br#"1	ensembl_havana	gene	1471765	1497848	.	+	.	gene_id "ENSG00000160072"; gene_version "20"; gene_name "ATAD3B"; gene_source "ensembl_havana"; gene_biotype "protein_coding";"#;
    c.bench_function("gtf_record_ref", |b| b.iter(|| GtfRecordRef::from_bytes(&line).unwrap()));
}

pub fn benchmark_gtf_record(c: &mut Criterion) {
    let line: &[u8] = br#"1	ensembl_havana	gene	1471765	1497848	.	+	.	gene_id "ENSG00000160072"; gene_version "20"; gene_name "ATAD3B"; gene_source "ensembl_havana"; gene_biotype "protein_coding";"#;
    c.bench_function("gtf_record_owned", |b| b.iter(|| GtfRecord::from_bytes(&line).unwrap()));
}

criterion_group!(
    benches, 
    benchmark_gtf_record,
    benchmark_gtf_record_ref);
criterion_main!(benches);
