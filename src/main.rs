extern crate beep;
extern crate dimensioned;
#[macro_use] extern crate structopt;

use beep::beep;
use std::error::Error;
use std::path::PathBuf;
use std::fs::OpenOptions;
use structopt::StructOpt;
use std::io::{Read, BufRead, BufReader};
use dimensioned::si;

/// Command-line options for this utility.
#[derive(StructOpt)]
struct Options {
  /// Activate raw mode.
  /// In raw mode, the input is interpreted as a binary stream of
  /// 64-bit floating point numbers.
  #[structopt(short = "r", long = "raw")]
  raw: bool,
  /// Input file.
  /// In normal operation, this file is read in text mode, i.e., as
  /// a line-delimited stream of decimal numbers.
  #[structopt(name = "FILE", parse(from_os_str))]
  input: PathBuf,
}

/// In normal (text) operation, the utility reads line-delimited,
/// ASCII floating point numbers from an input.
fn read_lines<R: BufRead>(input: R) -> Result<(), Box<Error>> {
  for line in input.lines() {
    let frequency = line?.parse::<f32>()?;
    beep((frequency as f64) * si::HZ);
  }
  Ok(())
}

/// In raw operation, the utility reads a binary stream of 64 bit
/// floating point numbers from an input.
fn read_bytes<R: Read>(mut input: R) -> Result<(), Box<Error>> {
  // stack-allocate a buffer of 4 bytes
  let mut buffer = [0; 4];
  // read exactly 8 bytes
  while let Ok(()) = input.read_exact(&mut buffer) {
    // interpret the bytes as an 8 byte float representing frequency
    let frequency : f32 = unsafe{std::mem::transmute(buffer)};
    // interpret the frequency as hertz, and beep it
    beep(frequency as f64 * si::HZ);
  }
  Ok(())
}

fn run(options: Options) -> Result<(), Box<Error>> {
  // open the input file in read-only mode
  let input = OpenOptions::new().read(true).open(options.input)?;

  // read the input file in the appropriate mode
  match options.raw {
    true  => read_bytes(input),
    false => read_lines(BufReader::new(input)),
  }
}

fn main() {
  // parse command line options
  let options = Options::from_args();

  // run and, if necessary, print error message to stderr
  if let Err(error) = run(options) {
    eprintln!("Error: {}", error);
    std::process::exit(1);
  }
}