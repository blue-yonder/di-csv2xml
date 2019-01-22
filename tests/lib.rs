use assert_cli;

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
        ]).succeeds()
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
        ]).succeeds()
        .stdout()
        .is(include_str!("simple_delete.xml")
            .replace("\r\n", "\n")
            .as_str()).unwrap();
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
        ]).succeeds()
        .stdout()
        .is(include_str!("simple_delete_all.xml")
            .replace("\r\n", "\n")
            .as_str()).unwrap();
}

#[test]
fn customer_extensions() {
    assert_cli::Assert::main_binary()
        .with_args(&["Root", "--input", "tests/customer_extensions.csv"])
        .succeeds()
        .stdout()
        .is(include_str!("customer_extensions.xml")
            .replace("\r\n", "\n")
            .as_str()).unwrap();
}
