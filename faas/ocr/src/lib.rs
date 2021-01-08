use rust_process_interface_library::Command;
use ssvm_wasi_helper::ssvm_wasi_helper::_initialize;
use std::fs::File;
use std::io::Read;
use std::str;
use std::io::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn ocr(data: &[u8]) -> String {
    _initialize();
    let mut pos = 0;
    let mut buffer = File::create("/temp_input.png").unwrap();

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..]).unwrap();
        pos += bytes_written;
    }
    let mut cmd = Command::new("tesseract");
    cmd.arg("/temp_input.png").arg("stdout");
    let out = cmd.output();
    println!("Code: {}", out.status);
    println!("STDERR: {}", str::from_utf8(&out.stderr).unwrap());
    println!("STDOUT: {}", str::from_utf8(&out.stdout).unwrap());
    str::from_utf8(&out.stdout).unwrap().to_string()
}
