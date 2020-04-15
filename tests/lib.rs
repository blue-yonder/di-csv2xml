use assert_cli;
use std::{fs::File, io::Read};
use tempfile::tempdir;

#[test]
fn simple() {
    assert_cli::Assert::main_binary()
        .with_args(&["Category", "--input", "tests/input.csv"])
        .succeeds()
        .stdout()
        .is(include_str!("output.xml").replace("\r\n", "\n").as_str())
        .unwrap();
}

#[test]
fn input_gz() {
    assert_cli::Assert::main_binary()
        .with_args(&["Category", "--input", "tests/input.csv.gz"])
        .succeeds()
        .stdout()
        .is(include_str!("output.xml").replace("\r\n", "\n").as_str())
        .unwrap();
}

#[test]
fn mask_text() {
    assert_cli::Assert::main_binary()
        .with_args(&["Text", "--input", "tests/text.csv"])
        .succeeds()
        .stdout()
        .is(include_str!("text.xml").replace("\r\n", "\n").as_str())
        .unwrap();
}

#[test]
fn semicolon_delimiter() {
    assert_cli::Assert::main_binary()
        .with_args(&[
            "Category",
            "--input",
            "tests/sem_delim.csv",
            "--delimiter",
            ";",
        ])
        .succeeds()
        .stdout()
        .is(include_str!("output.xml").replace("\r\n", "\n").as_str())
        .unwrap();
}

#[test]
fn delete_record() {
    assert_cli::Assert::main_binary()
        .with_args(&[
            "Root",
            "--input",
            "tests/simple.csv",
            "--record-type",
            "DeleteRecord",
        ])
        .succeeds()
        .stdout()
        .is(include_str!("simple_delete.xml")
            .replace("\r\n", "\n")
            .as_str())
        .unwrap();
}

#[test]
fn delete_all() {
    assert_cli::Assert::main_binary()
        .with_args(&[
            "Root",
            "--input",
            "tests/simple.csv",
            "--record-type",
            "DeleteAllRecords",
        ])
        .succeeds()
        .stdout()
        .is(include_str!("simple_delete_all.xml")
            .replace("\r\n", "\n")
            .as_str())
        .unwrap();
}

#[test]
fn customer_extensions() {
    assert_cli::Assert::main_binary()
        .with_args(&["Root", "--input", "tests/customer_extensions.csv"])
        .succeeds()
        .stdout()
        .is(include_str!("customer_extensions.xml")
            .replace("\r\n", "\n")
            .as_str())
        .unwrap();
}

#[test]
fn write_gz() {
    let out_dir = tempdir().unwrap();

    // Magic ".gz" file ending tells tool to compress data
    let out_path = out_dir.path().join("output-xml.gz");
    let out_str = out_path.to_str().expect("Tempfile path must be utf8");
    assert_cli::Assert::main_binary()
        .with_args(&[
            "Category",
            "--input",
            "tests/input.csv",
            "--output",
            out_str,
        ])
        .succeeds()
        .unwrap();

    // // Compare output file with expectation
    // let mut expected = Vec::new();
    // File::open("tests/output.xml.gz")
    //     .unwrap()
    //     .read_to_end(&mut expected)
    //     .unwrap();

    // let mut actual = Vec::new();
    // File::open(out_path)
    //     .unwrap()
    //     .read_to_end(&mut actual)
    //     .unwrap();

    // assert_eq!(expected, actual);

    // By closing the `out_dir` explicitly, we can check that it has been deleted successfully. If
    // we don't close it explicitly, the file will still be deleted when `file` goes out of scope,
    // but we won't know whether deleting the file succeeded.
    out_dir.close().unwrap();
}
