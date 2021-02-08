use wasm_bindgen::prelude::*;
use image;
use imageproc::{drawing};
use rusttype::{Font, Scale};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Watermark {
    text: String,
    left: u32,
    top: u32,
    font_size: f32,
}

const FONT_FILE : &[u8] = include_bytes!("LondrinaSolid-Black.ttf") as &[u8];

const TEMPLATE_BUF : &[u8] = include_bytes!("bg.png") as &[u8];

#[wasm_bindgen]
pub fn watermark(memes: &str) -> Vec<u8> {
  let mut img = image::load_from_memory(TEMPLATE_BUF).unwrap();

  let memes: Vec<Watermark> = serde_json::from_str(memes).unwrap();
  for m in memes {
    _watermark(m, &mut img);
  }

  let mut buf = vec![];
  img.write_to(&mut buf, image::ImageOutputFormat::Png).unwrap();
  return buf;
}

fn _watermark(w: Watermark, img: &mut image::DynamicImage) {
  let font_size = w.font_size;

  let font = Vec::from(FONT_FILE);
  let font = Font::try_from_vec(font).unwrap();

  let scale = Scale {
    x: font_size + 1.0,
    y: font_size + 1.0,
  };
  drawing::draw_text_mut(img, image::Rgba([0, 0, 0, 255u8]), w.left - 2, w.top - 2, scale, &font, &w.text);

  let scale = Scale {
    x: font_size,
    y: font_size,
  };
  drawing::draw_text_mut(img, image::Rgba([255u8, 255u8, 255u8, 255u8]), w.left, w.top, scale, &font, &w.text);
}