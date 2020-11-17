use wasm_bindgen::prelude::*;
use image::{GenericImageView};

#[wasm_bindgen]
pub fn flip(img_buf: &[u8]) -> Vec<u8> {
    println!("image size is {}", img_buf.len());
    let img = image::load_from_memory(img_buf).unwrap();
    let (w,h) = img.dimensions();
    println!("Image size {} {}", w, h);
    println!("Drawing ...");
    let filtered = img.fliph();
    println!("Returning ...");
    let mut buf = vec![];
    filtered.write_to(&mut buf, image::ImageOutputFormat::Png).unwrap();
    return buf; 
}