//Copyright (c) 2018 Lydia Simmons
//This software is licensed under the GNU General Public License v3.0.
//See the LICENSE file in this distribution for license terms. 

extern crate clap;
extern crate gabc_parser;
extern crate serde_json;
use clap::{App, Arg};
use gabc_parser::{parse_file, GabcFile};
use std::fs::File;
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
        Some(s) => Box::new(File::open(s).expect("Error opening file")),
        None => Box::new(std::io::stdin()),
    };

    let mut text = String::new();
    input_source
        .read_to_string(&mut text)
        .expect("Error reading file");

    let output: String = match matches.value_of("TARGET") {
        Some("json") => GabcFile::new(&text).as_json(),
        Some("lilypond") => GabcFile::new(&text).as_lilypond(),
        Some("debug-print") => gabc_parser::debug_print(parse_file(&text)),
        _ => panic!("Impossible target"),
    };

    let mut output_source: Box<Write> = match matches.value_of("output") {
        Some(s) => Box::new(File::create(s).expect("Error opening file")),
        None => Box::new(std::io::stdout()),
    };

    output_source.write(output.as_bytes())?;
    Ok(())
}
