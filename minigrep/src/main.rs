use std::env;
use std::process;

extern crate minigrep;
use minigrep::run;

fn main() {
  match env::args().collect::<Vec<_>>().as_slice() {
    [_, pattern, filename] => if let Err(e) = run(pattern, filename) {
      eprintln!("Error: {}", e);
      process::exit(1);
    },
    _ => {
      eprintln!("Usage: minigrep <pattern> <filename>");
      process::exit(2);
    }
  }
}
