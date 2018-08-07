extern crate clap;
extern crate gabc_parser;
extern crate serde_json;
use gabc_parser::{GabcFile, GABCParser, Rule, parse_file};
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let matches = App::new("gabc-converter")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use (blank for STDIN)")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name("target")
                .long("target")
                .help("Conversion target (\"lilypond\", \"json\", or \"debug-print\")")
                .required(true)
                .takes_value(true)
                .possible_values(&["lilypond", "json", "debug-print"]),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .long("output")
                .help("Sets the output file to use (blank for STDOUT)")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    let mut input_source: Box<Read> = match matches.value_of("INPUT") {
        Some(s) => Box::new(File::open(s).expect("Error opening file")),
        None => Box::new(std::io::stdin()),
    };

    let mut text = String::new();
    input_source.read_to_string(&mut text).expect("Error reading file");

    let output: String = match matches.value_of("target") {
        Some("json") => GabcFile::new(&text).as_json(),
        Some("lilypond") => GabcFile::new(&text).as_lilypond(),
        Some("debug-print") => gabc_parser::debug_print(parse_file(&text)),
        _ => panic!("Impossible target"),
    };

    let mut output_source: Box<Write> = match matches.value_of("OUTPUT") {
        Some(s) => Box::new(File::create(s).expect("Error opening file")),
        None => Box::new(std::io::stdout()),
    };

    output_source.write(output.as_bytes())?;
    Ok(())
}
