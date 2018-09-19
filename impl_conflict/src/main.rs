use std::convert;

fn main() {
  let s = String::from("fdsa");
  let a: A = s.into();
  println!("{:?}", a);
}

#[derive(Debug)]
struct A {
  s: String,
}

impl convert::From<String> for A {
  fn from(s: String) -> A {
    A {
      s: format!("asdf{}", s),
    }
  }
}

// impl convert::Into<A> for String {
//   fn into(self: String) -> A {
//     A { s: self }
//   }
// }
