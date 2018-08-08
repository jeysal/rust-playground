use std::collections::HashMap;

#[derive(Debug)]
pub struct NumStats {
  avg: f64,
  med: f64,
  mode: i32,
}
pub fn num_stats(numbers: Vec<i32>) -> Option<NumStats> {
  if numbers.is_empty() {
    None
  } else {
    Some(NumStats {
      avg: avg(&numbers),
      med: med(&numbers),
      mode: mode(&numbers),
    })
  }
}

fn avg(numbers: &Vec<i32>) -> f64 {
  numbers.iter().sum::<i32>() as f64 / numbers.len() as f64
}
fn med(numbers: &Vec<i32>) -> f64 {
  let mut sorted = numbers.clone();
  sorted.sort_unstable();
  let len = numbers.len();
  if len % 2 == 0 {
    (sorted[len / 2 - 1] + sorted[len / 2]) as f64 / 2.0
  } else {
    sorted[len / 2] as f64
  }
}
fn mode(numbers: &Vec<i32>) -> i32 {
  *(numbers
    .iter()
    .fold(HashMap::<i32, i32>::new(), |mut counts, num| {
      {
        let count = counts.entry(*num).or_insert(0);
        *count += 1;
      }
      counts
    })
    .iter()
    .max_by_key(|(_, &count)| count)
    .map(|(num, _)| num)
    .expect("cannot find most common number to determine the mode"))
}
