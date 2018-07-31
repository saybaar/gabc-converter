extern crate gabc_parser;
extern crate serde_json;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let first = env::args().nth(1).expect("Please supply a filename");
    let mut file = File::open(&first).expect("Error opening file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Error reading file");
    //derived from example from the book:
    let output = gabc_parser::parse_to_struct(&text);
    io::stdout().write(serde_json::to_string(&output).unwrap().as_bytes())?;
    Ok(())
}
