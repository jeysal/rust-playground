static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn pig_latin(sentence: &str) -> String {
  sentence
    .split_whitespace()
    .map(|word| {
      let mut chars = word.chars();
      let first_char = chars.next().unwrap();
      if VOWELS.contains(&first_char) {
        format!("{}hay", word)
      } else {
        let rest = chars.collect::<String>();
        format!("{rest}{first}ay", rest = rest, first = first_char)
      }
    })
    .collect::<Vec<_>>()
    .join(" ")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn starts_with_vowel() {
    assert_eq!("applehay", pig_latin("apple"))
  }

  #[test]
  fn starts_with_consonant() {
    assert_eq!("ineapplepay", pig_latin("pineapple"))
  }

  #[test]
  fn multiple_words() {
    assert_eq!("anhay applehay", pig_latin("an apple"))
  }

  #[test]
  fn empty() {
    assert_eq!("", pig_latin(""))
  }
}
