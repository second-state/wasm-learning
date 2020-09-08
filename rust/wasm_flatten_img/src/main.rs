use image::{GenericImageView, Pixel};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use rust_process_interface_library::Command;
use serde_json::{from_str, Value};
use std::error::Error;
use std::{env, str};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let img_in_path = &args[1];
    let img_out_path = &args[2];

    // Open image file and flatten.
    println!("Loading image ...");
    let mut img = image::open(img_in_path)?;
    let mut flattened: Vec<u32> = Vec::new();
    println!("Flattening ...");
    for (_x, _y, rgb) in img.pixels() {
        flattened.push(rgb[2] as u32);
        flattened.push(rgb[1] as u32);
        flattened.push(rgb[0] as u32);
    }

    // Call process.
    println!("Preparing command ...");
    let mut cmd = Command::new("face_detect");
    cmd.stdin(img.width().to_string())
        .stdin(" ")
        .stdin(img.height().to_string())
        .stdin(" ");
    for val in &flattened {
        cmd.stdin(val.to_string()).stdin(" ");
    }
    println!("Calling command ...");
    let out = cmd.output();

    // Parse results.
    let line = Pixel::from_slice(&[0, 255, 0, 0]);
    let stdout_json: Value = from_str(str::from_utf8(&out.stdout).expect("[]")).unwrap();
    let stdout_vec = stdout_json.as_array().unwrap();
    println!("Drawing box: {} results ...", stdout_vec.len());
    for i in 0..stdout_vec.len() {
        let xy = stdout_vec[i].as_array().unwrap();
        let x1: i32 = xy[0].as_f64().unwrap() as i32;
        let y1: i32 = xy[1].as_f64().unwrap() as i32;
        let x2: i32 = xy[2].as_f64().unwrap() as i32;
        let y2: i32 = xy[3].as_f64().unwrap() as i32;
        let rect = Rect::at(x1, y1).of_size((x2 - x1) as u32, (y2 - y1) as u32);
        draw_hollow_rect_mut(&mut img, rect, *line);
    }
    println!("Saving image ...");
    img.save(img_out_path)?;
    println!("Done.");
    Ok(())
}
