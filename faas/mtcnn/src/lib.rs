use wasm_bindgen::prelude::*;
use image::{GenericImageView, Pixel};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use rust_process_interface_library::Command;
use serde_json::{from_str, Value};
use std::str;
use std::time::{Instant};

#[wasm_bindgen]
pub fn infer(image_data: &[u8]) -> Vec<u8> {
    let start = Instant::now();
    let mut img = image::load_from_memory(image_data).unwrap();
    println!("Loaded image in ... {:?}", start.elapsed());

    let model_data: &[u8] = include_bytes!("mtcnn.pb");
    let model_params: &str = "{\"min_size\":[40],\"thresholds\":[0.6,0.7,0.7],\"factor\":[0.709]}";

    let mut cmd = Command::new("mtcnn");
    cmd.arg(model_data.len().to_string()) // model data length
        .arg("input") // Input tensor name
        .arg("box") // Output tensor name
        .arg(model_params) // Parameter tensor names and values
        .arg(img.width().to_string()) // Image width
        .arg(img.height().to_string()); // Image height
    cmd.stdin_u8vec(model_data);
    println!("Sent model in ... {:?}", start.elapsed());
    for (_x, _y, rgb) in img.pixels() {
        cmd.stdin_u8(rgb[2] as u8)
            .stdin_u8(rgb[1] as u8)
            .stdin_u8(rgb[0] as u8);
    }
    println!("Sent image in ... {:?}", start.elapsed());
    let out = cmd.output();
    println!("Executed command in ... {:?}", start.elapsed());
    if out.status != 0 {
        println!("ERROR CODE: {}", out.status);
        println!("STDERR: {}", str::from_utf8(&out.stderr).unwrap());
    }
    
    // Parse results.
    let stdout_json: Value = from_str(str::from_utf8(&out.stdout).expect("[]")).unwrap();
    let stdout_vec = stdout_json.as_array().unwrap();
    let mut iter = 0;
    let mut box_vec: Vec<[f32; 4]> = Vec::new();
    while (iter * 4) < stdout_vec.len() {
        box_vec.push([
            stdout_vec[4 * iter + 1].as_f64().unwrap() as f32, // x1
            stdout_vec[4 * iter].as_f64().unwrap() as f32,     // y1
            stdout_vec[4 * iter + 3].as_f64().unwrap() as f32, // x2
            stdout_vec[4 * iter + 2].as_f64().unwrap() as f32, // y2
        ]);
        iter += 1;
    }
    println!("Parsed results in ... {:?}", start.elapsed());

    println!("Drawing box: {} results ...", box_vec.len());
    let line = Pixel::from_slice(&[0, 255, 0, 0]);
    for i in 0..box_vec.len() {
        let xy = box_vec[i];
        let x1: i32 = xy[0] as i32;
        let y1: i32 = xy[1] as i32;
        let x2: i32 = xy[2] as i32;
        let y2: i32 = xy[3] as i32;
        let rect = Rect::at(x1, y1).of_size((x2 - x1) as u32, (y2 - y1) as u32);
        draw_hollow_rect_mut(&mut img, rect, *line);
    }
    
    let mut buf = Vec::new();
    img.write_to(&mut buf, image::ImageOutputFormat::Jpeg(80u8)).expect("Unable to write");
    println!("Drawn on image in ... {:?}", start.elapsed());

    return buf;
}
