# gabc-converter
This is a command-line program to convert gabc files to JSON or Lilypond. It provides a basic interface to functions in the (LINK) gabc-parser Rust library.

## Build and installation
To build and install this program, you will need the Rust programming language. To install Rust, see <https://rustup.rs/>.

Once Rust is installed, you can run "cargo test" to test the application, "cargo build" to build it (the binary will appear in target/debug/ and can be run from there), and "cargo install" to install it system-wide (see <https://doc.rust-lang.org/book/second-edition/ch14-04-installing-binaries.html> for more information on the "cargo install" command).

## Usage
gabc-converter <TARGET> [-i INPUT] [-o OUTPUT]
<TARGET> is one of json (for JSON output), lilypond (for Lilypond output), or debug-print (which will print the gabc input's parse tree - may be useful for analysis and debugging).
INPUT and OUTPUT files may optionally be specified with the -i and -o flags. If not specified, input will default to STDIN and output to STDOUT.

Usage examples:
With -i and -o flags:
gabc-converter json -i ./examples/ab_ortu_solis.gabc -o ./ab_ortu_solis.ly
On a Linux system, using I/O redirection instead of flags:
./examples/ab_ortu_solis.gabc > gabc-converter json > ./ab_ortu_solis.ly
Without an -o flag or output redirection, the program will simply print the output to STDOUT:
gabc-converter json -i ./examples/ab_ortu_solis.gabc

## Related work
(TODO) may be more mature options for Lilypond conversion.

## Gregorio and gabc
* <https://github.com/gregorio-project/gregorio>
* <http://gregorio-project.github.io/gabc/index.html>

## Lilypond
* <http://lilypond.org>
