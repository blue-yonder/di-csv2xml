use crate::{
    escape_str::escape_char_data,
    record_type::RecordType
};
use csv;
use std::io::{self, Read, Write};

const CUSTOMER_EXTENSION_PREFIX: &str = "CUEX_";

pub fn generate_xml<O: Write, I: Read>(
    mut out: O,
    input: &mut csv::Reader<I>,
    category: &str,
    record_type: RecordType,
) -> io::Result<()> {
    let header = input.headers()?.clone();
    let (customer_extension, standard): (Vec<_>, Vec<_>) =
        (0..header.len()).partition(|&index| header[index].starts_with(CUSTOMER_EXTENSION_PREFIX));
    // Write declaration
    out.write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")?;
    // Open root tag (Category)
    open_markup(&mut out, category)?;
    // Write one record for each entry in csv
    let mut record = csv::StringRecord::new();
    // We reuse with buffer to avoid allocations in case we need to escape character data.
    let mut char_data_buf = String::new();
    while input.read_record(&mut record)? {
        write_record(
            &mut out,
            &Record {
                standard: &standard,
                extensions: &customer_extension,
                tag_names: &header,
                values: &record,
            },
            record_type.as_str(),
            &mut char_data_buf
        )?;
    }
    // Close root tag (Category)
    out.write_all(b"\n")?;
    close_markup(&mut out, category)?;
    Ok(())
}

fn open_markup<W>(mut out: W, name: &str) -> io::Result<()>
where
    W: io::Write,
{
    out.write_all(b"<")?;
    out.write_all(name.as_bytes())?;
    out.write_all(b">")?;
    Ok(())
}

fn write_record<W>(mut out: W, record: &Record<'_>, record_type: &str, buf: &mut String) -> io::Result<()>
where
    W: io::Write,
{
    out.write_all(b"\n\t")?;
    open_markup(&mut out, record_type)?;
    for (name, value) in record.standard() {
        out.write_all(b"\n\t\t")?;
        open_markup(&mut out, name)?;
        out.write_all(escape_char_data(value, buf).as_bytes())?;
        close_markup(&mut out, name)?;
    }
    // customer extensions
    let mut extensions = record.extensions().peekable();
    if extensions.peek().is_some() {
        out.write_all(b"\n\t\t")?;
        open_markup(&mut out, "CustomerExtensions")?;
        for (name, value) in record.extensions() {
            out.write_all(b"\n\t\t\t")?;
            open_markup(&mut out, name)?;
            out.write_all(escape_char_data(value, buf).as_bytes())?;
            close_markup(&mut out, name)?;
        }
        out.write_all(b"\n\t\t")?;
        close_markup(&mut out, "CustomerExtensions")?;
    }

    out.write_all(b"\n\t")?;
    close_markup(&mut out, record_type)?;
    Ok(())
}

fn close_markup<W>(mut out: W, name: &str) -> io::Result<()>
where
    W: io::Write,
{
    out.write_all(b"</")?;
    out.write_all(name.as_bytes())?;
    out.write_all(b">")?;
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
