extern crate regex;
#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use clap::App;

fn read_lines(pattern: String, filename: String, count: bool) {
    let f = File::open(filename).expect("file not found");
    let buffered = BufReader::new(f);

    let re = Regex::new(&pattern).unwrap();
    let mut matched_lines = 0;

    for line in buffered.lines() {
        let line_string = line.expect("coutld not read line");
        matched_lines += re.find_iter(&line_string).count();
    }
    if count {
        println!("{}", matched_lines);
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let pattern = matches.value_of("pattern").unwrap().to_string();
    let filename = matches.value_of("filename").unwrap().to_string();
    let count = matches.is_present("count");

    read_lines(pattern, filename, count);
}

