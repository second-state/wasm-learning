use image::{
  GenericImageView,
  DynamicImage,
  Rgba,
  ImageResult,
};

#[derive(Debug)]
pub struct Point {
  pub x: u32,
  pub y: u32,
}

pub struct ImageCrop {
  pub original: DynamicImage,
}

impl ImageCrop {
  pub fn new(img: DynamicImage) -> ImageResult<ImageCrop> {
    Ok(ImageCrop {
      original: img,
    })
  }

  pub fn calculate_corners(&self) -> (Point, Point) {
      (self.top_left_corner(), self.bottom_right_corner())
  }

  fn is_white(pixel: Rgba<u8>) -> bool {
      pixel[0] != 255 &&
      pixel[1] != 255 &&
      pixel[2] != 255
  }

  fn top_left_corner(&self) -> Point {
      Point {
          x: self.top_left_corner_x(),
          y: self.top_left_corner_y(),
      }
  }

  fn top_left_corner_x(&self) -> u32 {
      for x in 0..(self.original.dimensions().0) {
          for y in 0..(self.original.dimensions().1) {
              let pixel = self.original.get_pixel(x, y);
              if Self::is_white(pixel) {
                  return x;
              }
          }
      }
      unreachable!();
  }

  fn top_left_corner_y(&self) -> u32 {
      for y in 0..(self.original.dimensions().1) {
          for x in 0..(self.original.dimensions().0) {
              let pixel = self.original.get_pixel(x, y);
              if Self::is_white(pixel) {
                  return y;
              }
          }
      }
      unreachable!();
  }

  fn bottom_right_corner(&self) -> Point {
      Point {
          x: self.bottom_right_corner_x(),
          y: self.bottom_right_corner_y(),
      }
  }

  fn bottom_right_corner_x(&self) -> u32 {
      let mut x = self.original.dimensions().0 as i32 - 1;
      // Using while loop as currently there is no reliable built-in
      // way to use custom negative steps when specifying range
      while x >= 0 {
          let mut y = self.original.dimensions().1 as i32 - 1;
          while y >= 0 {
              let pixel = self.original.get_pixel(x as u32, y as u32);
              if Self::is_white(pixel) {
                  return x as u32 + 1;
              }
              y -= 1;
          }
          x -= 1;
      }
      unreachable!();
  }

  fn bottom_right_corner_y(&self) -> u32 {
      let mut y = self.original.dimensions().1 as i32 - 1;
      // Using while loop as currently there is no reliable built-in
      // way to use custom negative steps when specifying range
      while y >= 0 {
          let mut x = self.original.dimensions().0 as i32 - 1;
          while x >= 0 {
              let pixel = self.original.get_pixel(x as u32, y as u32);
              if Self::is_white(pixel) {
                  return y as u32 + 1;
              }
              x -= 1;
          }
          y -= 1;
      }
      unreachable!();
  }
}