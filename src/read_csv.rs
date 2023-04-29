use std::io;

pub const CUSTOMER_EXTENSION_PREFIX: &str = "CUEX_";

/// Holds all the state we need to parse the input csv line by line.
pub struct CsvSource<R> {
    reader: csv::Reader<R>,
    /// Header of the csv file containing the names of the columns.
    header: csv::StringRecord,
    /// Current record which has been parsed from csv and ist to be written as XML.
    current_record: csv::StringRecord,
    /// Indices of standard records within csv.
    standard: Vec<usize>,
    /// Indices of extension records within csv. Columns with a column name starting with `CUEX_`
    /// are considered non standard extensions
    extensions: Vec<usize>,
}

impl<R: io::Read> CsvSource<R> {
    /// Creates a new `CsvSource` from any Read and a delimiter.
    pub fn new(input: R, delimiter: u8) -> io::Result<Self> {
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(delimiter)
            .from_reader(input);
        let header = reader.headers()?.clone();
        let (extensions, standard): (Vec<_>, Vec<_>) = (0..header.len())
            .partition(|&index| header[index].starts_with(CUSTOMER_EXTENSION_PREFIX));
        Ok(CsvSource {
            reader,
            header,
            standard,
            extensions,
            current_record: csv::StringRecord::new(),
        })
    }

    /// Return the next Record of the csv or `None`.
    pub fn read_record(&mut self) -> io::Result<Option<Record>> {
        Ok(if self.reader.read_record(&mut self.current_record)? {
            Some(Record {
                standard: &self.standard,
                extensions: &self.extensions,
                tag_names: &self.header,
                values: &self.current_record,
            })
        } else {
            None
        })
    }
}

/// Represents one data record (i.e. a single line of csv to be converted into an XML tag).  
pub struct Record<'a> {
    /// Indices of standard records within csv
    standard: &'a [usize],
    /// Indices of extension records within csv. These will show up within the
    /// `<CustomerExtensions>` tag within the XML.
    extensions: &'a [usize],
    /// Header of csv provides tag names
    tag_names: &'a csv::StringRecord,
    /// Csv row which provides the values of this record
    values: &'a csv::StringRecord,
}

impl<'a> Record<'a> {
    /// Returns an iterator over all standard tags. `Item = (tag_name, value)`
    pub fn standard(&self) -> impl Iterator<Item = (&str, &str)> {
        self.field_it(self.standard, 0)
    }

    /// Returns an iterator over all customer extension tags. `Item = (tag_name, value)`
    pub fn extensions(&self) -> impl Iterator<Item = (&str, &str)> {
        // This helps us to cut of the leading 'CUEX_' prefix from tag names
        let skip = CUSTOMER_EXTENSION_PREFIX.len();
        self.field_it(self.extensions, skip)
    }

    fn field_it(&'a self, fields: &'a [usize], skip: usize) -> impl Iterator<Item = (&str, &str)> {
        fields
            .iter()
            .map(move |&index| (&self.tag_names[index][skip..], &self.values[index]))
            // Empty strings are treated as null and will not be rendered in XML
            .filter(|&(_, v)| !v.is_empty())
    }
}
