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

        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(err) => return Err(From::from(err)),
        };

        match file.write_all(contents.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(From::from(err)),
        }
    }
}
