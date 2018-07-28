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
use redirect_rule::{build_conf, RedirectRule};
use std::error::Error;
use std::io;
use std::process;
use writer::write_conf;

fn main() {
    // fetch file from S3
    // parse and get set
    // map set to config file
    // print Etag
    // exit
    let matches = build_cli().get_matches();
    let outfile = matches.value_of("out").unwrap();
    let etag = matches.value_of("etag");

    // let file = fetcher::fetch(etag);
    // if file.not_modified() {
    // return exit_from_stale_file();
    // }
    // let parser = Parser::new(file.reader());
    let input = Box::new(io::stdin());
    let parser = Parser::new(input);

    match parser.get_rules() {
        Ok(rules) => generate_conf(outfile, rules /*, file*/),
        Err(err) => exit_from_parser(err),
    }
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

fn generate_conf(outfile: &str, rules: Vec<RedirectRule> /*, file: File*/) -> () {
    let conf = build_conf(rules);

    if let Err(err) = write_conf(outfile, &conf) {
        exit_from_builder(err);
    } else {
        // println!("{}", file.etag());
    }
}

fn exit_from_stale_file() {
    process::exit(3)
}

fn exit_from_parser(err: Box<Error>) {
    panic!("Error during CSV parsing: {}", err)
}

fn exit_from_builder(err: Box<Error>) {
    panic!("Error during file generation: {}", err);
}
