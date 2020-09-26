use wasm_bindgen::prelude::*;
use imageproc::{drawing};
use rusttype::{Font, Scale};
use image::{GenericImage,GenericImageView};

#[wasm_bindgen]
pub fn watermark (watermark_text: &str, img_buf: &[u8]) -> Vec<u8> {
    let mut img = image::load_from_memory(img_buf).unwrap();
    let (w,h) = img.dimensions();
    let scale = Scale {
      x: w as f32 /10.0,
      y: h as f32 /10.0,
    };

    let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    drawing::draw_text_mut(&mut img, image::Rgba([255u8, 255u8, 255u8, 255u8]), 0+(h/10),h/2, scale, &font, watermark_text);

    let mut buf = vec![];
    img.write_to(&mut buf, image::ImageOutputFormat::Png);
    return buf;
}
