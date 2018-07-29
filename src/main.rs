extern crate clap;
extern crate csv;
extern crate futures;
extern crate rusoto_core;
extern crate rusoto_s3;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod parser;
mod redirect_rule;
mod s3_fetcher;
mod writer;

use clap::{App, Arg};
use parser::Parser;
use s3_fetcher::FileError;
use std::process;
use writer::Writer;

fn main() {
    let matches = build_cli().get_matches();
    let outfile = matches.value_of("out").unwrap();
    let etag = matches.value_of("etag");

    let file = s3_fetcher::fetch(etag);
    if let Err(err) = file {
        return handle_file_errors(err);
    }
    let file = file.unwrap();
    let updated_etag = file.etag();
    let reader = file.into_reader();
    let parser = Parser::new(Box::new(reader));

    let rules = parser.get_rules().expect("Error during CSV parsing");
    let conf = redirect_rule::build_conf(&rules);

    let writer = Writer::new(outfile);
    writer.write(&conf).expect("Error during file generation");

    if let Some(etag) = updated_etag {
        println!("{}", etag);
    }
}

fn build_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("s3redirects")
        .version("0.1.0")
        .arg(
            Arg::with_name("out")
                .long("out")
                .short("o")
                .value_name("FILE")
                .help("Nginx configuration output file path")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("etag")
                .long("etag")
                .short("t")
                .value_name("ETAG")
                .help("Etag to use to check for fresh redirects")
                .takes_value(true),
        )
}

fn handle_file_errors(err: FileError) {
    match err {
        FileError::NotFound => panic!("File not found"),
        FileError::NotModified => process::exit(3),
        FileError::Unknown(err) => panic!("Unknown fetch error occurred: {}", err),
    }
}
