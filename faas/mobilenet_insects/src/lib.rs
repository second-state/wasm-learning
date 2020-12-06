use wasm_bindgen::prelude::*;
use ssvm_tensorflow_interface;
use std::str;
use std::time::{Instant};

#[wasm_bindgen]
pub fn infer(image_data: &[u8]) -> String {
    let start = Instant::now();
    let img = image::load_from_memory(image_data).unwrap().to_rgb();
    println!("Loaded image in ... {:?}", start.elapsed());
    let resized = image::imageops::thumbnail(&img, 192, 192);
    println!("Resized image in ... {:?}", start.elapsed());
    let mut flat_img: Vec<f32> = Vec::new();
    for rgb in resized.pixels() {
        flat_img.push(rgb[0] as f32 / 255.);
        flat_img.push(rgb[1] as f32 / 255.);
        flat_img.push(rgb[2] as f32 / 255.);
    }

    let model_data: &[u8] = include_bytes!("mobilenet_v2_192res_1.0_inat_insect.pb");
    let labels = include_str!("aiy_insects_V1_labelmap.txt");

    let mut session = ssvm_tensorflow_interface::Session::new(model_data, ssvm_tensorflow_interface::ModelType::TensorFlow);
    session.add_input("map/TensorArrayStack/TensorArrayGatherV3", &flat_img, &[1, 192, 192, 3])
           .add_output("prediction")
           .run();
    let res_vec: Vec<f32> = session.get_output("prediction");
    println!("Parsed output in ... {:?}", start.elapsed());

    let mut i = 0;
    let mut max_index: i32 = -1;
    let mut max_value: f32 = -1.0;
    while i < res_vec.len() {
        let cur = res_vec[i];
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
