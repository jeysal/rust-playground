use std::io;

fn main() {
  println!("Please enter a temperature in Fahrenheit:");

  let mut fahrenheit = String::new();
  io::stdin()
    .read_line(&mut fahrenheit)
    .expect("Failed to read Fahrenheit temperature");
  let fahrenheit: f32 = fahrenheit
    .trim()
    .parse()
    .expect("Your input was not a number");

  println!("{}°F = {}°C", fahrenheit, (fahrenheit - 32.0) * 5.0 / 9.0);
}
