mod generate_xml;
mod read_csv;
mod record_type;

use crate::{generate_xml::generate_xml, read_csv::CsvSource, record_type::RecordType};
use flate2::{bufread::GzDecoder, GzBuilder};
use indicatif::{ProgressBar, ProgressStyle};
use quicli::prelude::*;
use std::{fs::File, io, path::Path};
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
    #[structopt(long, short = "i", parse(from_os_str))]
    input: Option<std::path::PathBuf>,
    /// Path to output file. If ommited output is written to STDOUT.
    #[structopt(long, short = "o", parse(from_os_str))]
    output: Option<std::path::PathBuf>,
    /// Record type of generated XML. Should be either Record, DeleteRecord, DeleteAllRecords.
    #[structopt(long = "record-type", short = "r", default_value = "Record")]
    record_type: RecordType,
    /// Character used as delimiter between csv columns. While this tool assumes utf8 encoding,
    /// only ASCII delimiters are supported.
    #[structopt(long, short = "d", default_value = ",")]
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
        // Only show Progress bar, if both input and output are files.
        //
        // * We need the input to so we have the file metadata and therefore file length, to know
        // the amount of data we are going to proccess. Otherwise we can't set the length of the
        // progress bar.
        // * We don't want the Progress bar to interfere with the output, if writing to stdout.
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
            let file_with_pbar = progress_bar.wrap_read(file);

            if has_gz_extension(&input) {
                Box::new(GzDecoder::new(io::BufReader::new(file_with_pbar)))
            } else {
                Box::new(file_with_pbar)
            }
        } else {
            // Input file, but writing to stdout

            // Repeat if to avoid extra Box.
            if has_gz_extension(&input) {
                Box::new(GzDecoder::new(io::BufReader::new(file)))
            } else {
                Box::new(file)
            }
        }
    } else {
        // Input path not set => Just use stdin
        Box::new(io::stdin())
    };

    let reader = CsvSource::new(input, args.delimiter as u8)?;

    let mut out: Box<dyn io::Write> = if let Some(output) = args.output {
        let writer = io::BufWriter::new(File::create(&output)?);

        if has_gz_extension(&output) {
            Box::new(GzBuilder::new().write(writer, Default::default()))
        } else {
            Box::new(writer)
        }
    } else {
        Box::new(io::stdout())
    };
    generate_xml(&mut out, reader, &args.category, args.record_type)?;
    Ok(())
}

fn has_gz_extension(path: &Path) -> bool {
    match path.extension() {
        Some(ext) if ext == "gz" => true,
        _ => false,
    }
}
