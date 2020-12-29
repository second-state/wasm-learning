use wasm_bindgen::prelude::*;
use image::{ImageOutputFormat, GenericImageView, ImageFormat};

#[wasm_bindgen]
pub fn thumbnail(img_buf: &[u8]) -> Vec<u8> {
    println!("image size is {}", img_buf.len());
    let img = image::load_from_memory(img_buf).unwrap();
    let (w,h) = img.dimensions();
    println!("Image size {} {}", w, h);
    println!("Drawing ...");
    let width_thumb: u32 = 100;
    let height_thumb: u32 = 100;
    let filtered = img.thumbnail(width_thumb, height_thumb);
    println!("Returning ...");
    let mut buf = vec![];
    let image_format_detected: ImageFormat = image::guess_format(img_buf).unwrap();
    match image_format_detected {
        ImageFormat::Gif => {
            filtered.write_to(&mut buf, ImageOutputFormat::Gif).unwrap();

        },
        _ => {
            filtered.write_to(&mut buf, ImageOutputFormat::Png).unwrap();
        },
    }
    return buf;
}