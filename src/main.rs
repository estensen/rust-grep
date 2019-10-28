#[macro_use]
extern crate clap;
extern crate colored;
extern crate regex;

use clap::App;
use colored::*;
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn read_lines(pattern: String, filename: String, count: bool, color: bool) {
    let f = File::open(filename).expect("file not found");
    let buffered = BufReader::new(f);

    let re = Regex::new(&pattern).unwrap();
    let mut matched_lines = 0;

    for line in buffered.lines() {
        let line_string = line.expect("coutld not read line");
        if re.is_match(&line_string) {
            if count {
                matched_lines += 1;
            } else {
                if color {
                    let colored = re.replace_all(&pattern, &pattern.red());
                    println!("{}", &colored);
                } else {
                    println!("{}", &line_string);
                }
            }
        }
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
    let color = matches.is_present("color");

    read_lines(pattern, filename, count, color);
}

