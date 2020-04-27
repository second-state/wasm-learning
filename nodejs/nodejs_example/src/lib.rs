#[macro_use]
extern crate serde_derive;

use std::io::{BufReader, Cursor};
use wasm_bindgen::prelude::*;
use image::{GenericImageView, png, ImageEncoder, ImageFormat};
use image::imageops::FilterType;
use nodejs_helper;

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

  nodejs_helper::console::time("Resize file");
  let raw = nodejs_helper::fs::read_file_sync(&p.1);
  nodejs_helper::console::time_log("Resize file", "Done reading");
  let src = Picture {
    dim: p.0,
    raw: raw,
  };
  let target = resize_impl(&src);
  nodejs_helper::console::time_log("Resize file", "Done resizing");

  nodejs_helper::fs::write_file_sync(&p.2, &target.raw);
  nodejs_helper::console::time_log("Resize file", "Done writing");

  nodejs_helper::fs::copy_file_sync(&p.2, "tmp.png");
  nodejs_helper::console::time_log("Resize file", "Done copying");

  nodejs_helper::fs::unlink_sync(&p.2);
  nodejs_helper::fs::unlink_sync("tmp.png");
  nodejs_helper::console::time_log("Resize file", "Done deleting");

  nodejs_helper::console::time_end("Resize file");
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

  nodejs_helper::sqlite3::create("test.sqlite");
  nodejs_helper::sqlite3::update("test.sqlite", sql_create);
  nodejs_helper::sqlite3::update("test.sqlite", sql_insert);
}

#[wasm_bindgen]
pub fn query_sqlite() {
  let sql_query = "SELECT * FROM users;";
  let rows: String = nodejs_helper::sqlite3::query("test.sqlite", sql_query);
  let users: Vec<User> = serde_json::from_str(&rows).unwrap();
  for user in users.into_iter() {
    nodejs_helper::console::log(&(user.id.to_string() + " : " + &user.full_name));
  }
  nodejs_helper::fs::unlink_sync("test.sqlite");
}

#[wasm_bindgen]
pub fn fetch(url: &str) {
  let content = nodejs_helper::request::fetch_as_string(url);
  nodejs_helper::console::log(url);
  nodejs_helper::console::log(&content);
}

#[wasm_bindgen]
pub fn download(url: &str, path: &str) {
  let content = nodejs_helper::request::fetch(url);
  nodejs_helper::fs::write_file_sync(path, &content);
}

#[wasm_bindgen]
pub fn show_now() {
  nodejs_helper::console::log("Timestamp now: ");
  nodejs_helper::console::log(&nodejs_helper::date::timestamp());
}

#[wasm_bindgen]
pub fn utc_now() {
  nodejs_helper::console::log("UTC time: ");
  nodejs_helper::console::log(&nodejs_helper::date::utc_string());
}

#[wasm_bindgen]
pub fn my_time(tz: &str) {
  nodejs_helper::console::log(tz);
  nodejs_helper::console::log(&nodejs_helper::date::format_date("en-US", "long", "numeric", "long", "numeric", tz, "short"));
}
