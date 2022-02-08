use wasm_bindgen::prelude::*;
use ssvm_tensorflow_interface;
use image::{GenericImageView, Pixel};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use std::str;

#[wasm_bindgen]
pub fn infer(image_data: &[u8]) -> Vec<u8> {
    let mut img = image::load_from_memory(image_data).unwrap();
    let mut flat_img: Vec<f32> = Vec::new();
    for (_x, _y, rgb) in img.pixels() {
        flat_img.push(rgb[2] as f32);
        flat_img.push(rgb[1] as f32);
        flat_img.push(rgb[0] as f32);
    }

    let model_data: &[u8] = include_bytes!("detect_fast_nms.tflite");

    let mut session = ssvm_tensorflow_interface::Session::new(&model_data, ssvm_tensorflow_interface::ModelType::TensorFlowLite);
    session
           .add_input("input", &flat_img, &[300, 300, 3])
           .run();
    let res_vec: Vec<f32> = session.get_output("TFLite_Detection_PostProcess");
    //let num_class:Vec<f32>=session.get_output("TFLite_Detection_PostProcess:2");
    let res_score:Vec<f32>=session.get_output("TFLite_Detection_PostProcess:2");
    //let num:Vec<f32>=session.get_output("TFLite_Detection_PostProcess:3");
    // Parse results.
    let mut iter = 0;
    let mut box_vec: Vec<[f32; 4]> = Vec::new();
    //let mut num_vec:Vec<[f32; 4]> = Vec::new();

    while (iter * 4) < res_vec.len() {
        box_vec.push([
            res_vec[4 * iter + 1], // x1
            res_vec[4 * iter],     // y1
            res_vec[4 * iter + 3], // x2
            res_vec[4 * iter + 2], // y2
        ]);
        iter += 1;
    }

    println!("Parsed acc of reuslts are ... {:?}",res_score);

    //println!("Drawing box: {} results ...", box_vec.len());
    let line = Pixel::from_slice(&[0, 255, 255, 0]);
    for i in 0..box_vec.len() {
        if res_score[i]>0.5{

        let xy = box_vec[i];
        print!("{:?}.{:?}.{:?}.{:?}\n",xy[0],xy[1],xy[2],xy[3]);
        let x1: i32 = (xy[0]*(300 as f32)) as i32;
        let y1: i32 = (xy[1]*(300 as f32)) as i32;
        let x2: i32 =( xy[2]*(300 as f32)) as i32;
        let y2: i32 = (xy[3]*(300 as f32)) as i32;
        let rect = Rect::at(x1, y1).of_size(x2  as u32, y2  as u32);
        
        draw_hollow_rect_mut(&mut img, rect, *line);
        }
        
    }
    
    let mut buf = Vec::new();
    img.write_to(&mut buf, image::ImageOutputFormat::Jpeg(80u8)).expect("Unable to write");

    return buf;
}
