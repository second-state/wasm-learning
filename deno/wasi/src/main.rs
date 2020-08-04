use std::fs;
use std::fs::File;
use std::io::{Write, Read};
use std::env;

fn main() {
  println!("The env vars are as follows.");
  for (key, value) in env::vars() {
    println!("{}: {}", key, value);
  }

  create_file("/hello.txt", "Hello WASI");
  println!("{}", read_file("/hello.txt") );
  del_file("/hello.txt");
}

fn create_file(path: &str, content: &str) {
  let mut output = File::create(path).unwrap();
  output.write_all(content.as_bytes()).unwrap();
}

fn read_file(path: &str) -> String {
  let mut f = File::open(path).unwrap();
  let mut s = String::new();
  match f.read_to_string(&mut s) {
    Ok(_) => s,
    Err(e) => e.to_string(),
  }
}

fn del_file(path: &str) {
  fs::remove_file(path).expect("Unable to delete");
}
