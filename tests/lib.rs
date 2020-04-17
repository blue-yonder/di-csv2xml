use assert_cmd::Command;
use std::{fs::File, io::Read};
use tempfile::tempdir;

#[test]
fn simple() {
    Command::cargo_bin("di-csv2xml")
        .unwrap()
        .args(&["--category", "Category", "--input", "tests/input.csv"])
        .assert()
        .success()
        .stdout(include_str!("output.xml").replace("\r\n", "\n"));
}

#[test]
fn simple_stdin() {
    Command::cargo_bin("di-csv2xml")
        .unwrap()
        .write_stdin(include_str!("input.csv").replace("\r\n", "\n"))
        .args(&["--category", "Category", "--input", "-"])
        .assert()
        .success()
        .stdout(include_str!("output.xml").replace("\r\n", "\n"));
}

#[test]
fn input_gz() {
    Command::cargo_bin("di-csv2xml")
        .unwrap()
        .args(&["--category", "Category", "--input", "tests/input.csv.gz"])
        .assert()
        .success()
        .stdout(include_str!("output.xml").replace("\r\n", "\n"));
}

#[test]
fn mask_text() {
    Command::cargo_bin("di-csv2xml")
        .unwrap()
        .args(&["--category", "Text", "--input", "tests/text.csv"])
        .assert()
        .success()
        .stdout(include_str!("text.xml").replace("\r\n", "\n"));
}

#[test]
fn semicolon_delimiter() {
    Command::cargo_bin("di-csv2xml")
        .unwrap()
        .args(&[
            "--category",
            "Category",
            "--input",
            "tests/sem_delim.csv",
            "--delimiter",
            ";",
        ])
        .assert()
        .success()
        .stdout(include_str!("output.xml").replace("\r\n", "\n"));
}

#[test]
fn delete_record() {
    Command::cargo_bin("di-csv2xml")
        .unwrap()
        .args(&[
            "--category",
            "Root",
            "--input",
            "tests/simple.csv",
            "--record-type",
            "DeleteRecord",
        ])
        .assert()
        .success()
        .stdout(include_str!("simple_delete.xml").replace("\r\n", "\n"));
}

#[test]
fn delete_all() {
    Command::cargo_bin("di-csv2xml")
        .unwrap()
        .args(&[
            "--category",
            "Root",
            "--input",
            "tests/simple.csv",
            "--record-type",
            "DeleteAllRecords",
        ])
        .assert()
        .success()
        .stdout(include_str!("simple_delete_all.xml").replace("\r\n", "\n"));
}

#[test]
fn customer_extensions() {
    Command::cargo_bin("di-csv2xml")
        .unwrap()
        .args(&[
            "--category",
            "Root",
            "--input",
            "tests/customer_extensions.csv",
        ])
        .assert()
        .success()
        .stdout(include_str!("customer_extensions.xml").replace("\r\n", "\n"));
}

#[test]
fn write_gz() {
    let out_dir = tempdir().unwrap();

    // Magic ".gz" file ending tells tool to compress data
    let out_path = out_dir.path().join("output-xml.gz");
    let out_str = out_path.to_str().expect("Tempfile path must be utf8");
    Command::cargo_bin("di-csv2xml")
        .unwrap()
        .args(&[
            "--category",
            "Category",
            "--input",
            "tests/input.csv",
            "--output",
            out_str,
        ])
        .assert()
        .success();

    // Compare output file with expectation
    let mut expected = Vec::new();
    File::open("tests/output.xml.gz")
        .unwrap()
        .read_to_end(&mut expected)
        .unwrap();

    let mut actual = Vec::new();
    File::open(out_path)
        .unwrap()
        .read_to_end(&mut actual)
        .unwrap();

    assert_eq!(expected, actual);

    // By closing the `out_dir` explicitly, we can check that it has been deleted successfully. If
    // we don't close it explicitly, the file will still be deleted when `file` goes out of scope,
    // but we won't know whether deleting the file succeeded.
    out_dir.close().unwrap();
}
