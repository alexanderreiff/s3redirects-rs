use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

type EmptyResult = Result<(), Box<Error>>;

pub fn write_conf(outfile: &str, contents: &str) -> EmptyResult {
    let path = Path::new(outfile);

    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(err) => return Err(From::from(err)),
    };

    match file.write_all(contents.as_bytes()) {
        Ok(_) => Ok(()),
        Err(err) => return Err(From::from(err)),
    }
}
