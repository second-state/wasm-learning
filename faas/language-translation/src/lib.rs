use ssvm_process_interface::Command;
use std::str;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn translate(data: &str, translation: &str) -> String {
    let mut translation_param = String::from(":");
    translation_param.push_str(translation);
    let mut cmd = Command::new("trans");
    cmd.arg("-b").arg(translation_param).arg(data);
    let out = cmd.output();
    str::from_utf8(&out.stdout).unwrap().to_string()
}
