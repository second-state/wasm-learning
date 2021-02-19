use ssvm_process_interface::Command;
use std::io::prelude::*;
use std::str;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn detection_classification_and_recognition(data: &str) -> String {
    let mut cmd = Command::new("paddleocr");
    cmd.arg("--image_dir")
        .arg(data)
        .arg("--use_angle_cls")
        .arg("true")
        .arg("--lang")
        .arg("en");
    let out=cmd.output();
    let test_out:String=str::from_utf8(&out.stdout).unwrap().to_string();
    //println!("{:?}\n",test_out);
    str::from_utf8(&out.stdout).unwrap().to_string()
}
#[wasm_bindgen]
pub fn detection_and_recognition(data: &str) -> String {
    let mut cmd = Command::new("paddleocr");
    cmd.arg("--image_dir")
        .arg(data)
        .arg("--lang")
        .arg("en");
    let out=cmd.output();
    let test_out:String=str::from_utf8(&out.stdout).unwrap().to_string();
    //println!("{:?}\n",test_out);
    str::from_utf8(&out.stdout).unwrap().to_string()
}
#[wasm_bindgen]
pub fn classification_and_recognition(data: &str) -> String {
    let mut cmd = Command::new("paddleocr");
    cmd.arg("--image_dir")
        .arg(data)
        .arg("--use_angle_cls")
        .arg("true")
        .arg("--det")
        .arg("false")
        .arg("--lang")
        .arg("en");
    let out=cmd.output();
    let test_out:String=str::from_utf8(&out.stdout).unwrap().to_string();
    //println!("{:?}\n",test_out);
    str::from_utf8(&out.stdout).unwrap().to_string()
}
#[wasm_bindgen]
pub fn only_detection(data: &str) -> String {
    let mut cmd = Command::new("paddleocr");
    cmd.arg("--image_dir")
        .arg(data)
        .arg("--rec")
        .arg("false");
    let out=cmd.output();
    let test_out:String=str::from_utf8(&out.stdout).unwrap().to_string();
    //println!("{:?}\n",test_out);
    str::from_utf8(&out.stdout).unwrap().to_string()
}
#[wasm_bindgen]
pub fn only_recognition(data: &str) -> String {
    let mut cmd = Command::new("paddleocr");
    cmd.arg("--image_dir")
        .arg(data)
        .arg("--det")
        .arg("false")
        .arg("--lang")
        .arg("en");
    let out=cmd.output();
    let test_out:String=str::from_utf8(&out.stdout).unwrap().to_string();
    //println!("{:?}\n",test_out);
    str::from_utf8(&out.stdout).unwrap().to_string()
}
#[wasm_bindgen]
pub fn only_classification(data: &str) -> String {
    let mut cmd = Command::new("paddleocr");
    cmd.arg("--image_dir")
        .arg(data)
        .arg("--use_angle_cls")
        .arg("true")
        .arg("--det")
        .arg("--rec")
        .arg("false");
    let out= cmd.output();
    let test_out:String=str::from_utf8(&out.stdout).unwrap().to_string();
    //println!("{:?}\n",test_out);
    str::from_utf8(&out.stdout).unwrap().to_string()
}
