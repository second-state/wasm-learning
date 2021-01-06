use std::fs;
use std::str;
use std::fs::File;
use std::io::{Write, Read};
use wasm_bindgen::prelude::*;
use rust_process_interface_library::Command;
use ssvm_wasi_helper::ssvm_wasi_helper::_initialize;

#[wasm_bindgen]
pub fn ocr(img_buf: &[u8]) -> String {
  _initialize();
  // Create temp file to store image
  let path = String::from("temp_input_file");
  let mut temp_file = File::create(&path).unwrap();
  temp_file.write_all(img_buf).unwrap();
  // Create temp filename for output
  let mut output_path: String = "temp_output_file".to_owned();
  let output_extension: &str = ".txt";
  // Execute OCR
  let mut cmd = Command::new("tesseract");
  cmd.arg(&path)
  .arg(&output_path);

  let out = cmd.output();
  if out.status != 0 {
      println!("Code: {}", out.status);
      println!("STDERR: {}", str::from_utf8(&out.stderr).unwrap());
      println!("STDOUT: {}", str::from_utf8(&out.stdout).unwrap());
      output_path.push_str(output_extension);
      fs::remove_file(output_path).expect("Unable to delete");
      return out.status.to_string();
  }
  output_path.push_str(output_extension);
  let mut f = File::open(&output_path).unwrap();
  let mut s = String::new();
  match f.read_to_string(&mut s) {
    Ok(_) => {
      fs::remove_file(output_path).expect("Unable to delete");
      return s;
    },
    Err(e) => e.to_string(),
  }
}
