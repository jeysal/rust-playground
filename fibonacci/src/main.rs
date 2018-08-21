#![allow(dead_code)]

fn main() {
  println!("{}", fib(42));
}

fn fib(n: u32) -> u32 {
  fib_iter(n)
}

fn fib_recursive(n: u32) -> u32 {
  match n {
    0 | 1 => n,
    _ => fib_recursive(n - 2) + fib_recursive(n - 1),
  }
}

fn fib_memoization(n: u32) -> u32 {
  fib_mem(n as usize, &mut Vec::new())
}
fn fib_mem(n: usize, mem: &mut Vec<Option<u32>>) -> u32 {
  match mem.get(n) {
    Some(&Some(r)) => r,
    Some(None) | None => {
      let r = match n {
        0 | 1 => n as u32,
        _ => fib_mem(n - 2, mem) + fib_mem(n - 1, mem),
      };
      let mem_len = mem.len();
      if n >= mem_len {
        mem.resize(n + 1, None)
      }
      mem[n] = Some(r);
      r
    }
  }
}

fn fib_iter(n: u32) -> u32 {
  if n == 0 {
    return 0;
  }

  let mut i = 1;
  let mut prev = 0;
  let mut curr = 1;

  while i < n {
    i += 1;
    curr += prev;
    prev = curr - prev;
  }
  curr
}

#[cfg(test)]
mod tests {
  use super::fib;

  #[test]
  fn fib_0() {
    assert_eq!(0, fib(0))
  }

  #[test]
  fn fib_1() {
    assert_eq!(1, fib(1))
  }

  #[test]
  fn fib_2() {
    assert_eq!(1, fib(2))
  }

  #[test]
  fn fib_10() {
    assert_eq!(55, fib(10))
  }
}
