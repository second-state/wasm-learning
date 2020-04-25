#[macro_use]
extern crate serde_derive;

mod fs;
mod console;

use std::io::{BufReader, Cursor};
use wasm_bindgen::prelude::*;
use image::{GenericImageView, png, ImageEncoder, ImageFormat};
use image::imageops::FilterType;

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Debug)]
pub struct Dimension {
  pub width: u32,
  pub height: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Picture {
  pub dim: Dimension,
  pub raw: Vec<u8>,
}

#[wasm_bindgen]
pub fn resize_file(input: &str) {
  let p: (Dimension, String, String) = serde_json::from_str(input).unwrap();

  console::time("Read file");
  let raw = fs::read_file_sync(&(p.1));
  console::time_end("Read file");
  let src = Picture {
    dim: p.0,
    raw: raw,
  };
  console::time("Resize");
  let target = resize_impl(&src);
  console::time_end("Resize");

  console::time("Write file");
  fs::write_file_sync(&(p.2), &(target.raw));
  console::time_end("Write file");
}

pub fn resize_impl(src: &Picture) -> Picture {
  let cur = Cursor::new(&src.raw);

  let fin = BufReader::new(cur);

  // load the `image::DynamicImage`
  let mut img = image::load(fin, ImageFormat::Png).unwrap();

  // Resize the img in the memory.
  img = img.resize_to_fill(src.dim.width, src.dim.height, FilterType::Lanczos3);

  let (w, h) = img.dimensions();

  // Write the resized image to the vector.
  let mut cur: Cursor<Vec<u8>>  = Cursor::new(vec![]);
  png::PNGEncoder::new(&mut cur).write_image(&img.to_bytes(), w, h, img.color()).unwrap();

  let dim = Dimension {
    width: w,
    height: h,
  };

  Picture {
    dim,
    raw: cur.into_inner(),
  }
}
