#[macro_use]
extern crate serde_derive;

mod fs;
mod console;
mod sqlite3;

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

#[derive(Serialize, Deserialize)]
pub struct User {
  pub id: u32,
  pub full_name: String,
  pub created: String,
}

#[wasm_bindgen]
pub fn resize_file(input: &str) {
  let p: (Dimension, String, String) = serde_json::from_str(input).unwrap();

  console::time("Resize file");
  let raw = fs::read_file_sync(&p.1);
  console::time_log("Resize file", "Done reading");
  let src = Picture {
    dim: p.0,
    raw: raw,
  };
  let target = resize_impl(&src);
  console::time_log("Resize file", "Done resizing");

  fs::write_file_sync(&p.2, &target.raw);
  console::time_log("Resize file", "Done writing");

  fs::copy_file_sync(&p.2, "tmp.png");
  console::time_log("Resize file", "Done copying");

  fs::unlink_sync(&p.2);
  // fs::unlink_sync("tmp.png");
  console::time_log("Resize file", "Done deleting");

  console::time_end("Resize file");
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

#[wasm_bindgen]
pub fn create_sqlite() {
  let sql_create = "
CREATE TABLE users (
  id INTEGER PRIMARY KEY NOT NULL, 
  full_name TEXT NOT NULL, 
  created DATE NOT NULL
);";
  let sql_insert = "
INSERT INTO users 
VALUES 
(1, 'Bob McFett', '32-01-01'),
(2, 'Angus Vader', '02-03-04'),
(3, 'Imperator Colin', '01-01-01');";

  sqlite3::create("test.sqlite");
  sqlite3::update("test.sqlite", sql_create);
  sqlite3::update("test.sqlite", sql_insert);
}

#[wasm_bindgen]
pub fn query_sqlite() {
  let sql_query = "SELECT * FROM users;";
  let rows: String = sqlite3::query("test.sqlite", sql_query);
  let users: Vec<User> = serde_json::from_str(&rows).unwrap();
  for user in users.into_iter() {
    console::log(&(user.id.to_string() + " : " + &user.full_name));
  }
  fs::unlink_sync("test.sqlite");
}
