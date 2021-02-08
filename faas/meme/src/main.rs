use watermark_lib;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
  let out = watermark_lib::watermark(r#"[
    {
      "text": "CONTAINER",
      "left": 90,
      "top": 290,
      "font_size": 70.0
    },
    {
      "text": "IMAGE",
      "left": 140,
      "top": 340,
      "font_size": 70.0
    },
    {
      "text": "AWS",
      "left": 420,
      "top": 180,
      "font_size": 70.0
    },
    {
      "text": "LAMBDA",
      "left": 380,
      "top": 230,
      "font_size": 70.0
    },
    {
      "text": "ZIP",
      "left": 600,
      "top": 240,
      "font_size": 70.0
    },
    {
      "text": "PACKAGE",
      "left": 540,
      "top": 290,
      "font_size": 70.0
    }
  ]"#);

  let mut f = File::create("./result.png")?;
  f.write(&out)?;

  Ok(())
}
