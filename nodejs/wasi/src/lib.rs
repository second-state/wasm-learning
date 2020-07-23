use wasm_bindgen::prelude::*;
use rand::prelude::*;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::env;
use wasi::*;

fn populate_preopens() {
  extern "C" {
    fn __wasilibc_register_preopened_fd(
      fd : u32,
      path: *const u8,
    ) -> u32;
  }
  static mut populated: bool = false;
  if unsafe { populated } {
    return;
  }
  let mut fd = 0;
  while let Ok(prestat) = unsafe { fd_prestat_get(fd) } {
    match prestat.pr_type {
      PREOPENTYPE_DIR => {
        let path_len = unsafe { prestat.u.dir.pr_name_len };
        let mut path = [0].repeat(path_len + 1);
        path.shrink_to_fit();
        let ptr = path.as_mut_ptr();
        if let Ok(_) = unsafe { fd_prestat_dir_name(fd, ptr, path_len) } {
          unsafe { __wasilibc_register_preopened_fd(fd, ptr) };
        } else {
          break;
        }
      }
      _ => { break; }
    }
    fd += 1;
  }
  unsafe { populated = true };
}

#[wasm_bindgen]
pub fn get_random_i32() -> i32 {
  let x: i32 = random();
  return x;
}

#[wasm_bindgen]
pub fn get_random_bytes() -> Vec<u8> {
  let mut rng = thread_rng();
  let mut arr = [0u8; 128];
  rng.fill(&mut arr[..]);
  return arr.to_vec();
  /*
  let mut vec: Vec<u8> = vec![0; 128];
  getrandom::getrandom(&mut vec).unwrap();
  return vec;
   */
}

#[wasm_bindgen]
pub fn print_random_i32() -> i32 {
  let x: i32 = random();
  println!("Printed from wasm: A new random number is {}", x);
  return x;
}

#[wasm_bindgen]
pub fn print_env() -> i32 {
  println!("The env vars are as follows.");
  for (key, value) in env::vars() {
    println!("{}: {}", key, value);
  }

  println!("The args are as follows.");
  for argument in env::args() {
    println!("{}", argument);
  }

  /*
  match env::var("PATH") {
    Ok(path) => println!("PATH: {}", path),
    Err(e) => println!("Couldn't read PATH ({})", e),
  };
  */

  return 0;
}

#[wasm_bindgen]
pub fn create_file(path: &str, content: &str) -> String {
  populate_preopens();
  let mut output = File::create(path).unwrap();
  output.write_all(content.as_bytes()).unwrap();
  path.to_string()
}


