//Copyright (c) 2018 Lydia Simmons
//This software is licensed under the GNU General Public License v3.0.
//See the LICENSE file in this distribution for license terms.

extern crate clap;
extern crate gabc_parser;
extern crate serde_json;
use clap::{App, Arg};
use gabc_parser::{GabcFile, parse_gabc};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let matches = App::new("gabc-converter")
        .version("0.1")
        .author("Lydia Simmons <saybaar@gmail.com>")
        .about("Converts a gabc file to JSON or Lilypond")
        .arg(
            Arg::with_name("TARGET")
                .long("target")
                .help("Conversion target")
                .required(true)
                .possible_values(&["lilypond", "json", "debug-print"])
                .index(1),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .help("Sets the input file to use (blank for STDIN)")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("Sets the output file to use (blank for STDOUT)")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    let mut input_source: Box<Read> = match matches.value_of("input") {
        Some(s) => match File::open(s) {
            Ok(f) => Box::new(f),
            Err(e) => {
                println!("Error opening input file: {}", e);
                std::process::exit(1);
            },
        },
        None => Box::new(std::io::stdin()),
    };

    let mut text = String::new();
    input_source
        .read_to_string(&mut text)
        .expect("Error reading file");

    let output: String = match matches.value_of("TARGET") {
        Some("json") => GabcFile::new(&text).as_json(),
        Some("lilypond") => GabcFile::new(&text).as_lilypond(),
        Some("debug-print") => gabc_parser::debug_print(parse_gabc(&text, gabc_parser::Rule::file)),
        _ => panic!("Impossible target"),
    };

    let mut output_source: Box<Write> = match matches.value_of("output") {
        Some(s) => {
            match OpenOptions::new().write(true).create_new(true).open(s) {
                Ok(f) => Box::new(f),
                Err(e) => match e.kind() {
                    io::ErrorKind::AlreadyExists => ask_to_replace(&s),
                    _ => {
                        println!("Error opening output file: {}", e);
                        std::process::exit(1)
                    },
                },
            }
        },
        None => Box::new(std::io::stdout()),
    };

    output_source.write(output.as_bytes())?;
    Ok(())
}

fn ask_to_replace(dest: &str) -> Box<Write> {
    loop {
        println!("Replace existing file {} ? [y/N]", dest);
        let mut s = String::new();
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut s).expect("error reading line");
        match s.trim().as_ref() {
            "Y" | "y" => return Box::new(File::create(dest).expect("error replacing file")),
            "N" | "n" | "" => {
                println!("Exiting.");
                std::process::exit(0);
            },
            _ => (),
        }
    }
}
