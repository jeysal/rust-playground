extern crate regex;

use regex::Regex;
use std::error::Error;
use std::fs;

pub fn run(pattern: &str, filename: &str) -> Result<(), Box<Error>> {
  let regex = Regex::new(pattern)?;
  let text = fs::read_to_string(filename)?;

  for line in search(regex, text.as_str()) {
    println!("{}", line)
  }

  Ok(())
}

pub fn search<'a>(regex: Regex, text: &'a str) -> Vec<&'a str> {
  text.lines().filter(|line| regex.is_match(line)).collect()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn empty_text() {
    let regex = Regex::new(".*").unwrap();
    let text = "";

    assert_eq!(Vec::<&str>::new(), search(regex, text))
  }

  #[test]
  fn no_match() {
    let regex = Regex::new("asdf").unwrap();
    let text = "fdsa";

    assert_eq!(Vec::<&str>::new(), search(regex, text))
  }

  #[test]
  fn one_match() {
    let regex = Regex::new("duct").unwrap();
    let text = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(regex, text))
  }

  #[test]
  fn all_match() {
    let regex = Regex::new("t.").unwrap();
    let text = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(
      vec!["Rust:", "safe, fast, productive.", "Pick three."],
      search(regex, text)
    )
  }
}
