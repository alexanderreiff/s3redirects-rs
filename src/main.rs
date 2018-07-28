extern crate clap;
extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod parser;
mod redirect_rule;
mod writer;

use clap::{App, Arg};
use parser::Parser;
use std::io;
// use std::process;

fn main() {
    // fetch file from S3
    // parse and get set
    // map set to config file
    // print Etag
    // exit
    let matches = build_cli().get_matches();
    let outfile = matches.value_of("out").unwrap();
    let _etag = matches.value_of("etag");

    // let file = fetcher::fetch(etag);
    // if file.not_modified() {
    // return process::exit(3);
    // }
    // let parser = Parser::new(file.reader());
    let input = Box::new(io::stdin());
    let parser = Parser::new(input);

    let rules = parser.get_rules().expect("Error during CSV parsing");
    let conf = redirect_rule::build_conf(&rules);
    writer::write_conf(outfile, &conf).expect("Error during file generation");

    // println!("{}", file.etag());
}

fn build_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("s3redirects")
        .version("0.1.0")
        .arg(
            Arg::with_name("out")
                .short("o")
                .value_name("FILE")
                .help("Nginx configuration output file path")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("etag")
                .short("t")
                .value_name("ETAG")
                .help("Etag to use to check for fresh redirects")
                .takes_value(true),
        )
}
