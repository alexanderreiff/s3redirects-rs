use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct Writer {
    outfile: String,
}

impl Writer {
    pub fn new(outfile: &str) -> Self {
        Self {
            outfile: outfile.to_string(),
        }
    }

    pub fn write(&self, contents: &str) -> Result<(), Box<Error>> {
        let path = Path::new(&self.outfile);

        let mut file = File::create(&path)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }
}
