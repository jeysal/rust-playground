extern crate hello;

use hello::ThreadPool;

use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  let pool = ThreadPool::new(7);

  for stream in listener.incoming().take(10) {
    pool.execute(|| handle_connection(stream.unwrap()));
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 512];
  stream.read(&mut buffer).unwrap();

  let get_sleep = b"GET /sleep HTTP/1.1\r\n";
  if buffer.starts_with(get_sleep) {
    thread::sleep(Duration::from_secs(5));
  }

  let get_root = b"GET / HTTP/1.1\r\n";
  let resp = if buffer.starts_with(get_sleep) || buffer.starts_with(get_root) {
    format!(
      "HTTP/1.1 200 OK\r\nCache-Control: no-cache, no-store\r\n{}",
      fs::read_to_string("hello.html").unwrap()
    )
  } else {
    String::from("HTTP/1.1 400 NOT FOUND\r\n\r\n")
  };

  stream.write(resp.as_bytes()).unwrap();
  stream.flush().unwrap();
}
