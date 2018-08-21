fn largest<T: PartialOrd>(list: &[T]) -> &T {
  if list.len() == 0 {
    panic!("list must not be empty");
  }

  let mut largest = &list[0];

  for item in list.iter() {
    if item > largest {
      largest = &item;
    }
  }

  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = largest(&char_list);
  println!("The largest char is {}", result);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic(expected = "must not be empty")]
  fn empty_not_allowed() {
    largest::<()>(&[]);
  }

  #[test]
  fn single_element() {
    assert_eq!(1, *largest(&[1]))
  }

  #[test]
  fn int() {
    assert_eq!(1337, *largest(&[42, -42, 1337, 0]))
  }

  #[test]
  fn float() {
    assert_eq!(0.3, *largest(&[-0.2, 0.3, 0.0, 0.1]))
  }
}
