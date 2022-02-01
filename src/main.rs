mod generate_xml;
mod read_csv;
mod record_type;

use crate::{generate_xml::generate_xml, read_csv::CsvSource, record_type::RecordType};
use atty::{isnt, Stream};
use anyhow::Error;
use flate2::{bufread::GzDecoder, GzBuilder};
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs::File,
    io,
    path::{Path, PathBuf},
    str::FromStr,
    time::Instant,
};
use structopt::StructOpt;

/// Reads csv and writes xml. The resulting XML Document is intended for deliveries to the
/// Blue Yonder Supply and Demand API. This tool only checks for correct utf8 encoding and nothing
/// else.
#[derive(Debug, StructOpt)]
struct Cli {
    /// Root tag of generated XML.
    #[structopt(long, short = "c")]
    category: String,
    /// Path to input file. Set to "-" to use STDIN instead of a file.
    #[structopt(long, short = "i")]
    input: IoArg,
    /// Path to output file. Leave out or set to "-" to use STDOUT instead of a file.
    #[structopt(long, short = "o", default_value = "-")]
    output: IoArg,
    /// Record type of generated XML. Should be either Record, DeleteRecord, DeleteAllRecords.
    #[structopt(long = "record-type", short = "r", default_value = "Record")]
    record_type: RecordType,
    /// Character used as delimiter between csv columns. While this tool assumes utf8 encoding,
    /// only ASCII delimiters are supported.
    #[structopt(long, short = "d", default_value = ",")]
    delimiter: char,
}

/// IO argument for CLI tools which can either take a file or STDIN/STDOUT.
///
/// Caveat: stdin is represented as "-" at the command line. Which means your tool is going to have
/// a hard time operating on a file named "-".
#[derive(Debug)]
enum IoArg {
    /// Indicates that the IO is connected to stdin/stdout. Represented as a "-" on the command line.
    StdStream,
    /// Indicates that the IO is connected to a file. Contains the file path. Just enter a path
    /// at the command line.
    File(PathBuf),
}

impl IoArg {
    fn is_file(&self) -> bool {
        match self {
            IoArg::StdStream => false,
            IoArg::File(_) => true,
        }
    }
}

impl FromStr for IoArg {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s {
            "-" => IoArg::StdStream,
            other => IoArg::File(other.into()),
        };
        Ok(out)
    }
}

fn main() -> Result<(), Error> {
    let args = Cli::from_args();

    // Only initialized in case `input` specifies a file path, because only then we have information
    // about input length.
    //
    // We keep this in top level scope, since we want the progress bar to live during the whole
    // program execution, so it will be displayed.
    let progress_bar = if args.input.is_file() && (args.output.is_file() || isnt(Stream::Stdout)) {
        let progress_bar = ProgressBar::new(0);
        let fmt = "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})";
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template(fmt)
                .progress_chars("#>-"),
        );
        Some(progress_bar)
    } else {
        None
    };

    // Keep our reference to stdin alive, if need be. Only initialized if we don't read from a file
    // and read from stdin. We hold it alive at top level scop, so we can hold the lock to it, for
    // duration of the program.
    let std_in;

    // Same story for `std_out` as for stdin. We keep it alive for the duration of the program, but
    // delay initiaization until we know we need it (i.e. we are writing to stdout and not into a
    // file, we open in this code).
    let std_out;

    // Initial time measurement
    let initial_time = Instant::now();

    let input: Box<dyn io::Read> = match args.input {
        IoArg::File(input) => {
            // Path argument specified. Open file and initialize progress bar.
            let file = File::open(&input)?;
            // Only show Progress bar, if input is a file and output is not /dev/tty.
            //
            // * We need the input to so we have the file metadata and therefore file length, to know
            // the amount of data we are going to proccess. Otherwise we can't set the length of the
            // progress bar.
            // * We don't want the Progress bar to interfere with the output, if writing to /dev/tty.
            // Progress bar interferes with formatting if stdout and stderr both go to /dev/tty
            if let Some(progress_bar) = &progress_bar {
                let len = file.metadata()?.len();
                progress_bar.set_length(len);
                let file_with_pbar = progress_bar.wrap_read(file);

                if has_gz_extension(&input) {
                    Box::new(GzDecoder::new(io::BufReader::new(file_with_pbar)))
                } else {
                    Box::new(file_with_pbar)
                }
            } else {
                // Input file, but writing output to /dev/tty

                // Repeat if to avoid extra Box.
                if has_gz_extension(&input) {
                    Box::new(GzDecoder::new(io::BufReader::new(file)))
                } else {
                    Box::new(file)
                }
            }
        }
        IoArg::StdStream => {
            // Input path not set => Just use stdin
            std_in = io::stdin();
            Box::new(std_in.lock())
        }
    };

    let reader = CsvSource::new(input, args.delimiter as u8)?;

    let mut out: Box<dyn io::Write> = match args.output {
        IoArg::File(output) => {
            let writer = io::BufWriter::new(File::create(&output)?);

            if has_gz_extension(&output) {
                Box::new(GzBuilder::new().write(writer, Default::default()))
            } else {
                Box::new(writer)
            }
        }
        IoArg::StdStream => {
            std_out = io::stdout();
            let writer = io::BufWriter::new(std_out.lock());
            Box::new(writer)
        }
    };
    let num_records = generate_xml(&mut out, reader, &args.category, args.record_type)?;
    // Drop progress bar, so it's removed from stderr before we print the performance metrics.
    // Otherwise, the drop handler would erroneously remove the lower lines of the performance metrics output.
    std::mem::drop(progress_bar);
    print_performance_metrics(&initial_time, num_records);
    Ok(())
}

/// Takes a path and returns `true` if the path ends in a `.gz` extension.
fn has_gz_extension(path: &Path) -> bool {
    match path.extension() {
        Some(ext) if ext == "gz" => true,
        _ => false,
    }
}

fn print_performance_metrics(initial_time: &Instant, num_records: u64) {
    eprintln!(
        "Processed {} records in {}.",
        num_records,
        humantime::format_duration(initial_time.elapsed())
    );
}
