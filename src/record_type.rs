use strum_macros::{AsRefStr, EnumString};

/// Type which can hold all possible values for a record type
#[derive(Debug, EnumString, AsRefStr, Clone, Copy)]
pub enum RecordType {
    /// Represents a new or updated record
    Record,
    /// Indicates that a similar record should be deleted
    DeleteRecord,
    /// Indicates that all similar records with this attributes should be deleted
    DeleteAllRecords,
}

impl RecordType {
    /// Same as AsRef<str>::as_ref but with a name which conveys the intention clearer
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}
