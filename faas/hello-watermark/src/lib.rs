use wasm_bindgen::prelude::*;
use imageproc::{drawing};
use rusttype::{Font, Scale};
use image::{GenericImageView};

#[wasm_bindgen]
pub fn watermark (img_buf: &[u8]) -> Vec<u8> {
    // Read the input image
    let mut img = image::load_from_memory(img_buf).unwrap();
    let (w,h) = img.dimensions();
    let scale = Scale {
      x: w as f32 /10.0,
      y: h as f32 /10.0,
    };

    // Prepare fonts for the watermark
    let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    // Draw the watermark on the input image
    drawing::draw_text_mut(&mut img, image::Rgba([255u8, 255u8, 255u8, 255u8]), 0+(h/10),h/2, scale, &font, "Hello Second State");

    // Write and return the watermarked image
    let mut buf = vec![];
    img.write_to(&mut buf, image::ImageOutputFormat::Png).unwrap();
    return buf;
}
