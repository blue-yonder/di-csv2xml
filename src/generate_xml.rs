use crate::record_type::RecordType;
use csv;
use quick_xml::{Writer, events::{Event, BytesDecl, BytesStart, BytesText, BytesEnd}};
use std::io::{self, Read, Write};

const CUSTOMER_EXTENSION_PREFIX: &str = "CUEX_";

pub fn generate_xml<O: Write, I: Read>(
    out: O,
    input: &mut csv::Reader<I>,
    category: &str,
    record_type: RecordType,
) -> io::Result<()> {

    let mut writer = Writer::new_with_indent(out, b'\t', 1);
    let header = input.headers()?.clone();
    let (customer_extension, standard): (Vec<_>, Vec<_>) =
        (0..header.len()).partition(|&index| header[index].starts_with(CUSTOMER_EXTENSION_PREFIX));
    // Write declaration
    // <?xml version="1.0" encoding="UTF-8" ?>
    writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), None))).map_err(expect_io_error)?;
    // Open root tag (Category)
    open_markup(&mut writer, category)?;
    // Write one record for each entry in csv
    let mut record = csv::StringRecord::new();
    while input.read_record(&mut record)? {
        write_record(
            &mut writer,
            &Record {
                standard: &standard,
                extensions: &customer_extension,
                tag_names: &header,
                values: &record,
            },
            record_type.as_str(),
        )?;
    }
    // Close root tag (Category)
    close_markup(&mut writer, category)?;
    Ok(())
}

/// Unwraps io::error, panics if the it is not `quick_xml::Error::Io`
/// 
/// Call this then the only possible way to fail for XML writing is IO
fn expect_io_error(error: quick_xml::Error) -> io::Error {
    use quick_xml::Error::*;
    match error {
        Io(io_error) => io_error,
        _ => panic!("Unexpected failure: {}", error)
    }
}

fn open_markup<W>(writer: &mut Writer<W>, name: &str) -> io::Result<()>
where
    W: io::Write,
{
    writer.write_event(Event::Start(BytesStart::borrowed_name(name.as_bytes()))).map_err(
        // Only io errors can happen, every other variant should be logically impossible
        |error| match error {
            quick_xml::Error::Io(io_error) => io_error,
            _ => panic!("Unexpected error: {}"),
        }
    )?;
    Ok(())
}

fn write_record<W>(writer: &mut Writer<W>, record: &Record<'_>, record_type: &str) -> io::Result<()>
where
    W: io::Write,
{
    open_markup(writer, record_type)?;
    for (name, value) in record.standard() {
        open_markup(writer, name)?;
        write_text(writer, value)?;
        close_markup(writer, name)?;
    }
    // customer extensions
    let mut extensions = record.extensions().peekable();
    if extensions.peek().is_some() {
        open_markup(writer, "CustomerExtensions")?;
        for (name, value) in record.extensions() {
            open_markup(writer, name)?;
            write_text(writer, value)?;
            close_markup(writer, name)?;
        }
        close_markup(writer, "CustomerExtensions")?;
    }

    close_markup(writer, record_type)?;
    Ok(())
}

fn write_text<W>(writer: &mut Writer<W>, text: &str) -> io::Result<()> where W : io::Write{
    writer.write_event(Event::Text(BytesText::from_plain_str(text))).map_err(expect_io_error)?;
    Ok(())
}

fn close_markup<W>(writer: &mut Writer<W>, name: &str) -> io::Result<()>
where
    W: io::Write,
{
    writer.write_event(Event::End(BytesEnd::borrowed(name.as_bytes()))).map_err(expect_io_error)?;
    Ok(())
}

/// Represents one XML record
struct Record<'a> {
    /// Indices of standard records within csv
    standard: &'a [usize],
    /// Indices of extension records within csv
    extensions: &'a [usize],
    /// Header of csv provides tag names
    tag_names: &'a csv::StringRecord,
    /// Csv row which provides the values of this record
    values: &'a csv::StringRecord,
}

impl<'a> Record<'a> {
    /// Returns an iterator over all standard tags. `Item = (tag_name, value)`
    fn standard(&self) -> impl Iterator<Item = (&str, &str)> {
        self.standard.iter().map(move |&index| (&self.tag_names[index], &self.values[index]))
            // Empty strings are treated as null and will not be rendered in XML
            .filter(|&(_, ref v)| !v.is_empty())
    }

    /// Returns an iterator over all customer extension tags. `Item = (tag_name, value)`
    fn extensions(&self) -> impl Iterator<Item = (&str, &str)> {
        // This helps us to cut of the leading 'CUEX_' prefix from tag names
        let skip = CUSTOMER_EXTENSION_PREFIX.len();

        self.extensions.iter().map(move |&index| (&self.tag_names[index][skip..], &self.values[index]))
            // Empty strings are treated as null and will not be rendered in XML
            .filter(|&(_, ref v)| !v.is_empty())
    }
}
