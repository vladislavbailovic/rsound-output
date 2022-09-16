pub trait Buffer {
    fn get_buffer(&self) -> &[u8];
}

pub trait OutputRenderer: Buffer {
    fn get_header(&self) -> Option<Vec<u8>>;
    fn get_footer(&self) -> Option<Vec<u8>>;
}

pub trait Writer {
    fn write<T>(&self, renderer: T) -> Result<()>
    where
        T: OutputRenderer + 'static;
}

use std::fs::File;
use std::io::{BufWriter, Result, Write};

pub struct FileWriter {
    fname: String,
}

impl FileWriter {
    pub fn new(fname: &str) -> Self {
        Self {
            fname: fname.into(),
        }
    }

    pub fn open_file(&self) -> Result<BufWriter<File>> {
        Ok(BufWriter::new(File::create(&self.fname)?))
    }
}

impl Writer for FileWriter {
    fn write<T>(&self, renderer: T) -> Result<()> where T: OutputRenderer + 'static {
        let mut p = self.open_file()?;

        let header = &renderer.get_header();
        let footer = &renderer.get_footer();

        if let Some(header) = header {
            let _ = p.write(header)?;
        }
        let _ = p.write(renderer.get_buffer())?;
        if let Some(footer) = footer {
            let _ = p.write(footer)?;
        }

        Ok(())
    }
}


use std::io;

#[derive(Default)]
pub struct StdoutWriter;

impl StdoutWriter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Writer for StdoutWriter {
    fn write<T>(&self, renderer: T) -> Result<()>
    where
        T: OutputRenderer + 'static,
    {
        let mut stdout = io::stdout();
        let header = &renderer.get_header();
        let footer = &renderer.get_footer();

        if let Some(header) = header {
            let _ = stdout.write(header)?;
        }
        let _ = stdout.write(renderer.get_buffer())?;
        if let Some(footer) = footer {
            let _ = stdout.write(footer)?;
        }
        Ok(())
    }
}
