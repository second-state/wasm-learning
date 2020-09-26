use wasm_bindgen::prelude::*;
use rust_process_interface_library::Command;
use serde_json::{from_str, Value};
use std::str;

#[wasm_bindgen]
pub fn infer(image_data: &[u8]) -> String {
    println!("Loading image ...");
    let img = image::load_from_memory(image_data).unwrap().to_rgb();
    let resized = image::imageops::resize(&img, 224, 224, ::image::imageops::FilterType::Triangle);

    println!("Loading model ...");
    let model_data: &[u8] = include_bytes!("mobilenet_v2_1.4_224_frozen.pb");

    println!("Preparing command ...");
    let mut cmd = Command::new("image_classification_mobilenet");
    cmd.arg(model_data.len().to_string())
        .arg("input")
        .arg("MobilenetV2/Predictions/Softmax")
        .arg("224")
        .arg("224");
    for m in model_data {
        cmd.stdin_u8(*m);
    }
    for rgb in resized.pixels() {
        cmd.stdin_u8(rgb[0] as u8)
            .stdin_u8(rgb[1] as u8)
            .stdin_u8(rgb[2] as u8);
    }
    // Call command.
    println!("Calling command ...");
    let out = cmd.output();
    if out.status != 0 {
      println!("{}", str::from_utf8(&out.stderr).unwrap());
      return out.status.to_string();
    }
    
    // Parse results.
    let stdout_json: Value = from_str(str::from_utf8(&out.stdout).expect("[]")).unwrap();
    let stdout_vec = stdout_json.as_array().unwrap();

    let mut i = 0;
    let mut max_index: i32 = -1;
    let mut max_value: f64 = -1.0;
    while i < stdout_vec.len() {
        let cur = stdout_vec[i].as_f64().unwrap();
        if cur > max_value {
            max_value = cur;
            max_index = i as i32;
        }
        i += 1;
    }
    println!("{} : {}", max_index, max_value);

    let mut confidence = "low";
    if max_value > 0.75 {
        confidence = "very high";
    } else if max_value > 0.5 {
        confidence = "high";
    } else if max_value > 0.2 {
        confidence = "medium";
    }

    let labels = include_str!("imagenet_slim_labels.txt");
    let mut label_lines = labels.lines();
    for _i in 0..max_index {
      label_lines.next();
    }
    let ret: (String, String) = (label_lines.next().unwrap().to_string(), confidence.to_string());
    return serde_json::to_string(&ret).unwrap();
}
