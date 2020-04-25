use assert_cmd::Command;
use criterion::{criterion_group, criterion_main, Criterion};
use std::{fs::File, io::Read};
use tempfile::tempdir;

fn stream_io(input: &[u8]) {
    Command::cargo_bin("di-csv2xml")
        .unwrap()
        .write_stdin(input)
        .args(&["--category", "Category", "--input", "-"])
        .ok()
        .unwrap();
}

fn file_io(in_path: &str, out_path: &str) {
    Command::cargo_bin("di-csv2xml")
        .unwrap()
        .args(&[
            "--category",
            "Category",
            "--input",
            in_path,
            "--output",
            out_path,
        ])
        .ok()
        .unwrap();
}

fn stream_io_benchmark(c: &mut Criterion) {
    let mut buf_input = Vec::new();
    File::open("./benches/dummy_recs_200.csv")
        .unwrap()
        .read_to_end(&mut buf_input)
        .unwrap();
    c.bench_function("streaming 200 recs", |b| b.iter(|| stream_io(&buf_input)));
}

fn file_io_benchmark(c: &mut Criterion) {
    let out_dir = tempdir().unwrap();
    let out_path = out_dir.path().join("output-xml.gz");
    let out_str = out_path.to_str().expect("Tempfile path must be utf8");
    c.bench_function("file io 200 recs", |b| {
        b.iter(|| file_io("./benches/dummy_recs_200.csv", out_str))
    });
}

criterion_group!(benches, stream_io_benchmark, file_io_benchmark);
criterion_main!(benches);
