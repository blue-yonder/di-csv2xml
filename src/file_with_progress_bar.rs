use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::{Read, Result};

/// File wrapper displays a progress bar which gets updated depending on the cursor position.
pub struct FileWithProgressBar {
    file: File,
    pbar: ProgressBar,
}

impl FileWithProgressBar {
    pub fn new(file: File) -> Result<Self> {
        let len = file.metadata()?.len();
        let pbar = ProgressBar::new(len);
        let fmt = "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})";
        pbar.set_style(
            ProgressStyle::default_bar()
                .template(fmt)
                .progress_chars("#>-"),
        );
        Ok(FileWithProgressBar { file, pbar })
    }
}

impl Read for FileWithProgressBar {
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
        let inc = self.file.read(buffer)?;
        self.pbar.inc(inc as u64);
        Ok(inc)
    }
}
