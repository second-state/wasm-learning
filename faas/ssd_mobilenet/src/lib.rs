use wasm_bindgen::prelude::*;
use ssvm_tensorflow_interface;
use std::str;
use std::time::{Instant};

#[wasm_bindgen]
pub fn infer(image_data: &[u8]) -> String {
    let start = Instant::now();
    let img = image::load_from_memory(image_data).unwrap().to_rgb();
    println!("Loaded image in ... {:?}", start.elapsed());
    // Resize to 300px x 300px
    let resized = image::imageops::thumbnail(&img, 300, 300);
    println!("Resized image in ... {:?}", start.elapsed());
    let mut flat_img: Vec<f32> = Vec::new();
    for rgb in resized.pixels() {
        flat_img.push(rgb[0] as f32 / 255.);
        flat_img.push(rgb[1] as f32 / 255.);
        flat_img.push(rgb[2] as f32 / 255.);
    }
    // Load TFLite model data
    let model_data: &[u8] = include_bytes!("ssd_mobilenet_v1_1_default_1.tflite");
    let labels = include_str!("imagenet_slim_labels.txt");

    let mut session = ssvm_tensorflow_interface::Session::new(model_data, ssvm_tensorflow_interface::ModelType::TensorFlow);
    session.add_input("input", &flat_img, &[1, 224, 224, 3])
           .add_output("MobilenetV2/Predictions/Softmax")
           .run();
    let res_vec: Vec<f32> = session.get_output("MobilenetV2/Predictions/Softmax");
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

    let mut confidence = "low";
    if max_value > 0.75 {
        confidence = "very high";
    } else if max_value > 0.5 {
        confidence = "high";
    } else if max_value > 0.2 {
        confidence = "medium";
    }

    let mut label_lines = labels.lines();
    for _i in 0..max_index {
      label_lines.next();
    }
    let ret: (String, String) = (label_lines.next().unwrap().to_string(), confidence.to_string());
    println!("Finished post-processing in ... {:?}", start.elapsed());
    return serde_json::to_string(&ret).unwrap();
}
