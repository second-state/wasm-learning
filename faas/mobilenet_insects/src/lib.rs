use wasm_bindgen::prelude::*;
use rust_process_interface_library::Command;
use serde_json::{from_str, Value};
use std::str;
use std::time::{Instant};

#[wasm_bindgen]
pub fn infer(image_data: &[u8]) -> String {
    let start = Instant::now();
    let img = image::load_from_memory(image_data).unwrap().to_rgb();
    println!("Loaded image in ... {:?}", start.elapsed());
    let resized = image::imageops::thumbnail(&img, 192, 192);
    println!("Resized image in ... {:?}", start.elapsed());
    // let resized = image::imageops::resize(&img, 224, 224, ::image::imageops::FilterType::Triangle);
    // let resized = image::imageops::resize(&img, 224, 224, ::image::imageops::FilterType::Nearest);

    let model_data: &[u8] = include_bytes!("mobilenet_v2_192res_1.0_inat_insect.pb");
    let labels = include_str!("aiy_insects_V1_labelmap.txt");

    let mut cmd = Command::new("mobilenet_v2");
    cmd.arg(model_data.len().to_string())
        .arg("map/TensorArrayStack/TensorArrayGatherV3")
        .arg("prediction")
        .arg("192")
        .arg("192");
    cmd.stdin_u8vec(model_data);
    println!("Sent model in ... {:?}", start.elapsed());
    for rgb in resized.pixels() {
        cmd.stdin_u8(rgb[0] as u8)
            .stdin_u8(rgb[1] as u8)
            .stdin_u8(rgb[2] as u8);
    }
    println!("Sent image in ... {:?}", start.elapsed());
    // Call command.
    let out = cmd.output();
    println!("Executed command in ... {:?}", start.elapsed());
    if out.status != 0 {
      println!("{}", str::from_utf8(&out.stderr).unwrap());
      return out.status.to_string();
    }
    
    // Parse results.
    let stdout_json: Value = from_str(str::from_utf8(&out.stdout).expect("[]")).unwrap();
    let stdout_vec = stdout_json.as_array().unwrap();
    println!("Parsed output in ... {:?}", start.elapsed());

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

    let mut confidence = "could be";
    if max_value > 0.75 {
        confidence = "is very likely";
    } else if max_value > 0.5 {
        confidence = "is likely";
    } else if max_value > 0.2 {
        confidence = "could be";
    } else {
        return "It does not appears to be any insect in the picture.".to_string();
    }

    let mut label_lines = labels.lines();
    for _i in 0..max_index {
      label_lines.next();
    }

    let plant_name = label_lines.next().unwrap().to_string();
    return format!("It {} a <a href='https://www.google.com/search?q={}'>{}</a> in the picture", confidence.to_string(), plant_name, plant_name);
}
