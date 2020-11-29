use image::Rgba;
use image::ImageBuffer;
use image::DynamicImage;
use wasm_bindgen::prelude::*;
use image::{GenericImageView, ImageFormat};
use imageproc::geometric_transformations::Interpolation;
use imageproc::geometric_transformations::rotate_about_center;

#[wasm_bindgen]
pub fn rotate_an_image(img_buf: &[u8]) -> Vec<u8> {
    let dyn_img = image::load_from_memory(img_buf).unwrap();
    let (w,h) = dyn_img.dimensions();
    println!("Width: {:?}", w);
    println!("Height: {:?}", h);
    let image_format_detected: ImageFormat = image::guess_format(&img_buf).unwrap();
    println!("Format detected: {:?}", image_format_detected);

    // Convert - Option 1
    //let image_bytes = DynamicImage::into_bytes(dyn_img);
    //let image_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(w, h, image_bytes[..].to_vec()).unwrap();

    // Convert - Option 2
    println!("Converting DynamicImage to ImageBuffer ...");
    let image_bytes = DynamicImage::into_bytes(dyn_img);
    let image_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_vec(w, h, image_bytes[..].to_vec()).unwrap();
    println!("Rotating image ...");
    let rotated_image: ImageBuffer::<Rgba<u8>, Vec<u8>> = rotate_about_center(&image_buffer, 5.0, Interpolation::Nearest, Rgba([255, 0, 0, 0]));
    println!("Image rotated!");
    let mut buf = vec![];
    println!("Creating byte array to return");
    let img_to_write = image::load_from_memory(&rotated_image.into_raw()).unwrap();
    img_to_write.write_to(&mut buf, image::ImageOutputFormat::Png).unwrap();
    println!("Returning byte array now");
    return buf;
}