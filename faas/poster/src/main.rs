use watermark_lib;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
  let out = watermark_lib::watermark("Second State Functions");

  let mut f = File::create("./tmp.png")?;
  f.write(&out)?;

  Ok(())
}
