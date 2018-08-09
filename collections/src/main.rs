mod num_stats;
mod pig_latin;

fn main() {
  println!("{:?}", num_stats::num_stats(vec![0, 1, 2, 2, 4, 4, 4, 6]));
  println!(
    "{}",
    pig_latin::pig_latin("This is just a test sentence in English.")
  );
}
