use wasm_bindgen::prelude::*;
use image::{GenericImageView, Pixel};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use rust_process_interface_library::Command;
use serde_json::{from_str, Value};
use std::str;

#[wasm_bindgen]
pub fn infer(image_data: &[u8]) -> Vec<u8> {
    // Open image file.
    println!("Loading image ...");
    let mut img = image::load_from_memory(image_data).unwrap();

    // Flatten image.
    println!("Preparing command ...");
    let mut cmd = Command::new("face_detect");
    cmd.arg(img.width().to_string())
        .arg(img.height().to_string());
    for (_x, _y, rgb) in img.pixels() {
        cmd.stdin_u8(rgb[2] as u8)
            .stdin_u8(rgb[1] as u8)
            .stdin_u8(rgb[0] as u8);
    }
    // Call command.
    println!("Calling command ...");
    let out = cmd.output();
    
    // Parse results.
    let line = Pixel::from_slice(&[0, 255, 0, 0]);
    let stdout_json: Value = from_str(str::from_utf8(&out.stdout).expect("[]")).unwrap();
    let stdout_vec = stdout_json.as_array().unwrap();
    println!("Drawing box: {} results ...", stdout_vec.len());
    for i in 0..stdout_vec.len() {
        let xy = stdout_vec[i].as_array().unwrap();
        let x1: i32 = xy[0].as_f64().unwrap() as i32;
        let y1: i32 = xy[1].as_f64().unwrap() as i32;
        let x2: i32 = xy[2].as_f64().unwrap() as i32;
        let y2: i32 = xy[3].as_f64().unwrap() as i32;
        let rect = Rect::at(x1, y1).of_size((x2 - x1) as u32, (y2 - y1) as u32);
        draw_hollow_rect_mut(&mut img, rect, *line);
    }
    
    let mut buf = Vec::new();
    img.write_to(&mut buf, image::ImageOutputFormat::Png).expect("Unable to write");
    println!("Done.");

    return buf;
}
