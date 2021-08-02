use wasm_bindgen::prelude::*;
use ssvm_tensorflow_interface;
use image::{GenericImageView, Pixel, Rgba};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use std::str;
use std::time::{Instant};
// This is for labelling
use imageproc::drawing::{draw_text_mut};
use rusttype::{Font, Scale};


#[wasm_bindgen]
pub fn detect(image_data: &[u8]) -> Vec<u8> {
    let start = Instant::now();
    let mut img = image::load_from_memory(image_data).unwrap();
    let image_height: f32 = img.height() as f32;
    let image_width: f32 = img.width() as f32;
    let mut resized = image::imageops::thumbnail(&img, 320, 320);
    println!("Resized image in ... {:?}", start.elapsed());
    let mut flat_img: Vec<u8> = Vec::new();
    for rgb in resized.pixels() {
        flat_img.push(rgb[0]);
        flat_img.push(rgb[1]);
        flat_img.push(rgb[2]);
    }
    println!("Loaded image in ... {:?}", start.elapsed());

    let model_data: &[u8] = include_bytes!("/media/nvme/model.tflite");

    let mut session = ssvm_tensorflow_interface::Session::new(model_data, ssvm_tensorflow_interface::ModelType::TensorFlowLite);
    session.add_input("serving_default_images:0", &flat_img, &[1,320,320,3]);
    println!("Input added ...");
    session.add_output("Identity");
    println!("Output added ... Identity");
    // Bounding boxes
    session.add_output("StatefulPartitionedCall:3");
    // Categories of the objects detected
    session.add_output("StatefulPartitionedCall:2");
    // Scores of the detected boxes
    session.add_output("StatefulPartitionedCall:1");
    // The number of detected boxes
    session.add_output("StatefulPartitionedCall:0");
    println!("All preparation completed in ... {:?}", start.elapsed());
    session.run();
    println!("Session successfully ran in ... {:?}", start.elapsed());
    let res0: Vec<f32> = session.get_output("StatefulPartitionedCall:3");
    let res1: Vec<f32> = session.get_output("StatefulPartitionedCall:2");
    let res2: Vec<f32> = session.get_output("StatefulPartitionedCall:1");
    let res3: Vec<f32> = session.get_output("StatefulPartitionedCall:0");
    println!("Output obtained in ... {:?}", start.elapsed());
    println!("StatefulPartitionedCall:3:");
    println!("{:?}", res0);
    println!("StatefulPartitionedCall:2");
    println!("{:?}", res1);
    println!("StatefulPartitionedCall:1");
    println!("{:?}", res2);
    println!("StatefulPartitionedCall:0");
    println!("{:?}", res3);

    // Parse results.
    let mut iter = 0;
    let mut box_vec: Vec<[f32; 4]> = Vec::new();
    let mut label_vec: Vec<u8> = Vec::new();
    while (iter * 4) < res0.len() {
        // Check that the detection is high ranking
        if res2[iter] >= 0.3 {
            let x1 = res0[4 * iter + 1] * 512.;
            let y1 = res0[4 * iter] * 512.;
            let x2 = res0[4 * iter + 3]  * 512.;
            let y2 = res0[4 * iter + 2]  * 512.;
            if x1 > 0.0 && x2 > 0.0 && y1 > 0.0 && y2 > 0.0 { 
                box_vec.push([x1, y1, x2, y2]);
            }
            label_vec.push(res1[iter] as u8);
        }
        iter += 1;
    }
    println!("Parsed results in ... {:?}", start.elapsed());

    // Preparing to write the labels
    let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    println!("Drawing box: {} results ...", box_vec.len());
    let line = Pixel::from_slice(&[0, 255, 0, 0]);
    for i in 0..box_vec.len() {
        let xy = box_vec[i];
        let x1: i32 = xy[0] as i32;
        let y1: i32 = xy[1] as i32;
        let x2: i32 = xy[2] as i32;
        let y2: i32 = xy[3] as i32;
        println!("x1 : {:?}", x1);
        println!("y1 : {:?}", y1);
        println!("x2 : {:?}", x2);
        println!("y2 : {:?}", y2);
        let rect = Rect::at(x1, y1).of_size((x2 - x1) as u32, (y2 - y1) as u32);
        draw_hollow_rect_mut(&mut img, rect, *line);
        // There are 5 classes Salad, Seafood, Tomato, Baked goods, Cheese
        let mut text = "";
        if label_vec[i] == 1{
            text = "Tomato"
        }
        if label_vec[i] == 2{
            text = "2"
        }
        if label_vec[i] == 3{
            text = "3"
        }
        if label_vec[i] == 4{
            text = "4"
        }
        if label_vec[i] == 5{
            text = "5"
        }
        let mut yi: f32 = 0.0;
        if x1 - 15 > 15 {
            yi = x1 as f32 - 15.;
        } else {
            yi= x1 as f32 + 15.;
        }
        let scale = Scale {
            x: ((x2 - x1) / 6) as f32,
            y: ((x2 - x1) / 6) as f32,
        };
        println!("Drawing label at x: {:?} and y: {:?}", xy[0], yi);
        draw_text_mut(&mut img, Rgba([50u8, 50u8, 50u8, 0u8]), (x1 +1) as u32, (y1 +2) as u32, scale, &font, text);
        draw_text_mut(&mut img, Rgba([0u8, 255u8, 0u8, 0u8]), x1 as u32, y1 as u32, scale, &font, text);
    }

    // Save image to buffer so we can return it
    let mut buf = Vec::new();
    img.write_to(&mut buf, image::ImageOutputFormat::Jpeg(80u8)).expect("Unable to write");
    println!("Drawn on image in ... {:?}", start.elapsed());

    return buf;

}
