use crate::{
    read_csv::{CsvSource, Record},
    record_type::RecordType,
};
use quick_xml::{
    events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event},
    Writer,
};
use std::io::{self, Read, Write};

/// Emits XML to `out`. One record for each record found in `reader`.
///
/// The resulting XML is compatible with the Blue Yonder Supply and demand API. All customer
/// extensions are placed within a `CustomerExtension` tag in within the record. `category` is used
/// for the root tag name. `record_type` switches between `<Record>`, `<DeleteRecord>` and
/// `<DeleteAllRecords>` in markup.
pub fn generate_xml<O: Write, I: Read>(
    out: O,
    mut reader: CsvSource<I>,
    category: &str,
    record_type: RecordType,
) -> io::Result<()> {
    let mut writer = Writer::new_with_indent(out, b'\t', 1);
    // Write declaration
    // <?xml version="1.0" encoding="UTF-8" ?>
    writer
        .write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), None)))
        .map_err(expect_io_error)?;
    // Open root tag (Category)
    open_markup(&mut writer, category)?;
    while let Some(record) = reader.read_record()? {
        // Write one record for each entry in csv
        write_record(&mut writer, &record, record_type.as_str())?;
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
        _ => panic!("Unexpected failure: {}", error),
    }
}

fn open_markup<W>(writer: &mut Writer<W>, name: &str) -> io::Result<()>
where
    W: io::Write,
{
    writer
        .write_event(Event::Start(BytesStart::borrowed_name(name.as_bytes())))
        .map_err(expect_io_error)?;
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

fn write_text<W>(writer: &mut Writer<W>, text: &str) -> io::Result<()>
where
    W: io::Write,
{
    writer
        .write_event(Event::Text(BytesText::from_plain_str(text)))
        .map_err(expect_io_error)?;
    Ok(())
}

fn close_markup<W>(writer: &mut Writer<W>, name: &str) -> io::Result<()>
where
    W: io::Write,
{
    writer
        .write_event(Event::End(BytesEnd::borrowed(name.as_bytes())))
        .map_err(expect_io_error)?;
    Ok(())
}
