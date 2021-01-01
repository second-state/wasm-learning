use image::Bgra;
use image::ImageBuffer;
use image::DynamicImage;
use wasm_bindgen::prelude::*;
use image::{GenericImageView, ImageFormat};
use imageproc::geometric_transformations::Interpolation;
use imageproc::geometric_transformations::rotate_about_center;

#[wasm_bindgen]
pub fn rotate_an_image(img_buf: &[u8]) -> Vec<u8> {
    println!("Element 0: {:?}", img_buf[0]);
    println!("Element 1: {:?}", img_buf[1]);
    println!("Element 2: {:?}", img_buf[2]);
    let dyn_img = image::load_from_memory(img_buf).unwrap();
    let (w,h) = dyn_img.dimensions();
    println!("Width: {:?}", w);
    println!("Height: {:?}", h);
    let image_format_detected: ImageFormat = image::guess_format(&img_buf).unwrap();
    println!("Format detected: {:?}", image_format_detected);

    // Convert - Option 1
    let image_bytes = DynamicImage::as_bytes(&dyn_img);
    //println!("Element 0: {:?}", image_bytes[0]);
    //println!("Element 1: {:?}", image_bytes[1]);
    //println!("Element 2: {:?}", image_bytes[2]);
    //let image_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(w, h, image_bytes[..].to_vec()).unwrap();
    let image_buffer: ImageBuffer::<Bgra<u8>, Vec<u8>> = dyn_img.to_bgra8();

/*
    // Convert - Option 2
    println!("Converting DynamicImage to ImageBuffer ...");
    let image_bytes = DynamicImage::into_bytes(dyn_img);

    println!("Converted to bytes, success!");
    let image_buffer: ImageBuffer::<Rgba<u8>, Vec<u8>> = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_vec(w, h, image_bytes[..].to_vec()).unwrap();
    
    println!("TESTING - NOT Rotating image ...");
    let unrotated_rotated_image_as_arr: &[u8] = &image_buffer.into_vec();

    //println!("Rotating image ...");
    */
    let rotated_image: ImageBuffer::<Bgra<u8>, Vec<u8>> = rotate_about_center(&image_buffer, -0.095, Interpolation::Nearest, Bgra([255, 0, 0, 0]));
    //println!("Image rotated!");
    let mut buf = vec![];


    //let rotated_image_as_arr: &[u8] = &rotated_image.into_vec();
    //println!("Element 0: {:?}", rotated_image_as_arr[0]);
    //println!("Element 1: {:?}", rotated_image_as_arr[1]);
    //println!("Element 2: {:?}", rotated_image_as_arr[2]);
    //println!("{:?}", rotated_image_as_arr);
    match image::load_from_memory(&rotated_image.as_bytes()) {
        Ok(img_to_write) => {
            println!("Writing image data ...");
            img_to_write.write_to(&mut buf, image::ImageOutputFormat::Png).unwrap();
        }
        Err(e) => println!("Error: {:?}", &e.to_string()),
    };
    println!("Returning ...");
    
    return buf;
}