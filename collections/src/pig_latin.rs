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
