extern crate assert_cli;

use std::fs::File;
use std::io::Read;

#[test]
fn simple() {
    assert_cli::Assert::main_binary()
        .with_args(&[
            "Category",
            "--input",
            "tests/input.csv",
            "--output",
            "test_output.xml",
        ])
        .succeeds()
        .unwrap();
    let mut actual = String::new();
    File::open("test_output.xml")
        .unwrap()
        .read_to_string(&mut actual)
        .unwrap();
    assert_eq!(include_str!("output.xml"), actual);
}

#[test]
fn mask_text() {
    assert_cli::Assert::main_binary()
        .with_args(&[
            "Text",
            "--input",
            "tests/text.csv",
            "--output",
            "test_output_text.xml",
        ])
        .succeeds()
        .unwrap();
    let mut actual = String::new();
    File::open("test_output_text.xml")
        .unwrap()
        .read_to_string(&mut actual)
        .unwrap();
    assert_eq!(include_str!("text.xml"), actual);
}

#[test]
fn semicolon_delimiter() {
    assert_cli::Assert::main_binary()
        .with_args(&[
            "Category",
            "--input",
            "tests/sem_delim.csv",
            "--output",
            "test_output_sem_delim.xml",
            "--delimiter",
            ";",
        ])
        .succeeds()
        .unwrap();
    let mut actual = String::new();
    File::open("test_output_sem_delim.xml")
        .unwrap()
        .read_to_string(&mut actual)
        .unwrap();
    assert_eq!(include_str!("output.xml"), actual);
}

#[test]
fn delete_record() {
    assert_cli::Assert::main_binary()
        .with_args(&[
            "Root",
            "--input",
            "tests/simple.csv",
            "--output",
            "test_output_simple_delete.xml",
            "--record-type",
            "DeleteRecord",
        ])
        .succeeds()
        .unwrap();
    let mut actual = String::new();
    File::open("test_output_simple_delete.xml")
        .unwrap()
        .read_to_string(&mut actual)
        .unwrap();
    assert_eq!(include_str!("simple_delete.xml"), actual);
}

#[test]
fn delete_all() {
    assert_cli::Assert::main_binary()
        .with_args(&[
            "Root",
            "--input",
            "tests/simple.csv",
            "--output",
            "test_output_simple_delete_all.xml",
            "--record-type",
            "DeleteAllRecords",
        ])
        .succeeds()
        .unwrap();
    let mut actual = String::new();
    File::open("test_output_simple_delete_all.xml")
        .unwrap()
        .read_to_string(&mut actual)
        .unwrap();
    assert_eq!(include_str!("simple_delete_all.xml"), actual);
}

#[test]
fn customer_extensions() {
    assert_cli::Assert::main_binary()
        .with_args(&[
            "Root",
            "--input",
            "tests/customer_extensions.csv",
            "--output",
            "test_output_customer_extensions.xml",
        ])
        .succeeds()
        .unwrap();
    let mut actual = String::new();
    File::open("test_output_customer_extensions.xml")
        .unwrap()
        .read_to_string(&mut actual)
        .unwrap();
    assert_eq!(include_str!("customer_extensions.xml"), actual);
}
