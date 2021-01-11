use ssvm_process_interface::Command;
use ssvm_wasi_helper::ssvm_wasi_helper::_initialize;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn ocr(data: &[u8], language: &str) -> String {
    _initialize();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut guest_temp_input_filename = String::from("/");
    guest_temp_input_filename.push_str(&now.as_millis().to_string());
    let copy_of_guest_temp_input_filename = guest_temp_input_filename.clone();
    let mut pos = 0;
    let mut buffer = File::create(guest_temp_input_filename).unwrap();
    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..]).unwrap();
        pos += bytes_written;
    }
    let mut host_temp_input_filename = String::from("/tmp/");
    host_temp_input_filename.push_str(&now.as_millis().to_string());
    let copy_of_host_temp_input_filename = host_temp_input_filename.clone();

    let mut cmd = Command::new("tesseract");
    cmd.arg(&copy_of_host_temp_input_filename)
        .arg("stdout")
        .arg("--dpi")
        .arg("70")
        .arg("-l")
        .arg(language);

    let out = cmd.output();
    fs::remove_file(&copy_of_guest_temp_input_filename).unwrap();
    str::from_utf8(&out.stdout).unwrap().to_string()
}

#[wasm_bindgen]
pub fn translate(data: &str, translation: &str) -> String {
    let mut translation_param = String::from(":");
    translation_param.push_str(translation);
    let mut cmd = Command::new("trans");
    cmd.arg("-b").arg(translation_param).arg(data);
    let out = cmd.output();
    str::from_utf8(&out.stdout).unwrap().to_string()
}
