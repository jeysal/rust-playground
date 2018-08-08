fn main() {
  println!("{}", fib(42));
}

// plain recursion
//fn fib(n: u32) -> u32 {
//  match n {
//    0 | 1 => n,
//    _ => fib(n - 2) + fib(n - 1),
//  }
//}

// recursion with memoization
//fn fib(n: u32) -> u32 {
//  fib_mem(n as usize, &mut Vec::new())
//}
//fn fib_mem(n: usize, mem: &mut Vec<Option<u32>>) -> u32 {
//  match mem.get(n) {
//    Some(&Some(r)) => r,
//    Some(None) | None => {
//      let r = match n {
//        0 | 1 => n as u32,
//        _ => fib_mem(n - 2, mem) + fib_mem(n - 1, mem),
//      };
//      let mem_len = mem.len();
//      if n >= mem_len {
//        mem.resize(n + 1, None)
//      }
//      mem[n] = Some(r);
//      r
//    }
//  }
//}

// iteration
fn fib(n: u32) -> u32 {
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
