# gabc-converter
gabc-converter is a command-line tool to convert gabc files to JSON or Lilypond. It provides a basic interface to functions in the [gabc-parser](https://github.com/saybaar/gabc-parser) Rust library.

## Build and installation
To build and install this program, you will need the Rust programming language and its build tool, cargo. See [rustup.rs](https://rustup.rs/) to install them. Rust will require

Clone this repository with `git clone https://github.com/saybaar/gabc-converter.git`.

From the project directory, run `cargo build` to build the application without installing (the gabc-converter binary will appear in target/debug/ and can be run from there), or `cargo install` to build and install it (see [here](https://doc.rust-lang.org/book/second-edition/ch14-04-installing-binaries.html) for help with the "cargo install" command; in particular, it requires that `$HOME/.cargo/bin` is in your `$PATH` environment variable).  

This tool has only been tested on Linux systems. If you would like to use it on another OS and run into problems, please open an issue!

## Usage
```
gabc-converter <TARGET> [-i INPUT] [-o OUTPUT]
```
`<TARGET>` can be `json`, `lilypond`, or `debug-print` (which prints the gabc input's parse tree for analysis and debugging).  
Input and output files may optionally be specified with the `-i` and `-o` flags. If not specified, input will default to STDIN and output to STDOUT.

### Usage examples
With -i and -o flags:
```
gabc-converter lilypond -i ./examples/populus_sion.gabc -o ./populus_sion.ly
```
On a Linux system, using I/O redirection with STDIN and STDOUT instead of flags:
```
./examples/populus_sion.gabc > gabc-converter lilypond > ./populus_sion.ly
```
Without an -o flag or output redirection, the program will simply print the output to STDOUT:
```
gabc-converter lilypond -i ./examples/populus_sion.gabc
```

## Example gabc files
The gabc files in /examples should all play nicely with this program. populus_sion.gabc is the canonical example in [the gabc documentation](http://gregorio-project.github.io/gabc/details.html), and the other examples are from [gregobase](https://gregobase.selapa.net/).

## Limitations
This tool is under development and doesn't yet recognize all gabc syntax. Major gabc features not yet supported include:
* Accidentals and flat clefs (e.g. "cb2")
* gabc comments
* Text above or below the staff

Auto-generated Lilypond output may require adjustments, especially to the transposition range (which is c -> c' by default) or to correct formatting and alignment of lyrics.  

## Resources
### Other gabc tools
* [gabctk](https://github.com/jperon/gabctk): A toolkit for gabc, including conversion to Lilypond, abc, midi, and others. Written in Python and documented (only) in French.
* [gabc2mid](https://github.com/jperon/gabc2mid): An earlier iteration of gabctk with midi conversion only. Written in Python with English documentation available.
* [gabc-to-ly](https://github.com/ahinkley/gabc-to-ly): Conversion from gabc to Lilypond via a .csv file, which can be manually edited to add organ accompaniment chords. Written in Python.
* [lygre](https://github.com/igneus/lygre): Conversion from gabc to Lilypond. Written in Ruby.

### Gregorio and gabc
gabc is part of [the Gregorio project](http://gregorio-project.github.io/index.html), which also includes the GregorioTeX tool for rendering gabc. GregorioTeX can be [installed locally](http://gregorio-project.github.io/installation.html), which may be a complicated process. Web renderers like [run.gregoriochant.org](http://run.gregoriochant.org) are a simpler option.  
* [Gregorio on GitHub](https://github.com/gregorio-project/gregorio)
* [gabc documentation](http://gregorio-project.github.io/gabc/index.html)

### Lilypond
Lilypond files can be rendered with `lilypond file.ly` on a [local installation](http://lilypond.org/download.html), or through a web renderer like [lilybin](http://lilybin.com/). 
* [Lilypond documentation](http://lilypond.org/manuals.html)
_______________
Copyright (c) 2018 Lydia Simmons  
This software is licensed under the GNU General Public License v3.0. See the LICENSE file in this distribution for license terms.
