mod generate_xml;
mod read_csv;
mod record_type;

use crate::{generate_xml::generate_xml, read_csv::CsvSource, record_type::RecordType};
use indicatif::{ProgressBar, ProgressStyle};
use quicli::prelude::*;
use std::{fs::File, io};
use structopt::StructOpt;
use strum;

/// Reads csv and writes xml. The resulting XML Document is intended for deliveries to the
/// Blue Yonder Supply and Demand API. This tool only checks for correct utf8 encoding and nothing
/// else.
#[derive(Debug, StructOpt)]
struct Cli {
    /// Root tag of generated XML.
    #[structopt()]
    category: String,
    /// Path to input file. If ommited STDIN is used for input.
    #[structopt(long = "input", short = "i", parse(from_os_str))]
    input: Option<std::path::PathBuf>,
    /// Path to output file. If ommited output is written to STDOUT.
    #[structopt(long = "output", short = "o", parse(from_os_str))]
    output: Option<std::path::PathBuf>,
    /// Record type of generated XML. Should be either Record, DeleteRecord, DeleteAllRecords.
    #[structopt(long = "record-type", short = "r", default_value = "Record")]
    record_type: RecordType,
    /// Character used as delimiter between csv columns. While this tool assumes utf8 encoding,
    /// only ASCII delimiters are supported.
    #[structopt(long = "delimiter", short = "d", default_value = ",")]
    delimiter: char,
}

fn main() -> CliResult {
    let args = Cli::from_args();

    // Only initialized in case `input` specifies a file path, because only then we have information
    // about input length.
    //
    // We keep this in top level scope, since we want the progress bar to live during the whole
    // program execution, so it will be displayed.
    let progress_bar;

    let input: Box<dyn io::Read> = if let Some(input) = args.input {
        // Path argument specified. Open file and initialize progress bar.
        let file = File::open(&input)?;
        // Progress bar interferes with formatting if stdout and stderr both go to console
        if args.output.is_some() {
            let len = file.metadata()?.len();
            progress_bar = ProgressBar::new(len);
            let fmt = "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})";
            progress_bar.set_style(
                ProgressStyle::default_bar()
                    .template(fmt)
                    .progress_chars("#>-"),
            );
            Box::new(progress_bar.wrap_read(file))
        } else {
            Box::new(file)
        }
    } else {
        // just use stdin
        Box::new(io::stdin())
    };
    let reader = CsvSource::new(input, args.delimiter as u8)?;

    let mut out: Box<dyn io::Write> = if let Some(output) = args.output {
        Box::new(io::BufWriter::new(File::create(&output)?))
    } else {
        Box::new(io::stdout())
    };
    generate_xml(&mut out, reader, &args.category, args.record_type)?;
    Ok(())
}
