mod num_stats;

fn main() {
  let numbers = vec![0, 1, 2, 2, 4, 4, 4, 6];
  println!("{:?}", num_stats::num_stats(numbers));
}
